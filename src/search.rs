use std::collections::HashMap;

use crate::alphabet::make_alphabet;
use crate::compress::CompressedSeq;
use crate::fasta::Seq;

#[derive(Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub haystack: String,
    pub needle: String,
    pub offset: usize,
    pub hits: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Search {
    alphabet: HashMap<char, u64>,
    haystack: Seq,
    haystack_index: usize,
    haystack_size: usize,
    haystack_window: u64,
    needle_count: usize,
    needle_index: usize,
    needles: Vec<CompressedSeq>,
    start_index: usize,
}

impl Search {
    pub fn new(haystack: &Seq, needles: &Vec<Seq>, alphabet: &str) -> Search {
        let alphabet_map = make_alphabet(alphabet);
        let compressed_needles: Vec<_> = needles
            .iter()
            .map(|needle| CompressedSeq::from_seq(needle, &alphabet_map).unwrap())
            .collect();

        Search {
            alphabet: alphabet_map,
            haystack: (*haystack).clone(),
            haystack_index: 0,
            haystack_size: haystack.length,
            haystack_window: 0,
            needle_count: compressed_needles.len(),
            needle_index: 0,
            needles: compressed_needles,
            start_index: 0,
        }
    }
}

impl Iterator for Search {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        // If we don't have at least 32 nucleotides remaining, we
        // know we are finished.
        'search: while self.start_index <= self.haystack_size - 32 {
            // Bootstrap by encoding the next 31 nucleotides if we
            // haven't done it yet. This happens at the beginning of
            // a search and immediately after a bad character has
            // been encountered. We can ignore the possibility of a
            // missing alphabet character since we've already dealt
            // with the other (valid) possibility above.
            while self.haystack_index < self.start_index + 32 {
                // FIXME: Must have a bug when there's a blank in a certain place
                let next_char = self.haystack.sequence[self.haystack_index];

                // If we find a bad character, we basically just restart
                // the search from the next character.
                // TODO: Find out what an "N" represents
                if next_char == '-' || next_char == 'N' {
                    self.start_index = self.haystack_index + 1;
                    self.haystack_index = self.start_index;
                    continue 'search;
                }

                let mask = match self.alphabet.get(&next_char) {
                    Some(m) => m,
                    None => panic!(format!("unrecognized character {}", next_char)),
                };
                self.haystack_window = (self.haystack_window << 2) | *mask;
                self.haystack_index += 1;
            }

            // Compare the current haystack sequence against each of
            // the needle sequences and return the first match we fine.
            while self.needle_index < self.needle_count {
                let mut needle= &mut self.needles[self.needle_index];
                if needle.sequence == self.haystack_window {
                    needle.hits = needle.hits+1;
                    self.needle_index += 1;
                    return Some(SearchResult {
                        haystack: self.haystack.identifier.clone(),
                        needle: needle.identifier.clone(),
                        offset: self.haystack_index - 32,
                        hits: needle.hits,
                    });
                }
                self.needle_index += 1;
            }

            // Bump the start index in order to slide the window one
            // nucleotide to the right.
            self.start_index += 1;
            self.needle_index = 0;
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::fasta::Seq;
    use crate::search::Search;

    #[test]
    fn test_min_size_search() {
        let haystack = Seq::new("id", "dddddddddddddddddddddddddddddddd");
        let needles = vec![
            Seq::new("id", "cccccccccccccccccccccccccccccccc"),
            Seq::new("di", "dddddddddddddddddddddddddddddddd"),
        ];
        let search = Search::new(&haystack, &needles, "abcd");
        let results: Vec<_> = search.collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results.first().unwrap().needle, "di");
        assert_eq!(results.first().unwrap().offset, 0);
    }

    #[test]
    fn test_larger_search() {
        let haystack = Seq::new("id", "ababddddddddddddddddddddddddddddddddabab");
        let needles = vec![
            Seq::new("id", "cccccccccccccccccccccccccccccccc"),
            Seq::new("di", "dddddddddddddddddddddddddddddddd"),
        ];
        let search = Search::new(&haystack, &needles, "abcd");
        let results: Vec<_> = search.collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results.first().unwrap().needle, "di");
        assert_eq!(results.first().unwrap().offset, 4);
    }
}
