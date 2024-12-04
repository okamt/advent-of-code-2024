use aoc_runner_derive::aoc;

pub const SIDE: usize = 140;

pub type Count = usize;

// Solution: 2297
// Best: 225 us
#[aoc(day4, part1)]
fn part1(input: &str) -> Count {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> Count {
        let input = input.as_bytes();

        let mut count: Count = 0;

        for row_i in 0..SIDE {
            let row = &input[row_i * (SIDE + 1)..][..SIDE];
            count += memchr::memmem::find_iter(row, b"XMAS").count();
            count += memchr::memmem::find_iter(row, b"SAMX").count();

            if row_i < SIDE - 3 {
                let row2 = &input[(row_i + 1) * (SIDE + 1)..][..SIDE];
                let row3 = &input[(row_i + 2) * (SIDE + 1)..][..SIDE];
                let row4 = &input[(row_i + 3) * (SIDE + 1)..][..SIDE];

                fn is_xmas(a: u8, b: u8, c: u8, d: u8) -> bool {
                    match (a, b, c, d) {
                        (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X') => true,
                        _ => false,
                    }
                }

                for col_i in 0..SIDE {
                    if is_xmas(row[col_i], row2[col_i], row3[col_i], row4[col_i]) {
                        count += 1;
                    }
                    if col_i < SIDE - 3 {
                        if is_xmas(
                            row[col_i],
                            row2[col_i + 1],
                            row3[col_i + 2],
                            row4[col_i + 3],
                        ) {
                            count += 1;
                        }
                        if is_xmas(
                            row[col_i + 3],
                            row2[col_i + 2],
                            row3[col_i + 1],
                            row4[col_i],
                        ) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    unsafe { inner(input) }
}

// Solution: 1745
// Best: 68 us
#[aoc(day4, part2)]
fn part2(input: &str) -> Count {
    #[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
    unsafe fn inner(input: &str) -> Count {
        let input = input.as_bytes();

        let mut count: Count = 0;

        for row_i in 1..SIDE - 1 {
            let row_top = &input[(row_i - 1) * (SIDE + 1)..][..SIDE];
            let row_mid = &input[row_i * (SIDE + 1)..][..SIDE];
            let row_bot = &input[(row_i + 1) * (SIDE + 1)..][..SIDE];

            for col_i in 1..SIDE - 1 {
                if row_mid[col_i] == b'A'
                    && matches!(
                        (row_top[col_i - 1], row_bot[col_i + 1]),
                        (b'M', b'S') | (b'S', b'M')
                    )
                    && matches!(
                        (row_top[col_i + 1], row_bot[col_i - 1]),
                        (b'M', b'S') | (b'S', b'M')
                    )
                {
                    count += 1;
                }
            }
        }

        count
    }

    unsafe { inner(input) }
}
