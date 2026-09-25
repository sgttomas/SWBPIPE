//! Explicit straight pressure regions, normalized evidence and load ownership.
//!
//! Only the pressure-Poisson eigenload enters `eigenloads`; thermal and
//! mechanical loads remain owned by the caller. Terminal cap transfer is a
//! distinct applied-load ledger and must never be subtracted in wall recovery.

use super::pressure_exact::{InternalDifferentialPressure, IsotropicENu, SourceAnnulus};
use super::{
    diag, has_blocking, is_constant_effort_support, is_temperature_change_dimension,
    normalize_quantity, stable_suffix, BuiltModel, Diagnostic, LoadTargetInput, MaterialInput,
    PreviewLoadCase, PreviewModel, Quantity, DOF_PER_NODE,
};
use open_pipe_stress_units::Dimension;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

const EXACT_MODE: &str = "exact_straight_pressure_v2";
const EXACT_VERSION: &str = "2.0.0";
const PRESSURE_BASIS: &str = "internal_differential_zero_external_v1";
const MATERIAL_BASIS: &str = "homogeneous_isotropic_E_nu_v1";
/// Representation agreement only, not an engineering geometry tolerance.
const REPRESENTATION_GUARD: f64 = 64.0 * f64::EPSILON;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PressureContractInput {
    pub version: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PressureRegionInput {
    pub id: Option<String>,
    pub member_pipe_ids: Option<Vec<String>>,
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
    pub assembly_evidence: Value,
    pub evidence: Vec<Value>,
}

#[derive(Debug, Clone)]
pub(crate) struct ExactPressurePipeState {
    pub region_id: String,
    pub annulus: SourceAnnulus,
    pub pressure: InternalDifferentialPressure,
    pub material: IsotropicENu,
    /// Pressure-only applied RHS pair in this pipe's authored local i/j frame.
    pub eigenload_pair: [f64; 2],
}

pub(crate) fn is_exact(model: &PreviewModel) -> bool {
    model.schema_version == "0.3.0"
        && model.pressure_contract.as_ref().is_some_and(|contract| {
            contract.version.as_deref() == Some(EXACT_VERSION)
                && contract.mode.as_deref() == Some(EXACT_MODE)
        })
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

fn present(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|text| !text.trim().is_empty())
}

/// Validate the explicit profile and input namespace before unit conversion.
/// Region geometry is checked later against normalized, actually built pipes.
pub(crate) fn validate_profile(model: &PreviewModel, diagnostics: &mut Vec<Diagnostic>) {
    match model.schema_version.as_str() {
        "0.1.0" | "0.2.0" => {
            if model.pressure_contract.is_some() {
                problem(diagnostics, "PREVIEW_CONTRACT_VERSION_MISMATCH", &["pressure_contract"],
                    "pressure contract namespaces require model document 0.3.0; old inputs are not reinterpreted");
            }
        }
        "0.3.0" => match &model.pressure_contract {
            None => problem(
                diagnostics,
                "PRESSURE_CONTRACT_REQUIRED",
                &["pressure_contract"],
                "model document 0.3.0 requires an explicit pressure contract version and mode",
            ),
            Some(contract) => {
                if !matches!(
                    (contract.version.as_deref(), contract.mode.as_deref()),
                    (Some("1.0.0"), Some("legacy_pressure_v1"))
                        | (Some(EXACT_VERSION), Some(EXACT_MODE))
                ) {
                    problem(diagnostics, "PRESSURE_CONTRACT_UNSUPPORTED", &["pressure_contract"],
                        "supported pressure contracts are 1.0.0/legacy_pressure_v1 and 2.0.0/exact_straight_pressure_v2; incomplete or unknown contracts never fall back");
                }
            }
        },
        _ => problem(
            diagnostics,
            "PREVIEW_SCHEMA_VERSION_UNSUPPORTED",
            &["schema_version"],
            "direct mechanics solves support exactly model versions 0.1.0, 0.2.0 and 0.3.0",
        ),
    }

    for component in &model.components {
        if let Some(contract) = &component.objective_connector {
            let code = if model.schema_version != "0.3.0" {
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
    let exact = is_exact(model);
    if exact {
        // Metadata-only fitting records are also excluded: accepting one as a
        // straight span would misrepresent the explicit first composition.
        for component in &model.components {
            problem(diagnostics, "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED", &[&component.id],
                "the first exact pressure profile supports straight circular pipes only; fitting/component records require their own integrated mechanics proof");
        }
        for support in &model.supports {
            if support.nonlinear.is_some() || is_constant_effort_support(support) {
                problem(diagnostics, "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED", &[&support.id],
                    "the first exact pressure profile supports linear restraints and springs, not nonlinear or constant-effort support composition");
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
            problem(diagnostics, "EXACT_PRESSURE_COMBINATION_UNSUPPORTED", &[&combination.id],
                "exact pressure combinations require signed physical-state combination and pressure-region evidence; scalar row algebra is not a valid fallback");
        }
    }

    for case in &model.load_cases {
        if !exact {
            if case.pressure_regions.is_some() {
                problem(diagnostics, "PREVIEW_CONTRACT_VERSION_MISMATCH", &[&case.id, "pressure_regions"],
                    "pressure_regions requires the explicit exact pressure profile; legacy cases must omit the namespace");
            }
            for load in &case.primitive_loads {
                if (load.category == "pressure" || load.dimension == "pressure")
                    && load.magnitude.value != 0.0
                {
                    // Only named in-crate historical tests can enter this scope; normal builds have no selector.
                    #[cfg(test)]
                    if crate::historical_pressure_reference::active() {
                        continue;
                    }
                    problem(diagnostics, "PRESSURE_MODEL_REAUTHOR_REQUIRED", &[&case.id, &load.id],
                        "a fresh solve cannot publish the legacy nonzero pressure model; explicitly author exact pressure regions, closure paths and E/nu material inputs");
                }
            }
            continue;
        }
        if case.equivalent_static.is_some() {
            problem(
                diagnostics,
                "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED",
                &[&case.id, "equivalent_static"],
                "equivalent-static generation is outside the first exact pressure composition",
            );
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
    validate_profile(model, diagnostics);
    if has_blocking(diagnostics) || !is_exact(model) {
        return None;
    }
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
        assembly_evidence: Value::Null,
        evidence: Vec::new(),
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
        let Some(traversal) = traverse_region(model, built, case, region, diagnostics) else {
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
            let Some(&annulus) = built.exact_sections.get(&pipe.element_id) else {
                problem(
                    diagnostics,
                    "PRESSURE_REGION_SECTION_MISSING",
                    &[&case.id, id, &pipe.element_id],
                    "built exact source-annulus geometry is missing",
                );
                continue;
            };
            if let Some(reference) = first_bore {
                let scale = reference.abs().max(annulus.inner_radius_m().abs());
                if (reference - annulus.inner_radius_m()).abs() > REPRESENTATION_GUARD * scale {
                    problem(diagnostics, "PRESSURE_REGION_BORE_MISMATCH", &[&case.id, id, &pipe.element_id],
                        format!("region requires equal normalized bore; radius difference exceeds 64*epsilon*radius_scale (scale={scale:e} m); this is a representation guard, not a physical reducer allowance"));
                    continue;
                }
            } else {
                first_bore = Some(annulus.inner_radius_m());
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
            let material = match IsotropicENu::new(material_input.elastic_modulus.value, nu.value) {
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
            };
            let thermal_consumed = case.primitive_loads.iter().any(|load| {
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
            let loads = annulus
                .eigenload_pair(material, pressure, 0.0)
                .and_then(|eigen| annulus.cap_pair(pressure).map(|caps| (eigen, caps)))
                .and_then(|pairs| {
                    annulus.surface_stresses(pressure)?;
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
            let mut member_geometry = super::exact_section_evidence(&pipe.element_id, annulus);
            member_geometry["traversal_forward"] = json!(step.forward);
            geometry.push(member_geometry);
            material_evidence.push(json!({"pipe_id":pipe.element_id,"material_id":material_input.id,
                "E_pa":material.elastic_modulus_pa(),"nu":material.poisson_ratio(),"G_pa":material.shear_modulus_pa(),
                "constitutive_basis":MATERIAL_BASIS,"temperature_basis":temperature_basis(case),"provenance":material_input.provenance,
                "thermal_consumed":thermal_consumed,"alpha_per_kelvin":alpha.map(|quantity|quantity.value)}));
            load_evidence.push(json!({"pipe_id":pipe.element_id,"eigenload_pair_local_n":eigen,
                "mathematical_cap_pair_local_n":caps,"local_x_global":direction,"thermal_included":false}));
            region_states.push((
                step,
                ExactPressurePipeState {
                    region_id: id.to_string(),
                    annulus,
                    pressure,
                    material,
                    eigenload_pair: eigen,
                },
                caps,
                direction,
            ));
        }
        if region_states.len() != traversal.len() || has_blocking(diagnostics) {
            continue;
        }
        let terminals = region.terminals.as_ref()?;
        let mut terminal_evidence = Vec::new();
        for (index, terminal) in terminals.iter().enumerate() {
            let (step, _, caps, direction) = if index == 0 {
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
            let cap_global = direction.map(|value| value * caps[local_slot]);
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
        for (step, state, _, direction) in region_states {
            let pipe = &built.pipes[step.pipe_index];
            for (end, node) in [pipe.node_i.index, pipe.node_j.index]
                .into_iter()
                .enumerate()
            {
                for axis in 0..3 {
                    eigen_terms[node * DOF_PER_NODE + axis]
                        .push(direction[axis] * state.eigenload_pair[end]);
                    let sign = if end == 0 { 1.0 } else { -1.0 };
                    add_factor_term(
                        &mut factor_groups,
                        node * DOF_PER_NODE + axis,
                        &state,
                        direction[axis],
                        sign * 2.0 * state.material.poisson_ratio(),
                        id,
                        &pipe.element_id,
                        "poisson_eigen",
                    );
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
        output.evidence.push(json!({"region_id":id,"load_case_id":case.id,"profile_version":EXACT_VERSION,
            "profile_mode":EXACT_MODE,"member_pipe_ids":traversal.iter().map(|step| built.pipes[step.pipe_index].element_id.as_str()).collect::<Vec<_>>(),
            "terminals":terminal_evidence,"pressure_basis":PRESSURE_BASIS,"p_pa":pressure.pressure_pa(),"geometry":geometry,
            "materials":material_evidence,"applied_loads":load_evidence,"provenance":region.provenance,
            "approximation":"long_straight_annulus_small_strain_v2","external_pressure_increment_pa":0.0,
            "geometry_representation_guard":{"epsilon_multiplier":64,"meaning":"arithmetic_representation_only"}}));
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
    finish_source_groups(model, case, &mut output, factor_groups, diagnostics);
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

type FactorKey = (usize, u64, u64, u64, u64);
struct FactorGroup {
    geometry: SourceAnnulus,
    pressure: InternalDifferentialPressure,
    direction: f64,
    terms: Vec<(f64, String, String, &'static str)>,
}
#[allow(clippy::too_many_arguments)]
fn add_factor_term(
    groups: &mut BTreeMap<FactorKey, FactorGroup>,
    dof: usize,
    state: &ExactPressurePipeState,
    direction: f64,
    coefficient: f64,
    region: &str,
    pipe: &str,
    kind: &'static str,
) {
    if direction == 0.0 {
        return;
    } // Only a derived zero direction is omitted; source values remain in their ledgers.
    let (hi, lo) = state.annulus.source_bore_key();
    let magnitude = direction.abs();
    let key = (
        dof,
        state.pressure.pressure_pa().to_bits(),
        hi,
        lo,
        magnitude.to_bits(),
    );
    groups
        .entry(key)
        .or_insert_with(|| FactorGroup {
            geometry: state.annulus,
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
fn finish_source_groups(
    model: &PreviewModel,
    case: &PreviewLoadCase,
    output: &mut ExactPressureCase,
    groups: BTreeMap<FactorKey, FactorGroup>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut by_dof = vec![Vec::new(); output.assembled_loads.len()];
    let mut evidence = Vec::new();
    for ((dof, pbits, hi, lo, _), group) in groups {
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
        let value =
            match group
                .geometry
                .pressure_group_value(group.pressure, coefficient, group.direction)
            {
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
        evidence.push(json!({"node_ref":model.nodes[dof/DOF_PER_NODE].id,"component":(["Fx","Fy","Fz"][dof%DOF_PER_NODE]),
            "pressure_bits":format!("{pbits:016x}"),"source_inner_radius_hi_bits":format!("{hi:016x}"),"source_inner_radius_lo_bits":format!("{lo:016x}"),
            "direction_component_magnitude":group.direction,"coefficient_sum":coefficient,"assembled_force_n":value,
            "terms":group.terms.iter().map(|term|json!({"coefficient":term.0,"region_id":term.1,"pipe_id":term.2,"kind":term.3})).collect::<Vec<_>>()}));
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

fn traverse_region(
    model: &PreviewModel,
    built: &BuiltModel,
    case: &PreviewLoadCase,
    region: &PressureRegionInput,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Vec<TraversedPipe>> {
    let id = region.id.as_deref()?;
    let refs = [&*case.id, id];
    let members = region.member_pipe_ids.as_ref()?;
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
    for member in members {
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
    let origin = built.nodes.get(terminal_nodes[0])?.coordinates;
    let end = built.nodes.get(terminal_nodes[1])?.coordinates;
    let chord = std::array::from_fn::<_, 3, _>(|axis| end[axis] - origin[axis]);
    let length = chord[0].hypot(chord[1]).hypot(chord[2]);
    if !length.is_finite() || length <= 0.0 {
        problem(
            diagnostics,
            "PRESSURE_REGION_GEOMETRY_INVALID",
            &refs,
            "region terminal span must be finite and nonzero",
        );
        return None;
    }
    let direction = chord.map(|value| value / length);
    let mut previous = 0.0;
    for step in &traversal {
        let point = built.nodes.get(step.end_node)?.coordinates;
        let delta = std::array::from_fn::<_, 3, _>(|axis| (point[axis] - origin[axis]) / length);
        let projection = delta.iter().zip(direction).map(|(a, b)| a * b).sum::<f64>();
        let normal =
            std::array::from_fn::<_, 3, _>(|axis| delta[axis] - projection * direction[axis]);
        let residual = normal[0].hypot(normal[1]).hypot(normal[2]);
        if !projection.is_finite()
            || !residual.is_finite()
            || residual > REPRESENTATION_GUARD
            || projection <= previous
            || projection > 1.0 + REPRESENTATION_GUARD
        {
            problem(diagnostics,"PRESSURE_REGION_NONCOLLINEAR",&refs,
                format!("region must be a nonoverlapping collinear chain; perpendicular error exceeds 64*epsilon*terminal_span or traversal backtracks (reference_scale={length:e} m, normalized_error={residual:e}); this is a representation guard, not an engineering fit tolerance"));
            return None;
        }
        previous = projection;
    }
    Some(traversal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn input() -> Value {
        json!({
            "schema_version":"0.3.0", "document_kind":"openpipestress.product_preview.model",
            "pressure_contract":{"version":"2.0.0","mode":"exact_straight_pressure_v2"},
            "project":{"id":"project:region-test","units":{"length":"m"}},
            "analysis_status":{"mechanics":"not_run","rule_check":"not_run","professional_acceptance":"not_requested"},
            "nodes":[
                {"id":"node:A","position":{"x":0.0,"y":0.0,"z":0.0}},
                {"id":"node:B","position":{"x":2.0,"y":0.0,"z":0.0}},
                {"id":"node:C","position":{"x":5.0,"y":0.0,"z":0.0}}
            ],
            "pipe_segments":[
                {"id":"pipe:A","from":"node:A","to":"node:B","material":"material:A","y_reference":{"x":0.0,"y":1.0,"z":0.0},
                 "section":{"outside_diameter":{"value":4.0,"unit":"m"},"wall_thickness":{"value":1.0,"unit":"m"}}},
                {"id":"pipe:B","from":"node:B","to":"node:C","material":"material:A","y_reference":{"x":0.0,"y":1.0,"z":0.0},
                 "section":{"outside_diameter":{"value":4.0,"unit":"m"},"wall_thickness":{"value":1.0,"unit":"m"}}}
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
            result.pipe_states[&1].annulus.wall_area_m2(),
            8.0 * PI,
            8.0 * PI,
        );
        assert_eq!(result.evidence[0]["materials"][1]["E_pa"], 240.0);
    }

    #[test]
    fn signed_pressure_and_zero_poisson_limit_do_not_include_thermal_load() {
        let mut value = input();
        value["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] = json!(-3.0);
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
        assert_close(result.cap_loads[0], 3.0 * PI, 3.0 * PI);
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
        let mut old = input();
        old["schema_version"] = json!("0.2.0");
        expect_rejected(old, "PREVIEW_CONTRACT_VERSION_MISMATCH");
        let mut old = input();
        old["schema_version"] = json!("0.2.0");
        old.as_object_mut().unwrap().remove("pressure_contract");
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
        old["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = json!(0.0);
        diagnostics.clear();
        validate_profile(&parse(old), &mut diagnostics);
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
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
    fn either_derived_zero_direction_sign_contributes_no_force_group() {
        let annulus = SourceAnnulus::from_od_wall(0.12, 0.01).unwrap();
        let material = IsotropicENu::new(200e9, 0.3).unwrap();
        let pressure = InternalDifferentialPressure::new(2e6).unwrap();
        let state = ExactPressurePipeState {
            region_id: "r".to_string(),
            annulus,
            material,
            pressure,
            eigenload_pair: annulus.eigenload_pair(material, pressure, 0.0).unwrap(),
        };
        let mut groups = BTreeMap::new();
        for zero in [-0.0, 0.0] {
            add_factor_term(&mut groups, 0, &state, zero, 1.0, "r", "p", "terminal_cap");
        }
        assert!(groups.is_empty());
    }
}
