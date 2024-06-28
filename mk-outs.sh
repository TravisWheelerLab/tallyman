#!/usr/bin/env bash

set -u

PRG="cargo run --"
DNA_FA="tests/inputs/dna.fasta"

RNA_FA_50K="tests/inputs/rna-50k.fasta"
RNA_FA_100K="tests/inputs/rna-100k.fasta"

RNA_FQ_50K="tests/inputs/rna-50k.fastq"
RNA_FQ_100K="tests/inputs/rna-100k.fastq"

OUT_FA_50K="tests/outputs/out-50k-fasta.txt"
OUT_FA_100K="tests/outputs/out-100k-fasta.txt"

OUT_FQ_50K="tests/outputs/out-50k-fastq.txt"
OUT_FQ_100K="tests/outputs/out-100k-fastq.txt"

OUT_DIR="tests/outputs"

$PRG -j $DNA_FA -r $RNA_FA_50K  -o $OUT_DIR
$PRG -j $DNA_FA -r $RNA_FA_100K -o $OUT_DIR

$PRG -j $DNA_FA -r $RNA_FQ_50K  -o $OUT_DIR
$PRG -j $DNA_FA -r $RNA_FQ_100K -o $OUT_DIR
