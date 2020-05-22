use crate::fasta::SeqLoader;
use crate::search::Search;
use std::{env};
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
//use std::io::prelude::*;

pub mod alphabet;
pub mod compress;
pub mod fasta;
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
    }
    else if(args.len() - 1) > 0 && (args.len() - 1) < 3{
        println!("Usage: run [RNAseq file] [DNA file] [Output file]");
    }
    else{
        _rna_file = "fixtures/RNAs.fasta";
        _dna_file = "fixtures/DCEs.fasta";
        _outfile = "output.txt";
    }

    //let mut output = File::create(Path::new(_outfile)).expect("Unable to create outfile");
    let output = File::create(Path::new(_outfile)).unwrap();
    let mut writer = BufWriter::new(&output);


    let needles: Vec<_> = SeqLoader::from_path(Path::new(&_dna_file)).collect();
    println!("Loaded DCE sequences");
    //let haystacks = SeqLoader::from_path(Path::new("/home/sarah/RNAseq/SRR1099957.4.test.fasta"));
    let haystacks = SeqLoader::from_path(Path::new(&_rna_file));
    println!("Loaded RNA sequences");
    let alphabet = "ATCG";
    let mut count = 0;
    let n = 5478274;
    let check = 54782;
    for haystack in haystacks {
        count = count + 1;
        if count % check == 0{
            let progress = count / n;
            println!("Progress: {}", progress);
        }
        for result in Search::new(&haystack, &needles, &alphabet) {
            writeln!(&mut writer, "{} found in {} at offset {}. Total hits: {}",
                   result.needle, result.haystack, result.offset, result.hits).ok();

            println!(
                "{} found in {} at offset {}. Total hits: {}",
                result.needle, result.haystack, result.offset, result.hits
            );
        }
    }
    println!("{}\n", count);
}
