use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

use crate::compress::compress_chars;
use crate::fasta_read::SeqLoader;
use crate::hashmap::Hashmap;
use crate::search::{Search, SearchResult};
use crate::sequence::Seq;

pub mod alphabet;
pub mod compress;
pub mod constants;
pub mod fasta_read;
pub mod hash;
pub mod hashmap;
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
    let mut mapping = Hashmap::new(30592);

    // Load the DCE sequences and pre-compress them
    let dce_start = Instant::now();

    let mut needles = Vec::<u64>::new();
    let mut dce_loader = SeqLoader::from_path(Path::new(&_dna_file));
    while dce_loader.next_seq(&mut sequence) {
        let compressed_seq = compress_chars(sequence.characters, sequence.length);
        needles.push(compressed_seq);
        mapping.add(compressed_seq, sequence.identifier.clone());
        println!("Seq ID: {:?}", sequence.identifier.clone());
        println!("{:?}", compressed_seq);
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
    let mut search = Search::new(needles);
    let mut search_results = Vec::<SearchResult>::new();
    _writer
        .write_fmt(format_args!("dce index, rna identifier, offset\n"))
        .unwrap();
    while rna_loader.next_seq(&mut sequence) {
        search_results.clear();
        search.search(&sequence, &mut search_results);
        for result in &search_results {
            _writer
                .write_fmt(format_args!(
                    "{}, {}, {}, {}\n",
                    result.needle, result.haystack, result.offset, result.index
                ))
                .unwrap();
        }
    }

    let duration = rna_start.elapsed();
    println!("Time to search RNA sequences: {:?}", duration);

    for i in 0..search.needles.hits.len() {
        if search.needles.container[i] != 0 {
            let count = search.needles.hits[i];
            if count != 0 {
                for j in &mapping.dce_id[i] {
                    println!("{} hits: {}", j, count);
                    //TODO: This is having issues with immutable vs mutable borrowing of the mapping
                    //println!("{}, index {}: {} hits", j, &mapping.get_index(mapping.container[i]), count,);
                }
            }
        }
    }
}
