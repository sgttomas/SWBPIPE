"""RV94: TS dumps against RV94's oracle, and cross-language parity with PY and RS.
Usage: python rv94_compare_ts.py (run in the scratch folder)."""
import json
import sys

sys.path.insert(0, ".")
import rv94_oracle as O  # noqa: E402

inputs = [json.loads(l) for l in open("inputs.jsonl")]
load = lambda f: {d["id"]: d for d in map(json.loads, open(f))}  # noqa: E731
tsb, tsc, pyb, pyc, rsc = load("ts_base.jsonl"), load("ts_cand.jsonl"), load("py_base.jsonl"), load("py_cand.jsonl"), load("rs_cand.jsonl")
problems, changed, eligible, xlang = [], [], [], []
G7_DECLARED = {"SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE", "SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS"}


def want_withheld(py_summary, current):
    if not isinstance(py_summary, list):
        return None
    return [s["absolute_verified"] + s["not_covered"] if current else s["relative_verified"] + s["absolute_verified"] + s["not_covered"] + s["input_derived"] for s in py_summary]


for r in inputs:
    i = r["id"]
    b, c = tsb[i], tsc[i]
    g_pass = pyb[i]["reader"]["ok"] if r["g"] == "observe" else r["g"] == "pass"
    # reader: non-eligibility fields identical base->cand; eligibility per oracle
    nb = {k: v for k, v in b["reader"].items() if k not in ("numerical_eligible", "standing")}
    nc = {k: v for k, v in c["reader"].items() if k not in ("numerical_eligible", "standing")}
    if nb != nc:
        problems.append((i, "reader non-eligibility changed", nb, nc))
    for lane, d, flag in (("base", b, False), ("cand", c, True)):
        e = O.expect(flag, g_pass, r["source"], r["invocation"], r["requested"])
        rd = d["reader"]
        if rd["ok"] != e["reader_ok"]:
            problems.append((i, lane, "reader ok", rd, e)); continue
        if rd["ok"] and (rd["invocation_bound"], rd["numerical_eligible"], rd["standing"]) != (e["invocation_bound"], e["numerical_eligible"], e["standing_label"]):
            problems.append((i, lane, "reader eligibility", rd, e))
        if "seam_live" in d:
            for key, live in (("seam_live", True), ("seam_nolive", False)):
                t = O.ts_seam_expect(flag, g_pass, r["source"], r["invocation"], r["requested"], live)
                got = d[key]
                if (got["standing"], got["eligible"], got["status"]) != (t["standing"], t["eligible"], t["status"]):
                    problems.append((i, lane, key, got, t))
                if "finding" in t and got["findings"] != [t["finding"]]:
                    problems.append((i, lane, key, "finding", got["findings"], t["finding"]))
                if t["summary"] == "empty":
                    if got["summary"] != []:
                        problems.append((i, lane, key, "summary not empty", got["summary"]))
                else:
                    pys = (pyc if lane == "cand" else pyb)[i]["summary"]
                    w = want_withheld(pys, t["summary"] == "current") if pys else None
                    if w is not None and got["summary"] != w:
                        problems.append((i, lane, key, "withheld", got["summary"], w))
            if lane == "cand" and d["seam_live"]["standing"] == "numerically_eligible":
                eligible.append(i)
    if b != c:
        changed.append(i)
    # cross-language (candidate): reader gate/code, token vs TS seam_live token
    p, s_ = pyc[i]["reader"], rsc[i]["reader"]
    t = c["reader"]
    codes = {("py", p.get("code")), ("rs", s_.get("code")), ("ts", t.get("code"))}
    if not (p["ok"] == s_["ok"] == t["ok"]):
        xlang.append((i, "accept differs", p, s_, t))
    elif not p["ok"]:
        if not (p["gate"] == s_["gate"] == t["gate"]) or (p["code"] != t["code"]) or (s_["code"] != p["code"] and s_["code"] not in G7_DECLARED):
            xlang.append((i, "gate/code differs", codes))
    else:
        for k in ("invocation_bound", "numerical_eligible", "publication_sha256"):
            if not (p[k] == s_[k] == t[k]):
                xlang.append((i, k, p[k], s_[k], t[k]))
    if "seam_live" in c and pyc[i]["token"] != c["seam_live"]["standing"]:
        xlang.append((i, "token PY/RS vs TS(seam, live)", pyc[i]["token"], rsc[i]["token"], c["seam_live"]["standing"], c["seam_live"]["findings"]))

print(f"[ts] records={len(inputs)} problems={len(problems)} changed={len(changed)} seam_live_eligible={len(eligible)} xlang={len(xlang)}")
for p in problems[:40]:
    print("PROBLEM", json.dumps(p, default=str)[:500])
for x in xlang:
    print("XLANG", json.dumps(x, default=str)[:400])
print("CHANGED", json.dumps(changed))
print("ELIGIBLE", json.dumps(eligible))
