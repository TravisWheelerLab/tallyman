use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

pub struct SeqLoader<T: BufRead> {
    next_line: String,
    source_file: T,
}

impl SeqLoader<BufReader<File>> {
    pub fn from_path(path: &Path) -> SeqLoader<BufReader<File>> {
        let file = File::open(path).unwrap();
        SeqLoader {
            next_line: String::new(),
            source_file: BufReader::new(file),
        }
    }
}

impl<T: BufRead> SeqLoader<T> {
    pub fn new(file: T) -> SeqLoader<T> {
        SeqLoader {
            next_line: String::new(),
            source_file: file,
        }
    }
}

/// Implement an iterator in order to efficiently load sequences
/// one at a time.
///
/// TODO: Might be able to eek out better performance by parsing smarter
impl<T: BufRead> Iterator for SeqLoader<T> {
    type Item = Seq;

    fn next(&mut self) -> Option<Self::Item> {
        let mut identifier = String::new();
        let mut sequence = String::new();
        let mut next_line = self.next_line.to_string();

        loop {
            if !next_line.is_empty() {
                if next_line.starts_with(">") {
                    if identifier.is_empty() {
                        let mut next_chunk = next_line.as_str();
                        next_chunk = next_chunk.trim_start_matches(">");
                        next_chunk = next_chunk.trim();
                        identifier.push_str(next_chunk);
                    } else {
                        self.next_line = next_line.to_string();
                        break;
                    }
                } else {
                    let next_chunk = next_line.trim();
                    sequence.push_str(next_chunk);
                }
                next_line.clear();
            }

            match self.source_file.read_line(&mut next_line) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                }
                Err(_e) => return None,
            }
        }

        if !identifier.is_empty() && !sequence.is_empty() {
            Some(Seq::new(identifier.as_str(), sequence.as_str()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fasta::{Seq, SeqLoader};

    #[test]
    fn test_haystack() {
        let haystack = Seq::new("identifier", "sequence");
        assert_eq!(haystack.identifier.as_str(), "identifier");
        assert_eq!(haystack.sequence, "SEQUENCE".chars().collect::<Vec<char>>());
    }

    #[test]
    fn test_next() {
        let file = std::io::Cursor::new(String::from("> foo\nabcd\n> bar\ndcba"));
        let mut haystacks = SeqLoader::new(file);

        let first = haystacks.next().unwrap();
        assert_eq!(first.identifier, "foo");
        assert_eq!(first.sequence, "ABCD".chars().collect::<Vec<char>>());

        let second = haystacks.next().unwrap();
        assert_eq!(second.identifier, "bar");
        assert_eq!(second.sequence, "DCBA".chars().collect::<Vec<char>>());

        assert!(haystacks.next().is_none());
    }

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
