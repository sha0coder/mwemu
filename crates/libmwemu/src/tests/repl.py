import sys

code = open(sys.argv[1]).read().strip().split('\n')
kw = sys.argv[2]
rep = sys.argv[3]

for i in range(len(code)):
    if kw in code[i]:
        code[i] = rep

open(sys.argv[1],'w').write('\n'.join(code))

