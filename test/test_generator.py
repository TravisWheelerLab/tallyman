import sys
sys.path.insert(1, '../the_count/')
import the_count.count
import subprocess, shlex, argparse, random


def check_grep(filename, seq):
    #run a subprocess for "grep -o '[seq]' [filename] | wc -l" to get number of hits in fasta file
    command_line = "grep -o '" + seq + "' " + filename
    args = shlex.split(command_line)
    p = subprocess.Popen(args, stdout=subprocess.PIPE)
    wc = subprocess.Popen(['wc', '-l'], stdin=p.stdout, stdout=subprocess.PIPE,)
    end_of_pipe = wc.communicate()
    return int(end_of_pipe[0])

def gen_DNA(N, seq_inds, length, outF):
    DCE = {}
    in_seqs = {}
    for i in range(0,N):
        seq = ''.join(random.choice('CGTA') for _ in range(length))
        name = ">testSeq" + str(i+1)
        DCE[seq] = name
        if(seq_inds):
            if (seq_inds[0] == i):
                seq_inds.pop(0)
                in_seqs[seq] = name
            #write to DNA file here - that way we can just call count functions on the files to compare results
            outF.write(name)
            outF.write("\n")
            outF.write(seq)
            outF.write("\n")
    return DCE, in_seqs

def gen_RNA(number, lengths, inserts, reverse_ind, indices, outF):
    insert_seqs = list(inserts)
    for i in range(0,number):
        length = random.choice(lengths)
        seq = ''.join(random.choice('CGTA') for _ in range(length))
        name = ">SRRtest." + str(i + 1)
        if i in indices:
            #print(i)
            in_seq = insert_seqs.pop(0)
            #print(in_seq)
            if in_seq in reverse_ind:
                in_seq = in_seq.translate(str.maketrans('ACGT', 'TGCA'))[::-1]
                #print("Reversed: ", in_seq)
            length = length-32
            spot = random.randint(0, length)
            #print(in_seq, " is inserted into ", name)
            seq = seq[:spot] + in_seq + seq[spot+32:]

        outF.write(name)
        outF.write("\tlength=")
        outF.write(str(length))
        outF.write("\n")
        outF.write(seq)
        outF.write("\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a test file of RNA")
    parser.add_argument("--N", type=int, default=10)
    parser.add_argument("--K", type=int , default=8)
    parser.add_argument("--RNA", type=int, default=100000)
    parser.add_argument("--DNAfile", default="testDNA.fasta")
    parser.add_argument("--RNAfile", default="testRNA.fasta")
    parser.add_argument("--out", default="testhits.txt")
    args = parser.parse_args()
    RNA_lengths = [32, 60, 124, 151] #various lengths using for each RNAseq block, randomly chosen
    kmer_length = 32
    outD = open(args.DNAfile, "w")
    outR = open(args.RNAfile, "w")
    outF = open(args.out, "w")
    testhits = {}
    counthits = {}


    #choose (by index) which RNA sequences will have the known DNA seqs inserted into them
    RNA_inds = random.sample(range(0, args.RNA), args.K)

    #choose (by index) which DNA sequences will be the ones inserted into RNA
    seq_inds = random.sample(range(0, args.N), args.K)
    seq_inds.sort()
    #print(seq_inds)

    #create two different dictionaries that hold the generated DNA: first, the total
    #number of DNA sequences, AND also the ones that are to be inserted in RNAseqs file
    DCE, in_seqs = gen_DNA(args.N, seq_inds, kmer_length, outD)

    #choose which DNA sequences will be inserted as their reverse complement
    half = int(args.K / 2)
    #rev_inds = random.sample(range(0, args.N), half)
    rev_inds = random.sample(list(in_seqs), half)
    #rev_inds.sort()
    #print(rev_inds)

    #now generate the RNAseqs, which inserts the chosen DNA kmers and then writes it all to a file
    gen_RNA(args.RNA, RNA_lengths, in_seqs, rev_inds, RNA_inds, outR)

    outR.close()
    outD.close()
