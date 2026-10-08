#!/usr/bin/env python3
"""RV109 round 2: format ledger.py's output as the REVIEW.md ledger-extension table."""
import re
import sys

lines = open(sys.argv[1], encoding="utf-8").read().splitlines()
out = ["| # | File (PP) | Hunk (`98a77c716e` → `603e238517`) | Context | +/− | SP commit(s) | Reviewed |",
       "|---|---|---|---|---|---|---|"]
for l in lines:
    if not re.match(r"^\| \d+ \|", l):
        continue
    cells = [c.strip() for c in re.split(r"(?<!\\)\|", l)[1:-1]]
    n, f, hunk, ctx, plus, minus, commits = cells
    ctx = ctx.replace("\\|", "/").replace("`", "'")
    if len(ctx) > 60:
        ctx = ctx[:57] + "…"
    commits = ", ".join(f"`{c.strip()}`" for c in commits.split(","))
    out.append(f"| {n} | {f} | {hunk} | {ctx} | +{plus}/−{minus} | {commits} | RV109 r2 |")
print("\n".join(out))
