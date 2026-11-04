//! The modal result document (DEL-04-09 Design §5.3, increment I3a): a closed,
//! family-typed, transient projection of one run's `ModalOutcome`, or of its
//! refusal, as `openpipestress.modal_result` 0.1.0 under the semantic table
//! `openpipestress.result_semantics/modal-1` and the profile
//! `product_preview_modal_v1`.
//!
//! - **A family of its own.** The document is not a `MechanicsEnvelope` and
//!   adds nothing to `results.v0.3` or `analysis_run.v0.3` (O-8 superseded):
//!   no store, migration, write path or recording support exists for it.
//! - **Only in-run typed evidence.** Every value is copied or derived (ω, f,
//!   T from λ by the kernel's own `frequency_from_eigenvalue`; ω of a shift as
//!   √σ) from the outcome this run returned; nothing is read from a header,
//!   a stored result or the current authority.
//! - **Rows.** Every table row has a stable unique `row_id`, its source
//!   `ordinal` and a `checksum`, the SHA-256 of the checked canonical text of
//!   the row without its checksum; each table's `digest` is the SHA-256 of
//!   the canonical array of its rows' checksums. A row is a record of named
//!   scalars, never an array packed as one value.
//! - **Values.** Each published number is a value object carrying its kind,
//!   its unit (case-aware: a mode's shape, generalized mass and Γ take its
//!   normalization case's unit), its unit status, its own below-normal flag
//!   and its undefined reason. A value the checked profile cannot carry
//!   (non-finite, or of magnitude 2⁵³ or more) is null with its reason.
//! - **Units.** Catalog units only; the mass matrix's translation–rotation
//!   coupling has no catalog family (kg·m·rad⁻¹) and is published with its
//!   dimension exponents and `catalog_family_absent`, and magnitudes on the
//!   kernel's equilibrated basis carry no catalog dimension
//!   (`equilibrated_basis`). λ = ω² and μ are not published.
//! - **Admission.** Experimental: the integrity parameters are the caller's
//!   test-local set, labelled; `public_engineering_result` is false. Nothing
//!   outside the tests calls this module.
use super::eigen::{FiniteCensus, ReportedEndpoint};
use super::integrity::ModalIntegrityParameters;
use super::outcome::{run_modal_dense, ModalOutcome, ModalStanding};
use super::request::ModalPreviewRequest;
use crate::Diagnostic;
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_modal_eigen::clusters::StoppingBound;
use open_pipe_stress_modal_eigen::modal_quantities::{frequency_from_eigenvalue, NormalizationCase};
use open_pipe_stress_modal_eigen::sturm::{Placement, ShiftDecision};
use open_pipe_stress_units::UnitId;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

pub(crate) const MODAL_RESULT_KIND: &str = "openpipestress.modal_result";
pub(crate) const MODAL_RESULT_VERSION: &str = "0.1.0";
pub(crate) const MODAL_RESULT_SEMANTICS: &str = "openpipestress.result_semantics/modal-1";
pub(crate) const MODAL_RESULT_PROFILE: &str = "product_preview_modal_v1";
pub(crate) const MODAL_RESULT_CANONICAL: &str = "openpipestress_jcs_ijson_v1";
/// The profile's two fixed limitations (Design §5.2).
pub(crate) const MODAL_RESULT_LIMITATIONS: [&str; 2] = [
    "Small-displacement, linear-elastic Euler-Bernoulli frame modal preview about the unloaded configuration, with any nonlinear supports at the stated state of a named solved case, with consistent mass on straight members and lumped rigid-body mass on realized arcs and component bodies; numerical integrity does not establish physical correctness.",
    "The integrity checks are numerical estimates under stated premises, not mathematical bounds: each mode's residual is evaluated against a double-double re-formation of the stiffness and mass, its eigenvalue error is a first-order estimate, and the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues. They do not bound modelling, discretization or input error.",
];
/// The members a modal request may carry besides `model` and `materials`.
const MODAL_OPTION_MEMBERS: [&str; 7] = [
    "mode_count",
    "frequency_cutoff",
    "state_case",
    "modulus_basis_ref",
    "modulus_basis_temperature",
    "weight_to_mass_acceleration",
    "include_dynamic_only_restraints",
];
const COMPONENTS: [&str; 6] = ["ux", "uy", "uz", "rx", "ry", "rz"];
const DIRECTIONS: [&str; 3] = ["x", "y", "z"];
/// 2⁵³: the checked profile refuses an integral binary64 of this magnitude.
const PROFILE_LIMIT: f64 = 9_007_199_254_740_992.0;

/// How a kind's unit is selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Selector {
    Fixed,
    /// The mode's normalization case.
    Case,
    /// The DOF pair's class (translation or rotation, each end).
    PairClass,
}

/// One unit rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModalUnit {
    Catalog(UnitId),
    /// No catalog family; the dimension exponents (L, M, T, Θ, A).
    FamilyAbsent([i8; 5]),
    /// A magnitude on the kernel's diagonally equilibrated basis: no catalog
    /// dimension is claimed.
    Equilibrated,
}

use ModalUnit::{Catalog, Equilibrated, FamilyAbsent};
use UnitId as U;

const CASES: [&str; 2] = ["unit_max_translation", "unit_max_rotation"];
const ONE: [(&str, ModalUnit); 1] = [("fixed", Catalog(U::One))];
const EQ: [(&str, ModalUnit); 1] = [("fixed", Equilibrated)];
const METER: [(&str, ModalUnit); 1] = [("fixed", Catalog(U::Meter))];
const KILOGRAM: [(&str, ModalUnit); 1] = [("fixed", Catalog(U::Kilogram))];
const HERTZ: [(&str, ModalUnit); 1] = [("fixed", Catalog(U::Hertz))];
const RAD_PER_S: [(&str, ModalUnit); 1] = [("fixed", Catalog(U::RadianPerSecond))];
const SECOND: [(&str, ModalUnit); 1] = [("fixed", Catalog(U::Second))];
const ACCELERATION: [(&str, ModalUnit); 1] = [("fixed", Catalog(U::MeterPerSecondSquared))];
const SHAPE_TRANSLATION: [(&str, ModalUnit); 2] = [(CASES[0], Catalog(U::One)), (CASES[1], Catalog(U::MeterPerRadian))];
const SHAPE_ROTATION: [(&str, ModalUnit); 2] = [(CASES[0], Catalog(U::RadianPerMeter)), (CASES[1], Catalog(U::One))];
const GENERALIZED_MASS: [(&str, ModalUnit); 2] =
    [(CASES[0], Catalog(U::Kilogram)), (CASES[1], Catalog(U::KilogramSquareMeterPerSquareRadian))];
const PARTICIPATION: [(&str, ModalUnit); 2] = [(CASES[0], Catalog(U::One)), (CASES[1], Catalog(U::RadianPerMeter))];
const MASS_ENTRY: [(&str, ModalUnit); 3] = [
    ("translation_translation", Catalog(U::Kilogram)),
    ("translation_rotation", FamilyAbsent([1, 1, 0, 0, -1])),
    ("rotation_rotation", Catalog(U::KilogramSquareMeterPerSquareRadian)),
];

/// The modal-1 semantic table: every value kind, the table it appears in,
/// its selector and its unit rules. Its canonical text is the committed
/// `fixtures/results/semantic_contract_modal_1.json`, whose SHA-256 the
/// document names.
const MODAL_KINDS: &[(&str, &str, Selector, &[(&str, ModalUnit)])] = &[
    ("node_coordinate", "nodes", Selector::Fixed, &METER),
    ("mass_matrix_entry", "mass_matrix", Selector::PairClass, &MASS_ENTRY),
    ("modal_total_mass", "mass_totals", Selector::Fixed, &KILOGRAM),
    ("modal_free_dof_mass", "mass_totals", Selector::Fixed, &KILOGRAM),
    ("modal_circular_frequency", "modes", Selector::Fixed, &RAD_PER_S),
    ("modal_frequency", "modes", Selector::Fixed, &HERTZ),
    ("modal_period", "modes", Selector::Fixed, &SECOND),
    ("modal_generalized_mass", "modes", Selector::Case, &GENERALIZED_MASS),
    ("modal_cluster_error_estimate", "modes", Selector::Fixed, &ONE),
    ("mode_shape_translation", "shapes", Selector::Case, &SHAPE_TRANSLATION),
    ("mode_shape_rotation", "shapes", Selector::Case, &SHAPE_ROTATION),
    ("modal_participation_factor", "participation", Selector::Case, &PARTICIPATION),
    ("modal_effective_mass", "participation", Selector::Fixed, &KILOGRAM),
    ("modal_effective_mass_fraction", "participation", Selector::Fixed, &ONE),
    ("modal_cumulative_effective_mass_fraction", "participation", Selector::Fixed, &ONE),
    ("cluster_error_estimate", "clusters", Selector::Fixed, &ONE),
    ("computed_circular_frequency", "computed_modes", Selector::Fixed, &RAD_PER_S),
    ("computed_frequency", "computed_modes", Selector::Fixed, &HERTZ),
    ("residual_backward_error", "computed_modes", Selector::Fixed, &ONE),
    ("eigenvalue_condition_estimate", "computed_modes", Selector::Fixed, &ONE),
    ("eigenvalue_error_estimate", "computed_modes", Selector::Fixed, &ONE),
    ("sturm_shift_circular_frequency", "sturm_candidates", Selector::Fixed, &RAD_PER_S),
    ("sturm_formation_term", "sturm_candidates", Selector::Fixed, &EQ),
    ("sturm_formation_separation_ratio", "sturm_candidates", Selector::Fixed, &ONE),
    ("sturm_factor_term", "sturm_candidates", Selector::Fixed, &EQ),
    ("sturm_backward_error", "sturm_candidates", Selector::Fixed, &EQ),
    ("sturm_inverse_norm_estimate", "sturm_candidates", Selector::Fixed, &EQ),
    ("sturm_conditioning_product", "sturm_candidates", Selector::Fixed, &ONE),
    ("sturm_separation_ratio", "sturm_candidates", Selector::Fixed, &ONE),
    ("sturm_growth_factor", "sturm_candidates", Selector::Fixed, &ONE),
    ("sturm_max_abs_lower_factor", "sturm_candidates", Selector::Fixed, &EQ),
    ("sturm_max_abs_pivot", "sturm_candidates", Selector::Fixed, &EQ),
    ("sturm_max_abs_scaled_diagonal", "sturm_candidates", Selector::Fixed, &EQ),
    ("identity_relative_discrepancy", "identity", Selector::Fixed, &ONE),
    ("member_length", "discretization", Selector::Fixed, &METER),
    ("member_fixed_fixed_frequency", "discretization", Selector::Fixed, &HERTZ),
    ("member_frequency_ratio", "discretization", Selector::Fixed, &ONE),
    ("basis_no_translation_threshold", "header", Selector::Fixed, &ONE),
    ("basis_bounding_box_diagonal", "header", Selector::Fixed, &METER),
    ("basis_mass_center", "header", Selector::Fixed, &METER),
    ("request_frequency_cutoff", "header", Selector::Fixed, &HERTZ),
    ("request_weight_to_mass_acceleration", "header", Selector::Fixed, &ACCELERATION),
    ("identity_acceleration", "header", Selector::Fixed, &ACCELERATION),
    ("placement_circular_frequency", "header", Selector::Fixed, &RAD_PER_S),
    ("sturm_premise_parameter", "header", Selector::Fixed, &ONE),
    ("orthogonality_max_ratio", "header", Selector::Fixed, &ONE),
    ("formation_difference", "header", Selector::Fixed, &EQ),
    ("reformed_norm", "header", Selector::Fixed, &EQ),
    ("section_derivation_difference", "header", Selector::Fixed, &ONE),
];

fn modal_doc_selector(selector: Selector) -> &'static str {
    match selector {
        Selector::Fixed => "fixed",
        Selector::Case => "normalization_case",
        Selector::PairClass => "dof_pair_class",
    }
}

fn modal_doc_rule_value(unit: ModalUnit) -> Value {
    match unit {
        Catalog(id) => {
            let definition = id.definition();
            json!({"unit_status": "catalog", "unit": id.catalog_id(), "symbol": definition.symbol,
                "dimension": definition.dimension.as_str()})
        }
        FamilyAbsent(e) => json!({"unit_status": "catalog_family_absent", "unit": null, "symbol": null, "dimension": null,
            "dimension_exponents": {"length": e[0], "mass": e[1], "time": e[2], "temperature": e[3], "angle": e[4]}}),
        Equilibrated => json!({"unit_status": "equilibrated_basis", "unit": null, "symbol": null, "dimension": null}),
    }
}

/// The modal-1 semantic table as a value (its canonical text is the
/// committed table fixture).
pub(crate) fn modal_semantic_table() -> Value {
    let kinds: Vec<Value> = MODAL_KINDS
        .iter()
        .map(|(kind, table, selector, rules)| {
            let units: Map<String, Value> = rules.iter().map(|(key, unit)| ((*key).to_string(), modal_doc_rule_value(*unit))).collect();
            json!({"kind": kind, "table": table, "selector": modal_doc_selector(*selector), "units": units})
        })
        .collect();
    json!({"id": MODAL_RESULT_SEMANTICS, "document_kind": MODAL_RESULT_KIND, "schema_version": MODAL_RESULT_VERSION,
        "canonical_profile": MODAL_RESULT_CANONICAL, "kinds": kinds})
}

/// The SHA-256 (lowercase hex) of a value's checked canonical text; `None`
/// outside the profile.
pub(crate) fn modal_doc_sha256(value: &Value) -> Option<String> {
    let text = serde_json::to_string(value).ok()?;
    let canonical = canonical_json_checked_v1_text(&text).ok()?;
    Some(Sha256::digest(canonical.as_bytes()).iter().map(|b| format!("{b:02x}")).collect())
}

fn modal_doc_rule_of(kind: &str, key: &str) -> ModalUnit {
    MODAL_KINDS
        .iter()
        .find(|(k, ..)| *k == kind)
        .and_then(|(_, _, _, rules)| rules.iter().find(|(k, _)| *k == key))
        .map(|(_, unit)| *unit)
        .unwrap_or_else(|| unreachable!("modal-1 has no unit rule for the projection's kind and key"))
}

/// One value object: `kind`, `value`, `unit`, `unit_status`, `below_normal`
/// and `undefined_reason` (module docs). `key` selects the unit rule.
fn modal_doc_quantity(kind: &str, key: &str, value: Option<f64>, below_normal: bool, undefined: Option<&str>) -> Value {
    let (number, reason) = match (value, undefined) {
        (_, Some(reason)) => (Value::Null, Value::from(reason)),
        (None, None) => (Value::Null, Value::from("not_formed")),
        (Some(v), None) if !v.is_finite() => (Value::Null, Value::from("not_finite")),
        (Some(v), None) if v.abs() >= PROFILE_LIMIT => (Value::Null, Value::from("outside_canonical_profile")),
        (Some(v), None) => (json!(v), Value::Null),
    };
    let mut out = Map::new();
    out.insert("kind".into(), json!(kind));
    out.insert("value".into(), number);
    match modal_doc_rule_of(kind, key) {
        Catalog(id) => {
            out.insert("unit".into(), json!(id.catalog_id()));
            out.insert("unit_status".into(), json!("catalog"));
        }
        FamilyAbsent(e) => {
            out.insert("unit".into(), Value::Null);
            out.insert("unit_status".into(), json!("catalog_family_absent"));
            out.insert("dimension_exponents".into(), json!({"length": e[0], "mass": e[1], "time": e[2], "temperature": e[3], "angle": e[4]}));
        }
        Equilibrated => {
            out.insert("unit".into(), Value::Null);
            out.insert("unit_status".into(), json!("equilibrated_basis"));
        }
    }
    out.insert("below_normal".into(), json!(below_normal));
    out.insert("undefined_reason".into(), reason);
    Value::Object(out)
}

fn modal_doc_fixed(kind: &str, value: f64) -> Value {
    modal_doc_quantity(kind, "fixed", Some(value), false, None)
}

fn modal_doc_flagged(kind: &str, value: f64, below_normal: bool) -> Value {
    modal_doc_quantity(kind, "fixed", Some(value), below_normal, None)
}

fn modal_doc_optional(kind: &str, value: Option<f64>, undefined: &str) -> Value {
    match value {
        Some(v) => modal_doc_fixed(kind, v),
        None => modal_doc_quantity(kind, "fixed", None, false, Some(undefined)),
    }
}

/// A sealed row: `fields` with its `row_id`, `ordinal` and `checksum`.
fn modal_doc_row(row_id: String, ordinal: usize, fields: Value) -> Value {
    let mut row = match fields {
        Value::Object(map) => map,
        _ => unreachable!("a row's fields are an object"),
    };
    row.insert("row_id".into(), json!(row_id));
    row.insert("ordinal".into(), json!(ordinal));
    let checksum = modal_doc_sha256(&Value::Object(row.clone()));
    row.insert("checksum".into(), json!(checksum));
    Value::Object(row)
}

/// A published table: its rows, their count and its digest.
fn modal_doc_table(rows: Vec<Value>) -> Value {
    let checksums: Vec<Value> = rows.iter().map(|row| row["checksum"].clone()).collect();
    json!({"status": "published", "row_count": rows.len(), "digest": modal_doc_sha256(&Value::Array(checksums)), "rows": rows})
}

fn modal_doc_dof_id(node_ids: &[String], dof: usize) -> String {
    format!("dof:{}:{}", node_ids[dof / 6], COMPONENTS[dof % 6])
}

fn modal_doc_node_id(node_id: &str) -> String {
    format!("node:{node_id}")
}

fn modal_doc_mode_ref(index: usize) -> String {
    format!("mode:{index}")
}

fn modal_doc_computed_ref(index: usize) -> String {
    format!("computed:{index}")
}

fn modal_doc_cluster_ref(index: usize) -> String {
    format!("cluster:{index}")
}

fn modal_doc_case(case: NormalizationCase) -> &'static str {
    match case {
        NormalizationCase::UnitMaxTranslation => CASES[0],
        NormalizationCase::UnitMaxRotation => CASES[1],
    }
}

fn modal_doc_standing(standing: &ModalStanding) -> Value {
    match standing {
        ModalStanding::ChecksPassed => json!({"token": "checks_passed", "reasons": []}),
        ModalStanding::Sensitive(reasons) => json!({"token": "sensitive", "reasons": reasons}),
    }
}

fn modal_doc_stop(stop: StoppingBound) -> &'static str {
    match stop {
        StoppingBound::ModeCount => "mode_count",
        StoppingBound::FrequencyCutoff => "frequency_cutoff",
        StoppingBound::ModeCountAndFrequencyCutoff => "mode_count_and_frequency_cutoff",
        StoppingBound::SpectrumExhausted => "spectrum_exhausted",
    }
}

fn modal_doc_decision(decision: &ShiftDecision) -> &'static str {
    match decision {
        ShiftDecision::Accepted => "accepted",
        ShiftDecision::OutsideGap => "outside_gap",
        ShiftDecision::OutOfRange(_) => "out_of_range",
        ShiftDecision::SeparationOutOfRange { .. } => "separation_out_of_range",
        ShiftDecision::FactorizationRejected(_) => "factorization_rejected",
        ShiftDecision::Unseparated => "unseparated",
        ShiftDecision::GrowthLimit => "growth_limit",
        ShiftDecision::IllConditioned => "ill_conditioned",
    }
}

/// ω of a positive eigenvalue-domain value (λ = ω², σ); `None` otherwise.
fn modal_doc_angular(value: f64) -> Option<f64> {
    (value.is_finite() && value > 0.0).then(|| value.sqrt())
}

fn modal_doc_diagnostic_row(prefix: &str, ordinal: usize, diagnostic: &Diagnostic, source: &str) -> Value {
    modal_doc_row(
        format!("{prefix}:{ordinal}"),
        ordinal,
        json!({"diagnostic_id": diagnostic.id, "code": diagnostic.code, "severity": diagnostic.severity, "message": diagnostic.message,
            "affected_refs": diagnostic.affected_refs, "source": source}),
    )
}

/// Every parameter, by its field name (§3.4, as built). Nothing defaults.
fn modal_doc_parameters(params: &ModalIntegrityParameters) -> Value {
    json!({
        "infinite_mu_relative_threshold": params.infinite_mu_relative_threshold,
        "mass_pivot_relative_threshold": params.mass_pivot_relative_threshold,
        "resolution_floor_constant": params.resolution_floor_constant,
        "resolution_floor_multiple": params.resolution_floor_multiple,
        "formed_backward_error_multiple": params.formed_backward_error_multiple,
        "screen_error_estimate_threshold": params.screen_error_estimate_threshold,
        "max_ql_iterations_per_eigenvalue": params.max_ql_iterations_per_eigenvalue,
        "residual_threshold": params.residual_threshold,
        "orthogonality_threshold": params.orthogonality_threshold,
        "cluster_tolerance": params.cluster_tolerance,
        "no_translation_threshold": params.no_translation_threshold,
        "sturm_conditioning_multiple": params.sturm_conditioning_multiple,
        "sturm_separation_multiple": params.sturm_separation_multiple,
        "sturm_growth_limit": params.sturm_growth_limit,
        "sturm_max_candidates": params.sturm_max_candidates,
        "sturm_gap_log_positions": params.sturm_gap_log_positions,
        "sturm_whole_spectrum_offsets": params.sturm_whole_spectrum_offsets,
        "identity_tolerance": {"relative": params.identity_tolerance.relative, "absolute": params.identity_tolerance.absolute},
    })
}

/// The exact captured binding (Design §5.3): the request's hashes and its
/// options verbatim, the state case and the full parameter set.
fn modal_doc_binding(request: &Value, params: &ModalIntegrityParameters, state_case: Option<(usize, &str)>) -> Value {
    let options: Map<String, Value> = MODAL_OPTION_MEMBERS
        .iter()
        .filter_map(|name| request.get(*name).map(|value| ((*name).to_string(), value.clone())))
        .collect();
    let undeclared: Vec<&str> = request
        .as_object()
        .map(|members| {
            members.keys().map(String::as_str).filter(|k| *k != "model" && *k != "materials" && !MODAL_OPTION_MEMBERS.contains(k)).collect()
        })
        .unwrap_or_default();
    let materials = request.get("materials").cloned().unwrap_or_else(|| json!([]));
    let state = state_case.map(|(index, id)| {
        json!({"case_id": id, "case_index": index, "case_sha256": modal_doc_sha256(&request["model"]["load_cases"][index])})
    });
    let parameters = modal_doc_parameters(params);
    json!({
        "request_sha256": modal_doc_sha256(request),
        "model_sha256": request.get("model").and_then(modal_doc_sha256),
        "materials_sha256": modal_doc_sha256(&materials),
        "materials_member": if request.get("materials").is_some() { "authored" } else { "absent" },
        "options": options,
        "undeclared_members": undeclared,
        "state_case": state,
        "integrity_parameters_sha256": modal_doc_sha256(&parameters),
        "integrity_parameters": parameters,
    })
}

/// The identity members every document carries.
fn modal_doc_identity(binding: &Value, label: &str) -> Map<String, Value> {
    let run = modal_doc_sha256(&json!({"request_sha256": binding["request_sha256"], "integrity_parameters_sha256": binding["integrity_parameters_sha256"]}))
        .unwrap_or_default();
    let mut doc = Map::new();
    doc.insert("document_kind".into(), json!(MODAL_RESULT_KIND));
    doc.insert("schema_version".into(), json!(MODAL_RESULT_VERSION));
    doc.insert(
        "semantic_contract".into(),
        json!({"id": MODAL_RESULT_SEMANTICS, "sha256": modal_doc_sha256(&modal_semantic_table())}),
    );
    doc.insert("profile".into(), json!({"profile_id": MODAL_RESULT_PROFILE, "limitations": MODAL_RESULT_LIMITATIONS}));
    doc.insert("canonical_profile".into(), json!(MODAL_RESULT_CANONICAL));
    doc.insert(
        "admission".into(),
        json!({"status": "experimental", "public_engineering_result": false, "parameter_standing": "test_local", "parameter_label": label}),
    );
    doc.insert(
        "run".into(),
        json!({"run_id": format!("modal-run:{}", run.get(..16).unwrap_or("")), "mode_id_scope": "run_local", "solver_path": "dense_scrutiny"}),
    );
    doc.insert("binding".into(), binding.clone());
    doc
}

fn modal_doc_refused(binding: &Value, label: &str, refusals: &[Diagnostic]) -> Value {
    let mut doc = modal_doc_identity(binding, label);
    doc.insert("status".into(), json!("refused"));
    let rows = refusals.iter().enumerate().map(|(i, d)| modal_doc_diagnostic_row("refusal", i, d, "modal")).collect();
    doc.insert("refusals".into(), modal_doc_table(rows));
    Value::Object(doc)
}

/// The document of one modal request under `params` (module docs): the
/// solved projection, or the refused one with every blocking diagnostic.
/// `label` names the caller's parameter set.
pub(crate) fn modal_result_document(request: &Value, params: &ModalIntegrityParameters, label: &str) -> Value {
    let parsed = match ModalPreviewRequest::parse_modal_request(request.clone()) {
        Ok(parsed) => parsed,
        Err(refusal) => {
            let binding = modal_doc_binding(request, params, None);
            let diagnostic = crate::diag("diagnostic:modal:request:parse", "MODAL_REQUEST_INVALID", "blocking", refusal, vec!["request".into()]);
            return modal_doc_refused(&binding, label, &[diagnostic]);
        }
    };
    match run_modal_dense(&parsed, params) {
        Err(refusals) => {
            let state = parsed.state_case.as_deref().and_then(|id| parsed.model.load_cases.iter().position(|c| c.id == id).map(|i| (i, id)));
            modal_doc_refused(&modal_doc_binding(request, params, state), label, &refusals)
        }
        Ok(outcome) => {
            let state = outcome.state_case.as_ref().map(|case| (case.case_index, case.case_id.as_str()));
            let binding = modal_doc_binding(request, params, state);
            modal_doc_solved(&outcome, &binding, label)
        }
    }
}

fn modal_doc_solved(outcome: &ModalOutcome, binding: &Value, label: &str) -> Value {
    let ids = &outcome.node_ids;
    let dofs = 6 * ids.len();
    let mut free_index = vec![None; dofs];
    for (i, &dof) in outcome.free_dofs.iter().enumerate() {
        free_index[dof] = Some(i);
    }
    let mut doc = modal_doc_identity(binding, label);
    doc.insert("status".into(), json!("solved"));

    // The basis and the request quantities.
    let state = outcome.state_case.as_ref().map(|case| {
        let linear = outcome.nonlinear_state.as_ref();
        json!({"case_id": case.case_id, "case_index": case.case_index, "linearized": linear.is_some(),
            "solver_mode": linear.map(|s| s.solver_mode.as_str()),
            "restrained_dof_refs": linear.map(|s| s.restrained_dofs.iter().map(|&d| modal_doc_dof_id(ids, d)).collect::<Vec<_>>())})
    });
    let center = outcome.mass_summary.center_of_gravity.map(|c| {
        json!({"x": modal_doc_fixed("basis_mass_center", c[0]), "y": modal_doc_fixed("basis_mass_center", c[1]), "z": modal_doc_fixed("basis_mass_center", c[2])})
    });
    doc.insert(
        "basis".into(),
        json!({
            "route": if outcome.exact_route { "exact_pressure" } else { "pressure_free" },
            "no_translation_threshold": modal_doc_fixed("basis_no_translation_threshold", outcome.no_translation_threshold),
            "bounding_box_diagonal": modal_doc_fixed("basis_bounding_box_diagonal", outcome.bounding_box_diagonal),
            "weight_to_mass_acceleration": outcome.weight_to_mass_acceleration.map_or(Value::Null, |a| modal_doc_fixed("request_weight_to_mass_acceleration", a)),
            "identity_acceleration": modal_doc_fixed("identity_acceleration", outcome.identity.acceleration),
            "mass_center": center,
            "state_case": state,
        }),
    );
    let (mode_count, cutoff) = match outcome.bound {
        open_pipe_stress_modal_eigen::clusters::ExtractionBound::ModeCount(n) => (Some(n), None),
        open_pipe_stress_modal_eigen::clusters::ExtractionBound::FrequencyCutoff(f) => (None, Some(f)),
        open_pipe_stress_modal_eigen::clusters::ExtractionBound::Both { mode_count, frequency_cutoff } => (Some(mode_count), Some(frequency_cutoff)),
    };
    let computed_count = outcome.integrity.computed.len();
    let endpoint = match outcome.completed.endpoint {
        ReportedEndpoint::BelowComputedCluster { next_mode } => {
            json!({"kind": "below_computed_cluster", "next_computed_ref": modal_doc_computed_ref(next_mode + 1), "established_finite_mode_count": null})
        }
        ReportedEndpoint::BelowLowestMode => json!({"kind": "below_lowest_mode", "next_computed_ref": modal_doc_computed_ref(1), "established_finite_mode_count": null}),
        ReportedEndpoint::WholeFiniteSpectrum { established_finite_mode_count } => {
            json!({"kind": "whole_finite_spectrum", "next_computed_ref": null, "established_finite_mode_count": established_finite_mode_count})
        }
    };
    doc.insert(
        "extraction".into(),
        json!({
            "mode_count": mode_count,
            "frequency_cutoff": cutoff.map_or(Value::Null, |f| modal_doc_fixed("request_frequency_cutoff", f)),
            "stopped_by": modal_doc_stop(outcome.reported.stopped_by),
            "bounded_count": outcome.reported.bounded_count,
            "extension_count": outcome.reported.extension_count,
            "count": outcome.reported.count,
            "computed_count": computed_count,
            "endpoint": endpoint,
        }),
    );
    doc.insert(
        "census".into(),
        match &outcome.integrity.census {
            FiniteCensus::Established { finite_mode_count, structurally_massless } => json!({"kind": "established",
                "finite_mode_count": finite_mode_count, "structurally_massless": structurally_massless,
                "nonpositive_mu": null, "rejected_mu": null, "mass_block_established": null}),
            FiniteCensus::Unresolved { structurally_massless, nonpositive_mu, rejected_mu, mass_block_established } => json!({"kind": "unresolved",
                "finite_mode_count": null, "structurally_massless": structurally_massless, "nonpositive_mu": nonpositive_mu,
                "rejected_mu": rejected_mu, "mass_block_established": mass_block_established}),
        },
    );
    doc.insert("standing".into(), modal_doc_standing(&outcome.standing));

    // Check 4's header.
    let sturm = &outcome.integrity.sturm;
    let placement = match sturm.evidence.placement {
        Placement::Given => json!({"kind": "given", "lower": null, "upper": null, "established_finite_mode_count": null}),
        Placement::Gap { lower, upper } => json!({"kind": "gap",
            "lower": modal_doc_quantity("placement_circular_frequency", "fixed", modal_doc_angular(lower), false, None),
            "upper": modal_doc_quantity("placement_circular_frequency", "fixed", modal_doc_angular(upper), false, None),
            "established_finite_mode_count": null}),
        Placement::AboveSpectrum { top_eigenvalue, established_finite_mode_count } => json!({"kind": "above_spectrum",
            "lower": modal_doc_quantity("placement_circular_frequency", "fixed", modal_doc_angular(top_eigenvalue), false, None),
            "upper": null, "established_finite_mode_count": established_finite_mode_count}),
    };
    doc.insert(
        "sturm".into(),
        json!({"count": sturm.count, "reported": sturm.reported, "placement": placement,
            "conditioning_multiple": modal_doc_fixed("sturm_premise_parameter", sturm.evidence.conditioning_multiple),
            "conditioning_limit": modal_doc_fixed("sturm_premise_parameter", sturm.evidence.conditioning_limit),
            "separation_multiple": modal_doc_fixed("sturm_premise_parameter", sturm.evidence.separation_multiple),
            "growth_limit": modal_doc_fixed("sturm_premise_parameter", sturm.evidence.growth_limit)}),
    );
    let orthogonality = &outcome.integrity.orthogonality;
    doc.insert(
        "orthogonality".into(),
        json!({"max_ratio": modal_doc_flagged("orthogonality_max_ratio", orthogonality.max_ratio, orthogonality.max_ratio_below_normal_range),
            "pair": orthogonality.pair.map(|(a, b)| json!([modal_doc_mode_ref(a + 1), modal_doc_mode_ref(b + 1)]))}),
    );
    let formation = &outcome.integrity.formation;
    let free_ref = |row: usize| outcome.free_dofs.get(row).map(|&dof| modal_doc_dof_id(ids, dof));
    doc.insert(
        "formation".into(),
        json!({
            "stiffness_difference": modal_doc_fixed("formation_difference", formation.stiffness.norm),
            "stiffness_row_ref": free_ref(formation.stiffness.row),
            "stiffness_off_pattern": formation.stiffness.off_pattern,
            "mass_difference": modal_doc_fixed("formation_difference", formation.mass.norm),
            "mass_row_ref": free_ref(formation.mass.row),
            "mass_off_pattern": formation.mass.off_pattern,
            "reformed_stiffness_norm": modal_doc_fixed("reformed_norm", formation.scaling.reformed_stiffness_norm),
            "reformed_mass_norm": modal_doc_fixed("reformed_norm", formation.scaling.reformed_mass_norm),
            "section": formation.section.map(|s| json!({
                "area": modal_doc_fixed("section_derivation_difference", s.area),
                "second_moment": modal_doc_fixed("section_derivation_difference", s.second_moment),
                "torsion_constant": modal_doc_fixed("section_derivation_difference", s.torsion_constant)})),
        }),
    );
    let guard = &outcome.guard;
    doc.insert(
        "guard".into(),
        json!({"global_dimension": guard.global_dimension, "free_dimension": guard.free_dimension,
            "static_estimate_bytes": guard.static_estimate_bytes.to_string(), "ceiling_bytes": guard.ceiling_bytes.to_string(),
            "eigen_core_bytes": guard.eigen_core_bytes.to_string(), "operation_estimate": guard.operation_estimate.to_string(),
            "statement": "stated formulas, not measurements; no work guard is applied (O-4)"}),
    );

    let mut tables = Map::new();
    // Nodes and DOFs.
    let nodes = ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let c = outcome.node_coordinates[i];
            modal_doc_row(modal_doc_node_id(id), i, json!({"node_id": id, "x": modal_doc_fixed("node_coordinate", c[0]), "y": modal_doc_fixed("node_coordinate", c[1]),
                "z": modal_doc_fixed("node_coordinate", c[2])}))
        })
        .collect();
    tables.insert("nodes".into(), modal_doc_table(nodes));
    let dof_rows = (0..dofs)
        .map(|dof| {
            modal_doc_row(modal_doc_dof_id(ids, dof), dof, json!({"node_ref": modal_doc_node_id(&ids[dof / 6]), "component": COMPONENTS[dof % 6],
                "free_index": free_index[dof]}))
        })
        .collect();
    tables.insert("dofs".into(), modal_doc_table(dof_rows));

    // The formed global mass: the authoritative payload of the mass relations.
    let class = |dof: usize| if dof % 6 < 3 { "translation" } else { "rotation" };
    let mass_rows = outcome
        .mass_upper_entries
        .iter()
        .enumerate()
        .map(|(i, &(row, col, value))| {
            let pair = match (class(row), class(col)) {
                ("translation", "translation") => "translation_translation",
                ("rotation", "rotation") => "rotation_rotation",
                _ => "translation_rotation",
            };
            let (a, b) = (modal_doc_dof_id(ids, row), modal_doc_dof_id(ids, col));
            modal_doc_row(format!("mass:{a}|{b}"), i, json!({"row_dof_ref": a, "col_dof_ref": b, "pair_class": pair,
                "value": modal_doc_quantity("mass_matrix_entry", pair, Some(value), false, None)}))
        })
        .collect();
    tables.insert("mass_matrix".into(), modal_doc_table(mass_rows));
    let totals = (0..3)
        .map(|k| {
            modal_doc_row(format!("mass-total:{}", DIRECTIONS[k]), k, json!({"direction": DIRECTIONS[k], "basis_ref": {"ref_type": "model", "ref_id": "model"},
                "total_mass": modal_doc_fixed("modal_total_mass", outcome.mass_summary.total_by_direction[k]),
                "free_dof_mass": modal_doc_fixed("modal_free_dof_mass", outcome.free_dof_mass[k])}))
        })
        .collect();
    tables.insert("mass_totals".into(), modal_doc_table(totals));

    // The reported modes, their shapes and participation.
    let mut mode_rows = Vec::new();
    let mut shape_rows = Vec::new();
    let mut participation_rows = Vec::new();
    for mode in &outcome.modes {
        let case = modal_doc_case(mode.normalization);
        let quantity = |kind: &str, value: f64, below: bool| modal_doc_quantity(kind, "fixed", Some(value), below, None);
        mode_rows.push(modal_doc_row(
            modal_doc_mode_ref(mode.index),
            mode.index - 1,
            json!({"mode_index": mode.index, "computed_ref": modal_doc_computed_ref(mode.index), "cluster_ref": modal_doc_cluster_ref(mode.cluster + 1),
                "normalization_case": case, "reference_dof_ref": modal_doc_dof_id(ids, mode.reference_dof), "standing": modal_doc_standing(&mode.standing),
                "circular_frequency": quantity("modal_circular_frequency", mode.circular_frequency, false),
                "frequency": quantity("modal_frequency", mode.frequency, false),
                "period": quantity("modal_period", mode.period, false),
                "generalized_mass": modal_doc_quantity("modal_generalized_mass", case, Some(mode.generalized_mass), mode.below_normal_range.generalized_mass, None),
                "cluster_error_estimate": quantity("modal_cluster_error_estimate", mode.cluster_error_estimate, false)}),
        ));
        for dof in 0..dofs {
            let kind = if dof % 6 < 3 { "mode_shape_translation" } else { "mode_shape_rotation" };
            let below = mode.below_normal_range.shape.contains(&dof);
            let id = modal_doc_dof_id(ids, dof);
            shape_rows.push(modal_doc_row(
                format!("shape:{}:{id}", modal_doc_mode_ref(mode.index)),
                (mode.index - 1) * dofs + dof,
                json!({"mode_ref": modal_doc_mode_ref(mode.index), "dof_ref": id, "component": COMPONENTS[dof % 6],
                    "value": modal_doc_quantity(kind, case, Some(mode.shape[dof]), below, None)}),
            ));
        }
        let below = &mode.below_normal_range;
        for k in 0..3 {
            let fraction = match mode.effective_mass_fraction[k] {
                Some(f) => modal_doc_quantity("modal_effective_mass_fraction", "fixed", Some(f), below.effective_mass_fraction[k], None),
                None => modal_doc_quantity("modal_effective_mass_fraction", "fixed", None, false, Some("total_mass_zero")),
            };
            let cumulative = match mode.cumulative_effective_mass_fraction[k] {
                Some(f) => modal_doc_quantity("modal_cumulative_effective_mass_fraction", "fixed", Some(f), false, None),
                None => modal_doc_quantity("modal_cumulative_effective_mass_fraction", "fixed", None, false, Some("total_mass_zero")),
            };
            participation_rows.push(modal_doc_row(
                format!("participation:{}:{}", modal_doc_mode_ref(mode.index), DIRECTIONS[k]),
                (mode.index - 1) * 3 + k,
                json!({"mode_ref": modal_doc_mode_ref(mode.index), "direction": DIRECTIONS[k],
                    "participation_factor": modal_doc_quantity("modal_participation_factor", case, Some(mode.participation[k]), below.participation[k], None),
                    "effective_mass": modal_doc_quantity("modal_effective_mass", "fixed", Some(mode.effective_mass[k]), below.effective_mass[k], None),
                    "effective_mass_fraction": fraction, "cumulative_effective_mass_fraction": cumulative}),
            ));
        }
    }
    tables.insert("modes".into(), modal_doc_table(mode_rows));
    tables.insert("shapes".into(), modal_doc_table(shape_rows));
    tables.insert("participation".into(), modal_doc_table(participation_rows));

    // Clusters and every computed mode's check-2 evidence.
    let clusters = outcome
        .clusters
        .iter()
        .map(|c| {
            modal_doc_row(modal_doc_cluster_ref(c.id + 1), c.id, json!({"cluster_index": c.id + 1, "first_computed_ref": modal_doc_computed_ref(c.modes.start + 1),
                "last_computed_ref": modal_doc_computed_ref(c.modes.end), "reported": c.reported,
                "error_estimate": modal_doc_fixed("cluster_error_estimate", c.error_estimate)}))
        })
        .collect();
    tables.insert("clusters".into(), modal_doc_table(clusters));
    let computed = outcome
        .integrity
        .computed
        .iter()
        .map(|c| {
            let (f, omega, _) = frequency_from_eigenvalue(c.eigenvalue);
            modal_doc_row(modal_doc_computed_ref(c.index + 1), c.index, json!({"computed_index": c.index + 1, "cluster_ref": modal_doc_cluster_ref(c.cluster + 1),
                "reported": c.reported, "mode_ref": if c.reported { Value::from(modal_doc_mode_ref(c.index + 1)) } else { Value::Null },
                "band_recovered": c.band_recovered,
                "circular_frequency": modal_doc_fixed("computed_circular_frequency", omega), "frequency": modal_doc_fixed("computed_frequency", f),
                "backward_error": modal_doc_fixed("residual_backward_error", c.residual.backward_error),
                "condition": modal_doc_fixed("eigenvalue_condition_estimate", c.residual.condition),
                "error_estimate": modal_doc_fixed("eigenvalue_error_estimate", c.residual.error_estimate)}))
        })
        .collect();
    tables.insert("computed_modes".into(), modal_doc_table(computed));
    let candidates = sturm
        .evidence
        .candidates
        .iter()
        .enumerate()
        .map(|(j, c)| {
            let f = c.factored.as_ref();
            let factored = |kind: &str, value: Option<f64>| modal_doc_optional(kind, value, "not_factored");
            modal_doc_row(format!("sturm-candidate:{}", j + 1), j, json!({"candidate_index": j + 1, "decision": modal_doc_decision(&c.decision),
                "accepted": c.decision == ShiftDecision::Accepted,
                "negative_pivot_count": f.map(|f| f.negative_pivot_count),
                "shift": modal_doc_quantity("sturm_shift_circular_frequency", "fixed", modal_doc_angular(c.sigma), false, None),
                "formation_term": modal_doc_optional("sturm_formation_term", c.formation_term, "not_formed"),
                "formation_separation_ratio": modal_doc_optional("sturm_formation_separation_ratio", c.formation_separation_ratio, "no_neighbour_or_not_formed"),
                "factor_term": factored("sturm_factor_term", f.map(|f| f.factor_term)),
                "backward_error": factored("sturm_backward_error", f.map(|f| f.backward_error)),
                "inverse_norm_estimate": factored("sturm_inverse_norm_estimate", f.map(|f| f.inverse_norm_estimate)),
                "conditioning_product": factored("sturm_conditioning_product", f.map(|f| f.conditioning_product)),
                "separation_ratio": match f {
                    None => modal_doc_optional("sturm_separation_ratio", None, "not_factored"),
                    Some(f) => modal_doc_optional("sturm_separation_ratio", f.separation_ratio, "no_neighbour"),
                },
                "growth": factored("sturm_growth_factor", f.map(|f| f.growth)),
                "max_abs_lower_factor": factored("sturm_max_abs_lower_factor", f.map(|f| f.max_abs_lower_factor)),
                "max_abs_pivot": factored("sturm_max_abs_pivot", f.map(|f| f.max_abs_pivot)),
                "max_abs_scaled_diagonal": factored("sturm_max_abs_scaled_diagonal", f.map(|f| f.max_abs_scaled_diagonal))}))
        })
        .collect();
    tables.insert("sturm_candidates".into(), modal_doc_table(candidates));

    // The weight-basis identity, the nonlinear states, f_e, disclosures and
    // the non-blocking diagnostics.
    let identity = outcome
        .identity
        .discrepancies
        .iter()
        .enumerate()
        .map(|(i, d)| {
            modal_doc_row(format!("identity:{i}"), i, json!({"subject": d.subject, "axis": DIRECTIONS[d.axis], "comparison": d.kind,
                "relative": modal_doc_fixed("identity_relative_discrepancy", d.relative)}))
        })
        .collect();
    tables.insert("identity".into(), modal_doc_table(identity));
    let supports = outcome
        .nonlinear_state
        .iter()
        .flat_map(|s| s.supports.iter())
        .enumerate()
        .map(|(i, s)| {
            let treatment = match s.treatment {
                super::state::ModalSupportTreatment::Restrained => "restrained",
                super::state::ModalSupportTreatment::AlreadyRestrained => "already_restrained",
                super::state::ModalSupportTreatment::Free => "free",
                super::state::ModalSupportTreatment::FreeFrozenFriction => "free_frozen_friction",
            };
            modal_doc_row(format!("support:{}", s.support_id), i, json!({"support_id": s.support_id, "behavior": s.behavior,
                "dof_ref": modal_doc_dof_id(ids, s.global_dof), "state": s.state.as_str(), "treatment": treatment}))
        })
        .collect();
    tables.insert("supports".into(), modal_doc_table(supports));
    let discretization = outcome
        .discretization
        .iter()
        .enumerate()
        .map(|(i, m)| {
            modal_doc_row(format!("member:{}", m.pipe), i, json!({"member_ref": m.pipe, "length": modal_doc_fixed("member_length", m.length),
                "fixed_fixed_frequency": modal_doc_fixed("member_fixed_fixed_frequency", m.fixed_fixed_frequency),
                "ratio_to_highest_reported": modal_doc_optional("member_frequency_ratio", m.ratio_to_highest_reported, "no_reported_mode")}))
        })
        .collect();
    tables.insert("discretization".into(), modal_doc_table(discretization));
    let disclosures = outcome
        .disclosures
        .iter()
        .map(|text| (text, "modal"))
        .chain(outcome.identity.disclosures.iter().map(|text| (text, "weight_basis")))
        .enumerate()
        .map(|(i, (text, source))| modal_doc_row(format!("disclosure:{i}"), i, json!({"text": text, "source": source})))
        .collect();
    tables.insert("disclosures".into(), modal_doc_table(disclosures));
    let diagnostics = outcome
        .diagnostics
        .iter()
        .map(|d| (d, "modal"))
        .chain(outcome.nonlinear_state.iter().flat_map(|s| s.static_diagnostics.iter()).map(|d| (d, "state_case_solve")))
        .enumerate()
        .map(|(i, (d, source))| modal_doc_diagnostic_row("diagnostic", i, d, source))
        .collect();
    tables.insert("diagnostics".into(), modal_doc_table(diagnostics));
    doc.insert("tables".into(), Value::Object(tables));
    Value::Object(doc)
}
