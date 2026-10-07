#!/usr/bin/env python3
"""Copy a text file with machine paths replaced by placeholders (WT, VENV, P);
fails if any absolute user path remains. Usage: sanitize_copy.py SRC DST"""
import sys
WT = "WT"
VENV = "VENV"
src, dst = sys.argv[1], sys.argv[2]
text = open(src, encoding="utf-8", errors="strict").read()
text = text.replace(VENV, "VENV").replace(WT, "WT")
text = text.replace("/" + "private/tmp/claude-501", "SYSTEM_TMP")
import re
for bad in ("/" + "Users/", "/" + "private/", r"(^|[\s\"'=(])/" + "tmp/", "/" + "home/"):
    if re.search(bad, text, re.M):
        lines = [l for l in text.splitlines() if re.search(bad, l)][:3]
        sys.exit(f"unsanitized {bad} in {src}: {lines}")
open(dst, "w", encoding="utf-8").write(text)
