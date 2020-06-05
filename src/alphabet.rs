pub const ALPHABET: [char; 4] = ['A', 'C', 'G', 'T'];

#[inline]
pub fn encode_char(c: char) -> u64 {
    match c {
        'A' | 'a' => 0,
        'C' | 'c' => 1,
        'G' | 'g' => 2,
        'T' | 't' => 3,
        _ => panic!(format!("invalid character '{}'", c)),
    }
}

#[cfg(test)]
mod test {
    use crate::alphabet::encode_char;

    #[test]
    fn test_encode_char() {
        let low_a = encode_char('a');
        assert_eq!(low_a, 0);

        let cap_a = encode_char('A');
        assert_eq!(cap_a, 0);
    }
}
