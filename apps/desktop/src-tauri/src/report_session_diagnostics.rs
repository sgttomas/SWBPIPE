//! The session's Diagnostics entries in an exact-route or pressure-free
//! Timoshenko render, bound by the host (review of f3d02469c, finding 2).
//!
//! A bound render's `report_sections.diagnostics` opens with the projection
//! of the delivered result's diagnostics (`check_report_diagnostics`). The
//! entries after them are the session's, and no caller-supplied member (a
//! class, a reference type, a remediation text, an origin tag) can establish
//! that an entry is the session's: the host derives every entry the session
//! may append from what it holds, and admits an appended entry only when it
//! equals one of them on its code, class, severity, source, affected object,
//! message and remediation (the provenance is the session record, unbound).
//! An appended entry with a result diagnostic's code and affected object is
//! refused first, whatever its prose. The kinds, mirroring the desktop
//! builder (`apps/desktop/src/features/report/renderableReportInput.ts`
//! `buildRenderableReportInput`), each derived from the bound intake model:
//! - `COMPONENT_PROVENANCE_MISSING` for each component without provenance
//!   (`componentProvenanceDiagnostics`);
//! - `SPRING_HANGER_PROVENANCE_MISSING` for each spring-hanger or
//!   constant-effort support without `provenance` or `hanger.source_reference`
//!   (`springHangerDiagnostics`, `springHangerSupport.ts`);
//! - `MATERIAL_PROVENANCE_MISSING` and `SECTION_PROVENANCE_MISSING` for each
//!   material and section without provenance (`subjectProvenance`);
//! - a library store finding (`libraryFindingDiagnostics`):
//!   `MODEL-LIBRARY-REF-UNRESOLVED` or `MODEL-LIBRARY-REF-VERSION-CHANGED` on
//!   an entity that carries a `library_ref` in the bound model, with one of
//!   the store check's messages for that reference (`lib.rs`
//!   `library_ref_findings`). Whether the store holds the library is the one
//!   fact the render does not re-check: it has no store.
//!
//! Applied by both bound render commands, at their shared admission
//! (`report_render_exact::bind_parsed_admission`). Standard claim fence
//! applies (claims taxonomy per DEC-081).

use open_pipe_stress_report_package::bindings::expected_report_diagnostics;
use serde_json::{json, Map, Value};

/// The library store check's diagnostic source (desktop
/// `features/library/libraryRefFindings.ts` `LIBRARY_REF_CHECK_SOURCE`).
pub(crate) const LIBRARY_REF_CHECK_SOURCE: &str = "apps/desktop/src-tauri/src/lib.rs check_model_library_refs";
const PROVENANCE_WARNING: &str = "PROVENANCE_WARNING";
/// The fields an appended entry is compared on (the provenance is the session's).
const FIELDS: [&str; 7] = ["code", "class", "severity", "source", "affected_object", "message", "remediation"];

/// JavaScript's `String.prototype.trim` (the builder's): WhiteSpace and
/// LineTerminator, so U+FEFF is removed and U+0085 kept, unlike Rust's.
fn js_trim(text: &str) -> &str {
    let space = |c: char| {
        matches!(c, '\u{9}' | '\u{a}' | '\u{b}' | '\u{c}' | '\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
    };
    text.trim_matches(space)
}

/// The builder's `text`: a string non-empty after trimming, trimmed.
fn text(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str).map(js_trim).filter(|text| !text.is_empty())
}

/// The builder's `hasProvenance`: `provenance_record.source_name`, or a
/// non-empty `provenance` note (a string after trimming, or an object with a
/// member).
fn has_provenance(entity: &Value) -> bool {
    if text(entity.get("provenance_record").filter(|record| record.is_object()).and_then(|record| record.get("source_name"))).is_some() {
        return true;
    }
    match entity.get("provenance") {
        Some(Value::String(note)) => !js_trim(note).is_empty(),
        Some(Value::Object(note)) => !note.is_empty(),
        _ => false,
    }
}

/// `springHangerSupport.ts` `isSpringHangerSupport` (Rust trimming, as there).
fn is_spring_hanger(support: &Value) -> bool {
    let explicit = support.pointer("/hanger/hanger_type").and_then(Value::as_str).map(str::trim).unwrap_or("");
    let kind = if explicit.is_empty() { support.get("family").and_then(Value::as_str).map(str::trim) } else { Some(explicit) };
    matches!(kind, Some("variable_spring_hanger" | "spring_hanger" | "constant_effort_support"))
}

/// The JavaScript `a || b` of the builder on optional strings.
fn or_id<'a>(label: Option<&'a Value>, id: &'a str) -> &'a str {
    label.and_then(Value::as_str).filter(|label| !label.is_empty()).unwrap_or(id)
}

fn reference(ref_type: &str, ref_id: &str) -> Value {
    json!({"ref_type": ref_type, "ref_id": ref_id})
}

fn entry(code: &str, source: Value, affected: Value, message: String, remediation: &str) -> Value {
    json!({
        "code": code, "class": PROVENANCE_WARNING, "severity": "warning", "source": source,
        "affected_object": affected, "message": message, "remediation": remediation,
    })
}

fn items<'a>(model: &'a Value, member: &str) -> impl Iterator<Item = &'a Value> {
    model.get(member).and_then(Value::as_array).into_iter().flatten()
}

fn id(entity: &Value) -> &str {
    entity.get("id").and_then(Value::as_str).unwrap_or("")
}

/// The model's `library_ref` sites with their owning entity, in the
/// builder's walk (`libraryImportService.ts` `libraryRefSites`).
fn library_ref_sites<'a>(value: &'a Value, path: &str, owner: Option<&'a str>, out: &mut Vec<(String, &'a Map<String, Value>)>) {
    match value {
        Value::Array(entries) => {
            for (index, entry) in entries.iter().enumerate() {
                library_ref_sites(entry, &format!("{path}[{index}]"), owner, out);
            }
        }
        Value::Object(object) => {
            let named = object.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).or(owner);
            for (key, child) in object {
                let at = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                if key == "library_ref" {
                    if let Value::Object(reference) = child {
                        out.push((named.map_or_else(|| at.clone(), str::to_string), reference));
                    }
                }
                library_ref_sites(child, &at, named, out);
            }
        }
        _ => {}
    }
}

/// Every entry the session may append for `model`, by the kinds above.
pub(crate) fn session_diagnostics(model: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    for component in items(model, "components").filter(|component| !has_provenance(component)) {
        let id = id(component);
        out.push(entry(
            "COMPONENT_PROVENANCE_MISSING",
            reference("component", id),
            reference("component", id),
            format!("Component {} has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component.", or_id(component.get("label"), id)),
            "Enter component source/provenance before relying on this report.",
        ));
    }
    for support in items(model, "supports").filter(|support| is_spring_hanger(support)) {
        let blank = |value: Option<&Value>| value.and_then(Value::as_str).is_none_or(|text| js_trim(text).is_empty());
        if blank(support.get("provenance")) || blank(support.pointer("/hanger/source_reference")) {
            let id = id(support);
            out.push(entry(
                "SPRING_HANGER_PROVENANCE_MISSING",
                reference("support", id),
                reference("support", id),
                format!("Spring hanger {} is missing support provenance or hanger.source_reference.", or_id(support.get("label"), id)),
                "Enter spring-hanger source/provenance before relying on this report.",
            ));
        }
    }
    for (kind, member, label, code, title) in
        [("material", "materials", "label", "MATERIAL_PROVENANCE_MISSING", "Material"), ("section", "sections", "name", "SECTION_PROVENANCE_MISSING", "Section")]
    {
        for subject in items(model, member).filter(|subject| !has_provenance(subject)) {
            let id = id(subject);
            out.push(entry(
                code,
                reference(kind, id),
                reference(kind, id),
                format!("{title} {} has neither a provenance_record nor a provenance note; report provenance is incomplete for this {kind}.", text(subject.get(label)).unwrap_or(id)),
                &format!("Enter {kind} source/provenance before relying on this report."),
            ));
        }
    }
    let mut sites = Vec::new();
    library_ref_sites(model, "", None, &mut sites);
    let project = model.pointer("/project/id").and_then(Value::as_str).unwrap_or("").trim();
    for (entity, site) in sites {
        let member = |key: &str| site.get(key).and_then(Value::as_str);
        let (kind, library) = (member("library_kind").unwrap_or(""), member("library_id").unwrap_or(""));
        let finding = |code: &str, message: String| {
            entry(
                code,
                reference("diagnostic_source", LIBRARY_REF_CHECK_SOURCE),
                reference("model_entity", &entity),
                message,
                "Compare the library reference with the local library store before relying on this report.",
            )
        };
        out.push(finding("MODEL-LIBRARY-REF-UNRESOLVED", format!(
            "{entity} names {kind} library `{library}`, which the local store does not hold for project `{project}`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project."
        )));
        out.push(finding("MODEL-LIBRARY-REF-UNRESOLVED", format!(
            "{entity} names record `{}` of {kind} library `{library}`, which the stored library does not hold. The model's values stand.",
            member("record_id").unwrap_or("(none recorded)")
        )));
        if member("library_version").is_some() {
            out.push(finding("MODEL-LIBRARY-REF-VERSION-CHANGED", format!(
                "The stored record `{}` of {kind} library `{library}` has changed since {entity} recorded it. The model's values and recorded version stand; re-apply from the library to adopt the stored record.",
                member("record_id").unwrap_or("")
            )));
        }
    }
    out
}

fn affected_id(entry: &Value) -> Option<&Value> {
    entry.get("affected_object").and_then(|object| object.get("ref_id"))
}

/// The index, among `appended`, of the first entry with a code and affected
/// object of one of `results` (the result's projected diagnostics).
pub(crate) fn first_copy_of_a_result_diagnostic(appended: &[Value], results: &[Value]) -> Option<usize> {
    appended.iter().position(|entry| results.iter().any(|result| result.get("code") == entry.get("code") && affected_id(result) == affected_id(entry)))
}

/// The binding (module docs): every entry of `report_sections.diagnostics`
/// after the result's projected entries is one the host derives from `model`.
/// `Err` names the entry's index only.
pub(crate) fn check_session_diagnostics(input: &Value, source: &Value, model: &Value) -> Result<(), String> {
    let results = expected_report_diagnostics(source)?;
    let found = input.pointer("/report_sections/diagnostics").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    let appended = found.get(results.len()..).unwrap_or(&[]);
    if let Some(index) = first_copy_of_a_result_diagnostic(appended, &results) {
        return Err(format!("report diagnostic {} copies a result diagnostic's code and affected object", results.len() + index));
    }
    if appended.is_empty() {
        return Ok(());
    }
    let derived = session_diagnostics(model);
    for (index, entry) in appended.iter().enumerate() {
        if !derived.iter().any(|candidate| FIELDS.iter().all(|field| entry.get(*field) == candidate.get(*field))) {
            return Err(format!("report diagnostic {} is not a session diagnostic the host derives from the bound model", results.len() + index));
        }
    }
    Ok(())
}
