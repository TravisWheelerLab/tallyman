![Our mascot](character.png)

![The Count CI](https://github.com/TravisWheelerLab/the_count/workflows/The%20Count%20CI/badge.svg)

# The Count

A tool for counting exact K-mer occurrences in a DNA or RNA sequence.

## Rust Implementation

The Rust version of The Count supports Rust 1.43 and later. Tooling instructions
are below. They assume you already have the Rust toolchain installed. To do
this, see <https://rustup.rs>.

  - Run unit tests: `cargo test`
  - Run the demo: `cargo run`
  - Create a release build (faster): `cargo build --release`,
    the binary will end up in `target/release/`

## The Command Line Interface

## The Algorithms

## The Developer Tooling

This project uses [Poetry](https://python-poetry.org), see its
web site for installation instructions. Poetry handles creating
a virtual environment and fetching dependencies. Note that after
installation you may need to add `"$HOME/.poetry/bin"` to your
`PATH` environment variable.

Some basic Poetry commands are listed below:

  - Fetch dependencies: `poetry update`
  - Launch a shell inside the virtual environment: `poetry shell`
  - Run a command in the virtual environment: `poetry run <your command>`
      - Run unit tests: `poetry run pytest .`
      - Format the code: `poetry run black .`

## The Authors

  - Sarah Walling <sarah.walling@umontana.edu>
  - Travis Wheeler <travis.wheeler@umontana.edu>
  - George Lesica <george.lesica@umontana.edu>
