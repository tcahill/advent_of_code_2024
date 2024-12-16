use anyhow::{anyhow, Result};
use std::{env, fmt::Display, fs};
use advent_of_code_2024;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &fs::read_to_string(format!("input/2024/day{}.txt", args[1])).unwrap();

    let run: fn(&str) -> u64 = match args[1].as_str() {
        "1" => {
            match args[2].as_str() {
                "1" => advent_of_code_2024::day1::part1,
                "2" => advent_of_code_2024::day1::part2,
                _ => { return; },
            }
        },
        _ => { return; }
    };

    let mut solution = 0;
    for _ in 0..100 {
        solution = run(input);
    }

    println!("{}", solution);
}
