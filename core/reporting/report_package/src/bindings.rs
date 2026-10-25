//! The shared producer's request checks for a version 2.0.0 package (DEL-08-01
//! Design `report-package-current-results.md` §2.3 items 0-8). The Tauri
//! bridge and the headless runner both call [`check_request`] through
//! `wire::assemble_wire_request`, before assembly.
//!
//! These checks bind the request's members to one model, one run and one
//! validated source result. They are equality and derivation checks:
//! - the analysis record and the input manifest are not in the request, so
//!   their digests are bound, not recomputed;
//! - nothing here authenticates the source or signs the package.
//!
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::document_form;
use crate::wire::{LinkedSolverIdentity, ReportPackageRequest};
use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_canonical_json::canonical_number;
use open_pipe_stress_result_export::derivative::{digest, guard_json, validate_document};
use open_pipe_stress_result_export::semantic_contract::for_source;
use serde_json::{json, Value};
use std::collections::BTreeSet;

pub const BINDING_MISMATCH: &str = "REPORT-PACKAGE-BINDING-MISMATCH";
pub const SOURCE_INVALID: &str = "REPORT-PACKAGE-SOURCE-INVALID";
pub const REPORT_ROWS_MISMATCH: &str = "REPORT-PACKAGE-REPORT-ROWS-MISMATCH";
pub const RULE_CHECK_BINDING_MISMATCH: &str = "REPORT-PACKAGE-RULE-CHECK-BINDING-MISMATCH";
pub const RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED: &str = "REPORT-PACKAGE-RULE-PACK-CHECKSUM-BASIS-UNSUPPORTED";
pub const REPORT_CONTEXT_NOT_LOCAL_PRIVATE: &str = "REPORT-PACKAGE-REPORT-CONTEXT-NOT-LOCAL-PRIVATE";
pub const SOLVER_IDENTITY_MISMATCH: &str = "REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH";

/// The desktop's export profile; its result documents carry the received
/// Current carrier origin.
pub const DESKTOP_PROFILE: &str = "desktop_local_private_report_package_1";
/// The headless runner's export profile; its result documents carry the
/// attested headless producer origin.
pub const RUNNER_PROFILE: &str = "runner_local_private_report_package_1";
const DESKTOP_ORIGIN: &str = "received_current_dimension_absent";
const RUNNER_ORIGIN: &str = "attested_headless_producer";

/// Basis labels that describe a SHA-256 over RFC 8785 (JCS) bytes: the
/// analysis record's and the desktop report's label, and the derivative's.
const JCS_LABELS: [&str; 2] = ["rfc8785_jcs", "openpipestress_jcs_ijson_v1"];
const DERIVATIVE_LABEL: &str = "openpipestress_jcs_ijson_v1";
const RULE_TOKENS: [&str; 3] = ["RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"];
const DEC018_UNIT_SYSTEM: &str = "unit-system:dec-018-si-dual-display";
const STORAGE_CONVENTION: &str = "entered_units_preserved";
const QUANTITY_DISPLAY_POLICY: &str = "display result-row values with their explicit units; no report-time conversion";

fn mismatch(what: impl std::fmt::Display) -> String {
    format!("{BINDING_MISMATCH}: {what}")
}

fn require(condition: bool, what: impl std::fmt::Display) -> Result<(), String> {
    if condition { Ok(()) } else { Err(mismatch(what)) }
}

fn reference(ref_type: &str, ref_id: &str) -> Value {
    json!({"ref_type": ref_type, "ref_id": ref_id})
}

fn object_ref(object_type: &str, id: &str) -> Value {
    json!({"object_type": object_type, "ref": id})
}

fn text<'a>(value: &'a Value, pointer: &str) -> Option<&'a str> {
    value.pointer(pointer).and_then(Value::as_str)
}

fn array<'a>(value: &'a Value, pointer: &str) -> &'a [Value] {
    value.pointer(pointer).and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

/// A SHA-256 hash record: algorithm, an accepted JCS basis label, the payload
/// reference and the value.
fn check_hash(record: &Value, payload_ref: &Value, value: &str, labels: &[&str], what: &str) -> Result<(), String> {
    require(record["algorithm"] == "sha256", format!("{what}: algorithm"))?;
    require(
        record["canonicalization"].as_str().is_some_and(|label| labels.contains(&label)),
        format!("{what}: basis label"),
    )?;
    require(record["payload_ref"] == *payload_ref, format!("{what}: payload reference"))?;
    require(record["value"].as_str() == Some(value), format!("{what}: value"))
}

fn unique_refs<'a>(items: impl Iterator<Item = (&'a str, &'a str)>, what: &str) -> Result<BTreeSet<(String, String)>, String> {
    let mut set = BTreeSet::new();
    for (kind, id) in items {
        require(set.insert((kind.to_string(), id.to_string())), format!("{what}: duplicate reference"))?;
    }
    Ok(set)
}

fn status_set(items: &[Value]) -> BTreeSet<String> {
    items.iter().filter_map(Value::as_str).map(str::to_string).collect()
}

/// The report-row contract (§2.3 item 6): one row per source row, in order,
/// with the desktop's five-field projection
/// (`apps/desktop/src/features/report/reportResultRows.ts`). The value text is
/// the ECMAScript Number-to-String form (`canonical_number`), after
/// `guard_json`. Parity vectors:
/// `fixtures/reports/invented/report_result_rows_parity.json`.
pub fn expected_report_result_rows(source: &Value) -> Result<Vec<Value>, String> {
    guard_json(source).map_err(|detail| format!("{REPORT_ROWS_MISMATCH}: {detail}"))?;
    let rows = source["results"]
        .as_array()
        .ok_or_else(|| format!("{REPORT_ROWS_MISMATCH}: the source has no result rows"))?;
    rows.iter()
        .map(|row| {
            let field = |key: &str| {
                row[key].as_str().ok_or_else(|| format!("{REPORT_ROWS_MISMATCH}: a source row has no string {key}"))
            };
            let (id, kind, unit) = (field("id")?, field("kind")?, field("unit")?);
            let value = match &row["value"] {
                Value::Number(number) => canonical_number(number),
                _ => return Err(format!("{REPORT_ROWS_MISMATCH}: source row {id} has no numeric value")),
            };
            let label = match row.get("metadata") {
                None | Some(Value::Null) => kind.to_string(),
                Some(metadata) => match metadata["component"].as_str() {
                    Some(component) => format!("{kind} ({component})"),
                    None => return Err(format!("{REPORT_ROWS_MISMATCH}: source row {id} metadata has no component")),
                },
            };
            let case_ref = match row.get("basis_ref") {
                None | Some(Value::Null) => "TBD".to_string(),
                Some(basis) => match (basis["ref_type"].as_str(), basis["ref_id"].as_str()) {
                    (Some(kind), Some(basis_id)) => format!("{kind}:{basis_id}"),
                    _ => return Err(format!("{REPORT_ROWS_MISMATCH}: source row {id} has a malformed basis_ref")),
                },
            };
            Ok(json!({
                "row_id": format!("row:{id}"),
                "label": label,
                "case_ref": case_ref,
                "quantity_display": format!("{value} {unit}"),
                "source_ref": format!("result:{id}"),
            }))
        })
        .collect()
}

/// The safe-reference projection of the report's rule-pack checksum to its
/// audit basis and payload reference (§2.4). Any other declared pair refuses,
/// and so does a checksum whose payload reference names another rule pack:
/// projection never relabels another payload's checksum as this pack's.
fn projected_rule_pack_checksum(checksum: &Value, rule_pack_id: &str) -> Result<(audit::Canonicalization, String), String> {
    let algorithm = checksum["algorithm"].as_str().unwrap_or("not recorded");
    let basis = checksum["canonicalization"].as_str().unwrap_or("not recorded");
    let payload_type = checksum["payload_ref"]["ref_type"].as_str().unwrap_or("");
    if checksum["payload_ref"]["ref_id"].as_str() != Some(rule_pack_id) {
        return Err(format!(
            "{RULE_CHECK_BINDING_MISMATCH}: the rule pack's checksum names another payload than rule pack {rule_pack_id}"
        ));
    }
    match (algorithm, basis, payload_type) {
        ("sha256", "JCS", "rule_pack") => Ok((audit::Canonicalization::RulePackPayloadJcs, rule_pack_id.to_string())),
        ("sha256", "rfc8785_jcs", "rule_pack_document") => {
            Ok((audit::Canonicalization::Rfc8785Jcs, format!("rule_pack_document:{rule_pack_id}")))
        }
        _ => Err(format!(
            "{RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED}: the rule pack's checksum {algorithm}/{basis} has no audit basis the package can name"
        )),
    }
}

fn redistribution_token(status: audit::RedistributionStatus) -> &'static str {
    match status {
        audit::RedistributionStatus::PublicPermissive => "public_permissive",
        audit::RedistributionStatus::PrivateOnly => "private_only",
        audit::RedistributionStatus::Unknown => "unknown",
        audit::RedistributionStatus::ProtectedSuspected => "protected_suspected",
        audit::RedistributionStatus::InventedNonEngineeringExample => "invented_non_engineering_example",
        audit::RedistributionStatus::Tbd => "TBD",
    }
}

/// Items 0-8 of §2.3, in order. `audit_manifest` is the request's audit
/// manifest after the wire's own conversion.
pub fn check_request(
    request: &ReportPackageRequest,
    audit_manifest: &audit::AuditManifest,
    linked: &LinkedSolverIdentity<'_>,
) -> Result<(), String> {
    require(request.result_documents.len() == 1, "exactly one result document is required")?;
    require(
        request.source_results.len() == request.result_documents.len(),
        "each result document needs its source result",
    )?;
    let document = &request.result_documents[0];
    let source = &request.source_results[0];
    // Items 0-3: route preflight, the complete 0.3.0 form, header dispatch and
    // the self-hash (shared with the reader).
    document_form::check_route_admitted(document)?;
    document_form::validate_result_export_form(document)?;
    let header_table = document_form::check_transported_header(document)?;
    document_form::check_self_hash(document)?;
    // Item 4: the source selects the same table, and the document is valid
    // against it (row accounting, witnesses, origin and carrier hash).
    let (source_table, _) = for_source(source).map_err(|detail| format!("{SOURCE_INVALID}: {detail}"))?;
    require(std::ptr::eq(source_table, header_table), "the source and the document select different tables")?;
    validate_document(document, source).map_err(|detail| format!("{SOURCE_INVALID}: {detail}"))?;
    // Every checksum record names the subject its location derives (§2.4,
    // "Checksum subjects"); `validate_document` binds the row records to the
    // origin's carrier reference, and this binds that reference.
    document_form::check_document_subjects(document)?;

    let e = &document["result_envelope"];
    let report = &request.report;
    let summary = &report["calculation_report"]["model_input_summary"];
    let origin = &e["reproducibility"]["source_origin_bindings"][0];
    let origin_class = origin["origin_class"].as_str().unwrap_or("");
    let desktop = match (request.export_profile_id.as_str(), origin_class) {
        (DESKTOP_PROFILE, DESKTOP_ORIGIN) => true,
        (RUNNER_PROFILE, RUNNER_ORIGIN) => false,
        _ => return Err(mismatch("the export profile does not match the result document's origin")),
    };

    // Item 5: identity and hash bindings.
    let model = source["model_ref"].as_str().ok_or_else(|| mismatch("the source names no model"))?;
    require(e["model_ref"] == reference("model_payload", model), "document model")?;
    let audit_model = audit_manifest.model_hash.as_ref().ok_or_else(|| mismatch("audit model hash missing"))?;
    require(audit_model.payload_ref == model, "audit model-hash payload reference")?;
    require(
        request.source_model_ref.ref_type == "model" && request.source_model_ref.ref_id == model,
        "package source_model_ref",
    )?;
    require(report["report_sections"]["model_ref"] == reference("model", model), "report sections model")?;
    require(summary["model_ref"] == reference("model", model), "report model summary model_ref")?;
    require(summary["project_ref"] == reference("project", model), "report model summary project_ref")?;

    let model_hash = text(e, "/reproducibility/model_hash/value").ok_or_else(|| mismatch("document model hash"))?;
    check_hash(
        &e["reproducibility"]["model_hash"],
        &reference("model_payload", model),
        model_hash,
        &[DERIVATIVE_LABEL],
        "document model hash",
    )?;
    require(
        audit_model.value == model_hash
            && audit_model.canonicalization == audit::Canonicalization::Rfc8785Jcs
            && audit_model.payload_kind == audit::PayloadKind::ModelJson,
        "audit model hash",
    )?;
    check_hash(&summary["model_hash"], &reference("model", model), model_hash, &JCS_LABELS, "report model hash")?;

    let run = text(e, "/run_ref/ref_id").ok_or_else(|| mismatch("document run"))?;
    let run_ref = reference("analysis_run", run);
    require(e["run_ref"] == run_ref, "document run reference type")?;
    let basis_refs = &request.source_basis_refs;
    require(
        basis_refs.first().is_some_and(|first| first.ref_type == "analysis_run" && first.ref_id == run),
        "source_basis_refs[0] names the run",
    )?;
    require(report["report_sections"]["run_ref"] == run_ref, "report sections run")?;
    // The run group is tied to the source by origin; the mechanics run stays
    // bound to source.run_id by validate_document.
    if desktop {
        require(source["run_id"].as_str() == Some(run), "the desktop run is the source run")?;
    } else {
        require(origin["request_run_ref"] == reference("runner_run", run), "the runner run is the origin's request run")?;
    }

    let received_digest = digest(source).map_err(|detail| format!("{SOURCE_INVALID}: {detail}"))?;
    let envelope_ref = format!("result-envelope:{run}");
    let run_hashes = array(e, "/reproducibility/run_hashes");
    require(run_hashes.len() == 2, "the document carries the analysis-record and received-result hashes")?;
    let record_hash = run_hashes
        .iter()
        .find(|hash| hash["payload_ref"] == reference("AnalysisRun", run))
        .ok_or_else(|| mismatch("analysis-record run hash"))?;
    let record_value = record_hash["value"].as_str().ok_or_else(|| mismatch("analysis-record hash value"))?;
    check_hash(record_hash, &reference("AnalysisRun", run), record_value, &[DERIVATIVE_LABEL], "analysis-record hash")?;
    let received_hash = run_hashes
        .iter()
        .find(|hash| hash["payload_ref"] == reference("ResultEnvelope", &envelope_ref))
        .ok_or_else(|| mismatch("received-result run hash"))?;
    check_hash(
        received_hash,
        &reference("ResultEnvelope", &envelope_ref),
        &received_digest,
        &[DERIVATIVE_LABEL],
        "received-result hash",
    )?;
    require(origin["received_carrier_checksum"]["value"].as_str() == Some(&received_digest), "origin carrier digest")?;
    let audit_refs = array(report, "/calculation_report/audit_manifest_refs");
    require(audit_refs.len() == 2, "the report lists the run record and the received result")?;
    require(audit_refs[0]["ref"] == run_ref, "report run-record reference")?;
    check_hash(&audit_refs[0]["checksum"], &run_ref, record_value, &JCS_LABELS, "report run-record checksum")?;
    let report_envelope_ref = reference("result_envelope", &envelope_ref);
    require(audit_refs[1]["ref"] == report_envelope_ref, "report received-result reference")?;
    check_hash(&audit_refs[1]["checksum"], &report_envelope_ref, &received_digest, &JCS_LABELS, "report received-result checksum")?;

    let rows = array(source, "/results");
    let result_refs = array(report, "/calculation_report/result_export_refs");
    require(result_refs.len() == rows.len(), "report result references cover the source rows")?;
    let mut row_digests = Vec::with_capacity(rows.len());
    for (row, entry) in rows.iter().zip(result_refs) {
        let id = row["id"].as_str().ok_or_else(|| mismatch("source row id"))?;
        let row_digest = digest(row).map_err(|detail| format!("{SOURCE_INVALID}: {detail}"))?;
        let row_ref = reference("Result", id);
        require(entry["ref"] == row_ref, format!("report result reference {id}"))?;
        check_hash(&entry["checksum"], &row_ref, &row_digest, &JCS_LABELS, &format!("report row checksum {id}"))?;
        row_digests.push((id, row_digest));
    }

    let load_set = unique_refs(
        array(e, "/load_basis_refs").iter().filter_map(|item| Some((item["ref_type"].as_str()?, item["ref_id"].as_str()?))),
        "document load basis",
    )?;
    require(load_set.len() == array(e, "/load_basis_refs").len(), "document load basis references are complete")?;
    let package_loads = unique_refs(
        basis_refs.iter().skip(1).map(|item| (item.ref_type.as_str(), item.ref_id.as_str())),
        "source_basis_refs",
    )?;
    require(package_loads == load_set, "source_basis_refs[1..] equal the document load basis")?;

    // The current state and run sections of the handoff record.
    require(request.state_comparison_handoff_records.len() == 1, "exactly one handoff record set is required")?;
    let handoff = &request.state_comparison_handoff_records[0];
    require(
        handoff["section_set_id"].as_str().is_some_and(|id| id.ends_with(&format!(":{run}"))),
        "handoff section_set_id names the run",
    )?;
    let sections = array(handoff, "/sections/state_run_sections");
    let state = format!("state:{model}:preview");
    let of_kind = |kind: &str| sections.iter().filter(|section| section["section_kind"] == kind).collect::<Vec<_>>();
    let (model_states, run_records) = (of_kind("model_state_record"), of_kind("analysis_run_record"));
    require(model_states.len() == 1 && run_records.len() == 1, "exactly one current model-state and analysis-run section")?;
    let (model_state, run_record) = (model_states[0], run_records[0]);
    require(model_state["state_ref"] == object_ref("ModelState", &state), "handoff model-state reference")?;
    require(model_state["section_id"] == format!("state-run:{state}"), "handoff model-state section id")?;
    let state_hash = &array(model_state, "/hash_refs").first().cloned().unwrap_or(Value::Null);
    require(state_hash["payload_scope"] == "model_state", "handoff model-state hash scope")?;
    check_hash(state_hash, &object_ref("ModelState", &state), model_hash, &JCS_LABELS, "handoff model-state hash")?;
    require(run_record["state_ref"] == object_ref("ModelState", &state), "handoff analysis-run state reference")?;
    require(run_record["section_id"] == format!("state-run:{run}"), "handoff analysis-run section id")?;
    require(run_record["run_ref"] == object_ref("AnalysisRun", run), "handoff analysis-run reference")?;
    let handoff_hashes = array(run_record, "/hash_refs");
    require(handoff_hashes.len() == 2, "handoff analysis-run hash inventory")?;
    for (scope, payload, value) in [
        ("analysis_run_record", object_ref("AnalysisRun", run), record_value),
        ("received_result", object_ref("ResultEnvelope", &envelope_ref), received_digest.as_str()),
    ] {
        let hash = handoff_hashes
            .iter()
            .find(|hash| hash["payload_scope"] == scope)
            .ok_or_else(|| mismatch(format!("handoff {scope} hash")))?;
        check_hash(hash, &payload, value, &[DERIVATIVE_LABEL], &format!("handoff {scope} hash"))?;
    }
    let handoff_rows = array(run_record, "/result_refs");
    require(handoff_rows.len() == row_digests.len(), "handoff result references cover the source rows")?;
    for (entry, (id, row_digest)) in handoff_rows.iter().zip(&row_digests) {
        require(entry["result_ref"] == object_ref("Result", id), format!("handoff result reference {id}"))?;
        let hash = array(entry, "/hash_refs").first().cloned().unwrap_or(Value::Null);
        require(hash["payload_scope"] == "result_row", format!("handoff row hash scope {id}"))?;
        check_hash(&hash, &object_ref("Result", id), row_digest, &[DERIVATIVE_LABEL], &format!("handoff row hash {id}"))?;
    }
    let handoff_loads = unique_refs(
        array(run_record, "/load_basis_refs").iter().filter_map(|item| Some((item["object_type"].as_str()?, item["ref"].as_str()?))),
        "handoff load basis",
    )?;
    require(
        handoff_loads == load_set && handoff_loads.len() == array(run_record, "/load_basis_refs").len(),
        "handoff load basis equals the document's",
    )?;

    // Solver.
    let name_version = |value: &Value, name: &str, version: &str| {
        value[name].as_str() == Some(linked.solver_name) && value[version].as_str() == Some(linked.solver_version)
    };
    let solver_error = |what: &str| format!("{SOLVER_IDENTITY_MISMATCH}: {what} does not match the linked product-physics component");
    if !name_version(&e["solver_version"], "solver_name", "solver_version") {
        return Err(solver_error("the result document's solver"));
    }
    if !name_version(&source["producer"], "component_name", "component_version") {
        return Err(solver_error("the source producer"));
    }
    let context = &report["report_context"];
    if !name_version(context, "solver_name", "solver_version") {
        return Err(solver_error("the report context's solver"));
    }
    let build = text(e, "/solver_version/solver_build_ref").unwrap_or("");
    require(context["solver_build_ref"].as_str() == Some(build), "report context solver build stamp")?;
    let build_ok = if desktop {
        build == linked.solver_build_ref
    } else {
        let prefix = format!("headless_preview_producer; solver_component={}@{}", linked.solver_name, linked.solver_version);
        build.strip_prefix(&prefix).is_some_and(|rest| rest.is_empty() || rest.starts_with("; nonlinear_component="))
    };
    if !build_ok {
        return Err(solver_error("the result document's build stamp"));
    }

    // Input manifest.
    require(text(e, "/reproducibility/audit_manifest_ref/ref_id") == Some(audit_manifest.manifest_id.as_str()), "audit manifest id")?;
    let input = audit_manifest.input_manifest_hash.as_ref().ok_or_else(|| mismatch("audit input-manifest hash missing"))?;
    require(
        input.canonicalization == audit::Canonicalization::Rfc8785Jcs && input.payload_kind == audit::PayloadKind::InputManifestJson,
        "audit input-manifest hash basis",
    )?;
    let manifest_refs = array(run_record, "/reproducibility/input_manifest_refs");
    let manifest_hashes = array(run_record, "/reproducibility/input_manifest_hashes");
    require(manifest_refs.len() == 1 && manifest_hashes.len() == 1, "handoff input-manifest evidence")?;
    let manifest_ref = object_ref("InputManifest", &input.payload_ref);
    require(manifest_refs[0] == manifest_ref, "handoff input-manifest reference")?;
    require(manifest_hashes[0]["payload_scope"] == "input_manifest", "handoff input-manifest hash scope")?;
    check_hash(&manifest_hashes[0], &manifest_ref, &input.value, &["rfc8785_jcs"], "handoff input-manifest hash")?;
    require(summary["input_manifest_ref"]["ref_id"].as_str() == Some(input.payload_ref.as_str()), "report input-manifest reference")?;
    if desktop {
        require(origin["qualification_ref"] == reference("current_manifest", &input.payload_ref), "origin qualification reference")?;
        // §2.4: the desktop names its audit manifest by the input-manifest
        // reference, so the audit id (and with it the document's
        // `audit_manifest_ref`) and the input-manifest references name one
        // manifest. The runner names its audit manifest by its run.
        require(audit_manifest.manifest_id == input.payload_ref, "desktop audit manifest id is not the input-manifest reference")?;
    }

    // Unit system and the complete no-conversion display policy.
    require(text(e, "/unit_system_ref/ref_id") == Some(audit_manifest.unit_system_ref.as_str()), "audit unit system")?;
    if desktop {
        require(e["unit_system_ref"] == reference("unit_system", &format!("{model}:units")), "desktop unit system")?;
    }
    require(summary["unit_system_ref"] == reference("unit_system", DEC018_UNIT_SYSTEM), "report display-policy reference")?;
    let display = &summary["unit_display_summary"];
    require(display["storage_convention"] == STORAGE_CONVENTION, "unit display storage_convention")?;
    require(display["quantity_display_policy"] == QUANTITY_DISPLAY_POLICY, "unit display quantity_display_policy")?;
    require(display["conversion_performed"] == false, "unit display conversion_performed")?;
    let units: BTreeSet<&str> = rows.iter().filter_map(|row| row["unit"].as_str()).filter(|unit| !unit.is_empty()).collect();
    require(display["result_units"] == json!(units.into_iter().collect::<Vec<_>>()), "unit display result_units")?;

    // Status.
    let statuses = array(e, "/analysis_status");
    let tokens: Vec<&str> = statuses.iter().filter_map(Value::as_str).filter(|status| RULE_TOKENS.contains(status)).collect();
    match &request.rule_check_aggregate {
        Some(aggregate) => require(tokens == [aggregate.as_str()], "the document's rule token is the aggregate")?,
        None => require(
            tokens.iter().all(|token| *token == request.solve_rule_check_status),
            "the document's rule token is the solve status",
        )?,
    }
    let mut with_review = status_set(statuses);
    with_review.insert("HUMAN_REVIEW_REQUIRED".to_string());
    require(status_set(array(report, "/calculation_report/analysis_status")) == with_review, "report analysis status")?;
    require(status_set(array(run_record, "/analysis_status")) == status_set(statuses), "handoff analysis status")?;

    // Item 6: the displayed rows are derived from the validated source.
    let expected_rows = expected_report_result_rows(source)?;
    if report["result_rows"] != Value::Array(expected_rows) {
        return Err(format!("{REPORT_ROWS_MISMATCH}: the report's result rows are not the source rows' projection"));
    }

    // Item 7: rule-check content.
    let rule_error = |what: &str| format!("{RULE_CHECK_BINDING_MISMATCH}: {what}");
    let checks = &report["user_rule_checks"];
    let present = !checks.is_null();
    if request.rule_check_aggregate.is_some() != present {
        return Err(rule_error("the aggregate and the report's user-rule outcomes must be present together"));
    }
    if let Some(aggregate) = &request.rule_check_aggregate {
        if checks["aggregate_status"].as_str() != Some(aggregate.as_str()) {
            return Err(rule_error("the report's aggregate differs from the request's"));
        }
    }
    let packs = array(report, "/calculation_report/rule_pack_refs");
    if packs.len() != usize::from(present) || audit_manifest.rule_pack_refs.len() != packs.len() {
        return Err(rule_error("one rule-pack reference accompanies the user-rule outcomes, in the report and the audit"));
    }
    if let (Some(pack), Some(audited)) = (packs.first(), audit_manifest.rule_pack_refs.first()) {
        let id = pack["rule_pack_id"].as_str().unwrap_or("");
        if checks["rule_pack_id"].as_str() != Some(id) {
            return Err(rule_error("the outcomes and the reference name different rule packs"));
        }
        let (basis, payload_ref) = projected_rule_pack_checksum(&pack["checksum"], id)?;
        let checksum = audited.checksum.as_ref().ok_or_else(|| rule_error("the audit rule-pack checksum is missing"))?;
        let projected = audited.rule_pack_id == id
            && pack["version"].as_str() == Some(audited.rule_pack_version.as_str())
            && pack["source_notice"].as_str() == Some(audited.source_notice.as_str())
            && pack["redistribution_status"].as_str() == Some(redistribution_token(audited.redistribution_status))
            && audited.private_payload_redacted
            && checksum.canonicalization == basis
            && checksum.payload_kind == audit::PayloadKind::RulePackReference
            && checksum.payload_ref == payload_ref
            && pack["checksum"]["value"].as_str() == Some(checksum.value.as_str());
        if !projected {
            return Err(rule_error("the audit rule-pack reference is not the report reference's projection"));
        }
    }

    // Item 8: the local-private report context.
    if report["report_context"]["export_context"] != "local_private" {
        return Err(format!("{REPORT_CONTEXT_NOT_LOCAL_PRIVATE}: the report package renders under local_private only"));
    }
    Ok(())
}
