use crate::{compress::compress_seq, search::Search};
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use kseq::parse_path;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    time::Instant,
};

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
    let mut map = HashMap::new();
    let mut needles = vec![];
    let mut dna = get_reader(&args.dna)?;

    while let Some(rec) = dna.iter_record()? {
        match compress_seq(rec.seq()) {
            Some(comp) => {
                needles.push(comp);
                if map.get(&comp).is_some() {
                    eprintln!(
                        r#"WARNING: "{}" ({}) duplicated"#,
                        rec.seq(),
                        rec.head()
                    );
                } else {
                    map.insert(comp, rec.head().to_string());
                }
            }
            _ => eprintln!(
                r#"DNA sequence "{}" ({}) rejected"#,
                rec.seq(),
                rec.head()
            ),
        }
    }

    if args.verbose {
        eprintln!(
            "Time to load and hash DCE sequences: {:?}",
            timer.elapsed()
        );
    }

    if needles.is_empty() {
        bail!("No needles");
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

    let mut search = Search::new(&needles)?;
    while let Some(rec) = rna.iter_record()? {
        search.search(rec.seq());
    }

    if args.verbose {
        eprintln!("Time to search RNA sequences: {:?}", timer.elapsed());
    }

    for (i, count) in search.needles.hits.into_iter().enumerate() {
        if count > 0 {
            if let Some(name) = map.get(&search.needles.key[i]) {
                writeln!(out_file, "{name}\t{count}")?;
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn get_reader(filename: &str) -> Result<kseq::Paths> {
    parse_path(filename).map_err(|e| anyhow!("{filename}: {e}"))
}
