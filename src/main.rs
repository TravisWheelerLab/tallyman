use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, Write},
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
    let mut sequence = Seq::new();
    let mut map = MultiMap::new();

    // Load the DCE sequences and pre-compress them,
    // make the multimap for post-processing
    let timer = Instant::now();
    let mut needles = vec![];
    let mut dce_loader = SeqLoader::from_path(&args.dna)?;
    while dce_loader.next_seq(&mut sequence) {
        let compressed_seq =
            compress_chars(sequence.characters, sequence.length);
        needles.push(compressed_seq);
        map.insert(compressed_seq, sequence.identifier.clone());
    }

    //dbg!(&map);
    eprintln!("Time to load and hash DCE sequences: {:?}", timer.elapsed());

    // Open the output file
    //let outfile = &args.output.unwrap();
    //let output =
    //    File::create(&outfile).map_err(|e| anyhow!("{outfile}: {e}"))?;
    //let mut writer = BufWriter::new(&output);
    let mut out_file: Box<dyn Write> = match &args.output {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    // Search through each of the RNA sequences, reusing
    // the sequence and search results instances.
    let timer = Instant::now();

    let mut rna_loader = fasta_read::SeqLoader::from_path(&args.rna)?;
    let mut search = Search::new(needles);
    let mut results: Vec<SearchResult> = vec![];
    //out_file.write_fmt(format_args!("File: {}\n", args.rna))?;
    writeln!(out_file, "File: {}\n", args.rna)?;

    while rna_loader.next_seq(&mut sequence) {
        results.clear();
        search.search(&sequence, &mut results);
    }

    eprintln!("Time to search RNA sequences: {:?}", timer.elapsed());

    for i in 0..search.needles.hits.len() {
        if search.needles.container[i] != 0 {
            let count = search.needles.hits[i];
            if count != 0 {
                if let Some(names) = map.get_vec(&search.needles.container[i])
                {
                    println!("{names:?}");
                }
                //for j in names {
                //    for i in j {
                //        //println!("{}   {}", i, count);
                //        out_file
                //            .write_fmt(format_args!("{}\t{}\n", i, count))?;
                //    }
                //}
            }
        }
    }
    Ok(())
}
