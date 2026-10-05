"""RV91 r2: RVb1/RVb2 (which survive the committed tests) against RV91's r2 probe."""
import importlib.util, json, os, subprocess, sys
T3 = "WT"; S = f"{T3}/scratch/rv91_u6d"
LANE = f"{T3}/rv91/mut2/projects/chirality-piping/apps/desktop"
spec = importlib.util.spec_from_file_location("r2", f"{S}/r2/rv91_mutants_r2.py"); r2 = importlib.util.module_from_spec(spec); spec.loader.exec_module(r2)
plan = {m[0]: m for m in r2.PLAN}; out = {}
for mid in ("RVb1", "RVb2"):
    _, path, old, new, desc = plan[mid]; full = f"{LANE}/{path}"; text = open(full).read(); assert text.count(old) == 1
    open(full, "w").write(text.replace(old, new))
    try:
        p = subprocess.run(["../../node_modules/.bin/vitest", "run", "src/features/results/zzRV91R2.test.tsx"], cwd=LANE, env=dict(os.environ, TMPDIR=f"{S}/tmp", RV91_REVIEW_OUT=f"{S}/r2/probe_{mid}.json"), capture_output=True, text=True, timeout=900)
    finally: open(full, "w").write(text)
    out[mid] = {"desc": desc, "killed_by_rv91_probe": p.returncode != 0}
    print(mid, out[mid], flush=True)
json.dump(out, open(f"{S}/r2/probe_vs_mutants.json", "w"), indent=1)
