#!/usr/bin/env python3
"""I61 U7 slice A: the eligibility oracle (stdlib only; imports no product, reader or test code).

It derives the expected post-U7 eligibility from the contract texts alone:
  C1  = R/I32/f2a_wire_c1/WIRE_CONTRACT.md:160 ("Numerically eligible requires all applicable G
        checks with invocation, MECHANICS_SOLVED, requested load-case refs/order and each case
        selected or ordinarily eligible not_required ... An unavailable case, missing invocation
        or scope yields needs_recompute; malformed evidence yields unsupported.")
        and :162 ("Transport without raw rows ... is never eligible.")
  D2  = DESIGN_STANDING/DESIGN.md §4.9.4 (the standing table; ordinary eligibility of a
        not_required case = checks_passed, passive_model_basis, represented_equations_retained,
        evidence refs resolve; the base rules).
  D-U6-1 (RR "U6 plan accepted"): the reader's own flag gates `numerical_eligible` only; every
        G check runs regardless. The carrier adds the requested refs and the not_required rule.

Rule set:
  R0  G-status: whether the statement passes G0-G8 is taken from the shared expectation, never
      computed: corpus bases and must-pass entries pass (their `expected`); corpus mutations
      fail (their gate/code). A U6 carrier case passes G iff it is an unedited milestone
      successor; any unrehashed edit of the hashed statement fails G1, of the bound invocation
      fails G8, and a relabelled or legacy statement is not a successor (C1 G0/G1/G8; the
      case file's `expected_dispatch`).
  R1  reader `numerical_eligible` = U7 flag AND G passes AND an invocation is supplied AND
      status.mechanics == "MECHANICS_SOLVED" AND every receipt case status is "selected" or
      "not_required" (C1:160's reader part; D-U6-1). The reader's own `standing` label is
      "eligible" iff numerical_eligible, else "needs_recompute" (the readers' Validation
      contract, PY/TS; Rust has no label).
  R2  carrier token: "unsupported" if G fails; else "numerically_eligible" iff R1 holds AND the
      requested load-case refs equal the receipt's case basis_refs in order AND every
      not_required case is ordinarily eligible (D2 §4.9.4: its numerical_quality entry has the
      same basis_ref, solve_quality checks_passed, structural_status passive_model_basis,
      model_matrix_fidelity represented_equations_retained, accuracy_evidence not_claimed or
      reference_verified, and non-empty evidence_refs that resolve to unique result or
      diagnostic ids); else "needs_recompute".
  R3  requested refs for a corpus entry or the live milestone: the invocation's model
      load_cases in order, as {ref_type: "load_case", ref_id}; for a carrier case: its
      `requested` ("invocation" = as above; a list = itself).
  R4  edits: the shared entry format's set/remove on JSON paths (an integral number indexes an
      array), applied to the source, the invocation (invocation_edits) and after the rehash
      (after_rehash). Hashes are not recomputed: no rule above reads a hash.

Usage: eligibility_oracle.py P_ROOT OUT_JSON [--flag false]  (the default is the post-U7 flag, true)
"""
import json, sys
from copy import deepcopy
from pathlib import Path

P = Path(sys.argv[1]); OUT = Path(sys.argv[2])
FLAG = not (len(sys.argv) > 4 and sys.argv[3] == "--flag" and sys.argv[4] == "false")


def index(v):
    return int(v) if type(v) in (int, float) and v == int(v) and v >= 0 else None


def edit(doc, path, op, value=None):
    parent = doc
    for p in path[:-1]:
        i = index(p); parent = parent[i] if i is not None and isinstance(parent, list) else parent[p]
    last = path[-1]; i = index(last)
    key = i if i is not None and isinstance(parent, list) else last
    if op == "remove":
        del parent[key]
    else:
        assert op == "set", op
        parent[key] = deepcopy(value)


def requested_from(invocation):
    if invocation is None:
        return []
    return [{"ref_type": "load_case", "ref_id": c.get("id")} for c in invocation["request"]["model"]["load_cases"]]


def ordinarily_eligible(source, index_, case):
    """D2 §4.9.4: a not_required case's ordinary eligibility by the base rules."""
    quality = source.get("numerical_quality", {}).get("cases", [])
    if index_ >= len(quality):
        return False
    q = quality[index_]
    ids, seen = [], set()
    for key in ("results", "diagnostics"):
        for item in source.get(key, []):
            i = item.get("id")
            if not isinstance(i, str) or not i or i in seen:
                return False
            seen.add(i); ids.append(i)
    refs = q.get("evidence_refs")
    return (q.get("basis_ref") == case.get("basis_ref") and q.get("solve_quality") == "checks_passed"
            and q.get("structural_status") == "passive_model_basis" and q.get("model_matrix_fidelity") == "represented_equations_retained"
            and q.get("accuracy_evidence") in ("not_claimed", "reference_verified")
            and isinstance(refs, list) and bool(refs) and all(isinstance(r, str) and r in seen for r in refs))


def judge(source, invocation, g_passes, requested):
    cases = source["retained_precision"]["body"]["cases"] if g_passes else []
    eligible = (FLAG and g_passes and invocation is not None and source["status"]["mechanics"] == "MECHANICS_SOLVED"
                and all(c["status"] in ("selected", "not_required") for c in cases))
    if not g_passes:
        token = "unsupported"
    elif (eligible and requested == [c["basis_ref"] for c in cases]
          and all(c["status"] == "selected" or ordinarily_eligible(source, i, c) for i, c in enumerate(cases))):
        token = "numerically_eligible"
    else:
        token = "needs_recompute"
    reasons = []
    if g_passes:
        if invocation is None: reasons.append("no invocation")
        if source["status"]["mechanics"] != "MECHANICS_SOLVED": reasons.append("mechanics " + source["status"]["mechanics"])
        bad = [c["status"] for c in cases if c["status"] not in ("selected", "not_required")]
        if bad: reasons.append("case status " + ",".join(bad))
        if requested != [c["basis_ref"] for c in cases]: reasons.append("requested refs differ")
    return {"invocation_bound": g_passes and invocation is not None, "numerical_eligible": eligible,
            "reader_standing": "eligible" if eligible else "needs_recompute", "carrier_token": token,
            "why_not": reasons if not eligible or token != "numerically_eligible" else []}


report = {"flag": FLAG, "rules": "R0-R4 (docstring)", "corpus": {"cases": {}, "must_pass": {}, "mutations": {}}, "carrier_cases": {}, "milestone": {}}
corpus = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
bases = {c["id"]: c for c in corpus["cases"]}
for c in corpus["cases"]:
    inv = c.get("invocation")
    report["corpus"]["cases"][c["id"]] = {"with_invocation": judge(c["source"], inv, True, requested_from(inv)),
                                          "without_invocation": judge(c["source"], None, True, requested_from(inv))}
for kind in ("must_pass", "mutations"):
    for e in corpus[kind]:
        base = bases[e["base"]]
        source, inv = deepcopy(base["source"]), deepcopy(base.get("invocation"))
        for x in e.get("edits", []): edit(source, x["path"], x["op"], x.get("value"))
        for x in e.get("invocation_edits", []) or []: edit(inv, x["path"], x["op"], x.get("value"))
        for x in e.get("after_rehash", []) or []: edit(source, x["path"], x["op"], x.get("value"))
        g = kind == "must_pass"
        assert (e["expected"] == "pass") == g, e["id"]
        report["corpus"][kind][e["id"]] = judge(source, inv, g, requested_from(inv))

carriers = json.loads((P / "fixtures/results/retained_precision_carrier_cases.json").read_text())
docs = {k: json.loads((P / v["path"]).read_text()) for k, v in carriers["fixtures"].items()}
for c in carriers["cases"]:
    fx = carriers["fixtures"][c["fixture"]]; doc = deepcopy(docs[c["fixture"]])
    milestone = fx["shape"] == "milestone"
    source = doc["source"] if milestone else doc
    inv = deepcopy(doc["invocation"]) if (milestone and c["invocation"] == "fixture") else None
    g = milestone and not c["edits"]          # R0: an edited statement or invocation fails G1/G8
    requested = requested_from(doc["invocation"] if milestone else None) if c["requested"] == "invocation" else c["requested"]
    r = judge(source, inv, g, requested)
    r["case_file_expected_standing"] = c["expected_standing"]
    r["changes_at_u7"] = r["carrier_token"] != c["expected_standing"]
    report["carrier_cases"][c["id"]] = r

for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    report["milestone"][mode] = {"with_invocation": judge(doc["source"], doc["invocation"], True, requested_from(doc["invocation"])),
                                 "without_invocation": judge(doc["source"], None, True, requested_from(doc["invocation"])),
                                 "other_requested": judge(doc["source"], doc["invocation"], True, [{"ref_type": "load_case", "ref_id": "other"}])}

OUT.write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
el = lambda d: sorted(k for k, v in d.items() if v["numerical_eligible"])
print("flag", FLAG)
print("bases eligible with invocation:", sorted(k for k, v in report["corpus"]["cases"].items() if v["with_invocation"]["numerical_eligible"]))
print("bases not eligible with invocation:", {k: v["with_invocation"]["why_not"] for k, v in report["corpus"]["cases"].items() if not v["with_invocation"]["numerical_eligible"]})
print("must_pass eligible:", len(el(report["corpus"]["must_pass"])), "of", len(report["corpus"]["must_pass"]))
print("must_pass not eligible:", {k: v["why_not"] for k, v in report["corpus"]["must_pass"].items() if not v["numerical_eligible"]})
print("carrier changes at U7:", {k: (v["case_file_expected_standing"], v["carrier_token"]) for k, v in report["carrier_cases"].items() if v["changes_at_u7"]})
print("milestone:", {m: {k: v["carrier_token"] for k, v in x.items()} for m, x in report["milestone"].items()})
