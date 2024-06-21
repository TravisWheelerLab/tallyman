use crate::{
    compress::compress_seq,
    search::{Search, SearchResult},
};
use anyhow::{anyhow, Result};
use clap::Parser;
use kseq::parse_path;
use multimap::MultiMap;
use std::{
    fs::File,
    io::{self, Write},
    time::Instant,
};

pub mod alphabet;
pub mod compress;
pub mod constants;
pub mod hash;
pub mod search;
//pub mod sequence;

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
    // Load the DCE sequences and compress them,
    // make the multimap for post-processing
    let timer = Instant::now();
    let mut map = MultiMap::new();
    let mut needles = vec![];
    let mut dna = get_reader(&args.dna)?;

    while let Some(rec) = dna.iter_record()? {
        if let Some(comp) = compress_seq(rec.seq()) {
            needles.push(comp);
            map.insert(comp, rec.head().to_string());
        }
    }

    if args.verbose {
        eprintln!(
            "Time to load and hash DCE sequences: {:?}",
            timer.elapsed()
        );
    }

    let mut out_file: Box<dyn Write> = if args.output == *"-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(args.output)?)
    };

    // Search through each of the RNA sequences, reusing
    // the sequence and search results instances.
    let timer = Instant::now();
    let mut rna = get_reader(&args.rna)?;
    let mut results: Vec<SearchResult> = vec![];
    let mut search = Search::new(needles);

    writeln!(out_file, "File: {}", &args.rna)?;
    while let Some(rec) = rna.iter_record()? {
        results.clear();
        search.search(rec.seq(), rec.head(), &mut results);
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
