import subprocess, shlex, argparse

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
    p = 0.05  # false positive probability
    DCE = {}
    RNA = []
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

    with open(args.RNA, 'r') as fg:
        Lines = fg.readlines()
        for line in Lines:
            line.rstrip("\n")
            if '>' not in line:
                RNA.append(line.rstrip("\n").upper())
    fg.close()

    print("Length is:", length)
    for seq in RNA:
        count = 0
        name = ''
        #print(seq)
        for i in range(0, len(seq)-(length-1)):
            count = count + 1
            RNA32 = (seq[i:length+i])
            #print(RNA32)
            if RNA32 in DCE:
                name = DCE[RNA32]
                if name in hits:
                    hits[name] = hits[name]+1
                else:
                    hits[name] = 1
    for item in hits:
        print(hits[item], "hit(s) for", item)
