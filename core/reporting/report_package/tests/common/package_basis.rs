//! The invented report-package basis (DEL-08-01 Design
//! `report-package-current-results.md` §2.7, fixture transition): an explicit,
//! test-labelled fixture transformation of real producer output into a
//! version 2.0.0 package request. It is shared, by path inclusion, by the
//! witness generator (`core/runner/headless/tests/del1005_package_witness.rs`)
//! and this crate's tests.
//!
//! From the headless producer's document `D` and mechanics carrier `S`, it
//! does three things:
//! - it builds an invented analysis record for the package run, whose
//!   `analysis_run_record` digest is the SHA-256 of the RFC 8785 bytes of the
//!   record without that entry (`core/analysis_runs/compatibility.py:73-80`);
//! - it normalizes a clone of `D` (the two run-hash records, and for the
//!   desktop profile the received-Current origin and desktop identities);
//! - it derives the final document once with `derive_document`.
//!
//! Every other member then comes from the same basis. The result is versioned
//! input for archive and determinism checks, not an independent numerical or
//! physical-validity witness, and not a genuine desktop analysis record.

#![allow(dead_code)]

use open_pipe_stress_report_package::bindings::{expected_report_result_rows, DESKTOP_PROFILE, RUNNER_PROFILE};
use open_pipe_stress_result_export::derivative::{checksum, derive_document, digest, reference};
use serde_json::{json, Value};

pub const CALCULATION_REPORT_FIXTURE: &str =
    include_str!("../../../../../fixtures/reports/invented/calculation_report_fixture.json");
pub const DERIVATIVE_LABEL: &str = "openpipestress_jcs_ijson_v1";
pub const FIXTURE_LABEL: &str =
    "Invented DEL-10-05 report-package witness; test-labelled fixture transformation of real headless producer output; not engineering output";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Runner,
    Desktop,
}

pub struct Inputs<'a> {
    /// The solved model `M`.
    pub model: &'a Value,
    /// The mechanics carrier `S`.
    pub source: &'a Value,
    /// The headless producer's document `D`.
    pub producer_document: &'a Value,
    /// The solve request `D` was derived with (runner profile).
    pub solve_request: &'a Value,
    /// The committed invented input manifest and its id.
    pub input_manifest: &'a Value,
    pub input_manifest_id: &'a str,
    pub solver_name: &'a str,
    pub solver_version: &'a str,
    pub profile: Profile,
    /// Desktop profile only: a package run label other than the source run,
    /// for the relabelling refusal test. `None` for every committed basis.
    pub run_override: Option<&'a str>,
}

pub struct Basis {
    pub export_results: Value,
    pub analysis_record: Value,
}

fn object_ref(object_type: &str, id: &str) -> Value {
    json!({"object_type": object_type, "ref": id})
}

fn hash_record(canonicalization: &str, payload_ref: Value, value: &str) -> Value {
    json!({"algorithm": "sha256", "canonicalization": canonicalization, "payload_ref": payload_ref, "value": value})
}

fn scoped(canonicalization: &str, payload_ref: Value, scope: &str, value: &str) -> Value {
    json!({"algorithm": "sha256", "canonicalization": canonicalization, "payload_ref": payload_ref, "payload_scope": scope, "value": value})
}

fn boundary() -> Value {
    json!({
        "human_review_required": true,
        "software_makes_compliance_claim": false,
        "software_makes_certification_claim": false,
        "software_makes_sealing_claim": false,
        "software_makes_approval_claim": false,
        "software_makes_authentication_claim": false
    })
}

fn section_provenance() -> Value {
    json!({
        "source_name": "Invented DEL-10-05 report-package witness",
        "source_location": "core/reporting/report_package/tests/common/package_basis.rs",
        "source_license": "project invented",
        "contributor": "OpenPipeStress",
        "contributor_certification": "invented non-engineering example",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "accepted",
        "privacy_classification": "invented_public_example"
    })
}

/// The invented analysis record for the package run and its self-hash.
fn analysis_record(run: &str, mechanics_run: &str, state: &str, loads: &[Value], rows: &[Value], received: &str,
                   statuses: &[Value], manifest_ref: &Value, manifest_hash: &Value) -> Result<(Value, String), String> {
    let result_refs = rows
        .iter()
        .map(|row| {
            let id = row["id"].as_str().unwrap_or_default();
            Ok(json!({
                "result_ref": object_ref("Result", id),
                "hash_refs": [scoped(DERIVATIVE_LABEL, object_ref("Result", id), "result_row", &digest(row)?)],
                "privacy_classification": "invented_public_example",
                "provenance": section_provenance()
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut record = json!({
        "fixture_label": FIXTURE_LABEL,
        "record_shape": "invented_test_analysis_record_1",
        "self_hash_rule": "the analysis_run_record hash is the SHA-256 of the RFC 8785 bytes of this record with that hash entry removed from analysis_run.hashes (core/analysis_runs/compatibility.py:73-80)",
        "analysis_run": {
            "run_id": run,
            "mechanics_run_ref": {"ref_type": "mechanics_run", "ref_id": mechanics_run},
            "model_state_ref": object_ref("ModelState", state),
            "load_basis_refs": loads,
            "result_refs": result_refs,
            "hashes": [scoped(DERIVATIVE_LABEL, object_ref("ResultEnvelope", &format!("result-envelope:{run}")), "received_result", received)],
            "analysis_status": statuses,
            "reproducibility": {"input_manifest_refs": [manifest_ref], "input_manifest_hashes": [manifest_hash]},
            "professional_boundary": boundary()
        }
    });
    let value = digest(&record)?;
    record["analysis_run"]["hashes"]
        .as_array_mut()
        .unwrap()
        .insert(0, scoped(DERIVATIVE_LABEL, object_ref("AnalysisRun", run), "analysis_run_record", &value));
    Ok((record, value))
}

/// The record's projection without its own hash entry, as
/// `analysis_record_projection` defines it.
pub fn analysis_record_projection(record: &Value) -> Value {
    let mut projected = record.clone();
    let hashes = projected["analysis_run"]["hashes"].as_array_mut().unwrap();
    hashes.retain(|hash| hash["payload_scope"] != "analysis_run_record");
    projected
}

pub fn build(inputs: &Inputs<'_>) -> Result<Basis, String> {
    let (model, source) = (inputs.model, inputs.source);
    let model_id = source["model_ref"].as_str().ok_or("SOURCE_MODEL_MISSING")?.to_string();
    let mechanics_run = source["run_id"].as_str().ok_or("SOURCE_RUN_MISSING")?.to_string();
    let state = format!("state:{model_id}:preview");
    let rows = source["results"].as_array().ok_or("SOURCE_ROWS_MISSING")?.clone();
    let received = digest(source)?;
    let linked_build = format!("{}@{}", inputs.solver_name, inputs.solver_version);
    let manifest_digest = digest(inputs.input_manifest)?;
    let manifest_ref = object_ref("InputManifest", inputs.input_manifest_id);
    let manifest_hash = scoped("rfc8785_jcs", manifest_ref.clone(), "input_manifest", &manifest_digest);

    let mut base = inputs.producer_document.clone();
    let (run, origin, request, audit_id) = match inputs.profile {
        Profile::Runner => {
            let run = base["result_envelope"]["run_ref"]["ref_id"].as_str().ok_or("RUN_MISSING")?.to_string();
            let origin = base["result_envelope"]["reproducibility"]["source_origin_bindings"][0].clone();
            let audit = base["result_envelope"]["reproducibility"]["audit_manifest_ref"]["ref_id"]
                .as_str()
                .ok_or("AUDIT_REF_MISSING")?
                .to_string();
            (run, origin, Some(inputs.solve_request.clone()), audit)
        }
        Profile::Desktop => {
            let run = inputs.run_override.unwrap_or(&mechanics_run).to_string();
            let e = &mut base["result_envelope"];
            e["envelope_id"] = json!(format!("result-envelope:{run}"));
            e["run_ref"] = reference("analysis_run", &run);
            e["result_sets"][0]["set_id"] = json!(format!("result-set:{run}:mechanics"));
            e["result_sets"][0]["basis_ref"] = reference("analysis_run", &run);
            e["unit_system_ref"] = reference("unit_system", &format!("{model_id}:units"));
            e["solver_version"] = json!({"solver_name": inputs.solver_name, "solver_version": inputs.solver_version, "solver_build_ref": linked_build});
            let carrier = checksum(source, "received_current_dimension_absent_carrier", reference("received_current_carrier", &run))?;
            let origin = json!({
                "origin_id": "source-origin:current-received",
                "origin_class": "received_current_dimension_absent",
                "qualification_ref": reference("current_manifest", inputs.input_manifest_id),
                "authentic_producer_available": false,
                "received_carrier_checksum": carrier,
                "original_producer_checksum": null,
                "origin_limit": FIXTURE_LABEL,
                "actual_model_ref": reference("model_payload", &model_id),
                "mechanics_run_ref": reference("mechanics_run", &mechanics_run),
                "request_model_ref": null,
                "request_run_ref": null,
                "request_alias_disclosure": null
            });
            (run, origin, None, inputs.input_manifest_id.to_string())
        }
    };
    let envelope_ref = format!("result-envelope:{run}");
    let statuses = base["result_envelope"]["analysis_status"].clone();
    let statuses = statuses.as_array().ok_or("STATUS_MISSING")?.clone();
    let loads: Vec<Value> = base["result_envelope"]["load_basis_refs"]
        .as_array()
        .ok_or("LOADS_MISSING")?
        .iter()
        .map(|item| object_ref(item["ref_type"].as_str().unwrap_or_default(), item["ref_id"].as_str().unwrap_or_default()))
        .collect();
    let (record, record_value) =
        analysis_record(&run, &mechanics_run, &state, &loads, &rows, &received, &statuses, &manifest_ref, &manifest_hash)?;

    // Normalize the package base, then derive once.
    {
        let e = &mut base["result_envelope"];
        e["reproducibility"]["run_hashes"] = json!([
            hash_record(DERIVATIVE_LABEL, reference("AnalysisRun", &run), &record_value),
            hash_record(DERIVATIVE_LABEL, reference("ResultEnvelope", &envelope_ref), &received),
        ]);
        e["reproducibility"]["audit_manifest_ref"] = reference("audit_manifest", &audit_id);
        e["reproducibility"].as_object_mut().ok_or("REPRODUCIBILITY_MISSING")?.remove("derivative_hash");
    }
    let document = derive_document(base, model, source, origin, request.as_ref())?;
    let e = &document["result_envelope"];
    let model_hash = e["reproducibility"]["model_hash"]["value"].as_str().ok_or("MODEL_HASH_MISSING")?.to_string();
    let solver = e["solver_version"].clone();
    let unit_system = e["unit_system_ref"]["ref_id"].as_str().ok_or("UNIT_SYSTEM_MISSING")?.to_string();
    let result_units: std::collections::BTreeSet<&str> =
        rows.iter().filter_map(|row| row["unit"].as_str()).filter(|unit| !unit.is_empty()).collect();
    let mut sorted_statuses: Vec<&str> = statuses.iter().filter_map(Value::as_str).collect();
    sorted_statuses.sort();

    let provenance = section_provenance();
    let fixture: Value = serde_json::from_str(CALCULATION_REPORT_FIXTURE).map_err(|error| error.to_string())?;
    let mut report = fixture["calculation_report"].clone();
    report["report_id"] = json!(format!("report:{run}"));
    report["model_input_summary"] = json!({
        "project_ref": reference("project", &model_id),
        "model_ref": reference("model", &model_id),
        "persistence_ref": reference("project_persistence", "TBD"),
        "unit_system_ref": reference("unit_system", "unit-system:dec-018-si-dual-display"),
        "unit_display_summary": {
            "storage_convention": "entered_units_preserved",
            "model_units": model["project"]["units"].clone(),
            "result_units": result_units.into_iter().collect::<Vec<_>>(),
            "quantity_display_policy": "display result-row values with their explicit units; no report-time conversion",
            "conversion_performed": false
        },
        "model_hash": hash_record("rfc8785_jcs", reference("model", &model_id), &model_hash),
        "input_manifest_ref": reference("InputManifest", inputs.input_manifest_id),
        "provenance": provenance
    });
    report["load_case_summary"] = json!(model["load_cases"]
        .as_array()
        .ok_or("MODEL_LOAD_CASES_MISSING")?
        .iter()
        .map(|case| json!({
            "load_ref": reference("load_case", case["id"].as_str().unwrap_or_default()),
            "label": case["label"].as_str().filter(|label| !label.trim().is_empty()).unwrap_or("invented load case"),
            "basis": case["kind"].as_str().filter(|kind| !kind.trim().is_empty()).unwrap_or("invented"),
            "source": reference("model", &model_id),
            "provenance": provenance
        }))
        .collect::<Vec<_>>());
    let envelope = |reference_value: Value, schema: &str, value: &str| {
        json!({
            "ref": reference_value,
            "schema_ref": reference("schema", schema),
            "checksum": hash_record("rfc8785_jcs", reference_value, value),
            "privacy_classification": "invented_public_example",
            "provenance": provenance
        })
    };
    report["result_export_refs"] = json!(rows
        .iter()
        .map(|row| Ok(envelope(reference("Result", row["id"].as_str().unwrap_or_default()), "schemas/results.schema.yaml", &digest(row)?)))
        .collect::<Result<Vec<_>, String>>()?);
    report["audit_manifest_refs"] = json!([
        envelope(reference("analysis_run", &run), "schemas/analysis_run.schema.yaml", &record_value),
        envelope(reference("result_envelope", &envelope_ref), "schemas/results.schema.yaml", &received),
    ]);
    report["rule_pack_refs"] = json!([]);
    let mut report_statuses = sorted_statuses.clone();
    if !report_statuses.contains(&"HUMAN_REVIEW_REQUIRED") {
        report_statuses.push("HUMAN_REVIEW_REQUIRED");
        report_statuses.sort();
    }
    report["analysis_status"] = json!(report_statuses);

    let disclosures: Vec<Value> = report_statuses
        .iter()
        .map(|status| json!({
            "status": status,
            "source": reference("analysis_run", &run),
            "affected_object": reference("model", &model_id),
            "explanation": format!("Automatic analysis status reported by the invented run record: {status}."),
            "human_review_required": true,
            "human_acceptance_ref": null
        }))
        .collect();
    let report_sections = json!({
        "report_section_id": format!("sections:{run}"),
        "model_ref": reference("model", &model_id),
        "run_ref": reference("analysis_run", &run),
        "diagnostics": [],
        "analysis_status_disclosures": disclosures,
        "provenance_notes": [provenance],
        "user_supplied_values": [],
        "assumptions": [],
        "limitations": [{
            "limitation_id": format!("limitation:{run}:invented-witness"),
            "source": reference("report_package", "del1005-invented-package"),
            "affected_scope": reference("model", &model_id),
            "statement": "Invented witness data only; not engineering output.",
            "effect": {
                "mechanics_solve_qualified": true,
                "user_rule_check_qualified": false,
                "report_completeness": "qualified",
                "human_review_required": true
            },
            "provenance": provenance
        }],
        "unresolved_tbds": [],
        "professional_boundary": boundary()
    });
    let rendered = json!({
        "report_title": "Invented DEL-10-05 Export Results Report",
        "calculation_report": report,
        "report_sections": report_sections,
        "result_rows": expected_report_result_rows(source)?,
        "report_context": {
            "export_context": "local_private",
            "generated_at": null,
            "software_name": "Invented DEL-10-05 witness",
            "software_version": null,
            "build_release_identifier": null,
            "solver_name": solver["solver_name"],
            "solver_version": solver["solver_version"],
            "solver_build_ref": solver["solver_build_ref"],
            "analysis_run_name": null,
            "analysis_run_recorded_at": null,
            "solver_mode": null,
            "solver_settings_ref": null,
            "solver_settings": [],
            "model_coordinate_system": null,
            "result_coordinate_systems": []
        },
        "user_rule_checks": null
    });

    let unit_context = reference("unit_system", "unit-system:dec-018-si-dual-display");
    let state_section = json!({
        "section_id": format!("state-run:{state}"),
        "section_kind": "model_state_record",
        "state_ref": object_ref("ModelState", &state),
        "run_ref": null,
        "hash_refs": [scoped("rfc8785_jcs", object_ref("ModelState", &state), "model_state", &model_hash)],
        "diagnostics": [], "warnings": [], "assumptions": [],
        "analysis_status": sorted_statuses,
        "unit_context": unit_context,
        "solver_context": null, "settings_ref": null,
        "source_provenance": provenance,
        "privacy_classification": "invented_public_example",
        "review_state": "pending",
        "limitations": [],
        "professional_boundary": boundary()
    });
    let run_section = json!({
        "section_id": format!("state-run:{run}"),
        "section_kind": "analysis_run_record",
        "state_ref": object_ref("ModelState", &state),
        "run_ref": object_ref("AnalysisRun", &run),
        "hash_refs": record["analysis_run"]["hashes"],
        "diagnostics": [], "warnings": [], "assumptions": [],
        "analysis_status": sorted_statuses,
        "unit_context": unit_context,
        "solver_context": null, "settings_ref": null,
        "load_basis_refs": loads,
        "result_refs": record["analysis_run"]["result_refs"],
        "rule_pack_refs": [], "library_refs": [],
        "source_provenance": provenance,
        "privacy_classification": "invented_public_example",
        "review_state": "pending",
        "reproducibility": record["analysis_run"]["reproducibility"],
        "limitations": [],
        "professional_boundary": boundary()
    });
    let handoff = json!({
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-08-06",
        "package_id": "PKG-08",
        "scope_item": "SOW-024",
        "section_set_id": format!("invented-witness-session:{run}"),
        "section_contract_status": "backend_report_section_records_only",
        "source_notes": [FIXTURE_LABEL],
        "sections": {"state_run_sections": [state_section, run_section], "comparison_sections": [], "handoff_sections": []},
        "diagnostics": [],
        "professional_boundary": {
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false,
            "software_makes_authentication_claim": false,
            "software_creates_professional_reliance_record": false,
            "software_creates_external_validation_record": false
        }
    });

    let mut source_basis_refs = vec![reference("analysis_run", &run)];
    source_basis_refs.extend(e["load_basis_refs"].as_array().ok_or("LOADS_MISSING")?.iter().cloned());
    let (package_id, profile) = match inputs.profile {
        Profile::Runner => ("del1005-invented-package".to_string(), RUNNER_PROFILE),
        Profile::Desktop => (format!("desktop-report-{}", run.replace(':', "-")), DESKTOP_PROFILE),
    };
    let export_results = json!({
        "package_id": package_id,
        "export_profile_id": profile,
        "source_model_ref": reference("model", &model_id),
        "source_basis_refs": source_basis_refs,
        "report": rendered,
        "audit_manifest": {
            "manifest_id": audit_id,
            "model_hash": {"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_kind": "model_json", "payload_ref": model_id, "value": model_hash},
            "input_manifest_hash": {"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_kind": "input_manifest_json", "payload_ref": inputs.input_manifest_id, "value": manifest_digest},
            "solver_version": {"solver_name": inputs.solver_name, "solver_version": inputs.solver_version, "solver_build_ref": linked_build},
            "unit_system_ref": unit_system,
            "rule_pack_refs": [],
            "assets": [],
            "professional_boundary": boundary()
        },
        "result_documents": [document],
        "source_results": [source],
        "state_comparison_handoff_records": [handoff],
        "rule_check_aggregate": null,
        "solve_rule_check_status": source["status"]["rule_check"]
    });
    Ok(Basis { export_results, analysis_record: record })
}
