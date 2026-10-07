"""I82 B1-S: the scope of every roster row and of every (form, atom) term, read mechanically from the
multi-case chain's own outputs at c = 1, 2, 3 (D1 model caps, a = c, L = c*l), and checked at c = 4.

A term's coefficient k(c) is classified:
  invocation   k(1) = k(2) = k(3) = k(4)                        (per invocation)
  case         k(c) = c * k(1) for c = 2, 3, 4                   (per requested or attempted case; a = c here)
  affine       k(c) = alpha + beta * c exactly for c = 1..4, alpha != 0 (an invocation part plus a per-case part)
  growth       anything else: a container law's step (PushCap, buckets, tree nodes) over a count that grows with c,
               or a product of two c-dependent counts (e.g. c x D_env(c)); always nondecreasing in c
Rows (the chain's named rows) are classified the same way on their evaluated ASSUMED bytes (the illustrative table,
used only to see the scope; the maxima are evaluated in-build elsewhere).
Usage: python3 b1_classify.py <mc_runs dir> <out json>
"""
import json, os, sys

runs, out = sys.argv[1:3]
TAGS = ["d1_c1", "d1_c2", "d1_c3", "d1_c4"]
def load(tag, f):
    return json.load(open(os.path.join(runs, tag, "work", f)))

def classify(vals):
    k1, k2, k3, k4 = vals
    if k1 == k2 == k3 == k4:
        return "invocation"
    if k1 and k2 == 2 * k1 and k3 == 3 * k1 and k4 == 4 * k1:
        return "case"
    beta = k2 - k1
    alpha = k1 - beta
    if k3 == alpha + 3 * beta and k4 == alpha + 4 * beta:
        return "affine"
    return "growth"

trees = [load(t, "profile_tree.json") for t in TAGS]
terms = {}
summary = {}
for name in trees[0]["forms"]:
    keys = set()
    for t in trees:
        keys |= set(t["forms"][name])
    for a in sorted(keys):
        vals = [t["forms"][name].get(a, 0) for t in trees]
        cl = classify(vals)
        terms.setdefault(name, {})[a] = {"class": cl, "coef_c1_c4": vals}
        summary.setdefault(name, {}).setdefault(cl, 0)
        summary[name][cl] += 1

rows = {}
def rowset(fname, getter):
    data = [load(t, fname) for t in TAGS]
    for k in getter(data[0]):
        vals = [getter(d)[k] for d in data]
        rows[f"{fname}: {k}"] = {"class": classify(vals), "assumed_bytes_c1_c4": vals}
for mode in ("sparse", "dense"):
    rowset("ordinary_caps.caps.out.json", lambda d, m=mode: {k: v["assumed_bytes"] for k, v in d[m]["rows"].items()})
rowset("producer_caps.caps.out.json", lambda d: {k: v["assumed_bytes"] for k, v in d["rows"].items()})
rowset("g4_caps.caps.eps2.out.json", lambda d: {k: v["assumed_bytes"] for k, v in d["rows"].items()})
rowset("t25_g4.caps.eps2.out.json", lambda d: {k: v["assumed_bytes"] for k, v in d["rows"].items()})
rowset("text_closure.caps.json", lambda d: {k: v for k, v in d["atoms"].items()})
counts = {}
for k, v in rows.items():
    counts[v["class"]] = counts.get(v["class"], 0) + 1
json.dump({"tags": TAGS, "form_term_summary": summary, "form_terms": terms, "rows": rows, "row_class_counts": counts},
          open(out, "w"), indent=1)
print(json.dumps({"row_class_counts": counts,
                  "form_term_counts": {c: sum(s.get(c, 0) for s in summary.values()) for c in ("invocation", "case", "affine", "growth")}}))
