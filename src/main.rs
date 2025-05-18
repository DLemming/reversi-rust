use std::io::{self, Write};
mod game;
use game::game::GameState;
use game::r#move::Move;
use game::player::Player;

mod engine;
use engine::engine::Engine;

fn main() {
    let mut game: GameState = GameState::new();
    let engine = Engine::new(15);
    println!("{}", game);

    while !game.game_over() {
        println!("Current player: {}", game.current_player());

        // Get current player's move
        let mv: u64 = match game.current_player() {
            Player::White => get_engine_move(&game, &engine),
            Player::Black => get_human_move(&game),
        };

        game.apply_move(mv);
        game.switch_player();
        println!("{}", game);
    }

    match game.winner() {
        Some(player) => println!("Winner: {}", player),
        None => println!("It's a draw!"),
    }
}

fn get_human_move(game: &GameState) -> u64 {
    println!("Enter your move (e.g., E6):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input. Try again.");
            continue;
        }

        let trimmed = input.trim();

        let mv = match Move::new(trimmed) {
            Some(mv) => mv.0,
            None => {
                println!("Invalid move format, try again.");
                continue;
            }
        };

        if (game.board.legal_moves(game.current_player().to_bool()) & mv) == 0 {
            println!("Illegal move. Try again.");
            continue;
        }
        return mv;
    }
}

fn get_engine_move(game: &GameState, engine: &Engine) -> u64 {
    let (score, mv) = engine.search(game);

    let mv = match mv {
        Some(mv) => mv,
        None => {
            println!("ERROR. Engine did not find a move!");
            0
        }
    };

    println!(
        "Engine played: {}. Score in {} moves: {}.\n",
        Move::to_str(mv),
        engine.depth,
        score
    );
    println!("Nodes searched: {}", engine.node_counter.borrow());
    println!("Time elapsed: {:.2?}", engine.last_search_time.borrow());

    mv
}
