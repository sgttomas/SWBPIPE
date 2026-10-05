import os, subprocess, json
S = os.environ["S"]; WT = os.environ["WT"]; M = WT + "/rv90/mut/projects/chirality-piping"
RSF = M + "/core/reporting/result_export/src/retained_precision.rs"
F5RS = '        fail(list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>() == exact)?;'
MUT = {"RS01_f5_first_case_only": (F5RS, '        if i == 0 {\n' + F5RS + '\n        }'),
       "RS02_f5_prefix_tolerant": (F5RS, '        fail(exact.starts_with(&list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>()))?;')}
pristine = open(RSF, "rb").read()
for mid, (old, new) in MUT.items():
    t = pristine.decode(); assert t.count(old) == 1; open(RSF, "w").write(t.replace(old, new))
    try:
        assert subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode == 0
        p = subprocess.run(["cargo", "test", "--locked", "--offline", "--target-dir", WT + "/targets/rv90/mut", "--test", "rv90_dump"], cwd=M + "/core/reporting/result_export",
                           env=dict(os.environ, RV90_APPLIED=f"{S}/work/applied_probes_all.jsonl", RV90_OUT=f"{S}/work/out_rsmut_{mid}_probes.json", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2"), capture_output=True, text=True)
        assert p.returncode == 0, p.stderr[-2000:]
    finally:
        open(RSF, "wb").write(pristine)
    print(mid, "done")
