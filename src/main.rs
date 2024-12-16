use std::fs;
use advent_of_code_2024::RUN_FUNCS;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: usize,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file = args.file.unwrap_or(format!("input/2024/day{}.txt", args.day));

    let input = &fs::read_to_string(&file).unwrap();

    let run = RUN_FUNCS[args.day - 1][args.part - 1];

    let mut solution = 0;
    for _ in 0..args.count {
        solution = run(input);
    }

    println!("{}", solution);
}
