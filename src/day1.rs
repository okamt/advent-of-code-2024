use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    simd::{num::SimdUint, u32x4, Simd},
};

use aoc_runner_derive::aoc;
use nohash_hasher::NoHashHasher;

pub const LINES: usize = 1000;
pub const DIGITS: usize = 5;
pub const SPACES: usize = 3;

pub unsafe fn parse_num(input: *const u8) -> u64 {
    let mut simd: Simd<u32, 4> = u32x4::from_array([
        input.read() as u32,
        input.add(1).read() as u32,
        input.add(2).read() as u32,
        input.add(3).read() as u32,
    ]);
    simd &= u32x4::splat(0b1111);
    simd *= Simd::from_array([10_000, 1_000, 100, 10]);
    simd.reduce_sum() as u64 + (input.add(4).read() & 0b1111) as u64
}

pub fn parse_input(input: &str) -> ([u64; LINES], [u64; LINES]) {
    let mut lists = ([0u64; LINES], [0u64; LINES]);
    let input = input.as_ptr();
    for i in 0..1000 {
        let offset = i * (DIGITS + SPACES + DIGITS + 1);
        unsafe {
            lists.0[i] = parse_num(input.add(offset));
            lists.1[i] = parse_num(input.add(offset + DIGITS + SPACES));
        }
    }
    lists
}

// Solution: 2086478
// Best: ~17 us
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
// Best: ~17 us
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
