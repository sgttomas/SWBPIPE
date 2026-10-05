#!/usr/bin/env python3
"""I66 U6 repairs: mutants of this round's repairs and of RV88's surviving
mutants (scratch lane WT/scratch/i66_u6_postf/mut only). Each mutant replaces
one exact, unique snippet (or edits one schema), runs the owning suite, and
restores the file. A kill is a failing test only; a compile or syntax error is
reported, never counted. Args: label [ids...]."""
import json, py_compile, subprocess, sys, time
WT = "WT"
S = f"{WT}/scratch/i66_u6_postf"
CAND = f"{WT}/f2a-carriers/projects/chirality-piping"
MUT = f"{S}/mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
TARGET = f"{WT}/targets/i66-u6r-mut"
RUNNER, PP = "core/runner/headless/src/lib.rs", "core/product_physics/src/lib.rs"
CO, RE = "core/analysis_runs/compatibility.py", "core/analysis_runs/records.py"
SC, DV = "core/reporting/result_export/src/semantic_contract.rs", "core/reporting/result_export/src/derivative.rs"
TAURI = "apps/desktop/src-tauri/src/lib.rs"
RES, SN = "schemas/results.v0.3.schema.yaml", "schemas/stress_neutral_export.v0.3.schema.json"
PY_TESTS = "tests/test_retained_precision_carriers.py tests/test_analysis_run_compatibility.py"
SCHEMA_TESTS = "tests/test_retained_precision_schema.py"
SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"


def res_successor(d):
    return next(b for b in d["$defs"]["ResultEnvelope"]["oneOf"] if b["properties"]["producer"]["properties"]["semantic_contract_id"].get("const") == SUCC)


def sn_successor(d):
    return next(b for b in d["oneOf"] if b["properties"]["producer"]["properties"]["semantic_contract_id"].get("const") == SUCC)


def token_in_values(d):
    for name in ("QuantityResult", "PhysicsSourceQuantityResult", "PreviewPhysicsQuantityResult"):
        d["$defs"][name]["properties"]["recovery_method"] = {"type": "string"}


# (id, lang, file, old, new). G4 is a control: a seam named inside a test module
# is not a product caller, so the guard must still pass (expected SURVIVED).
M = [
    ("S1a_ne_comparison", "py", CO, "        if \"retained_precision\" not in run or not _same_canonical(run[\"retained_precision\"], source[\"retained_precision\"]):", "        if run.get(\"retained_precision\") != source[\"retained_precision\"]:"),
    ("S1b_sorted_json_text", "py", CO, "        return canonical_json_checked_v1(a) == canonical_json_checked_v1(b)", "        return json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True)"),
    ("S1c_unsafe_copy_equals", "py", CO, "    except ValueError:\n        return False", "    except ValueError:\n        return True"),
    ("S1d_missing_copy_unguarded", "py", CO, "        if \"retained_precision\" not in run or not _same_canonical(", "        if not _same_canonical("),
    ("G1_seam_named_deep_in_tauri", "rs", TAURI, "fn solver_result_row_value(envelope: &Value, result_id: &str) -> Result<Option<(f64, String)>, &'static str> {\n", "fn solver_result_row_value(envelope: &Value, result_id: &str) -> Result<Option<(f64, String)>, &'static str> {\n    // open_pipe_stress_result_export::semantic_contract::retained_standing_from(\n"),
    ("G2_seam_named_deep_in_runner", "rs", RUNNER, "fn invented_provenance() -> Provenance {\n", "fn invented_provenance() -> Provenance {\n    // semantic_contract::classification_summary_from\n"),
    ("G3_seam_alias_import_deep_in_pp", "rs", PP, "pub fn nonlinear_assembled_loop_context() -> NonlinearAssembledLoopContext {\n", "use open_pipe_stress_result_export::derivative::class_disclosure as cd;\npub fn nonlinear_assembled_loop_context() -> NonlinearAssembledLoopContext {\n"),
    ("N3a_seam_called_in_derivative", "rs", DV, "fn not_covered_message(kind: &str) -> String {\n", "fn not_covered_message(kind: &str) -> String {\n    let _ = crate::semantic_contract::class_binding_refusal(&AccuracyClass::NonQuantity);\n"),
    ("N3b_seam_named_atop_tauri", "rs", TAURI, "mod atomic_report_package_save;\n", "// s::retained_standing_from(\nmod atomic_report_package_save;\n"),
    ("G4_control_seam_in_test_module", "rs", TAURI, "#[cfg(test)]\nmod tests {\n", "#[cfg(test)]\nmod tests {\n    // retained_standing_from(class_binding_refusal)\n"),
]


def guard():
    if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
        print("MEMGUARD NOT RUNNING"); sys.exit(9)


def run_py(tests):
    return subprocess.run(f"cd {MUT} && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
                          f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1200 "
                          f"{VENV}/bin/python -m pytest -q -p no:cacheprovider -x {tests}", shell=True, capture_output=True, text=True)


def run_rs():
    return subprocess.run(f"cd {MUT}/core/reporting/result_export && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 "
                          f"cargo test --locked --offline --manifest-path {MUT}/core/reporting/result_export/Cargo.toml --target-dir {TARGET} --test retained_precision_carriers",
                          shell=True, capture_output=True, text=True)


def main(label, only):
    out = []
    for mid, lang, name, *rest in M:
        if only and mid not in only:
            continue
        guard()
        raw = open(f"{CAND}/{name}").read()
        target = f"{MUT}/{name}"
        if lang == "schema":
            data = json.loads(raw)
            before = json.dumps(data, sort_keys=True)
            rest[0](data)
            assert json.dumps(data, sort_keys=True) != before, mid
            mutated = json.dumps(data, indent=2) + "\n"
        else:
            old, new = rest
            if raw.count(old) != 1:
                out.append({"id": mid, "status": "NOT_APPLIED", "count": raw.count(old)}); print(json.dumps(out[-1]), flush=True); continue
            mutated = raw.replace(old, new)
        open(target, "w").write(mutated)
        t = time.time()
        try:
            if lang == "py":
                try:
                    py_compile.compile(target, cfile=f"{S}/mut_compile.pyc", doraise=True)
                except py_compile.PyCompileError as error:
                    out.append({"id": mid, "status": "SYNTAX_ERROR", "detail": str(error)[:200]}); print(json.dumps(out[-1]), flush=True); continue
            r = run_rs() if lang == "rs" else run_py(SCHEMA_TESTS if lang == "schema" else PY_TESTS)
        finally:
            open(target, "w").write(raw)
        text = r.stdout + r.stderr
        if lang == "rs":
            failed = [line.split(" ")[1] for line in text.splitlines() if line.startswith("test ") and line.endswith("FAILED")]
            compile_error = "could not compile" in text or "error[E" in text
            status = "COMPILE_ERROR" if compile_error else "KILLED" if failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"
        else:
            failed = [line.split(" ")[1] for line in r.stdout.splitlines() if line.startswith("FAILED ")]
            status = "KILLED" if r.returncode == 1 and failed else "SURVIVED" if r.returncode == 0 else f"ERROR_{r.returncode}"
        tail = [line for line in text.splitlines() if "test result" in line or " passed" in line or " failed" in line][-1:]
        out.append({"id": mid, "lang": lang, "file": name, "status": status, "killed_by": failed[:3], "summary": tail, "seconds": round(time.time() - t, 1)})
        print(json.dumps(out[-1]), flush=True)
    for _, _, name, *_ in M:
        assert open(f"{CAND}/{name}").read() == open(f"{MUT}/{name}").read(), name
    json.dump(out, open(f"{S}/logs/mutants_{label}.json", "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv[1], set(sys.argv[2:]))
