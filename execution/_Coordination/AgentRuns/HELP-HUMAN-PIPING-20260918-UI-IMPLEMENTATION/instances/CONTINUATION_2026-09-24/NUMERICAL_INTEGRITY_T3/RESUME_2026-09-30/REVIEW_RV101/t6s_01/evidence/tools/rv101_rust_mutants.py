"""RV101 Rust/Python mutants (scratch): I76's G/S/N/D mutants rerun, plus RV101's own.
Each edits one file in the mutation copy WT/rv101/mutr, runs the named check, and restores the
file byte for byte (sha256-checked). Cargo only through WT/tools/t3_cargo.sh.
Usage: rv101_rust_mutants.py <id>... | all"""
import hashlib, json, os, subprocess, sys, time
from pathlib import Path

WT = Path("WT")
S = WT / "scratch/rv101_t6s_01"
M = WT / "rv101/mutr/projects/chirality-piping"
RE = M / "core/reporting/result_export"
LOGS = S / "out/mut_rust"
LOGS.mkdir(parents=True, exist_ok=True)
VENV = Path("VENV")

def cargo(*args):
    env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=str(WT / "targets/rv101/mutr"), TMPDIR=str(S / "tmp"))
    return lambda log: subprocess.run([str(WT / "tools/t3_cargo.sh"), "test", "--locked", "--offline", *args], cwd=RE, env=env, stdout=log, stderr=subprocess.STDOUT).returncode

def pytest(*tests):
    env = dict(os.environ, OPENPIPESTRESS_CHECKED_JSON_BIN="/nonexistent/rv101-dummy-checked-json", OPENPIPESTRESS_UNITS_BIN="/nonexistent/rv101-dummy-units", PYTHONDONTWRITEBYTECODE="1", TMPDIR=str(S / "tmp"))
    return lambda log: subprocess.run([str(VENV / "bin/python"), "-m", "pytest", "-q", "-p", "no:cacheprovider", "--basetemp", str(S / "tmp/pytest_mut"), *tests], cwd=M, env=env, stdout=log, stderr=subprocess.STDOUT).returncode

def replace(path, old, new):
    def apply(text):
        assert text.count(old) == 1, (path, old, text.count(old))
        return text.replace(old, new)
    return path, apply

def json_edit(path, edit):
    def apply(text):
        value = json.loads(text); edit(value); return json.dumps(value, indent=2) + "\n"
    return path, apply

GOLDEN = cargo("--test", "retained_precision_derivative_golden")
SOURCE_BLOCKS = cargo("--test", "source_blocks")
DISPATCH = pytest("tests/test_results_dispatcher_v0_3.py", "tests/test_result_export_v0_2.py")
D = RE / "src/derivative.rs"
BASE_DISPATCHER = (WT / "rv101/base/projects/chirality-piping/schemas/results.schema.yaml").read_text()

def drop_charged_maximum(schema):
    del schema["$defs"]["work"]["properties"]["charged"]["maximum"]

def arm(value):
    def edit(schema):
        schema["oneOf"][2] = value
    return edit

MUTANTS = {
    # I76's, rerun.
    "G1_absolute_message_text": (replace(D, 'withheld from rule binding and reliance",\n                    f64::from_bits', 'withheld from rule binding",\n                    f64::from_bits'), GOLDEN, "killed"),
    "G2_receipt_not_copied": (replace(D, 'e["retained_precision"] = source["retained_precision"].clone();', 'let _ = &source["retained_precision"];'), GOLDEN, "killed"),
    "G3_bound_format": (replace(D, "b = {:e} {si}", "b = {:.6e} {si}"), GOLDEN, "killed"),
    "G4_contract_evidence_not_copied": (replace(D, "| crate::semantic_contract::PREVIEW_PHYSICS_ID\n                    | PREVIEW_PHYSICS_RETAINED_ID\n            )\n        ) {\n            e[\"contract_evidence\"]", "| crate::semantic_contract::PREVIEW_PHYSICS_ID\n            )\n        ) {\n            e[\"contract_evidence\"]"), GOLDEN, "killed"),
    "G5_absolute_rows_not_disclosed": (replace(D, "AccuracyClass::AbsoluteVerified { bound_bits } => Some(", "AccuracyClass::AbsoluteVerified { bound_bits } if false => Some("), GOLDEN, "killed"),
    "G6_golden_byte": ((M / "fixtures/results/retained_precision_successor_derivative_sparse_interactive.json", lambda text: text.replace('"schema_version":"0.3.0"', '"schema_version":"0.3.1"', 1)), GOLDEN, "killed"),
    "G7_pinned_successor_byte": (replace(M / "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json", "protected criteria are absent", "protected criteria are missing"), GOLDEN, "killed"),
    "S1_integer_bound_removed": (replace(RE / "src/source_blocks.rs", "        .filter(|n| *n <= 9_007_199_254_740_991)\n", ""), SOURCE_BLOCKS, "survives"),
    "S1_full_suite": (replace(RE / "src/source_blocks.rs", "        .filter(|n| *n <= 9_007_199_254_740_991)\n", ""), cargo("--no-fail-fast"), "survives"),
    "N1_receipt_shape_maximum_removed": (json_edit(M / "schemas/source_block_recovery.schema.json", drop_charged_maximum), SOURCE_BLOCKS, "killed"),
    "N2_checked_profile_bound_removed": (replace(M / "core/serialization/canonical_json/src/lib.rs", "if !(-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&v) {", "if false && !(-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&v) {"), SOURCE_BLOCKS, "killed"),
    "D1_ref_to_v0_2": (json_edit(M / "schemas/results.schema.yaml", arm({"$ref": "results.v0.2.schema.yaml"})), DISPATCH, "killed"),
    "D2_arm_dropped": (json_edit(M / "schemas/results.schema.yaml", arm({"not": {}})), DISPATCH, "killed"),
    "D3_base_inline_arm": ((M / "schemas/results.schema.yaml", lambda text: BASE_DISPATCHER), DISPATCH, "killed"),
    # RV101's own.
    "RV-R1_not_covered_message_text": (replace(D, "no verified accuracy for this quantity kind; source value/unit and annotation retained; withheld from rule binding and reliance", "no verified accuracy for this quantity kind; source value/unit and annotation retained"), GOLDEN, "?"),
    "RV-R1b_not_covered_message_text_carriers": (replace(D, "no verified accuracy for this quantity kind; source value/unit and annotation retained; withheld from rule binding and reliance", "no verified accuracy for this quantity kind; source value/unit and annotation retained"), cargo("--test", "retained_precision_carriers"), "?"),
    "RV-R2_si_unit_mm_kept": (replace(D, '"m" | "mm" => Some("m"),', '"m" => Some("m"),\n        "mm" => Some("mm"),'), GOLDEN, "?"),
    "RV-R3_publication_hash_layer_removed": (replace(RE / "src/source_blocks.rs", '    require!(\n        body["publication_sha256"] == domain_hash("source_blocks_publication_v1", &publication)?,\n        "PUBLICATION_HASH"\n    );', '    let _ = &publication;'), SOURCE_BLOCKS, "?"),
    "RV-R4_relative_rows_disclosed": (replace(D, "        AccuracyClass::NotCovered => Some((RETAINED_NOT_COVERED, not_covered_message(kind))),\n        _ => None,", "        AccuracyClass::NotCovered | AccuracyClass::RelativeVerified => Some((RETAINED_NOT_COVERED, not_covered_message(kind))),\n        _ => None,"), GOLDEN, "?"),
    "RV-D4_arm_admits_any_0_3_0": (json_edit(M / "schemas/results.schema.yaml", arm({"properties": {"schema_version": {"const": "0.3.0"}}, "required": ["schema_version"]})), DISPATCH, "?"),
    "RV-D5_arm_refs_version_file_but_forbids_receipt": (json_edit(M / "schemas/results.schema.yaml", arm({"allOf": [{"$ref": "results.v0.3.schema.yaml"}, {"properties": {"result_envelope": {"not": {"required": ["retained_precision"]}}}}]})), DISPATCH, "?"),
}

def run(name):
    (path, apply), check, expected = MUTANTS[name]
    original = path.read_bytes(); digest = hashlib.sha256(original).hexdigest()
    mutated = apply(original.decode())
    assert mutated.encode() != original, name
    path.write_text(mutated)
    started = time.time()
    try:
        with (LOGS / f"{name}.log").open("w") as log:
            code = check(log)
    finally:
        path.write_bytes(original)
        assert hashlib.sha256(path.read_bytes()).hexdigest() == digest, f"{name}: restore failed"
    outcome = "survives" if code == 0 else "killed"
    record = {"mutant": name, "file": str(path.relative_to(M)), "exit": code, "outcome": outcome, "expected": expected, "seconds": round(time.time() - started, 1)}
    with (LOGS / "summary.jsonl").open("a") as out:
        out.write(json.dumps(record) + "\n")
    print(json.dumps(record), flush=True)

if __name__ == "__main__":
    names = list(MUTANTS) if sys.argv[1:] == ["all"] else sys.argv[1:]
    for name in names:
        run(name)
