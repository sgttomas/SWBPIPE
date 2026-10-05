"""Row-level comparison of two text_budget outputs on different line bases: rows are matched by
(file, kind, enclosing fn name, ordinal within that fn and kind); prints every row whose mult,
bytes or req differs, every unmatched row with positive multiplicity, and the totals."""
import json, sys, collections
a, b = (json.load(open(p)) for p in sys.argv[1:3])
def key_rows(t):
    seen = collections.Counter(); out = {}
    for r in sorted(t["rows"], key=lambda r: (r["file"], r["line"], r["kind"])):
        fn = (r["fn"] or "").rsplit(":", 1)[-1]
        k0 = (r["file"], r["kind"], fn)
        out[k0 + (seen[k0],)] = r; seen[k0] += 1
    return out
A, B = key_rows(a), key_rows(b)
diff = [(k, A[k], B[k]) for k in A.keys() & B.keys() if (A[k]["mult"], A[k]["bytes"], A[k]["req"]) != (B[k]["mult"], B[k]["bytes"], B[k]["req"])]
only_a = [(k, A[k]) for k in A.keys() - B.keys() if A[k]["mult"]]
only_b = [(k, B[k]) for k in B.keys() - A.keys()]
res = {"rows": [len(A), len(B)], "changed": len(diff), "unmatched_positive_in_first": len(only_a), "only_in_second": len(only_b),
       "tav": [a["total_text_requested_bytes"], b["total_text_requested_bytes"]], "max_site": [a["largest_single_site_bytes"], b["largest_single_site_bytes"]],
       "D": [a["D_diagnostics"], b["D_diagnostics"]], "reachable_fns": [a["reachable_fns"], b["reachable_fns"]], "complete": [a["complete"], b["complete"]]}
print(json.dumps(res))
for k, x, y in sorted(diff)[:30]: print("CHANGED", k[0].split("core/")[-1], k[1], k[2], (x["line"], x["mult"], x["bytes"], x["req"]), "->", (y["line"], y["mult"], y["bytes"], y["req"]))
for k, x in sorted(only_a)[:20]: print("ONLY-FIRST", k[0].split("core/")[-1], k[1:], x["line"], x["mult"], x["req"])
for k, y in sorted(only_b)[:20]: print("ONLY-SECOND", k[0].split("core/")[-1], k[1:], y["line"], y["mult"], y["req"], y.get("reached"))
