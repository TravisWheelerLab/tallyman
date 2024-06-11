FIXTURES := fixtures/test-rna.fasta fixtures/test-dna.fasta
REL_EXE := target/release/the_count
RNA50 = tests/inputs/rna-50k.fasta
DNA_FA = tests/inputs/dna.fasta
DNA_FQ = tests/inputs/dna.fastq
BIG_DNA = fixtures/test-dna.fasta
BIG_RNA = fixtures/test-rna.fasta

fitty:
	cargo run -- -r $(RNA50) -d $(DNA_FA) -o -

big:
	cargo run -- -v -r $(BIG_RNA) -d $(BIG_DNA)

.PHONY: benchmark
benchmark: fixtures
	cargo build --release
	hyperfine --warmup 1 '$(REL_EXE) -r fixtures/test-rna.fasta -d fixtures/test-dna.fasta -o /dev/null'

.PHONY: cachegrind
cachegrind: fixtures
	valgrind --tool=cachegrind ./$(REL_EXE) -r fixtures/test-rna.fasta -d fixtures/test-dna.fasta -o /dev/null

.PHONY: fixtures
fixtures: $(FIXTURES)

.PHONY: setup-mac
setup-mac:
	brew install hyperfine
	# This is a Mac-compatible fork
	brew tap LouisBrunner/valgrind
	brew install --HEAD LouisBrunner/valgrind/valgrind

fixtures/%.fasta: fixtures/%.tar.xz
	tar -C fixtures -x -f $<
	touch $@
