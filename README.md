
# Tallyman

image::images/banana.jpeg[A Banana]

A tool for counting exact K-mer occurrences in a DNA or RNA sequence very, very
quickly (where K=32).

## Command Line Interface

`tallyman --rna <haystack> --dna <needles> -o <output>` 

* haystack is a FASTX file of sequences to be searched 
* needles are a FASTX file of 32-mers to be searched for

## Developer Tooling

Tallyman is implemented in the Rust programming language.
Tooling instructions are below. 
They assume you already have the Rust toolchain installed. 
To do this, see <https://rustup.rs>.

* Run unit tests: `cargo test`
* Run the demo: `cargo run`
* Create a release build (faster): `cargo build --release`, the binary will end up in `target/release/`
* Format the code (do this before pushing): `cargo fmt`

To run the benchmarks, you will need to install [hyperfine](https://github.com/sharkdp/hyperfine). 
On a Mac this can be done through Homebrew using `brew install hyperfine`. 
You can also use the `setup-mac` make target: `make setup-mac`.

Benchmarks may then be run with `make benchmark`. 
The default benchmark searches a file with 1 million auto-generated sequences for 999 auto-generated 32-mers.

## Authors

* Sarah Walling <sarah.walling@umontana.edu>
* Travis Wheeler <twheeler@arizona.edu>
* George Lesica <george.lesica@umontana.edu>
* Ken Youens-Clark <kyclark@arizona.edu>
