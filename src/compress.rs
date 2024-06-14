use crate::{alphabet::encode_char, constants::BUFFER_SIZE};
use anyhow::{bail, Result};

pub type CompressedSeq = u64;
const MIN_LEN: usize = 32;

/// Creates and returns a compressed version of the given `Seq`,
/// provided that the instance is exactly 32 base pairs long.
pub fn compress_seq(seq: &str) -> Result<CompressedSeq> {
    if seq.len() != MIN_LEN {
        bail!("sequence must have length 32")
    }

    let mut sequence = 0u64;
    for nuc in seq.chars() {
        let mask = encode_char(nuc);
        sequence = (sequence << 2) | mask;
    }
    Ok(sequence)
}

pub fn compress_chars(
    chars: [char; BUFFER_SIZE],
    length: usize,
) -> Result<CompressedSeq> {
    if length != MIN_LEN {
        bail!("sequence must have length 32")
    }

    let mut sequence = 0u64;
    for i in 0..length {
        let mask = encode_char(chars[i]);
        sequence = (sequence << 2) | mask;
    }
    Ok(sequence)
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
