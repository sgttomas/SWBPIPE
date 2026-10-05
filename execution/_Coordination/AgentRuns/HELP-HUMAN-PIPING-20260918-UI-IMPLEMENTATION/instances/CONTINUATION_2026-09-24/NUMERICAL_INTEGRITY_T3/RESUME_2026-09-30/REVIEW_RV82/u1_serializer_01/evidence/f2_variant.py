#!/usr/bin/env python3
"""RV82 F2 check (reader side only): the milestone receipt with the selected case's legacy
route rewritten to F2's shape {declined_without_attempt, null, work_ref -> 0/0/0 WorkReport},
receipt hash recomputed. Not a producer output; it shows the readers accept F2's wire form."""
import json, os, sys
P = os.environ["P"]; sys.path.insert(0, P)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1 as H
src, dst = os.environ["SRC"], os.environ["DST"]
for m in ["sparse_interactive", "dense_scrutiny"]:
    c = json.load(open(f"{src}/u1_milestone_{m}.json"))
    b = c["source"]["retained_precision"]["body"]
    b["ordinary_attempts"][0]["legacy_source"] = {"disposition": "declined_without_attempt", "diagnostic_ref": None, "work_ref": 0}
    b["legacy_source_work"] = [{"case_index": 0, "stage": "formation guard", "helper_stage": "source_closure", "charged": 0, "rejected": 0, "limit": 0, "settlement": "booked"}]
    c["source"]["retained_precision"]["receipt_sha256"] = H({"domain": "retained_precision_receipt_mp_v2", "payload": b})
    c["id"] = "f2_variant_" + m
    json.dump(c, open(f"{dst}/u1_milestone_f2variant_{m}.json", "w"), indent=2)
