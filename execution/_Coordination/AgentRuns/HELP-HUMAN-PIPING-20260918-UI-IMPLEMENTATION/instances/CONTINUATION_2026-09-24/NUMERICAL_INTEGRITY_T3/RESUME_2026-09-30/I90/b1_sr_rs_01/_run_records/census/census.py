"""I90 B1-SR-RS: the cascade census (PLAN_v2 §2.4; ruling point R5), mechanical.

Usage: python3 census.py <base_re.log> <cand_re.log> <corpus.json>

Both logs are `cargo test -- --nocapture` runs of RE's suite. RE's contract test prints one line
per shared mutation (`slice_outcomes`: tags I63_OUTCOME*, I61_OUTCOME*, I70_OUTCOME*, I83_OUTCOME*,
whose JSON carries the reader's observed first failure, or null for an admitted statement) and one
line per must-pass entry (`I63_MUST_PASS <id> <same>`: admitted with the base case's
classifications and the stated eligibility). The census compares the Rust reader's observed
outcome per entry on base (I1) and on the aligned reader, and requires every 07m entry present
exactly once on each side.
"""
import json, re, sys

def read(path):
    muts, must = {}, {}
    for line in open(path, encoding="utf-8"):
        m = re.match(r"^(I\d+_OUTCOME\w*) (\{.*\})$", line.rstrip("\n"))
        if m:
            d = json.loads(m.group(2))
            assert d["id"] not in muts, ("duplicate", d["id"])
            muts[d["id"]] = {"tag": m.group(1), "observed": d["observed"], "expected": d["expected"], "match": d["match"]}
            continue
        m = re.match(r'^I63_MUST_PASS "(.*)" (true|false)$', line.rstrip("\n"))
        if m:
            assert m.group(1) not in must, ("duplicate", m.group(1))
            must[m.group(1)] = m.group(2) == "true"
    return muts, must

base_m, base_p = read(sys.argv[1])
cand_m, cand_p = read(sys.argv[2])
corpus = json.load(open(sys.argv[3], encoding="utf-8"))
ids = [m["id"] for m in corpus["mutations"]]
pids = [p["id"] for p in corpus["must_pass"]]
assert len(ids) == 294 and len(set(ids)) == 294 and len(pids) == 28, (len(ids), len(pids))
changes = []
for i, mid in enumerate(ids):
    b, c = base_m.get(mid), cand_m.get(mid)
    if b is None or c is None or b["observed"] != c["observed"]:
        changes.append(("mutation", i, mid, b and b["observed"], c and c["observed"]))
    print(json.dumps({"kind": "mutation", "index": i, "id": mid, "base": b and b["observed"], "aligned": c and c["observed"], "same": b is not None and c is not None and b["observed"] == c["observed"]}))
for pid in pids:
    b, c = base_p.get(pid), cand_p.get(pid)
    if b is not True or c is not True:
        changes.append(("must_pass", pid, b, c))
    print(json.dumps({"kind": "must_pass", "id": pid, "base_admitted_as_stated": b, "aligned_admitted_as_stated": c}))
extra = (set(base_m) | set(cand_m)) - set(ids)
assert not extra, extra
print(f"CENSUS mutations {len(ids)} (base lines {len(base_m)}, aligned lines {len(cand_m)}); must_pass {len(pids)} (base {sum(base_p.values())} true, aligned {sum(cand_p.values())} true); changes {len(changes)}")
for ch in changes:
    print("CHANGE", ch)
sys.exit(1 if changes else 0)
