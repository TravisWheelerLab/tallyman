use anyhow::{anyhow, Result};
use clap::Parser;
use kseq::parse_path;
use std::{
    fs::File,
    io::{self, Write},
    time::Instant,
};

use crate::compress::compress_chars;
//use crate::constants::HASH_CAPACITY_MULTIPLE;
//use crate::hash::Hash;
use crate::{
    //fasta_read::SeqLoader,
    search::{Search, SearchResult},
    //sequence::Seq,
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
    #[arg(long, short, value_name = "OUT", default_value = "output.txt")]
    output: String,

    /// Verbose output
    #[arg(long, short)]
    verbose: bool,
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
    let mut map = MultiMap::new();

    // Load the DCE sequences and pre-compress them,
    // make the multimap for post-processing
    let timer = Instant::now();
    let mut needles = vec![];

    let mut dna = get_reader(&args.dna)?;
    while let Some(rec) = dna.iter_record()? {
        let compressed_seq = compress_chars(rec.seq());
        needles.push(compressed_seq);
        map.insert(compressed_seq, rec.head().to_string());
    }

    if args.verbose {
        eprintln!(
            "Time to load and hash DCE sequences: {:?}",
            timer.elapsed()
        );
    }

    let mut out_file: Box<dyn Write> = if args.output == "-".to_string() {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(args.output)?)
    };

    // Search through each of the RNA sequences, reusing
    // the sequence and search results instances.
    let timer = Instant::now();
    let mut rna = get_reader(&args.rna)?;
    let mut search = Search::new(needles);
    let mut results: Vec<SearchResult> = vec![];

    writeln!(out_file, "File: {}", args.rna)?;
    while let Some(rec) = rna.iter_record()? {
        results.clear();
        search.search(&rec.seq(), &rec.head(), &mut results);
    }

    if args.verbose {
        eprintln!("Time to search RNA sequences: {:?}", timer.elapsed());
    }

    for (i, count) in search.needles.hits.into_iter().enumerate() {
        if count > 0 {
            if let Some(names) = map.get_vec(&search.needles.container[i]) {
                for name in names {
                    writeln!(out_file, "{name}\t{count}")?;
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn get_reader(filename: &str) -> Result<kseq::Paths> {
    parse_path(filename).map_err(|e| anyhow!("{filename}: {e}"))
}
