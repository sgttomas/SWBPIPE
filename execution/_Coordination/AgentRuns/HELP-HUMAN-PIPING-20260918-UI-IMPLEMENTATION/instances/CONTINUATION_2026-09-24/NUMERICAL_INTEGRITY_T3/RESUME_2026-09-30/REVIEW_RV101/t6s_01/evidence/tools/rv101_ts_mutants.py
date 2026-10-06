"""RV101's own TypeScript mutants (scratch). Same mechanics as I75's driver: one exact,
single-occurrence replacement in the mutation copy, the implementers' ten test files run, the
file restored and checked by sha256. Usage: rv101_ts_mutants.py <copy>/projects/chirality-piping <log-dir> [ids]"""
import hashlib, json, pathlib, subprocess, sys
P = pathlib.Path(sys.argv[1]); LOG = pathlib.Path(sys.argv[2]); LOG.mkdir(parents=True, exist_ok=True)
F = "apps/desktop/src/features/"
DIS, SN, RE_, POL = F + "results/retainedPrecisionDisclosure.ts", F + "stress-neutral/StressNeutralExportPanel.tsx", F + "result-export/resultExportAdapter.ts", F + "results/outputPolicy.ts"
TESTS = ["src/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx", "src/features/results/outputPolicy.test.ts",
         "src/features/results/retainedPrecisionOutputRefusal.test.tsx", "src/features/results/loadReferenceOutputRefusal.test.tsx",
         "src/features/stress-neutral/StressNeutralExportPanel.test.tsx", "src/features/result-export/ResultExportPanel.test.tsx",
         "src/features/result-export/retainedPrecisionResultExport.test.tsx", "src/features/result-export/resultExportAdapter.test.ts",
         "src/features/result-export/physicsResultExport.test.ts", "src/features/results/retainedPrecisionIntegration.test.tsx"]
M = [
 ("RV-T1", "absolute row in a unit the reader does not normalize is valued (no disclosure)", DIS,
  'if (si === null) return { code: RETAINED_NOT_COVERED, message: retainedNotCoveredMessage(kind) };', 'if (si === null) return null;'),
 ("RV-T2", "header-only validator treats a not_covered withholding as blocking", SN,
  'new Set(["diagnostic_work", "retained_absolute_verified", "retained_not_covered"])', 'new Set(["diagnostic_work", "retained_absolute_verified"])'),
 ("RV-T3", "loss-report class sentence swaps the absolute and uncovered counts (builder and validator agree)", SN,
  'Of these, ${absolute} rows are verified only to the retained-precision receipt\'s absolute bound and ${uncovered} rows are uncovered',
  'Of these, ${uncovered} rows are verified only to the retained-precision receipt\'s absolute bound and ${absolute} rows are uncovered'),
 ("RV-T4", "not_covered disclosure message loses its withholding clause", DIS,
  'no verified accuracy for this quantity kind; source value/unit and annotation retained; withheld from rule binding and reliance`',
  'no verified accuracy for this quantity kind; source value/unit and annotation retained`'),
 ("RV-T5", "the bound is printed in the row's source unit instead of the SI unit", DIS,
  'absolute bound b = ${rustLowerExp(decodeBinary64(bits))} ${si} (binary64', 'absolute bound b = ${rustLowerExp(decodeBinary64(bits))} ${unit} (binary64'),
 ("RV-T6", "the policy fails open when standing cannot be computed", POL,
  'catch { standing = { eligible: false, findings: [] }; }', 'catch { standing = { eligible: true, findings: [] }; }'),
 ("RV-T7", "the derivative copies the receipt without its body hash member (not whole)", RE_,
  "if(route==='retained_preview_physics')e.retained_precision=structuredClone(source.retained_precision);",
  "if(route==='retained_preview_physics'){e.retained_precision=structuredClone(source.retained_precision);delete e.retained_precision.receipt_sha256;}"),
]
def run(tag):
    r = subprocess.run(["../../node_modules/.bin/vitest", "run", "--maxWorkers=6", "--reporter=json", f"--outputFile={LOG}/{tag}.json", *TESTS], cwd=P / "apps/desktop", capture_output=True, text=True)
    try:
        d = json.load(open(LOG / f"{tag}.json"))
        return d["numFailedTests"], d["numTotalTests"], [t["fullName"] for f in d["testResults"] for t in f["assertionResults"] if t["status"] == "failed"][:3], d["numFailedTestSuites"]
    except Exception:
        return None, None, [r.stdout[-500:], r.stderr[-500:]], None
ONLY = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else None
results = []
ctl = run("control"); results.append({"id": "control", "failed": ctl[0], "total": ctl[1]}); print("control", ctl[:2], flush=True)
assert ctl[0] == 0, ctl
for mid, desc, rel, old, new in M:
    if ONLY is not None and mid not in ONLY:
        continue
    path = P / rel; orig = path.read_bytes(); digest = hashlib.sha256(orig).hexdigest(); text = orig.decode()
    n = text.count(old)
    if n != 1:
        results.append({"id": mid, "desc": desc, "status": f"not applied ({n})"}); print(mid, "not applied", n, flush=True); continue
    path.write_text(text.replace(old, new))
    try:
        failed, total, names, suites = run(mid)
    finally:
        path.write_bytes(orig)
        assert hashlib.sha256(path.read_bytes()).hexdigest() == digest
    status = "killed" if failed else ("killed (suite error)" if suites else "SURVIVED")
    print(mid, status, failed, total, names[:2], flush=True)
    results.append({"id": mid, "desc": desc, "file": rel, "status": status, "failed": failed, "total": total, "first_failures": names})
json.dump(results, open(LOG / "rv101_ts_mutants.json", "w"), indent=1)
