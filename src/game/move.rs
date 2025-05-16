pub struct Move(pub u8);

impl Move {
    /// Create a Move from a string like "C3"
    pub fn new(s: &str) -> Option<Self> {
        Self::from_str(s)
    }

    /// Create a Move from file ('A'-'H') and rank ('1'-'8')
    fn from_str(s: &str) -> Option<Self> {
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
    pub fn to_str(mv: u8) -> String {
        let x = mv % 8;
        let y = mv / 8;
        let file = (b'A' + x) as char;
        let rank = (b'1' + y) as char;
        format!("{}{}", file, rank)
    }
}