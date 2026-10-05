import os, subprocess, json, sys
S = os.environ["S"]; WT = os.environ["WT"]; M = WT + "/rv90/mut/projects/chirality-piping"; F = M + "/core/analysis_runs/retained_precision.py"
F5PY = '        fail(refs == [d["id"] for d in diags if name in (d.get("affected_refs") or []) and not str(d.get("code")).startswith("RETAINED_PRECISION_")])'
MUT = {"PY01_f5_first_case_only": (F5PY, "        if i == 0:" + F5PY[7:]),
       "PY_list_only_membership_remedy": ('name in (d.get("affected_refs") or [])', 'isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]')}
pristine = open(F, "rb").read()
for mid, (old, new) in MUT.items():
    t = pristine.decode(); assert t.count(old) == 1; open(F, "w").write(t.replace(old, new))
    try:
        p = subprocess.run([os.environ["VENV"] + "/bin/python", S + "/work/py_validate.py", S + "/work/applied_probes_all.jsonl", f"{S}/work/out_pymut_{mid}_probes.json"], env=dict(os.environ, RV90_READER_ROOT=M), capture_output=True, text=True, cwd=S + "/work")
        assert p.returncode == 0, p.stderr[-2000:]
    finally:
        open(F, "wb").write(pristine)
    assert open(F, "rb").read() == pristine
    print(mid, "done")
