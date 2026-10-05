#!/usr/bin/env python3
"""I61 U3 grant 2 follow-on (D-U6-5) mutants: one textual edit each in the disposable `mut` tree (a copy of the
candidate), PP's whole `--lib` suite in the registered build, killed/survived, the failing tests,
and whether a compile error; pristine bytes restored and compared after every run.

Usage: mutants.py MUT_P_CORE LOG_DIR TARGET_DIR [ids...]
"""
import hashlib, json, os, subprocess, sys, time
M, LOG, TD = sys.argv[1], sys.argv[2], sys.argv[3]
only = set(sys.argv[4:])
PP = os.path.join(M, "product_physics")
LIB = os.path.join(PP, "src/lib.rs")
PROD = os.path.join(PP, "src/retained_product.rs")

FIX = os.path.join(M, "../fixtures/results")
MUTANTS = [
    # D-U6-5: one byte of U6's sparse carrier fixture (the copy in the mut tree) changed.
    ("DU65_carrier_byte", os.path.join(FIX, "retained_precision_milestone_successor_sparse_interactive.json"),
     '"id": "u1_milestone_sparse_interactive"', '"id": "u1_milestone_sparse_interactivE"'),
    # D-U6-5: the published successor document gains one key at the transfer.
    ("DU65_successor_altered", LIB,
     "    (frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))",
     "    (frozen.into_ordinary(), Ok(RetainedSuccessor({ let mut s = successor; s[\"i61\"] = serde_json::json!(1); s })))"),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
ENV.pop("RUSTFLAGS", None)
results = []
for mid, target, old, new in MUTANTS:
    if only and mid not in only:
        continue
    pristine = open(target, "rb").read(); psha = hashlib.sha256(pristine).hexdigest()
    text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
    open(target, "w").write(text.replace(old, new))
    t0 = time.time()
    try:
        if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0:
            sys.exit("MEMGUARD NOT RUNNING")
        proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1200", "cargo", "test", "--locked", "--offline", "--no-fail-fast",
                               "--lib", "--target-dir", TD], cwd=PP, env=ENV, capture_output=True, text=True)
    finally:
        open(target, "wb").write(pristine)
        assert hashlib.sha256(open(target, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"mutant_{mid}.log"), "w").write(out)
    failing = sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
    failing_new = [f for f in failing if "u3g2_" in f]
    # The Mac t13 (s11g_tests::t13_committed_fallback_uz_is_byte_identical) fails at base too; it
    # neither kills nor saves a mutant.
    failing = [f for f in failing if f != "s11g_tests::t13_committed_fallback_uz_is_byte_identical"]
    compile_error = "error[" in out or "could not compile" in out
    r = {"id": mid, "killed": bool(failing) or compile_error, "compile_error": compile_error,
         "killed_by_new_tests": failing_new, "failing_count": len(failing),
         "failing": failing[:12], "seconds": round(time.time() - t0, 1)}
    results.append(r); print(json.dumps(r), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants.json" if not only else "mutants_" + "_".join(sorted(only)) + ".json"), "w"), indent=1)
