use std::collections::HashSet;

use anyhow::{Context, Result};
use aoc_runner_derive::aoc;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

const X_SIZE: isize = 101;
const Y_SIZE: isize = 103;

fn advance(robots: &mut Vec<Robot>) {
    for robot in robots.iter_mut() {
        robot.position = (
            ((robot.position.0 as isize + robot.velocity.0).rem_euclid(X_SIZE)) as usize,
            ((robot.position.1 as isize + robot.velocity.1).rem_euclid(Y_SIZE)) as usize,
        )
    }
}

fn safety_factor(robots: &Vec<Robot>) -> u64 {
    let mut quadrant_scores = vec![0,0,0,0];

    let x_middle = (X_SIZE as usize) / 2;
    let y_middle = (Y_SIZE as usize) / 2;

    for robot in robots {
        if robot.position.0 < x_middle {
            if robot.position.1 < y_middle {
                quadrant_scores[0] += 1;
            } else if robot.position.1 > y_middle {
                quadrant_scores[1] += 1;
            }
        } else if robot.position.0 > x_middle {
            if robot.position.1 < y_middle {
                quadrant_scores[2] += 1;
            } else if robot.position.1 > y_middle {
                quadrant_scores[3] += 1;
            }
        }
    }

    quadrant_scores.into_iter().reduce(|acc, score| acc * score).unwrap()
}


fn contains_triangle(robots: &Vec<Robot>, triangle_size: u64) -> bool {
    let next_directions = Vec::<(isize, isize)>::from([(-1, 1), (0, 1), (1,1)]);
    let robot_locations = HashSet::<(usize, usize)>::from_iter(robots.iter().map(|robot| robot.position));

    for robot in robots.iter() {
        let mut stack = Vec::new();
        stack.push(robot.position);
        let mut found_size = 1;
        while let Some(r) = stack.pop() {
            let next_robots: Vec<(usize, usize)> = next_directions.iter().flat_map(|direction| {
                robot_locations.get(&((r.0 as isize + direction.0) as usize, (r.1 as isize + direction.1) as usize))
            }).map(|r| r.clone()).collect();

            if next_robots.len() == 3 {
                stack.extend_from_slice(&next_robots);
                found_size += 1;
                if found_size == triangle_size {
                    return true;
                }
            }

        }
    }

    false
}

fn display(robots: &Vec<Robot>) {
    let line = Vec::from_iter((0..X_SIZE).map(|_| '.'));
    let mut grid = Vec::from_iter((0..Y_SIZE).map(|_| line.clone()));

    for robot in robots.iter() {
        grid[robot.position.1][robot.position.0] = '#';
    }

    for line in grid.iter() {
        println!("{}", String::from_iter(line.iter()));
    }
}

fn steps_to_christmas_tree(robots: &mut Vec<Robot>) -> u64 {
    let mut steps = 1;
    loop {
        advance(robots);
        if contains_triangle(robots, 5) {
            return steps;
        }
        steps += 1;
    }
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=([\-0-9]+),([\-0-9]+)").unwrap();
    input.lines().map(|line| {
        let captures = re.captures(line).context("parse error")?;
        Ok(
            Robot{
                position: (captures[1].parse()?, captures[2].parse()?),
                velocity: (captures[3].parse()?, captures[4].parse()?),
            },
        )
    }).collect()
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u64 {
    let mut robots = parse_input(input).unwrap();
    for _ in 0..100 { advance(&mut robots); }
    safety_factor(&robots)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> u64 {
    let mut robots = parse_input(input).unwrap();
    let steps = steps_to_christmas_tree(&mut robots);
    display(&robots);
    steps
}
