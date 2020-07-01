use crate::alphabet::encode_char;
use crate::constants::HASH_CAPACITY_MULTIPLE;
use crate::hash::Hash;
use crate::sequence::Seq;
use std::ops::DerefMut;

pub struct SearchResult {
    pub haystack: String,
    pub needle: u64,
    pub offset: usize,
    pub hit_index: usize,
}

pub struct Search {
    haystack_index: usize,
    haystack_size: usize,
    haystack_window: u64,
    rev_haystack: u64,
    pub needles: Hash,
    start_index: usize,
}

impl Search {
    pub fn new(needles: Vec<u64>) -> Search {
        let mut needles_hash = Hash::new(needles.len() * HASH_CAPACITY_MULTIPLE);
        for needle in needles {
            needles_hash.add(needle);
        }

        Search {
            haystack_index: 0,
            haystack_size: 0,
            haystack_window: 0,
            rev_haystack: 0,
            needles: needles_hash,
            start_index: 0,
        }
    }

    pub fn search(&mut self, haystack: &Seq, results: &mut Vec<SearchResult>) {
        // Reset in preparation for the search.
        self.haystack_index = 0;
        self.haystack_size = haystack.length;
        self.haystack_window = 0;
        self.rev_haystack = 0;
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
                let next_char = haystack.characters[self.haystack_index];

                let mask = encode_char(next_char);

                // If we find a bad character, we basically just restart
                // the search from the next character.
                if mask == 255 {
                    self.start_index = self.haystack_index + 1;
                    self.haystack_index = self.start_index;
                    continue 'search;
                }

                self.haystack_window = (self.haystack_window << 2) | mask;
                let new_mask = !mask;
                self.rev_haystack = (self.rev_haystack >> 2) | (new_mask<<62);
                self.haystack_index += 1;
            }

            // Bump the start index in order to slide the window one
            // nucleotide to the right.
            self.start_index += 1;

            // Compare the current haystack sequence against each of
            // the needle sequences and return the first match we fine.
            if self.needles.contains(self.haystack_window){
                self.needles.inc_hits(self.haystack_window);
                let result = SearchResult {
                    // TODO: Can we get rid of this clone? Prolly not
                    haystack: haystack.identifier.clone(),
                    needle: self.haystack_window,
                    offset: self.haystack_index - 32,
                    hit_index: self.needles.get_index(self.haystack_window),
                };
                results.push(result);
            }
            if self.needles.contains(self.rev_haystack ) {
                self.needles.inc_hits(self.rev_haystack);
                let result = SearchResult {
                    // TODO: Can we get rid of this clone? Prolly not
                    haystack: haystack.identifier.clone(),
                    needle: self.rev_haystack,
                    offset: self.haystack_index - 32,
                    hit_index: self.needles.get_index(self.rev_haystack),
                };
                results.push(result);
            }
        }
    }

    pub fn print_hits(&mut self) {
        self.needles.print_hits_all();
    }
}


#[cfg(test)]
mod test {
    use crate::compress::compress_seq;
    use crate::search::{Search, SearchResult};
    use crate::sequence::Seq;

    #[test]
    fn test_min_size_search() {
        let haystack = Seq::pre_filled("id", "GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");
        let needles = vec![
            compress_seq("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC"),
            compress_seq("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG"),
        ];
        let mut results = Vec::<SearchResult>::new();
        let mut search = Search::new(needles_hash);
        search.search(&haystack, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results.first().unwrap().haystack, "id");
        // assert_eq!(results.first().unwrap().needle, 1);
        assert_eq!(results.first().unwrap().offset, 0);
    }

    #[test]
    fn test_larger_search() {
        let haystack = Seq::pre_filled("id", "ACACTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTACAC");
        let needles = vec![
            compress_seq("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC"),
            compress_seq("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT"),
        ];
        let mut results = Vec::<SearchResult>::new();
        let mut search = Search::new(needles_hash);
        search.search(&haystack, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results.first().unwrap().haystack, "id");
        // assert_eq!(results.first().unwrap().needle_index, 1);
        assert_eq!(results.first().unwrap().offset, 4);
    }
}
