"""I92 (B1 SR-TS) mutants: one edit each to a scratch archive of the head's TS reader, run against
retainedPrecision.test.ts (the TS lane), then the reader's bytes are restored and checked.
A mutant is killed when the file loads (all its tests are collected) and at least one assertion fails.
Run the whole programme as one job under the shared T3 lock."""
import hashlib, json, os, subprocess, sys

WT = "WT"
S = WT + "/scratch/i92_b1_sr_ts"
DESK = S + "/mut/projects/chirality-piping/apps/desktop"
TS = DESK + "/src/features/results/retainedPrecision.ts"
OUT = S + "/logs/mutants"
os.makedirs(OUT, exist_ok=True)
ORIGINAL = open(TS, "rb").read()
ORIGINAL_SHA = hashlib.sha256(ORIGINAL).hexdigest()

RELAXED = "    fail(stage.native === 'not_entered' ? a.run_ref === null : a.run_ref !== null || (stage.native === 'failed' && d38CaptureBeforeRun(a, c, s)));"
NOT_REQ = "if (c.status === 'not_required') fail(c.product_attempt_ref === null && a.initial.kind !== 'not_attempted' && q.solve_quality === 'checks_passed');"
P1 = "    fail(modeRows.length === 1 && modeRows[0].value === (invocation.solver_mode === 'dense_scrutiny' ? 2 : 1));"
MODES = "    const modeRows = own('linear_solver_mode_basis');"
MUTANTS = [
    ("N0", "the unmutated head", None, None),
    # R-D38 (DESIGN_v2 §2)
    ("D38-1", "the relaxed check restored: an entered native stage needs a Run", RELAXED, "    fail((stage.native === 'not_entered') === (a.run_ref === null));"),
    ("D38-2", "rule 3 dropped from the relaxed check: (4b) admits a completed native stage with no Run", RELAXED, "    fail(stage.native === 'not_entered' ? a.run_ref === null : a.run_ref !== null || d38CaptureBeforeRun(a, c, s));"),
    ("D38-3", "(4b) result unavailable dropped", "return a.result.kind === 'unavailable' && a.result.error?.kind === 'capture'", "return true && a.result.error?.kind === 'capture'"),
    ("D38-4", "(4b) error.kind capture dropped", "return a.result.kind === 'unavailable' && a.result.error?.kind === 'capture'", "return a.result.kind === 'unavailable'"),
    ("D38-5", "(4b) preparation completed dropped", "    && stage.preparation === 'completed' && STAGE_ORDER", "    && STAGE_ORDER"),
    ("D38-6", "(4b) later stages not_entered dropped", " && STAGE_ORDER.slice(2).every(k => stage[k] === 'not_entered')", ""),
    ("D38-7", "(4b) case unavailable dropped", "    && c.status === 'unavailable' && c.reason?.cause?.kind", "    && c.reason?.cause?.kind"),
    ("D38-8", "(4b) cause kind dropped", "&& c.reason?.cause?.kind === 'prepared_product_failure' && c.reason.cause.product_attempt_ref === a.id", "&& c.reason?.cause?.product_attempt_ref === a.id"),
    ("D38-9", "(4b) cause attempt dropped", " && c.reason.cause.product_attempt_ref === a.id\n", "\n"),
    ("D38-10", "(4b) reason code dropped", "    && c.reason.code === 'source_unavailable' && c.reason.phase", "    && c.reason.phase"),
    ("D38-11", "(4b) reason phase dropped", " && c.reason.phase === 'preparation'\n", "\n"),
    ("D38-12", "(4b) source non-null dropped", "    && s !== null && s.preparation?.attempt_ref === a.id", "    && s?.preparation?.attempt_ref === a.id"),
    ("D38-13", "(4b) the source binds the attempt dropped", "    && s !== null && s.preparation?.attempt_ref === a.id && a.source_ref", "    && s !== null && a.source_ref"),
    ("D38-14", "(4b) source equality dropped", " && a.source_ref === c.source_ref;\n}", ";\n}"),
    # G8 (DESIGN_v2 §3.2-§3.3)
    ("G8-1", "P1-P4 for case 0 only (the per-case loop narrowed)", MODES, "    if (ci > 0) continue; const modeRows = own('linear_solver_mode_basis');"),
    ("G8-2", "P1-P4 for selected cases only", MODES, "    if (b.cases[ci].status !== 'selected') continue; const modeRows = own('linear_solver_mode_basis');"),
    ("G8-3", "P1 dropped", P1, "    fail(true);"),
    ("G8-4", "P1's exactly one dropped (the first row's value only)", P1, "    fail(modeRows.length >= 1 && modeRows[0].value === (invocation.solver_mode === 'dense_scrutiny' ? 2 : 1));"),
    ("G8-5", "P2 dropped", "    fail(parityRows.length <= 1);", "    fail(true);"),
    ("G8-6", "P3 dropped", "    fail(!parityRows.length || invocation.solver_mode === 'dense_scrutiny');", "    fail(true);"),
    ("G8-7", "P4 dropped", "    fail(!parityRows.length || ordinary.w2?.kind !== 'published');", "    fail(true);"),
    ("G8-11", "P4 keyed on a triggered W2, not a published one", "    fail(!parityRows.length || ordinary.w2?.kind !== 'published');", "    fail(!parityRows.length || ordinary.w2?.kind === 'not_triggered');"),
    ("G8-8", "mode code 3 restored", P1, "    fail(modeRows.length === 1 && (invocation.solver_mode === 'dense_scrutiny' ? modeRows[0].value === 2 : [1, 3].includes(modeRows[0].value)));"),
    ("G8-9", "the requested mode's code back to INVOCATION_MISMATCH", "fail(ordinary.requested_mode === invocation.solver_mode);", "fail(ordinary.requested_mode === invocation.solver_mode, 'INVOCATION_MISMATCH');"),
    ("G8-10", "P1's code back to INVOCATION_MISMATCH", P1, P1[:-2] + ", 'INVOCATION_MISMATCH');"),
    # G5 not_required (DESIGN_v2 §3.3)
    ("G5-1", "not_required: initial.kind == report restored", NOT_REQ, NOT_REQ[:-2] + " && a.initial.kind === 'report');"),
    ("G5-2", "not_required: initial.outcome == checks_passed restored", NOT_REQ, NOT_REQ[:-2] + " && a.initial.outcome === 'checks_passed');"),
    ("G5-3", "not_required: w2.kind == not_triggered restored", NOT_REQ, NOT_REQ[:-2] + " && a.w2.kind === 'not_triggered');"),
    ("G5-4", "not_required: product_attempt_ref null dropped (kept rule)", NOT_REQ, NOT_REQ.replace("c.product_attempt_ref === null && ", "")),
    ("G5-5", "not_required: initial not_attempted dropped (kept rule)", NOT_REQ, NOT_REQ.replace(" && a.initial.kind !== 'not_attempted'", "")),
    ("G5-6", "not_required: verdict checks_passed dropped (kept rule)", NOT_REQ, NOT_REQ.replace(" && q.solve_quality === 'checks_passed'", "")),
    # RV108 N4
    ("N4-1", "the N4 rule removed (every row read with Object.hasOwn)", "for (const row of p.results) if (isObj(row) && Object.hasOwn(row, 'recovery_method'))", "for (const row of p.results) if (Object.hasOwn(row, 'recovery_method'))"),
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
