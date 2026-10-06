#!/usr/bin/env python3
"""I68 U8-1 mutants: one textual edit each in the disposable `mut` tree (base b1e2d7741e plus the
candidate's three files), then PP's whole `--lib` suite in the registered build through
WT/tools/t3_cargo.sh. Records killed/survived, the failing tests with their panic messages, and
whether the build failed (a compile error is not a kill). Pristine bytes are restored and compared
after every run.

Usage: mutants.py MUT_P LOG_DIR TARGET_DIR T3_CARGO [ids...]
"""
import hashlib, json, os, re, subprocess, sys, time
M, LOG, TD, CARGO = sys.argv[1:5]
only = set(sys.argv[5:])
PP = os.path.join(M, "core/product_physics")
LIB = os.path.join(PP, "src/lib.rs")
TESTS = os.path.join(PP, "src/retained_facade_tests.rs")
FIX = os.path.join(M, "fixtures/results/retained_precision_l0_successor_sparse_interactive.json")
MUTANTS = [
    # The brief's control: restoring all three loads must fail the Candidate assertion.
    ("M1_restore_all_three_loads", TESTS,
     '    raw["model"]["load_cases"][0]["primitive_loads"] = json!([first]);\n    raw\n}',
     '    let _ = first;\n    raw\n}'),
    # The brief's control: dropping the notice must fail the byte assertion (each real-input path).
    ("M2_candidate_without_notice", LIB,
     "        Err(refusal) => return notice.publish(refusal.ordinary, W1Fallback::Candidate),",
     "        Err(refusal) => return (refusal.ordinary, Err(W1Fallback::Candidate)),"),
    ("M3_preparation_without_notice", LIB,
     "        Err(failure) => return notice.publish(failure.ordinary, W1Fallback::Preparation),",
     "        Err(failure) => return (failure.ordinary, Err(W1Fallback::Preparation)),"),
    ("M4_native_without_notice", LIB,
     "        return notice.publish(prepared.into_ordinary(), W1Fallback::Native);",
     "        return (prepared.into_ordinary(), Err(W1Fallback::Native));"),
    # The brief's control: for L = 0, a corrupted body-1 row must fail the pin.
    ("M5_l0_body1_row_corrupted", TESTS,
     "        assert_eq!(output.successor(), Some(&successor));\n        let text = u8_l0_document(name, &raw, &successor);",
     "        assert_eq!(output.successor(), Some(&successor));\n        let mut successor = successor;\n"
     "        if let Some(row) = successor[\"results\"].as_array_mut().unwrap().iter_mut().find(|r| r[\"id\"] == \"result:disp:N2:ux\") { row[\"value\"] = json!(5e-324); }\n"
     "        let text = u8_l0_document(name, &raw, &successor);"),
    # D-U6-5: one byte of the sparse L = 0 fixture changed.
    ("M6_l0_fixture_byte", FIX,
     '"id": "u8_l0_isolated_node_sparse_interactive"',
     '"id": "u8_l0_isolated_node_sparse_interactivX"'),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=TD)
ENV.pop("RUSTFLAGS", None); ENV.pop("CARGO_ENCODED_RUSTFLAGS", None)
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
results = []
for mid, path, old, new in MUTANTS:
    if only and mid not in only: continue
    pristine = open(path, "rb").read()
    text = pristine.decode()
    assert text.count(old) == 1, (mid, text.count(old))
    open(path, "wb").write(text.replace(old, new).encode())
    log = os.path.join(LOG, f"mutant_{mid}.log")
    t0 = time.time()
    with open(log, "w") as fh:
        rc = subprocess.run([CARGO, "test", "--locked", "--offline", "--lib", "--no-fail-fast"], cwd=PP, env=ENV, stdout=fh, stderr=subprocess.STDOUT).returncode
    open(path, "wb").write(pristine)
    assert open(path, "rb").read() == pristine
    out = open(log, encoding="utf-8", errors="replace").read()
    compiled = "Running unittests" in out
    failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED$", out, re.M)))
    panics = re.findall(r"^thread '([^']+)' \(\d+\) panicked at ([^\n]+):\n([^\n]*)", out, re.M)
    summary = re.findall(r"^test result: .*$", out, re.M)
    results.append({"id": mid, "file": os.path.relpath(path, M), "rc": rc, "compiled": compiled,
                    "killed": compiled and bool(failed), "failed_tests": failed,
                    "panics": [{"test": t, "at": a, "message": m} for t, a, m in panics],
                    "summary": summary, "seconds": round(time.time() - t0, 1), "restored_sha256": sha(path)})
    print(json.dumps(results[-1]), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants.json"), "w"), indent=1)
