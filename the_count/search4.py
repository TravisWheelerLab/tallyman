import subprocess, shlex, argparse, sys

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Double check 4-s-l numbers")
    parser.add_argument("--RNA", default="/home/sarah/RNAseq/SRR1099957.4.test.fasta")
    parser.add_argument("--out", default="4-S-l.txt")
    args = parser.parse_args()
    count = 0
    length = 32
    loc = ""
    outF = open(args.out, "w")
    hit = False
    DCE = {}
    forCount = 0
    revCount = 0
    forward = "GAAGCCATCAGGACGTTCACGGCCGAGGTGGT"
    reverse = "ACCACCTCGGCCGTGAACGTCCTGATGGCTTC"
    DCE[forward] = "4-S-l-forward"
    DCE[reverse] = "4-S-l-reverse"
    print(args.RNA)
    print(DCE)

    fg = open(args.RNA, 'r')
    seq = ""
    while 1:
        lines = fg.readlines(100000)
        if not lines:
            break
        for line in lines:
            if '>' not in line:
                seq = seq + line.rstrip("\n").upper()
            else:
                for i in range(0, len(seq) - (length - 1)):
                    if seq[i:length + i] == forward:
                        forCount=forCount+1
                        count = count + 1
                        outF.write(loc)
                        outF.write("\t")
                        outF.write(str(i))
                        outF.write("\t")
                        outF.write("Forward\n")
                    if seq[i:length + i] == reverse:
                        revCount=revCount+1
                        count = count + 1
                        outF.write(loc)
                        outF.write("\t")
                        outF.write(str(i))
                        outF.write("\t")
                        outF.write("Reverse\n")
                loc = line.split(' ')
                loc = loc[0]
                seq = ""
    fg.close()
    outF.write("Forward count: ")
    outF.write(str(forCount))
    outF.write("Reverse count: ")
    outF.write(str(revCount))
    outF.close()
    print("Forward count:", forCount)
    print("Reverse count:", revCount)