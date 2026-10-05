#!/usr/bin/env python3
"""I67 U7 slice F part 2: the scope-clause mutants for Python and Rust (their one-line fence
extensions). Lane WT/scratch/i67_u6d/lanes/pyrsmut (the final candidate's P, without apps/).
Each mutant edits the case file's scope, runs Python's declared-differences test and Rust's
retained_precision_carriers test binary (include_str!, so it rebuilds), then restores.
Usage: pyrs_mutants.py <out.json>"""
import json, os, subprocess, sys
T3 = "WT"
S = f"{T3}/scratch/i67_u6d"
P = f"{S}/lanes/pyrsmut/projects/chirality-piping"
CASES = f"{P}/fixtures/results/retained_precision_carrier_cases.json"
CLAUSE = " A statement whose mechanics status is not MECHANICS_SOLVED"
M = [
    ("control", None),
    ("P1_clause_removed", lambda s: s[:s.index(CLAUSE)]),
    ("P2_python_code_misstated", lambda s: s.replace("Python SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "Python SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE")),
    ("P3_rust_code_misstated", lambda s: s.replace("Rust SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE", "Rust SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID")),
    ("P4_d_u7_6_removed", lambda s: s.replace("no carrier authenticates producer origin, and none claims", "none claims")),
]
def py():
    env = dict(os.environ, TMPDIR=f"{S}/tmp", PYTHONDONTWRITEBYTECODE="1",
               OPENPIPESTRESS_CHECKED_JSON_BIN=f"{T3}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson",
               OPENPIPESTRESS_UNITS_BIN=f"{T3}/targets/i52-readers/units/release/openpipestress_units")
    p = subprocess.run(["REPO_ROOT/projects/chirality-piping/.venv/bin/python", "-m", "pytest", "-q", "-p", "no:cacheprovider",
                        "tests/test_retained_precision_carriers.py::test_declared_differences_python"], cwd=P, env=env, capture_output=True, text=True, timeout=1800)
    return {"exit": p.returncode, "tail": p.stdout.strip().splitlines()[-1][:200]}
def rs():
    assert subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode == 0, "MEMGUARD NOT RUNNING"
    env = {k: v for k, v in os.environ.items() if k != "RUSTFLAGS"}
    env.update(TMPDIR=f"{S}/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
    p = subprocess.run(["cargo", "test", "--locked", "--offline", "--no-fail-fast", "--manifest-path", f"{P}/core/reporting/result_export/Cargo.toml",
                        "--target-dir", f"{T3}/targets/i67-u7f/re-mut", "--test", "retained_precision_carriers", "--", "u6_declared_differences_rust"],
                       cwd=f"{P}/core/reporting/result_export", env=env, capture_output=True, text=True, timeout=3600)
    lines = [l for l in (p.stdout + p.stderr).splitlines() if l.startswith("test ") or l.startswith("test result")]
    return {"exit": p.returncode, "tests": lines[:6]}
out = []
orig = open(CASES).read()
doc = json.loads(orig)
for name, fn in M:
    if fn:
        d = json.loads(orig); new = fn(d["scope"]); assert new != d["scope"]; d["scope"] = new
        open(CASES, "w").write(json.dumps(d, indent=2) + "\n")
    try:
        r = {"id": name, "python": py(), "rust": rs()}
    finally:
        open(CASES, "w").write(orig)
    r["python_killed"] = r["python"]["exit"] != 0; r["rust_killed"] = r["rust"]["exit"] != 0
    out.append(r); print(json.dumps(r)[:600], flush=True)
json.dump(out, open(sys.argv[1], "w"), indent=1)
