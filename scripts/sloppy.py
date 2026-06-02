#!/usr/bin/env python3
"""sloppy.py — find suspicious patterns in source code.

Walks a directory tree and scans C, C++, Rust, JS/TS and Python files
for code that smells like copy-paste mistakes or typos. Inspired by
PVS-Studio (Viva64) diagnostics:

  V501  identical sub-expressions around an operator (x == x, x && x, x - x)
  V519  variable assigned twice in a row (x = 5; x = 10;)
  V529  stray ; after if/while/for
  V533  wrong loop var in for (for(i=0; j<n; i++))
  V547  condition always false (x > 10 && x < 5)
  V549  identical adjacent function args (foo(x, x))
  V570  self-assignment (x = x)
  V571  duplicate sub-expression in &&/|| chain (x == 1 || x == 1)
  V583  ?: with identical branches (cond ? a : a)
  V587  broken swap a=b; b=a; (loses the original `a`)

Heuristic-only — expect some false positives. Review every hit.

Usage:
    python3 sloppy.py [path] [--ext .rs .c] [--checks self-cmp,dup-cmp]
    python3 sloppy.py --list-checks
"""

import argparse
import os
import re
import sys
from pathlib import Path

DEFAULT_EXTS = {
    ".c",
    ".h",
    ".cc",
    ".cpp",
    ".cxx",
    ".hpp",
    ".hh",
    ".rs",
    ".mjs",
    ".ts",
    ".py",
}
SKIP_DIRS = {
    ".git",
    "target",
    "node_modules",
    "build",
    "dist",
    "__pycache__",
    ".venv",
    "venv",
    ".mypy_cache",
    ".pytest_cache",
    ".next",
    ".cache",
    "vendor",
}

# An "expression": identifier, optionally chained via . -> or [index].
EXPR = r"[A-Za-z_]\w*(?:(?:\.|->)[A-Za-z_]\w*|\[[^\[\]\n]{1,40}\])*"
# Don't start matching mid-expression — refuse if preceded by `.` `]` `->`.
EXPR_START = r"(?<![\w.\]])(?<!->)"
# Don't accept a match that's just a prefix of a longer expression.
EXPR_END = r"(?![\w.\[])(?!->)"
# Tail: after the right operand, only "expression-closer" tokens are OK
# (otherwise we're capturing only part of a larger expression).
TAIL = r"(?=\s*(?:[);,{}\]]|&&|\|\||$))"


# ---------- Sanitizer: blank out strings and comments ----------


def _rust_char_lit_len(src, i):
    """Length of a Rust char literal starting at i, or 0 (it's a lifetime)."""
    n = len(src)
    if i + 1 >= n:
        return 0
    if src[i + 1] == "\\":
        j = i + 2
        end = min(n, i + 12)
        while j < end and src[j] != "'":
            j += 1
        return j - i + 1 if j < n and src[j] == "'" else 0
    if i + 2 < n and src[i + 2] == "'":
        return 3
    return 0


def sanitize(src, ext):
    """Blank out comments and string literals, preserving newlines/columns."""
    out = []
    n = len(src)
    i = 0
    in_line = in_block = in_str = in_triple = False
    quote = ""
    is_py = ext == ".py"
    is_rs = ext == ".rs"
    has_bt = ext in (".js", ".mjs", ".ts")

    while i < n:
        c = src[i]
        nx = src[i + 1] if i + 1 < n else ""

        if in_line:
            out.append(c if c in "\n\t" else " ")
            if c == "\n":
                in_line = False
            i += 1
            continue
        if in_block:
            if c == "*" and nx == "/":
                out.append("  ")
                in_block = False
                i += 2
                continue
            out.append(c if c in "\n\t" else " ")
            i += 1
            continue
        if in_triple:
            if src[i : i + 3] == quote * 3:
                out.append("   ")
                in_triple = False
                quote = ""
                i += 3
                continue
            out.append(c if c in "\n\t" else " ")
            i += 1
            continue
        if in_str:
            if c == "\\" and nx:
                out.append("  ")
                i += 2
                continue
            if c == quote:
                out.append(c)
                in_str = False
                quote = ""
                i += 1
                continue
            if c == "\n":  # recover from unterminated literal at EOL
                out.append(c)
                in_str = False
                quote = ""
                i += 1
                continue
            out.append(c if c == "\t" else " ")
            i += 1
            continue

        # --- top level ---
        if not is_py and c == "/" and nx == "/":
            in_line = True
            out.append("  ")
            i += 2
            continue
        if not is_py and c == "/" and nx == "*":
            in_block = True
            out.append("  ")
            i += 2
            continue
        if is_py and c == "#":
            in_line = True
            out.append(" ")
            i += 1
            continue
        if is_py and src[i : i + 3] in ('"""', "'''"):
            in_triple = True
            quote = c
            out.append("   ")
            i += 3
            continue
        if is_rs and c == "'":
            ln = _rust_char_lit_len(src, i)
            if ln:
                # Rust byte literal `b'X'`: also blank the preceding `b` so
                # `b'f'` and `b'h'` don't collapse to identical tokens.
                if (
                    i > 0
                    and src[i - 1] == "b"
                    and (i < 2 or not (src[i - 2].isalnum() or src[i - 2] == "_"))
                ):
                    if out and out[-1] == "b":
                        out[-1] = " "
                out.append(" " * ln)
                i += ln
            else:  # lifetime, pass through
                out.append(c)
                i += 1
            continue
        if is_rs and c == "b" and nx == '"':
            # Byte string `b"..."`: blank the `b` prefix.
            out.append(" ")
            i += 1
            continue
        if has_bt and c == "`":
            in_str = True
            quote = "`"
            out.append(c)
            i += 1
            continue
        if c in ('"', "'"):
            in_str = True
            quote = c
            out.append(c)
            i += 1
            continue
        out.append(c)
        i += 1
    return "".join(out)


# ---------- Checks ----------

CHECKS = {}


def check(name, desc, scope="line"):
    """`scope="line"`: fn(line, ext) -> [(col, snippet), ...].
    `scope="file"`: fn(lines, ext) -> [(line_no, col, snippet), ...]."""

    def deco(fn):
        CHECKS[name] = (desc, fn, scope)
        return fn

    return deco


def _bitwise_before(line, pos):
    """True if `line[:pos]` ends with a bitwise op (& | ^ << >>) — likely
    a mask check like `flags & MASK == MASK`, not a self-comparison."""
    before = line[:pos].rstrip()
    if not before:
        return False
    c = before[-1]
    if c in "&|":
        return not (len(before) >= 2 and before[-2] == c)  # exclude && ||
    if c == "^":
        return True
    if before.endswith("<<") or before.endswith(">>"):
        return True
    return False


@check("self-cmp", "x op x  (op: == != < > <= >=) — always true/false")
def _self_cmp(line, ext):
    pat = re.compile(
        rf"{EXPR_START}({EXPR}){EXPR_END}\s*(==|!=|<=|>=|<|>)\s*\1{EXPR_END}{TAIL}"
    )
    out = []
    for m in pat.finditer(line):
        if _bitwise_before(line, m.start()):
            continue
        out.append((m.start(), m.group(0)))
    return out


@check("self-logic", "x && x  or  x || x — redundant")
def _self_logic(line, ext):
    pat = re.compile(
        rf"{EXPR_START}({EXPR}){EXPR_END}\s*(&&|\|\|)\s*\1{EXPR_END}{TAIL}"
    )
    return [(m.start(), m.group(0)) for m in pat.finditer(line)]


@check("self-arith", "x op x  (op: - / ^ & |) — likely typo")
def _self_arith(line, ext):
    # `-` `/` `^` `&` are unambiguous.
    pat_other = re.compile(
        rf"{EXPR_START}({EXPR}){EXPR_END}\s*(-|/|\^|&(?!&))\s*\1{EXPR_END}"
    )
    out = [(m.start(), m.group(0)) for m in pat_other.finditer(line)]
    # `|` is overloaded as closure delimiter in Rust (`.map(|x| x.foo)`) and
    # as match alternation (`Some(x) | None`). Skip for Rust if any `|`
    # appeared on the line — likely a closure open.
    pat_or = re.compile(rf"{EXPR_START}({EXPR}){EXPR_END}\s+\|(?!\|)\s+\1{EXPR_END}")
    for m in pat_or.finditer(line):
        if ext == ".rs" and "|" in line[: m.start()]:
            continue
        out.append((m.start(), m.group(0)))
    return out


@check("self-assign", "x = x;  — no-op")
def _self_assign(line, ext):
    pat = re.compile(rf"^\s*({EXPR})\s*=\s*\1\s*;")
    m = pat.match(line)
    return [(m.start(), m.group(0))] if m else []


@check("empty-ctrl", "if/while/for followed by ;  — empty body")
def _empty_ctrl(line, ext):
    if ext == ".py":
        return []
    pat = re.compile(r"\b(if|while|for)\s*\((?:[^()]|\([^()]*\))*\)\s*;")
    out = []
    for m in pat.finditer(line):
        # skip   } while (cond);   end of do-while
        before = line[: m.start()].rstrip()
        if m.group(1) == "while" and before.endswith("}"):
            continue
        out.append((m.start(), m.group(0)))
    return out


@check("dup-cmp", "duplicate comparison in &&/|| chain (x==1 || x==1)")
def _dup_cmp(line, ext):
    pat = re.compile(
        rf"{EXPR_START}({EXPR}\s*(?:==|!=|<=|>=|<|>)\s*(?:{EXPR}|-?\d+))"
        rf"\s*(?:&&|\|\|)\s*\1{EXPR_END}"
    )
    return [(m.start(), m.group(0)) for m in pat.finditer(line)]


@check("ternary-same", "cond ? a : a  — identical branches")
def _ternary_same(line, ext):
    pat = re.compile(rf"\?\s*({EXPR}){EXPR_END}\s*:\s*\1{EXPR_END}\s*[;,)]")
    return [(m.start(), m.group(0)) for m in pat.finditer(line)]


@check("dead-range", "numeric range that can never be true")
def _dead_range(line, ext):
    out = []
    P = [
        (rf"\b({EXPR})\s*>\s*(-?\d+)\s*&&\s*\1\s*<\s*(-?\d+)\b", lambda a, b: a >= b),
        (rf"\b({EXPR})\s*<\s*(-?\d+)\s*&&\s*\1\s*>\s*(-?\d+)\b", lambda a, b: a <= b),
        (rf"\b({EXPR})\s*>=\s*(-?\d+)\s*&&\s*\1\s*<=\s*(-?\d+)\b", lambda a, b: a > b),
        (rf"\b({EXPR})\s*<=\s*(-?\d+)\s*&&\s*\1\s*>=\s*(-?\d+)\b", lambda a, b: a < b),
    ]
    for src_pat, bad in P:
        for m in re.finditer(src_pat, line):
            if bad(int(m.group(2)), int(m.group(3))):
                out.append((m.start(), m.group(0)))
    return out


C_LIKE = {".c", ".h", ".cc", ".cpp", ".cxx", ".hpp", ".hh", ".js", ".mjs", ".ts"}


@check("loop-vars", "for-loop with mismatched init/cond/inc variable (V533/V534)")
def _loop_vars(line, ext):
    if ext not in C_LIKE:  # Rust/Python don't have C-style for
        return []
    pat = re.compile(r"\bfor\s*\(\s*([^;]*);\s*([^;]*);\s*([^)]*)\)")
    out = []
    for m in pat.finditer(line):
        init, cond, inc = m.group(1), m.group(2), m.group(3)
        if not init.strip() or not cond.strip() or not inc.strip():
            continue  # for(;;) or partial
        if "," in init or "," in inc:
            continue  # multi-var loop, too noisy
        iv = re.search(r"\b([A-Za-z_]\w*)\s*=(?!=)", init)
        if not iv:
            continue
        var = iv.group(1)
        in_cond = re.search(rf"\b{re.escape(var)}\b", cond) is not None
        in_inc = re.search(rf"\b{re.escape(var)}\b", inc) is not None
        if not in_cond or not in_inc:
            out.append((m.start(), m.group(0)))
    return out


DUP_ARG_SKIP = {
    "_", "true", "false", "True", "False", "None", "null", "nullptr",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "i8", "i16", "i32", "i64", "i128", "isize",
    "f32", "f64", "bool", "char", "str", "String",
    "int", "uint", "long", "short", "byte", "void", "size_t",
    "_BYTE", "_WORD", "_DWORD", "_QWORD",
}
FORMATTER_NAMES = {
    "printf", "fprintf", "sprintf", "snprintf", "scanf", "fscanf",
    "log", "info", "warn", "error", "debug", "trace",
    "format", "println", "print", "eprintln", "eprint",
    "write", "writeln", "format_args", "panic", "assert", "unreachable",
}


@check("dup-args", "function call with identical adjacent args (V549)")
def _dup_args(line, ext):
    # Find `x , x` adjacencies, then verify we're inside an unbalanced `(`.
    # Two-step to avoid catastrophic backtracking on long lines.
    pat = re.compile(rf"{EXPR_START}({EXPR})\s*,\s*\1\s*[,)]")
    out = []
    for m in pat.finditer(line):
        if m.group(1) in DUP_ARG_SKIP:
            continue
        before = line[: m.start()]
        if before.count("(") - before.count(")") <= 0:
            continue  # not inside a call/paren group
        # Skip if inside a format-like macro/function (printf, log::trace!, ...).
        depth = 0
        open_pos = -1
        for j in range(len(before) - 1, -1, -1):
            ch = before[j]
            if ch == ")":
                depth += 1
            elif ch == "(":
                if depth == 0:
                    open_pos = j
                    break
                depth -= 1
        if open_pos >= 0:
            pre = before[:open_pos].rstrip()
            if pre.endswith("!"):
                continue  # Rust macro: format!, log::trace!, etc.
            name_m = re.search(r"(\w+)$", pre)
            if name_m and name_m.group(1) in FORMATTER_NAMES:
                continue
        out.append((m.start(), m.group(0)))
    return out


@check(
    "dup-assign",
    "variable assigned twice with no use in between (V519)",
    scope="file",
)
def _dup_assign(lines, ext):
    out = []
    pat = re.compile(rf"^\s*({EXPR})\s*=(?!=)\s*([^;]+);")
    prev = None  # (var, rhs, line_no, base_name)
    for i, line in enumerate(lines, 1):
        if not line.strip():
            continue
        # Reset across scope changes — branches/loops/blocks aren't dead-stores.
        if "{" in line or "}" in line:
            prev = None
        m = pat.match(line)
        if not m:
            if prev and re.search(rf"\b{re.escape(prev[3])}\b", line):
                prev = None
            continue
        cur_var, cur_rhs = m.group(1), m.group(2).strip()
        # Skip compound assigns mis-detected (e.g. starts with `=` of `>=` etc).
        base = re.match(r"[A-Za-z_]\w*", cur_var).group(0)
        if prev and prev[0] == cur_var:
            prev_rhs = prev[1]
            name_re = re.compile(rf"\b{re.escape(base)}\b")
            # Skip accumulator patterns (rhs references the var).
            if not name_re.search(prev_rhs) and not name_re.search(cur_rhs):
                if prev_rhs != cur_rhs:  # `x = 1; x = 1;` is dead-store too but quieter
                    out.append(
                        (
                            i,
                            m.start(1),
                            f"{cur_var} reassigned (prev line {prev[2]}: ={prev_rhs};)",
                        )
                    )
        prev = (cur_var, cur_rhs, i, base)
    return out


@check("swap-bug", "broken swap `a=b; b=a;` (V587) — loses original `a`", scope="file")
def _swap_bug(lines, ext):
    out = []
    pat = re.compile(rf"^\s*({EXPR})\s*=\s*({EXPR})\s*;\s*$")
    prev = None
    for i, line in enumerate(lines, 1):
        if not line.strip():
            continue
        m = pat.match(line)
        if not m:
            prev = None
            continue
        lhs, rhs = m.group(1), m.group(2)
        if prev:
            plhs, prhs, pi = prev
            if lhs == prhs and rhs == plhs and lhs != rhs:
                out.append((i, m.start(1), f"line {pi}: {plhs}={prhs}; → {lhs}={rhs};"))
        prev = (lhs, rhs, i)
    return out


# ---------- Driver ----------


def _color_supported():
    return sys.stdout.isatty() and os.environ.get("NO_COLOR") is None


def colorize(s, c):
    if not _color_supported():
        return s
    codes = {"red": 31, "yellow": 33, "cyan": 36, "dim": 2, "bold": 1}
    return f"\033[{codes[c]}m{s}\033[0m"


def iter_files(root, exts):
    for dp, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS and not d.startswith(".")]
        for f in files:
            if Path(f).suffix in exts:
                yield Path(dp) / f


def scan(path, enabled):
    try:
        src = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return []
    ext = path.suffix
    clean = sanitize(src, ext)
    lines = clean.splitlines()
    hits = []
    line_checks = [(n, CHECKS[n][1]) for n in enabled if CHECKS[n][2] == "line"]
    file_checks = [(n, CHECKS[n][1]) for n in enabled if CHECKS[n][2] == "file"]
    for ln_no, line in enumerate(lines, 1):
        if not line.strip():
            continue
        for name, fn in line_checks:
            for col, snippet in fn(line, ext):
                hits.append((ln_no, col, name, snippet.strip()))
    for name, fn in file_checks:
        for ln_no, col, snippet in fn(lines, ext):
            hits.append((ln_no, col, name, snippet.strip()))
    return hits


def main():
    ap = argparse.ArgumentParser(
        description=__doc__.splitlines()[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Exit code: 0 if no hits, 1 if hits found, 2 on bad args.",
    )
    ap.add_argument("path", nargs="?", default=".")
    ap.add_argument("--ext", nargs="+", help="restrict extensions (e.g. .rs .c)")
    ap.add_argument("--checks", help="comma-separated list (default: all)")
    ap.add_argument("--list-checks", action="store_true")
    ap.add_argument("--quiet", action="store_true", help="only print summary to stderr")
    args = ap.parse_args()

    if args.list_checks:
        for name, (desc, _, _) in CHECKS.items():
            print(f"  {name:<14} {desc}")
        return 0

    enabled = list(CHECKS.keys())
    if args.checks:
        wanted = [c.strip() for c in args.checks.split(",") if c.strip()]
        unknown = [c for c in wanted if c not in CHECKS]
        if unknown:
            print(f"unknown check(s): {', '.join(unknown)}", file=sys.stderr)
            return 2
        enabled = wanted

    exts = set(args.ext) if args.ext else DEFAULT_EXTS

    total = 0
    by_check = {}
    files = 0
    for f in iter_files(Path(args.path), exts):
        files += 1
        for ln, col, name, snip in scan(f, enabled):
            total += 1
            by_check[name] = by_check.get(name, 0) + 1
            if not args.quiet:
                loc = colorize(f"{f}:{ln}:{col + 1}", "cyan")
                tag = colorize(f"[{name}]", "yellow")
                print(f"{loc} {tag} {snip}")

    print(file=sys.stderr)
    print(colorize(f"scanned {files} files — {total} hit(s)", "bold"), file=sys.stderr)
    for name in sorted(by_check):
        print(f"  {name:<14} {by_check[name]}", file=sys.stderr)
    return 1 if total else 0


if __name__ == "__main__":
    sys.exit(main())
