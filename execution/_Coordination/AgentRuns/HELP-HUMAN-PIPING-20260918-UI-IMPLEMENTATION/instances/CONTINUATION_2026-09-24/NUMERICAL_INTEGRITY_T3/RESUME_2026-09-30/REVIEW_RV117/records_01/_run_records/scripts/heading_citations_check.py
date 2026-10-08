#!/usr/bin/env python3
"""RV117 (from RV110): quoted RR headings in WG's T3 section and RR's appended text resolve to RR '## ' headings at H.
A quotation counts as a heading citation if it is in double quotes or curly quotes and is >= 20 chars,
preceded by 'RR ' / 'ruling ' / ': ' in a rulings-in-force bullet, or ends with '…'/'...'. Each is checked
as a prefix (ellipsis stripped) of some heading's title (the part before ' (ROOT, ')."""
import re, sys
rr = open(sys.argv[1], encoding="utf-8").read().split("\n")
wg = open(sys.argv[2], encoding="utf-8").read().split("\n")
start_rr = int(sys.argv[3])  # first appended line (1-based)
heads = [re.sub(r"\s*\((ROOT|owner)[^)]*\)\s*$", "", l[3:]).strip() for l in rr if l.startswith("## ")]
def resolve(q):
    q = q.strip().rstrip("…").rstrip(".").rstrip("; ,").strip()
    return [h for h in heads if h.startswith(q) or h == q]
qre = re.compile(r"[\"“]([^\"”]{20,}?)[\"”]")
# WG T3 section
s = next(i for i,l in enumerate(wg) if l.startswith("**T3 rulings in force**"))
e = next(i for i in range(s+1, len(wg)) if wg[i].startswith("**Notes routed"))
out = []
for i in range(s, e):
    for q in qre.findall(wg[i]):
        r = resolve(q); out.append(("WG", i+1, q[:90], len(r)))
for i in range(start_rr-1, len(rr)):
    for q in qre.findall(rr[i]):
        if rr[i].count("RR ") or "ruling" in rr[i] or q.endswith("…") or "section" in rr[i]:
            r = resolve(q)
            out.append(("RR", i+1, q[:90], len(r)))
ok = sum(1 for o in out if o[3] >= 1)
print(f"quotations checked: {len(out)}; resolved: {ok}; unresolved: {len(out)-ok}")
for o in out:
    print(f"{o[0]}:{o[1]}\t{'OK' if o[3] else 'UNRESOLVED'}\t{o[2]}")
