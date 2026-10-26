//! The exact-route report sections' projection and the package's item 9
//! (W-11, DEL-08-01 Design `exact-route-report-sections.md` §2.4, §3.2-§3.5).
//!
//! Dormant: while item 0 (`document_form::check_route_admitted`) refuses the
//! exact identities, nothing here runs on a production package. The
//! projection is public so that one projection serves both commands: the
//! package's item 9 here, and the desktop's native render re-derivation
//! (§3.6). The source-derived part is [`expected_exact_route_core`]; item 9
//! adds the document- and audit-derived fields. Limitation statements come
//! only from the closed statement table
//! (`open_pipe_stress_report_renderer::exact_statements`): no producer string
//! is copied and no suffix is stripped. Shared vectors:
//! `fixtures/reports/invented/exact_report_sections_parity.json`. Equality and
//! derivation checks only; standing and custody are the desktop's (§3.1, §3.6).
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::bindings::RUNNER_PROFILE;
use crate::document_form::producer_contract_id;
use open_pipe_stress_audit_manifest as audit;
use open_pipe_stress_canonical_json::canonical_number;
use open_pipe_stress_report_renderer::exact_statements::{exact_statement_for, producer_sha256};
use open_pipe_stress_report_renderer::{ExactAdmissionBinding, ExactRouteSection, ReportHeadline};
use open_pipe_stress_result_export::derivative::{digest, guard_json};
use open_pipe_stress_result_export::reader_verdict;
use open_pipe_stress_result_export::semantic_contract::{PRESSURE_ID, PRESSURE_SOURCE_ID};
use serde::Serialize;
use serde_json::{json, Value};

pub const EXACT_SECTIONS_MISMATCH: &str = "REPORT-PACKAGE-EXACT-SECTIONS-MISMATCH";
pub const EXACT_LIMITATION_UNREGISTERED: &str = "REPORT-EXACT-LIMITATION-UNREGISTERED";
pub const EXACT_HEADLINE_UNREADABLE: &str = "REPORT-EXACT-HEADLINE-UNREADABLE";
pub const EXACT_RUNNER_PROFILE: &str = "REPORT-PACKAGE-EXACT-RUNNER-PROFILE";

/// The exact identities.
pub const EXACT_IDENTITIES: [&str; 2] = [PRESSURE_ID, PRESSURE_SOURCE_ID];

/// The headline labels (§3.3), the session's (`ResultsPanel.tsx`).
pub const NOMINAL_SECTION_LABEL: &str = "Nominal section (nominal wall)";
pub const REDUCED_SECTION_LABEL: &str = "Reduced section (wall minus mill tolerance and corrosion allowance)";
/// A null headline key (incomplete coverage) is withheld, never a value.
pub const WITHHELD_HEADLINE: &str =
    "withheld: a stress maximum on this basis is not available for every member in every load case";
/// The rendered-report surface's load/reference line (§3.2): the panels'
/// shared sentence and the rendered-report tail.
pub const LOAD_REFERENCE_TEMPLATE: &str = "This result includes resolved load/reference states (reference configuration {configurations}; load cases {cases}; history {histories}). Their published limitations are listed under Limitations.";

/// The source-derived part of the exact-route section (§3.5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactRouteCore {
    pub semantic_contract_id: String,
    pub run_ref: String,
    pub result_envelope_id: String,
    pub received_result_sha256: String,
    pub load_reference_states: Option<String>,
    pub headlines: Vec<ReportHeadline>,
}

/// One expected exact limitation entry (§2.4), without the `effect` and
/// `provenance` it shares with the report's technical-preview entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExpectedLimitation {
    pub limitation_id: String,
    pub source: Value,
    pub affected_scope: Value,
    pub statement: String,
}

fn mismatch(what: impl std::fmt::Display) -> String {
    format!("{EXACT_SECTIONS_MISMATCH}: {what}")
}

fn is_lower_hex_64(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Whether `id` names an exact route.
pub fn is_exact_identity(id: &str) -> bool {
    EXACT_IDENTITIES.contains(&id)
}

/// The runner-profile refusal (§3.5): only the desktop builds exact sections,
/// because only the desktop has the Export Result JSON admission and the
/// native source binding. It reads only the profile and the identity, so it
/// refuses whatever the report carries. The producer applies it directly
/// after item 0 and the reader beside its item-0 check.
pub fn check_runner_profile(export_profile_id: &str, document: &Value) -> Result<(), String> {
    if export_profile_id == RUNNER_PROFILE && producer_contract_id(document).is_some_and(is_exact_identity) {
        return Err(format!(
            "{EXACT_RUNNER_PROFILE}: the runner profile does not package pressure-2 or pressure-source-1 results; only the desktop builds their sections."
        ));
    }
    Ok(())
}

fn source_identity(source: &Value) -> Result<&str, String> {
    match source.pointer("/producer/semantic_contract_id").and_then(Value::as_str) {
        Some(id) if is_exact_identity(id) => Ok(id),
        _ => Err(mismatch("the source is not an exact-route result")),
    }
}

/// The load/reference line (§3.2): the published records' distinct
/// configurations, cases and histories, in published order; `None` without
/// the group or without one of the three identifiers (as the panels' line).
fn load_reference_line(source: &Value) -> Option<String> {
    let records = source.pointer("/contract_evidence/load_reference_states")?.as_array()?;
    if records.is_empty() {
        return None;
    }
    let distinct = |pick: &dyn Fn(&Value) -> Option<&str>| {
        let mut out: Vec<&str> = Vec::new();
        for record in records {
            if let Some(text) = pick(record).filter(|text| !text.is_empty()) {
                if !out.contains(&text) {
                    out.push(text);
                }
            }
        }
        out
    };
    let configurations = distinct(&|record| record["reference_configuration_id"].as_str());
    let cases = distinct(&|record| record["load_case_id"].as_str());
    let histories = distinct(&|record| record.pointer("/history/kind").and_then(Value::as_str));
    if configurations.is_empty() || cases.is_empty() || histories.is_empty() {
        return None;
    }
    // One pass per slot over the template's own text: identifiers are shown
    // verbatim, whatever they contain.
    let (head, rest) = LOAD_REFERENCE_TEMPLATE.split_once("{configurations}")?;
    let (middle, rest) = rest.split_once("{cases}")?;
    let (between, tail) = rest.split_once("{histories}")?;
    Some(format!(
        "{head}{}{middle}{}{between}{}{tail}",
        configurations.join(", "),
        cases.join(", "),
        histories.join(", ")
    ))
}

/// One basis's headline (§3.3) from `summary.<key>`.
fn headline(summary: &Value, basis: &str, label: &str) -> Result<ReportHeadline, String> {
    let unreadable = || format!("{EXACT_HEADLINE_UNREADABLE}: the {basis} headline is missing or not readable.");
    let key = format!("max_open_formula_stress_{basis}");
    match summary.get(&key) {
        None => Err(unreadable()),
        Some(Value::Null) => Ok(ReportHeadline {
            basis: basis.to_string(),
            label: label.to_string(),
            value_display: WITHHELD_HEADLINE.to_string(),
            location_ref: None,
            source_ref: None,
        }),
        Some(item) => {
            let (Some(Value::Number(value)), Some(unit), Some(location), Some(result)) = (
                item.get("value"),
                item["unit"].as_str(),
                item["location_ref"].as_str(),
                item["result_ref"].as_str(),
            ) else {
                return Err(unreadable());
            };
            Ok(ReportHeadline {
                basis: basis.to_string(),
                label: label.to_string(),
                value_display: format!("{} {unit}", canonical_number(value)),
                location_ref: Some(location.to_string()),
                source_ref: Some(result.to_string()),
            })
        }
    }
}

/// The source-derived part (§3.5): the identity, the run, the result
/// envelope, the received result's digest, the load/reference line and the
/// headlines, nominal then reduced.
pub fn expected_exact_route_core(source: &Value) -> Result<ExactRouteCore, String> {
    guard_json(source).map_err(|detail| mismatch(format!("the source is not I-JSON: {detail}")))?;
    let identity = source_identity(source)?;
    let run = source["run_id"].as_str().filter(|run| !run.is_empty()).ok_or_else(|| mismatch("the source names no run"))?;
    let summary = source.get("summary").filter(|summary| summary.is_object()).ok_or_else(|| {
        format!("{EXACT_HEADLINE_UNREADABLE}: the nominal_section headline is missing or not readable.")
    })?;
    Ok(ExactRouteCore {
        semantic_contract_id: identity.to_string(),
        run_ref: run.to_string(),
        result_envelope_id: format!("result-envelope:{run}"),
        received_result_sha256: digest(source).map_err(|detail| mismatch(format!("the source digest: {detail}")))?,
        load_reference_states: load_reference_line(source),
        headlines: vec![
            headline(summary, "nominal_section", NOMINAL_SECTION_LABEL)?,
            headline(summary, "reduced_section", REDUCED_SECTION_LABEL)?,
        ],
    })
}

/// The whole expected section (§3.5): the core, with the admission binding's
/// document- and audit-derived fields. `native_job_id` and
/// `received_bytes_sha256` are custody facts the shared producer cannot see;
/// they are taken as given, after a form check.
pub fn expected_exact_route_section(
    source: &Value,
    document: &Value,
    audit_manifest: &audit::AuditManifest,
    native_job_id: &str,
    received_bytes_sha256: &str,
) -> Result<ExactRouteSection, String> {
    let core = expected_exact_route_core(source)?;
    let e = &document["result_envelope"];
    let text = |pointer: &str, what: &str| {
        e.pointer(pointer).and_then(Value::as_str).map(str::to_string).ok_or_else(|| mismatch(format!("the document has no {what}")))
    };
    let document_identity = text("/producer/semantic_contract_id", "producer identity")?;
    let run_ref = text("/run_ref/ref_id", "run")?;
    let result_envelope_id = text("/envelope_id", "envelope id")?;
    if document_identity != core.semantic_contract_id {
        return Err(mismatch("semantic_contract_id"));
    }
    if run_ref != core.run_ref {
        return Err(mismatch("admission.run_ref"));
    }
    if result_envelope_id != core.result_envelope_id {
        return Err(mismatch("admission.result_envelope_id"));
    }
    let input = audit_manifest.input_manifest_hash.as_ref().ok_or_else(|| mismatch("the audit has no input-manifest hash"))?;
    if native_job_id.trim().is_empty() {
        return Err(mismatch("admission.native_job_id"));
    }
    if !is_lower_hex_64(received_bytes_sha256) {
        return Err(mismatch("admission.received_bytes_sha256"));
    }
    Ok(ExactRouteSection {
        semantic_contract_id: core.semantic_contract_id.clone(),
        admission: ExactAdmissionBinding {
            semantic_contract_id: document_identity,
            run_ref,
            result_envelope_id,
            received_result_sha256: core.received_result_sha256,
            model_sha256: text("/reproducibility/model_hash/value", "model hash")?,
            input_manifest_ref: input.payload_ref.clone(),
            input_manifest_sha256: input.value.clone(),
            derivative_sha256: text("/reproducibility/derivative_hash/value", "derivative hash")?,
            native_job_id: native_job_id.to_string(),
            received_bytes_sha256: received_bytes_sha256.to_string(),
        },
        load_reference_states: core.load_reference_states,
        headlines: core.headlines,
    })
}

/// The expected exact limitation entries for `run` (§2.4, §3.5): one per
/// published `formulation_basis.limitations` string, in order, whose UTF-8
/// SHA-256 must name a table entry, then the profile reference with the
/// source route's table binding.
pub fn expected_exact_limitations(source: &Value, run: &str) -> Result<Vec<ExpectedLimitation>, String> {
    let identity = source_identity(source)?;
    let limitations = source
        .pointer("/formulation_basis/limitations")
        .and_then(Value::as_array)
        .filter(|list| !list.is_empty())
        .ok_or_else(|| mismatch("the source publishes no limitations"))?;
    let unregistered =
        || format!("{EXACT_LIMITATION_UNREGISTERED}: {identity} publishes a limitation this report has no reviewed statement for.");
    let entry = |id: String, statement: String| ExpectedLimitation {
        limitation_id: id,
        source: json!({"ref_type": "semantic_contract", "ref_id": identity}),
        affected_scope: json!({"ref_type": "analysis_run", "ref_id": run}),
        statement,
    };
    let mut entries = Vec::with_capacity(limitations.len() + 1);
    for (index, text) in limitations.iter().enumerate() {
        let text = text.as_str().ok_or_else(unregistered)?;
        let statement = exact_statement_for(&producer_sha256(text)).ok_or_else(unregistered)?;
        entries.push(entry(format!("limitation:{run}:exact:{}", index + 1), statement.to_string()));
    }
    let (route, _) = reader_verdict::route(source).map_err(|detail| mismatch(format!("the source route: {detail}")))?;
    let binding = reader_verdict::binding(route);
    let (Some(table_id), Some(table_sha256)) = (binding["id"].as_str(), binding["sha256"].as_str()) else {
        return Err(mismatch("the source route has no table binding"));
    };
    entries.push(entry(
        format!("limitation:{run}:exact:profile-reference"),
        format!(
            "The profile limitations of the semantic table {table_id} (sha256 {table_sha256}) are bound by reference and are not restated in this report."
        ),
    ));
    Ok(entries)
}

fn first_difference(expected: &ExactRouteSection, found: &ExactRouteSection) -> &'static str {
    let (a, b) = (&expected.admission, &found.admission);
    let admission = [
        ("admission.semantic_contract_id", a.semantic_contract_id == b.semantic_contract_id),
        ("admission.run_ref", a.run_ref == b.run_ref),
        ("admission.result_envelope_id", a.result_envelope_id == b.result_envelope_id),
        ("admission.received_result_sha256", a.received_result_sha256 == b.received_result_sha256),
        ("admission.model_sha256", a.model_sha256 == b.model_sha256),
        ("admission.input_manifest_ref", a.input_manifest_ref == b.input_manifest_ref),
        ("admission.input_manifest_sha256", a.input_manifest_sha256 == b.input_manifest_sha256),
        ("admission.derivative_sha256", a.derivative_sha256 == b.derivative_sha256),
    ];
    if expected.semantic_contract_id != found.semantic_contract_id {
        return "semantic_contract_id";
    }
    if let Some((field, _)) = admission.iter().find(|(_, equal)| !equal) {
        return field;
    }
    if expected.load_reference_states != found.load_reference_states {
        return "load_reference_states";
    }
    if expected.headlines != found.headlines {
        return "headlines";
    }
    "exact_route"
}

/// Item 9 (§3.5), after item 8. A document with an exact identity requires the
/// report's `exact_route`, equal to the expected section, and its limitation
/// entries, after the technical-preview entry, equal to the expected entries,
/// with that entry's effect and provenance. Any other identity carries neither
/// `exact_route` nor an exact limitation entry.
pub fn check_exact_sections(
    source: &Value,
    document: &Value,
    audit_manifest: &audit::AuditManifest,
    report: &Value,
) -> Result<(), String> {
    let exact = producer_contract_id(document).is_some_and(is_exact_identity);
    let carried = report.get("exact_route").filter(|value| !value.is_null());
    let limitations = report.pointer("/report_sections/limitations").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    if !exact {
        if carried.is_some() {
            return Err(mismatch("exact_route: the result is not an exact-route result"));
        }
        if limitations.iter().any(|item| item["limitation_id"].as_str().is_some_and(|id| id.contains(":exact:"))) {
            return Err(mismatch("limitations: the result is not an exact-route result"));
        }
        return Ok(());
    }
    let carried = carried.ok_or_else(|| mismatch("exact_route: an exact-route result requires the report's exact_route"))?;
    let found: ExactRouteSection =
        serde_json::from_value(carried.clone()).map_err(|error| {
            // The error kind only, never the offending value (Codex review round 3 of #1639).
            mismatch(format!("exact_route: {} in the report's exact_route", open_pipe_stress_report_renderer::serde_error_kind(&error.to_string())))
        })?;
    let expected = expected_exact_route_section(
        source,
        document,
        audit_manifest,
        &found.admission.native_job_id,
        &found.admission.received_bytes_sha256,
    )?;
    if expected != found {
        return Err(mismatch(first_difference(&expected, &found)));
    }

    let run = &expected.admission.run_ref;
    let entries = expected_exact_limitations(source, run)?;
    check_exact_limitation_entries(&entries, run, limitations).map_err(|detail| mismatch(format!("limitations: {detail}")))
}

/// The report's limitation entries against the expected exact entries (§2.4):
/// the technical-preview entry of `run` first, then exactly the expected
/// entries, each sharing the technical-preview entry's effect and provenance
/// and carrying no other member. `Err` names the first difference. Item 9
/// uses it, and so does the desktop's native render re-derivation (§3.6).
pub fn check_exact_limitation_entries(entries: &[ExpectedLimitation], run: &str, limitations: &[Value]) -> Result<(), String> {
    let Some((preview, rest)) = limitations.split_first() else {
        return Err("the technical-preview entry is missing".to_string());
    };
    if preview["limitation_id"].as_str() != Some(format!("limitation:{run}:technical-preview").as_str()) {
        return Err("the technical-preview entry is not first".to_string());
    }
    if rest.len() != entries.len() {
        return Err("the exact limitation entries are not the source's".to_string());
    }
    for (item, entry) in rest.iter().zip(entries) {
        let equal = item["limitation_id"].as_str() == Some(entry.limitation_id.as_str())
            && item["source"] == entry.source
            && item["affected_scope"] == entry.affected_scope
            && item["statement"].as_str() == Some(entry.statement.as_str())
            && item["effect"] == preview["effect"]
            && item["provenance"] == preview["provenance"]
            && item.as_object().is_some_and(|object| object.len() == 6);
        if !equal {
            return Err(entry.limitation_id.clone());
        }
    }
    Ok(())
}
