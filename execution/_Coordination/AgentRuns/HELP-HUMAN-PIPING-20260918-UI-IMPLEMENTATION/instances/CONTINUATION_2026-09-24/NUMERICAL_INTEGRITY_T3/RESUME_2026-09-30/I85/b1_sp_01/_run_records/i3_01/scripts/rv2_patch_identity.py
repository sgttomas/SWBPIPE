#!/usr/bin/env python3
"""I85 B1-SP I3: RV109's six round-2 mutants in my driver (mutants_sp.py, ids RV2_*) carry RV109's
patch strings (evidence/tools/sp2_mutants.py) byte for byte: same file, same old text, same new text.
Usage: rv2_patch_identity.py <RV109 sp2_mutants.py> <my mutants_sp.py>"""
import os, sys
rv_path, mine_path = sys.argv[1:3]
rv = {}
exec(compile(open(rv_path).read().split("\ndef edits_ok")[0], rv_path, "exec"), rv)
src = open(mine_path).read()
mine = {}
head = src.split("\nENV = ")[0]
g = {"__name__": "x"}
argv, sys.argv = sys.argv, ["x", "/M", "/L", "/T", "/C", "/TMP"]
exec(compile(head, mine_path, "exec"), g)
sys.argv = argv
ok = True
for mid in ["M15_selected_diagnostics_reversed", "M16_frozen_attempt_ref_zero", "M17_frozen_attempt_id_zero",
            "M19_frozen_run_owner_ordinal", "M28_headline_tie_location_only", "M32_headline_stress_only"]:
    _, rv_edits = rv["M"][mid]
    my_edits = dict(g["MUTANTS"])["RV2_" + mid]
    a = [(f, o, n) for f, o, n in rv_edits]
    b = [(os.path.basename(f), o, n) for f, o, n in my_edits]
    same = a == b
    ok &= same
    print(f"{mid}: {len(a)} edit(s) in {sorted({f for f, _, _ in a})}; identical={same}")
print("ALL IDENTICAL" if ok else "DIFFERENCE")
sys.exit(0 if ok else 1)
