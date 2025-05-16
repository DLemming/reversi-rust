use std::fmt;
use crate::game::board::Bitboard;
use crate::game::player::Player;

pub struct GameState {
    pub board: Bitboard,
    to_move: Player,
    game_over:  bool
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Bitboard::new(),
            to_move: Player::Black,
            game_over: false,
            // could potentially cache legal moves here,
            // but not that compute intense in thise context
        }
    }

    pub fn apply_move(&mut self, mv: u8) {
        // replaces current board with new one
        self.board = self.board.apply_move(mv, self.to_move.to_bool());
    }

    pub fn switch_player(&mut self) {
        self.to_move = self.to_move.opponent();
        let mut legal_moves = self.board.legal_moves(self.to_move.to_bool());

        if legal_moves == 0 {
            self.to_move = self.to_move.opponent();
            legal_moves = self.board.legal_moves(self.to_move.to_bool());

            if legal_moves == 0 {
                self.game_over = true;
            }
        }
    }

    pub fn winner(&self) -> Option<Player> {
        let (white_score, black_score) = self.board.score();

        if white_score > black_score {
            Some(Player::White)
        } else if black_score > white_score {
            Some(Player::Black)
        } else {
            None // Draw
        }
    }

    pub fn game_over(&self) -> bool {
        return self.game_over
    }

    pub fn current_player(&self) -> Player {
        match self.to_move {
            Player::White => Player::White,
            Player::Black => Player::Black,
        }
    }
}

// Implement the Display trait for GameState
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // row and col enumerating from top left, as per reversi board convention
        // top left is bit index 63 (msb)
        writeln!(f, "  A B C D E F G H")?;
        for row in 0..8 {
            write!(f, "{} ", row+1)?;
            for col in 0..8 {
                let idx = row * 8 + col;
                let mask = 1u64 << idx;
                let c = if self.board.black & mask != 0 {
                    'B'
                } else if self.board.white & mask != 0 {
                    'W'
                } else if self.board.legal_moves(self.current_player().to_bool()) & mask != 0 {
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