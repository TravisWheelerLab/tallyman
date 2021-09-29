![Our mascot](character.png)

![The Count CI](https://github.com/TravisWheelerLab/the_count/workflows/The%20Count%20CI/badge.svg)

# The Count

A tool for counting exact K-mer occurrences in a DNA or RNA sequence very, very
quickly (where K=32).

## Command Line Interface

`the_count <haystack> <needles> <output>` where the haystack is a FASTA file
that contains sequences to be searched and the needles are a FASTA file that
contains 32-mers to be searched for.

A docker image is also provided:
```
docker pull traviswheelerlab/the_count
docker run --mount type=bind,src=$PWD,dst=/data traviswheelerlab/the_count /data/test-rna.fasta /data/human.fa /data/out.out
```

## Developer Tooling

The Count is implemented in the Rust programming language and supports Rust 1.43
and later. Tooling instructions are below. They assume you already have the Rust
toolchain installed. To do this, see <https://rustup.rs>.

  - Run unit tests: `cargo test`
  - Run the demo: `cargo run`
  - Create a release build (faster): `cargo build --release`,
    the binary will end up in `target/release/`
  - Format the code (do this before pushing): `cargo fmt`

To run the benchmarks, you will need to install
[hyperfine](https://github.com/sharkdp/hyperfine). On a Mac this can be done
through Homebrew using `brew install hyperfine`. You can also use the
`setup-mac` make target: `make setup-mac`.

Benchmarks may then be run with `make benchmark`. The default benchmark searches a
file with 1 million auto-generated sequences for 999 auto-generated 32-mers.

## Authors

  - Sarah Walling <sarah.walling@umontana.edu>
  - Travis Wheeler <travis.wheeler@umontana.edu>
  - George Lesica <george.lesica@umontana.edu>

