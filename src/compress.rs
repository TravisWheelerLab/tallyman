pub type CompressedSeq = u64;

const ALPHABET_ENCODINGS: [u64; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 1, 255, 255, 255, 2,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 255, 1, 255, 255,
    255, 2, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

#[inline]
pub fn encode_char(c: char) -> u64 {
    ALPHABET_ENCODINGS[c as usize]
}

/// Creates and returns a compressed version of the given `Seq`,
/// provided that the instance is exactly 32 base pairs long.
pub fn compress_seq(seq: &str) -> Option<CompressedSeq> {
    (seq.len() == 32).then(|| {
        seq.chars()
            .fold(0, |seq, chr| (seq << 2) | encode_char(chr))
    })
}

#[cfg(test)]
mod tests {
    use super::{compress_seq, encode_char};

    #[test]
    fn test_encode_char() {
        for (index, character) in ['A', 'C', 'G', 'T'].iter().enumerate() {
            assert_eq!(encode_char(*character), index as u64);
        }

        for (index, character) in ['a', 'c', 'g', 't'].iter().enumerate() {
            assert_eq!(encode_char(*character), index as u64);
        }
    }

    #[test]
    fn test_encode_invalid_char() {
        for character in &['N', 'F', 'K', 'z', 'y'] {
            assert_eq!(encode_char(*character), 255);
        }
    }

    #[test]
    fn test_compress_seq() {
        let res = compress_seq("A");
        assert!(res.is_none());

        let res = compress_seq(&"A".repeat(32));
        assert_eq!(res, Some(0));

        let res = compress_seq(&"a".repeat(32));
        assert_eq!(res, Some(0));

        let res = compress_seq(&"C".repeat(32));
        assert_eq!(res, Some(0x5555555555555555));

        let res = compress_seq(&"G".repeat(32));
        assert_eq!(res, Some(0xaaaaaaaaaaaaaaaa));

        let res = compress_seq(&"T".repeat(32));
        println!("{:#x}", res.unwrap());
        assert_eq!(res, Some(0xffffffffffffffff));

        let res = compress_seq(&"N".repeat(32));
        assert_eq!(res, Some(0xffffffffffffffff));

        let res = compress_seq(&"ACGT".repeat(8));
        assert_eq!(res, Some(0x1b1b1b1b1b1b1b1b));

        let res = compress_seq(&"TGCA".repeat(8));
        assert_eq!(res, Some(0xe4e4e4e4e4e4e4e4));
    }
}
