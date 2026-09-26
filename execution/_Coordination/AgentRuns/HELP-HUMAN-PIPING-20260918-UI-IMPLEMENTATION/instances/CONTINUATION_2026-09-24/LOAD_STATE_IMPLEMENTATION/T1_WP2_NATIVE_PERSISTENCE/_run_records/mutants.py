"""Mutation run for the T1 WP2 native acceptance (scratch copy only).

Usage: python mutants.py <repo_root> <scratch_dir>
Builds a partial scratch copy (the module plus the one fixture it includes,
at the same relative paths), applies each mutant to the copy, points the
harness #[path] at the copy and runs `cargo test`. A mutant is killed when
the test run fails. The repository file is never modified.
"""
import os, shutil, subprocess, sys

REPO, SCRATCH = sys.argv[1], sys.argv[2]
REL_MOD = "projects/chirality-piping/apps/desktop/src-tauri/src/model_document_migration.rs"
REL_FIX = "projects/chirality-piping/fixtures/product_preview/load_reference/connected.request.json"
COPY = os.path.join(SCRATCH, "mutant_copy")
HARNESS = os.path.join(SCRATCH, "mutant_harness")
TARGET = os.environ["CARGO_TARGET_DIR"]

BRANCH_OPEN = "    if document_version == parse_semver(LOAD_REFERENCE_MODEL_SCHEMA_VERSION).unwrap() {\n"
RET_NONE_LR = """        current.target_schema_version = LOAD_REFERENCE_MODEL_SCHEMA_VERSION.to_string();
        return EvaluatedModelDocument {
            migrated_document: None,
            status: current,
        };"""
NOOP = "        apply: |document| Ok(document.clone()),\n    }]"
EXPLICIT_03 = """        current.target_schema_version = EXPLICIT_PRESSURE_MODEL_SCHEMA_VERSION.to_string();
        return EvaluatedModelDocument {
            migrated_document: None,
            status: current,
        };"""

def remove_branch(src):
    start = src.index(BRANCH_OPEN)
    end = src.index("    if document_version == supported {", start)
    return src[:start] + src[end:]

MUTANTS = {
    "M1_remove_0_4_0_acceptance": remove_branch,
    "M2_retention_drop_unmodelled_record": lambda s: s.replace(RET_NONE_LR, RET_NONE_LR.replace(
        "migrated_document: None,",
        "migrated_document: Some({ let mut d = document.clone(); if let Some(o) = d.as_object_mut() { o.remove(\"reference_configurations\"); } d }),")),
    "M3_retention_down_migrate_to_0_3_0": lambda s: s.replace(RET_NONE_LR, """        current.target_schema_version = EXPLICIT_PRESSURE_MODEL_SCHEMA_VERSION.to_string();
        current.status = "migrated".to_string();
        return EvaluatedModelDocument {
            migrated_document: Some({ let mut d = document.clone(); d["schema_version"] = json!(EXPLICIT_PRESSURE_MODEL_SCHEMA_VERSION); d }),
            status: current,
        };"""),
    "M4_inject_default_into_0_4_0": lambda s: s.replace(RET_NONE_LR, RET_NONE_LR.replace(
        "migrated_document: None,",
        "migrated_document: if document.get(\"reference_configurations\").is_none() { let mut d = document.clone(); d[\"reference_configurations\"] = json!([]); Some(d) } else { None },")),
    "M5_inject_0_4_0_key_into_pre_0_4_migration": lambda s: s.replace(NOOP,
        "        apply: |document| { let mut d = document.clone(); d[\"reference_configurations\"] = json!([]); Ok(d) },\n    }]"),
    "M6_normalize_carried_keys_in_pre_0_4_migration": lambda s: s.replace(NOOP,
        "        apply: |document| { let mut d = document.clone(); if let Some(o) = d.as_object_mut() { o.remove(\"reference_configurations\"); } Ok(d) },\n    }]"),
    "M7_upgrade_0_3_0_carrying_keys_to_0_4_0": lambda s: s.replace(EXPLICIT_03, """        current.target_schema_version = EXPLICIT_PRESSURE_MODEL_SCHEMA_VERSION.to_string();
        if document.get("reference_configurations").is_some() {
            current.target_schema_version = LOAD_REFERENCE_MODEL_SCHEMA_VERSION.to_string();
            current.status = "migrated".to_string();
            return EvaluatedModelDocument {
                migrated_document: Some({ let mut d = document.clone(); d["schema_version"] = json!(LOAD_REFERENCE_MODEL_SCHEMA_VERSION); d }),
                status: current,
            };
        }
        return EvaluatedModelDocument {
            migrated_document: None,
            status: current,
        };"""),
    "M8_accept_any_0_4_x": lambda s: s.replace(BRANCH_OPEN,
        "    if document_version.0 == 0 && document_version.1 == 4 {\n"),
}

def run(name, mutate):
    shutil.rmtree(COPY, ignore_errors=True)
    for rel in (REL_MOD, REL_FIX):
        os.makedirs(os.path.dirname(os.path.join(COPY, rel)), exist_ok=True)
        shutil.copyfile(os.path.join(REPO, rel), os.path.join(COPY, rel))
    path = os.path.join(COPY, REL_MOD)
    src = open(path).read()
    if mutate is not None:
        mutated = mutate(src)
        assert mutated != src, f"{name}: mutation did not apply"
        open(path, "w").write(mutated)
    os.makedirs(os.path.join(HARNESS, "src"), exist_ok=True)
    shutil.copyfile(os.path.join(SCRATCH, "harness", "Cargo.toml"), os.path.join(HARNESS, "Cargo.toml"))
    shutil.copyfile(os.path.join(SCRATCH, "harness", "Cargo.lock"), os.path.join(HARNESS, "Cargo.lock"))
    open(os.path.join(HARNESS, "src", "lib.rs"), "w").write(
        f'#[path = "{path}"]\npub mod model_document_migration;\n')
    proc = subprocess.run(["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2"],
                          cwd=HARNESS, capture_output=True, text=True)
    out = proc.stdout + proc.stderr
    summary = [l for l in out.splitlines() if l.startswith("test result:") or "FAILED" in l or l.startswith("error")]
    return proc.returncode, summary

rc, summary = run("baseline", None)
print("baseline", "PASS" if rc == 0 else "FAIL", summary[:1])
assert rc == 0, "baseline must pass"
survivors = []
for name, mutate in MUTANTS.items():
    rc, summary = run(name, mutate)
    status = "KILLED" if rc != 0 else "SURVIVED"
    if rc == 0:
        survivors.append(name)
    failed = [l.strip() for l in summary if l.strip().startswith("test ") and "FAILED" in l]
    print(name, status, "; ".join(failed) or "; ".join(summary[:2]))
shutil.rmtree(COPY, ignore_errors=True)
shutil.rmtree(HARNESS, ignore_errors=True)
print("survivors:", survivors)
sys.exit(1 if survivors else 0)
