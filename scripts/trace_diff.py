#!/usr/bin/env python3
"""
Compare two instruction traces (x64dbg-style `idx | RIP | bytes | disasm | …` or mwemu CSV
with columns Index, Address, Bytes, Disassembly).

Find the first step where normalized keys differ. Use image bases so comparison is by RVA
(aligns different load addresses). Optional start RVA `-a` skips to a common entry (e.g.
LdrInitializeThunk / LdrpInitializeProcess) in both files independently.

Typical workflow (sync with mwemu `-a` / `-b`):
  cargo run --release -- -f sample.bin -6 --ssdt --init -vv -T /tmp/mw.txt \
    -b 0x140000000 -a 0x1000
  python3 scripts/trace_diff.py trace_LdrInitializeThunk.txt /tmp/mw.txt \
    -b 0x7FFE68000000 -B 0x180000000 -a 0x8CD0

  -b  base (ImageBase) of the *reference* trace (e.g. ntdll in x64dbg).
  -B  base of the *candidate* trace (e.g. mwemu maps64 ntdll base); defaults to -b if omitted.
  -a  RVA (hex) — first line in each trace with this RVA starts the diff (0 = from top).
"""
from __future__ import annotations

import argparse
import csv
import io
import re
import sys
from dataclasses import dataclass
from typing import Iterator, List, Optional, Tuple

# x64dbg / pipe trace: step | RIP | bytes | disassembly | ...
PIPE_LINE = re.compile(
    r"^\s*(\d+)\s*\|\s*([0-9A-Fa-f]+)\s*\|\s*([^|]*?)\s*\|\s*([^|]*)",
    re.IGNORECASE,
)


def parse_hex(s: str) -> int:
    s = s.strip().lower().replace("_", "")
    if s.startswith("0x"):
        return int(s, 16)
    return int(s, 16)


@dataclass
class Step:
    index: int
    rip: int
    bytes_raw: str
    disasm: str
    raw: str


def parse_pipe_line(line: str) -> Optional[Step]:
    m = PIPE_LINE.match(line)
    if not m:
        return None
    idx_s, rip_s, bytes_s, disasm_s = m.groups()
    return Step(
        index=int(idx_s, 10),
        rip=int(rip_s, 16),
        bytes_raw=bytes_s.strip(),
        disasm=disasm_s.strip(),
        raw=line.rstrip("\n"),
    )


def parse_csv_line(line: str) -> Optional[Step]:
    """mwemu header: "Index","Address","Bytes","Disassembly",..."""
    line = line.strip()
    if not line.startswith('"'):
        return None
    try:
        r = next(csv.reader(io.StringIO(line)))
    except csv.Error:
        return None
    if len(r) < 4:
        return None
    if r[0].lower() == "index":
        return None
    try:
        idx = int(r[0], 10)
        a = r[1].strip().strip('"')
        rip = int(a, 16) if a.lower().startswith("0x") else int(a, 16)
        b = r[2].strip()
        d = r[3].strip()
        return Step(index=idx, rip=rip, bytes_raw=b, disasm=d, raw=line)
    except (ValueError, IndexError):
        return None


def sniff_parser(first_lines: List[str]):
    for ln in first_lines:
        if parse_csv_line(ln):
            return "csv"
        if parse_pipe_line(ln):
            return "pipe"
    return "pipe"


def head_lines(path: str, n: int) -> List[str]:
    out: List[str] = []
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        for _ in range(n):
            try:
                out.append(next(f))
            except StopIteration:
                break
    return out


def iter_steps(path: str, fmt: str, max_lines: Optional[int]) -> Iterator[Step]:
    n = 0
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        for line in f:
            if max_lines is not None and n >= max_lines:
                break
            if fmt == "csv":
                s = parse_csv_line(line)
            else:
                s = parse_pipe_line(line)
            if s is None:
                continue
            n += 1
            yield s


def rva(rip: int, base: int) -> int:
    return (rip - base) & 0xFFFFFFFFFFFFFFFF


def normalize_bytes(s: str) -> str:
    # "40:53" / "[40, 53]" / "4053" -> 4053
    t = re.sub(r"[^0-9a-fA-F]", "", s)
    return t.lower()


def mnemonic(disasm: str) -> str:
    d = disasm.strip().lower()
    if not d:
        return ""
    # strip prefixes like "rep "
    return d.split()[0]


def compare_key(
    step: Step,
    base: int,
    syscall_only: bool,
) -> Optional[Tuple]:
    if syscall_only and "syscall" not in step.disasm.lower():
        return None
    rv = rva(step.rip, base)
    b = normalize_bytes(step.bytes_raw)
    m = mnemonic(step.disasm)
    if "syscall" in step.disasm.lower():
        return ("syscall", rv)
    return (rv, b, m)


def find_start_rva(
    path: str,
    fmt: str,
    base: int,
    target_rva: Optional[int],
    max_lines: Optional[int],
) -> int:
    """Return offset (0-based count of *parsed* steps) to first step with rva == target."""
    if target_rva is None:
        return 0
    want = target_rva & 0xFFFFFFFFFFFFFFFF
    off = 0
    for step in iter_steps(path, fmt, max_lines):
        if rva(step.rip, base) == want:
            return off
        off += 1
    print(
        f"warning: entry RVA 0x{want:x} not found in {path!r}; starting from 0",
        file=sys.stderr,
    )
    return 0


def load_all(path: str, fmt: str, max_lines: Optional[int]) -> List[Step]:
    return list(iter_steps(path, fmt, max_lines))


def main() -> int:
    ap = argparse.ArgumentParser(
        description="Diff two execution traces by RVA (+ bytes + mnemonic), with per-trace bases.",
    )
    ap.add_argument("reference", help="Reference trace (e.g. trace_LdrInitializeThunk.txt)")
    ap.add_argument("candidate", help="mwemu or second trace")
    ap.add_argument(
        "-b",
        "--base-ref",
        required=True,
        help="Image base for reference trace (hex), e.g. 0x7FFE68000000",
    )
    ap.add_argument(
        "-B",
        "--base-mine",
        default=None,
        help="Image base for candidate trace (hex). Default: same as -b",
    )
    ap.add_argument(
        "-a",
        "--entry",
        default=None,
        help="Start at this RVA (hex) in *both* traces (first matching RIP per file)",
    )
    ap.add_argument(
        "--max-lines",
        type=int,
        default=None,
        help="Stop after this many parsed steps per file (huge traces)",
    )
    ap.add_argument(
        "--syscall-only",
        action="store_true",
        help="Only compare lines whose disassembly contains 'syscall'",
    )
    args = ap.parse_args()

    base_ref = parse_hex(args.base_ref)
    base_mine = parse_hex(args.base_mine) if args.base_mine else base_ref
    entry_rva = parse_hex(args.entry) if args.entry else None

    head_ref = head_lines(args.reference, 24)
    head_mine = head_lines(args.candidate, 24)

    fmt_ref = sniff_parser(head_ref)
    fmt_mine = sniff_parser(head_mine)

    sr = find_start_rva(args.reference, fmt_ref, base_ref, entry_rva, args.max_lines)
    sm = find_start_rva(args.candidate, fmt_mine, base_mine, entry_rva, args.max_lines)

    steps_ref = load_all(args.reference, fmt_ref, args.max_lines)
    steps_mine = load_all(args.candidate, fmt_mine, args.max_lines)

    steps_ref = steps_ref[sr:]
    steps_mine = steps_mine[sm:]

    print(
        f"reference: format={fmt_ref} base=0x{base_ref:x} start_offset={sr} steps={len(steps_ref)}",
        file=sys.stderr,
    )
    print(
        f"candidate: format={fmt_mine} base=0x{base_mine:x} start_offset={sm} steps={len(steps_mine)}",
        file=sys.stderr,
    )
    if entry_rva is not None:
        print(f"entry_rva=0x{entry_rva:x}", file=sys.stderr)

    i, j = 0, 0
    compared = 0
    while i < len(steps_ref) and j < len(steps_mine):
        kr = compare_key(steps_ref[i], base_ref, args.syscall_only)
        km = compare_key(steps_mine[j], base_mine, args.syscall_only)
        if kr is None:
            i += 1
            continue
        if km is None:
            j += 1
            continue
        compared += 1
        if kr != km:
            print("First difference:", file=sys.stderr)
            print(
                f"  ref[{sr + i}] rva=0x{rva(steps_ref[i].rip, base_ref):x} key={kr!r}\n"
                f"    {steps_ref[i].raw[:200]}",
                file=sys.stderr,
            )
            print(
                f"  mine[{sm + j}] rva=0x{rva(steps_mine[j].rip, base_mine):x} key={km!r}\n"
                f"    {steps_mine[j].raw[:200]}",
                file=sys.stderr,
            )
            print(f"compared_steps={compared}")
            return 1
        i += 1
        j += 1

    if i < len(steps_ref) or j < len(steps_mine):
        print(
            "Traces ended aligned but one file has extra steps "
            f"(ref left={len(steps_ref)-i}, mine left={len(steps_mine)-j})",
            file=sys.stderr,
        )
        return 1

    print(f"OK: {compared} compared steps match (within mode).", file=sys.stderr)
    print(f"compared_steps={compared}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
