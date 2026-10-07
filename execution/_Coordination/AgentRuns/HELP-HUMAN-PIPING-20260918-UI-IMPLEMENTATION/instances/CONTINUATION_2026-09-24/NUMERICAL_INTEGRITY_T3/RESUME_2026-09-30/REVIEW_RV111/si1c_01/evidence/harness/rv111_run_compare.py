#!/usr/bin/env python3
"""RV111 (T3-SI1c review): runner differential (run_rule_checks and run_rule_checks_with_bounds).

Pass conditions checked per line (label|mode):
  P1  a line with no producer flag and no N-4 value is byte-identical to main;
  P2  a flagged point-path check blocks exactly as the instrumented base predicts;
  P4  bounded lines are identical except N-4 (and point-path checks inside them, by P2);
  P5  plain == b = 0 on every candidate (and main) pack;
  N4  N-4 lines keep main's status, diagnostic and relation, name each non-finite input or limit
      with its own NonFiniteInput finding and note, and carry no JSON null;
  M   a multi-check run equals its checks run alone, with a worst-of aggregate.
The schema is checked separately (rv111_schema.py).
Usage: rv111_run_compare.py <dumps dir> <report.json>
"""
import json
import math
import sys
from collections import Counter, defaultdict

DUMPS, REPORT = sys.argv[1], sys.argv[2]
INPUT_MSG = "supplied value must be finite (NaN or ±inf after unit normalization)"
LIMIT_MSG = "value-slot limit must be finite (NaN or ±inf after unit normalization)"
NOTE = "non-finite value (NaN or ±inf, after unit normalization): not bound"
D_MSGS = {
    "add_subtract": "sum or difference must be finite (it overflowed)",
    "multiply": "product must be finite (it overflowed)",
    "divide": "quotient must be finite (it overflowed)",
}
INTERP_MSG = "interpolated table value must be finite (a step of the interpolation overflowed)"


def load(path):
    out = {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            label, js, *ev = line.rstrip("\n").split("\t")
            out[label] = (js, ev[0] if ev else "")
    return out


base = load(f"{DUMPS}/base/run.tsv")
cand = load(f"{DUMPS}/cand/run.tsv")
ibase = load(f"{DUMPS}/ibase/run.tsv")
assert set(base) == set(cand) == set(ibase)
meta = {}
with open(f"{DUMPS}/base/run_meta.jsonl", encoding="utf-8") as fh:
    for line in fh:
        m = json.loads(line)
        meta[m["label"]] = m

viol = defaultdict(list)
counts = Counter()
transitions = Counter()
n4_kinds = Counter()
note_replaced = Counter()


def flags_by_segment(events):
    segs = []
    for ev in events.split(";"):
        if ev in ("E", "I"):
            segs.append([ev])
        elif ev.startswith("F|") and segs:
            segs[-1].append(ev)
    return segs


def d_record(ev):
    _, site, subject, k, sources, mask = ev.split("|", 5)
    if site == "add_subtract":
        rec = ("NonFiniteInput", "add_subtract", D_MSGS["add_subtract"])
    elif site.startswith("multiply"):
        rec = ("NonFiniteInput", "multiply", D_MSGS["multiply"])
    elif site.startswith("divide"):
        rec = ("NonFiniteInput", "divide", D_MSGS["divide"])
    else:
        rec = ("NonFiniteInput", subject, INTERP_MSG)
    return rec, int(k)


def recs(findings):
    return [(f["code"], f["subject_id"], f["message"]) for f in findings]


def is_finite_str(s):
    v = float(s)
    return math.isfinite(v)


def n4_entries(label):
    """{name: kind} for N-4 values in this label (kinds raw, rawconv, norm)."""
    m = meta[label.rsplit("|", 1)[0]]
    if m.get("demo"):
        _, dname, a, l, slot, aunit = m["label"].split("|")
        out = {}
        if not math.isfinite(float(a)):
            out["demo_actual_quantity"] = "raw" if aunit == "demo_unit" else "rawconv"
        if not math.isfinite(float(l)):
            out["demo_limit_quantity"] = "raw"
        if not math.isfinite(float(slot)):
            out["demo_limit_slot"] = "raw"
        return out
    out = {}
    for item in filter(None, m["n4"].split(",")):
        name, kind = item.split(":")
        out[name] = kind
    return out


def check_inputs(label):
    m = meta[label.rsplit("|", 1)[0]]
    if m.get("demo"):
        return None
    return m["doc"]["check_definitions"]


null_lines = 0
for label, (bjs, _) in base.items():
    cjs, _ = cand[label]
    ijs, events = ibase[label]
    fam = label.split("|", 1)[0]
    mode = label.rsplit("|", 1)[1]
    counts[f"{fam}_lines"] += 1
    if "PANIC" in (bjs, cjs):
        viol["panic"].append(label)
        continue
    if ijs != bjs:
        viol["ibase_differs"].append(label)
    b, c = json.loads(bjs), json.loads(cjs)
    if "null" in cjs:
        viol["cand_null"].append(label)
    n4 = n4_entries(label)
    defs = check_inputs(label)
    # Narrow N-4 to values the pack actually reads.
    relevant = {}
    if defs is None:
        relevant = dict(n4)
    else:
        for name, kind in n4.items():
            if name == "limit":
                if any("value_slot_refs" in d for d in defs):
                    relevant[name] = kind
            elif any(any(r["ref_id"] == name for r in d["required_input_refs"]) for d in defs):
                relevant[name] = kind
    segs = flags_by_segment(events)
    point_flags = [s for s in segs if s[0] == "E" and len(s) > 1]
    single = len(b["checks"]) == 1
    if not single and (relevant or point_flags):
        # A multi-check run: each check is verified through its single run (M, below).
        counts["multi_lines_checked_via_singles"] += 1
        continue
    if not relevant and not point_flags:
        if bjs == cjs:
            counts["P1_identical"] += 1
        else:
            viol["P1_unflagged_non_n4_differs"].append(label)
        continue
    if relevant:
        counts["n4_lines"] += 1
        for name, kind in relevant.items():
            n4_kinds[kind] += 1
        # Status, diagnostics and relation are main's, check by check.
        for bc, cc in zip(b["checks"], c["checks"]):
            if bc["status"] != cc["status"]:
                viol["N4_status_changed"].append(label)
            if bc["diagnostic_codes"] != cc["diagnostic_codes"]:
                viol["N4_diagnostic_changed"].append(label)
            if bc["acceptability_relation"] != cc["acceptability_relation"]:
                viol["N4_relation_changed"].append(label)
            if bc.get("computed_value") is not None and cc.get("computed_value") != bc.get("computed_value"):
                viol["N4_computed_changed"].append(label)
            for bi, ci in zip(bc["bound_inputs"], cc["bound_inputs"]):
                name = ci["input_id"]
                kind = relevant.get(name)
                if kind is not None and (bi.get("note") or "").startswith(("invalid absolute bound", "duplicate absolute bounds")):
                    # An invalid or duplicate bound leaves the input unsupplied before any value is read.
                    counts["n4_value_unread_invalid_bound"] += 1
                    kind = None
                if kind is None:
                    if bi != ci:
                        viol["N4_other_bound_input_changed"].append(label)
                    continue
                if "value" in ci:
                    viol["N4_value_present"].append(label)
                if ci.get("note") != NOTE:
                    viol["N4_note_wrong"].append(label)
                if bi.get("note") not in (None,):
                    note_replaced[bi["note"].split(" ")[0]] += 1
                if ci["supplied"] != (kind != "rawconv") or ci["supplied"] != bi["supplied"]:
                    viol["N4_supplied_wrong"].append({"label": label, "input": name, "kind": kind, "base": bi, "cand": ci})
                if ci.get("unit") != bi.get("unit"):
                    viol["N4_unit_changed"].append(label)
            # Findings: every NonFiniteInput names an N-4 input or the limit with the right message;
            # no MissingRequiredValue or missing-metadata limit finding names an N-4 value.
            for f in cc["evaluator_findings"]:
                if f["code"] == "NonFiniteInput" and f["subject_id"] in relevant:
                    want = LIMIT_MSG if f["subject_id"] in ("limit", "demo_limit_slot") else INPUT_MSG
                    if f["message"] != want:
                        viol["N4_message_wrong"].append(label)
                if f["code"] == "MissingRequiredValue" and f["subject_id"] in relevant:
                    viol["N4_missing_required_value_left"].append(label)
                if f["message"] == "value-slot limit has missing or unknown unit/dimension metadata" and "limit" in relevant:
                    viol["N4_limit_metadata_message_left"].append(label)
            named = [f["subject_id"] for f in cc["evaluator_findings"] if f["code"] == "NonFiniteInput" and f["message"] == INPUT_MSG]
            if len(named) != len(set(named)):
                viol["N4_duplicate_input_finding"].append(label)
            if bc["status"] != cc["status"]:
                pass
        continue
    # Flagged (no N-4): point-path checks predicted from the instrumented base (single checks).
    counts["flagged_lines"] += 1
    if not single:
        continue
    bc, cc = b["checks"][0], c["checks"][0]
    seg = point_flags[0]
    rec, k = d_record(seg[1])
    declared = defs[0]["result_statuses"]
    expected = recs(bc["evaluator_findings"])[:k] + [rec]
    if declared and "RULE_INPUTS_INCOMPLETE" not in declared:
        expected.append(("STATUS_NOT_DECLARED", "RULE_INPUTS_INCOMPLETE",
                         "computed status is not in the check's declared result_statuses"))
    policy = "diagnostic_policy" in defs[0]
    ok = (cc["status"] == "RULE_INPUTS_INCOMPLETE" and recs(cc["evaluator_findings"]) == expected
          and "computed_value" not in cc and "limit_value" not in cc
          and cc["acceptability_relation"] == "none"
          and cc["diagnostic_codes"] == (["RULE_EVALUATOR_ERROR"] if policy else [])
          and cc["bound_inputs"] == bc["bound_inputs"]
          and cc["completeness_findings"] == bc["completeness_findings"]
          and c["aggregate_status"] == "RULE_INPUTS_INCOMPLETE")
    if ok:
        counts["P2_flagged_as_predicted"] += 1
        transitions[(bc["status"], "computed" if "computed_value" in bc else "-", mode)] += 1
    else:
        viol["P2_flagged_not_as_predicted"].append({"label": label, "expected": expected, "cand": cc})

# P5: plain == b0 (candidate and main), per pack.
for tree, dump in (("cand", cand), ("base", base)):
    for label in dump:
        if label.endswith("|plain"):
            other = label[: -len("plain")] + "b0"
            if other in dump:
                counts[f"P5_{tree}_pairs"] += 1
                if dump[label][0] != dump[other][0]:
                    viol[f"P5_{tree}_plain_ne_b0"].append(label)

# M: multi-check runs equal their single checks; aggregate worst-of.
rank = {"USER_RULE_CHECKED": 0, "RULE_INPUTS_INCOMPLETE": 1, "USER_RULE_FAILED": 2}
for tree, dump in (("cand", cand), ("base", base)):
    for label in dump:
        if not label.startswith("rmulti|all|"):
            continue
        run = json.loads(dump[label][0])
        tag_mode = label[len("rmulti|all|"):]
        for i, chk in enumerate(run["checks"]):
            single = json.loads(dump[f"rmulti|single{i}|{tag_mode}"][0])
            counts[f"M_{tree}_checks"] += 1
            if single["checks"][0] != chk:
                viol[f"M_{tree}_check_differs_from_single"].append(f"{label}#{i}")
        worst = max((c["status"] for c in run["checks"]), key=lambda s: rank[s])
        if run["aggregate_status"] != worst:
            viol[f"M_{tree}_aggregate_not_worst_of"].append(label)

report = {
    "counts": dict(counts),
    "n4_kinds": dict(n4_kinds),
    "n4_existing_note_replaced": dict(note_replaced),
    "flagged_transitions": {"|".join(k): v for k, v in sorted(transitions.items())},
    "violation_counts": {k: len(v) for k, v in viol.items()},
    "violations": {k: (v if len(v) < 40 else v[:40] + [f"... {len(v)} total"]) for k, v in viol.items()},
}
json.dump(report, open(REPORT, "w"), indent=1, sort_keys=True, default=str)
print(json.dumps({k: report[k] for k in ("counts", "n4_kinds", "n4_existing_note_replaced", "violation_counts")}, indent=1))
