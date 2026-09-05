//! Library-import provenance validation seam (runtime port of DEL-03-07).
//!
//! This crate validates already-parsed material, section, and component
//! library import payloads at the import boundary. It is the runtime Rust
//! port of `core/library_import/provenance_checker.py` (DEL-03-07): the Python
//! module is the authored design contract, and this crate reproduces its
//! semantics so the Tauri/Rust desktop runtime can enforce the same import
//! boundary the rest of the application is built on. Cross-language parity is
//! pinned by the shared invented fixtures under `fixtures/` (see
//! `tests/provenance_parity.rs`).
//!
//! Like the Python contract, this crate does **not** parse external file
//! formats and does **not** make legal license or redistribution
//! determinations. Unresolved rights remain review findings for the human
//! project authority. Nothing here is a professional, certification, sealing,
//! authentication, or code-compliance claim — every status is a software
//! finding only.

use serde::Serialize;
use serde_json::{Map, Value};

/// Provenance fields every imported library object and record must carry
/// before an import can be accepted. Mirrors `REQUIRED_PROVENANCE_FIELDS` in
/// the Python contract.
pub const REQUIRED_PROVENANCE_FIELDS: [&str; 7] = [
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
];

const PUBLIC_OK_REDIS_STATUS: &str = "public_permissive";
const PRIVATE_REDIS_STATUS: &str = "private_only";
const PROTECTED_REDIS_STATUS: &str = "protected_suspected";

/// Source identifier stamped onto projected diagnostics. Honestly names the
/// runtime implementation that emitted the finding (the Python contract stamps
/// its own module path); semantic parity is on codes, severities, paths, and
/// outcomes, not on this identity string.
pub const DIAGNOSTIC_SOURCE: &str = "core.library_import.library_import_document";
const DIAGNOSTIC_CLASS: &str = "import_boundary";

/// Which library family a payload declares. Mirrors the Python `RECORD_KEYS`
/// dispatch table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LibraryKind {
    Material,
    Section,
    Component,
    Hanger,
}

impl LibraryKind {
    /// Parse the externally supplied `library_kind` token. Returns `None` for
    /// unsupported kinds (the Python contract raises `ValueError`; callers at
    /// the seam should reject unknown kinds rather than guess).
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "material" => Some(Self::Material),
            "section" => Some(Self::Section),
            "component" => Some(Self::Component),
            "hanger" => Some(Self::Hanger),
            _ => None,
        }
    }

    /// The `(library_metadata_key, records_array_key)` pair for this kind.
    fn keys(self) -> (&'static str, &'static str) {
        match self {
            Self::Material => ("material_library", "material_records"),
            Self::Section => ("section_library", "section_records"),
            Self::Component => ("component_library", "component_records"),
            Self::Hanger => ("hanger_library", "hanger_records"),
        }
    }

    /// The canonical lowercase token for this kind (round-trips with
    /// [`LibraryKind::from_token`]).
    pub fn token(self) -> &'static str {
        match self {
            Self::Material => "material",
            Self::Section => "section",
            Self::Component => "component",
            Self::Hanger => "hanger",
        }
    }
}

/// Whether the importing user intends the data to become public-bundled data
/// or to remain private/local. Mirrors the Python `intended_visibility`
/// argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntendedVisibility {
    Public,
    Private,
}

impl IntendedVisibility {
    /// Parse the externally supplied `intended_visibility` token.
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "public" => Some(Self::Public),
            "private" => Some(Self::Private),
            _ => None,
        }
    }
}

/// A single import-boundary finding. Mirrors the Python `ImportFinding`
/// dataclass: `code`/`severity`/`path`/`message`/`remediation` are the
/// semantic contract; `diagnostic_class`/`source` identify the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportFinding {
    pub code: String,
    /// `blocking` | `quarantine` | `review_required`.
    pub severity: String,
    pub path: String,
    pub message: String,
    pub remediation: String,
}

impl ImportFinding {
    fn new(code: &str, severity: &str, path: &str, message: &str, remediation: &str) -> Self {
        Self {
            code: code.to_string(),
            severity: severity.to_string(),
            path: path.to_string(),
            message: message.to_string(),
            remediation: remediation.to_string(),
        }
    }

    /// Project this finding into the PKG-02 import-boundary diagnostic
    /// envelope shape. Mirrors `ImportFinding.to_diagnostic`.
    pub fn to_diagnostic(&self) -> Value {
        let mut provenance = Map::new();
        provenance.insert(
            "source_name".to_string(),
            Value::String("library_import_payload".to_string()),
        );
        provenance.insert(
            "source_location".to_string(),
            Value::String(self.path.clone()),
        );
        provenance.insert(
            "review_status".to_string(),
            Value::String("diagnostic_generated".to_string()),
        );

        let mut envelope = Map::new();
        envelope.insert("code".to_string(), Value::String(self.code.clone()));
        envelope.insert("severity".to_string(), Value::String(self.severity.clone()));
        envelope.insert(
            "class".to_string(),
            Value::String(DIAGNOSTIC_CLASS.to_string()),
        );
        envelope.insert(
            "source".to_string(),
            Value::String(DIAGNOSTIC_SOURCE.to_string()),
        );
        envelope.insert(
            "affected_object".to_string(),
            Value::String(self.path.clone()),
        );
        envelope.insert("message".to_string(), Value::String(self.message.clone()));
        envelope.insert(
            "remediation".to_string(),
            Value::String(self.remediation.clone()),
        );
        envelope.insert("provenance".to_string(), Value::Object(provenance));
        Value::Object(envelope)
    }
}

/// The result of validating a library import payload. Mirrors the Python
/// `ImportValidationResult`: `outcome` plus the ordered findings, with
/// `accepted` and `diagnostics` projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportValidationResult {
    /// `QUARANTINE` | `REJECTED` | `REVIEW_REQUIRED` | `PRIVATE_LOCAL_ONLY` |
    /// `ACCEPTED_PUBLIC`.
    pub outcome: String,
    pub library_kind: LibraryKind,
    pub intended_visibility: IntendedVisibility,
    pub accepted: bool,
    pub findings: Vec<ImportFinding>,
}

impl ImportValidationResult {
    /// PKG-02 diagnostic-envelope projection of every finding.
    pub fn diagnostics(&self) -> Vec<Value> {
        self.findings.iter().map(ImportFinding::to_diagnostic).collect()
    }
}

/// Validate provenance and redistribution controls on a library payload.
///
/// `intended_visibility` is either public or private. Public imports must
/// carry accepted public-permissive provenance before they can be accepted.
/// Private imports may remain local/private, but still cannot bypass
/// protected-content quarantine or missing-provenance findings. Mirrors
/// `validate_library_import` in the Python contract.
pub fn validate_library_import(
    payload: &Value,
    library_kind: LibraryKind,
    intended_visibility: IntendedVisibility,
) -> ImportValidationResult {
    if library_kind == LibraryKind::Hanger {
        return validate_hanger_import(payload, intended_visibility);
    }
    let (library_key, records_key) = library_kind.keys();
    let mut findings: Vec<ImportFinding> = Vec::new();

    match payload.get(library_key).and_then(Value::as_object) {
        Some(library) => findings.extend(validate_object_disposition(
            library,
            library_key,
            intended_visibility,
        )),
        None => findings.push(ImportFinding::new(
            "IMPORT_LIBRARY_METADATA_MISSING",
            "blocking",
            library_key,
            "Library metadata object is missing.",
            "Provide library metadata with provenance before import.",
        )),
    }

    let records = match payload.get(records_key).and_then(Value::as_array) {
        Some(records) => records.as_slice(),
        None => {
            findings.push(ImportFinding::new(
                "IMPORT_RECORD_SET_MISSING",
                "blocking",
                records_key,
                "Library record array is missing.",
                "Provide records for the declared library kind before import.",
            ));
            &[]
        }
    };

    for (index, record) in records.iter().enumerate() {
        let record_path = format!("{records_key}[{index}]");
        match record.as_object() {
            Some(record) => {
                findings.extend(validate_object_disposition(
                    record,
                    &record_path,
                    intended_visibility,
                ));
                findings.extend(validate_nested_values(record, &record_path));
            }
            None => findings.push(ImportFinding::new(
                "IMPORT_RECORD_INVALID",
                "blocking",
                &record_path,
                "Library record is not an object.",
                "Provide an object with provenance and review metadata.",
            )),
        }
    }

    let outcome = determine_outcome(&findings, intended_visibility);
    let accepted = matches!(outcome.as_str(), "ACCEPTED_PUBLIC" | "PRIVATE_LOCAL_ONLY");

    ImportValidationResult {
        outcome,
        library_kind,
        intended_visibility,
        accepted,
        findings,
    }
}

/// Convenience entry point for the application seam: parse the externally
/// supplied tokens, then validate. Returns an `Err` with a stable code for an
/// unsupported `library_kind`/`intended_visibility` (the seam must reject,
/// never guess).
pub fn validate_library_import_tokens(
    payload: &Value,
    library_kind: &str,
    intended_visibility: &str,
) -> Result<ImportValidationResult, String> {
    let kind = LibraryKind::from_token(library_kind)
        .ok_or_else(|| format!("unsupported library_kind: {library_kind}"))?;
    let visibility = IntendedVisibility::from_token(intended_visibility)
        .ok_or_else(|| format!("unsupported intended_visibility: {intended_visibility}"))?;
    Ok(validate_library_import(payload, kind, visibility))
}

fn validate_object_disposition(
    item: &Map<String, Value>,
    path: &str,
    intended_visibility: IntendedVisibility,
) -> Vec<ImportFinding> {
    let mut findings: Vec<ImportFinding> = Vec::new();

    let provenance = match item.get("provenance").and_then(Value::as_object) {
        Some(provenance) => provenance,
        None => {
            findings.push(ImportFinding::new(
                "IMPORT_PROVENANCE_MISSING",
                "blocking",
                &format!("{path}.provenance"),
                "Required provenance object is missing.",
                "Record source, license, contributor, redistribution, and review metadata.",
            ));
            return findings;
        }
    };

    let mut missing: Vec<&str> = REQUIRED_PROVENANCE_FIELDS
        .iter()
        .copied()
        .filter(|field| !is_truthy(provenance.get(*field)))
        .collect();
    if !missing.is_empty() {
        missing.sort_unstable();
        findings.push(ImportFinding::new(
            "IMPORT_PROVENANCE_INCOMPLETE",
            "blocking",
            &format!("{path}.provenance"),
            &format!(
                "Required provenance fields are missing: {}.",
                missing.join(", ")
            ),
            "Complete required provenance fields before import acceptance.",
        ));
    }

    // `item.get(x) or provenance.get(x)` — the item-level field wins when
    // truthy, otherwise the provenance-level field is used.
    let redistribution_status = truthy_str(item.get("redistribution_status"))
        .or_else(|| truthy_str(provenance.get("redistribution_status")));
    let review_status = truthy_str(item.get("review_status"))
        .or_else(|| truthy_str(provenance.get("review_status")));
    let privacy_class = truthy_str(item.get("privacy_class"));

    if redistribution_status == Some(PROTECTED_REDIS_STATUS) || review_status == Some("quarantined")
    {
        findings.push(ImportFinding::new(
            "IMPORT_PROTECTED_CONTENT_SUSPECTED",
            "quarantine",
            path,
            "Import metadata indicates suspected protected content.",
            "Quarantine metadata and request human/legal review; do not publish values.",
        ));
    } else if redistribution_status == Some("rejected") || review_status == Some("rejected") {
        findings.push(ImportFinding::new(
            "IMPORT_REJECTED_SOURCE",
            "blocking",
            path,
            "Import metadata has a rejected source or review disposition.",
            "Reject this import or supply a reviewed source.",
        ));
    } else if intended_visibility == IntendedVisibility::Public {
        findings.extend(validate_public_disposition(
            path,
            redistribution_status,
            review_status,
            privacy_class,
        ));
    }

    findings
}

fn validate_public_disposition(
    path: &str,
    redistribution_status: Option<&str>,
    review_status: Option<&str>,
    privacy_class: Option<&str>,
) -> Vec<ImportFinding> {
    let mut findings: Vec<ImportFinding> = Vec::new();

    let privacy_is_private = privacy_class.is_some_and(|class| class.starts_with("private"));
    // UNKNOWN set is {"unknown", "TBD", None}: an absent/empty status counts.
    let redistribution_unknown = matches!(redistribution_status, None | Some("unknown") | Some("TBD"));

    if redistribution_status == Some(PRIVATE_REDIS_STATUS) || privacy_is_private {
        findings.push(ImportFinding::new(
            "IMPORT_PRIVATE_DATA_PUBLIC_BLOCKED",
            "blocking",
            path,
            "Private-only library data cannot be accepted as public data.",
            "Keep the import private or provide public-permissive reviewed provenance.",
        ));
    } else if redistribution_unknown {
        findings.push(ImportFinding::new(
            "IMPORT_REDIS_RIGHTS_MISSING",
            "blocking",
            path,
            "Redistribution rights are missing or unresolved for public import.",
            "Record public-permissive redistribution evidence and review disposition.",
        ));
    } else if redistribution_status != Some(PUBLIC_OK_REDIS_STATUS) {
        findings.push(ImportFinding::new(
            "IMPORT_REDIS_RIGHTS_UNACCEPTED",
            "blocking",
            path,
            "Redistribution status is not accepted for public import.",
            "Use public-permissive reviewed data or keep the data private.",
        ));
    }

    if review_status != Some("accepted") {
        findings.push(ImportFinding::new(
            "IMPORT_REVIEW_REQUIRED",
            "review_required",
            path,
            "Public import requires an accepted review disposition.",
            "Record maintainer review before accepting public data.",
        ));
    }

    findings
}

fn validate_nested_values<'a>(item: &'a Map<String, Value>, path: &str) -> Vec<ImportFinding> {
    let mut findings: Vec<ImportFinding> = Vec::new();
    let mut objects: Vec<(String, &'a Map<String, Value>)> = Vec::new();
    // Walk the record object itself (pre-order root), then every nested value.
    objects.push((path.to_string(), item));
    for (key, nested) in item {
        walk_value_objects(nested, format!("{path}.{key}"), &mut objects);
    }

    for (nested_path, value) in objects {
        if !value.contains_key("magnitude") {
            continue;
        }
        if !has_unit_metadata(value) {
            findings.push(ImportFinding::new(
                "IMPORT_UNIT_METADATA_MISSING",
                "blocking",
                &nested_path,
                "Unit-bearing imported value is missing unit or dimension metadata.",
                "Carry unit and dimension metadata through the import boundary.",
            ));
        }
        if !matches!(value.get("provenance"), Some(Value::Object(_))) {
            findings.push(ImportFinding::new(
                "IMPORT_VALUE_PROVENANCE_MISSING",
                "blocking",
                &nested_path,
                "Imported value is missing value-level provenance.",
                "Record value-level provenance for imported numeric data.",
            ));
        }
    }

    findings
}

/// Pre-order walk yielding every nested object (including the root) with its
/// dotted/indexed path. Mirrors the Python `_walk_value_objects` generator.
/// Owns the collected `Map` references via a clone-free borrow of the input
/// tree.
fn walk_value_objects<'a>(
    value: &'a Value,
    path: String,
    out: &mut Vec<(String, &'a Map<String, Value>)>,
) {
    match value {
        Value::Object(map) => {
            out.push((path.clone(), map));
            for (key, nested) in map {
                walk_value_objects(nested, format!("{path}.{key}"), out);
            }
        }
        Value::Array(items) => {
            for (index, nested) in items.iter().enumerate() {
                walk_value_objects(nested, format!("{path}[{index}]"), out);
            }
        }
        _ => {}
    }
}

fn has_unit_metadata(value: &Map<String, Value>) -> bool {
    let has_material_unit = value.contains_key("unit_ref") && value.contains_key("dimension_id");
    let has_component_unit = value.contains_key("unit") && value.contains_key("dimension");
    has_material_unit || has_component_unit
}

fn determine_outcome(findings: &[ImportFinding], intended_visibility: IntendedVisibility) -> String {
    if findings.iter().any(|f| f.severity == "quarantine") {
        return "QUARANTINE".to_string();
    }
    if findings.iter().any(|f| f.severity == "blocking") {
        return "REJECTED".to_string();
    }
    if findings.iter().any(|f| f.severity == "review_required") {
        return "REVIEW_REQUIRED".to_string();
    }
    match intended_visibility {
        IntendedVisibility::Private => "PRIVATE_LOCAL_ONLY".to_string(),
        IntendedVisibility::Public => "ACCEPTED_PUBLIC".to_string(),
    }
}

/// Python-truthiness for an optional JSON value: `None`/null/false, empty
/// strings, zero, and empty containers are falsy. Used for the
/// missing-provenance-field check and the `item-or-provenance` status
/// resolution.
fn is_truthy(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Number(n)) => n.as_f64().is_none_or(|f| f != 0.0),
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
    }
}

/// Return a non-empty string view of `value`, mirroring Python truthiness for
/// the `or`-fallback status resolution (a non-string or empty string is not a
/// usable status and falls through).
fn truthy_str(value: Option<&Value>) -> Option<&str> {
    match value {
        Some(Value::String(s)) if !s.is_empty() => Some(s.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn codes(result: &ImportValidationResult) -> Vec<String> {
        result.findings.iter().map(|f| f.code.clone()).collect()
    }

    #[test]
    fn token_parsers_reject_unsupported_inputs() {
        assert_eq!(LibraryKind::from_token("material"), Some(LibraryKind::Material));
        assert_eq!(LibraryKind::from_token("widget"), None);
        assert_eq!(
            IntendedVisibility::from_token("private"),
            Some(IntendedVisibility::Private)
        );
        assert_eq!(IntendedVisibility::from_token("secret"), None);

        let err = validate_library_import_tokens(&json!({}), "widget", "private")
            .expect_err("unsupported kind must error");
        assert!(err.contains("widget"));
        assert_eq!(LibraryKind::Material.token(), "material");
    }

    #[test]
    fn missing_library_metadata_and_records_block() {
        let result =
            validate_library_import(&json!({}), LibraryKind::Material, IntendedVisibility::Private);
        assert_eq!(result.outcome, "REJECTED");
        assert!(!result.accepted);
        assert!(codes(&result).contains(&"IMPORT_LIBRARY_METADATA_MISSING".to_string()));
        assert!(codes(&result).contains(&"IMPORT_RECORD_SET_MISSING".to_string()));
    }

    #[test]
    fn non_object_record_is_rejected() {
        let payload = json!({
            "material_library": {"provenance": full_provenance()},
            "material_records": ["not-an-object"],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Private);
        assert!(codes(&result).contains(&"IMPORT_RECORD_INVALID".to_string()));
        assert_eq!(result.outcome, "REJECTED");
    }

    #[test]
    fn incomplete_provenance_lists_sorted_missing_fields() {
        let payload = json!({
            "material_library": {
                "provenance": {"source_name": "x", "contributor": "y"}
            },
            "material_records": [],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Private);
        let finding = result
            .findings
            .iter()
            .find(|f| f.code == "IMPORT_PROVENANCE_INCOMPLETE")
            .expect("incomplete provenance finding");
        // Missing fields are reported sorted: contributor_certification,
        // redistribution_status, review_status, source_license, source_location.
        assert!(finding.message.contains(
            "contributor_certification, redistribution_status, review_status, \
             source_license, source_location"
        ));
    }

    #[test]
    fn nested_value_without_units_or_provenance_is_flagged() {
        let payload = json!({
            "component_library": {"provenance": full_provenance()},
            "component_records": [{
                "provenance": full_provenance(),
                "redistribution_status": "private_only",
                "fields": [{"value": {"magnitude": 1.0}}],
            }],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Component, IntendedVisibility::Private);
        let found = codes(&result);
        assert!(found.contains(&"IMPORT_UNIT_METADATA_MISSING".to_string()));
        assert!(found.contains(&"IMPORT_VALUE_PROVENANCE_MISSING".to_string()));
    }

    #[test]
    fn private_only_is_blocked_for_public_visibility() {
        let payload = json!({
            "material_library": {
                "provenance": full_provenance(),
                "redistribution_status": "private_only",
            },
            "material_records": [],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Public);
        assert!(codes(&result).contains(&"IMPORT_PRIVATE_DATA_PUBLIC_BLOCKED".to_string()));
        assert_eq!(result.outcome, "REJECTED");
    }

    #[test]
    fn unaccepted_public_redistribution_is_flagged() {
        let payload = json!({
            "material_library": {
                "provenance": full_provenance(),
                "redistribution_status": "some_other_status",
                "review_status": "accepted",
            },
            "material_records": [],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Public);
        assert!(codes(&result).contains(&"IMPORT_REDIS_RIGHTS_UNACCEPTED".to_string()));
        // review accepted -> no IMPORT_REVIEW_REQUIRED; still blocking.
        assert!(!codes(&result).contains(&"IMPORT_REVIEW_REQUIRED".to_string()));
        assert_eq!(result.outcome, "REJECTED");
    }

    #[test]
    fn rejected_source_disposition_blocks_import() {
        let payload = json!({
            "material_library": {
                "provenance": full_provenance(),
                "review_status": "rejected",
            },
            "material_records": [],
        });
        // Private visibility: the rejected disposition still blocks (it is
        // checked before the public-only branch).
        let result =
            validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Private);
        assert!(codes(&result).contains(&"IMPORT_REJECTED_SOURCE".to_string()));
        assert_eq!(result.outcome, "REJECTED");
    }

    #[test]
    fn private_privacy_class_blocks_public_import_even_with_permissive_redistribution() {
        let payload = json!({
            "material_library": {
                "provenance": full_provenance(),
                "redistribution_status": "public_permissive",
                "review_status": "accepted",
                "privacy_class": "private_project_data",
            },
            "material_records": [],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Public);
        assert!(codes(&result).contains(&"IMPORT_PRIVATE_DATA_PUBLIC_BLOCKED".to_string()));
        assert_eq!(result.outcome, "REJECTED");
    }

    #[test]
    fn item_level_status_overrides_provenance_status() {
        // Item-level redistribution_status ("rejected") wins over the truthy
        // provenance-level value ("public_permissive"), mirroring Python `or`.
        let payload = json!({
            "material_library": {
                "provenance": full_provenance(),
                "redistribution_status": "rejected",
            },
            "material_records": [],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Material, IntendedVisibility::Public);
        assert!(codes(&result).contains(&"IMPORT_REJECTED_SOURCE".to_string()));
        // The provenance-level "public_permissive" did not win, so no
        // public-disposition finding fires from this object.
        assert!(!codes(&result).contains(&"IMPORT_REDIS_RIGHTS_UNACCEPTED".to_string()));
    }

    #[test]
    fn diagnostic_envelope_carries_pkg02_shape() {
        let payload = json!({
            "component_library": {"provenance": full_provenance()},
            "component_records": [{
                "provenance": full_provenance(),
                "redistribution_status": "private_only",
                "fields": [{"value": {"magnitude": 1.0, "provenance": full_provenance()}}],
            }],
        });
        let result =
            validate_library_import(&payload, LibraryKind::Component, IntendedVisibility::Private);
        let diagnostic = result
            .diagnostics()
            .into_iter()
            .find(|d| d["code"] == "IMPORT_UNIT_METADATA_MISSING")
            .expect("unit metadata diagnostic");
        assert_eq!(diagnostic["class"], "import_boundary");
        assert_eq!(diagnostic["source"], DIAGNOSTIC_SOURCE);
        assert_eq!(diagnostic["provenance"]["source_name"], "library_import_payload");
        assert!(!diagnostic["remediation"].as_str().unwrap().is_empty());
    }

    fn full_provenance() -> Value {
        json!({
            "source_name": "Invented source",
            "source_location": "tests",
            "source_license": "public test license",
            "contributor": "OpenPipeStress",
            "contributor_certification": "invented non-engineering value",
            "redistribution_status": "public_permissive",
            "review_status": "accepted",
        })
    }
}

// One schema source shared with Python; no new runtime dependency or catalog.
const HANGER_SCHEMA: &str = include_str!("../../../../schemas/hanger.schema.yaml");
fn hanger_invalid(path: &str) -> ImportFinding {
    ImportFinding::new("IMPORT_HANGER_SCHEMA_INVALID", "blocking", path,
        "Hanger value does not satisfy the declared schema.",
        "Supply explicit supported fields, compatible quantities and complete provenance.")
}

/// Bounded interpreter for precisely the keywords authored in HANGER_SCHEMA.
/// Unsupported schema constructs fail closed; not a general JSON Schema engine.
fn hanger_shape(value: &Value, schema: &Value, root: &Value, path: &str, out: &mut Vec<ImportFinding>) {
    let allowed = ["$schema", "$id", "$defs", "title", "description", "$ref", "type",
        "additionalProperties", "required", "properties", "items", "enum", "const", "pattern", "exclusiveMinimum"];
    if schema.as_object().is_none_or(|s| s.keys().any(|k| !allowed.contains(&k.as_str()))) {
        out.push(hanger_invalid(path)); return;
    }
    if let Some(reference) = schema["$ref"].as_str() {
        if let Some(target) = reference.strip_prefix('#').and_then(|p| root.pointer(p)) {
            hanger_shape(value, target, root, path, out);
        } else { out.push(hanger_invalid(path)); }
        return;
    }
    let kind = schema["type"].as_str().unwrap_or("");
    let valid_type = match kind {
        "object" => value.is_object(), "array" => value.is_array(), "string" => value.is_string(),
        "number" => value.as_f64().is_some_and(|n| n.is_finite()), _ => false,
    };
    if !valid_type { out.push(hanger_invalid(path)); return; }
    if schema.get("enum").is_some_and(|e| !e.as_array().is_some_and(|a| a.contains(value)))
        || schema.get("const").is_some_and(|c| c != value)
        || schema.get("pattern").is_some_and(|p| p != "\\S" || value.as_str().is_none_or(|v| v.trim_matches(|c: char| c.is_whitespace() || ('\u{001c}'..='\u{001f}').contains(&c)).is_empty()))
        || schema.get("exclusiveMinimum").is_some_and(|m| value.as_f64().unwrap() <= m.as_f64().unwrap()) {
        out.push(hanger_invalid(path));
    }
    if let Some(object) = value.as_object() {
        let mut missing: Vec<&str> = schema["required"].as_array().into_iter().flatten()
            .filter_map(Value::as_str).filter(|key| !object.contains_key(*key)).collect();
        missing.sort_unstable();
        for key in missing { out.push(ImportFinding::new("IMPORT_HANGER_SCHEMA_INVALID", "blocking",
            &format!("{path}.{key}"), "Required hanger field is missing.", "Supply the required field explicitly.")); }
        let mut keys: Vec<&String> = object.keys().collect(); keys.sort();
        for key in keys {
            let next = format!("{path}.{key}");
            if let Some(property) = schema["properties"].get(key) {
                hanger_shape(&object[key], property, root, &next, out);
            } else if schema["additionalProperties"] == false {
                out.push(ImportFinding::new("IMPORT_HANGER_SCHEMA_INVALID", "blocking", &next,
                    "Unknown hanger field.", "Remove unsupported fields."));
            }
        }
    } else if let Some(array) = value.as_array() {
        for (i, item) in array.iter().enumerate() { hanger_shape(item, &schema["items"], root, &format!("{path}[{i}]"), out); }
    }
}

fn validate_hanger_import(payload: &Value, visibility: IntendedVisibility) -> ImportValidationResult {
    let schema: Value = serde_json::from_str(HANGER_SCHEMA).expect("embedded hanger schema is valid JSON");
    let mut findings = Vec::new();
    hanger_shape(payload, &schema, &schema, "$", &mut findings);
    let mut objects = Vec::new();
    walk_value_objects(payload, "$".into(), &mut objects);
    objects.sort_by(|a,b| a.0.cmp(&b.0));
    for (path, item) in objects {
        if item.contains_key("provenance") || (!path.ends_with(".provenance") &&
            ["redistribution_status", "review_status", "privacy_class"].iter().any(|key| item.contains_key(*key))) {
            let provenance = item.get("provenance").unwrap_or(&Value::Null);
            findings.extend(validate_hanger_disposition(item, &path, visibility));
            if let Some(p) = provenance.as_object() {
                if ["redistribution_status", "review_status"].iter().any(|key|
                    item.contains_key(*key) && item.get(*key) != p.get(*key)) {
                    let mut wrapper = Map::new(); wrapper.insert("provenance".into(), provenance.clone());
                    findings.extend(validate_object_disposition(&wrapper, &format!("{path}.provenance"), visibility));
                }
            }
        }
    }
    let mut seen = std::collections::HashSet::new();
    for (i, record) in payload["hanger_records"].as_array().into_iter().flatten().enumerate() {
        if let Some(id) = record.get("hanger_id").and_then(Value::as_str) {
            if !seen.insert(id) { findings.push(ImportFinding::new("IMPORT_HANGER_DUPLICATE_ID", "blocking",
                &format!("$.hanger_records[{i}].hanger_id"), "Duplicate hanger identity.", "Use a unique hanger_id within the library.")); }
        }
    }
    let outcome = determine_outcome(&findings, visibility);
    ImportValidationResult { accepted: matches!(outcome.as_str(), "PRIVATE_LOCAL_ONLY" | "ACCEPTED_PUBLIC"),
        outcome, library_kind: LibraryKind::Hanger, intended_visibility: visibility, findings }
}


fn validate_hanger_disposition(item: &Map<String, Value>, path: &str, visibility: IntendedVisibility) -> Vec<ImportFinding> {
    // Hanger privacy includes project_private; preserve legacy-family semantics.
    // This temporary diagnostic view never changes the imported source payload.
    let mut view = item.clone();
    if view.get("privacy_class").and_then(Value::as_str) == Some("project_private") {
        view.insert("privacy_class".into(), Value::String("private_project_data".into()));
    }
    let mut findings = validate_object_disposition(&view, path, visibility);
    // Malformed provenance must not suppress an explicit item quarantine marker.
    if !item.get("provenance").is_some_and(Value::is_object)
        && (item.get("redistribution_status").and_then(Value::as_str) == Some("protected_suspected")
            || item.get("review_status").and_then(Value::as_str) == Some("quarantined")) {
        findings.push(ImportFinding::new("IMPORT_PROTECTED_CONTENT_SUSPECTED", "quarantine", path,
            "Import metadata indicates suspected protected content.",
            "Quarantine metadata and request human/legal review; do not publish values."));
    }
    findings
}
