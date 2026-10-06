"""RV101 tsc exhaustiveness controls (scratch copy WT/rv101/tscc). Each control is one edit, tsc --noEmit, restore."""
import subprocess, pathlib, hashlib, json, sys
P = pathlib.Path("WT/rv101/tscc/projects/chirality-piping")
OUT = pathlib.Path(sys.argv[1]); OUT.mkdir(parents=True, exist_ok=True)
D = "apps/desktop/src/features/results/"
C = [
 ("C0", "unmodified", None, None, None),
 ("C1", "a new route in SourceContract (B3's physics_retained)", D + "numericalResultQuality.ts", '| "retained_preview_physics" | "unsupported";', '| "retained_preview_physics" | "physics_retained" | "unsupported";'),
 ("C2", "the successor entry leaves a surface unnamed (rule-check)", D + "outputPolicy.ts", '"rule-check": "refused", "report-lint"', '"report-lint"'),
 ("C3", "a new output surface is added to OUTPUT_SURFACES", D + "outputPolicy.ts", '"report-package", "result-export", "stress-neutral",\n] as const;', '"report-package", "result-export", "stress-neutral", "new-surface",\n] as const;'),
 ("C4", "a route entry is removed (unsupported)", D + "outputPolicy.ts", '  unsupported: UNGATED,\n', ''),
 ("C5", "a surface decision outside the two values", D + "outputPolicy.ts", '"result-export": "admitted_when_eligible", "stress-neutral"', '"result-export": "admitted", "stress-neutral"'),
]
res = []
for cid, desc, rel, old, new in C:
    orig = None
    if rel:
        path = P / rel; orig = path.read_bytes(); text = orig.decode(); n = text.count(old)
        if n != 1: res.append({"id": cid, "desc": desc, "status": f"anchor {n}"}); continue
        path.write_text(text.replace(old, new))
    try:
        r = subprocess.run(["../../node_modules/.bin/tsc", "--noEmit", "-p", "."], cwd=P / "apps/desktop", capture_output=True, text=True)
    finally:
        if rel: path.write_bytes(orig)
    errs = [l for l in r.stdout.splitlines() if "error TS" in l]
    (OUT / f"{cid}.log").write_text(r.stdout + r.stderr)
    res.append({"id": cid, "desc": desc, "exit": r.returncode, "errors": len(errs), "first": [e.split("/apps/desktop/")[-1][:220] for e in errs[:4]]})
    print(json.dumps(res[-1]), flush=True)
(OUT / "tsc_controls.json").write_text(json.dumps(res, indent=1))
