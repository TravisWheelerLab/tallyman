use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::{fs, path::Path};
use tempfile::NamedTempFile;

const PRG: &str = "the_count";
const DNA_FA: &str = "tests/inputs/dna.fasta";
const DNA_FQ: &str = "tests/inputs/dna.fastq";
const RNA_FA_50K: &str = "tests/inputs/rna-50k.fasta";
const RNA_FQ_50K: &str = "tests/inputs/rna-50k.fastq";
const RNA_FA_100K: &str = "tests/inputs/rna-100k.fasta";
const RNA_FQ_100K: &str = "tests/inputs/rna-100k.fastq";
const OUT_FA_50K: &str = "tests/outputs/out-50k-fasta.txt";
const OUT_FA_100K: &str = "tests/outputs/out-100k-fasta.txt";
const OUT_FQ_50K: &str = "tests/outputs/out-50k-fastq.txt";
const OUT_FQ_100K: &str = "tests/outputs/out-100k-fastq.txt";

// --------------------------------------------------
fn gen_bad_file() -> String {
    loop {
        let filename = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// --------------------------------------------------
#[test]
fn dies_bad_rna_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args(["-r", &bad, "-d", DNA_FA])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_dna_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args(["-d", &bad, "-r", RNA_FA_50K])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(rna: &str, dna: &str, expected_file: &str) -> Result<()> {
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();
    Command::cargo_bin(PRG)?
        .args(["-d", dna, "-r", rna, "-o", &outpath])
        .assert()
        .success();

    assert!(Path::new(outpath).exists());

    let expected = fs::read_to_string(expected_file)?;
    let actual = fs::read_to_string(outpath)?;
    assert_eq!(&actual, &expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn run_50k_fasta() -> Result<()> {
    run(RNA_FA_50K, DNA_FA, OUT_FA_50K)
}

// --------------------------------------------------
#[test]
fn run_50k_fastq() -> Result<()> {
    run(RNA_FQ_50K, DNA_FQ, OUT_FQ_50K)
}

// --------------------------------------------------
#[test]
fn run_100k_fasta() -> Result<()> {
    run(RNA_FA_100K, DNA_FA, OUT_FA_100K)
}

// --------------------------------------------------
#[test]
fn run_100k_fastq() -> Result<()> {
    run(RNA_FQ_100K, DNA_FQ, OUT_FQ_100K)
}
