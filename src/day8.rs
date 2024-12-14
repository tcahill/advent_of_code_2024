use std::collections::{HashMap, HashSet};

use anyhow::Result;
use aoc_runner_derive::aoc;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct Grid {
    antennae: HashMap<char, Vec<Position>>,
    x_max: u8,
    y_max: u8,
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut antennae = HashMap::new();
        let mut x_max = 0;
        let mut y_max = 0;

        for (y, line) in value.lines().enumerate() {
            if x_max == 0 {
                x_max = (line.len() - 1) as u8;
            }
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    antennae.entry(c).or_insert(Vec::new()).push(Position { x: x.try_into()?, y: y.try_into()? })
                }
            }
            y_max += 1;
        }

        y_max -= 1;

        Ok(Grid { antennae, x_max, y_max })
    }
}

fn count_antinodes(grid: &Grid) -> u64 {
    HashSet::<Position>::from_iter(grid.antennae.values().flat_map(|positions| {
        positions.iter().enumerate().flat_map(|(i, pos)| {
            positions.iter().enumerate().filter(move |(j, _)| *j != i).flat_map(|(_, other)| {
                let antinode_x = pos.x.checked_add_signed(pos.x as i8 - other.x as i8)?;
                let antinode_y = pos.y.checked_add_signed(pos.y as i8 - other.y as i8)?;

                if antinode_x > grid.x_max || antinode_y > grid.y_max {
                    return None;
                }

                Some(Position {
                    x: antinode_x,
                    y: antinode_y,
                })
            })
        })
    })).len() as u64
}

fn count_antinodes_with_harmonics(grid: &Grid) -> u64 {
    HashSet::<Position>::from_iter(grid.antennae.values().flat_map(|positions| {
        positions.iter().enumerate().flat_map(|(i, pos)| {
            positions.iter().enumerate().filter(move |(j, _)| *j != i).flat_map(|(_, other)| {
                let mut diff_x = pos.x as i8 - other.x as i8;
                let mut diff_y = pos.y as i8 - other.y as i8;
                let gcd = num::integer::gcd(diff_x, diff_y);
                diff_x = diff_x / gcd;
                diff_y = diff_y / gcd;

                let mut antinode_x = pos.x as i8;
                let mut antinode_y = pos.y as i8;

                let mut antinodes = Vec::new();
                while antinode_x >= 0 && antinode_y >= 0 && antinode_x <= grid.x_max as i8 && antinode_y <= grid.y_max as i8 {
                    antinodes.push(Position{x: antinode_x as u8, y: antinode_y as u8 });
                    antinode_x += diff_x;
                    antinode_y += diff_y;
                }

                antinodes
            })
        })
    })).len() as u64
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    let grid: Grid = input.try_into().unwrap();
    count_antinodes(&grid)
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u64 {
    let grid: Grid = input.try_into().unwrap();
    count_antinodes_with_harmonics(&grid)
}
