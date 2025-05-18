use std::fmt;

pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn to_bool(&self) -> bool {
        matches!(self, Player::White) // true for White, false for Black
    }

    pub fn opponent(&self) -> Self {
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
