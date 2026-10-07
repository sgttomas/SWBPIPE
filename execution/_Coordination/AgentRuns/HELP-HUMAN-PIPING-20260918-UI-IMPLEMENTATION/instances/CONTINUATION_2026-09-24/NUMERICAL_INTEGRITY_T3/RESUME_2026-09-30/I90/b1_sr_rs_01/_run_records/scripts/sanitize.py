#!/usr/bin/env python3
"""I90 B1-SR-RS: copy a log or script into the records with placeholder paths only (after I85's
sanitize.py). Machine paths become WT, VENV and ~ (WT is three levels above this script). A line
over 4,000 bytes is cut to its first 1,000 bytes, followed by its full length and sha256.

With --filter, only cargo's Running, test, panic, failure, warning and result lines are kept
(the runner suite's debug output is dropped).

Usage: sanitize.py [--filter] SRC DST
"""
import hashlib, os, re, sys
args = sys.argv[1:]
filt = args[0] == "--filter"
if filt: args = args[1:]
src, dst = args
KEEP = re.compile(r"^\s*(Running|Doc-tests)|^test |panicked|^failures|^test result|^---- |^error|^warning")
WT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
VENV = os.path.join(os.path.dirname(os.path.dirname(WT)), "projects", "chirality-piping", ".venv")
HOME = os.path.expanduser("~")
TMP = "/" + "private/tmp"  # the system temp directory: never in a record
out = []
for line in open(src, encoding="utf-8", errors="replace").read().split("\n"):
    line = line.replace(VENV, "VENV").replace(WT, "WT").replace(HOME, "~")
    assert TMP not in line, (src, line[:200])
    if filt and not KEEP.search(line):
        continue
    b = line.encode()
    if len(b) > 4000:
        line = b[:1000].decode("utf-8", "ignore") + f" [... cut: {len(b)} bytes, sha256 {hashlib.sha256(b).hexdigest()}]"
    out.append(line)
os.makedirs(os.path.dirname(dst), exist_ok=True)
open(dst, "w", encoding="utf-8").write("\n".join(out))
