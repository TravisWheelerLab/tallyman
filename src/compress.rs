use crate::alphabet::encode_char;

pub type CompressedSeq = u64;

/// Creates and returns a compressed version of the given `Seq`,
/// provided that the instance is exactly 32 base pairs long.
pub fn compress_seq(seq: &str) -> CompressedSeq {
    if seq.len() != 32 {
        panic!("sequence must have length 32")
    } else {
        let mut sequence = 0u64;
        for nuc in seq.chars() {
            let mask = encode_char(nuc);
            sequence = (sequence << 2) | mask;
        }
        sequence
    }
}

pub fn compress_chars(chars: [char; 256], length: usize) -> CompressedSeq {
    let mut sequence = 0u64;
    for i in 0..length {
        let mask = encode_char(chars[i]);
        sequence = (sequence << 2) | mask;
    }
    sequence
}

#[cfg(test)]
mod tests {
    use crate::compress::compress_seq;

    #[test]
    fn new_compressed_seq() {
        let seq = "ACGTACGTACGTACGTACGTACGTACGTACGT";
        let comp_seq = compress_seq(seq);
        assert_eq!(comp_seq, 0x1b1b1b1b1b1b1b1b);
    }
}
