#!/usr/bin/env python3
"""I67 U7 repair (u7_repair_01): the Python and Rust mutants for D-U7-4's summary forms (S-1) and
the N-3 clause and probe. Lane WT/scratch/i67_u6d/lanes/pyrs9 (the candidate's P, without apps/).
Each mutant edits the case file or the corpus (parsed, written back in the file's own format),
runs Python's declared-differences, corpus-count and probe tests and Rust's declared-differences
and corpus-mutation tests (include_str!, so they rebuild), then restores. Cargo runs one job at a
time, after the candidate chain's cargo run. Usage: pyrs_mutants_r9.py <out.json>"""
import json, os, subprocess, sys, time
T3 = "WT"
S = f"{T3}/scratch/i67_u6d"
P = f"{S}/lanes/pyrs9/projects/chirality-piping"
CASES = f"{P}/fixtures/results/retained_precision_carrier_cases.json"
CORPUS = f"{P}/fixtures/results/retained_precision_cases.json"
D74 = "D-U7-4:ts_requires_live_native_capture"
PROBE = "g7_not_required_quality_enum_invalid"
N3 = " An invalid enum value in a not_required case's quality is refused at G7"
def d74(doc): return next(e for e in doc["declared_differences"] if e["id"] == D74)
def sumforms(doc): return [f for f in d74(doc)["forms"] if f["subject"] == "summary"]
def drop_sumforms(doc): d74(doc)["forms"] = [f for f in d74(doc)["forms"] if f["subject"] != "summary"]
def side(lang, value):
    def fn(doc):
        for f in sumforms(doc): f["expected"][lang] = {"summary": value}
    return fn
def drop_n3(doc):
    i = doc["scope"].index(N3); j = doc["scope"].index("I67 u7_repair_01).", i) + len("I67 u7_repair_01).")
    doc["scope"] = doc["scope"][:i] + doc["scope"][j:]
def scope_replace(old, new):
    def fn(doc):
        assert doc["scope"].count(old) == 1, old
        doc["scope"] = doc["scope"].replace(old, new)
    return fn
def drop_probe(doc): doc["mutations"] = [m for m in doc["mutations"] if m["id"] != PROBE]
def probe_field(path, code):
    def fn(doc):
        at = next(m for m in doc["mutations"] if m["id"] == PROBE)
        for k in path: at = at[k]
        at["code"] = code
    return fn
M = [
    ("control", None, None),
    ("S01_summary_forms_removed", CASES, drop_sumforms),
    ("S02_ts_side_flipped", CASES, side("typescript", "by_validated_class_current")),
    ("S03_python_side_flipped", CASES, side("python", "by_validated_class")),
    ("S04_rust_side_flipped", CASES, side("rust", "by_validated_class")),
    ("N01_clause_removed", CASES, drop_n3),
    ("N02p_python_code_misstated", CASES, scope_replace("Python SOURCE_NUMERICAL_CASE_INVALID", "Python SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")),
    ("N02r_rust_code_misstated", CASES, scope_replace("Rust SOURCE_NUMERICAL_CASE_INVALID", "Rust SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")),
    ("N03_probe_removed", CORPUS, drop_probe),
    ("N04p_probe_python_expectation_flipped", CORPUS, probe_field(["expected"], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")),
    ("N04r_probe_rust_expectation_flipped", CORPUS, probe_field(["expected_by_reader", "rust"], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")),
]
def py():
    env = dict(os.environ, TMPDIR=f"{S}/tmp", PYTHONDONTWRITEBYTECODE="1",
               OPENPIPESTRESS_CHECKED_JSON_BIN=f"{T3}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson",
               OPENPIPESTRESS_UNITS_BIN=f"{T3}/targets/i52-readers/units/release/openpipestress_units")
    p = subprocess.run(["REPO_ROOT/projects/chirality-piping/.venv/bin/python", "-m", "pytest", "-q", "-p", "no:cacheprovider",
                        "tests/test_retained_precision_carriers.py", "tests/test_retained_precision_contract.py",
                        "-k", f"declared_differences_python or snapshot_07_counts_and_entry_format or {PROBE}"], cwd=P, env=env, capture_output=True, text=True, timeout=1800)
    return {"exit": p.returncode, "tail": (p.stdout.strip().splitlines() or [""])[-1][:200]}
def rs():
    assert subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode == 0, "MEMGUARD NOT RUNNING"
    env = {k: v for k, v in os.environ.items() if k != "RUSTFLAGS"}
    env.update(TMPDIR=f"{S}/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
    p = subprocess.run(["cargo", "test", "--locked", "--offline", "--no-fail-fast", "--manifest-path", f"{P}/core/reporting/result_export/Cargo.toml",
                        "--target-dir", f"{T3}/targets/i67-u7f/re-mut9", "--test", "retained_precision_carriers", "--test", "retained_precision_contract", "--",
                        "u6_declared_differences_rust", "shared_rehashed_first_failure_mutations", "snapshot_07h_mutation_outcomes"],
                       cwd=f"{P}/core/reporting/result_export", env=env, capture_output=True, text=True, timeout=3600)
    lines = [l for l in (p.stdout + p.stderr).splitlines() if (l.startswith("test ") and " ... " in l) or l.startswith("test result")]
    return {"exit": p.returncode, "tests": lines}
out = []
while not os.path.exists(f"{S}/r9/cand.done"): time.sleep(5)
for name, path, fn in M:
    orig = open(path).read() if path else None
    if fn:
        d = json.loads(orig); assert json.dumps(d, indent=2) + "\n" == orig
        fn(d); new = json.dumps(d, indent=2) + "\n"; assert new != orig
        open(path, "w").write(new)
    try:
        r = {"id": name, "python": py(), "rust": rs()}
    finally:
        if path: open(path, "w").write(orig)
    r["python_killed"] = r["python"]["exit"] != 0; r["rust_killed"] = r["rust"]["exit"] != 0
    out.append(r); print(name, "py", "KILLED" if r["python_killed"] else "passes", r["python"]["tail"][:80], "| rs", "KILLED" if r["rust_killed"] else "passes", [t for t in r["rust"]["tests"] if "FAILED" in t][:3], flush=True)
json.dump(out, open(sys.argv[1], "w"), indent=1)
