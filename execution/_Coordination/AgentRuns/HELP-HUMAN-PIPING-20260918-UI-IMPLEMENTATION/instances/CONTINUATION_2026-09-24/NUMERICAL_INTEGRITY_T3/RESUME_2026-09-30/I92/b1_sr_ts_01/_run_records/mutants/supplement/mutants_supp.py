"""I92 (B1 SR-TS) mutants: one edit each to a scratch archive of the head's TS reader, run against
retainedPrecision.test.ts (the TS lane), then the reader's bytes are restored and checked.
A mutant is killed when the file loads (all its tests are collected) and at least one assertion fails.
Run the whole programme as one job under the shared T3 lock."""
import hashlib, json, os, subprocess, sys

WT = "WT"
S = WT + "/scratch/i92_b1_sr_ts"
DESK = S + "/mut/projects/chirality-piping/apps/desktop"
TS = DESK + "/src/features/results/retainedPrecision.ts"
OUT = S + "/logs/mutants_supp"
os.makedirs(OUT, exist_ok=True)
ORIGINAL = open(TS, "rb").read()
ORIGINAL_SHA = hashlib.sha256(ORIGINAL).hexdigest()

RELAXED = "    fail(stage.native === 'not_entered' ? a.run_ref === null : a.run_ref !== null || (stage.native === 'failed' && d38CaptureBeforeRun(a, c, s)));"
NOT_REQ = "if (c.status === 'not_required') fail(c.product_attempt_ref === null && a.initial.kind !== 'not_attempted' && q.solve_quality === 'checks_passed');"
P1 = "    fail(modeRows.length === 1 && modeRows[0].value === (invocation.solver_mode === 'dense_scrutiny' ? 2 : 1));"
MODES = "    const modeRows = own('linear_solver_mode_basis');"
MUTANTS = [
    # Supplement after RV113 N-1: R-D38 (4a) refused (a failed native stage then needs (4b), Run or not).
    ("D38-15", "(4a) dropped from the relaxed check: a failed native stage needs (4b) even with a Run", RELAXED, "    fail(stage.native === 'not_entered' ? a.run_ref === null : stage.native === 'failed' ? d38CaptureBeforeRun(a, c, s) : a.run_ref !== null);"),
]

def run(mid):
    out = f"{OUT}/{mid}.json"
    env = dict(os.environ, TMPDIR=S + "/tmp/mut")
    os.makedirs(env["TMPDIR"], exist_ok=True)
    p = subprocess.run(["../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts", "--reporter=json", f"--outputFile={out}"],
                       cwd=DESK, env=env, capture_output=True, text=True)
    open(f"{OUT}/{mid}.log", "w").write(p.stdout + p.stderr)
    r = json.load(open(out))
    failed = [a["fullName"] for t in r["testResults"] for a in t["assertionResults"] if a["status"] == "failed"]
    load_errors = [t.get("message", "") for t in r["testResults"] if not t["assertionResults"]]
    return p.returncode, r["numTotalTests"], r["numPassedTests"], failed, load_errors

results = []
for mid, what, old, new in MUTANTS:
    text = ORIGINAL.decode()
    if old is not None:
        assert text.count(old) == 1, (mid, text.count(old))
        open(TS, "w").write(text.replace(old, new))
    try:
        rc, total, passed, failed, load = run(mid)
    finally:
        open(TS, "wb").write(ORIGINAL)
        assert hashlib.sha256(open(TS, "rb").read()).hexdigest() == ORIGINAL_SHA, "restore failed"
    state = "passes" if mid == "N0" and rc == 0 and not failed else ("killed" if failed and not load and total == 490 else ("LOAD_FAILURE" if load or total != 490 else "survived"))
    rec = {"id": mid, "what": what, "old": old, "new": new, "rc": rc, "total": total, "passed": passed, "result": state, "failed_tests": failed, "load_errors": load}
    results.append(rec)
    print(json.dumps({k: rec[k] for k in ("id", "result", "total", "passed")}), "|", "; ".join(f.split(" > ")[-1][:90] for f in failed[:4]), flush=True)
with open(f"{OUT}/mutants.jsonl", "w") as f:
    for rec in results: f.write(json.dumps(rec) + "\n")
print("restored sha256", hashlib.sha256(open(TS, "rb").read()).hexdigest(), "==", ORIGINAL_SHA)
