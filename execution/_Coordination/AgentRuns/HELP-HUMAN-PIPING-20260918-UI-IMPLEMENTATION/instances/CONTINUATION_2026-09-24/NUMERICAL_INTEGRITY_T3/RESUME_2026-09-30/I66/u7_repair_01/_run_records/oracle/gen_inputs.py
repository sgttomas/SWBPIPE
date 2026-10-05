#!/usr/bin/env python3
"""I66 U7 slice F, the oracle-diff control: build every input once, as JSON, so that the base
and candidate lanes of each language read identical bytes. Run from a P root (the candidate);
it uses the shared corpus helper `apply_entry` only to apply the corpus's own edit grammar
(rehash included). Keys follow slice A's oracle (`oracle_post_u7.json`):
  cases|<id>|with_invocation, cases|<id>|without_invocation, must_pass|<id>, mutations|<id>,
  carrier|<case id>, milestone|<mode>|{with_invocation,without_invocation,other_requested},
  declared|<entry id>|<form label>|<fixture id> (every form of every declared difference).
Usage: gen_inputs.py OUT_JSON"""
import json, sys
from copy import deepcopy
from pathlib import Path

sys.path.insert(0, "."); sys.path.insert(0, "tests")
from tests.test_retained_precision_contract import apply_entry, corpus  # noqa: E402

ROOT = Path(".")


def requested_from(invocation):
    return [{"ref_type": "load_case", "ref_id": c["id"]} for c in invocation["request"]["model"]["load_cases"]] if invocation else []


out = []
data = corpus()
for fixture in data["cases"]:
    inv = fixture["invocation"]
    out.append({"key": f"cases|{fixture['id']}|with_invocation", "source": fixture["source"], "invocation": inv, "requested": requested_from(inv), "binding": True})
    out.append({"key": f"cases|{fixture['id']}|without_invocation", "source": fixture["source"], "invocation": None, "requested": requested_from(inv)})
for kind in ("must_pass", "mutations"):
    for entry in data[kind]:
        fixture = next(f for f in data["cases"] if f["id"] == entry["base"])
        source, invocation = apply_entry(fixture, entry)
        out.append({"key": f"{kind}|{entry['id']}", "source": source, "invocation": invocation, "requested": requested_from(fixture["invocation"])})

cases = json.loads((ROOT / "fixtures/results/retained_precision_carrier_cases.json").read_text())
docs = {}
for fid, spec in cases["fixtures"].items():
    doc = json.loads((ROOT / spec["path"]).read_text())
    docs[fid] = doc if spec["shape"] == "milestone" else {"source": doc, "invocation": None}


def apply_case(case, doc):
    source, invocation = deepcopy(doc["source"]), deepcopy(doc["invocation"])
    for edit in case["edits"]:
        assert edit["op"] == "set"
        target = source if edit["target"] == "source" else invocation
        for key in edit["path"][:-1]:
            target = target[key]
        target[edit["path"][-1]] = edit["value"]
    refs = requested_from(doc["invocation"]) if case["requested"] == "invocation" else case["requested"]
    if isinstance(case["invocation"], dict):
        return source, deepcopy(case["invocation"]), refs
    return source, (invocation if case["invocation"] is not None else None), refs


for case in cases["cases"]:
    source, invocation, refs = apply_case(case, docs[case["fixture"]])
    out.append({"key": f"carrier|{case['id']}", "source": source, "invocation": invocation, "requested": refs})
for entry in cases["declared_differences"]:
    for form in entry["forms"]:
        for fid in form["fixtures"]:
            source, invocation, refs = apply_case(form, docs[fid])
            out.append({"key": f"declared|{entry['id']}|{form['label']}|{fid}", "source": source, "invocation": invocation, "requested": refs,
                        "binding": form["subject"] == "binding"})
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    inv = doc["invocation"]
    out.append({"key": f"milestone|{mode}|with_invocation", "source": doc["source"], "invocation": inv, "requested": requested_from(inv), "binding": True, "milestone": mode})
    out.append({"key": f"milestone|{mode}|without_invocation", "source": doc["source"], "invocation": None, "requested": requested_from(inv)})
    out.append({"key": f"milestone|{mode}|other_requested", "source": doc["source"], "invocation": inv, "requested": [{"ref_type": "load_case", "ref_id": "other"}]})
assert len({r["key"] for r in out}) == len(out)
Path(sys.argv[1]).write_text(json.dumps(out))
print(len(out), "inputs")
