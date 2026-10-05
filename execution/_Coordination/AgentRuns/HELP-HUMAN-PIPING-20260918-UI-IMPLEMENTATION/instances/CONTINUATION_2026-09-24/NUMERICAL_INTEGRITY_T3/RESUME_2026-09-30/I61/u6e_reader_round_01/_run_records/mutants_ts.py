#!/usr/bin/env python3
"""I61 U6e TypeScript reader mutants in the disposable `mutts` lane: one edit each, the
retained-precision vitest file, killed/survived, bytes restored."""
import hashlib, json, os, subprocess, sys, time
D, LOG = sys.argv[1], sys.argv[2]
TARGET = os.path.join(D, "src/features/results/retainedPrecision.ts")
F5 = "    fail(a.diagnostic_refs.length === exact.length && a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));"
MUTANTS = [
 ("TS01_f5_removed", F5, "    void exact;"),
 ("TS02_f5_order_free", F5, "    fail(a.diagnostic_refs.length === exact.length && a.diagnostic_refs.every((ref: string) => exact.includes(ref)));"),
 ("TS03_f5_retained_included", " && !String(d.code).startsWith('RETAINED_PRECISION_')).map(d => d.id);", ").map(d => d.id);"),
 ("TS04_d37_values_widened", "  values: ['CCCCCF----'],", "  values: ['CCCCCF----', 'CCCCCC----'],"),
 ("TS05_d37_abandoned_widened_after_certificate", "  abandoned: ['CCCCF-----', 'CCCCCCF---', 'CCCCCCC---'],", "  abandoned: ['CCCCF-----', 'CCCCCCF---', 'CCCCCCC---', 'CCCCCCCC--'],"),
]
pristine = open(TARGET, "rb").read(); psha = hashlib.sha256(pristine).hexdigest(); results = []
for mid, old, new in MUTANTS:
    text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
    open(TARGET, "w").write(text.replace(old, new))
    try:
        t0 = time.time()
        proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "900", "../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts"], cwd=D, capture_output=True, text=True)
    finally:
        open(TARGET, "wb").write(pristine); assert hashlib.sha256(open(TARGET, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"mutant_{mid}.log"), "w").write(out)
    failed = [l.strip() for l in out.splitlines() if l.strip().startswith(("×", "FAIL"))][:8]
    summary = [l.strip() for l in out.splitlines() if "Tests" in l and ("passed" in l or "failed" in l)][-1:]
    r = {"id": mid, "killed": proc.returncode != 0, "summary": summary, "failing": failed, "seconds": round(time.time() - t0, 1)}
    results.append(r); print(json.dumps(r), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants_ts.json"), "w"), indent=1)
