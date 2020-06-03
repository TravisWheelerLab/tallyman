use crate::seqloader::{SeqLoader, Seq};
use crate::search::Search;
use crate::compress::CompressedSeq;
use crate::alphabet::make_alphabet;

extern crate multimap;
use multimap::MultiMap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use bio::io::fasta;

pub mod alphabet;
pub mod compress;
pub mod fasta_read;
pub mod hash;
pub mod seqloader;
pub mod search;

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
    let alphabet = "ATCG";
    let mut needles: MultiMap<u64, String> = MultiMap::with_capacity(5000);
    let mut hits = HashMap::new();
    let alphabet_map = make_alphabet(alphabet);

    let reader = fasta::Reader::from_file(Path::new(&_dna_file)).unwrap();
    for result in reader.records() {
        let record = result.unwrap();
        let seq = std::str::from_utf8(record.seq()).unwrap().to_string().to_uppercase();
        //use compressed seq method to immediately store DCEs as their compressed versions
        let compressed_seq = CompressedSeq::from_seq(&seq, &alphabet_map).unwrap();

        needles.insert(compressed_seq.sequence, record.id().to_string());
        hits.insert(record.id().to_string(), 0);
    }
    println!("Loaded DCE sequences");

    //use existing Seq structure for loading RNAseqs
    let mut loader = fasta_read::SeqLoader::from_path(Path::new(&_rna_file));
    println!("Loaded RNA sequences");

    writeln!(&mut _writer, "DCE     Hits").ok();

    let mut haystack = seqloader::Seq::new("", "");
    while loader.next_seq(&mut haystack) {
        //println!("Next RNAseq input");
        for result in Search::new(&haystack, &needles, &alphabet_map) {
            for name in &result.needle{
                let num_hits = hits.get(name).unwrap();
                let num_hits = *num_hits as i64;
                let num_hits = num_hits+1;
                hits.insert(name.parse().unwrap(), num_hits);
            }

            println!(
                "{:?} found in {} at offset {}",
                result.needle, result.haystack, result.offset
            );
        }
    }
    for (name, num) in hits.iter() {
        writeln!(
            &mut _writer,
            "{}     {}",
            name, num
        )
            .ok();
    }
}
