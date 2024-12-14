use aoc_runner_derive::aoc;
use std::{collections::HashSet, ops::Add};
use thiserror::Error;

#[derive(Debug, Error)]
enum GridError {
    #[error("cycle detected")]
    CycleDetected,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: u8,
    y: u8,
}

impl <'a> Add<&'a Direction> for &'a Position {
    type Output = Option<Position>;

    fn add(self, direction: &'a Direction) -> Option<Position> {
        Some(Position {
            x: self.x.checked_add_signed(direction.x)?,
            y: self.y.checked_add_signed(direction.y)?,
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Direction {
    x: i8,
    y: i8,
}

impl Direction {
    fn rotate_clockwise_90(&mut self) {
        let new_x = self.y * -1;
        let old_x = self.x;

        self.y = old_x;
        self.x = new_x;
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Guard  {
    position: Position,
    direction: Direction,
}

#[derive(Clone)]
struct Grid {
    guard: Guard,
    obstacles: HashSet<Position>,
    visited: HashSet<Guard>,
    x_max: u8,
    y_max: u8,
    cycle_detected: bool,
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        let mut obstacles = HashSet::new();
        let visited = HashSet::new();
        let mut guard = Guard {
            position: Position {x: 0, y: 0 },
            direction: Direction { x: 0, y: -1 },
        };
        let mut x_max = 0;
        let mut y_max = 0;

        for (y, line) in value.lines().enumerate() {
            if x_max == 0 {
                x_max = (line.len() - 1).try_into()?;
            }

            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    obstacles.insert(Position{ x: x.try_into()?, y: y.try_into()? });
                } else if c == '^' {
                    guard.position = Position{ x: x.try_into()?, y: y.try_into()? };
                }
            }

            y_max += 1;
        }

        y_max -= 1;

        Ok(Self { obstacles, guard, visited, x_max, y_max, cycle_detected: false })
    }
}

impl Iterator for Grid {
    type Item = Result<Guard, GridError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycle_detected {
            return None;
        }

        let next_position = (&self.guard.position + &self.guard.direction)?;
        if next_position.x > self.x_max || next_position.y >  self.y_max {
            return None;
        }

        if self.obstacles.contains(&next_position) {
            self.guard.direction.rotate_clockwise_90();
        } else {
            self.guard.position  = next_position;
        }

        if self.visited.contains(&self.guard) {
            self.cycle_detected = true;
            return Some(Err(GridError::CycleDetected));
        }

        self.visited.insert(self.guard.clone());

        Some(Ok(self.guard.clone()))
    }
}

fn count_positions(grid: &mut Grid) -> u64 {
    let starting_position = grid.guard.position.clone();
    let mut positions = HashSet::<Position>::from_iter(grid.flat_map(|guard| Ok::<_, GridError>(guard?.position)));
    positions.insert(starting_position);
    positions.len() as u64
}

fn count_cycles(grid: &mut Grid) -> u64 {
    let initial_grid = grid.clone();

    let positions = HashSet::<Position>::from_iter(
        grid.flat_map(|guard| Ok::<_, GridError>(guard?.position))
            .filter(|pos| {
                let mut grid = initial_grid.clone();
                grid.obstacles.insert(pos.clone());
                grid.last().unwrap().is_err()
            })
    );

    positions.len() as u64
}

#[aoc(day6, part1)]
pub fn run(input: &str) -> u64 {
    let mut grid: Grid = input.try_into().unwrap();
    count_positions(&mut grid)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    let mut grid: Grid = input.try_into().unwrap();
    count_cycles(&mut grid)
}
