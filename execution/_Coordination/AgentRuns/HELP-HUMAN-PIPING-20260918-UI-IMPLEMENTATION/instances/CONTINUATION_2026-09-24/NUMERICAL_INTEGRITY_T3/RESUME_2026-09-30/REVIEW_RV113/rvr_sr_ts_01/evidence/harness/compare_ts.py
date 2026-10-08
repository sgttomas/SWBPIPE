"""RV113 (RV-R) comparisons for the TypeScript reader.

census <corpus> <ts_i1.jsonl> <ts_head.jsonl> <out.json> [<rs_head census jsonl>]
    TS at I1 against TS at the head, entry by entry (input digest; bound, unbound and transport verdicts), and each
    side against the corpus's own TypeScript expectation (expected_by_reader.typescript, else expected). With RS's
    census, also the bound (gate, code) or eligibility of TS against RS, entry by entry.
probes <probes.json> <ts_i1.jsonl> <ts_head.jsonl> <rs_i1.jsonl> <rs_head.jsonl> <out.json>
    Each probe's verdict in TS (I1, head) against RS (I1, head) and against this reviewer's RS expectation.
"""
import json
import sys


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def short(v):
    if v is None:
        return None
    if "ok" in v:
        return {"admitted": v["ok"]["numerical_eligible"]}
    if "err" in v:
        return {"gate": v["err"]["gate"], "code": v["err"]["code"]}
    return v


def census(corpus, i1p, hp, out, rsp=None):
    d = json.load(open(corpus))
    a, b = load(i1p), load(hp)
    rs = load(rsp) if rsp else None
    assert len(a) == len(b) == len(d["cases"]) + len(d["mutations"]) + len(d["must_pass"])
    changes, inputs, misses, parity = [], [], {"i1": [], "head": []}, []
    for k, (x, y) in enumerate(zip(a, b)):
        assert (x["set"], x["i"], x["id"]) == (y["set"], y["i"], y["id"])
        if "materialize_error" in x or "materialize_error" in y:
            changes.append({"id": x["id"], "materialize_error": [x.get("materialize_error"), y.get("materialize_error")]})
            continue
        if x["input_sha256"] != y["input_sha256"]:
            inputs.append(x["id"])
        for key in ("bound", "unbound", "transport"):
            if x[key] != y[key]:
                changes.append({"set": x["set"], "id": x["id"], "verdict": key, "i1": x[key], "head": y[key]})
        if x["set"] == "mutation":
            m = d["mutations"][x["i"]]
            want = m.get("expected_by_reader", {}).get("typescript", m["expected"])
            for lab, line in (("i1", x), ("head", y)):
                if short(line["bound"]) != {"gate": want["gate"], "code": want["code"]}:
                    misses[lab].append({"id": x["id"], "want": want, "got": short(line["bound"])})
        elif x["set"] == "must_pass":
            el = d["must_pass"][x["i"]]["expected_eligibility"]
            for lab, line in (("i1", x), ("head", y)):
                ok = line["bound"].get("ok")
                if not (ok and ok["invocation_bound"] == el["invocation_bound"] and ok["numerical_eligible"] == el["numerical_eligible"] and ok["standing"] == el["standing"]):
                    misses[lab].append({"id": x["id"], "want": el, "got": line["bound"]})
        else:
            c = d["cases"][x["i"]]
            for lab, line in (("i1", x), ("head", y)):
                ok = line["bound"].get("ok")
                if not ok or ok["numerical_eligible"] != c["expected"]["numerical_eligible"]:
                    misses[lab].append({"id": x["id"], "want": c["expected"], "got": line["bound"]})
        if rs is not None:
            r = rs[k]
            assert r["id"] == y["id"]
            if short(r["bound"]) != short(y["bound"]):
                parity.append({"set": y["set"], "id": y["id"], "rs_head": short(r["bound"]), "ts_head": short(y["bound"])})
    res = {"entries": len(a), "inputs_differ": inputs, "verdict_changes": changes, "expectation_misses": misses,
           "ts_vs_rs_bound_differences": parity}
    json.dump(res, open(out, "w"), indent=1)
    print(json.dumps({"entries": len(a), "inputs_differ": len(inputs), "verdict_changes": len(changes),
                      "misses_i1": len(misses["i1"]), "misses_head": len(misses["head"]),
                      "ts_vs_rs_differences": None if rs is None else len(parity)}))
    for p in parity:
        print("  TS/RS:", json.dumps(p))


def probes(pp, ti, th, ri, rh, out):
    ps = json.load(open(pp))
    ti, th, ri, rh = load(ti), load(th), load(ri), load(rh)
    rows = []
    for k, p in enumerate(ps):
        row = {"id": p["id"], "item": p["item"], "want_rs_head": p["want"], "want_rs_i1": p["want_i1"],
               "ts_i1": short(ti[k]["bound"]), "ts_head": short(th[k]["bound"]),
               "rs_i1": short(ri[k]["bound"]) if k < len(ri) else None, "rs_head": short(rh[k]["bound"]),
               "ts_standing_head": th[k]["bound"].get("ok", {}).get("standing")}
        assert th[k]["id"] == p["id"] == rh[k]["id"] == ti[k]["id"]
        row["ts_head_equals_rs_head"] = row["ts_head"] == row["rs_head"]
        row["rs_head_as_expected"] = "observe" in p["want"] or row["rs_head"] == p["want"]
        rows.append(row)
    diffs = [r for r in rows if not r["ts_head_equals_rs_head"]]
    json.dump({"probes": len(ps), "ts_head_vs_rs_head_differences": len(diffs), "rows": rows}, open(out, "w"), indent=1)
    print(json.dumps({"probes": len(ps), "ts_head_vs_rs_head_differences": len(diffs),
                      "rs_head_not_as_expected": sum(1 for r in rows if not r["rs_head_as_expected"])}))
    for r in diffs:
        print("  ", r["id"], "TS", r["ts_head"], "RS", r["rs_head"], "| TS@I1", r["ts_i1"])


if __name__ == "__main__":
    if sys.argv[1] == "census":
        census(*sys.argv[2:7])
    else:
        probes(*sys.argv[2:8])
