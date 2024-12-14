use anyhow::{Context, Result};
use aoc_runner_derive::aoc;
use regex::Regex;

#[derive(Debug)]
struct CraneGame {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl TryFrom<&str> for CraneGame {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut lines = value.lines();
        let button_re = Regex::new(r"Button [A|B]: X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let prize_re = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

        let line = lines.next().unwrap();
        let capture_a = button_re.captures(line).context("parse error")?;
        let capture_b = button_re.captures(lines.next().context("parse error")?).context("parse error")?;
        let capture_prize = prize_re.captures(lines.next().context("parse error")?).context("parse error")?;

        Ok(CraneGame {
            a: (capture_a[1].parse()?, capture_a[2].parse()?),
            b: (capture_b[1].parse()?, capture_b[2].parse()?),
            prize: (capture_prize[1].parse()?, capture_prize[2].parse()?),
        })
    }
}

fn min_tokens(game: &CraneGame) -> Option<u64> {
    // a * a_x + b * b_x = p_x
    // a * a_y + b * b_y = p_y
    // a = (b_y * p_x - b_x * p_y) / (a_x * b_y - a_y  * b_x)
    // b = (p_y - a_y * a) / b_y

    let a_numerator = game.b.1 * game.prize.0 - game.b.0 * game.prize.1;
    let a_denominator = game.a.0 * game.b.1 - game.a.1 * game.b.0;

    if !(a_numerator % a_denominator == 0) {
        return None;
    }

    let a = a_numerator / a_denominator;
    let b = (game.prize.1 - game.a.1 * a) / game.b.1;

    Some((a * 3 + b) as u64)
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u64 {
    let games: Vec<CraneGame> = input.split("\n\n").map( |game| game.try_into() ).collect::<Result<Vec<CraneGame>>>().unwrap();
    games.iter().flat_map(|game| min_tokens(&game)).sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> u64 {
    let games: Vec<CraneGame> = input.split("\n\n")
        .map( |game| {
            let mut game: CraneGame = game.try_into()?;
            game.prize.0 += 10000000000000;
            game.prize.1 += 10000000000000;
            Ok(game)
        })
        .collect::<Result<Vec<CraneGame>>>().unwrap();

    games.iter().flat_map(|game| min_tokens(&game)).sum()
}
