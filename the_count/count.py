DCEs = {}


with open("../fixtures/DCEs.fasta", 'r') as f:
    data = f.read()

for line in data:
    line.rstrip("\n")
    #if '>' in line:
