use core::str;
use std::hint::unreachable_unchecked;

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

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
    todo!()
}
