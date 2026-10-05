import os, subprocess, json, sys
S = os.environ["S"]; M = os.environ["WT"] + "/rv90/mut/projects/chirality-piping"
TSF = M + "/apps/desktop/src/features/results/retainedPrecision.ts"
F5TS = "    fail(a.diagnostic_refs.length === exact.length && a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));"
MUT = {"pristine": None,
 "TS01_f5_length_check_dropped": (F5TS, "    fail(a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));"),
 "TS02_f5_first_case_only": (F5TS, "    if (ci === 0)" + F5TS[3:]),
 "TS04_f5_isArray_guard_dropped": ("ds.filter(d => Array.isArray(d.affected_refs) && d.affected_refs.includes(cid)", "ds.filter(d => d.affected_refs?.includes(cid)")}
pristine = open(TSF, "rb").read()
for mid, e in MUT.items():
    if e:
        t = pristine.decode(); assert t.count(e[0]) == 1; open(TSF, "w").write(t.replace(e[0], e[1]))
    try:
        out = f"{S}/work/out_tsmut_{mid}_probes.json"
        p = subprocess.run(["../../node_modules/.bin/vitest", "run", "src/features/results/rv90_dump.test.ts", "--maxWorkers=1"], cwd=M + "/apps/desktop", env=dict(os.environ, RV90_APPLIED=f"{S}/work/applied_probes_all.jsonl", RV90_OUT=out), capture_output=True, text=True)
        assert p.returncode == 0, p.stdout[-2000:]
    finally:
        open(TSF, "wb").write(pristine)
    print(mid, "done")
