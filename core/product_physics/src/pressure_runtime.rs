//! Explicit pressure regions, normalized evidence and load ownership.
//!
//! Only the pressure-Poisson eigenload enters `eigenloads`; thermal and
//! mechanical loads remain owned by the caller. Terminal cap transfer is a
//! distinct applied-load ledger and must never be subtracted in wall recovery.
//!
//! T4-U2 (H-2): under `3.0.0/exact_pressure_v3` a region may change direction
//! at realized bends. Straight members keep the pre-cancelled ledger and its
//! collinearity guard per straight run; each arc carries its own state
//! (`ArcPressure`), its −c_b pair and, at each bend-adjacent junction, the
//! remainder +pAi(t_in − t_out) in the source groups; its K_b·u_free(ε_p) is
//! pushed by the case ledger (`lib.rs`), and `bend_pressure` recovers it.

use super::pressure_exact::{ExactSectionBases, InternalDifferentialPressure, IsotropicENu, SectionBasis};
use super::{
    diag, has_blocking, is_temperature_change_dimension,
    normalize_quantity, stable_suffix, BuiltModel, Diagnostic, LoadTargetInput, MaterialInput,
    PreviewLoadCase, PreviewModel, Quantity, DOF_PER_NODE,
};
use open_pipe_stress_correct_norm::norm3;
use open_pipe_stress_units::Dimension;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

pub(crate) use super::exact_admission::ExactContract;
use super::exact_admission::{self, component_family, support_family, Admission, ExactFamily};
/// The retired legacy and v2 contracts: recognized only so that they are refused by name.
const RETIRED_MODE: &str = "legacy_pressure_v1";
const RETIRED_VERSION: &str = "1.0.0";
const RETIRED_V2_MODE: &str = "exact_straight_pressure_v2";
const RETIRED_V2_VERSION: &str = "2.0.0";
const PRESSURE_BASIS: &str = "internal_differential_zero_external_v1";
const MATERIAL_BASIS: &str = "homogeneous_isotropic_E_nu_v1";
/// Representation agreement only, not an engineering geometry tolerance.
const REPRESENTATION_GUARD: f64 = 64.0 * f64::EPSILON;
/// T4-U2: the region rule at realized bends, as published in evidence.
const ARC_TANGENCY_RULE: &str = "a region changes direction only at a realized bend; each straight run keeps the 64*epsilon collinearity guard (a straight-straight kink is PRESSURE_REGION_NONCOLLINEAR); at each bend-adjacent junction theta = atan2(|t_in x t_out|, t_in . t_out) <= alpha_tan is admitted and carried exactly by the remainder pAi(t_in - t_out), and a larger theta is refused as a mitre (PRESSURE_REGION_MITRE_UNSUPPORTED)";
/// T4-U2 (D-D): the named refusal of a direction change above alpha_tan at a
/// realized bend's end; static text (T3 O-10).
const MITRE_UNSUPPORTED: &str = "PRESSURE_REGION_MITRE_UNSUPPORTED";
/// The exact contract's two shear models (DEL-05-03 Design §3, §5).
pub(crate) const EULER_BERNOULLI: &str = "euler_bernoulli";
pub(crate) const TIMOSHENKO_ANNULUS_ENERGY: &str = "timoshenko_annulus_energy_v1";

/// T4-U8 (DEL-05-03 Design §3.6): whether an exact-contract document selects
/// the Timoshenko straight member (per document; realized arcs stay
/// Euler-Bernoulli).
pub(crate) fn selects_timoshenko(model: &PreviewModel) -> bool {
    is_exact(model)
        && model.pressure_contract.as_ref().and_then(|contract| contract.shear_model.as_deref())
            == Some(TIMOSHENKO_ANNULUS_ENERGY)
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PressureContractInput {
    pub version: Option<String>,
    pub mode: Option<String>,
    pub shear_model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PressureRegionInput {
    pub id: Option<String>,
    pub member_pipe_ids: Option<Vec<String>>,
    /// Optional current-format connector edges of the region chain.
    #[serde(default, deserialize_with = "super::deserialize_present_value")]
    pub member_connector_ids: Option<Vec<String>>,
    pub pressure_basis: Option<String>,
    pub pressure: Option<Quantity>,
    pub terminals: Option<Vec<PressureTerminalInput>>,
    pub provenance: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PressureTerminalInput {
    pub node_ref: Option<String>,
    pub closure_transfer: Option<String>,
    pub provenance: Option<String>,
}

#[derive(Debug)]
pub(crate) struct ExactPressureCase {
    /// Indices address `BuiltModel::pipes`, never traversal-order indices.
    pub pipe_states: BTreeMap<usize, ExactPressurePipeState>,
    pub eigenloads: Vec<f64>,
    pub cap_loads: Vec<f64>,
    pub assembled_loads: Vec<f64>,
    /// S11 section 4.2: each source group's value, (dof, value, source), in
    /// group order. The case ledger pushes these operands, never the
    /// pre-summed `assembled_loads` total (kept as evidence).
    pub assembled_operands: Vec<(usize, f64, String)>,
    pub assembled_operand_bounds: Vec<f64>,
    /// Complete original factor-group source operands, before primitive collection.
    pub closure_sources: Vec<Vec<(String, open_pipe_stress_frame_kernel::structural::LoadFormula)>>,
    pub assembly_evidence: Value,
    pub evidence: Vec<Value>,
    pub joint_pressures: BTreeMap<String, f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct ExactPressurePipeState {
    pub region_id: String,
    /// D-5: the member's nominal and reduced sections and its pressure bore;
    /// every pressure load and the wall force use the bore, stresses each basis.
    pub sections: ExactSectionBases,
    pub pressure: InternalDifferentialPressure,
    pub material: IsotropicENu,
    /// Pressure-only applied RHS pair in this pipe's authored local i/j frame
    /// (straight members; `[0, 0]` on a realized arc, whose eigenstrain is
    /// `arc.strain`).
    pub eigenload_pair: [f64; 2],
    /// T4-U2: a realized curved bend's pressure data (H-2); `None` on a
    /// straight member.
    pub arc: Option<ArcPressure>,
}

/// T4-U2 (H-2): the pressure data of one realized arc in a v3 region. Its
/// member-owned term is K_b·u_free(ε_p) − c_b, with c_b = [−pAi·t_i,
/// +pAi·t_j]: the load pushes K_b·u_free(`strain`) (`bend_pressure`) and the
/// source groups carry −c_b; recovery subtracts u_free(`strain`) with the
/// thermal free expansion, then adds pAi to the axial force.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ArcPressure {
    /// The arc's unit end tangents [t_i, t_j], oriented i → j (CB's shared
    /// `arc_geometry` through the element).
    pub tangents: [[f64; 3]; 2],
    /// ε_p = (1 − 2ν)pAi/(E·As) as represented (`arc_pressure_strain`): the
    /// one value the load and the recovery share.
    pub strain: f64,
}

/// T4-U2: a bend-adjacent junction of a region chain, in traversal order.
#[derive(Debug, Clone, Copy)]
struct Junction {
    node: usize,
    /// Traversal indices of the incoming and outgoing members.
    before: usize,
    after: usize,
    /// Traversal-oriented unit directions at the node.
    t_in: [f64; 3],
    t_out: [f64; 3],
    theta: f64,
}

/// The sole current exact contract selects this route.
pub(crate) fn is_exact(model: &PreviewModel) -> bool {
    ExactContract::of(model).is_some()
}

/// The exact contract a model declares, if it is on the exact route.
pub(crate) fn exact_contract(model: &PreviewModel) -> Option<ExactContract> {
    ExactContract::of(model)
}

fn problem(
    diagnostics: &mut Vec<Diagnostic>,
    code: &str,
    refs: &[&str],
    message: impl Into<String>,
) {
    let mut finding = diag(
        &format!(
            "diagnostic:pressure-runtime:{}:{code}",
            stable_suffix(&refs.join(":"))
        ),
        code,
        "blocking",
        message,
        refs.iter().map(|value| value.to_string()).collect(),
    );
    finding.source = Some("core/product_physics/src/pressure_runtime.rs".to_string());
    diagnostics.push(finding);
}

/// T4-U0 (A3): the exact route's pressure recovery decision for one straight
/// member of an exact pressure region. A region member recovers from its
/// straight mechanical/thermal end actions (returned). A region member without
/// them is refused by name through `problem` and `None` is returned: it is
/// never recovered on its chord. Called only from `lib.rs`'s
/// `append_exact_pressure_results`; since T4-U2 a realized arc takes
/// `bend_pressure::append_arc_pressure_results` instead, so this refusal is a
/// guard that no admitted path reaches.
pub(crate) fn exact_member_recovery<'m>(
    diagnostics: &mut Vec<Diagnostic>,
    case_id: &str,
    pipe_id: &str,
    state: &ExactPressurePipeState,
    mechanical: Option<&'m [f64]>,
) -> Option<&'m [f64]> {
    if mechanical.is_none() {
        problem(
            diagnostics,
            "EXACT_PRESSURE_REGION_MEMBER_NOT_STRAIGHT",
            &[case_id, &state.region_id, pipe_id],
            "a curved (realized) bend member in an exact pressure region has no straight-member pressure recovery; it is refused, not recovered on its chord",
        );
    }
    mechanical
}

fn present(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|text| !text.trim().is_empty())
}

/// Validate the explicit profile and input namespace before unit conversion.
/// Region geometry is checked later against normalized, actually built pipes.
pub(crate) fn validate_profile(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    // T4-U3 (D-4): legacy joints are refused first, on every route and version.
    super::joint::refuse_legacy_joints(model, diagnostics);
    match model.schema_version.as_str() {
        crate::MODEL_SCHEMA_VERSION => {
            if let Some(contract) = &model.pressure_contract {
                let declared = (contract.version.as_deref(), contract.mode.as_deref());
                if declared == (Some(RETIRED_VERSION), Some(RETIRED_MODE))
                    || declared == (Some(RETIRED_V2_VERSION), Some(RETIRED_V2_MODE))
                {
                    // Retired contracts are refused by name with the re-author
                    // diagnostic (owner, 2026-10-10); nothing is reinterpreted.
                    problem(diagnostics, "PRESSURE_MODEL_REAUTHOR_REQUIRED", &["pressure_contract"],
                        "this pressure contract is retired; re-author the model with pressure_contract 3.0.0/exact_pressure_v3 (shear_model euler_bernoulli), an explicit corrosion_allowance on every pipe section, explicit pressure_regions (an explicit [] for an unpressurized case) and E/nu materials");
                } else if ExactContract::of(model).is_none() {
                    problem(diagnostics, "PRESSURE_CONTRACT_UNSUPPORTED", &["pressure_contract"],
                        "model document 0.7.0 accepts only 3.0.0/exact_pressure_v3; incomplete or unknown contracts never fall back");
                } else {
                    match contract.shear_model.as_deref() {
                        None => problem(diagnostics, "EXACT_PRESSURE_SHEAR_MODEL_REQUIRED",
                            &["pressure_contract", "shear_model"], "an exact pressure contract requires an explicit shear_model"),
                        // T4-U8 (DEL-05-03 Design §5): both shear models are admitted.
                        Some(EULER_BERNOULLI | TIMOSHENKO_ANNULUS_ENERGY) => {}
                        Some(_) => problem(diagnostics, "PRESSURE_CONTRACT_UNSUPPORTED",
                            &["pressure_contract", "shear_model"], "unknown shear_model; no fallback is used"),
                    }
                }
            }
        }
        version if crate::is_retired_model_schema_version(version) => problem(
            diagnostics,
            crate::MODEL_SCHEMA_VERSION_RETIRED,
            &["schema_version"],
            crate::retired_model_schema_message(version),
        ),
        _ => problem(
            diagnostics,
            "PREVIEW_SCHEMA_VERSION_UNSUPPORTED",
            &["schema_version"],
            "direct mechanics solves support exactly model version 0.7.0",
        ),
    }
    if model.schema_version == crate::MODEL_SCHEMA_VERSION {
        for pipe in &model.pipe_segments {
            match (&model.pressure_contract, &pipe.section.corrosion_allowance) {
                (Some(_), None) => problem(diagnostics,
                    "EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED", &[&pipe.id, "corrosion_allowance"],
                    "an exact pressure member section requires an explicit corrosion_allowance; zero is an authored value, never a default"),
                (None, Some(_)) => problem(diagnostics,
                    "CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT", &[&pipe.id, "corrosion_allowance"],
                    "corrosion_allowance requires an explicit pressure_contract"),
                _ => {}
            }
        }
    }

    let v3 = exact_contract(model) == Some(ExactContract::PressureV3);
    for component in &model.components {
        if let Some(contract) = &component.objective_connector {
            // T4-U3: a v3 joint's connector goes to the joint classifier.
            if v3
                && component.kind == "expansion_joint"
                && contract.get("version").and_then(Value::as_str) == Some("1.0.0")
            {
                continue;
            }
            let code = if model.pressure_contract.is_none() {
                "PREVIEW_CONTRACT_VERSION_MISMATCH"
            } else if contract.get("version").and_then(Value::as_str) != Some("1.0.0") {
                "OBJECTIVE_CONNECTOR_VERSION_UNSUPPORTED"
            } else {
                "OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED"
            };
            problem(diagnostics, code, &[&component.id, "objective_connector"],
                "an explicitly requested objective connector is not implemented in this pressure integration and is never ignored");
        }
    }
    let contract = exact_contract(model);
    let exact = contract.is_some();
    if let Some(contract) = contract {
        // T4-U2a: every non-base family goes through the admission seam, in
        // v2's emission order; under v2 the table returns v2's existing code
        // and text, under v3 it names the family. Each refusal is pushed here
        // through `problem`, as v2 did. Metadata-only fitting records are also
        // excluded: accepting one as a straight span would misrepresent the
        // explicit composition.
        for component in &model.components {
            // A legacy joint already carries its refusal; the seam skips it.
            if super::joint::is_legacy_joint(component) {
                continue;
            }
            if let Admission::Refused { code, message } =
                exact_admission::admission(contract, component_family(component))
            {
                problem(diagnostics, code, &[&component.id], message);
            }
        }
        // W-15 (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §2.7):
        // the admitted reducer is refused case by case where its mapped member
        // belongs to a pressure region.
        for refs in exact_admission::reducer_region_refusals(model) {
            problem(diagnostics, exact_admission::REDUCER_IN_REGION, &refs, exact_admission::REDUCER_IN_REGION_TEXT);
        }
        for support in &model.supports {
            if let Some(family) = support_family(support) {
                if let Admission::Refused { code, message } =
                    exact_admission::admission(contract, family)
                {
                    problem(diagnostics, code, &[&support.id], message);
                }
            }
        }
        check_suffixes(
            model.pipe_segments.iter().map(|pipe| pipe.id.as_str()),
            "pipe",
            diagnostics,
        );
        check_suffixes(
            model.nodes.iter().map(|node| node.id.as_str()),
            "node",
            diagnostics,
        );
        check_suffixes(
            model.supports.iter().map(|support| support.id.as_str()),
            "support",
            diagnostics,
        );
        check_suffixes(
            model.load_cases.iter().map(|case| case.id.as_str()),
            "load_case",
            diagnostics,
        );
        for combination in &model.combinations {
            if let Admission::Refused { code, message } =
                exact_admission::admission(contract, ExactFamily::Combination)
            {
                problem(diagnostics, code, &[&combination.id], message);
            }
        }
        if v3 {
            super::joint::classify_connectors(model, diagnostics);
        }
    }

    for case in &model.load_cases {
        if !exact {
            if case.pressure_regions.is_some() {
                problem(diagnostics, "PREVIEW_CONTRACT_VERSION_MISMATCH", &[&case.id, "pressure_regions"],
                    "pressure_regions requires the explicit exact pressure profile; legacy cases must omit the namespace");
            }
            // Legacy pressure is retired on every route: a pressure primitive of any
            // value, zero included, is refused in a document without the exact contract.
            for load in &case.primitive_loads {
                if load.category == "pressure" || load.dimension == "pressure" {
                    problem(diagnostics, "PRESSURE_MODEL_REAUTHOR_REQUIRED", &[&case.id, &load.id],
                        "legacy pressure primitives are retired, zero values included; remove the primitive, or re-author the model with pressure_contract 3.0.0/exact_pressure_v3 (shear_model euler_bernoulli), an explicit corrosion_allowance on every pipe section, explicit pressure_regions (an explicit [] for an unpressurized case) and E/nu materials");
                }
            }
            continue;
        }
        let contract = contract.expect("exact cases have a contract");
        if case.equivalent_static.is_some() {
            if let Admission::Refused { code, message } =
                exact_admission::admission(contract, ExactFamily::EquivalentStatic)
            {
                problem(diagnostics, code, &[&case.id, "equivalent_static"], message);
            } else if super::case_state::carries_fields(model) {
                // W-16 slice c (§3): a load/reference-state document (the
                // document-level detector, an authored null group included)
                // refuses every authored case's generation, listed in the
                // group or not, before the effective cases are selected.
                problem(diagnostics, exact_admission::EQUIVALENT_STATIC_LOAD_STATE,
                    &[&case.id, "equivalent_static"], exact_admission::EQUIVALENT_STATIC_LOAD_STATE_TEXT);
            }
        }
        for load in &case.primitive_loads {
            if load.category == "pressure" || load.dimension == "pressure" {
                problem(diagnostics, "EXACT_PRESSURE_REQUIRES_REGION", &[&case.id, &load.id],
                    "exact pressure is supplied only by pressure_regions; pressure-category or pressure-dimensional primitives are not a competing selector, including zero values");
            }
        }
        let Some(regions) = &case.pressure_regions else {
            problem(diagnostics, "EXACT_PRESSURE_REGIONS_REQUIRED", &[&case.id, "pressure_regions"],
                "every exact-profile case requires pressure_regions, including explicit [] when no pressure region is present");
            continue;
        };
        // Prospective runtime identity is (load_case_id, region_id); another
        // case may explicitly author the same named physical region.
        let mut region_ids = HashSet::new();
        let mut owned_pipes = HashSet::new();
        for (index, region) in regions.iter().enumerate() {
            let placeholder = format!("pressure_regions[{index}]");
            let id = present(&region.id).unwrap_or(&placeholder);
            if present(&region.id).is_none() {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_INPUT_MISSING",
                    &[&case.id, id, "id"],
                    "pressure region requires a nonempty ID",
                );
            } else {
                if !region_ids.insert(id.to_string()) {
                    problem(
                        diagnostics,
                        "PRESSURE_REGION_ID_DUPLICATE",
                        &[&case.id, id],
                        "pressure region IDs must be unique within a load case",
                    );
                }
            }
            if region.pressure_basis.as_deref() != Some(PRESSURE_BASIS) {
                problem(diagnostics, "PRESSURE_REGION_BASIS_UNSUPPORTED", &[&case.id, id, "pressure_basis"],
                    "pressure region requires internal_differential_zero_external_v1; external/submerged pressure is not inferred");
            }
            if region
                .pressure
                .as_ref()
                .map_or(true, |pressure| !pressure.value.is_finite())
            {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_INPUT_MISSING",
                    &[&case.id, id, "pressure"],
                    "pressure region requires an explicit finite signed pressure quantity",
                );
            }
            if present(&region.provenance).is_none() {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_INPUT_MISSING",
                    &[&case.id, id, "provenance"],
                    "pressure region requires a nonempty source/provenance record",
                );
            }
            match &region.member_pipe_ids {
                Some(members) if !members.is_empty() => {
                    let mut local = HashSet::new();
                    for member in members {
                        if member.trim().is_empty() || !local.insert(member) {
                            problem(
                                diagnostics,
                                "PRESSURE_REGION_MEMBERS_INVALID",
                                &[&case.id, id, member],
                                "region members must be nonempty unique pipe IDs",
                            );
                        } else if !owned_pipes.insert(member) {
                            problem(
                                diagnostics,
                                "PRESSURE_REGION_PIPE_OVERLAP",
                                &[&case.id, id, member],
                                "a pipe may belong to only one pressure region in a load case",
                            );
                        }
                    }
                }
                _ => problem(
                    diagnostics,
                    "PRESSURE_REGION_INPUT_MISSING",
                    &[&case.id, id, "member_pipe_ids"],
                    "pressure region requires a nonempty member_pipe_ids list",
                ),
            }
            match &region.terminals {
                Some(terminals) if terminals.len() == 2 => {
                    for (terminal_index, terminal) in terminals.iter().enumerate() {
                        let field = format!("terminals[{terminal_index}]");
                        if present(&terminal.node_ref).is_none()
                            || present(&terminal.provenance).is_none()
                        {
                            problem(
                                diagnostics,
                                "PRESSURE_TERMINAL_INPUT_MISSING",
                                &[&case.id, id, &field],
                                "each terminal requires explicit node_ref and nonempty provenance",
                            );
                        }
                        if !matches!(
                            terminal.closure_transfer.as_deref(),
                            Some("transfers_to_wall" | "separately_supported_or_compensated")
                        ) {
                            problem(diagnostics, "PRESSURE_TERMINAL_CLOSURE_INVALID", &[&case.id, id, &field],
                                "each terminal requires transfers_to_wall or separately_supported_or_compensated; closure transfer is never inferred");
                        }
                    }
                }
                _ => problem(
                    diagnostics,
                    "PRESSURE_TERMINALS_INVALID",
                    &[&case.id, id, "terminals"],
                    "pressure region requires exactly two ordered, distinct physical terminals",
                ),
            }
        }
    }
}

fn check_suffixes<'a>(
    ids: impl Iterator<Item = &'a str>,
    kind: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut seen = HashMap::new();
    for id in ids {
        if let Some(previous) = seen.insert(stable_suffix(id), id) {
            problem(diagnostics, "EXACT_PRESSURE_RESULT_ID_COLLISION", &[kind, previous, id],
                "exact pressure result identifiers must remain unique after stable-suffix normalization");
        }
    }
}

pub(crate) fn normalize_region_units(model: &mut PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    if !is_exact(model) {
        return;
    }
    for case in &mut model.load_cases {
        for (index, region) in case.pressure_regions.iter_mut().flatten().enumerate() {
            let placeholder = format!("pressure_regions[{index}]");
            let id = present(&region.id).unwrap_or(&placeholder);
            if let Some(pressure) = &mut region.pressure {
                if !pressure.value.is_finite() {
                    problem(
                        diagnostics,
                        "PRESSURE_REGION_PRESSURE_INVALID",
                        &[&case.id, id],
                        "pressure magnitude must be finite before normalization",
                    );
                    continue;
                }
                normalize_quantity(
                    pressure,
                    Dimension::Pressure,
                    &format!(
                        "diagnostic:unit-conversion:pressure-region:{}:{}",
                        stable_suffix(&case.id),
                        stable_suffix(id)
                    ),
                    vec![case.id.clone(), id.to_string(), "pressure".to_string()],
                    diagnostics,
                );
            }
        }
    }
}

struct TraversedPipe {
    pipe_index: usize,
    start_node: usize,
    end_node: usize,
    forward: bool,
}

/// Build from normalized geometry and the caller's already selected E/nu basis.
/// Errors return no partial case. Legacy mode returns None without exact loads.
pub(crate) fn build_pressure_case(
    model: &PreviewModel,
    built: &BuiltModel,
    materials: &[MaterialInput],
    case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<ExactPressureCase> {
    build_pressure_case_with_members(model, built, materials, case, None, diagnostics)
}

/// As `build_pressure_case`, but a resolved load/reference-state case supplies
/// each member's own selected E/nu pair by pipe ID. Thermal strain is owned by
/// the resolved member state and is never read from legacy primitives here.
pub(crate) fn build_pressure_case_with_members(
    model: &PreviewModel,
    built: &BuiltModel,
    materials: &[MaterialInput],
    case: &PreviewLoadCase,
    member_pairs: Option<&HashMap<String, IsotropicENu>>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<ExactPressureCase> {
    validate_profile(model, diagnostics);
    if has_blocking(diagnostics) || !is_exact(model) {
        return None;
    }
    assemble_pressure_case(model, built, materials, case, member_pairs, diagnostics)
}

/// The pressure producer after profile and connector classification. Kept
/// separate from document admission: B7b owns the 0.5.0 activation boundary.
fn assemble_pressure_case(
    model: &PreviewModel,
    built: &BuiltModel,
    materials: &[MaterialInput],
    case: &PreviewLoadCase,
    member_pairs: Option<&HashMap<String, IsotropicENu>>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<ExactPressureCase> {
    let Some(regions) = &case.pressure_regions else {
        problem(
            diagnostics,
            "EXACT_PRESSURE_REGIONS_REQUIRED",
            &[&case.id],
            "exact case requires pressure_regions",
        );
        return None;
    };
    let mut output = ExactPressureCase {
        pipe_states: BTreeMap::new(),
        eigenloads: vec![0.0; built.nodes.len() * DOF_PER_NODE],
        cap_loads: vec![0.0; built.nodes.len() * DOF_PER_NODE],
        assembled_loads: vec![0.0; built.nodes.len() * DOF_PER_NODE],
        assembled_operands: Vec::new(),
        assembled_operand_bounds: Vec::new(),
        closure_sources: Vec::new(),
        assembly_evidence: Value::Null,
        evidence: Vec::new(),
        joint_pressures: BTreeMap::new(),
    };
    let mut factor_groups = BTreeMap::new();
    let mut cap_terms = vec![Vec::new(); output.cap_loads.len()];
    let mut eigen_terms = vec![Vec::new(); output.eigenloads.len()];
    let mut material_map = HashMap::new();
    for material in materials {
        if material_map
            .insert(material.id.as_str(), material)
            .is_some()
        {
            problem(diagnostics, "EXACT_PRESSURE_MATERIAL_AMBIGUOUS", &[&material.id],
                "selected material list contains duplicate IDs; no material-list merge is performed");
        }
    }
    if has_blocking(diagnostics) {
        return None;
    }

    for region in regions {
        let id = region.id.as_deref()?;
        let Some((traversal, junctions)) = traverse_region(model, built, case, region, diagnostics)
        else {
            continue;
        };
        let input_pressure = region.pressure.as_ref()?;
        if input_pressure.unit != "Pa" {
            problem(
                diagnostics,
                "PRESSURE_REGION_NOT_NORMALIZED",
                &[&case.id, id],
                "pressure assembly requires canonical Pa input after the public normalization seam",
            );
            continue;
        }
        let pressure = match InternalDifferentialPressure::new(input_pressure.value) {
            Ok(value) => value,
            Err(error) => {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_PRESSURE_INVALID",
                    &[&case.id, id],
                    format!("invalid normalized pressure: {error:?}"),
                );
                continue;
            }
        };
        let mut geometry = Vec::new();
        let mut material_evidence = Vec::new();
        let mut load_evidence = Vec::new();
        let mut first_bore: Option<f64> = None;
        let mut region_states = Vec::new();
        for step in &traversal {
            let pipe = &built.pipes[step.pipe_index];
            let authored = model
                .pipe_segments
                .iter()
                .find(|p| p.id == pipe.element_id)?;
            let Some(_section) = built.sections.get(&pipe.element_id) else {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_SECTION_MISSING",
                    &[&case.id, id, &pipe.element_id],
                    "built pipe section is missing",
                );
                continue;
            };
            let Some(&sections) = built.exact_sections.get(&pipe.element_id) else {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_SECTION_MISSING",
                    &[&case.id, id, &pipe.element_id],
                    "built exact source-annulus geometry is missing",
                );
                continue;
            };
            // D-5: the equal-bore check compares pressure bores r_i,r.
            let bore = sections.pressure_bore_radius_m();
            if let Some(reference) = first_bore {
                let scale = reference.abs().max(bore.abs());
                if (reference - bore).abs() > REPRESENTATION_GUARD * scale {
                    problem(diagnostics, "PRESSURE_REGION_BORE_MISMATCH", &[&case.id, id, &pipe.element_id],
                        format!("region requires equal normalized bore; radius difference exceeds 64*epsilon*radius_scale (scale={scale:e} m); this is a representation guard, not a physical reducer allowance"));
                    continue;
                }
            } else {
                first_bore = Some(bore);
            }
            let resolved_pair = member_pairs.map(|pairs| pairs.get(&pipe.element_id));
            if resolved_pair == Some(None) {
                problem(
                    diagnostics,
                    "LOAD_STATE_MEMBER_MATERIAL_MISSING",
                    &[&case.id, id, &pipe.element_id],
                    "the resolved case does not supply this pressure member's selected E/nu pair",
                );
                continue;
            }
            let Some(material_input) = material_map.get(authored.material.as_str()) else {
                problem(
                    diagnostics,
                    "EXACT_PRESSURE_MATERIAL_MISSING",
                    &[&case.id, id, &authored.material],
                    "the selected material list does not cover a pressure member",
                );
                continue;
            };
            let material = if let Some(Some(&pair)) = resolved_pair {
                pair
            } else {
            let Some(nu) = &material_input.poisson_ratio else {
                problem(
                    diagnostics,
                    "EXACT_PRESSURE_POISSON_RATIO_REQUIRED",
                    &[&case.id, &authored.material],
                    "selected exact material requires explicit Poisson ratio",
                );
                continue;
            };
            if material_input.constitutive_basis.as_deref() != Some(MATERIAL_BASIS)
                || material_input.elastic_modulus.unit != "Pa"
                || nu.unit != "1"
            {
                problem(diagnostics, "EXACT_PRESSURE_MATERIAL_BASIS_REQUIRED", &[&case.id, &authored.material],
                    "region assembly requires a normalized common homogeneous_isotropic_E_nu_v1 material basis (E in Pa, nu in 1)");
                continue;
            }
            match IsotropicENu::new(material_input.elastic_modulus.value, nu.value) {
                Ok(value) => value,
                Err(error) => {
                    problem(
                        diagnostics,
                        "EXACT_PRESSURE_MATERIAL_INVALID",
                        &[&case.id, &authored.material],
                        format!("selected E/nu pair is invalid: {error:?}"),
                    );
                    continue;
                }
            }
            };
            let thermal_consumed = member_pairs.is_none() && case.primitive_loads.iter().any(|load| {
                load.category == "thermal"
                    && is_temperature_change_dimension(&load.dimension)
                    && matches!(&load.target, LoadTargetInput::Element { pipe: target } if target == &pipe.element_id)
            });
            let alpha = thermal_consumed
                .then(|| material_input.thermal_expansion_coefficient.as_ref())
                .flatten();
            if thermal_consumed && alpha.map_or(true, |quantity| !quantity.value.is_finite()) {
                problem(diagnostics, "THERMAL_EXPANSION_INPUT_MISSING", &[&case.id, &authored.material, &pipe.element_id],
                    "thermal pressure-region composition requires finite alpha from the same selected material basis; absence is not zero");
                continue;
            }
            // D-5: the Poisson pair and caps on the pressure bore; each
            // basis's own surface stresses must be representable.
            let loads = sections
                .poisson_eigenload_pair(material, pressure)
                .and_then(|eigen| sections.cap_pair(pressure).map(|caps| (eigen, caps)))
                .and_then(|pairs| {
                    for basis in SectionBasis::BOTH {
                        sections.surface_stresses(basis, pressure)?;
                    }
                    Ok(pairs)
                });
            let (eigen, caps) = match loads {
                Ok(pairs) => pairs,
                Err(error) => {
                    problem(
                        diagnostics,
                        "EXACT_PRESSURE_OUTPUT_UNREPRESENTABLE",
                        &[&case.id, id, &pipe.element_id],
                        format!("pressure load or surface stress cannot be represented: {error:?}"),
                    );
                    continue;
                }
            };
            let direction = match pipe
                .frame_element()
                .map_err(|error| error.to_string())
                .and_then(|frame| frame.orientation().map_err(|error| error.to_string()))
            {
                Ok(orientation) => orientation.local_axes[0],
                Err(error) => {
                    problem(
                        diagnostics,
                        "PRESSURE_REGION_GEOMETRY_INVALID",
                        &[&case.id, id, &pipe.element_id],
                        error.to_string(),
                    );
                    continue;
                }
            };
            // T4-U2: a realized arc in a v3 region (bends never reach here
            // under v2: the seam refuses them). Its end directions are the
            // arc's end tangents; a straight member's are its local x.
            let arc_build = super::realized_arc(built, step.pipe_index);
            let arc = match arc_build {
                None => None,
                Some(bend) => match (
                    bend.macro_element.end_tangents(),
                    sections.arc_pressure_strain(material, pressure),
                ) {
                    (Ok(tangents), Ok(strain)) => Some(ArcPressure { tangents, strain }),
                    (Err(_), _) => {
                        problem(diagnostics, "PRESSURE_REGION_GEOMETRY_INVALID", &[&case.id, id, &pipe.element_id],
                            "the realized bend's end tangents cannot be formed from its chord, radius and y_reference");
                        continue;
                    }
                    (_, Err(_)) => {
                        problem(diagnostics, "EXACT_PRESSURE_OUTPUT_UNREPRESENTABLE", &[&case.id, id, &pipe.element_id],
                            "the realized bend's pressure strain (1-2nu)pAi/(E As) cannot be represented");
                        continue;
                    }
                },
            };
            let end_directions = arc.map_or([direction, direction], |arc| arc.tangents);
            let mut member_geometry = super::exact_section_evidence(&pipe.element_id, sections);
            member_geometry["traversal_forward"] = json!(step.forward);
            if let Some(arc) = &arc {
                super::mark_realized_arc(&mut member_geometry);
                member_geometry["arc_end_tangents_global"] = json!(arc.tangents);
                // T4-U4b (§2.5): R, an operand of the toroidal hoop rows.
                member_geometry["bend_radius_m"] = json!(arc_build.expect("a realized arc").bend_radius);
            }
            geometry.push(member_geometry);
            material_evidence.push(json!({"pipe_id":pipe.element_id,"material_id":material_input.id,
                "E_pa":material.elastic_modulus_pa(),"nu":material.poisson_ratio(),"G_pa":material.shear_modulus_pa(),
                "constitutive_basis":MATERIAL_BASIS,"temperature_basis":if member_pairs.is_some() { json!("resolved_member_state") } else { temperature_basis(case) },"provenance":material_input.provenance,
                "thermal_consumed":thermal_consumed,"alpha_per_kelvin":alpha.map(|quantity|quantity.value)}));
            match &arc {
                None => load_evidence.push(json!({"pipe_id":pipe.element_id,"eigenload_pair_local_n":eigen,
                    "mathematical_cap_pair_local_n":caps,"local_x_global":direction,"thermal_included":false})),
                Some(arc) => load_evidence.push(json!({"pipe_id":pipe.element_id,
                    "bend_term":"K_b*u_free(eps_p)-c_b","arc_pressure_strain":arc.strain,
                    "arc_pressure_strain_definition":"(1-2nu)pAi/(E As)",
                    "mathematical_cap_pair_local_n":caps,"arc_end_tangents_global":arc.tangents,
                    "bend_cap_pair_removed_global_n":[arc.tangents[0].map(|v| -v * caps[0]), arc.tangents[1].map(|v| -v * caps[1])],
                    "thermal_included":false})),
            }
            region_states.push((
                step,
                ExactPressurePipeState {
                    region_id: id.to_string(),
                    sections,
                    pressure,
                    material,
                    eigenload_pair: if arc.is_some() { [0.0, 0.0] } else { eigen },
                    arc,
                },
                caps,
                end_directions,
            ));
        }
        if region_states.len() != traversal.len() || has_blocking(diagnostics) {
            continue;
        }
        let terminals = region.terminals.as_ref()?;
        let mut terminal_evidence = Vec::new();
        for (index, terminal) in terminals.iter().enumerate() {
            let (step, _, caps, directions) = if index == 0 {
                &region_states[0]
            } else {
                region_states.last()?
            };
            let node = if index == 0 {
                step.start_node
            } else {
                step.end_node
            };
            let pipe = &built.pipes[step.pipe_index];
            let local_slot = if node == pipe.node_i.index { 0 } else { 1 };
            let cap_global = directions[local_slot].map(|value| value * caps[local_slot]);
            let transfers = terminal.closure_transfer.as_deref() == Some("transfers_to_wall");
            let transferred = if transfers { cap_global } else { [0.0; 3] };
            for axis in 0..3 {
                cap_terms[node * DOF_PER_NODE + axis].push(transferred[axis]);
            }
            terminal_evidence.push(
                json!({"node_ref":terminal.node_ref,"closure_transfer":terminal.closure_transfer,
                "provenance":terminal.provenance,"closure_pressure_load_global_n":cap_global,
                "pipe_cap_transfer_global_n":transferred,"remote_closure_support_reaction_global_n":
                    if transfers { Value::Null } else { json!(cap_global.map(|value| -value)) },
                "remote_closure_excluded_from_pipe_solve":!transfers}),
            );
        }
        // T4-U2 (H-2): at each bend-adjacent junction the remainder
        // +pAi(t_in - t_out), in traversal order. Each direction's term is
        // keyed with the state of the member it belongs to, so the arc's own
        // tangent term cancels its -c_b term exactly (same key, coefficients
        // +1 and -1): an admitted kink is carried exactly, never snapped.
        let mut junction_evidence = Vec::new();
        for junction in &junctions {
            for (traversal_index, t, coefficient) in
                [(junction.before, junction.t_in, 1.0), (junction.after, junction.t_out, -1.0)]
            {
                let (step, state, _, _) = &region_states[traversal_index];
                let pipe = &built.pipes[step.pipe_index];
                let source_zero = source_zero_axes(built, step.pipe_index);
                for axis in 0..3 {
                    add_factor_term(
                        &mut factor_groups,
                        junction.node * DOF_PER_NODE + axis,
                        state,
                        t[axis],
                        source_zero[axis],
                        coefficient,
                        id,
                        &pipe.element_id,
                        "kink_remainder",
                    );
                }
            }
            junction_evidence.push(json!({"node_ref":model.nodes[junction.node].id,
                "pipe_in":built.pipes[region_states[junction.before].0.pipe_index].element_id,
                "pipe_out":built.pipes[region_states[junction.after].0.pipe_index].element_id,
                "t_in_global":junction.t_in,"t_out_global":junction.t_out,"theta_rad":junction.theta}));
        }
        let has_arc = region_states.iter().any(|(_, state, _, _)| state.arc.is_some());
        for (step, state, _, directions) in region_states {
            let pipe = &built.pipes[step.pipe_index];
            let source_zero = source_zero_axes(built, step.pipe_index);
            for (end, node) in [pipe.node_i.index, pipe.node_j.index]
                .into_iter()
                .enumerate()
            {
                let direction = directions[end];
                for axis in 0..3 {
                    let sign = if end == 0 { 1.0 } else { -1.0 };
                    if state.arc.is_none() {
                        eigen_terms[node * DOF_PER_NODE + axis]
                            .push(direction[axis] * state.eigenload_pair[end]);
                        add_factor_term(
                            &mut factor_groups,
                            node * DOF_PER_NODE + axis,
                            &state,
                            direction[axis],
                            source_zero[axis],
                            sign * 2.0 * state.material.poisson_ratio(),
                            id,
                            &pipe.element_id,
                            "poisson_eigen",
                        );
                    } else {
                        // -c_b: +pAi t_i at node i, -pAi t_j at node j (H-2).
                        add_factor_term(
                            &mut factor_groups,
                            node * DOF_PER_NODE + axis,
                            &state,
                            direction[axis],
                            source_zero[axis],
                            sign,
                            id,
                            &pipe.element_id,
                            "bend_cap_removed",
                        );
                    }
                    let node_ref = &model.nodes[node].id;
                    if terminals.iter().any(|terminal| {
                        terminal.node_ref.as_ref() == Some(node_ref)
                            && terminal.closure_transfer.as_deref() == Some("transfers_to_wall")
                    }) {
                        add_factor_term(
                            &mut factor_groups,
                            node * DOF_PER_NODE + axis,
                            &state,
                            direction[axis],
                            source_zero[axis],
                            -sign,
                            id,
                            &pipe.element_id,
                            "terminal_cap",
                        );
                    }
                }
            }
            if output.pipe_states.insert(step.pipe_index, state).is_some() {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_PIPE_OVERLAP",
                    &[&case.id, id, &pipe.element_id],
                    "multiple pressure regions own the same built pipe",
                );
            }
        }
        let contract = model.pressure_contract.as_ref().and_then(|contract| ExactContract::from_declared(contract.version.as_deref(), contract.mode.as_deref())).expect("classified exact pressure contract");
        output.evidence.push(json!({"region_id":id,"load_case_id":case.id,"profile_version":contract.version(),
            "profile_mode":contract.mode(),"member_pipe_ids":traversal.iter().map(|step| built.pipes[step.pipe_index].element_id.as_str()).collect::<Vec<_>>(),
            "terminals":terminal_evidence,"pressure_basis":PRESSURE_BASIS,"p_pa":pressure.pressure_pa(),"geometry":geometry,
            "materials":material_evidence,"applied_loads":load_evidence,"provenance":region.provenance,
            "approximation":"long_straight_annulus_small_strain_v2","external_pressure_increment_pa":0.0,
            "geometry_representation_guard":{"epsilon_multiplier":64,"meaning":"arithmetic_representation_only"}}));
        if has_arc {
            // T4-U2: only a region with a realized arc carries these keys, so
            // a straight-only region's evidence is unchanged (SP-1).
            let evidence = output.evidence.last_mut().expect("pushed above");
            evidence["approximation"] = json!("straight_lame_annulus_and_realized_arc_member_term_h2_small_strain_v3");
            evidence["tangency_tolerance_rad"] = json!(super::TANGENCY_TOLERANCE_RAD);
            evidence["tangency_rule"] = json!(ARC_TANGENCY_RULE);
            evidence["bend_adjacent_junctions"] = json!(junction_evidence);
            // T4-U4b (§2.5): the two arc definitions and what each neglects.
            evidence["arc_stress_definitions"] = super::u4b_rows::arc_stress_definitions();
        }
        add_joint_terms(model, built, case, region, pressure, &mut output, &mut factor_groups, diagnostics);
    }
    for dof in 0..output.cap_loads.len() {
        match (
            crate::pressure_sum::exact_sum(cap_terms[dof].iter().copied()),
            crate::pressure_sum::exact_sum(eigen_terms[dof].iter().copied()),
        ) {
            (Ok(cap), Ok(eigen)) => {
                output.cap_loads[dof] = cap;
                output.eigenloads[dof] = eigen;
            }
            _ => problem(
                diagnostics,
                "PRESSURE_LEDGER_SUM_UNREPRESENTABLE",
                &[&case.id],
                "rounded observational cap/eigen ledger sum is not representable",
            ),
        }
    }
    finish_source_groups(model, built, case, &mut output, factor_groups, diagnostics);
    if output
        .eigenloads
        .iter()
        .chain(&output.cap_loads)
        .any(|value| !value.is_finite())
    {
        problem(
            diagnostics,
            "EXACT_PRESSURE_OUTPUT_UNREPRESENTABLE",
            &[&case.id],
            "assembled pressure load vector contains a nonfinite sum",
        );
    }
    if has_blocking(diagnostics) {
        None
    } else {
        Some(output)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FactorArea { Bore(u64, u64), Joint(u64) }
type FactorKey = (usize, u64, FactorArea, u64);
struct FactorGroup {
    geometry: Option<ExactSectionBases>,
    pressure: InternalDifferentialPressure,
    direction: f64,
    terms: Vec<(f64, String, String, &'static str)>,
}

fn add_joint_terms(
    model: &PreviewModel,
    built: &BuiltModel,
    case: &PreviewLoadCase,
    region: &PressureRegionInput,
    pressure: InternalDifferentialPressure,
    output: &mut ExactPressureCase,
    groups: &mut BTreeMap<FactorKey, FactorGroup>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(ids) = region.member_connector_ids.as_ref().filter(|ids| !ids.is_empty()) else { return; };
    let region_id = region.id.as_deref().expect("classified region");
    let mut edges = Vec::new();
    for id in ids {
        let Some(index) = built.connector_records.iter().position(|record| &record.component_id == id) else {
            problem(diagnostics, super::joint::PRESSURE_INTERFACE_UNRESOLVED, &[&case.id, region_id, id], "listed joint was not built");
            continue;
        };
        let connector = &built.connectors[index];
        let Some(area) = built.connector_records[index].pressure.as_ref() else {
            problem(diagnostics, super::joint::PRESSURE_INTERFACE_UNRESOLVED, &[&case.id, region_id, id], "listed joint has no effective-area law");
            continue;
        };
        let mut faces = Vec::new();
        for node in [connector.node_i().index, connector.node_j().index] {
            let face = output.pipe_states.iter().filter(|(index, state)| {
                let pipe = &built.pipes[**index];
                state.region_id == region_id && state.arc.is_none() && [pipe.node_i.index, pipe.node_j.index].contains(&node)
            }).map(|(index, _)| *index).collect::<Vec<_>>();
            if face.len() != 1 { break; }
            faces.push(face[0]);
        }
        if faces.len() != 2 {
            problem(diagnostics, super::joint::PRESSURE_INTERFACE_UNRESOLVED, &[&case.id, region_id, id], "each joint face needs exactly one built straight pressure member");
            continue;
        }
        let pe = pressure.pressure_pa() * area.effective_area;
        if !pe.is_finite() {
            problem(diagnostics, "EXACT_PRESSURE_OUTPUT_UNREPRESENTABLE", &[&case.id, region_id, id], "joint pressure force is not representable");
            continue;
        }
        let mut pair = [[0.0; 3]; 2];
        let mut differences = [0.0; 2];
        let mut bore = [0.0; 2];
        for (end, node) in [connector.node_i().index, connector.node_j().index].into_iter().enumerate() {
            let sign = if end == 0 { -1.0 } else { 1.0 };
            let face_index = faces[end];
            let pipe = &built.pipes[face_index];
            let state = &output.pipe_states[&face_index];
            let direction = pipe.frame_element().expect("classified face frame").orientation().expect("classified face orientation").local_axes[0];
            let face_sign = if node == pipe.node_i.index { -1.0 } else { 1.0 };
            let source_zero = source_zero_axes(built, face_index);
            bore[end] = state.sections.pressure_bore_area_m2();
            let cap = state.sections.pressure_group_value(pressure, 1.0, 1.0).expect("validated face cap");
            differences[end] = crate::pressure_sum::exact_sum([pe, -cap]).expect("finite area difference");
            for axis in 0..3 {
                let dof = node * DOF_PER_NODE + axis;
                let d = connector.axes()[axis][0];
                // Authored zero only. A nonzero source whose fold is zero
                // retains the complete source and its closure identity.
                if d != 0.0 {
                    groups.entry((dof, pressure.pressure_pa().to_bits(), FactorArea::Joint(area.effective_area.to_bits()), d.abs().to_bits()))
                        .or_insert_with(|| FactorGroup { geometry: None, pressure, direction: d.abs(), terms: Vec::new() })
                        .terms.push((sign * if d.is_sign_negative() { -1.0 } else { 1.0 }, region_id.into(), id.clone(), "joint_effective_area"));
                }
                add_factor_term(groups, dof, state, direction[axis], source_zero[axis], face_sign, region_id, &pipe.element_id, "joint_face_cap");
                pair[end][axis] = crate::pressure_sum::exact_sum([sign * pe * d, face_sign * cap * direction[axis]]).expect("finite attachment pair");
            }
        }
        output.joint_pressures.insert(id.clone(), pressure.pressure_pa());
        edges.push(json!({"connector_id":id,"node_i_ref":model.nodes[connector.node_i().index].id,"node_j_ref":model.nodes[connector.node_j().index].id,
            "face_i_pipe_id":built.pipes[faces[0]].element_id,"face_j_pipe_id":built.pipes[faces[1]].element_id,"p_pa":pressure.pressure_pa(),
            "effective_area_m2":area.effective_area,"face_bore_area_m2":bore,"joint_pressure_force_n":pe,"area_difference_force_n":differences,
            "attachment_pair_global_n":pair,"representation":"B: p(Ae - Ai) at the attachments; joint_effective_area + joint_face_cap"}));
    }
    let evidence = output.evidence.last_mut().expect("region evidence");
    evidence["member_connector_ids"] = json!(ids);
    evidence["joint_edges"] = json!(edges);
    evidence["approximation"] = json!(if evidence.get("bend_adjacent_junctions").is_some() {
        "straight_lame_annulus_realized_arc_and_joint_area_difference_h2_small_strain_v3"
    } else { "straight_lame_annulus_and_joint_area_difference_h2_small_strain_v3" });
}
/// The global axes on which a member's direction component is source-known
/// exactly zero: its chord has a zero component there and, on a realized
/// arc, so has its y_reference. Then the component's p re-formation
/// (`PressureOperand`, from the same chord, radius and y_reference) is exactly
/// zero with a zero enclosure radius. Any other component is formed, and
/// DEL-04-01 `kd5-member-relative-bound.md` section 1 ("One ε") forbids
/// omitting it because its binary64 fold is zero: such a summand keeps its
/// zero-valued group and its closure operand, so the ordinary force bits are
/// unchanged and the formed load enters the closed residual.
fn source_zero_axes(built: &BuiltModel, pipe_index: usize) -> [bool; 3] {
    let zero = |xi: [f64; 3], xj: [f64; 3], yr: Option<[f64; 3]>| {
        [0, 1, 2].map(|k| xi[k] == xj[k] && yr.map_or(true, |yr| yr[k] == 0.0))
    };
    match built
        .curved_bend_elements
        .iter()
        .find(|bend| bend.pipe_index == pipe_index)
    {
        Some(bend) => {
            let m = &bend.macro_element;
            zero(m.node_i.coordinates, m.node_j.coordinates, Some(m.y_reference))
        }
        None => built.pipes[pipe_index]
            .frame_element()
            .map_or([false; 3], |frame| zero(frame.node_i.coordinates, frame.node_j.coordinates, None)),
    }
}
#[allow(clippy::too_many_arguments)]
fn add_factor_term(
    groups: &mut BTreeMap<FactorKey, FactorGroup>,
    dof: usize,
    state: &ExactPressurePipeState,
    direction: f64,
    source_zero: bool,
    coefficient: f64,
    region: &str,
    pipe: &str,
    kind: &'static str,
) {
    if source_zero && direction == 0.0 {
        return;
    } // Only a source-known zero is omitted (K-D5 "One ε"); a formed component whose binary64 fold is zero is kept.
    let (hi, lo) = state.sections.source_bore_key();
    let magnitude = direction.abs();
    let key = (
        dof,
        state.pressure.pressure_pa().to_bits(),
        FactorArea::Bore(hi, lo),
        magnitude.to_bits(),
    );
    groups
        .entry(key)
        .or_insert_with(|| FactorGroup {
            geometry: Some(state.sections),
            pressure: state.pressure,
            direction: magnitude,
            terms: Vec::new(),
        })
        .terms
        .push((
            if direction.is_sign_negative() {
                -coefficient
            } else {
                coefficient
            },
            region.to_string(),
            pipe.to_string(),
            kind,
        ));
}
/// K-D5's load closure (DEL-04-01 `kd5-member-relative-bound.md` section 1,
/// "T4-U2 pressure operand groups"): one `PressureOperand` per factor-group
/// summand, bound to its own pipe, coefficient and direction, from the
/// producer's enumeration of the group (never from the supplied primitives).
///
/// - A straight member's summand (`poisson_eigen`, `terminal_cap`,
///   `kink_remainder`) acts along its chord (local x).
/// - A realized arc's summand (`bend_cap_removed`, `terminal_cap`,
///   `kink_remainder`) acts along its end tangent at the group's node,
///   formed at p from the arc's chord, radius and y_reference as K_int,arc is
///   (T4-U2 H-2: c_b and the remainders lie along the end tangents).
///
/// The coefficient's sign is the summand's signed contribution relative to
/// the binary64 direction the producer used; its magnitude is 1 for caps and
/// remainders and 2|nu| for the straight Poisson pair, so the re-formed value
/// is p*pi*r_i^2 times the summand's signed coefficient times the direction.
fn closure_source(
    built: &BuiltModel,
    output: &ExactPressureCase,
    dof: usize,
    term: &(f64, String, String, &'static str),
) -> (String, open_pipe_stress_frame_kernel::structural::LoadFormula) {
    use open_pipe_stress_frame_kernel::structural::{
        LoadFormula, PressureCoefficient, PressureDirection, PressureGeometry,
    };
    if term.3 == "joint_effective_area" {
        let index = built.connector_records.iter().position(|record| record.component_id == term.2).expect("classified joint pressure source");
        let connector = &built.connectors[index];
        let model = built.connector_records[index].pressure.as_ref().expect("classified joint area");
        let axis = dof % DOF_PER_NODE;
        let end = usize::from(dof / DOF_PER_NODE == connector.node_j().index);
        let direction = connector.axes()[axis][0];
        let sign = if term.0.is_sign_negative() == direction.is_sign_negative() { 1.0 } else { -1.0 };
        // The case's owning region is the sole pressure authority.
        let pressure = output.evidence.iter().find(|region| region["region_id"] == term.1).and_then(|region| region["p_pa"].as_f64()).expect("joint pressure region");
        return (format!("{}:{}:{}", term.1, term.2, term.3), LoadFormula::JointEffectiveArea {
            connector: connector.clone(), end, pressure, effective_area: model.effective_area, axis, sign,
        });
    }
    let pipe_index = built
        .pipes
        .iter()
        .position(|pipe| pipe.element_id == term.2)
        .expect("pressure source pipe");
    let pipe = &built.pipes[pipe_index];
    let state = &output.pipe_states[&pipe_index];
    let axis = dof % DOF_PER_NODE;
    let (geometry, direction, binary64) = match (
        state.arc,
        built
            .curved_bend_elements
            .iter()
            .find(|bend| bend.pipe_index == pipe_index),
    ) {
        (Some(arc), Some(bend)) => {
            let end_j = dof / DOF_PER_NODE == pipe.node_j.index;
            (
                PressureGeometry::Curved { arc: super::curved_formation_of(bend) },
                PressureDirection::Tangent { end_j, axis },
                arc.tangents[usize::from(end_j)][axis],
            )
        }
        _ => {
            let frame = pipe.frame_element().expect("validated pressure frame");
            let direction = frame.orientation().expect("validated orientation").local_axes[0][axis];
            (PressureGeometry::Straight { frame }, PressureDirection::Chord { axis }, direction)
        }
    };
    let sign = if term.0.is_sign_negative() == binary64.is_sign_negative() { 1.0 } else { -1.0 };
    let coefficient = if term.3 == "poisson_eigen" {
        PressureCoefficient::Poisson {
            poisson_ratio: state.material.poisson_ratio(),
            sign: sign * 2.0 * if state.material.poisson_ratio().is_sign_negative() { -1.0 } else { 1.0 },
        }
    } else {
        PressureCoefficient::Constant(sign)
    };
    // T4-U8 (DEL-05-03 Design §4 item 2): a straight operand's identity begins
    // with its member's identity (`member:{len}:{pipe}:`), so K-D5 binds its
    // frame to that member's formation-source frame; an arc operand keeps its
    // region-first identity.
    let identity = match geometry {
        PressureGeometry::Straight { .. } => format!("{}{}:{}", super::member_prefix(&term.2), term.1, term.3),
        _ => format!("{}:{}:{}", term.1, term.2, term.3),
    };
    (
        identity,
        LoadFormula::PressureOperand {
            geometry,
            pressure: state.pressure.pressure_pa(),
            // D-5: Ai on the pressure bore, D/2 - t_r.
            outer_diameter: state.sections.outside_diameter_m(),
            wall_thickness: state.sections.pressure_wall_m(),
            coefficient,
            direction,
        },
    )
}
fn finish_source_groups(
    model: &PreviewModel,
    built: &BuiltModel,
    case: &PreviewLoadCase,
    output: &mut ExactPressureCase,
    groups: BTreeMap<FactorKey, FactorGroup>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut by_dof = vec![Vec::new(); output.assembled_loads.len()];
    let mut evidence = Vec::new();
    for ((dof, pbits, area, _), group) in groups {
        let coefficient =
            match crate::pressure_sum::exact_sum(group.terms.iter().map(|term| term.0)) {
                Ok(value) => value,
                Err(error) => {
                    problem(
                        diagnostics,
                        "PRESSURE_ASSEMBLY_COEFFICIENT_UNREPRESENTABLE",
                        &[&case.id],
                        format!("source coefficient sum cannot be represented: {error:?}"),
                    );
                    continue;
                }
            };
        let formed = match area {
            FactorArea::Bore(..) => group.geometry.expect("bore geometry").pressure_group_value(group.pressure, coefficient, group.direction),
            FactorArea::Joint(bits) => {
                use super::pressure_exact::{Scaled, ExactPressureError};
                let value = Scaled::from_f64(group.pressure.pressure_pa()).mul(Scaled::from_f64(f64::from_bits(bits))).mul(Scaled::from_f64(coefficient)).mul(Scaled::from_f64(group.direction)).to_f64();
                if value.is_finite() { Ok(value) } else { Err(ExactPressureError::NonRepresentableLoad) }
            }
        };
        let value = match formed {
                Ok(value) => value,
                Err(error) => {
                    problem(
                        diagnostics,
                        "PRESSURE_ASSEMBLY_SOURCE_UNREPRESENTABLE",
                        &[&case.id],
                        format!("source-group pressure load cannot be represented: {error:?}"),
                    );
                    continue;
                }
            };
        by_dof[dof].push(value);
        let source = group
            .terms
            .first()
            .map_or_else(|| case.id.clone(), |term| term.1.clone());
        // Enumerate every original source constituent. This list is independent
        // of the primitive vector attached later by the ledger producer.
        let sources = group.terms.iter().map(|term| closure_source(built, output, dof, term)).collect();
        output.closure_sources.push(sources);
        output.assembled_operands.push((dof, value, source));
        let bound = if matches!(area, FactorArea::Joint(_)) && value == 0.0
            && group.pressure.pressure_pa() != 0.0 && coefficient != 0.0 && group.direction != 0.0
        {
            // A nonzero authored product may underflow at its final binary64
            // fold. Retain its source and one absolute subnormal reserve.
            f64::from_bits(1)
        } else { super::exact_pressure_operand_bound(value) };
        output.assembled_operand_bounds.push(bound);
        // The published groups observe the binary64 RHS. A formed group whose
        // binary64 direction folds to zero (K-D5 "One ε") adds exactly zero
        // to it: it stays a formed ledger term with its closure operands
        // above, and the observation lists, as before, only groups with a
        // nonzero binary64 direction (the reader's RHS_DIRECTION).
        if group.direction == 0.0 {
            continue;
        }
        let mut group_evidence = json!({"node_ref":model.nodes[dof/DOF_PER_NODE].id,"component":(["Fx","Fy","Fz"][dof%DOF_PER_NODE]),
            "pressure_bits":format!("{pbits:016x}"),
            "direction_component_magnitude":group.direction,"coefficient_sum":coefficient,"assembled_force_n":value,
            "terms":group.terms.iter().map(|term| {
                let mut evidence = json!({"coefficient":term.0,"region_id":term.1,"kind":term.3});
                evidence[if term.3 == "joint_effective_area" { "connector_id" } else { "pipe_id" }] = json!(term.2);
                evidence
            }).collect::<Vec<_>>()});
        match area {
            FactorArea::Bore(hi, lo) => {
                group_evidence["source_inner_radius_hi_bits"] = json!(format!("{hi:016x}"));
                group_evidence["source_inner_radius_lo_bits"] = json!(format!("{lo:016x}"));
            }
            FactorArea::Joint(bits) => group_evidence["effective_area_bits"] = json!(format!("{bits:016x}")),
        }
        evidence.push(group_evidence);
    }
    for (dof, values) in by_dof.iter().enumerate() {
        match crate::pressure_sum::exact_sum(values.iter().copied()) {
            Ok(total) => output.assembled_loads[dof] = total,
            Err(_) => problem(
                diagnostics,
                "PRESSURE_ASSEMBLY_SOURCE_UNREPRESENTABLE",
                &[&case.id],
                "pressure source group sum cannot be represented",
            ),
        }
    }
    let max_rhs = output
        .assembled_loads
        .iter()
        .enumerate()
        .filter(|(dof, _)| dof % DOF_PER_NODE < 3)
        .fold(0.0_f64, |m, (_, v)| m.max(v.abs()));
    // Normalize each positive magnitude first: an overflowing unnormalized sum
    // must not manufacture a failure when the screening ratio is representable.
    let mut maximum_ratio = 0.0_f64;
    for values in &by_dof {
        if max_rhs == 0.0 {
            if values.iter().any(|v| *v != 0.0) {
                maximum_ratio = f64::INFINITY;
                break;
            }
        } else {
            match crate::pressure_sum::exact_sum(values.iter().map(|v| v.abs() / max_rhs)) {
                Ok(ratio) => maximum_ratio = maximum_ratio.max(ratio),
                Err(_) => {
                    maximum_ratio = f64::INFINITY;
                    break;
                }
            }
        }
    }
    let screen = 32.0 * f64::EPSILON * maximum_ratio;
    if !screen.is_finite() || screen > 1e-9 {
        problem(diagnostics,"PRESSURE_ASSEMBLY_CANCELLATION_UNRESOLVED",&[&case.id],"different source factors strongly cancel beyond the bounded pressure-RHS screening domain; no source-accuracy or Current result is inferred from a small residual of rounded loads");
    }
    output.assembly_evidence = json!({"method":"source_factor_grouped_pressure_rhs_v1","load_case_id":case.id,
        "node_order":model.nodes.iter().map(|node|&node.id).collect::<Vec<_>>(),"dof_order":["Fx","Fy","Fz","Mx","My","Mz"],
        "assembled_pressure_rhs_global":output.assembled_loads,"groups":evidence,"rounded_cap_and_eigen_ledgers_are_observational":true,
        "rounded_cap_rhs_global":output.cap_loads,"rounded_poisson_rhs_global":output.eigenloads,"dof_units":["N","N","N","N*m","N*m","N*m"],
        "cancellation_screen":if screen.is_finite(){Some(screen)}else{None},"screen_limit":1e-9,"screen_roundoff_multiplier":32,
        "screen_is_not_numerical_qualification":true});
}

fn temperature_basis(case: &PreviewLoadCase) -> Value {
    if let Some(id) = &case.modulus_basis_ref {
        json!({"selection":"exact_point","point_id":id})
    } else if let Some(temperature) = &case.modulus_basis_temperature {
        json!({"selection":"interpolation","temperature_value":temperature.value,"temperature_unit":temperature.unit})
    } else {
        json!({"selection":"base_material"})
    }
}

/// The region's chain in traversal order and, where it contains a realized
/// arc, its bend-adjacent junctions (T4-U2). A region without an arc takes
/// exactly the pre-T4-U2 path: one collinear chain between its terminals.
fn traverse_region(
    model: &PreviewModel,
    built: &BuiltModel,
    case: &PreviewLoadCase,
    region: &PressureRegionInput,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<(Vec<TraversedPipe>, Vec<Junction>)> {
    let id = region.id.as_deref()?;
    let refs = [&*case.id, id];
    let pipe_members = region.member_pipe_ids.as_ref()?;
    // A joint is a graph edge, represented here only by its replaced span's
    // endpoints. The span never becomes a member pipe or a load producer.
    let joint_records = built.connector_records.iter().filter(|record| region.member_connector_ids.as_ref().is_some_and(|ids| ids.contains(&record.component_id))).collect::<Vec<_>>();
    let joint_spans = joint_records.iter().map(|record| record.span_index).collect::<HashSet<_>>();
    let mut members = pipe_members.clone();
    members.extend(joint_records.iter().map(|record| record.span_id.clone()));
    let terminals = region.terminals.as_ref()?;
    if members.is_empty() || terminals.len() != 2 {
        return None;
    }
    let node_map = model
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let terminal_nodes = terminals
        .iter()
        .map(|terminal| {
            terminal
                .node_ref
                .as_deref()
                .and_then(|name| node_map.get(name))
                .copied()
        })
        .collect::<Option<Vec<_>>>();
    let Some(terminal_nodes) = terminal_nodes else {
        problem(
            diagnostics,
            "PRESSURE_TERMINAL_NODE_UNKNOWN",
            &refs,
            "pressure terminals must reference existing model nodes",
        );
        return None;
    };
    if terminal_nodes[0] == terminal_nodes[1] {
        problem(
            diagnostics,
            "PRESSURE_TERMINALS_INVALID",
            &refs,
            "pressure terminals must be distinct chain endpoints",
        );
        return None;
    }
    let pipe_map = built
        .pipes
        .iter()
        .enumerate()
        .map(|(index, pipe)| (pipe.element_id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let mut adjacency: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let member_set = members.iter().map(String::as_str).collect::<HashSet<_>>();
    for member in &members {
        let Some(&index) = pipe_map.get(member.as_str()) else {
            problem(
                diagnostics,
                "PRESSURE_REGION_MEMBER_UNKNOWN",
                &[&case.id, id, member],
                "region member must reference an actually built straight pipe",
            );
            return None;
        };
        let pipe = &built.pipes[index];
        for node in [pipe.node_i.index, pipe.node_j.index] {
            adjacency.entry(node).or_default().push(index);
        }
    }
    for (&node, edges) in &adjacency {
        let terminal = terminal_nodes.contains(&node);
        if edges.len() != if terminal { 1 } else { 2 } {
            problem(diagnostics,"PRESSURE_REGION_TOPOLOGY_INVALID",&refs,"region must be one acyclic chain with the declared terminals as its only degree-one nodes");
            return None;
        }
        if !terminal
            && model.pipe_segments.iter().any(|pipe| {
                !member_set.contains(pipe.id.as_str())
                    && (node_map.get(pipe.from.as_str()) == Some(&node)
                        || node_map.get(pipe.to.as_str()) == Some(&node))
            })
        {
            problem(diagnostics,"PRESSURE_REGION_INTERNAL_BRANCH",&refs,"a pressure-region internal node cannot connect to an external pipe; pressure membership and closure are never inferred");
            return None;
        }
    }
    if !terminal_nodes
        .iter()
        .all(|node| adjacency.contains_key(node))
    {
        problem(
            diagnostics,
            "PRESSURE_TERMINALS_INVALID",
            &refs,
            "declared terminal is not a region chain endpoint",
        );
        return None;
    }
    let mut traversal = Vec::new();
    let mut seen = HashSet::new();
    let mut current = terminal_nodes[0];
    while current != terminal_nodes[1] {
        let candidates = adjacency
            .get(&current)?
            .iter()
            .filter(|index| !seen.contains(*index))
            .copied()
            .collect::<Vec<_>>();
        if candidates.len() != 1 {
            break;
        }
        let index = candidates[0];
        seen.insert(index);
        let pipe = &built.pipes[index];
        let forward = current == pipe.node_i.index;
        let next = if forward {
            pipe.node_j.index
        } else {
            pipe.node_i.index
        };
        traversal.push(TraversedPipe {
            pipe_index: index,
            start_node: current,
            end_node: next,
            forward,
        });
        current = next;
    }
    if current != terminal_nodes[1] || traversal.len() != members.len() {
        problem(diagnostics,"PRESSURE_REGION_TOPOLOGY_INVALID",&refs,"region is disconnected, cyclic, or does not traverse every member exactly once between its terminals");
        return None;
    }
    // T4-U2: each arc member's end tangents [t_i, t_j] (i -> j).
    let mut arc_tangents = HashMap::new();
    for step in &traversal {
        if let Some(bend) = built
            .curved_bend_elements
            .iter()
            .find(|bend| bend.pipe_index == step.pipe_index)
        {
            let Ok(tangents) = bend.macro_element.end_tangents() else {
                problem(diagnostics, "PRESSURE_REGION_GEOMETRY_INVALID", &refs,
                    "a realized bend's end tangents cannot be formed from its chord, radius and y_reference");
                return None;
            };
            arc_tangents.insert(step.pipe_index, tangents);
        }
    }
    if arc_tangents.is_empty() {
        collinear_run(built, &traversal, terminal_nodes[0], terminal_nodes[1], &refs, diagnostics)?;
        return Some((traversal.into_iter().filter(|step| !joint_spans.contains(&step.pipe_index)).collect(), Vec::new()));
    }
    // Straight runs between arcs: each keeps the collinearity guard (D-E).
    let mut run_start = None;
    for (index, step) in traversal.iter().enumerate() {
        if arc_tangents.contains_key(&step.pipe_index) {
            continue;
        }
        let start = *run_start.get_or_insert(index);
        let run_ends = traversal
            .get(index + 1)
            .map_or(true, |next| arc_tangents.contains_key(&next.pipe_index));
        if run_ends {
            let steps = &traversal[start..=index];
            collinear_run(built, steps, steps[0].start_node, step.end_node, &refs, diagnostics)?;
            run_start = None;
        }
    }
    // The traversal-oriented directions of a member at its start and end.
    let directions = |step: &TraversedPipe| -> Option<[[f64; 3]; 2]> {
        let [t_i, t_j] = match arc_tangents.get(&step.pipe_index) {
            Some(&tangents) => tangents,
            None => {
                let axis = built.pipes[step.pipe_index]
                    .frame_element()
                    .ok()?
                    .orientation()
                    .ok()?
                    .local_axes[0];
                [axis, axis]
            }
        };
        Some(if step.forward {
            [t_i, t_j]
        } else {
            [t_j.map(|value| -value), t_i.map(|value| -value)]
        })
    };
    let mut junctions = Vec::new();
    for before in 0..traversal.len().saturating_sub(1) {
        let after = before + 1;
        let (incoming, outgoing) = (&traversal[before], &traversal[after]);
        if !arc_tangents.contains_key(&incoming.pipe_index)
            && !arc_tangents.contains_key(&outgoing.pipe_index)
        {
            continue;
        }
        let pipes = [
            built.pipes[incoming.pipe_index].element_id.as_str(),
            built.pipes[outgoing.pipe_index].element_id.as_str(),
        ];
        let node_id = model.nodes[incoming.end_node].id.as_str();
        let junction_refs = [&*case.id, id, node_id, pipes[0], pipes[1]];
        let (Some([_, t_in]), Some([t_out, _])) = (directions(incoming), directions(outgoing)) else {
            problem(diagnostics, "PRESSURE_REGION_GEOMETRY_INVALID", &junction_refs,
                "a bend-adjacent member's direction cannot be formed");
            return None;
        };
        let Ok(theta) = super::kink(t_in, t_out) else {
            problem(diagnostics, "PRESSURE_REGION_GEOMETRY_INVALID", &junction_refs,
                "the direction change at a bend-adjacent junction is not defined");
            return None;
        };
        if !(theta <= super::TANGENCY_TOLERANCE_RAD) {
            problem(diagnostics, MITRE_UNSUPPORTED, &junction_refs,
                "the direction change at a realized bend's end exceeds the tangency tolerance alpha_tan = 1e-3 rad (theta = atan2(|t_in x t_out|, t_in . t_out)); a mitre is not admitted under 3.0.0/exact_pressure_v3, so the region is refused; make the bend tangent to its neighbours");
            return None;
        }
        junctions.push(Junction { node: incoming.end_node, before, after, t_in, t_out, theta });
    }
    // No joint is arc-adjacent (classifier). Reindex each arc junction after
    // removing the virtual edges so the remaining member-state list stays exact.
    for junction in &mut junctions {
        junction.before -= traversal[..junction.before].iter().filter(|step| joint_spans.contains(&step.pipe_index)).count();
        junction.after -= traversal[..junction.after].iter().filter(|step| joint_spans.contains(&step.pipe_index)).count();
    }
    Some((traversal.into_iter().filter(|step| !joint_spans.contains(&step.pipe_index)).collect(), junctions))
}

/// One collinear chain of straight members between two nodes: today's
/// representation guard (64ε of the span), unchanged; the whole region when
/// it has no realized arc.
fn collinear_run(
    built: &BuiltModel,
    steps: &[TraversedPipe],
    origin_node: usize,
    end_node: usize,
    refs: &[&str],
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<()> {
    let origin = built.nodes.get(origin_node)?.coordinates;
    let end = built.nodes.get(end_node)?.coordinates;
    let chord = std::array::from_fn::<_, 3, _>(|axis| end[axis] - origin[axis]);
    let length = norm3(chord[0], chord[1], chord[2]);
    if !length.is_finite() || length <= 0.0 {
        problem(
            diagnostics,
            "PRESSURE_REGION_GEOMETRY_INVALID",
            refs,
            "region terminal span must be finite and nonzero",
        );
        return None;
    }
    let direction = chord.map(|value| value / length);
    let mut previous = 0.0;
    for step in steps {
        let point = built.nodes.get(step.end_node)?.coordinates;
        let delta = std::array::from_fn::<_, 3, _>(|axis| (point[axis] - origin[axis]) / length);
        let projection = delta.iter().zip(direction).map(|(a, b)| a * b).sum::<f64>();
        let normal =
            std::array::from_fn::<_, 3, _>(|axis| delta[axis] - projection * direction[axis]);
        let residual = norm3(normal[0], normal[1], normal[2]);
        if !projection.is_finite()
            || !residual.is_finite()
            || residual > REPRESENTATION_GUARD
            || projection <= previous
            || projection > 1.0 + REPRESENTATION_GUARD
        {
            problem(diagnostics,"PRESSURE_REGION_NONCOLLINEAR",refs,
                format!("region must be a nonoverlapping collinear chain; perpendicular error exceeds 64*epsilon*terminal_span or traversal backtracks (reference_scale={length:e} m, normalized_error={residual:e}); this is a representation guard, not an engineering fit tolerance"));
            return None;
        }
        previous = projection;
    }
    Some(())
}

#[cfg(test)]
#[path = "joint_pressure_tests.rs"]
mod joint_pressure_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn input() -> Value {
        json!({
            "schema_version":"0.7.0", "document_kind":"openpipestress.product_preview.model",
            "pressure_contract":{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"},
            "project":{"id":"project:region-test","units":{"length":"m"}},
            "analysis_status":{"mechanics":"not_run","rule_check":"not_run","professional_acceptance":"not_requested"},
            "nodes":[
                {"id":"node:A","position":{"x":0.0,"y":0.0,"z":0.0}},
                {"id":"node:B","position":{"x":2.0,"y":0.0,"z":0.0}},
                {"id":"node:C","position":{"x":5.0,"y":0.0,"z":0.0}}
            ],
            "pipe_segments":[
                {"id":"pipe:A","from":"node:A","to":"node:B","material":"material:A","y_reference":{"x":0.0,"y":1.0,"z":0.0},
                 "section":{"outside_diameter":{"value":4.0,"unit":"m"},"wall_thickness":{"value":1.0,"unit":"m"},
                    "corrosion_allowance":{"value":0.0,"unit":"m"}}},
                {"id":"pipe:B","from":"node:B","to":"node:C","material":"material:A","y_reference":{"x":0.0,"y":1.0,"z":0.0},
                 "section":{"outside_diameter":{"value":4.0,"unit":"m"},"wall_thickness":{"value":1.0,"unit":"m"},
                    "corrosion_allowance":{"value":0.0,"unit":"m"}}}
            ],
            "materials":[{"id":"material:A","constitutive_basis":"homogeneous_isotropic_E_nu_v1",
                "elastic_modulus":{"value":120.0,"unit":"Pa"},"poisson_ratio":{"value":0.25,"unit":"1"},
                "shear_modulus":{"value":48.0,"unit":"Pa"},"provenance":"independent rational fixture"}],
            "supports":[],"components":[],"combinations":[],
            "load_cases":[{"id":"case:A","primitive_loads":[],"pressure_regions":[{
                "id":"region:A","member_pipe_ids":["pipe:B","pipe:A"],
                "pressure_basis":"internal_differential_zero_external_v1","pressure":{"value":3.0,"unit":"Pa"},
                "terminals":[
                    {"node_ref":"node:A","closure_transfer":"transfers_to_wall","provenance":"explicit left closure"},
                    {"node_ref":"node:C","closure_transfer":"transfers_to_wall","provenance":"explicit right closure"}
                ],"provenance":"independent rational fixture"}]}]
        })
    }

    fn parse(value: Value) -> PreviewModel {
        serde_json::from_value(value).unwrap()
    }

    fn assemble(value: Value) -> (Option<ExactPressureCase>, Vec<Diagnostic>) {
        let model = parse(value);
        let mut diagnostics = Vec::new();
        let built = super::super::build_model(&model, &model.materials, &mut diagnostics).unwrap();
        let output = build_pressure_case(
            &model,
            &built,
            &model.materials,
            &model.load_cases[0],
            &mut diagnostics,
        );
        (output, diagnostics)
    }

    fn expect_rejected(value: Value, code: &str) {
        let (output, diagnostics) = assemble(value);
        assert!(output.is_none(), "unexpected assembled pressure case");
        assert!(
            diagnostics.iter().any(|d| d.code == code),
            "{code} absent: {diagnostics:?}"
        );
    }

    fn assert_close(actual: f64, expected: f64, scale: f64) {
        // Local adapter arithmetic check; no protected solver tolerance changes.
        assert!(actual.is_finite());
        assert!(
            (actual - expected).abs() <= 128.0 * f64::EPSILON * scale,
            "actual={actual:e}; expected={expected:e}; scale={scale:e}"
        );
    }

    #[test]
    fn region_load_ledgers_are_separate_and_orientation_invariant() {
        for mask in 0..4 {
            let mut value = input();
            for index in 0..2 {
                if mask & (1 << index) != 0 {
                    let from = value["pipe_segments"][index]["from"].take();
                    value["pipe_segments"][index]["from"] =
                        value["pipe_segments"][index]["to"].take();
                    value["pipe_segments"][index]["to"] = from;
                }
            }
            let (result, diagnostics) = assemble(value);
            assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
            let result = result.unwrap();
            assert_eq!(result.pipe_states.len(), 2);
            for dof in 0..18 {
                let expected_eigen = match dof {
                    0 => 1.5 * PI,
                    12 => -1.5 * PI,
                    _ => 0.0,
                };
                let expected_cap = match dof {
                    0 => -3.0 * PI,
                    12 => 3.0 * PI,
                    _ => 0.0,
                };
                assert_close(result.eigenloads[dof], expected_eigen, 3.0 * PI);
                assert_close(result.cap_loads[dof], expected_cap, 3.0 * PI);
            }
            assert_eq!(
                result.evidence[0]["member_pipe_ids"],
                json!(["pipe:A", "pipe:B"])
            );
            assert_eq!(
                result.evidence[0]["approximation"],
                "long_straight_annulus_small_strain_v2"
            );
        }
    }

    #[test]
    fn closure_ledger_excludes_remote_supports_and_does_not_invent_second_cap() {
        for separate in [1usize, 2, 3] {
            let mut value = input();
            for terminal in 0..2 {
                if separate & (1 << terminal) != 0 {
                    value["load_cases"][0]["pressure_regions"][0]["terminals"][terminal]
                        ["closure_transfer"] = json!("separately_supported_or_compensated");
                }
            }
            let (result, diagnostics) = assemble(value);
            assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
            let result = result.unwrap();
            for (terminal, node, pressure_force) in [(0, 0, -3.0 * PI), (1, 2, 3.0 * PI)] {
                let remote = separate & (1 << terminal) != 0;
                assert_close(
                    result.cap_loads[node * 6],
                    if remote { 0.0 } else { pressure_force },
                    3.0 * PI,
                );
                let evidence = &result.evidence[0]["terminals"][terminal];
                assert_eq!(evidence["remote_closure_excluded_from_pipe_solve"], remote);
                if remote {
                    assert_close(
                        evidence["remote_closure_support_reaction_global_n"][0]
                            .as_f64()
                            .unwrap(),
                        -pressure_force,
                        3.0 * PI,
                    );
                }
            }
            assert_close(result.eigenloads[0], 1.5 * PI, 3.0 * PI);
        }
    }

    #[test]
    fn unequal_wall_and_material_preserve_bore_caps_and_local_poisson_loads() {
        let mut value = input();
        value["materials"].as_array_mut().unwrap().push(
            json!({"id":"material:B","constitutive_basis":MATERIAL_BASIS,
            "elastic_modulus":{"value":240.0,"unit":"Pa"},"poisson_ratio":{"value":0.1,"unit":"1"},
            "shear_modulus":{"value":240.0/2.2,"unit":"Pa"}}),
        );
        value["pipe_segments"][1]["material"] = json!("material:B");
        value["pipe_segments"][1]["section"]["outside_diameter"]["value"] = json!(6.0);
        value["pipe_segments"][1]["section"]["wall_thickness"]["value"] = json!(2.0);
        let (result, diagnostics) = assemble(value);
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        let result = result.unwrap();
        assert_close(result.cap_loads[0], -3.0 * PI, 3.0 * PI);
        assert_close(result.cap_loads[12], 3.0 * PI, 3.0 * PI);
        assert_close(result.eigenloads[6], -0.9 * PI, 3.0 * PI);
        assert_close(
            result.pipe_states[&1].sections.nominal().wall_area_m2(),
            8.0 * PI,
            8.0 * PI,
        );
        assert_eq!(result.evidence[0]["materials"][1]["E_pa"], 240.0);
    }

    #[test]
    fn zero_poisson_limit_does_not_include_thermal_load() {
        // T4-U0: the input's p = +3 replaces the former p = -3, which v2
        // refused by name (B7b deleted v2; see
        // `negative_pressure_assembles_under_the_current_contract`);
        // the cap load at node A is -p*Ai = -3*pi with Ai = pi.
        let mut value = input();
        value["materials"][0]["poisson_ratio"]["value"] = json!(0.0);
        value["materials"][0]["shear_modulus"]["value"] = json!(60.0);
        value["materials"][0]["thermal_expansion_coefficient"] =
            json!({"value":0.00001,"unit":"1/K"});
        value["load_cases"][0]["primitive_loads"] = json!([{"id":"thermal:A","category":"thermal","dimension":"temperature_interval",
            "target":{"type":"element","pipe":"pipe:A"},"direction":"global_x","magnitude":{"value":20.0,"unit":"K"}}]);
        let (result, diagnostics) = assemble(value);
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        let result = result.unwrap();
        assert!(result.eigenloads.iter().all(|value| *value == 0.0));
        assert_close(result.cap_loads[0], -3.0 * PI, 3.0 * PI);
    }

    /// T4-U0 A2.6 under the current contract: B7b deleted v2's named refusal of
    /// p < 0; `3.0.0/exact_pressure_v3` assembles a negative differential (its
    /// external-pressure stability is not assessed, as its formulation states)
    /// and both signed zeros, with no negative-pressure finding.
    #[test]
    fn negative_pressure_assembles_under_the_current_contract() {
        for pressure in [-3.0, -5e-324, -0.0, 0.0] {
            let mut value = input();
            value["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] = json!(pressure);
            let (result, diagnostics) = assemble(value);
            assert!(
                !diagnostics.iter().any(|d| d.code == "PRESSURE_REGION_PRESSURE_NEGATIVE"),
                "{pressure:e}: {diagnostics:?}"
            );
            assert!(!has_blocking(&diagnostics), "{pressure:e}: {diagnostics:?}");
            let result = result.expect("the current contract assembles a signed pressure");
            if pressure == 0.0 {
                assert!(result.cap_loads.iter().all(|value| *value == 0.0));
            }
        }
    }

    fn pipe_state(region_id: &str) -> ExactPressurePipeState {
        ExactPressurePipeState {
            region_id: region_id.to_string(),
            sections: ExactSectionBases::derive(4.0, 1.0, 0.0, 0.0).unwrap(),
            pressure: InternalDifferentialPressure::new(3.0).unwrap(),
            material: IsotropicENu::new(120.0, 0.25).unwrap(),
            eigenload_pair: [0.0, 0.0],
            arc: None,
        }
    }

    /// T4-U0 A3 (Option 1): pins the recovery decision only. Its call site
    /// (`lib.rs`'s `append_exact_pressure_results`, reached only for a member
    /// of an exact pressure region) is unreachable through the public entry
    /// until T4-U2a lifts the component refusal; the end-to-end wiring test (no
    /// panic, named refusal, no maximum row) lands with T4-U2a.
    #[test]
    fn region_member_without_straight_recovery_is_refused_by_name() {
        let state = pipe_state("region:A");
        let mechanical = [0.0; 12];
        // A curved (realized) bend in a region: no straight mechanical recovery.
        let mut diagnostics = Vec::new();
        assert!(
            exact_member_recovery(&mut diagnostics, "case:A", "pipe:bend", &state, None).is_none(),
            "a region member without straight recovery is not recovered"
        );
        assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
        let refusal = &diagnostics[0];
        assert_eq!(refusal.code, "EXACT_PRESSURE_REGION_MEMBER_NOT_STRAIGHT");
        assert_eq!(refusal.severity, "blocking");
        assert_eq!(refusal.affected_refs, ["case:A", "region:A", "pipe:bend"]);
        assert_eq!(
            refusal.id,
            "diagnostic:pressure-runtime:case-A-region-A-pipe-bend:EXACT_PRESSURE_REGION_MEMBER_NOT_STRAIGHT"
        );
        assert_eq!(
            refusal.source.as_deref(),
            Some("core/product_physics/src/pressure_runtime.rs")
        );
        assert!(refusal.message.ends_with("it is refused, not recovered on its chord"));
        // A straight region member recovers from its own mechanical actions,
        // with no finding.
        let mut diagnostics = Vec::new();
        let actions = exact_member_recovery(
            &mut diagnostics,
            "case:A",
            "pipe:A",
            &state,
            Some(&mechanical),
        )
        .expect("a straight region member recovers");
        assert!(std::ptr::eq(actions, &mechanical[..]));
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
    }

    #[test]
    fn profile_dispatch_requires_explicit_regions_and_refuses_legacy_pressure() {
        let mut missing = input();
        missing["load_cases"][0]
            .as_object_mut()
            .unwrap()
            .remove("pressure_regions");
        expect_rejected(missing, "EXACT_PRESSURE_REGIONS_REQUIRED");
        let mut empty = input();
        empty["load_cases"][0]["pressure_regions"] = json!([]);
        assert!(assemble(empty).0.unwrap().pipe_states.is_empty());
        // Regions without the contract (B7b: the current document's pressure-free
        // form) are a namespace mismatch, never ignored.
        let mut old = input();
        old.as_object_mut().unwrap().remove("pressure_contract");
        for pipe in old["pipe_segments"].as_array_mut().unwrap() {
            pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
        }
        expect_rejected(old, "PREVIEW_CONTRACT_VERSION_MISMATCH");
        let mut old = input();
        old.as_object_mut().unwrap().remove("pressure_contract");
        for pipe in old["pipe_segments"].as_array_mut().unwrap() {
            pipe["section"].as_object_mut().unwrap().remove("corrosion_allowance");
        }
        old["load_cases"][0]
            .as_object_mut()
            .unwrap()
            .remove("pressure_regions");
        old["load_cases"][0]["primitive_loads"] = json!([{"id":"pressure:A","category":"pressure","dimension":"pressure",
            "target":{"type":"element","pipe":"pipe:A"},"direction":"global_x","magnitude":{"value":3.0,"unit":"Pa"}}]);
        let mut diagnostics = Vec::new();
        validate_profile(&parse(old.clone()), &mut diagnostics);
        assert!(diagnostics
            .iter()
            .any(|d| d.code == "PRESSURE_MODEL_REAUTHOR_REQUIRED"));
        // Retired product-wide (U3, D-2 A1): a zero-valued pressure primitive is
        // refused too, by the same code, with the re-author text.
        old["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(0.0);
        diagnostics.clear();
        validate_profile(&parse(old.clone()), &mut diagnostics);
        assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
        assert_eq!(diagnostics[0].code, "PRESSURE_MODEL_REAUTHOR_REQUIRED");
        // B7b A5: the re-author text names the current contract.
        assert!(diagnostics[0].message.contains("3.0.0/exact_pressure_v3"), "{}", diagnostics[0].message);
        // Without the primitive, the 0.7.0 document is pressure-free and admitted.
        old["load_cases"][0]["primitive_loads"] = json!([]);
        diagnostics.clear();
        validate_profile(&parse(old.clone()), &mut diagnostics);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        // B7a: the retired 0.1.0 and 0.2.0 are refused by name with the re-author text,
        // with or without a contract; nothing is migrated or reinterpreted.
        for retired in crate::RETIRED_MODEL_SCHEMA_VERSIONS {
            let mut value = old.clone();
            value["schema_version"] = json!(retired);
            diagnostics.clear();
            validate_profile(&parse(value), &mut diagnostics);
            assert_eq!(diagnostics.iter().map(|d| d.code.as_str()).collect::<Vec<_>>(), [crate::MODEL_SCHEMA_VERSION_RETIRED], "{retired}: {diagnostics:?}");
            assert_eq!(diagnostics[0].affected_refs, ["schema_version"]);
            assert!(diagnostics[0].message.contains(retired) && diagnostics[0].message.contains("re-author"), "{}", diagnostics[0].message);
        }
        // An unknown contract stays unsupported on the current document; the two
        // retired labels (1.0.0/legacy_pressure_v1 and, since B7b,
        // 2.0.0/exact_straight_pressure_v2) take the re-author code below.
        let mut unknown = input();
        unknown["pressure_contract"] = json!({"version":"1.0.1","mode":"legacy_pressure_v1"});
        expect_rejected(unknown, "PRESSURE_CONTRACT_UNSUPPORTED");
        // B7b deleted RV127 N-7's 0.3.0-era namespace pins (their documents are
        // retired by version, above); the retired labels' refusal is checked below.
    }

    /// U3 (D-2 A1): the retired 1.0.0/legacy_pressure_v1 label is refused with
    /// the re-author code; B7b: so is the retired 2.0.0/exact_straight_pressure_v2
    /// label. Either can only appear on the current 0.7.0 document now.
    #[test]
    fn retired_legacy_pressure_label_takes_the_reauthor_code() {
        for label in [
            json!({"version":"1.0.0","mode":"legacy_pressure_v1"}),
            json!({"version":"2.0.0","mode":"exact_straight_pressure_v2"}),
        ] {
            let mut labelled = input();
            labelled["pressure_contract"] = label.clone();
            labelled["load_cases"][0].as_object_mut().unwrap().remove("pressure_regions");
            let mut diagnostics = Vec::new();
            validate_profile(&parse(labelled), &mut diagnostics);
            assert_eq!(
                diagnostics.iter().map(|d| d.code.as_str()).collect::<Vec<_>>(),
                ["PRESSURE_MODEL_REAUTHOR_REQUIRED"],
                "{label}: {diagnostics:?}"
            );
            assert_eq!(diagnostics[0].affected_refs, ["pressure_contract"]);
            assert!(diagnostics[0].message.contains("retired") && diagnostics[0].message.contains("3.0.0/exact_pressure_v3"));
        }
    }

    #[test]
    fn malformed_pressure_namespaces_and_competing_selectors_never_fall_back() {
        let mut value = input();
        value["pressure_contract"]["unexpected"] = json!(true);
        assert!(serde_json::from_value::<PreviewModel>(value).is_err());
        let mut value = input();
        value["load_cases"][0]["pressure_regions"][0]["terminals"][0]["guessed_closure"] =
            json!(true);
        assert!(serde_json::from_value::<PreviewModel>(value).is_err());
        for (category, dimension) in [("pressure", "force"), ("occasional", "pressure")] {
            let mut value = input();
            value["load_cases"][0]["primitive_loads"] = json!([{"id":"pressure:A","category":category,"dimension":dimension,
                "target":{"type":"element","pipe":"pipe:A"},"direction":"global_x","magnitude":{"value":0.0,"unit":"Pa"}}]);
            expect_rejected(value, "EXACT_PRESSURE_REQUIRES_REGION");
        }
    }

    #[test]
    fn explicit_terminals_members_and_pressure_basis_are_required() {
        let mutations = [
            (
                "/load_cases/0/pressure_regions/0/terminals/0/closure_transfer",
                json!("invented_default"),
                "PRESSURE_TERMINAL_CLOSURE_INVALID",
            ),
            (
                "/load_cases/0/pressure_regions/0/terminals",
                json!([]),
                "PRESSURE_TERMINALS_INVALID",
            ),
            (
                "/load_cases/0/pressure_regions/0/terminals/0/node_ref",
                json!("node:unknown"),
                "PRESSURE_TERMINAL_NODE_UNKNOWN",
            ),
            (
                "/load_cases/0/pressure_regions/0/terminals/1/node_ref",
                json!("node:A"),
                "PRESSURE_TERMINALS_INVALID",
            ),
            (
                "/load_cases/0/pressure_regions/0/member_pipe_ids",
                json!(["pipe:missing"]),
                "PRESSURE_REGION_MEMBER_UNKNOWN",
            ),
            (
                "/load_cases/0/pressure_regions/0/member_pipe_ids",
                json!(["pipe:A", "pipe:A"]),
                "PRESSURE_REGION_MEMBERS_INVALID",
            ),
            (
                "/load_cases/0/pressure_regions/0/pressure_basis",
                json!("external_pressure"),
                "PRESSURE_REGION_BASIS_UNSUPPORTED",
            ),
        ];
        for (pointer, replacement, code) in mutations {
            let mut value = input();
            *value.pointer_mut(pointer).unwrap() = replacement;
            expect_rejected(value, code);
        }
    }

    #[test]
    fn chain_geometry_and_region_ownership_reject_ambiguous_composition() {
        let mut value = input();
        value["nodes"][1]["position"]["y"] = json!(0.1);
        expect_rejected(value, "PRESSURE_REGION_NONCOLLINEAR");
        let mut value = input();
        value["nodes"][1]["position"]["x"] = json!(6.0);
        expect_rejected(value, "PRESSURE_REGION_NONCOLLINEAR");
        let mut value = input();
        value["pipe_segments"][1]["section"]["wall_thickness"]["value"] = json!(0.5);
        expect_rejected(value, "PRESSURE_REGION_BORE_MISMATCH");
        let mut value = input();
        let mut duplicate = value["load_cases"][0]["pressure_regions"][0].clone();
        duplicate["id"] = json!("region:B");
        value["load_cases"][0]["pressure_regions"]
            .as_array_mut()
            .unwrap()
            .push(duplicate);
        expect_rejected(value, "PRESSURE_REGION_PIPE_OVERLAP");
        let mut value = input();
        value["nodes"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"node:D","position":{"x":2.0,"y":0.0,"z":2.0}}));
        let mut branch = value["pipe_segments"][0].clone();
        branch["id"] = json!("pipe:D");
        branch["from"] = json!("node:B");
        branch["to"] = json!("node:D");
        value["pipe_segments"].as_array_mut().unwrap().push(branch);
        expect_rejected(value, "PRESSURE_REGION_INTERNAL_BRANCH");
    }

    #[test]
    fn normalized_unit_path_and_nonrepresentable_output_are_explicit() {
        let mut value = input();
        value["load_cases"][0]["pressure_regions"][0]["pressure"] =
            json!({"value":3.0,"unit":"MPa"});
        let mut model = parse(value);
        let mut diagnostics = Vec::new();
        normalize_region_units(&mut model, &mut diagnostics);
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        let pressure = model.load_cases[0].pressure_regions.as_ref().unwrap()[0]
            .pressure
            .as_ref()
            .unwrap();
        assert_eq!(pressure.unit, "Pa");
        assert_eq!(pressure.value, 3e6);
        let mut value = input();
        value["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] = json!(f64::MAX);
        expect_rejected(value, "EXACT_PRESSURE_OUTPUT_UNREPRESENTABLE");
    }

    #[test]
    fn region_identity_and_pipe_ownership_are_scoped_to_the_case() {
        let mut value = input();
        let mut second = value["load_cases"][0].clone();
        second["id"] = json!("case:B");
        second["pressure_regions"][0]["id"] = json!("region:B");
        value["load_cases"].as_array_mut().unwrap().push(second);
        let mut diagnostics = Vec::new();
        validate_profile(&parse(value.clone()), &mut diagnostics);
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        value["load_cases"][1]["pressure_regions"][0]["id"] = json!("region:A");
        validate_profile(&parse(value.clone()), &mut diagnostics);
        assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
        let duplicate = value["load_cases"][0]["pressure_regions"][0].clone();
        value["load_cases"][0]["pressure_regions"]
            .as_array_mut()
            .unwrap()
            .push(duplicate);
        validate_profile(&parse(value), &mut diagnostics);
        assert!(diagnostics
            .iter()
            .any(|d| d.code == "PRESSURE_REGION_ID_DUPLICATE"));
    }

    #[test]
    fn generic_node_and_support_row_suffix_collisions_block_exact_publication() {
        for kind in ["node", "support"] {
            let mut value = input();
            if kind == "node" {
                let mut node = value["nodes"][0].clone();
                node["id"] = json!("node-A");
                value["nodes"].as_array_mut().unwrap().push(node);
            } else {
                value["supports"] = json!([
                    {"id":"support:A","node":"node:A","restraints":["UX"]},
                    {"id":"support-A","node":"node:C","restraints":["UX"]}
                ]);
            }
            let mut diagnostics = Vec::new();
            validate_profile(&parse(value), &mut diagnostics);
            assert!(
                diagnostics.iter().any(|finding| {
                    finding.code == "EXACT_PRESSURE_RESULT_ID_COLLISION"
                        && finding
                            .affected_refs
                            .iter()
                            .any(|reference| reference == kind)
                }),
                "{diagnostics:?}"
            );
        }
    }
}

#[cfg(test)]
mod grouping_identity_tests {
    use super::*;
    use crate::pressure_exact::SourceAnnulus;
    #[test]
    fn source_bore_identity_does_not_round_distinct_sources_to_match() {
        let a = SourceAnnulus::from_od_wall(5.0 / 32.0, 1.0 / 64.0).unwrap();
        let b = SourceAnnulus::from_od_wall(3.0 / 16.0, 1.0 / 32.0).unwrap();
        assert_eq!(a.source_bore_key(), b.source_bore_key());
        let wall = 1e-12_f64;
        let c = SourceAnnulus::from_od_wall(0.12, wall).unwrap();
        let d = SourceAnnulus::from_od_wall(0.12, f64::from_bits(wall.to_bits() + 1)).unwrap();
        assert_eq!(c.inner_radius_m(), d.inner_radius_m());
        assert_ne!(c.source_bore_key(), d.source_bore_key());
    }
    #[test]
    fn either_source_known_zero_direction_sign_contributes_no_force_group() {
        let sections = ExactSectionBases::derive(0.12, 0.01, 0.0, 0.0).unwrap();
        let material = IsotropicENu::new(200e9, 0.3).unwrap();
        let pressure = InternalDifferentialPressure::new(2e6).unwrap();
        let state = ExactPressurePipeState {
            region_id: "r".to_string(),
            sections,
            material,
            pressure,
            eigenload_pair: sections.poisson_eigenload_pair(material, pressure).unwrap(),
            arc: None,
        };
        let mut groups = BTreeMap::new();
        for zero in [-0.0, 0.0] {
            add_factor_term(&mut groups, 0, &state, zero, true, 1.0, "r", "p", "terminal_cap");
        }
        assert!(groups.is_empty());
    }
    /// K-D5 "One ε": a formed direction component whose binary64 fold is
    /// zero is kept, both signs in the one zero-magnitude group with their
    /// signed coefficients, and its group value is zero.
    #[test]
    fn a_formed_direction_folded_to_zero_keeps_its_force_group() {
        let sections = ExactSectionBases::derive(0.12, 0.01, 0.0, 0.0).unwrap();
        let annulus = sections.reduced();
        let material = IsotropicENu::new(200e9, 0.3).unwrap();
        let pressure = InternalDifferentialPressure::new(2e6).unwrap();
        let state = ExactPressurePipeState {
            region_id: "r".to_string(),
            sections,
            material,
            pressure,
            eigenload_pair: [0.0, 0.0],
            arc: None,
        };
        let mut groups = BTreeMap::new();
        for zero in [-0.0, 0.0] {
            add_factor_term(&mut groups, 0, &state, zero, false, 1.0, "r", "p", "bend_cap_removed");
        }
        assert_eq!(groups.len(), 1);
        let group = groups.values().next().unwrap();
        assert_eq!(group.direction.to_bits(), 0.0_f64.to_bits());
        assert_eq!(group.terms.iter().map(|term| term.0).collect::<Vec<_>>(), [-1.0, 1.0]);
        assert_eq!(annulus.pressure_group_value(pressure, 0.0, group.direction).unwrap(), 0.0);
    }
}
