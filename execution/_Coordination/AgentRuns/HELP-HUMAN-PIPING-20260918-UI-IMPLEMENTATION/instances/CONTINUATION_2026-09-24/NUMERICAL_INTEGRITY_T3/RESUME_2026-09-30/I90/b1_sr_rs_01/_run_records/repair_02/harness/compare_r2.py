"""I90 SR-RS repair 2: compare RV113's harness output on b5cb7faaeb (base) and on the repair head.

Usage: compare_r2.py <out dir> <corpus.json> <probes_all.json> <TS verdict files...>
- Census: every 07m entry (17 bases, 294 mutations, 28 must-pass) has the same input and the same
  bound, unbound, transport and standing verdicts on both sides; and, at the head, every verdict
  equals the corpus's Rust expectation.
- Probes (RV113's 103, its (f)/(g) table and its repair-01 table): RS at base and head, the ruled
  target where an item rules one, and TS's present result (RV113's records at 7e47e51b5d).
"""
import json, sys
out, corpus_path, probes_path, *ts_files = sys.argv[1:]
def load(p): return {(r["set"], r["i"], r["id"]): r for r in map(json.loads, open(p))}
def short(v):
    if v is None: return None
    if "err" in v: return f'{v["err"]["gate"]} {v["err"]["code"].replace("RETAINED_PRECISION_", "")}'
    if "ok" in v: return f'admitted eligible={str(v["ok"]["numerical_eligible"]).lower()}'
    return json.dumps(v)[:80]
corpus = json.load(open(corpus_path))
base, head = load(f"{out}/census_base.jsonl"), load(f"{out}/census_head.jsonl")
assert set(base) == set(head) and len(head) == 17 + 294 + 28, (len(base), len(head))
changes, misses = [], []
for k in sorted(head, key=lambda k: (k[0], k[1])):
    b, h = base[k], head[k]
    for field in ("input_sha256", "bound", "unbound", "transport", "standing"):
        if b[field] != h[field]:
            changes.append((k, field, short(b[field]) if field != "input_sha256" else b[field], short(h[field]) if field != "input_sha256" else h[field]))
    kind, i, _ = k
    if kind == "mutation":
        m = corpus["mutations"][i]; want = (m.get("expected_by_reader") or {}).get("rust") or m["expected"]
        if "err" not in h["bound"] or (h["bound"]["err"]["gate"], h["bound"]["err"]["code"]) != (want["gate"], want["code"]):
            misses.append((k, short(h["bound"]), want))
    elif kind == "must_pass":
        m = corpus["must_pass"][i]
        if "ok" not in h["bound"] or h["bound"]["ok"]["numerical_eligible"] != m["expected_eligibility"]["numerical_eligible"]:
            misses.append((k, short(h["bound"]), m["expected_eligibility"]))
    else:
        c = corpus["cases"][i]
        if "ok" not in h["bound"] or h["bound"]["ok"]["numerical_eligible"] != c["expected"]["numerical_eligible"]:
            misses.append((k, short(h["bound"]), c["expected"]))
print(f"CENSUS {len(head)} entries: {len(changes)} changes (input, bound, unbound, transport, standing) b5cb7faaeb -> head; {len(misses)} misses against the corpus's Rust expectations")
for c in changes: print("CHANGE", c)
for m in misses: print("MISS", m)
probes = json.load(open(probes_path))
pb, ph = load(f"{out}/probes_base.jsonl"), load(f"{out}/probes_head.jsonl")
ts = {}
for f in ts_files:
    for r in map(json.loads, open(f)): ts[r["id"]] = r
G3, ATT, PRODUCT, INV = "G3 COVERAGE_MISMATCH", "G5 ATTEMPT_MISMATCH", "G5 PRODUCT_ATTEMPT_MISMATCH", "G8 INVOCATION_MISMATCH"
def ruled(p):
    pid = p["id"]
    if pid.startswith("f_"): return ("1 (f)", G3, G3)
    if pid.startswith("g_"): return ("2 (g)", "admitted eligible=true" if pid == "g_pressure_contract_null" else INV, "admitted eligible=false")
    if pid in ("r_b_basis_ref_7", "r_c_basis_omits_case_1", "r_c_missing_sourceless_basis"): return ("1 ordinary basis", ATT, ATT)
    if pid in ("c2_receipt_phase_kernel", "c2_receipt_code_facade", "c2_receipt_phase_preparation", "c2_facade_phase_kernel", "x_reason_cause_receipt_failure"): return ("3 C2", ATT, None)
    if pid == "c2_receipt_ok": return ("3 C2", "admitted eligible=false", None)
    if pid == "c2_facade_ok": return ("3 C2", PRODUCT, None)
    return (None, None, None)
rows, off, eq_ts, ne_ts = [], [], 0, []
for i, p in enumerate(probes):
    k = ("probe", i, p["id"]); b, h, t = pb[k], ph[k], ts.get(p["id"])
    item, want_b, want_u = ruled(p)
    row = {"id": p["id"], "set": p["set"], "item": item,
           "rs_base": {f: short(b[f]) for f in ("bound", "unbound", "transport")},
           "rs_head": {f: short(h[f]) for f in ("bound", "unbound", "transport")},
           "ts_present": {f: short(t[f]) for f in ("bound", "unbound", "transport")} if t else None,
           "ruled": {"bound": want_b, "unbound": want_u} if item else "unchanged from b5cb7faaeb (bound, unbound)"}
    if item:
        ok = row["rs_head"]["bound"] == want_b and (want_u is None or row["rs_head"]["unbound"] == want_u)
    else:
        ok = all(row["rs_head"][f] == row["rs_base"][f] for f in ("bound", "unbound"))
    row["head_as_ruled"] = ok
    if not ok: off.append(row)
    if t:
        same = {f: row["rs_head"][f] == row["ts_present"][f] for f in ("bound", "unbound", "transport")}
        row["head_equals_ts_present"] = same
        if all(same.values()): eq_ts += 1
        else: ne_ts.append((p["id"], {f: (row["rs_head"][f], row["ts_present"][f]) for f, v in same.items() if not v}))
    rows.append(row)
json.dump(rows, open(f"{out}/PROBE_TABLE_R2.json", "w"), indent=1)
print(f"PROBES {len(probes)}: {len(probes) - len(off)} as ruled (or unchanged where no item rules); {len(off)} not")
for r in off: print("OFF", json.dumps(r))
changed = [r for r in rows if r["rs_base"] != r["rs_head"]]
print(f"PROBES changed b5cb7faaeb -> head: {len(changed)}")
for r in changed: print("CHANGED", r["id"], r["item"], json.dumps(r["rs_base"]), "->", json.dumps(r["rs_head"]))
print(f"TS present: equal on all three verdicts for {eq_ts} of {sum(1 for r in rows if r['ts_present'])}")
for x in ne_ts: print("TS-DIFF", json.dumps(x))
