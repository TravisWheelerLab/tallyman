use crate::fasta::SeqLoader;
use crate::search::Search;
use std::path::Path;

pub mod alphabet;
pub mod compress;
pub mod fasta;
pub mod search;

fn main() {
    let haystacks = SeqLoader::from_path(Path::new("fixtures/RNAs.fasta"));
    let needles: Vec<_> = SeqLoader::from_path(Path::new("fixtures/DCEs.fasta")).collect();
    let alphabet = "ATCG";
    for haystack in haystacks {
        for result in Search::new(&haystack, &needles, &alphabet) {
            println!(
                "{} found in {} at offset {}. Total hits: {}",
                result.needle, result.haystack, result.offset, result.hits
            );
        }
    }
}
