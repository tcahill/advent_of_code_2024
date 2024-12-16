use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2024;

pub fn benchmark_year2024(c: &mut Criterion) {
    c.bench_function("day 1 part 1", |b| b.iter(|| advent_of_code_2024::day1::part1(black_box(&fs::read_to_string("input/2024/day1.txt").unwrap()))));
    c.bench_function("day 1 part 2", |b| b.iter(|| advent_of_code_2024::day1::part2(black_box(&fs::read_to_string("input/2024/day1.txt").unwrap()))));
}

criterion_group!(benches, benchmark_year2024);
criterion_main!(benches);
