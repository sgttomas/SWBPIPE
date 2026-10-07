"""RV113 (RV-R) comparisons.

census <corpus> <i1.jsonl> <head.jsonl> <out.json>
    Entry by entry: the input bytes' digest and every verdict (bound, unbound,
    transport, standing) at I1 against the head; and each reader's verdict against
    the corpus's own Rust expectation (expected_by_reader.rust, else expected).
probes <probes.json> <i1.jsonl> <head.jsonl> <out.json>
    Each probe's verdict at I1 and the head against this reviewer's expectations.
suites <out.json> <label=log> ...
    Test-by-test outcomes per binary from cargo test logs; pairs labels i1:/head:.
"""
import json
import re
import sys


def load_lines(path):
    return [json.loads(l) for l in open(path) if l.strip()]


# The corpus labels the standing "eligible"; the API names it "numerically_eligible".
STANDING = {"eligible": "numerically_eligible"}


def short(v):
    if "ok" in v:
        return {"admitted": v["ok"]["numerical_eligible"]}
    if "err" in v:
        return {"gate": v["err"]["gate"], "code": v["err"]["code"]}
    return v


def census(corpus, i1p, headp, out):
    d = json.load(open(corpus))
    i1, hd = load_lines(i1p), load_lines(headp)
    assert len(i1) == len(hd) == len(d["cases"]) + len(d["mutations"]) + len(d["must_pass"]), (len(i1), len(hd))
    changes, inputs_differ, expect = [], [], {"i1": [], "head": []}
    counts = {"base": 0, "mutation": 0, "must_pass": 0}
    for a, b in zip(i1, hd):
        assert (a["set"], a["i"], a["id"]) == (b["set"], b["i"], b["id"])
        counts[a["set"]] += 1
        if "materialize_error" in a or "materialize_error" in b:
            changes.append({"id": a["id"], "materialize_error": [a.get("materialize_error"), b.get("materialize_error")]})
            continue
        if a["input_sha256"] != b["input_sha256"]:
            inputs_differ.append(a["id"])
        for k in ("bound", "unbound", "transport", "standing"):
            if a[k] != b[k]:
                changes.append({"set": a["set"], "i": a["i"], "id": a["id"], "verdict": k, "i1": a[k], "head": b[k]})
        # The corpus's own expectation for the bound (invocation) verdict.
        if a["set"] == "mutation":
            m = d["mutations"][a["i"]]
            want = m.get("expected_by_reader", {}).get("rust", m["expected"])
            for lab, line in (("i1", a), ("head", b)):
                got = short(line["bound"])
                if got != {"gate": want["gate"], "code": want["code"]}:
                    expect[lab].append({"id": a["id"], "want": want, "got": got})
        elif a["set"] == "must_pass":
            m = d["must_pass"][a["i"]]
            el = m["expected_eligibility"]
            for lab, line in (("i1", a), ("head", b)):
                ok = line["bound"].get("ok")
                good = ok is not None and ok["invocation_bound"] == el["invocation_bound"] and ok["numerical_eligible"] == el["numerical_eligible"] and line["standing"] == STANDING.get(el["standing"], el["standing"])
                if not good:
                    expect[lab].append({"id": a["id"], "want": el, "got": [line["bound"], line["standing"]]})
        else:
            c = d["cases"][a["i"]]
            for lab, line in (("i1", a), ("head", b)):
                ok = line["bound"].get("ok")
                if ok is None or ok["numerical_eligible"] != c["expected"]["numerical_eligible"]:
                    expect[lab].append({"id": a["id"], "want": c["expected"], "got": line["bound"]})
    res = {"counts": counts, "entries": len(i1), "inputs_differ": inputs_differ, "verdict_changes": changes,
           "corpus_expectation_misses": expect}
    json.dump(res, open(out, "w"), indent=1)
    print(json.dumps({"counts": counts, "inputs_differ": len(inputs_differ), "verdict_changes": len(changes),
                      "misses_i1": len(expect["i1"]), "misses_head": len(expect["head"])}))


def probes(pp, i1p, headp, out):
    ps = json.load(open(pp))
    i1, hd = load_lines(i1p), load_lines(headp)
    assert len(ps) == len(i1) == len(hd)
    rows, bad = [], 0
    for p, a, b in zip(ps, i1, hd):
        assert p["id"] == a["id"] == b["id"]
        if "materialize_error" in a or "materialize_error" in b:
            rows.append({"id": p["id"], "materialize_error": [a.get("materialize_error"), b.get("materialize_error")]})
            bad += 1
            continue
        gi, gh = short(a["bound"]), short(b["bound"])
        observe = "observe" in p["want"]
        ok_i1, ok_h = (True, True) if observe else (gi == p["want_i1"], gh == p["want"])
        bad += (not ok_i1) + (not ok_h)
        rows.append({"id": p["id"], "item": p["item"], "want_head": p["want"], "head": gh, "head_ok": ok_h,
                     "want_i1": p["want_i1"], "i1": gi, "i1_ok": ok_i1, "standing_head": b["standing"],
                     "standing_i1": a["standing"], "same_input": a["input_sha256"] == b["input_sha256"], "note": p.get("note", "")})
    json.dump({"probes": len(ps), "mismatches": bad, "rows": rows}, open(out, "w"), indent=1)
    for r in rows:
        if not (r.get("head_ok") and r.get("i1_ok")):
            print("MISMATCH", json.dumps(r))
    print(json.dumps({"probes": len(ps), "mismatches": bad}))


RUN = re.compile(r"^\s+Running (\S+)(?: \((.*)\))?")
TEST = re.compile(r"^test (\S+) \.\.\. (ok|FAILED|ignored)")


def parse(log):
    out, binary = {}, None
    for line in open(log, errors="replace"):
        m = RUN.match(line)
        if m:
            binary = m.group(1)
            if binary.startswith("unittests"):
                binary = "unittests " + (m.group(2) or "").split("/deps/")[-1].rsplit("-", 1)[0]
            continue
        m = TEST.match(line)
        if m and binary:
            out[f"{binary} :: {m.group(1)}"] = m.group(2)
    return out


def suites(out, pairs):
    logs = dict(p.split("=", 1) for p in pairs)
    res = {}
    for lab, log in logs.items():
        res[lab] = parse(log)
    comp = {}
    for lab in logs:
        if lab.startswith("i1:"):
            name = lab[3:]
            a, b = res[lab], res.get("head:" + name, {})
            keys = sorted(set(a) | set(b))
            diff = [{"test": k, "i1": a.get(k), "head": b.get(k)} for k in keys if a.get(k) != b.get(k)]
            tally = lambda r: {s: sum(1 for v in r.values() if v == s) for s in ("ok", "FAILED", "ignored")}
            comp[name] = {"i1": tally(a), "head": tally(b), "differences": diff,
                          "failed_both": sorted(k for k in keys if a.get(k) == b.get(k) == "FAILED")}
    json.dump({"per_test": res, "comparison": comp}, open(out, "w"), indent=1)
    for name, c in comp.items():
        print(name, json.dumps({k: c[k] for k in ("i1", "head")}), "differences:", len(c["differences"]), "failed_both:", len(c["failed_both"]))
        for x in c["differences"]:
            print("   ", json.dumps(x))


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "census":
        census(*sys.argv[2:6])
    elif cmd == "probes":
        probes(*sys.argv[2:6])
    elif cmd == "suites":
        suites(sys.argv[2], sys.argv[3:])
