use std::collections::VecDeque;

use anyhow::Result;
use aoc_runner_derive::aoc;
use itertools::Itertools;

fn move_robot(robot: &(usize, usize), grid: &mut Vec<Vec<char>>, instruction: (isize, isize))-> (usize, usize) {
    let mut next_pos = (robot.0.checked_add_signed(instruction.0).unwrap(), robot.1.checked_add_signed(instruction.1).unwrap());
    let mut boxes_to_move = Vec::new();
    while grid[next_pos.1][next_pos.0] == 'O' {
        boxes_to_move.push(next_pos);
        next_pos = (next_pos.0.checked_add_signed(instruction.0).unwrap(), next_pos.1.checked_add_signed(instruction.1).unwrap());
    }

    if grid[next_pos.1][next_pos.0] == '.' {
        for pos in boxes_to_move.into_iter().rev() {
            let next_pos = (pos.0.checked_add_signed(instruction.0).unwrap(), pos.1.checked_add_signed(instruction.1).unwrap());
            grid[next_pos.1][next_pos.0] = 'O';
            grid[pos.1][pos.0] = '.';
        }

        let next_robot_pos = (robot.0.checked_add_signed(instruction.0).unwrap(), robot.1.checked_add_signed(instruction.1).unwrap());
        grid[robot.1][robot.0] = '.';
        grid[next_robot_pos.1][next_robot_pos.0] = '@';

        return next_robot_pos;
    }

    robot.clone()
}

fn parse_instructions(input: &str) -> Vec<(isize, isize)> {
    input.chars().map(|c| {
        match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        }
    }).collect()
}

fn find_robot(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '@' {
                return (x, y);
            }
        }
    }

    (0,0)
}

fn gps_sum(grid: &Vec<Vec<char>>) -> u64 {
    grid.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, c)| {
            if *c == 'O' {
                100 * y + x
            } else {
                0
            }
        }).sum::<usize>()
    }).sum::<usize>() as u64
}

fn wide_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().flat_map(|c| {
        match c {
            '#' => ['#', '#'],
            '@' => ['@', '.'],
            'O' => ['[', ']'],
            '.' => ['.', '.'],
            _ => ['?', '?'],
        }
    }).collect()).collect()
}

fn move_robot_p2(robot: &(usize, usize), grid: &mut Vec<Vec<char>>, instruction: (isize, isize))-> (usize, usize) {
    let mut next_positions = VecDeque::from([
        ((robot.0 as isize + instruction.0) as usize, (robot.1 as isize + instruction.1) as usize),
    ]);
    let mut boxes_to_move = Vec::new();

    while let Some(next_pos) = next_positions.pop_front() {
        let c = grid[next_pos.1][next_pos.0];
        if c == '[' {
            println!("opening box: {:?}", next_pos);
            let other_box_pos = (next_pos.0 + 1, next_pos.1);
            boxes_to_move.push((next_pos, other_box_pos));
            if instruction.1 != 0 {
                next_positions.push_back(((next_pos.0 as isize + instruction.0) as usize, (next_pos.1 as isize + instruction.1) as usize));
                next_positions.push_back(((other_box_pos.0 as isize + instruction.0) as usize, (other_box_pos.1 as isize + instruction.1) as usize));
            } else if instruction.0 == 1 {
                next_positions.push_back(((other_box_pos.0 as isize + instruction.0) as usize, (other_box_pos.1 as isize + instruction.1) as usize));
            } else {
                next_positions.push_back(((next_pos.0 as isize + instruction.0) as usize, (next_pos.1 as isize + instruction.1) as usize));
            }
        } else if c == ']' {
            println!("closing box: {:?}", next_pos);
            let other_box_pos = ((next_pos.0 as isize - 1) as usize, next_pos.1);
            boxes_to_move.push((other_box_pos, next_pos));
            if instruction.1 != 0 {
                next_positions.push_back(((next_pos.0 as isize + instruction.0) as usize, (next_pos.1 as isize + instruction.1) as usize));
                next_positions.push_back(((other_box_pos.0 as isize + instruction.0) as usize, (other_box_pos.1 as isize + instruction.1) as usize));
            } else if instruction.0 == 1 {
                next_positions.push_back(((next_pos.0 as isize + instruction.0) as usize, (next_pos.1 as isize + instruction.1) as usize));
            } else {
                next_positions.push_back(((other_box_pos.0 as isize + instruction.0) as usize, (other_box_pos.1 as isize + instruction.1) as usize));
            }
        } else if c == '#' {
            return robot.clone();
        }

        println!("pushed: {:?}", boxes_to_move.iter().last())
    }

    for b in boxes_to_move.iter().rev() {
        grid[(b.0.1 as isize + instruction.1) as usize][(b.0.0 as isize + instruction.0) as usize] = '[';
        grid[(b.1.1 as isize + instruction.1) as usize][(b.1.0 as isize + instruction.0) as usize] = ']';
        if instruction.1 != 0 {
            grid[b.0.1][b.0.0] = '.';
            grid[b.1.1][b.1.0] = '.';
        } else if instruction.0 == 1 {
            grid[b.0.1][b.0.0] = '.';
        } else {
            grid[b.1.1][b.1.0] = '.';
        }
    }

    let next_robot_pos = (robot.0.checked_add_signed(instruction.0).unwrap(), robot.1.checked_add_signed(instruction.1).unwrap());
    grid[robot.1][robot.0] = '.';
    grid[next_robot_pos.1][next_robot_pos.0] = '@';
    next_robot_pos
}

pub fn display(grid: &Vec<Vec<char>>) {
    for line in grid.iter() {
        println!("{}", String::from_iter(line.iter()))
    }
}

fn wide_gps_sum(grid: &Vec<Vec<char>>) -> u64 {
    grid.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, c)| {
            if *c == '[' {
                100 * y + x
            } else {
                0
            }
        }).sum::<usize>()
    }).sum::<usize>() as u64
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u64 {
    let (grid_input, instruction_input) = input.split("\n\n").next_tuple().unwrap();
    let mut grid: Vec<Vec<_>> = grid_input.lines().map(|line| line.chars().collect()).collect();
    let instructions = parse_instructions(instruction_input);
    let mut robot = find_robot(&grid);

    for instruction in instructions {
        robot = move_robot(&robot, &mut grid, instruction);
        display(&grid);
    }

    gps_sum(&grid)
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u64 {
    let (grid_input, instruction_input) = input.split("\n\n").next_tuple().unwrap();
    let mut grid: Vec<Vec<_>> = wide_grid(grid_input);
    let instructions = parse_instructions(instruction_input);
    let mut robot = find_robot(&grid);

    for instruction in instructions {
        println!("move {:?}", instruction);
        robot = move_robot_p2(&robot, &mut grid, instruction);
        display(&grid);
        println!("\n\n");
    }

    wide_gps_sum(&grid)
}
