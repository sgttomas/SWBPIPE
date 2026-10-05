"""QUALIFICATION §11 (RV87 G6r N-1): compare a run's positive-multiplicity non-candidates (the
expressions the identifier predicate does not treat as identifier-bearing; text_budget.py with
TB_NONCAND_OUT) with the set RV87 read at G6 (rv87_g6r_noncandidates.out.json, 410 rows).
Rows are matched by (file, enclosing fn name, expression, kind/spec, rule class, multiplicity), so
line shifts do not matter; RV87's dump cut long expressions, so an expression matches when one is a
prefix of the other. Any row not matched is a new non-candidate: it must be read (by type and
binding) before the run is relied on, and the script exits 1."""
import json, sys, collections
rv_p, run_p = sys.argv[1:3]
rv = json.load(open(rv_p))["rows"]; run = json.load(open(run_p))
def key(r):
    return (r["site"].rsplit(":", 1)[0].split("core/")[-1], (r["fn"] or "").rsplit(":", 1)[-1], r["kind_or_spec"], r["class"], r["mult"])
expr = lambda r: " ".join(r["expr"].split())
pool = collections.defaultdict(list)
for r in rv: pool[key(r)].append(expr(r))
new = []
for r in run:
    cands = pool.get(key(r), [])
    e = expr(r)
    hit = next((i for i, x in enumerate(cands) if x == e or e.startswith(x) or x.startswith(e)), None)
    if hit is None: new.append(r)
    else: cands.pop(hit)
gone = [(k, x) for k, xs in pool.items() for x in xs]
out = {"rv87_rows": len(rv), "run_rows": len(run), "new_noncandidates": new, "rv87_rows_absent_now": gone,
       "verdict": "carried: every non-candidate is one RV87 read" if not new else "NEW non-candidates: read them"}
print(json.dumps(out, indent=1))
sys.exit(1 if new else 0)
