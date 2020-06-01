//use crate::seqloader::Seq;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompressedSeq {
    pub sequence: u64,
}

impl CompressedSeq {
    /// Creates and returns a compressed version of the given `Seq`,
    /// provided that the instance is exactly 32 base pairs long.
    ///
    /// TODO: Handle shorter sequences?
    pub fn from_seq(seq: &String, alphabet: &HashMap<char, u64>) -> Option<CompressedSeq> {
        if seq.len() != 32 {
            None
        } else {
            let mut sequence = 0u64;
            for nuc in seq.chars() {
                let mask = match alphabet.get(&nuc) {
                    Some(mask) => *mask,
                    None => return None,
                };
                sequence = (sequence << 2) | mask;
            }
            Some(CompressedSeq {
                sequence,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alphabet::make_alphabet;
    use crate::compress::CompressedSeq;
    use crate::seqloader::Seq;

    #[test]
    fn new_compressed_seq() {
        let seq = Seq::new("id", "abcdabcdabcdabcdabcdabcdabcdabcd");
        let alpha = make_alphabet("abcd");
        let comp_seq = CompressedSeq::from_seq(&seq, &alpha).unwrap();
        assert_eq!(comp_seq.identifier, seq.identifier);
        assert_eq!(comp_seq.length, seq.length);
        assert_eq!(comp_seq.sequence, 0x1b1b1b1b1b1b1b1b);
    }

    #[test]
    fn new_compressed_small_seq() {
        let seq = Seq::new("id", "abcd");
        let alpha = make_alphabet("abcd");
        let comp_seq = CompressedSeq::from_seq(&seq, &alpha);
        assert_eq!(comp_seq, None);
    }

    #[test]
    fn new_compressed_large_seq() {
        let seq = Seq::new("id", "abcdabcdabcdabcdabcdabcdabcdabcdaaaa");
        let alpha = make_alphabet("abcd");
        let comp_seq = CompressedSeq::from_seq(&seq, &alpha);
        assert_eq!(comp_seq, None);
    }
}
