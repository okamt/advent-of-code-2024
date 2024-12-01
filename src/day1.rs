use std::{collections::HashMap, hash::BuildHasherDefault};

use aoc_runner_derive::aoc;
use nohash_hasher::NoHashHasher;

use crate::utils;

pub const LINES: usize = 1000;
pub const DIGITS: usize = 5;
pub const SPACES: usize = 3;

pub fn parse_input(input: &str) -> ([u64; LINES], [u64; LINES]) {
    let mut lists = ([0u64; LINES], [0u64; LINES]);
    let input = input.as_bytes();
    for i in 0..1000 {
        let offset = i * (DIGITS + SPACES + DIGITS + 1);
        lists.0[i] = utils::parse_u64_fast::<DIGITS>(&input[offset..]);
        lists.1[i] = utils::parse_u64_fast::<DIGITS>(&input[offset + DIGITS + SPACES..]);
    }
    lists
}

// Solution: 2086478
// Best: ~18 us
#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let mut lists = parse_input(input);
    lists.0.sort_unstable();
    lists.1.sort_unstable();
    let mut sum = 0;
    for i in 0..lists.0.len() {
        sum += lists.0[i].abs_diff(lists.1[i]);
    }
    sum
}

// Solution: 24941624
// Best: ~18 us
#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let lists = parse_input(input);

    let mut frequencies: HashMap<_, _, BuildHasherDefault<NoHashHasher<u64>>> =
        HashMap::with_capacity_and_hasher(1000, BuildHasherDefault::default());
    lists
        .1
        .into_iter()
        .for_each(|item| *frequencies.entry(item).or_default() += 1);

    let mut sum = 0;
    for left in lists.0.into_iter() {
        sum += left * frequencies.get(&left).copied().unwrap_or(0) as u64;
    }
    sum
}
