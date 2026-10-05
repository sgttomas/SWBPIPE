#!/usr/bin/env python3
"""RV95 mutants: one exact edit each in the copy (asserted unique), the named test command,
then restore and verify the pristine hash. A kill is a failing test (exit != 0 with a test
failure in the log), never a compile error; compile errors are reported as such."""
import hashlib, json, os, subprocess, sys, time
T3 = "WT"
S = T3 + "/scratch/rv95_u9_01"; P = T3 + "/rv95/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv/bin/python"
ENV = dict(os.environ, TMPDIR=S + "/tmp", PYTHONDONTWRITEBYTECODE="1", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2",
           OPENPIPESTRESS_CHECKED_JSON_BIN=T3 + "/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson",
           OPENPIPESTRESS_UNITS_BIN=T3 + "/targets/i52-readers/units/release/openpipestress_units")
ENV.pop("RUSTFLAGS", None); ENV.pop("CARGO_ENCODED_RUSTFLAGS", None)
def cargo(manifest_dir, target, *args):
    return (P + "/" + manifest_dir, ["cargo", "test", "--locked", "--offline", "--target-dir", T3 + "/targets/" + target, *args])
def py(*tests):
    return (P, [VENV, "-m", "pytest", "-q", "-p", "no:cacheprovider", "--basetemp=" + S + "/tmp/pytest_mut", "-x", *tests])
def vitest(*files):
    return (P + "/apps/desktop", ["../../node_modules/.bin/vitest", "run", "--maxWorkers=4", *files])
MUTANTS = [
 ("R1", "PP: precommit validation result ignored (W1 publishes an unvalidated successor)", "core/product_physics/src/lib.rs",
  "    if let Err(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)) {",
  "    if let Some(error) = open_pipe_stress_result_export::retained_precision::validate(&successor, Some(&invocation)).err().filter(|_| false) {",
  cargo("core/product_physics", "rv95/pp", "--lib", "u3g2_")),
 ("R2", "PP: D1.3 pressure-contract clause removed from admission", "core/product_physics/src/retained_memory.rs",
  "    if m.pressure_contract.is_some() {\n        return refuse(C::Namespace, F::PressureContract);\n    }\n", "",
  cargo("core/product_physics", "rv95/pp", "--lib", "retained")),
 ("R3", "PP: the N1 notice is not appended after a W1 fallback", "core/product_physics/src/lib.rs",
  "        ordinary.diagnostics.push(notice);\n", "        let _ = notice;\n",
  cargo("core/product_physics", "rv95/pp", "--lib", "u3g2_")),
 ("R4", "Rust reader: G8 invocation binding skipped", "core/reporting/result_export/src/retained_precision.rs",
  "    if let Some(inv) = actual_invocation {\n        g8(source, inv)?;", "    if let Some(inv) = actual_invocation.filter(|_| false) {\n        g8(source, inv)?;",
  cargo("core/reporting/result_export", "rv95/re")),
 ("P1", "Python carrier: main's contract_evidence call site reverted to plain != (PR1078 interplay)", "core/analysis_runs/compatibility.py",
  '        if not _same_canonical(run.get("contract_evidence"), source["contract_evidence"]):',
  '        if run.get("contract_evidence") != source["contract_evidence"]:',
  py("tests/test_analysis_run_compatibility.py")),
 ("P2", "Python carrier: retained_precision copy compared with plain != instead of canonical bytes", "core/analysis_runs/compatibility.py",
  '        if "retained_precision" not in run or not _same_canonical(run["retained_precision"], source["retained_precision"]):',
  '        if "retained_precision" not in run or run["retained_precision"] != source["retained_precision"]:',
  py("tests/test_retained_precision_carriers.py")),
 ("T1", "TS standing: the live native capture conjunct removed (D-U7-4)", "apps/desktop/src/features/results/retainedPrecisionStanding.ts",
  "  if (registration.live?.(model) !== true) return", "  if (false) return",
  vitest("src/features/results", "src/services")),
 ("C1", "CI policy: NUMERICAL_APP_INPUTS emptied", "tools/ci/e2e_plan.py",
  "NUMERICAL_APP_INPUTS = {\n    PROJECT + 'apps/desktop/src-tauri/src/lib.rs',\n}", "NUMERICAL_APP_INPUTS = set()",
  py("tests/test_ci_numerical.py", "tests/test_ci_e2e_plan.py")),
 ("C2", "CI policy pin only: test_ci_e2e_plan alone with NUMERICAL_APP_INPUTS emptied", "tools/ci/e2e_plan.py",
  "NUMERICAL_APP_INPUTS = {\n    PROJECT + 'apps/desktop/src-tauri/src/lib.rs',\n}", "NUMERICAL_APP_INPUTS = set()",
  py("tests/test_ci_e2e_plan.py")),
 ("S1", "source_blocks integer: the 2^53-1 bound removed", "core/reporting/result_export/src/source_blocks.rs",
  "        .filter(|n| *n <= 9_007_199_254_740_991)\n", "",
  cargo("core/reporting/result_export", "rv95/re")),
 ("G1", "U1 pin: the dense ordinary digest altered (the gate must be active on this Mac)", "core/product_physics/src/retained_wire_tests.rs",
  '    ("dense_scrutiny", 69366, "21ca629c27e6ca03b1411c8c51097f7a50f90429045b1e36313163014dd4278a"),',
  '    ("dense_scrutiny", 69366, "21ca629c27e6ca03b1411c8c51097f7a50f90429045b1e36313163014dd4278b"),',
  cargo("core/product_physics", "rv95/pp", "--lib", "u1_ordinary_bytes_unchanged_under_capture")),
 ("G2", "U1 pin gate forced off (the other-target path, run on this Mac: equalities only)", "core/product_physics/src/retained_wire_tests.rs",
  'const ORDINARY_PINNED_TARGET: bool = cfg!(all(target_arch = "aarch64", target_os = "macos"));',
  'const ORDINARY_PINNED_TARGET: bool = false;',
  cargo("core/product_physics", "rv95/pp", "--lib", "u1_ordinary_bytes_unchanged_under_capture")),
]
only = set(sys.argv[1:])
results = []
for mid, what, rel, old, new, (cwd, cmd) in MUTANTS:
    if only and mid not in only: continue
    path = P + "/" + rel
    src = open(path, encoding="utf-8").read(); h0 = hashlib.sha256(src.encode()).hexdigest()
    assert src.count(old) == 1, (mid, src.count(old))
    open(path, "w", encoding="utf-8").write(src.replace(old, new))
    t = time.time()
    try:
        r = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "2400", *cmd], cwd=cwd, env=ENV, capture_output=True, text=True)
        log = r.stdout + r.stderr
    finally:
        open(path, "w", encoding="utf-8").write(src)
        assert hashlib.sha256(open(path, encoding="utf-8").read().encode()).hexdigest() == h0
    compile_error = ("error[E" in log or "could not compile" in log) and "test result" not in log
    failed_tests = [l.strip() for l in log.splitlines() if l.strip().startswith(("FAILED ", "test ")) and ("FAILED" in l)] + \
                   [l.strip() for l in log.splitlines() if l.strip().startswith(("×", "FAIL "))]
    status = "COMPILE_ERROR" if compile_error else ("KILLED" if r.returncode != 0 else "SURVIVED")
    open(f"{S}/logs/mutant_{mid}.log", "w").write(log)
    results.append({"id": mid, "what": what, "file": rel, "status": status, "rc": r.returncode, "seconds": round(time.time() - t),
                    "first_failures": failed_tests[:4]})
    print(json.dumps(results[-1]), flush=True)
json.dump(results, open(f"{S}/evidence/mutants_{'_'.join(sorted(only)) or 'all'}.json", "w"), indent=1)
