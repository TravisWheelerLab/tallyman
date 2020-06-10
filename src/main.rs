use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

use crate::compress::compress_chars;
use crate::fasta_read::SeqLoader;
use crate::search::{Search, SearchResult};
use crate::sequence::Seq;

pub mod alphabet;
pub mod compress;
pub mod fasta_read;
pub mod hash;
pub mod search;
pub mod sequence;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut _rna_file = "";
    let mut _dna_file = "";
    let mut _outfile = "";

    if (args.len() - 1) == 3 {
        _rna_file = &args[1];
        _dna_file = &args[2];
        _outfile = &args[3];
    } else if (args.len() - 1) > 0 && (args.len() - 1) < 3 {
        println!("Usage: run [RNAseq file] [DNA file] [Output file]");
    } else {
        _rna_file = "fixtures/RNAs.fasta";
        _dna_file = "fixtures/DCEs.fasta";
        _outfile = "output.txt";
    }

    // Create reusable FASTA reading machinery
    let mut sequence = Seq::new();

    // Load the DCE sequences and pre-compress them
    let dce_start = Instant::now();

    let mut hits = Vec::<usize>::new();
    let mut needles = Vec::<u64>::new();
    let mut dce_loader = SeqLoader::from_path(Path::new(&_dna_file));
    while dce_loader.next_seq(&mut sequence) {
        let compressed_seq = compress_chars(sequence.characters, sequence.length);
        hits.push(0);
        needles.push(compressed_seq);
    }

    let duration = dce_start.elapsed();
    println!("Time to load and hash DCE sequences: {:?}", duration);

    // Open the output file
    let output = File::create(Path::new(_outfile)).unwrap();
    let mut _writer = BufWriter::new(&output);

    // Search through each of the RNA sequences, reusing
    // the sequence and search results instances.
    let rna_start = Instant::now();

    let mut rna_loader = fasta_read::SeqLoader::from_path(Path::new(&_rna_file));
    let mut search = Search::new(&needles);
    let mut search_results = Vec::<SearchResult>::new();
    while rna_loader.next_seq(&mut sequence) {
        search_results.clear();
        search.search(&sequence, &mut search_results);
        for result in &search_results {
            hits[result.needle] += 1;
            // println!(
            //     "{:?} found in {} at offset {}",
            //     result.needle, result.haystack, result.offset
            // );
        }
    }

    let duration = rna_start.elapsed();
    println!("Time to search RNA sequences: {:?}", duration);

    // Report result summary
    // for (index, count) in hits.iter().enumerate() {
    //     if *count != 0 {
    //         println!("{} found {} times", index, count);
    //     }
    // }
}
