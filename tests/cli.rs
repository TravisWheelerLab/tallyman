use anyhow::Result;
//use pretty_assertions::assert_eq;
use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::{fs, path::Path};
use tempfile::NamedTempFile;

const PRG: &str = "the_count";
const DNA_FA1: &str = "tests/inputs/human.fa";
const DNA_FA2: &str = "tests/inputs/human.fasta";
const DNA_FQ1: &str = "tests/inputs/human.fq";
const DNA_FQ2: &str = "tests/inputs/human.fastq";
const RNA_FA1: &str = "tests/inputs/rna1.fasta";
const OUT1: &str = "tests/outputs/out1.txt";

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
        .args(["-r", &bad, "-d", DNA_FA1])
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
        .args(["-d", &bad, "-r", RNA_FA1])
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

// --------------------------------------------------
#[test]
fn runs1() -> Result<()> {
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();
    Command::cargo_bin(PRG)?
        .args(["-d", DNA_FA1, "-r", RNA_FA1, "-o", &outpath])
        .assert()
        .success();

    assert!(Path::new(outpath).exists());

    let expected = fs::read_to_string(OUT1)?;
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);
    Ok(())
}
