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
//! (RFC 8785), for a result the product reader admits and a run record made
//! for it. Outside that domain the two differ, and the native side refuses:
//! a row with no id, a unit or a diagnostic reference that is not text, and a
//! number the checked canonical form does not admit are each written out in
//! some form by the builder and refused here. One difference is not a
//! refusal: given a run record entry with no row digest the builder writes
//! `TBD`, and the producer writes the row's digest.
//!
//! The rows and the Diagnostics are the projections the request checks
//! already apply ([`crate::bindings::expected_report_result_rows`],
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
/// `conversion_performed`. A row with no unit, or an empty one, adds none; a
/// unit that is not text refuses.
pub fn unit_display_summary(model: &Value, source: &Value) -> Result<Value, String> {
    let units = model.pointer("/project/units").filter(|units| units.is_object()).ok_or_else(|| mismatch("the model's project units"))?;
    let mut stored = Vec::new();
    for row in source_rows(source)? {
        match row.get("unit") {
            None | Some(Value::Null) => {}
            Some(Value::String(unit)) if unit.is_empty() => {}
            Some(Value::String(unit)) => stored.push(unit.clone()),
            Some(_) => return Err(mismatch("a source row's unit is not text")),
        }
    }
    let result_units = sorted_unique(stored.into_iter());
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

const SECTION_REFS: &str = "/calculation_report/report_section_refs";

/// The four members [`place_members`] writes: the object that holds each, its
/// key, and its name in a refusal.
const PLACED: [(&str, &str, &str); 4] = [
    ("", "result_rows", "result_rows"),
    ("/calculation_report", "result_export_refs", "calculation_report.result_export_refs"),
    ("/calculation_report/model_input_summary", "unit_display_summary", "calculation_report.model_input_summary.unit_display_summary"),
    ("/report_context", "result_coordinate_systems", "report_context.result_coordinate_systems"),
];

/// The report's one `report_section_refs` entry's checksum object.
fn sections_checksum(report: &mut Value) -> Result<&mut serde_json::Map<String, Value>, String> {
    report
        .pointer_mut(SECTION_REFS)
        .and_then(Value::as_array_mut)
        .filter(|refs| refs.len() == 1)
        .and_then(|refs| refs[0].get_mut("checksum"))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| not_placed("the report input has no single calculation_report.report_section_refs entry with a checksum"))
}

/// A Diagnostics entry's code and affected object id, when both are text.
fn copy_key(entry: &Value) -> Option<(&str, &str)> {
    Some((entry.get("code")?.as_str()?, entry.get("affected_object")?.get("ref_id")?.as_str()?))
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
/// derives a member does not also take it from its caller. So does one with
/// no place for a member. Every refusal is decided before anything is
/// written, so a refused input is unchanged.
///
/// An entry a caller sends in `report_sections.diagnostics` with the code and
/// the affected object of one of the result's entries is a copy of it and
/// refuses, naming its index only: the rule the desktop host's
/// `report_session_diagnostics` applies to an entry after the result's. The
/// caller's other entries are kept as sent, after the result's. Whether they
/// are the session's is not examined here: that is the host's check, which
/// the exact-route and pressure-free Timoshenko bindings run (DEL-08-01's
/// scope records that other pressure-free reports have none). The generic
/// route's numerical-health lines, which the desktop appends to
/// `result_rows`, are not placed here.
///
/// The completed sections no longer have the checksum the input stated, so
/// its value is removed: a report input placed and not sealed
/// ([`seal_report_sections`]) does not decode as a renderer input.
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
    for (pointer, key, path) in PLACED {
        let object = report.pointer(pointer).and_then(Value::as_object).ok_or_else(|| not_placed(format_args!("the report input has no {pointer} object")))?;
        if object.get(key).is_some_and(|value| !value.is_null()) {
            return Err(not_placed(format_args!("the report input already carries {path}")));
        }
    }
    let session = report
        .pointer("/report_sections/diagnostics")
        .and_then(Value::as_array)
        .ok_or_else(|| not_placed("the report input has no report_sections.diagnostics list"))?;
    let of_the_result: std::collections::HashSet<(&str, &str)> = diagnostics.iter().filter_map(copy_key).collect();
    if let Some(index) = session.iter().position(|entry| copy_key(entry).is_some_and(|key| of_the_result.contains(&key))) {
        return Err(not_placed(format_args!("report_sections.diagnostics entry {index} copies a result diagnostic's code and affected object")));
    }
    drop(of_the_result);
    sections_checksum(report)?;
    // Nothing below refuses: every place was found above.
    for ((pointer, key, _), value) in PLACED.into_iter().zip([Value::Array(rows), Value::Array(refs), units, json!(systems)]) {
        if let Some(object) = report.pointer_mut(pointer).and_then(Value::as_object_mut) {
            object.insert(key.to_string(), value);
        }
    }
    if let Some(session) = report.pointer_mut("/report_sections/diagnostics").and_then(Value::as_array_mut) {
        diagnostics.append(session);
        *session = diagnostics;
    }
    sections_checksum(report)?.remove("value");
    Ok(())
}

/// Writes the checksum of the completed `report_sections` into the report's
/// one `calculation_report.report_section_refs` entry: the SHA-256 of the
/// sections' canonical form, as the desktop's builder computes it once every
/// section member is in place. Called after [`place_members`] and after any
/// other member the host writes into the sections.
///
/// The entry must be the builder's: it names these sections
/// (`report_sections`, their `report_section_id`), and its checksum states
/// SHA-256 over RFC 8785 for that same reference. Any other entry refuses and
/// nothing is written.
pub fn seal_report_sections(report: &mut Value) -> Result<(), String> {
    let sections = report.get("report_sections").filter(|sections| sections.is_object()).ok_or_else(|| not_placed("the report input has no report_sections object"))?;
    let id = sections["report_section_id"].as_str().ok_or_else(|| not_placed("the report sections have no report_section_id"))?;
    let named = reference("report_sections", id);
    let checksum = digest(sections).map_err(|detail| format!("{SOURCE_INVALID}: {detail}"))?;
    let entry = report.pointer(SECTION_REFS).and_then(Value::as_array).filter(|refs| refs.len() == 1).map(|refs| &refs[0]);
    let stated = entry.and_then(|entry| entry.get("checksum"));
    let is_the_builders = entry.is_some_and(|entry| entry["ref"] == named)
        && stated.is_some_and(|stated| stated["algorithm"] == "sha256" && stated["canonicalization"] == "rfc8785_jcs" && stated["payload_ref"] == named);
    if !is_the_builders {
        return Err(not_placed("the report input's calculation_report.report_section_refs is not one SHA-256 RFC 8785 reference to its report_sections"));
    }
    sections_checksum(report)?.insert("value".to_string(), Value::String(checksum));
    Ok(())
}

/// The result export document the desktop's assembly builds for a received
/// result (`buildCurrentResultExport`): the document base from the run record
/// and the input manifest's reference and solver basis, the received-carrier
/// origin, and the derivative of `source`. Of the run record it reads the
/// `analysis_run` header and the count of its diagnostics, not its
/// `result_refs`.
///
/// Not every run the desktop's assembly accepts: a run record that carries a
/// diagnostic the source does not (a rule-pack reference exclusion) is refused
/// by the document base (`REPORT-USER-RELEASE-BASIS-INVALID`), where the
/// desktop's assembly adds a document diagnostic for it.
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
