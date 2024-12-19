use std::collections::HashSet;
use anyhow::Result;
use itertools::Itertools;

use std::collections::{BinaryHeap, HashMap, VecDeque};


const DIRECTIONS: &[(isize, isize)] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

const X_MAX: usize = 70;
const Y_MAX: usize = 70;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    score: usize,
    position: (usize, usize),
}

impl Ord for State{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.score.partial_cmp(&self.score)
    }
}

fn a_star(corrupted: &HashSet<(usize, usize)>) -> Option<u64> {
    let start = (0, 0);
    let end = (X_MAX, Y_MAX);

    let mut lowest_score = HashMap::new();
    lowest_score.insert(start, 0);

    let start_score = manhattan_distance(start, end);

    let mut heuristic_score = HashMap::new();
    heuristic_score.insert(start, manhattan_distance(start, end));

    let mut queue = BinaryHeap::new();
    queue.push(State { score: start_score, position: start });

    let mut prev_pos: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    while let Some(state) = queue.pop() {
        let pos = state.position;
        if state.position == end {
            break;
        }

        for direction in DIRECTIONS.iter() {
            let maybe_neighbor = (
                pos.0.checked_add_signed(direction.0),
                pos.1.checked_add_signed(direction.1),
            );

            if maybe_neighbor.0.is_none() || maybe_neighbor.1.is_none() {
                continue;
            }

            let neighbor = (maybe_neighbor.0.unwrap(), maybe_neighbor.1.unwrap());

            if neighbor.0 > X_MAX || neighbor.1 > Y_MAX {
                continue;
            }

            if !corrupted.contains(&neighbor) {
                let score = lowest_score.get(&pos).unwrap() + 1;
                if lowest_score.get(&neighbor).is_none() || *lowest_score.get(&neighbor).unwrap() > score {
                    let entry = prev_pos.entry(neighbor).or_insert((0,0));
                    *entry = pos;

                    let s = lowest_score.entry(neighbor).or_insert(0);
                    *s = score;

                    let estimated = score + manhattan_distance(end, neighbor);
                    let u = heuristic_score.entry(neighbor).or_insert(0);
                    *u = estimated;

                    queue.push(State { score: estimated, position: neighbor });
                }
            }
        }
    }

    score(&prev_pos)
}

fn score(prev_pos: &HashMap<(usize, usize), (usize, usize)>) -> Option<u64> {
    let mut current = (X_MAX, Y_MAX);
    let mut score = 0;
    while let Some(prev) = prev_pos.get(&current) {
        score += 1;
        current = *prev;
    };

    if current != (0, 0) { return None ; }

    Some( score as u64)
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    HashSet::from_iter(
        input.lines().take(1024)
            .map(|line| {
                line.split(',').map(|num| num.parse().unwrap()).next_tuple().unwrap()
            })
    )
}

fn input_iter<'a>(input: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    input.lines().map(|line| {
        line.split(',').map(|num| num.parse::<usize>().unwrap()).next_tuple::<(usize, usize)>().unwrap()
    })
}

pub fn part1(input: &str) -> u64 {
    let corrupted_locations = parse_input(input);
    a_star(&corrupted_locations).unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut corrupted = HashSet::new();
    for pos in input_iter(input) {
        corrupted.insert(pos);
        if a_star(&corrupted).is_none() {
            println!("{:?}", pos);
            break;
        }
    }
    0
}
