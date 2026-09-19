#!/usr/bin/env python3
"""Retain an agent's final message in the run's records, verbatim, with its hashes.

usage: retain_return.py <agent output jsonl> <marker text> <dest path> <title line>
                        [--sub 'literal=>replacement' ...]

What it does:
  1. finds the last assistant text block that contains the marker (as relay.py does);
  2. records the SHA-256 of that message exactly as the host stored it;
  3. replaces every git worktree root of this repository with {REPO_ROOT}, because authored
     files carry no machine path;
  4. applies each declared substitution. Use one only where a child QUOTED a path pattern it
     searched for and the repository's path-anchor validator reads the literal as a machine
     path. Every substitution is stated in the retained file's header;
  5. refuses to write if any machine path text remains;
  6. writes title, header, the message and the standard claim fence, and prints the file's
     SHA-256, the verbatim SHA-256 and the line count.

Run the repository validators chained with && before every commit of a retained file.
"""
import hashlib
import json
import os
import re
import subprocess
import sys

FENCE = "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)."
_HEADS = ["Users", "private", "home", "tmp", "var/folders"]
MACHINE = re.compile(r"(?:/(?:%s)/[^\s)\"'`<>]+)" % "|".join(_HEADS))

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

def worktree_roots(near):
    res = subprocess.run(["git", "worktree", "list", "--porcelain"], cwd=near,
                         capture_output=True, text=True)
    if res.returncode != 0:
        return []  # the destination is outside a git working tree; the machine-path refusal below still applies
    out = res.stdout
    roots = [l[len("worktree "):] for l in out.splitlines() if l.startswith("worktree ")]
    return sorted(roots, key=len, reverse=True)

if __name__ == "__main__":
    args = sys.argv[1:]
    if len(args) < 4:
        sys.exit(__doc__)
    src, marker, dest, title = args[:4]
    subs, rest = [], args[4:]
    while rest:
        assert rest[0] == "--sub" and len(rest) >= 2, rest
        lit, rep = rest[1].split("=>", 1)
        subs.append((lit, rep))
        rest = rest[2:]
    text = last_text(src, marker)
    if not text:
        sys.exit("marker not found in any assistant text block")
    verbatim_sha = hashlib.sha256(text.encode("utf-8")).hexdigest()
    nlines = len(text.splitlines())
    dest_dir = os.path.dirname(os.path.abspath(dest))
    os.makedirs(dest_dir, exist_ok=True)
    for root in worktree_roots(dest_dir):
        text = text.replace(root, "{REPO_ROOT}")
    for lit, rep in subs:
        if text.count(lit) < 1:
            sys.exit("declared literal not found: %r" % lit)
        text = text.replace(lit, rep)
    bad = MACHINE.findall(text) + MACHINE.findall(title)
    if bad:
        sys.exit("machine path text remains; declare a --sub for a quoted pattern: %r" % bad)
    body = text.rstrip()
    if body.endswith(FENCE):
        body = body[: -len(FENCE)].rstrip()
    hdr = ("Retained by ROOT. What follows is the agent's final message as the host stored it, "
           "with the machine's repository path replaced by `{REPO_ROOT}`")
    if subs:
        hdr += (". One further kind of substitution is made and nothing else is changed: the agent "
                "quoted path patterns it searched for, and the repository's path-anchor validator "
                "reads such a literal as a machine path wherever it appears, so ROOT writes "
                + "; ".join("the pattern it quoted as %s" % rep for _, rep in subs) + ".")
    else:
        hdr += "; nothing else is changed."
    hdr += (" The message exactly as the host stored it has SHA-256 `%s` (%d lines), by this script."
            % (verbatim_sha, nlines))
    with open(dest, "w", encoding="utf-8") as f:
        f.write(title + "\n\n" + hdr + "\n\n---\n\n" + body + "\n\n" + FENCE + "\n")
    print(hashlib.sha256(open(dest, "rb").read()).hexdigest(), "verbatim", verbatim_sha, nlines)
