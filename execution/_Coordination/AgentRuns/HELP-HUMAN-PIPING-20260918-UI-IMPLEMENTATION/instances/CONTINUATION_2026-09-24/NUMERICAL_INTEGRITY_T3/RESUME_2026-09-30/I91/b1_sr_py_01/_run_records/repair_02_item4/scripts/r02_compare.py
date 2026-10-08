"""I91 repair 02: compare a run of RV113's harness (census and probes) with RV113's published outputs.

Usage: r02_compare.py <run dir> <rv113 dir> <corpus.json> <out.json>
- census: this run's bound, unbound and transport verdicts against RV113's `py_head.jsonl` (SR-PY at
  11cc14e3e6), entry by entry (full verdicts, detail included), and the bound verdict against the corpus's
  Python expectation (`expected_by_reader.python`, else `expected`; must-pass: pass with its eligibility);
- probes (v5, fg, rp): this run's verdicts against PY at 11cc14e3e6, RS and TS (RV113's runs), simplified to
  ("admitted", eligible) or (gate, code).
"""
import collections, json, sys
from pathlib import Path

RUN, RV, CORPUS, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4])


def lines(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def simple(v):
    if v is None:
        return None
    if "ok" in v:
        return ["admitted", v["ok"]["numerical_eligible"]]
    if "err" in v:
        return [v["err"]["gate"], v["err"]["code"]]
    return ["escape", v.get("escape")]


report = {}
# Census.
mine = {(r["set"], r["i"]): r for r in lines(RUN / "census_rv113.jsonl")}
theirs = {(r["set"], r["i"]): r for r in lines(RV / "census_py_head.jsonl")}
assert mine.keys() == theirs.keys(), "census entries differ"
changes = []
for k in sorted(mine):
    a, b = mine[k], theirs[k]
    if a["input_sha256"] != b["input_sha256"]:
        changes.append([k, "input"])
    for v in ("bound", "unbound", "transport"):
        if a[v] != b[v]:
            changes.append([list(k), a["id"], v, b[v], a[v]])
corpus = json.loads(CORPUS.read_text())
misses = []
for k, r in mine.items():
    kind, i = k
    if kind == "mutation":
        m = corpus["mutations"][i]
        want = m.get("expected_by_reader", {}).get("python", m["expected"])
        got = r["bound"].get("err")
        if got is None or {"gate": got["gate"], "code": got["code"]} != want:
            misses.append([r["id"], want, r["bound"]])
    elif kind == "must_pass":
        m = corpus["must_pass"][i]
        got = r["bound"].get("ok")
        want = m["expected_eligibility"]
        if got is None or {x: got[x] for x in ("invocation_bound", "numerical_eligible", "standing")} != want:
            misses.append([r["id"], want, r["bound"]])
    else:
        c = corpus["cases"][i]
        got = r["bound"].get("ok")
        if got is None or {x: got[x] for x in ("invocation_bound", "numerical_eligible", "standing")} != c["expected"]:
            misses.append([r["id"], c["expected"], r["bound"]])
report["census"] = {"entries": len(mine), "verdicts": 3 * len(mine), "changes_against_11cc14e3e6": changes, "misses_against_corpus": misses}
print(f"census: {len(mine)} entries x 3 verdicts; {len(changes)} changes against 11cc14e3e6; {len(misses)} misses against the corpus")
for c in changes:
    print("  change", c)
# Probes.
sets = {"v5": ("py_head_v5.jsonl", "probes_rs_head.jsonl", "probes_ts_head.jsonl"), "fg": ("py_head_fg.jsonl", "rs_fg.jsonl", "ts_fg.jsonl"),
        "rp": ("py_head_rp.jsonl", "rs_rp.jsonl", "ts_rp.jsonl")}
report["probes"] = {}
for name, (pyh, rs, ts) in sets.items():
    now = {r["id"]: r for r in lines(RUN / f"probes_{name}.jsonl")}
    old = {r["id"]: r for r in lines(RV / pyh)}
    rsr = {r["id"]: r for r in lines(RV / rs)}
    tsr = {r["id"]: r for r in lines(RV / ts)}
    rows = []
    tally = collections.Counter()
    for pid, r in now.items():
        row = {"id": pid}
        for v in ("bound", "unbound", "transport"):
            n, o = simple(r.get(v)), simple(old.get(pid, {}).get(v))
            row[v] = {"py_r02": n, "py_11cc14e3e6": o, "rs": simple(rsr.get(pid, {}).get(v)), "ts": simple(tsr.get(pid, {}).get(v))}
            tally[(v, "changed" if n != o else "same")] += 1
            tally[(v, "eq_ts" if n == row[v]["ts"] else "ne_ts")] += 1
            tally[(v, "eq_rs" if n == row[v]["rs"] else "ne_rs")] += 1
        if r.get("input_sha256") != old.get(pid, {}).get("input_sha256"):
            row["input_differs"] = True
            tally["input_differs"] += 1
        rows.append(row)
    report["probes"][name] = {"count": len(rows), "tally": {f"{k}": v for k, v in sorted(tally.items(), key=str)}, "rows": rows}
    print(f"probes {name}: {len(rows)}; " + ", ".join(f"{k}={v}" for k, v in sorted(tally.items(), key=str)))
OUT.write_text(json.dumps(report, indent=1))
