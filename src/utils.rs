#[inline(always)]
pub fn parse_u64_fast<const DIGITS: usize>(input: &[u8]) -> u64 {
    let mut result = 0u64;

    for i in 0..DIGITS {
        result = result * 10 + (input[i] - b'0') as u64;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_u64_fast_1_to_5_digits() {
        assert_eq!(parse_u64_fast::<1>("3".as_bytes()), 3);
        assert_eq!(parse_u64_fast::<2>("31".as_bytes()), 31);
        assert_eq!(parse_u64_fast::<3>("314".as_bytes()), 314);
        assert_eq!(parse_u64_fast::<4>("3141".as_bytes()), 3141);
        assert_eq!(parse_u64_fast::<5>("31415".as_bytes()), 31415);
    }
}
