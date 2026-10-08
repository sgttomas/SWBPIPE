#!/usr/bin/env python3
"""I89 B1-SA (copied from I85 b1_st_01): copy a log or script into the records with placeholder paths only.

Machine paths become WT, VENV and ~ (WT is two levels above this script's scratch folder; VENV
is the piping project's virtual environment beside WT's parent checkout). A line over 4,000
bytes is cut to its first 1,000 bytes, followed by its full length and sha256 (the producer's
committed cfg(test) prints, such as I51_DUAL_LANES and I51_FROZEN_*, are the only such lines).
With --filter, only cargo's Running, test, panic, failure and result lines are kept (the runner
suite's debug output is dropped).

Usage: sanitize.py [--filter] SRC DST
"""
import hashlib, os, re, sys
args = sys.argv[1:]
filt = args[0] == "--filter"
if filt: args = args[1:]
src, dst = args
# I89 follow-up 01: this script lives in WT/scratch/i89_b1_sa/scripts, one level deeper than
# I85's copy, so WT is four levels above it (the first version used three, so it replaced
# WT/scratch with "WT" and left other WT paths in a home-relative form).
WT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
assert os.path.isdir(os.path.join(WT, "tools")) and os.path.isdir(os.path.join(WT, "guard")), WT
VENV = os.path.join(os.path.dirname(os.path.dirname(WT)), "projects", "chirality-piping", ".venv")
HOME = os.path.expanduser("~")
KEEP = re.compile(r"^\s*(Running|Doc-tests)|^test |panicked|^failures|^test result|^---- |^error|^warning: `")
out = []
for line in open(src, encoding="utf-8", errors="replace").read().split("\n"):
    line = line.replace(VENV, "VENV").replace(WT, "WT").replace(HOME, "~")
    if filt and not KEEP.search(line):
        continue
    b = line.encode()
    if len(b) > 4000:
        line = b[:1000].decode("utf-8", "ignore") + f" [... cut: {len(b)} bytes, sha256 {hashlib.sha256(b).hexdigest()}]"
    out.append(line)
text = "\n".join(out)
assert HOME not in text and "/" + "private/" not in text and "~" + "/" not in text, src
assert ".claude" + "/worktrees" not in text and "swbpipe" + "-control-layer" not in text, src
open(dst, "w", encoding="utf-8").write(text)
