#!/usr/bin/env python3
"""Pin the VP-ROBUST "no Passed breach" gate's exception lists.

ROOT, 2026-09-26: frozen-reference cases only, per entry; the skew case is not an exception.
ROOT, 2026-09-27 (ruling on I4's F12 stop): the 7 formation/formed-term triples below are moved
out of the S11 list into a separate, exactly pinned FORMATION list (entry, case, quantity, mode).
They were misclassified as S11 class by P1's heuristic and the 2026-09-26 re-pin; they are
pre-existing on main. The S11 list must be empty after S11-F; the formation list must be empty
when S11-G lands (or per-row justification to ROOT); any triple outside both lists fails the gate.

Usage (from T3/): python3 GATE/pin_s11_exceptions.py
Writes GATE/S11_EXCEPTIONS.json and GATE/FORMATION_EXCEPTIONS.json. Source: P1's final record
DETECTION/results.json (section `exceptions`) and REFERENCES/references.json (c0f14201c).
Standard library only; deterministic.
"""
import hashlib
import json

RESULTS = "DETECTION/results.json"
REFS = "REFERENCES/references.json"

# (entry, case, quantity) -> owner and cause (ROOT 2026-09-27)
FORMATION = {
    ("typed", "RF-CANCEL-UDL-W1e80", "th.S1.RZ"): "load formation: SP fixed-end coefficients formed by different binary64 expressions at i and j (S11 s2.2, s10 item 2); S11-G load-row guard, then F3 (W1b)",
    ("captured", "RF-CANCEL-UDL-W1e8", "th.S1.RZ"): "load formation (as above); S11-G load-row guard, then F3 (W1b)",
    ("typed", "RF-CANCEL-UDL-W1e8", "th.S1.RZ"): "load formation (as above); S11-G load-row guard, then F3 (W1b)",
    ("typed", "RF-CANCEL-F-G1e80-GnG-INPLANE", "Mb.M1.j"): "formed recovery term K_e*u roundoff (S11 s10 item 3); S11-G recovery guard, then F2 (W1a)",
    ("typed", "RF-CANCEL-F-G1e80-GnG-INPLANE", "Mb.M2.i"): "formed recovery term K_e*u roundoff; S11-G recovery guard, then F2 (W1a)",
    ("typed", "RF-CANCEL-M-G1e80-GnG-INPLANE", "Mb.M2.i"): "formed recovery term K_e*u roundoff; S11-G recovery guard, then F2 (W1a)",
    ("typed", "RF-CANCEL-M-G1e80-GnG-INPLANE", "Mb.M2.j"): "formed recovery term K_e*u roundoff; S11-G recovery guard, then F2 (W1a)",
}


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


frozen = set(json.load(open(REFS))["cases"])
ex = json.load(open(RESULTS))["exceptions"]
SECTIONS = {"captured": ["captured"], "typed": ["typed_on_captured", "typed_secondary"]}

rows = []  # (entry, case, quantity, mode)
for entry, secs in SECTIONS.items():
    for sec in secs:
        for r in ex[sec]["s11_class"]:
            if r["case"] in frozen:
                rows.append((entry, r["case"], r["quantity"], r["mode"]))
rows = sorted(set(rows))

s11 = sorted({(e, c, q) for e, c, q, m in rows if (e, c, q) not in FORMATION})
form_rows = [[e, c, q, m] for e, c, q, m in rows if (e, c, q) in FORMATION]
assert {(e, c, q) for e, c, q, m in form_rows} == set(FORMATION), "formation triples must all be present"

src = {"results_json_sha256": sha(RESULTS), "references_json_sha256": sha(REFS)}


def counts(triples):
    out = {}
    for e in ("captured", "typed"):
        t = [x for x in triples if x[0] == e]
        out[e] = {"triples": len(t), "cases": len({x[1] for x in t})}
    return out


with open("GATE/S11_EXCEPTIONS.json", "w") as f:
    json.dump({
        "ruling": "ROOT 2026-09-26 (frozen-reference only, per entry; skew case not an exception); ROOT 2026-09-27 (7 formation-class triples moved to FORMATION_EXCEPTIONS.json). Must be empty after S11-F.",
        "source": src,
        "counts": counts(s11),
        "not_exceptions": ["RF-SKEW-T-CANT-OFF-122-r1e-04 (K-D5 required true positive, both entries)"],
        "triples": [list(t) for t in s11],
    }, f, indent=1)
    f.write("\n")

with open("GATE/FORMATION_EXCEPTIONS.json", "w") as f:
    json.dump({
        "ruling": "ROOT 2026-09-27 (on I4's F12 stop, option (c)). Pre-existing on main (P1 baseline); S11-F makes none worse. Must be empty when S11-G lands, or per-row justification to ROOT for waiting on F2/F3.",
        "source": src,
        "counts": {"triples": len(FORMATION), "rows_with_mode": len(form_rows)},
        "owners": {" | ".join(k): v for k, v in sorted(FORMATION.items())},
        "rows": form_rows,
    }, f, indent=1)
    f.write("\n")

print(json.dumps({"s11": counts(s11), "formation_triples": len(FORMATION), "formation_rows": len(form_rows)}))
