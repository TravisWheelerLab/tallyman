from bloomfilter import BloomFilter
import subprocess, shlex

def file_len(fname):
    #run a subprocess for "grep -o '>' DCEs.fasta | wc -l" to get number of sequences in fasta file
    command_line = "grep -o '>' ../fixtures/DCEs.fasta"
    args = shlex.split(command_line)
    p = subprocess.Popen(args, stdout=subprocess.PIPE)
    wc = subprocess.Popen(['wc', '-l'], stdin=p.stdout, stdout=subprocess.PIPE,)
    end_of_pipe = wc.communicate()
    return int(end_of_pipe[0])

if __name__ == "__main__":
    filename = '../fixtures/DCEs.fasta'
    RNAfile = '../fixtures/RNAseqs.fasta'
    n = file_len(filename)
    p = 0.05  # false positive probability
    #bloomf = BloomFilter(n, p)
    DCE = {}
    RNA = []
    curr_seq = ""
    hits = {}


    with open(filename, 'r') as f:
        Lines = f.readlines()
        for line in Lines:
            line.rstrip("\n")
            if '>' not in line:
                DCE[line.rstrip("\n").upper()] = curr_seq
            else:
                curr_seq = line.rstrip("\n")
    f.close()

    with open(RNAfile, 'r') as fg:
        Lines = fg.readlines()
        for line in Lines:
            line.rstrip("\n")
            if '>' not in line:
                RNA.append(line.rstrip("\n").upper())
    fg.close()

    length = 32
    for seq in RNA:
        count = 0
        #print(seq)
        for i in range(0, len(seq)-(length-1)):
            count = count + 1
            RNA32 = (seq[i:length+i])
            #print(RNA32)
            if RNA32 in DCE:
                print("Hit for", DCE[RNA32], "in chunk", count)