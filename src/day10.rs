use std::collections::HashSet;

use anyhow::Result;
use aoc_runner_derive::aoc;

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect()
}

fn sum_trailhead_scores(grid: &[Vec<u8>]) -> u64 {
    let mut total_score = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, level) in line.iter().enumerate() {
            if *level == 0 {
                let ends = trail_ends((x,y), grid);
                let end_set = HashSet::<&(usize, usize)>::from_iter(ends.iter());

                total_score += end_set.len();
            }
        }
    }

    total_score as u64
}

fn trail_ends(point: (usize, usize), grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let level = grid[point.1][point.0];
    if level == 9 {
        let mut v = Vec::new();
        v.push(point);
        return v;
    }

    let x_max = grid[0].len() - 1;
    let y_max = grid.len() - 1;

    DIRECTIONS.iter().flat_map(|direction| {
        let next_x = point.0.checked_add_signed(direction.0)?;
        let next_y = point.1.checked_add_signed(direction.1)?;

        if next_x > x_max || next_y > y_max || grid[next_y][next_x] != level + 1{
            None
        } else {
            Some(trail_ends((next_x, next_y), grid))
        }
    }).fold(Vec::new(), |mut acc, mut i| {acc.append(&mut i); acc})
}

fn sum_trailhead_scores_part2(grid: &[Vec<u8>]) -> u64 {
    let mut total_score = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, level) in line.iter().enumerate() {
            if *level == 0 {
                total_score += unique_trail_count((x,y), grid);
            }
        }
    }

    total_score as u64
}

fn unique_trail_count(point: (usize, usize), grid: &[Vec<u8>]) -> u64 {
    let level = grid[point.1][point.0];
    if level == 9 {
        return 1;
    }

    let x_max = grid[0].len() - 1;
    let y_max = grid.len() - 1;

    DIRECTIONS.iter().flat_map(|direction| {
        let next_x = point.0.checked_add_signed(direction.0)?;
        let next_y = point.1.checked_add_signed(direction.1)?;

        if next_x > x_max || next_y > y_max || grid[next_y][next_x] != level + 1{
            None
        } else {
            Some(unique_trail_count((next_x, next_y), grid))
        }
    }).reduce(|acc, i| acc + i).unwrap_or(0)
}


#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    let grid = parse_input(input);
    sum_trailhead_scores(&grid)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let grid = parse_input(input);
    sum_trailhead_scores_part2(&grid)
}
