//! Structured editor-operation validation, diff preview, and apply seam.
//!
//! This crate is the DEL-16-02 / DEL-16-03 runtime seam for the desktop app:
//! it consumes the UI's structured `EditorOperationIntent` records, validates
//! them against a model document without mutating it, produces deterministic
//! diff-preview rows, and — only for `apply` — returns a *new* model document
//! with the single validated change applied plus an honest acceptance receipt.
//!
//! Boundary rules carried from the PKG-16 contracts:
//! - the input model document is never mutated in place;
//! - blocked operations are findings, not silent fallbacks (no invented
//!   values, no unit conversion, no geometry defaults);
//! - receipts record a user-initiated local-session acceptance basis only and
//!   never a professional, certification, sealing, approval, authentication,
//!   or code-compliance claim.

use serde::Serialize;
use serde_json::{Number, Value};
use sha2::{Digest, Sha256};

pub const OPERATION_APPLIER_VERSION: &str = "0.1.0";
pub const OUTCOME_DOCUMENT_KIND: &str = "openpipestress.desktop.operation_outcome";
/// Canonicalization label carried in hash evidence. True RFC 8785 (JCS)
/// since completion-plan H5: ECMAScript number rendering, UTF-16 key sort,
/// `JSON.stringify` string escaping — see `core/serialization/canonical_json`.
pub const BACKEND_CANONICALIZATION: &str = "rfc8785_jcs";
const ENGINE_SOURCE: &str = "core/model_operations/operation_applier";

/// Restraint direction vocabulary used by the preview model documents.
const RESTRAINT_TOKENS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];

/// Canonical dimension vocabulary: the accepted PKG-02 set shared by
/// `docs/SPEC.md` §4, `schemas/units.schema.yaml` `DimensionId`, and the
/// DEL-16-02 validation-preview engine.
const CANONICAL_DIMENSIONS: [&str; 30] = [
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
    "moment",
    "pressure",
    "stress",
    "area",
    "volume",
    "density",
    "linear_stiffness",
    "rotational_stiffness",
    "displacement",
    "velocity",
    "acceleration",
    "thermal_conductivity",
    "specific_heat",
    "thermal_expansion_coefficient",
    "second_moment_area",
    "section_modulus",
    "mass_per_length",
    "volume_per_length",
    "slope",
    "force_per_length",
    "TBD",
];

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OutcomeDiagnostic {
    pub id: String,
    pub code: String,
    pub severity: String,
    pub message: String,
    pub remediation: String,
    pub affected_refs: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DiffPreviewRow {
    pub entity_ref: String,
    pub object_type: String,
    pub field_path: String,
    pub before: String,
    pub after: String,
    pub unit: String,
    pub dimension: String,
    pub change_kind: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ModelBasisEvidence {
    pub claimed_model_hash: String,
    pub claimed_hash_canonicalization: String,
    pub backend_model_hash: String,
    pub backend_canonicalization: String,
    pub binding_status: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ValidationStates {
    pub schema_validation: String,
    pub reference_validation: String,
    pub unit_validation: String,
    pub before_state_validation: String,
    pub diff_preview_status: String,
    pub application_status: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AcceptanceRecord {
    pub acceptance_basis: String,
    pub acceptance_is_professional_approval: bool,
    pub persistence_status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OperationOutcome {
    pub schema_version: String,
    pub document_kind: String,
    pub deliverable_refs: Vec<String>,
    pub mode: String,
    pub application_route: String,
    pub operation_id: String,
    pub change_id: String,
    pub operation_kind: String,
    pub change_kind: String,
    pub target_object_type: String,
    pub target_ref: String,
    pub validation: ValidationStates,
    pub diff_preview: Vec<DiffPreviewRow>,
    pub diagnostics: Vec<OutcomeDiagnostic>,
    pub model_basis: ModelBasisEvidence,
    pub input_model_unchanged: bool,
    pub applied_model: Option<Value>,
    pub applied_model_backend_hash: Option<String>,
    pub acceptance: AcceptanceRecord,
    pub audit_boundary: Value,
    pub professional_boundary: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitSource {
    /// Quantity stored as `{ value, unit }`; the unit lives next to the value.
    SiblingUnitField,
    /// Bare numeric field whose unit basis is the project unit system entry
    /// with this key (e.g. node positions use `project.units.length`).
    ProjectUnits(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldKind {
    Text,
    /// Bare numeric scalar with no unit metadata, such as a dimensionless
    /// combination factor.
    Number {
        require_positive: bool,
    },
    /// Numeric quantity; `require_positive` rejects non-physical zero/negative
    /// geometry and modulus values.
    Quantity {
        require_positive: bool,
        unit_source: UnitSource,
    },
    /// Reference to an entity id inside another model collection.
    EntityRef {
        collection: &'static str,
    },
    /// Comma-separated restraint direction tokens stored as a string array.
    RestraintSet,
}

struct FieldRule {
    field_path: &'static str,
    kind: FieldKind,
}

/// Inspector-offered fields whose application is deliberately deferred to a
/// later completion-plan item. Returned as explicit blocked findings so the
/// scope limit stays visible instead of silently half-working.
const DEFERRED_FIELDS: [(&str, &str, &str); 2] = [
    ("Component", "kind", "component-kind editing is deferred to the component editor scope (completion plan Phase C/D)"),
    ("Combination", "terms", "combination term editing is deferred to the load case manager (completion plan A4)"),
];

fn field_rules(object_type: &str) -> &'static [FieldRule] {
    match object_type {
        "Material" => &[
            FieldRule {
                field_path: "label",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "provenance",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "elastic_modulus.value",
                kind: FieldKind::Quantity {
                    require_positive: true,
                    unit_source: UnitSource::SiblingUnitField,
                },
            },
            FieldRule {
                field_path: "shear_modulus.value",
                kind: FieldKind::Quantity {
                    require_positive: true,
                    unit_source: UnitSource::SiblingUnitField,
                },
            },
            FieldRule {
                field_path: "thermal_expansion_coefficient.value",
                kind: FieldKind::Quantity {
                    require_positive: false,
                    unit_source: UnitSource::SiblingUnitField,
                },
            },
        ],
        "Node" => &[
            FieldRule {
                field_path: "label",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "provenance",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "position.x",
                kind: FieldKind::Quantity {
                    require_positive: false,
                    unit_source: UnitSource::ProjectUnits("length"),
                },
            },
            FieldRule {
                field_path: "position.y",
                kind: FieldKind::Quantity {
                    require_positive: false,
                    unit_source: UnitSource::ProjectUnits("length"),
                },
            },
            FieldRule {
                field_path: "position.z",
                kind: FieldKind::Quantity {
                    require_positive: false,
                    unit_source: UnitSource::ProjectUnits("length"),
                },
            },
        ],
        "Element" => &[
            FieldRule {
                field_path: "label",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "provenance",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "material",
                kind: FieldKind::EntityRef {
                    collection: "materials",
                },
            },
            FieldRule {
                field_path: "section.outside_diameter.value",
                kind: FieldKind::Quantity {
                    require_positive: true,
                    unit_source: UnitSource::SiblingUnitField,
                },
            },
            FieldRule {
                field_path: "section.wall_thickness.value",
                kind: FieldKind::Quantity {
                    require_positive: true,
                    unit_source: UnitSource::SiblingUnitField,
                },
            },
        ],
        "Support" => &[
            FieldRule {
                field_path: "label",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "provenance",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "node",
                kind: FieldKind::EntityRef {
                    collection: "nodes",
                },
            },
            FieldRule {
                field_path: "restraints",
                kind: FieldKind::RestraintSet,
            },
        ],
        "Component" => &[
            FieldRule {
                field_path: "label",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "provenance",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "node",
                kind: FieldKind::EntityRef {
                    collection: "nodes",
                },
            },
        ],
        "Load" => &[
            FieldRule {
                field_path: "label",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "status",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "kind",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "provenance",
                kind: FieldKind::Text,
            },
        ],
        "Combination" => &[
            FieldRule {
                field_path: "label",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "basis",
                kind: FieldKind::Text,
            },
            FieldRule {
                field_path: "provenance",
                kind: FieldKind::Text,
            },
        ],
        _ => &[],
    }
}

fn collection_for(object_type: &str) -> Option<&'static str> {
    match object_type {
        "Material" => Some("materials"),
        "Node" => Some("nodes"),
        "Element" => Some("pipe_segments"),
        "Support" => Some("supports"),
        "Component" => Some("components"),
        "Load" => Some("load_cases"),
        "Combination" => Some("combinations"),
        _ => None,
    }
}

/// Validate a structured editor-operation intent against a model document.
/// Never mutates or returns a model.
pub fn validate_operation(
    model: &Value,
    intent: &Value,
    claimed_model_hash: Option<&Value>,
) -> OperationOutcome {
    run(model, intent, claimed_model_hash, Mode::ValidateOnly)
}

/// Validate and, when no blocking finding exists, apply the single change to
/// a cloned model document. The input model is never mutated in place.
pub fn apply_operation(
    model: &Value,
    intent: &Value,
    claimed_model_hash: Option<&Value>,
) -> OperationOutcome {
    run(model, intent, claimed_model_hash, Mode::Apply)
}

/// JSON-string boundary for the wasm32 browser-engine build (`--features
/// wasm`; DEC-020 / ADR-0001). The exports accept and return JSON strings so
/// the JavaScript shim stays a thin transport. Malformed input JSON returns a
/// structured error envelope (`document_kind`
/// `openpipestress.desktop.wasm_engine_input_error`) instead of trapping —
/// the shim surfaces it as an explicit diagnostic, never a silent fallback.
/// Native builds and tests do not compile this module.
#[cfg(feature = "wasm")]
mod wasm_api {
    use super::{apply_operation, validate_operation};
    use serde_json::{json, Value};
    use wasm_bindgen::prelude::wasm_bindgen;
    use wasm_bindgen::JsError;

    const INPUT_ERROR_DOCUMENT_KIND: &str = "openpipestress.desktop.wasm_engine_input_error";

    fn input_error(input_label: &str, detail: String) -> String {
        json!({
            "document_kind": INPUT_ERROR_DOCUMENT_KIND,
            "error": {
                "code": "WASM-ENGINE-INPUT-JSON-INVALID",
                "severity": "blocking",
                "input": input_label,
                "message": format!("{input_label} is not valid JSON: {detail}"),
            }
        })
        .to_string()
    }

    fn parse_input(input_label: &str, payload: &str) -> Result<Value, String> {
        serde_json::from_str(payload).map_err(|error| input_error(input_label, error.to_string()))
    }

    fn run_json(
        model_json: &str,
        intent_json: &str,
        claimed_model_hash_json: &str,
        apply: bool,
    ) -> String {
        let model = match parse_input("model", model_json) {
            Ok(value) => value,
            Err(envelope) => return envelope,
        };
        let intent = match parse_input("intent", intent_json) {
            Ok(value) => value,
            Err(envelope) => return envelope,
        };
        let claimed = match parse_input("claimed_model_hash", claimed_model_hash_json) {
            Ok(value) => value,
            Err(envelope) => return envelope,
        };
        let claimed_ref = if claimed.is_null() {
            None
        } else {
            Some(&claimed)
        };
        let mut outcome = if apply {
            apply_operation(&model, &intent, claimed_ref)
        } else {
            validate_operation(&model, &intent, claimed_ref)
        };
        // Honest receipt: this outcome was produced through the in-process
        // wasm engine, not the Tauri backend command route.
        outcome.application_route = "local_wasm_engine".to_string();
        serde_json::to_string(&outcome)
            .unwrap_or_else(|error| input_error("outcome_serialization", error.to_string()))
    }

    /// Validate a structured editor-operation intent. Returns the
    /// `OperationOutcome` document as a JSON string.
    #[wasm_bindgen]
    pub fn validate_operation_json(
        model_json: &str,
        intent_json: &str,
        claimed_model_hash_json: &str,
    ) -> String {
        run_json(model_json, intent_json, claimed_model_hash_json, false)
    }

    /// Validate and, when no blocking finding exists, apply the single change
    /// to a cloned model document. Returns the `OperationOutcome` document as
    /// a JSON string.
    #[wasm_bindgen]
    pub fn apply_operation_json(
        model_json: &str,
        intent_json: &str,
        claimed_model_hash_json: &str,
    ) -> String {
        run_json(model_json, intent_json, claimed_model_hash_json, true)
    }

    fn parse_value(value_json: &str) -> Result<Value, JsError> {
        serde_json::from_str(value_json).map_err(|error| {
            JsError::new(&format!(
                "WASM-ENGINE-INPUT-JSON-INVALID: value is not valid JSON: {error}"
            ))
        })
    }

    /// Canonicalize a JSON document with the engine's canonical form —
    /// RFC 8785 (JCS) since completion-plan H5 — the same function the
    /// engine uses for model hashes. H1 / F-5a hash seam: the desktop
    /// frontend has no canonicalization of its own.
    #[wasm_bindgen]
    pub fn canonical_json_string(value_json: &str) -> Result<String, JsError> {
        Ok(super::canonical_json(&parse_value(value_json)?))
    }

    /// Lowercase-hex SHA-256 of the canonical form of a JSON document.
    /// H1 / F-5a hash seam companion to [`canonical_json_string`].
    #[wasm_bindgen]
    pub fn canonical_sha256_hex(value_json: &str) -> Result<String, JsError> {
        Ok(super::sha256_hex(&super::canonical_json(&parse_value(
            value_json,
        )?)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    ValidateOnly,
    Apply,
}

struct Checker {
    diagnostics: Vec<OutcomeDiagnostic>,
    schema_blocked: bool,
    reference_state: &'static str,
    unit_state: &'static str,
    before_state: &'static str,
}

impl Checker {
    fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            schema_blocked: false,
            reference_state: "not_run",
            unit_state: "not_run",
            before_state: "not_run",
        }
    }

    fn blocking(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|item| item.severity == "blocking")
    }

    fn push(
        &mut self,
        code: &str,
        severity: &str,
        message: String,
        remediation: &str,
        affected: Vec<String>,
    ) {
        let id = format!(
            "diagnostic:operation-applier:{}:{:03}",
            code.to_ascii_lowercase(),
            self.diagnostics.len() + 1
        );
        self.diagnostics.push(OutcomeDiagnostic {
            id,
            code: code.to_string(),
            severity: severity.to_string(),
            message,
            remediation: remediation.to_string(),
            affected_refs: affected,
            source: ENGINE_SOURCE.to_string(),
        });
    }
}

fn run(
    model: &Value,
    intent: &Value,
    claimed_model_hash: Option<&Value>,
    mode: Mode,
) -> OperationOutcome {
    let mut checker = Checker::new();

    let operation_id =
        string_at(intent, &["operation_id"]).unwrap_or_else(|| "operation:unknown".to_string());
    let change_id =
        string_at(intent, &["change", "change_id"]).unwrap_or_else(|| "change:unknown".to_string());
    let operation_kind = string_at(intent, &["operation_kind"]).unwrap_or_default();
    let change_kind = string_at(intent, &["change", "change_kind"]).unwrap_or_default();
    let object_type = string_at(intent, &["target", "object_type"]).unwrap_or_default();
    let target_ref = string_at(intent, &["target", "ref"]).unwrap_or_default();
    let field_path = string_at(intent, &["change", "field_path"]).unwrap_or_default();
    let before = string_at(intent, &["change", "before"]).unwrap_or_default();
    let after = string_at(intent, &["change", "after"]).unwrap_or_default();
    let unit = string_at(intent, &["change", "unit"]).unwrap_or_default();
    let dimension = string_at(intent, &["change", "dimension"]).unwrap_or_default();

    check_intent_structure(intent, &operation_id, &mut checker);
    check_audit_boundary(intent, &operation_id, &mut checker);
    check_kinds(&operation_kind, &change_kind, &operation_id, &mut checker);

    let model_basis = model_basis_evidence(model, claimed_model_hash);

    // Field-level resolution proceeds only for supported structured payloads;
    // underspecified viewport gestures are held rather than invented.
    let mut resolved: Option<ResolvedField> = None;
    if !checker.schema_blocked && is_modify_change_kind(&change_kind) {
        resolved = resolve_field(
            model,
            &object_type,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &mut checker,
        );
    }
    let mut created_node: Option<Value> = None;
    if !checker.schema_blocked && change_kind == "create_node" {
        created_node = resolve_create_node(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_pipe: Option<Value> = None;
    if !checker.schema_blocked && change_kind == "connect_pipe_run" {
        created_pipe = resolve_connect_pipe_run(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_section: Option<Value> = None;
    if !checker.schema_blocked && change_kind == "create_section" {
        created_section = resolve_create_section(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_material: Option<Value> = None;
    if !checker.schema_blocked && change_kind == "create_material" {
        created_material = resolve_create_material(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_support: Option<Value> = None;
    if !checker.schema_blocked && change_kind == "create_support" {
        created_support = resolve_create_support(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_load_case: Option<Value> = None;
    if !checker.schema_blocked && change_kind == "create_load_case" {
        created_load_case = resolve_create_load_case(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_primitive_load: Option<(String, Value)> = None;
    if !checker.schema_blocked && change_kind == "create_primitive_load" {
        created_primitive_load = resolve_create_primitive_load(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_combination: Option<Value> = None;
    if !checker.schema_blocked && change_kind == "create_combination" {
        created_combination = resolve_create_combination(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut created_combination_term: Option<(String, Value)> = None;
    if !checker.schema_blocked && change_kind == "create_combination_term" {
        created_combination_term = resolve_create_combination_term(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }
    let mut deleted_combination_term: Option<(String, usize)> = None;
    if !checker.schema_blocked && change_kind == "delete_combination_term" {
        deleted_combination_term = resolve_delete_combination_term(
            model,
            &target_ref,
            &field_path,
            &before,
            &after,
            &unit,
            &dimension,
            &object_type,
            &mut checker,
        );
    }

    let blocking = checker.blocking();
    let diff_preview = if let (Some(field), false) = (&resolved, blocking) {
        vec![DiffPreviewRow {
            entity_ref: target_ref.clone(),
            object_type: object_type.clone(),
            field_path: field_path.clone(),
            before: field.current_display.clone(),
            after: after.clone(),
            unit: unit.clone(),
            dimension: dimension.clone(),
            change_kind: change_kind.clone(),
        }]
    } else if (created_node.is_some()
        || created_pipe.is_some()
        || created_section.is_some()
        || created_material.is_some()
        || created_support.is_some()
        || created_load_case.is_some()
        || created_primitive_load.is_some()
        || created_combination.is_some()
        || created_combination_term.is_some()
        || deleted_combination_term.is_some())
        && !blocking
    {
        vec![DiffPreviewRow {
            entity_ref: target_ref.clone(),
            object_type: object_type.clone(),
            field_path: field_path.clone(),
            before: before.clone(),
            after: after.clone(),
            unit: unit.clone(),
            dimension: dimension.clone(),
            change_kind: change_kind.clone(),
        }]
    } else {
        Vec::new()
    };

    let mut applied_model = None;
    let mut applied_model_backend_hash = None;
    if mode == Mode::Apply && !blocking {
        if let Some(field) = &resolved {
            let mut next_model = model.clone();
            if apply_resolved_field(
                &mut next_model,
                &object_type,
                &target_ref,
                field,
                &mut checker,
            ) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some(node) = &created_node {
            let mut next_model = model.clone();
            if apply_created_node(&mut next_model, node) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some(pipe) = &created_pipe {
            let mut next_model = model.clone();
            if apply_created_pipe(&mut next_model, pipe) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some(section) = &created_section {
            let mut next_model = model.clone();
            if apply_created_section(&mut next_model, section) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some(material) = &created_material {
            let mut next_model = model.clone();
            if apply_created_material(&mut next_model, material) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some(support) = &created_support {
            let mut next_model = model.clone();
            if apply_created_support(&mut next_model, support) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some(load_case) = &created_load_case {
            let mut next_model = model.clone();
            if apply_created_load_case(&mut next_model, load_case) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some((load_case_id, primitive_load)) = &created_primitive_load {
            let mut next_model = model.clone();
            if apply_created_primitive_load(&mut next_model, load_case_id, primitive_load) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some(combination) = &created_combination {
            let mut next_model = model.clone();
            if apply_created_combination(&mut next_model, combination) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some((combination_id, term)) = &created_combination_term {
            let mut next_model = model.clone();
            if apply_created_combination_term(&mut next_model, combination_id, term) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        } else if let Some((combination_id, term_index)) = &deleted_combination_term {
            let mut next_model = model.clone();
            if apply_deleted_combination_term(&mut next_model, combination_id, *term_index) {
                applied_model_backend_hash = Some(format!(
                    "sha256:{}",
                    sha256_hex(&canonical_json(&next_model))
                ));
                applied_model = Some(next_model);
            }
        }
    }

    let blocking_after_apply = checker.blocking();
    let applied = applied_model.is_some();

    let validation = ValidationStates {
        schema_validation: if checker.schema_blocked {
            "blocked"
        } else {
            "passed"
        }
        .to_string(),
        reference_validation: checker.reference_state.to_string(),
        unit_validation: checker.unit_state.to_string(),
        before_state_validation: checker.before_state.to_string(),
        diff_preview_status: if blocking_after_apply {
            "blocked_by_validation"
        } else {
            "generated"
        }
        .to_string(),
        application_status: match (mode, applied, blocking_after_apply) {
            (Mode::ValidateOnly, _, _) => "not_applied",
            (Mode::Apply, true, _) => "applied_to_session_model",
            (Mode::Apply, false, _) => "blocked",
        }
        .to_string(),
    };

    let acceptance = if applied {
        AcceptanceRecord {
            acceptance_basis: "user_initiated_apply_in_local_session".to_string(),
            acceptance_is_professional_approval: false,
            persistence_status: "session_state_only_not_yet_saved".to_string(),
        }
    } else {
        AcceptanceRecord {
            acceptance_basis: "none_validation_only".to_string(),
            acceptance_is_professional_approval: false,
            persistence_status: "not_applicable_no_application".to_string(),
        }
    };

    let mut diagnostics = checker.diagnostics;
    diagnostics.sort_by(|left, right| {
        (left.code.as_str(), left.message.as_str())
            .cmp(&(right.code.as_str(), right.message.as_str()))
    });

    OperationOutcome {
        schema_version: OPERATION_APPLIER_VERSION.to_string(),
        document_kind: OUTCOME_DOCUMENT_KIND.to_string(),
        deliverable_refs: vec!["DEL-16-02".to_string(), "DEL-16-03".to_string()],
        mode: match mode {
            Mode::ValidateOnly => "validate_only".to_string(),
            Mode::Apply => "apply".to_string(),
        },
        application_route: "tauri_backend_apply".to_string(),
        operation_id,
        change_id,
        operation_kind,
        change_kind,
        target_object_type: object_type,
        target_ref,
        validation,
        diff_preview,
        diagnostics,
        model_basis,
        input_model_unchanged: true,
        applied_model,
        applied_model_backend_hash,
        acceptance,
        audit_boundary: serde_json::json!({
            "mutation_route": "structured_operations_only",
            "direct_model_mutation_allowed": false,
            "requires_user_acceptance": true,
            "input_model_mutated_in_place": false,
            "applied_model_is_new_document": applied,
        }),
        professional_boundary: serde_json::json!({
            "human_review_required": true,
            "software_makes_compliance_claim": false,
            "software_makes_certification_claim": false,
            "software_makes_sealing_claim": false,
            "software_makes_approval_claim": false,
            "software_makes_authentication_claim": false,
        }),
    }
}

struct ResolvedField {
    kind: FieldKind,
    /// Display form of the current model value using the same conventions the
    /// inspector uses to populate `change.before`.
    current_display: String,
    /// Parsed replacement value, ready to write on apply.
    applied_value: Value,
    /// Path segments inside the entity object (array indices allowed).
    segments: Vec<String>,
}

fn check_intent_structure(intent: &Value, operation_id: &str, checker: &mut Checker) {
    let required: [&[&str]; 10] = [
        &["operation_id"],
        &["operation_kind"],
        &["operation_status"],
        &["target", "object_type"],
        &["target", "ref"],
        &["change", "change_id"],
        &["change", "change_kind"],
        &["change", "field_path"],
        &["audit_boundary"],
        &["professional_boundary"],
    ];
    for path in required {
        if value_at(intent, path).is_none() {
            checker.schema_blocked = true;
            checker.push(
                "OP-INTENT-FIELD-MISSING",
                "blocking",
                format!(
                    "Operation intent is missing required field `{}`.",
                    path.join(".")
                ),
                "Provide a complete structured editor-operation intent record.",
                vec![operation_id.to_string()],
            );
        }
    }
    // `before` and `after` must be present as strings (they may be empty;
    // emptiness is judged per field kind later).
    for leaf in ["before", "after", "unit", "dimension"] {
        if string_at(intent, &["change", leaf]).is_none() {
            checker.schema_blocked = true;
            checker.push(
                "OP-INTENT-FIELD-MISSING",
                "blocking",
                format!("Operation intent is missing required field `change.{leaf}`."),
                "Provide a complete structured editor-operation intent record.",
                vec![operation_id.to_string()],
            );
        }
    }
}

fn check_audit_boundary(intent: &Value, operation_id: &str, checker: &mut Checker) {
    let route = string_at(intent, &["audit_boundary", "mutation_route"]).unwrap_or_default();
    let direct = value_at(intent, &["audit_boundary", "direct_model_mutation_allowed"])
        .and_then(Value::as_bool)
        .unwrap_or(true);
    if route != "structured_operations_only" || direct {
        checker.schema_blocked = true;
        checker.push(
            "OP-DIRECT-MUTATION-BLOCKED",
            "blocking",
            "Operation intent requests a mutation route outside structured operations; direct model mutation is not allowed.".to_string(),
            "Route all model changes through structured operations with direct_model_mutation_allowed=false.",
            vec![operation_id.to_string()],
        );
    }
}

fn is_modify_change_kind(change_kind: &str) -> bool {
    matches!(change_kind, "set_field" | "update_load" | "update_support")
}

fn check_kinds(operation_kind: &str, change_kind: &str, operation_id: &str, checker: &mut Checker) {
    let supported_change = matches!(
        change_kind,
        "set_field"
            | "update_load"
            | "update_support"
            | "create_node"
            | "connect_pipe_run"
            | "create_section"
            | "create_material"
            | "create_support"
            | "create_load_case"
            | "create_primitive_load"
            | "create_combination"
            | "create_combination_term"
            | "delete_combination_term"
            | "insert_component_symbol"
    );
    if !supported_change {
        checker.schema_blocked = true;
        checker.push(
            "OP-KIND-UNSUPPORTED",
            "blocking",
            format!(
                "Change kind `{change_kind}` is outside the structured editor-operation taxonomy."
            ),
            "Use a supported change kind from the structured operation contract.",
            vec![operation_id.to_string()],
        );
        return;
    }

    let expected_operation_kind = match change_kind {
        "set_field" | "update_load" | "update_support" => "modify",
        "create_node"
        | "create_section"
        | "create_material"
        | "create_support"
        | "create_load_case"
        | "create_primitive_load"
        | "create_combination"
        | "create_combination_term" => "create",
        "delete_combination_term" => "delete",
        "connect_pipe_run" => "connect",
        "insert_component_symbol" => "insert",
        _ => unreachable!("supported_change guarantees a known change kind"),
    };
    if operation_kind != expected_operation_kind {
        checker.schema_blocked = true;
        checker.push(
            "OP-CHANGE-KIND-MISMATCH",
            "blocking",
            format!(
                "Operation kind `{operation_kind}` does not match change kind `{change_kind}` (expected `{expected_operation_kind}`)."
            ),
            "Emit operation and change kinds from the same structured-operation taxonomy row.",
            vec![operation_id.to_string()],
        );
    }

    if matches!(change_kind, "insert_component_symbol") {
        checker.schema_blocked = true;
        checker.push(
            "OP-GEOMETRY-INPUT-INCOMPLETE",
            "blocking",
            format!(
                "Viewport gesture intent `{change_kind}` does not yet carry explicit geometry/connectivity inputs; values are not invented on the user's behalf."
            ),
            "Capture explicit geometry inputs in the viewport editing tools (completion plan A3) before applying this intent kind.",
            vec![operation_id.to_string()],
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_node(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Node" || field_path != "nodes" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-NODE-SHAPE-INVALID",
            "blocking",
            "Create-node intents must target object_type `Node` with field_path `nodes`."
                .to_string(),
            "Refresh the viewport create-node intent from the explicit node geometry form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    let stored_unit = value_at(model, &["project", "units", "length"]).and_then(Value::as_str);
    checker.unit_state = "passed";
    let Some(stored_unit) = stored_unit else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project length unit metadata is missing; explicit node coordinates cannot be accepted.".to_string(),
            "Repair the model document's project.units.length metadata before creating nodes.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if unit != stored_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match project length unit `{stored_unit}`; unit conversion is unavailable until the units engine lands."),
            "Enter node coordinates in the project length unit; no silent conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "length" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-node dimension `{dimension}` must be `length`."),
            "Emit create-node coordinates with explicit length dimension metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "nodes", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Node `{target_ref}` already exists in the current model."),
            "Choose a new stable node id; create operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CREATE-NODE-PAYLOAD-INVALID",
            "blocking",
            "Create-node payload is not valid JSON.".to_string(),
            "Emit the explicit node geometry payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-NODE-PAYLOAD-INVALID",
            "blocking",
            "Create-node payload must be a JSON object.".to_string(),
            "Emit id, label, position, and provenance fields for the new node.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let label = record
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let position = record.get("position");
    let Some(position) = position.and_then(Value::as_object) else {
        checker.push(
            "OP-CREATE-NODE-PAYLOAD-INVALID",
            "blocking",
            "Create-node payload must include matching id, non-empty label, non-empty provenance, and position.".to_string(),
            "Refresh the viewport create-node intent from explicit user-entered node fields.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let x = position
        .get("x")
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite());
    let y = position
        .get("y")
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite());
    let z = position
        .get("z")
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite());
    if id != target_ref
        || label.is_empty()
        || provenance.is_empty()
        || x.is_none()
        || y.is_none()
        || z.is_none()
    {
        checker.push(
            "OP-CREATE-NODE-PAYLOAD-INVALID",
            "blocking",
            "Create-node payload must include matching id, non-empty label, non-empty provenance, and finite numeric x/y/z coordinates.".to_string(),
            "Refresh the viewport create-node intent from explicit user-entered node fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    Some(serde_json::json!({
        "id": id,
        "label": label,
        "position": { "x": x.unwrap(), "y": y.unwrap(), "z": z.unwrap() },
        "provenance": provenance,
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_connect_pipe_run(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Element" || field_path != "pipe_segments" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CONNECT-PIPE-SHAPE-INVALID",
            "blocking",
            "Connect-pipe intents must target object_type `Element` with field_path `pipe_segments`.".to_string(),
            "Refresh the viewport connect-pipe intent from the explicit pipe connectivity form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    let stored_unit = value_at(model, &["project", "units", "length"]).and_then(Value::as_str);
    checker.unit_state = "passed";
    let Some(stored_unit) = stored_unit else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project length unit metadata is missing; explicit pipe geometry cannot be accepted.".to_string(),
            "Repair the model document's project.units.length metadata before creating pipe segments.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if unit != stored_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match project length unit `{stored_unit}`; unit conversion is unavailable until the units engine lands."),
            "Enter pipe section dimensions in the project length unit; no silent conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "length" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Connect-pipe dimension `{dimension}` must be `length`."),
            "Emit pipe section geometry with explicit length dimension metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "pipe_segments", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Pipe segment `{target_ref}` already exists in the current model."),
            "Choose a new stable pipe id; connect operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe payload is not valid JSON.".to_string(),
            "Emit the explicit pipe connectivity payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe payload must be a JSON object.".to_string(),
            "Emit id, label, from, to, section, material, y_reference, and provenance fields for the new pipe segment.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let label = record
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let from = record
        .get("from")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let to = record
        .get("to")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let material = record
        .get("material")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let outside_diameter = quantity_value(record, &["section", "outside_diameter"], stored_unit);
    let wall_thickness = quantity_value(record, &["section", "wall_thickness"], stored_unit);
    let y_reference = vector_value(record, "y_reference");

    if id != target_ref
        || label.is_empty()
        || from.is_empty()
        || to.is_empty()
        || from == to
        || material.is_empty()
        || provenance.is_empty()
        || outside_diameter.is_none()
        || wall_thickness.is_none()
        || y_reference.is_none()
    {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe payload must include matching id, non-empty label/from/to/material/provenance, distinct endpoint nodes, positive OD/wall quantities in the project length unit, and a non-zero y_reference vector.".to_string(),
            "Refresh the viewport connect-pipe intent from explicit user-entered pipe fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "nodes", from).is_none() || find_entity(model, "nodes", to).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PIPE-ENDPOINT-NOT-FOUND",
            "blocking",
            format!("Pipe `{target_ref}` references endpoint nodes `{from}` and `{to}`, but at least one is absent from the current model."),
            "Create or select existing endpoint nodes before connecting a pipe segment.",
            vec![target_ref.to_string(), from.to_string(), to.to_string()],
        );
        return None;
    }
    if find_entity(model, "materials", material).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PIPE-MATERIAL-NOT-FOUND",
            "blocking",
            format!("Pipe `{target_ref}` references material `{material}`, which is absent from the current model."),
            "Select an existing material before connecting a pipe segment.",
            vec![target_ref.to_string(), material.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    let (yrx, yry, yrz) = y_reference.unwrap();
    Some(serde_json::json!({
        "id": id,
        "label": label,
        "from": from,
        "to": to,
        "section": {
            "outside_diameter": { "value": outside_diameter.unwrap(), "unit": stored_unit },
            "wall_thickness": { "value": wall_thickness.unwrap(), "unit": stored_unit }
        },
        "material": material,
        "y_reference": { "x": yrx, "y": yry, "z": yrz },
        "provenance": provenance,
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_section(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Section" || field_path != "sections" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-SECTION-SHAPE-INVALID",
            "blocking",
            "Create-section intents must target object_type `Section` with field_path `sections`."
                .to_string(),
            "Refresh the section creation intent from the explicit section authoring form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    let stored_unit = value_at(model, &["project", "units", "length"]).and_then(Value::as_str);
    checker.unit_state = "passed";
    let Some(stored_unit) = stored_unit else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project length unit metadata is missing; explicit section geometry cannot be accepted.".to_string(),
            "Repair the model document's project.units.length metadata before creating sections.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if unit != stored_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match project length unit `{stored_unit}`; unit conversion is unavailable until the units engine lands."),
            "Enter section dimensions in the project length unit; no silent conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "length" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-section dimension `{dimension}` must be `length`."),
            "Emit section creation intents with length dimension metadata for OD and wall quantities.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if model.get("sections").is_some() && !model.get("sections").is_some_and(Value::is_array) {
        checker.reference_state = "blocked";
        checker.push(
            "OP-SECTION-COLLECTION-MISSING",
            "blocking",
            "Model document does not expose a sections array.".to_string(),
            "Repair the model document before creating sections.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "sections", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Section `{target_ref}` already exists in the current model."),
            "Choose a new stable section id; create operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CREATE-SECTION-PAYLOAD-INVALID",
            "blocking",
            "Create-section payload is not valid JSON.".to_string(),
            "Emit the explicit section payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-SECTION-PAYLOAD-INVALID",
            "blocking",
            "Create-section payload must be a JSON object.".to_string(),
            "Emit id, name, section_type, properties.outside_diameter, properties.wall_thickness, and provenance fields.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let name = record
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let section_type = record
        .get("section_type")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let outside_diameter = quantity_value(record, &["properties", "outside_diameter"], stored_unit);
    let wall_thickness = quantity_value(record, &["properties", "wall_thickness"], stored_unit);
    if id != target_ref
        || name.is_empty()
        || section_type != "pipe"
        || provenance.is_empty()
        || outside_diameter.is_none()
        || wall_thickness.is_none()
    {
        checker.push(
            "OP-CREATE-SECTION-PAYLOAD-INVALID",
            "blocking",
            "Create-section payload must include matching id, non-empty name/provenance, section_type `pipe`, and positive OD/wall quantities in the project length unit.".to_string(),
            "Refresh the section creation intent from explicit user-entered section fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let outside_diameter = outside_diameter.unwrap();
    let wall_thickness = wall_thickness.unwrap();
    if wall_thickness >= outside_diameter / 2.0 {
        checker.push(
            "OP-CREATE-SECTION-PAYLOAD-INVALID",
            "blocking",
            "Create-section wall thickness must be less than the outside-diameter radius."
                .to_string(),
            "Enter physically possible pipe section dimensions before creating the section.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    Some(serde_json::json!({
        "id": id,
        "name": name,
        "section_type": "pipe",
        "properties": {
            "outside_diameter": { "value": outside_diameter, "unit": stored_unit },
            "wall_thickness": { "value": wall_thickness, "unit": stored_unit }
        },
        "provenance": provenance,
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_material(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Material" || field_path != "materials" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-MATERIAL-SHAPE-INVALID",
            "blocking",
            "Create-material intents must target object_type `Material` with field_path `materials`.".to_string(),
            "Refresh the material creation intent from the explicit material authoring form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    let stored_stress_unit =
        value_at(model, &["project", "units", "pressure"]).and_then(Value::as_str);
    checker.unit_state = "passed";
    let Some(stored_stress_unit) = stored_stress_unit else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project pressure/stress unit metadata is missing; explicit material moduli cannot be accepted.".to_string(),
            "Repair the model document's project.units.pressure metadata before creating materials.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if unit != stored_stress_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match project stress unit `{stored_stress_unit}`; unit conversion is unavailable until the units engine lands."),
            "Enter material modulus values in the project stress unit; no silent conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "stress" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-material dimension `{dimension}` must be `stress`."),
            "Emit material creation intents with stress dimension metadata for modulus quantities.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if model.get("materials").is_some() && !model.get("materials").is_some_and(Value::is_array) {
        checker.reference_state = "blocked";
        checker.push(
            "OP-MATERIAL-COLLECTION-MISSING",
            "blocking",
            "Model document does not expose a materials array.".to_string(),
            "Repair the model document before creating materials.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "materials", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Material `{target_ref}` already exists in the current model."),
            "Choose a new stable material id; create operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CREATE-MATERIAL-PAYLOAD-INVALID",
            "blocking",
            "Create-material payload is not valid JSON.".to_string(),
            "Emit the explicit material payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-MATERIAL-PAYLOAD-INVALID",
            "blocking",
            "Create-material payload must be a JSON object.".to_string(),
            "Emit id, label, elastic_modulus, shear_modulus, optional thermal_expansion_coefficient, and provenance fields.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let label = record
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let elastic_modulus = quantity_value(record, &["elastic_modulus"], stored_stress_unit);
    let shear_modulus = quantity_value(record, &["shear_modulus"], stored_stress_unit);
    if id != target_ref
        || label.is_empty()
        || provenance.is_empty()
        || elastic_modulus.is_none()
        || shear_modulus.is_none()
    {
        checker.push(
            "OP-CREATE-MATERIAL-PAYLOAD-INVALID",
            "blocking",
            "Create-material payload must include matching id, non-empty label/provenance, and positive elastic/shear modulus quantities in the project stress unit.".to_string(),
            "Refresh the material creation intent from explicit user-entered material fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let mut material = serde_json::json!({
        "id": id,
        "label": label,
        "elastic_modulus": { "value": elastic_modulus.unwrap(), "unit": stored_stress_unit },
        "shear_modulus": { "value": shear_modulus.unwrap(), "unit": stored_stress_unit },
        "provenance": provenance,
    });
    if record.get("thermal_expansion_coefficient").is_some() {
        let Some(stored_temperature_unit) =
            value_at(model, &["project", "units", "temperature"]).and_then(Value::as_str)
        else {
            checker.unit_state = "blocked";
            checker.push(
                "OP-UNIT-METADATA-MISSING",
                "blocking",
                "Project temperature unit metadata is missing; thermal expansion cannot be accepted.".to_string(),
                "Repair the model document's project.units.temperature metadata before creating thermal expansion quantities.",
                vec![target_ref.to_string()],
            );
            return None;
        };
        let expected_thermal_unit = format!("1/{stored_temperature_unit}");
        let Some(thermal) = quantity_value_any_sign(
            record,
            &["thermal_expansion_coefficient"],
            &expected_thermal_unit,
        ) else {
            checker.push(
                "OP-CREATE-MATERIAL-PAYLOAD-INVALID",
                "blocking",
                format!("Thermal expansion must be a finite quantity in unit `{expected_thermal_unit}`."),
                "Enter thermal expansion in the reciprocal project temperature unit, or omit it.",
                vec![target_ref.to_string()],
            );
            return None;
        };
        material["thermal_expansion_coefficient"] = serde_json::json!({
            "value": thermal,
            "unit": expected_thermal_unit,
        });
    }

    Some(material)
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_support(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Support" || field_path != "supports" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-SUPPORT-SHAPE-INVALID",
            "blocking",
            "Create-support intents must target object_type `Support` with field_path `supports`."
                .to_string(),
            "Refresh the support creation intent from the explicit support authoring form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    checker.unit_state = "passed";
    if unit != "none" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match stored unit `none` for support creation."),
            "Create support records with unit `none`; stiffness support authoring is a later bounded tranche.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-support dimension `{dimension}` must be `dimensionless`."),
            "Emit support creation intents with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if !model.get("supports").is_some_and(Value::is_array) {
        checker.reference_state = "blocked";
        checker.push(
            "OP-SUPPORT-COLLECTION-MISSING",
            "blocking",
            "Model document does not expose a supports array.".to_string(),
            "Repair the model document before creating supports.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "supports", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Support `{target_ref}` already exists in the current model."),
            "Choose a new stable support id; create operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CREATE-SUPPORT-PAYLOAD-INVALID",
            "blocking",
            "Create-support payload is not valid JSON.".to_string(),
            "Emit the explicit support payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-SUPPORT-PAYLOAD-INVALID",
            "blocking",
            "Create-support payload must be a JSON object.".to_string(),
            "Emit id, label, node, restraints, and provenance fields for the new support.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let label = record
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let node = record
        .get("node")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let Some(raw_restraints) = record.get("restraints").and_then(Value::as_array) else {
        checker.push(
            "OP-CREATE-SUPPORT-PAYLOAD-INVALID",
            "blocking",
            "Create-support payload must include a restraints string array.".to_string(),
            "Refresh the support creation intent from explicit user-entered restraint tokens.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let restraints = normalize_restraint_tokens(
        raw_restraints
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        target_ref,
        checker,
    )?;
    if id != target_ref || label.is_empty() || node.is_empty() || provenance.is_empty() {
        checker.push(
            "OP-CREATE-SUPPORT-PAYLOAD-INVALID",
            "blocking",
            "Create-support payload must include matching id and non-empty label/node/provenance fields.".to_string(),
            "Refresh the support creation intent from explicit user-entered support fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "nodes", node).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-SUPPORT-NODE-NOT-FOUND",
            "blocking",
            format!("Support `{target_ref}` references node `{node}`, which is absent from the current model."),
            "Create or select an existing node before creating a support.",
            vec![target_ref.to_string(), node.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    Some(serde_json::json!({
        "id": id,
        "label": label,
        "node": node,
        "restraints": restraints,
        "provenance": provenance,
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_load_case(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Load" || field_path != "load_cases" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-LOAD-CASE-SHAPE-INVALID",
            "blocking",
            "Create-load-case intents must target object_type `Load` with field_path `load_cases`."
                .to_string(),
            "Refresh the load-case creation intent from the explicit Load Cases manager form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    checker.unit_state = "passed";
    if unit != "none" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match stored unit `none` for load-case creation."),
            "Create load-case records with unit `none`; primitive-load quantities are authored separately.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-load-case dimension `{dimension}` must be `dimensionless`."),
            "Emit load-case metadata with dimensionless metadata; primitive-load quantities are authored separately.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "load_cases", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Load case `{target_ref}` already exists in the current model."),
            "Choose a new stable load-case id; create operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CREATE-LOAD-CASE-PAYLOAD-INVALID",
            "blocking",
            "Create-load-case payload is not valid JSON.".to_string(),
            "Emit the explicit load-case metadata payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-LOAD-CASE-PAYLOAD-INVALID",
            "blocking",
            "Create-load-case payload must be a JSON object.".to_string(),
            "Emit id, label, kind, status, provenance, and empty primitive_loads fields for the new load case.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let label = record
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let kind = record
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let status = record
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let primitive_loads_empty = match record.get("primitive_loads") {
        None => true,
        Some(value) => value.as_array().is_some_and(Vec::is_empty),
    };

    if id != target_ref
        || label.is_empty()
        || kind.is_empty()
        || status.is_empty()
        || provenance.is_empty()
        || !primitive_loads_empty
    {
        checker.push(
            "OP-CREATE-LOAD-CASE-PAYLOAD-INVALID",
            "blocking",
            "Create-load-case payload must include matching id, non-empty label/kind/status/provenance, and no primitive load records.".to_string(),
            "Create the load-case shell first; author primitive loads through explicit primitive-load operations.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    Some(serde_json::json!({
        "id": id,
        "label": label,
        "kind": kind,
        "status": status,
        "provenance": provenance,
        "primitive_loads": []
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_primitive_load(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<(String, Value)> {
    if object_type != "Load" || field_path != "primitive_loads" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-PRIMITIVE-LOAD-SHAPE-INVALID",
            "blocking",
            "Create-primitive-load intents must target object_type `Load` with field_path `primitive_loads`."
                .to_string(),
            "Refresh the primitive-load creation intent from the explicit Load Cases manager form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    if find_entity(model, "load_cases", target_ref).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Load case `{target_ref}` was not found in the current model."),
            "Create or select an existing load case before authoring primitive loads.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Ok(payload) = serde_json::from_str::<Value>(after) else {
        checker.push(
            "OP-CREATE-PRIMITIVE-LOAD-PAYLOAD-INVALID",
            "blocking",
            "Create-primitive-load payload is not valid JSON.".to_string(),
            "Emit the explicit primitive-load payload as JSON in change.after.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-PRIMITIVE-LOAD-PAYLOAD-INVALID",
            "blocking",
            "Create-primitive-load payload must be a JSON object.".to_string(),
            "Emit id, category, node target, direction, magnitude, dimension, and provenance fields.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let category = record
        .get("category")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let direction = record
        .get("direction")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let payload_dimension = record
        .get("dimension")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let target_type = record
        .get("target")
        .and_then(Value::as_object)
        .and_then(|target| target.get("type"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let target_node = record
        .get("target")
        .and_then(Value::as_object)
        .and_then(|target| target.get("node"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let target_pipe = record
        .get("target")
        .and_then(Value::as_object)
        .and_then(|target| target.get("pipe"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let target_support = record
        .get("target")
        .and_then(Value::as_object)
        .and_then(|target| target.get("support"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let target_dof = record
        .get("target")
        .and_then(Value::as_object)
        .and_then(|target| target.get("dof"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let magnitude_value = record
        .get("magnitude")
        .and_then(Value::as_object)
        .and_then(|magnitude| magnitude.get("value"))
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite());
    let magnitude_unit = record
        .get("magnitude")
        .and_then(Value::as_object)
        .and_then(|magnitude| magnitude.get("unit"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();

    if !matches!(
        category,
        "concentrated_force"
            | "distributed_force"
            | "concentrated_moment"
            | "pressure"
            | "thermal"
            | "imposed_displacement"
    ) {
        checker.push(
            "OP-CREATE-PRIMITIVE-LOAD-PAYLOAD-INVALID",
            "blocking",
            "Create-primitive-load payload category must be `concentrated_force`, `distributed_force`, `concentrated_moment`, `pressure`, `thermal`, or `imposed_displacement`.".to_string(),
            "Refresh the primitive-load creation intent from explicit user-entered primitive-load fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let imposed_is_rotational =
        category == "imposed_displacement" && matches!(direction, "RX" | "RY" | "RZ");
    let expected_dimension = if category == "distributed_force" {
        "force_per_length"
    } else if category == "concentrated_moment" {
        "moment"
    } else if category == "pressure" {
        "pressure"
    } else if category == "thermal" {
        "temperature_interval"
    } else if category == "imposed_displacement" && imposed_is_rotational {
        "rotation"
    } else if category == "imposed_displacement" {
        "displacement"
    } else {
        "force"
    };
    let stored_force_unit = value_at(model, &["project", "units", "force"]).and_then(Value::as_str);
    let stored_length_unit =
        value_at(model, &["project", "units", "length"]).and_then(Value::as_str);
    let stored_pressure_unit =
        value_at(model, &["project", "units", "pressure"]).and_then(Value::as_str);
    let stored_temperature_unit =
        value_at(model, &["project", "units", "temperature"]).and_then(Value::as_str);
    let stored_angle_unit = value_at(model, &["project", "units", "angle"]).and_then(Value::as_str);
    let requires_force_unit = matches!(
        category,
        "concentrated_force" | "distributed_force" | "concentrated_moment"
    );
    let requires_length_unit = matches!(category, "distributed_force" | "concentrated_moment")
        || (category == "imposed_displacement" && !imposed_is_rotational);
    let requires_angle_unit = category == "imposed_displacement" && imposed_is_rotational;
    checker.unit_state = "passed";
    if requires_force_unit && stored_force_unit.is_none() {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project force unit metadata is missing; primitive force loads cannot be accepted."
                .to_string(),
            "Repair the model document's project.units.force metadata before creating force loads.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if requires_length_unit && stored_length_unit.is_none() {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project length unit metadata is missing; distributed force, concentrated moment, or translational imposed displacement primitive loads cannot be accepted.".to_string(),
            "Repair the model document's project.units.length metadata before creating distributed loads, concentrated moments, or translational imposed displacements.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if category == "pressure" && stored_pressure_unit.is_none() {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project pressure unit metadata is missing; pressure primitive loads cannot be accepted.".to_string(),
            "Repair the model document's project.units.pressure metadata before creating pressure primitives.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if category == "thermal" && stored_temperature_unit.is_none() {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project temperature unit metadata is missing; thermal primitive loads cannot be accepted.".to_string(),
            "Repair the model document's project.units.temperature metadata before creating thermal primitives.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if requires_angle_unit && stored_angle_unit.is_none() {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-METADATA-MISSING",
            "blocking",
            "Project angle unit metadata is missing; rotational imposed displacement primitives cannot be accepted.".to_string(),
            "Repair the model document's project.units.angle metadata before creating rotational imposed displacements.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let expected_unit = if category == "distributed_force" {
        format!(
            "{}/{}",
            stored_force_unit.unwrap_or(""),
            stored_length_unit.unwrap_or("")
        )
    } else if category == "concentrated_moment" {
        format!(
            "{}*{}",
            stored_force_unit.unwrap_or(""),
            stored_length_unit.unwrap_or("")
        )
    } else if category == "pressure" {
        stored_pressure_unit.unwrap_or("").to_string()
    } else if category == "thermal" {
        stored_temperature_unit.unwrap_or("").to_string()
    } else if category == "imposed_displacement" && imposed_is_rotational {
        stored_angle_unit.unwrap_or("").to_string()
    } else if category == "imposed_displacement" {
        stored_length_unit.unwrap_or("").to_string()
    } else {
        stored_force_unit.unwrap_or("").to_string()
    };
    if unit != expected_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match expected primitive-load unit `{expected_unit}`; unit conversion is unavailable until the units engine lands."),
            "Enter primitive load values in the project unit basis; no silent conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != expected_dimension {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-primitive-load dimension `{dimension}` must be `{expected_dimension}` for category `{category}`."),
            "Emit primitive-load creation intents with dimension metadata that matches the selected primitive-load category.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let direction_valid = if category == "concentrated_moment" {
        matches!(direction, "rotation_x" | "rotation_y" | "rotation_z")
    } else if category == "imposed_displacement" {
        matches!(direction, "UX" | "UY" | "UZ" | "RX" | "RY" | "RZ")
    } else {
        matches!(direction, "global_x" | "global_y" | "global_z")
    };

    if id.is_empty()
        || !direction_valid
        || (category == "concentrated_force" && (target_type != "node" || target_node.is_empty()))
        || (category == "concentrated_moment" && (target_type != "node" || target_node.is_empty()))
        || (matches!(category, "distributed_force" | "pressure" | "thermal")
            && (target_type != "element" || target_pipe.is_empty()))
        || (category == "imposed_displacement"
            && (target_type != "support"
                || target_support.is_empty()
                || target_dof != direction
                || !matches!(direction, "UX" | "UY" | "UZ" | "RX" | "RY" | "RZ")))
        || magnitude_value.is_none()
        || magnitude_unit != expected_unit
        || payload_dimension != expected_dimension
        || provenance.is_empty()
    {
        checker.push(
            "OP-CREATE-PRIMITIVE-LOAD-PAYLOAD-INVALID",
            "blocking",
            "Create-primitive-load payload must include non-empty id/provenance, a supported category with matching node, pipe, or support target, compatible direction, finite magnitude in the expected unit, and matching dimension.".to_string(),
            "Refresh the primitive-load creation intent from explicit user-entered primitive-load fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_primitive_load(model, id).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Primitive load `{id}` already exists in the current model."),
            "Choose a new stable primitive-load id; create operations never overwrite existing primitive loads.",
            vec![target_ref.to_string(), id.to_string()],
        );
        return None;
    }
    if category == "concentrated_force" && find_entity(model, "nodes", target_node).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND",
            "blocking",
            format!("Primitive load `{id}` references node `{target_node}`, which is absent from the current model."),
            "Select an existing node target before authoring a concentrated force.",
            vec![target_ref.to_string(), target_node.to_string()],
        );
        return None;
    }
    if category == "concentrated_moment" && find_entity(model, "nodes", target_node).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND",
            "blocking",
            format!("Primitive load `{id}` references node `{target_node}`, which is absent from the current model."),
            "Select an existing node target before authoring a concentrated moment.",
            vec![target_ref.to_string(), target_node.to_string()],
        );
        return None;
    }
    if matches!(category, "distributed_force" | "pressure" | "thermal")
        && find_entity(model, "pipe_segments", target_pipe).is_none()
    {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND",
            "blocking",
            format!("Primitive load `{id}` references pipe `{target_pipe}`, which is absent from the current model."),
            "Select an existing pipe target before authoring an element-targeted primitive load.",
            vec![target_ref.to_string(), target_pipe.to_string()],
        );
        return None;
    }
    if category == "imposed_displacement"
        && find_entity(model, "supports", target_support).is_none()
    {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND",
            "blocking",
            format!("Primitive load `{id}` references support `{target_support}`, which is absent from the current model."),
            "Select an existing support target before authoring an imposed displacement.",
            vec![target_ref.to_string(), target_support.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";

    let target = if category == "imposed_displacement" {
        serde_json::json!({ "type": "support", "support": target_support, "dof": target_dof })
    } else if matches!(category, "distributed_force" | "pressure" | "thermal") {
        serde_json::json!({ "type": "element", "pipe": target_pipe })
    } else {
        serde_json::json!({ "type": "node", "node": target_node })
    };

    Some((
        target_ref.to_string(),
        serde_json::json!({
            "id": id,
            "category": category,
            "target": target,
            "direction": direction,
            "magnitude": { "value": magnitude_value.unwrap(), "unit": expected_unit },
            "dimension": expected_dimension,
            "provenance": provenance,
        }),
    ))
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_combination(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<Value> {
    if object_type != "Combination" || field_path != "combinations" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-COMBINATION-SHAPE-INVALID",
            "blocking",
            "Create-combination intents must target object_type `Combination` with field_path `combinations`.".to_string(),
            "Refresh the combination creation intent from the explicit Load Cases manager form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    checker.unit_state = "passed";
    if unit != "none" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!(
                "Intent unit `{unit}` does not match stored unit `none` for combination creation."
            ),
            "Create combination records with unit `none`; term factors are dimensionless scalars.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-combination dimension `{dimension}` must be `dimensionless`."),
            "Emit combination creation with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if model.get("combinations").is_some()
        && !model.get("combinations").is_some_and(Value::is_array)
    {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-COLLECTION-MISSING",
            "blocking",
            "Model document does not expose a combinations array.".to_string(),
            "Repair the model document before creating combinations.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "combinations", target_ref).is_some() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-ALREADY-EXISTS",
            "blocking",
            format!("Combination `{target_ref}` already exists in the current model."),
            "Choose a new stable combination id; create operations never overwrite existing entities.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let payload: Value = match serde_json::from_str(after) {
        Ok(payload) => payload,
        Err(_) => {
            checker.push(
                "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
                "blocking",
                "Create-combination payload is not valid JSON.".to_string(),
                "Emit the explicit combination payload as JSON in change.after.",
                vec![target_ref.to_string()],
            );
            return None;
        }
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
            "blocking",
            "Create-combination payload must be a JSON object.".to_string(),
            "Emit id, label, basis, terms, and provenance fields for the new combination.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let id = record
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let label = record
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let basis = record
        .get("basis")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let provenance = record
        .get("provenance")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let Some(term_payloads) = record.get("terms").and_then(Value::as_array) else {
        checker.push(
            "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
            "blocking",
            "Create-combination payload must include at least one explicit term.".to_string(),
            "Refresh the combination creation intent from explicit user-entered combination fields.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if id != target_ref
        || label.is_empty()
        || basis != "mechanics"
        || provenance.is_empty()
        || term_payloads.is_empty()
    {
        checker.push(
            "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
            "blocking",
            "Create-combination payload must include matching id, non-empty label/provenance, basis `mechanics`, and at least one explicit term.".to_string(),
            "Refresh the combination creation intent from explicit user-entered combination fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let mut terms = Vec::new();
    let mut seen_load_cases = std::collections::HashSet::new();
    for raw_term in term_payloads {
        let Some(term) = raw_term.as_object() else {
            checker.push(
                "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
                "blocking",
                "Create-combination terms must be JSON objects.".to_string(),
                "Emit each term as { load_case, factor }.",
                vec![target_ref.to_string()],
            );
            return None;
        };
        let load_case = term
            .get("load_case")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim();
        let factor = term.get("factor").and_then(Value::as_f64);
        let factor_is_finite = factor.map(f64::is_finite).unwrap_or(false);
        if load_case.is_empty() || !factor_is_finite {
            checker.push(
                "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
                "blocking",
                "Create-combination terms must include load_case references and finite numeric factors.".to_string(),
                "Refresh the combination creation intent from explicit user-entered term fields.",
                vec![target_ref.to_string()],
            );
            return None;
        }
        if !seen_load_cases.insert(load_case.to_string()) {
            checker.push(
                "OP-COMBINATION-TERM-DUPLICATE",
                "blocking",
                format!("Combination `{target_ref}` repeats load case `{load_case}` in its initial terms."),
                "Use one explicit factor per load case; duplicate operands are blocked.",
                vec![target_ref.to_string(), load_case.to_string()],
            );
            return None;
        }
        if find_entity(model, "load_cases", load_case).is_none() {
            checker.reference_state = "blocked";
            checker.push(
                "OP-COMBINATION-TERM-LOAD-NOT-FOUND",
                "blocking",
                format!("Combination term references load case `{load_case}`, which is absent from the current model."),
                "Select an existing load case before creating a combination.",
                vec![target_ref.to_string(), load_case.to_string()],
            );
            return None;
        }
        terms.push(serde_json::json!({ "load_case": load_case, "factor": factor.unwrap() }));
    }

    checker.reference_state = "passed";
    Some(serde_json::json!({
        "id": id,
        "label": label,
        "basis": basis,
        "terms": terms,
        "provenance": provenance,
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_create_combination_term(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<(String, Value)> {
    if object_type != "Combination" || field_path != "terms" {
        checker.schema_blocked = true;
        checker.push(
            "OP-CREATE-COMBINATION-TERM-SHAPE-INVALID",
            "blocking",
            "Create-combination-term intents must target object_type `Combination` with field_path `terms`.".to_string(),
            "Refresh the combination-term creation intent from the explicit Load Cases manager form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    check_before("not_present", before, target_ref, field_path, checker);

    checker.unit_state = "passed";
    if unit != "none" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match stored unit `none` for combination term creation."),
            "Create combination terms with unit `none`; term factors are dimensionless scalars.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Create-combination-term dimension `{dimension}` must be `dimensionless`."),
            "Emit combination-term factors with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Some(combination) = find_entity(model, "combinations", target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Combination `{target_ref}` was not found in the current model."),
            "Select an existing combination before creating a term.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let payload: Value = match serde_json::from_str(after) {
        Ok(payload) => payload,
        Err(_) => {
            checker.push(
                "OP-CREATE-COMBINATION-TERM-PAYLOAD-INVALID",
                "blocking",
                "Create-combination-term payload is not valid JSON.".to_string(),
                "Emit the explicit combination-term payload as JSON in change.after.",
                vec![target_ref.to_string()],
            );
            return None;
        }
    };
    let Some(record) = payload.as_object() else {
        checker.push(
            "OP-CREATE-COMBINATION-TERM-PAYLOAD-INVALID",
            "blocking",
            "Create-combination-term payload must be a JSON object.".to_string(),
            "Emit load_case and factor fields for the new term.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let load_case = record
        .get("load_case")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let factor = record.get("factor").and_then(Value::as_f64);
    let factor_is_finite = factor.map(f64::is_finite).unwrap_or(false);
    if load_case.is_empty() || !factor_is_finite {
        checker.push(
            "OP-CREATE-COMBINATION-TERM-PAYLOAD-INVALID",
            "blocking",
            "Create-combination-term payload must include a load_case reference and finite numeric factor.".to_string(),
            "Refresh the combination-term creation intent from explicit user-entered term fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if find_entity(model, "load_cases", load_case).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-TERM-LOAD-NOT-FOUND",
            "blocking",
            format!("Combination term references load case `{load_case}`, which is absent from the current model."),
            "Select an existing load case before creating a combination term.",
            vec![target_ref.to_string(), load_case.to_string()],
        );
        return None;
    }
    if !combination.get("terms").is_some_and(Value::is_array) {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-TERMS-MISSING",
            "blocking",
            format!("Combination `{target_ref}` does not expose a terms array."),
            "Repair the model document before creating combination terms.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";
    Some((
        target_ref.to_string(),
        serde_json::json!({ "load_case": load_case, "factor": factor.unwrap() }),
    ))
}

#[allow(clippy::too_many_arguments)]
fn resolve_delete_combination_term(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<(String, usize)> {
    if object_type != "Combination" {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-COMBINATION-TERM-SHAPE-INVALID",
            "blocking",
            "Delete-combination-term intents must target object_type `Combination`.".to_string(),
            "Refresh the combination-term delete intent from the explicit Load Cases manager form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let Some(term_index) = combination_term_index_from_field_path(field_path) else {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-COMBINATION-TERM-SHAPE-INVALID",
            "blocking",
            "Delete-combination-term intents must target an indexed field_path like `terms.2`."
                .to_string(),
            "Refresh the combination-term delete intent from the selected combination term row.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if after != "not_present" {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-DELETE-AFTER-VALUE-INVALID",
            "blocking",
            "Delete-combination-term intents must use after-value `not_present`.".to_string(),
            "Re-queue the delete intent from the selected combination term row.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    checker.unit_state = "passed";
    if unit != "none" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match stored unit `none` for combination term deletion."),
            "Delete combination terms with unit `none`; term factors are dimensionless scalars.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Delete-combination-term dimension `{dimension}` must be `dimensionless`."),
            "Emit combination-term deletion with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Some(combination) = find_entity(model, "combinations", target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Combination `{target_ref}` was not found in the current model."),
            "Select an existing combination before deleting a term.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(terms) = combination.get("terms").and_then(Value::as_array) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-TERMS-MISSING",
            "blocking",
            format!("Combination `{target_ref}` does not expose a terms array."),
            "Repair the model document before deleting combination terms.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(term) = terms.get(term_index) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-TERM-NOT-FOUND",
            "blocking",
            format!("Combination `{target_ref}` does not have a term at index {term_index}."),
            "Re-queue the delete intent from the current combination term list.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let load_case = term
        .get("load_case")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let factor = term.get("factor").and_then(Value::as_f64);
    let factor_is_finite = factor.map(f64::is_finite).unwrap_or(false);
    if load_case.is_empty() || !factor_is_finite {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-TERM-PAYLOAD-INVALID",
            "blocking",
            format!("Combination `{target_ref}` term {term_index} is not a valid {{ load_case, factor }} record."),
            "Repair the model document before deleting combination terms.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    checker.reference_state = "passed";
    check_before(
        &combination_term_display(load_case, factor.unwrap()),
        before,
        target_ref,
        field_path,
        checker,
    );
    Some((target_ref.to_string(), term_index))
}

#[allow(clippy::too_many_arguments)]
fn resolve_field(
    model: &Value,
    object_type: &str,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    checker: &mut Checker,
) -> Option<ResolvedField> {
    let Some(collection) = collection_for(object_type) else {
        checker.schema_blocked = true;
        checker.push(
            "OP-OBJECT-TYPE-UNSUPPORTED",
            "blocking",
            format!("Target object type `{object_type}` is not an editable model collection."),
            "Target one of the structured model collections.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let Some(entity) = find_entity(model, collection, target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Target `{target_ref}` was not found in model collection `{collection}`."),
            "Refresh the editor intent against the current model document.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    if let Some((_, _, reason)) =
        DEFERRED_FIELDS
            .iter()
            .find(|(deferred_type, deferred_path, _)| {
                *deferred_type == object_type && *deferred_path == field_path
            })
    {
        checker.push(
            "OP-FIELD-EDIT-DEFERRED",
            "blocking",
            format!("Editing `{object_type}.{field_path}` is not applied in this tranche: {reason}."),
            "Wait for the named completion-plan item; the intent remains reviewable and is not silently applied.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    // Dynamic paths: primitive_loads.<index>.magnitude.value and
    // terms.<index>.factor are existing child fields, not whole-record
    // creation/editing.
    let rule_kind = if object_type == "Load" && is_primitive_magnitude_path(field_path) {
        Some(FieldKind::Quantity {
            require_positive: false,
            unit_source: UnitSource::SiblingUnitField,
        })
    } else if object_type == "Combination" && is_combination_term_factor_path(field_path) {
        Some(FieldKind::Number {
            require_positive: false,
        })
    } else {
        field_rules(object_type)
            .iter()
            .find(|rule| rule.field_path == field_path)
            .map(|rule| rule.kind)
    };
    let Some(kind) = rule_kind else {
        checker.push(
            "OP-FIELD-PATH-UNSUPPORTED",
            "blocking",
            format!("Field path `{field_path}` on `{object_type}` is not an editable field in the apply-operation seam."),
            "Edit one of the supported structured fields for this object type.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let segments: Vec<String> = field_path.split('.').map(str::to_string).collect();

    match kind {
        FieldKind::Text => {
            let current = value_at_segments(entity, &segments)
                .and_then(Value::as_str)
                .map(str::to_string);
            let Some(current) = current else {
                checker.push(
                    "OP-FIELD-NOT-PRESENT",
                    "blocking",
                    format!("Field `{field_path}` is not present as text on `{target_ref}`."),
                    "Refresh the editor intent against the current model document.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            check_before(&current, before, target_ref, field_path, checker);
            let trimmed = after.trim();
            if trimmed.is_empty() {
                checker.push(
                    "OP-VALUE-EMPTY",
                    "blocking",
                    format!("Replacement value for `{field_path}` is empty."),
                    "Provide a non-empty replacement value (or `TBD` to mark an explicit unknown).",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            Some(ResolvedField {
                kind,
                current_display: current,
                applied_value: Value::String(trimmed.to_string()),
                segments,
            })
        }
        FieldKind::Number { require_positive } => {
            let value_node = value_at_segments(entity, &segments);
            let Some(current_number) = value_node.and_then(Value::as_f64) else {
                checker.push(
                    "OP-NUMBER-FIELD-MISSING",
                    "blocking",
                    format!("Numeric field `{field_path}` is not present on `{target_ref}`."),
                    "Refresh the editor intent against the current model document.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            checker.unit_state = "passed";
            if unit != "none" {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
                    "blocking",
                    format!(
                        "Intent unit `{unit}` does not match stored unit `none` for dimensionless numeric field `{field_path}`."
                    ),
                    "Enter the scalar value with unit `none`; no silent conversion is performed.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            if !CANONICAL_DIMENSIONS.contains(&dimension) {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-UNIT-DIMENSION-UNKNOWN",
                    "blocking",
                    format!("Dimension `{dimension}` is outside the canonical dimension vocabulary."),
                    "Use a canonical dimension token; vocabulary changes await the D-01 unit-catalog ruling.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            if dimension != "dimensionless" {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-UNIT-DIMENSION-MISMATCH",
                    "blocking",
                    format!(
                        "Dimensionless numeric field `{field_path}` cannot be edited with dimension `{dimension}`."
                    ),
                    "Use dimension `dimensionless` for scalar combination factors.",
                    vec![target_ref.to_string()],
                );
                return None;
            }

            check_before_numeric(current_number, before, target_ref, field_path, checker);

            let Some(parsed) = parse_finite_number(after) else {
                checker.push(
                    "OP-VALUE-NOT-NUMERIC",
                    "blocking",
                    format!(
                        "Replacement value `{after}` for `{field_path}` is not a finite number."
                    ),
                    "Provide a finite numeric value.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            if require_positive && parsed <= 0.0 {
                checker.push(
                    "OP-VALUE-NOT-POSITIVE",
                    "blocking",
                    format!(
                        "Replacement value `{after}` for `{field_path}` must be greater than zero."
                    ),
                    "Provide a positive scalar value.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            let Some(number) = Number::from_f64(parsed) else {
                checker.push(
                    "OP-VALUE-NOT-NUMERIC",
                    "blocking",
                    format!("Replacement value `{after}` for `{field_path}` cannot be encoded as a JSON number."),
                    "Provide a finite numeric value.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            Some(ResolvedField {
                kind,
                current_display: display_number(current_number),
                applied_value: Value::Number(number),
                segments,
            })
        }
        FieldKind::Quantity {
            require_positive,
            unit_source,
        } => {
            let value_node = value_at_segments(entity, &segments);
            let Some(current_number) = value_node.and_then(Value::as_f64) else {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-QUANTITY-OBJECT-MISSING",
                    "blocking",
                    format!(
                        "Quantity field `{field_path}` is not present on `{target_ref}`; authoring a new quantity requires explicit unit entry (completion plan Phase B, decision D-01)."
                    ),
                    "Author new quantity fields once unit entry exists; existing quantities remain editable.",
                    vec![target_ref.to_string()],
                );
                return None;
            };

            // Unit metadata: the intent must carry the field's stored unit
            // basis; conversion is unavailable until the units engine
            // (Phase B). Sibling-unit quantities carry their own unit; bare
            // numeric fields resolve through the project unit system.
            let stored_unit = match unit_source {
                UnitSource::SiblingUnitField => {
                    let mut unit_segments = segments.clone();
                    unit_segments.pop();
                    unit_segments.push("unit".to_string());
                    value_at_segments(entity, &unit_segments)
                        .and_then(Value::as_str)
                        .map(str::to_string)
                }
                UnitSource::ProjectUnits(unit_key) => model
                    .get("project")
                    .and_then(|project| project.get("units"))
                    .and_then(|units| units.get(unit_key))
                    .and_then(Value::as_str)
                    .map(str::to_string),
            };
            checker.unit_state = "passed";
            match stored_unit {
                None => {
                    checker.unit_state = "blocked";
                    checker.push(
                        "OP-UNIT-METADATA-MISSING",
                        "blocking",
                        format!("Stored quantity `{field_path}` on `{target_ref}` has no unit metadata."),
                        "Repair the model document's unit metadata before editing this quantity.",
                        vec![target_ref.to_string()],
                    );
                    return None;
                }
                Some(stored) => {
                    if unit != stored {
                        checker.unit_state = "blocked";
                        checker.push(
                            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
                            "blocking",
                            format!(
                                "Intent unit `{unit}` does not match stored unit `{stored}` for `{field_path}`; unit conversion is unavailable until the units engine lands (completion plan Phase B, decision D-01)."
                            ),
                            "Enter the value in the stored unit; no silent conversion is performed.",
                            vec![target_ref.to_string()],
                        );
                        return None;
                    }
                }
            }
            if !CANONICAL_DIMENSIONS.contains(&dimension) {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-UNIT-DIMENSION-UNKNOWN",
                    "blocking",
                    format!("Dimension `{dimension}` is outside the canonical dimension vocabulary."),
                    "Use a canonical dimension token; vocabulary changes await the D-01 unit-catalog ruling.",
                    vec![target_ref.to_string()],
                );
                return None;
            }

            check_before_numeric(current_number, before, target_ref, field_path, checker);

            let Some(parsed) = parse_finite_number(after) else {
                checker.push(
                    "OP-VALUE-NOT-NUMERIC",
                    "blocking",
                    format!(
                        "Replacement value `{after}` for `{field_path}` is not a finite number."
                    ),
                    "Provide a finite numeric value in the stored unit.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            if require_positive && parsed <= 0.0 {
                checker.push(
                    "OP-VALUE-NOT-POSITIVE",
                    "blocking",
                    format!("Replacement value `{after}` for `{field_path}` must be greater than zero to remain physically meaningful."),
                    "Provide a positive value for geometric and stiffness quantities.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            let Some(number) = Number::from_f64(parsed) else {
                checker.push(
                    "OP-VALUE-NOT-NUMERIC",
                    "blocking",
                    format!("Replacement value `{after}` for `{field_path}` cannot be encoded as a JSON number."),
                    "Provide a finite numeric value in the stored unit.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            Some(ResolvedField {
                kind,
                current_display: display_number(current_number),
                applied_value: Value::Number(number),
                segments,
            })
        }
        FieldKind::EntityRef {
            collection: ref_collection,
        } => {
            let current = value_at_segments(entity, &segments)
                .and_then(Value::as_str)
                .map(str::to_string);
            let Some(current) = current else {
                checker.push(
                    "OP-FIELD-NOT-PRESENT",
                    "blocking",
                    format!("Reference field `{field_path}` is not present on `{target_ref}`."),
                    "Refresh the editor intent against the current model document.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            check_before(&current, before, target_ref, field_path, checker);
            let replacement = after.trim();
            if find_entity(model, ref_collection, replacement).is_none() {
                checker.reference_state = "blocked";
                checker.push(
                    "OP-REFERENCE-NOT-FOUND",
                    "blocking",
                    format!("Replacement reference `{replacement}` does not exist in model collection `{ref_collection}`."),
                    "Reference an existing entity id; referenced entities are not created implicitly.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            if checker.reference_state == "not_run" {
                checker.reference_state = "passed";
            }
            Some(ResolvedField {
                kind,
                current_display: current,
                applied_value: Value::String(replacement.to_string()),
                segments,
            })
        }
        FieldKind::RestraintSet => {
            let current_tokens: Vec<String> = value_at_segments(entity, &segments)
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            let current_display = current_tokens.join(", ");
            check_before(&current_display, before, target_ref, field_path, checker);

            let tokens = normalize_restraint_tokens(
                after.split(',').map(str::to_string).collect(),
                target_ref,
                checker,
            )?;
            Some(ResolvedField {
                kind,
                current_display,
                applied_value: Value::Array(tokens.into_iter().map(Value::String).collect()),
                segments,
            })
        }
    }
}

fn check_before(
    current: &str,
    claimed_before: &str,
    target_ref: &str,
    field_path: &str,
    checker: &mut Checker,
) {
    if current == claimed_before {
        checker.before_state = "passed";
    } else {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-STALE-BEFORE-VALUE",
            "blocking",
            format!(
                "Intent before-value `{claimed_before}` no longer matches the current value `{current}` of `{field_path}`."
            ),
            "Re-queue the edit from the current model state; stale intents are not applied.",
            vec![target_ref.to_string()],
        );
    }
}

fn check_before_numeric(
    current: f64,
    claimed_before: &str,
    target_ref: &str,
    field_path: &str,
    checker: &mut Checker,
) {
    // Numeric comparison avoids cross-language float formatting differences
    // between the UI (ECMAScript number-to-string) and this crate.
    let matches = parse_finite_number(claimed_before)
        .map(|claimed| claimed == current)
        .unwrap_or(false);
    if matches {
        checker.before_state = "passed";
    } else {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-STALE-BEFORE-VALUE",
            "blocking",
            format!(
                "Intent before-value `{claimed_before}` no longer matches the current value `{}` of `{field_path}`.",
                display_number(current)
            ),
            "Re-queue the edit from the current model state; stale intents are not applied.",
            vec![target_ref.to_string()],
        );
    }
}

fn apply_resolved_field(
    model: &mut Value,
    object_type: &str,
    target_ref: &str,
    field: &ResolvedField,
    checker: &mut Checker,
) -> bool {
    let Some(collection) = collection_for(object_type) else {
        return false;
    };
    let Some(entity) = find_entity_mut(model, collection, target_ref) else {
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Target `{target_ref}` disappeared from `{collection}` during apply."),
            "Refresh the editor intent against the current model document.",
            vec![target_ref.to_string()],
        );
        return false;
    };
    let Some(slot) = value_at_segments_mut(entity, &field.segments) else {
        checker.push(
            "OP-FIELD-NOT-PRESENT",
            "blocking",
            format!(
                "Field path `{}` could not be resolved for write on `{target_ref}`.",
                field.segments.join(".")
            ),
            "Refresh the editor intent against the current model document.",
            vec![target_ref.to_string()],
        );
        return false;
    };
    *slot = field.applied_value.clone();
    let _ = field.kind;
    true
}

fn apply_created_node(model: &mut Value, node: &Value) -> bool {
    let Some(nodes) = model.get_mut("nodes").and_then(Value::as_array_mut) else {
        return false;
    };
    nodes.push(node.clone());
    true
}

fn apply_created_pipe(model: &mut Value, pipe: &Value) -> bool {
    let Some(pipes) = model.get_mut("pipe_segments").and_then(Value::as_array_mut) else {
        return false;
    };
    pipes.push(pipe.clone());
    true
}

fn apply_created_section(model: &mut Value, section: &Value) -> bool {
    if model.get("sections").is_none() {
        model["sections"] = Value::Array(Vec::new());
    }
    let Some(sections) = model.get_mut("sections").and_then(Value::as_array_mut) else {
        return false;
    };
    sections.push(section.clone());
    true
}

fn apply_created_material(model: &mut Value, material: &Value) -> bool {
    if model.get("materials").is_none() {
        model["materials"] = Value::Array(Vec::new());
    }
    let Some(materials) = model.get_mut("materials").and_then(Value::as_array_mut) else {
        return false;
    };
    materials.push(material.clone());
    true
}

fn apply_created_support(model: &mut Value, support: &Value) -> bool {
    let Some(supports) = model.get_mut("supports").and_then(Value::as_array_mut) else {
        return false;
    };
    supports.push(support.clone());
    true
}

fn apply_created_load_case(model: &mut Value, load_case: &Value) -> bool {
    let Some(load_cases) = model.get_mut("load_cases").and_then(Value::as_array_mut) else {
        return false;
    };
    load_cases.push(load_case.clone());
    true
}

fn apply_created_primitive_load(
    model: &mut Value,
    load_case_id: &str,
    primitive_load: &Value,
) -> bool {
    let Some(load_case) = find_entity_mut(model, "load_cases", load_case_id) else {
        return false;
    };
    if load_case.get("primitive_loads").is_none() {
        load_case["primitive_loads"] = Value::Array(Vec::new());
    }
    let Some(primitive_loads) = load_case
        .get_mut("primitive_loads")
        .and_then(Value::as_array_mut)
    else {
        return false;
    };
    primitive_loads.push(primitive_load.clone());
    true
}

fn apply_created_combination(model: &mut Value, combination: &Value) -> bool {
    if model.get("combinations").is_none() {
        model["combinations"] = Value::Array(Vec::new());
    }
    let Some(combinations) = model.get_mut("combinations").and_then(Value::as_array_mut) else {
        return false;
    };
    combinations.push(combination.clone());
    true
}

fn apply_created_combination_term(model: &mut Value, combination_id: &str, term: &Value) -> bool {
    let Some(combination) = find_entity_mut(model, "combinations", combination_id) else {
        return false;
    };
    let Some(terms) = combination.get_mut("terms").and_then(Value::as_array_mut) else {
        return false;
    };
    terms.push(term.clone());
    true
}

fn apply_deleted_combination_term(
    model: &mut Value,
    combination_id: &str,
    term_index: usize,
) -> bool {
    let Some(combination) = find_entity_mut(model, "combinations", combination_id) else {
        return false;
    };
    let Some(terms) = combination.get_mut("terms").and_then(Value::as_array_mut) else {
        return false;
    };
    if term_index >= terms.len() {
        return false;
    }
    terms.remove(term_index);
    true
}

fn model_basis_evidence(model: &Value, claimed_model_hash: Option<&Value>) -> ModelBasisEvidence {
    let backend_model_hash = format!("sha256:{}", sha256_hex(&canonical_json(model)));
    match claimed_model_hash {
        Some(claim) if !claim.is_null() => {
            let claimed_value = claim
                .get("value")
                .and_then(Value::as_str)
                .unwrap_or("claimed_hash_value_missing")
                .to_string();
            let claimed_canonicalization = claim
                .get("canonicalization")
                .and_then(Value::as_str)
                .unwrap_or("claimed_canonicalization_missing")
                .to_string();
            ModelBasisEvidence {
                claimed_model_hash: claimed_value,
                claimed_hash_canonicalization: claimed_canonicalization,
                backend_model_hash,
                backend_canonicalization: BACKEND_CANONICALIZATION.to_string(),
                // Both lanes share the RFC 8785 form since H5, but a claimed
                // hash may still describe an older document state or an older
                // canonicalization, so the two hashes are recorded side by
                // side; equality is not evaluated and not claimed — the
                // before-state check remains the staleness guard.
                binding_status: "claimed_hash_echoed_cross_canonicalization_equality_not_evaluated"
                    .to_string(),
            }
        }
        _ => ModelBasisEvidence {
            claimed_model_hash: "not_provided".to_string(),
            claimed_hash_canonicalization: "not_provided".to_string(),
            backend_model_hash,
            backend_canonicalization: BACKEND_CANONICALIZATION.to_string(),
            binding_status: "no_claimed_hash_before_state_check_is_the_staleness_guard".to_string(),
        },
    }
}

fn is_primitive_magnitude_path(field_path: &str) -> bool {
    let segments: Vec<&str> = field_path.split('.').collect();
    segments.len() == 4
        && segments[0] == "primitive_loads"
        && segments[1].chars().all(|ch| ch.is_ascii_digit())
        && !segments[1].is_empty()
        && segments[2] == "magnitude"
        && segments[3] == "value"
}

fn is_combination_term_factor_path(field_path: &str) -> bool {
    let segments: Vec<&str> = field_path.split('.').collect();
    segments.len() == 3
        && segments[0] == "terms"
        && segments[1].chars().all(|ch| ch.is_ascii_digit())
        && !segments[1].is_empty()
        && segments[2] == "factor"
}

fn combination_term_index_from_field_path(field_path: &str) -> Option<usize> {
    let segments: Vec<&str> = field_path.split('.').collect();
    if segments.len() == 2
        && segments[0] == "terms"
        && segments[1].chars().all(|ch| ch.is_ascii_digit())
        && !segments[1].is_empty()
    {
        segments[1].parse::<usize>().ok()
    } else {
        None
    }
}

fn combination_term_display(load_case: &str, factor: f64) -> String {
    format!("{load_case} x {}", display_number(factor))
}

fn quantity_value(
    record: &serde_json::Map<String, Value>,
    path: &[&str],
    expected_unit: &str,
) -> Option<f64> {
    let quantity = value_in_object(record, path)?.as_object()?;
    let unit = quantity.get("unit").and_then(Value::as_str)?;
    if unit != expected_unit {
        return None;
    }
    let value = quantity.get("value").and_then(Value::as_f64)?;
    (value.is_finite() && value > 0.0).then_some(value)
}

fn quantity_value_any_sign(
    record: &serde_json::Map<String, Value>,
    path: &[&str],
    expected_unit: &str,
) -> Option<f64> {
    let quantity = value_in_object(record, path)?.as_object()?;
    let unit = quantity.get("unit").and_then(Value::as_str)?;
    if unit != expected_unit {
        return None;
    }
    let value = quantity.get("value").and_then(Value::as_f64)?;
    value.is_finite().then_some(value)
}

fn vector_value(record: &serde_json::Map<String, Value>, key: &str) -> Option<(f64, f64, f64)> {
    let vector = record.get(key)?.as_object()?;
    let x = vector.get("x").and_then(Value::as_f64)?;
    let y = vector.get("y").and_then(Value::as_f64)?;
    let z = vector.get("z").and_then(Value::as_f64)?;
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return None;
    }
    (x != 0.0 || y != 0.0 || z != 0.0).then_some((x, y, z))
}

fn normalize_restraint_tokens(
    raw_tokens: Vec<String>,
    target_ref: &str,
    checker: &mut Checker,
) -> Option<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    for raw in raw_tokens {
        let token = raw.trim().to_ascii_uppercase();
        if token.is_empty() {
            continue;
        }
        if !RESTRAINT_TOKENS.contains(&token.as_str()) {
            checker.push(
                "OP-RESTRAINT-TOKEN-INVALID",
                "blocking",
                format!(
                    "Restraint token `{token}` is not in the restraint vocabulary {}.",
                    RESTRAINT_TOKENS.join("/")
                ),
                "Use comma-separated restraint direction tokens from the model vocabulary.",
                vec![target_ref.to_string()],
            );
            return None;
        }
        if !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    if tokens.is_empty() {
        checker.push(
            "OP-RESTRAINT-SET-EMPTY",
            "blocking",
            "Replacement restraint set is empty; removing a support entirely is a delete operation, not a field edit.".to_string(),
            "Provide at least one restraint direction token.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    Some(tokens)
}

fn value_in_object<'a>(
    record: &'a serde_json::Map<String, Value>,
    path: &[&str],
) -> Option<&'a Value> {
    let mut current = record.get(*path.first()?)?;
    for segment in &path[1..] {
        current = current.get(*segment)?;
    }
    Some(current)
}

fn find_entity<'a>(model: &'a Value, collection: &str, entity_ref: &str) -> Option<&'a Value> {
    model
        .get(collection)?
        .as_array()?
        .iter()
        .find(|item| item.get("id").and_then(Value::as_str) == Some(entity_ref))
}

fn find_primitive_load<'a>(model: &'a Value, primitive_ref: &str) -> Option<&'a Value> {
    model
        .get("load_cases")?
        .as_array()?
        .iter()
        .filter_map(|load_case| load_case.get("primitive_loads")?.as_array())
        .flat_map(|primitive_loads| primitive_loads.iter())
        .find(|item| item.get("id").and_then(Value::as_str) == Some(primitive_ref))
}

fn find_entity_mut<'a>(
    model: &'a mut Value,
    collection: &str,
    entity_ref: &str,
) -> Option<&'a mut Value> {
    model
        .get_mut(collection)?
        .as_array_mut()?
        .iter_mut()
        .find(|item| item.get("id").and_then(Value::as_str) == Some(entity_ref))
}

fn value_at<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for segment in path {
        current = current.get(segment)?;
    }
    Some(current)
}

fn string_at(value: &Value, path: &[&str]) -> Option<String> {
    value_at(value, path)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn value_at_segments<'a>(value: &'a Value, segments: &[String]) -> Option<&'a Value> {
    let mut current = value;
    for segment in segments {
        current = match current {
            Value::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
            _ => current.get(segment)?,
        };
    }
    Some(current)
}

fn value_at_segments_mut<'a>(value: &'a mut Value, segments: &[String]) -> Option<&'a mut Value> {
    let mut current = value;
    for segment in segments {
        current = match current {
            Value::Array(items) => items.get_mut(segment.parse::<usize>().ok()?)?,
            other => other.get_mut(segment)?,
        };
    }
    Some(current)
}

fn parse_finite_number(raw: &str) -> Option<f64> {
    let parsed = raw.trim().parse::<f64>().ok()?;
    parsed.is_finite().then_some(parsed)
}

/// Format a number the way the inspector displays integers and short floats;
/// used for diagnostics/diff display only — staleness checks are numeric.
fn display_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// The engine's canonical JSON form — RFC 8785 (JCS) since completion-plan
/// H5: object keys sorted by UTF-16 code units, ECMAScript `Number::toString`
/// number rendering, `JSON.stringify` string escaping. Identical values now
/// produce identical canonical bytes in every lane (JS `JSON.stringify`
/// transport, raw-file serde parse, engine-internal serde value). Public
/// because it is the single canonicalization for every frontend hash seam
/// (completion-plan H1 / verification F-5a): the `wasm_api` exports wrap it
/// and the desktop `hashService` consumes those exports — no TypeScript
/// canonicalization exists. Implemented by the shared
/// `open_pipe_stress_canonical_json` crate, which the headless runner
/// checksums also use.
pub use open_pipe_stress_canonical_json::canonical_json;

/// Lowercase-hex SHA-256. Public for the same H1 / F-5a hash seam as
/// [`canonical_json`].
pub fn sha256_hex(payload: &str) -> String {
    let digest = Sha256::digest(payload.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sample_model() -> Value {
        json!({
            "schema_version": "0.1.0",
            "document_kind": "openpipestress.product_preview.model",
            "project": { "id": "project:test", "name": "Test", "units": { "length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC" } },
            "materials": [
                {
                    "id": "material:steel",
                    "label": "Invented steel",
                    "elastic_modulus": { "value": 200000000000.0, "unit": "Pa" },
                    "shear_modulus": { "value": 77000000000.0, "unit": "Pa" },
                    "thermal_expansion_coefficient": { "value": 0.000012, "unit": "1/degC" },
                    "provenance": "invented_example"
                }
            ],
            "nodes": [
                { "id": "node:N-1", "label": "Anchor", "position": { "x": 0.0, "y": 0.0, "z": 0.0 }, "provenance": "invented_example" },
                { "id": "node:N-2", "label": "Elbow", "position": { "x": 3.2, "y": 0.0, "z": 0.0 }, "provenance": "invented_example" }
            ],
            "pipe_segments": [
                {
                    "id": "pipe:P-1",
                    "label": "Run",
                    "from": "node:N-1",
                    "to": "node:N-2",
                    "section": {
                        "outside_diameter": { "value": 0.168, "unit": "m" },
                        "wall_thickness": { "value": 0.007, "unit": "m" }
                    },
                    "material": "material:steel",
                    "provenance": "invented_example"
                }
            ],
            "supports": [
                { "id": "support:S-1", "label": "Anchor support", "node": "node:N-1", "restraints": ["UX", "UY", "UZ"], "provenance": "invented_example" }
            ],
            "components": [
                { "id": "component:C-1", "label": "Bend", "kind": "bend", "node": "node:N-2", "provenance": "invented_example" }
            ],
            "load_cases": [
                {
                    "id": "load:L-1",
                    "label": "Weight",
                    "kind": "primitive_user_load",
                    "status": "preview_only",
                    "provenance": "invented_example",
                    "primitive_loads": [
                        {
                            "id": "load:L-1-Z",
                            "magnitude": { "value": -190.0, "unit": "N/m" },
                            "dimension": "force_per_length"
                        }
                    ]
                }
            ],
            "combinations": [
                { "id": "combination:C-OP", "label": "Operating", "basis": "mechanics", "terms": [], "provenance": "invented_example" }
            ],
            "diagnostics": []
        })
    }

    fn modify_intent(
        object_type: &str,
        target_ref: &str,
        change_kind: &str,
        field_path: &str,
        before: &str,
        after: &str,
        unit: &str,
        dimension: &str,
    ) -> Value {
        json!({
            "operation_id": format!("op:test-{}", field_path.replace('.', "-")),
            "operation_kind": "modify",
            "operation_status": "proposed",
            "author_type": "user",
            "target": { "object_type": object_type, "ref": target_ref },
            "change": {
                "change_id": format!("change:test-{}", field_path.replace('.', "-")),
                "change_kind": change_kind,
                "field_label": field_path,
                "field_path": field_path,
                "before": before,
                "after": after,
                "unit": unit,
                "dimension": dimension,
                "source_note": "test"
            },
            "validation": {
                "schema_validation": "not_run",
                "constraint_validation": "not_run",
                "unit_validation": "not_run",
                "diff_preview_status": "not_generated",
                "application_status": "not_applied"
            },
            "audit_boundary": {
                "mutation_route": "structured_operations_only",
                "direct_model_mutation_allowed": false,
                "requires_user_acceptance": true,
                "mutates_accepted_model_state": false
            },
            "professional_boundary": {
                "human_review_required": true,
                "software_makes_compliance_claim": false,
                "software_makes_certification_claim": false,
                "software_makes_sealing_claim": false,
                "software_makes_approval_claim": false,
                "software_makes_authentication_claim": false
            },
            "rationale": "test intent"
        })
    }

    fn codes(outcome: &OperationOutcome) -> Vec<&str> {
        outcome
            .diagnostics
            .iter()
            .map(|item| item.code.as_str())
            .collect()
    }

    #[test]
    fn validate_passes_and_never_mutates_for_a_current_quantity_edit() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "elastic_modulus.value",
            "200000000000",
            "195000000000",
            "Pa",
            "stress",
        );
        let outcome = validate_operation(&model, &intent, None);
        assert_eq!(
            model, before_snapshot,
            "validate must not mutate the input model"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(outcome.validation.application_status, "not_applied");
        assert_eq!(outcome.validation.diff_preview_status, "generated");
        assert_eq!(outcome.validation.before_state_validation, "passed");
        assert!(outcome.applied_model.is_none());
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].before, "200000000000");
        assert_eq!(outcome.diff_preview[0].after, "195000000000");
        assert!(outcome.input_model_unchanged);
        assert_eq!(outcome.acceptance.acceptance_basis, "none_validation_only");
    }

    #[test]
    fn apply_returns_a_new_model_with_only_the_requested_field_changed() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "elastic_modulus.value",
            "200000000000",
            "195000000000",
            "Pa",
            "stress",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(
            outcome.acceptance.acceptance_basis,
            "user_initiated_apply_in_local_session"
        );
        assert!(!outcome.acceptance.acceptance_is_professional_approval);
        let applied = outcome
            .applied_model
            .expect("applied model must be returned");
        assert_eq!(
            applied["materials"][0]["elastic_modulus"]["value"],
            json!(195000000000.0)
        );
        let mut expected = before_snapshot.clone();
        expected["materials"][0]["elastic_modulus"]["value"] = json!(195000000000.0);
        assert_eq!(applied, expected, "only the requested field may change");
        assert!(outcome
            .applied_model_backend_hash
            .unwrap()
            .starts_with("sha256:"));
    }

    #[test]
    fn apply_blocks_a_stale_before_value_without_application() {
        let model = sample_model();
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "elastic_modulus.value",
            "123",
            "195000000000",
            "Pa",
            "stress",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert!(codes(&outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert_eq!(outcome.validation.before_state_validation, "blocked_stale");
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn explicit_create_node_payload_applies_without_mutating_input() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "node:N-3",
            "label": "New node",
            "position": { "x": 4.5, "y": 1.25, "z": 0.75 },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Node",
            "node:N-3",
            "create_node",
            "nodes",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "m",
            "length",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "nodes");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(applied["nodes"].as_array().expect("nodes array").len(), 3);
        assert_eq!(applied["nodes"][2], payload);
        assert_eq!(
            outcome.professional_boundary["software_makes_approval_claim"],
            json!(false)
        );
    }

    #[test]
    fn explicit_create_material_payload_applies_without_mutating_input() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "material:user-alloy",
            "label": "User alloy",
            "elastic_modulus": { "value": 125000000000.0, "unit": "Pa" },
            "shear_modulus": { "value": 48000000000.0, "unit": "Pa" },
            "thermal_expansion_coefficient": { "value": 0.000010, "unit": "1/degC" },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Material",
            "material:user-alloy",
            "create_material",
            "materials",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "Pa",
            "stress",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "materials");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["materials"]
                .as_array()
                .expect("materials array")
                .len(),
            2
        );
        assert_eq!(applied["materials"][1], payload);
    }

    #[test]
    fn create_material_blocks_duplicate_id_and_invalid_quantity_payloads() {
        let model = sample_model();
        let duplicate_payload = json!({
            "id": "material:steel",
            "label": "Duplicate material",
            "elastic_modulus": { "value": 125000000000.0, "unit": "Pa" },
            "shear_modulus": { "value": 48000000000.0, "unit": "Pa" },
            "provenance": "user_entered_local_preview"
        });
        let mut duplicate = modify_intent(
            "Material",
            "material:steel",
            "create_material",
            "materials",
            "not_present",
            &serde_json::to_string(&duplicate_payload).expect("payload json"),
            "Pa",
            "stress",
        );
        duplicate["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &duplicate, None);
        assert!(codes(&outcome).contains(&"OP-TARGET-ALREADY-EXISTS"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());

        let invalid_payload = json!({
            "id": "material:user-alloy",
            "label": "Invalid material",
            "elastic_modulus": { "value": -1.0, "unit": "Pa" },
            "shear_modulus": { "value": 48000000000.0, "unit": "Pa" },
            "provenance": "user_entered_local_preview"
        });
        let mut invalid = modify_intent(
            "Material",
            "material:user-alloy",
            "create_material",
            "materials",
            "not_present",
            &serde_json::to_string(&invalid_payload).expect("payload json"),
            "Pa",
            "stress",
        );
        invalid["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &invalid, None);
        assert!(codes(&outcome).contains(&"OP-CREATE-MATERIAL-PAYLOAD-INVALID"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn explicit_create_section_payload_applies_without_mutating_input() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "section:user-pipe",
            "name": "User pipe section",
            "section_type": "pipe",
            "properties": {
                "outside_diameter": { "value": 0.114, "unit": "m" },
                "wall_thickness": { "value": 0.006, "unit": "m" }
            },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Section",
            "section:user-pipe",
            "create_section",
            "sections",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "m",
            "length",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "sections");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["sections"]
                .as_array()
                .expect("sections array")
                .len(),
            1
        );
        assert_eq!(applied["sections"][0], payload);
    }

    #[test]
    fn create_section_blocks_duplicate_id_and_invalid_geometry_payloads() {
        let mut model = sample_model();
        model["sections"] = json!([
            {
                "id": "section:existing",
                "name": "Existing pipe section",
                "section_type": "pipe",
                "properties": {
                    "outside_diameter": { "value": 0.114, "unit": "m" },
                    "wall_thickness": { "value": 0.006, "unit": "m" }
                },
                "provenance": "invented_example"
            }
        ]);
        let duplicate_payload = json!({
            "id": "section:existing",
            "name": "Duplicate section",
            "section_type": "pipe",
            "properties": {
                "outside_diameter": { "value": 0.114, "unit": "m" },
                "wall_thickness": { "value": 0.006, "unit": "m" }
            },
            "provenance": "user_entered_local_preview"
        });
        let mut duplicate = modify_intent(
            "Section",
            "section:existing",
            "create_section",
            "sections",
            "not_present",
            &serde_json::to_string(&duplicate_payload).expect("payload json"),
            "m",
            "length",
        );
        duplicate["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &duplicate, None);
        assert!(codes(&outcome).contains(&"OP-TARGET-ALREADY-EXISTS"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());

        let invalid_payload = json!({
            "id": "section:user-pipe",
            "name": "Invalid pipe section",
            "section_type": "pipe",
            "properties": {
                "outside_diameter": { "value": 0.114, "unit": "m" },
                "wall_thickness": { "value": 0.06, "unit": "m" }
            },
            "provenance": "user_entered_local_preview"
        });
        let mut invalid = modify_intent(
            "Section",
            "section:user-pipe",
            "create_section",
            "sections",
            "not_present",
            &serde_json::to_string(&invalid_payload).expect("payload json"),
            "m",
            "length",
        );
        invalid["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &invalid, None);
        assert!(codes(&outcome).contains(&"OP-CREATE-SECTION-PAYLOAD-INVALID"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn explicit_create_support_payload_applies_without_mutating_input() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "support:S-2",
            "label": "New guide support",
            "node": "node:N-2",
            "restraints": ["UY", "UZ", "RX"],
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Support",
            "support:S-2",
            "create_support",
            "supports",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "supports");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["supports"]
                .as_array()
                .expect("supports array")
                .len(),
            2
        );
        assert_eq!(applied["supports"][1], payload);
    }

    #[test]
    fn create_support_blocks_duplicate_id_and_missing_node_reference() {
        let model = sample_model();
        let duplicate_payload = json!({
            "id": "support:S-1",
            "label": "Duplicate support",
            "node": "node:N-2",
            "restraints": ["UX"],
            "provenance": "user_entered_local_preview"
        });
        let mut duplicate = modify_intent(
            "Support",
            "support:S-1",
            "create_support",
            "supports",
            "not_present",
            &serde_json::to_string(&duplicate_payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        duplicate["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &duplicate, None);
        assert!(codes(&outcome).contains(&"OP-TARGET-ALREADY-EXISTS"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());

        let missing_node_payload = json!({
            "id": "support:S-2",
            "label": "Dangling support",
            "node": "node:missing",
            "restraints": ["UX"],
            "provenance": "user_entered_local_preview"
        });
        let mut missing_node = modify_intent(
            "Support",
            "support:S-2",
            "create_support",
            "supports",
            "not_present",
            &serde_json::to_string(&missing_node_payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        missing_node["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &missing_node, None);
        assert!(codes(&outcome).contains(&"OP-SUPPORT-NODE-NOT-FOUND"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn explicit_create_load_case_payload_applies_empty_shell_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "load:L-2",
            "label": "User load case",
            "kind": "primitive_user_load",
            "status": "draft",
            "provenance": "user_entered_local_preview",
            "primitive_loads": []
        });
        let mut intent = modify_intent(
            "Load",
            "load:L-2",
            "create_load_case",
            "load_cases",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "load_cases");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["load_cases"]
                .as_array()
                .expect("load cases array")
                .len(),
            2
        );
        assert_eq!(applied["load_cases"][1], payload);
        assert_eq!(applied["load_cases"][0], before_snapshot["load_cases"][0]);

        let non_empty_payload = json!({
            "id": "load:L-3",
            "label": "Bad load case",
            "kind": "primitive_user_load",
            "status": "draft",
            "provenance": "user_entered_local_preview",
            "primitive_loads": [{ "id": "load:L-3-F" }]
        });
        let mut non_empty = modify_intent(
            "Load",
            "load:L-3",
            "create_load_case",
            "load_cases",
            "not_present",
            &serde_json::to_string(&non_empty_payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        non_empty["operation_kind"] = json!("create");
        let blocked = apply_operation(&model, &non_empty, None);
        assert!(codes(&blocked).contains(&"OP-CREATE-LOAD-CASE-PAYLOAD-INVALID"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_create_primitive_load_payload_applies_concentrated_force_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "load:L-1-F1",
            "category": "concentrated_force",
            "target": { "type": "node", "node": "node:N-2" },
            "direction": "global_y",
            "magnitude": { "value": 250.0, "unit": "N" },
            "dimension": "force",
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "N",
            "force",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "primitive_loads");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["load_cases"][0]["primitive_loads"]
                .as_array()
                .expect("primitive loads array")
                .len(),
            2
        );
        assert_eq!(applied["load_cases"][0]["primitive_loads"][1], payload);

        let duplicate = apply_operation(&applied, &intent, None);
        assert!(codes(&duplicate).contains(&"OP-TARGET-ALREADY-EXISTS"));
        assert!(duplicate.applied_model.is_none());

        let missing_node_payload = json!({
            "id": "load:L-1-F2",
            "category": "concentrated_force",
            "target": { "type": "node", "node": "node:missing" },
            "direction": "global_y",
            "magnitude": { "value": 250.0, "unit": "N" },
            "dimension": "force",
            "provenance": "user_entered_local_preview"
        });
        let mut missing_node = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&missing_node_payload).expect("payload json"),
            "N",
            "force",
        );
        missing_node["operation_kind"] = json!("create");
        let blocked = apply_operation(&model, &missing_node, None);
        assert!(codes(&blocked).contains(&"OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_create_primitive_load_payload_applies_distributed_force_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "load:L-1-D1",
            "category": "distributed_force",
            "target": { "type": "element", "pipe": "pipe:P-1" },
            "direction": "global_z",
            "magnitude": { "value": 125.0, "unit": "N/m" },
            "dimension": "force_per_length",
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "N/m",
            "force_per_length",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].unit, "N/m");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["load_cases"][0]["primitive_loads"]
                .as_array()
                .expect("primitive loads array")
                .len(),
            2
        );
        assert_eq!(applied["load_cases"][0]["primitive_loads"][1], payload);

        let duplicate = apply_operation(&applied, &intent, None);
        assert!(codes(&duplicate).contains(&"OP-TARGET-ALREADY-EXISTS"));
        assert!(duplicate.applied_model.is_none());

        let missing_pipe_payload = json!({
            "id": "load:L-1-D2",
            "category": "distributed_force",
            "target": { "type": "element", "pipe": "pipe:missing" },
            "direction": "global_z",
            "magnitude": { "value": 125.0, "unit": "N/m" },
            "dimension": "force_per_length",
            "provenance": "user_entered_local_preview"
        });
        let mut missing_pipe = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&missing_pipe_payload).expect("payload json"),
            "N/m",
            "force_per_length",
        );
        missing_pipe["operation_kind"] = json!("create");
        let blocked = apply_operation(&model, &missing_pipe, None);
        assert!(codes(&blocked).contains(&"OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_create_primitive_load_payload_applies_concentrated_moment_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "load:L-1-M1",
            "category": "concentrated_moment",
            "target": { "type": "node", "node": "node:N-2" },
            "direction": "rotation_z",
            "magnitude": { "value": 80.0, "unit": "N*m" },
            "dimension": "moment",
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "N*m",
            "moment",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].dimension, "moment");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["load_cases"][0]["primitive_loads"]
                .as_array()
                .expect("primitive loads array")
                .len(),
            2
        );
        assert_eq!(applied["load_cases"][0]["primitive_loads"][1], payload);

        let duplicate = apply_operation(&applied, &intent, None);
        assert!(codes(&duplicate).contains(&"OP-TARGET-ALREADY-EXISTS"));
        assert!(duplicate.applied_model.is_none());

        let missing_node_payload = json!({
            "id": "load:L-1-M2",
            "category": "concentrated_moment",
            "target": { "type": "node", "node": "node:missing" },
            "direction": "rotation_z",
            "magnitude": { "value": 80.0, "unit": "N*m" },
            "dimension": "moment",
            "provenance": "user_entered_local_preview"
        });
        let mut missing_node = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&missing_node_payload).expect("payload json"),
            "N*m",
            "moment",
        );
        missing_node["operation_kind"] = json!("create");
        let blocked = apply_operation(&model, &missing_node, None);
        assert!(codes(&blocked).contains(&"OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_create_primitive_load_payload_applies_pressure_and_thermal_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let pressure_payload = json!({
            "id": "load:L-1-P1",
            "category": "pressure",
            "target": { "type": "element", "pipe": "pipe:P-1" },
            "direction": "global_x",
            "magnitude": { "value": 1200000.0, "unit": "Pa" },
            "dimension": "pressure",
            "provenance": "user_entered_local_preview"
        });
        let mut pressure_intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&pressure_payload).expect("payload json"),
            "Pa",
            "pressure",
        );
        pressure_intent["operation_kind"] = json!("create");

        let pressure_outcome = apply_operation(&model, &pressure_intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            pressure_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            pressure_outcome.diagnostics
        );
        assert_eq!(
            pressure_outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(pressure_outcome.validation.reference_validation, "passed");
        assert_eq!(pressure_outcome.validation.unit_validation, "passed");
        let pressure_applied = pressure_outcome.applied_model.expect("applied model");
        assert_eq!(
            pressure_applied["load_cases"][0]["primitive_loads"][1],
            pressure_payload
        );

        let thermal_payload = json!({
            "id": "load:L-1-T1",
            "category": "thermal",
            "target": { "type": "element", "pipe": "pipe:P-1" },
            "direction": "global_z",
            "magnitude": { "value": 12.5, "unit": "degC" },
            "dimension": "temperature_interval",
            "provenance": "user_entered_local_preview"
        });
        let mut thermal_intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&thermal_payload).expect("payload json"),
            "degC",
            "temperature_interval",
        );
        thermal_intent["operation_kind"] = json!("create");
        let thermal_outcome = apply_operation(&pressure_applied, &thermal_intent, None);

        assert!(
            thermal_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            thermal_outcome.diagnostics
        );
        assert_eq!(
            thermal_outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(thermal_outcome.validation.reference_validation, "passed");
        assert_eq!(thermal_outcome.validation.unit_validation, "passed");
        let thermal_applied = thermal_outcome.applied_model.expect("applied model");
        assert_eq!(
            thermal_applied["load_cases"][0]["primitive_loads"][2],
            thermal_payload
        );

        let missing_pipe_payload = json!({
            "id": "load:L-1-P2",
            "category": "pressure",
            "target": { "type": "element", "pipe": "pipe:missing" },
            "direction": "global_x",
            "magnitude": { "value": 1200000.0, "unit": "Pa" },
            "dimension": "pressure",
            "provenance": "user_entered_local_preview"
        });
        let mut missing_pipe = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&missing_pipe_payload).expect("payload json"),
            "Pa",
            "pressure",
        );
        missing_pipe["operation_kind"] = json!("create");
        let blocked = apply_operation(&model, &missing_pipe, None);
        assert!(codes(&blocked).contains(&"OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_create_primitive_load_payload_applies_imposed_displacement_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let displacement_payload = json!({
            "id": "load:L-1-I1",
            "category": "imposed_displacement",
            "target": { "type": "support", "support": "support:S-1", "dof": "UZ" },
            "direction": "UZ",
            "magnitude": { "value": -0.006, "unit": "m" },
            "dimension": "displacement",
            "provenance": "user_entered_local_preview"
        });
        let mut displacement_intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&displacement_payload).expect("payload json"),
            "m",
            "displacement",
        );
        displacement_intent["operation_kind"] = json!("create");

        let displacement_outcome = apply_operation(&model, &displacement_intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            displacement_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            displacement_outcome.diagnostics
        );
        assert_eq!(
            displacement_outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(
            displacement_outcome.validation.reference_validation,
            "passed"
        );
        assert_eq!(displacement_outcome.validation.unit_validation, "passed");
        let displacement_applied = displacement_outcome.applied_model.expect("applied model");
        assert_eq!(
            displacement_applied["load_cases"][0]["primitive_loads"][1],
            displacement_payload
        );

        let rotation_payload = json!({
            "id": "load:L-1-I2",
            "category": "imposed_displacement",
            "target": { "type": "support", "support": "support:S-1", "dof": "RX" },
            "direction": "RX",
            "magnitude": { "value": 0.01, "unit": "rad" },
            "dimension": "rotation",
            "provenance": "user_entered_local_preview"
        });
        let mut rotation_intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&rotation_payload).expect("payload json"),
            "rad",
            "rotation",
        );
        rotation_intent["operation_kind"] = json!("create");
        let rotation_outcome = apply_operation(&displacement_applied, &rotation_intent, None);

        assert!(
            rotation_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            rotation_outcome.diagnostics
        );
        assert_eq!(
            rotation_outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(rotation_outcome.validation.reference_validation, "passed");
        assert_eq!(rotation_outcome.validation.unit_validation, "passed");
        let rotation_applied = rotation_outcome.applied_model.expect("applied model");
        assert_eq!(
            rotation_applied["load_cases"][0]["primitive_loads"][2],
            rotation_payload
        );

        let missing_support_payload = json!({
            "id": "load:L-1-I3",
            "category": "imposed_displacement",
            "target": { "type": "support", "support": "support:missing", "dof": "UZ" },
            "direction": "UZ",
            "magnitude": { "value": -0.006, "unit": "m" },
            "dimension": "displacement",
            "provenance": "user_entered_local_preview"
        });
        let mut missing_support = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&missing_support_payload).expect("payload json"),
            "m",
            "displacement",
        );
        missing_support["operation_kind"] = json!("create");
        let blocked = apply_operation(&model, &missing_support, None);
        assert!(codes(&blocked).contains(&"OP-PRIMITIVE-LOAD-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_connect_pipe_payload_applies_without_mutating_input() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "pipe:P-2",
            "label": "User pipe",
            "from": "node:N-1",
            "to": "node:N-2",
            "section": {
                "outside_diameter": { "value": 0.114, "unit": "m" },
                "wall_thickness": { "value": 0.006, "unit": "m" }
            },
            "material": "material:steel",
            "y_reference": { "x": 0.0, "y": 0.0, "z": 1.0 },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Element",
            "pipe:P-2",
            "connect_pipe_run",
            "pipe_segments",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "m",
            "length",
        );
        intent["operation_kind"] = json!("connect");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "pipe_segments");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["pipe_segments"]
                .as_array()
                .expect("pipe segments array")
                .len(),
            2
        );
        assert_eq!(applied["pipe_segments"][1], payload);
        assert_eq!(
            outcome.professional_boundary["software_makes_approval_claim"],
            json!(false)
        );
    }

    #[test]
    fn underspecified_connect_pipe_gesture_remains_blocked() {
        let model = sample_model();
        let mut intent = modify_intent(
            "Element",
            "pipe:viewport-preview:node:N-1-to-node:N-2",
            "connect_pipe_run",
            "viewport.connect_pipe_run",
            "not_present",
            "node:N-1 -> node:N-2",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("connect");

        let outcome = apply_operation(&model, &intent, None);

        assert!(codes(&outcome).contains(&"OP-CONNECT-PIPE-SHAPE-INVALID"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn apply_blocks_unit_mismatch_without_conversion() {
        let model = sample_model();
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "elastic_modulus.value",
            "200000000000",
            "29000",
            "ksi",
            "stress",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert!(codes(&outcome).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));
        assert_eq!(outcome.validation.unit_validation, "blocked");
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn apply_blocks_unknown_dimension_tokens() {
        let model = sample_model();
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "elastic_modulus.value",
            "200000000000",
            "195000000000",
            "Pa",
            "made_up_dimension",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert!(codes(&outcome).contains(&"OP-UNIT-DIMENSION-UNKNOWN"));
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn apply_blocks_non_numeric_and_non_positive_quantities() {
        let model = sample_model();
        let non_numeric = modify_intent(
            "Element",
            "pipe:P-1",
            "set_field",
            "section.wall_thickness.value",
            "0.007",
            "thick",
            "m",
            "length",
        );
        let outcome = apply_operation(&model, &non_numeric, None);
        assert!(codes(&outcome).contains(&"OP-VALUE-NOT-NUMERIC"));

        let non_positive = modify_intent(
            "Element",
            "pipe:P-1",
            "set_field",
            "section.wall_thickness.value",
            "0.007",
            "0",
            "m",
            "length",
        );
        let outcome = apply_operation(&model, &non_positive, None);
        assert!(codes(&outcome).contains(&"OP-VALUE-NOT-POSITIVE"));
    }

    #[test]
    fn negative_load_magnitudes_remain_editable() {
        let model = sample_model();
        let intent = modify_intent(
            "Load",
            "load:L-1",
            "update_load",
            "primitive_loads.0.magnitude.value",
            "-190",
            "-240",
            "N/m",
            "force_per_length",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"],
            json!(-240.0)
        );
    }

    #[test]
    fn load_case_status_and_kind_metadata_apply_without_mutating_input() {
        let model = sample_model();
        let before_snapshot = model.clone();

        let status_intent = modify_intent(
            "Load",
            "load:L-1",
            "update_load",
            "status",
            "preview_only",
            "TBD",
            "none",
            "dimensionless",
        );
        let status_outcome = apply_operation(&model, &status_intent, None);
        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            status_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            status_outcome.diagnostics
        );
        assert_eq!(
            status_outcome.validation.application_status,
            "applied_to_session_model"
        );
        let status_applied = status_outcome.applied_model.expect("status applied model");
        assert_eq!(status_applied["load_cases"][0]["status"], json!("TBD"));
        assert_eq!(
            status_applied["load_cases"][0]["kind"],
            before_snapshot["load_cases"][0]["kind"]
        );

        let kind_intent = modify_intent(
            "Load",
            "load:L-1",
            "update_load",
            "kind",
            "primitive_user_load",
            "TBD",
            "none",
            "dimensionless",
        );
        let kind_outcome = apply_operation(&model, &kind_intent, None);
        assert!(
            kind_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            kind_outcome.diagnostics
        );
        let kind_applied = kind_outcome.applied_model.expect("kind applied model");
        assert_eq!(kind_applied["load_cases"][0]["kind"], json!("TBD"));
        assert_eq!(
            kind_applied["load_cases"][0]["status"],
            before_snapshot["load_cases"][0]["status"]
        );
    }

    #[test]
    fn combination_term_factor_applies_without_whole_term_editing() {
        let mut model = sample_model();
        model["combinations"][0]["terms"] = json!([
            { "load_case": "load:L-1", "factor": 1.0 },
            { "load_case": "load:L-1", "factor": 0.5 }
        ]);
        let before_snapshot = model.clone();
        let intent = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "terms.1.factor",
            "0.5",
            "0.75",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(applied["combinations"][0]["terms"][0]["factor"], json!(1.0));
        assert_eq!(
            applied["combinations"][0]["terms"][1]["factor"],
            json!(0.75)
        );
        assert_eq!(
            applied["combinations"][0]["terms"][1]["load_case"],
            json!("load:L-1")
        );

        let whole_terms = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "terms",
            "load:L-1 x 1; load:L-1 x 0.5",
            "load:L-1 x 1; load:L-1 x 0.75",
            "none",
            "dimensionless",
        );
        let blocked = apply_operation(&model, &whole_terms, None);
        assert!(codes(&blocked).contains(&"OP-FIELD-EDIT-DEFERRED"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn combination_basis_applies_without_term_or_provenance_mutation() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let intent = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "basis",
            "mechanics",
            "mechanics_user_review",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["combinations"][0]["basis"],
            json!("mechanics_user_review")
        );
        assert_eq!(
            applied["combinations"][0]["terms"],
            before_snapshot["combinations"][0]["terms"]
        );
        assert_eq!(
            applied["combinations"][0]["provenance"],
            before_snapshot["combinations"][0]["provenance"]
        );

        let empty = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "basis",
            "mechanics",
            " ",
            "none",
            "dimensionless",
        );
        let blocked = apply_operation(&model, &empty, None);
        assert!(codes(&blocked).contains(&"OP-VALUE-EMPTY"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_create_combination_payload_creates_mechanics_record_with_initial_term() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "combination:C-NEW",
            "label": "User mechanics combination",
            "basis": "mechanics",
            "terms": [{ "load_case": "load:L-1", "factor": 1.25 }],
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Combination",
            "combination:C-NEW",
            "create_combination",
            "combinations",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.before_state_validation, "passed");
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.diff_preview[0].field_path, "combinations");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(applied["combinations"][1], payload);

        let mut missing_load = modify_intent(
            "Combination",
            "combination:C-BAD",
            "create_combination",
            "combinations",
            "not_present",
            &serde_json::to_string(&json!({
                "id": "combination:C-BAD",
                "label": "Bad combination",
                "basis": "mechanics",
                "terms": [{ "load_case": "load:missing", "factor": 1.0 }],
                "provenance": "user_entered_local_preview"
            }))
            .expect("payload json"),
            "none",
            "dimensionless",
        );
        missing_load["operation_kind"] = json!("create");
        let missing_outcome = apply_operation(&model, &missing_load, None);
        assert_eq!(missing_outcome.validation.application_status, "blocked");
        assert!(codes(&missing_outcome).contains(&"OP-COMBINATION-TERM-LOAD-NOT-FOUND"));

        let mut duplicate = modify_intent(
            "Combination",
            "combination:C-OP",
            "create_combination",
            "combinations",
            "not_present",
            &serde_json::to_string(&json!({
                "id": "combination:C-OP",
                "label": "Duplicate",
                "basis": "mechanics",
                "terms": [{ "load_case": "load:L-1", "factor": 1.0 }],
                "provenance": "user_entered_local_preview"
            }))
            .expect("payload json"),
            "none",
            "dimensionless",
        );
        duplicate["operation_kind"] = json!("create");
        let duplicate_outcome = apply_operation(&model, &duplicate, None);
        assert_eq!(duplicate_outcome.validation.application_status, "blocked");
        assert!(codes(&duplicate_outcome).contains(&"OP-TARGET-ALREADY-EXISTS"));
    }

    #[test]
    fn explicit_create_combination_term_payload_appends_one_term_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let payload = json!({ "load_case": "load:L-1", "factor": 0.25 });
        let mut intent = modify_intent(
            "Combination",
            "combination:C-OP",
            "create_combination_term",
            "terms",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.validation.unit_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(applied["combinations"][0]["terms"], json!([payload]));
        assert_eq!(
            applied["combinations"][0]["basis"],
            before_snapshot["combinations"][0]["basis"]
        );

        let mut missing_load = modify_intent(
            "Combination",
            "combination:C-OP",
            "create_combination_term",
            "terms",
            "not_present",
            r#"{"load_case":"load:missing","factor":1}"#,
            "none",
            "dimensionless",
        );
        missing_load["operation_kind"] = json!("create");
        let blocked = apply_operation(&model, &missing_load, None);
        assert!(codes(&blocked).contains(&"OP-COMBINATION-TERM-LOAD-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_delete_combination_term_removes_one_indexed_term_only() {
        let mut model = sample_model();
        model["combinations"][0]["terms"] = json!([
            { "load_case": "load:L-1", "factor": 0.25 },
            { "load_case": "load:L-2", "factor": 1.0 }
        ]);
        let before_snapshot = model.clone();
        let mut intent = modify_intent(
            "Combination",
            "combination:C-OP",
            "delete_combination_term",
            "terms.0",
            "load:L-1 x 0.25",
            "not_present",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("delete");

        let outcome = apply_operation(&model, &intent, None);

        assert_eq!(
            model, before_snapshot,
            "apply must not mutate the input model in place"
        );
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.before_state_validation, "passed");
        assert_eq!(outcome.validation.reference_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["combinations"][0]["terms"],
            json!([{ "load_case": "load:L-2", "factor": 1.0 }])
        );

        let mut stale = modify_intent(
            "Combination",
            "combination:C-OP",
            "delete_combination_term",
            "terms.0",
            "load:L-1 x 1",
            "not_present",
            "none",
            "dimensionless",
        );
        stale["operation_kind"] = json!("delete");
        let stale_outcome = apply_operation(&model, &stale, None);
        assert!(codes(&stale_outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert!(stale_outcome.applied_model.is_none());

        let mut missing_term = modify_intent(
            "Combination",
            "combination:C-OP",
            "delete_combination_term",
            "terms.9",
            "load:L-9 x 1",
            "not_present",
            "none",
            "dimensionless",
        );
        missing_term["operation_kind"] = json!("delete");
        let blocked = apply_operation(&model, &missing_term, None);
        assert!(codes(&blocked).contains(&"OP-COMBINATION-TERM-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn apply_blocks_missing_targets_and_dangling_references() {
        let model = sample_model();
        let missing_target = modify_intent(
            "Material",
            "material:missing",
            "set_field",
            "label",
            "x",
            "y",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &missing_target, None);
        assert!(codes(&outcome).contains(&"OP-TARGET-NOT-FOUND"));
        assert_eq!(outcome.validation.reference_validation, "blocked");

        let dangling = modify_intent(
            "Element",
            "pipe:P-1",
            "set_field",
            "material",
            "material:steel",
            "material:unknown",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &dangling, None);
        assert!(codes(&outcome).contains(&"OP-REFERENCE-NOT-FOUND"));
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn support_restraints_apply_with_vocabulary_enforcement() {
        let model = sample_model();
        let valid = modify_intent(
            "Support",
            "support:S-1",
            "update_support",
            "restraints",
            "UX, UY, UZ",
            "UX, UZ",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &valid, None);
        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(applied["supports"][0]["restraints"], json!(["UX", "UZ"]));

        let invalid = modify_intent(
            "Support",
            "support:S-1",
            "update_support",
            "restraints",
            "UX, UY, UZ",
            "UX, FLY",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &invalid, None);
        assert!(codes(&outcome).contains(&"OP-RESTRAINT-TOKEN-INVALID"));

        let empty = modify_intent(
            "Support",
            "support:S-1",
            "update_support",
            "restraints",
            "UX, UY, UZ",
            " ",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &empty, None);
        assert!(codes(&outcome).contains(&"OP-RESTRAINT-SET-EMPTY"));
    }

    #[test]
    fn underspecified_viewport_node_gestures_block_instead_of_invention() {
        let model = sample_model();
        let mut intent = modify_intent(
            "Node",
            "node:viewport-preview-created",
            "create_node",
            "viewport.create_node",
            "not_present",
            "node:viewport-preview-created",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &intent, None);
        assert!(codes(&outcome).contains(&"OP-CREATE-NODE-SHAPE-INVALID"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn deferred_inspector_fields_block_with_explicit_scope_finding() {
        let model = sample_model();
        let intent = modify_intent(
            "Component",
            "component:C-1",
            "set_field",
            "kind",
            "bend",
            "tee",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert!(codes(&outcome).contains(&"OP-FIELD-EDIT-DEFERRED"));
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn direct_mutation_requests_are_blocked() {
        let model = sample_model();
        let mut intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "label",
            "Invented steel",
            "Renamed",
            "none",
            "dimensionless",
        );
        intent["audit_boundary"]["direct_model_mutation_allowed"] = json!(true);
        let outcome = apply_operation(&model, &intent, None);
        assert!(codes(&outcome).contains(&"OP-DIRECT-MUTATION-BLOCKED"));
        assert!(outcome.applied_model.is_none());
    }

    #[test]
    fn unsupported_field_paths_and_object_types_are_blocked() {
        let model = sample_model();
        let unsupported_field = modify_intent(
            "Node",
            "node:N-1",
            "set_field",
            "secret.path",
            "a",
            "b",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &unsupported_field, None);
        assert!(codes(&outcome).contains(&"OP-FIELD-PATH-UNSUPPORTED"));

        let unsupported_type = modify_intent(
            "Diagnostic",
            "diagnostic:D-1",
            "set_field",
            "label",
            "a",
            "b",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &unsupported_type, None);
        assert!(codes(&outcome).contains(&"OP-OBJECT-TYPE-UNSUPPORTED"));
    }

    #[test]
    fn model_basis_evidence_echoes_claimed_hash_without_cross_canonicalization_equality_claims() {
        let model = sample_model();
        let claimed = json!({
            "algorithm": "sha256",
            "canonicalization": "rfc8785_jcs",
            "value": "sha256:abc123",
            "payload_scope": "model_payload"
        });
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "label",
            "Invented steel",
            "Renamed steel",
            "none",
            "dimensionless",
        );
        let outcome = validate_operation(&model, &intent, Some(&claimed));
        assert_eq!(outcome.model_basis.claimed_model_hash, "sha256:abc123");
        assert_eq!(
            outcome.model_basis.backend_canonicalization,
            BACKEND_CANONICALIZATION
        );
        assert!(outcome
            .model_basis
            .backend_model_hash
            .starts_with("sha256:"));
        assert_eq!(
            outcome.model_basis.binding_status,
            "claimed_hash_echoed_cross_canonicalization_equality_not_evaluated"
        );

        let without = validate_operation(&model, &intent, None);
        assert_eq!(without.model_basis.claimed_model_hash, "not_provided");
        assert_eq!(
            without.model_basis.binding_status,
            "no_claimed_hash_before_state_check_is_the_staleness_guard"
        );
    }

    #[test]
    fn outcomes_are_deterministic_for_identical_inputs() {
        let model = sample_model();
        let intent = modify_intent(
            "Node",
            "node:N-2",
            "set_field",
            "position.x",
            "3.2",
            "3.5",
            "m",
            "length",
        );
        let first_outcome = apply_operation(&model, &intent, None);
        assert_eq!(
            first_outcome.validation.application_status, "applied_to_session_model",
            "determinism check must compare applied outcomes, not identically blocked ones: {:?}",
            first_outcome.diagnostics
        );
        let first = serde_json::to_string(&first_outcome).unwrap();
        let second = serde_json::to_string(&apply_operation(&model, &intent, None)).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn node_position_edits_resolve_units_from_the_project_unit_system() {
        let model = sample_model();
        let applied = apply_operation(
            &model,
            &modify_intent(
                "Node",
                "node:N-2",
                "set_field",
                "position.y",
                "0",
                "1.1",
                "m",
                "length",
            ),
            None,
        );
        assert!(
            applied.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            applied.diagnostics
        );
        assert_eq!(
            applied.applied_model.expect("applied model")["nodes"][1]["position"]["y"],
            json!(1.1)
        );

        let mismatched = apply_operation(
            &model,
            &modify_intent(
                "Node",
                "node:N-2",
                "set_field",
                "position.y",
                "0",
                "3.6",
                "ft",
                "length",
            ),
            None,
        );
        assert!(codes(&mismatched).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));

        let mut unitless_model = sample_model();
        unitless_model["project"]["units"]
            .as_object_mut()
            .unwrap()
            .remove("length");
        let missing = apply_operation(
            &unitless_model,
            &modify_intent(
                "Node",
                "node:N-2",
                "set_field",
                "position.y",
                "0",
                "1.1",
                "m",
                "length",
            ),
            None,
        );
        assert!(codes(&missing).contains(&"OP-UNIT-METADATA-MISSING"));
    }

    #[test]
    fn missing_intent_fields_are_reported_per_field() {
        let model = sample_model();
        let intent = json!({ "operation_id": "op:incomplete" });
        let outcome = validate_operation(&model, &intent, None);
        assert!(
            codes(&outcome)
                .iter()
                .filter(|code| **code == "OP-INTENT-FIELD-MISSING")
                .count()
                >= 5
        );
        assert_eq!(outcome.validation.schema_validation, "blocked");
        assert_eq!(outcome.validation.application_status, "not_applied");
    }

    #[test]
    fn professional_boundary_never_claims_compliance_or_approval() {
        let model = sample_model();
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "label",
            "Invented steel",
            "Renamed",
            "none",
            "dimensionless",
        );
        for outcome in [
            validate_operation(&model, &intent, None),
            apply_operation(&model, &intent, None),
        ] {
            assert_eq!(
                outcome.professional_boundary["human_review_required"],
                json!(true)
            );
            for claim in [
                "software_makes_compliance_claim",
                "software_makes_certification_claim",
                "software_makes_sealing_claim",
                "software_makes_approval_claim",
                "software_makes_authentication_claim",
            ] {
                assert_eq!(
                    outcome.professional_boundary[claim],
                    json!(false),
                    "claim {claim} must stay false"
                );
            }
            assert!(!outcome.acceptance.acceptance_is_professional_approval);
        }
    }

    #[test]
    fn thermal_expansion_authoring_requires_existing_quantity() {
        let mut model = sample_model();
        model["materials"][0]
            .as_object_mut()
            .unwrap()
            .remove("thermal_expansion_coefficient");
        let intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "thermal_expansion_coefficient.value",
            "TBD",
            "0.000011",
            "TBD",
            "thermal_expansion_coefficient",
        );
        let outcome = apply_operation(&model, &intent, None);
        assert!(codes(&outcome).contains(&"OP-QUANTITY-OBJECT-MISSING"));
        assert!(outcome.applied_model.is_none());
    }
}
