"""I99 B3-W (I86's compare_runs.py, renamed): compare the probe lines (I99_*) of two runs.
usage: python3 compare_runs.py <run1.log> <run2.log>
Every I99_ line must be identical, in order (the probe prints no timings)."""
import sys

def probe_lines(path):
    out = []
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.rstrip("\n")
        if line.startswith("test "):
            if " I99_" not in line:
                continue
            line = line[line.index(" I99_") + 1:]
        if line.startswith("I99_"):
            out.append(line)
    return out

a, b = probe_lines(sys.argv[1]), probe_lines(sys.argv[2])
same = a == b
print(f"{sys.argv[1].split('/')[-1]} vs {sys.argv[2].split('/')[-1]}: run1 lines={len(a)} run2 lines={len(b)} identical={same}")
if not same:
    for i, (x, y) in enumerate(zip(a, b)):
        if x != y:
            print(f"first difference at probe line {i}:\n  {x[:300]}\n  {y[:300]}")
            break
sys.exit(0 if same else 1)
