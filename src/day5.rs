use anyhow::Result;
use aoc_runner_derive::aoc;

use std::{collections::{HashMap, HashSet}, mem};

fn valid_update_sum(rules: &mut HashMap<u64, Vec<u64>>, updates: &Vec<Vec<u64>>) -> u64 {
    let mut sum = 0;

    for update in updates.iter() {
        if valid_update(update, rules) {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn valid_update(update: &[u64], rules: &mut HashMap<u64, Vec<u64>>) -> bool {
    let mut pages = HashSet::<u64>::new();
    let mut possible_violations = HashSet::<u64>::new();

    for page in update {
        if possible_violations.contains(page) {
            return false;
        }

        let dependencies = rules.entry(*page).or_default();

        for dep in dependencies.iter() {
            if !pages.contains(dep) {
                possible_violations.insert(*dep);
            }
        }

        pages.insert(*page);
    }

    true
}

fn invalid_update_sum(rules: &mut HashMap<u64, Vec<u64>>, updates: &mut Vec<Vec<u64>>) -> u64 {
    let mut sum = 0;

    for update in updates.iter_mut() {
        if fix_invalid_update(update, rules) {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn fix_invalid_update(update: &mut Vec<u64>, rules: &mut HashMap<u64, Vec<u64>>) -> bool {
    let mut pages = HashSet::<u64>::new();
    let mut possible_violations = HashMap::<u64, HashSet<u64>>::new();
    let mut modified = false;

    let mut new_update = Vec::new();

    'outer: for page in update.iter() {
        let dependencies = rules.entry(*page).or_default();

        for dep in dependencies.iter() {
            if !pages.contains(dep) {
                possible_violations.entry(*dep).or_insert(HashSet::new()).insert(*page);
            }
        }

        pages.insert(*page);

        if possible_violations.contains_key(page) {
            let dependents = &possible_violations[page];
            for j in 0..new_update.len() {
                if dependents.contains(&new_update[j]) {
                    new_update.insert(j, *page);
                    modified = true;
                    continue 'outer;
                }
            }
        }

        new_update.push(*page);

    }

    let _ = mem::replace(update, new_update);
    modified
}

fn parse_input(input: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();


    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() { break; }
        let pages: Vec<_> = line.split('|').map(|x| x.parse::<u64>().unwrap()).collect();
        rules.entry(pages[1]).or_insert(Vec::new()).push(pages[0]);
    }

    for line in lines {
        let update: Vec<_> = line.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        updates.push(update);
    }

    (rules, updates)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u64 {
    let (mut rules, updates) = parse_input(input);
    valid_update_sum(&mut rules, &updates)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u64 {
    let (mut rules, mut updates) = parse_input(input);
    invalid_update_sum(&mut rules, &mut updates)
}
