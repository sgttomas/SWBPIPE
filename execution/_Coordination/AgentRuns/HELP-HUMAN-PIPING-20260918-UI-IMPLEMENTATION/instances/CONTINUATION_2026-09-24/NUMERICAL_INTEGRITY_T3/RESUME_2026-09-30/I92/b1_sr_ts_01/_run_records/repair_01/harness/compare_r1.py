"""I92 repair 01: compare RV113-harness outputs. Usage: compare_r1.py <R> <S2 logs dir> <head label>
1. The census over 07m, 7e47e51b5d (base) against the head: input, bound, unbound and transport, entry by entry.
2. Misses against the corpus's TypeScript expectations (bound first failures; must-pass and base eligibility).
3. TS (head) against RS at I90's 6e3e4fe219 (I90 REPAIR_02's census_head.jsonl and probes_head.jsonl), normalized:
   an error is (gate, code); an admission is (ok, invocation_bound, numerical_eligible, publication_sha256).
4. The 135 probes, base against head, and each changed probe."""
import json, sys, collections
R, L, HEAD = sys.argv[1:]
RS = f"{R}/I90/b1_sr_rs_01/_run_records/repair_02/out"
def load(p): return [json.loads(l) for l in open(p) if l.strip()]
def key(l): return (l["set"], l["i"], l["id"])
def norm(v):
    if "err" in v: return ["err", v["err"]["gate"], v["err"]["code"]]
    if "ok" in v: return ["ok", v["ok"]["invocation_bound"], v["ok"]["numerical_eligible"], v["ok"]["publication_sha256"]]
    return ["other", json.dumps(v)]
V = ("bound", "unbound", "transport")
cb, ch = load(f"{L}/census_base.jsonl"), load(f"{L}/census_" + HEAD + ".jsonl")
B, H = {key(l): l for l in cb}, {key(l): l for l in ch}
assert list(B) == list(H) and len(B) == 339
changes = collections.Counter(); changed = []
for k in B:
    if B[k]["input_sha256"] != H[k]["input_sha256"]: changes["input"] += 1; changed.append((k, "input"))
    for v in V:
        if B[k][v] != H[k][v]: changes[v] += 1; changed.append((k, v, norm(B[k][v]), norm(H[k][v])))
print("CENSUS 339 entries, 7e47e51b5d -> head:", {v: changes[v] for v in ("input",) + V})
for c in changed: print("  CHANGE", json.dumps(c))
# corpus expectations (TS)
P = sys.argv[1].split("/execution/")[0]
corpus = json.load(open(f"{P}/fixtures/results/retained_precision_cases.json"))
miss = 0
for l in ch:
    if l["set"] == "mutation":
        m = corpus["mutations"][l["i"]]; want = (m.get("expected_by_reader") or {}).get("typescript") or m["expected"]
        if "err" not in l["bound"] or [l["bound"]["err"]["gate"], l["bound"]["err"]["code"]] != [want["gate"], want["code"]]: miss += 1; print("  MISS", l["id"])
    elif l["set"] == "must_pass":
        m = corpus["must_pass"][l["i"]]; e = m["expected_eligibility"]; o = l["bound"].get("ok")
        if not o or [o["invocation_bound"], o["numerical_eligible"], o["standing"]] != [e["invocation_bound"], e["numerical_eligible"], e["standing"]]: miss += 1; print("  MISS", l["id"])
    else:
        c = corpus["cases"][l["i"]]; e = c["expected"]; o = l["bound"].get("ok")
        if not o or [o["invocation_bound"], o["numerical_eligible"], o["standing"]] != [e["invocation_bound"], e["numerical_eligible"], e["standing"]]: miss += 1; print("  MISS", l["id"])
print("MISSES against the corpus's TypeScript expectations (head):", miss)
# TS head vs RS
for name, ts, rs in (("census", ch, load(f"{RS}/census_head.jsonl")), ("probes", load(f"{L}/probes_" + HEAD + ".jsonl"), load(f"{RS}/probes_head.jsonl"))):
    T, Rr = {(l["set"], l["i"], l["id"]): l for l in ts}, {(l["set"], l["i"], l["id"]): l for l in rs}
    assert set(T) == set(Rr), name
    diff = [(k, v, norm(T[k][v]), norm(Rr[k][v])) for k in T for v in V if norm(T[k][v]) != norm(Rr[k][v])]
    eq = sum(1 for k in T if all(norm(T[k][v]) == norm(Rr[k][v]) for v in V))
    print(f"TS head vs RS 6e3e4fe219, {name}: {eq} of {len(T)} equal on all three verdicts; {len(diff)} verdict differences")
    for d in diff: print("  DIFF", json.dumps(d))
# probes base vs head
pb, ph = load(f"{L}/probes_base.jsonl"), load(f"{L}/probes_" + HEAD + ".jsonl")
PB, PH = {key(l): l for l in pb}, {key(l): l for l in ph}
pc = [(k, v, norm(PB[k][v]), norm(PH[k][v])) for k in PB for v in V if PB[k][v] != PH[k][v]]
print(f"PROBES 135, 7e47e51b5d -> head: {len({c[0] for c in pc})} probes changed, {len(pc)} verdicts")
for c in pc: print("  PCHANGE", json.dumps(c))
