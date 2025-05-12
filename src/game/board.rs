use std::fmt;

// --------------------------------------
// Public API
// --------------------------------------

pub struct Bitboard {
    pub white: u64,
    pub black: u64,
    /// precomputed legal moves for the *side to move* (set in `new`)
    pub valid: u64,
}

impl Bitboard {
    pub fn new(is_white: bool) -> Self {
        // Starting position: center four
        let white = (1u64 << 27) | (1u64 << 36);
        let black = (1u64 << 28) | (1u64 << 35) ;

        // test complex position
        //let white = 0x244A148810000000;
        //let black = 0x8A15681142240000;

        // Build board and then compute legal moves
        let mut board = Bitboard { white, black, valid: 0 };
        board.valid = board.legal_moves(is_white);
        board
    }

    /// Returns a bitmask of all empty squares where `side` can play
    pub fn legal_moves(&self, is_white: bool) -> u64 {
        let (player, opponent) = self.get_sides(is_white);

        // Sweep in all 8 directions given delta_bit
        let mut moves: u64 = 0;
        moves |= moves_dir(player, opponent, 1, NOT_H_FILE);              // East
        moves |= moves_dir(player, opponent, -1, NOT_A_FILE);             // West
        moves |= moves_dir(player, opponent, 8, NOT_ROW_8);               // South
        moves |= moves_dir(player, opponent, -8, NOT_ROW_1);              // North
        moves |= moves_dir(player, opponent, 9, NOT_H_FILE_OR_ROW_8);     // SE
        moves |= moves_dir(player, opponent, 7, NOT_A_FILE_OR_ROW_8);     // SW
        moves |= moves_dir(player, opponent, -7, NOT_H_FILE_OR_ROW_1);    // NE
        moves |= moves_dir(player, opponent, -9, NOT_A_FILE_OR_ROW_1);    // NW

        // Only empty squares
        moves & !(player | opponent)
    }

    // Sweep in all 8 directions and flip discs if necessary
    pub fn apply_move(&self, bit_idx: u8, is_white: bool) -> Bitboard {
        let (player, opponent) = self.get_sides(is_white);

        let move_bit = 1u64 << bit_idx;
        let mut flips = 0;

        // sweep along all 8 directions
        for delta in DIRECTIONS {
            flips |= flips_dir(player, opponent, bit_idx, delta);
        }

        let mut white = self.white;
        let mut black = self.black;

        if is_white {
            white |= move_bit | flips;
            black &= !flips;
        } else {
            black |= move_bit | flips;
            white &= !flips;
        }

        Bitboard { white, black, valid: 0}
    }

    /// Get Player, Opponent from Board and is_white
    fn get_sides(&self, is_white: bool) -> (u64, u64) {
        if is_white {
            (self.white, self.black)
        } else {
            (self.black, self.white)
        }
    }

    /// Get a players score
    pub fn score(&self, is_white: bool) -> u32 {
        if is_white {
            self.white.count_ones()
        } else {
            self.black.count_ones()
        }
    }
}


// --------------------------------------
// Display impl: pretty‐print board + valid
// --------------------------------------

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // row and col enumerating from top left, as per reversi board convention
        // top left is bit index 63 (msb)
        writeln!(f, "  A B C D E F G H")?;
        for row in 0..8 {
            write!(f, "{} ", row+1)?;
            for col in 0..8 {
                let idx = row * 8 + col;
                let mask = 1u64 << idx;
                let c = if self.black & mask != 0 {
                    'B'
                } else if self.white & mask != 0 {
                    'W'
                } else if self.valid & mask != 0 {
                    '□' // legal move
                } else {
                    '.'
                };
                write!(f, "{} ", c)?;
            }
            write!(f, "{}", row+1)?;
            writeln!(f)?;
        }
        writeln!(f, "  A B C D E F G H")?;
        Ok(())
    }
}


// --------------------------------------
// Internal helpers & constants
// --------------------------------------

const DIRECTIONS: [i8; 8] = [
    -9, -8, -7,
    -1,      1,
     7,  8,  9
];

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
const NOT_ROW_1:  u64 = 0xffffffffffffff00;
const NOT_ROW_8:  u64 = 0x00ffffffffffffff;

const NOT_A_FILE_OR_ROW_1: u64 = NOT_A_FILE & NOT_ROW_1;
const NOT_A_FILE_OR_ROW_8: u64 = NOT_A_FILE & NOT_ROW_8;
const NOT_H_FILE_OR_ROW_1: u64 = NOT_H_FILE & NOT_ROW_1;
const NOT_H_FILE_OR_ROW_8: u64 = NOT_H_FILE & NOT_ROW_8;

/// Compute all candidate moves in one direction:
/// - `player`, `opponent`: bitboards
/// - `delta`: positive = <<, negative = >>
/// - `edge_mask`: to prevent wrap‐around
///
fn moves_dir(player_disks: u64, opponent_disks: u64, delta: i8, mask: u64) -> u64 {
    let shift = delta.abs() as u8;
    let mask_opponent = opponent_disks & mask;

    // 1) adjacent opponent stones
    let mut temp = if delta > 0 {
        (player_disks << shift) & mask_opponent
    } else {
        (player_disks >> shift) & mask_opponent
    };
    
    // 2) sweep opponent chain
    let mut flips = temp;
    for _ in 0..5 {
        temp = if delta > 0 {
            (temp << shift) & mask_opponent
        } else {
            (temp >> shift) & mask_opponent
        };
        flips |= temp;
    }

    // 3) step into candidate square (square right after traversed opponents)
    if delta > 0 {
        flips << shift
    } else {
        flips >> shift
    }
}

/// Compute all disks to flip given a move
fn flips_dir(player: u64, opponent: u64, bit_idx: u8, delta: i8) -> u64 {
    let mut flips = 0u64;
    let mut current = bit_idx as i8;

    loop {
        current += delta;

        // Check board bounds
        if current < 0 || current > 63 {
            return 0;
        }

        let bit = 1u64 << current;

        if bit & opponent != 0 {
            flips |= bit; // potential flip
        } else if bit & player != 0 {
            return flips; // confirmed flip
        } else {
            return 0; // empty square or invalid
        }
    }
}