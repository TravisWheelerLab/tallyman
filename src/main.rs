use crate::{
    compress::compress_seq,
    //search::{Search, SearchResult},
    search::Search,
};
use anyhow::{anyhow, Result};
use clap::Parser;
//use dashmap::DashMap;
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
    writeln!(out_file, "File: {}", &args.rna)?;

    //let mut results: Vec<SearchResult> = vec![];
    let mut search = Search::new(needles);
    while let Some(rec) = rna.iter_record()? {
        //results.clear();
        //search.search(rec.seq(), rec.head(), &mut results);
        search.search(rec.seq());
    }
    //dbg!(&search.needles.hits);

    for (i, count) in search.needles.hits.into_iter().enumerate() {
        if count > 0 {
            if let Some(names) = map.get_vec(&search.needles.container[i]) {
                for name in names {
                    writeln!(out_file, "{name}\t{count}")?;
                }
            }
        }
    }

    // Using DashMap
    //let search = Search::new(needles);
    //let hits: DashMap<u64, u32> = DashMap::new();

    //while let Some(rec) = rna.iter_record()? {
    //    search.pure_search(&search.needles, rec.seq(), &hits);
    //}

    // Attempt to use parallel but runs out of memory
    //let mut seqs = vec![];
    //while let Some(rec) = rna.iter_record()? {
    //    seqs.push(rec.seq().to_string());
    //}
    //seqs.par_iter().for_each(|seq| {
    //    search.pure_search(&search.needles, &seq, &hits);
    //});

    //dbg!(&hits);
    //for hit in hits.iter() {
    //    if let Some(names) = map.get_vec(&hit.key()) {
    //        for name in names {
    //            writeln!(out_file, "{name}\t{}", hit.value())?;
    //        }
    //    }
    //}

    if args.verbose {
        eprintln!("Time to search RNA sequences: {:?}", timer.elapsed());
    }

    Ok(())
}

// --------------------------------------------------
fn get_reader(filename: &str) -> Result<kseq::Paths> {
    parse_path(filename).map_err(|e| anyhow!("{filename}: {e}"))
}
