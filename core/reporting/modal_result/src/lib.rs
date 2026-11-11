//! The Rust reader of the modal result family `openpipestress.modal_result`
//! 0.1.0, under the semantic table
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
//! - `evidence`: e_λ = η·κ_λ, the census, the identity discrepancies and
//!   the subject census (`census`) against the identity, discretization and
//!   supports tables;
//! - `standing`: the Sensitive standing re-derived from η, τ_r and τ_o;
//! - `units`: every value's unit against modal-1, and every
//!   `equilibrated_basis` value's kind against modal-1's closed list.
//!
//! With the bound request supplied, `mass` and `participation` are also held
//! to the mass the request's model implies (`model_mass`): the full global M
//! entry by entry, T_k, the free-DOF mass, the mass centre, m_i, Γ, m_eff and
//! the fractions from the published shapes. Without it, or beyond that
//! derivation's basis, both standings are scoped unsupported by name. With
//! it, `evidence` also re-derives the subject list from the model and holds
//! the census's counts, its digest and every table's subjects to it; without
//! it, the identities are not re-derived and `evidence` stays scoped.
//!
//! Each standing is passed, failed (with the first difference), scoped
//! unsupported (with its scope) or not checked. A withheld authoritative
//! payload is scoped unsupported: nothing is reconstructed from a digest.
//! A relation that the published binary64 values cannot decide (a cluster
//! boundary within ω's rounding) is scoped unsupported, never guessed.
//!
//! **Nulls are bounds.** A value null `outside_canonical_profile` or
//! `not_finite` is at least 2⁵³; every relation consuming it is decided on
//! intervals: exact when every operand is published, failed when the bounds
//! are disjoint from the published values, scoped only when they overlap. A
//! null whose reason states no size fails where a relation needs the value,
//! unless the reason is that relation's own case (`no_neighbour`,
//! `total_mass_zero`).
//!
//! **Admission.** Public engineering-result admission is disabled: the
//! verdict states it for every document, with the open conditions.
//!
//! **Placement**: a leaf crate of its own, as the hanger-sizing
//! family's reader is. The modal family is not the `results.v0.3` envelope
//! that `result_export` reads, and the reader is reached from no retained
//! root, so it is kept out of that crate's priced surface. It depends on no
//! product crate.
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_units::{canonical_unit, catalog, Dimension};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

mod census;
mod model_mass;
mod working_bound;

pub use working_bound::{modal_reader_working_bound, modal_reader_working_terms};

pub const MODAL_RESULT_FAMILY: &str = "openpipestress.modal_result";
pub const MODAL_RESULT_SCHEMA_VERSION: &str = "0.1.0";
pub const MODAL_RESULT_SEMANTICS_ID: &str = "openpipestress.result_semantics/modal-1";
/// The SHA-256 of modal-1's canonical text (`fixtures/results/semantic_contract_modal_1.json`).
pub const MODAL_RESULT_SEMANTICS_SHA256: &str = "21d0012729d8466436d0c064d7edecbdc0fc481bf101b6514a65eecbee6fb23b";
pub const MODAL_RESULT_PROFILE_ID: &str = "product_preview_modal_v1";
pub const MODAL_RESULT_CANONICAL_PROFILE: &str = "openpipestress_jcs_ijson_v1";
pub const MODAL_RESULT_LIMITATIONS: [&str; 2] = [
    "Small-displacement, linear-elastic Euler-Bernoulli frame modal preview about the unloaded configuration, with any nonlinear supports at the stated state of a named solved case, with consistent mass on straight members and lumped rigid-body mass on realized arcs and component bodies; numerical integrity does not establish physical correctness.",
    "The integrity checks are numerical estimates under stated premises, not mathematical bounds: each mode's residual is evaluated against a double-double re-formation of the stiffness and mass, its eigenvalue error is a first-order estimate, and the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues. They do not bound modelling, discretization or input error.",
];
/// Why public engineering-result admission stays disabled.
pub const MODAL_RESULT_ADMISSION_CONDITIONS: [&str; 6] = [
    "production integrity parameters are not pinned (O-6, D-TOL): every document is formed with a labelled test-local set",
    "a document is fully validated only when every mandatory relation passes: an unresolved Bessel relation (interim contract), a subject census not re-derived from the bound request, or a partition from reported nonlinear support states leaves it unmet, whatever the units",
    "the dense work budget is unmeasured and no work guard is applied (O-4)",
    "no native, worker or custody path forms, stores or transports the document",
    "no published piping benchmark is compared (FR-SOL-016 partial; O-3 held)",
    "the reader re-derives the mass from the bound request's model, but not the stiffness, the re-formation, the eigen solve or the nonlinear support states' qualification",
];

const U: f64 = 1.1102230246251565e-16;
/// The reader rounding allowance multiple of every tolerance.
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
}

/// The unit statuses a modal-1 value may carry: `catalog`, and
/// `equilibrated_basis` for a kind in the closed list. The former
/// `catalog_family_absent` is no longer one (every kind has its family).
const UNIT_STATUSES: [&str; 2] = ["catalog", "equilibrated_basis"];

/// modal-1's closed list of the kinds that may carry `equilibrated_basis`:
/// a coordinate-basis magnitude whose exponents mix, claiming no dimension
/// and no physical unit (it is not dimensionless). Design §5.3.5's trace
/// leaves it empty; a value with that status whose kind is not listed fails
/// units.
const EQUILIBRATED_BASIS_KINDS: [&str; 0] = [];

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
const R_KGM_RAD: Rule = Rule::Catalog("unit:kilogram_meter_per_radian", "kg*m/rad", "mass_length_per_rotation");
/// A magnitude of D·M·D, D = diag(|k_ii|^-½) of the formed K_ff: s².
const R_S2: Rule = Rule::Catalog("unit:square_second", "s^2", "time_squared");

/// modal-1, as this reader states it: (kind, table, selector, rules).
const KINDS: &[(&str, &str, &str, &[(&str, Rule)])] = &[
    ("node_coordinate", "nodes", "fixed", &[("fixed", R_M)]),
    ("mass_matrix_entry", "mass_matrix", "dof_pair_class", &[
        ("translation_translation", R_KG), ("translation_rotation", R_KGM_RAD), ("rotation_rotation", R_KGM2)]),
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
    ("sturm_formation_term", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_formation_separation_ratio", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_factor_term", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_backward_error", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_inverse_norm_estimate", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_conditioning_product", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_separation_ratio", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_growth_factor", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_max_abs_lower_factor", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_max_abs_pivot", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
    ("sturm_max_abs_scaled_diagonal", "sturm_candidates", "fixed", &[("fixed", R_ONE)]),
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
    ("stiffness_formation_difference", "header", "fixed", &[("fixed", R_ONE)]),
    ("mass_formation_difference", "header", "fixed", &[("fixed", R_S2)]),
    ("reformed_stiffness_norm", "header", "fixed", &[("fixed", R_ONE)]),
    ("reformed_mass_norm", "header", "fixed", &[("fixed", R_S2)]),
    ("section_derivation_difference", "header", "fixed", &[("fixed", R_ONE)]),
];

fn mr_rule_json(rule: Rule) -> Value {
    match rule {
        Rule::Catalog(id, symbol, dimension) => {
            json!({"unit_status": "catalog", "unit": id, "symbol": symbol, "dimension": dimension})
        }
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
        "canonical_profile": MODAL_RESULT_CANONICAL_PROFILE, "equilibrated_basis_kinds": EQUILIBRATED_BASIS_KINDS, "kinds": kinds})
}

/// Each modal-1 catalog rule against the unit catalog: the id exists, is
/// the canonical unit of its stated dimension and has the stated symbol.
pub fn modal_reader_catalog_differences() -> Vec<String> {
    let mut out = Vec::new();
    for (kind, _, _, rules) in KINDS {
        for (key, rule) in *rules {
            let Rule::Catalog(id, symbol, dimension) = rule;
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
    mr_sha256_text(&serde_json::to_string(value).ok()?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalReadStatus {
    Passed,
    Failed,
    /// Consistent with its operands' stated sets but not decided by them:
    /// never passed.
    Unresolved,
    ScopedUnsupported,
    NotChecked,
}

impl ModalReadStatus {
    pub fn modal_status_token(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Unresolved => "unresolved",
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
    /// The instances left unresolved by their operands' stated sets.
    pub unresolved: Vec<String>,
}

impl ModalReadStanding {
    fn mr_unchecked() -> Self {
        Self { status: ModalReadStatus::NotChecked, difference: None, scope: Vec::new(), unresolved: Vec::new() }
    }
    fn mr_passed() -> Self {
        Self { status: ModalReadStatus::Passed, difference: None, scope: Vec::new(), unresolved: Vec::new() }
    }
    fn mr_failed(difference: String) -> Self {
        Self { status: ModalReadStatus::Failed, difference: Some(difference), scope: Vec::new(), unresolved: Vec::new() }
    }
    /// Failed (the first difference); unresolved, when an instance is left
    /// open by its operands' sets; scoped, when evidence is withheld or
    /// outside the catalog; else passed.
    fn mr_of(result: Result<Vec<String>, String>) -> Self {
        match result {
            Err(difference) => Self::mr_failed(difference),
            Ok(items) => {
                let (unresolved, scope): (Vec<String>, Vec<String>) = items.into_iter().partition(|x| x.starts_with(MR_UNRESOLVED));
                let unresolved: Vec<String> = unresolved.into_iter().map(|x| x[MR_UNRESOLVED.len()..].to_string()).collect();
                let status = if !unresolved.is_empty() {
                    ModalReadStatus::Unresolved
                } else if !scope.is_empty() {
                    ModalReadStatus::ScopedUnsupported
                } else {
                    ModalReadStatus::Passed
                };
                Self { status, difference: None, scope, unresolved }
            }
        }
    }
    fn mr_json(&self) -> Value {
        json!({"status": self.status.modal_status_token(), "difference": self.difference, "scope": self.scope, "unresolved": self.unresolved})
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
    /// `consistent`, `consistent_with_unresolved`,
    /// `consistent_with_scoped_unsupported`, `refusal_consistent` or
    /// `rejected`.
    pub decision: &'static str,
}

/// What every verdict establishes, and no more.
pub const MODAL_VERDICT_LIMIT: &str = "per-relation consistency, not joint realizability: each relation is decided on its own operands, so a document every relation admits may still have no single coherent set of underlying values; feasibility-oracle agreement does not establish payload coherence either";

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
            "decision": self.decision, "validation": {"status": self.modal_validation(), "unmet": self.modal_unmet()},
            "limit": MODAL_VERDICT_LIMIT, "public_admission": {"status": "disabled", "conditions": MODAL_RESULT_ADMISSION_CONDITIONS}})
    }

    /// The relations that have not passed: every one is mandatory.
    pub fn modal_unmet(&self) -> Vec<&'static str> {
        MODAL_RELATIONS.iter().zip(&self.relations).filter(|(_, r)| r.status != ModalReadStatus::Passed).map(|(n, _)| *n).collect()
    }

    /// `fully_validated` only when every mandatory governing relation has
    /// passed: unresolved or scoped evidence confers no validated or
    /// current-authority status. Otherwise `not_fully_validated`.
    pub fn modal_validation(&self) -> &'static str {
        if self.decision == "consistent" && self.modal_unmet().is_empty() {
            "fully_validated"
        } else {
            "not_fully_validated"
        }
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

/// One parsed value object: what the relations read of it. Its kind and unit
/// are read again from the document by the units relation ([`Unitized`]), so
/// nothing of the document is copied.
#[derive(Debug, Clone, Copy)]
struct Vo<'v> {
    value: Option<f64>,
    below_normal: bool,
    undefined: Option<&'v str>,
}

fn mr_vo<'v>(value: &'v Value, path: &str, kinds: &[&str]) -> R<Vo<'v>> {
    let map = value.as_object().ok_or_else(|| format!("{path}: not a value object"))?;
    let status = mr_text(map.get("unit_status").unwrap_or(&Value::Null), &mr_at(path, "unit_status"))?;
    let mut keys = vec!["kind", "value", "unit", "unit_status", "below_normal", "undefined_reason"];
    if status == "catalog_family_absent" {
        keys.push("dimension_exponents");
    }
    mr_object(value, path, &keys)?;
    let kind = mr_text(&map["kind"], &mr_at(path, "kind"))?;
    if !kinds.contains(&kind) {
        return Err(format!("{path}: kind `{kind}` is not {kinds:?}"));
    }
    let undefined = match &map["undefined_reason"] {
        Value::Null => None,
        other => Some(mr_text(other, &mr_at(path, "undefined_reason"))?),
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
    if !map["unit"].is_null() {
        mr_text(&map["unit"], &mr_at(path, "unit"))?;
    }
    if status == "catalog_family_absent" {
        let e = mr_object(&map["dimension_exponents"], &mr_at(path, "dimension_exponents"), &["length", "mass", "time", "temperature", "angle"])?;
        let read = |k: &str| e[k].as_i64().ok_or_else(|| format!("{path}/dimension_exponents/{k}: not an integer"));
        for k in ["length", "mass", "time", "temperature", "angle"] {
            read(k)?;
        }
    }
    Ok(Vo { value: number, below_normal, undefined })
}

/// A published table's rows, or `None` when withheld.
struct Table<'v> {
    rows: Option<&'v [Value]>,
}

fn mr_rule_of(kind: &str, key: &str) -> Option<Rule> {
    KINDS.iter().find(|(k, ..)| *k == kind).and_then(|(.., rules)| rules.iter().find(|(k, _)| *k == key)).map(|(_, r)| *r)
}

/// Units: one value object against its rule (`key` the selector key). The
/// value object passed closure ([`mr_vo`]), so its kind and unit status are
/// strings and its unit a string or null; it carries dimension exponents
/// exactly when its status is `catalog_family_absent`.
fn mr_unit_check(vo: &Value, key: &str, path: &MrPath) -> R<()> {
    let kind = vo["kind"].as_str().unwrap_or("");
    let unit_status = vo["unit_status"].as_str().unwrap_or("");
    let unit = vo["unit"].as_str();
    if !UNIT_STATUSES.contains(&unit_status) {
        return Err(format!("{path}: `{unit_status}` is not a modal-1 unit status"));
    }
    if unit_status == "equilibrated_basis" && !EQUILIBRATED_BASIS_KINDS.contains(&kind) {
        return Err(format!("{path}: `{kind}` is not in modal-1's closed equilibrated_basis list"));
    }
    let rule = mr_rule_of(kind, key).ok_or_else(|| format!("{path}: modal-1 has no `{key}` rule for `{kind}`"))?;
    let Rule::Catalog(id, ..) = rule;
    if unit_status != "catalog" || unit != Some(id) || unit_status == "catalog_family_absent" {
        return Err(format!("{path}: unit {unit:?} ({unit_status}) is not modal-1's {id}"));
    }
    Ok(())
}

/// Reads a document text: the governed entry. The original text is first
/// admitted by the checked profile `openpipestress_jcs_ijson_v1`
/// (`canonical_json_checked_v1_text`: duplicate object members, non-finite
/// numbers and integers or integral values beyond 2⁵³ − 1 are refused), so
/// nothing a lossy map would collapse reaches the reader.
pub fn read_modal_result_text(text: &str, supplied: ModalSupplied<'_>) -> ModalReadVerdict {
    if let Err(error) = canonical_json_checked_v1_text(text) {
        return mr_closure_failed(format!("document: outside the checked profile {MODAL_RESULT_CANONICAL_PROFILE} ({error})"));
    }
    match serde_json::from_str::<Value>(text) {
        Ok(doc) => read_modal_result(&doc, supplied),
        Err(error) => mr_closure_failed(format!("document: not JSON ({error})")),
    }
}

fn mr_closure_failed(difference: String) -> ModalReadVerdict {
    ModalReadVerdict {
        closure: ModalReadStanding::mr_failed(difference),
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

/// Reads an already parsed document (module docs). A parsed value has lost
/// any duplicate member, so a caller holding the text uses
/// [`read_modal_result_text`].
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
    // The model-derived mass (module docs), from the supplied request only
    // when it is the bound one.
    let bound = binding.status != ModalReadStatus::Failed;
    let model = model_mass::mm_derive(&parsed, supplied.request, bound);
    let relations = vec![
        ModalReadStanding::mr_of(mr_mass(&parsed).and_then(|items| model_mass::mm_mass_relation(&parsed, &model, items))),
        ModalReadStanding::mr_of(mr_normalization(&parsed)),
        ModalReadStanding::mr_of(mr_participation(&parsed).and_then(|items| model_mass::mm_participation_relation(&parsed, &model, items))),
        ModalReadStanding::mr_of(mr_counts(&parsed)),
        ModalReadStanding::mr_of(mr_clusters(&parsed)),
        ModalReadStanding::mr_of(mr_sturm(&parsed)),
        ModalReadStanding::mr_of(mr_evidence(&parsed).and_then(|items| census::cs_relation(&parsed, supplied.request, bound, &model, items))),
        ModalReadStanding::mr_of(mr_standing(&parsed)),
        ModalReadStanding::mr_of(mr_units(&parsed)),
    ];
    let failed = binding.status == ModalReadStatus::Failed || relations.iter().any(|r| r.status == ModalReadStatus::Failed);
    let unresolved = relations.iter().any(|r| r.status == ModalReadStatus::Unresolved);
    let scoped = relations.iter().any(|r| r.status == ModalReadStatus::ScopedUnsupported);
    let decision = if failed {
        "rejected"
    } else if unresolved {
        "consistent_with_unresolved"
    } else if scoped {
        "consistent_with_scoped_unsupported"
    } else {
        "consistent"
    };
    let health = if failed { "not_assessed" } else if parsed.sensitive { "sensitive" } else { "checks_passed" };
    ModalReadVerdict {
        closure: ModalReadStanding::mr_passed(),
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
    mr_binding_closure(&doc["binding"], doc["status"] != "refused")
}

/// The binding's shape; `solved` is whether the document is read as solved
/// (its subject census is then an object, else null).
fn mr_binding_closure(binding: &Value, solved: bool) -> R<()> {
    let b = mr_object(binding, "/binding", &[
        "request_sha256", "model_sha256", "materials_sha256", "materials_member", "options", "undeclared_members", "state_case",
        "integrity_parameters_sha256", "integrity_parameters", "subject_census",
    ])?;
    census::cs_shape(&b["subject_census"], solved)?;
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

/// SHA-256 (lowercase hex) of the checked canonical text of a JSON text.
fn mr_sha256_text(text: &str) -> Option<String> {
    let canonical = canonical_json_checked_v1_text(text).ok()?;
    Some(Sha256::digest(canonical.as_bytes()).iter().map(|b| format!("{b:02x}")).collect())
}

/// A row's checksum: [`modal_reader_sha256`] of the row less its `checksum`
/// member, serialized from a map of references (the same text a copy of the
/// row would serialize to), so the row itself is not copied.
fn mr_row_checksum(row: &Map<String, Value>) -> Option<String> {
    let body: std::collections::BTreeMap<&str, &Value> = row.iter().filter(|(k, _)| k.as_str() != "checksum").map(|(k, v)| (k.as_str(), v)).collect();
    mr_sha256_text(&serde_json::to_string(&body).ok()?)
}

/// A published or withheld table, its rows' checksums, digest, ordinals and
/// globally unique ids (closure). `table` is the `/tables/{name}` member.
fn mr_table<'v>(table: Option<&'v Value>, name: &str, ids: &mut HashSet<&'v str>) -> R<Table<'v>> {
    let path = format!("/tables/{name}");
    let table = table.ok_or_else(|| format!("{path}: missing"))?;
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
        if !ids.insert(id) {
            return Err(format!("{rpath}/row_id: `{id}` is not unique"));
        }
        let checksum = mr_text(map.get("checksum").unwrap_or(&Value::Null), &mr_at(&rpath, "checksum"))?;
        if mr_row_checksum(map).as_deref() != Some(checksum) {
            return Err(format!("{rpath}/checksum: not the row's checksum"));
        }
        checksums.push(checksum);
    }
    // The digest of the checksums' array: the same text as an array of string values.
    if serde_json::to_string(&checksums).ok().and_then(|text| mr_sha256_text(&text)).as_deref() != t["digest"].as_str() {
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

/// Where a value object is, for the units relation's difference: `head`,
/// then `/rows/{row}` when it is a table row's, then `/{field}`.
#[derive(Debug, Clone, Copy)]
struct MrPath {
    head: &'static str,
    row: Option<usize>,
    field: &'static str,
}

impl MrPath {
    fn mr_row(head: &'static str, row: usize, field: &'static str) -> Self {
        MrPath { head, row: Some(row), field }
    }
    fn mr_header(head: &'static str, field: &'static str) -> Self {
        MrPath { head, row: None, field }
    }
}

impl std::fmt::Display for MrPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.row {
            Some(row) => write!(f, "{}/rows/{row}/{}", self.head, self.field),
            None => write!(f, "{}/{}", self.head, self.field),
        }
    }
}

/// A value object that passed closure, kept for the units relation: the
/// document's own value object (read there by reference), its selector key
/// and where it is.
struct Unitized<'v> {
    value: &'v Value,
    key: &'static str,
    path: MrPath,
}

struct Mode<'v> {
    case: usize,
    reference: usize,
    sensitive: bool,
    reasons: usize,
    omega: Option<f64>,
    frequency: Option<f64>,
    period: Option<f64>,
    generalized_mass: Vo<'v>,
    cluster_estimate: Num<'v>,
    cluster: usize,
}

struct Participation<'v> {
    gamma: Vo<'v>,
    effective: Vo<'v>,
    fraction: Vo<'v>,
    cumulative: Vo<'v>,
}

struct Computed<'v> {
    cluster: usize,
    reported: bool,
    mode: Option<usize>,
    omega: Option<f64>,
    frequency: Option<f64>,
    eta: Num<'v>,
    condition: Num<'v>,
    estimate: Num<'v>,
}

struct Candidate<'v> {
    decision: &'v str,
    pivots: Option<usize>,
    shift: Num<'v>,
    formation_term: Num<'v>,
    /// Plain binary64 at the producer and never flagged; not re-derived (a stated
    /// limit), but its domain is enforced in the sturm relation like every other value.
    formation_separation: Num<'v>,
    factor_term: Num<'v>,
    backward_error: Num<'v>,
    inverse_norm: Num<'v>,
    product: Num<'v>,
    separation: Num<'v>,
    growth: Num<'v>,
    /// max|L|, max|d|, max|ã|: on a factored candidate, published or
    /// outside the profile.
    maxima: [Num<'v>; 3],
}

/// A published number with what its null states (Design §5.3.4's dependency
/// table): its value, its null reason (the document's string) and its
/// below-normal flag.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Num<'v> {
    value: Option<f64>,
    reason: Option<&'v str>,
    below: bool,
}

/// 2⁵³: the checked profile carries no integral binary64 this large.
const PROFILE_FLOOR: f64 = 9_007_199_254_740_992.0;
/// 2⁻¹⁰⁷⁴: the smallest subnormal binary64.
const SUB_LOW: f64 = 4.940_656_458_412_465_4e-324;

impl<'v> Num<'v> {
    fn mr_from(vo: &Vo<'v>) -> Self {
        Num { value: vo.value, reason: vo.undefined, below: vo.below_normal }
    }
}

/// The values a quantity can take.
/// `parts` are closed intervals of reals `(lo, hi, nonzero)`: an infinite end
/// means unbounded, not ±∞, and `nonzero` excludes 0 (the producer's flagged
/// 0, which is a nonzero value below 2⁻¹⁰²² with any extended exponent, or a
/// product of such). `wild` means the binary64 value can be ±∞ or NaN, either
/// stated (`not_finite`) or by overflow of a real beyond the largest binary64.
/// Arithmetic rounds outward, is exact at zero, and never turns a nonzero
/// underflow into 0.
#[derive(Debug, Clone)]
struct Reach {
    parts: Vec<(f64, f64, bool)>,
    wild: bool,
}

/// A real product: zero when a factor is zero, whatever the other end.
fn mr_real_mul(x: f64, y: f64) -> f64 {
    if x == 0.0 || y == 0.0 {
        0.0
    } else {
        x * y
    }
}

/// An interval holding the real product of two endpoints: 4u relative, and
/// the smallest subnormal when the product is below the normal range (an
/// underflow is never an exact 0).
fn mr_product_ends(x: f64, y: f64) -> (f64, f64) {
    let v = mr_real_mul(x, y);
    if v.is_infinite() || (v == 0.0 && (x == 0.0 || y == 0.0)) {
        return (v, v);
    }
    let slack = 4.0 * U * v.abs() + if v.abs() < f64::MIN_POSITIVE { SUB_LOW } else { 0.0 };
    (v - slack, v + slack)
}

/// +1 or −1 when every value of the part has that sign, else 0.
fn mr_part_sign(p: &(f64, f64, bool)) -> i32 {
    if p.0 > 0.0 || (p.0 == 0.0 && p.2) {
        1
    } else if p.1 < 0.0 || (p.1 == 0.0 && p.2) {
        -1
    } else {
        0
    }
}

/// Whether the part holds no 0.
fn mr_part_nonzero(p: &(f64, f64, bool)) -> bool {
    p.0 > 0.0 || p.1 < 0.0 || p.2
}

/// The sign class of a part's values, counting a closed 0 end as either.
fn mr_part_side(p: &(f64, f64, bool)) -> i32 {
    if p.0 >= 0.0 {
        1
    } else if p.1 <= 0.0 {
        -1
    } else {
        0
    }
}

impl Reach {
    fn mr_new(parts: Vec<(f64, f64, bool)>, mut wild: bool) -> Reach {
        let mut parts: Vec<(f64, f64, bool)> = parts.into_iter().map(|(lo, hi, nz)| (lo, hi, nz && lo <= 0.0 && 0.0 <= hi)).collect();
        parts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
        if parts.iter().any(|p| p.1 > f64::MAX || p.0 < -f64::MAX) {
            wild = true;
        }
        if parts.len() > 8 {
            let lo = parts.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
            let hi = parts.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
            parts = vec![(lo, hi, false)];
        }
        Reach { parts, wild }
    }
    fn mr_exact(x: f64) -> Reach {
        Reach { parts: vec![(x, x, false)], wild: false }
    }
    fn mr_point(&self) -> Option<f64> {
        match self.parts.as_slice() {
            [(lo, hi, _)] if !self.wild && lo == hi => Some(*lo),
            _ => None,
        }
    }
    fn mr_reals(&self) -> Reach {
        Reach { parts: self.parts.clone(), wild: false }
    }
    fn mr_lowest(&self) -> f64 {
        self.parts.iter().map(|p| p.0).fold(f64::INFINITY, f64::min)
    }
    fn mr_highest(&self) -> f64 {
        self.parts.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max)
    }
    /// Each part grown by `relative` times its ends' magnitudes (0 stays 0).
    fn mr_slack(&self, relative: f64) -> Reach {
        Reach::mr_new(self.parts.iter().map(|&(lo, hi, nz)| (lo - mr_real_mul(relative, lo.abs()), hi + mr_real_mul(relative, hi.abs()), nz)).collect(), self.wild)
    }
    fn mr_squared(&self) -> Reach {
        if self.mr_point().is_some() {
            return mr_join(self, self, true);
        }
        let parts = self
            .parts
            .iter()
            .map(|p| {
                let (a, b) = (mr_product_ends(p.0, p.0), mr_product_ends(p.1, p.1));
                let bottom = if p.0 < 0.0 && 0.0 < p.1 { 0.0 } else { a.0.min(b.0).max(0.0) };
                (bottom, a.1.max(b.1), mr_part_nonzero(p))
            })
            .collect();
        Reach::mr_new(parts, self.wild)
    }
    /// Whether the producer's threshold test can come out true and false: each
    /// part contributes its ends; a non-finite value contributes `wild`.
    fn mr_can(&self, test: impl Fn(f64) -> bool, wild: (bool, bool)) -> (bool, bool) {
        let (mut yes, mut no) = if self.wild { wild } else { (false, false) };
        for &(lo, hi, _) in &self.parts {
            for v in [lo, hi] {
                if test(v) {
                    yes = true;
                } else {
                    no = true;
                }
            }
        }
        (yes, no)
    }
}

/// The values a value object states, or `None` for a null whose reason is a
/// claim (the relation consuming it checks the claim).
fn mr_reach(n: &Num, signed: bool) -> Option<Reach> {
    let span = |lo: f64, hi: f64, nz: bool| if signed { vec![(lo, hi, nz), (-hi, -lo, nz)] } else { vec![(lo, hi, nz)] };
    match (n.value, n.reason.as_deref()) {
        // The producer's flagged 0: nonzero, below 2⁻¹⁰²², unbounded below.
        (Some(_), _) if n.below => Some(Reach::mr_new(span(0.0, f64::MIN_POSITIVE, true), false)),
        (Some(v), _) => Some(Reach::mr_exact(v)),
        (None, Some("outside_canonical_profile")) => Some(Reach::mr_new(span(PROFILE_FLOOR, f64::MAX, false), false)),
        (None, Some("not_finite")) => Some(Reach { parts: Vec::new(), wild: true }),
        _ => None,
    }
}

/// The value the producer itself carries forward (the shape into m and φᵀp_k,
/// a fraction into its cumulative sum): the published number, a flagged 0
/// being 0; a null outside the profile or not finite as its set.
fn mr_published(n: &Num, signed: bool) -> Option<Reach> {
    match n.value {
        Some(v) => Some(Reach::mr_exact(v)),
        None => mr_reach(n, signed),
    }
}

/// The producer's rounding rule a field is published under.
#[derive(Clone, Copy, PartialEq)]
enum MrRule {
    /// `WideDd::to_published` (dd.rs:476): 0, normal, or a flagged nonzero
    /// below 2⁻¹⁰²².
    Published,
    /// `to_normal_or_zero` (dd.rs:507): 0 or normal, never flagged.
    NormalOrZero,
    /// A plain binary64 value: never flagged.
    Binary64,
}

/// A value outside its field's producer domain is no producer output.
fn mr_domain(n: &Num, rule: MrRule, what: &str) -> R<()> {
    let Some(v) = n.value else { return Ok(()) };
    let subnormal = v != 0.0 && v.abs() < f64::MIN_POSITIVE;
    match rule {
        MrRule::NormalOrZero if n.below || subnormal => Err(format!(
            "{what}: {}, but the producer publishes it only as 0 or normal (to_normal_or_zero)",
            if n.below { "flagged below-normal" } else { "subnormal" }
        )),
        MrRule::Published if !n.below && subnormal => {
            Err(format!("{what}: an unflagged subnormal, but the producer flags every nonzero value below 2⁻¹⁰²² (to_published)"))
        }
        MrRule::Binary64 if n.below => Err(format!("{what}: flagged below-normal, but the producer never flags this field")),
        _ => Ok(()),
    }
}

fn mr_need(n: &Num, signed: bool, what: &str) -> R<Reach> {
    mr_reach(n, signed)
        .ok_or_else(|| format!("{what}: null, and its reason `{}` states no value where one is needed", n.reason.as_deref().unwrap_or("")))
}

/// a + b or a·b. Two published points combine as the producer's binary64
/// operation does (its rounding is the relations' allowance), except that a
/// nonzero product below the normal range stays nonzero; a real result beyond
/// the largest binary64 stays real and marks a possible overflow.
fn mr_join(a: &Reach, b: &Reach, product: bool) -> Reach {
    if let (Some(x), Some(y)) = (a.mr_point(), b.mr_point()) {
        let v = if product { x * y } else { x + y };
        if v.is_infinite() {
            return Reach::mr_new(vec![if v > 0.0 { (f64::MAX, f64::INFINITY, false) } else { (f64::NEG_INFINITY, -f64::MAX, false) }], true);
        }
        if product && x != 0.0 && y != 0.0 && v.abs() < f64::MIN_POSITIVE {
            // Rounded within one subnormal step, and nonzero.
            let (bottom, top) = ((v.abs() - SUB_LOW).max(0.0), v.abs() + SUB_LOW);
            let part = if (x > 0.0) == (y > 0.0) { (bottom, top, true) } else { (-top, -bottom, true) };
            return Reach::mr_new(vec![part], false);
        }
        return Reach::mr_exact(v);
    }
    let mut parts = Vec::new();
    for p in &a.parts {
        for q in &b.parts {
            if product {
                let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
                for x in [p.0, p.1] {
                    for y in [q.0, q.1] {
                        let (l, h) = mr_product_ends(x, y);
                        lo = lo.min(l);
                        hi = hi.max(h);
                    }
                }
                match mr_part_sign(p) * mr_part_sign(q) {
                    1 => lo = lo.max(0.0),
                    -1 => hi = hi.min(0.0),
                    _ => {}
                }
                parts.push((lo, hi, mr_part_nonzero(p) && mr_part_nonzero(q)));
            } else {
                let (lo, hi) = (p.0 + q.0, p.1 + q.1);
                let lo = if lo.is_finite() { lo - 4.0 * U * lo.abs() } else { lo };
                let hi = if hi.is_finite() { hi + 4.0 * U * hi.abs() } else { hi };
                // Same-signed parts, one of them nonzero, have a nonzero sum.
                let (sp, sq) = (mr_part_side(p), mr_part_side(q));
                parts.push((lo, hi, sp == sq && sp != 0 && (mr_part_nonzero(p) || mr_part_nonzero(q))));
            }
        }
    }
    // A non-finite operand gives a non-finite result (0·∞ is NaN).
    Reach::mr_new(parts, a.wild || b.wild)
}

fn mr_sum(items: impl IntoIterator<Item = Reach>) -> Reach {
    items.into_iter().fold(Reach::mr_exact(0.0), |acc, x| mr_join(&acc, &x, false))
}

/// Whether some value lies in both (or both may be non-finite); a shared 0
/// counts only when neither part excludes it.
fn mr_overlap(a: &Reach, b: &Reach) -> bool {
    (a.wild && b.wild)
        || a.parts.iter().any(|x| {
            b.parts.iter().any(|y| {
                let (lo, hi) = (x.0.max(y.0), x.1.min(y.1));
                lo < hi || (lo == hi && (lo != 0.0 || !(mr_part_nonzero(x) || mr_part_nonzero(y))))
            })
        })
}

/// Whether some x in a and y in b have |x − y| ≤ relative·max(|x|, |y|).
fn mr_near(a: &Reach, b: &Reach, relative: f64) -> bool {
    mr_overlap(&a.mr_slack(relative), b) || mr_overlap(a, &b.mr_slack(relative))
}

/// The marker of an unresolved instance among a relation's items: consistent
/// with its operands' stated sets but not decided by them (never passed).
const MR_UNRESOLVED: &str = "unresolved: ";

/// A relation the producer forms through a quotient (Γ²·m = m_eff,
/// fraction·T_k = m_eff, Γ·m = φᵀp_k, m_eff·m = (φᵀp_k)²), on sets of reals.
/// A non-finite operand leaves it unresolved; a real beyond the largest
/// binary64 is not one, and cannot equal a finite side, so only reals count.
#[allow(clippy::too_many_arguments)]
fn mr_quotient(scope: &mut Vec<String>, label: &str, operands: &[&Reach], lhs: &Reach, rhs: &Reach, relative: f64, one_sided: bool, failure: String) -> R<()> {
    if operands.iter().any(|x| x.wild) {
        scope.push(format!("{MR_UNRESOLVED}{label}: an operand is not finite (±∞ or NaN)"));
        return Ok(());
    }
    let (lhs, rhs) = (lhs.mr_reals(), rhs.mr_reals());
    let holds = if one_sided { mr_overlap(&lhs.mr_slack(relative), &rhs) } else { mr_near(&lhs, &rhs, relative) };
    if !holds {
        return Err(failure);
    }
    scope.push(format!("{MR_UNRESOLVED}{label} is undecidable on the published bounds"));
    Ok(())
}

/// Whether a binary64 product of nonzero factors kept the real product's
/// magnitude: a nonzero product below the normal range would read as (near)
/// 0 on the point path, so the set path decides it instead.
fn mr_kept(product: f64, factors: &[f64]) -> bool {
    !(factors.iter().all(|f| *f != 0.0) && product.abs() < f64::MIN_POSITIVE)
}

/// One entry's term of m = Σ_ij φ_i·M_ij·φ_j. The producer sums each full
/// triple product exactly (`mass_bilinear`, modal_quantities.rs:294,
/// `ExactSum::add_product`), so the coefficient w·φ_i·φ_j is formed on sets,
/// which keep a nonzero underflow nonzero, never as a binary64 product.
/// `None` only for an exactly zero coefficient against a finite entry.
fn mr_generalized_mass_term(i: usize, j: usize, phi: &[f64], entry: &Reach) -> Option<Reach> {
    let w = Reach::mr_exact(if i == j { 1.0 } else { 2.0 });
    let coefficient = mr_join(&mr_join(&w, &Reach::mr_exact(phi[i]), true), &Reach::mr_exact(phi[j]), true);
    if coefficient.mr_point() == Some(0.0) && !entry.wild {
        return None;
    }
    Some(mr_join(&coefficient, entry, true))
}

/// m against φᵀM_ffφ from its terms (`mr_generalized_mass_term`), within
/// A(N+4)u·Σ|terms| over the N free pairs: decided on points only when every
/// term is a point.
fn mr_generalized_mass_check(scope: &mut Vec<String>, label: usize, gm_num: &Num, gm: &Reach, mass_terms: &[Reach], count: usize) -> R<()> {
    let allowance = ALLOW * ((count + 4) as f64) * U;
    let points: Option<Vec<f64>> = mass_terms.iter().map(Reach::mr_point).collect();
    if let (Some(points), Some(published)) = (points, gm.mr_point()) {
        let (s, a) = points.iter().fold((0.0f64, 0.0f64), |(s, a), x| (s + x, a + x.abs()));
        if !mr_tol_close(published, s, allowance * a) {
            return Err(format!("mode {label}: m = {published:e} is not φᵀM_ffφ = {s:e}"));
        }
        return Ok(());
    }
    let derived = mr_sum(mass_terms.iter().map(|t| t.mr_slack(allowance)));
    if !mr_overlap(gm, &derived) {
        if gm_num.below {
            return Err(format!("mode {label}: m published as below-normal, but φᵀM_ffφ from the payload is normal"));
        }
        return Err(format!("mode {label}: m is not φᵀM_ffφ for any value the bounds allow"));
    }
    scope.push(format!("{MR_UNRESOLVED}mode {label}: m = φᵀM_ffφ is undecidable on the published bounds"));
    Ok(())
}

/// φᵀp_k's terms per direction, ONE per mass entry (`entries` as (row,
/// col)): (entry index, combined coefficient, Σ|coefficients|), and the
/// incidence count. An entry M_ij meets φᵀp_k in row i (i free, j along k)
/// and in row j (j free, i along k); its coefficients add (the reviewer's
/// +1 and −1 on one entry are exactly 0).
fn mr_excitation_terms(free: &[bool], entries: &[(usize, usize)], phi: &[f64]) -> ([Vec<(usize, f64, f64)>; 3], [usize; 3]) {
    let mut terms: [Vec<(usize, f64, f64)>; 3] = Default::default();
    let mut counts = [0usize; 3];
    for (index, &(i, j)) in entries.iter().enumerate() {
        let mut held: [Option<(f64, f64)>; 3] = [None; 3];
        if free[i] && j % 6 < 3 {
            let h = held[j % 6].get_or_insert((0.0, 0.0));
            h.0 += phi[i];
            h.1 += phi[i].abs();
            counts[j % 6] += 1;
        }
        if i != j && free[j] && i % 6 < 3 {
            let h = held[i % 6].get_or_insert((0.0, 0.0));
            h.0 += phi[j];
            h.1 += phi[j].abs();
            counts[i % 6] += 1;
        }
        for (k, h) in held.iter().enumerate() {
            if let Some((c, w)) = h {
                terms[k].push((index, *c, *w));
            }
        }
    }
    (terms, counts)
}



/// One identity row: what it compares and its relative discrepancy.
struct IdentityRow<'v> {
    subject_kind: &'v str,
    subject: &'v str,
    axis: usize,
    comparison: &'v str,
    relative: Num<'v>,
}

/// One support row of a linearized state case.
struct SupportRow<'v> {
    id: &'v str,
    behavior: &'v str,
    dof: usize,
    state: &'v str,
    treatment: &'v str,
}

/// Everything the relations read, after closure. Every string is the
/// document's own, borrowed: nothing of the document is copied.
struct Parsed<'v> {
    nodes: Vec<[Option<f64>; 3]>,
    /// The node ids, in node order.
    node_ids: Vec<&'v str>,
    /// `pressure_free` or `exact_pressure`.
    route: &'v str,
    /// The published mass centre, when stated.
    mass_center: Option<[Num<'v>; 3]>,
    /// The support rows, in row order.
    support_rows: Vec<SupportRow<'v>>,
    /// Per global DOF, its free index.
    free: Vec<Option<usize>>,
    /// (row, col, value); `None` when withheld.
    mass: Option<Vec<(usize, usize, Num<'v>)>>,
    totals: [Num<'v>; 3],
    free_totals: [Num<'v>; 3],
    modes: Vec<Mode<'v>>,
    /// Per mode, per global DOF; `None` when withheld.
    shapes: Option<Vec<Vec<Num<'v>>>>,
    participation: Vec<[Participation<'v>; 3]>,
    clusters: Vec<(usize, usize, bool, Num<'v>)>,
    computed: Vec<Computed<'v>>,
    candidates: Vec<Candidate<'v>>,
    /// The identity rows, in row order.
    identity: Vec<IdentityRow<'v>>,
    /// The straight members of the discretization table, in row order.
    members: Vec<&'v str>,
    /// The binding's subject census.
    census: census::Census<'v>,
    /// The state case's linearization: `None` without a state case.
    linearized: Option<bool>,
    restrained_refs: Vec<usize>,
    /// (DOF, treatment) of each support row.
    supports: Vec<(usize, &'v str)>,
    /// (text, source) of each disclosure.
    disclosures: Vec<(&'v str, &'v str)>,
    /// The bound integrity parameters (an object, by closure).
    params: Option<&'v Map<String, Value>>,
    unitized: Vec<Unitized<'v>>,
    tau_t: Option<f64>,
    ell: Option<f64>,
    mode_count: Option<usize>,
    cutoff: Option<f64>,
    stopped_by: &'v str,
    bounded: usize,
    extension: usize,
    count: usize,
    computed_count: usize,
    endpoint: &'v str,
    next_computed: Option<usize>,
    established_endpoint: Option<usize>,
    census_established: Option<usize>,
    sensitive: bool,
    set_reasons: usize,
    sturm_count: usize,
    sturm_reported: usize,
    placement: &'v str,
    placement_lower: Option<f64>,
    placement_upper: Option<f64>,
    placement_established: Option<usize>,
    kappa_c: Option<f64>,
    limit: Option<f64>,
    kappa_s: Option<f64>,
    growth_limit: Option<f64>,
    ortho: Num<'v>,
    ortho_pair: bool,
    formation: Vec<Option<f64>>,
}

fn mr_index(text: &str, prefix: &str, path: &str) -> R<usize> {
    text.strip_prefix(prefix)
        .and_then(|rest| rest.parse::<usize>().ok())
        .filter(|&i| i >= 1 && format!("{prefix}{i}") == text)
        .ok_or_else(|| format!("{path}: `{text}` is not a `{prefix}<n>` reference"))
}

fn mr_unitize<'v>(out: &mut Vec<Unitized<'v>>, value: &'v Value, key: &'static str, path: MrPath) {
    out.push(Unitized { value, key, path });
}

/// Closure of a solved document; returns what the relations read.
fn mr_closure<'v>(doc: &'v Value) -> R<Parsed<'v>> {
    mr_object(doc, "", &SOLVED_KEYS)?;
    mr_identity(doc)?;
    mr_const(&doc["status"], "/status", "solved")?;
    let mut unitized = Vec::new();
    let params = doc["binding"]["integrity_parameters"].as_object();
    let census = census::cs_shape(&doc["binding"]["subject_census"], true)?.ok_or("/binding/subject_census: not an object")?;

    // Tables first: the headers refer into them.
    let tables = mr_object(&doc["tables"], "/tables", &TABLES)?;
    let mut ids = HashSet::new();
    let mut get = |name: &str| mr_table(tables.get(name), name, &mut ids).map(|t| t.rows);

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
            mr_unitize(&mut unitized, &r[*axis], "fixed", MrPath::mr_row("/tables/nodes", i, *axis));
        }
        // Keyed by the row's own id, which is `node:{id}` (checked above).
        node_index.insert(r["row_id"].as_str().unwrap_or(""), i);
        node_ids.push(id);
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
        mr_id_is(r, &path, &format!("dof:{}:{component}", node_ids[node]))?;
        // Keyed by the row's own id, which is `dof:{node}:{component}` (checked above).
        dof_index.insert(r["row_id"].as_str().unwrap_or(""), i);
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
                mr_unitize(&mut unitized, &r["value"], pair, MrPath::mr_row("/tables/mass_matrix", i, "value"));
                out.push((a, b, Num::mr_from(&vo)));
            }
            Some(out)
        }
    };
    let total_rows = get("mass_totals")?.ok_or("/tables/mass_totals: withheld")?;
    if total_rows.len() != 3 {
        return Err("/tables/mass_totals: one row per direction".into());
    }
    let none = || Num { value: None, reason: None, below: false };
    let (mut totals, mut free_totals) = ([none(), none(), none()], [none(), none(), none()]);
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
        totals[k] = Num::mr_from(&t);
        free_totals[k] = Num::mr_from(&f);
        mr_unitize(&mut unitized, &r["total_mass"], "fixed", MrPath::mr_row("/tables/mass_totals", k, "total_mass"));
        mr_unitize(&mut unitized, &r["free_dof_mass"], "fixed", MrPath::mr_row("/tables/mass_totals", k, "free_dof_mass"));
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
            mr_unitize(&mut unitized, &r[field], "fixed", MrPath::mr_row("/tables/computed_modes", i, field));
            vos.push(vo);
        }
        computed.push(Computed { cluster, reported, mode, omega: vos[0].value, frequency: vos[1].value, eta: Num::mr_from(&vos[2]),
            condition: Num::mr_from(&vos[3]), estimate: Num::mr_from(&vos[4]) });
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
        mr_unitize(&mut unitized, &r["error_estimate"], "fixed", MrPath::mr_row("/tables/clusters", i, "error_estimate"));
        clusters.push((first, last, reported, Num::mr_from(&vo)));
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
        let mut get_vo = |field: &'static str, kind: &str, key: &'static str| -> R<Vo> {
            let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
            mr_unitize(&mut unitized, &r[field], key, MrPath::mr_row("/tables/modes", i, field));
            Ok(vo)
        };
        let omega = get_vo("circular_frequency", "modal_circular_frequency", "fixed")?.value;
        let frequency = get_vo("frequency", "modal_frequency", "fixed")?.value;
        let period = get_vo("period", "modal_period", "fixed")?.value;
        let generalized_mass = get_vo("generalized_mass", "modal_generalized_mass", CASES[case])?;
        let cluster_estimate = Num::mr_from(&get_vo("cluster_error_estimate", "modal_cluster_error_estimate", "fixed")?);
        modes.push(Mode { case, reference, sensitive: token == "sensitive", reasons: reasons.len(), omega, frequency, period, generalized_mass,
            cluster_estimate, cluster });
    }
    let shapes = match get("shapes")? {
        None => None,
        Some(rows) => {
            if rows.len() != n * ndof {
                return Err("/tables/shapes: one row per mode and DOF".into());
            }
            let mut out = vec![vec![Num { value: None, reason: None, below: false }; ndof]; n];
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
                mr_unitize(&mut unitized, &r["value"], CASES[modes[m].case], MrPath::mr_row("/tables/shapes", i, "value"));
                out[m][dof] = Num::mr_from(&vo);
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
            let mut read = |field: &'static str, kind: &str, key: &'static str| -> R<Vo> {
                let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
                mr_unitize(&mut unitized, &r[field], key, MrPath::mr_row("/tables/participation", i, field));
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
        let decision = mr_one_of(&r["decision"], &mr_at(&path, "decision"), &DECISIONS)?;
        if mr_flag(&r["accepted"], &mr_at(&path, "accepted"))? != (decision == "accepted") {
            return Err(format!("{path}/accepted: not its decision"));
        }
        let pivots = if r["negative_pivot_count"].is_null() { None } else { Some(mr_count(&r["negative_pivot_count"], &mr_at(&path, "negative_pivot_count"))?) };
        let mut read = |field: &'static str, kind: &str| -> R<Vo> {
            let vo = mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
            mr_unitize(&mut unitized, &r[field], "fixed", MrPath::mr_row("/tables/sturm_candidates", i, field));
            Ok(vo)
        };
        let mut num = |field: &'static str, kind: &str| -> R<Num> { Ok(Num::mr_from(&read(field, kind)?)) };
        let shift = num("shift", "sturm_shift_circular_frequency")?;
        let formation_term = num("formation_term", "sturm_formation_term")?;
        let formation_separation = num("formation_separation_ratio", "sturm_formation_separation_ratio")?;
        let factor_term = num("factor_term", "sturm_factor_term")?;
        let backward_error = num("backward_error", "sturm_backward_error")?;
        let inverse_norm = num("inverse_norm_estimate", "sturm_inverse_norm_estimate")?;
        let product = num("conditioning_product", "sturm_conditioning_product")?;
        let separation = num("separation_ratio", "sturm_separation_ratio")?;
        let growth = num("growth", "sturm_growth_factor")?;
        let maxima = [
            num("max_abs_lower_factor", "sturm_max_abs_lower_factor")?,
            num("max_abs_pivot", "sturm_max_abs_pivot")?,
            num("max_abs_scaled_diagonal", "sturm_max_abs_scaled_diagonal")?,
        ];
        candidates.push(Candidate { decision, pivots, shift, formation_term, formation_separation, factor_term, backward_error, inverse_norm, product, separation, growth, maxima });
    }
    let identity_rows = get("identity")?.ok_or("/tables/identity: withheld")?;
    let mut identity = Vec::new();
    for (i, row) in identity_rows.iter().enumerate() {
        let path = format!("/tables/identity/rows/{i}");
        let r = mr_fields(row, &path, &["subject", "subject_kind", "axis", "comparison", "relative"])?;
        mr_id_is(r, &path, &format!("identity:{i}"))?;
        let subject = mr_text(&r["subject"], &mr_at(&path, "subject"))?;
        let subject_kind = mr_one_of(&r["subject_kind"], &mr_at(&path, "subject_kind"), &["member", "component_body", "model"])?;
        if subject_kind == "model" && subject != "model" {
            return Err(format!("{path}: the aggregate subject is `model`"));
        }
        let axis = mr_one_of(&r["axis"], &mr_at(&path, "axis"), &DIRECTIONS)?;
        let comparison = mr_one_of(&r["comparison"], &mr_at(&path, "comparison"), &["nodal", "resultant"])?;
        let vo = mr_vo(&r["relative"], &mr_at(&path, "relative"), &["identity_relative_discrepancy"])?;
        mr_unitize(&mut unitized, &r["relative"], "fixed", MrPath::mr_row("/tables/identity", i, "relative"));
        let axis = DIRECTIONS.iter().position(|d| *d == axis).unwrap_or(0);
        identity.push(IdentityRow { subject_kind, subject, axis, comparison, relative: Num::mr_from(&vo) });
    }
    let mut supports = Vec::new();
    let mut support_rows = Vec::new();
    for (i, row) in get("supports")?.ok_or("/tables/supports: withheld")?.iter().enumerate() {
        let path = format!("/tables/supports/rows/{i}");
        let r = mr_fields(row, &path, &["support_id", "behavior", "dof_ref", "state", "treatment"])?;
        let id = mr_text(&r["support_id"], &mr_at(&path, "support_id"))?;
        mr_id_is(r, &path, &format!("support:{id}"))?;
        let behavior = mr_text(&r["behavior"], &mr_at(&path, "behavior"))?;
        let dof = dof_of(&r["dof_ref"], &mr_at(&path, "dof_ref"))?;
        let state = mr_one_of(&r["state"], &mr_at(&path, "state"), &["active", "inactive", "sticking", "sliding"])?;
        let treatment = mr_one_of(&r["treatment"], &mr_at(&path, "treatment"), &["restrained", "already_restrained", "free", "free_frozen_friction"])?;
        supports.push((dof, treatment));
        support_rows.push(SupportRow { id, behavior, dof, state, treatment });
    }
    let mut members = Vec::new();
    for (i, row) in get("discretization")?.ok_or("/tables/discretization: withheld")?.iter().enumerate() {
        let path = format!("/tables/discretization/rows/{i}");
        let r = mr_fields(row, &path, &["member_ref", "length", "fixed_fixed_frequency", "ratio_to_highest_reported"])?;
        let id = mr_text(&r["member_ref"], &mr_at(&path, "member_ref"))?;
        members.push(id);
        mr_id_is(r, &path, &format!("member:{id}"))?;
        for (field, kind) in [("length", "member_length"), ("fixed_fixed_frequency", "member_fixed_fixed_frequency"), ("ratio_to_highest_reported", "member_frequency_ratio")] {
            mr_vo(&r[field], &mr_at(&path, field), &[kind])?;
            mr_unitize(&mut unitized, &r[field], "fixed", MrPath::mr_row("/tables/discretization", i, field));
        }
    }
    let mut disclosures = Vec::new();
    for (i, row) in get("disclosures")?.ok_or("/tables/disclosures: withheld")?.iter().enumerate() {
        let path = format!("/tables/disclosures/rows/{i}");
        let r = mr_fields(row, &path, &["text", "source"])?;
        mr_id_is(r, &path, &format!("disclosure:{i}"))?;
        let text = mr_text(&r["text"], &mr_at(&path, "text"))?;
        let source = mr_one_of(&r["source"], &mr_at(&path, "source"), &["modal", "weight_basis"])?;
        disclosures.push((text, source));
    }
    for (i, row) in get("diagnostics")?.ok_or("/tables/diagnostics: withheld")?.iter().enumerate() {
        let path = format!("/tables/diagnostics/rows/{i}");
        mr_diagnostic_row(row, &path, &format!("diagnostic:{i}"), &["modal", "state_case_solve"])?;
    }

    // Headers.
    let basis = mr_object(&doc["basis"], "/basis", &["route", "no_translation_threshold", "bounding_box_diagonal", "weight_to_mass_acceleration",
        "identity_acceleration", "mass_center", "state_case"])?;
    let route = mr_one_of(&basis["route"], "/basis/route", &["pressure_free", "exact_pressure"])?;
    let mut header_vo = |value: &'v Value, path: MrPath, kind: &str| -> R<Vo<'v>> {
        let vo = mr_vo(value, &path.to_string(), &[kind])?;
        mr_unitize(&mut unitized, value, "fixed", path);
        Ok(vo)
    };
    let tau_t = header_vo(&basis["no_translation_threshold"], MrPath::mr_header("/basis", "no_translation_threshold"), "basis_no_translation_threshold")?.value;
    let ell = header_vo(&basis["bounding_box_diagonal"], MrPath::mr_header("/basis", "bounding_box_diagonal"), "basis_bounding_box_diagonal")?.value;
    if !basis["weight_to_mass_acceleration"].is_null() {
        header_vo(&basis["weight_to_mass_acceleration"], MrPath::mr_header("/basis", "weight_to_mass_acceleration"), "request_weight_to_mass_acceleration")?;
    }
    header_vo(&basis["identity_acceleration"], MrPath::mr_header("/basis", "identity_acceleration"), "identity_acceleration")?;
    let mut mass_center = None;
    if !basis["mass_center"].is_null() {
        let c = mr_object(&basis["mass_center"], "/basis/mass_center", &["x", "y", "z"])?;
        let mut center = [none(), none(), none()];
        for (k, axis) in ["x", "y", "z"].into_iter().enumerate() {
            center[k] = Num::mr_from(&header_vo(&c[axis], MrPath::mr_header("/basis/mass_center", axis), "basis_mass_center")?);
        }
        mass_center = Some(center);
    }
    let (mut linearized_state, mut restrained_refs) = (None, Vec::new());
    if !basis["state_case"].is_null() {
        let s = mr_object(&basis["state_case"], "/basis/state_case", &["case_id", "case_index", "linearized", "solver_mode", "restrained_dof_refs"])?;
        mr_text(&s["case_id"], "/basis/state_case/case_id")?;
        mr_count(&s["case_index"], "/basis/state_case/case_index")?;
        let linearized = mr_flag(&s["linearized"], "/basis/state_case/linearized")?;
        linearized_state = Some(linearized);
        if linearized {
            mr_const(&s["solver_mode"], "/basis/state_case/solver_mode", "dense_scrutiny")?;
            for (i, r) in mr_list(&s["restrained_dof_refs"], "/basis/state_case/restrained_dof_refs")?.iter().enumerate() {
                restrained_refs.push(dof_of(r, &format!("/basis/state_case/restrained_dof_refs/{i}"))?);
            }
        } else if !s["solver_mode"].is_null() || !s["restrained_dof_refs"].is_null() {
            return Err("/basis/state_case: a case that linearizes no support states no mode or restrained set".into());
        }
    }
    let x = mr_object(&doc["extraction"], "/extraction", &["mode_count", "frequency_cutoff", "stopped_by", "bounded_count", "extension_count",
        "count", "computed_count", "endpoint"])?;
    let mode_count = if x["mode_count"].is_null() { None } else { Some(mr_count(&x["mode_count"], "/extraction/mode_count")?) };
    let cutoff = if x["frequency_cutoff"].is_null() { None } else { header_vo(&x["frequency_cutoff"], MrPath::mr_header("/extraction", "frequency_cutoff"), "request_frequency_cutoff")?.value };
    if mode_count.is_none() && x["frequency_cutoff"].is_null() {
        return Err("/extraction: a modal result states its bound".into());
    }
    let stopped_by = mr_one_of(&x["stopped_by"], "/extraction/stopped_by", &["mode_count", "frequency_cutoff", "mode_count_and_frequency_cutoff",
        "spectrum_exhausted"])?;
    let e = mr_object(&x["endpoint"], "/extraction/endpoint", &["kind", "next_computed_ref", "established_finite_mode_count"])?;
    let endpoint = mr_one_of(&e["kind"], "/extraction/endpoint/kind", &["below_computed_cluster", "below_lowest_mode", "whole_finite_spectrum"])?;
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
    let placement = mr_one_of(&p["kind"], "/sturm/placement/kind", &["given", "gap", "above_spectrum"])?;
    let mut optional_vo = |value: &'v Value, path: MrPath, kind: &str| -> R<Option<f64>> {
        if value.is_null() {
            return Ok(None);
        }
        Ok(header_vo(value, path, kind)?.value)
    };
    let placement_lower = optional_vo(&p["lower"], MrPath::mr_header("/sturm/placement", "lower"), "placement_circular_frequency")?;
    let placement_upper = optional_vo(&p["upper"], MrPath::mr_header("/sturm/placement", "upper"), "placement_circular_frequency")?;
    let placement_established = if p["established_finite_mode_count"].is_null() { None } else {
        Some(mr_count(&p["established_finite_mode_count"], "/sturm/placement/established_finite_mode_count")?)
    };
    let kappa_c = header_vo(&s["conditioning_multiple"], MrPath::mr_header("/sturm", "conditioning_multiple"), "sturm_premise_parameter")?.value;
    let limit = header_vo(&s["conditioning_limit"], MrPath::mr_header("/sturm", "conditioning_limit"), "sturm_premise_parameter")?.value;
    let kappa_s = header_vo(&s["separation_multiple"], MrPath::mr_header("/sturm", "separation_multiple"), "sturm_premise_parameter")?.value;
    let growth_limit = header_vo(&s["growth_limit"], MrPath::mr_header("/sturm", "growth_limit"), "sturm_premise_parameter")?.value;
    let o = mr_object(&doc["orthogonality"], "/orthogonality", &["max_ratio", "pair"])?;
    let ortho = Num::mr_from(&header_vo(&o["max_ratio"], MrPath::mr_header("/orthogonality", "max_ratio"), "orthogonality_max_ratio")?);
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
    for (field, kind) in [("stiffness_difference", "stiffness_formation_difference"), ("mass_difference", "mass_formation_difference"),
        ("reformed_stiffness_norm", "reformed_stiffness_norm"), ("reformed_mass_norm", "reformed_mass_norm")] {
        formation.push(header_vo(&f[field], MrPath::mr_header("/formation", field), kind)?.value);
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
            formation.push(header_vo(&sec[field], MrPath::mr_header("/formation/section", field), "section_derivation_difference")?.value);
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
        node_ids,
        route,
        mass_center,
        support_rows,
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
        members,
        census,
        linearized: linearized_state,
        restrained_refs,
        supports,
        disclosures,
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
        let rows = mr_table(Some(&doc["refusals"]), "refusals", &mut ids)?.rows.ok_or("/refusals: withheld")?;
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
        closure: ModalReadStanding::mr_passed(),
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
                // An absent member hashes as `[]` (borrowed, not copied).
                let empty = Value::Array(Vec::new());
                let materials = request.get("materials").unwrap_or(&empty);
                if modal_reader_sha256(materials).as_deref() != b["materials_sha256"].as_str() {
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
    p.params.and_then(|m| m.get(name)).and_then(Value::as_f64)
}

fn mr_tol_close(published: f64, derived: f64, tolerance: f64) -> bool {
    (published - derived).abs() <= tolerance
}

/// Mass: T_k and the free-DOF mass from the mass entries, the diagonal, and
/// Bessel's inequality, each relation on its own operands.
/// Agent 0's Bessel interim contract: true only when Σ m_eff ≤ T_k·(1 + 16(n+2)u) is
/// established for the given upper ends of the effective masses and lower end of T_k.
/// The comparison rounds outward: each rounded partial sum is stepped up one ulp
/// (covering its round-to-nearest error), the allowed total is stepped down one ulp,
/// and the slack 1 + 16(n+2)u is exact in binary64. Outward rounding can only turn an
/// established case into an unestablished one, never the reverse.
fn mr_bessel_established(effective_highs: &[f64], t_low: f64, n: usize) -> bool {
    let highest = effective_highs.iter().fold(0.0_f64, |acc, x| (acc + x).next_up());
    let slack = 1.0 + ALLOW * ((n + 2) as f64) * U;
    highest <= (t_low * slack).next_down()
}

fn mr_mass(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    let n = p.modes.len();
    for k in 0..3 {
        let d = DIRECTIONS[k];
        for (name, num) in [(format!("T_{d}"), &p.totals[k]), (format!("free-DOF mass {d}"), &p.free_totals[k])] {
            mr_domain(num, MrRule::Binary64, &name)?;
            mr_need(num, false, &name)?;
            if num.value.is_some_and(|v| v < 0.0) {
                return Err(format!("{name}: negative"));
            }
        }
    }
    for k in 0..3 {
        let d = DIRECTIONS[k];
        let t = mr_need(&p.totals[k], false, &format!("T_{d}"))?;
        // A claim-null m_eff fails participation; m_eff ≥ 0, so leaving it out
        // keeps a lower bound on the sum.
        let effective: Vec<Reach> = (0..n).filter_map(|m| mr_reach(&Num::mr_from(&p.participation[m][k].effective), false)).collect();
        if t.wild || effective.iter().any(|e| e.wild) {
            scope.push(format!("{MR_UNRESOLVED}Σ m_eff,{d} ≤ T_{d}: an operand is not finite (±∞ or NaN)"));
            continue;
        }
        // Agent 0's interim contract: Bessel passes only where
        // sound arithmetic establishes Σ m_eff ≤ T_k from the published operands;
        // otherwise it is unresolved, never refused. The inequality holds for
        // modes orthogonal in the assembled M, which the document does not
        // evidence (its published deviation is on the re-formed M̂).
        // The comparison rounds outward (review of f5c66b167): an upper bound on the
        // sum (each rounded partial sum stepped up one ulp, which covers its
        // round-to-nearest error) against a lower bound on the allowed total (the
        // rounded product stepped down one ulp). The slack 1 + 16(n+2)u is exact in
        // binary64. Outward rounding can only make a pass unresolved, never the reverse.
        let highs: Vec<f64> = effective.iter().map(Reach::mr_highest).collect();
        if !mr_bessel_established(&highs, t.mr_lowest(), n) {
            scope.push(format!("{MR_UNRESOLVED}Bessel: Σ m_eff,{d} ≤ T_{d} is not established within the strict rounding allowance, and no common-M orthogonality bound is published (a temporary capability limit)"));
        }
    }
    let Some(entries) = &p.mass else {
        scope.push("mass_matrix withheld: T_k and the free-DOF mass are not re-derived".to_string());
        return Ok(scope);
    };
    let mut sets = Vec::with_capacity(entries.len());
    for (i, j, v) in entries {
        mr_domain(v, MrRule::Binary64, &format!("mass entry ({i}, {j})"))?;
        // A diagonal entry is a mass (nonnegative); an off-diagonal one has either sign.
        let s = mr_need(v, i != j, &format!("mass entry ({i}, {j})"))?;
        if i == j && v.value.is_some_and(|x| x < 0.0) {
            return Err(format!("mass diagonal at DOF {i} is negative"));
        }
        sets.push((*i, *j, s));
    }
    for k in 0..3 {
        let d = DIRECTIONS[k];
        for (name, published, free_only) in [(format!("T_{d}"), &p.totals[k], false), (format!("free-DOF mass {d}"), &p.free_totals[k], true)] {
            // T_k = R_kᵀ M R_k over every DOF (restrained included); the free-DOF
            // mass over pairs of free DOFs.
            let terms: Vec<(f64, &Reach)> = sets
                .iter()
                .filter(|(i, j, _)| i % 6 == k && j % 6 == k && !(free_only && (p.free[*i].is_none() || p.free[*j].is_none())))
                .map(|(i, j, s)| (if i == j { 1.0 } else { 2.0 }, s))
                .collect();
            let allowance = ALLOW * ((terms.len() + 4) as f64) * U;
            let stated = mr_need(published, false, &name)?;
            let points: Option<Vec<f64>> = terms.iter().map(|(w, s)| s.mr_point().map(|x| w * x)).collect();
            if let (Some(points), Some(value)) = (points, stated.mr_point()) {
                let (s, a) = points.iter().fold((0.0f64, 0.0f64), |(s, a), x| (s + x, a + x.abs()));
                if !mr_tol_close(value, s, allowance * a) {
                    return Err(format!("{name} is not the sum of its mass entries ({value:e} against {s:e})"));
                }
                continue;
            }
            let derived = mr_sum(terms.iter().map(|(w, s)| mr_join(&Reach::mr_exact(*w), s, true).mr_slack(allowance)));
            if !mr_overlap(&stated, &derived) {
                return Err(format!("{name} is not the sum of its mass entries for any value its bounds allow"));
            }
            scope.push(format!("{MR_UNRESOLVED}{name} = Σ of its mass entries is undecidable on the published bounds"));
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
        if shape[mode.reference].value != Some(1.0) || shape[mode.reference].below {
            return Err(format!("mode {label}: its reference component is not exactly +1"));
        }
        let (mut t, mut theta) = (0.0f64, 0.0f64);
        for (dof, value) in shape.iter().enumerate() {
            mr_domain(value, MrRule::Published, &format!("mode {label}: shape component {dof}"))?;
            let v = value.value.ok_or_else(|| format!("mode {label}: a shape component is not published"))?;
            if p.free[dof].is_none() {
                // A below-normal component is nonzero.
                if v != 0.0 || value.below {
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

/// Participation: m_i, Γ, m_eff, Γ²m, the fractions and their cumulative
/// sums, and m_i, Γ and m_eff against the payload; each relation on its own
/// operands, a null scoping only the relations that consume it.
fn mr_participation(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    let n = p.modes.len();
    let au = ALLOW * U;
    for (m, mode) in p.modes.iter().enumerate() {
        let label = m + 1;
        let gm_num = Num::mr_from(&mode.generalized_mass);
        mr_domain(&gm_num, MrRule::Published, &format!("mode {label}: m"))?;
        for k in 0..3 {
            let row = &p.participation[m][k];
            for (name, vo) in [("Γ", &row.gamma), ("m_eff", &row.effective), ("fraction", &row.fraction)] {
                mr_domain(&Num::mr_from(vo), MrRule::Published, &format!("mode {label}: {name} along {}", DIRECTIONS[k]))?;
            }
            mr_domain(&Num::mr_from(&row.cumulative), MrRule::Binary64, &format!("mode {label}: cumulative fraction along {}", DIRECTIONS[k]))?;
        }
        let gm = mr_need(&gm_num, false, &format!("mode {label}: m"))?;
        if gm_num.value.is_some_and(|v| !gm_num.below && v <= 0.0) {
            return Err(format!("mode {label}: m is not positive"));
        }
        for k in 0..3 {
            let d = DIRECTIONS[k];
            let row = &p.participation[m][k];
            let (g_num, e_num) = (Num::mr_from(&row.gamma), Num::mr_from(&row.effective));
            let gamma = mr_need(&g_num, true, &format!("mode {label}: Γ along {d}"))?;
            let effective = mr_need(&e_num, false, &format!("mode {label}: m_eff along {d}"))?;
            if e_num.value.is_some_and(|v| v < 0.0) {
                return Err(format!("mode {label}: m_eff,{d} is negative"));
            }
            let flagged = |nums: &[&Num]| if nums.iter().any(|x| x.below) { " (a side flagged below-normal)" } else { "" };
            // Γ²·m = m_eff.
            match (gamma.mr_point(), gm.mr_point(), effective.mr_point()) {
                (Some(g), Some(mass), Some(e)) if mr_kept(g * g, &[g, g]) && mr_kept(g * g * mass, &[g, mass]) => {
                    let g2m = g * g * mass;
                    if !mr_tol_close(g2m, e, au * e.max(g2m)) {
                        return Err(format!("mode {label}: Γ_{d}²·m = {g2m:e} is not m_eff = {e:e}"));
                    }
                }
                _ => mr_quotient(&mut scope, &format!("mode {label}: Γ²·m = m_eff along {d}"), &[&gamma, &gm, &effective],
                    &mr_join(&gamma.mr_squared(), &gm, true), &effective, au, false,
                    format!("mode {label}: Γ²·m = m_eff along {d} is contradicted by the bounds{}", flagged(&[&g_num, &gm_num, &e_num])))?,
            }
            // The fraction: undefined exactly when T_k = 0, which the null claims;
            // the claim is checked against T_k and m_eff.
            let total = mr_need(&p.totals[k], false, &format!("T_{d}"))?;
            let (f_num, c_num) = (Num::mr_from(&row.fraction), Num::mr_from(&row.cumulative));
            let zero = (f_num.reason.as_deref() == Some("total_mass_zero"), c_num.reason.as_deref() == Some("total_mass_zero"));
            if zero.0 || zero.1 {
                if !(zero.0 && zero.1) {
                    return Err(format!("mode {label}: the fraction and its cumulative sum along {d} disagree on T_k = 0"));
                }
                if total.mr_point() != Some(0.0) {
                    return Err(format!("mode {label}: the fraction along {d} claims T_k = 0, but T_k cannot be 0"));
                }
                if effective.mr_point() != Some(0.0) {
                    return Err(format!("mode {label}: with T_{d} = 0 the fraction is undefined and m_eff is 0"));
                }
                continue;
            }
            if total.mr_point() == Some(0.0) {
                return Err(format!("mode {label}: with T_{d} = 0 the fraction is undefined and m_eff is 0"));
            }
            let fraction = mr_need(&f_num, false, &format!("mode {label}: fraction along {d}"))?;
            if f_num.value.is_some_and(|v| v < 0.0) {
                return Err(format!("mode {label}: the fraction along {d} is negative"));
            }
            match (fraction.mr_point(), total.mr_point(), effective.mr_point()) {
                (Some(f), Some(t), Some(e)) if mr_kept(f * t, &[f, t]) => {
                    let ft = f * t;
                    if !mr_tol_close(ft, e, au * e.max(ft)) {
                        return Err(format!("mode {label}: fraction {d}·T = {ft:e} is not m_eff = {e:e}"));
                    }
                }
                _ => mr_quotient(&mut scope, &format!("mode {label}: fraction·T_k = m_eff along {d}"), &[&fraction, &total, &effective],
                    &mr_join(&fraction, &total, true), &effective, au, false,
                    format!("mode {label}: fraction·T_k = m_eff along {d} is contradicted by the bounds{}", flagged(&[&f_num, &e_num])))?,
            }
        }
    }
    // The cumulative sums, each on the fractions up to it and itself.
    for k in 0..3 {
        let d = DIRECTIONS[k];
        let mut running: Vec<Reach> = Vec::new();
        for m in 0..n {
            let label = m + 1;
            let row = &p.participation[m][k];
            let f_num = Num::mr_from(&row.fraction);
            if f_num.reason.as_deref() == Some("total_mass_zero") {
                break;
            }
            // The producer sums the PUBLISHED fractions (a flagged fraction is its
            // published 0), modal_quantities.rs:460.
            running.push(mr_published(&f_num, false).ok_or_else(|| {
                format!("mode {label}: fraction along {d}: null, and its reason `{}` states no value where one is needed", f_num.reason.as_deref().unwrap_or(""))
            })?);
            let c_num = Num::mr_from(&row.cumulative);
            let cumulative = mr_need(&c_num, false, &format!("mode {label}: cumulative fraction along {d}"))?;
            if c_num.value.is_some_and(|v| v < 0.0) {
                return Err(format!("mode {label}: the cumulative fraction along {d} is negative"));
            }
            let allowance = ALLOW * ((m + 2) as f64) * U;
            let points: Option<Vec<f64>> = running.iter().map(Reach::mr_point).collect();
            if let (Some(points), Some(c)) = (points, cumulative.mr_point()) {
                let s: f64 = points.iter().sum();
                if !mr_tol_close(c, s, allowance * s) {
                    return Err(format!("mode {label}: cumulative fraction {d} = {c:e} is not the running sum {s:e}"));
                }
                continue;
            }
            let derived = mr_sum(running.iter().cloned()).mr_slack(allowance);
            if !mr_overlap(&cumulative, &derived) {
                return Err(format!("mode {label}: cumulative fraction {d} is not the running sum for any value its bounds allow"));
            }
            scope.push(format!("{MR_UNRESOLVED}mode {label}: the cumulative fraction along {d} is undecidable on the published bounds"));
        }
    }
    let (Some(entries), Some(shapes)) = (&p.mass, &p.shapes) else {
        scope.push("mass_matrix or shapes withheld: m_i and φᵀp_k are not re-derived".to_string());
        return Ok(scope);
    };
    for (m, mode) in p.modes.iter().enumerate() {
        let label = m + 1;
        // m and φᵀp_k are formed from the PUBLISHED shape (a flagged component
        // is its published 0), modal_quantities.rs:377, :398 and :426.
        let mut phi = Vec::with_capacity(shapes[m].len());
        for s in &shapes[m] {
            phi.push(s.value.ok_or_else(|| format!("mode {label}: a shape component is not published"))?);
        }
        // One variable per entry. m = Σ_ij φ_i M_ij φ_j over the full symmetric
        // M_ff gives an off-diagonal entry the single coefficient 2·φ_i·φ_j.
        // φᵀp_k, p_k = (M R_k)_f (stiffness.rs:255, mass.rs:147), meets an entry
        // in row i (free, j along k) and in row j (free, i along k): the
        // coefficients are combined into one, and the allowance is held to the
        // sum of their magnitudes (each row of p_k is rounded on its own).
        let free: Vec<bool> = p.free.iter().map(Option::is_some).collect();
        let positions: Vec<(usize, usize)> = entries.iter().map(|(i, j, _)| (*i, *j)).collect();
        let (combined, incidences) = mr_excitation_terms(&free, &positions, &phi);
        let mut mass_terms: Vec<Reach> = Vec::new();
        // Per direction: (combined coefficient, Σ|coefficients|, the entry).
        let mut excitation: [Vec<(f64, f64, Reach)>; 3] = Default::default();
        let mut consumed: [Vec<Reach>; 4] = Default::default();
        let mut counts = [incidences[0], incidences[1], incidences[2], 0];
        let mut cursor = [0usize; 3];
        for (index, (i, j, v)) in entries.iter().enumerate() {
            let (i, j) = (*i, *j);
            let in_mass = free[i] && free[j];
            let hits: Vec<(usize, f64, f64)> = (0..3)
                .filter_map(|k| {
                    let t = combined[k].get(cursor[k]).filter(|t| t.0 == index).copied();
                    if t.is_some() {
                        cursor[k] += 1;
                    }
                    t.map(|(_, c, w)| (k, c, w))
                })
                .collect();
            if !in_mass && hits.is_empty() {
                continue;
            }
            let entry = mr_need(v, i != j, &format!("mass entry ({i}, {j})"))?;
            if in_mass {
                counts[3] += 1;
                consumed[3].push(entry.clone());
                if let Some(term) = mr_generalized_mass_term(i, j, &phi, &entry) {
                    mass_terms.push(term);
                }
            }
            for (k, c, w) in hits {
                consumed[k].push(entry.clone());
                if w != 0.0 || entry.wild {
                    excitation[k].push((c, w, entry.clone()));
                }
            }
        }
        let gm_num = Num::mr_from(&mode.generalized_mass);
        let gm = mr_need(&gm_num, false, &format!("mode {label}: m"))?;
        mr_generalized_mass_check(&mut scope, label, &gm_num, &gm, &mass_terms, counts[3])?;
        for k in 0..3 {
            let d = DIRECTIONS[k];
            let row = &p.participation[m][k];
            let (g_num, e_num) = (Num::mr_from(&row.gamma), Num::mr_from(&row.effective));
            let gamma = mr_need(&g_num, true, &format!("mode {label}: Γ along {d}"))?;
            let effective = mr_need(&e_num, false, &format!("mode {label}: m_eff along {d}"))?;
            let e = ALLOW * ((counts[k] + 6) as f64) * U;
            let points: Option<Vec<(f64, f64, f64)>> =
                excitation[k].iter().map(|(c, w, x)| x.mr_point().filter(|x| mr_kept(c * x, &[*c, *x])).map(|x| (*c, *w, x))).collect();
            let points = points.map(|points| points.iter().fold((0.0f64, 0.0f64), |(s, a), (c, w, x)| (s + c * x, a + w * x.abs())));
            let decided = match (points, gm.mr_point(), gamma.mr_point(), effective.mr_point()) {
                (Some((s, a)), Some(published), Some(g), Some(eff))
                    if mr_kept(g * published, &[g, published]) && mr_kept(eff * published, &[eff, published]) && mr_kept(s * s, &[s, s]) =>
                {
                    Some((s, a, published, g, eff))
                }
                _ => None,
            };
            if let Some((s, a, published, g, eff)) = decided {
                let gm_ = g * published;
                if !mr_tol_close(gm_, s, e * (a + gm_.abs())) {
                    return Err(format!("mode {label}: Γ_{d}·m = {gm_:e} is not φᵀp_{d} = {s:e}"));
                }
                let em = eff * published;
                if !mr_tol_close(em, s * s, e * (2.0 * s.abs() * a + em.abs()) + (e * a) * (e * a)) {
                    return Err(format!("mode {label}: m_eff,{d}·m = {em:e} is not (φᵀp_k)² = {:e}", s * s));
                }
                continue;
            }
            // Each entry once: c·x ± e·w·|x| is exactly x·[c − e·w, c + e·w] (the
            // allowance is proportional to the same x).
            let s = mr_sum(excitation[k].iter().map(|(c, w, x)| mr_join(&Reach::mr_new(vec![(c - e * w, c + e * w, false)], false), x, true)));
            let mut operands: Vec<&Reach> = vec![&gamma, &gm];
            operands.extend(consumed[k].iter());
            let flag = |x: &Num| if x.below { " (flagged below-normal)" } else { "" };
            mr_quotient(&mut scope, &format!("mode {label}: Γ·m = φᵀp_k along {d}"), &operands, &mr_join(&gamma, &gm, true), &s, e, true,
                format!("mode {label}: Γ_{d}{} disagrees with φᵀp_k from the payload for any value the bounds allow", flag(&g_num)))?;
            operands[0] = &effective;
            mr_quotient(&mut scope, &format!("mode {label}: m_eff·m = (φᵀp_k)² along {d}"), &operands, &mr_join(&effective, &gm, true), &s.mr_squared(), e, true,
                format!("mode {label}: m_eff,{d}{} disagrees with φᵀp_k from the payload for any value the bounds allow", flag(&e_num)))?;
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
    if total == 0 {
        return Err("a solved document has at least one computed mode".into());
    }
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
        if !(omega > 0.0 && omega.is_finite() && f > 0.0 && f.is_finite()) {
            return Err(format!("computed mode {label}: ω and f are positive and finite (ω = {omega:e}, f = {f:e})"));
        }
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
        let period = mode.period.unwrap_or(f64::NAN);
        if !(f > 0.0 && f.is_finite() && period > 0.0 && period.is_finite()) {
            return Err(format!("mode {}: f and T are positive and finite (f = {f:e}, T = {period:e})", m + 1));
        }
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
    for (c, (first, last, reported, estimate)) in p.clusters.iter().enumerate() {
        let (first, last, reported) = (*first, *last, *reported);
        let id = c + 1;
        let members: Vec<usize> = (0..total).filter(|&i| p.computed[i].cluster == id).collect();
        if members.first().map(|i| i + 1) != Some(first) || members.last().map(|i| i + 1) != Some(last) {
            return Err(format!("cluster {id}: its first and last computed modes are not its members'"));
        }
        if reported != (first <= p.modes.len()) {
            return Err(format!("cluster {id}: its reported flag is not the set's"));
        }
        // The cluster's e_λ is its members' NaN-propagating maximum (the
        // producer's `max_propagating_nan`): bit for bit when all are published.
        mr_domain(estimate, MrRule::NormalOrZero, &format!("cluster {id}: e_λ"))?;
        let mut sets = Vec::with_capacity(members.len());
        for &i in &members {
            sets.push(mr_need(&p.computed[i].estimate, false, &format!("computed mode {}: e_λ", i + 1))?);
        }
        let stated = mr_need(estimate, false, &format!("cluster {id}: e_λ"))?;
        let points: Option<Vec<f64>> = sets.iter().map(Reach::mr_point).collect();
        match (points, stated.mr_point()) {
            (Some(points), Some(v)) => {
                if Some(v) != points.iter().copied().reduce(f64::max) {
                    return Err(format!("cluster {id}: its e_λ is not its members' largest"));
                }
            }
            _ => {
                let any_wild = sets.iter().any(|s| s.wild);
                // All finite: the maximum ranges over [max of lows, max of highs];
                // a non-finite member makes it non-finite.
                let maximum = if any_wild {
                    Reach { parts: Vec::new(), wild: true }
                } else {
                    let lo = sets.iter().map(Reach::mr_lowest).fold(f64::NEG_INFINITY, f64::max);
                    let hi = sets.iter().map(Reach::mr_highest).fold(f64::NEG_INFINITY, f64::max);
                    let nonzero = sets.iter().all(|s| s.parts.iter().all(mr_part_nonzero));
                    Reach::mr_new(vec![(lo, hi, nonzero)], false)
                };
                if !mr_overlap(&stated, &maximum) {
                    return Err(format!("cluster {id}: its e_λ is not its members' largest for any value the bounds allow"));
                }
                scope.push(format!("{MR_UNRESOLVED}cluster {id}: its e_λ against its members' is undecidable on the published bounds"));
            }
        }
    }
    for (m, mode) in p.modes.iter().enumerate() {
        mr_domain(&mode.cluster_estimate, MrRule::NormalOrZero, &format!("mode {}: its cluster e_λ", m + 1))?;
        if Some(&mode.cluster_estimate) != p.clusters.get(mode.cluster.wrapping_sub(1)).map(|c| &c.3) {
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
        // The kernel breaks a cluster where fl(λ_b − λ_a) > fl(δ_c·λ_b)
        // (clusters.rs:71); the published ω = fl(√λ), so each fl(ω²) is within
        // 3u·λ (+O(u²)) of its λ. With the subtractions and products rounded,
        // reader and kernel differ by at most (8u + 5u·δ_c)·λ_b + O(u²);
        // 16u(1 + δ_c)·ω_b² covers it, and the sum with the threshold, for
        // every admitted δ_c in (0, 1).
        let gap = b * b - a * a;
        let threshold = delta * (b * b);
        let margin = 16.0 * U * (1.0 + delta) * (b * b);
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

/// The factored values of a Sturm candidate (the formation term aside).
const MR_FACTORED: [&str; 9] = ["factor term", "backward error", "inverse-norm estimate", "conditioning product", "separation ratio", "growth",
    "max|L|", "max|d|", "max|ã|"];

fn mr_factored<'a, 'v>(c: &'a Candidate<'v>) -> [&'a Num<'v>; 9] {
    [&c.factor_term, &c.backward_error, &c.inverse_norm, &c.product, &c.separation, &c.growth, &c.maxima[0], &c.maxima[1], &c.maxima[2]]
}

/// A value the producer forms by one + or ·: bit for bit when every operand
/// is published, else within 8u on sets (a non-finite operand gives a
/// non-finite result).
fn mr_formed(scope: &mut Vec<String>, label: &str, lhs: &Reach, a: &Reach, b: &Reach, product: bool, relation: &str, contradiction: &str) -> R<()> {
    if let (Some(l), Some(x), Some(y)) = (lhs.mr_point(), a.mr_point(), b.mr_point()) {
        let v = if product { x * y } else { x + y };
        if l != v {
            return Err(format!("{label}: {relation} does not hold"));
        }
        return Ok(());
    }
    if !mr_overlap(lhs, &mr_join(a, b, product).mr_slack(8.0 * U)) {
        return Err(format!("{label}: {contradiction}"));
    }
    scope.push(format!("{MR_UNRESOLVED}{label}: {relation} is undecidable on the published bounds"));
    Ok(())
}

/// One candidate: its claims, its two formulas and its decision against the
/// producer's three tests, each on its own operands.
fn mr_candidate(scope: &mut Vec<String>, c: &Candidate, label: usize, sturm_count: usize, limit: f64, g_max: f64, ks: f64) -> R<()> {
    for (name, v) in [("shift", &c.shift), ("formation term", &c.formation_term), ("formation separation", &c.formation_separation)].into_iter().chain(MR_FACTORED.iter().copied().zip(mr_factored(c))) {
        mr_domain(v, MrRule::Binary64, &format!("candidate {label}: {name}"))?;
    }
    let Some(pivots) = c.pivots else {
        for (name, v) in MR_FACTORED.iter().zip(mr_factored(c)) {
            if v.value.is_some() || v.reason.as_deref() != Some("not_factored") {
                return Err(format!("candidate {label}: not factored, but its {name} is not null `not_factored`"));
            }
        }
        if matches!(c.decision, "accepted" | "growth_limit" | "ill_conditioned") {
            return Err(format!("candidate {label}: decision `{}` needs a factorization", c.decision));
        }
        return Ok(());
    };
    let formation = mr_reach(&c.formation_term, false)
        .ok_or_else(|| format!("candidate {label}: its formation term is null without a profile bound"))?;
    let mut sets = Vec::with_capacity(9);
    for (name, v) in MR_FACTORED.iter().zip(mr_factored(c)) {
        match mr_reach(v, false) {
            Some(s) => sets.push(s),
            // `no_neighbour` claims no neighbouring computed eigenvalue; every
            // placement has one (the gap's upper mode, or the top mode).
            None if *name == "separation ratio" => {
                return Err(format!("candidate {label}: its separation ratio is null `{}`, which a factored candidate with a neighbour cannot be",
                    v.reason.as_deref().unwrap_or("")))
            }
            None => return Err(format!("candidate {label}: its {name} is null without a profile bound")),
        }
    }
    let (factor, be, nu, product, separation, growth) = (&sets[0], &sets[1], &sets[2], &sets[3], &sets[4], &sets[5]);
    let here = format!("candidate {label}");
    mr_formed(scope, &here, be, factor, &formation, false, "e_b = factor term + formation term", "e_b = factor term + formation term is contradicted by the bounds")?;
    mr_formed(scope, &here, product, be, nu, true, "conditioning product = e_b·ν", "the conditioning product is contradicted by the bounds of e_b·ν")?;
    // separated: s ≥ κ_s·e_b, within 8u of the band (a non-finite ratio has
    // e_b = 0, so is separated); growth and conditioning: the producer's ≤,
    // false for ±∞ and NaN.
    let separated = (separation.wild || separation.parts.iter().any(|p| p.1 >= ks * (1.0 - 8.0 * U)),
        separation.parts.iter().any(|p| p.0 < ks * (1.0 + 8.0 * U)));
    let within = growth.mr_can(|g| g <= g_max, (false, true));
    let conditioned = product.mr_can(|x| x <= limit, (false, true));
    match c.decision {
        "accepted" => {
            if pivots != sturm_count {
                return Err(format!("the accepted candidate's negative-pivot count {pivots} is not the Sturm count {sturm_count}"));
            }
            for (name, (yes, no), failure) in [("conditioning product", conditioned, "fails the conditioning premise"), ("growth", within, "exceeds the growth limit"),
                ("separation ratio", separated, "fails the separation premise")] {
                if !yes {
                    return Err(format!("the accepted shift {failure}: its {name} cannot meet it"));
                }
                if no {
                    scope.push(format!("{MR_UNRESOLVED}the accepted {name} is undecidable against its premise on the published bounds"));
                }
            }
        }
        "growth_limit" if !separated.0 || !within.1 => return Err(format!("candidate {label}: refused for growth within the limit")),
        "ill_conditioned" if !separated.0 || !within.0 || !conditioned.1 => {
            return Err(format!("candidate {label}: refused as ill-conditioned within the premise"))
        }
        "unseparated" if !separated.1 => return Err(format!("candidate {label}: refused as unseparated with its separation above κ_s")),
        _ => {}
    }
    Ok(())
}

/// Sturm: the candidates, the accepted shift's premises and placement.
fn mr_sturm(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    let s_max = p.params.and_then(|m| m.get("sturm_max_candidates")).and_then(Value::as_u64).ok_or("s_max: not bound")? as usize;
    let kc = p.kappa_c.ok_or("κ_c: not published")?;
    let limit = p.limit.ok_or("1/κ_c: not published")?;
    let ks = p.kappa_s.ok_or("κ_s: not published")?;
    let g_max = p.growth_limit.ok_or("g_max: not published")?;
    if !(kc >= 1.0 && ks >= 1.0 && g_max >= 1.0) {
        return Err("κ_c, κ_s and g_max are each at least 1".into());
    }
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
        mr_candidate(&mut scope, c, label, p.sturm_count, limit, g_max, ks)?;
    }
    let n = p.modes.len();
    let omega = |i: usize| p.computed.get(i).and_then(|c| c.omega);
    // The accepted shift's values: finite, so a non-finite or claim-null one
    // is in no gap.
    let shift = match p.candidates.last() {
        Some(c) => Some((mr_need(&c.shift, false, "the accepted shift")?, c.shift.value.is_none())),
        None => None,
    };
    if shift.as_ref().is_some_and(|(s, _)| s.parts.is_empty()) {
        return Err("the accepted shift is not finite".into());
    }
    match p.endpoint {
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
                // The producer forms ω_c = fl(2π·f_c), λ_c = fl(ω_c²) (outcome.rs:310) and
                // publishes fl(√λ_c). Where ω_c² is below the normal range (f_c below about
                // 2.4·10⁻¹⁵⁵ Hz) the square keeps fewer significant bits, so no relative
                // allowance about 2π·f_c holds: the same binary64 operations, bit for bit.
                let omega_c = TWO_PI * cut;
                // The producer refuses λ_c unless it is finite and positive
                // (`InvalidGap`, sturm.rs), so a square that underflows to zero
                // or overflows has no solved document.
                let square = omega_c * omega_c;
                if !(square.is_finite() && square > 0.0) {
                    return Err("the cut-off's square fl((2π·f_c)²) is not a finite positive eigenvalue, which the producer refuses".into());
                }
                let expected = square.sqrt();
                if lower.to_bits() != expected.to_bits() {
                    return Err("the gap's lower end is not √fl((2π·f_c)²)".into());
                }
            }
            if let Some((shift, null)) = &shift {
                if !mr_overlap(shift, &Reach::mr_new(vec![(lower, upper, false)], false)) {
                    return Err(if *null { "the accepted shift is outside the profile (ω ≥ 2⁵³), above its gap" } else { "the accepted shift is not in its gap" }.into());
                }
                if shift.mr_point().is_none() {
                    scope.push(format!("{MR_UNRESOLVED}the accepted shift's place in its gap is undecidable on the published bounds"));
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
            if let Some((shift, _)) = &shift {
                if shift.mr_highest() < lower {
                    return Err("the accepted whole-spectrum shift is below the top mode".into());
                }
                if shift.mr_point().is_none() {
                    scope.push(format!("{MR_UNRESOLVED}the accepted shift's place above the spectrum is undecidable on the published bounds"));
                }
            }
        }
    }
    Ok(scope)
}

/// Evidence: e_λ = η·κ_λ, the identity discrepancies, the formation values.
fn mr_evidence(p: &Parsed) -> R<Vec<String>> {
    let mut scope = Vec::new();
    for (i, c) in p.computed.iter().enumerate() {
        let nums = [&c.eta, &c.condition, &c.estimate];
        for (name, v) in ["η", "κ_λ", "e_λ"].iter().zip(nums) {
            mr_domain(v, MrRule::NormalOrZero, &format!("computed mode {}: {name}", i + 1))?;
        }
        let mut sets = Vec::with_capacity(3);
        for v in nums {
            sets.push(mr_reach(v, false).ok_or_else(|| format!("computed mode {}: a check-2 value is null without a profile bound", i + 1))?);
        }
        if nums.iter().any(|v| v.value.is_some_and(|x| x < 0.0)) {
            return Err(format!("computed mode {}: a negative check-2 value", i + 1));
        }
        // The producer multiplies in extended range and then requires 0 or
        // normal (reformation/mod.rs:964): a nonzero product below the normal
        // range is no published 0, so the point path decides only a kept product.
        let kept = match (sets[0].mr_point(), sets[1].mr_point()) {
            (Some(a), Some(b)) => mr_kept(a * b, &[a, b]),
            _ => true,
        };
        let (Some(eta), Some(kappa), Some(e), true) = (sets[0].mr_point(), sets[1].mr_point(), sets[2].mr_point(), kept) else {
            // e_λ is the producer's η·κ_λ: a non-finite factor gives a non-finite e_λ.
            let product = mr_join(&sets[0], &sets[1], true);
            if !(sets[2].wild && product.wild) && !mr_near(&sets[2], &product.mr_reals(), 8.0 * U) {
                return Err(format!("computed mode {}: e_λ = η·κ_λ is contradicted by the bounds", i + 1));
            }
            scope.push(format!("{MR_UNRESOLVED}computed mode {}: e_λ = η·κ_λ is undecidable on the published bounds", i + 1));
            continue;
        };
        let product = eta * kappa;
        if !mr_tol_close(e, product, 8.0 * U * e.max(product)) {
            return Err(format!("computed mode {}: e_λ = {e:e} is not η·κ_λ = {product:e}", i + 1));
        }
    }
    mr_identity_coverage(p)?;
    mr_state_coverage(p)?;
    if p.linearized == Some(true) {
        // The supports table's states, and the partition the identity rows are
        // formed on, are reported: no payload qualifies them (as for the
        // model-derived mass and participation).
        scope.push("supports: the reported nonlinear support states are conditional on their qualification as the static solve's converged, confirmed states, which no payload establishes".to_string());
    }
    if !p.disclosures.iter().any(|(text, source)| *source == "modal" && text.starts_with("numerical integrity does not establish physical correctness")) {
        return Err("disclosures: the fixed numerical-integrity disclosure is missing".into());
    }
    let tolerance = p.params.and_then(|m| m.get("identity_tolerance"));
    let relative = tolerance.and_then(|t| t["relative"].as_f64()).ok_or("the identity tolerance is not bound")?;
    let absolute = tolerance.and_then(|t| t["absolute"].as_f64()).ok_or("the identity tolerance is not bound")?;
    // The relative discrepancy's producer domain (plain binary64, never flagged)
    // holds whatever the tolerance, so it is enforced unconditionally; only the
    // tolerance comparison needs the scale an absolute tolerance does not publish.
    for (i, row) in p.identity.iter().enumerate() {
        mr_domain(&row.relative, MrRule::Binary64, &format!("identity row {i}"))?;
    }
    if absolute == 0.0 {
        for (i, row) in p.identity.iter().enumerate() {
            let d = mr_need(&row.relative, false, &format!("identity row {i}"))?;
            if row.relative.value.is_some_and(|x| x < 0.0) {
                return Err(format!("identity row {i}: a negative relative discrepancy"));
            }
            // The producer's test is d > tolerance: ∞ fails it, NaN does not.
            let (within, beyond) = d.mr_can(|x| x <= relative, (true, true));
            if !within {
                return Err(if row.relative.value.is_none() {
                    format!("identity row {i}: relative discrepancy outside the profile (at least 2⁵³) exceeds the bound tolerance")
                } else {
                    format!("identity row {i}: relative discrepancy exceeds the bound tolerance {relative:e}")
                });
            }
            if beyond && d.wild {
                scope.push(format!("{MR_UNRESOLVED}identity row {i}: its discrepancy against the tolerance is undecidable (not finite)"));
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
    mr_domain(&p.ortho, MrRule::Published, "the orthogonality deviation")?;
    mr_need(&p.ortho, false, "the orthogonality deviation")?;
    if let Some(ortho) = p.ortho.value {
        if ortho < 0.0 {
            return Err("a negative orthogonality deviation".into());
        }
    }
    if p.ortho_pair != (p.modes.len() >= 2) {
        return Err("the orthogonality pair is stated exactly when two modes are reported".into());
    }
    Ok(scope)
}

/// The identity rows the run must carry, from the document's own structure:
/// along each axis, in axis order, one `nodal` row of kind `member` per
/// straight member of the discretization table (and no other nodal row),
/// `resultant` rows for other subjects (arcs, bodies) and the aggregate (kind
/// `model`) last and once; the same (kind, subject) list along every axis. The
/// kind keeps a member named `model` apart from the aggregate. Whether the
/// listed subjects are the model's is the subject census's (`census`).
fn mr_identity_coverage(p: &Parsed) -> R<()> {
    let mut per_axis: [Vec<(&str, &str, &str)>; 3] = Default::default();
    let mut last_axis = 0;
    for (i, row) in p.identity.iter().enumerate() {
        if row.axis < last_axis {
            return Err(format!("identity row {i}: rows are in axis order"));
        }
        last_axis = row.axis;
        per_axis[row.axis].push((row.subject_kind, row.subject, row.comparison));
    }
    let straights: HashSet<&str> = p.members.iter().copied().collect();
    let first: Vec<(&str, &str)> = per_axis[0].iter().map(|(kind, s, _)| (*kind, *s)).collect();
    for (k, rows) in per_axis.iter().enumerate() {
        let axis = DIRECTIONS[k];
        let Some((&last, before)) = rows.split_last() else {
            return Err(format!("identity: no rows along {axis}; the model's resultant row is missing"));
        };
        if last != ("model", "model", "resultant") {
            return Err(format!("identity: the model's aggregate resultant row along {axis} is missing or not last"));
        }
        let mut seen = HashSet::new();
        for &(kind, subject, comparison) in before {
            if kind == "model" {
                return Err(format!("identity: a second aggregate row along {axis}"));
            }
            if !seen.insert((kind, subject)) {
                return Err(format!("identity: {kind} {subject} is compared twice along {axis}"));
            }
            let straight = kind == "member" && straights.contains(subject);
            if (comparison == "nodal") != straight {
                return Err(format!("identity: {kind} {subject} along {axis} is compared {comparison}, but the discretization table's straight members are compared nodally and only they"));
            }
        }
        for member in &p.members {
            if !seen.contains(&("member", *member)) {
                return Err(format!("identity: straight member {member} has no nodal row along {axis}"));
            }
        }
        let subjects: Vec<(&str, &str)> = rows.iter().map(|(kind, s, _)| (*kind, *s)).collect();
        if subjects != first {
            return Err(format!("identity: the subjects along {axis} are not those along x"));
        }
    }
    Ok(())
}

/// The nonlinear states against the partition: a linearized state case lists
/// its supports, its restrained set is exactly the document's restrained
/// DOFs, and every rigid treatment's DOF is in it; no state, no supports.
fn mr_state_coverage(p: &Parsed) -> R<()> {
    match p.linearized {
        Some(true) => {
            if p.supports.is_empty() {
                return Err("supports: a linearized state case lists its supports".into());
            }
            let restrained: HashSet<usize> = (0..p.free.len()).filter(|&d| p.free[d].is_none()).collect();
            let listed: HashSet<usize> = p.restrained_refs.iter().copied().collect();
            if listed != restrained || listed.len() != p.restrained_refs.len() {
                return Err("supports: the linearized restrained set is not the document's restrained DOFs".into());
            }
            for (dof, treatment) in &p.supports {
                if (*treatment == "restrained" || *treatment == "already_restrained") && !listed.contains(dof) {
                    return Err(format!("supports: a {treatment} support's DOF {dof} is free"));
                }
            }
        }
        _ => {
            if !p.supports.is_empty() {
                return Err("supports: support states without a linearized state case".into());
            }
        }
    }
    Ok(())
}

/// Standing: each mode Sensitive exactly when η > τ_r, the set exactly when
/// a mode is or the orthogonality deviation exceeds τ_o.
fn mr_standing(p: &Parsed) -> R<Vec<String>> {
    let tau_r = mr_param(p, "residual_threshold").ok_or("τ_r: not bound")?;
    let tau_o = mr_param(p, "orthogonality_threshold").ok_or("τ_o: not bound")?;
    // The producer's tests are η > τ_r and the deviation > τ_o: true for +∞,
    // false for NaN.
    let mut scope = Vec::new();
    for (m, mode) in p.modes.iter().enumerate() {
        let eta = mr_need(&p.computed[m].eta, false, &format!("mode {}: η", m + 1))?;
        let (yes, no) = eta.mr_can(|x| x > tau_r, (true, true));
        if !(if mode.sensitive { yes } else { no }) || (mode.reasons > 0) != mode.sensitive {
            return Err(format!("mode {}: Sensitive exactly when η > τ_r, with its reason", m + 1));
        }
        if yes && no {
            scope.push(format!("{MR_UNRESOLVED}mode {}: η > τ_r is undecidable (η not finite)", m + 1));
        }
    }
    // The set: Sensitive when a mode's token is, or the deviation exceeds τ_o.
    let (yes, no) = if p.modes.iter().any(|m| m.sensitive) {
        (true, false)
    } else {
        mr_need(&p.ortho, false, "the orthogonality deviation")?.mr_can(|x| x > tau_o, (true, true))
    };
    mr_need(&p.ortho, false, "the orthogonality deviation")?;
    if !(if p.sensitive { yes } else { no }) || (p.set_reasons > 0) != p.sensitive {
        return Err("the set is Sensitive exactly when a mode is or the orthogonality deviation exceeds τ_o".into());
    }
    if yes && no {
        scope.push(format!("{MR_UNRESOLVED}the set's standing is undecidable (the deviation is not finite)"));
    }
    Ok(scope)
}

/// Units: every value object against modal-1. Every kind has its catalog
/// unit, so nothing is scoped.
fn mr_units(p: &Parsed) -> R<Vec<String>> {
    for u in &p.unitized {
        mr_unit_check(u.value, u.key, &u.path)?;
    }
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bessel_comparison_is_outward_rounded_at_the_strict_boundary() {
        // Review of f5c66b167: with T = 1.3 and effective masses [1.3000000000000094, 0],
        // exact arithmetic puts the sum 8.88e-17 above T·(1 + 16(n+2)u). A round-to-nearest
        // comparison passed it; the outward comparison must not establish it.
        let t = 1.3_f64;
        let n = 2;
        let exact_excess = {
            // Σ m_eff − T(1 + 16(n+2)u), formed exactly from the binary64 operands as an
            // integer multiple of 2^-104 (all values here are of that granularity).
            let scale = 2f64.powi(104);
            let sum = (1.3000000000000094_f64 * scale) as i128;
            let allowed = (t * scale) as i128 + ((t * scale) as i128 * 16 * (n as i128 + 2)) / (1_i128 << 53);
            sum - allowed
        };
        assert!(exact_excess > 0, "the boundary witness must exceed the allowance exactly");
        assert!(!mr_bessel_established(&[1.3000000000000094, 0.0], t, n));
        // A clearly established case still passes.
        assert!(mr_bessel_established(&[0.6, 0.6], t, n));
        assert!(mr_bessel_established(&[1.3, 0.0], t, n));
    }

    #[test]
    fn the_readers_table_names_canonical_catalog_units() {
        assert_eq!(modal_reader_catalog_differences(), Vec::<String>::new());
    }

    /// Third re-check, finding 3: a free/free off-diagonal entry meets φᵀp_k
    /// in two rows. With coefficients +1 and −1 it is ONE term with
    /// coefficient exactly 0 (its allowance held to the magnitudes), so with
    /// m = 2⁻¹⁰²² and Γ = 0.5 the relation Γ·m = φᵀp_k is contradicted even
    /// for a flagged (nonzero, below 2⁻¹⁰²²) entry.
    #[test]
    fn one_mass_entry_is_one_variable_in_its_excitation_sum() {
        let free = vec![true; 13];
        let mut phi = vec![0.0; 13];
        phi[6] = 1.0;
        phi[12] = -1.0;
        let (terms, counts) = mr_excitation_terms(&free, &[(6, 12)], &phi);
        assert_eq!((terms[0].clone(), counts[0]), (vec![(0, 0.0, 2.0)], 2));
        let flagged = mr_reach(&Num { value: Some(0.0), reason: None, below: true }, true).unwrap();
        let e = ALLOW * ((counts[0] + 6) as f64) * U;
        let s = mr_join(&Reach::mr_new(vec![(-e * 2.0, e * 2.0, false)], false), &flagged, true);
        let gamma_m = mr_join(&Reach::mr_exact(0.5), &Reach::mr_exact(f64::MIN_POSITIVE), true);
        assert!(!mr_overlap(&gamma_m.mr_slack(e), &s));
        // Two independent unknowns would admit it: the identity decides.
        let independent = mr_sum([mr_join(&Reach::mr_exact(1.0), &flagged, true), mr_join(&Reach::mr_exact(-1.0), &flagged, true)]);
        assert!(mr_overlap(&gamma_m.mr_slack(e), &independent));
    }

    /// Class confirmation of cab0ea356, finding 1: the producer sums m's
    /// triple products exactly (`mass_bilinear`, modal_quantities.rs:294).
    /// Its probe (`modal_producer_edges.json`, from `normalize_mode`: a unit
    /// rotation, a 2⁻⁶⁰⁰ translation, mass 1 on that translation only) has
    /// m = 2⁻¹²⁰⁰, a flagged 0. The reader's own m reconstruction keeps the
    /// underflowing coefficient nonzero and accepts it as unresolved.
    #[test]
    fn the_producers_underflowing_generalized_mass_is_accepted_unresolved() {
        let edges: Value = serde_json::from_str(include_str!("../../../../fixtures/results/modal_producer_edges.json")).unwrap();
        let probe = &edges["relation_probes"][0];
        let mut phi = vec![0.0; 12];
        for (dof, vo) in probe["global_dofs"].as_array().unwrap().iter().zip(probe["shape"].as_array().unwrap()) {
            assert_eq!(vo["below_normal"], false);
            phi[dof.as_u64().unwrap() as usize] = vo["value"].as_f64().unwrap();
        }
        let entry = &probe["mass_entries"][0];
        let (i, j, value) = (entry[0].as_u64().unwrap() as usize, entry[1].as_u64().unwrap() as usize, entry[2].as_f64().unwrap());
        assert_eq!(phi[i] * phi[j], 0.0, "the binary64 coefficient underflows");
        let term = mr_generalized_mass_term(i, j, &phi, &Reach::mr_exact(value)).unwrap();
        let gm_num = Num { value: probe["generalized_mass"]["value"].as_f64(), reason: None, below: probe["generalized_mass"]["below_normal"] == true };
        let gm = mr_reach(&gm_num, false).unwrap();
        let mut scope = Vec::new();
        mr_generalized_mass_check(&mut scope, 1, &gm_num, &gm, &[term], 1).unwrap();
        assert!(scope.first().is_some_and(|x| x.starts_with(MR_UNRESOLVED)), "{scope:?}");
    }

    /// Agent 0's Bessel contract: the re-formed M̂'s deviation bounds nothing
    /// on the assembled M. M = I, M̂ = [[1, −1], [−1, 2]], φ₁ = (1, 0),
    /// φ₂ = (1, 1), r = (1, 1): exactly M̂-orthogonal, yet on M
    /// Σ m_eff = 1 + 2 = 3 > T = 2. The corpus carries these modes with
    /// T_x = 3.5 (passed), T_x = 2 (unresolved), each with a wrong Γ²m too
    /// (refused).
    #[test]
    fn bessel_correlated_mode_witness() {
        let dot = |a: [f64; 2], b: [f64; 2], m: [[f64; 2]; 2]| (0..2).flat_map(|i| (0..2).map(move |j| a[i] * m[i][j] * b[j])).sum::<f64>();
        let (m, m_hat, r) = ([[1.0, 0.0], [0.0, 1.0]], [[1.0, -1.0], [-1.0, 2.0]], [1.0, 1.0]);
        let phi = [[1.0, 0.0], [1.0, 1.0]];
        assert!(dot(phi[0], phi[1], m_hat) == 0.0 && dot(phi[0], phi[1], m) != 0.0);
        let total: f64 = phi.iter().map(|p| dot(*p, r, m).powi(2) / dot(*p, *p, m)).sum();
        assert_eq!((total, dot(r, r, m)), (3.0, 2.0));
    }

    /// Third re-check, finding 1: a flagged value is nonzero and unbounded
    /// below; a nonzero product that underflows is never 0.
    #[test]
    fn a_flagged_value_is_nonzero_and_unbounded_below() {
        let flagged = mr_reach(&Num { value: Some(0.0), reason: None, below: true }, false).unwrap();
        assert!(!mr_overlap(&flagged, &Reach::mr_exact(0.0)));
        let product = mr_join(&Reach::mr_exact(3.37e-99), &Reach::mr_exact(1e-300), true);
        assert!(product.mr_point().is_none() && !mr_overlap(&product, &Reach::mr_exact(0.0)) && mr_overlap(&product, &flagged));
    }
}
