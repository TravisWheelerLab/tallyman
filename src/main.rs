use std::path::Path;

use crate::fasta::SeqLoader;
use crate::search::Search;
use std::io::{stdout, Write};

pub mod alphabet;
pub mod compress;
pub mod fasta;
pub mod search;

fn main() {
    let haystacks = SeqLoader::from_path(Path::new("experiments/SRR1099957.fasta"));
    let needles: Vec<_> =
        SeqLoader::from_path(Path::new("experiments/dce-sequences.fasta")).collect();

    let mut done = 0;
    for haystack in haystacks {
        for result in Search::new(&haystack, &needles, "ATCG") {
            eprint!("\r");
            println!("{}", result.haystack);
        }

        done += 1;

        if done % 100 == 0 {
            eprint!("\r");
            eprint!("{} of 45409706 completed", done);
            stdout().flush();
        }
    }
}
