//! The Rust reader of the modal result family `openpipestress.modal_result`
//! 0.1.0 (DEL-04-09 Design §5.3, increment I3a), under the semantic table
//! `openpipestress.result_semantics/modal-1`. It shares no code with the
//! producer (`product_physics::modal::document`) or with the Python reader
//! (`tests/verifiers/modal_evidence.py`); the relations and their rounding
//! allowances are the Design's, implemented here independently.
//!
//! It re-derives the governed relations **without an eigen solve**, from the
//! document's own authoritative payload (the formed global mass and the
//! published shapes):
//! - `mass`: T_k = R_kᵀ M R_k and the free-DOF mass from the mass entries,
//!   Bessel's Σ m_eff,ik ≤ T_k, a non-negative mass diagonal;
//! - `normalization`: each mode's case and sign (its reference component
//!   exactly +1, the case's components at most 1, case (b)'s translations
//!   at most τ_t·ℓ), restrained components exactly 0, ℓ from the nodes;
//! - `participation`: m_i = φᵀM_ffφ, φᵀp_k with p_k = (M R_k)_f, Γ, m_eff,
//!   Γ²m = m_eff, the fractions against T_k and their cumulative sums;
//! - `counts`: the reported set against the bound and the computed
//!   frequencies, its cluster extension and endpoint, f = ω/2π and T = 1/f;
//! - `clusters`: the chained ids, the cluster rows, the δ_c rule from ω and
//!   the cluster error estimates;
//! - `sturm`: the count against the reported set, the placement in its gap,
//!   each candidate's decision against its premises and the parameters;
//! - `evidence`: e_λ = η·κ_λ, the census and the identity discrepancies;
//! - `standing`: the Sensitive standing re-derived from η, τ_r and τ_o;
//! - `units`: every value's unit against modal-1.
//!
//! Each standing is passed, failed (with the first difference), scoped
//! unsupported (with its scope) or not checked. A withheld authoritative
//! payload is scoped unsupported: nothing is reconstructed from a digest.
//! A relation that the published binary64 values cannot decide (a cluster
//! boundary within ω's rounding) is scoped unsupported, never guessed.
//!
//! **Admission.** Public engineering-result admission is disabled: the
//! verdict states it for every document, with the open conditions.
//!
//! **Placement** (Design §5.3): a leaf crate of its own, as the hanger-sizing
//! family's reader is. The modal family is not the `results.v0.3` envelope
//! that `result_export` reads, and the reader is reached from no retained
//! root, so it is kept out of that crate's priced surface. It depends on no
//! product crate.
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_units::{canonical_unit, catalog, Dimension};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

pub const MODAL_RESULT_FAMILY: &str = "openpipestress.modal_result";
pub const MODAL_RESULT_SCHEMA_VERSION: &str = "0.1.0";
pub const MODAL_RESULT_SEMANTICS_ID: &str = "openpipestress.result_semantics/modal-1";
/// The SHA-256 of modal-1's canonical text (`fixtures/results/semantic_contract_modal_1.json`).
pub const MODAL_RESULT_SEMANTICS_SHA256: &str = "22884200060d6bae480ad922d799131486ac439d77ecd8b03516c310305c8e82";
pub const MODAL_RESULT_PROFILE_ID: &str = "product_preview_modal_v1";
pub const MODAL_RESULT_CANONICAL_PROFILE: &str = "openpipestress_jcs_ijson_v1";
pub const MODAL_RESULT_LIMITATIONS: [&str; 2] = [
    "Small-displacement, linear-elastic Euler-Bernoulli frame modal preview about the unloaded configuration, with any nonlinear supports at the stated state of a named solved case, with consistent mass on straight members and lumped rigid-body mass on realized arcs and component bodies; numerical integrity does not establish physical correctness.",
    "The integrity checks are numerical estimates under stated premises, not mathematical bounds: each mode's residual is evaluated against a double-double re-formation of the stiffness and mass, its eigenvalue error is a first-order estimate, and the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues. They do not bound modelling, discretization or input error.",
];
/// Why public engineering-result admission stays disabled (Design §5.3).
pub const MODAL_RESULT_ADMISSION_CONDITIONS: [&str; 6] = [
    "production integrity parameters are not pinned (O-6, D-TOL): every document is formed with a labelled test-local set",
    "the unit catalog has no family for the mass matrix's translation-rotation coupling (kg*m/rad), and the equilibrated-basis evidence has no catalog dimension",
    "the dense work budget is unmeasured and no work guard is applied (O-4)",
    "no native, worker or custody path forms, stores or transports the document",
    "no published piping benchmark is compared (FR-SOL-016 partial; O-3 held)",
    "the reader's re-derivation stops at the document's payload: the stiffness, the re-formation and the eigen solve are not re-derived",
];

const U: f64 = 1.1102230246251565e-16;
/// The reader rounding allowance multiple of every tolerance (Design §5.3).
const ALLOW: f64 = 16.0;
const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
const COMPONENTS: [&str; 6] = ["ux", "uy", "uz", "rx", "ry", "rz"];
const DIRECTIONS: [&str; 3] = ["x", "y", "z"];
const CASES: [&str; 2] = ["unit_max_translation", "unit_max_rotation"];
const PAIR_CLASSES: [&str; 3] = ["translation_translation", "translation_rotation", "rotation_rotation"];
const TABLES: [&str; 15] = [
    "nodes", "dofs", "mass_matrix", "mass_totals", "modes", "shapes", "participation", "clusters", "computed_modes",
    "sturm_candidates", "identity", "supports", "discretization", "disclosures", "diagnostics",
];
const WITHHOLDABLE: [&str; 2] = ["mass_matrix", "shapes"];
const OPTION_MEMBERS: [&str; 7] = [
    "mode_count", "frequency_cutoff", "state_case", "modulus_basis_ref", "modulus_basis_temperature",
    "weight_to_mass_acceleration", "include_dynamic_only_restraints",
];
const PARAMETER_FIELDS: [&str; 18] = [
    "infinite_mu_relative_threshold", "mass_pivot_relative_threshold", "resolution_floor_constant", "resolution_floor_multiple",
    "formed_backward_error_multiple", "screen_error_estimate_threshold", "max_ql_iterations_per_eigenvalue", "residual_threshold",
    "orthogonality_threshold", "cluster_tolerance", "no_translation_threshold", "sturm_conditioning_multiple",
    "sturm_separation_multiple", "sturm_growth_limit", "sturm_max_candidates", "sturm_gap_log_positions",
    "sturm_whole_spectrum_offsets", "identity_tolerance",
];
const DECISIONS: [&str; 8] = [
    "accepted", "outside_gap", "out_of_range", "separation_out_of_range", "factorization_rejected", "unseparated",
    "growth_limit", "ill_conditioned",
];

/// How a unit rule is stated in modal-1.
#[derive(Debug, Clone, Copy)]
enum Rule {
    /// A catalog unit: (id, symbol, dimension).
    Catalog(&'static str, &'static str, &'static str),
    /// No catalog family: the exponents (L, M, T, Θ, A).
    Absent([i64; 5]),
    /// The kernel's equilibrated basis: no catalog dimension.
    Equilibrated,
}

const R_ONE: Rule = Rule::Catalog("unit:one", "1", "dimensionless");
const R_M: Rule = Rule::Catalog("unit:meter", "m", "length");
const R_KG: Rule = Rule::Catalog("unit:kilogram", "kg", "mass");
const R_HZ: Rule = Rule::Catalog("unit:hertz", "Hz", "frequency");
const R_RAD_S: Rule = Rule::Catalog("unit:radian_per_second", "rad/s", "angular_frequency");
const R_S: Rule = Rule::Catalog("unit:second", "s", "time");
const R_ACC: Rule = Rule::Catalog("unit:meter_per_second_squared", "m/s^2", "acceleration");
const R_RAD_M: Rule = Rule::Catalog("unit:radian_per_meter", "rad/m", "rotation_per_length");
const R_M_RAD: Rule = Rule::Catalog("unit:meter_per_radian", "m/rad", "length_per_rotation");
const R_KGM2: Rule = Rule::Catalog("unit:kilogram_square_meter_per_square_radian", "kg*m^2/rad^2", "rotational_generalized_mass");
const R_EQ: Rule = Rule::Equilibrated;

/// modal-1, as this reader states it: (kind, table, selector, rules).
const KINDS: &[(&str, &str, &str, &[(&str, Rule)])] = &[
    ("node_coordinate", "nodes", "fixed", &[("fixed", R_M)]),
    ("mass_matrix_entry", "mass_matrix", "dof_pair_class", &[
        ("translation_translation", R_KG), ("translation_rotation", Rule::Absent([1, 1, 0, 0, -1])), ("rotation_rotation", R_KGM2)]),
    ("modal_total_mass", "mass_totals", "fixed", &[("fixed", R_KG)]),
    ("modal_free_dof_mass", "mass_totals", "fixed", &[("fixed", R_KG)]),
    ("modal_circular_frequency", "modes", "fixed", &[("fixed", R_RAD_S)]),
    ("modal_frequency", "modes", "fixed", &[("fixed", R_HZ)]),
    ("modal_period", "modes", "fixed", &[("fixed", R_S)]),
    ("modal_generalized_mass", "modes", "normalization_case", &[("unit_max_translation", R_KG), ("unit_max_rotation", R_KGM2)]),
    ("modal_cluster_error_estimate", "modes", "fixed", &[("fixed", R_ONE)]),
    ("mode_shape_translation", "shapes", "normalization_case", &[("unit_max_translation", R_ONE), ("unit_max_rotation", R_M_RAD)]),
    ("mode_shape_rotation", "shapes", "normalization_case", &[("unit_max_translation", R_RAD_M), ("unit_max_rotation", R_ONE)]),
    ("modal_participation_factor", "participation", "normalization_case", &[("unit_max_translation", R_ONE), ("unit_max_rotation", R_RAD_M)]),
    ("modal_effective_mass", "participation", "fixed", &[("fixed", R_KG)]),
    ("modal_effective_mass_fraction", "participation", "fixed", &[("fixed", R_ONE)]),
    ("modal_cumulative_effective_mass_fraction", "participation", "fixed", &[("fixed", R_ONE)]),
    ("cluster_error_estimate", "clusters", "fixed", &[("fixed", R_ONE)]),
    ("computed_circular_frequency", "computed_modes", "fixed", &[("fixed", R_RAD_S)]),
    ("computed_frequency", "computed_modes", "fixed", &[("fixed", R_HZ)]),
    ("residual_backward_error", "computed_modes", "fixed", &[("fixed", R_ONE)]),
    ("eigenvalue_condition_estimate", "computed_modes", "fixed", &[("fixed", R_ONE)]),
    ("eigenvalue_error_estimate", "computed_modes", "fixed", &[("fixed", R_ONE)]),
    ("sturm_shift_circular_frequency", "sturm_candidates", "fixed", &[("fixed", R_RAD_S)]),
    ("sturm_formation_term", "sturm_candidates", "fixed", &[("fixed", R_EQ)]),
    ("sturm_formation_separation_ratio", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_factor_term", "sturm_candidates", "fixed", &[("fixed", R_EQ)]),
    ("sturm_backward_error", "sturm_candidates", "fixed", &[("fixed", R_EQ)]),
    ("sturm_inverse_norm_estimate", "sturm_candidates", "fixed", &[("fixed", R_EQ)]),
    ("sturm_conditioning_product", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_separation_ratio", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_growth_factor", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_max_abs_lower_factor", "sturm_candidates", "fixed", &[("fixed", R_EQ)]),
    ("sturm_max_abs_pivot", "sturm_candidates", "fixed", &[("fixed", R_EQ)]),
    ("sturm_max_abs_scaled_diagonal", "sturm_candidates", "fixed", &[("fixed", R_EQ)]),
    ("identity_relative_discrepancy", "identity", "fixed", &[("fixed", R_ONE)]),
    ("member_length", "discretization", "fixed", &[("fixed", R_M)]),
    ("member_fixed_fixed_frequency", "discretization", "fixed", &[("fixed", R_HZ)]),
    ("member_frequency_ratio", "discretization", "fixed", &[("fixed", R_ONE)]),
    ("basis_no_translation_threshold", "header", "fixed", &[("fixed", R_ONE)]),
    ("basis_bounding_box_diagonal", "header", "fixed", &[("fixed", R_M)]),
    ("basis_mass_center", "header", "fixed", &[("fixed", R_M)]),
    ("request_frequency_cutoff", "header", "fixed", &[("fixed", R_HZ)]),
    ("request_weight_to_mass_acceleration", "header", "fixed", &[("fixed", R_ACC)]),
    ("identity_acceleration", "header", "fixed", &[("fixed", R_ACC)]),
    ("placement_circular_frequency", "header", "fixed", &[("fixed", R_RAD_S)]),
    ("sturm_premise_parameter", "header", "fixed", &[("fixed", R_ONE)]),
    ("orthogonality_max_ratio", "header", "fixed", &[("fixed", R_ONE)]),
    ("formation_difference", "header", "fixed", &[("fixed", R_EQ)]),
    ("reformed_norm", "header", "fixed", &[("fixed", R_EQ)]),
    ("section_derivation_difference", "header", "fixed", &[("fixed", R_ONE)]),
];

fn mr_rule_json(rule: Rule) -> Value {
    match rule {
        Rule::Catalog(id, symbol, dimension) => {
            json!({"unit_status": "catalog", "unit": id, "symbol": symbol, "dimension": dimension})
        }
        Rule::Absent(e) => json!({"unit_status": "catalog_family_absent", "unit": null, "symbol": null, "dimension": null,
            "dimension_exponents": {"length": e[0], "mass": e[1], "time": e[2], "temperature": e[3], "angle": e[4]}}),
        Rule::Equilibrated => json!({"unit_status": "equilibrated_basis", "unit": null, "symbol": null, "dimension": null}),
    }
}

/// modal-1 as this reader states it; its canonical SHA-256 must be
/// [`MODAL_RESULT_SEMANTICS_SHA256`] and the committed table's.
pub fn modal_reader_semantic_table() -> Value {
    let kinds: Vec<Value> = KINDS
        .iter()
        .map(|(kind, table, selector, rules)| {
            let units: Map<String, Value> = rules.iter().map(|(key, rule)| ((*key).to_string(), mr_rule_json(*rule))).collect();
            json!({"kind": kind, "table": table, "selector": selector, "units": units})
        })
        .collect();
    json!({"id": MODAL_RESULT_SEMANTICS_ID, "document_kind": MODAL_RESULT_FAMILY, "schema_version": MODAL_RESULT_SCHEMA_VERSION,
        "canonical_profile": MODAL_RESULT_CANONICAL_PROFILE, "kinds": kinds})
}

/// Each modal-1 catalog rule against the unit catalog: the id exists, is
/// the canonical unit of its stated dimension and has the stated symbol.
pub fn modal_reader_catalog_differences() -> Vec<String> {
    let mut out = Vec::new();
    for (kind, _, _, rules) in KINDS {
        for (key, rule) in *rules {
            let Rule::Catalog(id, symbol, dimension) = rule else { continue };
            let found = catalog().iter().find(|unit| unit.catalog_id() == *id).map(|unit| unit.definition());
            let Some(definition) = found else {
                out.push(format!("{kind}/{key}: {id} is not a catalog unit"));
                continue;
            };
            let canonical = Dimension::from_schema_value(dimension).ok().and_then(canonical_unit).map(|u| u.catalog_id());
            if definition.symbol != *symbol || definition.dimension.as_str() != *dimension || canonical != Some(*id) {
                out.push(format!("{kind}/{key}: {id} is not the canonical {dimension} unit with symbol {symbol}"));
            }
        }
    }
    out
}

/// SHA-256 (lowercase hex) of the checked canonical text, or `None`.
pub fn modal_reader_sha256(value: &Value) -> Option<String> {
    let text = serde_json::to_string(value).ok()?;
    let canonical = canonical_json_checked_v1_text(&text).ok()?;
    Some(Sha256::digest(canonical.as_bytes()).iter().map(|b| format!("{b:02x}")).collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalReadStatus {
    Passed,
    Failed,
    ScopedUnsupported,
    NotChecked,
}

impl ModalReadStatus {
    pub fn modal_status_token(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::ScopedUnsupported => "scoped_unsupported",
            Self::NotChecked => "not_checked",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModalReadStanding {
    pub status: ModalReadStatus,
    /// The first difference, for a failed standing.
    pub difference: Option<String>,
    /// What is out of scope, for a scoped standing.
    pub scope: Vec<String>,
}

impl ModalReadStanding {
    fn mr_unchecked() -> Self {
        Self { status: ModalReadStatus::NotChecked, difference: None, scope: Vec::new() }
    }
    fn mr_of(result: Result<Vec<String>, String>) -> Self {
        match result {
            Ok(scope) if scope.is_empty() => Self { status: ModalReadStatus::Passed, difference: None, scope },
            Ok(scope) => Self { status: ModalReadStatus::ScopedUnsupported, difference: None, scope },
            Err(difference) => Self { status: ModalReadStatus::Failed, difference: Some(difference), scope: Vec::new() },
        }
    }
    fn mr_json(&self) -> Value {
        json!({"status": self.status.modal_status_token(), "difference": self.difference, "scope": self.scope})
    }
}

/// The relation standings, in the verdict's order.
pub const MODAL_RELATIONS: [&str; 9] =
    ["mass", "normalization", "participation", "counts", "clusters", "sturm", "evidence", "standing", "units"];

#[derive(Debug, Clone, PartialEq)]
pub struct ModalReadVerdict {
    pub closure: ModalReadStanding,
    pub binding: ModalReadStanding,
    /// The supplied inputs `binding` could not check.
    pub binding_not_checked: Vec<String>,
    /// In [`MODAL_RELATIONS`] order.
    pub relations: Vec<ModalReadStanding>,
    /// `checks_passed`, `sensitive` or `not_assessed`.
    pub health: &'static str,
    /// The blocking codes of a refused document, in order.
    pub refusal_codes: Vec<String>,
    /// `consistent`, `consistent_with_scoped_unsupported`,
    /// `refusal_consistent` or `rejected`.
    pub decision: &'static str,
}

impl ModalReadVerdict {
    pub fn modal_verdict_json(&self) -> Value {
        let mut relations = Map::new();
        for (name, standing) in MODAL_RELATIONS.iter().zip(&self.relations) {
            relations.insert((*name).to_string(), standing.mr_json());
        }
        let mut binding = self.binding.mr_json();
        binding["not_checked"] = json!(self.binding_not_checked);
        json!({"family": MODAL_RESULT_FAMILY, "closure": self.closure.mr_json(), "binding": binding, "relations": relations,
            "health": self.health, "sensitive": self.health == "sensitive", "refusal_codes": self.refusal_codes,
            "decision": self.decision, "public_admission": {"status": "disabled", "conditions": MODAL_RESULT_ADMISSION_CONDITIONS}})
    }
}

/// What the caller supplies beside the document.
#[derive(Debug, Clone, Copy, Default)]
pub struct ModalSupplied<'a> {
    /// The modal request the document claims to bind.
    pub request: Option<&'a Value>,
}

type R<T> = Result<T, String>;

fn mr_at(path: &str, key: &str) -> String {
    format!("{path}/{key}")
}

fn mr_object<'v>(value: &'v Value, path: &str, keys: &[&str]) -> R<&'v Map<String, Value>> {
    let map = value.as_object().ok_or_else(|| format!("{path}: not an object"))?;
    for key in keys {
        if !map.contains_key(*key) {
            return Err(format!("{path}: missing `{key}`"));
        }
    }
    if let Some(extra) = map.keys().find(|k| !keys.contains(&k.as_str())) {
        return Err(format!("{path}: undeclared `{extra}`"));
    }
    Ok(map)
}

fn mr_text<'v>(value: &'v Value, path: &str) -> R<&'v str> {
    value.as_str().ok_or_else(|| format!("{path}: not a string"))
}

fn mr_count(value: &Value, path: &str) -> R<usize> {
    value.as_u64().map(|v| v as usize).ok_or_else(|| format!("{path}: not a non-negative integer"))
}

fn mr_flag(value: &Value, path: &str) -> R<bool> {
    value.as_bool().ok_or_else(|| format!("{path}: not a boolean"))
}

fn mr_list<'v>(value: &'v Value, path: &str) -> R<&'v [Value]> {
    value.as_array().map(Vec::as_slice).ok_or_else(|| format!("{path}: not an array"))
}

fn mr_number(value: &Value, path: &str) -> R<f64> {
    value.as_f64().filter(|v| v.is_finite()).ok_or_else(|| format!("{path}: not a finite number"))
}

fn mr_const(value: &Value, path: &str, expected: &str) -> R<()> {
    if value.as_str() == Some(expected) {
        Ok(())
    } else {
        Err(format!("{path}: expected `{expected}`"))
    }
}

fn mr_one_of<'v>(value: &'v Value, path: &str, allowed: &[&str]) -> R<&'v str> {
    let text = mr_text(value, path)?;
    if allowed.contains(&text) {
        Ok(text)
    } else {
        Err(format!("{path}: `{text}` is not one of {allowed:?}"))
    }
}

fn mr_hex(value: &Value, path: &str) -> R<()> {
    let text = mr_text(value, path)?;
    if text.len() == 64 && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        Ok(())
    } else {
        Err(format!("{path}: not a lowercase SHA-256"))
    }
}

/// One parsed value object.
#[derive(Debug, Clone)]
struct Vo {
    kind: String,
    value: Option<f64>,
    unit: Option<String>,
    unit_status: String,
    below_normal: bool,
    exponents: Option<[i64; 5]>,
    undefined: Option<String>,
}

fn mr_vo(value: &Value, path: &str, kinds: &[&str]) -> R<Vo> {
    let map = value.as_object().ok_or_else(|| format!("{path}: not a value object"))?;
    let status = mr_text(map.get("unit_status").unwrap_or(&Value::Null), &mr_at(path, "unit_status"))?.to_string();
    let mut keys = vec!["kind", "value", "unit", "unit_status", "below_normal", "undefined_reason"];
    if status == "catalog_family_absent" {
        keys.push("dimension_exponents");
    }
    mr_object(value, path, &keys)?;
    let kind = mr_text(&map["kind"], &mr_at(path, "kind"))?.to_string();
    if !kinds.contains(&kind.as_str()) {
        return Err(format!("{path}: kind `{kind}` is not {kinds:?}"));
    }
    let undefined = match &map["undefined_reason"] {
        Value::Null => None,
        other => Some(mr_text(other, &mr_at(path, "undefined_reason"))?.to_string()),
    };
    let number = match (&map["value"], &undefined) {
        (Value::Null, Some(_)) => None,
        (Value::Null, None) => return Err(format!("{path}: a null value states its undefined_reason")),
        (_, Some(_)) => return Err(format!("{path}: an undefined value is null")),
        (v, None) => Some(mr_number(v, &mr_at(path, "value"))?),
    };
    let below_normal = mr_flag(&map["below_normal"], &mr_at(path, "below_normal"))?;
    if below_normal && number != Some(0.0) {
        return Err(format!("{path}: a below-normal value is published as 0"));
    }
    let unit = match &map["unit"] {
        Value::Null => None,
        other => Some(mr_text(other, &mr_at(path, "unit"))?.to_string()),
    };
    let exponents = if status == "catalog_family_absent" {
        let e = mr_object(&map["dimension_exponents"], &mr_at(path, "dimension_exponents"), &["length", "mass", "time", "temperature", "angle"])?;
        let read = |k: &str| e[k].as_i64().ok_or_else(|| format!("{path}/dimension_exponents/{k}: not an integer"));
        Some([read("length")?, read("mass")?, read("time")?, read("temperature")?, read("angle")?])
    } else {
        None
    };
    Ok(Vo { kind, value: number, unit, unit_status: status, below_normal, exponents, undefined })
}

/// A published table's rows, or `None` when withheld.
struct Table<'v> {
    rows: Option<&'v [Value]>,
}

fn mr_rule_of(kind: &str, key: &str) -> Option<Rule> {
    KINDS.iter().find(|(k, ..)| *k == kind).and_then(|(.., rules)| rules.iter().find(|(k, _)| *k == key)).map(|(_, r)| *r)
}

/// Units: one value object against its rule (`key` the selector key).
fn mr_unit_check(vo: &Vo, key: &str, path: &str, scope: &mut Vec<String>) -> R<()> {
    let rule = mr_rule_of(&vo.kind, key).ok_or_else(|| format!("{path}: modal-1 has no `{key}` rule for `{}`", vo.kind))?;
    match rule {
        Rule::Catalog(id, ..) => {
            if vo.unit_status != "catalog" || vo.unit.as_deref() != Some(id) {
                return Err(format!("{path}: unit {:?} ({}) is not modal-1's {id}", vo.unit, vo.unit_status));
            }
        }
        Rule::Absent(e) => {
            if vo.unit_status != "catalog_family_absent" || vo.unit.is_some() || vo.exponents != Some(e) {
                return Err(format!("{path}: modal-1 states no catalog family here, with exponents {e:?}"));
            }
            let note = format!("catalog_family_absent: `{}` {key} (exponents {e:?})", vo.kind);
            if !scope.contains(&note) {
                scope.push(note);
            }
        }
        Rule::Equilibrated => {
            if vo.unit_status != "equilibrated_basis" || vo.unit.is_some() {
                return Err(format!("{path}: modal-1 states the equilibrated basis here"));
            }
            let note = format!("equilibrated_basis: `{}`", vo.kind);
            if !scope.contains(&note) {
                scope.push(note);
            }
        }
    }
    Ok(())
}

/// Reads a document text.
pub fn read_modal_result_text(text: &str, supplied: ModalSupplied<'_>) -> ModalReadVerdict {
    match serde_json::from_str::<Value>(text) {
        Ok(doc) => read_modal_result(&doc, supplied),
        Err(error) => mr_closure_failed(format!("document: not JSON ({error})")),
    }
}

fn mr_closure_failed(difference: String) -> ModalReadVerdict {
    ModalReadVerdict {
        closure: ModalReadStanding { status: ModalReadStatus::Failed, difference: Some(difference), scope: Vec::new() },
        binding: ModalReadStanding::mr_unchecked(),
        binding_not_checked: Vec::new(),
        relations: MODAL_RELATIONS.iter().map(|_| ModalReadStanding::mr_unchecked()).collect(),
        health: "not_assessed",
        refusal_codes: Vec::new(),
        decision: "rejected",
    }
}

const SOLVED_KEYS: [&str; 18] = [
    "document_kind", "schema_version", "semantic_contract", "profile", "canonical_profile", "admission", "run", "binding",
    "status", "basis", "extraction", "census", "standing", "sturm", "orthogonality", "formation", "guard", "tables",
];
const REFUSED_KEYS: [&str; 10] = [
    "document_kind", "schema_version", "semantic_contract", "profile", "canonical_profile", "admission", "run", "binding",
    "status", "refusals",
];

/// Reads a parsed document (module docs).
pub fn read_modal_result(doc: &Value, supplied: ModalSupplied<'_>) -> ModalReadVerdict {
    let status = doc.get("status").and_then(Value::as_str).unwrap_or("");
    if status == "refused" {
        return mr_read_refused(doc, supplied);
    }
    let parsed = match mr_closure(doc) {
        Ok(parsed) => parsed,
        Err(difference) => return mr_closure_failed(difference),
    };
    let (binding, not_checked) = mr_binding(doc, supplied);
    let relations = vec![
        ModalReadStanding::mr_of(mr_mass(&parsed)),
        ModalReadStanding::mr_of(mr_normalization(&parsed)),
        ModalReadStanding::mr_of(mr_participation(&parsed)),
        ModalReadStanding::mr_of(mr_counts(&parsed)),
        ModalReadStanding::mr_of(mr_clusters(&parsed)),
        ModalReadStanding::mr_of(mr_sturm(&parsed)),
        ModalReadStanding::mr_of(mr_evidence(&parsed)),
        ModalReadStanding::mr_of(mr_standing(&parsed)),
        ModalReadStanding::mr_of(mr_units(&parsed)),
    ];
    let failed = binding.status == ModalReadStatus::Failed || relations.iter().any(|r| r.status == ModalReadStatus::Failed);
    let scoped = relations.iter().any(|r| r.status == ModalReadStatus::ScopedUnsupported);
    let decision = if failed {
        "rejected"
    } else if scoped {
        "consistent_with_scoped_unsupported"
    } else {
        "consistent"
    };
    let health = if failed { "not_assessed" } else if parsed.sensitive { "sensitive" } else { "checks_passed" };
    ModalReadVerdict {
        closure: ModalReadStanding { status: ModalReadStatus::Passed, difference: None, scope: Vec::new() },
        binding,
        binding_not_checked: not_checked,
        relations,
        health,
        refusal_codes: Vec::new(),
        decision,
    }
}

/// The identity members both statuses share (closure).
fn mr_identity(doc: &Value) -> R<()> {
    mr_const(&doc["document_kind"], "/document_kind", MODAL_RESULT_FAMILY)?;
    mr_const(&doc["schema_version"], "/schema_version", MODAL_RESULT_SCHEMA_VERSION)?;
    let sc = mr_object(&doc["semantic_contract"], "/semantic_contract", &["id", "sha256"])?;
    mr_const(&sc["id"], "/semantic_contract/id", MODAL_RESULT_SEMANTICS_ID)?;
    mr_const(&sc["sha256"], "/semantic_contract/sha256", MODAL_RESULT_SEMANTICS_SHA256)?;
    let profile = mr_object(&doc["profile"], "/profile", &["profile_id", "limitations"])?;
    mr_const(&profile["profile_id"], "/profile/profile_id", MODAL_RESULT_PROFILE_ID)?;
    let limitations = mr_list(&profile["limitations"], "/profile/limitations")?;
    if limitations.len() != 2 || limitations.iter().zip(MODAL_RESULT_LIMITATIONS).any(|(a, b)| a.as_str() != Some(b)) {
        return Err("/profile/limitations: not the profile's two fixed limitations".into());
    }
    mr_const(&doc["canonical_profile"], "/canonical_profile", MODAL_RESULT_CANONICAL_PROFILE)?;
    let admission = mr_object(&doc["admission"], "/admission", &["status", "public_engineering_result", "parameter_standing", "parameter_label"])?;
    mr_const(&admission["status"], "/admission/status", "experimental")?;
    if admission["public_engineering_result"] != Value::Bool(false) {
        return Err("/admission/public_engineering_result: modal_result 0.1.0 admits no public engineering result".into());
    }
    mr_const(&admission["parameter_standing"], "/admission/parameter_standing", "test_local")?;
    if mr_text(&admission["parameter_label"], "/admission/parameter_label")?.is_empty() {
        return Err("/admission/parameter_label: empty".into());
    }
    let run = mr_object(&doc["run"], "/run", &["run_id", "mode_id_scope", "solver_path"])?;
    mr_text(&run["run_id"], "/run/run_id")?;
    mr_const(&run["mode_id_scope"], "/run/mode_id_scope", "run_local")?;
    mr_const(&run["solver_path"], "/run/solver_path", "dense_scrutiny")?;
    mr_binding_closure(&doc["binding"])
}

fn mr_binding_closure(binding: &Value) -> R<()> {
    let b = mr_object(binding, "/binding", &[
        "request_sha256", "model_sha256", "materials_sha256", "materials_member", "options", "undeclared_members", "state_case",
        "integrity_parameters_sha256", "integrity_parameters",
    ])?;
    mr_hex(&b["request_sha256"], "/binding/request_sha256")?;
    if !b["model_sha256"].is_null() {
        mr_hex(&b["model_sha256"], "/binding/model_sha256")?;
    }
    mr_hex(&b["materials_sha256"], "/binding/materials_sha256")?;
    mr_one_of(&b["materials_member"], "/binding/materials_member", &["authored", "absent"])?;
    let options = b["options"].as_object().ok_or("/binding/options: not an object")?;
    if let Some(extra) = options.keys().find(|k| !OPTION_MEMBERS.contains(&k.as_str())) {
        return Err(format!("/binding/options: `{extra}` is not a modal request option"));
    }
    for (i, member) in mr_list(&b["undeclared_members"], "/binding/undeclared_members")?.iter().enumerate() {
        mr_text(member, &format!("/binding/undeclared_members/{i}"))?;
    }
    if !b["state_case"].is_null() {
        let s = mr_object(&b["state_case"], "/binding/state_case", &["case_id", "case_index", "case_sha256"])?;
        mr_text(&s["case_id"], "/binding/state_case/case_id")?;
        mr_count(&s["case_index"], "/binding/state_case/case_index")?;
        mr_hex(&s["case_sha256"], "/binding/state_case/case_sha256")?;
    }
    mr_hex(&b["integrity_parameters_sha256"], "/binding/integrity_parameters_sha256")?;
    let p = mr_object(&b["integrity_parameters"], "/binding/integrity_parameters", &PARAMETER_FIELDS)?;
    for field in PARAMETER_FIELDS {
        let path = format!("/binding/integrity_parameters/{field}");
        match field {
            "max_ql_iterations_per_eigenvalue" | "sturm_max_candidates" => {
                mr_count(&p[field], &path)?;
            }
            "sturm_gap_log_positions" | "sturm_whole_spectrum_offsets" => {
                for (i, v) in mr_list(&p[field], &path)?.iter().enumerate() {
                    mr_number(v, &format!("{path}/{i}"))?;
                }
            }
            "identity_tolerance" => {
                let t = mr_object(&p[field], &path, &["relative", "absolute"])?;
                mr_number(&t["relative"], &format!("{path}/relative"))?;
                mr_number(&t["absolute"], &format!("{path}/absolute"))?;
            }
            _ => {
                mr_number(&p[field], &path)?;
            }
        }
    }
    Ok(())
}

/// A published or withheld table, its rows' checksums, digest, ordinals and
/// globally unique ids (closure).
fn mr_table<'v>(tables: &'v Value, name: &str, ids: &mut HashSet<String>) -> R<Table<'v>> {
    let path = format!("/tables/{name}");
    let table = tables.get(name).ok_or_else(|| format!("{path}: missing"))?;
    let status = mr_text(table.get("status").unwrap_or(&Value::Null), &mr_at(&path, "status"))?;
    if status == "withheld" {
        if !WITHHOLDABLE.contains(&name) {
            return Err(format!("{path}: only the authoritative payload tables {WITHHOLDABLE:?} may be withheld"));
        }
        let t = mr_object(table, &path, &["status", "reason"])?;
        mr_text(&t["reason"], &mr_at(&path, "reason"))?;
        return Ok(Table { rows: None });
    }
    let t = mr_object(table, &path, &["status", "row_count", "digest", "rows"])?;
    mr_const(&t["status"], &mr_at(&path, "status"), "published")?;
    let rows = mr_list(&t["rows"], &mr_at(&path, "rows"))?;
    if mr_count(&t["row_count"], &mr_at(&path, "row_count"))? != rows.len() {
        return Err(format!("{path}/row_count: not the number of rows"));
    }
    let mut checksums = Vec::with_capacity(rows.len());
    for (i, row) in rows.iter().enumerate() {
        let rpath = format!("{path}/rows/{i}");
        let map = row.as_object().ok_or_else(|| format!("{rpath}: not an object"))?;
        let id = mr_text(map.get("row_id").unwrap_or(&Value::Null), &mr_at(&rpath, "row_id"))?;
        if mr_count(map.get("ordinal").unwrap_or(&Value::Null), &mr_at(&rpath, "ordinal"))? != i {
            return Err(format!("{rpath}/ordinal: not its position"));
        }
        if !ids.insert(id.to_string()) {
            return Err(format!("{rpath}/row_id: `{id}` is not unique"));
        }
        let checksum = mr_text(map.get("checksum").unwrap_or(&Value::Null), &mr_at(&rpath, "checksum"))?;
        let mut body = map.clone();
        body.remove("checksum");
        if modal_reader_sha256(&Value::Object(body)).as_deref() != Some(checksum) {
            return Err(format!("{rpath}/checksum: not the row's checksum"));
        }
        checksums.push(Value::from(checksum));
    }
    if modal_reader_sha256(&Value::Array(checksums)).as_deref() != t["digest"].as_str() {
        return Err(format!("{path}/digest: not the digest of its rows' checksums"));
    }
    Ok(Table { rows: Some(rows) })
}

fn mr_fields<'v>(row: &'v Value, path: &str, fields: &[&str]) -> R<&'v Map<String, Value>> {
    let mut keys: Vec<&str> = fields.to_vec();
    keys.extend(["row_id", "ordinal", "checksum"]);
    mr_object(row, path, &keys)
}

fn mr_id_is(row: &Map<String, Value>, path: &str, expected: &str) -> R<()> {
    if row["row_id"].as_str() == Some(expected) {
        Ok(())
    } else {
        Err(format!("{path}/row_id: expected `{expected}`"))
    }
}

/// A unit-checked value object, kept for the units relation.
struct Unitized {
    vo: Vo,
    key: String,
    path: String,
}

struct Mode {
    case: usize,
    reference: usize,
    sensitive: bool,
    reasons: usize,
    omega: Option<f64>,
    frequency: Option<f64>,
    period: Option<f64>,
    generalized_mass: Vo,
    cluster_estimate: Option<f64>,
    cluster: usize,
}

struct Participation {
    gamma: Vo,
    effective: Vo,
    fraction: Vo,
    cumulative: Vo,
}

struct Computed {
    cluster: usize,
    reported: bool,
    mode: Option<usize>,
    omega: Option<f64>,
    frequency: Option<f64>,
    eta: Option<f64>,
    condition: Option<f64>,
    estimate: Option<f64>,
}

struct Candidate {
    decision: String,
    pivots: Option<usize>,
    shift: Option<f64>,
    formation_term: Option<f64>,
    factor_term: Option<f64>,
    backward_error: Option<f64>,
    inverse_norm: Option<f64>,
    product: Option<f64>,
    separation: Vo,
    growth: Option<f64>,
}

/// Everything the relations read, after closure.
struct Parsed {
    nodes: Vec<[Option<f64>; 3]>,
    /// Per global DOF, its free index.
    free: Vec<Option<usize>>,
    /// (row, col, value); `None` when withheld.
    mass: Option<Vec<(usize, usize, Option<f64>)>>,
    totals: [Option<f64>; 3],
    free_totals: [Option<f64>; 3],
    modes: Vec<Mode>,
    /// Per mode, per global DOF; `None` when withheld.
    shapes: Option<Vec<Vec<Option<f64>>>>,
    participation: Vec<[Participation; 3]>,
    clusters: Vec<(usize, usize, bool, Option<f64>)>,
    computed: Vec<Computed>,
    candidates: Vec<Candidate>,
    identity: Vec<Option<f64>>,
    params: Map<String, Value>,
    unitized: Vec<Unitized>,
    tau_t: Option<f64>,
    ell: Option<f64>,
    mode_count: Option<usize>,
    cutoff: Option<f64>,
    stopped_by: String,
    bounded: usize,
    extension: usize,
    count: usize,
    computed_count: usize,
    endpoint: String,
    next_computed: Option<usize>,
    established_endpoint: Option<usize>,
    census_established: Option<usize>,
    sensitive: bool,
    set_reasons: usize,
    sturm_count: usize,
    sturm_reported: usize,
    placement: String,
    placement_lower: Option<f64>,
    placement_upper: Option<f64>,
    placement_established: Option<usize>,
    kappa_c: Option<f64>,
    limit: Option<f64>,
    kappa_s: Option<f64>,
    growth_limit: Option<f64>,
    ortho: Option<f64>,
    ortho_pair: bool,
    formation: Vec<Option<f64>>,
}

fn mr_index(text: &str, prefix: &str, path: &str) -> R<usize> {
    text.strip_prefix(prefix)
        .and_then(|rest| rest.parse::<usize>().ok())
        .filter(|&i| i >= 1 && format!("{prefix}{i}") == text)
        .ok_or_else(|| format!("{path}: `{text}` is not a `{prefix}<n>` reference"))
}

fn mr_unitize(out: &mut Vec<Unitized>, vo: &Vo, key: &str, path: &str) {
    out.push(Unitized { vo: vo.clone(), key: key.to_string(), path: path.to_string() });
}

/// Closure of a solved document; returns what the relations read.
fn mr_closure(doc: &Value) -> R<Parsed> {
    mr_object(doc, "", &SOLVED_KEYS)?;
    mr_identity(doc)?;
    mr_const(&doc["status"], "/status", "solved")?;
    let mut unitized = Vec::new();
    let params = doc["binding"]["integrity_parameters"].as_object().cloned().unwrap_or_default();

    // Tables first: the headers refer into them.
    let tables = mr_object(&doc["tables"], "/tables", &TABLES)?;
    let tables_value = Value::Object(tables.clone());
    let mut ids = HashSet::new();
    let mut get = |name: &str| mr_table(&tables_value, name, &mut ids).map(|t| t.rows);

    let node_rows = get("nodes")?.ok_or("/tables/nodes: withheld")?;
    let mut node_index = HashMap::new();
    let mut node_ids = Vec::new();
    let mut nodes = Vec::new();
    for (i, row) in node_rows.iter().enumerate() {
        let path = format!("/tables/nodes/rows/{i}");
        let r = mr_fields(row, &path, &["node_id", "x", "y", "z"])?;
        let id = mr_text(&r["node_id"], &mr_at(&path, "node_id"))?;
        mr_id_is(r, &path, &format!("node:{id}"))?;
        let mut c = [None; 3];
        for (k, axis) in ["x", "y", "z"].iter().enumerate() {
            let vo = mr_vo(&r[*axis], &mr_at(&path, axis), &["node_coordinate"])?;
            c[k] = vo.value;
            mr_unitize(&mut unitized, &vo, "fixed", &mr_at(&path, axis));
        }
        node_index.insert(format!("node:{id}"), i);
        node_ids.push(id.to_string());
        nodes.push(c);
    }
    let ndof = 6 * nodes.len();
    let dof_rows = get("dofs")?.ok_or("/tables/dofs: withheld")?;
    if dof_rows.len() != ndof {
        return Err("/tables/dofs: not six DOFs per node".into());
    }
    let mut dof_index = HashMap::new();
    let mut free = Vec::with_capacity(ndof);
    for (i, row) in dof_rows.iter().enumerate() {
        let path = format!("/tables/dofs/rows/{i}");
        let r = mr_fields(row, &path, &["node_ref", "component", "free_index"])?;
        let node = *node_index.get(mr_text(&r["node_ref"], &mr_at(&path, "node_ref"))?).ok_or_else(|| format!("{path}/node_ref: no such node"))?;
        let component = mr_one_of(&r["component"], &mr_at(&path, "component"), &COMPONENTS)?;
        let c = COMPONENTS.iter().position(|x| *x == component).unwrap_or(0);
        if 6 * node + c != i {
            return Err(format!("{path}: not DOF {i} in node-major order"));
        }
        let id = format!("dof:{}:{component}", node_ids[node]);
        mr_id_is(r, &path, &id)?;
        dof_index.insert(id, i);
        free.push(if r["free_index"].is_null() { None } else { Some(mr_count(&r["free_index"], &mr_at(&path, "free_index"))?) });
    }
    let mut expected_free = 0;
    for (i, f) in free.iter().enumerate() {
        if let Some(f) = f {
            if *f != expected_free {
                return Err(format!("/tables/dofs/rows/{i}/free_index: free indices ascend from 0 in DOF order"));
            }
            expected_free += 1;
        }
    }
    let dof_of = |value: &Value, path: &str| -> R<usize> {
        let text = mr_text(value, path)?;
        dof_index.get(text).copied().ok_or_else(|| format!("{path}: `{text}` is no DOF"))
    };

    let mass = match get("mass_matrix")? {
        None => None,
        Some(rows) => {
            let mut out = Vec::with_capacity(rows.len());
            let mut seen = HashSet::new();
            for (i, row) in rows.iter().enumerate() {
                let path = format!("/tables/mass_matrix/rows/{i}");
                let r = mr_fields(row, &path, &["row_dof_ref", "col_dof_ref", "pair_class", "value"])?;
                let a = dof_of(&r["row_dof_ref"], &mr_at(&path, "row_dof_ref"))?;
                let b = dof_of(&r["col_dof_ref"], &mr_at(&path, "col_dof_ref"))?;
                if a > b || !seen.insert((a, b)) {
                    return Err(format!("{path}: entries are distinct upper positions (row ≤ col)"));
                }
                let pair = match (a % 6 < 3, b % 6 < 3) {
                    (true, true) => PAIR_CLASSES[0],
                    (false, false) => PAIR_CLASSES[2],
                    _ => PAIR_CLASSES[1],
                };
                mr_const(&r["pair_class"], &mr_at(&path, "pair_class"), pair)?;
                mr_id_is(r, &path, &format!("mass:{}|{}", r["row_dof_ref"].as_str().unwrap_or(""), r["col_dof_ref"].as_str().unwrap_or("")))?;
                let vo = mr_vo(&r["value"], &mr_at(&path, "value"), &["mass_matrix_entry"])?;
                mr_unitize(&mut unitized, &vo, pair, &mr_at(&path, "value"));
                out.push((a, b, vo.value));
            }
            Some(out)
        }
    };
    let total_rows = get("mass_totals")?.ok_or("/tables/mass_totals: withheld")?;
    if total_rows.len() != 3 {
        return Err("/tables/mass_totals: one row per direction".into());
    }
    let (mut totals, mut free_totals) = ([None; 3], [None; 3]);
    for (k, row) in total_rows.iter().enumerate() {
        let path = format!("/tables/mass_totals/rows/{k}");
        let r = mr_fields(row, &path, &["direction", "basis_ref", "total_mass", "free_dof_mass"])?;
        mr_const(&r["direction"], &mr_at(&path, "direction"), DIRECTIONS[k])?;
        mr_id_is(r, &path, &format!("mass-total:{}", DIRECTIONS[k]))?;
        if r["basis_ref"] != json!({"ref_type": "model", "ref_id": "model"}) {
            return Err(format!("{path}/basis_ref: the global mass is on the model basis"));
        }
        let t = mr_vo(&r["total_mass"], &mr_at(&path, "total_mass"), &["modal_total_mass"])?;
        let f = mr_vo(&r["free_dof_mass"], &mr_at(&path, "free_dof_mass"), &["modal_free_dof_mass"])?;
        totals[k] = t.value;
        free_totals[k] = f.value;
        mr_unitize(&mut unitized, &t, "fixed", &mr_at(&path, "total_mass"));
        mr_unitize(&mut unitized, &f, "fixed", &mr_at(&path, "free_dof_mass"));
    }

    // Computed modes and clusters (modes refer to both).
    let computed_rows = get("computed_modes")?.ok_or("/tables/computed_modes: withheld")?;
    let cluster_rows = get("clusters")?.ok_or("/tables/clusters: withheld")?;
    let mode_rows = get("modes")?.ok_or("/tables/modes: withheld")?;
    let n = mode_rows.len();
    let mut computed = Vec::with_capacity(computed_rows.len());
    for (i, row) in computed_rows.iter().enumerate() {
        let path = format!("/tables/computed_modes/rows/{i}");
        let r = mr_fields(row, &path, &["computed_index", "cluster_ref", "reported", "mode_ref", "band_recovered", "circular_frequency",
            "frequency", "backward_error", "condition", "error_estimate"])?;
        if mr_count(&r["computed_index"], &mr_at(&path, "computed_index"))? != i + 1 {
            return Err(format!("{path}/computed_index: not {}", i + 1));
        }
        mr_id_is(r, &path, &format!("computed:{}", i + 1))?;
        let cluster = mr_index(mr_text(&r["cluster_ref"], &mr_at(&path, "cluster_ref"))?, "cluster:", &mr_at(&path, "cluster_ref"))?;
        if cluster > cluster_rows.len() {
            return Err(format!("{path}/cluster_ref: no such cluster"));
        }
        let reported = mr_flag(&r["reported"], &mr_at(&path, "reported"))?;
        let mode = if r["mode_ref"].is_null() {
            None
        } else {
            let m = mr_index(mr_text(&r["mode_ref"], &mr_at(&path, "mode_ref"))?, "mode:", &mr_at(&path, "mode_ref"))?;
            if m > n {
                return Err(format!("{path}/mode_ref: no such mode"));
            }
            Some(m)
        };
        mr_flag(&r["band_recovered"], &mr_at(&path, "band_recovered"))?;
        let mut vos = Vec::new();
        for (field, kind) in [("circular_frequency", "computed_circular_frequency"), ("frequency", "computed_frequency"),
            ("backward_error", "residual_backward_error"), ("condition", "eigenvalue_condition_estimate"), ("error_estimate", "eigenvalue_error_estimate")] {
            let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
            mr_unitize(&mut unitized, &vo, "fixed", &mr_at(&path, field));
            vos.push(vo.value);
        }
        computed.push(Computed { cluster, reported, mode, omega: vos[0], frequency: vos[1], eta: vos[2], condition: vos[3], estimate: vos[4] });
    }
    let mut clusters = Vec::with_capacity(cluster_rows.len());
    for (i, row) in cluster_rows.iter().enumerate() {
        let path = format!("/tables/clusters/rows/{i}");
        let r = mr_fields(row, &path, &["cluster_index", "first_computed_ref", "last_computed_ref", "reported", "error_estimate"])?;
        if mr_count(&r["cluster_index"], &mr_at(&path, "cluster_index"))? != i + 1 {
            return Err(format!("{path}/cluster_index: not {}", i + 1));
        }
        mr_id_is(r, &path, &format!("cluster:{}", i + 1))?;
        let first = mr_index(mr_text(&r["first_computed_ref"], &mr_at(&path, "first_computed_ref"))?, "computed:", &mr_at(&path, "first_computed_ref"))?;
        let last = mr_index(mr_text(&r["last_computed_ref"], &mr_at(&path, "last_computed_ref"))?, "computed:", &mr_at(&path, "last_computed_ref"))?;
        if first > last || last > computed.len() {
            return Err(format!("{path}: not a run of computed modes"));
        }
        let reported = mr_flag(&r["reported"], &mr_at(&path, "reported"))?;
        let vo = mr_vo(&r["error_estimate"], &mr_at(&path, "error_estimate"), &["cluster_error_estimate"])?;
        mr_unitize(&mut unitized, &vo, "fixed", &mr_at(&path, "error_estimate"));
        clusters.push((first, last, reported, vo.value));
    }
    let mut modes = Vec::with_capacity(n);
    for (i, row) in mode_rows.iter().enumerate() {
        let path = format!("/tables/modes/rows/{i}");
        let r = mr_fields(row, &path, &["mode_index", "computed_ref", "cluster_ref", "normalization_case", "reference_dof_ref", "standing",
            "circular_frequency", "frequency", "period", "generalized_mass", "cluster_error_estimate"])?;
        if mr_count(&r["mode_index"], &mr_at(&path, "mode_index"))? != i + 1 {
            return Err(format!("{path}/mode_index: not {}", i + 1));
        }
        mr_id_is(r, &path, &format!("mode:{}", i + 1))?;
        mr_const(&r["computed_ref"], &mr_at(&path, "computed_ref"), &format!("computed:{}", i + 1))?;
        let cluster = mr_index(mr_text(&r["cluster_ref"], &mr_at(&path, "cluster_ref"))?, "cluster:", &mr_at(&path, "cluster_ref"))?;
        let case_text = mr_one_of(&r["normalization_case"], &mr_at(&path, "normalization_case"), &CASES)?;
        let case = if case_text == CASES[0] { 0 } else { 1 };
        let reference = dof_of(&r["reference_dof_ref"], &mr_at(&path, "reference_dof_ref"))?;
        let s = mr_object(&r["standing"], &mr_at(&path, "standing"), &["token", "reasons"])?;
        let token = mr_one_of(&s["token"], &format!("{path}/standing/token"), &["checks_passed", "sensitive"])?;
        let reasons = mr_list(&s["reasons"], &format!("{path}/standing/reasons"))?;
        for (j, reason) in reasons.iter().enumerate() {
            mr_text(reason, &format!("{path}/standing/reasons/{j}"))?;
        }
        let mut get_vo = |field: &str, kind: &str, key: &str| -> R<Vo> {
            let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
            mr_unitize(&mut unitized, &vo, key, &mr_at(&path, field));
            Ok(vo)
        };
        let omega = get_vo("circular_frequency", "modal_circular_frequency", "fixed")?.value;
        let frequency = get_vo("frequency", "modal_frequency", "fixed")?.value;
        let period = get_vo("period", "modal_period", "fixed")?.value;
        let generalized_mass = get_vo("generalized_mass", "modal_generalized_mass", CASES[case])?;
        let cluster_estimate = get_vo("cluster_error_estimate", "modal_cluster_error_estimate", "fixed")?.value;
        modes.push(Mode { case, reference, sensitive: token == "sensitive", reasons: reasons.len(), omega, frequency, period, generalized_mass,
            cluster_estimate, cluster });
    }
    let shapes = match get("shapes")? {
        None => None,
        Some(rows) => {
            if rows.len() != n * ndof {
                return Err("/tables/shapes: one row per mode and DOF".into());
            }
            let mut out = vec![vec![None; ndof]; n];
            for (i, row) in rows.iter().enumerate() {
                let path = format!("/tables/shapes/rows/{i}");
                let (m, dof) = (i / ndof.max(1), i % ndof.max(1));
                let r = mr_fields(row, &path, &["mode_ref", "dof_ref", "component", "value"])?;
                mr_const(&r["mode_ref"], &mr_at(&path, "mode_ref"), &format!("mode:{}", m + 1))?;
                if dof_of(&r["dof_ref"], &mr_at(&path, "dof_ref"))? != dof {
                    return Err(format!("{path}/dof_ref: not DOF {dof} of mode {}", m + 1));
                }
                mr_const(&r["component"], &mr_at(&path, "component"), COMPONENTS[dof % 6])?;
                mr_id_is(r, &path, &format!("shape:mode:{}:{}", m + 1, r["dof_ref"].as_str().unwrap_or("")))?;
                let kind = if dof % 6 < 3 { "mode_shape_translation" } else { "mode_shape_rotation" };
                let vo = mr_vo(&r["value"], &mr_at(&path, "value"), &[kind])?;
                mr_unitize(&mut unitized, &vo, CASES[modes[m].case], &mr_at(&path, "value"));
                out[m][dof] = vo.value;
            }
            Some(out)
        }
    };
    let participation_rows = get("participation")?.ok_or("/tables/participation: withheld")?;
    if participation_rows.len() != 3 * n {
        return Err("/tables/participation: one row per mode and direction".into());
    }
    let mut participation = Vec::with_capacity(n);
    for m in 0..n {
        let mut row_set = Vec::with_capacity(3);
        for k in 0..3 {
            let i = 3 * m + k;
            let path = format!("/tables/participation/rows/{i}");
            let r = mr_fields(&participation_rows[i], &path, &["mode_ref", "direction", "participation_factor", "effective_mass",
                "effective_mass_fraction", "cumulative_effective_mass_fraction"])?;
            mr_const(&r["mode_ref"], &mr_at(&path, "mode_ref"), &format!("mode:{}", m + 1))?;
            mr_const(&r["direction"], &mr_at(&path, "direction"), DIRECTIONS[k])?;
            mr_id_is(r, &path, &format!("participation:mode:{}:{}", m + 1, DIRECTIONS[k]))?;
            let mut read = |field: &str, kind: &str, key: &str| -> R<Vo> {
                let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
                mr_unitize(&mut unitized, &vo, key, &mr_at(&path, field));
                Ok(vo)
            };
            let gamma = read("participation_factor", "modal_participation_factor", CASES[modes[m].case])?;
            let effective = read("effective_mass", "modal_effective_mass", "fixed")?;
            let fraction = read("effective_mass_fraction", "modal_effective_mass_fraction", "fixed")?;
            let cumulative = read("cumulative_effective_mass_fraction", "modal_cumulative_effective_mass_fraction", "fixed")?;
            row_set.push(Participation { gamma, effective, fraction, cumulative });
        }
        let mut it = row_set.into_iter();
        let (Some(a), Some(b), Some(c)) = (it.next(), it.next(), it.next()) else { unreachable!() };
        participation.push([a, b, c]);
    }
    let candidate_rows = get("sturm_candidates")?.ok_or("/tables/sturm_candidates: withheld")?;
    let mut candidates = Vec::with_capacity(candidate_rows.len());
    for (i, row) in candidate_rows.iter().enumerate() {
        let path = format!("/tables/sturm_candidates/rows/{i}");
        let r = mr_fields(row, &path, &["candidate_index", "decision", "accepted", "negative_pivot_count", "shift", "formation_term",
            "formation_separation_ratio", "factor_term", "backward_error", "inverse_norm_estimate", "conditioning_product", "separation_ratio",
            "growth", "max_abs_lower_factor", "max_abs_pivot", "max_abs_scaled_diagonal"])?;
        if mr_count(&r["candidate_index"], &mr_at(&path, "candidate_index"))? != i + 1 {
            return Err(format!("{path}/candidate_index: not {}", i + 1));
        }
        mr_id_is(r, &path, &format!("sturm-candidate:{}", i + 1))?;
        let decision = mr_one_of(&r["decision"], &mr_at(&path, "decision"), &DECISIONS)?.to_string();
        if mr_flag(&r["accepted"], &mr_at(&path, "accepted"))? != (decision == "accepted") {
            return Err(format!("{path}/accepted: not its decision"));
        }
        let pivots = if r["negative_pivot_count"].is_null() { None } else { Some(mr_count(&r["negative_pivot_count"], &mr_at(&path, "negative_pivot_count"))?) };
        let mut read = |field: &str, kind: &str| -> R<Vo> {
            let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
            mr_unitize(&mut unitized, &vo, "fixed", &mr_at(&path, field));
            Ok(vo)
        };
        let shift = read("shift", "sturm_shift_circular_frequency")?.value;
        let formation_term = read("formation_term", "sturm_formation_term")?.value;
        read("formation_separation_ratio", "sturm_formation_separation_ratio")?;
        let factor_term = read("factor_term", "sturm_factor_term")?.value;
        let backward_error = read("backward_error", "sturm_backward_error")?.value;
        let inverse_norm = read("inverse_norm_estimate", "sturm_inverse_norm_estimate")?.value;
        let product = read("conditioning_product", "sturm_conditioning_product")?.value;
        let separation = read("separation_ratio", "sturm_separation_ratio")?;
        let growth = read("growth", "sturm_growth_factor")?.value;
        read("max_abs_lower_factor", "sturm_max_abs_lower_factor")?;
        read("max_abs_pivot", "sturm_max_abs_pivot")?;
        read("max_abs_scaled_diagonal", "sturm_max_abs_scaled_diagonal")?;
        candidates.push(Candidate { decision, pivots, shift, formation_term, factor_term, backward_error, inverse_norm, product, separation, growth });
    }
    let identity_rows = get("identity")?.ok_or("/tables/identity: withheld")?;
    let mut identity = Vec::new();
    for (i, row) in identity_rows.iter().enumerate() {
        let path = format!("/tables/identity/rows/{i}");
        let r = mr_fields(row, &path, &["subject", "axis", "comparison", "relative"])?;
        mr_id_is(r, &path, &format!("identity:{i}"))?;
        mr_text(&r["subject"], &mr_at(&path, "subject"))?;
        mr_one_of(&r["axis"], &mr_at(&path, "axis"), &DIRECTIONS)?;
        mr_one_of(&r["comparison"], &mr_at(&path, "comparison"), &["nodal", "resultant"])?;
        let vo = mr_vo(&r["relative"], &mr_at(&path, "relative"), &["identity_relative_discrepancy"])?;
        mr_unitize(&mut unitized, &vo, "fixed", &mr_at(&path, "relative"));
        identity.push(vo.value);
    }
    for (i, row) in get("supports")?.ok_or("/tables/supports: withheld")?.iter().enumerate() {
        let path = format!("/tables/supports/rows/{i}");
        let r = mr_fields(row, &path, &["support_id", "behavior", "dof_ref", "state", "treatment"])?;
        let id = mr_text(&r["support_id"], &mr_at(&path, "support_id"))?;
        mr_id_is(r, &path, &format!("support:{id}"))?;
        mr_text(&r["behavior"], &mr_at(&path, "behavior"))?;
        dof_of(&r["dof_ref"], &mr_at(&path, "dof_ref"))?;
        mr_one_of(&r["state"], &mr_at(&path, "state"), &["active", "inactive", "sticking", "sliding"])?;
        mr_one_of(&r["treatment"], &mr_at(&path, "treatment"), &["restrained", "already_restrained", "free", "free_frozen_friction"])?;
    }
    for (i, row) in get("discretization")?.ok_or("/tables/discretization: withheld")?.iter().enumerate() {
        let path = format!("/tables/discretization/rows/{i}");
        let r = mr_fields(row, &path, &["member_ref", "length", "fixed_fixed_frequency", "ratio_to_highest_reported"])?;
        let id = mr_text(&r["member_ref"], &mr_at(&path, "member_ref"))?;
        mr_id_is(r, &path, &format!("member:{id}"))?;
        for (field, kind) in [("length", "member_length"), ("fixed_fixed_frequency", "member_fixed_fixed_frequency"), ("ratio_to_highest_reported", "member_frequency_ratio")] {
            let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
            mr_unitize(&mut unitized, &vo, "fixed", &mr_at(&path, field));
        }
    }
    for (i, row) in get("disclosures")?.ok_or("/tables/disclosures: withheld")?.iter().enumerate() {
        let path = format!("/tables/disclosures/rows/{i}");
        let r = mr_fields(row, &path, &["text", "source"])?;
        mr_id_is(r, &path, &format!("disclosure:{i}"))?;
        mr_text(&r["text"], &mr_at(&path, "text"))?;
        mr_one_of(&r["source"], &mr_at(&path, "source"), &["modal", "weight_basis"])?;
    }
    for (i, row) in get("diagnostics")?.ok_or("/tables/diagnostics: withheld")?.iter().enumerate() {
        let path = format!("/tables/diagnostics/rows/{i}");
        mr_diagnostic_row(row, &path, &format!("diagnostic:{i}"), &["modal", "state_case_solve"])?;
    }

    // Headers.
    let basis = mr_object(&doc["basis"], "/basis", &["route", "no_translation_threshold", "bounding_box_diagonal", "weight_to_mass_acceleration",
        "identity_acceleration", "mass_center", "state_case"])?;
    mr_one_of(&basis["route"], "/basis/route", &["pressure_free", "exact_pressure"])?;
    let mut header_vo = |value: &Value, path: &str, kind: &str| -> R<Vo> {
        let vo = mr_vo(value, path, &[kind])?;
        mr_unitize(&mut unitized, &vo, "fixed", path);
        Ok(vo)
    };
    let tau_t = header_vo(&basis["no_translation_threshold"], "/basis/no_translation_threshold", "basis_no_translation_threshold")?.value;
    let ell = header_vo(&basis["bounding_box_diagonal"], "/basis/bounding_box_diagonal", "basis_bounding_box_diagonal")?.value;
    if !basis["weight_to_mass_acceleration"].is_null() {
        header_vo(&basis["weight_to_mass_acceleration"], "/basis/weight_to_mass_acceleration", "request_weight_to_mass_acceleration")?;
    }
    header_vo(&basis["identity_acceleration"], "/basis/identity_acceleration", "identity_acceleration")?;
    if !basis["mass_center"].is_null() {
        let c = mr_object(&basis["mass_center"], "/basis/mass_center", &["x", "y", "z"])?;
        for axis in ["x", "y", "z"] {
            header_vo(&c[axis], &format!("/basis/mass_center/{axis}"), "basis_mass_center")?;
        }
    }
    if !basis["state_case"].is_null() {
        let s = mr_object(&basis["state_case"], "/basis/state_case", &["case_id", "case_index", "linearized", "solver_mode", "restrained_dof_refs"])?;
        mr_text(&s["case_id"], "/basis/state_case/case_id")?;
        mr_count(&s["case_index"], "/basis/state_case/case_index")?;
        let linearized = mr_flag(&s["linearized"], "/basis/state_case/linearized")?;
        if linearized {
            mr_const(&s["solver_mode"], "/basis/state_case/solver_mode", "dense_scrutiny")?;
            for (i, r) in mr_list(&s["restrained_dof_refs"], "/basis/state_case/restrained_dof_refs")?.iter().enumerate() {
                dof_of(r, &format!("/basis/state_case/restrained_dof_refs/{i}"))?;
            }
        } else if !s["solver_mode"].is_null() || !s["restrained_dof_refs"].is_null() {
            return Err("/basis/state_case: a case that linearizes no support states no mode or restrained set".into());
        }
    }
    let x = mr_object(&doc["extraction"], "/extraction", &["mode_count", "frequency_cutoff", "stopped_by", "bounded_count", "extension_count",
        "count", "computed_count", "endpoint"])?;
    let mode_count = if x["mode_count"].is_null() { None } else { Some(mr_count(&x["mode_count"], "/extraction/mode_count")?) };
    let cutoff = if x["frequency_cutoff"].is_null() { None } else { header_vo(&x["frequency_cutoff"], "/extraction/frequency_cutoff", "request_frequency_cutoff")?.value };
    if mode_count.is_none() && x["frequency_cutoff"].is_null() {
        return Err("/extraction: a modal result states its bound".into());
    }
    let stopped_by = mr_one_of(&x["stopped_by"], "/extraction/stopped_by", &["mode_count", "frequency_cutoff", "mode_count_and_frequency_cutoff",
        "spectrum_exhausted"])?.to_string();
    let e = mr_object(&x["endpoint"], "/extraction/endpoint", &["kind", "next_computed_ref", "established_finite_mode_count"])?;
    let endpoint = mr_one_of(&e["kind"], "/extraction/endpoint/kind", &["below_computed_cluster", "below_lowest_mode", "whole_finite_spectrum"])?.to_string();
    let next_computed = if e["next_computed_ref"].is_null() { None } else {
        Some(mr_index(mr_text(&e["next_computed_ref"], "/extraction/endpoint/next_computed_ref")?, "computed:", "/extraction/endpoint/next_computed_ref")?)
    };
    let established_endpoint = if e["established_finite_mode_count"].is_null() { None } else {
        Some(mr_count(&e["established_finite_mode_count"], "/extraction/endpoint/established_finite_mode_count")?)
    };
    let c = mr_object(&doc["census"], "/census", &["kind", "finite_mode_count", "structurally_massless", "nonpositive_mu", "rejected_mu",
        "mass_block_established"])?;
    let census_kind = mr_one_of(&c["kind"], "/census/kind", &["established", "unresolved"])?;
    mr_count(&c["structurally_massless"], "/census/structurally_massless")?;
    let census_established = if census_kind == "established" {
        for k in ["nonpositive_mu", "rejected_mu", "mass_block_established"] {
            if !c[k].is_null() {
                return Err(format!("/census/{k}: an established census states none"));
            }
        }
        Some(mr_count(&c["finite_mode_count"], "/census/finite_mode_count")?)
    } else {
        if !c["finite_mode_count"].is_null() {
            return Err("/census/finite_mode_count: an unresolved census states none".into());
        }
        mr_count(&c["nonpositive_mu"], "/census/nonpositive_mu")?;
        mr_count(&c["rejected_mu"], "/census/rejected_mu")?;
        mr_flag(&c["mass_block_established"], "/census/mass_block_established")?;
        None
    };
    let st = mr_object(&doc["standing"], "/standing", &["token", "reasons"])?;
    let token = mr_one_of(&st["token"], "/standing/token", &["checks_passed", "sensitive"])?;
    let set_reasons = mr_list(&st["reasons"], "/standing/reasons")?;
    for (i, reason) in set_reasons.iter().enumerate() {
        mr_text(reason, &format!("/standing/reasons/{i}"))?;
    }
    let s = mr_object(&doc["sturm"], "/sturm", &["count", "reported", "placement", "conditioning_multiple", "conditioning_limit",
        "separation_multiple", "growth_limit"])?;
    let p = mr_object(&s["placement"], "/sturm/placement", &["kind", "lower", "upper", "established_finite_mode_count"])?;
    let placement = mr_one_of(&p["kind"], "/sturm/placement/kind", &["given", "gap", "above_spectrum"])?.to_string();
    let mut optional_vo = |value: &Value, path: &str, kind: &str| -> R<Option<f64>> {
        if value.is_null() {
            return Ok(None);
        }
        Ok(header_vo(value, path, kind)?.value)
    };
    let placement_lower = optional_vo(&p["lower"], "/sturm/placement/lower", "placement_circular_frequency")?;
    let placement_upper = optional_vo(&p["upper"], "/sturm/placement/upper", "placement_circular_frequency")?;
    let placement_established = if p["established_finite_mode_count"].is_null() { None } else {
        Some(mr_count(&p["established_finite_mode_count"], "/sturm/placement/established_finite_mode_count")?)
    };
    let kappa_c = header_vo(&s["conditioning_multiple"], "/sturm/conditioning_multiple", "sturm_premise_parameter")?.value;
    let limit = header_vo(&s["conditioning_limit"], "/sturm/conditioning_limit", "sturm_premise_parameter")?.value;
    let kappa_s = header_vo(&s["separation_multiple"], "/sturm/separation_multiple", "sturm_premise_parameter")?.value;
    let growth_limit = header_vo(&s["growth_limit"], "/sturm/growth_limit", "sturm_premise_parameter")?.value;
    let o = mr_object(&doc["orthogonality"], "/orthogonality", &["max_ratio", "pair"])?;
    let ortho = header_vo(&o["max_ratio"], "/orthogonality/max_ratio", "orthogonality_max_ratio")?.value;
    let ortho_pair = !o["pair"].is_null();
    if ortho_pair {
        let pair = mr_list(&o["pair"], "/orthogonality/pair")?;
        if pair.len() != 2 {
            return Err("/orthogonality/pair: two modes".into());
        }
        let a = mr_index(mr_text(&pair[0], "/orthogonality/pair/0")?, "mode:", "/orthogonality/pair/0")?;
        let b = mr_index(mr_text(&pair[1], "/orthogonality/pair/1")?, "mode:", "/orthogonality/pair/1")?;
        if !(a < b && b <= n) {
            return Err("/orthogonality/pair: two reported modes, ascending".into());
        }
    }
    let f = mr_object(&doc["formation"], "/formation", &["stiffness_difference", "stiffness_row_ref", "stiffness_off_pattern", "mass_difference",
        "mass_row_ref", "mass_off_pattern", "reformed_stiffness_norm", "reformed_mass_norm", "section"])?;
    let mut formation = Vec::new();
    for (field, kind) in [("stiffness_difference", "formation_difference"), ("mass_difference", "formation_difference"),
        ("reformed_stiffness_norm", "reformed_norm"), ("reformed_mass_norm", "reformed_norm")] {
        formation.push(header_vo(&f[field], &format!("/formation/{field}"), kind)?.value);
    }
    for field in ["stiffness_row_ref", "mass_row_ref"] {
        if !f[field].is_null() {
            dof_of(&f[field], &format!("/formation/{field}"))?;
        }
    }
    mr_count(&f["stiffness_off_pattern"], "/formation/stiffness_off_pattern")?;
    mr_count(&f["mass_off_pattern"], "/formation/mass_off_pattern")?;
    if !f["section"].is_null() {
        let sec = mr_object(&f["section"], "/formation/section", &["area", "second_moment", "torsion_constant"])?;
        for field in ["area", "second_moment", "torsion_constant"] {
            formation.push(header_vo(&sec[field], &format!("/formation/section/{field}"), "section_derivation_difference")?.value);
        }
    }
    let g = mr_object(&doc["guard"], "/guard", &["global_dimension", "free_dimension", "static_estimate_bytes", "ceiling_bytes", "eigen_core_bytes",
        "operation_estimate", "statement"])?;
    if mr_count(&g["global_dimension"], "/guard/global_dimension")? != ndof || mr_count(&g["free_dimension"], "/guard/free_dimension")? != expected_free {
        return Err("/guard: not the document's DOF counts".into());
    }
    for field in ["static_estimate_bytes", "ceiling_bytes", "eigen_core_bytes", "operation_estimate"] {
        let text = mr_text(&g[field], &format!("/guard/{field}"))?;
        if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("/guard/{field}: a decimal integer"));
        }
    }
    mr_text(&g["statement"], "/guard/statement")?;
    // Cross-references.
    for (i, m) in modes.iter().enumerate() {
        if m.cluster > clusters.len() {
            return Err(format!("/tables/modes/rows/{i}/cluster_ref: no such cluster"));
        }
    }
    let count_of = |v: &Value, path: &str| mr_count(v, path);
    Ok(Parsed {
        nodes,
        free,
        mass,
        totals,
        free_totals,
        modes,
        shapes,
        participation,
        clusters,
        computed,
        candidates,
        identity,
        params,
        unitized,
        tau_t,
        ell,
        mode_count,
        cutoff,
        stopped_by,
        bounded: count_of(&x["bounded_count"], "/extraction/bounded_count")?,
        extension: count_of(&x["extension_count"], "/extraction/extension_count")?,
        count: count_of(&x["count"], "/extraction/count")?,
        computed_count: count_of(&x["computed_count"], "/extraction/computed_count")?,
        endpoint,
        next_computed,
        established_endpoint,
        census_established,
        sensitive: token == "sensitive",
        set_reasons: set_reasons.len(),
        sturm_count: count_of(&s["count"], "/sturm/count")?,
        sturm_reported: count_of(&s["reported"], "/sturm/reported")?,
        placement,
        placement_lower,
        placement_upper,
        placement_established,
        kappa_c,
        limit,
        kappa_s,
        growth_limit,
        ortho,
        ortho_pair,
        formation,
    })
}

fn mr_diagnostic_row(row: &Value, path: &str, id: &str, sources: &[&str]) -> R<String> {
    let r = mr_fields(row, path, &["diagnostic_id", "code", "severity", "message", "affected_refs", "source"])?;
    mr_id_is(r, path, id)?;
    mr_text(&r["diagnostic_id"], &mr_at(path, "diagnostic_id"))?;
    let code = mr_text(&r["code"], &mr_at(path, "code"))?;
    if code.is_empty() || !code.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_') {
        return Err(format!("{path}/code: an upper-case code"));
    }
    let severity = mr_text(&r["severity"], &mr_at(path, "severity"))?;
    mr_text(&r["message"], &mr_at(path, "message"))?;
    for (i, reference) in mr_list(&r["affected_refs"], &mr_at(path, "affected_refs"))?.iter().enumerate() {
        mr_text(reference, &format!("{path}/affected_refs/{i}"))?;
    }
    mr_one_of(&r["source"], &mr_at(path, "source"), sources)?;
    Ok(format!("{severity}\u{0}{code}"))
}

fn mr_read_refused(doc: &Value, supplied: ModalSupplied<'_>) -> ModalReadVerdict {
    let closure = (|| -> R<Vec<String>> {
        mr_object(doc, "", &REFUSED_KEYS)?;
        mr_identity(doc)?;
        let mut ids = HashSet::new();
        let wrapper = json!({"refusals": doc["refusals"]});
        let rows = mr_table(&wrapper, "refusals", &mut ids)?.rows.ok_or("/refusals: withheld")?;
        let mut codes = Vec::new();
        for (i, row) in rows.iter().enumerate() {
            let tagged = mr_diagnostic_row(row, &format!("/refusals/rows/{i}"), &format!("refusal:{i}"), &["modal"])?;
            if let Some(code) = tagged.strip_prefix("blocking\u{0}") {
                codes.push(code.to_string());
            }
        }
        if codes.is_empty() {
            return Err("/refusals: a refused document carries at least one blocking diagnostic".into());
        }
        Ok(codes)
    })();
    let codes = match closure {
        Ok(codes) => codes,
        Err(difference) => return mr_closure_failed(difference.replace("/tables/refusals", "/refusals")),
    };
    let (binding, not_checked) = mr_binding(doc, supplied);
    let decision = if binding.status == ModalReadStatus::Failed { "rejected" } else { "refusal_consistent" };
    ModalReadVerdict {
        closure: ModalReadStanding { status: ModalReadStatus::Passed, difference: None, scope: Vec::new() },
        binding,
        binding_not_checked: not_checked,
        relations: MODAL_RELATIONS.iter().map(|_| ModalReadStanding::mr_unchecked()).collect(),
        health: "not_assessed",
        refusal_codes: codes,
        decision,
    }
}

/// The binding (module docs): its internal hashes and run id, the request
/// options against the extraction and basis, and a supplied request.
fn mr_binding(doc: &Value, supplied: ModalSupplied<'_>) -> (ModalReadStanding, Vec<String>) {
    let mut not_checked = Vec::new();
    let result = (|| -> R<Vec<String>> {
        let b = &doc["binding"];
        if modal_reader_sha256(&b["integrity_parameters"]).as_deref() != b["integrity_parameters_sha256"].as_str() {
            return Err("/binding/integrity_parameters_sha256: not the parameters' hash".into());
        }
        let run = modal_reader_sha256(&json!({"request_sha256": b["request_sha256"], "integrity_parameters_sha256": b["integrity_parameters_sha256"]}))
            .unwrap_or_default();
        if doc["run"]["run_id"].as_str() != Some(format!("modal-run:{}", run.get(..16).unwrap_or("")).as_str()) {
            return Err("/run/run_id: not the binding's run id".into());
        }
        let params = &b["integrity_parameters"];
        let options = &b["options"];
        if doc["status"] == "solved" {
            if doc["basis"]["no_translation_threshold"]["value"] != params["no_translation_threshold"] {
                return Err("/basis/no_translation_threshold: not the bound τ_t".into());
            }
            let s = &doc["sturm"];
            for (field, param) in [("conditioning_multiple", "sturm_conditioning_multiple"), ("separation_multiple", "sturm_separation_multiple"),
                ("growth_limit", "sturm_growth_limit")] {
                if s[field]["value"] != params[param] {
                    return Err(format!("/sturm/{field}: not the bound {param}"));
                }
            }
            if doc["extraction"]["mode_count"] != *options.get("mode_count").unwrap_or(&Value::Null) {
                return Err("/extraction/mode_count: not the request's".into());
            }
            for (option, header, dimension) in [("frequency_cutoff", &doc["extraction"]["frequency_cutoff"], Dimension::Frequency),
                ("weight_to_mass_acceleration", &doc["basis"]["weight_to_mass_acceleration"], Dimension::Acceleration)] {
                match (options.get(option), header.is_null()) {
                    (None, true) => {}
                    (Some(q), false) => {
                        let value = q["value"].as_f64().ok_or_else(|| format!("/binding/options/{option}: not a quantity"))?;
                        let unit = q["unit"].as_str().ok_or_else(|| format!("/binding/options/{option}: not a quantity"))?;
                        let converted = open_pipe_stress_units::unit_by_symbol(unit, dimension)
                            .ok()
                            .and_then(|id| open_pipe_stress_units::canonical_unit(dimension).and_then(|to| open_pipe_stress_units::convert(value, id, to).ok()))
                            .ok_or_else(|| format!("/binding/options/{option}: not convertible"))?;
                        let published = header["value"].as_f64().unwrap_or(f64::NAN);
                        if !((published - converted).abs() <= 4.0 * U * converted.abs()) {
                            return Err(format!("{option}: the published value is not the request's"));
                        }
                    }
                    _ => return Err(format!("{option}: stated in only one of the request and the document")),
                }
            }
            let bound = &b["state_case"];
            let basis_case = &doc["basis"]["state_case"];
            if bound.is_null() != basis_case.is_null()
                || (!bound.is_null() && (bound["case_id"] != basis_case["case_id"] || bound["case_index"] != basis_case["case_index"]))
            {
                return Err("/basis/state_case: not the bound state case".into());
            }
            if bound.is_null() != options.get("state_case").is_none() || (!bound.is_null() && options["state_case"] != bound["case_id"]) {
                return Err("/binding/state_case: not the request's state_case".into());
            }
        }
        match supplied.request {
            None => not_checked.push("request".to_string()),
            Some(request) => {
                if modal_reader_sha256(request).as_deref() != b["request_sha256"].as_str() {
                    return Err("/binding/request_sha256: not the supplied request's".into());
                }
                let model = request.get("model").and_then(modal_reader_sha256);
                if model.as_deref() != b["model_sha256"].as_str() {
                    return Err("/binding/model_sha256: not the supplied model's".into());
                }
                let materials = request.get("materials").cloned().unwrap_or_else(|| json!([]));
                if modal_reader_sha256(&materials).as_deref() != b["materials_sha256"].as_str() {
                    return Err("/binding/materials_sha256: not the supplied materials'".into());
                }
                for member in OPTION_MEMBERS {
                    if request.get(member) != options.get(member) {
                        return Err(format!("/binding/options/{member}: not the supplied request's"));
                    }
                }
                if !b["state_case"].is_null() {
                    let index = b["state_case"]["case_index"].as_u64().unwrap_or(u64::MAX) as usize;
                    let case = request["model"]["load_cases"].get(index);
                    if case.and_then(|c| c.get("id")) != Some(&b["state_case"]["case_id"])
                        || case.and_then(modal_reader_sha256).as_deref() != b["state_case"]["case_sha256"].as_str()
                    {
                        return Err("/binding/state_case: not the supplied model's case".into());
                    }
                }
            }
        }
        Ok(Vec::new())
    })();
    (ModalReadStanding::mr_of(result), not_checked)
}

fn mr_param(p: &Parsed, name: &str) -> Option<f64> {
    p.params.get(name).and_then(Value::as_f64)
}

fn mr_tol_close(published: f64, derived: f64, tolerance: f64) -> bool {
    (published - derived).abs() <= tolerance
}

/// Mass: T_k and the free-DOF mass from the mass entries, the diagonal, and
/// Bessel's inequality over the published effective masses.
fn mr_mass(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    for k in 0..3 {
        let total = p.totals[k].ok_or_else(|| format!("T_{}: not published", DIRECTIONS[k]))?;
        if total < 0.0 {
            return Err(format!("T_{}: negative", DIRECTIONS[k]));
        }
        let mut sum = 0.0f64;
        let mut count = 0usize;
        for m in 0..p.modes.len() {
            if let Some(e) = p.participation[m][k].effective.value {
                sum += e;
                count += 1;
            }
        }
        if sum > total * (1.0 + ALLOW * ((count + 2) as f64) * U) {
            return Err(format!("Σ m_eff,{} = {sum:e} exceeds T_{} = {total:e} (Bessel's inequality)", DIRECTIONS[k], DIRECTIONS[k]));
        }
    }
    let Some(entries) = &p.mass else {
        scope.push("mass_matrix withheld: T_k and the free-DOF mass are not re-derived".to_string());
        return Ok(scope);
    };
    for &(i, j, v) in entries {
        let v = v.ok_or("a mass entry is not published")?;
        if i == j && v < 0.0 {
            return Err(format!("mass diagonal at DOF {i} is negative"));
        }
    }
    for k in 0..3 {
        let (mut s, mut a, mut n, mut fs, mut fa, mut fneed) = (0.0f64, 0.0f64, 0usize, 0.0f64, 0.0f64, 0usize);
        for &(i, j, v) in entries {
            let v = v.unwrap_or(0.0);
            if i % 6 == k && j % 6 == k {
                let w = if i == j { 1.0 } else { 2.0 };
                let term = w * v;
                s += term;
                a += term.abs();
                n += 1;
                if p.free[i].is_some() && p.free[j].is_some() {
                    fs += term;
                    fa += term.abs();
                    fneed += 1;
                }
            }
        }
        let total = p.totals[k].unwrap_or(f64::NAN);
        if !mr_tol_close(total, s, ALLOW * ((n + 4) as f64) * U * a) {
            return Err(format!("T_{} = {total:e} is not R_kᵀ M R_k = {s:e} of the mass entries", DIRECTIONS[k]));
        }
        let free_total = p.free_totals[k].ok_or_else(|| format!("free-DOF mass {}: not published", DIRECTIONS[k]))?;
        if !mr_tol_close(free_total, fs, ALLOW * ((fneed + 4) as f64) * U * fa) {
            return Err(format!("free-DOF mass {} = {free_total:e} is not R_k,fᵀ M_ff R_k,f = {fs:e}", DIRECTIONS[k]));
        }
    }
    Ok(scope)
}

/// ℓ from the published node coordinates.
fn mr_diagonal(p: &Parsed) -> R<f64> {
    let mut low = [f64::INFINITY; 3];
    let mut high = [f64::NEG_INFINITY; 3];
    for node in &p.nodes {
        for k in 0..3 {
            let c = node[k].ok_or("a node coordinate is not published")?;
            if c < low[k] {
                low[k] = c;
            }
            if c > high[k] {
                high[k] = c;
            }
        }
    }
    let d = [high[0] - low[0], high[1] - low[1], high[2] - low[2]];
    Ok((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt())
}

/// Normalization: each mode's case and sign (module docs).
fn mr_normalization(p: &Parsed) -> R<Vec<String>> {
    let ell = p.ell.ok_or("ℓ: not published")?;
    let derived = mr_diagonal(p)?;
    if !(ell > 0.0 && mr_tol_close(ell, derived, 8.0 * U * derived)) {
        return Err(format!("ℓ = {ell:e} is not the nodes' bounding-box diagonal {derived:e}"));
    }
    let tau = p.tau_t.ok_or("τ_t: not published")?;
    let Some(shapes) = &p.shapes else {
        return Ok(vec!["shapes withheld: the normalization cases and signs are not re-derived".to_string()]);
    };
    for (m, mode) in p.modes.iter().enumerate() {
        let shape = &shapes[m];
        let label = m + 1;
        if p.free[mode.reference].is_none() {
            return Err(format!("mode {label}: its reference DOF is restrained"));
        }
        if (mode.reference % 6 < 3) != (mode.case == 0) {
            return Err(format!("mode {label}: its reference DOF is not of its case's kind"));
        }
        if shape[mode.reference] != Some(1.0) {
            return Err(format!("mode {label}: its reference component is not exactly +1"));
        }
        let (mut t, mut theta) = (0.0f64, 0.0f64);
        for (dof, value) in shape.iter().enumerate() {
            let v = value.ok_or_else(|| format!("mode {label}: a shape component is not published"))?;
            if p.free[dof].is_none() {
                if v != 0.0 {
                    return Err(format!("mode {label}: restrained DOF {dof} is not exactly 0"));
                }
                continue;
            }
            if dof % 6 < 3 {
                t = t.max(v.abs());
            } else {
                theta = theta.max(v.abs());
            }
        }
        if mode.case == 0 {
            if t > 1.0 {
                return Err(format!("mode {label}: a translation exceeds its unit maximum"));
            }
            if tau * ell * theta * (1.0 - 8.0 * U) >= 1.0 {
                return Err(format!("mode {label}: published in case (a), but its translation is at most τ_t·ℓ times its rotation"));
            }
        } else {
            if theta > 1.0 {
                return Err(format!("mode {label}: a rotation exceeds its unit maximum"));
            }
            if t > tau * ell * (1.0 + 8.0 * U) {
                return Err(format!("mode {label}: published in case (b), but a translation exceeds τ_t·ℓ"));
            }
        }
    }
    Ok(Vec::new())
}

/// Participation: m_i, φᵀp_k, Γ, m_eff, Γ²m, the fractions and the
/// cumulative sums, from the published shapes and mass entries.
fn mr_participation(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    let mut cumulative = [0.0f64; 3];
    for (m, mode) in p.modes.iter().enumerate() {
        let label = m + 1;
        let mass_value = mode.generalized_mass.value.ok_or_else(|| format!("mode {label}: m not published"))?;
        if !(mass_value > 0.0 || mode.generalized_mass.below_normal) {
            return Err(format!("mode {label}: m is not positive"));
        }
        for k in 0..3 {
            let row = &p.participation[m][k];
            let gamma = row.gamma.value.ok_or_else(|| format!("mode {label}: Γ_{} not published", DIRECTIONS[k]))?;
            let effective = row.effective.value.ok_or_else(|| format!("mode {label}: m_eff,{} not published", DIRECTIONS[k]))?;
            if effective < 0.0 {
                return Err(format!("mode {label}: m_eff,{} is negative", DIRECTIONS[k]));
            }
            let flagged = row.gamma.below_normal || row.effective.below_normal || mode.generalized_mass.below_normal;
            let g2m = gamma * gamma * mass_value;
            if !flagged && !mr_tol_close(g2m, effective, ALLOW * U * effective.max(g2m)) {
                return Err(format!("mode {label}: Γ_{}²·m = {g2m:e} is not m_eff = {effective:e}", DIRECTIONS[k]));
            }
            let total = p.totals[k].unwrap_or(f64::NAN);
            if total == 0.0 {
                if row.fraction.undefined.as_deref() != Some("total_mass_zero") || effective != 0.0 {
                    return Err(format!("mode {label}: with T_{} = 0 the fraction is undefined and m_eff is 0", DIRECTIONS[k]));
                }
            } else {
                let fraction = row.fraction.value.ok_or_else(|| format!("mode {label}: fraction {} undefined with T_k > 0", DIRECTIONS[k]))?;
                let ft = fraction * total;
                if !row.fraction.below_normal && !row.effective.below_normal && !mr_tol_close(ft, effective, ALLOW * U * effective.max(ft)) {
                    return Err(format!("mode {label}: fraction {}·T = {ft:e} is not m_eff = {effective:e}", DIRECTIONS[k]));
                }
                cumulative[k] += fraction;
                let published = row.cumulative.value.ok_or_else(|| format!("mode {label}: cumulative {} undefined", DIRECTIONS[k]))?;
                if !mr_tol_close(published, cumulative[k], ALLOW * ((m + 2) as f64) * U * cumulative[k]) {
                    return Err(format!("mode {label}: cumulative fraction {} = {published:e} is not the running sum {:e}", DIRECTIONS[k], cumulative[k]));
                }
            }
        }
    }
    let (Some(entries), Some(shapes)) = (&p.mass, &p.shapes) else {
        scope.push("mass_matrix or shapes withheld: m_i and φᵀp_k are not re-derived".to_string());
        return Ok(scope);
    };
    for (m, mode) in p.modes.iter().enumerate() {
        let label = m + 1;
        let phi: Vec<f64> = shapes[m].iter().map(|v| v.unwrap_or(f64::NAN)).collect();
        let (mut mass, mut abs_mass, mut terms) = (0.0f64, 0.0f64, 0usize);
        for &(i, j, v) in entries {
            if p.free[i].is_some() && p.free[j].is_some() {
                let w = if i == j { 1.0 } else { 2.0 };
                let term = w * (phi[i] * v.unwrap_or(0.0) * phi[j]);
                mass += term;
                abs_mass += term.abs();
                terms += 1;
            }
        }
        let published = mode.generalized_mass.value.unwrap_or(f64::NAN);
        if !mode.generalized_mass.below_normal && !mr_tol_close(published, mass, ALLOW * ((terms + 4) as f64) * U * abs_mass) {
            return Err(format!("mode {label}: m = {published:e} is not φᵀM_ffφ = {mass:e}"));
        }
        for k in 0..3 {
            let (mut s, mut a, mut n) = (0.0f64, 0.0f64, 0usize);
            for &(i, j, v) in entries {
                let v = v.unwrap_or(0.0);
                if p.free[i].is_some() && j % 6 == k {
                    let term = phi[i] * v;
                    s += term;
                    a += term.abs();
                    n += 1;
                }
                if i != j && p.free[j].is_some() && i % 6 == k {
                    let term = phi[j] * v;
                    s += term;
                    a += term.abs();
                    n += 1;
                }
            }
            let row = &p.participation[m][k];
            if row.gamma.below_normal || row.effective.below_normal || mode.generalized_mass.below_normal {
                continue;
            }
            let gamma = row.gamma.value.unwrap_or(f64::NAN);
            let effective = row.effective.value.unwrap_or(f64::NAN);
            let gm = gamma * published;
            let e = ALLOW * ((n + 6) as f64) * U;
            if !mr_tol_close(gm, s, e * (a + gm.abs())) {
                return Err(format!("mode {label}: Γ_{}·m = {gm:e} is not φᵀp_{} = {s:e}", DIRECTIONS[k], DIRECTIONS[k]));
            }
            let em = effective * published;
            if !mr_tol_close(em, s * s, e * (2.0 * s.abs() * a + em.abs()) + (e * a) * (e * a)) {
                return Err(format!("mode {label}: m_eff,{}·m = {em:e} is not (φᵀp_k)² = {:e}", DIRECTIONS[k], s * s));
            }
        }
    }
    Ok(scope)
}

/// The kernel's reported-set rule over the computed frequencies and cluster
/// ids: (bounded count, stop token, count).
fn mr_reported_set(frequencies: &[f64], clusters: &[usize], mode_count: Option<usize>, cutoff: Option<f64>) -> R<(usize, &'static str, usize)> {
    let total = frequencies.len();
    let by_count = |n: usize| (n.min(total), n <= total);
    let by_cutoff = |f: f64| {
        let count = frequencies.iter().take_while(|&&x| x <= f).count();
        (count, count < total)
    };
    let (bounded, stop) = match (mode_count, cutoff) {
        (Some(n), None) => {
            let (c, reached) = by_count(n);
            (c, if reached { "mode_count" } else { "spectrum_exhausted" })
        }
        (None, Some(f)) => {
            let (c, reached) = by_cutoff(f);
            (c, if reached { "frequency_cutoff" } else { "spectrum_exhausted" })
        }
        (Some(n), Some(f)) => {
            let (nc, nr) = by_count(n);
            let (fc, fr) = by_cutoff(f);
            let c = nc.min(fc);
            let stop = match (nr && nc == c, fr && fc == c) {
                (true, true) => "mode_count_and_frequency_cutoff",
                (true, false) => "mode_count",
                (false, true) => "frequency_cutoff",
                (false, false) => "spectrum_exhausted",
            };
            (c, stop)
        }
        (None, None) => return Err("no bound".into()),
    };
    let mut count = bounded;
    if count > 0 {
        let last = clusters[count - 1];
        while count < total && clusters[count] == last {
            count += 1;
        }
    }
    Ok((bounded, stop, count))
}

/// Counts: the reported set against its bound, its endpoint, the modes
/// against the computed ones, and f and T against ω.
fn mr_counts(p: &Parsed) -> R<Vec<String>> {
    let n = p.modes.len();
    let total = p.computed.len();
    if n != p.count || p.computed_count != total || n > total {
        return Err(format!("{n} modes and {total} computed modes against the stated count {} of {}", p.count, p.computed_count));
    }
    if p.extension != p.count - p.bounded.min(p.count) || p.bounded > p.count {
        return Err("the cluster extension is not the count beyond the bound".into());
    }
    let mut frequencies = Vec::with_capacity(total);
    let mut previous: Option<(f64, f64)> = None;
    for (i, c) in p.computed.iter().enumerate() {
        let label = i + 1;
        if c.reported != (i < n) || c.mode != (i < n).then_some(label) {
            return Err(format!("computed mode {label}: its reported flag and mode reference are not the reported set's"));
        }
        let omega = c.omega.ok_or_else(|| format!("computed mode {label}: ω not published"))?;
        let f = c.frequency.ok_or_else(|| format!("computed mode {label}: f not published"))?;
        if f != omega / TWO_PI {
            return Err(format!("computed mode {label}: f is not ω/2π"));
        }
        if let Some((po, pf)) = previous {
            if omega < po || f < pf {
                return Err(format!("computed mode {label}: not in ascending order"));
            }
        }
        previous = Some((omega, f));
        frequencies.push(f);
    }
    for (m, mode) in p.modes.iter().enumerate() {
        let c = &p.computed[m];
        if mode.omega.map(f64::to_bits) != c.omega.map(f64::to_bits) || mode.frequency.map(f64::to_bits) != c.frequency.map(f64::to_bits) || mode.cluster != c.cluster {
            return Err(format!("mode {}: not computed mode {}", m + 1, m + 1));
        }
        let f = mode.frequency.unwrap_or(f64::NAN);
        if mode.period != Some(1.0 / f) {
            return Err(format!("mode {}: T is not 1/f", m + 1));
        }
    }
    let ids: Vec<usize> = p.computed.iter().map(|c| c.cluster).collect();
    let (bounded, stop, count) = mr_reported_set(&frequencies, &ids, p.mode_count, p.cutoff)?;
    if bounded != p.bounded || stop != p.stopped_by || count != p.count {
        return Err(format!("the bound selects {bounded} (stopped by {stop}) and completes to {count}; the document states {} ({}) and {}",
            p.bounded, p.stopped_by, p.count));
    }
    let expected = if n == 0 {
        "below_lowest_mode"
    } else if n < total {
        "below_computed_cluster"
    } else {
        "whole_finite_spectrum"
    };
    if p.endpoint != expected {
        return Err(format!("endpoint {} is not {expected}", p.endpoint));
    }
    match expected {
        "whole_finite_spectrum" => {
            if p.next_computed.is_some() || p.established_endpoint != Some(total) || p.census_established != Some(total) {
                return Err("the whole finite spectrum is claimed only on an established census of every computed mode".into());
            }
        }
        _ => {
            if p.next_computed != Some(n + 1) || p.established_endpoint.is_some() {
                return Err("the set ends below the next computed mode".into());
            }
        }
    }
    if let Some(established) = p.census_established {
        if established != total {
            return Err("an established census counts every computed mode".into());
        }
    }
    if p.sturm_count != n || p.sturm_reported != n {
        return Err(format!("the Sturm count {} and its reported {} are not the {n} modes reported", p.sturm_count, p.sturm_reported));
    }
    Ok(Vec::new())
}

/// Clusters: chained ids, the rows, the boundary, δ_c from ω, e_λ.
fn mr_clusters(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    let delta = mr_param(p, "cluster_tolerance").ok_or("δ_c: not bound")?;
    let total = p.computed.len();
    for (i, c) in p.computed.iter().enumerate() {
        let ok = if i == 0 { c.cluster == 1 } else { c.cluster == p.computed[i - 1].cluster || c.cluster == p.computed[i - 1].cluster + 1 };
        if !ok {
            return Err(format!("computed mode {}: cluster ids are not chained from 1", i + 1));
        }
    }
    let highest = p.computed.last().map_or(0, |c| c.cluster);
    if p.clusters.len() != highest {
        return Err(format!("{} cluster rows for {highest} clusters", p.clusters.len()));
    }
    for (c, &(first, last, reported, estimate)) in p.clusters.iter().enumerate() {
        let id = c + 1;
        let members: Vec<usize> = (0..total).filter(|&i| p.computed[i].cluster == id).collect();
        if members.first().map(|i| i + 1) != Some(first) || members.last().map(|i| i + 1) != Some(last) {
            return Err(format!("cluster {id}: its first and last computed modes are not its members'"));
        }
        if reported != (first <= p.modes.len()) {
            return Err(format!("cluster {id}: its reported flag is not the set's"));
        }
        let mut max = 0.0f64;
        for &i in &members {
            let e = p.computed[i].estimate.ok_or_else(|| format!("computed mode {}: e_λ not published", i + 1))?;
            if e > max {
                max = e;
            }
        }
        if estimate != Some(max) {
            return Err(format!("cluster {id}: its e_λ is not its members' largest"));
        }
    }
    for (m, mode) in p.modes.iter().enumerate() {
        if mode.cluster_estimate != p.clusters.get(mode.cluster.wrapping_sub(1)).and_then(|c| c.3) {
            return Err(format!("mode {}: its cluster e_λ is not its cluster's", m + 1));
        }
    }
    let n = p.modes.len();
    if n > 0 && n < total && p.computed[n - 1].cluster == p.computed[n].cluster {
        return Err("the reported set ends inside a cluster".into());
    }
    let mut undetermined = 0usize;
    for i in 1..total {
        let (a, b) = (p.computed[i - 1].omega.unwrap_or(f64::NAN), p.computed[i].omega.unwrap_or(f64::NAN));
        let gap = b * b - a * a;
        let threshold = delta * (b * b);
        let margin = 8.0 * U * (b * b);
        let same = p.computed[i].cluster == p.computed[i - 1].cluster;
        if gap > threshold + margin && same {
            return Err(format!("computed modes {} and {}: share a cluster across a relative gap above δ_c", i, i + 1));
        }
        if gap < threshold - margin && !same {
            return Err(format!("computed modes {} and {}: split within δ_c", i, i + 1));
        }
        if !(gap > threshold + margin || gap < threshold - margin) {
            undetermined += 1;
        }
    }
    if undetermined > 0 {
        scope.push(format!("{undetermined} adjacent pairs lie within ω's rounding of the δ_c boundary; λ is not published"));
    }
    Ok(scope)
}

/// Sturm: the candidates, the accepted shift's premises and placement.
fn mr_sturm(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    let s_max = p.params.get("sturm_max_candidates").and_then(Value::as_u64).ok_or("s_max: not bound")? as usize;
    let kc = p.kappa_c.ok_or("κ_c: not published")?;
    let limit = p.limit.ok_or("1/κ_c: not published")?;
    let ks = p.kappa_s.ok_or("κ_s: not published")?;
    let g_max = p.growth_limit.ok_or("g_max: not published")?;
    if limit != 1.0 / kc {
        return Err("the conditioning limit is not 1/κ_c".into());
    }
    let count = p.candidates.len();
    if count == 0 || count > s_max {
        return Err(format!("{count} candidates against s_max = {s_max}"));
    }
    for (j, c) in p.candidates.iter().enumerate() {
        let label = j + 1;
        let last = j + 1 == count;
        if (c.decision == "accepted") != last {
            return Err(format!("candidate {label}: only the last candidate is accepted"));
        }
        let factored = c.pivots.is_some();
        let separated = match c.separation.value {
            None if c.separation.undefined.as_deref() == Some("no_neighbour") => Some(true),
            None => None,
            Some(r) => Some(r >= ks * (1.0 - 8.0 * U)),
        };
        match c.decision.as_str() {
            "accepted" | "unseparated" | "growth_limit" | "ill_conditioned" if !factored && c.decision != "unseparated" => {
                return Err(format!("candidate {label}: decision `{}` needs a factorization", c.decision));
            }
            _ => {}
        }
        if factored {
            let (Some(be), Some(ft), Some(fo)) = (c.backward_error, c.factor_term, c.formation_term) else {
                scope.push(format!("candidate {label}: a backward-error term is outside the profile"));
                continue;
            };
            if be != ft + fo {
                return Err(format!("candidate {label}: e_b is not the factor term plus the formation term"));
            }
            match (c.product, c.inverse_norm) {
                (Some(product), Some(nu)) if product != be * nu => {
                    return Err(format!("candidate {label}: the conditioning product is not e_b·ν"));
                }
                (Some(_), Some(_)) => {}
                _ => scope.push(format!("candidate {label}: e_b·ν is outside the profile")),
            }
        }
        if c.decision == "accepted" {
            if c.pivots != Some(p.sturm_count) {
                return Err("the accepted candidate's negative-pivot count is not the Sturm count".into());
            }
            let product = c.product.ok_or("the accepted e_b·ν is not published")?;
            let growth = c.growth.ok_or("the accepted growth is not published")?;
            if product > limit {
                return Err("the accepted shift fails the conditioning premise e_b·ν ≤ 1/κ_c".into());
            }
            if growth > g_max {
                return Err("the accepted shift exceeds the growth limit".into());
            }
            if separated != Some(true) {
                return Err("the accepted shift fails the separation premise".into());
            }
        }
        if factored {
            match c.decision.as_str() {
                "growth_limit" if !(c.growth.map_or(true, |g| g > g_max) && separated != Some(false)) => {
                    return Err(format!("candidate {label}: refused for growth within the limit"));
                }
                "ill_conditioned" if !(c.product.map_or(true, |x| x > limit) && c.growth.map_or(false, |g| g <= g_max)) => {
                    return Err(format!("candidate {label}: refused as ill-conditioned within the premise"));
                }
                "unseparated" if separated == Some(true) && c.separation.value.map_or(false, |r| r >= ks * (1.0 + 8.0 * U)) => {
                    return Err(format!("candidate {label}: refused as unseparated with its separation above κ_s"));
                }
                _ => {}
            }
        }
    }
    let n = p.modes.len();
    let omega = |i: usize| p.computed.get(i).and_then(|c| c.omega);
    let accepted = p.candidates.last().and_then(|c| c.shift);
    match p.endpoint.as_str() {
        "below_computed_cluster" | "below_lowest_mode" => {
            if p.placement != "gap" || p.placement_established.is_some() {
                return Err("a set below a computed cluster is counted in a gap".into());
            }
            let upper = p.placement_upper.ok_or("the gap's upper end is not published")?;
            if Some(upper.to_bits()) != omega(n).map(f64::to_bits) {
                return Err("the gap's upper end is not the next computed mode's ω".into());
            }
            let lower = p.placement_lower.ok_or("the gap's lower end is not published")?;
            if n > 0 {
                if Some(lower.to_bits()) != omega(n - 1).map(f64::to_bits) {
                    return Err("the gap's lower end is not the last reported mode's ω".into());
                }
            } else {
                let cut = p.cutoff.ok_or("an empty set has a cut-off")?;
                let expected = TWO_PI * cut;
                if !mr_tol_close(lower, expected, 8.0 * U * expected) {
                    return Err("the gap's lower end is not 2π·f_c".into());
                }
            }
            if let Some(shift) = accepted {
                if !(lower <= shift && shift <= upper) {
                    return Err("the accepted shift is not in its gap".into());
                }
            }
        }
        _ => {
            if p.placement != "above_spectrum" || p.placement_upper.is_some() || p.placement_established != Some(n) {
                return Err("the whole spectrum is counted above it, on its established count".into());
            }
            let lower = p.placement_lower.ok_or("the top eigenvalue is not published")?;
            if n == 0 || Some(lower.to_bits()) != omega(n - 1).map(f64::to_bits) {
                return Err("the placement's top is not the top mode's ω".into());
            }
            if let Some(shift) = accepted {
                if shift < lower {
                    return Err("the accepted whole-spectrum shift is below the top mode".into());
                }
            }
        }
    }
    if accepted.is_none() {
        scope.push("the accepted shift is outside the profile".to_string());
    }
    Ok(scope)
}

/// Evidence: e_λ = η·κ_λ, the identity discrepancies, the formation values.
fn mr_evidence(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    for (i, c) in p.computed.iter().enumerate() {
        let (Some(eta), Some(kappa), Some(e)) = (c.eta, c.condition, c.estimate) else {
            scope.push(format!("computed mode {}: a check-2 value is outside the profile", i + 1));
            continue;
        };
        if eta < 0.0 || kappa < 0.0 || e < 0.0 {
            return Err(format!("computed mode {}: a negative check-2 value", i + 1));
        }
        let product = eta * kappa;
        if !mr_tol_close(e, product, 8.0 * U * e.max(product)) {
            return Err(format!("computed mode {}: e_λ = {e:e} is not η·κ_λ = {product:e}", i + 1));
        }
    }
    let tolerance = p.params.get("identity_tolerance");
    let relative = tolerance.and_then(|t| t["relative"].as_f64()).ok_or("the identity tolerance is not bound")?;
    let absolute = tolerance.and_then(|t| t["absolute"].as_f64()).ok_or("the identity tolerance is not bound")?;
    if absolute == 0.0 {
        for (i, d) in p.identity.iter().enumerate() {
            let d = d.ok_or_else(|| format!("identity row {i}: not published"))?;
            if !(d >= 0.0 && d <= relative) {
                return Err(format!("identity row {i}: relative discrepancy {d:e} exceeds the bound tolerance {relative:e}"));
            }
        }
    } else {
        scope.push("an absolute identity tolerance: its scale is not published".to_string());
    }
    for v in p.formation.iter().flatten() {
        if *v < 0.0 {
            return Err("a negative formation-difference or norm".into());
        }
    }
    if let Some(ortho) = p.ortho {
        if ortho < 0.0 {
            return Err("a negative orthogonality deviation".into());
        }
    }
    if p.ortho_pair != (p.modes.len() >= 2) {
        return Err("the orthogonality pair is stated exactly when two modes are reported".into());
    }
    Ok(scope)
}

/// Standing: each mode Sensitive exactly when η > τ_r, the set exactly when
/// a mode is or the orthogonality deviation exceeds τ_o.
fn mr_standing(p: &Parsed) -> R<Vec<String>> {
    let tau_r = mr_param(p, "residual_threshold").ok_or("τ_r: not bound")?;
    let tau_o = mr_param(p, "orthogonality_threshold").ok_or("τ_o: not bound")?;
    let mut any = false;
    for (m, mode) in p.modes.iter().enumerate() {
        let eta = p.computed[m].eta.ok_or_else(|| format!("mode {}: η not published", m + 1))?;
        let sensitive = eta > tau_r;
        if mode.sensitive != sensitive || (mode.reasons > 0) != sensitive {
            return Err(format!("mode {}: Sensitive exactly when η > τ_r, with its reason", m + 1));
        }
        any |= sensitive;
    }
    let ortho = p.ortho.ok_or("the orthogonality deviation is not published")?;
    let sensitive = any || ortho > tau_o;
    if p.sensitive != sensitive || (p.set_reasons > 0) != sensitive {
        return Err("the set is Sensitive exactly when a mode is or the orthogonality deviation exceeds τ_o".into());
    }
    Ok(Vec::new())
}

/// Units: every value object against modal-1.
fn mr_units(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    for u in &p.unitized {
        mr_unit_check(&u.vo, &u.key, &u.path, &mut scope)?;
    }
    Ok(scope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_readers_table_names_canonical_catalog_units() {
        assert_eq!(modal_reader_catalog_differences(), Vec::<String>::new());
    }
}
