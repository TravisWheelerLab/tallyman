#!/usr/bin/env python3
"""
Author : Ken Youens-Clark <kyclark@gmail.com>
Date   : 2024-06-10
Purpose: Make FASTQ from FASTA
"""

import argparse
import os
from Bio import SeqIO
from typing import NamedTuple, TextIO


class Args(NamedTuple):
    """Command-line arguments"""

    file: TextIO
    out_file: TextIO


# --------------------------------------------------
def get_args() -> Args:
    """Get command-line arguments"""

    parser = argparse.ArgumentParser(
        description="Make FASTQ from FASTA",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "file",
        help="Input FASTA file",
        metavar="FASTA",
        type=argparse.FileType("rt"),
    )

    parser.add_argument(
        "-o",
        "--out-file",
        help="Output FASTQ file",
        metavar="FASTQ",
        type=argparse.FileType("wt"),
    )

    args = parser.parse_args()

    if not args.out_file:
        basename = os.path.splitext(os.path.basename(args.file.name))[0]
        args.out_file = open(f"{basename}.fastq", "wt", encoding="UTF-8")

    return Args(file=args.file, out_file=args.out_file)


# --------------------------------------------------
def main() -> None:
    """Make a jazz noise here"""

    args = get_args()

    for rec in SeqIO.parse(args.file, "fasta"):
        print(
            "@{}\n{}\n+\n{}".format(rec.id, rec.seq, "A" * len(rec.seq)),
            file=args.out_file,
        )

    print(f'Done, see output file "{args.out_file.name}"')


# --------------------------------------------------
if __name__ == "__main__":
    main()
