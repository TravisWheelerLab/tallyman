import subprocess, shlex, argparse, sys

def file_len(fname):
    #run a subprocess for "grep -o '>' DCEs.fasta | wc -l" to get number of sequences in fasta file
    command_line = "grep -o '>' " + fname
    args = shlex.split(command_line)
    p = subprocess.Popen(args, stdout=subprocess.PIPE)
    wc = subprocess.Popen(['wc', '-l'], stdin=p.stdout, stdout=subprocess.PIPE,)
    end_of_pipe = wc.communicate()
    return int(end_of_pipe[0])

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Count number of exact matches of short sequences in RNAseq data")
    parser.add_argument("--DNA", default="fixtures/DCEs.fasta")
    parser.add_argument("--RNA", default="fixtures/RNAseqs.fasta")
    args = parser.parse_args()

    n = file_len(args.DNA)
    DCE = {}
    curr_seq = ""
    hits = {}
    length = 0

    with open(args.DNA, 'r') as f:
        Lines = f.readlines()
        for line in Lines:
            line.rstrip("\n")
            if '>' not in line:
                DCE[line.rstrip("\n").upper()] = curr_seq
                length = len(line)
            else:
                curr_seq = line.rstrip("\n")
    f.close()
    print("DCEs read into hash of size", sys.getsizeof(DCE), "bytes")
    print("Kmer length is", length, "nt")

    with open(args.RNA, 'r') as fg:
        seq = ""
        Lines = fg.readlines()
        for line in Lines:
            line.rstrip("\n")
            if '>' not in line:
                seq = seq + line.rstrip("\n").upper()
            else:
                name = ''
                for i in range(0, len(seq) - (length - 1)):
                    if (seq[i:length + i]) in DCE:
                        if DCE[(seq[i:length + i])] in hits:
                            hits[DCE[(seq[i:length + i])]] = hits[DCE[(seq[i:length + i])]] + 1
                        else:
                            hits[DCE[(seq[i:length + i])]] = 1
    fg.close()


    for item in hits:
        print(hits[item], "hit(s) for", item)
