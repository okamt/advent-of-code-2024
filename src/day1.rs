use std::simd::{num::SimdUint, u32x4, u8x4, Simd};

use aoc_runner_derive::aoc;

pub const LINES: usize = 1000;
pub const DIGITS: usize = 5;
pub const SPACES: usize = 3;

#[inline(always)]
pub unsafe fn parse_num(input: *const u8) -> u32 {
    let mut simd: Simd<u32, 4> =
        u8x4::from_slice(unsafe { std::slice::from_raw_parts(input, 4) }).cast();
    simd &= u32x4::splat(0b1111);
    simd *= Simd::from_array([10_000, 1_000, 100, 10]);
    simd.reduce_sum() + (input.add(4).read() & 0b1111) as u32
}

#[inline(always)]
pub fn parse_input(input: &str) -> ([u32; LINES], [u32; LINES]) {
    let mut lists = ([0; LINES], [0; LINES]);
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
// Best: ~16 us
#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
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
// Best: ~5 us
#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let lists = parse_input(input);
    let mut frequencies = [0u8; 90_000];
    for right in lists.1.into_iter() {
        frequencies[(right - 10_000) as usize] += 1;
    }
    let mut sum = 0;
    for left in lists.0.into_iter() {
        sum += left * frequencies[(left - 10_000) as usize] as u32;
    }
    sum
}
