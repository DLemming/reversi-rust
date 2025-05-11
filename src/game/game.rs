use std::fmt;
use super::board::Bitboard;

pub struct Move(pub u8);

impl Move {
    /// Create a Move from a string like "C3"
    pub fn new(s: &str) -> Self {
        Self::from_str(s).unwrap_or_else(|| panic!("Invalid move string: {}", s))
    }

    /// Create a Move from file ('A'-'H') and rank ('1'-'8')
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }

        let file = s.chars().next().unwrap().to_ascii_uppercase();
        let rank = s.chars().nth(1).unwrap();

        if !('A'..='H').contains(&file) || !('1'..='8').contains(&rank) {
            return None;
        }

        let x = (file as u8) - b'A';
        let y = (rank as u8) - b'1';
        Some(Self(y * 8 + x))
    }

    /// Convert back to "A1"-style string
    pub fn to_str(self) -> String {
        let x = self.0 % 8;
        let y = self.0 / 8;
        let file = (b'A' + x) as char;
        let rank = (b'1' + y) as char;
        format!("{}{}", file, rank)
    }
}

enum Player { Black, White }

impl Player {
    fn to_bool(&self) -> bool {
        matches!(self, Player::White) // true for White, false for Black
    }

    fn opponent(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

// Implement the Display trait for Player
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let player_str = match self {
            Player::Black => "Black",
            Player::White => "White",
        };
        write!(f, "{}", player_str)
    }
}

pub struct GameState {
    board: Bitboard,
    to_move: Player,
    game_over:  bool
}

impl GameState {
    pub fn new() -> Self {
        let to_move: Player = Player::Black;
        let board: Bitboard = Bitboard::new(to_move.to_bool());
        let game_over: bool = false;

        println!("Initial State:\n{}", board);

        GameState {
            board,
            to_move,
            game_over
        }
    }

    pub fn play_move(&mut self, mv: Move) {
        self.board.apply_move(mv.0, self.to_move.to_bool());
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
        println!("Current Player: {}", self.to_move);
        println!("{}", self.board);
    }
}