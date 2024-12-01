use std::{collections::HashMap, hash::BuildHasherDefault};

use aoc_runner_derive::aoc;
use bytes::Buf;
use nohash_hasher::NoHashHasher;

use crate::utils;

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    const CAPACITY: usize = 1000;
    let mut lists = (Vec::with_capacity(CAPACITY), Vec::with_capacity(CAPACITY));
    let mut input = input.as_bytes();
    let mut is_right = false;
    while input.has_remaining() {
        let byte = input[0];
        if byte.is_ascii_digit() {
            let list = if !is_right {
                &mut lists.0
            } else {
                &mut lists.1
            };
            list.push(utils::parse_u64_fast(&mut input));
            is_right = !is_right;
        } else {
            input.advance(1);
        }
    }
    lists
}

// Solution: 2086478
// Best: ~24 us
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
// Best: ~24 us
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
