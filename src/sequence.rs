#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seq {
    pub identifier: String,
    pub length: usize,
    pub sequence: Vec<char>,
}

impl Seq {
    pub fn new(identifier: &str, sequence: &str) -> Seq {
        Seq {
            identifier: String::from(identifier),
            length: sequence.len(),
            sequence: sequence.to_uppercase().chars().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sequence::Seq;

    #[test]
    fn test_seq_equality() {
        let s0 = Seq::new("foo", "abcd");
        let s1 = Seq::new("foo", "abcd");
        let s2 = Seq::new("bar", "abcd");
        let s3 = Seq::new("foo", "dcba");

        assert_eq!(s0, s1);
        assert_ne!(s0, s2);
        assert_ne!(s0, s3);
    }
}
