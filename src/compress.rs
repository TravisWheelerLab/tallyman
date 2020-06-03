use crate::alphabet::{encode_char, ALPHABET};
use std::collections::HashMap;

pub type CompressedSeq = u64;

/// Creates and returns a compressed version of the given `Seq`,
/// provided that the instance is exactly 32 base pairs long.
pub fn compress_seq(seq: &str) -> Option<CompressedSeq> {
    if seq.len() != 32 {
        None
    } else {
        let mut sequence = 0u64;
        for nuc in seq.chars() {
            let mask = encode_char(nuc);
            sequence = (sequence << 2) | mask;
        }
        Some(sequence)
    }
}

#[cfg(test)]
mod tests {
    use crate::compress::compress_seq;

    #[test]
    fn new_compressed_seq() {
        let seq = "ACGTACGTACGTACGTACGTACGTACGTACGT";
        let comp_seq = compress_seq(seq).unwrap();
        assert_eq!(comp_seq, 0x1b1b1b1b1b1b1b1b);
    }
}
