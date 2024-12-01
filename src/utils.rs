use bytes::Buf;

#[inline(always)]
pub fn parse_u64_fast(input: &mut &[u8]) -> u64 {
    let mut result = 0u64;

    while input.has_remaining() {
        let byte = input.get_u8();
        if !byte.is_ascii_digit() {
            break;
        }
        result = result * 10 + (byte - b'0') as u64;
    }

    result
}
