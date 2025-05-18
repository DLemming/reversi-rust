use std::ops::{Shl, Shr};

#[derive(Copy, Clone)]
pub struct Bitboard {
    pub white: u64,
    pub black: u64,
}

impl Bitboard {
    pub fn new() -> Self {
        // Starting position: center four
        let white = (1u64 << 27) | (1u64 << 36);
        let black = (1u64 << 28) | (1u64 << 35);

        // Build board
        Bitboard { white, black }
    }

    /// Returns a bitmask of all empty squares where `side` can play
    #[inline(always)]
    pub fn legal_moves(&self, is_white: bool) -> u64 {
        let (player, opponent) = if is_white {
            (self.white, self.black)
        } else {
            (self.black, self.white)
        };

        // Sweep in all 8 directions given delta_bit
        let mut moves: u64 = 0;
        moves |= moves_dir(player, opponent, -9, NOT_A_FILE_OR_ROW_1); // NW
        moves |= moves_dir(player, opponent, -8, NOT_ROW_1); // North
        moves |= moves_dir(player, opponent, -7, NOT_H_FILE_OR_ROW_1); // NE
        moves |= moves_dir(player, opponent, -1, NOT_A_FILE); // West
        moves |= moves_dir(player, opponent, 1, NOT_H_FILE); // East
        moves |= moves_dir(player, opponent, 7, NOT_A_FILE_OR_ROW_8); // SW
        moves |= moves_dir(player, opponent, 8, NOT_ROW_8); // South
        moves |= moves_dir(player, opponent, 9, NOT_H_FILE_OR_ROW_8); // SE

        // Only empty squares
        moves & !(player | opponent)
    }

    /// Sweep in all 8 directions and flip discs if necessary
    #[inline(always)]
    pub fn apply_move(&self, mv: u64, is_white: bool) -> Bitboard {
        let white;
        let black;

        if is_white {
            (white, black) = sweep(self.white, self.black, mv);
        } else {
            (black, white) = sweep(self.black, self.white, mv);
        };

        Bitboard { white, black }
    }

    /// Get a player scores
    #[inline(always)]
    pub fn score(&self) -> (i8, i8) {
        (self.white.count_ones() as i8, self.black.count_ones() as i8)
    }
}

pub struct BitIter64(pub u64);

impl Iterator for BitIter64 {
    type Item = u64;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let bit = 1u64 << self.0.trailing_zeros();
            self.0 &= self.0 - 1; // clear lowest set bit
            Some(bit)
        }
    }
}

// --------------------------------------
// Internal helpers & constants
// --------------------------------------

/* const _DIRECTIONS: [i8; 8] = [
    -9, -8, -7,
    -1,      1,
     7,  8,  9
]; */

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
const NOT_ROW_1: u64 = 0xffffffffffffff00;
const NOT_ROW_8: u64 = 0x00ffffffffffffff;
const NOT_A_FILE_OR_ROW_1: u64 = NOT_A_FILE & NOT_ROW_1;
const NOT_A_FILE_OR_ROW_8: u64 = NOT_A_FILE & NOT_ROW_8;
const NOT_H_FILE_OR_ROW_1: u64 = NOT_H_FILE & NOT_ROW_1;
const NOT_H_FILE_OR_ROW_8: u64 = NOT_H_FILE & NOT_ROW_8;

// zero overhead, wouldn't have done this otherwise
#[inline(always)]
fn sweep(player: u64, opponent: u64, mv: u64) -> (u64, u64) {
    // sweep along all 8 directions
    let mut flips = 0;
    flips |= flips_dir_faster(player, opponent, mv, -9, NOT_A_FILE_OR_ROW_1); // NW
    flips |= flips_dir_faster(player, opponent, mv, -8, NOT_ROW_1); // North
    flips |= flips_dir_faster(player, opponent, mv, -7, NOT_H_FILE_OR_ROW_1); // NE
    flips |= flips_dir_faster(player, opponent, mv, -1, NOT_A_FILE); // West
    flips |= flips_dir_faster(player, opponent, mv, 1, NOT_H_FILE); // East
    flips |= flips_dir_faster(player, opponent, mv, 7, NOT_A_FILE_OR_ROW_8); // SW
    flips |= flips_dir_faster(player, opponent, mv, 8, NOT_ROW_8); // South
    flips |= flips_dir_faster(player, opponent, mv, 9, NOT_H_FILE_OR_ROW_8); // SE
    let player = player | mv | flips;
    let opponent = opponent & !flips;
    (player, opponent)
}

/// Compute all candidate moves in one direction:
#[inline(always)]
fn moves_dir(player_disks: u64, opponent_disks: u64, delta: i8, mask: u64) -> u64 {
    let shift = delta.abs() as u8;
    let opponent_masked = opponent_disks & mask;

    // 0) define shift function to avoid branching
    let bitshift: fn(u64, u8) -> u64 = if delta > 0 { u64::shl } else { u64::shr };

    // 2) sweep opponent chain
    let mut temp = bitshift(player_disks, shift) & opponent_masked;
    let mut flips = temp;

    for _ in 0..5 {
        temp = bitshift(temp, shift) & opponent_masked;
        flips |= temp;
    }

    // 3) step into candidate square (square right after traversed opponents)
    bitshift(flips, shift)
}

/// Compute all disks to flip given a move
#[inline(always)]
// slower but better to understand version of flips_dir_faster
fn _flips_dir(player: u64, opponent: u64, mv: u64, delta: i8, mask: u64) -> u64 {
    let shift = delta.abs() as u8;
    let bitshift: fn(u64, u8) -> u64 = if delta > 0 { u64::shl } else { u64::shr };

    // start bit = the move played
    let mut flips = 0u64;
    let mut bit = mv & mask;
    bit = bitshift(bit, shift);

    while (bit & opponent) != 0 {
        flips |= bit; // collect potential flips

        bit &= mask;
        bit = bitshift(bit, shift);
    }

    if (bit & player) != 0 { flips } else { 0 }
}

#[inline(always)]
fn flips_dir_faster(player: u64, opponent: u64, mv: u64, delta: i8, mask: u64) -> u64 {
    let shift = delta.abs() as u8;
    let bitshift: fn(u64, u8) -> u64 = if delta > 0 { u64::shl } else { u64::shr };

    let opponent_masked = opponent & mask;

    let b0 = mv & mask; // origin masked
    let m1 = opponent_masked & bitshift(b0, shift); // first step: is the neighbor an opponent?
    let m2 = opponent_masked & bitshift(m1, shift); // two in a row?
    let m3 = opponent_masked & bitshift(m2, shift); // three in a row?
    let m4 = opponent_masked & bitshift(m3, shift); // four in a row?
    let m5 = opponent_masked & bitshift(m4, shift); // five in a row?
    let m6 = opponent_masked & bitshift(m5, shift); // six a row?

    let flips = m1 | m2 | m3 | m4 | m5 | m6; // valid disks that will be flipped, if next disk is player
    let cap = bitshift(flips, shift); // shift possible flips 
    if (cap & player) != 0 { flips } else { 0 }
}
