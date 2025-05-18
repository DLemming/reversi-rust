use crate::game::board::Bitboard;

pub struct Node {
    pub board: Bitboard,
    pub is_white: bool,
    pub legal_moves: u64,
}

impl Node {
    pub fn new(board: Bitboard, is_white: bool, legal_moves: u64) -> Self {
        Node {
            board,
            is_white,
            legal_moves,
        }
    }

    #[inline(always)]
    pub fn apply_move(&self, mv: u64) -> Self {
        let new_board = self.board.apply_move(mv, self.is_white);
        let (new_player, legal_moves) = switch_player(&new_board, self.is_white);

        Node::new(new_board, new_player, legal_moves)
    }
}

#[inline(always)]
fn switch_player(board: &Bitboard, is_white: bool) -> (bool, u64) {
    let opponent = !is_white;
    let legal_moves = board.legal_moves(opponent);

    match legal_moves == 0 {
        true => (is_white, board.legal_moves(is_white)), // current player moves again
        false => (opponent, legal_moves),                // regular switch
    }
}
