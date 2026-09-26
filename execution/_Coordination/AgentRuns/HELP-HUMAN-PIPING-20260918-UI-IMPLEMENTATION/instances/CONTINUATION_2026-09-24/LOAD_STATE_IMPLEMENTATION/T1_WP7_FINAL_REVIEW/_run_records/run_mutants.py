#!/usr/bin/env python3
"""WP7 reviewer mutants on core/product_physics/src/lib.rs (scratch archive only).

Usage: run_mutants.py <WORKING_ROOT> <LOG_DIR>
Each mutant is one exact replacement; the source is restored and checked by
sha256 after every run. Tests: the in-crate load-state test modules and the
load_reference_state_runtime integration target, with the shared target and
CARGO_INCREMENTAL=0 supplied by the caller's environment.
"""
import hashlib, pathlib, subprocess, sys

root = pathlib.Path(sys.argv[1]); logs = pathlib.Path(sys.argv[2]); logs.mkdir(parents=True, exist_ok=True)
crate = root / "core/product_physics"; src = crate / "src/lib.rs"
MUTANTS = {
    "M-SF1-no-fallback": (
        "        source_budget.load_state_join_failure.clone(),\n    ) {",
        "        None::<String>,\n    ) {"),
    "M-PRESCRIBED-writeback-removed": (
        "        for &(dof, value) in &prescribed {\n            displacements[dof] = value;\n        }",
        "        for &(_dof, _value) in &prescribed {\n        }"),
    "M-OBSERVATION-uncoupled": (
        "    let observation_force = if load_state.is_some() {",
        "    let observation_force = if false && load_state.is_some() {"),
}
TESTS = [
    ["cargo", "test", "--locked", "--offline", "-j", "2", "--lib", "--", "load_state", "case_state"],
    ["cargo", "test", "--locked", "--offline", "-j", "2", "--test", "load_reference_state_runtime"],
]
original = src.read_bytes(); digest = hashlib.sha256(original).hexdigest()
results = {}
for name, (old, new) in MUTANTS.items():
    text = original.decode()
    assert text.count(old) == 1, name
    src.write_text(text.replace(old, new, 1))
    killed = False
    with open(logs / f"{name}.log", "w") as log:
        for cmd in TESTS:
            log.write(f"# {' '.join(cmd)}\n"); log.flush()
            rc = subprocess.run(cmd, cwd=crate, stdout=log, stderr=subprocess.STDOUT).returncode
            log.write(f"# exit {rc}\n"); log.flush()
            if rc != 0:
                killed = True
                break
    src.write_bytes(original)
    assert hashlib.sha256(src.read_bytes()).hexdigest() == digest
    results[name] = "killed" if killed else "SURVIVED"
    print(name, results[name], flush=True)
print("restored sha256", digest)
