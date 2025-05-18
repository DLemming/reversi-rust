use criterion::{criterion_group, criterion_main, Criterion};
use reversi::engine::engine::Engine;
use reversi::game::game::GameState;

fn game_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimax");

    group.measurement_time(std::time::Duration::from_secs(10)); // or however long you want
    group.sample_size(10); // Optional: how many samples Criterion collects

    let depth = 9;
    
    group.bench_function("Run entire game", |b| {
        b.iter(|| {
            let engine = Engine::new(depth);
            let mut game = GameState::new();

            while !game.game_over() {
                let mv: u64 = get_engine_move(&game, &engine);

                game.apply_move(mv);
                game.switch_player();
            }

            fn get_engine_move(game: &GameState, engine: &Engine) -> u64 {
                let (_score, mv) = engine.search(game);

                let mv: u64 = match mv {
                    Some(mv) => mv,
                    None => {
                        println!("ERROR. Should not happen. Engine did not find a move!");
                        0
                    }
                }; 
                mv
            }
        });
    });
}

criterion_group!(benches, game_benchmark);
criterion_main!(benches);