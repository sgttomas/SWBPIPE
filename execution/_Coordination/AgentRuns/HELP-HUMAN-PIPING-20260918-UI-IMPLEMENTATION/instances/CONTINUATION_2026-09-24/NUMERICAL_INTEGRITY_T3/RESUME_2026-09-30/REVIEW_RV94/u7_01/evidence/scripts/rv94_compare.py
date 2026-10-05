"""RV94: compare a language's base and candidate dumps with RV94's oracle.
Usage: python rv94_compare.py <inputs.jsonl> <lang> <base.jsonl> <cand.jsonl>
Prints mismatches and the changed set; exit 1 on any mismatch."""
import json
import sys

sys.path.insert(0, sys.argv[0].rsplit("/", 1)[0])
import rv94_oracle as O  # noqa: E402

inputs = [json.loads(l) for l in open(sys.argv[1])]
lang = sys.argv[2]
base = {d["id"]: d for d in map(json.loads, open(sys.argv[3]))}
cand = {d["id"]: d for d in map(json.loads, open(sys.argv[4]))}
problems, changed, eligible_ids = [], [], []
ELIG = {"numerical_eligible", "standing"}


def withheld_ok(summary, current):
    for s in summary:
        want = s["absolute_verified"] + s["not_covered"] if current else s["relative_verified"] + s["absolute_verified"] + s["not_covered"] + s["input_derived"]
        if s["withheld"] != want or s["interval_bindable"] != 0:
            return False
    return True


for r in inputs:
    i = r["id"]
    b, c = base[i], cand[i]
    g = r["g"]
    if g == "observe":
        g_pass = b["reader"]["ok"]
    else:
        g_pass = g == "pass"
    # 1. gate identity: base == candidate on every non-eligibility field
    rb, rc = dict(b["reader"]), dict(c["reader"])
    for k in ELIG:
        rb.pop(k, None); rc.pop(k, None)
    if rb != rc:
        problems.append((i, "reader non-eligibility fields changed", rb, rc))
    # summaries: identical apart from withheld
    sb = [{k: v for k, v in s.items() if k != "withheld"} for s in b["summary"]] if isinstance(b["summary"], list) else b["summary"]
    sc = [{k: v for k, v in s.items() if k != "withheld"} for s in c["summary"]] if isinstance(c["summary"], list) else c["summary"]
    if sb != sc:
        problems.append((i, "summary class counts changed", sb, sc))
    # 2. oracle, both lanes
    for lane, d, flag in (("base", b, False), ("cand", c, True)):
        e = O.expect(flag, g_pass, r["source"], r["invocation"], r["requested"])
        rd = d["reader"]
        if rd["ok"] != e["reader_ok"]:
            problems.append((i, lane, "reader ok", rd["ok"], e["reader_ok"])); continue
        if rd["ok"]:
            if rd["invocation_bound"] != e["invocation_bound"] or rd["numerical_eligible"] != e["numerical_eligible"]:
                problems.append((i, lane, "reader eligibility", rd, e))
            if "standing" in rd and rd["standing"] != e["standing_label"]:
                problems.append((i, lane, "reader label", rd["standing"], e["standing_label"]))
            if not isinstance(d["summary"], list) or not d["summary"] or not withheld_ok(d["summary"], e["summary_current"]):
                problems.append((i, lane, "summary", d["summary"], e["summary_current"]))
        else:
            if r["g"] == "fail" and r["group"] == "mutation":
                want = json.loads(r["note"])
                if {"gate": rd["gate"], "code": rd["code"]} != want:
                    problems.append((i, lane, "mutation first failure", rd, want))
            if d["summary"] != [] and r["invocation"] is not None:
                problems.append((i, lane, "summary of refused statement", d["summary"]))
        if d["token"] != e["token"]:
            problems.append((i, lane, "token", d["token"], e["token"]))
        if lane == "cand" and e["token"] == "numerically_eligible":
            eligible_ids.append(i)
    if b != c:
        changed.append(i)

print(f"[{lang}] records={len(inputs)} problems={len(problems)} changed(base->cand)={len(changed)} cand_token_eligible={len(eligible_ids)}")
for p in problems[:60]:
    print("PROBLEM", json.dumps(p, default=str)[:600])
print("CHANGED", json.dumps(changed))
print("ELIGIBLE", json.dumps(eligible_ids))
sys.exit(1 if problems else 0)
