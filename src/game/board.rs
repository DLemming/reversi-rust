use std::ops::{Shl, Shr};

#[derive(Copy, Clone)]
pub struct Bitboard {
    pub white: u64,
    pub black: u64,
}

pub struct BitIter(pub u64);

impl Bitboard {
    pub fn new() -> Self {
        // Starting position: center four
        let white = (1u64 << 27) | (1u64 << 36);
        let black = (1u64 << 28) | (1u64 << 35) ;

        // test complex position
        //let white = 0x244A148810000000;
        //let black = 0x8A15681142240000;

        // Build board
        Bitboard { white, black }
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

    /// Sweep in all 8 directions and flip discs if necessary
    pub fn apply_move(&self, bit_idx: u8, is_white: bool) -> Bitboard {
        let (player, opponent) = self.get_sides(is_white);

        // sweep along all 8 directions
        let mut flips = 0;
        for delta in DIRECTIONS {
            flips |= flips_dir(player, opponent, bit_idx, delta);
        }

        // Return new, updated bitboard
        let move_bit = 1u64 << bit_idx;
        if is_white {
            let white = self.white | move_bit | flips;
            let black = self.black & !flips;
            return Bitboard { white, black }
        } else {
            let black = self.black | move_bit | flips;
            let white = self.white & !flips;
            return Bitboard { white, black }
        }
    }

    /// Get a player scores
    pub fn score(&self) -> (i8, i8) {
        (self.white.count_ones() as i8, self.black.count_ones() as i8)
    }

    /// Helper to get current player's and opponent's disks
    #[inline(always)]
    fn get_sides(&self, is_white: bool) -> (u64, u64) {
        match is_white {
            true => (self.white, self.black),
            false => (self.black, self.white)
        }
    }

}

impl Iterator for BitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let bit = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1; // clear lowest set bit
            Some(bit)
        }
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
#[inline(always)]
fn moves_dir(player_disks: u64, opponent_disks: u64, delta: i8, mask: u64) -> u64 {
    let shift = delta.abs() as u8;
    let mask_opponent = opponent_disks & mask;

    // 0) define shift function closure to avoid branching
    let bitshift: fn(u64, u8) -> u64 = if delta > 0 { u64::shl } else { u64::shr };
    
    // 2) sweep opponent chain
    let mut temp = bitshift(player_disks, shift) & mask_opponent;
    let mut flips = temp;
    for _ in 0..5 {
        temp = bitshift(temp, shift) & mask_opponent;
        flips |= temp;
    }

    // 3) step into candidate square (square right after traversed opponents)
    bitshift(flips, shift)
}

/// Compute all disks to flip given a move
#[inline(always)]
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
