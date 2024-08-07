FIXTURES := fixtures/test-rna.fasta fixtures/test-dna.fasta
REL_EXE := target/release/tallyman
RNA50 = tests/inputs/rna-50k.fasta
RNA100 = tests/inputs/rna-100k.fasta
DNA_FA = tests/inputs/dna.fasta
DNA_FQ = tests/inputs/dna.fastq
DNA_DUP = tests/inputs/dups.fa
DNA_SHORT = tests/inputs/too_short.fa
BIG_DNA = fixtures/test-dna.fasta
BIG_RNA = fixtures/test-rna.fasta

fitty:
	cargo run -- -v -r $(RNA50) -d $(DNA_FA) -o -

dups:
	cargo run -- -v -r $(RNA50) -d $(DNA_DUP) -o -

short:
	cargo run -- -v -r $(RNA50) -d $(DNA_SHORT) -o -

hunna:
	cargo run -- -v -r $(RNA100) -d $(DNA_FA) -o -

big:
	cargo run -- -v -r $(BIG_RNA) -d $(BIG_DNA) -o /dev/null

io:
	cargo instruments -t io --release -- -r $(BIG_RNA) -d $(BIG_DNA) -o /dev/null

alloc:
	cargo instruments -t Allocations --release -- -r $(BIG_RNA) -d $(BIG_DNA) -o /dev/null

time:
	cargo instruments -t time --release -- -r $(BIG_RNA) -d $(BIG_DNA) -o /dev/null

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
