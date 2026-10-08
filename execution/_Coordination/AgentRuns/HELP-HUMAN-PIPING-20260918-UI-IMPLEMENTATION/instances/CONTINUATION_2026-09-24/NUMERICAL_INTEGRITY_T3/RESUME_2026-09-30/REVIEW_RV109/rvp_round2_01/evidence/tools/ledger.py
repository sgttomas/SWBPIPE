#!/usr/bin/env python3
"""RV109 round 2: the PR-head ledger extension for SP's hunks of 98a77c716e..603e238517.

For every default (-U3) hunk of SP's eight files, the commits that wrote its added lines (git blame
at the head, restricted to the range) and that removed its deleted lines (each commit's own '-'
lines for that file). Read-only git (GIT_OPTIONAL_LOCKS=0). Prints a Markdown table.

Usage: ledger.py <repo> <base> <head>
"""
import os
import re
import subprocess
import sys

FILES = ["lib.rs", "retained_product.rs", "retained_receipt.rs", "retained_wire.rs", "retained_facade_tests.rs",
         "retained_product_tests.rs", "retained_wire_tests.rs", "retained_tests_hooks/grant2.rs"]
PREFIX = "projects/chirality-piping/core/product_physics/src/"
ENV = dict(os.environ, GIT_OPTIONAL_LOCKS="0")


def git(repo, *args):
    return subprocess.run(["git", "-C", repo, *args], capture_output=True, text=True, env=ENV, check=True).stdout


def main():
    repo, base, head = sys.argv[1:4]
    commits = git(repo, "rev-list", "--reverse", f"{base}..{head}", "--", *[PREFIX + f for f in FILES]).split()
    short = {c: c[:10] for c in commits}
    removed = {}
    for c in commits:
        for f in FILES:
            text = git(repo, "show", "--format=", "-U0", c, "--", PREFIX + f)
            removed[(c, f)] = {l[1:] for l in text.splitlines() if l.startswith("-") and not l.startswith("---")}
    rows = []
    n = 0
    for f in FILES:
        diff = git(repo, "diff", base, head, "--", PREFIX + f)
        hunks = re.split(r"(?m)^(?=@@ )", diff)
        for h in hunks[1:]:
            header = h.splitlines()[0]
            m = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@ ?(.*)", header)
            new_line = int(m.group(3))
            plus, minus = [], []
            ln = new_line
            for line in h.splitlines()[1:]:
                if line.startswith("+"):
                    plus.append(ln)
                    ln += 1
                elif line.startswith("-"):
                    minus.append(line[1:])
                elif line.startswith("\\"):
                    continue
                else:
                    ln += 1
            authors = set()
            if plus:
                blame = git(repo, "blame", "-s", "-l", "-L", f"{min(plus)},{max(plus)}", f"{base}..{head}", "--", PREFIX + f)
                bl = blame.splitlines()
                for i, line in enumerate(bl):
                    if min(plus) + i in plus:
                        sha = line.split()[0].lstrip("^")
                        if not line.startswith("^"):
                            authors.add(sha)
            deleters = set()
            for text in minus:
                for c in commits:
                    if text in removed[(c, f)]:
                        deleters.add(c)
            touched = [short[c] for c in commits if any(a.startswith(c[:10]) or c.startswith(a[:10]) for a in authors) or c in deleters]
            n += 1
            rows.append((n, f, f"{header.split('@@')[1].strip()}", (m.group(5) or "").strip()[:70], len(plus), len(minus), ", ".join(touched) or "?"))
    print("| # | File (PP) | Hunk | Context (git's function line) | +lines | −lines | SP commits |")
    print("|---|---|---|---|---|---|---|")
    for r in rows:
        ctx = r[3].replace("|", "\\|").replace("`", "'")
        print(f"| {r[0]} | `{r[1]}` | `{r[2]}` | {ctx} | {r[4]} | {r[5]} | {r[6]} |")
    unknown = [r for r in rows if r[6] == "?"]
    print(f"\nHunks: {len(rows)}; with no attributed commit: {len(unknown)}")


if __name__ == "__main__":
    main()
