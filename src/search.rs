use crate::alphabet::encode_char;
use crate::seqloader::Seq;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub haystack: String,
    pub needle: usize,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Search {
    haystack_index: usize,
    haystack_size: usize,
    haystack_window: u64,
    needles: HashMap<u64, usize>,
    start_index: usize,
}

impl Search {
    pub fn new(needles: &Vec<u64>) -> Search {
        let mut needles_map = HashMap::with_capacity(needles.len());
        for (index, needle) in needles.iter().enumerate() {
            needles_map.insert(*needle, index);
        }
        Search {
            haystack_index: 0,
            haystack_size: 0,
            haystack_window: 0,
            needles: needles_map,
            start_index: 0,
        }
    }

    pub fn search(&mut self, haystack: &Seq, results: &mut Vec<SearchResult>) {
        // Reset in preparation for the search.
        self.haystack_index = 0;
        self.haystack_size = haystack.length;
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
                let next_char = haystack.sequence[self.haystack_index];

                // If we find a bad character, we basically just restart
                // the search from the next character.
                // TODO: Find out what an "N" represents
                if next_char == '-' || next_char == 'N' {
                    self.start_index = self.haystack_index + 1;
                    self.haystack_index = self.start_index;
                    continue 'search;
                }

                let mask = encode_char(next_char);
                self.haystack_window = (self.haystack_window << 2) | mask;
                self.haystack_index += 1;
            }

            // Bump the start index in order to slide the window one
            // nucleotide to the right.
            self.start_index += 1;

            // Compare the current haystack sequence against each of
            // the needle sequences and return the first match we fine.
            // FIXME: This should use the custom hash, build in new()
            if self.needles.contains_key(&self.haystack_window) {
                let result = SearchResult {
                    // TODO: Can we get rid of this clone? Prolly not
                    haystack: haystack.identifier.clone(),
                    needle: self.needles[&self.haystack_window],
                    offset: self.haystack_index - 32,
                };
                results.push(result);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::compress::compress_seq;
    use crate::search::{Search, SearchResult};
    use crate::seqloader::Seq;

    #[test]
    fn test_min_size_search() {
        let haystack = Seq::new("id", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
        let needles = vec![
            compress_seq("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC").unwrap(),
            compress_seq("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap(),
        ];
        let mut results = Vec::<SearchResult>::new();
        let mut search = Search::new(&needles);
        search.search(&haystack, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results.first().unwrap().haystack, "id");
        assert_eq!(results.first().unwrap().needle, 1);
        assert_eq!(results.first().unwrap().offset, 0);
    }

    #[test]
    fn test_larger_search() {
        let haystack = Seq::new("id", "ACACTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTACAC");
        let needles = vec![
            compress_seq("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC").unwrap(),
            compress_seq("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT").unwrap(),
        ];
        let mut results = Vec::<SearchResult>::new();
        let mut search = Search::new(&needles);
        search.search(&haystack, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results.first().unwrap().haystack, "id");
        assert_eq!(results.first().unwrap().needle, 1);
        assert_eq!(results.first().unwrap().offset, 4);
    }
}
