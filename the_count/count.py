import subprocess, shlex, argparse, sys

def file_len(fname):
    #run a subprocess for "grep -o '>' DCEs.fasta | wc -l" to get number of sequences in fasta file
    command_line = "grep -o '>' " + fname
    args = shlex.split(command_line)
    p = subprocess.Popen(args, stdout=subprocess.PIPE)
    wc = subprocess.Popen(['wc', '-l'], stdin=p.stdout, stdout=subprocess.PIPE,)
    end_of_pipe = wc.communicate()
    return int(end_of_pipe[0])

def make_results_hash(DCE, hits):
    hitcmp = {}
    for item in DCE:
        if isinstance(DCE[item], list): #check to see if there are multiple DCE names for this sequence
            for seq in DCE[item]: #and if so go through them one by one
                if seq in hits:
                    hitcmp[seq] = hits[seq]
                else:
                    hitcmp[seq] = 0
        else:
            if DCE[item] in hits:
                hitcmp[DCE[item]] = hits[DCE[item]]
            else:
                hitcmp[DCE[item]] = 0
    return hitcmp

def write_results(hits, filename):
    outF = open(filename, "w")
    outF.write("DCE_ID\tHits\n")
    for item in hits:
        outF.write(item)
        outF.write("\t")
        outF.write(str(hits[item]))
        outF.write("\n")
    outF.close()

def read_DNA(filename):
    DCE = {}
    curr_seq = ""
    length = 0
    f = open(filename, 'r')
    while 1:
        lines = f.readlines()
        if not lines:
            break
        for line in lines:
            if '>' not in line:
                if line in DCE:
                    names = []
                    if isinstance(DCE[line.rstrip("\n").upper()], list): #check to see if there are already multiple sequence names
                        for seq in DCE[line.rstrip("\n").upper()]: #and iterate through each to append to the new list
                            names.append(seq)
                        names.append(curr_seq)
                        DCE[line] = names
                    else:   #there is only one existing sequence name - start a list with that and the new name
                        names.append(DCE[line.rstrip("\n").upper()])
                        names.append(curr_seq)
                        DCE[line.rstrip("\n").upper()] = names
                else:   #it doesn't already exist - put it in as-is
                    DCE[line.rstrip("\n").upper()] = curr_seq
                length = len(line)
            else:
                curr_seq = line.rstrip("\n")
    f.close()
    return DCE, length

def read_RNA(filename, length, DCE):
    n = file_len(filename)
    hits = {}
    count = 0
    loc = 0
    fg = open(filename, 'r')
    seq = ""
    while 1:
        lines = fg.readlines(100000)
        if not lines:
            break
        for line in lines:
            if '>' not in line:
                # TODO: Is there a better way to achieve this? This change (from original approach that assumed the sequence was only on 1 line) made it about 10x slower
                seq = seq + line.rstrip("\n").upper()
            else:  # This is a sequence name line - can stop concatenating the previous sequence and search for its 32mers
                # and search on the fully concatenated sequence chunk
                for i in range(0, len(seq) - (length - 1)):
                    if (seq[i:length + i]) in DCE:
                        if isinstance(DCE[seq[i:length + i]], list): #check for multiple sequence names
                            for item in DCE[seq[i:length + i]]:
                                if item in hits: #and put each sequence name in hits SEPARATELY, with the shared hit count
                                    hits[item] = hits[item] + 1
                                else:
                                    hits[item] = 1
                        else:
                            if DCE[seq[i:length + i]] in hits:
                                hits[DCE[seq[i:length + i]]] = hits[DCE[seq[i:length + i]]] + 1
                            else:
                                hits[DCE[seq[i:length + i]]] = 1

                    # now do revcomp search
                    reverse_complement = seq[i:length + i].translate(str.maketrans('ACGT', 'TGCA'))[::-1]
                    if (reverse_complement) in DCE:
                        if isinstance(DCE[reverse_complement], list): #check for multiple seq names
                            for item in DCE[reverse_complement]:
                                if item in hits: #and put each in hits as their own entry, with the shared hit count
                                    hits[item] = hits[item] + 1
                                else:
                                    hits[item] = 1
                        else:
                            if DCE[reverse_complement] in hits:
                                hits[DCE[reverse_complement]] = hits[DCE[reverse_complement]] + 1
                            else:
                                hits[DCE[reverse_complement]] = 1

                loc = line.split(' ')  # will need location information at some point
                loc = loc[0]
                seq = ""
                check = int(n / 100)
                count = count + 1
                if (count % check == 0):
                    progress = count / n
                    # print("Progress: {:.1%}".format(progress))
    print("Count hits:", hits)
    fg.close()
    return hits

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Count number of exact matches of short sequences in RNAseq data")
    parser.add_argument("--DNA", default="fixtures/DCEs.fasta")
    parser.add_argument("--RNA", default="fixtures/RNAseqs.fasta")
    parser.add_argument("--out", default="out.txt")
    args = parser.parse_args()

    DCE, length = read_DNA(args.DNA)
    print("DCEs read into hash of size", sys.getsizeof(DCE), "bytes")
    print("Kmer length is", length, "nt")

    hits = read_RNA(args.RNA, length, DCE)

    write_results(hits, args.out)