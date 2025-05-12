use std::io::{self, Write};
mod game;
use game::game::GameState;
use game::r#move::Move;


fn main() {
    let mut game = GameState::new();

    loop {
        // Show game
        println!("{}", game);

        if game.game_over {
            match game.winner() {
                Some(player) => println!("Winner: {}", player),
                None => println!("It's a draw!"),
            }
            break;
        }
        println!("{} to move. Enter your move (e.g., E6):", game.to_move);

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input. Try again.");
            continue;
        }

        let trimmed = input.trim();
        let mv = Move::new(trimmed);
        // let mv: Move = match Move::new(trimmed) {
        //     Some(mv) => mv,
        //     None => {
        //         println!("Invalid move format, try again.");
        //         continue;
        //     }
        // };

        if (game.board.valid & (1u64 << mv.0)) == 0 {
            println!("Illegal move. Try again.");
            continue;
        }

        game.play_move(mv);
    }
}