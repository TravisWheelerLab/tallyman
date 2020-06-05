extern crate multimap;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

use bio::io::fasta;

use crate::compress::compress_seq;
use crate::search::{Search, SearchResult};

pub mod alphabet;
pub mod compress;
pub mod fasta_read;
pub mod search;
pub mod seqloader;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
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

    //open output file
    let output = File::create(Path::new(_outfile)).unwrap();
    let mut _writer = BufWriter::new(&output);

    //read DCEs in to hash structures
    let mut needles = Vec::<u64>::new();
    let mut hits = Vec::<usize>::new();

    // Load DCE sequences and pre-compress them.
    let dce_start = Instant::now();
    let reader = fasta::Reader::from_file(Path::new(&_dna_file)).unwrap();
    for record in reader.records() {
        let seq = std::str::from_utf8(record.unwrap().seq())
            .unwrap()
            .to_string()
            .to_uppercase();
        let compressed_seq = compress_seq(&seq).unwrap();

        needles.push(compressed_seq);
        hits.push(0);
    }
    let duration = dce_start.elapsed();
    println!("Time elapsed for hashing DCEs is: {:?}", duration);

    // Search through each of the RNA sequences, reusing
    // the sequence and search results instances.
    let rna_start = Instant::now();
    let mut loader = fasta_read::SeqLoader::from_path(Path::new(&_rna_file));
    let mut search = Search::new(&needles);
    let mut search_results = Vec::<SearchResult>::new();
    let mut haystack = seqloader::Seq::new("", "");
    while loader.next_seq(&mut haystack) {
        search_results.clear();
        search.search(&haystack, &mut search_results);

        //println!("Next RNAseq input");
        for result in &search_results {
            hits[result.needle] += 1;
            println!(
                "{:?} found in {} at offset {}",
                result.needle, result.haystack, result.offset
            );
        }
    }
    let duration = rna_start.elapsed();
    println!("Time elapsed for search is: {:?}", duration);

    /*for (name, num) in hits.iter() {
        writeln!(
            &mut _writer,
            "{}     {}",
            name, num
        )
            .ok();
    }*/
}
