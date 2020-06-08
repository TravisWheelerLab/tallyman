.PHONY: benchmark
benchmark: fixtures
	cargo build --release
	hyperfine --warmup 1 'target/release/the_count fixtures/test-rna.fasta fixtures/test-dna.fasta fixtures/test-output.txt'

.PHONY: fixtures
fixtures: fixtures/test-rna.fasta fixtures/test-dna.fasta

.PHONY: setup-mac
setup-mac:
	brew install hyperfine

fixtures/%.fasta: fixtures/%.tar.xz
	tar -C fixtures -x -f $<
