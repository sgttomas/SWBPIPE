"""RV113 (RV-R) comparisons for the Python reader.

census <corpus> <py_i1.jsonl> <py_head.jsonl> <rs_head.jsonl> <ts_head.jsonl> <out.json>
    Entry by entry: PY's verdicts (bound, unbound, transport) at I1 against the head; PY's bound verdict against the
    corpus's own expectation (expected_by_reader.python, else expected); and PY's bound verdict against RS's and TS's.
probes <probes.json> <py_i1.jsonl> <py_head.jsonl> <rs_head.jsonl> <ts_head.jsonl> <out.json>
    Each probe: PY at I1 and the head, against RS and TS at their heads and this reviewer's head expectation.
scdiff <i1.jsonl> <head.jsonl> <out.json>
    `_source_contract`: every input whose I1 outcome was not a TypeError must read identically at the head.
"""
import collections
import json
import sys

# The corpus and the PY and TS APIs name the standing "eligible"; the RS census records "numerically_eligible".
STANDING = {"numerically_eligible": "eligible"}


WT = "WT"


def norm(text):
    """The two copies differ only by their root; the outcome records the contract file's path."""
    for v in ("py-i1", "py-head"):
        text = text.replace(f"{WT}/rv113/{v}/projects/chirality-piping", "P")
    return text.replace(WT, "WT")


def load(path):
    return [json.loads(norm(l)) for l in open(path) if l.strip()]


def short(v):
    if v is None:
        return None
    if "ok" in v:
        return {"admitted": v["ok"]["numerical_eligible"]}
    if "err" in v:
        return {"gate": v["err"]["gate"], "code": v["err"]["code"]}
    return v


def standing(v):
    return v["ok"].get("standing") if v and "ok" in v else None


def census(corpus, i1p, hp, rsp, tsp, out):
    d = json.load(open(corpus))
    i1, hd, rs, ts = load(i1p), load(hp), load(rsp), load(tsp)
    n = len(d["cases"]) + len(d["mutations"]) + len(d["must_pass"])
    assert len(i1) == len(hd) == len(rs) == len(ts) == n, (len(i1), len(hd), len(rs), len(ts), n)
    changes, inputs_differ, miss, vs_rs, vs_ts, escapes = [], [], {"i1": [], "head": []}, [], [], []
    counts = collections.Counter()
    for a, b, r, t in zip(i1, hd, rs, ts):
        assert (a["set"], a["i"], a["id"]) == (b["set"], b["i"], b["id"]) == (r["set"], r["i"], r["id"]) == (t["set"], t["i"], t["id"])
        counts[a["set"]] += 1
        if any("materialize_error" in x for x in (a, b)):
            changes.append({"id": a["id"], "materialize_error": [a.get("materialize_error"), b.get("materialize_error")]}); continue
        if a["input_sha256"] != b["input_sha256"]:
            inputs_differ.append(a["id"])
        for k in ("bound", "unbound", "transport"):
            if a[k] != b[k]:
                changes.append({"set": a["set"], "id": a["id"], "verdict": k, "i1": a[k], "head": b[k]})
            if "escape" in b[k]:
                escapes.append({"id": a["id"], "verdict": k, "head": b[k]})
        if short(b["bound"]) != short(r["bound"]) or ("ok" in b["bound"] and standing(b["bound"]) != STANDING.get(r.get("standing"), r.get("standing"))):
            vs_rs.append({"id": a["id"], "py": short(b["bound"]), "rs": short(r["bound"])})
        if short(b["bound"]) != short(t["bound"]):
            vs_ts.append({"id": a["id"], "py": short(b["bound"]), "ts": short(t["bound"])})
        if a["set"] == "mutation":
            m = d["mutations"][a["i"]]
            want = m.get("expected_by_reader", {}).get("python", m["expected"])
            for lab, line in (("i1", a), ("head", b)):
                if short(line["bound"]) != {"gate": want["gate"], "code": want["code"]}:
                    miss[lab].append({"id": a["id"], "want": want, "got": short(line["bound"])})
        elif a["set"] == "must_pass":
            el = d["must_pass"][a["i"]]["expected_eligibility"]
            for lab, line in (("i1", a), ("head", b)):
                ok = line["bound"].get("ok")
                if not (ok and ok["invocation_bound"] == el["invocation_bound"] and ok["numerical_eligible"] == el["numerical_eligible"]
                        and ok["standing"] == el["standing"]):
                    miss[lab].append({"id": a["id"], "want": el, "got": line["bound"]})
        else:
            c = d["cases"][a["i"]]
            for lab, line in (("i1", a), ("head", b)):
                ok = line["bound"].get("ok")
                if ok is None or ok["numerical_eligible"] != c["expected"]["numerical_eligible"]:
                    miss[lab].append({"id": a["id"], "want": c["expected"], "got": line["bound"]})
    res = {"counts": counts, "entries": n, "inputs_differ": inputs_differ, "verdict_changes_i1_to_head": changes,
           "corpus_expectation_misses": miss, "head_py_vs_rs": vs_rs, "head_py_vs_ts": vs_ts, "escapes_head": escapes}
    json.dump(res, open(out, "w"), indent=1, sort_keys=True)
    print(json.dumps({"counts": counts, "inputs_differ": len(inputs_differ), "verdict_changes": len(changes),
                      "misses_i1": len(miss["i1"]), "misses_head": len(miss["head"]), "py_vs_rs": len(vs_rs),
                      "py_vs_ts": len(vs_ts), "escapes": len(escapes)}))


def probes(pp, i1p, hp, rsp, tsp, out):
    ps = json.load(open(pp))
    i1, hd, rs, ts = load(i1p), load(hp), load(rsp), load(tsp)
    assert len(ps) == len(i1) == len(hd) == len(rs) == len(ts)
    rows = []
    for p, a, b, r, t in zip(ps, i1, hd, rs, ts):
        assert p["id"] == a["id"] == b["id"] == r["id"] == t["id"]
        py_i1, py_h, rsv, tsv = short(a.get("bound")), short(b.get("bound")), short(r.get("bound")), short(t.get("bound"))
        rows.append({"id": p["id"], "item": p.get("item"), "want_head": p.get("want"), "py_i1": py_i1, "py_head": py_h,
                     "rs_head": rsv, "ts_head": tsv, "py_eq_rs": py_h == rsv, "py_eq_ts": py_h == tsv,
                     "py_changed": py_i1 != py_h, "materialize_error": b.get("materialize_error"), "note": p.get("note", "")})
    json.dump({"probes": len(ps), "rows": rows}, open(out, "w"), indent=1, sort_keys=True)
    for r in rows:
        if not (r["py_eq_rs"] and r["py_eq_ts"]):
            print("DIFF", r["id"], "py", r["py_head"], "rs", r["rs_head"], "ts", r["ts_head"])
    print(json.dumps({"probes": len(ps), "py_ne_rs": sum(not r["py_eq_rs"] for r in rows), "py_ne_ts": sum(not r["py_eq_ts"] for r in rows),
                      "py_changed_i1_head": sum(r["py_changed"] for r in rows)}))


def scdiff(i1p, hp, out):
    i1, hd = load(i1p), load(hp)
    assert [x["id"] for x in i1] == [x["id"] for x in hd]
    same, differ, typeerr = 0, [], collections.Counter()
    for a, b in zip(i1, hd):
        if a["outcome"][0] == "TypeError":
            typeerr[(a["outcome"][0], b["outcome"][0])] += 1
            if b["outcome"][0] != "ValueError":
                differ.append({"id": a["id"], "i1": a["outcome"], "head": b["outcome"], "class": "typeerror_not_valueerror"})
            continue
        if a["outcome"] == b["outcome"]:
            same += 1
        else:
            differ.append({"id": a["id"], "i1": a["outcome"], "head": b["outcome"], "class": "non_typeerror_changed"})
    kinds = collections.Counter(a["outcome"][0] for a in i1)
    head_msgs = collections.Counter(b["outcome"][1] for a, b in zip(i1, hd) if a["outcome"][0] == "TypeError")
    res = {"inputs": len(i1), "i1_outcome_kinds": kinds, "non_typeerror_identical": same, "differ": differ,
           "typeerror_transitions": {f"{k[0]}->{k[1]}": v for k, v in typeerr.items()}, "head_messages_for_i1_typeerrors": head_msgs}
    json.dump(res, open(out, "w"), indent=1, sort_keys=True)
    print(json.dumps({"inputs": len(i1), "i1_kinds": kinds, "non_typeerror_identical": same, "differ": len(differ),
                      "typeerror_transitions": res["typeerror_transitions"], "head_msgs": head_msgs}))


if __name__ == "__main__":
    cmd = sys.argv[1]
    {"census": census, "probes": probes, "scdiff": scdiff}[cmd](*sys.argv[2:])
