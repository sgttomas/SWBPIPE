#!/usr/bin/env python3
"""Pin the VP-ROBUST "no Passed breach" gate's S11 exceptions (ROOT, 2026-09-26).

Usage (from T3/): python3 GATE/pin_s11_exceptions.py > GATE/S11_EXCEPTIONS.json

Source: P1's final record DETECTION/results.json, section `exceptions`. Frozen-reference cases only
(case ids present in REFERENCES/references.json at c0f14201c); P1's own probes (S11-PROBE-*) and
V1-CHECK-L-* are excluded. Triples are (entry, case, quantity), with modes merged.
- captured: exceptions.captured.s11_class
- typed:    exceptions.typed_on_captured.s11_class (bit-identical to captured) plus
            exceptions.typed_secondary.s11_class (the G = 1e80 cases the captured entry refuses)
The skew case RF-SKEW-T-CANT-OFF-122-r1e-04 (other_class) is NOT an exception: it is K-D5's
required true positive on both entries. Standard library only; deterministic.
"""
import hashlib
import json

RESULTS = "DETECTION/results.json"
REFS = "REFERENCES/references.json"


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


frozen = set(json.load(open(REFS))["cases"])
ex = json.load(open(RESULTS))["exceptions"]


def pairs(section):
    return {(r["case"], r["quantity"]) for r in ex[section]["s11_class"] if r["case"] in frozen}


cap = pairs("captured")
typ = pairs("typed_on_captured") | pairs("typed_secondary")
triples = sorted([["captured", c, q] for c, q in cap] + [["typed", c, q] for c, q in typ])
cases = {e: sorted({c for ee, c, _ in triples if ee == e}) for e in ("captured", "typed")}
print(json.dumps({
    "ruling": "ROOT 2026-09-26: frozen-reference cases only, per entry; skew case is not an exception",
    "source": {"results_json_sha256": sha(RESULTS), "references_json_sha256": sha(REFS)},
    "counts": {e: {"triples": sum(1 for t in triples if t[0] == e), "cases": len(v)} for e, v in cases.items()},
    "cases": cases,
    "not_exceptions": ["RF-SKEW-T-CANT-OFF-122-r1e-04 (K-D5 required true positive, both entries)"],
    "triples": triples,
}, indent=1))
