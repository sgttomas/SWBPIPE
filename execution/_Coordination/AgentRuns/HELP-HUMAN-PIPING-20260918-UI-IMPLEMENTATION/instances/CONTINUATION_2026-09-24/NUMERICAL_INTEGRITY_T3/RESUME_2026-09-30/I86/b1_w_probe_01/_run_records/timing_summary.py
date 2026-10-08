"""I86 B1-SW item 4: summarize the /usr/bin/time -l logs written by timing.sh.

Usage: python3 timing_summary.py <timing_dir> > timing_summary.md
Per run: the outcome and the furthest phase (zz_i86_once's I86_ONCE line), real/user/sys
seconds, maximum resident set size and peak memory footprint (bytes, macOS time -l), the
test's own elapsed time, and the W1 phase marks (ms from the call's start).
"""
import os
import re
import sys

d = sys.argv[1]
rows = []
for name in sorted(os.listdir(d)):
    if not name.endswith(".log") or name == "timing_index.log":
        continue
    text = open(os.path.join(d, name), encoding="utf-8", errors="replace").read()
    m = re.search(r"([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys", text)
    rss = re.search(r"(\d+)\s+maximum resident set size", text)
    peak = re.search(r"(\d+)\s+peak memory footprint", text)
    once = re.search(r"I86_ONCE (\S+) (\S+) path=(\S+) outcome=(.*?) phase=(.*?) elapsed_ms=(\d+)", text)
    marks = re.findall(r"I86_MARK (\S+) t_us=(\d+)", text)
    t0 = int(marks[0][1]) if marks else 0
    phases = ", ".join(f"{p} {(int(t) - t0) / 1000:.0f}" for p, t in marks[1:]) if marks else ""
    parts = name[:-4].split("__")
    rows.append([
        parts[0], parts[1] if len(parts) > 1 else "-", parts[2] if len(parts) > 2 else "floor", parts[-1] if len(parts) > 3 else parts[0].split("_r")[-1],
        once.group(4) if once else ("(no test run)" if "floor" in name else "?"), once.group(5) if once else "-",
        m.group(1) if m else "?", m.group(2) if m else "?", m.group(3) if m else "?",
        f"{int(rss.group(1)):,}" if rss else "?", f"{int(peak.group(1)):,}" if peak else "?",
        once.group(6) if once else "-", phases,
    ])
print("| Input | Mode | Path | Rep | Outcome | Furthest phase | real s | user s | sys s | max RSS (B) | peak footprint (B) | test elapsed ms | marks (ms from call_start) |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    print("| " + " | ".join(str(x).replace("|", "/") for x in r) + " |")
