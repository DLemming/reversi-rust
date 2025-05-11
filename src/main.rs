mod game;
use game::game::GameState;
use game::game::Move;


fn main() {
    let mut reversi_game = GameState::new();

    reversi_game.play_move(Move::new("E6")); // now black to move
    reversi_game.play_move(Move::new("F6"));
    reversi_game.play_move(Move::new("F5"));
    reversi_game.play_move(Move::new("D6"));
    reversi_game.play_move(Move::new("C7"));
    reversi_game.play_move(Move::new("F4"));
    reversi_game.play_move(Move::new("G6"));
    reversi_game.play_move(Move::new("D7"));
    reversi_game.play_move(Move::new("E7"));
    reversi_game.play_move(Move::new("D8"));
    reversi_game.play_move(Move::new("C8"));
    reversi_game.play_move(Move::new("B8"));
    reversi_game.play_move(Move::new("B7"));
    reversi_game.play_move(Move::new("G5"));
}