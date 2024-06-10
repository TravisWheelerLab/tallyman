use anyhow::Result;
use bio::io::{fasta, fastq};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    time::Instant,
};

extern crate multimap;
use crate::compress::compress_chars;
//use crate::constants::HASH_CAPACITY_MULTIPLE;
//use crate::hash::Hash;
use crate::{
    fasta_read::SeqLoader,
    search::{Search, SearchResult},
    sequence::Seq,
};
use multimap::MultiMap;

pub mod alphabet;
pub mod compress;
pub mod constants;
pub mod fasta_read;
pub mod hash;
pub mod search;
pub mod sequence;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// RNA file
    #[arg(long, short, value_name = "RNA")]
    rna: String,

    /// DNA file
    #[arg(long, short, value_name = "DNA")]
    dna: String,

    /// Output file
    #[arg(long, short, value_name = "OUT")]
    output: Option<String>,

    /// Verbose output
    #[arg(long, short)]
    verbose: bool,
}

#[derive(Debug)]
enum FileFormat {
    Fasta,
    Fastq,
    Unknown,
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    // Create reusable FASTA reading machinery
    //let mut map = MultiMap::new();

    // Load the DCE sequences and pre-compress them,
    // make the multimap for post-processing
    //let timer = Instant::now();
    //let mut needles = vec![];
    //
    let format = guess_format(&args.dna)?;
    println!("format {format:?}");

    //let dna = File::open(&args.dna)?;
    //let mut records = fasta::Reader::new(dna).records();
    //while let Some(Ok(rec)) = records.next() {
    //    println!("{}", std::str::from_utf8(rec.seq())?);
    //    //let compressed_seq = compress_chars(rec.seq());
    //    //needles.push(compressed_seq);
    //    //map.insert(compressed_seq, rec.id().to_string());
    //}

    //let mut sequence = Seq::new();
    ////let mut dce_loader = SeqLoader::from_path(&args.dna)?;
    ////while dce_loader.next_seq(&mut sequence) {
    ////    let compressed_seq =
    ////        compress_chars(sequence.characters, sequence.length);
    ////    needles.push(compressed_seq);
    ////    map.insert(compressed_seq, sequence.identifier.clone());
    ////}

    //if args.verbose {
    //    eprintln!(
    //        "Time to load and hash DCE sequences: {:?}",
    //        timer.elapsed()
    //    );
    //}

    //let mut out_file: Box<dyn Write> = match &args.output {
    //    Some(out_name) => Box::new(File::create(out_name)?),
    //    _ => Box::new(io::stdout()),
    //};

    //// Search through each of the RNA sequences, reusing
    //// the sequence and search results instances.
    //let timer = Instant::now();
    //let mut rna_loader = SeqLoader::from_path(&args.rna)?;
    //let mut search = Search::new(needles);
    //let mut results: Vec<SearchResult> = vec![];

    //writeln!(out_file, "File: {}", args.rna)?;
    //while rna_loader.next_seq(&mut sequence) {
    //    results.clear();
    //    search.search(&sequence, &mut results);
    //}

    //if args.verbose {
    //    eprintln!("Time to search RNA sequences: {:?}", timer.elapsed());
    //}

    //for (i, count) in search.needles.hits.into_iter().enumerate() {
    //    if count > 0 {
    //        if let Some(names) = map.get_vec(&search.needles.container[i]) {
    //            for name in names {
    //                writeln!(out_file, "{name}\t{count}")?;
    //            }
    //        }
    //    }
    //}

    Ok(())
}

fn guess_format(file: &str) -> Result<FileFormat> {
    let mut fh = BufReader::new(File::open(file)?);
    let mut line = String::new();
    let _ = fh.read_line(&mut line)?;
    if line.starts_with(">") {
        Ok(FileFormat::Fasta)
    } else if line.starts_with("@") {
        Ok(FileFormat::Fastq)
    } else {
        Ok(FileFormat::Unknown)
    }
}
