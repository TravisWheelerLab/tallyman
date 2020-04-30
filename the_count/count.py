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
    parser.add_argument("--out", default="out.txt")
    args = parser.parse_args()

    n = file_len(args.RNA)
    DCE = {}
    curr_seq = ""
    hits = {}
    length = 0
    count = 0
    loc = 0

    f = open(args.DNA, 'r')
    while 1:
        lines = f.readlines(100000)
        if not lines:
            break
        for line in lines:
            if '>' not in line:
                DCE[line.rstrip("\n").upper()] = curr_seq
                length = len(line)
            else:
                curr_seq = line.rstrip("\n")

    f.close()
    print("DCEs read into hash of size", sys.getsizeof(DCE), "bytes")
    print("Kmer length is", length, "nt")

    fg = open(args.RNA, 'r')
    seq = ""
    while 1:
        lines = fg.readlines(100000)
        if not lines:
            break
        for line in lines:
            if '>' not in line:
                #TODO: Is there a better way to achieve this? This change (from original approach that assumed the sequence was only on 1 line) made it about 10x slower
                seq = seq + line.rstrip("\n").upper()
            else: #This is a sequence name line - can stop concatenating the previous sequence and search for its 32mers
                #First, grab location information - will need to report this at some point
                loc = line.split(' ')
                loc = loc[0]

                #Then search on the fully concatenated sequence chunk
                for i in range(0, len(seq) - (length - 1)):
                    if (seq[i:length + i]) in DCE:
                        if DCE[(seq[i:length + i])] in hits:
                            hits[DCE[(seq[i:length + i])]] = hits[DCE[(seq[i:length + i])]] + 1
                        else:
                            hits[DCE[(seq[i:length + i])]] = 1
                    #now do revcomp search
                    reverse_complement = seq[i:length + i].translate(str.maketrans('ACGT', 'TGCA'))[::-1]
                    if(reverse_complement) in DCE:
                        if reverse_complement in hits:
                            hits[reverse_complement] = hits[reverse_complement] + 1
                        else:
                            hits[reverse_complement] = 1
                seq = ""
                check = int(n/100)
                count = count + 1
                if(count % check == 0):
                    progress = count / n
                    print("Progress: {:.1%}".format(progress))
    fg.close()

    outF = open(args.out, "w")
    outF.write("DCE_ID\tHits\n")
    for item in hits:
        outF.write(item)
        outF.write("\t")
        outF.write(str(hits[item]))
        outF.write("\n")