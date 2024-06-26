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
fn dies_bad_reads_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args(["-r", &bad, "-j", DNA_FA])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn dies_bad_junction_file() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .args(["-j", &bad, "-r", RNA_FA_50K])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
fn run(
    read_files: &[&str],
    junction_file: &str,
    expected_files: &[&str],
) -> Result<()> {
    // outdir will be removed when var leaves scope
    let outdir = TempDir::new()?;
    let mut args: Vec<String> = vec![
        "-j".to_string(),
        junction_file.to_string(),
        "-o".to_string(),
        outdir.path().to_string_lossy().to_string(),
        "-r".to_string(),
    ];

    for read_file in read_files {
        args.push(read_file.to_string());
    }

    Command::cargo_bin(PRG)?.args(&args).assert().success();

    for (read_file, expected_file) in zip(read_files, expected_files) {
        // Output file is read basename + ".txt"
        let mut read_base = Path::new(&read_file)
            .file_name()
            .ok_or(anyhow!("No basename"))?
            .to_os_string();
        read_base.push(".txt");
        let outpath = &outdir.path().join(&read_base);
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
