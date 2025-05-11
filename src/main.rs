mod game;
use game::board::Bitboard;


fn main() {
    let mut board = Bitboard::new(false);

    board = board.apply_move(26, false);
    board.valid = board.legal_moves(true);

    println!("{board}");
}