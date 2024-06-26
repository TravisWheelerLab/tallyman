use crate::{compress::compress_seq, search::Search};
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use kseq::parse_path;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    time::Instant,
};

pub mod compress;
pub mod constants;
pub mod hash;
pub mod search;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// DNA file
    #[arg(short, long, value_name = "DNA")]
    dna: String,

    /// RNA file
    #[arg(short, long, value_name = "RNA", num_args(1..))]
    rna: Vec<String>,

    /// Output directory
    #[arg(short, long, value_name = "OUTDIR", default_value = "out")]
    outdir: String,

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

    let outdir = Path::new(&args.outdir);

    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }

    args.rna
        .into_par_iter()
        .try_for_each(|filename| -> Result<()> {
            let basename = Path::new(&filename)
                .file_name()
                .ok_or(anyhow!("basename"))?;

            let mut basename = basename.to_os_string();
            basename.push(".txt");
            let out_path = &outdir.join(basename);
            let mut out_file = File::create(out_path)?;

            // Search through each of the RNA sequences, reusing
            // the sequence and search results instances.
            let timer = Instant::now();
            let mut rna: kseq::Paths = get_reader(&filename)?;
            writeln!(out_file, "File: {}", &filename)?;

            let mut search: Search = Search::new(&needles)?;
            while let Some(rec) = rna.iter_record()? {
                search.search(rec.seq());
            }

            if args.verbose {
                eprintln!(
                    r#"Time to search "{filename}": {:?}"#,
                    timer.elapsed()
                );
            }

            for (i, count) in search.needles.hits.into_iter().enumerate() {
                if count > 0 {
                    if let Some(name) = map.get(&search.needles.key[i]) {
                        writeln!(out_file, "{name}\t{count}")?;
                    }
                }
            }
            Ok(())
        })?;

    Ok(())
}

// --------------------------------------------------
fn get_reader(filename: &str) -> Result<kseq::Paths> {
    parse_path(filename).map_err(|e| anyhow!("{filename}: {e}"))
}
