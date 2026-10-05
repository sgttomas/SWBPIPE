"""RV79 single-edit mutants (confirmation 06: M33-M39 aimed at D37 on READER 85905e95e9; confirmation 05: adds M30-M32 aimed at D34, on READER 63355a91d2; confirmation 04: the 22 mutants plus M23-M29 aimed at D32, on READER abcb16fd27; confirmation 03: same 22 mutants on READER a894d9d0ba; confirmation 02: the same 22 mutants re-targeted to READER b36739112a; M04 and M20 follow the renamed R2/R3 lines)

Original: RV79 single-edit mutants of P/core/analysis_runs/retained_precision.py.

Each mutant starts from the pristine archived bytes (sha256 checked), applies exactly one
textual replacement (the old text must occur exactly once), runs the two Python test files
and restores the pristine bytes. Run from WT/rv79/P with the reader's environment.
"""
import hashlib, json, os, subprocess, sys, time
PRISTINE_SHA = "d77008e24fe1775c9def1e6b838fb535fb3155f1df3f427a227d3d990433228e"
TARGET = "core/analysis_runs/retained_precision.py"
MUTANTS = [
 ("M33_d37_removed", "D37: the error/stage-record check removed",
  'fail(tuple(st[k] for k in STAGE_ORDER) in ERROR_STAGE_RECORDS.get(result["error"]["kind"], ()))', 'pass'),
 ("M34_observable_narrowed", "D37: observable loses its g5a-failed record (F, F)",
  '"observable": {done(8, [F, C]), done(8, [F, F])}', '"observable": {done(8, [F, C])}'),
 ("M35_abandoned_widened_after_certificate", "D37: abandoned also admitted after a passed certificate",
  '"abandoned": {done(4, [F]), done(6, [F]), done(7, [])}', '"abandoned": {done(4, [F]), done(6, [F]), done(7, []), done(8, [])}'),
 ("M36_proof_drops_unchecked_pair", "D37: certify_final failure without the verdict copy (N, N) no longer admitted",
  'for x, y in ((N, N), (C, C), (C, F), (F, C), (F, F))', 'for x, y in ((C, C), (C, F), (F, C), (F, F))'),
 ("M37_values_widened", "D37: values also admitted with values completed",
  '"values": {done(5, [F])}', '"values": {done(5, [F]), done(6, [])}'),
 ("M38_numeric_widened", "D37: numeric also admitted with observables and g5a not entered",
  '"numeric": {done(10, [])}', '"numeric": {done(10, []), done(8, [])}'),
 ("M39_g5a_widened", "D37: g5a also admitted with g5a not entered",
  '"g5a": {done(9, [F])}', '"g5a": {done(9, [F]), done(8, [])}'),
]

def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()

def main(log_dir, python):
    pristine = open(TARGET, "rb").read()
    assert hashlib.sha256(pristine).hexdigest() == PRISTINE_SHA, "not the reviewed bytes"
    results = []
    only = set(sys.argv[3:])
    for mid, desc, old, new in MUTANTS:
        if only and mid not in only: continue
        text = pristine.decode()
        count = text.count(old)
        if count != 1:
            results.append({"id": mid, "description": desc, "status": f"NOT_APPLIED (occurrences={count})"}); continue
        open(TARGET, "w").write(text.replace(old, new))
        mutated_sha = sha(TARGET)
        t0 = time.time()
        proc = subprocess.run([python, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_schema.py"], capture_output=True, text=True, timeout=1200)
        open(TARGET, "wb").write(pristine)
        assert sha(TARGET) == PRISTINE_SHA
        out = proc.stdout + proc.stderr
        open(os.path.join(log_dir, f"mutant_{mid}.log"), "w").write(out)
        summary = [l for l in out.splitlines() if " passed" in l or " failed" in l or "error" in l.lower()][-1:] or ["?"]
        failed = sorted({l.split("::", 1)[1].split(" ")[0] for l in out.splitlines() if l.startswith("FAILED ")})
        results.append({"id": mid, "description": desc, "mutated_sha256": mutated_sha, "exit": proc.returncode,
                        "killed": proc.returncode != 0, "summary": summary[0], "killing_tests": failed[:12], "killing_test_count": len(failed),
                        "seconds": round(time.time() - t0, 1)})
        print(json.dumps(results[-1]), flush=True)
    json.dump(results, open(os.path.join(log_dir, "mutants.json"), "w"), indent=1)

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
