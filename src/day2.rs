use std::hint::unreachable_unchecked;

use aoc_runner_derive::aoc;

pub const REPORTS: usize = 1000;
pub const MAX_LEVELS: usize = 8;
pub const EMPTY_LEVEL: u8 = 0;

pub fn parse_input(input: &str) -> [[u8; MAX_LEVELS]; REPORTS] {
    let mut reports = [[EMPTY_LEVEL; MAX_LEVELS]; REPORTS];
    let input = input.as_bytes();
    let mut report_idx = 0;
    let mut level_idx = 0;
    let mut num_start_idx = 0;
    let mut index = 0;

    macro_rules! parse_num {
        () => {
            let digits = index - num_start_idx;
            reports[report_idx][level_idx] = match digits {
                1 => input[num_start_idx] & 0b1111,
                2 => (input[num_start_idx] & 0b1111) * 10 + (input[num_start_idx + 1] & 0b1111),
                _ => unsafe { unreachable_unchecked() },
            };
        };
    }

    while index < input.len() {
        let r#char = input[index];
        match r#char {
            b'\n' => {
                parse_num!();

                report_idx += 1;
                level_idx = 0;
                num_start_idx = index + 1;
            }
            b' ' => {
                parse_num!();

                level_idx += 1;
                num_start_idx = index + 1;
            }
            _ => {}
        }
        index += 1;
    }

    parse_num!();

    reports
}

// Solution: 326
// Best:
#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let reports = parse_input(input);
    let mut valid = REPORTS;

    //let mut i = 0;
    for report in reports.iter() {
        //let previous_valid = valid;

        macro_rules! check {
            ($op:tt) => {
                let mut index = 2;
                let mut previous = report[1];
                while index < MAX_LEVELS {
                    let current = report[index];
                    if current == 0 {
                        break;
                    }
                    if !(previous $op current) || !(1..=3).contains(&previous.abs_diff(current)) {
                        valid -= 1;
                        break;
                    }
                    index += 1;
                    previous = current;
                }
            };
        }

        if !(1..=3).contains(&report[0].abs_diff(report[1])) {
            valid -= 1;
        } else {
            if report[0] > report[1] {
                check!(>);
            } else {
                check!(<);
            }
        }

        //let is_valid = previous_valid == valid;
        //println!(
        //    "{} {} {:?}",
        //    i,
        //    if is_valid { "###" } else { "   " },
        //    report
        //);
        //i += 1;
    }

    valid
}

// Solution:
// Best:
//#[aoc(day2, part2)]
//pub fn part2(input: &str) -> u32 {
//    let lists = parse_input(input);
//}
