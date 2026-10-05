"""RV87: text sites that copy a result row's id (row.id / result.id / r.id / entity_ref /
result_ref) priced at <= 128 B with multiplicity >= 2,000 in a per-branch T08 text budget.
Result ids are <= 1,024 B (the text run's own `result_id` class; longest reached template 872 B).
Clones are exact (capacity = length), so the under-count per row is mult * (1024 - priced).
Usage: python3 rv87_idclass_scan.py <packet _run_records> <basis-revision source root (projects/chirality-piping at b1f80234dc)> <suffix>
Stdlib only; reads the source snapshot given (no Git access from this script)."""
import json, os, re, sys
RR, SRC, SUF = sys.argv[1], sys.argv[2], sys.argv[3]
out = {}
for br in ("X", "W"):
    d = json.load(open(os.path.join(RR, f"text_budget_{br}.caps{SUF}.out.json")))
    cache = {}
    hits, tot = [], 0
    for r in d["rows"]:
        if r["mult"] >= 2000 and r["bytes"] <= 128 and r["kind"] in ("clone_text", "to_owned", "to_string", "into_text"):
            f = r["file"]
            if f not in cache:
                p = os.path.join(SRC, f)
                cache[f] = open(p).read().split("\n") if os.path.exists(p) else []
            s = cache[f][r["line"] - 1] if 0 < r["line"] <= len(cache[f]) else ""
            if re.search(r"\b(row|result|r|item|res|x)\.id\b", s) or re.search(r"result_ref|entity_ref", s):
                priced = max(8, 2 * r["bytes"])
                under = r["mult"] * max(0, 1024 - priced)
                hits.append({"site": f"{f}:{r['line']}", "kind": r["kind"], "mult": r["mult"], "priced_each": priced,
                             "under_count": under, "source": s.strip()[:120]})
                tot += under
    out[br] = {"sites": hits, "under_count_total": tot}
print(json.dumps(out, indent=1))
