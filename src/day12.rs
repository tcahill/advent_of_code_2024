use std::collections::HashSet;

use anyhow::Result;
use aoc_runner_derive::aoc;

const DIRECTIONS: &[(isize, isize)] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

#[derive(Debug)]
struct GardenPlot {
    perimeter: HashSet::<((isize, isize), (isize, isize))>,
    area: u64,
}

fn calculate_garden_plots(grid: &Vec<Vec<char>>) -> Vec<GardenPlot> {
    let mut garden_plots = Vec::new();
    let mut accounted_spaces = HashSet::new();
    let x_max = grid[0].len() - 1;
    let y_max = grid.len() - 1;

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if accounted_spaces.contains(&(x as isize, y as isize)) {
                continue;
            }

            accounted_spaces.insert((x as isize,y as isize));
            let mut stack = Vec::from_iter(DIRECTIONS.iter().map(|direction| {
                (((x as isize) + direction.0, (y as isize) + direction.1), direction)
            }));
            let mut visited = HashSet::<(isize, isize)>::from([(x as isize, y as isize)]);

            let mut perimeter = HashSet::new();
            let mut area = 1;

            while let Some((position, direction)) = stack.pop() {
                if position.0 < 0 || position.1 < 0 || position.0 > x_max as isize || position.1 > y_max as isize || grid[position.1 as usize][position.0 as usize] != *c {
                    perimeter.insert(((position.0 - direction.0, position.1 - direction.1), *direction));
                } else {
                    if visited.contains(&position) {
                        continue;
                    }

                    accounted_spaces.insert(position);
                    area += 1;

                    for direction in DIRECTIONS.iter() {
                        let neighbor = (position.0 + direction.0, position.1 + direction.1);
                        stack.push((neighbor, direction));
                    }
                }

                visited.insert(position);
            }

            garden_plots.push(GardenPlot { area, perimeter });
        }
    }


    garden_plots
}

fn num_sides(garden_plot: &GardenPlot) -> u64 {
    let mut visited = HashSet::<((isize, isize), (isize, isize))>::new();
    let mut sides = 0;

    for (position, direction) in garden_plot.perimeter.iter() {
        if visited.contains(&(*position, *direction)) {
            continue;
        }

        sides += 1;
        visited.insert((*position, *direction));

        let mut stack = Vec::new();
        stack.push((position.0 + direction.1, position.1 + direction.0));
        stack.push((position.0 - direction.1, position.1 - direction.0));

        while let Some(other) = stack.pop() {
            if !visited.contains(&(other, *direction)) && garden_plot.perimeter.contains(&(other, *direction)) {
                visited.insert((other, *direction));
                stack.push((other.0 + direction.1, other.1 + direction.0));
                stack.push((other.0 - direction.1, other.1 - direction.0));
            }
        }
    }

    sides
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
    let grid: Vec<_> = input.lines().map(|line| line.chars().collect()).collect();
    let garden_plots = calculate_garden_plots(&grid);
    garden_plots.iter().map(|plot| plot.perimeter.len() as u64 * plot.area).sum::<u64>()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
    let grid: Vec<_> = input.lines().map(|line| line.chars().collect()).collect();
    let garden_plots = calculate_garden_plots(&grid);
    garden_plots.iter().map(|plot| num_sides(plot) * plot.area).sum::<u64>()
}
