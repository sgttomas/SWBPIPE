#!/usr/bin/env python3
"""RV93 addendum 01: mutants for D-U6-5 on the final memory head (7f07a2f7b4). One edit each in
RV93's disposable copy; PP --lib in the given build; pristine bytes restored and checked.
Usage: d_u6_5_mutants.py <copy projects/chirality-piping> <log dir> <target dir> <tag> [RUSTFLAGS] [ids...]"""
import hashlib, json, os, subprocess, sys, time
W, LOG, TD, TAG = sys.argv[1:5]
rustflags = sys.argv[5] if len(sys.argv) > 5 and sys.argv[5].startswith("--") else None
only = set(sys.argv[6:] if rustflags else sys.argv[5:])
PP = os.path.join(W, "core/product_physics")
FX = os.path.join(W, "fixtures/results")
SP = os.path.join(FX, "retained_precision_milestone_successor_sparse_interactive.json")
DS = os.path.join(FX, "retained_precision_milestone_successor_dense_scrutiny.json")
LIB = os.path.join(PP, "src/lib.rs")
TST = os.path.join(PP, "src/retained_facade_tests.rs")
MUT = [
    ("D1_dense_carrier_trailing_newline", DS, None, "\n"),          # append a newline
    ("D2_sparse_carrier_one_digit", SP, "1.0900377190420868e-106", "1.0900377190420867e-106"),
    ("D3_carriers_swapped_in_test", TST,
     '        include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),\n        include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"),\n    ];\n    for ((mode, (name, file_sha, receipt_sha)), carrier)',
     '        include_str!("../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"),\n        include_str!("../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json"),\n    ];\n    for ((mode, (name, file_sha, receipt_sha)), carrier)'),
    ("D4_published_successor_loses_a_diagnostic_after_precommit", LIB,
     "    (frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))",
     "    if let Some(d) = successor[\"diagnostics\"].as_array_mut() { d.pop(); }\n    (frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))"),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2"); ENV.pop("RUSTFLAGS", None)
if rustflags: ENV["RUSTFLAGS"] = rustflags
res = []
for mid, target, old, new in MUT:
    if only and mid not in only: continue
    pristine = open(target, "rb").read(); psha = hashlib.sha256(pristine).hexdigest(); text = pristine.decode()
    if old is None: mutated = text + new
    else:
        assert text.count(old) == 1, (mid, text.count(old)); mutated = text.replace(old, new)
    open(target, "w").write(mutated)
    t0 = time.time()
    try:
        if subprocess.run(["ps", "-p", "5387"], capture_output=True).returncode != 0: sys.exit("MEMGUARD NOT RUNNING")
        proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1500", "cargo", "test", "--locked", "--offline", "--no-fail-fast", "--lib", "--target-dir", TD],
                              cwd=PP, env=ENV, capture_output=True, text=True)
    finally:
        open(target, "wb").write(pristine); assert hashlib.sha256(open(target, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"dmut_{TAG}_{mid}.log"), "w").write(out)
    failing = sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
    failing = [f for f in failing if f != "s11g_tests::t13_committed_fallback_uz_is_byte_identical"]
    ce = "error[" in out or "could not compile" in out
    r = {"id": mid, "build": TAG, "killed": bool(failing) or ce, "compile_error": ce, "killed_by_d_u6_5": any("d_u6_5" in f for f in failing),
         "failing": [f.split("::")[-1] for f in failing], "seconds": round(time.time() - t0, 1)}
    res.append(r); print(json.dumps(r), flush=True)
json.dump(res, open(os.path.join(LOG, f"d_u6_5_mutants_{TAG}.json"), "w"), indent=1)
