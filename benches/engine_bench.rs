use criterion::{criterion_group, criterion_main, Criterion};
use reversi::engine::engine::Engine;
use reversi::game::game::GameState;

fn minimax_benchmark(c: &mut Criterion) {

    let mut group = c.benchmark_group("minimax");

    group.measurement_time(std::time::Duration::from_secs(10)); // or however long you want
    group.sample_size(10); // Optional: how many samples Criterion collects

    let depth = 15;
    let engine = Engine::new(depth);
    let state = GameState::new();

    group.bench_function("minimax depth {depth}", |b| {
        b.iter(|| {
            // Important: don't optimize away
            criterion::black_box(engine.search(&state));
        });
    });
}

criterion_group!(benches, minimax_benchmark);
criterion_main!(benches);