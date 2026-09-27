#!/usr/bin/env python3
"""Pin the VP-ROBUST "no Passed breach" gate's exception lists.

ROOT, 2026-09-26: frozen-reference cases only, per entry; the skew case is not an exception.
ROOT, 2026-09-27 (ruling on I4's F12 stop): the 7 formation/formed-term triples below are moved
out of the S11 list into a separate, exactly pinned FORMATION list (entry, case, quantity, mode).
They were misclassified as S11 class by P1's heuristic and the 2026-09-26 re-pin; they are
pre-existing on main. The S11 list must be empty after S11-F; the formation list must be empty
when S11-G lands (or per-row justification to ROOT); any triple outside both lists fails the gate.
ROOT, 2026-09-27: the S11 list is emptied at the S11-F merge (PR1000, 43b8f83aa), on RV3's F12
verification (REVIEW/S11F_REVIEW.md, bf82f6cfd): no listed triple remains breached on either entry
or in either mode. The pre-S11-F pin (221 triples) stays derivable from the same sources and is
recorded in the file's `emptied` field; any Passed breach outside FORMATION now fails the gate.

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
    ("captured", "RF-CANCEL-UDL-W1e8", "th.S1.RZ"): "load formation (as above); S11-G load-row guard, then F3 (W1b). ROOT 2026-09-27 amendment: after S11-F the published value is the correctly rounded net of the represented terms, 3% worse than base (46.47x -> 47.99x the criterion, both modes); S11-G is required to catch this row",
    ("typed", "RF-CANCEL-UDL-W1e8", "th.S1.RZ"): "load formation (as above); S11-G load-row guard, then F3 (W1b). ROOT 2026-09-27 amendment: after S11-F the published value is the correctly rounded net of the represented terms, 3% worse than base (46.47x -> 47.99x the criterion, both modes); S11-G is required to catch this row",
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
        "ruling": "ROOT 2026-09-26 (frozen-reference only, per entry; skew case not an exception); ROOT 2026-09-27 (7 formation-class triples moved to FORMATION_EXCEPTIONS.json). Must be empty after S11-F. ROOT 2026-09-27: emptied at S11-F merge 43b8f83aa on RV3's F12 verification, bf82f6cfd.",
        "source": src,
        "emptied": {
            "at_merge": "43b8f83aa338979055390e0bab923401eea57a85 (PR1000, S11-F)",
            "verification": "REVIEW/S11F_REVIEW.md (RV3, PASS), commit bf82f6cfd: F12 gate harness passes; none of the pinned triples remains breached on either entry or mode",
            "pinned_before_s11f": {"counts": counts(s11), "triples_sha256": hashlib.sha256(json.dumps([list(t) for t in s11]).encode()).hexdigest()},
        },
        "counts": counts([]),
        "not_exceptions": ["RF-SKEW-T-CANT-OFF-122-r1e-04 (K-D5 required true positive, both entries)"],
        "triples": [],
    }, f, indent=1)
    f.write("\n")

with open("GATE/FORMATION_EXCEPTIONS.json", "w") as f:
    json.dump({
        "ruling": "ROOT 2026-09-27 (on I4's F12 stop, option (c)). Pre-existing on main (P1 baseline). Condition amended 2026-09-27: after S11-F each row publishes the correctly rounded net of the represented terms and stays exactly pinned here (10 rows bit-identical or better; the 4 UDL-W1e8 rows 3% worse). Must be empty when S11-G lands, or per-row justification to ROOT for waiting on F2/F3.",
        "source": src,
        "counts": {"triples": len(FORMATION), "rows_with_mode": len(form_rows)},
        "owners": {" | ".join(k): v for k, v in sorted(FORMATION.items())},
        "rows": form_rows,
    }, f, indent=1)
    f.write("\n")

print(json.dumps({"s11": counts([]), "s11_pinned_before_s11f": counts(s11), "formation_triples": len(FORMATION), "formation_rows": len(form_rows)}))
