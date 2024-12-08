use aoc2024::day01::{day1_part1, day1_part2};
use aoc2024::day02::{day2_part1, day2_part2};
use aoc2024::day03::{day3_part1, day3_part2};
use aoc2024::day04::{day4_part1, day4_part2};
use aoc2024::day05::{day5_part1, day5_part2};
use aoc2024::day06::{day6_part1, day6_part2};
use aoc2024::day07::{day7_part1, day7_part2};
use aoc2024::day08::{day8_part1, day8_part2};
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn benchmark_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("aoc2024");

    group
        .sample_size(10)
        .measurement_time(std::time::Duration::from_secs(3));

    let all = std::env::var("PUZZLE").is_err();
    if all || std::env::var("PUZZLE").unwrap() == "d1p1" {
        group.bench_function("d1_part1", |b| b.iter(|| day1_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d1p2" {
        group.bench_function("d1_part2", |b| b.iter(|| day1_part2()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d2p1" {
        group.bench_function("d2_part1", |b| b.iter(|| day2_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d2p2" {
        group.bench_function("d2_part2", |b| b.iter(|| day2_part2()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d3p1" {
        group.bench_function("d3_part1", |b| b.iter(|| day3_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d3p2" {
        group.bench_function("d3_part2", |b| b.iter(|| day3_part2()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d4p1" {
        group.bench_function("d4_part1", |b| b.iter(|| day4_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d4p2" {
        group.bench_function("d4_part2", |b| b.iter(|| day4_part2()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d5p1" {
        group.bench_function("d5_part1", |b| b.iter(|| day5_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d5p2" {
        group.bench_function("d5_part2", |b| b.iter(|| day5_part2()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d6p1" {
        group.bench_function("d6_part1", |b| b.iter(|| day6_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d6p2" {
        group.bench_function("d6_part2", |b| b.iter(|| day6_part2()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d7p1" {
        group.bench_function("d7_part1", |b| b.iter(|| day7_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d7p2" {
        group.bench_function("d7_part2", |b| b.iter(|| day7_part2()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d8p1" {
        group.bench_function("d8_part1", |b| b.iter(|| day8_part1()));
    }
    if all || std::env::var("PUZZLE").unwrap() == "d8p2" {
        group.bench_function("d8_part2", |b| b.iter(|| day8_part2()));
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(
            100,
            Output::Flamegraph(None)
        ));
    targets = benchmark_all
}
criterion_main!(benches);
