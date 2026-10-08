#!/usr/bin/env python3
"""RV111: drop full-line // comments (//, ///, //!) and blank lines; optionally keep only the
non-test part (before the first `#[cfg(test)]` line). Prints the code-only text."""
import sys
path, part = sys.argv[1], (sys.argv[2] if len(sys.argv) > 2 else "all")
out = []
for line in open(path, encoding="utf-8"):
    if part == "nontest" and line.startswith("#[cfg(test)]"):
        break
    s = line.strip()
    if not s or s.startswith("//"):
        continue
    out.append(line.rstrip("\n"))
sys.stdout.write("\n".join(out) + "\n")
