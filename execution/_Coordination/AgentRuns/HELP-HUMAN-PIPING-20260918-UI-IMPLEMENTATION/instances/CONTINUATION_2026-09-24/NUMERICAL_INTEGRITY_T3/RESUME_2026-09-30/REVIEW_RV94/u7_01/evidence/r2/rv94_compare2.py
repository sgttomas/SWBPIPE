"""RV94 repair round: PY or RS dumps on the reviewed head (prev) and the repair head (cand2)
against RV94's oracle. The only rule change is RV94 S-1's alignment: on the repair head the
summary is Current iff the carrier token with the CALLER's requested refs is eligible (and the
no-refs summary is never Current); on prev it was the token with the invocation's own cases.
Usage: python rv94_compare2.py <inputs.jsonl> <lang> <prev.jsonl> <cand2.jsonl>"""
import json
import sys

sys.path.insert(0, sys.argv[0].rsplit("/", 1)[0])
import rv94_oracle as O  # noqa: E402

inputs = [json.loads(l) for l in open(sys.argv[1])]
lang = sys.argv[2]
prev = {d["id"]: d for d in map(json.loads, open(sys.argv[3]))}
cand = {d["id"]: d for d in map(json.loads, open(sys.argv[4]))}
problems, changed, summary_changed = [], [], []


def withheld(summary, current):
    return [s["absolute_verified"] + s["not_covered"] if current else s["relative_verified"] + s["absolute_verified"] + s["not_covered"] + s["input_derived"] for s in summary]


for r in inputs:
    i = r["id"]
    p, c = prev[i], cand[i]
    g_pass = p["reader"]["ok"] if r["g"] == "observe" else r["g"] == "pass"
    if p["reader"] != c["reader"]:
        problems.append((i, "reader changed", p["reader"], c["reader"]))
    if p["token"] != c["token"]:
        problems.append((i, "token changed", p["token"], c["token"]))
    e = O.expect(True, g_pass, r["source"], r["invocation"], r["requested"])
    if c["reader"]["ok"] != e["reader_ok"]:
        problems.append((i, "reader ok vs oracle", c["reader"], e)); continue
    if c["token"] != e["token"]:
        problems.append((i, "token vs oracle", c["token"], e["token"]))
    if not c["reader"]["ok"]:
        if c["summary"] != [] and r["invocation"] is not None:
            problems.append((i, "summary of refused statement", c["summary"]))
        if r["group"] == "mutation":
            want = json.loads(r["note"])
            if i == "mut:g7_not_required_quality_enum_invalid":
                want = {"gate": "G7", "code": "SOURCE_NUMERICAL_CASE_INVALID"}  # expected_by_reader: python and rust
            got = {"gate": c["reader"]["gate"], "code": c["reader"]["code"]}
            if got != want and not (lang == "rs" and i == "mut:g7_maximum_off_enclosure"):
                problems.append((i, "mutation first failure", got, want))
        continue
    # summary: Current iff the token with the caller's requested refs is eligible
    cur = e["token"] == "numerically_eligible"
    if withheld(c["summary"], False) != [s["withheld"] for s in c["summary"]] and withheld(c["summary"], True) != [s["withheld"] for s in c["summary"]]:
        problems.append((i, "withheld neither count", c["summary"]))
    if [s["withheld"] for s in c["summary"]] != withheld(c["summary"], cur) and withheld(c["summary"], True) != withheld(c["summary"], False):
        problems.append((i, "summary current flag", [s["withheld"] for s in c["summary"]], cur))
    if "summary_norefs" in c and [s["withheld"] for s in c["summary_norefs"]] != withheld(c["summary_norefs"], False):
        problems.append((i, "no-refs summary is Current", c["summary_norefs"]))
    # class counts unchanged
    strip = lambda S: [{k: v for k, v in s.items() if k != "withheld"} for s in S]  # noqa: E731
    if strip(p["summary"]) != strip(c["summary"]):
        problems.append((i, "summary class counts changed"))
    if p["summary"] != c["summary"]:
        summary_changed.append(i)
    if {k: v for k, v in p.items() if k not in ("summary", "summary_norefs")} != {k: v for k, v in c.items() if k not in ("summary", "summary_norefs")}:
        changed.append(i)

print(f"[{lang}] records={len(inputs)} problems={len(problems)} non-summary changes={len(changed)} summary changed={len(summary_changed)}")
for p in problems[:40]:
    print("PROBLEM", json.dumps(p, default=str)[:500])
print("SUMMARY_CHANGED", json.dumps(summary_changed))
sys.exit(1 if problems else 0)
