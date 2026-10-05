#!/usr/bin/env python3
"""I61 U6e Rust reader mutants in the disposable `mut` tree: one edit each, the whole
result_export suite, killed/survived (and whether a compile error), bytes restored."""
import hashlib, json, os, subprocess, sys, time
M, LOG, TARGET_DIR = sys.argv[1], sys.argv[2], sys.argv[3]
CRATE = os.path.join(M, "core/reporting/result_export")
TARGET = os.path.join(CRATE, "src/retained_precision.rs")
MUTANTS = [
 ("RS01_f5_removed", '        fail(list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>() == exact)?;', '        let _ = exact;'),
 ("RS02_f5_retained_included", '.filter(|d| list(&d["affected_refs"]).contains(cid) && !text(&d["code"]).starts_with("RETAINED_PRECISION_"))', '.filter(|d| list(&d["affected_refs"]).contains(cid))'),
 ("RS03_d37_values_widened", '        "values" => first_failed == Some("values"),', '        "values" => first_failed == Some("values") || (first_failed.is_none() && is("values", "completed") && is("aliases", "not_entered")),'),
 ("RS04_d37_abandoned_widened_after_certificate", '                    && is("certificate", "not_entered"))', '                    && is("certificate", "not_entered"))\n                || (certified && is("observables", "not_entered"))'),
 ("RS05_m50_statement_normalized", '    if let Some(r) = owned.get_mut("retained_precision") {\n        normalize(r);\n    }', '    normalize(&mut owned);'),
]
pristine = open(TARGET, "rb").read(); psha = hashlib.sha256(pristine).hexdigest()
only = set(sys.argv[4:]); results = []
for mid, old, new in MUTANTS:
    if only and mid not in only: continue
    text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
    open(TARGET, "w").write(text.replace(old, new))
    try:
        if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0: sys.exit("MEMGUARD NOT RUNNING")
        t0 = time.time()
        env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
        proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1200", "cargo", "test", "--locked", "--offline", "--no-fail-fast", "--target-dir", TARGET_DIR],
                              cwd=CRATE, env=env, capture_output=True, text=True)
    finally:
        open(TARGET, "wb").write(pristine); assert hashlib.sha256(open(TARGET, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"mutant_{mid}.log"), "w").write(out)
    failed = sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
    r = {"id": mid, "killed": proc.returncode != 0, "compile_error": "error[" in out or "could not compile" in out, "killing_tests": failed[:8], "seconds": round(time.time() - t0, 1)}
    results.append(r); print(json.dumps(r), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants_rs.json"), "w"), indent=1)
