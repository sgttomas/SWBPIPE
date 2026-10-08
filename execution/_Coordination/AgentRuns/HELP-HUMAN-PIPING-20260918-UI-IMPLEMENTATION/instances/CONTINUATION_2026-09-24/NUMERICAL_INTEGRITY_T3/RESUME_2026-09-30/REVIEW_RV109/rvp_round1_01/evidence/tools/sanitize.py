#!/usr/bin/env python3
"""RV109: copy a text file into the records with machine paths replaced by placeholders.

Usage: sanitize.py <src> <dst> [--filter REGEX]
- R, T, NUM, WT and ~ replace the machine paths (longest first);
- a line over 4,000 bytes is cut to 1,000 bytes, followed by its length and sha256;
- with --filter, only lines matching REGEX are kept.
"""
import hashlib
import re
import sys

WT = "WT"
NUM = WT + "/numerics"
T = NUM + "/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3"
R = T + "/RESUME_2026-09-30"
VENV = "VENV"
REPL = [(R, "R"), (T, "T"), (NUM, "NUM"), (WT, "WT"), (VENV, "VENV"), ("<tmp>", "<tmp>"), ("~", "~")]


def clean(line):
    for old, new in REPL:
        line = line.replace(old, new)
    return line


def main():
    src, dst = sys.argv[1], sys.argv[2]
    pat = None
    if len(sys.argv) > 4 and sys.argv[3] == "--filter":
        pat = re.compile(sys.argv[4])
    out = []
    with open(src, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            line = line.rstrip("\n")
            if pat and not pat.search(line):
                continue
            line = clean(line)
            raw = line.encode("utf-8")
            if len(raw) > 4000:
                digest = hashlib.sha256(raw).hexdigest()
                line = raw[:1000].decode("utf-8", errors="ignore") + f" …[cut: {len(raw)} bytes, sha256 {digest}]"
            out.append(line)
    with open(dst, "w", encoding="utf-8") as fh:
        fh.write("\n".join(out) + ("\n" if out else ""))


if __name__ == "__main__":
    main()
