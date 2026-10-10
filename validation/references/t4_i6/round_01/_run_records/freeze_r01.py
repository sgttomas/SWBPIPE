"""T4-I6 R01: combine the per-item parts into ../u1_reference_cases_r01.json (new values only).
usage: python -I -B freeze_r01.py  (after i1..i8 have written parts/*.json)"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
doc = {
    "record": "T4-I6 repair round 01 supplement to R4/T4-I6/u1_reference_cases.json (round 00, SHA256SUMS digest 146f734a...); new values only",
    "status": "frozen for refutation; references are EXACT (B1, decimal 110 digits unless stated); every 'emulation' or 'labelled' field is a binary64 model of a plausible implementation, never product code",
    "definition": "round-00 U1_REFERENCE.md B1 (unchanged)",
    "units": "SI (m, N, Pa, rad); DOF order per node [ux, uy, uz, rx, ry, rz]; node i then node j",
    "code_basis_read": "origin/main 10b70036ef (git show HEAD in WT/t4-code)",
}
for name in ("i1_curved122", "i2_cskew", "i3_k2", "i4_k1", "i5_o4", "i6_p4", "i7_m31b0", "i8_acceptance"):
    with open(os.path.join(HERE, "parts", name + ".json")) as fh:
        part = json.load(fh)
    for k, v in part.items():
        assert k not in doc, k
        doc[k] = v
with open(os.path.join(HERE, "..", "u1_reference_cases_r01.json"), "w") as fh:
    json.dump(doc, fh, indent=1)
    fh.write("\n")
print("keys:", ", ".join(k for k in doc))
