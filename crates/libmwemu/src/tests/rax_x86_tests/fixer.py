log = open("log").read().strip().split("\n")


path = ""

for line in log:
    if "-->" in line:
        off = line.find("rax_x86_tests/")
        if off == -1:
            continue
        off += 14
        spl = line[off:].split(":")
        path = spl[0]
        num = int(spl[1])
        print(f" {path} {num}")
        code = open(path).read().split("\n")
        if "mut emu =" not in code[num - 1]:
            continue
        code[num - 1] = code[num - 1].replace("mut emu =", "emu =")
        open(path, "w").write("\n".join(code))
        print("fixed.")
