use std::{fs::File, io::Write, path::Path, time::Instant};

use crate::compress::compress_chars;
//use crate::constants::HASH_CAPACITY_MULTIPLE;
use crate::fasta_read::SeqLoader;
//use crate::hash::Hash;
use crate::search::{Search, SearchResult};
use crate::sequence::Seq;
use anyhow::Result;
use clap::Parser;
use multimap::MultiMap;

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

pub mod alphabet;
pub mod compress;
pub mod constants;
pub mod fasta_read;
pub mod hash;
pub mod search;
pub mod sequence;

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
    let mut sequence = Seq::new();
    let mut map = MultiMap::new();

    // Load the DCE sequences and pre-compress them, and
    // make the multimap for post-processing
    let dce_start = Instant::now();

    let mut needles = Vec::<u64>::new();
    let mut dce_loader = SeqLoader::from_path(Path::new(&args.dna))?;
    while dce_loader.next_seq(&mut sequence) {
        let compressed_seq =
            compress_chars(sequence.characters, sequence.length);
        needles.push(compressed_seq);
        map.insert(compressed_seq, sequence.identifier.clone());
    }

    let duration = dce_start.elapsed();
    println!("Time to load and hash DCE sequences: {:?}", duration);

    // Open the output file
    let mut output = File::create(Path::new(&args.output))?;

    // Search through each of the RNA sequences, reusing
    // the sequence and search results instances.
    let rna_start = Instant::now();

    let mut rna_loader = SeqLoader::from_path(Path::new(&args.rna))?;
    let mut search = Search::new(needles);
    let mut search_results = Vec::<SearchResult>::new();
    writeln!(output, "File: {}", &args.rna)?;
    while rna_loader.next_seq(&mut sequence) {
        search_results.clear();
        search.search(&sequence, &mut search_results);
    }

    let duration = rna_start.elapsed();
    println!("Time to search RNA sequences: {:?}", duration);

    for i in 0..search.needles.hits.len() {
        if search.needles.container[i] != 0 {
            let count = search.needles.hits[i];
            if count != 0 {
                if let Some(names) = map.get_vec(&search.needles.container[i])
                {
                    for name in names {
                        writeln!(output, "{}\t{}", name, count)?;
                    }
                }
            }
        }
    }

    Ok(())
}
