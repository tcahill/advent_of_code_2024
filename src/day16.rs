use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};


const DIRECTIONS: &[(isize, isize)] = &[
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

#[derive(Debug)]
struct Maze {
    start: (usize, usize),
    end: (usize, usize),
    walls: HashSet<(usize, usize)>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    score: usize,
    position: (usize, usize),
    orientation: (isize, isize)
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

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut start = (0,0);
        let mut end = (0,0);
        let mut walls = HashSet::new();

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = (x,y);
                    },
                    'E' => {
                        end = (x,y);
                    },
                    '#' => {
                        walls.insert((x,y));
                    },
                    _ => {}
                }
            }
        }

        Self { start, end, walls }
    }
}

fn a_star(maze: &Maze) -> u64 {
    let mut lowest_score = HashMap::new();
    lowest_score.insert(maze.start, 0);

    let start_score = manhattan_distance(maze.start, maze.end);

    let mut heuristic_score = HashMap::new();
    heuristic_score.insert(maze.start, manhattan_distance(maze.start, maze.end));

    let mut queue = BinaryHeap::new();
    queue.push(State { score: start_score, position: maze.start, orientation: (1,0) });

    let mut prev_pos: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    while let Some(state) = queue.pop() {
        let pos = state.position;
        if state.position == maze.end {
            break;
        }

        for direction in DIRECTIONS.iter() {
            if *direction == (state.orientation.0 * -1, state.orientation.1 * -1) {
                continue;
            }

            let neighbor = (
                (pos.0 as isize + direction.0) as usize,
                (pos.1 as isize + direction.1) as usize,
            );

            if !maze.walls.contains(&neighbor) {
                let score = lowest_score.get(&pos).unwrap() + distance(pos, neighbor, state.orientation);
                if lowest_score.get(&neighbor).is_none() || *lowest_score.get(&neighbor).unwrap() > score {
                    let entry = prev_pos.entry(neighbor).or_insert((0,0));
                    *entry = pos;

                    let s = lowest_score.entry(neighbor).or_insert(0);
                    *s = score;

                    let estimated = score + manhattan_distance(maze.end, neighbor);
                    let u = heuristic_score.entry(neighbor).or_insert(0);
                    *u = estimated;

                    queue.push(State { score: estimated, position: neighbor, orientation: *direction });
                }
            }
        }
    }

    score(maze.end, &prev_pos)
}

fn score(end: (usize, usize), prev_pos: &HashMap<(usize, usize), (usize, usize)>) -> u64 {
    let mut current = end;
    let prev = prev_pos.get(&end).unwrap();
    let mut score = 0;
    let mut current_orientation = (prev.0 as isize - current.0 as isize, prev.1 as isize - current.1 as isize);
    while let Some(prev) = prev_pos.get(&current) {
        score += distance(current, *prev, current_orientation);
        current_orientation = (prev.0 as isize - current.0 as isize, prev.1 as isize - current.1 as isize);

        current = *prev;
    };

    if current_orientation != (-1,0) {
        score += 1000;
    }

    score as u64
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn distance(a: (usize, usize), b: (usize, usize), orientation: (isize, isize)) -> usize {
    let offset = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
    let angle = (offset.0 - orientation.0, offset.1 - orientation.1);
    if angle == (0,0) {
        1
    } else {
        1001
    }
}

fn all_lowest_score_paths(maze: &Maze) -> u64 {
    let mut lowest_score = HashMap::new();
    for direction in DIRECTIONS.iter() {
        lowest_score.insert((maze.start, *direction), 0);
    }

    let mut queue = BinaryHeap::new();
    queue.push(State { score: 0, position: maze.start, orientation: (1, 0) });

    let mut prev_positions: HashMap<State, Vec<State>> = HashMap::new();
    while let Some(state) = queue.pop() {
        let pos = state.position;
        if pos == maze.end {
            continue;
        }

        for direction in DIRECTIONS.iter() {
            if *direction == (state.orientation.0 * -1, state.orientation.1 * -1) {
                continue;
            }

            let neighbor = (
                (pos.0 as isize + direction.0) as usize,
                (pos.1 as isize + direction.1) as usize,
            );

            if !maze.walls.contains(&neighbor) {
                let key = (neighbor, *direction);
                let score = lowest_score.get(&(pos, state.orientation)).unwrap() + distance(pos, neighbor, state.orientation);

                let next_state = State { score, position: neighbor, orientation: *direction };
                if lowest_score.get(&key).is_none() || *lowest_score.get(&key).unwrap() > score {
                    let entry = prev_positions.entry(next_state).or_insert(Vec::new());
                    entry.clear();
                    entry.push(state.clone());

                    let s = lowest_score.entry(key).or_insert(0);
                    *s = score;

                    queue.push(next_state);
                } else if *lowest_score.get(&key).unwrap() == score {
                    let entry = prev_positions.entry(next_state).or_insert(Vec::new());
                    entry.push(state);
                }
            }
        }
    }

    count_positions_on_shortest_paths(maze.end, &prev_positions)
}

fn count_positions_on_shortest_paths(end: (usize, usize), prev_positions: &HashMap<State, Vec<State>>) -> u64 {
    let min_score = prev_positions.iter().filter_map(|prev| (prev.0.position == end).then(|| prev.0.score)).min().unwrap();

    let mut queue = VecDeque::from_iter(prev_positions.iter().filter_map(|prev| (prev.0.position == end && prev.0.score == min_score).then(|| prev.0)));
    let mut positions = HashSet::new();
    while let Some(current) = queue.pop_back() {
        if let Some(prev) = prev_positions.get(&current) {
            for prev_state in prev.iter() {
                positions.insert(prev_state.position);
                queue.push_front(prev_state);
            }
        }
    }

    (positions.len() + 1) as u64
}

pub fn part1(input: &str) -> u64 {
    let maze: Maze = input.into();
    a_star(&maze)
}

pub fn part2(input: &str) -> u64 {
    let maze: Maze = input.into();
    all_lowest_score_paths(&maze)
}
