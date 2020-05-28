use crate::seqloader::SeqLoader;
use crate::search::Search;
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

    let output = File::create(Path::new(_outfile)).unwrap();
    let mut _writer = BufWriter::new(&output);

    let mut needles = MultiMap::with_capacity(5000);
    let mut hits = HashMap::new();

    let reader = fasta::Reader::from_file(Path::new(&_dna_file)).unwrap();
    for result in reader.records() {
        let record = result.unwrap();
        //println!("{}", std::str::from_utf8(record.seq()).unwrap().to_string());
        needles.insert(std::str::from_utf8(record.seq()).unwrap().to_string().to_uppercase(), record.id().to_string());
        hits.insert(record.id().to_string(), 0);
    }

    for (key, values) in needles.iter_all() {
        //println!("key: {:?}, values: {:?}", key, values);
        writeln!(
            &mut _writer,
            "Sequence: {:?}, IDs: {:?}",
            key, values
        )
            .ok();
    }

    //let needles: Vec<_> = SeqLoader::from_path(Path::new(&_dna_file)).collect();
    println!("Loaded DCE sequences");
    let haystacks = SeqLoader::from_path(Path::new(&_rna_file));
    println!("Loaded RNA sequences");
/*    let alphabet = "ATCG";
    let mut count = 0;
    let n = 5478274;
    let check = 54782;
    for haystack in haystacks {
        count = count + 1;
        if count % check == 0 {
            let progress = count / n;
            println!("Progress: {}", progress);
        }
        for result in Search::new(&haystack, &needles, &alphabet) {
            writeln!(
                &mut writer,
                "{} found in {} at offset {}",
                result.needle, result.haystack, result.offset
            )
            .ok();

            println!(
                "{} found in {} at offset {}",
                result.needle, result.haystack, result.offset
            );
        }
    }
    println!("{}\n", count);*/
}
