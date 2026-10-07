"""RV108: every corpus entry in each reader against its own expectation; base vs head differentials."""
import json, sys
S = sys.argv[1]
def load(n): return {json.loads(l)["id"]: json.loads(l) for l in open(f"{S}/out/{n}.jsonl")}
entries = {e["id"]: e for e in json.load(open(f"{S}/probes/corpus_entries.json"))}
py, pyb, rs, ts, tsb = load("py_cand_corpus_entries"), load("py_base_corpus_entries"), load("rs_corpus"), load("ts_cand_corpus"), load("ts_base_corpus")
lang = {"python": py, "rust": rs, "typescript": ts}
def obs(x):
    if x.get("ok"): return "pass"
    return {"gate": x["gate"], "code": x["code"]} if "gate" in x else x
fails = []
for eid, e in entries.items():
    for name, out in lang.items():
        o = obs(out[eid]["raw_inv"])
        if e["family"] == "corpus_mutation":
            want = (e.get("expected_by_reader") or {}).get(name, e["expected"])
            if o != want: fails.append((eid, name, want, o))
        else:
            if o != "pass": fails.append((eid, name, "pass", o))
            want_el = e["expected_eligibility"]["numerical_eligible"] if e["family"] == "corpus_must_pass" else None
            if want_el is not None and out[eid]["raw_inv"].get("eligible") != want_el: fails.append((eid, name, "eligible", want_el, out[eid]["raw_inv"]))
print("entries", len(entries), "x 3 readers; failures against own expectation:", len(fails))
for f in fails: print("  FAIL", f)
print("07m entries 286-293 and 277, observed in each reader:")
for eid in entries:
    i = eid.split(":")
    if i[0] == "mutation" and (int(i[1]) >= 286 or int(i[1]) == 277):
        print("  ", eid, "| py", obs(py[eid]["raw_inv"]), "| rs", obs(rs[eid]["raw_inv"]), "| ts", obs(ts[eid]["raw_inv"]), "| ts@base", obs(tsb[eid]["raw_inv"]), "| expected", entries[eid]["expected"])
def diff(a, b, label, raw_only):
    d = []
    for eid in a:
        for ep in a[eid]:
            if ep in ("id", "family") or (raw_only and ep not in ("raw_inv", "raw_none", "c_raw")): continue
            if a[eid][ep] != b[eid][ep]: d.append((eid, ep, a[eid][ep], b[eid][ep]))
    print(label, "differences:", len(d))
    for x in d: print("   ", x)
diff(tsb, ts, "TS base vs head (all entry points), over the 07m corpus:", False)
diff(pyb, py, "Python base vs head, raw entry points (raw_inv, raw_none, c_raw), over the 07m corpus:", True)
tdiff = [(eid, pyb[eid]["c_transport"], py[eid]["c_transport"]) for eid in py if pyb[eid]["c_transport"] != py[eid]["c_transport"]]
from collections import Counter
print("Python base vs head, transport dispatch: changed", len(tdiff), Counter((json.dumps(a)[:70], json.dumps(b)[:90]) for _, a, b in tdiff).most_common(10))
