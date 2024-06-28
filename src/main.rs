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
    /// Junctions file
    #[arg(short, long, value_name = "JUNCTIONS")]
    junctions: String,

    /// Reads file(s)
    #[arg(short, long, value_name = "READS", num_args(1..), required(true))]
    reads: Vec<String>,

    /// Output directory
    #[arg(short, long, value_name = "OUTDIR", default_value = "out")]
    outdir: String,

    /// Threads
    #[arg(short, long, value_name = "THREADS")]
    threads: Option<usize>,

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
    // Optionally set num of threads, default will use all available
    if let Some(num) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num)
            .build_global()
            .unwrap();
    }

    // Load the DCE sequences and compress them,
    // make the multimap for post-processing
    let timer = Instant::now();
    let mut map = HashMap::new();
    let mut junctions = vec![];
    let mut junctions_file = get_reader(&args.junctions)?;

    while let Some(rec) = junctions_file.iter_record()? {
        match compress_seq(rec.seq()) {
            Some(comp) => {
                junctions.push(comp);
                if map.get(&comp).is_some() {
                    eprintln!(
                        r#"WARNING: Junction sequence "{}" ({}) duplicated"#,
                        rec.seq(),
                        rec.head()
                    );
                } else {
                    map.insert(comp, rec.head().to_string());
                }
            }
            _ => eprintln!(
                r#"Junction sequence "{}" ({}) rejected"#,
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

    if junctions.is_empty() {
        bail!("No junctions");
    }

    let outdir = Path::new(&args.outdir);

    if !outdir.exists() {
        fs::create_dir_all(&outdir)?;
    }

    args.reads
        .into_par_iter()
        .try_for_each(|reads_file| -> Result<()> {
            let basename = Path::new(&reads_file)
                .file_name()
                .ok_or(anyhow!("basename"))?
                .to_os_string();

            let mut out_data_file = basename.clone();
            out_data_file.push(".txt");
            let out_data_path = &outdir.join(out_data_file);
            let mut out_data = File::create(out_data_path)?;

            let mut out_count_file = basename.clone();
            out_count_file.push(".count");
            let out_count_path = &outdir.join(out_count_file);
            let mut out_count = File::create(out_count_path)?;

            // Search through each of the RNA sequences, reusing
            // the sequence and search results instances.
            let timer = Instant::now();
            let mut reads: kseq::Paths = get_reader(&reads_file)?;
            writeln!(out_data, "File: {}", &reads_file)?;

            let mut search: Search = Search::new(&junctions)?;
            let mut read_count = 0;
            while let Some(rec) = reads.iter_record()? {
                search.search(rec.seq());
                read_count += 1;
            }

            if args.verbose {
                eprintln!(
                    r#"Time to search "{reads_file}": {:?}"#,
                    timer.elapsed()
                );
            }

            for (i, count) in search.junctions.hits.into_iter().enumerate() {
                if count > 0 {
                    if let Some(name) = map.get(&search.junctions.key[i]) {
                        writeln!(out_data, "{name}\t{count}")?;
                    }
                }
            }

            writeln!(out_count, "{read_count}")?;
            Ok(())
        })?;

    Ok(())
}

// --------------------------------------------------
fn get_reader(filename: &str) -> Result<kseq::Paths> {
    parse_path(filename).map_err(|e| anyhow!("{filename}: {e}"))
}
