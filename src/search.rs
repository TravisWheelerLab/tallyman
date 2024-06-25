use crate::{
    compress::encode_char, constants::HASH_CAPACITY_MULTIPLE, hash::Hash,
};
use anyhow::Result;

pub struct Search {
    haystack_index: usize,
    haystack_size: usize,
    haystack_window: u64,
    pub needles: Hash,
    start_index: usize,
}

impl Search {
    pub fn new(needles: &Vec<u64>) -> Result<Search> {
        let mut needles_hash =
            Hash::new(needles.len() * HASH_CAPACITY_MULTIPLE);

        for needle in needles {
            needles_hash.add(*needle)?;
        }

        Ok(Search {
            haystack_index: 0,
            haystack_size: 0,
            haystack_window: 0,
            needles: needles_hash,
            start_index: 0,
        })
    }

    pub fn search(&mut self, sequence: &str) {
        let sequence: Vec<char> = sequence.chars().collect();

        // Reset in preparation for the search.
        self.haystack_index = 0;
        self.haystack_size = sequence.len();
        self.haystack_window = 0;
        self.start_index = 0;

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
                let mask = encode_char(sequence[self.haystack_index]);

                // If we find a bad character, we basically just restart
                // the search from the next character.
                if mask == 255 {
                    self.start_index = self.haystack_index + 1;
                    self.haystack_index = self.start_index;
                    continue 'search;
                }

                self.haystack_window = (self.haystack_window << 2) | mask;
                self.haystack_index += 1;
            }

            // Bump the start index in order to slide the window one
            // nucleotide to the right.
            self.start_index += 1;
            let _ = self.needles.inc_hits(self.haystack_window);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{compress::compress_seq, search::Search};

    #[test]
    fn test_search() {
        let needles = vec![
            compress_seq(&"T".repeat(32)).unwrap(),
            compress_seq(&"G".repeat(32)).unwrap(),
        ];
        let mut search = Search::new(&needles).unwrap();
        search.search("AAGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGAA");

        let res = search.needles.get_hits(needles[0]);
        assert_eq!(res, Some(0));

        let res = search.needles.get_hits(needles[1]);
        assert_eq!(res, Some(1));

        let missing = compress_seq(&"C".repeat(32)).unwrap();
        let res = search.needles.get_hits(missing);
        assert!(res.is_none());
    }

    #[test]
    fn test_search_with_n() {
        let needles = vec![
            compress_seq(&"ACGT".repeat(8)).unwrap(),
            compress_seq(&"G".repeat(32)).unwrap(),
        ];
        let mut search = Search::new(&needles).unwrap();

        search.search("AANACGTACGTACGTACGTACGTACGTACGTACGTNNAA");

        let res = search.needles.get_hits(needles[0]);
        assert_eq!(res, Some(1));

        let res = search.needles.get_hits(needles[1]);
        assert_eq!(res, Some(0));
    }
}
