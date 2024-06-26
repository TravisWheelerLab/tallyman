use anyhow::{anyhow, Result};
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};
use std::{fs, iter::zip, path::Path};
use tempfile::TempDir;

const PRG: &str = "tallyman";
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
fn run(rna_files: &[&str], dna: &str, expected_files: &[&str]) -> Result<()> {
    // outdir will be removed when var leaves scope
    let outdir = TempDir::new()?;
    let mut args: Vec<String> = vec![
        "-d".to_string(),
        dna.to_string(),
        "-o".to_string(),
        outdir.path().to_string_lossy().to_string(),
        "-r".to_string(),
    ];

    for file in rna_files {
        args.push(file.to_string());
    }

    Command::cargo_bin(PRG)?.args(&args).assert().success();

    for (rna_file, expected_file) in zip(rna_files, expected_files) {
        // Output file is RNA basename + ".txt"
        let mut rna_base = Path::new(&rna_file)
            .file_name()
            .ok_or(anyhow!("No basename"))?
            .to_os_string();
        rna_base.push(".txt");
        let outpath = &outdir.path().join(&rna_base);
        assert!(outpath.exists());

        let expected = fs::read_to_string(expected_file)?;
        let actual = fs::read_to_string(outpath)?;
        assert_eq!(&actual, &expected);
    }

    Ok(())
}

// --------------------------------------------------
#[test]
fn run_50k_fasta() -> Result<()> {
    run(&[RNA_FA_50K], DNA_FA, &[OUT_FA_50K])
}

// --------------------------------------------------
#[test]
fn run_50k_fastq() -> Result<()> {
    run(&[RNA_FQ_50K], DNA_FQ, &[OUT_FQ_50K])
}

// --------------------------------------------------
#[test]
fn run_100k_fasta() -> Result<()> {
    run(&[RNA_FA_100K], DNA_FA, &[OUT_FA_100K])
}

// --------------------------------------------------
#[test]
fn run_100k_fastq() -> Result<()> {
    run(&[RNA_FQ_100K], DNA_FQ, &[OUT_FQ_100K])
}

// --------------------------------------------------
#[test]
fn run_50k_100k_fastq() -> Result<()> {
    run(
        &[RNA_FA_50K, RNA_FQ_100K],
        DNA_FQ,
        &[OUT_FA_50K, OUT_FQ_100K],
    )
}
