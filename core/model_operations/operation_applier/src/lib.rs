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
//!   values, no ungoverned unit conversion, no geometry defaults);
//! - receipts record a user-initiated local-session acceptance basis only and
//!   never a professional, certification, sealing, approval, authentication,
//!   or code-compliance claim.

use open_pipe_stress_units::{convert_for_dimension, unit_by_symbol, Dimension};
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

#[derive(Debug, Clone, PartialEq)]
struct EnteredQuantity {
    value: f64,
    unit: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitSource {
    /// Quantity stored as `{ value, unit }`; the unit lives next to the value.
    SiblingUnitField,
    /// Bare numeric field whose unit basis is the project unit system entry
    /// with this key (e.g. node positions use `project.units.length`).
    ProjectUnits(&'static str),
}

/// Value-range constraint for optional user-entered quantity slots. Each
/// token mirrors the constraint the downstream consumer
/// (`core/product_physics` / `core/loads/primitive_loads`) already enforces
/// at solve time for the same slot; the applier introduces no threshold of
/// its own.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ValueConstraint {
    /// Any finite value (finiteness is enforced for every numeric edit).
    AnyFinite,
    /// Strictly positive, mirroring the crate convention that rejects
    /// non-physical zero/negative geometry, modifier, and stiffness values.
    Positive,
    /// Non-negative, mirroring the effective-wall consumer that rejects a
    /// negative mill-tolerance reduction while accepting an explicit zero.
    NonNegative,
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
    /// Optional user-entered free-text slot (schema `Option<String>`);
    /// authorable when absent. Absence is displayed and staleness-checked as
    /// the explicit `TBD` sentinel the inspector emits.
    OptionalText,
    /// Optional user-entered id slot validated against the schema `Id`
    /// pattern `^[A-Za-z][A-Za-z0-9_.:-]*$`; authorable when absent.
    OptionalId,
    /// Optional reference to an entity id inside another model collection;
    /// authorable when absent. Referenced entities are never created
    /// implicitly.
    OptionalEntityRef {
        collection: &'static str,
    },
    /// Optional closed-vocabulary token slot; authorable when absent.
    OptionalEnum {
        tokens: &'static [&'static str],
    },
    /// Optional comma-separated list of entity references stored as a string
    /// array (schema `minItems: 1`); authorable when absent.
    OptionalEntityRefList {
        collection: &'static str,
    },
    /// Optional dimensioned `{ value, unit }` quantity slot pinned to one
    /// schema dimension. Editable in place, and authorable when absent
    /// through explicit unit entry (the B2 `{value, unit}` payload or the
    /// intent's explicit unit field) — no default and no hidden fallback
    /// unit is ever supplied.
    OptionalQuantity {
        constraint: ValueConstraint,
        dimension: &'static str,
    },
}

struct FieldRule {
    field_path: &'static str,
    kind: FieldKind,
}

/// Closed combination-basis set (TP-APP-R2-COMBEXPR-001). The tokens mirror
/// `core/loads/load_case_algebra::AlgebraExpression` vocabulary:
/// `mechanics` = linear combination over explicit terms;
/// `result_state_subtraction` = `minuend_id` − `subtrahend_id`;
/// `range_envelope` = `mode`-selected value across `operand_ids`.
/// Code/rule and owner-basis combinations remain private/deferred.
const COMBINATION_BASIS_CLOSED_SET: [&str; 3] =
    ["mechanics", "result_state_subtraction", "range_envelope"];

/// Closed range-envelope mode tokens, mirroring
/// `core/loads/load_case_algebra::RangeMode`.
const COMBINATION_RANGE_MODE_TOKENS: [&str; 4] = ["min", "max", "min_abs", "max_abs"];

/// Closed global-axis vocabulary for the wind equivalent-static direction
/// slot, mirroring `core/loads/primitive_loads` global-axis handling and the
/// `core/product_physics` preview-model contract (DEC-068 item 2).
const WIND_DIRECTION_TOKENS: [&str; 3] = ["global_x", "global_y", "global_z"];

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
            // DEC-068 item 3: user-entered absolute mill-tolerance thickness
            // reduction consumed by the effective-wall calculation. Zero is a
            // meaningful explicit entry; a negative reduction is rejected the
            // same way the effective-wall consumer rejects it.
            FieldRule {
                field_path: "section.mill_tolerance.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::NonNegative,
                    dimension: "length",
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
            // Component-geometry field families (TP-APP-R5-FIELDRULES-001).
            // Every slot is user-entered `Option<...>` data in the preview
            // model contract (`core/product_physics` component inputs); the
            // value constraints mirror the solve-side validation for the same
            // slot, and `geometry.center_of_gravity` stays unsupported (its
            // vector payload format needs a design ruling).
            //
            // Bend / elbow family.
            FieldRule {
                field_path: "geometry.bend_pipe_ref",
                kind: FieldKind::OptionalEntityRef {
                    collection: "pipe_segments",
                },
            },
            FieldRule {
                field_path: "geometry.bend_radius.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "length",
                },
            },
            FieldRule {
                field_path: "geometry.bend_angle.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "angle",
                },
            },
            FieldRule {
                field_path: "geometry.bend_plane_orientation",
                kind: FieldKind::OptionalText,
            },
            // Branch family.
            FieldRule {
                field_path: "geometry.branch_header_pipe_ref",
                kind: FieldKind::OptionalEntityRef {
                    collection: "pipe_segments",
                },
            },
            FieldRule {
                field_path: "geometry.branch_branch_pipe_ref",
                kind: FieldKind::OptionalEntityRef {
                    collection: "pipe_segments",
                },
            },
            FieldRule {
                field_path: "geometry.branch_run_size.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "length",
                },
            },
            FieldRule {
                field_path: "geometry.branch_header_size.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "length",
                },
            },
            FieldRule {
                field_path: "geometry.branch_connection_angle.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "angle",
                },
            },
            FieldRule {
                field_path: "geometry.branch_connection_type",
                kind: FieldKind::OptionalText,
            },
            // Rigid / semi-rigid family.
            FieldRule {
                field_path: "geometry.rigid_pipe_ref",
                kind: FieldKind::OptionalEntityRef {
                    collection: "pipe_segments",
                },
            },
            FieldRule {
                field_path: "geometry.rigid_body_length.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "length",
                },
            },
            FieldRule {
                field_path: "geometry.end_a_size.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "length",
                },
            },
            FieldRule {
                field_path: "geometry.end_b_size.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "length",
                },
            },
            FieldRule {
                field_path: "geometry.weight.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::AnyFinite,
                    dimension: "force",
                },
            },
            FieldRule {
                field_path: "geometry.stiffness_behavior_reference",
                kind: FieldKind::OptionalText,
            },
            // Expansion-joint family (DEC-045 / TP-R4-D4-EJSTIFF-001 entry
            // seam: explicit pipe mapping, user-entered effective pressure
            // area and movement limit, provenance references).
            FieldRule {
                field_path: "geometry.expansion_joint_pipe_ref",
                kind: FieldKind::OptionalEntityRef {
                    collection: "pipe_segments",
                },
            },
            FieldRule {
                field_path: "geometry.effective_area.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "area",
                },
            },
            FieldRule {
                field_path: "geometry.movement_limit.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "length",
                },
            },
            FieldRule {
                field_path: "geometry.hardware_reference",
                kind: FieldKind::OptionalText,
            },
            FieldRule {
                field_path: "geometry.manufacturer_reference",
                kind: FieldKind::OptionalText,
            },
            FieldRule {
                field_path: "geometry.pressure_thrust_reference",
                kind: FieldKind::OptionalText,
            },
            // User-entered modifier values (no code table, no catalog value,
            // no default; positivity mirrors the product-physics validation
            // for each slot).
            FieldRule {
                field_path: "modifiers.sif_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "modifiers.branch_header_sif_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "modifiers.branch_branch_sif_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "modifiers.flexibility_factor_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "modifiers.stiffness_scaling_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "modifiers.linear_stiffness_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "linear_stiffness",
                },
            },
            FieldRule {
                field_path: "modifiers.rotational_stiffness_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "rotational_stiffness",
                },
            },
            FieldRule {
                field_path: "modifiers.axial_stiffness_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "linear_stiffness",
                },
            },
            FieldRule {
                field_path: "modifiers.lateral_stiffness_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "linear_stiffness",
                },
            },
            FieldRule {
                field_path: "modifiers.angular_stiffness_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "rotational_stiffness",
                },
            },
            FieldRule {
                field_path: "modifiers.torsional_stiffness_user_value.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "rotational_stiffness",
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
            // DEC-068 item 1: user-assigned temperature-point basis id (or
            // the reserved label `material_base_values`). The applier
            // enforces the schema Id shape only; exact-selection matching
            // against stored temperature points is the solve-side contract
            // (`core/product_physics`), which blocks a dangling reference —
            // no interpolation anywhere (D-38 remains AWAITING_RULING).
            FieldRule {
                field_path: "modulus_basis_ref",
                kind: FieldKind::OptionalId,
            },
            // DEC-068 item 2: user-entered static-equivalent generation
            // inputs. Constraints mirror `core/loads/primitive_loads`
            // generation validation (gravity finite positive; g-factors,
            // pressure, and shape factor finite; direction a global axis;
            // exposed spans a non-empty pipe-reference list). No code
            // coefficient, catalog value, or default is supplied here.
            FieldRule {
                field_path: "equivalent_static.seismic.gravity_acceleration.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::Positive,
                    dimension: "acceleration",
                },
            },
            FieldRule {
                field_path: "equivalent_static.seismic.g_factor_x.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::AnyFinite,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "equivalent_static.seismic.g_factor_y.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::AnyFinite,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "equivalent_static.seismic.g_factor_z.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::AnyFinite,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "equivalent_static.wind.pressure.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::AnyFinite,
                    dimension: "pressure",
                },
            },
            FieldRule {
                field_path: "equivalent_static.wind.shape_factor.value",
                kind: FieldKind::OptionalQuantity {
                    constraint: ValueConstraint::AnyFinite,
                    dimension: "dimensionless",
                },
            },
            FieldRule {
                field_path: "equivalent_static.wind.direction",
                kind: FieldKind::OptionalEnum {
                    tokens: &WIND_DIRECTION_TOKENS,
                },
            },
            FieldRule {
                field_path: "equivalent_static.wind.exposed_pipe_refs",
                kind: FieldKind::OptionalEntityRefList {
                    collection: "pipe_segments",
                },
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
    let mut deleted_node: Option<String> = None;
    if !checker.schema_blocked && change_kind == "delete_node" {
        deleted_node = resolve_delete_node(
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
    let mut deleted_pipe: Option<String> = None;
    if !checker.schema_blocked && change_kind == "delete_pipe_run" {
        deleted_pipe = resolve_delete_pipe_run(
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
    let mut deleted_load_case: Option<String> = None;
    if !checker.schema_blocked && change_kind == "delete_load_case" {
        deleted_load_case = resolve_delete_load_case(
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
    let mut deleted_primitive_load: Option<(String, usize)> = None;
    if !checker.schema_blocked && change_kind == "delete_primitive_load" {
        deleted_primitive_load = resolve_delete_primitive_load(
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
    let mut deleted_combination: Option<String> = None;
    if !checker.schema_blocked && change_kind == "delete_combination" {
        deleted_combination = resolve_delete_combination(
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
    let mut deleted_support: Option<String> = None;
    if !checker.schema_blocked && change_kind == "delete_support" {
        deleted_support = resolve_delete_support(
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
        || deleted_node.is_some()
        || created_pipe.is_some()
        || deleted_pipe.is_some()
        || created_section.is_some()
        || created_material.is_some()
        || created_support.is_some()
        || created_load_case.is_some()
        || deleted_load_case.is_some()
        || created_primitive_load.is_some()
        || deleted_primitive_load.is_some()
        || created_combination.is_some()
        || deleted_combination.is_some()
        || created_combination_term.is_some()
        || deleted_combination_term.is_some()
        || deleted_support.is_some())
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
        } else if let Some(node_id) = &deleted_node {
            let mut next_model = model.clone();
            if apply_deleted_node(&mut next_model, node_id) {
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
        } else if let Some(pipe_id) = &deleted_pipe {
            let mut next_model = model.clone();
            if apply_deleted_pipe(&mut next_model, pipe_id) {
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
        } else if let Some(load_case_id) = &deleted_load_case {
            let mut next_model = model.clone();
            if apply_deleted_load_case(&mut next_model, load_case_id) {
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
        } else if let Some((load_case_id, primitive_index)) = &deleted_primitive_load {
            let mut next_model = model.clone();
            if apply_deleted_primitive_load(&mut next_model, load_case_id, *primitive_index) {
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
        } else if let Some(combination_id) = &deleted_combination {
            let mut next_model = model.clone();
            if apply_deleted_combination(&mut next_model, combination_id) {
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
        } else if let Some(support_id) = &deleted_support {
            let mut next_model = model.clone();
            if apply_deleted_support(&mut next_model, support_id) {
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

/// How a validated write resolves its target path on apply.
#[derive(Debug, Clone, Copy, PartialEq)]
enum WriteMode {
    /// The full path must already exist in the entity (required fields).
    RequireExisting,
    /// Missing intermediate objects along the path are created (optional
    /// schema slots being authored); an existing non-object intermediate is
    /// a blocking finding, never silently overwritten.
    CreatePath,
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
    /// Additional sibling writes that belong to the same validated edit.
    additional_writes: Vec<(Vec<String>, Value)>,
    /// Whether apply may create missing intermediate objects on the path.
    write_mode: WriteMode,
}

struct QuantityEdit {
    value: f64,
    unit: String,
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
            | "delete_node"
            | "connect_pipe_run"
            | "delete_pipe_run"
            | "create_section"
            | "create_material"
            | "create_support"
            | "create_load_case"
            | "delete_load_case"
            | "create_primitive_load"
            | "delete_primitive_load"
            | "create_combination"
            | "delete_combination"
            | "create_combination_term"
            | "delete_support"
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
        "delete_node" => "delete",
        "delete_load_case" => "delete",
        "delete_primitive_load" => "delete",
        "delete_combination" => "delete",
        "delete_support" => "delete",
        "delete_combination_term" => "delete",
        "delete_pipe_run" => "delete",
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
    if !unit_symbol_matches_dimension(stored_unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Project length unit metadata `{stored_unit}` is not an accepted DEC-018 length unit."),
            "Repair the model document's project.units.length metadata before creating nodes.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if !unit_symbol_matches_dimension(unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` is not an accepted DEC-018 length unit; project length metadata is `{stored_unit}`."),
            "Select an accepted length unit from the DEC-018 catalog; no hidden fallback unit is supplied.",
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

    let Some(x) = length_value_in_unit(
        x.unwrap(),
        unit,
        stored_unit,
        target_ref,
        "Node X coordinate",
        checker,
    ) else {
        return None;
    };
    let Some(y) = length_value_in_unit(
        y.unwrap(),
        unit,
        stored_unit,
        target_ref,
        "Node Y coordinate",
        checker,
    ) else {
        return None;
    };
    let Some(z) = length_value_in_unit(
        z.unwrap(),
        unit,
        stored_unit,
        target_ref,
        "Node Z coordinate",
        checker,
    ) else {
        return None;
    };

    Some(serde_json::json!({
        "id": id,
        "label": label,
        "position": { "x": x, "y": y, "z": z },
        "provenance": provenance,
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_delete_node(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<String> {
    if object_type != "Node" || field_path != "nodes" {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-NODE-SHAPE-INVALID",
            "blocking",
            "Delete-node intents must target object_type `Node` with field_path `nodes`."
                .to_string(),
            "Refresh the node delete intent from the selected node row.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if after != "not_present" {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-DELETE-AFTER-VALUE-INVALID",
            "blocking",
            "Delete-node intents must use after-value `not_present`.".to_string(),
            "Re-queue the delete intent from the selected node row.",
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
            format!("Intent unit `{unit}` does not match stored unit `none` for node deletion."),
            "Delete node records with unit `none`; no unit conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Delete-node dimension `{dimension}` must be `dimensionless`."),
            "Emit node deletion with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Some(node) = find_entity(model, "nodes", target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Node `{target_ref}` was not found in the current model."),
            "Select an existing node before deleting it.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let current_display = node_delete_display(node);
    if current_display.is_empty() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-NODE-PAYLOAD-INVALID",
            "blocking",
            format!("Node `{target_ref}` does not expose a valid deletion summary."),
            "Repair the model document before deleting node records.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let references = node_references(model, target_ref);
    if !references.is_empty() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-NODE-DELETE-REFERENCED",
            "blocking",
            format!(
                "Node `{target_ref}` is still referenced by model entities: {}.",
                references.join(", ")
            ),
            "Delete or retarget dependent pipes, supports, components, and primitive loads before deleting the node.",
            std::iter::once(target_ref.to_string())
                .chain(references)
                .collect(),
        );
        return None;
    }
    checker.reference_state = "passed";
    check_before(&current_display, before, target_ref, field_path, checker);
    Some(target_ref.to_string())
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
    if !unit_symbol_matches_dimension(stored_unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Project length unit metadata `{stored_unit}` is not an accepted DEC-018 length unit."),
            "Repair the model document's project.units.length metadata before creating pipe segments.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if !unit_symbol_matches_dimension(unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` is not an accepted DEC-018 length unit; project length metadata is `{stored_unit}`."),
            "Select an accepted length unit from the DEC-018 catalog; no hidden fallback unit is supplied.",
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
    let outside_diameter = dimensioned_quantity(
        record,
        &["section", "outside_diameter"],
        Dimension::Length,
        true,
    );
    let wall_thickness = dimensioned_quantity(
        record,
        &["section", "wall_thickness"],
        Dimension::Length,
        true,
    );
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
            "Connect-pipe payload must include matching id, non-empty label/from/to/material/provenance, distinct endpoint nodes, positive OD/wall quantities in accepted DEC-018 length units, and a non-zero y_reference vector.".to_string(),
            "Refresh the viewport connect-pipe intent from explicit user-entered pipe fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let outside_diameter = outside_diameter.unwrap();
    let wall_thickness = wall_thickness.unwrap();
    let Some(outside_diameter_for_check) =
        quantity_value_in_unit(&outside_diameter, stored_unit, Dimension::Length)
    else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            "Pipe outside-diameter unit could not be converted through the accepted DEC-018 length catalog.".to_string(),
            "Select an accepted length unit from the DEC-018 catalog.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(wall_thickness_for_check) =
        quantity_value_in_unit(&wall_thickness, stored_unit, Dimension::Length)
    else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            "Pipe wall-thickness unit could not be converted through the accepted DEC-018 length catalog.".to_string(),
            "Select an accepted length unit from the DEC-018 catalog.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if wall_thickness_for_check >= outside_diameter_for_check / 2.0 {
        checker.push(
            "OP-CONNECT-PIPE-PAYLOAD-INVALID",
            "blocking",
            "Connect-pipe wall thickness must be less than the outside-diameter radius."
                .to_string(),
            "Enter physically possible pipe section dimensions before connecting a pipe segment.",
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
            "outside_diameter": { "value": outside_diameter.value, "unit": outside_diameter.unit },
            "wall_thickness": { "value": wall_thickness.value, "unit": wall_thickness.unit }
        },
        "material": material,
        "y_reference": { "x": yrx, "y": yry, "z": yrz },
        "provenance": provenance,
    }))
}

#[allow(clippy::too_many_arguments)]
fn resolve_delete_pipe_run(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<String> {
    if object_type != "Element" || field_path != "pipe_segments" {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-PIPE-SHAPE-INVALID",
            "blocking",
            "Delete-pipe intents must target object_type `Element` with field_path `pipe_segments`."
                .to_string(),
            "Refresh the pipe delete intent from the selected pipe segment row.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if after != "not_present" {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-DELETE-AFTER-VALUE-INVALID",
            "blocking",
            "Delete-pipe intents must use after-value `not_present`.".to_string(),
            "Re-queue the delete intent from the selected pipe segment row.",
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
            format!("Intent unit `{unit}` does not match stored unit `none` for pipe deletion."),
            "Delete pipe records with unit `none`; no unit conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Delete-pipe dimension `{dimension}` must be `dimensionless`."),
            "Emit pipe deletion with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Some(pipe) = find_entity(model, "pipe_segments", target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Pipe segment `{target_ref}` was not found in the current model."),
            "Select an existing pipe segment before deleting it.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let current_display = pipe_delete_display(pipe);
    if current_display.is_empty() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PIPE-PAYLOAD-INVALID",
            "blocking",
            format!("Pipe segment `{target_ref}` does not expose a valid deletion summary."),
            "Repair the model document before deleting pipe records.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let references = pipe_primitive_load_references(model, target_ref);
    if !references.is_empty() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PIPE-DELETE-REFERENCED",
            "blocking",
            format!(
                "Pipe segment `{target_ref}` is still referenced by primitive loads: {}.",
                references.join(", ")
            ),
            "Delete or retarget dependent primitive loads before deleting the pipe segment.",
            std::iter::once(target_ref.to_string())
                .chain(references)
                .collect(),
        );
        return None;
    }
    checker.reference_state = "passed";
    check_before(&current_display, before, target_ref, field_path, checker);
    Some(target_ref.to_string())
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
    if !unit_symbol_matches_dimension(stored_unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Project length unit metadata `{stored_unit}` is not an accepted DEC-018 length unit."),
            "Repair the model document's project.units.length metadata before creating sections.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if !unit_symbol_matches_dimension(unit, Dimension::Length) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` is not an accepted DEC-018 length unit; project length metadata is `{stored_unit}`."),
            "Select an accepted length unit from the DEC-018 catalog; no hidden fallback unit is supplied.",
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
    let outside_diameter = dimensioned_quantity(
        record,
        &["properties", "outside_diameter"],
        Dimension::Length,
        true,
    );
    let wall_thickness = dimensioned_quantity(
        record,
        &["properties", "wall_thickness"],
        Dimension::Length,
        true,
    );
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
            "Create-section payload must include matching id, non-empty name/provenance, section_type `pipe`, and positive OD/wall quantities in accepted DEC-018 length units.".to_string(),
            "Refresh the section creation intent from explicit user-entered section fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let outside_diameter = outside_diameter.unwrap();
    let wall_thickness = wall_thickness.unwrap();
    let Some(outside_diameter_for_check) =
        quantity_value_in_unit(&outside_diameter, stored_unit, Dimension::Length)
    else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            "Outside-diameter unit could not be converted through the accepted DEC-018 length catalog.".to_string(),
            "Select an accepted length unit from the DEC-018 catalog.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(wall_thickness_for_check) =
        quantity_value_in_unit(&wall_thickness, stored_unit, Dimension::Length)
    else {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            "Wall-thickness unit could not be converted through the accepted DEC-018 length catalog.".to_string(),
            "Select an accepted length unit from the DEC-018 catalog.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if wall_thickness_for_check >= outside_diameter_for_check / 2.0 {
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
            "outside_diameter": { "value": outside_diameter.value, "unit": outside_diameter.unit },
            "wall_thickness": { "value": wall_thickness.value, "unit": wall_thickness.unit }
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
    if !unit_symbol_matches_dimension(stored_stress_unit, Dimension::Stress) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Project pressure/stress unit metadata `{stored_stress_unit}` is not an accepted DEC-018 stress unit."),
            "Repair the model document's project.units.pressure metadata before creating materials.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if !unit_symbol_matches_dimension(unit, Dimension::Stress) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` is not an accepted DEC-018 stress unit; project stress metadata is `{stored_stress_unit}`."),
            "Select an accepted stress unit from the DEC-018 catalog; no hidden fallback unit is supplied.",
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
    let elastic_modulus =
        dimensioned_quantity(record, &["elastic_modulus"], Dimension::Stress, true);
    let shear_modulus = dimensioned_quantity(record, &["shear_modulus"], Dimension::Stress, true);
    if id != target_ref
        || label.is_empty()
        || provenance.is_empty()
        || elastic_modulus.is_none()
        || shear_modulus.is_none()
    {
        checker.push(
            "OP-CREATE-MATERIAL-PAYLOAD-INVALID",
            "blocking",
            "Create-material payload must include matching id, non-empty label/provenance, and positive elastic/shear modulus quantities in accepted DEC-018 stress units.".to_string(),
            "Refresh the material creation intent from explicit user-entered material fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let elastic_modulus = elastic_modulus.unwrap();
    let shear_modulus = shear_modulus.unwrap();

    let mut material = serde_json::json!({
        "id": id,
        "label": label,
        "elastic_modulus": { "value": elastic_modulus.value, "unit": elastic_modulus.unit },
        "shear_modulus": { "value": shear_modulus.value, "unit": shear_modulus.unit },
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
        if !unit_symbol_matches_dimension(stored_temperature_unit, Dimension::Temperature) {
            checker.unit_state = "blocked";
            checker.push(
                "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
                "blocking",
                format!("Project temperature unit metadata `{stored_temperature_unit}` is not an accepted DEC-018 temperature unit."),
                "Repair the model document's project.units.temperature metadata before creating thermal expansion quantities.",
                vec![target_ref.to_string()],
            );
            return None;
        }
        let expected_thermal_unit = format!("1/{stored_temperature_unit}");
        let Some(thermal) = dimensioned_quantity_any_sign(
            record,
            &["thermal_expansion_coefficient"],
            Dimension::ThermalExpansionCoefficient,
        ) else {
            checker.push(
                "OP-CREATE-MATERIAL-PAYLOAD-INVALID",
                "blocking",
                format!("Thermal expansion must be a finite quantity in an accepted DEC-018 thermal-expansion unit; project reciprocal temperature metadata is `{expected_thermal_unit}`."),
                "Select an accepted thermal-expansion coefficient unit, or omit the coefficient.",
                vec![target_ref.to_string()],
            );
            return None;
        };
        material["thermal_expansion_coefficient"] = serde_json::json!({
            "value": thermal.value,
            "unit": thermal.unit,
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
    let support_create_is_dimensionless = unit == "none" && dimension == "dimensionless";
    let support_create_has_stiffness_unit = dimension == "linear_stiffness"
        && unit_symbol_matches_dimension(unit, Dimension::LinearStiffness);
    if !support_create_is_dimensionless && !support_create_has_stiffness_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` and dimension `{dimension}` do not match dimensionless support creation or accepted DEC-018 linear stiffness entry."),
            "Create bare support records with unit `none`, or include a positive linear_stiffness quantity with an accepted DEC-018 stiffness unit.",
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
    if record.get("properties").is_some() && !record.get("properties").is_some_and(Value::is_object)
    {
        checker.push(
            "OP-CREATE-SUPPORT-PAYLOAD-INVALID",
            "blocking",
            "Create-support properties payload must be an object when supplied.".to_string(),
            "Refresh the support creation intent from explicit user-entered support fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let stiffness_present = value_in_object(record, &["properties", "linear_stiffness"]).is_some();
    let linear_stiffness = if stiffness_present {
        match dimensioned_quantity(
            record,
            &["properties", "linear_stiffness"],
            Dimension::LinearStiffness,
            true,
        ) {
            Some(quantity) => Some(quantity),
            None => {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-CREATE-SUPPORT-PAYLOAD-INVALID",
                    "blocking",
                    "Create-support linear_stiffness must be a positive quantity with an accepted DEC-018 stiffness unit.".to_string(),
                    "Select an accepted linear stiffness unit and enter a positive stiffness value.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
        }
    } else {
        None
    };
    if linear_stiffness.is_some() && !support_create_has_stiffness_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-MISMATCH",
            "blocking",
            "Create-support payload includes linear_stiffness but the intent metadata is not dimension `linear_stiffness` with its entered unit.".to_string(),
            "Refresh the support creation intent from the support stiffness controls.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if linear_stiffness.is_none() && !support_create_is_dimensionless {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-MISMATCH",
            "blocking",
            "Create-support intent declares stiffness metadata but the payload has no linear_stiffness quantity.".to_string(),
            "Either enter support stiffness or create the support as dimensionless restraint data.",
            vec![target_ref.to_string()],
        );
        return None;
    }
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

    let mut support = serde_json::json!({
        "id": id,
        "label": label,
        "node": node,
        "restraints": restraints,
        "provenance": provenance,
    });
    if let Some(quantity) = linear_stiffness {
        support["properties"] = serde_json::json!({
            "linear_stiffness": { "value": quantity.value, "unit": quantity.unit }
        });
    }
    Some(support)
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
fn resolve_delete_load_case(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<String> {
    if object_type != "Load" || field_path != "load_cases" {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-LOAD-CASE-SHAPE-INVALID",
            "blocking",
            "Delete-load-case intents must target object_type `Load` with field_path `load_cases`."
                .to_string(),
            "Refresh the load-case delete intent from the selected load-case row.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if after != "not_present" {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-DELETE-AFTER-VALUE-INVALID",
            "blocking",
            "Delete-load-case intents must use after-value `not_present`.".to_string(),
            "Re-queue the delete intent from the selected load-case row.",
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
            format!("Intent unit `{unit}` does not match stored unit `none` for load-case deletion."),
            "Delete load-case records with unit `none`; primitive-load quantities are nested records.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Delete-load-case dimension `{dimension}` must be `dimensionless`."),
            "Emit load-case deletion with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Some(load_case) = find_entity(model, "load_cases", target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Load case `{target_ref}` was not found in the current model."),
            "Select an existing load case before deleting it.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(current_display) = load_case_delete_display(load_case) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-LOAD-CASE-PAYLOAD-INVALID",
            "blocking",
            format!("Load case `{target_ref}` is not a valid deletable load-case record."),
            "Repair the model document before deleting load cases.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let references = load_case_combination_references(model, target_ref);
    if !references.is_empty() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-LOAD-CASE-DELETE-REFERENCED",
            "blocking",
            format!(
                "Load case `{target_ref}` is still referenced by combinations: {}.",
                references.join(", ")
            ),
            "Delete or retarget dependent combination terms before deleting the load case.",
            std::iter::once(target_ref.to_string())
                .chain(references)
                .collect(),
        );
        return None;
    }

    checker.reference_state = "passed";
    check_before(&current_display, before, target_ref, field_path, checker);
    Some(target_ref.to_string())
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
    let expected_dimension_enum = match Dimension::from_schema_value(expected_dimension) {
        Ok(dimension) => dimension,
        Err(error) => {
            checker.unit_state = "blocked";
            checker.push(
                "OP-UNIT-DIMENSION-UNKNOWN",
                "blocking",
                format!("Create-primitive-load dimension `{expected_dimension}` is not accepted by the DEC-018 catalog: {error}."),
                "Refresh the primitive-load creation intent from the governed dimension vocabulary.",
                vec![target_ref.to_string()],
            );
            return None;
        }
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
    if unit != expected_unit && !unit_symbol_matches_dimension(unit, expected_dimension_enum) {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` is not an accepted DEC-018 unit for primitive-load dimension `{expected_dimension}`; project unit basis is `{expected_unit}`."),
            "Select an accepted unit from the DEC-018 catalog; no hidden fallback unit is supplied.",
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
        || magnitude_unit != unit
        || payload_dimension != expected_dimension
        || provenance.is_empty()
    {
        checker.push(
            "OP-CREATE-PRIMITIVE-LOAD-PAYLOAD-INVALID",
            "blocking",
            "Create-primitive-load payload must include non-empty id/provenance, a supported category with matching node, pipe, or support target, compatible direction, finite magnitude whose unit matches the intent unit, and matching dimension.".to_string(),
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
            "magnitude": { "value": magnitude_value.unwrap(), "unit": magnitude_unit },
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
    if id != target_ref || label.is_empty() || provenance.is_empty() {
        checker.push(
            "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
            "blocking",
            "Create-combination payload must include matching id and non-empty label/provenance."
                .to_string(),
            "Refresh the combination creation intent from explicit user-entered combination fields.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if !COMBINATION_BASIS_CLOSED_SET.contains(&basis) {
        checker.push(
            "OP-COMBINATION-BASIS-UNSUPPORTED",
            "blocking",
            format!(
                "Combination basis `{basis}` is outside the explicit closed set `mechanics`, `result_state_subtraction`, `range_envelope`; code/rule and owner-basis combinations remain private/deferred."
            ),
            "Choose one of the closed-set bases in the Load Cases manager form.",
            vec![target_ref.to_string(), basis.to_string()],
        );
        return None;
    }

    match basis {
        "result_state_subtraction" => resolve_create_subtraction_combination(
            model, record, target_ref, checker,
        )
        .map(|(minuend_id, subtrahend_id)| {
            serde_json::json!({
                "id": id,
                "label": label,
                "basis": basis,
                "minuend_id": minuend_id,
                "subtrahend_id": subtrahend_id,
                "terms": [],
                "provenance": provenance,
            })
        }),
        "range_envelope" => resolve_create_range_combination(model, record, target_ref, checker)
            .map(|(operand_ids, mode)| {
                serde_json::json!({
                    "id": id,
                    "label": label,
                    "basis": basis,
                    "operand_ids": operand_ids,
                    "mode": mode,
                    "terms": [],
                    "provenance": provenance,
                })
            }),
        _ => {
            resolve_create_mechanics_combination(model, record, target_ref, checker).map(|terms| {
                serde_json::json!({
                    "id": id,
                    "label": label,
                    "basis": basis,
                    "terms": terms,
                    "provenance": provenance,
                })
            })
        }
    }
}

/// Per-basis payload shape: blocks create payloads that carry fields
/// belonging to a different basis, so applied records always match exactly
/// one closed-set expression shape.
fn check_combination_payload_shape(
    record: &serde_json::Map<String, Value>,
    basis: &str,
    forbidden_fields: &[&str],
    target_ref: &str,
    checker: &mut Checker,
) -> Option<()> {
    let mut stray_fields = forbidden_fields
        .iter()
        .filter(|field| record.get(**field).is_some())
        .copied()
        .collect::<Vec<_>>();
    if record
        .get("terms")
        .is_some_and(|terms| !terms.as_array().is_some_and(Vec::is_empty))
        && basis != "mechanics"
    {
        stray_fields.push("terms");
    }
    if stray_fields.is_empty() {
        return Some(());
    }
    checker.push(
        "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
        "blocking",
        format!(
            "Create-combination payload for basis `{basis}` carries fields belonging to a different basis: {}.",
            stray_fields.join(", ")
        ),
        "Emit exactly the fields of the selected basis: mechanics = terms; result_state_subtraction = minuend_id + subtrahend_id; range_envelope = operand_ids + mode.",
        vec![target_ref.to_string()],
    );
    None
}

fn check_combination_operand_load_case(
    model: &Value,
    operand_ref: &str,
    target_ref: &str,
    checker: &mut Checker,
) -> Option<()> {
    if find_entity(model, "load_cases", operand_ref).is_none() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-OPERAND-LOAD-NOT-FOUND",
            "blocking",
            format!("Combination operand references load case `{operand_ref}`, which is absent from the current model."),
            "Select existing load cases before creating a combination.",
            vec![target_ref.to_string(), operand_ref.to_string()],
        );
        return None;
    }
    Some(())
}

fn resolve_create_mechanics_combination(
    model: &Value,
    record: &serde_json::Map<String, Value>,
    target_ref: &str,
    checker: &mut Checker,
) -> Option<Vec<Value>> {
    check_combination_payload_shape(
        record,
        "mechanics",
        &["minuend_id", "subtrahend_id", "operand_ids", "mode"],
        target_ref,
        checker,
    )?;
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
    if term_payloads.is_empty() {
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
    Some(terms)
}

fn resolve_create_subtraction_combination(
    model: &Value,
    record: &serde_json::Map<String, Value>,
    target_ref: &str,
    checker: &mut Checker,
) -> Option<(String, String)> {
    check_combination_payload_shape(
        record,
        "result_state_subtraction",
        &["operand_ids", "mode"],
        target_ref,
        checker,
    )?;
    let minuend_id = record
        .get("minuend_id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let subtrahend_id = record
        .get("subtrahend_id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    if minuend_id.is_empty() || subtrahend_id.is_empty() {
        checker.push(
            "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
            "blocking",
            "result_state_subtraction payloads must include explicit non-empty minuend_id and subtrahend_id load-case references.".to_string(),
            "Select a minuend and a subtrahend load case in the Load Cases manager form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if minuend_id == subtrahend_id {
        checker.push(
            "OP-COMBINATION-OPERAND-DUPLICATE",
            "blocking",
            format!("Combination `{target_ref}` subtracts load case `{minuend_id}` from itself; subtraction requires two distinct load cases."),
            "Select two distinct load cases; self-subtraction is blocked.",
            vec![target_ref.to_string(), minuend_id.to_string()],
        );
        return None;
    }
    check_combination_operand_load_case(model, minuend_id, target_ref, checker)?;
    check_combination_operand_load_case(model, subtrahend_id, target_ref, checker)?;
    checker.reference_state = "passed";
    Some((minuend_id.to_string(), subtrahend_id.to_string()))
}

fn resolve_create_range_combination(
    model: &Value,
    record: &serde_json::Map<String, Value>,
    target_ref: &str,
    checker: &mut Checker,
) -> Option<(Vec<String>, String)> {
    check_combination_payload_shape(
        record,
        "range_envelope",
        &["minuend_id", "subtrahend_id"],
        target_ref,
        checker,
    )?;
    let mode = record
        .get("mode")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    if !COMBINATION_RANGE_MODE_TOKENS.contains(&mode) {
        checker.push(
            "OP-COMBINATION-RANGE-MODE-UNKNOWN",
            "blocking",
            format!(
                "Range-envelope mode `{mode}` is outside the explicit closed set `min`, `max`, `min_abs`, `max_abs`."
            ),
            "Choose one of the closed-set range modes in the Load Cases manager form.",
            vec![target_ref.to_string(), mode.to_string()],
        );
        return None;
    }
    let operand_payloads = record
        .get("operand_ids")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut operand_ids = Vec::new();
    for operand in &operand_payloads {
        let operand_ref = operand.as_str().unwrap_or("").trim();
        if operand_ref.is_empty() {
            operand_ids.clear();
            break;
        }
        operand_ids.push(operand_ref.to_string());
    }
    if operand_ids.is_empty() {
        checker.push(
            "OP-CREATE-COMBINATION-PAYLOAD-INVALID",
            "blocking",
            "range_envelope payloads must include operand_ids as a non-empty array of explicit load-case references.".to_string(),
            "Select at least one operand load case in the Load Cases manager form.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let mut seen_operands = std::collections::HashSet::new();
    for operand_ref in &operand_ids {
        if !seen_operands.insert(operand_ref.clone()) {
            checker.push(
                "OP-COMBINATION-OPERAND-DUPLICATE",
                "blocking",
                format!("Combination `{target_ref}` repeats load case `{operand_ref}` in its range-envelope operands."),
                "List each operand load case once; duplicate operands are blocked.",
                vec![target_ref.to_string(), operand_ref.to_string()],
            );
            return None;
        }
        check_combination_operand_load_case(model, operand_ref, target_ref, checker)?;
    }
    checker.reference_state = "passed";
    Some((operand_ids, mode.to_string()))
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
    let combination_basis = combination
        .get("basis")
        .and_then(Value::as_str)
        .unwrap_or("");
    if combination_basis != "mechanics" {
        checker.push(
            "OP-COMBINATION-TERM-BASIS-UNSUPPORTED",
            "blocking",
            format!(
                "Combination `{target_ref}` has basis `{combination_basis}`; explicit terms belong to mechanics-basis combinations only."
            ),
            "Subtraction and range-envelope combinations carry their own operand references; create a mechanics combination for factored terms.",
            vec![target_ref.to_string(), combination_basis.to_string()],
        );
        return None;
    }

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
fn resolve_delete_primitive_load(
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
    if object_type != "Load" {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-PRIMITIVE-LOAD-SHAPE-INVALID",
            "blocking",
            "Delete-primitive-load intents must target object_type `Load`.".to_string(),
            "Refresh the primitive-load delete intent from the explicit Load Cases manager row.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let Some(primitive_index) = primitive_load_index_from_field_path(field_path) else {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-PRIMITIVE-LOAD-SHAPE-INVALID",
            "blocking",
            "Delete-primitive-load intents must target an indexed field_path like `primitive_loads.2`.".to_string(),
            "Refresh the primitive-load delete intent from the selected primitive load row.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    if after != "not_present" {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-DELETE-AFTER-VALUE-INVALID",
            "blocking",
            "Delete-primitive-load intents must use after-value `not_present`.".to_string(),
            "Re-queue the delete intent from the selected primitive load row.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Some(load_case) = find_entity(model, "load_cases", target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Load case `{target_ref}` was not found in the current model."),
            "Select an existing load case before deleting a primitive load.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(primitive_loads) = load_case.get("primitive_loads").and_then(Value::as_array) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PRIMITIVE-LOADS-MISSING",
            "blocking",
            format!("Load case `{target_ref}` does not expose a primitive_loads array."),
            "Repair the model document before deleting primitive loads.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(primitive_load) = primitive_loads.get(primitive_index) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PRIMITIVE-LOAD-NOT-FOUND",
            "blocking",
            format!("Load case `{target_ref}` does not have a primitive load at index {primitive_index}."),
            "Re-queue the delete intent from the current primitive-load list.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some((current_display, current_unit, current_dimension)) =
        primitive_load_delete_display(primitive_load)
    else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-PRIMITIVE-LOAD-PAYLOAD-INVALID",
            "blocking",
            format!("Load case `{target_ref}` primitive load {primitive_index} is not a valid primitive-load record."),
            "Repair the model document before deleting primitive loads.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    checker.unit_state = "passed";
    if unit != current_unit {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!("Intent unit `{unit}` does not match stored unit `{current_unit}` for primitive-load deletion."),
            "Re-queue primitive-load deletion from the current row; no unit conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != current_dimension {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Delete-primitive-load dimension `{dimension}` does not match stored dimension `{current_dimension}`."),
            "Emit primitive-load deletion with the selected load's current dimension metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    checker.reference_state = "passed";
    check_before(&current_display, before, target_ref, field_path, checker);
    Some((target_ref.to_string(), primitive_index))
}

#[allow(clippy::too_many_arguments)]
fn resolve_delete_combination(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<String> {
    if object_type != "Combination" || field_path != "combinations" {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-COMBINATION-SHAPE-INVALID",
            "blocking",
            "Delete-combination intents must target object_type `Combination` with field_path `combinations`."
                .to_string(),
            "Refresh the combination delete intent from the selected combination row.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if after != "not_present" {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-DELETE-AFTER-VALUE-INVALID",
            "blocking",
            "Delete-combination intents must use after-value `not_present`.".to_string(),
            "Re-queue the delete intent from the selected combination row.",
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
            format!(
                "Intent unit `{unit}` does not match stored unit `none` for combination deletion."
            ),
            "Delete combination records with unit `none`; term factors are dimensionless scalars.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Delete-combination dimension `{dimension}` must be `dimensionless`."),
            "Emit combination deletion with dimensionless metadata.",
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
            "Select an existing combination before deleting it.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(current_display) = combination_delete_display(combination) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-COMBINATION-PAYLOAD-INVALID",
            "blocking",
            format!("Combination `{target_ref}` is not a valid deletable combination record."),
            "Repair the model document before deleting combinations.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    checker.reference_state = "passed";
    check_before(&current_display, before, target_ref, field_path, checker);
    Some(target_ref.to_string())
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
fn resolve_delete_support(
    model: &Value,
    target_ref: &str,
    field_path: &str,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    object_type: &str,
    checker: &mut Checker,
) -> Option<String> {
    if object_type != "Support" || field_path != "supports" {
        checker.schema_blocked = true;
        checker.push(
            "OP-DELETE-SUPPORT-SHAPE-INVALID",
            "blocking",
            "Delete-support intents must target object_type `Support` with field_path `supports`."
                .to_string(),
            "Refresh the support delete intent from the selected support row.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if after != "not_present" {
        checker.before_state = "blocked_stale";
        checker.push(
            "OP-DELETE-AFTER-VALUE-INVALID",
            "blocking",
            "Delete-support intents must use after-value `not_present`.".to_string(),
            "Re-queue the delete intent from the selected support row.",
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
            format!("Intent unit `{unit}` does not match stored unit `none` for support deletion."),
            "Delete support records with unit `none`; no unit conversion is performed.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    if dimension != "dimensionless" {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-UNKNOWN",
            "blocking",
            format!("Delete-support dimension `{dimension}` must be `dimensionless`."),
            "Emit support deletion with dimensionless metadata.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let Some(support) = find_entity(model, "supports", target_ref) else {
        checker.reference_state = "blocked";
        checker.push(
            "OP-TARGET-NOT-FOUND",
            "blocking",
            format!("Support `{target_ref}` was not found in the current model."),
            "Select an existing support before deleting it.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let current_label = support
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    if current_label.is_empty() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-SUPPORT-PAYLOAD-INVALID",
            "blocking",
            format!("Support `{target_ref}` does not expose a valid label."),
            "Repair the model document before deleting support records.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let references = support_primitive_load_references(model, target_ref);
    if !references.is_empty() {
        checker.reference_state = "blocked";
        checker.push(
            "OP-SUPPORT-DELETE-REFERENCED",
            "blocking",
            format!(
                "Support `{target_ref}` is still referenced by primitive loads: {}.",
                references.join(", ")
            ),
            "Delete or retarget dependent imposed-displacement primitive loads before deleting the support.",
            std::iter::once(target_ref.to_string())
                .chain(references)
                .collect(),
        );
        return None;
    }
    checker.reference_state = "passed";
    check_before(current_label, before, target_ref, field_path, checker);
    Some(target_ref.to_string())
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
            if object_type == "Combination" && field_path == "basis" {
                if !COMBINATION_BASIS_CLOSED_SET.contains(&trimmed) {
                    checker.push(
                        "OP-COMBINATION-BASIS-UNSUPPORTED",
                        "blocking",
                        format!(
                            "Combination basis `{trimmed}` is outside the explicit closed set `mechanics`, `result_state_subtraction`, `range_envelope`; code/rule and owner-basis combinations remain private/deferred."
                        ),
                        "Choose one of the closed-set bases in the Load Cases manager.",
                        vec![target_ref.to_string(), trimmed.to_string()],
                    );
                    return None;
                }
                if let Some(message) = combination_basis_shape_mismatch(entity, trimmed) {
                    checker.push(
                        "OP-COMBINATION-BASIS-SHAPE-MISMATCH",
                        "blocking",
                        format!("Combination `{target_ref}` cannot change basis to `{trimmed}`: {message}"),
                        "Basis edits require the stored payload fields of the target basis; delete and recreate the combination to change its expression shape.",
                        vec![target_ref.to_string(), trimmed.to_string()],
                    );
                    return None;
                }
            }
            Some(ResolvedField {
                kind,
                current_display: current,
                applied_value: Value::String(trimmed.to_string()),
                segments,
                additional_writes: Vec::new(),
                write_mode: WriteMode::RequireExisting,
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
                additional_writes: Vec::new(),
                write_mode: WriteMode::RequireExisting,
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

            // Unit metadata: project-unit quantities keep their project unit
            // basis. Sibling-unit quantities may atomically update value and
            // sibling unit through the B2 unit-aware payload.
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
            let dimension_enum = match Dimension::from_schema_value(dimension) {
                Ok(dimension_enum) => dimension_enum,
                Err(error) => {
                    checker.unit_state = "blocked";
                    checker.push(
                        "OP-UNIT-DIMENSION-UNKNOWN",
                        "blocking",
                        format!(
                            "Dimension `{dimension}` is outside the DEC-018 catalog vocabulary: {error}."
                        ),
                        "Use a governed dimension token for quantity edits.",
                        vec![target_ref.to_string()],
                    );
                    return None;
                }
            };
            let quantity_edit = if matches!(unit_source, UnitSource::SiblingUnitField)
                || after.trim_start().starts_with('{')
            {
                parse_quantity_edit(after, target_ref, field_path, checker)?
            } else {
                None
            };
            let requested_unit = quantity_edit
                .as_ref()
                .map(|edit| edit.unit.as_str())
                .unwrap_or(unit);
            if requested_unit != unit {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-QUANTITY-PAYLOAD-INVALID",
                    "blocking",
                    format!(
                        "Quantity payload unit `{requested_unit}` must match intent unit `{unit}`."
                    ),
                    "Refresh the quantity edit intent from the selected value and unit fields.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            checker.unit_state = "passed";
            let stored_unit = match stored_unit {
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
                    let unit_matches_stored = unit == stored;
                    let unit_matches_dimension =
                        unit_symbol_matches_dimension(unit, dimension_enum);
                    if !unit_matches_stored && !unit_matches_dimension {
                        checker.unit_state = "blocked";
                        checker.push(
                            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
                            "blocking",
                            format!(
                                "Intent unit `{unit}` does not match stored unit `{stored}` for `{field_path}` and is not accepted for dimension `{dimension}`."
                            ),
                            "Select an accepted DEC-018 unit; no hidden fallback unit is supplied.",
                            vec![target_ref.to_string()],
                        );
                        return None;
                    }
                    if matches!(unit_source, UnitSource::ProjectUnits(_))
                        && !unit_symbol_matches_dimension(&stored, dimension_enum)
                    {
                        checker.unit_state = "blocked";
                        checker.push(
                            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
                            "blocking",
                            format!(
                                "Stored project unit `{stored}` is not accepted for dimension `{dimension}`."
                            ),
                            "Repair the model document's project unit metadata before editing this quantity.",
                            vec![target_ref.to_string()],
                        );
                        return None;
                    }
                    stored
                }
            };
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

            let entered_value = if let Some(edit) = quantity_edit.as_ref() {
                edit.value
            } else {
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
                parsed
            };
            let parsed = if matches!(unit_source, UnitSource::ProjectUnits(_))
                && requested_unit != stored_unit
            {
                let entered_quantity = EnteredQuantity {
                    value: entered_value,
                    unit: requested_unit.to_string(),
                };
                let Some(converted) =
                    quantity_value_in_unit(&entered_quantity, &stored_unit, dimension_enum)
                else {
                    checker.unit_state = "blocked";
                    checker.push(
                        "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
                        "blocking",
                        format!(
                            "Intent unit `{requested_unit}` could not be converted to stored project unit `{stored_unit}` for `{field_path}`."
                        ),
                        "Select an accepted DEC-018 unit for the project-unit field.",
                        vec![target_ref.to_string()],
                    );
                    return None;
                };
                converted
            } else {
                entered_value
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
            let additional_writes =
                if let (UnitSource::SiblingUnitField, Some(edit)) = (unit_source, quantity_edit) {
                    let mut unit_segments = segments.clone();
                    unit_segments.pop();
                    unit_segments.push("unit".to_string());
                    vec![(unit_segments, Value::String(edit.unit))]
                } else {
                    Vec::new()
                };
            Some(ResolvedField {
                kind,
                current_display: display_number(current_number),
                applied_value: Value::Number(number),
                segments,
                additional_writes,
                write_mode: WriteMode::RequireExisting,
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
                additional_writes: Vec::new(),
                write_mode: WriteMode::RequireExisting,
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
                additional_writes: Vec::new(),
                write_mode: WriteMode::RequireExisting,
            })
        }
        FieldKind::OptionalText => {
            let replacement =
                resolve_optional_text_common(entity, &segments, before, after, target_ref, field_path, checker)?;
            Some(ResolvedField {
                kind,
                current_display: optional_text_current_display(entity, &segments),
                applied_value: Value::String(replacement),
                segments,
                additional_writes: Vec::new(),
                write_mode: WriteMode::CreatePath,
            })
        }
        FieldKind::OptionalId => {
            let replacement =
                resolve_optional_text_common(entity, &segments, before, after, target_ref, field_path, checker)?;
            if !is_valid_schema_id(&replacement) {
                checker.push(
                    "OP-ID-PATTERN-INVALID",
                    "blocking",
                    format!(
                        "Replacement value `{replacement}` for `{field_path}` does not match the schema Id pattern `^[A-Za-z][A-Za-z0-9_.:-]*$`."
                    ),
                    "Enter a user-assigned id starting with a letter and using only letters, digits, `_`, `.`, `:`, or `-`.",
                    vec![target_ref.to_string(), replacement.clone()],
                );
                return None;
            }
            Some(ResolvedField {
                kind,
                current_display: optional_text_current_display(entity, &segments),
                applied_value: Value::String(replacement),
                segments,
                additional_writes: Vec::new(),
                write_mode: WriteMode::CreatePath,
            })
        }
        FieldKind::OptionalEnum { tokens } => {
            let replacement =
                resolve_optional_text_common(entity, &segments, before, after, target_ref, field_path, checker)?;
            if !tokens.contains(&replacement.as_str()) {
                checker.push(
                    "OP-ENUM-TOKEN-INVALID",
                    "blocking",
                    format!(
                        "Replacement value `{replacement}` for `{field_path}` is outside the explicit closed vocabulary {}.",
                        tokens.join("/")
                    ),
                    "Choose one of the closed-vocabulary tokens; no other value is applied.",
                    vec![target_ref.to_string(), replacement.clone()],
                );
                return None;
            }
            Some(ResolvedField {
                kind,
                current_display: optional_text_current_display(entity, &segments),
                applied_value: Value::String(replacement),
                segments,
                additional_writes: Vec::new(),
                write_mode: WriteMode::CreatePath,
            })
        }
        FieldKind::OptionalEntityRef {
            collection: ref_collection,
        } => {
            let replacement =
                resolve_optional_text_common(entity, &segments, before, after, target_ref, field_path, checker)?;
            if find_entity(model, ref_collection, &replacement).is_none() {
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
                current_display: optional_text_current_display(entity, &segments),
                applied_value: Value::String(replacement),
                segments,
                additional_writes: Vec::new(),
                write_mode: WriteMode::CreatePath,
            })
        }
        FieldKind::OptionalEntityRefList {
            collection: ref_collection,
        } => {
            let current_tokens: Option<Vec<String>> = value_at_segments(entity, &segments)
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                });
            let current_display = current_tokens
                .map(|tokens| tokens.join(", "))
                .unwrap_or_else(|| "TBD".to_string());
            check_before(&current_display, before, target_ref, field_path, checker);
            let mut tokens: Vec<String> = Vec::new();
            for raw in after.split(',') {
                let token = raw.trim().to_string();
                if token.is_empty() {
                    continue;
                }
                if !tokens.contains(&token) {
                    tokens.push(token);
                }
            }
            if tokens.is_empty() {
                checker.push(
                    "OP-VALUE-EMPTY",
                    "blocking",
                    format!(
                        "Replacement reference list for `{field_path}` is empty; the schema requires at least one entry."
                    ),
                    "Provide at least one comma-separated entity reference.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            for token in &tokens {
                if find_entity(model, ref_collection, token).is_none() {
                    checker.reference_state = "blocked";
                    checker.push(
                        "OP-REFERENCE-NOT-FOUND",
                        "blocking",
                        format!("Replacement reference `{token}` does not exist in model collection `{ref_collection}`."),
                        "Reference existing entity ids; referenced entities are not created implicitly.",
                        vec![target_ref.to_string(), token.clone()],
                    );
                    return None;
                }
            }
            if checker.reference_state == "not_run" {
                checker.reference_state = "passed";
            }
            Some(ResolvedField {
                kind,
                current_display,
                applied_value: Value::Array(tokens.into_iter().map(Value::String).collect()),
                segments,
                additional_writes: Vec::new(),
                write_mode: WriteMode::CreatePath,
            })
        }
        FieldKind::OptionalQuantity {
            constraint,
            dimension: rule_dimension,
        } => resolve_optional_quantity(
            entity,
            segments,
            before,
            after,
            unit,
            dimension,
            rule_dimension,
            constraint,
            kind,
            target_ref,
            field_path,
            checker,
        ),
    }
}

/// Shared resolution for the optional text-shaped slots (`OptionalText`,
/// `OptionalId`, `OptionalEnum`, `OptionalEntityRef`): staleness-checks the
/// current value (absence is the explicit `TBD` sentinel the inspector
/// emits) and requires a non-empty replacement. Returns the trimmed
/// replacement string.
fn resolve_optional_text_common(
    entity: &Value,
    segments: &[String],
    before: &str,
    after: &str,
    target_ref: &str,
    field_path: &str,
    checker: &mut Checker,
) -> Option<String> {
    let current_display = optional_text_current_display(entity, segments);
    check_before(&current_display, before, target_ref, field_path, checker);
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
    Some(trimmed.to_string())
}

fn optional_text_current_display(entity: &Value, segments: &[String]) -> String {
    value_at_segments(entity, segments)
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| "TBD".to_string())
}

/// Schema `Id` pattern from `schemas/model.schema.yaml`
/// (`^[A-Za-z][A-Za-z0-9_.:-]*$`), implemented directly so the shape check
/// stays dependency-free.
fn is_valid_schema_id(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | ':' | '-'))
}

/// Resolution for optional `{ value, unit }` quantity slots pinned to one
/// schema dimension. Present slots are edited in place (sibling-unit
/// semantics, unit-preserving, no conversion); absent slots are authored
/// from the explicit user-entered unit — never from a default.
#[allow(clippy::too_many_arguments)]
fn resolve_optional_quantity(
    entity: &Value,
    segments: Vec<String>,
    before: &str,
    after: &str,
    unit: &str,
    dimension: &str,
    rule_dimension: &'static str,
    constraint: ValueConstraint,
    kind: FieldKind,
    target_ref: &str,
    field_path: &str,
    checker: &mut Checker,
) -> Option<ResolvedField> {
    if dimension != rule_dimension {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-DIMENSION-MISMATCH",
            "blocking",
            format!(
                "Field `{field_path}` carries dimension `{rule_dimension}`; intent dimension `{dimension}` is not accepted."
            ),
            "Emit the quantity edit with the field's schema dimension.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    let dimension_enum = match Dimension::from_schema_value(rule_dimension) {
        Ok(dimension_enum) => dimension_enum,
        Err(error) => {
            checker.unit_state = "blocked";
            checker.push(
                "OP-UNIT-DIMENSION-UNKNOWN",
                "blocking",
                format!(
                    "Dimension `{rule_dimension}` is outside the DEC-018 catalog vocabulary: {error}."
                ),
                "Use a governed dimension token for quantity edits.",
                vec![target_ref.to_string()],
            );
            return None;
        }
    };

    let quantity_edit = if after.trim_start().starts_with('{') {
        parse_quantity_edit(after, target_ref, field_path, checker)?
    } else {
        None
    };
    if let Some(edit) = quantity_edit.as_ref() {
        if edit.unit != unit {
            checker.unit_state = "blocked";
            checker.push(
                "OP-QUANTITY-PAYLOAD-INVALID",
                "blocking",
                format!(
                    "Quantity payload unit `{}` must match intent unit `{unit}`.",
                    edit.unit
                ),
                "Refresh the quantity edit intent from the selected value and unit fields.",
                vec![target_ref.to_string()],
            );
            return None;
        }
    }
    // Unit shape: the entered unit must be an accepted DEC-018 symbol for the
    // field's dimension. Dimensionless slots also accept the stored `none`
    // marker the preview model uses for user-entered scalar factors.
    let unit_ok = unit_symbol_matches_dimension(unit, dimension_enum)
        || (rule_dimension == "dimensionless" && unit == "none");
    if !unit_ok {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!(
                "Intent unit `{unit}` is not an accepted DEC-018 unit for dimension `{rule_dimension}` on `{field_path}`."
            ),
            "Select an accepted DEC-018 unit; no hidden fallback unit is supplied.",
            vec![target_ref.to_string()],
        );
        return None;
    }

    let entered_value = if let Some(edit) = quantity_edit.as_ref() {
        edit.value
    } else {
        let Some(parsed) = parse_finite_number(after) else {
            checker.push(
                "OP-VALUE-NOT-NUMERIC",
                "blocking",
                format!("Replacement value `{after}` for `{field_path}` is not a finite number."),
                "Provide a finite numeric value in the entered unit.",
                vec![target_ref.to_string()],
            );
            return None;
        };
        parsed
    };
    match constraint {
        ValueConstraint::AnyFinite => {}
        ValueConstraint::Positive => {
            if entered_value <= 0.0 {
                checker.push(
                    "OP-VALUE-NOT-POSITIVE",
                    "blocking",
                    format!(
                        "Replacement value `{after}` for `{field_path}` must be greater than zero to remain physically meaningful."
                    ),
                    "Provide a positive value for geometric, modifier, and stiffness quantities.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
        }
        ValueConstraint::NonNegative => {
            if entered_value < 0.0 {
                checker.push(
                    "OP-VALUE-NEGATIVE",
                    "blocking",
                    format!(
                        "Replacement value `{after}` for `{field_path}` must not be negative; an explicit zero is accepted."
                    ),
                    "Provide a non-negative value; the downstream consumer rejects a negative reduction.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
        }
    }
    let Some(number) = Number::from_f64(entered_value) else {
        checker.push(
            "OP-VALUE-NOT-NUMERIC",
            "blocking",
            format!(
                "Replacement value `{after}` for `{field_path}` cannot be encoded as a JSON number."
            ),
            "Provide a finite numeric value in the entered unit.",
            vec![target_ref.to_string()],
        );
        return None;
    };

    let current = value_at_segments(entity, &segments).and_then(Value::as_f64);
    match current {
        Some(current_number) => {
            let mut unit_segments = segments.clone();
            unit_segments.pop();
            unit_segments.push("unit".to_string());
            let stored_unit = value_at_segments(entity, &unit_segments)
                .and_then(Value::as_str)
                .map(str::to_string);
            let Some(stored_unit) = stored_unit else {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-UNIT-METADATA-MISSING",
                    "blocking",
                    format!(
                        "Stored quantity `{field_path}` on `{target_ref}` has no unit metadata."
                    ),
                    "Repair the model document's unit metadata before editing this quantity.",
                    vec![target_ref.to_string()],
                );
                return None;
            };
            if quantity_edit.is_none() && unit != stored_unit {
                checker.unit_state = "blocked";
                checker.push(
                    "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
                    "blocking",
                    format!(
                        "Intent unit `{unit}` does not match stored unit `{stored_unit}` for `{field_path}`; changing the stored unit requires the explicit value-and-unit payload and no silent conversion is performed."
                    ),
                    "Refresh the quantity edit intent from the selected value and unit fields.",
                    vec![target_ref.to_string()],
                );
                return None;
            }
            checker.unit_state = "passed";
            check_before_numeric(current_number, before, target_ref, field_path, checker);
            let additional_writes = if quantity_edit.is_some() {
                vec![(unit_segments, Value::String(unit.to_string()))]
            } else {
                Vec::new()
            };
            Some(ResolvedField {
                kind,
                current_display: display_number(current_number),
                applied_value: Value::Number(number),
                segments,
                additional_writes,
                write_mode: WriteMode::RequireExisting,
            })
        }
        None => {
            // Authoring an absent optional slot: the whole `{ value, unit }`
            // record is written from the explicit user entry; absence is
            // staleness-checked as the inspector's `TBD` sentinel.
            checker.unit_state = "passed";
            check_before("TBD", before, target_ref, field_path, checker);
            let mut parent_segments = segments;
            parent_segments.pop();
            Some(ResolvedField {
                kind,
                current_display: "TBD".to_string(),
                applied_value: serde_json::json!({
                    "value": Value::Number(number),
                    "unit": unit,
                }),
                segments: parent_segments,
                additional_writes: Vec::new(),
                write_mode: WriteMode::CreatePath,
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
    let main_written = match field.write_mode {
        WriteMode::RequireExisting => match value_at_segments_mut(entity, &field.segments) {
            Some(slot) => {
                *slot = field.applied_value.clone();
                true
            }
            None => false,
        },
        WriteMode::CreatePath => {
            write_value_creating_path(entity, &field.segments, &field.applied_value)
        }
    };
    if !main_written {
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
    }
    for (segments, value) in &field.additional_writes {
        let extra_written = match field.write_mode {
            WriteMode::RequireExisting => match value_at_segments_mut(entity, segments) {
                Some(slot) => {
                    *slot = value.clone();
                    true
                }
                None => false,
            },
            WriteMode::CreatePath => write_value_creating_path(entity, segments, value),
        };
        if !extra_written {
            checker.push(
                "OP-FIELD-NOT-PRESENT",
                "blocking",
                format!(
                    "Field path `{}` could not be resolved for write on `{target_ref}`.",
                    segments.join(".")
                ),
                "Refresh the editor intent against the current model document.",
                vec![target_ref.to_string()],
            );
            return false;
        }
    }
    let _ = field.kind;
    true
}

/// Write `value` at `segments` inside `entity`, creating missing
/// intermediate objects along the way (optional schema slots being
/// authored). Array segments must already exist and resolve by index —
/// arrays are never created implicitly — and an existing non-object
/// intermediate is a failure, never silently overwritten. Deterministic:
/// object-key insertion order does not affect the RFC 8785 canonical form.
fn write_value_creating_path(entity: &mut Value, segments: &[String], value: &Value) -> bool {
    let Some((last, parents)) = segments.split_last() else {
        return false;
    };
    let mut current = entity;
    for segment in parents {
        if current.is_array() {
            let Ok(index) = segment.parse::<usize>() else {
                return false;
            };
            let Some(next) = current.get_mut(index) else {
                return false;
            };
            current = next;
            continue;
        }
        let Some(map) = current.as_object_mut() else {
            return false;
        };
        let next = map
            .entry(segment.clone())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        current = next;
    }
    let Some(map) = current.as_object_mut() else {
        return false;
    };
    map.insert(last.clone(), value.clone());
    true
}

fn apply_created_node(model: &mut Value, node: &Value) -> bool {
    let Some(nodes) = model.get_mut("nodes").and_then(Value::as_array_mut) else {
        return false;
    };
    nodes.push(node.clone());
    true
}

fn apply_deleted_node(model: &mut Value, node_id: &str) -> bool {
    let Some(nodes) = model.get_mut("nodes").and_then(Value::as_array_mut) else {
        return false;
    };
    let Some(index) = nodes
        .iter()
        .position(|node| node.get("id").and_then(Value::as_str) == Some(node_id))
    else {
        return false;
    };
    nodes.remove(index);
    true
}

fn apply_created_pipe(model: &mut Value, pipe: &Value) -> bool {
    let Some(pipes) = model.get_mut("pipe_segments").and_then(Value::as_array_mut) else {
        return false;
    };
    pipes.push(pipe.clone());
    true
}

fn apply_deleted_pipe(model: &mut Value, pipe_id: &str) -> bool {
    let Some(pipes) = model.get_mut("pipe_segments").and_then(Value::as_array_mut) else {
        return false;
    };
    let Some(index) = pipes
        .iter()
        .position(|pipe| pipe.get("id").and_then(Value::as_str) == Some(pipe_id))
    else {
        return false;
    };
    pipes.remove(index);
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

fn apply_deleted_load_case(model: &mut Value, load_case_id: &str) -> bool {
    let Some(load_cases) = model.get_mut("load_cases").and_then(Value::as_array_mut) else {
        return false;
    };
    let Some(index) = load_cases
        .iter()
        .position(|load_case| load_case.get("id").and_then(Value::as_str) == Some(load_case_id))
    else {
        return false;
    };
    load_cases.remove(index);
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

fn apply_deleted_primitive_load(
    model: &mut Value,
    load_case_id: &str,
    primitive_index: usize,
) -> bool {
    let Some(load_case) = find_entity_mut(model, "load_cases", load_case_id) else {
        return false;
    };
    let Some(primitive_loads) = load_case
        .get_mut("primitive_loads")
        .and_then(Value::as_array_mut)
    else {
        return false;
    };
    if primitive_index >= primitive_loads.len() {
        return false;
    }
    primitive_loads.remove(primitive_index);
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

fn apply_deleted_combination(model: &mut Value, combination_id: &str) -> bool {
    let Some(combinations) = model.get_mut("combinations").and_then(Value::as_array_mut) else {
        return false;
    };
    let Some(index) = combinations.iter().position(|combination| {
        combination.get("id").and_then(Value::as_str) == Some(combination_id)
    }) else {
        return false;
    };
    combinations.remove(index);
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

fn apply_deleted_support(model: &mut Value, support_id: &str) -> bool {
    let Some(supports) = model.get_mut("supports").and_then(Value::as_array_mut) else {
        return false;
    };
    let Some(index) = supports
        .iter()
        .position(|support| support.get("id").and_then(Value::as_str) == Some(support_id))
    else {
        return false;
    };
    supports.remove(index);
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

fn parse_quantity_edit(
    after: &str,
    target_ref: &str,
    field_path: &str,
    checker: &mut Checker,
) -> Option<Option<QuantityEdit>> {
    let trimmed = after.trim();
    if !trimmed.starts_with('{') {
        return Some(None);
    }
    let Ok(value) = serde_json::from_str::<Value>(trimmed) else {
        checker.push(
            "OP-QUANTITY-PAYLOAD-INVALID",
            "blocking",
            format!("Quantity payload `{after}` is not valid JSON."),
            "Refresh the quantity edit intent from the selected value and unit fields.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(record) = value.as_object() else {
        checker.push(
            "OP-QUANTITY-PAYLOAD-INVALID",
            "blocking",
            "Quantity payload must be a JSON object with value and unit.".to_string(),
            "Refresh the quantity edit intent from the selected value and unit fields.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let Some(value) = record
        .get("value")
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite())
    else {
        checker.push(
            "OP-VALUE-NOT-NUMERIC",
            "blocking",
            format!("Quantity payload for `{field_path}` must carry a finite numeric value."),
            "Provide a finite quantity value.",
            vec![target_ref.to_string()],
        );
        return None;
    };
    let unit = record
        .get("unit")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    if unit.is_empty() {
        checker.unit_state = "blocked";
        checker.push(
            "OP-QUANTITY-PAYLOAD-INVALID",
            "blocking",
            "Quantity payload must carry a non-empty unit.".to_string(),
            "Select an explicit quantity unit.",
            vec![target_ref.to_string()],
        );
        return None;
    }
    Some(Some(QuantityEdit {
        value,
        unit: unit.to_string(),
    }))
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

fn primitive_load_index_from_field_path(field_path: &str) -> Option<usize> {
    let segments: Vec<&str> = field_path.split('.').collect();
    if segments.len() == 2
        && segments[0] == "primitive_loads"
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

fn load_case_delete_display(load_case: &Value) -> Option<String> {
    let id = load_case.get("id").and_then(Value::as_str)?.trim();
    let label = load_case
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .trim();
    let kind = load_case
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .trim();
    let status = load_case
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .trim();
    let primitive_count = load_case
        .get("primitive_loads")
        .and_then(Value::as_array)?
        .len();
    if id.is_empty() {
        return None;
    }
    Some(format!(
        "{id}; {label}; {kind}; {status}; primitives={primitive_count}"
    ))
}

/// Returns a blocking description when the stored combination record does not
/// already carry exactly the payload fields the requested basis evaluates;
/// `None` means the basis edit keeps the record honestly evaluable. Cross-
/// shape basis changes are re-authoring (delete + create), not field edits.
fn combination_basis_shape_mismatch(combination: &Value, target_basis: &str) -> Option<String> {
    let terms_is_array = combination.get("terms").is_some_and(Value::is_array);
    let has_terms = combination
        .get("terms")
        .and_then(Value::as_array)
        .is_some_and(|terms| !terms.is_empty());
    let has_field = |field: &str| {
        combination
            .get(field)
            .and_then(Value::as_str)
            .is_some_and(|value| !value.trim().is_empty())
    };
    let operand_ids = combination.get("operand_ids").and_then(Value::as_array);
    let has_operands = operand_ids.is_some_and(|ids| {
        !ids.is_empty()
            && ids
                .iter()
                .all(|id| id.as_str().is_some_and(|value| !value.trim().is_empty()))
    });
    let has_subtraction_fields = has_field("minuend_id") || has_field("subtrahend_id");
    let has_range_fields = operand_ids.is_some() || combination.get("mode").is_some();

    match target_basis {
        "mechanics" => {
            if !terms_is_array {
                return Some("the stored record has no terms array.".to_string());
            }
            if has_subtraction_fields || has_range_fields {
                return Some(
                    "the stored record carries subtraction or range-envelope fields that the mechanics basis does not evaluate."
                        .to_string(),
                );
            }
            None
        }
        "result_state_subtraction" => {
            if has_terms || has_range_fields {
                return Some(
                    "the stored record carries terms or range-envelope fields that the subtraction basis does not evaluate."
                        .to_string(),
                );
            }
            if !(has_field("minuend_id") && has_field("subtrahend_id")) {
                return Some(
                    "the stored record does not carry the minuend_id and subtrahend_id load-case references the subtraction basis evaluates."
                        .to_string(),
                );
            }
            None
        }
        "range_envelope" => {
            if has_terms || has_subtraction_fields {
                return Some(
                    "the stored record carries terms or subtraction fields that the range-envelope basis does not evaluate."
                        .to_string(),
                );
            }
            let mode_known = combination
                .get("mode")
                .and_then(Value::as_str)
                .is_some_and(|mode| COMBINATION_RANGE_MODE_TOKENS.contains(&mode.trim()));
            if !has_operands || !mode_known {
                return Some(
                    "the stored record does not carry the non-empty operand_ids list and closed-set mode the range-envelope basis evaluates."
                        .to_string(),
                );
            }
            None
        }
        _ => Some(format!("`{target_basis}` is not an evaluable basis.")),
    }
}

fn combination_delete_display(combination: &Value) -> Option<String> {
    let id = combination.get("id").and_then(Value::as_str)?.trim();
    let label = combination
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .trim();
    let basis = combination
        .get("basis")
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .trim();
    let terms = combination.get("terms").and_then(Value::as_array)?;
    let mut term_items = Vec::new();
    for term in terms {
        let load_case = term.get("load_case").and_then(Value::as_str)?.trim();
        let factor = term.get("factor").and_then(Value::as_f64)?;
        if load_case.is_empty() || !factor.is_finite() {
            return None;
        }
        term_items.push(combination_term_display(load_case, factor));
    }
    let term_display = if term_items.is_empty() {
        "none".to_string()
    } else {
        term_items.join("; ")
    };
    Some(format!(
        "{id}; {label}; basis={basis}; terms={term_display}"
    ))
}

fn primitive_load_delete_display(primitive_load: &Value) -> Option<(String, String, String)> {
    let id = primitive_load.get("id").and_then(Value::as_str)?.trim();
    let category = primitive_load
        .get("category")
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .trim();
    let target = primitive_load_target_display(primitive_load);
    let direction = primitive_load
        .get("direction")
        .and_then(Value::as_str)
        .unwrap_or("TBD")
        .trim();
    let dimension = primitive_load
        .get("dimension")
        .and_then(Value::as_str)?
        .trim();
    let magnitude = primitive_load.get("magnitude")?.as_object()?;
    let value = magnitude.get("value")?.as_f64()?;
    if !value.is_finite() {
        return None;
    }
    let unit = magnitude.get("unit").and_then(Value::as_str)?.trim();
    if id.is_empty() || dimension.is_empty() || unit.is_empty() {
        return None;
    }
    Some((
        format!(
            "{id}; {category}; {target}; {direction}; {} {unit}; {dimension}",
            display_number(value)
        ),
        unit.to_string(),
        dimension.to_string(),
    ))
}

fn primitive_load_target_display(primitive_load: &Value) -> String {
    let Some(target) = primitive_load.get("target").and_then(Value::as_object) else {
        return "target:TBD".to_string();
    };
    let target_type = target
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("target")
        .trim();
    for key in ["pipe", "node", "support"] {
        if let Some(value) = target.get(key).and_then(Value::as_str) {
            return format!("{target_type}:{}", value.trim());
        }
    }
    format!("{target_type}:TBD")
}

fn dimensioned_quantity(
    record: &serde_json::Map<String, Value>,
    path: &[&str],
    dimension: Dimension,
    require_positive: bool,
) -> Option<EnteredQuantity> {
    let quantity = value_in_object(record, path)?.as_object()?;
    let unit = quantity.get("unit").and_then(Value::as_str)?.trim();
    if !unit_symbol_matches_dimension(unit, dimension) {
        return None;
    }
    let value = quantity.get("value").and_then(Value::as_f64)?;
    if !value.is_finite() || (require_positive && value <= 0.0) {
        return None;
    }
    Some(EnteredQuantity {
        value,
        unit: unit.to_string(),
    })
}

fn dimensioned_quantity_any_sign(
    record: &serde_json::Map<String, Value>,
    path: &[&str],
    dimension: Dimension,
) -> Option<EnteredQuantity> {
    dimensioned_quantity(record, path, dimension, false)
}

fn quantity_value_in_unit(
    quantity: &EnteredQuantity,
    target_unit: &str,
    dimension: Dimension,
) -> Option<f64> {
    let from = unit_by_symbol(&quantity.unit, dimension).ok()?;
    let to = unit_by_symbol(target_unit, dimension).ok()?;
    convert_for_dimension(quantity.value, dimension, from, to).ok()
}

fn length_value_in_unit(
    value: f64,
    unit: &str,
    target_unit: &str,
    target_ref: &str,
    label: &str,
    checker: &mut Checker,
) -> Option<f64> {
    let quantity = EnteredQuantity {
        value,
        unit: unit.to_string(),
    };
    let converted = quantity_value_in_unit(&quantity, target_unit, Dimension::Length);
    if converted.is_none() {
        checker.unit_state = "blocked";
        checker.push(
            "OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE",
            "blocking",
            format!(
                "{label} unit could not be converted through the accepted DEC-018 length catalog."
            ),
            "Select an accepted length unit from the DEC-018 catalog.",
            vec![target_ref.to_string()],
        );
    }
    converted
}

fn unit_symbol_matches_dimension(symbol: &str, dimension: Dimension) -> bool {
    unit_by_symbol(symbol, dimension).is_ok()
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

fn node_references(model: &Value, node_ref: &str) -> Vec<String> {
    let mut references = Vec::new();

    if let Some(pipes) = model.get("pipe_segments").and_then(Value::as_array) {
        for pipe in pipes {
            let pipe_id = pipe
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or("pipe:unknown");
            let from = pipe.get("from").and_then(Value::as_str);
            let to = pipe.get("to").and_then(Value::as_str);
            if from == Some(node_ref) || to == Some(node_ref) {
                references.push(pipe_id.to_string());
            }
        }
    }

    if let Some(supports) = model.get("supports").and_then(Value::as_array) {
        for support in supports {
            let support_node = support.get("node").and_then(Value::as_str);
            if support_node == Some(node_ref) {
                references.push(
                    support
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("support:unknown")
                        .to_string(),
                );
            }
        }
    }

    if let Some(components) = model.get("components").and_then(Value::as_array) {
        for component in components {
            let component_node = component.get("node").and_then(Value::as_str);
            if component_node == Some(node_ref) {
                references.push(
                    component
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("component:unknown")
                        .to_string(),
                );
            }
        }
    }

    if let Some(load_cases) = model.get("load_cases").and_then(Value::as_array) {
        for load_case in load_cases {
            let Some(primitive_loads) = load_case.get("primitive_loads").and_then(Value::as_array)
            else {
                continue;
            };
            for primitive_load in primitive_loads {
                let target_node = primitive_load
                    .get("target")
                    .and_then(Value::as_object)
                    .and_then(|target| target.get("node"))
                    .and_then(Value::as_str);
                if target_node == Some(node_ref) {
                    references.push(
                        primitive_load
                            .get("id")
                            .and_then(Value::as_str)
                            .unwrap_or("primitive_load:unknown")
                            .to_string(),
                    );
                }
            }
        }
    }

    references.sort();
    references.dedup();
    references
}

fn node_delete_display(node: &Value) -> String {
    let label = node
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let Some(position) = node.get("position").and_then(Value::as_object) else {
        return String::new();
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
    if label.is_empty() || x.is_none() || y.is_none() || z.is_none() {
        return String::new();
    }
    format!(
        "{label}; x={}; y={}; z={}",
        display_number(x.unwrap()),
        display_number(y.unwrap()),
        display_number(z.unwrap())
    )
}

fn support_primitive_load_references(model: &Value, support_ref: &str) -> Vec<String> {
    let Some(load_cases) = model.get("load_cases").and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut references = Vec::new();
    for load_case in load_cases {
        let Some(primitive_loads) = load_case.get("primitive_loads").and_then(Value::as_array)
        else {
            continue;
        };
        for primitive_load in primitive_loads {
            let target_support = primitive_load
                .get("target")
                .and_then(Value::as_object)
                .and_then(|target| target.get("support"))
                .and_then(Value::as_str);
            if target_support == Some(support_ref) {
                references.push(
                    primitive_load
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("primitive_load:unknown")
                        .to_string(),
                );
            }
        }
    }
    references.sort();
    references.dedup();
    references
}

fn pipe_primitive_load_references(model: &Value, pipe_ref: &str) -> Vec<String> {
    let Some(load_cases) = model.get("load_cases").and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut references = Vec::new();
    for load_case in load_cases {
        let Some(primitive_loads) = load_case.get("primitive_loads").and_then(Value::as_array)
        else {
            continue;
        };
        for primitive_load in primitive_loads {
            let target_pipe = primitive_load
                .get("target")
                .and_then(Value::as_object)
                .and_then(|target| target.get("pipe"))
                .and_then(Value::as_str);
            if target_pipe == Some(pipe_ref) {
                references.push(
                    primitive_load
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("primitive_load:unknown")
                        .to_string(),
                );
            }
        }
    }
    references.sort();
    references.dedup();
    references
}

fn pipe_delete_display(pipe: &Value) -> String {
    let label = pipe
        .get("label")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let from = pipe
        .get("from")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    let to = pipe.get("to").and_then(Value::as_str).unwrap_or("").trim();
    let material = pipe
        .get("material")
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim();
    if label.is_empty() || from.is_empty() || to.is_empty() || material.is_empty() {
        return String::new();
    }
    format!("{label}; {from}->{to}; material={material}")
}

fn load_case_combination_references(model: &Value, load_case_ref: &str) -> Vec<String> {
    let Some(combinations) = model.get("combinations").and_then(Value::as_array) else {
        return Vec::new();
    };
    let mut references = Vec::new();
    for combination in combinations {
        let combination_id = combination
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("combination:unknown");
        if let Some(terms) = combination.get("terms").and_then(Value::as_array) {
            for (index, term) in terms.iter().enumerate() {
                let term_load_case = term.get("load_case").and_then(Value::as_str);
                if term_load_case == Some(load_case_ref) {
                    references.push(format!("{combination_id}.terms.{index}"));
                }
            }
        }
        for field in ["minuend_id", "subtrahend_id"] {
            if combination.get(field).and_then(Value::as_str) == Some(load_case_ref) {
                references.push(format!("{combination_id}.{field}"));
            }
        }
        if let Some(operand_ids) = combination.get("operand_ids").and_then(Value::as_array) {
            for (index, operand) in operand_ids.iter().enumerate() {
                if operand.as_str() == Some(load_case_ref) {
                    references.push(format!("{combination_id}.operand_ids.{index}"));
                }
            }
        }
    }
    references.sort();
    references.dedup();
    references
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
    fn explicit_create_node_payload_normalizes_compatible_entered_length_units() {
        let model = sample_model();
        let payload = json!({
            "id": "node:N-3",
            "label": "Metric draft node",
            "position": { "x": 4500.0, "y": 1250.0, "z": 750.0 },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Node",
            "node:N-3",
            "create_node",
            "nodes",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "mm",
            "length",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(outcome.validation.unit_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(applied["nodes"][2]["position"]["x"], json!(4.5));
        assert_eq!(applied["nodes"][2]["position"]["y"], json!(1.25));
        assert_eq!(applied["nodes"][2]["position"]["z"], json!(0.75));
    }

    #[test]
    fn explicit_delete_node_removes_unreferenced_node_only() {
        let mut model = sample_model();
        model["nodes"]
            .as_array_mut()
            .expect("nodes array")
            .push(json!({
                "id": "node:N-3",
                "label": "Free node",
                "position": { "x": 4.0, "y": 1.5, "z": 0.0 },
                "provenance": "invented_example"
            }));
        let before_snapshot = model.clone();
        let mut intent = modify_intent(
            "Node",
            "node:N-3",
            "delete_node",
            "nodes",
            "Free node; x=4; y=1.5; z=0",
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
        assert_eq!(outcome.validation.reference_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].field_path, "nodes");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(applied["nodes"].as_array().expect("nodes array").len(), 2);
        assert!(find_entity(&applied, "nodes", "node:N-3").is_none());

        let mut stale = modify_intent(
            "Node",
            "node:N-3",
            "delete_node",
            "nodes",
            "Renamed node; x=4; y=1.5; z=0",
            "not_present",
            "none",
            "dimensionless",
        );
        stale["operation_kind"] = json!("delete");
        let stale_outcome = apply_operation(&model, &stale, None);
        assert!(codes(&stale_outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert!(stale_outcome.applied_model.is_none());

        let mut missing = modify_intent(
            "Node",
            "node:missing",
            "delete_node",
            "nodes",
            "Missing node; x=0; y=0; z=0",
            "not_present",
            "none",
            "dimensionless",
        );
        missing["operation_kind"] = json!("delete");
        let blocked = apply_operation(&model, &missing, None);
        assert!(codes(&blocked).contains(&"OP-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn delete_node_blocks_model_references() {
        let mut model = sample_model();
        model["components"][0]["node"] = json!("node:N-1");
        model["load_cases"][0]["primitive_loads"][0] = json!({
            "id": "load:L-1-Y",
            "category": "concentrated_force",
            "target": { "type": "node", "node": "node:N-1" },
            "direction": "global_y",
            "magnitude": { "value": 12.0, "unit": "N" },
            "dimension": "force",
            "provenance": "invented_example"
        });
        let mut intent = modify_intent(
            "Node",
            "node:N-1",
            "delete_node",
            "nodes",
            "Anchor; x=0; y=0; z=0",
            "not_present",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("delete");

        let outcome = apply_operation(&model, &intent, None);

        assert!(codes(&outcome).contains(&"OP-NODE-DELETE-REFERENCED"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
        assert_eq!(
            outcome.diagnostics[0].affected_refs,
            vec![
                "node:N-1".to_string(),
                "component:C-1".to_string(),
                "load:L-1-Y".to_string(),
                "pipe:P-1".to_string(),
                "support:S-1".to_string(),
            ]
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
    fn explicit_create_material_payload_preserves_compatible_entered_units() {
        let model = sample_model();
        let payload = json!({
            "id": "material:user-mpa-alloy",
            "label": "User MPa alloy",
            "elastic_modulus": { "value": 125000.0, "unit": "MPa" },
            "shear_modulus": { "value": 48000.0, "unit": "MPa" },
            "thermal_expansion_coefficient": { "value": 0.000010, "unit": "1/K" },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Material",
            "material:user-mpa-alloy",
            "create_material",
            "materials",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "MPa",
            "stress",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.unit_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
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

        let incompatible_payload = json!({
            "id": "material:user-length-alloy",
            "label": "Invalid material unit",
            "elastic_modulus": { "value": 125000.0, "unit": "m" },
            "shear_modulus": { "value": 48000.0, "unit": "m" },
            "provenance": "user_entered_local_preview"
        });
        let mut incompatible = modify_intent(
            "Material",
            "material:user-length-alloy",
            "create_material",
            "materials",
            "not_present",
            &serde_json::to_string(&incompatible_payload).expect("payload json"),
            "m",
            "stress",
        );
        incompatible["operation_kind"] = json!("create");
        let outcome = apply_operation(&model, &incompatible, None);
        assert!(codes(&outcome).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));
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
    fn explicit_create_section_payload_preserves_compatible_entered_length_unit() {
        let model = sample_model();
        let payload = json!({
            "id": "section:user-mm-pipe",
            "name": "User millimeter pipe section",
            "section_type": "pipe",
            "properties": {
                "outside_diameter": { "value": 114.0, "unit": "mm" },
                "wall_thickness": { "value": 6.0, "unit": "mm" }
            },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Section",
            "section:user-mm-pipe",
            "create_section",
            "sections",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "mm",
            "length",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.unit_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
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
    fn explicit_create_support_preserves_entered_linear_stiffness_unit() {
        let model = sample_model();
        let payload = json!({
            "id": "support:S-2",
            "label": "New spring support",
            "node": "node:N-2",
            "restraints": ["UY"],
            "properties": {
                "linear_stiffness": { "value": 12500.0, "unit": "N/m" }
            },
            "provenance": "user_entered_local_preview"
        });
        let mut intent = modify_intent(
            "Support",
            "support:S-2",
            "create_support",
            "supports",
            "not_present",
            &serde_json::to_string(&payload).expect("payload json"),
            "N/m",
            "linear_stiffness",
        );
        intent["operation_kind"] = json!("create");

        let outcome = apply_operation(&model, &intent, None);

        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(
            outcome.validation.application_status,
            "applied_to_session_model"
        );
        assert_eq!(outcome.validation.unit_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
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
    fn explicit_create_primitive_load_preserves_compatible_entered_units() {
        let model = sample_model();
        let force_payload = json!({
            "id": "load:L-1-F2",
            "category": "concentrated_force",
            "target": { "type": "node", "node": "node:N-2" },
            "direction": "global_y",
            "magnitude": { "value": 55.0, "unit": "lbf" },
            "dimension": "force",
            "provenance": "user_entered_local_preview"
        });
        let mut force_intent = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&force_payload).expect("payload json"),
            "lbf",
            "force",
        );
        force_intent["operation_kind"] = json!("create");

        let force_outcome = apply_operation(&model, &force_intent, None);

        assert!(
            force_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            force_outcome.diagnostics
        );
        assert_eq!(force_outcome.validation.unit_validation, "passed");
        let force_applied = force_outcome.applied_model.expect("applied model");
        assert_eq!(
            force_applied["load_cases"][0]["primitive_loads"][1],
            force_payload
        );

        let pressure_payload = json!({
            "id": "load:L-1-P2",
            "category": "pressure",
            "target": { "type": "element", "pipe": "pipe:P-1" },
            "direction": "global_x",
            "magnitude": { "value": 850.0, "unit": "kPa" },
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
            "kPa",
            "pressure",
        );
        pressure_intent["operation_kind"] = json!("create");

        let pressure_outcome = apply_operation(&force_applied, &pressure_intent, None);

        assert!(
            pressure_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            pressure_outcome.diagnostics
        );
        assert_eq!(pressure_outcome.validation.unit_validation, "passed");
        let pressure_applied = pressure_outcome.applied_model.expect("applied model");
        assert_eq!(
            pressure_applied["load_cases"][0]["primitive_loads"][2],
            pressure_payload
        );

        let incompatible_payload = json!({
            "id": "load:L-1-F3",
            "category": "concentrated_force",
            "target": { "type": "node", "node": "node:N-2" },
            "direction": "global_y",
            "magnitude": { "value": 55.0, "unit": "mm" },
            "dimension": "force",
            "provenance": "user_entered_local_preview"
        });
        let mut incompatible = modify_intent(
            "Load",
            "load:L-1",
            "create_primitive_load",
            "primitive_loads",
            "not_present",
            &serde_json::to_string(&incompatible_payload).expect("payload json"),
            "mm",
            "force",
        );
        incompatible["operation_kind"] = json!("create");

        let blocked = apply_operation(&model, &incompatible, None);
        assert!(codes(&blocked).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));
        assert_eq!(blocked.validation.unit_validation, "blocked");
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
    fn explicit_connect_pipe_payload_preserves_compatible_entered_length_units() {
        let model = sample_model();
        let payload = json!({
            "id": "pipe:P-2",
            "label": "Millimeter pipe",
            "from": "node:N-1",
            "to": "node:N-2",
            "section": {
                "outside_diameter": { "value": 114.0, "unit": "mm" },
                "wall_thickness": { "value": 6.0, "unit": "mm" }
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
            "mm",
            "length",
        );
        intent["operation_kind"] = json!("connect");

        let outcome = apply_operation(&model, &intent, None);

        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(outcome.validation.unit_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["pipe_segments"][1]["section"]["outside_diameter"]["value"],
            json!(114.0)
        );
        assert_eq!(
            applied["pipe_segments"][1]["section"]["outside_diameter"]["unit"],
            json!("mm")
        );
        assert_eq!(
            applied["pipe_segments"][1]["section"]["wall_thickness"]["value"],
            json!(6.0)
        );
        assert_eq!(
            applied["pipe_segments"][1]["section"]["wall_thickness"]["unit"],
            json!("mm")
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
            "m",
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
    fn primitive_magnitude_edit_preserves_compatible_entered_unit() {
        let model = sample_model();
        let payload = json!({ "value": -28.0, "unit": "lbf/ft" });
        let intent = modify_intent(
            "Load",
            "load:L-1",
            "update_load",
            "primitive_loads.0.magnitude.value",
            "-190",
            &payload.to_string(),
            "lbf/ft",
            "force_per_length",
        );

        let outcome = apply_operation(&model, &intent, None);

        assert!(
            outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            outcome.diagnostics
        );
        assert_eq!(outcome.validation.unit_validation, "passed");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"],
            json!(-28.0)
        );
        assert_eq!(
            applied["load_cases"][0]["primitive_loads"][0]["magnitude"]["unit"],
            json!("lbf/ft")
        );

        let incompatible_payload = json!({ "value": -28.0, "unit": "mm" });
        let incompatible = modify_intent(
            "Load",
            "load:L-1",
            "update_load",
            "primitive_loads.0.magnitude.value",
            "-190",
            &incompatible_payload.to_string(),
            "mm",
            "force_per_length",
        );

        let blocked = apply_operation(&model, &incompatible, None);
        assert!(codes(&blocked).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));
        assert_eq!(blocked.validation.unit_validation, "blocked");
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn sibling_quantity_edit_preserves_compatible_entered_units() {
        let model = sample_model();
        let material_payload = json!({ "value": 210000.0, "unit": "MPa" });
        let material_intent = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "elastic_modulus.value",
            "200000000000",
            &material_payload.to_string(),
            "MPa",
            "stress",
        );

        let material_outcome = apply_operation(&model, &material_intent, None);

        assert!(
            material_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            material_outcome.diagnostics
        );
        assert_eq!(material_outcome.validation.unit_validation, "passed");
        let applied_material = material_outcome.applied_model.expect("applied model");
        assert_eq!(
            applied_material["materials"][0]["elastic_modulus"]["value"],
            json!(210000.0)
        );
        assert_eq!(
            applied_material["materials"][0]["elastic_modulus"]["unit"],
            json!("MPa")
        );

        let section_payload = json!({ "value": 168.0, "unit": "mm" });
        let section_intent = modify_intent(
            "Element",
            "pipe:P-1",
            "set_field",
            "section.outside_diameter.value",
            "0.168",
            &section_payload.to_string(),
            "mm",
            "length",
        );

        let section_outcome = apply_operation(&model, &section_intent, None);

        assert!(
            section_outcome.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            section_outcome.diagnostics
        );
        assert_eq!(section_outcome.validation.unit_validation, "passed");
        let applied_section = section_outcome.applied_model.expect("applied model");
        assert_eq!(
            applied_section["pipe_segments"][0]["section"]["outside_diameter"]["value"],
            json!(168.0)
        );
        assert_eq!(
            applied_section["pipe_segments"][0]["section"]["outside_diameter"]["unit"],
            json!("mm")
        );

        let incompatible_payload = json!({ "value": 210000.0, "unit": "mm" });
        let incompatible = modify_intent(
            "Material",
            "material:steel",
            "set_field",
            "elastic_modulus.value",
            "200000000000",
            &incompatible_payload.to_string(),
            "mm",
            "stress",
        );

        let blocked = apply_operation(&model, &incompatible, None);
        assert!(codes(&blocked).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));
        assert_eq!(blocked.validation.unit_validation, "blocked");
        assert!(blocked.applied_model.is_none());
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
    fn combination_basis_edits_validate_the_closed_set_and_payload_shape() {
        // Legacy repair path: a free-text basis recorded before the closed
        // set existed can be set back to `mechanics` because the stored
        // payload already carries the mechanics shape.
        let mut model = sample_model();
        model["combinations"][0]["basis"] = json!("mechanics_user_review");
        model["combinations"][0]["terms"] = json!([{ "load_case": "load:L-1", "factor": 1.0 }]);
        let before_snapshot = model.clone();
        let repair = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "basis",
            "mechanics_user_review",
            "mechanics",
            "none",
            "dimensionless",
        );
        let outcome = apply_operation(&model, &repair, None);
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
        assert_eq!(applied["combinations"][0]["basis"], json!("mechanics"));
        assert_eq!(
            applied["combinations"][0]["terms"],
            before_snapshot["combinations"][0]["terms"]
        );
        assert_eq!(
            applied["combinations"][0]["provenance"],
            before_snapshot["combinations"][0]["provenance"]
        );

        // Free-text basis tokens are no longer accepted (behavior change
        // recorded by TP-APP-R2-COMBEXPR-001): the closed set is
        // mechanics / result_state_subtraction / range_envelope.
        let free_text = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "basis",
            "mechanics_user_review",
            "owner_design_basis",
            "none",
            "dimensionless",
        );
        let blocked = apply_operation(&model, &free_text, None);
        assert!(codes(&blocked).contains(&"OP-COMBINATION-BASIS-UNSUPPORTED"));
        assert!(blocked.applied_model.is_none());

        // Cross-shape basis edits are re-authoring, not field edits: the
        // stored mechanics payload has no minuend/subtrahend references.
        let cross_shape = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "basis",
            "mechanics_user_review",
            "result_state_subtraction",
            "none",
            "dimensionless",
        );
        let blocked = apply_operation(&model, &cross_shape, None);
        assert!(codes(&blocked).contains(&"OP-COMBINATION-BASIS-SHAPE-MISMATCH"));
        assert!(blocked.applied_model.is_none());

        let empty = modify_intent(
            "Combination",
            "combination:C-OP",
            "update_load",
            "basis",
            "mechanics_user_review",
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

    fn sample_model_with_second_load_case() -> Value {
        let mut model = sample_model();
        model["load_cases"].as_array_mut().unwrap().push(json!({
            "id": "load:L-2",
            "label": "Alternate",
            "kind": "primitive_user_load",
            "status": "preview_only",
            "provenance": "invented_example",
            "primitive_loads": []
        }));
        model
    }

    fn create_combination_intent(target_ref: &str, payload: &Value) -> Value {
        let mut intent = modify_intent(
            "Combination",
            target_ref,
            "create_combination",
            "combinations",
            "not_present",
            &serde_json::to_string(payload).expect("payload json"),
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("create");
        intent
    }

    #[test]
    fn explicit_create_subtraction_combination_creates_minuend_subtrahend_record() {
        let model = sample_model_with_second_load_case();
        let before_snapshot = model.clone();
        let payload = json!({
            "id": "combination:C-DIFF",
            "label": "User subtraction combination",
            "basis": "result_state_subtraction",
            "minuend_id": "load:L-1",
            "subtrahend_id": "load:L-2",
            "provenance": "user_entered_local_preview"
        });

        let outcome = apply_operation(
            &model,
            &create_combination_intent("combination:C-DIFF", &payload),
            None,
        );

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
            applied["combinations"][1],
            json!({
                "id": "combination:C-DIFF",
                "label": "User subtraction combination",
                "basis": "result_state_subtraction",
                "minuend_id": "load:L-1",
                "subtrahend_id": "load:L-2",
                "terms": [],
                "provenance": "user_entered_local_preview"
            })
        );

        let mut wrong_shape = payload.clone();
        wrong_shape["id"] = json!("combination:C-DIFF-TERMS");
        wrong_shape["terms"] = json!([{ "load_case": "load:L-1", "factor": 1.0 }]);
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-DIFF-TERMS", &wrong_shape),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-CREATE-COMBINATION-PAYLOAD-INVALID"));
        assert!(blocked.applied_model.is_none());

        let mut self_reference = payload.clone();
        self_reference["id"] = json!("combination:C-DIFF-SELF");
        self_reference["subtrahend_id"] = json!("load:L-1");
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-DIFF-SELF", &self_reference),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-COMBINATION-OPERAND-DUPLICATE"));
        assert!(blocked.applied_model.is_none());

        let mut missing_ref = payload.clone();
        missing_ref["id"] = json!("combination:C-DIFF-MISSING");
        missing_ref["subtrahend_id"] = json!("load:missing");
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-DIFF-MISSING", &missing_ref),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-COMBINATION-OPERAND-LOAD-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_create_range_envelope_combination_creates_operand_mode_record() {
        let model = sample_model_with_second_load_case();
        let payload = json!({
            "id": "combination:C-ENV",
            "label": "User range envelope combination",
            "basis": "range_envelope",
            "operand_ids": ["load:L-1", "load:L-2"],
            "mode": "max_abs",
            "provenance": "user_entered_local_preview"
        });

        let outcome = apply_operation(
            &model,
            &create_combination_intent("combination:C-ENV", &payload),
            None,
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
            applied["combinations"][1],
            json!({
                "id": "combination:C-ENV",
                "label": "User range envelope combination",
                "basis": "range_envelope",
                "operand_ids": ["load:L-1", "load:L-2"],
                "mode": "max_abs",
                "terms": [],
                "provenance": "user_entered_local_preview"
            })
        );

        let mut unknown_mode = payload.clone();
        unknown_mode["id"] = json!("combination:C-ENV-MODE");
        unknown_mode["mode"] = json!("largest");
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-ENV-MODE", &unknown_mode),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-COMBINATION-RANGE-MODE-UNKNOWN"));
        assert!(blocked.applied_model.is_none());

        let mut duplicate_operand = payload.clone();
        duplicate_operand["id"] = json!("combination:C-ENV-DUP");
        duplicate_operand["operand_ids"] = json!(["load:L-1", "load:L-1"]);
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-ENV-DUP", &duplicate_operand),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-COMBINATION-OPERAND-DUPLICATE"));
        assert!(blocked.applied_model.is_none());

        let mut wrong_shape = payload.clone();
        wrong_shape["id"] = json!("combination:C-ENV-SHAPE");
        wrong_shape["minuend_id"] = json!("load:L-1");
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-ENV-SHAPE", &wrong_shape),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-CREATE-COMBINATION-PAYLOAD-INVALID"));
        assert!(blocked.applied_model.is_none());

        let mut empty_operands = payload.clone();
        empty_operands["id"] = json!("combination:C-ENV-EMPTY");
        empty_operands["operand_ids"] = json!([]);
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-ENV-EMPTY", &empty_operands),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-CREATE-COMBINATION-PAYLOAD-INVALID"));
        assert!(blocked.applied_model.is_none());

        let mut unknown_basis = payload.clone();
        unknown_basis["id"] = json!("combination:C-ENV-BASIS");
        unknown_basis["basis"] = json!("user_rule_pack");
        let blocked = apply_operation(
            &model,
            &create_combination_intent("combination:C-ENV-BASIS", &unknown_basis),
            None,
        );
        assert!(codes(&blocked).contains(&"OP-COMBINATION-BASIS-UNSUPPORTED"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn combination_term_creation_requires_a_mechanics_basis_target() {
        let mut model = sample_model_with_second_load_case();
        model["combinations"] = json!([{
            "id": "combination:C-DIFF",
            "label": "User subtraction combination",
            "basis": "result_state_subtraction",
            "minuend_id": "load:L-1",
            "subtrahend_id": "load:L-2",
            "terms": [],
            "provenance": "user_entered_local_preview"
        }]);
        let mut intent = modify_intent(
            "Combination",
            "combination:C-DIFF",
            "create_combination_term",
            "terms",
            "not_present",
            "{\"load_case\":\"load:L-1\",\"factor\":1.0}",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("create");

        let blocked = apply_operation(&model, &intent, None);

        assert!(codes(&blocked).contains(&"OP-COMBINATION-TERM-BASIS-UNSUPPORTED"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn load_case_delete_stays_blocked_for_subtraction_and_range_references() {
        let mut model = sample_model_with_second_load_case();
        model["combinations"] = json!([
            {
                "id": "combination:C-DIFF",
                "label": "User subtraction combination",
                "basis": "result_state_subtraction",
                "minuend_id": "load:L-1",
                "subtrahend_id": "load:L-2",
                "terms": [],
                "provenance": "user_entered_local_preview"
            },
            {
                "id": "combination:C-ENV",
                "label": "User range envelope combination",
                "basis": "range_envelope",
                "operand_ids": ["load:L-2"],
                "mode": "max",
                "terms": [],
                "provenance": "user_entered_local_preview"
            }
        ]);
        let mut intent = modify_intent(
            "Load",
            "load:L-2",
            "delete_load_case",
            "load_cases",
            "load:L-2; Alternate; primitive_user_load; preview_only; primitives=0",
            "not_present",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("delete");

        let blocked = apply_operation(&model, &intent, None);

        assert!(codes(&blocked).contains(&"OP-LOAD-CASE-DELETE-REFERENCED"));
        assert!(blocked.applied_model.is_none());
        assert!(blocked.diagnostics.iter().any(|item| {
            item.code == "OP-LOAD-CASE-DELETE-REFERENCED"
                && item.message.contains("combination:C-DIFF.subtrahend_id")
                && item.message.contains("combination:C-ENV.operand_ids.0")
        }));
    }

    #[test]
    fn explicit_delete_combination_removes_one_combination_only() {
        let mut model = sample_model();
        model["combinations"] = json!([
            {
                "id": "combination:C-OP",
                "label": "Operating",
                "basis": "mechanics",
                "terms": [
                    { "load_case": "load:L-1", "factor": 0.25 },
                    { "load_case": "load:L-2", "factor": 1.0 }
                ],
                "provenance": "invented_example"
            },
            {
                "id": "combination:C-KEEP",
                "label": "Keep",
                "basis": "mechanics",
                "terms": [
                    { "load_case": "load:L-1", "factor": 1.0 }
                ],
                "provenance": "invented_example"
            }
        ]);
        let before_snapshot = model.clone();
        let mut intent = modify_intent(
            "Combination",
            "combination:C-OP",
            "delete_combination",
            "combinations",
            "combination:C-OP; Operating; basis=mechanics; terms=load:L-1 x 0.25; load:L-2 x 1",
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
            applied["combinations"],
            json!([
                {
                    "id": "combination:C-KEEP",
                    "label": "Keep",
                    "basis": "mechanics",
                    "terms": [
                        { "load_case": "load:L-1", "factor": 1.0 }
                    ],
                    "provenance": "invented_example"
                }
            ])
        );

        let mut stale = modify_intent(
            "Combination",
            "combination:C-OP",
            "delete_combination",
            "combinations",
            "combination:C-OP; Operating; basis=mechanics; terms=load:L-1 x 1",
            "not_present",
            "none",
            "dimensionless",
        );
        stale["operation_kind"] = json!("delete");
        let stale_outcome = apply_operation(&model, &stale, None);
        assert!(codes(&stale_outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert!(stale_outcome.applied_model.is_none());

        let mut missing = modify_intent(
            "Combination",
            "combination:C-MISSING",
            "delete_combination",
            "combinations",
            "combination:C-MISSING; Missing; basis=mechanics; terms=none",
            "not_present",
            "none",
            "dimensionless",
        );
        missing["operation_kind"] = json!("delete");
        let blocked = apply_operation(&model, &missing, None);
        assert!(codes(&blocked).contains(&"OP-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_delete_primitive_load_removes_one_indexed_load_only() {
        let mut model = sample_model();
        model["load_cases"][0]["primitive_loads"] = json!([
            {
                "id": "load:L-1-Z",
                "category": "distributed_force",
                "target": { "type": "element", "pipe": "pipe:P-1" },
                "direction": "global_z",
                "magnitude": { "value": -190.0, "unit": "N/m" },
                "dimension": "force_per_length",
                "provenance": "invented_example"
            },
            {
                "id": "load:L-1-Y",
                "category": "concentrated_force",
                "target": { "type": "node", "node": "node:N-2" },
                "direction": "global_y",
                "magnitude": { "value": 125.0, "unit": "N" },
                "dimension": "force",
                "provenance": "invented_example"
            }
        ]);
        let before_snapshot = model.clone();
        let mut intent = modify_intent(
            "Load",
            "load:L-1",
            "delete_primitive_load",
            "primitive_loads.0",
            "load:L-1-Z; distributed_force; element:pipe:P-1; global_z; -190 N/m; force_per_length",
            "not_present",
            "N/m",
            "force_per_length",
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
            applied["load_cases"][0]["primitive_loads"],
            json!([{
                "id": "load:L-1-Y",
                "category": "concentrated_force",
                "target": { "type": "node", "node": "node:N-2" },
                "direction": "global_y",
                "magnitude": { "value": 125.0, "unit": "N" },
                "dimension": "force",
                "provenance": "invented_example"
            }])
        );

        let mut stale = modify_intent(
            "Load",
            "load:L-1",
            "delete_primitive_load",
            "primitive_loads.0",
            "load:L-1-Z; distributed_force; element:pipe:P-1; global_z; -95 N/m; force_per_length",
            "not_present",
            "N/m",
            "force_per_length",
        );
        stale["operation_kind"] = json!("delete");
        let stale_outcome = apply_operation(&model, &stale, None);
        assert!(codes(&stale_outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert!(stale_outcome.applied_model.is_none());

        let mut missing_load = modify_intent(
            "Load",
            "load:L-1",
            "delete_primitive_load",
            "primitive_loads.9",
            "load:L-1-X; distributed_force; element:pipe:P-1; global_z; -95 N/m; force_per_length",
            "not_present",
            "N/m",
            "force_per_length",
        );
        missing_load["operation_kind"] = json!("delete");
        let blocked = apply_operation(&model, &missing_load, None);
        assert!(codes(&blocked).contains(&"OP-PRIMITIVE-LOAD-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_delete_load_case_removes_unreferenced_case_only() {
        let mut model = sample_model();
        model["load_cases"] = json!([
            {
                "id": "load:L-1",
                "label": "Weight",
                "kind": "primitive_user_load",
                "status": "preview_only",
                "provenance": "invented_example",
                "primitive_loads": []
            },
            {
                "id": "load:L-2",
                "label": "Alternate",
                "kind": "primitive_user_load",
                "status": "preview_only",
                "provenance": "invented_example",
                "primitive_loads": [
                    {
                        "id": "load:L-2-Y",
                        "category": "concentrated_force",
                        "target": { "type": "node", "node": "node:N-2" },
                        "direction": "global_y",
                        "magnitude": { "value": 125.0, "unit": "N" },
                        "dimension": "force",
                        "provenance": "invented_example"
                    }
                ]
            }
        ]);
        model["combinations"][0]["terms"] = json!([{ "load_case": "load:L-1", "factor": 1.0 }]);
        let before_snapshot = model.clone();
        let mut intent = modify_intent(
            "Load",
            "load:L-2",
            "delete_load_case",
            "load_cases",
            "load:L-2; Alternate; primitive_user_load; preview_only; primitives=1",
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
        assert_eq!(applied["load_cases"].as_array().unwrap().len(), 1);
        assert_eq!(applied["load_cases"][0]["id"], json!("load:L-1"));

        let mut stale = modify_intent(
            "Load",
            "load:L-2",
            "delete_load_case",
            "load_cases",
            "load:L-2; Alternate; primitive_user_load; preview_only; primitives=0",
            "not_present",
            "none",
            "dimensionless",
        );
        stale["operation_kind"] = json!("delete");
        let stale_outcome = apply_operation(&model, &stale, None);
        assert!(codes(&stale_outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert!(stale_outcome.applied_model.is_none());

        let mut referenced = modify_intent(
            "Load",
            "load:L-1",
            "delete_load_case",
            "load_cases",
            "load:L-1; Weight; primitive_user_load; preview_only; primitives=0",
            "not_present",
            "none",
            "dimensionless",
        );
        referenced["operation_kind"] = json!("delete");
        let blocked = apply_operation(&model, &referenced, None);
        assert!(codes(&blocked).contains(&"OP-LOAD-CASE-DELETE-REFERENCED"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn explicit_delete_pipe_run_removes_unreferenced_pipe_only() {
        let mut model = sample_model();
        model["load_cases"][0]["primitive_loads"] = json!([]);
        model["pipe_segments"].as_array_mut().unwrap().push(json!({
            "id": "pipe:P-2",
            "label": "Branch run",
            "from": "node:N-1",
            "to": "node:N-2",
            "section": {
                "outside_diameter": { "value": 0.114, "unit": "m" },
                "wall_thickness": { "value": 0.006, "unit": "m" }
            },
            "material": "material:steel",
            "provenance": "invented_example"
        }));
        let before_snapshot = model.clone();
        let mut intent = modify_intent(
            "Element",
            "pipe:P-1",
            "delete_pipe_run",
            "pipe_segments",
            "Run; node:N-1->node:N-2; material=material:steel",
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
        assert_eq!(applied["pipe_segments"].as_array().unwrap().len(), 1);
        assert_eq!(applied["pipe_segments"][0]["id"], json!("pipe:P-2"));

        let mut stale = modify_intent(
            "Element",
            "pipe:P-1",
            "delete_pipe_run",
            "pipe_segments",
            "Renamed run; node:N-1->node:N-2; material=material:steel",
            "not_present",
            "none",
            "dimensionless",
        );
        stale["operation_kind"] = json!("delete");
        let stale_outcome = apply_operation(&model, &stale, None);
        assert!(codes(&stale_outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert!(stale_outcome.applied_model.is_none());

        let mut missing = modify_intent(
            "Element",
            "pipe:P-missing",
            "delete_pipe_run",
            "pipe_segments",
            "Missing run; node:N-1->node:N-2; material=material:steel",
            "not_present",
            "none",
            "dimensionless",
        );
        missing["operation_kind"] = json!("delete");
        let blocked = apply_operation(&model, &missing, None);
        assert!(codes(&blocked).contains(&"OP-TARGET-NOT-FOUND"));
        assert!(blocked.applied_model.is_none());
    }

    #[test]
    fn delete_pipe_run_blocks_primitive_load_reference() {
        let mut model = sample_model();
        model["load_cases"][0]["primitive_loads"][0] = json!({
            "id": "load:L-1-Z",
            "category": "distributed_force",
            "target": { "type": "element", "pipe": "pipe:P-1" },
            "direction": "global_z",
            "magnitude": { "value": -190.0, "unit": "N/m" },
            "dimension": "force_per_length",
            "provenance": "invented_example"
        });
        let mut intent = modify_intent(
            "Element",
            "pipe:P-1",
            "delete_pipe_run",
            "pipe_segments",
            "Run; node:N-1->node:N-2; material=material:steel",
            "not_present",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("delete");

        let outcome = apply_operation(&model, &intent, None);

        assert!(codes(&outcome).contains(&"OP-PIPE-DELETE-REFERENCED"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
        assert_eq!(
            outcome.diagnostics[0].affected_refs,
            vec!["pipe:P-1".to_string(), "load:L-1-Z".to_string()]
        );
    }

    #[test]
    fn explicit_delete_support_removes_unreferenced_support_only() {
        let model = sample_model();
        let before_snapshot = model.clone();
        let mut intent = modify_intent(
            "Support",
            "support:S-1",
            "delete_support",
            "supports",
            "Anchor support",
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
            applied["supports"]
                .as_array()
                .expect("supports array")
                .len(),
            0
        );

        let mut stale = modify_intent(
            "Support",
            "support:S-1",
            "delete_support",
            "supports",
            "Renamed support",
            "not_present",
            "none",
            "dimensionless",
        );
        stale["operation_kind"] = json!("delete");
        let stale_outcome = apply_operation(&model, &stale, None);
        assert!(codes(&stale_outcome).contains(&"OP-STALE-BEFORE-VALUE"));
        assert!(stale_outcome.applied_model.is_none());
    }

    #[test]
    fn delete_support_blocks_imposed_displacement_reference() {
        let mut model = sample_model();
        model["load_cases"][0]["primitive_loads"][0] = json!({
            "id": "load:L-1-I1",
            "category": "imposed_displacement",
            "target": { "type": "support", "support": "support:S-1", "dof": "UZ" },
            "direction": "UZ",
            "magnitude": { "value": -0.006, "unit": "m" },
            "dimension": "displacement",
            "provenance": "invented_example"
        });
        let mut intent = modify_intent(
            "Support",
            "support:S-1",
            "delete_support",
            "supports",
            "Anchor support",
            "not_present",
            "none",
            "dimensionless",
        );
        intent["operation_kind"] = json!("delete");

        let outcome = apply_operation(&model, &intent, None);

        assert!(codes(&outcome).contains(&"OP-SUPPORT-DELETE-REFERENCED"));
        assert_eq!(outcome.validation.application_status, "blocked");
        assert!(outcome.applied_model.is_none());
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

        let converted = apply_operation(
            &model,
            &modify_intent(
                "Node",
                "node:N-2",
                "set_field",
                "position.y",
                "0",
                "3.280839895013123",
                "ft",
                "length",
            ),
            None,
        );
        assert!(
            converted.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            converted.diagnostics
        );
        let converted_model = converted.applied_model.expect("applied model");
        let converted_y = converted_model["nodes"][1]["position"]["y"]
            .as_f64()
            .expect("converted y coordinate");
        assert!((converted_y - 1.0).abs() <= 1.0e-12);

        let payload = json!({ "value": 1200.0, "unit": "mm" });
        let payload_converted = apply_operation(
            &model,
            &modify_intent(
                "Node",
                "node:N-2",
                "set_field",
                "position.y",
                "0",
                &payload.to_string(),
                "mm",
                "length",
            ),
            None,
        );
        assert!(
            payload_converted.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            payload_converted.diagnostics
        );
        assert_eq!(
            payload_converted.applied_model.expect("applied model")["nodes"][1]["position"]["y"],
            json!(1.2)
        );

        let incompatible = apply_operation(
            &model,
            &modify_intent(
                "Node",
                "node:N-2",
                "set_field",
                "position.y",
                "0",
                "3.6",
                "Pa",
                "length",
            ),
            None,
        );
        assert!(codes(&incompatible).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));

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

    /// Live probe for TP-APP-R5-FIELDRULES-001: GUI inspector bend-geometry
    /// intents must apply to the session model instead of blocking on
    /// `OP-FIELD-PATH-UNSUPPORTED`.
    #[test]
    fn bend_radius_value_edit_applies_to_existing_component_geometry() {
        let mut model = sample_model();
        model["components"][0]["geometry"] = json!({
            "bend_radius": { "value": 0.45, "unit": "m" }
        });
        let intent = modify_intent(
            "Component",
            "component:C-1",
            "set_field",
            "geometry.bend_radius.value",
            "0.45",
            "{\"value\":0.5,\"unit\":\"m\"}",
            "m",
            "length",
        );
        let outcome = apply_operation(&model, &intent, None);
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
            applied["components"][0]["geometry"]["bend_radius"]["value"],
            json!(0.5)
        );
        assert_eq!(
            applied["components"][0]["geometry"]["bend_radius"]["unit"],
            json!("m")
        );
    }

    #[test]
    fn bend_geometry_authoring_creates_absent_optional_slots_from_explicit_entry() {
        // sample_model's component:C-1 carries no geometry block at all.
        let model = sample_model();
        let before_snapshot = model.clone();
        let intent = modify_intent(
            "Component",
            "component:C-1",
            "set_field",
            "geometry.bend_radius.value",
            "TBD",
            "{\"value\":0.45,\"unit\":\"m\"}",
            "m",
            "length",
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
        assert_eq!(outcome.validation.unit_validation, "passed");
        assert_eq!(outcome.validation.before_state_validation, "passed");
        assert_eq!(outcome.diff_preview.len(), 1);
        assert_eq!(outcome.diff_preview[0].before, "TBD");
        let applied = outcome.applied_model.expect("applied model");
        assert_eq!(
            applied["components"][0]["geometry"]["bend_radius"],
            json!({ "value": 0.45, "unit": "m" })
        );
        let mut expected = before_snapshot.clone();
        expected["components"][0]["geometry"] =
            json!({ "bend_radius": { "value": 0.45, "unit": "m" } });
        assert_eq!(applied, expected, "only the requested slot may be created");
    }

    #[test]
    fn bend_pipe_ref_requires_an_existing_pipe_segment() {
        let model = sample_model();
        let accepted = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "geometry.bend_pipe_ref",
                "TBD",
                "pipe:P-1",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            accepted.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            accepted.diagnostics
        );
        assert_eq!(accepted.validation.reference_validation, "passed");
        assert_eq!(
            accepted.applied_model.expect("applied model")["components"][0]["geometry"]
                ["bend_pipe_ref"],
            json!("pipe:P-1")
        );

        let dangling = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "geometry.bend_pipe_ref",
                "TBD",
                "pipe:P-999",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(codes(&dangling).contains(&"OP-REFERENCE-NOT-FOUND"));
        assert!(dangling.applied_model.is_none());
    }

    #[test]
    fn optional_quantity_rules_enforce_dimension_unit_and_positivity() {
        let model = sample_model();
        // Wrong dimension token for the pinned rule dimension.
        let wrong_dimension = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "geometry.bend_radius.value",
                "TBD",
                "{\"value\":0.45,\"unit\":\"m\"}",
                "m",
                "area",
            ),
            None,
        );
        assert!(codes(&wrong_dimension).contains(&"OP-UNIT-DIMENSION-MISMATCH"));

        // Unit outside the accepted catalog set for the dimension.
        let wrong_unit = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "geometry.bend_radius.value",
                "TBD",
                "{\"value\":0.45,\"unit\":\"Pa\"}",
                "Pa",
                "length",
            ),
            None,
        );
        assert!(codes(&wrong_unit).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));

        // Non-physical zero geometry value.
        let non_positive = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "geometry.bend_radius.value",
                "TBD",
                "{\"value\":0,\"unit\":\"m\"}",
                "m",
                "length",
            ),
            None,
        );
        assert!(codes(&non_positive).contains(&"OP-VALUE-NOT-POSITIVE"));

        // Authoring an absent slot claims a numeric before-value: stale.
        let stale = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "geometry.bend_radius.value",
                "0.45",
                "{\"value\":0.5,\"unit\":\"m\"}",
                "m",
                "length",
            ),
            None,
        );
        assert!(codes(&stale).contains(&"OP-STALE-BEFORE-VALUE"));
    }

    #[test]
    fn expansion_joint_stiffness_edits_apply_and_preserve_entered_units() {
        let mut model = sample_model();
        model["components"][0]["kind"] = json!("expansion_joint");
        model["components"][0]["modifiers"] = json!({
            "axial_stiffness_user_value": { "value": 3200000.0, "unit": "N/m" }
        });
        // In-place edit of the existing user-entered stiffness value.
        let edited = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "modifiers.axial_stiffness_user_value.value",
                "3200000",
                "{\"value\":3400000,\"unit\":\"N/m\"}",
                "N/m",
                "linear_stiffness",
            ),
            None,
        );
        assert!(
            edited.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            edited.diagnostics
        );
        let applied = edited.applied_model.expect("applied model");
        assert_eq!(
            applied["components"][0]["modifiers"]["axial_stiffness_user_value"],
            json!({ "value": 3400000.0, "unit": "N/m" })
        );

        // Authoring the absent torsional slot creates the record verbatim.
        let authored = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "modifiers.torsional_stiffness_user_value.value",
                "TBD",
                "{\"value\":620000,\"unit\":\"N*m/rad\"}",
                "N*m/rad",
                "rotational_stiffness",
            ),
            None,
        );
        assert!(
            authored.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            authored.diagnostics
        );
        assert_eq!(
            authored.applied_model.expect("applied model")["components"][0]["modifiers"]
                ["torsional_stiffness_user_value"],
            json!({ "value": 620000.0, "unit": "N*m/rad" })
        );

        // A plain-number edit may not silently change the stored unit.
        let unit_drift = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "modifiers.axial_stiffness_user_value.value",
                "3200000",
                "3400",
                "N/mm",
                "linear_stiffness",
            ),
            None,
        );
        assert!(codes(&unit_drift).contains(&"OP-UNIT-MISMATCH-CONVERSION-UNAVAILABLE"));
    }

    #[test]
    fn dimensionless_modifier_values_accept_the_stored_none_unit_marker() {
        let mut model = sample_model();
        model["components"][0]["modifiers"] = json!({
            "sif_user_value": { "value": 1.15, "unit": "none" }
        });
        let edited = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "modifiers.sif_user_value.value",
                "1.15",
                "1.3",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            edited.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            edited.diagnostics
        );
        assert_eq!(
            edited.applied_model.expect("applied model")["components"][0]["modifiers"]
                ["sif_user_value"],
            json!({ "value": 1.3, "unit": "none" })
        );

        let authored = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "modifiers.flexibility_factor_user_value.value",
                "TBD",
                "1.08",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            authored.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            authored.diagnostics
        );
        assert_eq!(
            authored.applied_model.expect("applied model")["components"][0]["modifiers"]
                ["flexibility_factor_user_value"],
            json!({ "value": 1.08, "unit": "none" })
        );
    }

    #[test]
    fn mill_tolerance_accepts_explicit_zero_and_rejects_negative_entries() {
        let model = sample_model();
        let zero = apply_operation(
            &model,
            &modify_intent(
                "Element",
                "pipe:P-1",
                "set_field",
                "section.mill_tolerance.value",
                "TBD",
                "{\"value\":0,\"unit\":\"m\"}",
                "m",
                "length",
            ),
            None,
        );
        assert!(
            zero.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            zero.diagnostics
        );
        assert_eq!(
            zero.applied_model.expect("applied model")["pipe_segments"][0]["section"]
                ["mill_tolerance"],
            json!({ "value": 0.0, "unit": "m" })
        );

        let negative = apply_operation(
            &model,
            &modify_intent(
                "Element",
                "pipe:P-1",
                "set_field",
                "section.mill_tolerance.value",
                "TBD",
                "{\"value\":-0.0005,\"unit\":\"m\"}",
                "m",
                "length",
            ),
            None,
        );
        assert!(codes(&negative).contains(&"OP-VALUE-NEGATIVE"));
        assert!(negative.applied_model.is_none());
    }

    #[test]
    fn modulus_basis_ref_enforces_the_schema_id_shape() {
        let model = sample_model();
        let accepted = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "modulus_basis_ref",
                "TBD",
                "tpoint:200C",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            accepted.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            accepted.diagnostics
        );
        assert_eq!(
            accepted.applied_model.expect("applied model")["load_cases"][0]["modulus_basis_ref"],
            json!("tpoint:200C")
        );

        let reserved = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "modulus_basis_ref",
                "TBD",
                "material_base_values",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            reserved.diagnostics.is_empty(),
            "the reserved material_base_values label must stay accepted: {:?}",
            reserved.diagnostics
        );

        let malformed = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "modulus_basis_ref",
                "TBD",
                "9bad id",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(codes(&malformed).contains(&"OP-ID-PATTERN-INVALID"));
        assert!(malformed.applied_model.is_none());
    }

    #[test]
    fn equivalent_static_generation_inputs_author_nested_user_entered_slots() {
        let model = sample_model();
        let gravity = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "equivalent_static.seismic.gravity_acceleration.value",
                "TBD",
                "{\"value\":9.80665,\"unit\":\"m/s^2\"}",
                "m/s^2",
                "acceleration",
            ),
            None,
        );
        assert!(
            gravity.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            gravity.diagnostics
        );
        assert_eq!(
            gravity.applied_model.expect("applied model")["load_cases"][0]["equivalent_static"]
                ["seismic"]["gravity_acceleration"],
            json!({ "value": 9.80665, "unit": "m/s^2" })
        );

        // g-factors are finite user-entered scalars; sign is meaningful.
        let g_factor = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "equivalent_static.seismic.g_factor_x.value",
                "TBD",
                "{\"value\":-0.3,\"unit\":\"none\"}",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            g_factor.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            g_factor.diagnostics
        );

        let direction = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "equivalent_static.wind.direction",
                "TBD",
                "global_x",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            direction.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            direction.diagnostics
        );
        assert_eq!(
            direction.applied_model.expect("applied model")["load_cases"][0]["equivalent_static"]
                ["wind"]["direction"],
            json!("global_x")
        );

        let bad_direction = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "equivalent_static.wind.direction",
                "TBD",
                "north",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(codes(&bad_direction).contains(&"OP-ENUM-TOKEN-INVALID"));

        let exposed = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "equivalent_static.wind.exposed_pipe_refs",
                "TBD",
                "pipe:P-1",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(
            exposed.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            exposed.diagnostics
        );
        assert_eq!(
            exposed.applied_model.expect("applied model")["load_cases"][0]["equivalent_static"]
                ["wind"]["exposed_pipe_refs"],
            json!(["pipe:P-1"])
        );

        let dangling_exposed = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "equivalent_static.wind.exposed_pipe_refs",
                "TBD",
                "pipe:P-1, pipe:P-404",
                "none",
                "dimensionless",
            ),
            None,
        );
        assert!(codes(&dangling_exposed).contains(&"OP-REFERENCE-NOT-FOUND"));

        let non_positive_gravity = apply_operation(
            &model,
            &modify_intent(
                "Load",
                "load:L-1",
                "update_load",
                "equivalent_static.seismic.gravity_acceleration.value",
                "TBD",
                "{\"value\":0,\"unit\":\"m/s^2\"}",
                "m/s^2",
                "acceleration",
            ),
            None,
        );
        assert!(codes(&non_positive_gravity).contains(&"OP-VALUE-NOT-POSITIVE"));
    }

    #[test]
    fn optional_slot_authoring_is_deterministic_and_never_mutates_input() {
        let model = sample_model();
        let snapshot = model.clone();
        let intent = modify_intent(
            "Load",
            "load:L-1",
            "update_load",
            "equivalent_static.wind.pressure.value",
            "TBD",
            "{\"value\":450,\"unit\":\"Pa\"}",
            "Pa",
            "pressure",
        );
        let first_outcome = apply_operation(&model, &intent, None);
        assert_eq!(
            first_outcome.validation.application_status, "applied_to_session_model",
            "determinism check must compare applied outcomes: {:?}",
            first_outcome.diagnostics
        );
        assert_eq!(model, snapshot, "apply must not mutate the input model");
        let first = serde_json::to_string(&first_outcome).unwrap();
        let second = serde_json::to_string(&apply_operation(&model, &intent, None)).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn center_of_gravity_stays_unsupported_pending_payload_ruling() {
        let model = sample_model();
        let outcome = apply_operation(
            &model,
            &modify_intent(
                "Component",
                "component:C-1",
                "set_field",
                "geometry.center_of_gravity",
                "TBD",
                "x=0.1, y=0.2, z=0.3 m",
                "m",
                "length",
            ),
            None,
        );
        assert!(codes(&outcome).contains(&"OP-FIELD-PATH-UNSUPPORTED"));
        assert!(outcome.applied_model.is_none());
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
