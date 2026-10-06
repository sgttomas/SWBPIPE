"""I76 T6S mutants (scratch only): each mutant edits one file in the mutation
copy WT/scratch/i76_t6s/mut (base c1bfc460fc plus I76's files), runs the named
check, records the outcome, and restores the file byte for byte.
Usage: mutants.py <id>... (or 'all')."""
import hashlib, json, os, subprocess, sys, time
from pathlib import Path

WT = Path("WT")
S = WT / "scratch/i76_t6s"
M = S / "mut/projects/chirality-piping"
RE = M / "core/reporting/result_export"
LOGS = S / "logs/mutants"
LOGS.mkdir(parents=True, exist_ok=True)

def cargo(*args):
    env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=str(WT / "targets/i76-t6s-mut"))
    return lambda log, name: subprocess.run([str(WT / "tools/t3_cargo.sh"), "test", "--locked", "--offline", *args],
                                            cwd=RE, env=env, stdout=log, stderr=subprocess.STDOUT).returncode

def pytest(*tests):
    return lambda log, name: subprocess.run([str(S / "run_py2.sh"), f"mut_{name}", str(M), *tests],
                                            stdout=log, stderr=subprocess.STDOUT).returncode

def tests_id(tests):
    return hashlib.sha256(" ".join(tests).encode()).hexdigest()[:8]

def replace(path, old, new, count=1):
    def apply(text):
        assert text.count(old) >= 1, (path, old)
        return text.replace(old, new, count)
    return path, apply

def json_edit(path, edit):
    def apply(text):
        value = json.loads(text)
        edit(value)
        return json.dumps(value, indent=2) + "\n"
    return path, apply

GOLDEN = cargo("--test", "retained_precision_derivative_golden")
SOURCE_BLOCKS = cargo("--test", "source_blocks")
DISPATCH = pytest("tests/test_results_dispatcher_v0_3.py", "tests/test_result_export_v0_2.py")
D = "src/derivative.rs"
MAX = 9007199254740991

def drop_charged_maximum(schema):
    del schema["$defs"]["work"]["properties"]["charged"]["maximum"]

def dispatcher_arm(value):
    def edit(schema):
        schema["oneOf"][2] = value
    return edit

BASE_DISPATCHER = (S / "base/projects/chirality-piping/schemas/results.schema.yaml").read_text()

MUTANTS = {
    # The golden test (T6S-2).
    "G1_absolute_message_text": (replace(RE / D, 'withheld from rule binding and reliance",\n                    f64::from_bits', 'withheld from rule binding",\n                    f64::from_bits'), GOLDEN, "killed"),
    "G2_receipt_not_copied": (replace(RE / D, 'e["retained_precision"] = source["retained_precision"].clone();', 'let _ = &source["retained_precision"];'), GOLDEN, "killed"),
    "G3_bound_format": (replace(RE / D, "b = {:e} {si}", "b = {:.6e} {si}"), GOLDEN, "killed"),
    "G4_contract_evidence_not_copied": (replace(RE / D, "| crate::semantic_contract::PREVIEW_PHYSICS_ID\n                    | PREVIEW_PHYSICS_RETAINED_ID\n            )\n        ) {\n            e[\"contract_evidence\"]", "| crate::semantic_contract::PREVIEW_PHYSICS_ID\n            )\n        ) {\n            e[\"contract_evidence\"]"), GOLDEN, "killed"),
    "G5_absolute_rows_not_disclosed": (replace(RE / D, "AccuracyClass::AbsoluteVerified { bound_bits } => Some(", "AccuracyClass::AbsoluteVerified { bound_bits } if false => Some("), GOLDEN, "killed"),
    "G6_golden_byte": (replace(M / "fixtures/results/retained_precision_successor_derivative_sparse_interactive.json", '"schema_version":"0.3.0"', '"schema_version":"0.3.1"'), GOLDEN, "killed"),
    "G7_pinned_successor_byte": (replace(M / "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json", "protected criteria are absent", "protected criteria are missing"), GOLDEN, "killed"),
    # RV95 N-5: S1 must survive (equivalent at the public API); N1 and N2 remove a masking layer.
    "S1_integer_bound_removed": (replace(RE / "src/source_blocks.rs", "        .filter(|n| *n <= 9_007_199_254_740_991)\n", ""), SOURCE_BLOCKS, "survives"),
    "S1_full_suite": (replace(RE / "src/source_blocks.rs", "        .filter(|n| *n <= 9_007_199_254_740_991)\n", ""), cargo("--no-fail-fast"), "survives"),
    "N1_receipt_shape_maximum_removed": (json_edit(M / "schemas/source_block_recovery.schema.json", drop_charged_maximum), SOURCE_BLOCKS, "killed"),
    "N2_checked_profile_bound_removed": (replace(M / "core/serialization/canonical_json/src/lib.rs", "if !(-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&v) {", "if false && !(-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&v) {"), SOURCE_BLOCKS, "killed"),
    # The dispatcher (T6S-1).
    "D1_ref_to_v0_2": (json_edit(M / "schemas/results.schema.yaml", dispatcher_arm({"$ref": "results.v0.2.schema.yaml"})), DISPATCH, "killed"),
    "D2_arm_dropped": (json_edit(M / "schemas/results.schema.yaml", dispatcher_arm({"not": {}})), DISPATCH, "killed"),
    "D3_base_inline_arm": ((M / "schemas/results.schema.yaml", lambda text: BASE_DISPATCHER), DISPATCH, "killed"),
}

def run(name):
    (path, apply), check, expected = MUTANTS[name]
    original = path.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    mutated = apply(original.decode())
    assert mutated.encode() != original, name
    path.write_text(mutated)
    started = time.time()
    try:
        with (LOGS / f"{name}.log").open("w") as log:
            code = check(log, name)
    finally:
        path.write_bytes(original)
        assert hashlib.sha256(path.read_bytes()).hexdigest() == digest, f"{name}: restore failed"
    outcome = "survives" if code == 0 else "killed"
    record = {"mutant": name, "file": str(path.relative_to(M)), "exit": code, "outcome": outcome,
              "expected": expected, "as_expected": outcome == expected, "seconds": round(time.time() - started, 1)}
    with (LOGS / "summary.jsonl").open("a") as out:
        out.write(json.dumps(record) + "\n")
    print(json.dumps(record), flush=True)

if __name__ == "__main__":
    names = list(MUTANTS) if sys.argv[1:] == ["all"] else sys.argv[1:]
    for name in names:
        run(name)
