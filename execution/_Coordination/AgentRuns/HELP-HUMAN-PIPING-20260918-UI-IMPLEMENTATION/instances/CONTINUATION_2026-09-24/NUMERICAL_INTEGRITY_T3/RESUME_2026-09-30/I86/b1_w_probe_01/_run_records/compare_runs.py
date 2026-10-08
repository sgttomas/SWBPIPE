"""I86 B1-SW: compare the probe lines (I86_*) of two runs, ignoring timings.

Usage: python3 compare_runs.py <run1.log> <run2.log> [--section <test name>] [--ignore-assembly]
Timing tokens (ms=, direct_ms=, elapsed_ms=) and the I86_MARK lines (wall-clock phase
marks) are removed; every other I86_ line must be identical, in order.
--section keeps only the lines printed by that test (libtest's "test <name> ..." blocks).
--ignore-assembly drops the `assembly=[...]` field of I86_ORDINARY, which the probe gained
after the first controls run (a print added between builds; no input or producer change).
"""
import re
import sys

TIMING = re.compile(r" (?:ms|direct_ms|elapsed_ms)=\d+")
ASSEMBLY = re.compile(r" assembly=\[[^\]]*\]")


def probe_lines(path, section=None, ignore_assembly=False):
    out = []
    on = section is None
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.rstrip("\n")
        if line.startswith("test "):
            if section is not None:
                on = section in line.split(" ... ")[0]
            # A test's first print shares its line with libtest's "test <name> ... " prefix.
            if " I86_" not in line:
                continue
            line = line[line.index(" I86_") + 1:]
        if not on or not line.startswith("I86_") or line.startswith("I86_MARK "):
            continue
        line = TIMING.sub("", line)
        if ignore_assembly:
            line = ASSEMBLY.sub("", line)
        out.append(line)
    return out


args = sys.argv[1:]
section = args[args.index("--section") + 1] if "--section" in args else None
ignore = "--ignore-assembly" in args
a, b = probe_lines(args[0], section, ignore), probe_lines(args[1], section, ignore)
same = a == b
print(f"{args[0].split('/')[-1]} vs {args[1].split('/')[-1]} section={section} ignore_assembly={ignore}: run1 lines={len(a)} run2 lines={len(b)} identical={same}")
if not same:
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            print(f"first difference at probe line {i}:\n  {x[:300]}\n  {y[:300]}")
            break
sys.exit(0 if same else 1)
