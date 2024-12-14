use std::{collections::VecDeque, ops::Range};

use anyhow::{Context, Result};
use aoc_runner_derive::aoc;

#[derive(Debug, Eq, Hash, PartialEq)]
struct FileSegment {
    id: u32,
    span: Range<usize>,
}

fn parse_input(input: &str) -> Result<VecDeque<FileSegment>> {
    let mut current_id = 0;
    let mut current_index = 0;
    let mut is_file = true;

    let mut files = VecDeque::new();

    for c in input.trim().chars() {
        let size = c.to_digit(10).with_context(|| format!("parse error on {}", c))? as usize;
        if is_file {
            files.push_back(FileSegment {
                id: current_id,
                span: (current_index..current_index+size),
            });
            current_id += 1;
            current_index += size;
        } else {
            current_index += size;
        };

        is_file = !is_file;
    }

    Ok(files)
}

fn compact(files: &mut VecDeque<FileSegment>) -> () {
   while let Some((index, span)) = next_empty_space(files) {
        let file = files.pop_back().unwrap();
        let (filled, remainder) = move_file(file, span);
        files.insert(index + 1, filled);
        if let Some(remainder) = remainder { files.push_back(remainder); }
    }
}

fn next_empty_space(files: &VecDeque<FileSegment>) -> Option<(usize, Range<usize>)> {
    files.iter().take(files.len() - 1).enumerate().map(|(i, file)| {
        (i, (file.span.end..files[i+1].span.start))
    }).find(|(_, span)| span.len() > 0)
}

fn move_file(file: FileSegment, span: Range<usize>) -> (FileSegment, Option<FileSegment>) {
    let mut remainder = None;
    let diff = span.len() as i32 - file.span.len() as i32;
    let mut file = file;
    let id = file.id;
    let moved_span;

    if diff < 0 {
        file.span = file.span.start + span.len()..file.span.end;
        remainder = Some(file);
        moved_span = span.clone();
    } else if diff == 0 {
        moved_span = span.clone();
    } else {
        moved_span = span.start..(span.end - diff as usize);
    }

    (FileSegment { id, span: moved_span }, remainder)
}

fn checksum(files: &VecDeque<FileSegment>) -> u64 {
    files.iter().fold(0, |acc, file| {
        acc + file.span.clone().fold(0, |acc, i| {
            acc + (i as u64 * file.id as u64)
        })
    }) as u64
}

fn compact_whole(files: &mut VecDeque<FileSegment>) -> () {
    let file_ids = (0..files.len()).rev();
    for id in file_ids {
        let (i, file) = files.iter().enumerate().rev().find(|(_, file)| file.id == id as u32).unwrap();
        if let Some((insert_index, span)) = find_space(file, files) {
            let mut file = files.remove(i).unwrap();
            file.span = span;
            files.insert(insert_index, file);
        }
    }
}

fn find_space(file: &FileSegment, files: &VecDeque<FileSegment>) -> Option<(usize, Range<usize>)> {
    let (index, other) = files.iter().take_while(|other| other.span.start < file.span.start).enumerate().find(|(i, other)| {
        files[i+1].span.start - other.span.end >= file.span.len()
    })?;

    Some((index + 1, (other.span.end..other.span.end + file.span.len())))
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> u64 {
    let mut file_segments = parse_input(input).unwrap();
    compact(&mut file_segments);
    checksum(&file_segments)
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> u64 {
    let mut file_segments = parse_input(input).unwrap();
    compact_whole(&mut file_segments);
    checksum(&file_segments)
}
