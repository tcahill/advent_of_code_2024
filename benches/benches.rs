use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2024::RUN_FUNCS;


pub fn benchmark_year2024(c: &mut Criterion) {
    for (i, day) in RUN_FUNCS.iter().enumerate() {
        c.bench_function(&format!("day {} part 1", i+1), |b| b.iter(|| day[0](black_box(&fs::read_to_string(format!("input/2024/day{}.txt", i+1)).unwrap()))));
        c.bench_function(&format!("day {} part 2", i+1), |b| b.iter(|| day[1](black_box(&fs::read_to_string(format!("input/2024/day{}.txt", i+1)).unwrap()))));
    }
}

criterion_group!(benches, benchmark_year2024);
criterion_main!(benches);
