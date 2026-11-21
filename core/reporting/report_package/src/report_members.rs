//! The native producers of a report input's row-scale members (DEL-08-01).
//!
//! A report input has members whose size follows the result's row count: the
//! result rows, the result export references, the result's Diagnostics
//! entries, the unit display summary's result units and the report context's
//! result coordinate systems. The desktop's builder
//! (`apps/desktop/src/features/report/renderableReportInput.ts`) writes them
//! from a whole result. The functions here produce the same members from the
//! same result, so that a host which holds the result can write them itself
//! and take from its caller only the members that do not follow the rows.
//!
//! Each producer's output equals the desktop builder's in canonical form
//! (RFC 8785). The rows and the Diagnostics are the projections the request
//! checks already apply ([`crate::bindings::expected_report_result_rows`],
//! [`crate::bindings::expected_report_diagnostics`]); the references carry the
//! row digest [`crate::bindings::check_report_result_refs`] requires. Parity:
//! `tests/report_members_parity.rs`, on report inputs the desktop's builder
//! wrote (`fixtures/reports/invented/exact_report_package_requests/`,
//! `fixtures/reports/invented/user_release_exact/` and
//! `fixtures/reports/invented/user_release_derivation_vectors.json`).
//!
//! Nothing here binds a source to a job, decides what may leave, or renders:
//! the caller has bound the result before it derives from it. Standard claim
//! fence applies (claims taxonomy per DEC-081).

use crate::bindings::{expected_report_diagnostics, expected_report_result_rows, BINDING_MISMATCH, SOURCE_INVALID};
use crate::quantity_display::{source_displays_scaled_value, QUANTITY_DISPLAY_POLICY};
use crate::user_release::{current_received_origin, current_result_document_base, js_trim, report_envelope, session_provenance, sorted_unique};
use open_pipe_stress_result_export::derivative::{derive_document, digest, reference};
use serde_json::{json, Value};

/// A report input offered to [`place_members`] that already carries a member
/// the host derives, or that has no place for one.
pub const MEMBERS_NOT_PLACED: &str = "REPORT-MEMBERS-NOT-PLACED";

const RESULTS_SCHEMA: &str = "schemas/results.schema.yaml";
const STORAGE_CONVENTION: &str = "entered_units_preserved";

fn mismatch(what: impl std::fmt::Display) -> String {
    format!("{BINDING_MISMATCH}: {what}")
}

fn not_placed(what: impl std::fmt::Display) -> String {
    format!("{MEMBERS_NOT_PLACED}: {what}")
}

fn source_rows(source: &Value) -> Result<&Vec<Value>, String> {
    source["results"].as_array().ok_or_else(|| mismatch("the source has no result rows"))
}

/// The report's result rows: one per source row, in order
/// ([`expected_report_result_rows`], the report-row contract).
pub fn result_rows(source: &Value) -> Result<Vec<Value>, String> {
    expected_report_result_rows(source)
}

/// The report's `calculation_report.result_export_refs`: one entry per source
/// row, in order, naming the row and carrying the SHA-256 of its canonical
/// form, with the results schema reference, the private project class and the
/// session `provenance`. The desktop's builder reads the same reference and
/// digest from the run record's `result_refs`, which that record derives from
/// the row; this reads the row.
pub fn result_export_refs(source: &Value, provenance: &Value) -> Result<Vec<Value>, String> {
    source_rows(source)?
        .iter()
        .map(|row| {
            let id = row["id"].as_str().ok_or_else(|| mismatch("source row id"))?;
            let row_digest = digest(row).map_err(|detail| format!("{SOURCE_INVALID}: {detail}"))?;
            Ok(report_envelope(reference("Result", id), RESULTS_SCHEMA, Some(&row_digest), provenance))
        })
        .collect()
}

/// The report's entries for the result's diagnostics, in source order
/// ([`expected_report_diagnostics`]), each with the session `provenance`: the
/// first entries of `report_sections.diagnostics`.
pub fn result_diagnostics(source: &Value, provenance: &Value) -> Result<Vec<Value>, String> {
    Ok(expected_report_diagnostics(source)?
        .into_iter()
        .map(|mut entry| {
            entry["provenance"] = provenance.clone();
            entry
        })
        .collect())
}

/// The report's `unit_display_summary`: the storage convention, the model's
/// project units, the source rows' stored units (each once, in UTF-16 code
/// unit order, as the desktop sorts them), the display policy and
/// `conversion_performed`.
pub fn unit_display_summary(model: &Value, source: &Value) -> Result<Value, String> {
    let units = model.pointer("/project/units").filter(|units| units.is_object()).ok_or_else(|| mismatch("the model's project units"))?;
    let result_units = sorted_unique(source_rows(source)?.iter().filter_map(|row| row["unit"].as_str()).filter(|unit| !unit.is_empty()).map(str::to_string));
    Ok(json!({
        "storage_convention": STORAGE_CONVENTION,
        "model_units": units,
        "result_units": result_units,
        "quantity_display_policy": QUANTITY_DISPLAY_POLICY,
        "conversion_performed": source_displays_scaled_value(source)
    }))
}

/// The report context's `result_coordinate_systems`: each coordinate system a
/// source row's metadata names, once, in UTF-16 code unit order. A name that
/// is blank after JavaScript's `trim`, or `TBD`, is not one.
pub fn result_coordinate_systems(source: &Value) -> Result<Vec<String>, String> {
    Ok(sorted_unique(
        source_rows(source)?
            .iter()
            .filter_map(|row| row.pointer("/metadata/coordinate_system").and_then(Value::as_str))
            .filter(|system| !js_trim(system).is_empty() && *system != "TBD")
            .map(str::to_string),
    ))
}

fn member<'a>(report: &'a mut Value, pointer: &str) -> Result<&'a mut serde_json::Map<String, Value>, String> {
    report.pointer_mut(pointer).and_then(Value::as_object_mut).ok_or_else(|| not_placed(format_args!("the report input has no {pointer} object")))
}

fn place(object: &mut serde_json::Map<String, Value>, key: &str, value: Value, path: &str) -> Result<(), String> {
    match object.get(key) {
        None | Some(Value::Null) => {
            object.insert(key.to_string(), value);
            Ok(())
        }
        Some(_) => Err(not_placed(format_args!("the report input already carries {path}"))),
    }
}

/// Writes the row-scale members into a report input that came without them:
/// `result_rows`, `calculation_report.result_export_refs`, the unit display
/// summary, `report_context.result_coordinate_systems`, and the result's
/// Diagnostics entries before the entries `report_sections.diagnostics`
/// already holds (the session's, which follow the result's). The provenance
/// is the session's for `model` ([`session_provenance`]).
///
/// A report input that already carries one of the four members refuses
/// ([`MEMBERS_NOT_PLACED`]), naming the member and no value: a host that
/// derives a member does not also take it from its caller. The Diagnostics
/// cannot be told apart this way; the caller's own check of the session
/// entries follows. `report_section_refs` is not written: see
/// [`seal_report_sections`].
pub fn place_members(report: &mut Value, model: &Value, source: &Value) -> Result<(), String> {
    let provenance = session_provenance(model)?;
    let rows = result_rows(source)?;
    let refs = result_export_refs(source, &provenance)?;
    let mut diagnostics = result_diagnostics(source, &provenance)?;
    let units = unit_display_summary(model, source)?;
    let systems = result_coordinate_systems(source)?;
    if !report.is_object() {
        return Err(not_placed("the report input is not an object"));
    }
    place(member(report, "")?, "result_rows", Value::Array(rows), "result_rows")?;
    place(member(report, "/calculation_report")?, "result_export_refs", Value::Array(refs), "calculation_report.result_export_refs")?;
    place(
        member(report, "/calculation_report/model_input_summary")?,
        "unit_display_summary",
        units,
        "calculation_report.model_input_summary.unit_display_summary",
    )?;
    place(member(report, "/report_context")?, "result_coordinate_systems", json!(systems), "report_context.result_coordinate_systems")?;
    let session = report
        .pointer_mut("/report_sections/diagnostics")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| not_placed("the report input has no report_sections.diagnostics list"))?;
    diagnostics.append(session);
    *session = diagnostics;
    Ok(())
}

/// Writes the checksum of the completed `report_sections` into the report's
/// one `calculation_report.report_section_refs` entry: the SHA-256 of the
/// sections' canonical form, as the desktop's builder computes it once every
/// section member is in place. Called after [`place_members`] and after any
/// other member the host writes into the sections.
pub fn seal_report_sections(report: &mut Value) -> Result<(), String> {
    let sections = report.get("report_sections").filter(|sections| sections.is_object()).ok_or_else(|| not_placed("the report input has no report_sections object"))?;
    let checksum = digest(sections).map_err(|detail| format!("{SOURCE_INVALID}: {detail}"))?;
    let refs = report
        .pointer_mut("/calculation_report/report_section_refs")
        .and_then(Value::as_array_mut)
        .filter(|refs| refs.len() == 1)
        .ok_or_else(|| not_placed("the report input has no single calculation_report.report_section_refs entry"))?;
    let entry = refs[0].get_mut("checksum").and_then(Value::as_object_mut).ok_or_else(|| not_placed("the report section reference has no checksum"))?;
    entry.insert("value".to_string(), Value::String(checksum));
    Ok(())
}

/// The result export document the desktop's assembly builds for a received
/// result (`buildCurrentResultExport`): the document base from the run record
/// and the input manifest's reference and solver basis, the received-carrier
/// origin, and the derivative of `source`. Of the run record it reads the
/// `analysis_run` header and the count of its diagnostics, not its
/// `result_refs`.
pub fn result_document(model: &Value, source: &Value, run_record: &Value, manifest_ref: &str, solver_basis: &Value) -> Result<Value, String> {
    let origin = current_received_origin(model, source, manifest_ref)?;
    let base = current_result_document_base(model, source, run_record, manifest_ref, solver_basis)?;
    derive_document(base, model, source, origin, None)
}

/// The derivative hash a result export `document` states, which an exact
/// report's admission binds as `derivative_sha256`.
pub fn derivative_sha256(document: &Value) -> Result<&str, String> {
    document
        .pointer("/result_envelope/reproducibility/derivative_hash/value")
        .and_then(Value::as_str)
        .ok_or_else(|| mismatch("the result document states no derivative hash"))
}
