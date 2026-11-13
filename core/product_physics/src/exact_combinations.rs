//! W-16 slice b (DEL-05-02 Design `exact-combinations.md`): model-authored load
//! combinations on the exact pressure route (pressure-2, FR-SOL-009).
//!
//! A combination is formed after every primitive case has published its rows,
//! from those rows and the case evidence the envelope publishes beside them:
//! - **Pairing (§3).** Rows pair by a typed structural key from their structured
//!   fields (kind, unit, entity, station, frame, component and section basis);
//!   row ids are never parsed. A key carried by more than one row of a case
//!   (the tie rods of one connector) cannot pair; tie-rod rows are withheld by
//!   name.
//! - **Pressure-free operand recovery (§3).** A pressure-family row that an
//!   operand lacks is recovered at p = 0 when that member is published in the
//!   operand's case and lies in none of its pressure regions: pressure-only
//!   quantities are zero; N_w = S = N from the operand's axial-force rows
//!   (negated at end_i); the end action is the end row itself; the membrane is
//!   N divided once by the basis's published area leaf. Every recovered value
//!   is a witness in the evidence. Any other absence is a missing source.
//! - **Gates (§4, mechanics only, in precedence order):** mixed modulus basis
//!   (as on PF), mixed resolved member stiffness, a nonzero installed
//!   connector or tie-rod reference with factors that fail the shared unit
//!   factor-sum rule (`load_case_algebra::factor_sum_within_unit_tolerance`),
//!   and (RT-A3 increment 3) a consuming constant-effort support with factors
//!   that fail the same rule ([`CONSTANT_EFFORT`], PF's reason and text).
//! - **Standing (§5).** Combination rows are derived from case results; they
//!   have no numerical check of their own and never enter headlines, maxima or
//!   numerical eligibility. The evidence states each operand's standing.
//!
//! Every value is formed by the load-case algebra PF uses
//! (`open_pipe_stress_load_case_algebra`); a combined magnitude is the norm of
//! the combined components, except in a range envelope, which selects it.

use super::*;
use combination_plan::Target;
use serde_json::{json, Value};

/// `exact_combinations[].standing` (§5).
pub(crate) const STANDING: &str = "derived_from_case_results";
/// The provenance of a recovered operand value (§3).
pub(crate) const RECOVERY_METHOD: &str = "pressure_free_operand_recovery";
/// `exact_combinations[].source_recovery` (§7).
pub(crate) const SOURCE_RECOVERY_EXCLUSION: &str = "excluded: a document with a load combination attempts no retained-source recovery for any of its cases, primitive cases included; every case publishes ordinary structural recovery";
/// §4 gate 1, as on PF.
pub(crate) const MODULUS_BASIS_MIXED: &str = "COMBINATION_MODULUS_BASIS_MIXED";
/// §4 gate 2.
pub(crate) const RESOLVED_STIFFNESS_MIXED: &str = "COMBINATION_RESOLVED_STIFFNESS_MIXED";
/// absolute-sum-srss §5.2 on this route.
pub(crate) const NONLINEAR: &str = "NONLINEAR_COMBINATION_REQUIRES_SOLVE";
/// §4 gate 3.
pub(crate) const INSTALLED_REFERENCE: &str = "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE";
/// absolute-sum-srss §5.1: gate 3 for an absolute sum or SRSS whose operands' κ is
/// not exactly 0.
pub(crate) const INSTALLED_REFERENCE_UNSIGNED: &str = "INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND";
/// §4 gate 4 (RT-A3 increment 3; DEL-04-07 `pressure-contract.md`,
/// "Constant-effort supports (RT-A3 increment 3)"): PF's reason, on EX.
///
/// Order: last. The stiffness gates (1, 2) decide whether the operands are
/// states of one system at all, which no choice of factors repairs; the two
/// factor-sum gates (3, 4) presuppose one system and decide whether a fixed
/// load is counted once. Gate 4 shares gate 3's predicate and follows it, so
/// every reason published before this increment is unchanged and the pressure
/// table's gate list is extended by appending. PF has no stiffness or
/// connector gates and orders this reason before its modulus gate; where both
/// hold the two routes name different reasons for the same withholding.
pub(crate) const CONSTANT_EFFORT: &str = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE";
/// absolute-sum-srss §5.1: gate 4 for an absolute sum or SRSS whose operands' κ is
/// not exactly 0, in a model with a consuming constant-effort support (PF's code).
pub(crate) const CONSTANT_EFFORT_UNSIGNED: &str = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND";
/// absolute-sum-srss §5.1: gate 4 for a `mechanics` combination naming an operand
/// whose κ is undefined, in a model with a consuming constant-effort support (PF's
/// code and text).
pub(crate) const CONSTANT_EFFORT_OPERAND_MIXED: &str = preview_physics::CONSTANT_EFFORT_OPERAND_MIXED;
/// §5: the product text of a published combination.
pub(crate) const DERIVED: &str = "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS";
/// §7: the user-facing statement of the source-recovery exclusion.
pub(crate) const SOURCE_RECOVERY_EXCLUDED: &str = "EXACT_COMBINATION_SOURCE_RECOVERY_EXCLUDED";
const SOURCE_MISSING: &str = "LOAD_COMBINATION_SOURCE_RESULT_MISSING";
const ROW_WITHHELD: &str = "LOAD_COMBINATION_ROW_WITHHELD";
const SUBTRACTION_SIGN: &str = "positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; a signed difference of solved states, not an equilibrium state";
const SECTION_SEPARATOR: &str = "; section_basis=";

/// The pressure-2 table's `signed_linear` kinds (its `combination_policy.linear_state_kinds`;
/// pinned to the table's `combination_algebra` by this module's test).
pub(crate) const SIGNED_LINEAR_KINDS: [&str; 39] = [
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
    "element_local_axial_force",
    "element_local_shear_force_y",
    "element_local_shear_force_z",
    "element_local_torsional_moment",
    "element_local_bending_moment_y",
    "element_local_bending_moment_z",
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
    "support_reaction_component_v2",
    "constant_effort_support_applied_load",
    "element_global_end_force_v1",
    "element_global_end_moment_v1",
    "pipe_wall_endpoint_action_v2",
    "pipe_wall_axial_force_v2",
    "pipe_effective_axial_force_v2",
    "pipe_axial_membrane_stress_v2",
    "pipe_lame_radial_stress_v2",
    "pipe_lame_hoop_stress_v2",
    "pipe_toroidal_membrane_hoop_stress_v1",
    "pipe_toroidal_radial_surface_stress_v1",
    "connector_generalized_translation_v1",
    "connector_generalized_rotation_v1",
    "connector_generalized_force_v1",
    "connector_generalized_moment_v1",
    "connector_endpoint_force_v1",
    "connector_endpoint_moment_v1",
    "connector_pressure_force_v1",
    "connector_effective_axial_force_v1",
    "connector_tie_rod_force_v1",
];
/// The pressure-2 table's `recomputed_from_combined` kinds.
pub(crate) const RECOMPUTED_KINDS: [&str; 3] =
    ["displacement_magnitude", "support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"];
const TIE_ROD_KIND: &str = "connector_tie_rod_force_v1";
const SUPPORT_COMPONENT: &str = "support_reaction_component_v2";
const SUPPORT_FORCE: &str = "support_reaction_force_magnitude_v2";
const SUPPORT_MOMENT: &str = "support_reaction_moment_magnitude_v2";
const SUPPORT_COMPONENTS: [&str; 6] = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const DISPLACEMENT_COMPONENTS: [(&str, &str); 3] = [
    ("global_nodal_displacement_x", "nodal_displacement_x"),
    ("global_nodal_displacement_y", "nodal_displacement_y"),
    ("global_nodal_displacement_z", "nodal_displacement_z"),
];
/// §3: pressure-only quantities, exactly zero at p = 0.
const ZERO_AT_NO_PRESSURE: [&str; 4] = [
    "pipe_lame_radial_stress_v2",
    "pipe_lame_hoop_stress_v2",
    "pipe_toroidal_membrane_hoop_stress_v1",
    "pipe_toroidal_radial_surface_stress_v1",
];
const WALL_FORCE: &str = "pipe_wall_axial_force_v2";
const EFFECTIVE_FORCE: &str = "pipe_effective_axial_force_v2";
const END_ACTION: &str = "pipe_wall_endpoint_action_v2";
const MEMBRANE: &str = "pipe_axial_membrane_stress_v2";
const AXIAL_FORCE: &str = "element_local_axial_force";
/// The withheld reasons (§3).
const TIE_ROD_ROW: &str = "tie-rod row";
const MISSING_KEY_FIELDS: &str = "row missing key fields";
const DIMENSIONLESS: &str = "dimensionless row";
const ALGEBRA_REFUSED: &str = "load-case algebra refused";
const SUPPORT_SOURCE_MISSING: &str = "source support row missing in an operand case";

/// The facts a combination reads beside the published rows: the envelope's own
/// evidence, before it is placed in `contract_evidence`.
pub(crate) struct Facts<'a> {
    pub exact_cases: &'a [Value],
    pub pressure: &'a [Value],
    pub load_reference_states: &'a [Value],
    pub connector: &'a Value,
}

/// The typed structural pairing key (§3). Its fields borrow from the keyed row
/// (the section from its metadata basis) or are constants; they compare by value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key<'r> {
    kind: &'r str,
    unit: &'r str,
    entity: &'r str,
    location: &'r str,
    component: &'r str,
    frame: &'r str,
    section: &'r str,
}

impl<'r> Key<'r> {
    /// `None` for a row whose family needs a field it lacks (a row without
    /// metadata other than a displacement magnitude, keyed by its node).
    fn of(row: &'r ResultItem) -> Option<Self> {
        // A magnitude is keyed by kind, unit and node: a load case's row carries no
        // metadata, and an unsigned or nested combination's states only its
        // derivation (absolute-sum-srss §4).
        let (location, component, frame, section) = match &row.metadata {
            _ if row.kind == "displacement_magnitude" => Default::default(),
            Some(m) => (m.location.as_str(), m.component.as_str(), m.coordinate_system.as_str(), section_of(&m.basis)),
            None => return None,
        };
        Some(Key { kind: &row.kind, unit: &row.unit, entity: &row.entity_ref, location, component, frame, section })
    }

    fn with(&self, kind: &'r str, component: &'r str, location: &'r str, frame: &'r str, unit: &'r str, section: &'r str) -> Key<'r> {
        Key { kind, unit, entity: self.entity, location, component, frame, section }
    }

    /// The combined row's id: the combination and the key, each part length-prefixed.
    fn combined_id(&self, combination: &str) -> String {
        format!(
            "result:combination:{}",
            exact_source_identity(&[combination, self.kind, self.unit, self.entity, self.location, self.component, self.frame, self.section])
        )
    }
}

/// The section basis a row's metadata basis names, or "" when it names none.
fn section_of(basis: &str) -> &str {
    basis.split_once(SECTION_SEPARATOR).map_or("", |(_, section)| section)
}

fn signed(kind: &str) -> bool {
    SIGNED_LINEAR_KINDS.contains(&kind)
}
fn recomputed(kind: &str) -> bool {
    RECOMPUTED_KINDS.contains(&kind)
}
fn support_kind(kind: &str) -> bool {
    matches!(kind, SUPPORT_COMPONENT | SUPPORT_FORCE | SUPPORT_MOMENT)
}

/// One operand value of a combined row: an operand's row (a load case's, or a
/// combination's own published row, absolute-sum-srss §4), or a load case's
/// recovered value.
#[derive(Debug, Clone)]
enum Source<'a> {
    Row { target: Target<'a>, id: &'a str },
    Recovered { case: &'a str, index: usize },
}

/// The evidence of an operand's source row.
fn row_source(target: &Target<'_>, id: &str) -> Value {
    match target {
        Target::Case(case) => json!({"load_case_id": case, "result_id": id}),
        Target::Combination(combination) => json!({"combination_id": combination, "result_id": id}),
    }
}

impl Source<'_> {
    fn evidence(&self) -> Value {
        match self {
            Source::Row { target, id } => row_source(target, id),
            Source::Recovered { case, index } => json!({"load_case_id": case, "recovery_index": index}),
        }
    }
}

/// The outcome of one key.
enum Outcome<'a> {
    Row { value: f64, sources: Vec<Source<'a>> },
    Missing(Vec<String>),
    Withheld,
}

/// The case evidence of one load case.
struct CaseFacts<'a> {
    /// `pipe_sections` by pipe id.
    sections: HashMap<&'a str, &'a Value>,
    /// `pipe_materials` by pipe id.
    materials: HashMap<&'a str, &'a Value>,
    material_basis: &'a Value,
    /// Every member of a pressure region of this case.
    region_members: HashSet<&'a str>,
    /// The load/reference-state record's members by pipe id (load-state documents).
    states: HashMap<&'a str, &'a Value>,
}

fn case_facts<'a>(facts: &Facts<'a>, case: &str) -> CaseFacts<'a> {
    let exact = facts.exact_cases.iter().find(|c| c["load_case_id"] == case);
    let indexed = |key: &str| -> HashMap<&'a str, &'a Value> {
        exact
            .and_then(|c| c[key].as_array())
            .map(|items| items.iter().filter_map(|item| item["pipe_id"].as_str().map(|id| (id, item))).collect())
            .unwrap_or_default()
    };
    let region_members = facts
        .pressure
        .iter()
        .filter(|region| region["load_case_id"] == case)
        .flat_map(|region| region["member_pipe_ids"].as_array().map(Vec::as_slice).unwrap_or(&[]))
        .filter_map(Value::as_str)
        .collect();
    let states = facts
        .load_reference_states
        .iter()
        .find(|record| record["load_case_id"] == case)
        .and_then(|record| record["members"].as_array())
        .map(|members| members.iter().filter_map(|m| m["pipe_id"].as_str().map(|id| (id, m))).collect())
        .unwrap_or_default();
    CaseFacts {
        sections: indexed("pipe_sections"),
        materials: indexed("pipe_materials"),
        material_basis: exact.map_or(&Value::Null, |c| &c["material_basis"]),
        region_members,
        states,
    }
}

/// §4 gate 2: a member's resolved stiffness in one case, from the case's published
/// evidence: its selected E/nu/G and material, its property-temperature selection
/// (the case's material basis and, on a load/reference-state document, the member's
/// selection kind, temperature and interpolation), and its section record (both
/// section bases, the shear model and the Timoshenko shear area). The W-15 stiffness
/// scale is a model fact, the same in every case.
fn stiffness(case: &CaseFacts<'_>, pipe: &str) -> Value {
    let material = case.materials.get(pipe).copied().unwrap_or(&Value::Null);
    let state = case.states.get(pipe).copied().unwrap_or(&Value::Null);
    json!({
        "E_pa": material["E_pa"], "nu": material["nu"], "G_pa": material["G_pa"],
        "material_id": material["material_id"], "constitutive_basis": material["constitutive_basis"],
        "material_selection_kind": material.get("material_selection_kind").cloned().unwrap_or(Value::Null),
        "material_basis": case.material_basis,
        "state_selection": if state.is_null() { Value::Null } else { json!({
            "material_selection_kind": state["material_selection_kind"],
            "material_selection_temperature_k": state["material_selection_temperature_k"],
            "interpolation_fraction": state["interpolation_fraction"],
            "selected_E_pa": state["selected_E_pa"],
            "selected_nu": state["selected_nu"],
        }) },
        "section": case.sections.get(pipe).copied().unwrap_or(&Value::Null),
    })
}

/// §4 gate 3's model condition: a connector whose raw installed reference is
/// nonzero (q_ref, a null-space reference included) or a tie rod whose reference
/// extension is nonzero.
fn installed_reference(connector: &Value) -> bool {
    connector.as_array().is_some_and(|records| {
        records.iter().any(|record| {
            record["q_ref"].as_array().is_some_and(|q| q.iter().any(|v| v.as_f64().is_some_and(|v| v != 0.0)))
                || record["tie_rods"].as_array().is_some_and(|rods| {
                    rods.iter().any(|rod| rod["reference_extension_m"].as_f64().is_some_and(|v| v != 0.0))
                })
        })
    })
}

/// The gates (§4; absolute-sum-srss §5), in precedence order: a withheld operand
/// combination; then, for `mechanics`, `absolute_sum` and `srss`, gates 1 and 2
/// over every load case the combination reaches through its operands, and gate 3
/// on the fixed-load count κ. Subtractions and envelopes are ungated otherwise. A
/// flat combination meets today's gates exactly.
fn gate_reason(
    model: &PreviewModel,
    plan: &combination_plan::Plan<'_>,
    index: usize,
    facts: &Facts<'_>,
    targets: &[Target<'_>],
    withheld: &HashMap<String, &'static str>,
) -> Option<(&'static str, String)> {
    let combination = &model.combinations[index];
    let operands = preview_physics::withheld_operands(targets, withheld);
    if !operands.is_empty() {
        return Some((preview_physics::OPERAND_WITHHELD, preview_physics::operand_withheld_message(&operands)));
    }
    let gated = |code: &'static str| Some((code, gate_message(code).to_string()));
    let basis = combination.basis.as_str();
    if !matches!(basis, "mechanics" | "absolute_sum" | "srss") {
        return None;
    }
    // absolute-sum-srss §5.2, as on PF: in a model with nonlinear supports a mechanics
    // combination is withheld unless decision OQ-1's predicate admits it. Plain EX
    // admits nonlinear supports; S6 extends their connector and load-state reachability.
    if basis == "mechanics" && has_nonlinear_support(model) && !plan.solved_state_plus_increments(combination) {
        return gated(NONLINEAR);
    }
    let reached = &plan.reached[index];
    let mut scratch = Vec::new();
    let keys: HashSet<Option<String>> = reached
        .iter()
        .filter_map(|case| model.load_cases.iter().find(|c| c.id == *case))
        .map(|case| modulus_basis_key(case, &mut scratch))
        .collect();
    if keys.len() > 1 {
        return gated(MODULUS_BASIS_MIXED);
    }
    let cases: Vec<CaseFacts<'_>> = reached.iter().map(|case| case_facts(facts, case)).collect();
    let mut members: Vec<&str> = Vec::new();
    for case in &cases {
        for pipe in case.materials.keys().chain(case.sections.keys()) {
            if !members.contains(pipe) {
                members.push(pipe);
            }
        }
    }
    for pipe in members {
        let first = stiffness(&cases[0], pipe);
        if cases[1..].iter().any(|case| stiffness(case, pipe) != first) {
            return gated(RESOLVED_STIFFNESS_MIXED);
        }
    }
    // Gate 3 reads κ (absolute-sum-srss §5.1); flat, it is the shared unit
    // factor-sum rule. A non-finite factor (refused before the solve by
    // LOAD_COMBINATION_FACTOR_INVALID) makes κ undefined, which never passes. §5.1
    // names no operand-mixed code for this source, so a mechanics operand with an
    // undefined κ withholds under gate 3's own code (§10).
    let fixed_load_withholding = || {
        let operand_kappa: Vec<_> = targets.iter().map(|target| plan.kappa_of(target)).collect();
        if basis == "mechanics" {
            open_pipe_stress_load_case_algebra::mechanics_fixed_load_gate(&plan.kappa[index], operand_kappa.iter())
        } else {
            open_pipe_stress_load_case_algebra::unsigned_fixed_load_gate(operand_kappa.iter())
        }
    };
    if installed_reference(facts.connector) && fixed_load_withholding().is_some() {
        return gated(if basis == "mechanics" { INSTALLED_REFERENCE } else { INSTALLED_REFERENCE_UNSIGNED });
    }
    // Gate 4: a consuming constant-effort support (DEC-049: exactly one
    // translational restraint DOF and a finite positive constant load, no
    // nonlinear field) is in every case state, so a combination counts its
    // force κ times. It is one more fixed-load source for the same κ rule as
    // gate 3 (flat, the unit factor-sum rule), under PF's codes, and follows
    // gate 3, which wins where both hold. As on PF, a mechanics operand with an
    // undefined κ withholds as CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED; a
    // mechanics combination whose own κ is undefined, its operands' defined, as
    // CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE (§5.1).
    if constant_effort_solve_dispositions(model).iter().any(|(_, disposition)| disposition.is_ok()) {
        if let Some(withholding) = fixed_load_withholding() {
            use open_pipe_stress_load_case_algebra::FixedLoadWithholding as W;
            return gated(match withholding {
                W::UnitFactorSum => CONSTANT_EFFORT,
                W::OperandUndefined => CONSTANT_EFFORT_OPERAND_MIXED,
                W::UnsignedOperand => CONSTANT_EFFORT_UNSIGNED,
            });
        }
    }
    None
}

/// Whether the model has a nonlinear support record (the model condition of PF's
/// `NONLINEAR_COMBINATION_REQUIRES_SOLVE`).
fn has_nonlinear_support(model: &PreviewModel) -> bool {
    model.supports.iter().any(|support| support.nonlinear.is_some())
}

fn gate_message(code: &str) -> &'static str {
    match code {
        NONLINEAR => "Combination withheld: the model has nonlinear supports, so superposing case states is not a solved state.",
        MODULUS_BASIS_MIXED => "Combination withheld: its load cases were solved on different modulus bases.",
        RESOLVED_STIFFNESS_MIXED => "Combination withheld: a member's resolved stiffness (its selected E/nu, property-temperature selection, section bases or shear model) differs between its load cases, so superposing their states is not a state of one system.",
        CONSTANT_EFFORT => "Combination withheld: a constant-effort support would be counted by the sum of factors instead of once.",
        CONSTANT_EFFORT_UNSIGNED => "Combination withheld: a constant-effort support's force acts in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.",
        CONSTANT_EFFORT_OPERAND_MIXED => "Combination withheld: an operand's fixed-load count is undefined, so the number of times this combination counts a constant-effort support's force cannot be decided.",
        INSTALLED_REFERENCE_UNSIGNED => "Combination withheld: a connector or tie rod has a nonzero installed reference, which acts in every case state; an absolute sum or SRSS of operands whose fixed-load count is not exactly 0 would count it once per term with its sign discarded.",
        _ => "Combination withheld: a connector or tie rod has a nonzero installed reference, which superposition counts by the sum of the factors instead of once, and the factors do not sum to 1 within 64 epsilon.",
    }
}

/// The rows of one operand case, by key, in publication order.
struct CaseRows<'r> {
    by_key: HashMap<Key<'r>, KeyRows<'r>>,
    order: Vec<&'r ResultItem>,
}

/// The rows of one case under one key, in publication order: almost always one.
enum KeyRows<'r> {
    One(&'r ResultItem),
    Several(Vec<&'r ResultItem>),
}

impl<'r> CaseRows<'r> {
    fn keyed(&self, key: &Key<'r>) -> Option<&[&'r ResultItem]> {
        self.by_key.get(key).map(|rows| match rows {
            KeyRows::One(row) => std::slice::from_ref(row),
            KeyRows::Several(rows) => rows.as_slice(),
        })
    }
}

/// The rows of one operand: a load case's, or a combination's own published rows.
fn case_rows<'r>(results: &'r [ResultItem], target: &Target<'_>) -> CaseRows<'r> {
    let mut by_key: HashMap<Key<'r>, KeyRows<'r>> = HashMap::new();
    let mut order = Vec::new();
    for row in results.iter().filter(|r| target.owns(r)) {
        order.push(row);
        if let Some(key) = Key::of(row) {
            match by_key.entry(key) {
                std::collections::hash_map::Entry::Vacant(slot) => {
                    slot.insert(KeyRows::One(row));
                }
                std::collections::hash_map::Entry::Occupied(mut slot) => match slot.get_mut() {
                    KeyRows::One(first) => {
                        let first = *first;
                        slot.insert(KeyRows::Several(vec![first, row]));
                    }
                    KeyRows::Several(rows) => rows.push(row),
                },
            }
        }
    }
    CaseRows { by_key, order }
}

/// §3: the operand value of a pressure-family key that the operand case lacks,
/// recovered at p = 0, with its witness; `None` when the member does not
/// qualify or a source row is missing (a missing source).
fn recover<'r>(key: &Key<'r>, case: &str, rows: &CaseRows<'r>, facts: &CaseFacts<'_>) -> Option<(f64, Value)> {
    let pipe = key.entity;
    if !facts.sections.contains_key(pipe) || facts.region_members.contains(pipe) {
        return None;
    }
    let witness = |source: Option<&ResultItem>, sign: &str, area: Option<f64>, value: f64| {
        json!({
            "load_case_id": case, "pipe_id": pipe, "kind": key.kind, "component": key.component,
            "location": key.location, "section_basis": if key.section.is_empty() { Value::Null } else { json!(key.section) },
            "method": RECOVERY_METHOD, "source_result_id": source.map(|row| row.id.clone()),
            "sign": sign, "area_m2": area,
            "rounding": if area.is_some() { "one_binary64_division" } else { "none" },
            "value": value,
        })
    };
    if ZERO_AT_NO_PRESSURE.contains(&key.kind) {
        return Some((0.0, witness(None, "none", None, 0.0)));
    }
    // The axial force row at the key's own station.
    let axial = || -> Option<&ResultItem> {
        let found = rows.keyed(&key.with(AXIAL_FORCE, "axial_force", key.location, "element_local", "N", ""))?;
        (found.len() == 1).then(|| found[0])
    };
    match key.kind {
        END_ACTION => {
            let source = axial()?;
            Some((source.value, witness(Some(source), "unchanged", None, source.value)))
        }
        WALL_FORCE | EFFECTIVE_FORCE | MEMBRANE => {
            let source = axial()?;
            // A negated zero is +0.0 (0 - N), so a witness never holds a -0.0
            // that a value-preserving re-serialization would rewrite.
            let (n, sign) = if key.location == "end_i" { (0.0 - source.value, "negated") } else { (source.value, "unchanged") };
            if key.kind == MEMBRANE {
                let area = facts.sections.get(pipe)?.get(key.section)?["As_m2"].as_f64()?;
                let value = n / area;
                Some((value, witness(Some(source), sign, Some(area), value)))
            } else {
                Some((n, witness(Some(source), sign, None, n)))
            }
        }
        _ => None,
    }
}

struct Built {
    rows: Vec<ResultItem>,
    row_evidence: Vec<Value>,
    recoveries: Vec<Value>,
    missing: Vec<Value>,
    withheld: Vec<String>,
    withheld_evidence: Vec<Value>,
}

/// A combined row; `solved_state_plus_increments` is decision OQ-1's admission in a
/// model with nonlinear supports, which states its own sign convention.
#[allow(clippy::too_many_arguments)]
fn combined_row_with(combination: &PreviewCombination, expression: &CombinationExpression<'_>, solved_state_plus_increments: bool, template: &ResultItem, key: &Key<'_>, value: f64, refs: Vec<String>) -> ResultItem {
    let mut row = template.clone();
    row.id = key.combined_id(&combination.id);
    row.value = value;
    row.basis_ref = Some(ResultBasisRef { ref_type: "combination".to_string(), ref_id: combination.id.clone() });
    row.source_result_refs = refs;
    row.component_body = None;
    if let Some(metadata) = row.metadata.as_mut() {
        // §2: a stress row keeps its operand rows' metadata basis (recovery token
        // and section-basis suffix), so it keeps its pressure-2 signature, whose
        // `source_basis` is that token; PF's generic basis replacement is used
        // for the other rows only. The sign convention follows the expression.
        if key.section.is_empty() {
            metadata.basis = expression.result_metadata_basis().to_string();
        }
        metadata.sign_convention = match expression {
            CombinationExpression::ResultStateSubtraction { .. } => SUBTRACTION_SIGN.to_string(),
            _ => expression.result_sign_convention(solved_state_plus_increments),
        };
    }
    row
}

/// The source refs of a combined row: each operand's row, or its recovery's source row.
fn source_refs(sources: &[Source], recoveries: &[Value]) -> Vec<String> {
    sources
        .iter()
        .filter_map(|source| match source {
            Source::Row { id, .. } => Some(id.to_string()),
            Source::Recovered { index, .. } => recoveries[*index]["source_result_id"].as_str().map(str::to_string),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn build<'r>(combination: &PreviewCombination, expression: &CombinationExpression<'_>, oq1: bool, operands: &'r [Target<'r>], results: &'r [ResultItem], facts: &'r Facts<'r>) -> Built {
    let range = matches!(expression, CombinationExpression::RangeEnvelope { .. });
    let rows: Vec<CaseRows<'_>> = operands.iter().map(|target| case_rows(results, target)).collect();
    // A combination operand publishes every row it forms; only a load case's
    // pressure-family value is ever recovered (§3).
    let case_facts: Vec<Option<CaseFacts<'_>>> = operands
        .iter()
        .map(|target| match target {
            Target::Case(case) => Some(case_facts(facts, case)),
            Target::Combination(_) => None,
        })
        .collect();
    let mut built = Built { rows: Vec::new(), row_evidence: Vec::new(), recoveries: Vec::new(), missing: Vec::new(), withheld: Vec::new(), withheld_evidence: Vec::new() };
    let withhold = |built: &mut Built, name: String, reason: &'static str| {
        if !built.withheld_evidence.iter().any(|w| w["name"] == name.as_str()) {
            built.withheld.push(format!("{name} ({reason})"));
            built.withheld_evidence.push(json!({"name": name, "reason": reason}));
        }
    };

    // The keys in first-seen order (operand order, then publication order),
    // with the first operand row of each as the template. `seen` gives each
    // key's place in `order`; the outcomes are kept by that place.
    let mut order: Vec<(Key<'_>, &ResultItem)> = Vec::new();
    let mut seen: HashMap<Key<'_>, usize> = HashMap::new();
    for case_rows in &rows {
        for row in &case_rows.order {
            let kind = row.kind.as_str();
            if !(signed(kind) || recomputed(kind)) || support_kind(kind) {
                continue;
            }
            // Invariant: unreachable on current producers. `Key::of` refuses only a
            // row without metadata other than a displacement magnitude. The case rows
            // without metadata are `displacement_magnitude`, `reaction_resultant` and
            // `open_formula_stress_summary` (lib.rs:6049, 6224 and 6973, in
            // `solve_prepared_load_case`), the last two not combined kinds, and the
            // source receipt's bare rows (`source_receipt/rows.rs` `bare`), published
            // only by a source-selected case, which a document with a combination never has.
            let Some(key) = Key::of(row) else {
                withhold(&mut built, row.id.clone(), MISSING_KEY_FIELDS);
                continue;
            };
            if let std::collections::hash_map::Entry::Vacant(slot) = seen.entry(key) {
                slot.insert(order.len());
                order.push((key, row));
            }
        }
    }

    // Phase 1: every key that is not recomputed from combined components.
    let mut outcomes: Vec<Option<Outcome<'_>>> = std::iter::repeat_with(|| None).take(order.len()).collect();
    for (place, (key, template)) in order.iter().enumerate() {
        if recomputed(key.kind) && !range {
            continue;
        }
        // A key carried by several rows of one operand cannot pair: tie rods
        // by name, any other as missing a key field. The first operand that
        // carries several names them.
        if let Some(several) = rows.iter().find_map(|r| r.keyed(key).filter(|found| found.len() > 1)) {
            let reason = if key.kind == TIE_ROD_KIND { TIE_ROD_ROW } else { MISSING_KEY_FIELDS };
            for row in several {
                withhold(&mut built, row.id.clone(), reason);
            }
            outcomes[place] = Some(Outcome::Withheld);
            continue;
        }
        if key.kind == TIE_ROD_KIND {
            withhold(&mut built, template.id.clone(), TIE_ROD_ROW);
            outcomes[place] = Some(Outcome::Withheld);
            continue;
        }
        let mut sources = Vec::new();
        let mut values = Vec::new();
        let mut missing = Vec::new();
        for (index, target) in operands.iter().enumerate() {
            if let Some(found) = rows[index].keyed(key) {
                values.push(found[0].value);
                sources.push(Source::Row { target: *target, id: &found[0].id });
            } else if let Some((case, (value, witness))) = case_facts[index]
                .as_ref()
                .and_then(|facts| recover(key, target.id(), &rows[index], facts).map(|recovered| (target.id(), recovered)))
            {
                built.recoveries.push(witness);
                values.push(value);
                sources.push(Source::Recovered { case, index: built.recoveries.len() - 1 });
            } else {
                missing.push(target.id().to_string());
            }
        }
        if !missing.is_empty() {
            // Recovered witnesses of a row that is not published are not kept.
            let kept = sources.iter().filter(|s| matches!(s, Source::Recovered { .. })).count();
            built.recoveries.truncate(built.recoveries.len() - kept);
            outcomes[place] = Some(Outcome::Missing(missing));
            continue;
        }
        // Invariant: unreachable on current producers. Every signed-linear and
        // recomputed kind of the pressure-2 table has unit N, N*m, Pa, MPa, m, mm or
        // rad, each mapped by `algebra_dimension` (lib.rs:14975), as
        // `combined_kind_units_have_an_algebra_dimension` checks; a row of such a kind
        // in another unit has no pressure-2 signature (result_export `semantic_contract.rs`).
        let Some(dimension) = algebra_dimension(template) else {
            let kept = sources.iter().filter(|s| matches!(s, Source::Recovered { .. })).count();
            built.recoveries.truncate(built.recoveries.len() - kept);
            withhold(&mut built, template.id.clone(), DIMENSIONLESS);
            outcomes[place] = Some(Outcome::Withheld);
            continue;
        };
        match expression.evaluate_values(&values, dimension) {
            Some(value) => {
                outcomes[place] = Some(Outcome::Row { value, sources });
            }
            // Invariant: unreachable provided every recovered operand is finite.
            // `algebra_value` refuses only a non-finite operand (`AlgebraQuantity::new`,
            // load_case_algebra lib.rs:83) or an evaluation finding. A case-row operand
            // is finite (`solve_prepared_load_case` refuses any other, lib.rs:7203). A
            // recovered operand is zero, the axial row, or that row over the published
            // wall area (`recover`). The area is positive and finite
            // (`source_geometry.rs:55`) but has no lower bound (`validate_wall` requires
            // only 0 < t <= od - t), so the quotient of a finite force by a small area
            // can overflow (1e308 / 0.5 is +inf): `AlgebraQuantity::new` then refuses it
            // and the row is withheld here as ALGEBRA_REFUSED, its recoveries rolled
            // back. No upstream bound excludes that recovery overflow. The operands share
            // one dimension and two automatic statuses, and validation refuses the other
            // findings' inputs before the solve: a repeated or non-finite term
            // (validation.rs:2044 and 2065), a self-subtraction (2110) and a repeated
            // range operand (2173). Overflow of a linear combination's result itself,
            // from finite operands, is not a refusal and never reaches this branch: a
            // linear sum rounds to the signed infinity (`combination_value`,
            // load_case_algebra lib.rs:604) and a subtraction overflows to it
            // (lib.rs:421). The value entry refuses factors above 2^53 but the typed
            // entry accepts any finite factor; either way an infinite result row blocks
            // the envelope (lib.rs:3563). An absolute sum or SRSS is different (RT-A4,
            // absolute-sum-srss §3, §6): its kernel reports a result above the binary64
            // range, or an overflowing scaled term, as the `NonRepresentable` finding,
            // never an infinity, so that overflow from finite operands does reach this
            // branch and withholds the row as ALGEBRA_REFUSED.
            None => {
                let kept = sources.iter().filter(|s| matches!(s, Source::Recovered { .. })).count();
                built.recoveries.truncate(built.recoveries.len() - kept);
                withhold(&mut built, template.id.clone(), ALGEBRA_REFUSED);
                outcomes[place] = Some(Outcome::Withheld);
            }
        }
    }

    // Phase 2: publish in key order; a magnitude is the norm of its combined
    // components (outside range envelopes, which select it in phase 1).
    // Invariant: each magnitude precedes its components in `order`, so it reads
    // their outcomes before their own places `take()` them. It holds because a
    // load case publishes every node's `displacement_magnitude` before any
    // displacement component (`solve_prepared_load_case`), and `order` follows
    // operand publication order. A component taken first would read as missing.
    // A combination operand publishes in its own key order, so it keeps the
    // invariant for its dependants (absolute-sum-srss §4).
    for (place, (key, template)) in order.iter().enumerate() {
        if recomputed(key.kind) && !range {
            let components: Vec<Key<'_>> = DISPLACEMENT_COMPONENTS
                .iter()
                .map(|&(kind, component)| key.with(kind, component, "node", "global", key.unit, ""))
                .collect();
            // Invariant: the refused, missing and absent-component paths below are
            // unreachable. Every operand case that reaches `append` published each
            // model node's magnitude and three displacement components once
            // (`solve_prepared_load_case`, lib.rs:6017-6065): a case that returns
            // without them adds a blocking diagnostic, on which the case loop returns
            // the blocked envelope (`run_linear_static_preview_observed_inner`,
            // lib.rs:3255-3410). So each component key is in `seen`, with one row in
            // every operand: not co-keyed, not a tie rod, in mm, and finite (above).
            let mut vector = [0.0; 3];
            let mut missing: Vec<String> = Vec::new();
            let mut refused = false;
            for (slot, component) in components.iter().enumerate() {
                match seen.get(component).and_then(|&at| outcomes[at].as_ref()) {
                    Some(Outcome::Row { value, .. }) => vector[slot] = *value,
                    Some(Outcome::Withheld) => refused = true,
                    other => {
                        // A component no operand publishes is missing in every operand.
                        let cases: Vec<String> = match other {
                            Some(Outcome::Missing(cases)) => cases.clone(),
                            _ => operands.iter().map(|c| c.id().to_string()).collect(),
                        };
                        for case in cases {
                            if !missing.contains(&case) {
                                missing.push(case);
                            }
                        }
                    }
                }
            }
            if !missing.is_empty() {
                built.missing.push(json!({"source_result_id": template.id, "missing_load_case_ids": missing}));
                continue;
            }
            if refused {
                withhold(&mut built, template.id.clone(), ALGEBRA_REFUSED);
                continue;
            }
            // §3, §6: an unsigned magnitude outside the binary64 range is refused,
            // never published as an infinity; a linear combination keeps today's rule.
            let value = norm3(vector[0], vector[1], vector[2]);
            if expression.is_unsigned() && !value.is_finite() {
                withhold(&mut built, template.id.clone(), ALGEBRA_REFUSED);
                continue;
            }
            let refs = components.iter().map(|c| c.combined_id(&combination.id)).collect();
            let row = combined_row_with(combination, expression, oq1, template, key, value, refs);
            built.row_evidence.push(json!({"result_id": row.id, "sources": []}));
            built.rows.push(row);
            continue;
        }
        match outcomes[place].take() {
            Some(Outcome::Row { value, sources }) => {
                let refs = source_refs(&sources, &built.recoveries);
                let row = combined_row_with(combination, expression, oq1, template, key, value, refs);
                built.row_evidence.push(json!({"result_id": row.id, "sources": sources.iter().map(Source::evidence).collect::<Vec<_>>()}));
                built.rows.push(row);
            }
            Some(Outcome::Missing(cases)) => {
                built.missing.push(json!({"source_result_id": template.id, "missing_load_case_ids": cases}));
            }
            Some(Outcome::Withheld) | None => {}
        }
    }

    // Signed support actions, one group per support that publishes an Fx row
    // in some operand (in operand and publication order): whole or not at
    // all, withheld by name (§3), as on PF.
    let mut supports: Vec<&str> = Vec::new();
    for case_rows in &rows {
        for row in &case_rows.order {
            if row.kind == SUPPORT_COMPONENT && row.metadata.as_ref().is_some_and(|m| m.component == "Fx") && !supports.contains(&row.entity_ref.as_str()) {
                supports.push(&row.entity_ref);
            }
        }
    }
    for support in supports {
        let find = |index: usize, kind: &str, component: &str| -> Option<&ResultItem> {
            rows[index].order.iter().copied().find(|r| r.kind == kind && r.entity_ref == support && r.metadata.as_ref().is_some_and(|m| m.component == component))
        };
        let group = || -> Result<Vec<(ResultItem, Value)>, &'static str> {
            let mut out = Vec::new();
            let mut combined = [0.0; 6];
            let mut component_ids = Vec::new();
            for (slot, component) in SUPPORT_COMPONENTS.iter().enumerate() {
                let sources: Vec<&ResultItem> = (0..operands.len()).map(|i| find(i, SUPPORT_COMPONENT, component)).collect::<Option<_>>().ok_or(SUPPORT_SOURCE_MISSING)?;
                let dimension = if slot < 3 { LoadDimension::Force } else { LoadDimension::Moment };
                let values: Vec<f64> = sources.iter().map(|s| s.value).collect();
                let value = expression.evaluate_values(&values, dimension).ok_or(ALGEBRA_REFUSED)?;
                combined[slot] = value;
                let key = Key::of(sources[0]).ok_or(SUPPORT_SOURCE_MISSING)?;
                let row = combined_row_with(combination, expression, oq1, sources[0], &key, value, sources.iter().map(|s| s.id.clone()).collect());
                component_ids.push(row.id.clone());
                let evidence = json!({"result_id": row.id, "sources": operands.iter().zip(&sources).map(|(target, s)| row_source(target, &s.id)).collect::<Vec<_>>()});
                out.push((row, evidence));
            }
            for (kind, component, slots) in [(SUPPORT_FORCE, "force_magnitude", 0..3), (SUPPORT_MOMENT, "moment_magnitude", 3..6)] {
                let sources: Vec<&ResultItem> = (0..operands.len()).filter_map(|i| find(i, kind, component)).collect();
                let template = *sources.first().ok_or(SUPPORT_SOURCE_MISSING)?;
                let key = Key::of(template).ok_or(SUPPORT_SOURCE_MISSING)?;
                let (value, refs, evidence_sources) = if range {
                    if sources.len() != operands.len() {
                        return Err(SUPPORT_SOURCE_MISSING);
                    }
                    let dimension = if component == "force_magnitude" { LoadDimension::Force } else { LoadDimension::Moment };
                    let values: Vec<f64> = sources.iter().map(|s| s.value).collect();
                    let value = expression.evaluate_values(&values, dimension).ok_or(ALGEBRA_REFUSED)?;
                    (value, sources.iter().map(|s| s.id.clone()).collect::<Vec<_>>(),
                        operands.iter().zip(&sources).map(|(target, s)| row_source(target, &s.id)).collect::<Vec<_>>())
                } else {
                    let c = &combined[slots.clone()];
                    let value = norm3(c[0], c[1], c[2]);
                    // §3, §6: as for the node magnitudes; the group is withheld.
                    if expression.is_unsigned() && !value.is_finite() {
                        return Err(ALGEBRA_REFUSED);
                    }
                    (value, component_ids[slots].to_vec(), Vec::new())
                };
                let row = combined_row_with(combination, expression, oq1, template, &key, value, refs);
                let evidence = json!({"result_id": row.id, "sources": evidence_sources});
                out.push((row, evidence));
            }
            Ok(out)
        };
        match group() {
            Ok(group) => {
                for (row, evidence) in group {
                    built.rows.push(row);
                    built.row_evidence.push(evidence);
                }
            }
            Err(reason) => withhold(&mut built, format!("support-action:{}:{}", support.len(), support), reason),
        }
    }
    // §4: a magnitude row of an unsigned or nested combination states its derivation;
    // a flat combination's keeps the load case's absent metadata (V-8).
    if expression.is_unsigned() || operands.iter().any(|t| matches!(t, Target::Combination(_))) {
        let sign = match expression {
            CombinationExpression::ResultStateSubtraction { .. } => SUBTRACTION_SIGN.to_string(),
            _ => expression.result_sign_convention(oq1),
        };
        for row in built.rows.iter_mut().filter(|r| r.kind == "displacement_magnitude") {
            row.metadata = Some(crate::derived_magnitude_metadata(expression.result_metadata_basis(), &sign));
        }
    }
    built
}

/// The complete expression as authored. A term names its operand by kind:
/// `{load_case_id, factor}` or `{combination_id, factor}` (absolute-sum-srss §2).
pub(crate) fn expression_evidence(combination: &PreviewCombination) -> Value {
    match combination.basis.as_str() {
        "mechanics" | "absolute_sum" | "srss" => json!({"terms": combination.terms.iter().map(|t| match &t.combination {
            Some(id) => json!({"combination_id": id, "factor": t.factor}),
            None => json!({"load_case_id": t.load_case, "factor": t.factor}),
        }).collect::<Vec<_>>()}),
        "result_state_subtraction" => json!({"minuend_id": combination.minuend_id, "subtrahend_id": combination.subtrahend_id}),
        _ => json!({"operand_ids": combination.operand_ids, "mode": combination.mode}),
    }
}

/// One operand's record: a load case's modulus selection and standing, or the
/// combination it names, whose own record states its operands (§4).
pub(crate) fn operand_evidence(model: &PreviewModel, target: &Target<'_>) -> Value {
    let case = match target {
        Target::Case(case) => *case,
        Target::Combination(id) => return json!({"combination_id": id}),
    };
    let load_case = model.load_cases.iter().find(|c| c.id == case);
    json!({
        "load_case_id": case,
        "modulus_basis_ref": load_case.and_then(|c| c.modulus_basis_ref.clone()),
        "modulus_basis_temperature": load_case.and_then(|c| c.modulus_basis_temperature.as_ref()).map(|q| json!({"value": q.value, "unit": q.unit})),
        // Filled from the envelope's own numerical quality once it is assessed (`state_operand_standing`).
        "numerical_quality": Value::Null,
    })
}

fn identity(parts: &[&str]) -> String {
    exact_source_identity(parts)
}

/// Appends every combination's rows and diagnostics on an exact-route envelope
/// and returns `contract_evidence.exact_combinations`, one record per
/// combination (§6). Called only when the model has combinations.
pub(crate) fn append(model: &PreviewModel, facts: Facts<'_>, results: &mut Vec<ResultItem>, diagnostics: &mut Vec<Diagnostic>) -> Vec<Value> {
    let has_maximum = results.iter().any(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case")
        && matches!(r.kind.as_str(), "pipe_elastic_normal_stress_maximum_v2" | u4b_rows::EXTREME_KIND | u4b_rows::SHEAR_MAX_KIND));
    let has_intensified = results.iter().any(|r| r.basis_ref.as_ref().is_some_and(|b| b.ref_type == "load_case")
        && matches!(r.kind.as_str(), u4b_rows::DIRECTIONAL_KIND | u4b_rows::DIRECTIONAL_MAX_KIND | u4b_rows::BRANCH_KIND));
    let mut records = Vec::new();
    // Every combination in evaluation order (absolute-sum-srss §2): an operand
    // combination's rows are appended before its dependants read them.
    let plan = combination_plan::Plan::new(model);
    let mut withheld_combinations: HashMap<String, &'static str> = HashMap::new();
    for &index in &plan.order {
        let combination = &model.combinations[index];
        // Validation refuses every unresolvable shape before the solve.
        let Some(expression) = CombinationExpression::resolve(combination) else { continue };
        let operands = expression.targets(&plan.resolver);
        let mut record = json!({
            "combination_id": combination.id,
            "basis": combination.basis,
            "expression": expression_evidence(combination),
            "operands": operands.iter().map(|target| operand_evidence(model, target)).collect::<Vec<_>>(),
            "gate": {"withheld": false, "reason": null},
            "rows": [], "recoveries": [], "missing_sources": [], "withheld": [],
            "standing": STANDING,
            "source_recovery": SOURCE_RECOVERY_EXCLUSION,
        });
        if plan.kappa[index].is_unrepresentable() {
            diagnostics.push(diag(
                &format!("diagnostic:exact-combination:fixed-load-count:{}", identity(&[&combination.id])),
                preview_physics::FIXED_LOAD_COUNT_UNREPRESENTABLE,
                "info",
                preview_physics::fixed_load_count_message(&combination.id),
                vec![combination.id.clone()],
            ));
        }
        if let Some((code, message)) = gate_reason(model, &plan, index, &facts, &operands, &withheld_combinations) {
            let mut refs = vec![combination.id.clone()];
            if code == preview_physics::OPERAND_WITHHELD {
                refs.extend(preview_physics::withheld_operands(&operands, &withheld_combinations).iter().map(|(id, _)| id.to_string()));
            }
            diagnostics.push(diag(
                &format!("diagnostic:exact-combination:gate:{}", identity(&[&combination.id])),
                code,
                "warning",
                message,
                refs,
            ));
            record["gate"] = json!({"withheld": true, "reason": code});
            records.push(record);
            withheld_combinations.insert(combination.id.clone(), code);
            continue;
        }
        // Decision OQ-1's rows state their own sign convention (§5.2).
        let oq1 = has_nonlinear_support(model) && plan.solved_state_plus_increments(combination);
        let built = build(combination, &expression, oq1, &operands, results.as_slice(), &facts);
        for missing in &built.missing {
            let source = missing["source_result_id"].as_str().unwrap_or_default();
            let cases = missing["missing_load_case_ids"].as_array().map(|c| c.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")).unwrap_or_default();
            diagnostics.push(diag(
                &format!("diagnostic:exact-combination:source-missing:{}", identity(&[&combination.id, source])),
                SOURCE_MISSING,
                "warning",
                format!("combination {} omits {source} because a matching source row is missing in operand case {cases}", combination.id),
                vec![combination.id.clone()],
            ));
        }
        if !built.withheld.is_empty() {
            const NAMED: usize = 8;
            let mut named = built.withheld.iter().take(NAMED).cloned().collect::<Vec<_>>().join(", ");
            if built.withheld.len() > NAMED {
                named.push_str(&format!(", and {} more", built.withheld.len() - NAMED));
            }
            diagnostics.push(diag(
                &format!("diagnostic:exact-combination:row-withheld:{}", identity(&[&combination.id])),
                ROW_WITHHELD,
                "warning",
                format!("combination {} withholds {} combined row(s) that could not be formed: {named}", combination.id, built.withheld.len()),
                vec![combination.id.clone()],
            ));
        }
        diagnostics.push(diag(
            &format!("diagnostic:exact-combination:derived:{}", identity(&[&combination.id])),
            DERIVED,
            "info",
            format!("combination {} publishes rows derived from case results: they never count towards numerical eligibility, and only the derived row groups that contract_evidence.combination_certificate establishes carry a stated bound; each operand's standing and numerical findings are stated in contract_evidence.exact_combinations, and the headlines and stress maxima cover the load cases only", combination.id),
            vec![combination.id.clone()],
        ));
        if has_maximum {
            diagnostics.push(diag(
                &format!("diagnostic:exact-combination:maximum:{}", identity(&[&combination.id])),
                "COMBINATION_STRESS_MAXIMUM_UNAVAILABLE",
                "info",
                "stress maxima and section stress extremes are not combined; combine the signed stress component rows instead",
                vec![combination.id.clone()],
            ));
        }
        if has_intensified {
            diagnostics.push(diag(
                &format!("diagnostic:exact-combination:intensified:{}", identity(&[&combination.id])),
                "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE",
                "info",
                "intensified measures are never combined (their ranges and combinations follow the owner's rule in T6)",
                vec![combination.id.clone()],
            ));
        }
        // The evidence moves into the record, held at its length as a copy would be;
        // the rows go straight to the envelope, after every earlier combination's,
        // where a later combination naming this one reads them.
        let array = |mut items: Vec<Value>| {
            items.shrink_to_fit();
            Value::Array(items)
        };
        record["rows"] = array(built.row_evidence);
        record["recoveries"] = array(built.recoveries);
        record["missing_sources"] = array(built.missing);
        record["withheld"] = array(built.withheld_evidence);
        records.push(record);
        results.extend(built.rows);
    }
    if !model.combinations.is_empty() {
        diagnostics.push(diag(
            "diagnostic:exact-combination:source-recovery-excluded",
            SOURCE_RECOVERY_EXCLUDED,
            "info",
            "this exact-route document has load combinations, so retained-source recovery is not attempted for any of its cases, primitive cases included; every case publishes ordinary structural recovery",
            // In evaluation order, as the records (a flat document's is its array order).
            plan.order.iter().map(|&index| model.combinations[index].id.clone()).collect(),
        ));
    }
    records
}

/// §5: each operand's standing and numerical findings, from the envelope's own
/// assessed numerical quality (its load-case record, verbatim; keyed on the record's
/// `basis_ref` type and id, as both readers key it).
pub(crate) fn state_operand_standing(records: &mut [Value], quality: &NumericalQuality) {
    let cases = serde_json::to_value(&quality.cases).unwrap_or(Value::Null);
    for record in records {
        if let Some(operands) = record["operands"].as_array_mut() {
            // A combination operand states its standing in its own record (§4).
            // RT-A5: a held operand's standing is the held solve's (`held_standing`), not the case's.
            for operand in operands.iter_mut().filter(|o| o.get("load_case_id").is_some() && o.get("support_state").is_none()) {
                let case = operand["load_case_id"].clone();
                operand["numerical_quality"] = cases
                    .as_array()
                    .and_then(|cases| cases.iter().find(|c| c["basis_ref"]["ref_type"] == "load_case" && c["basis_ref"]["ref_id"] == case))
                    .cloned()
                    .unwrap_or(Value::Null);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The kind classes are the pressure-2 table's `combination_algebra`, and its
    /// combination policy states the same linear and recomputed kinds.
    #[test]
    fn kind_classes_are_the_pressure_table_classes() {
        let table: Value = serde_json::from_str(include_str!("../../../fixtures/results/semantic_contract_v0_3_pressure_3.json")).unwrap();
        let mut by_kind: HashMap<&str, HashSet<&str>> = HashMap::new();
        for row in table["rows"].as_array().unwrap() {
            by_kind.entry(row["kind"].as_str().unwrap()).or_default().insert(row["combination_algebra"].as_str().unwrap());
        }
        for (kind, classes) in &by_kind {
            assert_eq!(classes.len(), 1, "{kind}");
            let class = *classes.iter().next().unwrap();
            assert_eq!(class == "signed_linear", signed(kind), "{kind}");
            assert_eq!(class == "recomputed_from_combined", recomputed(kind), "{kind}");
        }
        assert!(SIGNED_LINEAR_KINDS.iter().chain(&RECOMPUTED_KINDS).all(|k| by_kind.contains_key(k)));
        let policy = &table["combination_policy"];
        assert_eq!(policy["policy_id"], "PRESSURE-2-COMBINATION-v2");
        let listed = |key: &str| policy[key].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(listed("linear_state_kinds"), SIGNED_LINEAR_KINDS.to_vec());
        assert_eq!(listed("recomputed_magnitude_kinds"), RECOMPUTED_KINDS.to_vec());
        let codes = |key: &str| policy[key].as_array().unwrap().iter().map(|g| g["code"].as_str().unwrap()).collect::<Vec<_>>();
        // RT-A4 (absolute-sum-srss §10.2): `gate_reason`'s order. The operand gate comes first; then,
        // for a mechanics combination, the nonlinear gate (reached once the route admits nonlinear
        // supports) before gates 1 to 3 and gate 4's two codes, and for an absolute sum or SRSS the
        // unsigned list.
        assert_eq!(policy["operand_gate"]["code"], preview_physics::OPERAND_WITHHELD);
        assert_eq!(codes("mechanics_gates"), [NONLINEAR, MODULUS_BASIS_MIXED, RESOLVED_STIFFNESS_MIXED, INSTALLED_REFERENCE,
            CONSTANT_EFFORT_OPERAND_MIXED, CONSTANT_EFFORT]);
        assert_eq!(codes("unsigned_gates"), [MODULUS_BASIS_MIXED, RESOLVED_STIFFNESS_MIXED, INSTALLED_REFERENCE_UNSIGNED,
            CONSTANT_EFFORT_UNSIGNED]);
        assert_eq!(policy["gated_forms"], json!(["mechanics", "absolute_sum", "srss"]));
    }

    #[test]
    fn the_section_basis_is_read_from_the_metadata_basis_only() {
        assert_eq!(section_of("recovered_from_local_element_stiffness; section_basis=nominal_section"), "nominal_section");
        assert_eq!(section_of("recovered_from_local_element_stiffness"), "");
        assert_eq!(section_of(""), "");
    }

    #[test]
    fn combined_ids_are_length_prefixed_and_distinct_for_delimiter_rich_ids() {
        let key = |entity: &'static str, location: &'static str| Key {
            kind: "k", unit: "N", entity, location, component: "c", frame: "f", section: "",
        };
        assert_ne!(key("a:b", "c").combined_id("x"), key("a", "b:c").combined_id("x"));
        assert_eq!(key("p", "end_i").combined_id("comb:1"), "result:combination:6:comb:11:k1:N1:p5:end_i1:c1:f0:");
    }

    #[test]
    fn installed_reference_reads_q_ref_and_rod_reference_extension() {
        assert!(!installed_reference(&json!([])));
        assert!(!installed_reference(&json!([{"q_ref": [0.0, -0.0, 0.0, 0.0, 0.0, 0.0]}])));
        assert!(installed_reference(&json!([{"q_ref": [0.0, 0.0, 1e-300, 0.0, 0.0, 0.0]}])));
        assert!(installed_reference(&json!([{"q_ref": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0], "tie_rods": [{"reference_extension_m": 0.0}, {"reference_extension_m": -1e-4}]}])));
    }

    fn row(kind: &str, entity: &str, location: &str, component: &str, value: f64) -> ResultItem {
        ResultItem { component_body: None, id: format!("result:{kind}:{entity}:{location}"), kind: kind.into(), value, unit: "N".into(),
            entity_ref: entity.into(), basis_ref: Some(ResultBasisRef { ref_type: "load_case".into(), ref_id: "case:b".into() }),
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata { component: component.into(), coordinate_system: "element_local".into(), location: location.into(),
                basis: "recovered_from_local_element_stiffness".into(), sign_convention: "s".into() }) }
    }

    /// §3: a pressure-family operand value is recovered only for a member the
    /// case publishes outside every one of its regions, from its own axial
    /// force row; every other absence is a missing source.
    #[test]
    fn recovery_requires_a_published_member_outside_every_region_and_its_force_row() {
        let section = json!({"nominal_section": {"As_m2": 0.5}, "reduced_section": {"As_m2": 0.25}});
        let published = [row(AXIAL_FORCE, "pipe:p", "end_i", "axial_force", -8.0), row(AXIAL_FORCE, "pipe:p", "midspan", "axial_force", 8.0)];
        let rows = case_rows(&published, &Target::Case("case:b"));
        let facts = |sections: bool, region: bool| CaseFacts {
            sections: if sections { HashMap::from([("pipe:p", &section)]) } else { HashMap::new() },
            materials: HashMap::new(), material_basis: &Value::Null,
            region_members: if region { HashSet::from(["pipe:p"]) } else { HashSet::new() },
            states: HashMap::new(),
        };
        let key = |kind: &'static str, component: &'static str, location: &'static str, frame: &'static str, section: &'static str| Key {
            kind, unit: if section.is_empty() { "N" } else { "Pa" }, entity: "pipe:p", location, component, frame, section };
        let wall = key(WALL_FORCE, "wall_axial_force", "end_i", "element_local", "");
        let (value, witness) = recover(&wall, "case:b", &rows, &facts(true, false)).unwrap();
        assert_eq!((value, witness["sign"].as_str()), (8.0, Some("negated")));
        let membrane = key(MEMBRANE, "axial_membrane_stress", "midspan", "pipe_section", "reduced_section");
        let (value, witness) = recover(&membrane, "case:b", &rows, &facts(true, false)).unwrap();
        assert_eq!((value, witness["area_m2"].as_f64(), witness["rounding"].as_str()), (32.0, Some(0.25), Some("one_binary64_division")));
        let lame = key("pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress", "midspan", "pipe_section", "nominal_section");
        assert_eq!(recover(&lame, "case:b", &rows, &facts(true, false)).unwrap().0.to_bits(), 0);
        // A member in a region of the case, a member the case does not publish,
        // a station without its force row, and a kind outside the pressure family.
        assert!(recover(&wall, "case:b", &rows, &facts(true, true)).is_none());
        assert!(recover(&wall, "case:b", &rows, &facts(false, false)).is_none());
        assert!(recover(&key(WALL_FORCE, "wall_axial_force", "quarter_1", "element_local", ""), "case:b", &rows, &facts(true, false)).is_none());
        assert!(recover(&key("element_local_axial_normal_stress", "axial_normal_stress", "midspan", "element_local", "nominal_section"), "case:b", &rows, &facts(true, false)).is_none());
        // Mismatched bases never pair: the section basis is part of the key.
        assert_ne!(key(MEMBRANE, "c", "midspan", "f", "nominal_section"), key(MEMBRANE, "c", "midspan", "f", "reduced_section"));
    }

    /// Every signed-linear and recomputed kind of the pressure-2 table has a unit the
    /// load-case algebra gives a dimension (phase 1's dimensionless invariant).
    #[test]
    fn combined_kind_units_have_an_algebra_dimension() {
        let table: Value = serde_json::from_str(include_str!("../../../fixtures/results/semantic_contract_v0_3_pressure_3.json")).unwrap();
        for signature in table["rows"].as_array().unwrap() {
            let kind = signature["kind"].as_str().unwrap();
            if signed(kind) || recomputed(kind) {
                let mut published = row(kind, "pipe:p", "midspan", "c", 1.0);
                published.unit = signature["unit"].as_str().unwrap().into();
                assert!(algebra_dimension(&published).is_some(), "{kind} {}", published.unit);
            }
        }
    }

    /// A row of `case` with its own id.
    fn in_case(case: &str, id: &str, mut published: ResultItem) -> ResultItem {
        published.id = id.into();
        published.basis_ref = Some(ResultBasisRef { ref_type: "load_case".into(), ref_id: case.into() });
        published
    }

    /// `build` of one combination over `results`, with no pressure regions, load
    /// states or connectors.
    fn build_of(combination: Value, results: &[ResultItem], exact_cases: &[Value]) -> Built {
        let combination: PreviewCombination = serde_json::from_value(combination).unwrap();
        let expression = CombinationExpression::resolve(&combination).unwrap();
        let facts = Facts { exact_cases, pressure: &[], load_reference_states: &[], connector: &json!([]) };
        // #49's helper on the RT-A4 signature: flat load-case operands, no OQ-1
        // (these paths predate nesting; their expected outcomes are unchanged).
        let cases: Vec<&str> = results.iter().filter_map(|row| row.basis_ref.as_ref()).map(|b| b.ref_id.as_str()).collect();
        let operands: Vec<Target<'_>> = expression.targets(&combination_plan::Resolver::from_cases(cases));
        build(&combination, &expression, false, &operands, results, &facts)
    }

    fn mechanics(terms: &[(&str, f64)]) -> Value {
        json!({"id": "comb:1", "basis": "mechanics",
            "terms": terms.iter().map(|(case, factor)| json!({"load_case": case, "factor": factor})).collect::<Vec<_>>()})
    }

    /// #49's unreached defensive path (a): a key that is not a tie rod's, carried by
    /// several rows of one operand, is withheld as missing a key field, naming the
    /// rows of the first operand that carries several, in publication order; the
    /// other keys publish. The expected outcome is the pre-#49 code's (e1a483b6c).
    #[test]
    fn co_keyed_rows_that_are_not_tie_rods_are_withheld_as_missing_a_key_field() {
        let axial = |case: &str, id: &str, pipe: &str, value: f64| in_case(case, id, row(AXIAL_FORCE, pipe, "midspan", "axial_force", value));
        let results = [
            axial("case:a", "a:p", "pipe:p", 1.0),
            axial("case:a", "a:q", "pipe:q", 3.0),
            axial("case:a", "a:r1", "pipe:r", 1.0),
            axial("case:a", "a:r2", "pipe:r", 1.0),
            axial("case:a", "a:r3", "pipe:r", 1.0),
            axial("case:b", "b:p1", "pipe:p", 2.0),
            axial("case:b", "b:q", "pipe:q", 4.0),
            axial("case:b", "b:p2", "pipe:p", 5.0),
            axial("case:b", "b:r1", "pipe:r", 1.0),
            axial("case:b", "b:r2", "pipe:r", 1.0),
        ];
        let built = build_of(mechanics(&[("case:a", 1.0), ("case:b", 2.0)]), &results, &[]);
        let names = ["b:p1", "b:p2", "a:r1", "a:r2", "a:r3"];
        assert_eq!(built.withheld_evidence, names.iter().map(|name| json!({"name": name, "reason": MISSING_KEY_FIELDS})).collect::<Vec<_>>());
        assert_eq!(built.withheld, names.iter().map(|name| format!("{name} ({MISSING_KEY_FIELDS})")).collect::<Vec<_>>());
        assert_eq!(built.rows.len(), 1);
        assert_eq!((built.rows[0].value, built.rows[0].source_result_refs.clone()), (11.0, vec!["a:q".to_string(), "b:q".to_string()]));
        assert_eq!(built.row_evidence.len(), 1);
        assert!(built.missing.is_empty() && built.recoveries.is_empty());
    }

    /// #49's unreached defensive path (b): a key recovered in one operand and
    /// missing in another is a missing source, and its recovered witness is rolled
    /// back; an earlier key's kept witness keeps its index. The expected outcome is
    /// the pre-#49 code's (e1a483b6c).
    #[test]
    fn a_recovery_beside_a_missing_operand_is_rolled_back() {
        let wall = |case: &str, id: &str, location: &str, value: f64| in_case(case, id, row(WALL_FORCE, "pipe:p", location, "wall_axial_force", value));
        let axial = |id: &str, location: &str, value: f64| in_case("case:b", id, row(AXIAL_FORCE, "pipe:p", location, "axial_force", value));
        let results = [
            wall("case:a", "a:wall:mid", "midspan", 10.0),
            wall("case:a", "a:wall:end_j", "end_j", 20.0),
            axial("b:axial:mid", "midspan", 7.0),
            axial("b:axial:end_j", "end_j", 9.0),
            wall("case:c", "c:wall:mid", "midspan", 30.0),
        ];
        // case:b publishes the member outside every region, so it recovers both wall
        // keys; case:c does not publish the member, so it recovers neither.
        let exact_cases = [json!({"load_case_id": "case:b", "pipe_sections": [{"pipe_id": "pipe:p"}]})];
        let built = build_of(mechanics(&[("case:a", 1.0), ("case:b", 1.0), ("case:c", 1.0)]), &results, &exact_cases);
        assert_eq!(built.recoveries.len(), 1);
        assert_eq!((built.recoveries[0]["location"].as_str(), built.recoveries[0]["source_result_id"].as_str()), (Some("midspan"), Some("b:axial:mid")));
        assert_eq!(built.rows.len(), 1);
        assert_eq!(built.rows[0].value, 47.0);
        assert_eq!(built.rows[0].source_result_refs, ["a:wall:mid", "b:axial:mid", "c:wall:mid"]);
        assert_eq!(built.row_evidence[0]["sources"], json!([
            {"load_case_id": "case:a", "result_id": "a:wall:mid"},
            {"load_case_id": "case:b", "recovery_index": 0},
            {"load_case_id": "case:c", "result_id": "c:wall:mid"},
        ]));
        assert_eq!(built.missing, [
            json!({"source_result_id": "a:wall:end_j", "missing_load_case_ids": ["case:c"]}),
            json!({"source_result_id": "b:axial:mid", "missing_load_case_ids": ["case:a", "case:c"]}),
            json!({"source_result_id": "b:axial:end_j", "missing_load_case_ids": ["case:a", "case:c"]}),
        ]);
        assert!(built.withheld.is_empty());
    }

    /// #49's unreached defensive path (c): a support group is published whole or
    /// withheld by name. Withheld: a component missing in an operand, magnitudes in
    /// no operand, and (range envelopes only) a magnitude missing in an operand. The
    /// expected outcome is the pre-#49 code's (e1a483b6c).
    #[test]
    fn a_support_group_with_a_missing_source_row_is_withheld_whole() {
        let support = |case: &str, entity: &str, kind: &str, component: &str, value: f64| {
            in_case(case, &format!("{case}:{entity}:{component}"), row(kind, entity, "node", component, value))
        };
        let mut results = Vec::new();
        for (case, scale) in [("case:a", 1.0), ("case:b", 2.0)] {
            for entity in ["supp:s1", "supp:s2", "supp:s3", "supp:s4"] {
                for (slot, component) in SUPPORT_COMPONENTS.iter().enumerate() {
                    // s1 lacks Mz in case:b.
                    if !(entity == "supp:s1" && case == "case:b" && *component == "Mz") {
                        results.push(support(case, entity, SUPPORT_COMPONENT, component, scale * (slot + 1) as f64));
                    }
                }
                // s3 publishes no magnitude; s4 publishes its magnitudes in case:a only.
                if entity != "supp:s3" && (entity != "supp:s4" || case == "case:a") {
                    results.push(support(case, entity, SUPPORT_FORCE, "force_magnitude", 100.0 * scale));
                    results.push(support(case, entity, SUPPORT_MOMENT, "moment_magnitude", 200.0 * scale));
                }
            }
        }
        let withheld = |names: &[&str]| -> Vec<Value> {
            names.iter().map(|name| json!({"name": format!("support-action:7:{name}"), "reason": SUPPORT_SOURCE_MISSING})).collect()
        };
        let built = build_of(mechanics(&[("case:a", 1.0), ("case:b", 1.0)]), &results, &[]);
        assert_eq!(built.withheld_evidence, withheld(&["supp:s1", "supp:s3"]));
        assert_eq!(built.withheld, [format!("support-action:7:supp:s1 ({SUPPORT_SOURCE_MISSING})"), format!("support-action:7:supp:s3 ({SUPPORT_SOURCE_MISSING})")]);
        // s2 and s4 publish whole: six components (3 x their slot) and the two norms.
        assert_eq!(built.rows.len(), 16);
        assert_eq!(built.rows.len(), built.row_evidence.len());
        for group in built.rows.chunks(8) {
            let values: Vec<f64> = group.iter().map(|r| r.value).collect();
            assert_eq!(values[..6], [3.0, 6.0, 9.0, 12.0, 15.0, 18.0]);
            assert_eq!(values[6..], [norm3(3.0, 6.0, 9.0), norm3(12.0, 15.0, 18.0)]);
        }
        assert_eq!(built.rows.iter().map(|r| r.entity_ref.as_str()).collect::<HashSet<_>>(), HashSet::from(["supp:s2", "supp:s4"]));
        let range = build_of(json!({"id": "comb:1", "basis": "range_envelope", "operand_ids": ["case:b", "case:a"], "mode": "max"}), &results, &[]);
        assert_eq!(range.withheld_evidence, withheld(&["supp:s1", "supp:s3", "supp:s4"]));
        assert_eq!(range.rows.len(), 8);
        assert_eq!(range.rows.iter().map(|r| r.value).collect::<Vec<_>>(), [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 200.0, 400.0]);
    }

    /// An operand's standing is its load case's record: a quality record of another
    /// basis type that shares the case's id is never copied (keyed as the readers key it).
    #[test]
    fn operand_standing_is_keyed_on_the_load_case_basis_type() {
        let entry = |ref_type: &str, solve_quality: NumericalQualityStatus| NumericalCaseQuality {
            basis_ref: ResultBasisRef { ref_type: ref_type.into(), ref_id: "case:S".into() },
            structural_status: StructuralStatus::PassiveModelBasis,
            solve_quality,
            model_matrix_fidelity: ModelMatrixFidelity::NotAssessed,
            accuracy_evidence: AccuracyEvidence::NotClaimed,
            evidence_refs: Vec::new(),
        };
        let quality = NumericalQuality {
            cases: vec![entry("combination", NumericalQualityStatus::Unresolved), entry("load_case", NumericalQualityStatus::ChecksPassed)],
            ..unassessed_numerical_quality()
        };
        let mut records = vec![json!({"operands": [{"load_case_id": "case:S", "numerical_quality": null}]})];
        state_operand_standing(&mut records, &quality);
        let stated = &records[0]["operands"][0]["numerical_quality"];
        assert_eq!(stated["basis_ref"], json!({"ref_type": "load_case", "ref_id": "case:S"}));
        assert_eq!(stated, &serde_json::to_value(&quality.cases[1]).unwrap());
    }

    /// absolute-sum-srss §5.2 on EX, at the gate (no EX solve reaches a nonlinear model
    /// before RT-A3 increment 4): decision OQ-1's predicate, RT-A3's reference shape
    /// `mechanics(1·A + 1·srss(result_state_subtraction(A, B)))` admitted, and the
    /// singleton, two load cases, a factor other than 1, an undefined κ and a nonzero κ
    /// withheld with NONLINEAR_COMBINATION_REQUIRES_SOLVE; without a nonlinear support
    /// none of them is withheld by it.
    #[test]
    fn the_nonlinear_gate_admits_exactly_decision_oq1() {
        let model = |nonlinear: bool| -> PreviewModel {
            let mut supports = vec![json!({"id": "s:a", "node": "n:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]})];
            if nonlinear {
                supports.push(json!({"id": "s:gap", "node": "n:b", "family": "nonlinear", "restraints": [],
                    "nonlinear": {"behavior": "gap", "closes_when": "positive_displacement", "dof": "UX", "gap": {"value": 0.001, "unit": "m"}, "initial_state": "inactive"}}));
            }
            let term = |kind: &str, id: &str, factor: f64| if kind == "case" { json!({"load_case": id, "factor": factor}) } else { json!({"combination": id, "factor": factor}) };
            serde_json::from_value(json!({
                "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
                "project": {"id": "p", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
                "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
                "nodes": [], "pipe_segments": [], "supports": supports, "components": [], "materials": [],
                "load_cases": [{"id": "A", "primitive_loads": []}, {"id": "B", "primitive_loads": []}],
                "combinations": [
                    {"id": "D", "basis": "result_state_subtraction", "minuend_id": "A", "subtrahend_id": "B"},
                    {"id": "E", "basis": "srss", "terms": [term("comb", "D", 1.0)]},
                    {"id": "ref", "basis": "mechanics", "terms": [term("case", "A", 1.0), term("comb", "E", 1.0)]},
                    {"id": "singleton", "basis": "mechanics", "terms": [term("case", "A", 1.0)]},
                    {"id": "two-cases", "basis": "mechanics", "terms": [term("case", "A", 1.0), term("case", "B", 1.0), term("comb", "E", 1.0)]},
                    {"id": "factor", "basis": "mechanics", "terms": [term("case", "A", 1.5), term("comb", "E", 1.0)]},
                    {"id": "S", "basis": "srss", "terms": [term("case", "A", 1.0)]},
                    {"id": "undefined", "basis": "mechanics", "terms": [term("case", "B", 1.0), term("comb", "S", 1.0)]},
                    {"id": "R", "basis": "range_envelope", "operand_ids": ["A", "B"], "mode": "max"},
                    {"id": "nonzero", "basis": "mechanics", "terms": [term("case", "B", 1.0), term("comb", "R", 1.0)]}
                ]
            })).unwrap()
        };
        let connector = json!([]);
        let facts = Facts { exact_cases: &[], pressure: &[], load_reference_states: &[], connector: &connector };
        for nonlinear in [true, false] {
            let m = model(nonlinear);
            let plan = combination_plan::Plan::new(&m);
            let gate = |id: &str| {
                let index = m.combinations.iter().position(|c| c.id == id).unwrap();
                let expression = CombinationExpression::resolve(&m.combinations[index]).unwrap();
                let targets = expression.targets(&plan.resolver);
                gate_reason(&m, &plan, index, &facts, &targets, &HashMap::new()).map(|(code, _)| code)
            };
            assert!(plan.kappa[0].is_exact_zero(), "κ(subtraction) = 0");
            for id in ["D", "E", "ref", "S", "R"] {
                assert_eq!(gate(id), None, "nonlinear={nonlinear} {id}");
            }
            for id in ["singleton", "two-cases", "factor", "undefined", "nonzero"] {
                assert_eq!(gate(id), nonlinear.then_some(NONLINEAR), "nonlinear={nonlinear} {id}");
            }
            assert!(plan.solved_state_plus_increments(&m.combinations[2]));
        }
    }
}
