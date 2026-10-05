#!/usr/bin/env python3
"""I66 U6a mutants (scratch lane only). Each mutant edits one site of the frozen
candidate in WT/scratch/i66_u6a_slice_01/mut, runs the focused suites, and
restores the file. A mutant is killed only when a test fails; a compile error
is reported separately and never counts as a kill."""
import json, os, re, shutil, subprocess, sys, time
WT = "WT"
S = f"{WT}/scratch/i66_u6a_slice_01"
CAND = f"{WT}/f2a-carriers/projects/chirality-piping"
MUT = f"{S}/mut/projects/chirality-piping"
VENV = "REPO_ROOT/projects/chirality-piping/.venv"
SC = "core/reporting/result_export/src/semantic_contract.rs"
DV = "core/reporting/result_export/src/derivative.rs"
PY = "core/analysis_runs/retained_precision.py"
M = [
 ("S01_meta_branch_off", SC, "    if is_retained(source) {\n        // Transport checks", "    if false {\n        // Transport checks"),
 ("S02_raw_branch_off", SC, "    if is_retained(source) {\n        // The accepted reader, G0-G7", "    if false {\n        // The accepted reader, G0-G7"),
 ("S03_raw_skips_reader", SC, "        crate::retained_precision::validate(source, None).map_err(retained_error)?;\n        return Ok((preview_physics_retained_contract(), \"0.3.0\"));", "        return Ok((preview_physics_retained_contract(), \"0.3.0\"));"),
 ("S04_meta_skips_reader", SC, "        crate::retained_precision::validate_transport_metadata(source).map_err(retained_error)?;\n", ""),
 ("S05_member_guard_dropped", SC, "    forbid_retained_member(source)?;\n", ""),
 ("S06_member_guard_null", SC, "source.get(\"retained_precision\").is_some() {", "source.get(\"retained_precision\").is_some_and(|v| !v.is_null()) {"),
 ("S07_row_guard_dropped", SC, "    forbid_retained_rows(source)?;\n", ""),
 ("S08_row_guard_token", SC, ".any(|row| row[\"recovery_method\"] == crate::retained_precision::METHOD)", ".any(|row| row[\"recovery_method\"] == \"other\")"),
 ("S09_error_text_code_only", SC, "    error.detail.unwrap_or(error.code)", "    error.code"),
 ("S10_fresh_set", SC, "    PREVIEW_PHYSICS_RETAINED_ID,\n];", "];"),
 ("S11_standing_branch_off", SC, "    if is_retained(source) {\n        // Validated once, with the invocation", "    if false {\n        // Validated once, with the invocation"),
 ("S12_standing_err_recompute", SC, "        Err(_) => \"unsupported\",\n    }\n}\n/// D2 4.9.9: per-case", "        Err(_) => \"needs_recompute\",\n    }\n}\n/// D2 4.9.9: per-case"),
 ("S13_standing_ignores_invocation", SC, "    match crate::retained_precision::validate(source, actual_invocation) {", "    match crate::retained_precision::validate(source, None) {"),
 ("S14_unbound_ok", SC, "    if !validation.invocation_bound\n        || !validation.numerical_eligible", "    if !validation.numerical_eligible"),
 ("S15_eligible_ignored", SC, "        || !validation.numerical_eligible\n", "\n"),
 ("S16_len_check", SC, "        || expected.len() != requested_basis_refs.len()\n", "\n"),
 ("S17_ref_equality", SC, "        || expected.iter().zip(requested_basis_refs).any(|(a, b)| *a != b)\n", "\n"),
 ("S18_mechanics", SC, "        || source[\"status\"][\"mechanics\"] != \"MECHANICS_SOLVED\"\n        || !not_required", "        || !not_required"),
 ("S19_not_required_conjunct", SC, "        || !not_required_cases_ordinarily_eligible(source, cases)\n", "\n"),
 ("S20_nr_basis_ref", SC, "        q[\"basis_ref\"] == case[\"basis_ref\"]\n            && q[\"solve_quality\"]", "        q[\"solve_quality\"]"),
 ("S21_nr_solve_quality", SC, "            && q[\"solve_quality\"] == \"checks_passed\"\n", "\n"),
 ("S22_nr_structural", SC, "            && q[\"structural_status\"] == \"passive_model_basis\"\n", "\n"),
 ("S23_nr_fidelity", SC, "            && q[\"model_matrix_fidelity\"] == \"represented_equations_retained\"\n", "\n"),
 ("S24_nr_accuracy", SC, "Some(\"not_claimed\" | \"reference_verified\")\n            )\n            && q[\"evidence_refs\"]", "Some(\"not_claimed\" | \"reference_verified\" | \"unresolved\")\n            )\n            && q[\"evidence_refs\"]"),
 ("S25_nr_refs_nonempty", SC, "            && q[\"evidence_refs\"].as_array().is_some_and(|refs| {\n                !refs.is_empty()\n                    && refs", "            && q[\"evidence_refs\"].as_array().is_some_and(|refs| {\n                refs"),
 ("S26_nr_refs_resolve", SC, ".all(|r| r.as_str().is_some_and(|id| ids.contains(id)))\n            })\n    })", ".all(|r| r.as_str().is_some())\n            })\n    })"),
 ("S27_nr_duplicate_ids", SC, "                Some(id) if ids.insert(id) => {}", "                Some(id) => { ids.insert(id); }"),
 ("S28_nr_missing_quality", SC, "        let Some(q) = quality.get(index) else {\n            return false;", "        let Some(q) = quality.get(index) else {\n            return true;"),
 ("S29a_status_selected", SC, "            Some(\"selected\") => return true,", "            Some(\"selected\") => return false,"),
 ("S29b_status_other", SC, "            _ => return false,\n        }\n        let Some(q)", "            _ => return true,\n        }\n        let Some(q)"),
 ("S29c_status_not_required", SC, "            Some(\"not_required\") => {}", "            Some(\"not_required\") => return true,"),
 ("S30_binding_branch_off", SC, "    if is_retained(envelope) {\n        return retained_binding_refusal(envelope, row);", "    if false {\n        return retained_binding_refusal(envelope, row);"),
 ("S31_binding_refused_none", SC, "        return Some(RULE_QUANTITY_NOT_COVERED);\n    };\n    let id = row", "        return None;\n    };\n    let id = row"),
 ("S32_binding_absolute", SC, "        AccuracyClass::AbsoluteVerified { .. } => Some(RULE_QUANTITY_BELOW_VERIFIED_FLOOR),", "        AccuracyClass::AbsoluteVerified { .. } => None,"),
 ("S33_binding_not_covered", SC, "        AccuracyClass::NotCovered => Some(RULE_QUANTITY_NOT_COVERED),\n        _ => None,\n    }\n}\n/// D2 4.9.4", "        AccuracyClass::NotCovered => None,\n        _ => None,\n    }\n}\n/// D2 4.9.4"),
 ("S34_summary_withheld_current", SC, "if current { n[1] + n[2] } else", "if current { n[1] } else"),
 ("S35_summary_withheld_held", SC, "else { n[0] + n[1] + n[2] + n[3] }", "else { n[0] + n[1] + n[2] }"),
 ("S36_summary_requested", SC, "                .map(|c| serde_json::json!({\"ref_type\":\"load_case\",\"ref_id\":c[\"id\"]}))", "                .map(|c| serde_json::json!({\"ref_type\":\"load_case\",\"ref_id\":c[\"name\"]}))"),
 ("S37_summary_class_index", SC, "                    AccuracyClass::InputDerived => 3,", "                    AccuracyClass::InputDerived => 0,"),
 ("S38_summary_interval", SC, "\"interval_bindable\":0,", "\"interval_bindable\":1,"),
 ("S39_row_classes_none", SC, "    if !is_retained(source) {\n        return Ok(None);\n    }\n    let validation", "    if true {\n        return Ok(None);\n    }\n    let validation"),
 ("S40_table_hash", SC, "    if format!(\"{:x}\", Sha256::digest(bytes)) != PREVIEW_PHYSICS_RETAINED_SHA256 {", "    if false {"),
 ("D01_derive_no_classes", DV, "    let (table, version) = for_source(source)?;\n    let classes = retained_row_classes(source)?;\n    guard_json(model)?;", "    let (table, version) = for_source(source)?;\n    let classes: Option<std::collections::HashMap<String, AccuracyClass>> = None;\n    guard_json(model)?;"),
 ("D02_derive_no_evidence", DV, "                    | PREVIEW_PHYSICS_RETAINED_ID\n", "\n"),
 ("D03_derive_no_receipt", DV, "            e[\"retained_precision\"] = source[\"retained_precision\"].clone();\n", ""),
 ("D05_derive_no_override", DV, "if review_missing || physical_missing || class.is_some() {", "if review_missing || physical_missing {"),
 ("D06_derive_reason", DV, "            let reason = if let Some((code, _)) = &class {\n                *code\n            } else if review_missing {", "            let reason = if review_missing {"),
 ("D07_derive_message", DV, "\"message\":class.as_ref().map(|(_, message)| message.clone()).unwrap_or_else(||", "\"message\":None::<String>.unwrap_or_else(||"),
 ("D08_disclosure_absolute", DV, "        AccuracyClass::AbsoluteVerified { bound_bits } => Some((", "        AccuracyClass::AbsoluteVerified { bound_bits } if false => Some(("),
 ("D09_disclosure_not_covered", DV, "        AccuracyClass::NotCovered => Some((", "        AccuracyClass::NotCovered if false => Some(("),
 ("D10_disclosure_bound_text", DV, "(binary64 {bound_bits:016x})", "(binary64 {bound_bits:x})"),
 ("D11_validate_no_classes", DV, "    let (table, version) = for_source(source)?;\n    let classes = retained_row_classes(source)?;\n    if matches!(", "    let (table, version) = for_source(source)?;\n    let classes: Option<std::collections::HashMap<String, AccuracyClass>> = None;\n    if matches!("),
 ("D12_validate_receipt_equality", DV, "        if doc[\"result_envelope\"][\"retained_precision\"] != source[\"retained_precision\"] {", "        if false {"),
 ("D13_validate_downgrade", DV, "    } else if doc[\"result_envelope\"].get(\"retained_precision\").is_some() {\n        return Err(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN.into());\n    }\n    if version", "    }\n    if version"),
 ("D14_validate_override", DV, "        if class.is_some() {\n            disposition = \"disclosed\";\n        }", "        if false {\n            disposition = \"disclosed\";\n        }"),
 ("D15_validate_consistency", DV, "                if !consistent {", "                if false {"),
 ("D16_validate_claim", DV, "                    None => !claimed,", "                    None => true,"),
 ("D17_validate_claim_not_covered", DV, "                    || target[\"reason_code\"] == RETAINED_NOT_COVERED;", ";"),
 ("D18_validate_message", DV, "target[\"reason_code\"] == *code && target[\"message\"] == *message", "target[\"reason_code\"] == *code"),
 ("P01_public_entry_refuses", PY, "    return _validate_draft(source, invocation)", "    _need(_IMPLEMENTATION_COMPLETE, \"G0\", \"SOURCE_PRODUCER_CONTRACT_UNSUPPORTED\")\n    return _validate_draft(source, invocation)"),
 ("P02_public_entry_drops_invocation", PY, "    return _validate_draft(source, invocation)", "    return _validate_draft(source, None)"),
 ("P03_public_entry_eligible", PY, "    return _validate_draft(source, invocation)", "    result = _validate_draft(source, invocation)\n    return dict(result, numerical_eligible=invocation is not None)"),
]

def run_rust(label):
    cmd = f"cd {MUT}/core/reporting/result_export && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --target-dir {WT}/targets/i66-u6a/mut --test retained_precision_carriers --test preview_physics_contract --test derivative_contract"
    return subprocess.run(cmd, shell=True, capture_output=True, text=True)

def run_py(label):
    cmd = (f"cd {MUT} && PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN={WT}/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson "
           f"OPENPIPESTRESS_UNITS_BIN={WT}/targets/i52-readers/units/release/openpipestress_units perl -e 'alarm shift; exec @ARGV' 1200 "
           f"{VENV}/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_contract.py -k 'public_entry or not_qualification'")
    return subprocess.run(cmd, shell=True, capture_output=True, text=True)

def main(only):
    results = []
    for mid, rel, old, new in M:
        if only and mid not in only:
            continue
        if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
            print("MEMGUARD NOT RUNNING"); sys.exit(9)
        src = open(f"{CAND}/{rel}").read()
        assert src.count(old) == 1, (mid, src.count(old))
        open(f"{MUT}/{rel}", "w").write(src.replace(old, new))
        t = time.time()
        r = run_py(mid) if rel == PY else run_rust(mid)
        out = r.stdout + r.stderr
        compile_error = ("error[E" in out or "could not compile" in out) if rel != PY else ("SyntaxError" in out or "IndentationError" in out)
        failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", out, re.M) + re.findall(r"^FAILED (\S+)", out, re.M)))
        killed = r.returncode != 0 and not compile_error and bool(failed)
        status = "KILLED" if killed else ("COMPILE_ERROR" if compile_error else ("SURVIVED" if r.returncode == 0 else "ERROR"))
        results.append({"id": mid, "file": rel, "status": status, "failed": failed[:6], "seconds": round(time.time() - t)})
        print(f"{mid} {status} {failed[:3]}", flush=True)
        shutil.copyfile(f"{CAND}/{rel}", f"{MUT}/{rel}")
    with open(f"{S}/logs/mutants_{os.environ.get('MUT_TAG', 'all')}.json", "w") as fh:
        json.dump(results, fh, indent=1)

if __name__ == "__main__":
    main(sys.argv[1:])
