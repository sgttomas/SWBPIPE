#!/usr/bin/env python3
"""I61 U6e Python reader mutants: one textual edit each in the disposable `mut` tree, the two
Python contract files, killed/survived, then the pristine bytes restored."""
import hashlib, json, os, subprocess, sys, time
M = sys.argv[1]; LOG = sys.argv[2]
TARGET = os.path.join(M, "core/analysis_runs/retained_precision.py")
F5 = '        fail(refs == [d["id"] for d in diags if name in (d.get("affected_refs") or []) and not str(d.get("code")).startswith("RETAINED_PRECISION_")])'
MUTANTS = [
 ("PY01_f5_removed", F5, "        pass"),
 ("PY02_f5_retained_included", ' and not str(d.get("code")).startswith("RETAINED_PRECISION_")])', '])'),
 ("PY03_f5_order_free", F5, F5.replace("fail(refs == [", "fail(sorted(refs) == sorted([").replace('"RETAINED_PRECISION_")])', '"RETAINED_PRECISION_")]))')),
 ("PY04_f5_case_filter_removed", 'if name in (d.get("affected_refs") or []) and not str(', 'if not str('),
 # RV79's D37 mutants, verbatim (M35 and M37 survived 07f: RV79-N1).
 ("M33_d37_removed", 'fail(tuple(st[k] for k in STAGE_ORDER) in ERROR_STAGE_RECORDS.get(result["error"]["kind"], ()))', 'pass'),
 ("M34_observable_narrowed", '"observable": {done(8, [F, C]), done(8, [F, F])}', '"observable": {done(8, [F, C])}'),
 ("M35_abandoned_widened_after_certificate", '"abandoned": {done(4, [F]), done(6, [F]), done(7, [])}', '"abandoned": {done(4, [F]), done(6, [F]), done(7, []), done(8, [])}'),
 ("M36_proof_drops_unchecked_pair", 'for x, y in ((N, N), (C, C), (C, F), (F, C), (F, F))', 'for x, y in ((C, C), (C, F), (F, C), (F, F))'),
 ("M37_values_widened", '"values": {done(5, [F])}', '"values": {done(5, [F]), done(6, [])}'),
 ("M38_numeric_widened", '"numeric": {done(10, [])}', '"numeric": {done(10, []), done(8, [])}'),
 ("M39_g5a_widened", '"g5a": {done(9, [F])}', '"g5a": {done(9, [F]), done(8, [])}'),
 # RV80-N2 in Python: the D32 normalization widened to the statement (Rust's M50).
 ("PY_M50_statement_normalized", '_normalize_integrals(receipt)  # D34, then D32', '_normalize_integrals(snapshot)  # D34, then D32'),
]
pristine = open(TARGET, "rb").read(); psha = hashlib.sha256(pristine).hexdigest()
only = set(sys.argv[3:]); results = []
for mid, old, new in MUTANTS:
    if only and mid not in only: continue
    text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
    open(TARGET, "w").write(text.replace(old, new))
    try:
        t0 = time.time()
        proc = subprocess.run([sys.executable, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_schema.py"], cwd=M, capture_output=True, text=True, timeout=1200)
    finally:
        open(TARGET, "wb").write(pristine); assert hashlib.sha256(open(TARGET, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"mutant_{mid}.log"), "w").write(out)
    failed = sorted({l.split("::", 1)[1].split(" ")[0] for l in out.splitlines() if l.startswith("FAILED ")})
    syntax = "SyntaxError" in out or "ImportError" in out
    r = {"id": mid, "killed": proc.returncode != 0, "error_not_test": syntax, "killing_tests": failed[:8], "killing_test_count": len(failed), "seconds": round(time.time() - t0, 1)}
    results.append(r); print(json.dumps(r), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants_py.json"), "w"), indent=1)
