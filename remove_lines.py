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
    parser = argparse.ArgumentParser(description="Remove lines from RNAseq data")
    parser.add_argument("--RNA", default="SRR1099957.part-4.fasta")
    parser.add_argument("--out", default="SRR1099957.4.test.fasta")
    args = parser.parse_args()
    seq = ""

    outF = open(args.out, "w")

    fg = open(args.RNA, 'r')
    line = fg.readline()
    outF.write(line)
    seq = ""
    while 1:
        lines = fg.readlines(100000)
        if not lines:
            break
        for line in lines:
            if '>' not in line:
                seq = seq + line.rstrip("\n").upper()
            else:
                outF.write(seq)
                outF.write("\n")
                outF.write(line)
                seq = ""
    fg.close()
    outF.close()
