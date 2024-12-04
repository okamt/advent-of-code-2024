use core::str;
use std::{collections::VecDeque, hint::unreachable_unchecked};

use aoc_runner_derive::aoc;

pub type Sum = u32;

// Solution: 178886550
// Best: 11 us
#[aoc(day3, part1)]
fn part1(input: &str) -> Sum {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> Sum {
        let input = input.as_bytes();
        let mut digit_index;
        let mut first;
        let mut digit_buffer: [Sum; 3] = [0, 0, 0];
        let mut sum = 0;
        for mut index in memchr::memmem::find_iter(input, "mul(") {
            digit_index = 0;
            first = 0;
            index += 4;
            let mut r#char;
            loop {
                r#char = input[index];
                match r#char {
                    b')' => {
                        let second = match digit_index {
                            1 => digit_buffer[0],
                            2 => digit_buffer[0] * 10 + digit_buffer[1],
                            3 => digit_buffer[0] * 100 + digit_buffer[1] * 10 + digit_buffer[2],
                            _ => unsafe { unreachable_unchecked() },
                        };
                        sum += (first as Sum) * (second as Sum);
                        break;
                    }
                    b',' => {
                        first = match digit_index {
                            1 => digit_buffer[0],
                            2 => digit_buffer[0] * 10 + digit_buffer[1],
                            3 => digit_buffer[0] * 100 + digit_buffer[1] * 10 + digit_buffer[2],
                            _ => unsafe { unreachable_unchecked() },
                        };
                        digit_index = 0;
                        index += 1;
                    }
                    digit => {
                        if digit_index > 2 {
                            break;
                        }
                        digit_buffer[digit_index] = (digit & 0b1111) as Sum;
                        digit_index += 1;
                        index += 1;
                    }
                }
            }
        }

        sum
    }

    unsafe { inner(input) }
}

// Solution: 87163705
// Best: 13 us
#[aoc(day3, part2)]
fn part2(input: &str) -> Sum {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> Sum {
        let input = input.as_bytes();

        let mut dos = VecDeque::with_capacity(100);
        let mut donts = VecDeque::with_capacity(100);

        memchr::memmem::find_iter(input, "do()").for_each(|index| dos.push_back(index));
        memchr::memmem::find_iter(input, "don't()").for_each(|index| donts.push_back(index));

        let mut digit_index;
        let mut first;
        let mut digit_buffer: [Sum; 3] = [0, 0, 0];
        let mut sum = 0;
        let mut should = true;
        for mut index in memchr::memmem::find_iter(input, "mul(") {
            'dos: loop {
                match (dos.front(), donts.front()) {
                    (Some(&do_index), Some(&dont_index)) => {
                        if do_index < dont_index {
                            if do_index < index {
                                dos.pop_front();
                                should = true;
                            } else {
                                break 'dos;
                            }
                        } else {
                            if dont_index < index {
                                donts.pop_front();
                                should = false;
                            } else {
                                break 'dos;
                            }
                        }
                    }
                    (Some(&do_index), None) => {
                        if do_index < index {
                            dos.pop_front();
                            should = true;
                        } else {
                            break 'dos;
                        }
                    }
                    (None, Some(&dont_index)) => {
                        if dont_index < index {
                            donts.pop_front();
                            should = false;
                        } else {
                            break 'dos;
                        }
                    }
                    (None, None) => break 'dos,
                }
            }

            if !should {
                continue;
            }

            digit_index = 0;
            first = 0;
            index += 4;
            let mut r#char;
            loop {
                r#char = input[index];
                match r#char {
                    b')' => {
                        let second = match digit_index {
                            1 => digit_buffer[0],
                            2 => digit_buffer[0] * 10 + digit_buffer[1],
                            3 => digit_buffer[0] * 100 + digit_buffer[1] * 10 + digit_buffer[2],
                            _ => unsafe { unreachable_unchecked() },
                        };
                        sum += (first as Sum) * (second as Sum);
                        break;
                    }
                    b',' => {
                        first = match digit_index {
                            1 => digit_buffer[0],
                            2 => digit_buffer[0] * 10 + digit_buffer[1],
                            3 => digit_buffer[0] * 100 + digit_buffer[1] * 10 + digit_buffer[2],
                            _ => unsafe { unreachable_unchecked() },
                        };
                        digit_index = 0;
                        index += 1;
                    }
                    digit => {
                        if digit_index > 2 {
                            break;
                        }
                        digit_buffer[digit_index] = (digit & 0b1111) as Sum;
                        digit_index += 1;
                        index += 1;
                    }
                }
            }
        }

        sum
    }

    unsafe { inner(input) }
}
