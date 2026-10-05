"""RV87 (u4_g7_01): compare two TEXT runs on different line bases without line numbers: per
(file, kind, enclosing fn name), the multiset of (mult, bytes, req). Reports every group whose
positive-multiplicity rows differ, and the extra rows (with their multiplicity).
Usage: python3 rv87_rows_multiset.py <first run json> <second run json>   (stdlib only)"""
import json, sys, collections
a, b = (json.load(open(p)) for p in sys.argv[1:3])
def groups(t):
    g = collections.defaultdict(list)
    for r in t["rows"]:
        g[(r["file"].split("core/")[-1], r["kind"], (r["fn"] or "").rsplit(":", 1)[-1])].append((r["mult"], r["bytes"], r["req"], r["line"]))
    return g
A, B = groups(a), groups(b)
diff, extra = [], []
for k in sorted(set(A) | set(B)):
    pa = sorted(x[:3] for x in A.get(k, []) if x[0] > 0); pb = sorted(x[:3] for x in B.get(k, []) if x[0] > 0)
    if pa != pb:
        diff.append({"group": k, "first": pa, "second": pb})
    na, nb = len(A.get(k, [])), len(B.get(k, []))
    if nb > na:
        extra.append({"group": k, "second_rows": [(x[3], x[0]) for x in B[k]], "first_rows": na})
print(json.dumps({"tav": [a["total_text_requested_bytes"], b["total_text_requested_bytes"]], "rows": [len(a["rows"]), len(b["rows"])],
                  "positive_groups_differing": diff, "groups_with_extra_rows": extra}, indent=1))
