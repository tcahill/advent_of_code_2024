use anyhow::Result;
use aoc_runner_derive::aoc;
use regex::Regex;

const DIRECTIONS: &[(i64, i64)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIAGONALS: &[&[(i64, i64)]] = &[
    &[(-1, -1), (1, 1)],
    &[(-1, 1), (1, -1)],
];

#[aoc(day4, part1)]
fn part1(input: &str) -> u64 {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut count = 0;

    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'X' {
                count += xmas_count_at_point((i, j), &grid);
            }
        }
    }

    count
}

fn xmas_count_at_point(point: (usize, usize), grid: &[&[u8]]) -> u64 {
    DIRECTIONS.iter().filter(|direction| { search(point, direction, grid) }).count() as u64
}

fn search(point: (usize, usize), direction: &(i64, i64), grid: &[&[u8]]) -> bool {
    for (i, c) in "MAS".as_bytes().iter().enumerate() {
        let offset = (direction.0 * (i + 1) as i64, direction.1 * (i + 1) as i64);
        let Some((x, y)) = add_to_point(point, offset, grid) else { return false };

        if grid[x as usize][y as usize] != *c {
            return false;
        }
    }

    true
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u64 {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut count = 0;

    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == b'A' {
                if cross_mas_at_point((i, j), &grid) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn cross_mas_at_point(point: (usize, usize), grid: &[&[u8]]) -> bool {
    let mut count = 0;
    for diagonal in DIAGONALS.iter() {
        let offset1 = (diagonal[0].0 as i64, diagonal[0].1 as i64);
        let offset2 = (diagonal[1].0 as i64, diagonal[1].1 as i64);

        let Some((x1, y1)) = add_to_point(point, offset1, grid) else { continue; };
        let Some((x2, y2)) = add_to_point(point, offset2, grid) else { continue; };

        let c1 = grid[x1][y1];
        let c2 = grid[x2][y2];

        if c1 == b'M' && c2 == b'S' || c1 == b'S' && c2 == b'M' {
            count += 1
        }
    }

    count == 2
}

fn add_to_point(point: (usize, usize), offset: (i64, i64), grid: &[&[u8]]) -> Option<(usize, usize)> {
    let x = point.0 as i64 + (offset.0);
    let y = point.1 as i64 + (offset.1);
    if x < 0 || y < 0 || x >= (grid.len() as i64) || y >= (grid[0].len() as i64) {
        return None;
    }

    return Some((x as usize, y as usize))
}
