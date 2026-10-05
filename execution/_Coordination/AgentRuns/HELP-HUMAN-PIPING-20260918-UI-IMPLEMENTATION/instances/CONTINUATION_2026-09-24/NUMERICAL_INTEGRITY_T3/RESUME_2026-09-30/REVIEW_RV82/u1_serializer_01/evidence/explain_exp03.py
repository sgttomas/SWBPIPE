#!/usr/bin/env python3
"""RV82 item 3: apply only the three listed member changes to experiment 03's receipt,
recompute the two dependent hashes, and require byte equality with U1's file."""
import json, os, sys, hashlib
P = os.environ["P"]; sys.path.insert(0, P)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1 as H
OUT, EXP = os.environ["OUT"], os.environ["EXP"]
MSG = ("Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load case. Its published rows carry "
       "recovery_method; the retained_precision receipt binds their certified classes, the native attempts and work, and the ordinary-attempt evidence.")
ok = True
for m in ["sparse_interactive", "dense_scrutiny"]:
    e = json.load(open(f"{EXP}/milestone_{m}.json"))
    s = e["source"]; d = s["diagnostics"][-1]
    assert d["code"] == "RETAINED_PRECISION_SELECTED"
    d["id"] = "diagnostic:retained-precision:case:selected"; d["message"] = MSG                   # G-a
    s["retained_precision"]["body"]["ordinary_attempts"][0]["formation"]["d5_diagnostic_ref"] = "diagnostic:numerical-integrity:case"  # F1
    env = {k: v for k, v in s.items() if k != "retained_precision"}
    body = s["retained_precision"]["body"]
    body["publication_sha256"] = H({"domain": "retained_precision_publication_mp_v2", "payload": env})
    s["retained_precision"]["receipt_sha256"] = H({"domain": "retained_precision_receipt_mp_v2", "payload": body})
    e["id"] = "u1_" + e["id"]
    text = json.dumps(e, indent=2, ensure_ascii=False)
    u = open(f"{OUT}/u1_milestone_{m}.json").read()
    same_value = json.loads(u) == e
    same_bytes = text == u
    print(m, "value-equal:", same_value, "byte-equal(py pretty):", same_bytes, "u1 sha256:", hashlib.sha256(u.encode()).hexdigest())
    ok &= same_value
sys.exit(0 if ok else 1)
