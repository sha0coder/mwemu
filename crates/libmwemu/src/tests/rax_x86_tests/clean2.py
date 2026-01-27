"""
x86 test conversor from rax to mwemu
@sha0coder

I created those regex manually to modify all the rax
x86's tests to use mwemu api.
It's not necesary to launch this script again because
the tests where already converted.

"""

import os
import re
import sys

root_dir = sys.argv[1]


class Fix:
    def __init__(self):
        self.data = []
        self._debug = False

    def debug(self):
        self._debug = True

    def no_debug(self):
        self._debug = False

    def write(self, data):
        self.data = data.split("\n")

    def contains(self, data):
        for l in self.data:
            if data in l:
                return True
        return False

    def read(self):
        return "\n".join(self.data)

    def replace_line(self, a, b):
        """
        If a is matched, the whole line is replaced by b.
        for doing partial replacements use sub
        """
        for i in range(len(self.data)):
            if (
                not self.data[i].startswith("// ")
                and not self.data[i].startswith("fn ")
                and not self.data[i].startswith("    // ")
            ):
                if re.search(a, self.data[i]):
                    self.data[i] = b

    def sub(self, a, b):
        """
        Line based sub regex to regex or regex to string.
        since the code is sotred in an array there is no \n bytes.
        """
        for i in range(len(self.data)):
            if (
                not self.data[i].startswith("//")
                and not self.data[i].startswith("fn ")
                and not self.data[i].startswith("    // ")
            ):
                self.data[i] = re.sub(a, b, self.data[i])
                if "\1" in self.data[i] or "\2" in self.data[i] or "\3" in self.data[i]:
                    print(f"/!\\ SOH detected: {a} -> {b}")
                    sys.exit(1)

    def sub_all(self, a, b):
        """
        This puts all the code array in one string with \n
        And apply the sub there.
        """
        tmp = self.read()
        tmp = re.sub(a, b, tmp, flags=re.MULTILINE)
        if "\1" in tmp or "\2" in tmp or "\3" in tmp:
            print(f"/!\\ SOH detected: {a} -> {b}")
        self.write(tmp)

    def add(self, a, b):
        """
        If matches a, then concat b.
        Usually for creating a new line after a.
        """
        for i in range(len(self.data)):
            if not self.data[i].startswith("//"):
                if re.search(a, self.data[i]):
                    self.data[i] += b

    def clean(self, a):
        """
        If matches remove the whole line.
        """
        self.data = [d for d in self.data if not re.search(a, d)]

    def find(self, patt):
        return self.read().find(patt)

    def search(self, patt):
        for d in self.data:
            if re.search(patt, d):
                return True
        return False

    def search_all(self, patt):
        re.search(patt, "\n".join(self.data))


def fix_file(file_path):
    print(f"Fixing {file_path}...")
    fix = Fix()
    with open(file_path, "r") as f:
        fix.write(f.read())

    # 0. Cleanup specific imports and legacy code

    off = fix.find("\ruse")
    if off >= 0:
        print(f"found off: {off} {file_path}")
        sys.exit(1)

    # emu.regs_mut().rsp = () ->
    #

    fix.clean(r"let mut emu = emu64\(\);")
    fix.clean(r"emu\.load_code_bytes\(\&code\);")
    fix.sub_all(
        r"0xf4, // (.*)\n    \];\n",
        r"0xf4, // \1\n    ];\n    let mut emu = emu64();\n    emu.load_code_bytes(&code);\n",
    )
    fix.sub_all(
        r"0xf4\]; //(.*)\n",
        r"0xf4]; //\1\n    let mut emu = emu64();\n    emu.load_code_bytes(&code);\n",
    )
    fix.sub_all(
        r"0xf4, // (.*)\n        \];\n",
        r"0xf4, // \1\n        ];\n        let mut emu = emu64();\n        emu.load_code_bytes(&code);\n",
    )

    with open(file_path, "w") as f:
        f.write(fix.read())


for root, dirs, files in os.walk(root_dir):
    for file in files:
        if file.endswith(".rs"):
            fix_file(os.path.join(root, file))
