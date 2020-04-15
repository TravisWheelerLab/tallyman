import argparse

from .fasta_utils import as_sequence_to_name, sequence_count


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Count number of exact matches of short sequences in RNAseq data"
    )
    parser.add_argument("--DNA", default="fixtures/DCEs.fasta")
    parser.add_argument("--RNA", default="fixtures/RNAseqs.fasta")
    args = parser.parse_args()

    # filename = 'fixtures/DCEs.fasta'
    # RNAfile = 'fixtures/RNAseqs.fasta'
    with open(args.DNA, "r") as dna_file:
        n = sequence_count(dna_file)
    p = 0.05  # false positive probability

    with open(args.DNA, "r") as f:
        DCE = as_sequence_to_name(f)

    RNA = []
    curr_seq = ""
    hits = {}

    # TODO: Should we make this a parameter and then raise if we get the wrong length?
    length = 32

    with open(args.RNA, "r") as fg:
        for line in fg:
            line.rstrip("\n")
            if ">" not in line:
                RNA.append(line.rstrip("\n").upper())

    print("Length is:", length)
    for seq in RNA:
        count = 0
        name = ""
        # print(seq)
        for i in range(0, len(seq) - (length - 1)):
            count = count + 1
            RNA32 = seq[i : length + i]
            # print(RNA32)
            if RNA32 in DCE:
                name = DCE[RNA32]
                if name in hits:
                    hits[name] = hits[name] + 1
                else:
                    hits[name] = 1
    for item in hits:
        print(hits[item], "hit(s) for", item)
