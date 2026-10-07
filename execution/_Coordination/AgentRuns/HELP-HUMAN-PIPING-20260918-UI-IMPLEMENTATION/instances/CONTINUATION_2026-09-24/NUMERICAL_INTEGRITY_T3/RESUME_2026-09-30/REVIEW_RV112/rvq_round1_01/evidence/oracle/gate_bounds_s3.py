#!/usr/bin/env python3
"""RV112 (RV-Q round 1): every gate bound SA writes, evaluated from I82's profile trees with I82's
evaluator (b1_eval.py, unchanged, sha256 c404e8db...), at c = 1 (the registered tree, byte-identical
to I82's d1_c1) and at option S3 (I82's d1_c3, = ADDENDUM_01's). Independent of the Rust code:
the expressions are written here from DESIGN_v2 §6, I82 STUDY §4.3 and PLAN_v2 §2.3.

Also evaluates the maximum value G-B's LateObservationBytes can legitimately take at case k
(0-based) of a C-case invocation under the same forms, to test SA's expression T11 - T11_late_capture.
Usage: gate_bounds_s3.py <oracle dir>   (prints JSON)
"""
import json, sys
sys.path.insert(0, sys.argv[1])
import b1_eval as E

D = sys.argv[1]
atoms = E.load_atoms(f"{D}/law_record.txt")
n = m = g = 32
RESTRAINTS, SPRINGS, MATERIALS, l, L = 192, 192, 4, 128, 384


def push_capacity(h):
    if h == 0:
        return 0
    c = 4
    while c < h:
        c *= 2
    return c


def forms_of(tree):
    v = dict(atoms)
    for k, x in tree["text_atoms"].items():
        if k.startswith("Text("):
            v[k] = x
    return {name: E.form(tree["forms"][name], v) for name in ("T11", "T11_late_capture", "T11_ordinary_seed", "NOTICE")
            if name in tree["forms"]}


out = {}
for c in (1, 2, 3, 4):
    tree = json.load(open(f"{D}/d1_c{c}.json"))
    f = forms_of(tree)
    ta = tree["text_atoms"]
    P = 7 * n + 51 * m + 8 * g + 3
    k_r = min(6 * n, RESTRAINTS)
    late = {"BuiltNodes": n, "BuiltMembers": m, "BuiltFrameElements": m, "BuiltSupports": g, "CaseLoads": l,
            "CaseLoadsTotal": c * l, "Restrained": k_r, "Springs": SPRINGS, "Materials": 2 * MATERIALS,
            "LateObservationBytes (SA: T11 - T11_late)": f["T11"] - f["T11_late_capture"]}
    complete = {"EnvelopeResults": c * P, "EnvelopeResultCapacity": push_capacity(c * P),
                "EnvelopeResultTextBytes": 2 * c * P * ta["Text(row)"], "EnvelopeDiagnostics": ta["D_env"],
                "EnvelopeDiagnosticCapacity": push_capacity(ta["D_env"]), "EnvelopeDiagnosticTextBytes": 2 * ta["Text(diag_env)"],
                "EnvelopeMaxStringBytes": "L_PUB (TEXT max_site; not in the tree)", "DiagnosticIdMaxBytes": "L_DIAGID (not in the tree)",
                "ContractEvidenceStatus": 0, "ContractEvidenceArrayElements": c * (3 * m + 2 * g),
                "ContractEvidenceObjects": c * (3 + m + g), "ContractEvidenceEntries": c * (9 + 15 * m + 2 * g),
                "ContractEvidenceStringBytes": c * (m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64)),
                "ContractEvidenceKeyBytes": c * ((9 + 15 * m + 2 * g) * 40), "SourceBlockRecovery": 0,
                "ObservationBytes": f["T11"], "OrdinarySeedBytes": f["T11_ordinary_seed"],
                "RetainedErrorTextBytes": c * (3 * m + 1) * ta["Text(err)"], "OrdinarySolveNotAttempted": 0}
    # G-B at case k sees: the invocation part, k+1 cases' pre-late capture, and k cases' late capture.
    # T11(c) is affine in c over I82's c = 1..4 trees (checked below), so T11 = inv + c*(early + late).
    per_case_late = f["T11_late_capture"] // c
    assert per_case_late * c == f["T11_late_capture"], "T11_late_capture is exactly c x one case's"
    t11 = {cc: forms_of(json.load(open(f"{D}/d1_c{cc}.json")))["T11"] for cc in (1, 2, 3, 4)}
    step = t11[2] - t11[1]
    assert all(t11[cc + 1] - t11[cc] == step for cc in (1, 2, 3)), "T11 affine in c"
    inv, early = t11[1] - step, step - per_case_late
    assert inv + c * (early + per_case_late) == f["T11"]
    g_b_max = {k: inv + (k + 1) * early + k * per_case_late for k in range(c)}
    out[f"c={c}"] = {"forms": f, "text_atoms": {k: ta[k] for k in ("D_env", "Text(diag_env)", "Text(row)", "Text(err)")},
                     "late": late, "complete": complete,
                     "g_b_LateObservationBytes": {"SA_bound": f["T11"] - f["T11_late_capture"],
                                                  "priced_max_at_case_k": g_b_max, "T11_affine": {"invocation": inv, "per_case_pre_late": early, "per_case_late": per_case_late},
                                                  "correct_bound_T11_minus_one_case_late": f["T11"] - per_case_late,
                                                  "deficit_at_last_case": f["T11"] - per_case_late - (f["T11"] - f["T11_late_capture"])}}
print(json.dumps(out, indent=1))
