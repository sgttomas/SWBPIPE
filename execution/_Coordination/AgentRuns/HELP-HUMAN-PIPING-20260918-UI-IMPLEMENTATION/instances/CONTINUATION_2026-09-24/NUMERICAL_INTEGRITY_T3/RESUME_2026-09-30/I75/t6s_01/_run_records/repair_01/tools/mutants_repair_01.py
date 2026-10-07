"""I75 REPAIR_01 mutant programme for rustLowerExp (RV101 SF-1; the general form). Each mutant is one exact, single-occurrence
replacement in a scratch copy of retainedPrecisionDisclosure.ts; the control run must pass; a mutant is killed
when the selected test files fail. The file is restored and compared byte for byte after every run.
Usage: python3 mutants_repair_01.py <copy>/projects/chirality-piping <log-dir>"""
import json, pathlib, subprocess, sys
P = pathlib.Path(sys.argv[1]); LOG = pathlib.Path(sys.argv[2]); LOG.mkdir(parents=True, exist_ok=True)
DIS = P / "apps/desktop/src/features/results/retainedPrecisionDisclosure.ts"
TESTS = ["src/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx", "src/features/result-export/retainedPrecisionResultExport.test.tsx"]
M = [
 ("S01", "repair reverted: the shortest form only (V8 ties to even)", '(Number(nearest) === magnitude ? nearest : shortest)', 'shortest'),
 ("S02", "first form: only a 17-digit shortest form reprinted (misses 16-digit ties)", '(Number(nearest) === magnitude ? nearest : shortest)', '(shortest.split("e")[0].replace(".", "").length === 17 ? nearest : shortest)'),
 ("S03", "round-trip guard dropped (nearest always printed)", '(Number(nearest) === magnitude ? nearest : shortest)', 'nearest'),
 ("S04", "one digit too many (toExponential(n))", '.replace(".", "").length - 1)', '.replace(".", "").length)'),
 ("S05", "digit count includes the point", 'shortest.split("e")[0].replace(".", "").length - 1', 'shortest.split("e")[0].length - 1'),
 ("S06", "signed value reprinted (a negative tie keeps V8's lower candidate)", 'magnitude.toExponential(shortest', 'value.toExponential(shortest'),
 ("S07", "guard compared with the signed value", 'Number(nearest) === magnitude', 'Number(nearest) === value'),
 ("S08", "e+ exponent left unnormalized", ' : shortest).replace("e+", "e")}`', ' : shortest)}`'),
 ("S09", "negative-zero sign dropped", 'value < 0 || Object.is(value, -0) ? "-" : ""', 'value < 0 ? "-" : ""'),
]
orig = DIS.read_bytes(); src = orig.decode()
def run(tag):
    r = subprocess.run(["../../node_modules/.bin/vitest", "run", *TESTS], cwd=P / "apps/desktop", capture_output=True, text=True)
    (LOG / f"{tag}.log").write_text(r.stdout[-6000:] + "\n--- stderr ---\n" + r.stderr[-3000:])
    return r.returncode
out = {"control": None, "mutants": []}
out["control"] = run("control")
if out["control"] != 0: print("CONTROL FAILED"); json.dump(out, open(LOG / "summary.json", "w"), indent=1); sys.exit(1)
for mid, desc, old, new in M:
    n = src.count(old); row = {"id": mid, "description": desc, "occurrences": n}
    if n != 1: row["status"] = "NOT_APPLIED"
    else:
        DIS.write_text(src.replace(old, new))
        try: rc = run(mid)
        finally: DIS.write_bytes(orig)
        row["rc"] = rc; row["status"] = "killed" if rc != 0 else "SURVIVED"
    assert DIS.read_bytes() == orig
    out["mutants"].append(row); print(mid, row["status"], flush=True)
json.dump(out, open(LOG / "summary.json", "w"), indent=1)
print("restored", DIS.read_bytes() == orig)
