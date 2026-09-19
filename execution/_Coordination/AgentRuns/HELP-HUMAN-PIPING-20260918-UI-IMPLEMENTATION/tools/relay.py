#!/usr/bin/env python3
"""Extract an agent's final message, byte for byte, to a file OUTSIDE the repository.

usage: relay.py <agent output jsonl> <marker text> <dest path>

The host's task notification names the agent's output file (a JSONL transcript). Never read
that file into a conversation: it is the whole transcript. This script finds the last
assistant text block that contains the marker, writes it unchanged to <dest>, and prints the
SHA-256 of the file and its line count.

Use it when a nested child's completion came to ROOT and not to its manager (a background
child whose manager had stopped): send the manager the path, the SHA-256 and a short digest
labelled as ROOT's reading, and ask it to ignore a duplicate. The manager verifies the hash
and retains the return itself. ROOT's digest is a pointer and never the return.
"""
import hashlib
import json
import sys

def last_text(path, marker):
    last = None
    for line in open(path, encoding="utf-8"):
        try:
            d = json.loads(line)
        except Exception:
            continue
        m = d.get("message") or {}
        if m.get("role") != "assistant":
            continue
        for c in m.get("content") or []:
            if isinstance(c, dict) and c.get("type") == "text" and marker in (c.get("text") or ""):
                last = c["text"]
    return last

if __name__ == "__main__":
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    src, marker, dest = sys.argv[1:4]
    text = last_text(src, marker)
    if not text:
        sys.exit("marker not found in any assistant text block")
    with open(dest, "w", encoding="utf-8") as f:
        f.write(text)
    print(hashlib.sha256(open(dest, "rb").read()).hexdigest(), len(text.splitlines()), "lines")
