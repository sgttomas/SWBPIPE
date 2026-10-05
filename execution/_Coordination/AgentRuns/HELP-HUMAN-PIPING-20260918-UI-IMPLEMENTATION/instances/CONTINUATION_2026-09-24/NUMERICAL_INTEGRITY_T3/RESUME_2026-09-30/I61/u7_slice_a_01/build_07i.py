#!/usr/bin/env python3
"""I61 U7 slice A: stage snapshot 07i's eligibility expectations from the oracle (records only).

Usage: build_07i.py P_ROOT ORACLE_JSON OUT_CASES OUT_CARRIER_CASES
- Each corpus base's `expected` (already {invocation_bound, numerical_eligible, standing}) takes the
  oracle's post-U7 value with the base's own invocation.
- Each must-pass entry gains `expected_eligibility` (same three keys), placed after `expected`.
- The carrier case file's `expected_standing` takes the oracle's carrier token; its note drops the
  held-eligibility sentence, and its scope gains D-U7-6's no-producer-origin sentence.
Everything else is copied unchanged (asserted), in the files' own format (json indent=2 plus newline).
"""
import json, sys
from copy import deepcopy
from pathlib import Path
P, ORACLE, OUT, OUT_CARRIER = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4])
oracle = json.loads(ORACLE.read_text()); assert oracle["flag"] is True
src = P / "fixtures/results/retained_precision_cases.json"
raw = src.read_bytes(); data = json.loads(raw); before = deepcopy(data)
assert json.dumps(data, indent=2).encode() + b"\n" == raw
assert (len(data["cases"]), len(data["mutations"]), len(data["must_pass"])) == (15, 277, 23), "07h"
triple = lambda r: {"invocation_bound": r["invocation_bound"], "numerical_eligible": r["numerical_eligible"], "standing": r["reader_standing"]}
for c in data["cases"]:
    assert set(c["expected"]) == {"invocation_bound", "numerical_eligible", "standing"}
    c["expected"] = triple(oracle["corpus"]["cases"][c["id"]]["with_invocation"])
new_must = []
for e in data["must_pass"]:
    assert "expected_eligibility" not in e
    out = {}
    for k, v in e.items():
        out[k] = v
        if k == "expected":
            out["expected_eligibility"] = triple(oracle["corpus"]["must_pass"][e["id"]])
    new_must.append(out)
data["must_pass"] = new_must
# Unchanged: everything else.
for key in data:
    if key not in ("cases", "must_pass"):
        assert data[key] == before[key], key
for a, b in zip(data["cases"], before["cases"]):
    assert {k: v for k, v in a.items() if k != "expected"} == {k: v for k, v in b.items() if k != "expected"}
for a, b in zip(data["must_pass"], before["must_pass"]):
    assert {k: v for k, v in a.items() if k != "expected_eligibility"} == b
OUT.write_text(json.dumps(data, indent=2) + "\n")

csrc = P / "fixtures/results/retained_precision_carrier_cases.json"
craw = csrc.read_bytes(); cdata = json.loads(craw); cbefore = deepcopy(cdata)
indent = 2 if json.dumps(cdata, indent=2).encode() + b"\n" == craw else None
assert indent == 2, "carrier case file format"
changed = []
for c in cdata["cases"]:
    token = oracle["carrier_cases"][c["id"]]["carrier_token"]
    if token != c["expected_standing"]:
        changed.append((c["id"], c["expected_standing"], token)); c["expected_standing"] = token
# The note's held-eligibility sentence, and D-U7-6's scope sentence (appended; every existing
# scope substring the three languages assert is kept).
HELD = "Eligibility is held until U7, so no case expects numerically_eligible."
NOW = ("Since U7 the readers' eligibility is switched on: an unedited milestone case with its fixture invocation and "
       "the invocation's requested cases expects numerically_eligible, and every other case keeps its expectation.")
assert cdata["note"].count(HELD) == 1
cdata["note"] = cdata["note"].replace(HELD, NOW)
ORIGIN = (" A numerically_eligible standing is a property of the supplied statement and its actual invocation, as the "
          "accepted reader checks them: no carrier authenticates producer origin, and none claims that a registered "
          "producer made the statement (ROOT_RULINGS_V1 \"RV93 on U3 grant 2: PASS; RV89 confirms the final basis "
          "re-qualified; U7 planned and ruled\", D-U7-6).")
assert ORIGIN.strip() not in cdata["scope"]
cdata["scope"] = cdata["scope"] + ORIGIN
for a, b in zip(cdata["cases"], cbefore["cases"]):
    assert {k: v for k, v in a.items() if k != "expected_standing"} == {k: v for k, v in b.items() if k != "expected_standing"}
assert {k: v for k, v in cdata.items() if k not in ("cases", "note", "scope")} == {k: v for k, v in cbefore.items() if k not in ("cases", "note", "scope")}
assert cdata["scope"].startswith(cbefore["scope"])
OUT_CARRIER.write_text(json.dumps(cdata, indent=2) + "\n")
print("bases eligible:", sum(c["expected"]["numerical_eligible"] for c in data["cases"]), "of", len(data["cases"]))
print("must-pass eligible:", sum(e["expected_eligibility"]["numerical_eligible"] for e in data["must_pass"]), "of", len(data["must_pass"]))
print("carrier cases changed:", changed)
