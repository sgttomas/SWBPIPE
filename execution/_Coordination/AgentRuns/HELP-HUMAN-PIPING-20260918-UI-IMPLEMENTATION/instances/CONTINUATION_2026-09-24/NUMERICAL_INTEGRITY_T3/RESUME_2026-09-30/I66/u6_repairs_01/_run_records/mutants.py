#!/usr/bin/env python3
"""I66 U6 repairs: mutants of this round's repairs and of RV88's surviving
mutants (scratch lane WT/scratch/i66_u6_repairs/mut only). Each mutant replaces
one exact, unique snippet (or edits one schema), runs the owning suite, and
restores the file. A kill is a failing test only; a compile or syntax error is
reported, never counted. Args: label [ids...]."""
import json, py_compile, subprocess, sys, time
WT = "WT"
S = f"{WT}/scratch/i66_u6_repairs"
CAND = f"{WT}/f2a-carriers/projects/chirality-piping"
MUT = f"{S}/mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
TARGET = f"{WT}/targets/i66-u6r-mut"
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


# (id, lang, file, old, new) for text mutants; (id, "schema", file, fn) for schema mutants.
M = [
    # U6b S-1 and RV88 U6b N-3 (Python).
    ("P01_legacy_branch_skips_token_guard", "py", CO, "        if check_receipt and _has_retained_rows(source):\n            raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)\n        return SEMANTIC_CONTRACT_ID", "        return SEMANTIC_CONTRACT_ID"),
    ("P02_v02_constructor_admits_token_rows", "py", CO, ", \"source_block_recovery\", \"retained_precision\")) or _has_retained_rows(received):", ", \"source_block_recovery\", \"retained_precision\")):"),
    ("P03_wrapper_admits_token_rows", "py", RE, "    if isinstance(mechanics_result, Mapping) and (\"retained_precision\" in mechanics_result or (\n            isinstance(rows, list) and any(", "    if isinstance(mechanics_result, Mapping) and (\"retained_precision\" in mechanics_result or (False and\n            isinstance(rows, list) and any("),
    ("P04_Q02_token_guard_first_row_only", "py", CO, "row.get(\"recovery_method\") == RETAINED_METHOD for row in rows)", "row.get(\"recovery_method\") == RETAINED_METHOD for row in rows[:1])"),
    ("P05_Q06_refs_order_insensitive", "py", CO, "list(requested_basis_refs) != expected", "(len(requested_basis_refs) != len(expected) or any(ref not in expected for ref in requested_basis_refs))"),
    ("P06_wrapper_token_first_row_only", "py", RE, "== \"contribution_preserving_multiprecision_v1\" for row in rows)))", "== \"contribution_preserving_multiprecision_v1\" for row in rows[:1])))"),
    ("P07_legacy_guard_ungated_transport", "py", CO, "        if check_receipt and _has_retained_rows(source):\n            raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)\n        return SEMANTIC_CONTRACT_ID", "        if _has_retained_rows(source):\n            raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)\n        return SEMANTIC_CONTRACT_ID"),
    ("P08_token_guard_on_key_presence", "py", CO, "row.get(\"recovery_method\") == RETAINED_METHOD for row in rows)", "\"recovery_method\" in row for row in rows)"),
    ("P09_seam_named_in_records", "py", RE, "\"\"\"Immutable analysis-run records for DEL-14-02.", "# _retained_standing_from(\n\"\"\"Immutable analysis-run records for DEL-14-02."),
    ("P10_python_transport_admitted", "py", CO, "    if not check_receipt:\n        # The Python", "    if False:\n        # The Python"),
    ("P11_refused_statement_binds", "py", CO, "    except (ValueError, KeyError, TypeError, AttributeError):\n        return RULE_QUANTITY_NOT_COVERED", "    except (ValueError, KeyError, TypeError, AttributeError):\n        return None"),
    ("P12_Q01_member_guard_null_slips", "py", CO, "    if isinstance(source, Mapping) and \"retained_precision\" in source:", "    if isinstance(source, Mapping) and source.get(\"retained_precision\") is not None:"),
    ("P13_Q05_wrapper_null_slips", "py", RE, "(\"retained_precision\" in mechanics_result or (", "(mechanics_result.get(\"retained_precision\") is not None or ("),
    # U6a S-1 (RV88's R01, R02, R05, R06), S-2 and N-3 (Rust).
    ("R01_row_guard_first_row_only", "rs", SC, "            rows.iter()\n                .any(|row| row[\"recovery_method\"] == crate::retained_precision::METHOD)", "            rows.first().into_iter()\n                .any(|row| row[\"recovery_method\"] == crate::retained_precision::METHOD)"),
    ("R02_doc_downgrade_ignores_null", "rs", DV, "    } else if doc[\"result_envelope\"].get(\"retained_precision\").is_some() {", "    } else if doc[\"result_envelope\"].get(\"retained_precision\").is_some_and(|v| !v.is_null()) {"),
    ("R05_refs_order_insensitive", "rs", SC, "        || expected.iter().zip(requested_basis_refs).any(|(a, b)| *a != b)\n", "        || expected.iter().any(|a| !requested_basis_refs.contains(*a))\n"),
    ("R06_member_guard_skips_legacy", "rs", SC, "    if !is_retained(source) && source.get(\"retained_precision\").is_some() {", "    if !is_retained(source) && source[\"schema_version\"] != \"0.1.0\" && source.get(\"retained_precision\").is_some() {"),
    ("S2a_mm_printed_as_row_unit", "rs", DV, "        \"m\" | \"mm\" => Some(\"m\"),", "        \"m\" => Some(\"m\"),\n        \"mm\" => Some(\"mm\"),"),
    ("S2b_MPa_printed_as_row_unit", "rs", DV, "        \"Pa\" | \"MPa\" => Some(\"Pa\"),", "        \"Pa\" => Some(\"Pa\"),\n        \"MPa\" => Some(\"MPa\"),"),
    ("S2c_unit_dropped", "rs", DV, "b = {:e} {si} (binary64", "b = {:e} (binary64"),
    ("S2d_kNm_mapped_to_N", "rs", DV, "        \"N*m\" | \"kN*m\" => Some(\"N*m\"),", "        \"N*m\" => Some(\"N*m\"),\n        \"kN*m\" => Some(\"N\"),"),
    ("S2e_unknown_unit_claims_absolute", "rs", DV, "            None => (RETAINED_NOT_COVERED, not_covered_message(kind)),", "            None => (RETAINED_ABSOLUTE_VERIFIED, not_covered_message(kind)),"),
    ("S2f_validate_assumes_Pa", "rs", DV, "            row[\"unit\"].as_str().ok_or(\"SOURCE_UNIT_MISSING\")?,\n            classes.as_ref()", "            \"Pa\",\n            classes.as_ref()"),
    ("S2g_rad_unknown", "rs", DV, "        \"rad\" => Some(\"rad\"),\n", ""),
    ("N3a_seam_called_in_derivative", "rs", DV, "fn not_covered_message(kind: &str) -> String {\n", "fn not_covered_message(kind: &str) -> String {\n    let _ = crate::semantic_contract::class_binding_refusal(&AccuracyClass::NonQuantity);\n"),
    ("N3b_seam_named_in_tauri", "rs", TAURI, "mod atomic_report_package_save;\n", "// s::retained_standing_from(\nmod atomic_report_package_save;\n"),
    # U6c N-1 and N-4 (schemas).
    ("Y4_results_successor_admits_sbr", "schema", RES, lambda d: res_successor(d).pop("not")),
    ("Y11_value_rows_admit_token", "schema", RES, token_in_values),
    ("W06_sn_successor_annotations_optional", "schema", SN, lambda d: sn_successor(d)["required"].remove("source_annotations")),
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
