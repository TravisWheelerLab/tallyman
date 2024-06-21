use crate::{alphabet::encode_char, constants::BUFFER_SIZE};

pub type CompressedSeq = u64;

/// Creates and returns a compressed version of the given `Seq`,
/// provided that the instance is exactly 32 base pairs long.
pub fn compress_seq(seq: &str) -> Option<CompressedSeq> {
    (seq.len() == 32).then(|| {
        seq.chars()
            .fold(0, |seq, chr| (seq << 2) | encode_char(chr))
    })
}

pub fn compress_chars(
    chars: [char; BUFFER_SIZE],
    length: usize,
) -> CompressedSeq {
    chars
        .iter()
        .take(length)
        .fold(0, |seq, chr| (seq << 2) | encode_char(*chr))
}

#[cfg(test)]
mod tests {
    use crate::compress::compress_seq;

    #[test]
    fn new_compressed_seq() {
        let seq = "ACGTACGTACGTACGTACGTACGTACGTACGT";
        let comp = compress_seq(seq);
        assert_eq!(comp, Some(0x1b1b1b1b1b1b1b1b));

        let rev: String = seq.chars().rev().collect();
        let comp = compress_seq(&rev);
        assert_eq!(comp, Some(0xe4e4e4e4e4e4e4e4));
    }
}
