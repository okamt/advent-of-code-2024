use std::{collections::HashMap, hash::BuildHasherDefault};

use aoc_runner_derive::aoc;
use nohash_hasher::NoHashHasher;

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    const CAPACITY: usize = 1000;
    let mut lists = (Vec::with_capacity(CAPACITY), Vec::with_capacity(CAPACITY));
    for line in input.lines() {
        let mut nums = line
            .split("   ")
            .map(|s| unsafe { s.parse().unwrap_unchecked() });
        lists.0.push(unsafe { nums.next().unwrap_unchecked() });
        lists.1.push(unsafe { nums.next().unwrap_unchecked() });
    }
    lists
}

// Solution: 2086478
// Best: ~80 us
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
// Best: ~80 us
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
