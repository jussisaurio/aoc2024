use aoc2024::day06::day6_part2;
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn benchmark_day06(c: &mut Criterion) {
    let mut group = c.benchmark_group("day06");

    group
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(20))
        .bench_function("part2", |b| b.iter(|| day6_part2()));

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(
            100,
            Output::Flamegraph(None)
        ));
    targets = benchmark_day06
}
criterion_main!(benches);
