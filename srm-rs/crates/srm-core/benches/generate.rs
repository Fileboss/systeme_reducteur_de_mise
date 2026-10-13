use criterion::{Criterion, criterion_group, criterion_main};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use srm_core::generate::{AnnealParams, Budget, anneal::anneal, greedy::greedy};
use srm_core::odds::best_match_distribution;
use srm_core::space::Space;
use srm_core::table::Table;

fn bench(c: &mut Criterion) {
    let space = Space::new(20, 3).unwrap();
    c.bench_function("glouton 20/3", |b| {
        b.iter(|| greedy(&space, &mut ChaCha8Rng::seed_from_u64(1)))
    });

    let start = greedy(&space, &mut ChaCha8Rng::seed_from_u64(1));
    c.bench_function("recuit 20/3, 100 000 mouvements", |b| {
        b.iter(|| {
            let budget = Budget {
                max_moves: 100_000,
                deadline: None,
            };
            anneal(
                &space,
                start.clone(),
                AnnealParams::default(),
                budget,
                &mut ChaCha8Rng::seed_from_u64(1),
            )
        })
    });

    let grids = Table::embedded(20, 3).unwrap().grids().to_vec();
    c.bench_function("distribution exacte, 20/3", |b| {
        b.iter(|| best_match_distribution(&grids))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench
}
criterion_main!(benches);
