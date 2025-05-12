use std::fmt;
use super::board::Bitboard;
use super::player::Player;
use super::r#move::Move;

pub struct GameState {
    pub board: Bitboard,
    pub to_move: Player,
    pub game_over:  bool
}

impl GameState {
    pub fn new() -> Self {
        let to_move: Player = Player::Black;
        let board: Bitboard = Bitboard::new(to_move.to_bool());
        let game_over: bool = false;

        GameState {
            board,
            to_move,
            game_over
        }
    }

    pub fn play_move(&mut self, mv: Move) {
        // replaces current board with new one
        self.board = self.board.apply_move(mv.0, self.to_move.to_bool());
        self.to_move = self.to_move.opponent();

        let mut legal_moves = self.board.legal_moves(self.to_move.to_bool());
        if legal_moves == 0 {
            // pass turn
            self.to_move = self.to_move.opponent();

            legal_moves = self.board.legal_moves(self.to_move.to_bool());
            if legal_moves == 0 {
                self.game_over = true;
            }
        }
        self.board.valid = legal_moves;
    }

    pub fn winner(&self) -> Option<Player> {
        let white_score = self.board.score(Player::White.to_bool());
        let black_score = self.board.score(Player::Black.to_bool());

        if white_score > black_score {
            Some(Player::White)
        } else if black_score > white_score {
            Some(Player::Black)
        } else {
            None // Draw
        }
    }
}

// Implement the Display trait for GameState
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //println!("Current Player: {}", self.to_move);
        write!(f, "{}", self.board)
    }
}
