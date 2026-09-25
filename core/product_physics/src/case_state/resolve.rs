//! Resolve one authored load case into the single case used by assembly,
//! recovery and evidence. Returns a complete case or blocking diagnostics,
//! never a partially plausible case.
use super::input::*;
use super::material::{self, AnalysisBasisOverride, MaterialSelection, ResolvedMemberMaterial};
use super::temperature::TemperatureIdentity;
use super::thermal::{
    self, CoefficientData, FitInput, NormalizedExpansionLaw, ResolvedStrain, ThermalInput,
    ThermalPoint,
};
use super::{LOAD_REFERENCE_STATE_CONTRACT, LOAD_STATE_PROFILE_ID};
use crate::pressure_exact::IsotropicENu;
use crate::{
    diag, dof_index, is_constant_effort_support, is_variable_spring_hanger, parse_dof,
    stable_suffix, Diagnostic, MaterialInput, PreviewLoadCase, PreviewModel, PreviewSupport,
    Quantity, DOF_PER_NODE,
};
use open_pipe_stress_linear_supports::FrameDof;
use open_pipe_stress_units::{canonical_unit, convert_for_dimension, unit_by_symbol, Dimension};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};

const SOURCE: &str = "core/product_physics/src/case_state/resolve.rs";

fn block(diagnostics: &mut Vec<Diagnostic>, code: &str, refs: &[&str], message: impl Into<String>) {
    let mut finding = diag(
        &format!(
            "diagnostic:load-state:{}:{code}",
            stable_suffix(&refs.join(":"))
        ),
        code,
        "blocking",
        message,
        refs.iter().map(|value| value.to_string()).collect(),
    );
    finding.source = Some(SOURCE.to_string());
    diagnostics.push(finding);
}

fn nonempty(value: &str) -> bool {
    !value.trim().is_empty()
}

/// Normalize one consumed quantity with the existing units catalog only.
fn normalized(quantity: &Quantity, dimension: Dimension) -> Result<f64, String> {
    let from = unit_by_symbol(&quantity.unit, dimension).map_err(|e| e.to_string())?;
    let to = canonical_unit(dimension).ok_or("no canonical unit for dimension")?;
    let value =
        convert_for_dimension(quantity.value, dimension, from, to).map_err(|e| e.to_string())?;
    if !value.is_finite() {
        return Err("consumed quantity must be finite".into());
    }
    if dimension == Dimension::Temperature && value < 0.0 {
        return Err("absolute temperature cannot be below zero kelvin".into());
    }
    Ok(value)
}

fn is_spring(support: &PreviewSupport) -> bool {
    support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support)
}

/// Structural validation before unit normalization. Runs for every document so
/// that an older document cannot silently carry the new namespace.
pub(crate) fn validate_document(
    model: &PreviewModel,
    request_materials_supplied: bool,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // An explicit null is authored presence in every version: an older
    // document carrying a new key at all blocks, and a 0.4.0 document may not
    // author a null where it must either omit the key or supply a value.
    let carries = model.reference_configurations.is_authored()
        || model
            .material_expansion_laws
            .iter()
            .any(Authored::is_authored)
        || !model.request_material_expansion_laws.is_empty()
        || model
            .load_cases
            .iter()
            .any(|case| case.analysis_state.is_authored());
    if !super::is_load_state(model) {
        if carries {
            block(diagnostics, "LOAD_STATE_CONTRACT_VERSION_MISMATCH", &["schema_version"],
                "reference_configurations, material expansion_laws (model or request materials) and case analysis_state require model document 0.4.0; they are never ignored or reinterpreted in an older document, and an explicit null counts as carrying the key");
        }
        return;
    }
    let mut nulls = Vec::new();
    if model.reference_configurations.is_null() {
        nulls.push("reference_configurations".to_string());
    }
    for (index, laws) in model.material_expansion_laws.iter().enumerate() {
        if laws.is_null() {
            nulls.push(format!("materials[{index}].expansion_laws"));
        }
    }
    for case in &model.load_cases {
        if case.analysis_state.is_null() {
            nulls.push(format!("{}.analysis_state", case.id));
        }
    }
    for path in &nulls {
        block(
            diagnostics,
            "LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED",
            &[path.as_str()],
            "an explicit null is authored presence, not absence; omit the key or supply its value",
        );
    }
    if request_materials_supplied {
        block(diagnostics, "LOAD_STATE_REQUEST_MATERIALS_UNSUPPORTED", &["materials"],
            "model document 0.4.0 resolves members from model materials and their expansion laws; a separate request material list is not merged");
    }
    if model.load_cases.is_empty() {
        block(
            diagnostics,
            "LOAD_STATE_CASES_REQUIRED",
            &["load_cases"],
            "model document 0.4.0 requires at least one load case with an explicit analysis_state",
        );
    }
    let pipe_ids = model
        .pipe_segments
        .iter()
        .map(|pipe| pipe.id.as_str())
        .collect::<HashSet<_>>();
    let mut configurations = HashSet::new();
    match model.reference_configurations.value() {
        None => block(
            diagnostics,
            "LOAD_STATE_REFERENCE_CONFIGURATION_REQUIRED",
            &["reference_configurations"],
            "model document 0.4.0 requires explicit reference_configurations",
        ),
        Some(list) => {
            for configuration in list {
                let id = configuration.id.as_str();
                if !nonempty(id) || !configurations.insert(id) {
                    block(
                        diagnostics,
                        "LOAD_STATE_REFERENCE_CONFIGURATION_INVALID",
                        &["reference_configurations", id],
                        "reference configuration IDs must be nonempty and unique",
                    );
                }
                if !nonempty(&configuration.provenance) {
                    block(
                        diagnostics,
                        "LOAD_STATE_PROVENANCE_REQUIRED",
                        &[id, "provenance"],
                        "reference configuration requires nonempty provenance",
                    );
                }
                let mut covered = HashSet::new();
                for member in &configuration.member_references {
                    let pipe = member.pipe_ref.as_str();
                    if !pipe_ids.contains(pipe) {
                        block(
                            diagnostics,
                            "LOAD_STATE_REFERENCE_MEMBER_UNKNOWN",
                            &[id, pipe],
                            "member reference names no pipe in this model",
                        );
                    } else if !covered.insert(pipe) {
                        block(diagnostics, "LOAD_STATE_REFERENCE_MEMBER_DUPLICATE", &[id, pipe],
                            "a pipe may have only one member reference per configuration; no precedence is inferred");
                    }
                    if !nonempty(&member.provenance) {
                        block(
                            diagnostics,
                            "LOAD_STATE_PROVENANCE_REQUIRED",
                            &[id, pipe, "provenance"],
                            "member reference requires nonempty provenance",
                        );
                    }
                }
                for pipe in &model.pipe_segments {
                    if !covered.contains(pipe.id.as_str()) {
                        block(diagnostics, "LOAD_STATE_REFERENCE_MEMBER_MISSING", &[id, &pipe.id],
                            "every pipe requires an explicit reference state; no installation temperature or fit is assumed");
                    }
                }
            }
        }
    }
    for (material, laws) in model.materials.iter().zip(&model.material_expansion_laws) {
        let mut ids = HashSet::new();
        for law in laws.value().into_iter().flatten() {
            if !nonempty(law.id()) || !ids.insert(law.id()) {
                block(
                    diagnostics,
                    "LOAD_STATE_EXPANSION_LAW_INVALID",
                    &[&material.id, law.id()],
                    "expansion law IDs must be nonempty and unique on their material",
                );
            }
            if !nonempty(law.provenance()) {
                block(
                    diagnostics,
                    "LOAD_STATE_PROVENANCE_REQUIRED",
                    &[&material.id, law.id(), "provenance"],
                    "expansion law requires nonempty provenance",
                );
            }
        }
    }
    for case in &model.load_cases {
        validate_case(model, case, &configurations, &pipe_ids, diagnostics);
    }
}

fn validate_case(
    model: &PreviewModel,
    case: &PreviewLoadCase,
    configurations: &HashSet<&str>,
    pipe_ids: &HashSet<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let id = case.id.as_str();
    let Some(state) = case.analysis_state.value() else {
        block(diagnostics, "LOAD_STATE_ANALYSIS_STATE_REQUIRED", &[id, "analysis_state"],
            "every 0.4.0 load case requires an explicit analysis_state; case labels never imply temperatures, fit or support state");
        return;
    };
    if case.modulus_basis_ref.is_some() || case.modulus_basis_temperature.is_some() {
        block(diagnostics, "LOAD_STATE_CASE_MODULUS_BASIS_UNSUPPORTED", &[id],
            "case-wide modulus_basis_ref/modulus_basis_temperature are superseded by per-element material_selection and cannot also be supplied");
    }
    if state.contract != LOAD_REFERENCE_STATE_CONTRACT {
        block(
            diagnostics,
            "LOAD_STATE_CONTRACT_UNSUPPORTED",
            &[id, "analysis_state.contract"],
            format!("supported analysis_state contract is {LOAD_REFERENCE_STATE_CONTRACT}"),
        );
    }
    if !nonempty(&state.provenance) {
        block(
            diagnostics,
            "LOAD_STATE_PROVENANCE_REQUIRED",
            &[id, "analysis_state.provenance"],
            "analysis_state requires nonempty provenance",
        );
    }
    if !configurations.contains(state.reference_configuration_ref.as_str()) {
        block(
            diagnostics,
            "LOAD_STATE_REFERENCE_CONFIGURATION_UNRESOLVED",
            &[id, &state.reference_configuration_ref],
            "analysis_state names no model reference configuration",
        );
    }
    let mut elements = HashSet::new();
    for element in &state.element_states {
        let pipe = element.pipe_ref.as_str();
        if !pipe_ids.contains(pipe) {
            block(
                diagnostics,
                "LOAD_STATE_ELEMENT_UNKNOWN",
                &[id, pipe],
                "element state names no pipe in this model",
            );
        } else if !elements.insert(pipe) {
            block(diagnostics, "LOAD_STATE_ELEMENT_DUPLICATE", &[id, pipe],
                "a pipe may have only one element state per case; no array-order precedence is applied");
        }
        if element.mass_state_ref.is_some() {
            block(diagnostics, "LOAD_STATE_MASS_STATE_UNSUPPORTED", &[id, pipe, "mass_state_ref"],
                "per-case mass states are outside this capability; mass-state input is refused rather than ignored");
        }
    }
    for pipe in &model.pipe_segments {
        if !elements.contains(pipe.id.as_str()) {
            block(diagnostics, "LOAD_STATE_ELEMENT_MISSING", &[id, &pipe.id],
                "every pipe requires an explicit element state; no temperature, material basis or unchanged state is assumed");
        }
    }
    let support_ids = model
        .supports
        .iter()
        .map(|support| support.id.as_str())
        .collect::<HashSet<_>>();
    let mut supports = HashSet::new();
    for support_state in &state.support_states {
        let support = support_state.support_ref.as_str();
        if !support_ids.contains(support) {
            block(
                diagnostics,
                "LOAD_STATE_SUPPORT_UNKNOWN",
                &[id, support],
                "support state names no model support",
            );
        } else if !supports.insert(support) {
            block(
                diagnostics,
                "LOAD_STATE_SUPPORT_DUPLICATE",
                &[id, support],
                "a support may have only one support state per case",
            );
        }
        if !matches!(
            support_state.participation,
            ParticipationInput::ActiveModelDevice {}
        ) {
            block(diagnostics, "LOAD_STATE_SUPPORT_PARTICIPATION_UNSUPPORTED", &[id, support, "participation"],
                "inactive and locked support states are outside this capability; the support is not silently kept, removed or locked");
        }
        if support_state.base_motion.is_some() || support_state.device_reference.is_some() {
            block(diagnostics, "LOAD_STATE_SUPPORT_REFERENCE_UNSUPPORTED", &[id, support],
                "spring base motion and device preload/reference laws are outside this capability; no rigid substitute is applied");
        }
        let mut dofs = HashSet::new();
        for motion in support_state.boundary_motion.iter().flatten() {
            match parse_dof(&motion.dof) {
                Ok(dof) => {
                    if !dofs.insert(dof_index(dof)) {
                        block(
                            diagnostics,
                            "LOAD_STATE_BOUNDARY_MOTION_DUPLICATE",
                            &[id, support, &motion.dof],
                            "each support DOF may carry at most one boundary motion",
                        );
                    }
                }
                Err(message) => block(
                    diagnostics,
                    "LOAD_STATE_BOUNDARY_MOTION_DOF_INVALID",
                    &[id, support, &motion.dof],
                    message,
                ),
            }
        }
    }
    for support in &model.supports {
        if !supports.contains(support.id.as_str()) {
            block(diagnostics, "LOAD_STATE_SUPPORT_STATE_MISSING", &[id, &support.id],
                "every model support requires an explicit support state; omission is not permission to keep, remove or lock it");
        }
    }
    let mut stored = HashMap::<&str, usize>::new();
    for load in &case.primitive_loads {
        *stored.entry(load.id.as_str()).or_default() += 1;
    }
    let mut referenced = HashSet::new();
    for source in &state.load_sources {
        let source_ref = source.source_ref.as_str();
        if !referenced.insert(source_ref) {
            block(diagnostics, "LOAD_STATE_SOURCE_DUPLICATE", &[id, source_ref],
                "a physical source may be included once; equal IDs are rejected rather than guessed as intentional doubling, and an explicit factor requests scaling");
            continue;
        }
        match stored.get(source_ref) {
            None => block(
                diagnostics,
                "LOAD_STATE_SOURCE_UNRESOLVED",
                &[id, source_ref],
                "load source names no primitive stored in this load case",
            ),
            Some(count) if *count > 1 => block(
                diagnostics,
                "LOAD_STATE_SOURCE_AMBIGUOUS",
                &[id, source_ref],
                "the referenced stored primitive ID is not unique in this load case",
            ),
            Some(_) => {}
        }
        if !source.factor.is_finite() || source.factor == 0.0 {
            block(
                diagnostics,
                "LOAD_STATE_SOURCE_FACTOR_INVALID",
                &[id, source_ref],
                "source factor must be finite and nonzero; omit the source to exclude it",
            );
        }
        if case
            .primitive_loads
            .iter()
            .any(|load| load.id == source_ref && load.category == "thermal")
        {
            block(diagnostics, "LOAD_STATE_LEGACY_THERMAL_PRIMITIVE_UNSUPPORTED", &[id, source_ref],
                "thermal strain is owned by the resolved element thermal_state; a legacy thermal primitive cannot also be consumed");
        }
    }
}

pub(crate) struct ResolvedMember {
    pub pipe_index: usize,
    pub pipe_id: String,
    pub material: ResolvedMemberMaterial,
    pub strain: ResolvedStrain,
}

pub(crate) struct ResolvedCase {
    /// One record per built pipe, in `BuiltModel::pipes` order.
    pub members: Vec<ResolvedMember>,
    pub pairs: HashMap<String, IsotropicENu>,
    /// Prescribed nonzero-capable boundary values by global DOF; absent
    /// restrained DOFs are explicit zero.
    pub prescribed: BTreeMap<usize, f64>,
    /// The authored case restricted to its declared ordinary source ledger.
    pub effective_case: PreviewLoadCase,
    /// Resolved consumed inputs, published as the case evidence record.
    pub evidence: Value,
}

fn selection(
    input: &MaterialSelectionInput,
    identity: &TemperatureIdentity,
) -> Result<MaterialSelection, String> {
    Ok(match input {
        MaterialSelectionInput::ExplicitBaseProperties {
            material_ref,
            applicability_reference,
        } => MaterialSelection::ExplicitBaseProperties {
            material_ref: material_ref.clone(),
            applicability_reference: applicability_reference.clone(),
        },
        MaterialSelectionInput::ExactPoint {
            material_ref,
            point_ref,
        } => MaterialSelection::ExactPoint {
            material_ref: material_ref.clone(),
            point_ref: point_ref.clone(),
        },
        MaterialSelectionInput::TemperatureInterpolation {
            material_ref,
            temperature,
            interpolation: InterpolationPolicy::PiecewiseLinear,
            extrapolation: ExtrapolationPolicy::Forbidden,
        } => MaterialSelection::TemperatureInterpolation {
            material_ref: material_ref.clone(),
            temperature: identity.canonical(temperature)?,
        },
    })
}

/// Identity scope of one member: every absolute temperature its selection,
/// reference and expansion law actually compare. Unrelated material points of
/// a fixed or named-point selection are not registered.
fn member_identity(
    authored: &MaterialInput,
    element: &ElementStateInput,
    reference: &MemberReferenceInput,
    law: Option<&ExpansionLawInput>,
) -> Result<TemperatureIdentity, (&'static str, String)> {
    let mut identity = TemperatureIdentity::default();
    let mut add = |quantity: &Quantity| -> Result<(), (&'static str, String)> {
        let binary = normalized(quantity, Dimension::Temperature)
            .map_err(|message| ("LOAD_STATE_QUANTITY_INVALID", message))?;
        identity
            .register(quantity, binary)
            .map_err(|message| ("LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED", message))
    };
    for point in &authored.temperature_points {
        let relevant = match &element.material_selection {
            MaterialSelectionInput::ExplicitBaseProperties { .. } => false,
            MaterialSelectionInput::ExactPoint { point_ref, .. } => &point.id == point_ref,
            MaterialSelectionInput::TemperatureInterpolation { .. } => true,
        };
        if let (true, Some(temperature)) = (relevant, &point.temperature) {
            add(temperature)?;
        }
    }
    if let MaterialSelectionInput::TemperatureInterpolation { temperature, .. } =
        &element.material_selection
    {
        add(temperature)?;
    }
    if let Some(temperature) = &element.operating_temperature {
        add(temperature)?;
    }
    if let ReferenceBasisInput::TemperatureReference {
        installation_temperature,
    } = &reference.basis
    {
        add(installation_temperature)?;
    }
    for temperature in law.map(law_temperatures).unwrap_or_default() {
        add(temperature)?;
    }
    identity
        .check_order()
        .map_err(|message| ("LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED", message))?;
    Ok(identity)
}

fn law_temperatures(law: &ExpansionLawInput) -> Vec<&Quantity> {
    match law {
        ExpansionLawInput::EngineeringSecant {
            datum_temperature,
            data,
            ..
        } => std::iter::once(datum_temperature)
            .chain(match data {
                SecantDataInput::Constant { .. } => Vec::new(),
                SecantDataInput::Table { points, .. } => {
                    points.iter().map(|point| &point.temperature).collect()
                }
            })
            .collect(),
        ExpansionLawInput::EngineeringDilation {
            datum_temperature,
            data: DilationDataInput::Table { points, .. },
            ..
        } => std::iter::once(datum_temperature)
            .chain(points.iter().map(|point| &point.temperature))
            .collect(),
        ExpansionLawInput::DifferentialPerDatumLength {
            datum_temperature,
            data: CoefficientTableInput::Table { points, .. },
            ..
        }
        | ExpansionLawInput::LogarithmicPerCurrentLength {
            datum_temperature,
            data: CoefficientTableInput::Table { points, .. },
            ..
        } => std::iter::once(datum_temperature)
            .chain(points.iter().map(|point| &point.temperature))
            .collect(),
    }
}

fn coefficient_points(
    points: &[CoefficientPointInput],
    identity: &TemperatureIdentity,
) -> Result<Vec<ThermalPoint>, String> {
    points
        .iter()
        .map(|point| {
            Ok(ThermalPoint {
                temperature_kelvin: identity.kelvin(&point.temperature)?,
                value: normalized(&point.coefficient, Dimension::ThermalExpansionCoefficient)?,
            })
        })
        .collect()
}

fn normalized_law(
    law: &ExpansionLawInput,
    identity: &TemperatureIdentity,
) -> Result<NormalizedExpansionLaw, String> {
    Ok(match law {
        ExpansionLawInput::EngineeringSecant {
            datum_temperature,
            data,
            ..
        } => NormalizedExpansionLaw::EngineeringSecant {
            datum_kelvin: identity.kelvin(datum_temperature)?,
            data: match data {
                SecantDataInput::Constant { coefficient } => CoefficientData::Constant(normalized(
                    coefficient,
                    Dimension::ThermalExpansionCoefficient,
                )?),
                SecantDataInput::Table { points, .. } => {
                    CoefficientData::Table(coefficient_points(points, identity)?)
                }
            },
        },
        ExpansionLawInput::EngineeringDilation {
            datum_temperature,
            data: DilationDataInput::Table { points, .. },
            ..
        } => NormalizedExpansionLaw::EngineeringDilation {
            datum_kelvin: identity.kelvin(datum_temperature)?,
            points: points
                .iter()
                .map(|point| {
                    Ok(ThermalPoint {
                        temperature_kelvin: identity.kelvin(&point.temperature)?,
                        value: normalized(&point.dilation, Dimension::Dimensionless)?,
                    })
                })
                .collect::<Result<_, String>>()?,
        },
        ExpansionLawInput::DifferentialPerDatumLength {
            datum_temperature,
            data: CoefficientTableInput::Table { points, .. },
            ..
        } => NormalizedExpansionLaw::DifferentialPerDatumLength {
            datum_kelvin: identity.kelvin(datum_temperature)?,
            points: coefficient_points(points, identity)?,
        },
        ExpansionLawInput::LogarithmicPerCurrentLength {
            datum_temperature,
            data: CoefficientTableInput::Table { points, .. },
            ..
        } => NormalizedExpansionLaw::LogarithmicPerCurrentLength {
            datum_kelvin: identity.kelvin(datum_temperature)?,
            points: coefficient_points(points, identity)?,
        },
    })
}

fn node_position(model: &PreviewModel, id: &str) -> Option<[f64; 3]> {
    model
        .nodes
        .iter()
        .find(|node| node.id == id)
        .map(|node| [node.position.x, node.position.y, node.position.z])
}

/// Hash of the normalized geometry projection the reference configuration binds.
pub(crate) fn reference_geometry_sha256(model: &PreviewModel) -> String {
    let projection = json!({
        "projection": "authored_model_geometry_v1",
        "nodes": model.nodes.iter().map(|node| json!({"id":node.id,"position_m":[node.position.x,node.position.y,node.position.z]})).collect::<Vec<_>>(),
        "pipes": model.pipe_segments.iter().map(|pipe| json!({"id":pipe.id,"from":pipe.from,"to":pipe.to,
            "y_reference":pipe.y_reference.map(|v| [v.x,v.y,v.z])})).collect::<Vec<_>>(),
    });
    let text = open_pipe_stress_canonical_json::canonical_json(&projection);
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

fn segments(list: &[super::thermal::ConsumedSegment]) -> Vec<Value> {
    list.iter()
        .map(|segment| {
            json!({"use": segment.use_kind.as_str(), "lower_index": segment.lower_index,
                "upper_index": segment.upper_index, "start_k": segment.start_kelvin, "end_k": segment.end_kelvin})
        })
        .collect()
}

fn optional_kelvin(value: Option<f64>) -> Value {
    value.map_or(Value::Null, |v| json!(v))
}

/// Resolve one case from normalized model/materials. `validate_document` must
/// already have passed without blocking diagnostics.
pub(crate) fn resolve_case(
    model: &PreviewModel,
    materials: &[MaterialInput],
    case: &PreviewLoadCase,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<ResolvedCase> {
    let before = diagnostics.len();
    let id = case.id.as_str();
    let state = case.analysis_state.value()?;
    let configuration = model
        .reference_configurations
        .value()?
        .iter()
        .find(|configuration| configuration.id == state.reference_configuration_ref)?;
    let mut members = Vec::new();
    let mut pairs = HashMap::new();
    let mut member_evidence = Vec::new();
    let mut contributions = Vec::new();
    for (pipe_index, pipe) in model.pipe_segments.iter().enumerate() {
        let (Some(element), Some(reference)) = (
            state
                .element_states
                .iter()
                .find(|element| element.pipe_ref == pipe.id),
            configuration
                .member_references
                .iter()
                .find(|member| member.pipe_ref == pipe.id),
        ) else {
            continue;
        };
        let matches = materials
            .iter()
            .enumerate()
            .filter(|(_, material)| material.id == pipe.material)
            .collect::<Vec<_>>();
        let [(material_index, material)] = matches.as_slice() else {
            block(diagnostics, "LOAD_STATE_MATERIAL_UNRESOLVED", &[id, &pipe.id, &pipe.material],
                "the pipe's material must resolve to exactly one model material; no alias or merge is created");
            continue;
        };
        let authored_material = &model.materials[*material_index];
        let law_input = match &element.thermal_state {
            ThermalStateInput::FreeLengthState { expansion_law_ref } => model
                .material_expansion_laws[*material_index]
                .value()
                .into_iter()
                .flatten()
                .find(|law| law.id() == expansion_law_ref),
            _ => None,
        };
        let identity = match member_identity(authored_material, element, reference, law_input) {
            Ok(identity) => identity,
            Err((code, message)) => {
                block(diagnostics, code, &[id, &pipe.id], message);
                continue;
            }
        };
        // Same material identity and data; only compared absolute temperatures
        // are replaced by their exact-identity canonical kelvin values.
        let mut member_material = (*material).clone();
        let mut canonical_failure = None;
        for (point, authored) in member_material
            .temperature_points
            .iter_mut()
            .zip(&authored_material.temperature_points)
        {
            if let Some(temperature) = &authored.temperature {
                match identity.canonical(temperature) {
                    Ok(canonical) => point.temperature = Some(canonical),
                    Err(_)
                        if matches!(
                            element.material_selection,
                            MaterialSelectionInput::ExplicitBaseProperties { .. }
                        ) || matches!(&element.material_selection, MaterialSelectionInput::ExactPoint { point_ref, .. } if &point.id != point_ref) =>
                        {}
                    Err(message) => canonical_failure = Some(message),
                }
            }
        }
        let canonical_selection = selection(&element.material_selection, &identity);
        let canonical_operating = element
            .operating_temperature
            .as_ref()
            .map(|temperature| identity.canonical(temperature))
            .transpose();
        let (canonical_selection, canonical_operating) =
            match (canonical_failure, canonical_selection, canonical_operating) {
                (None, Ok(selection), Ok(operating)) => (selection, operating),
                (Some(message), _, _) | (_, Err(message), _) | (_, _, Err(message)) => {
                    block(
                        diagnostics,
                        "LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED",
                        &[id, &pipe.id],
                        message,
                    );
                    continue;
                }
            };
        let override_input =
            element
                .analysis_basis_override
                .as_ref()
                .map(|value| AnalysisBasisOverride {
                    reason: value.reason.clone(),
                    provenance: value.provenance.clone(),
                });
        let resolved_material = match material::select_for_member(
            &member_material,
            &canonical_selection,
            canonical_operating.as_ref(),
            override_input.as_ref(),
        ) {
            Ok(value) => value,
            Err(error) => {
                block(
                    diagnostics,
                    error.code,
                    &[id, &pipe.id, &error.material_id],
                    format!("{}; point_refs={:?}", error.message, error.point_refs),
                );
                continue;
            }
        };
        let installation = match &reference.basis {
            ReferenceBasisInput::TemperatureReference {
                installation_temperature,
            } => match identity.kelvin(installation_temperature) {
                Ok(value) => Some(value),
                Err(message) => {
                    block(
                        diagnostics,
                        "LOAD_STATE_TEMPERATURE_IDENTITY_UNRESOLVED",
                        &[id, &pipe.id, "installation_temperature"],
                        message,
                    );
                    continue;
                }
            },
            ReferenceBasisInput::DirectStrainReference {} => None,
        };
        let mut law_id = None;
        let thermal_input = match &element.thermal_state {
            ThermalStateInput::UnchangedReference { provenance } => {
                if !nonempty(provenance) {
                    block(
                        diagnostics,
                        "LOAD_STATE_PROVENANCE_REQUIRED",
                        &[id, &pipe.id, "thermal_state"],
                        "unchanged reference requires nonempty provenance",
                    );
                    continue;
                }
                ThermalInput::UnchangedReference
            }
            ThermalStateInput::ExplicitIntervalStrain {
                strain,
                interval_reference,
                provenance,
            } => {
                if !nonempty(interval_reference) || !nonempty(provenance) {
                    block(diagnostics, "LOAD_STATE_PROVENANCE_REQUIRED", &[id, &pipe.id, "thermal_state"],
                        "explicit interval strain requires nonempty interval_reference and provenance");
                    continue;
                }
                match normalized(strain, Dimension::Dimensionless) {
                    Ok(strain) => ThermalInput::ExplicitIntervalStrain { strain },
                    Err(message) => {
                        block(
                            diagnostics,
                            "LOAD_STATE_QUANTITY_INVALID",
                            &[id, &pipe.id, "thermal_state.strain"],
                            message,
                        );
                        continue;
                    }
                }
            }
            ThermalStateInput::ConstantAlphaInterval {
                coefficient,
                temperature_change,
                coefficient_meaning: CoefficientMeaning::EngineeringInterval,
                provenance,
            } => {
                if !nonempty(provenance) {
                    block(
                        diagnostics,
                        "LOAD_STATE_PROVENANCE_REQUIRED",
                        &[id, &pipe.id, "thermal_state"],
                        "constant-alpha interval requires nonempty provenance",
                    );
                    continue;
                }
                match (
                    normalized(coefficient, Dimension::ThermalExpansionCoefficient),
                    normalized(temperature_change, Dimension::TemperatureInterval),
                ) {
                    (Ok(coefficient_per_kelvin), Ok(temperature_change_kelvin)) => {
                        ThermalInput::ConstantAlphaInterval {
                            coefficient_per_kelvin,
                            temperature_change_kelvin,
                        }
                    }
                    (Err(message), _) | (_, Err(message)) => {
                        block(
                            diagnostics,
                            "LOAD_STATE_QUANTITY_INVALID",
                            &[id, &pipe.id, "thermal_state"],
                            message,
                        );
                        continue;
                    }
                }
            }
            ThermalStateInput::FreeLengthState { expansion_law_ref } => {
                let Some(installation_kelvin) = installation else {
                    block(diagnostics, "LOAD_STATE_FREE_LENGTH_REQUIRES_TEMPERATURE_REFERENCE", &[id, &pipe.id],
                        "a free-length thermal state requires a temperature_reference member basis; a direct-strain reference supplies no installation temperature");
                    continue;
                };
                let Some(operating_kelvin) = resolved_material.operating_temperature_k else {
                    block(diagnostics, "LOAD_STATE_OPERATING_TEMPERATURE_REQUIRED", &[id, &pipe.id],
                        "a free-length thermal state requires the actual element operating_temperature; none is inferred from a property point or ambient value");
                    continue;
                };
                let law = model.material_expansion_laws[*material_index]
                    .value()
                    .into_iter()
                    .flatten()
                    .find(|law| law.id() == expansion_law_ref);
                let Some(law) = law else {
                    block(
                        diagnostics,
                        "LOAD_STATE_EXPANSION_LAW_UNRESOLVED",
                        &[id, &pipe.id, expansion_law_ref],
                        "the expansion law must be owned by this member's actual material record",
                    );
                    continue;
                };
                match normalized_law(law, &identity) {
                    Ok(law) => {
                        law_id = Some(expansion_law_ref.clone());
                        ThermalInput::FreeLengthState {
                            installation_kelvin,
                            operating_kelvin,
                            law,
                        }
                    }
                    Err(message) => {
                        block(
                            diagnostics,
                            "LOAD_STATE_QUANTITY_INVALID",
                            &[id, &pipe.id, expansion_law_ref],
                            message,
                        );
                        continue;
                    }
                }
            }
        };
        let (Some(from), Some(to)) = (
            node_position(model, &pipe.from),
            node_position(model, &pipe.to),
        ) else {
            continue;
        };
        let length = (to[0] - from[0])
            .hypot(to[1] - from[1])
            .hypot(to[2] - from[2]);
        let (fit_input, fit_kind, fit_value) = match &reference.fit {
            FitReferenceInput::NoFit {} => (FitInput::None, "none", Value::Null),
            FitReferenceInput::NaturalLengthChange { length_change } => {
                match normalized(length_change, Dimension::Length) {
                    Ok(change_m) => (
                        FitInput::NaturalLengthChange {
                            change_m,
                            reference_length_m: length,
                        },
                        "natural_length_change",
                        json!({"length_change_m": change_m}),
                    ),
                    Err(message) => {
                        block(
                            diagnostics,
                            "LOAD_STATE_QUANTITY_INVALID",
                            &[id, &pipe.id, "fit.length_change"],
                            message,
                        );
                        continue;
                    }
                }
            }
            FitReferenceInput::FitStrain { strain } => {
                match normalized(strain, Dimension::Dimensionless) {
                    Ok(strain) => (
                        FitInput::EngineeringStrain { strain },
                        "fit_strain",
                        json!({"strain": strain}),
                    ),
                    Err(message) => {
                        block(
                            diagnostics,
                            "LOAD_STATE_QUANTITY_INVALID",
                            &[id, &pipe.id, "fit.strain"],
                            message,
                        );
                        continue;
                    }
                }
            }
        };
        let strain = match thermal::resolve_strain(&thermal_input, &fit_input) {
            Ok(value) => value,
            Err(error) => {
                block(diagnostics, "LOAD_STATE_STRAIN_UNRESOLVED", &[id, &pipe.id],
                    format!("explicit thermal/fit definition cannot be resolved over the consumed interval: {error:?}; no extrapolation, fallback or default is applied"));
                continue;
            }
        };
        let pair = resolved_material.pair;
        member_evidence.push(json!({
            "pipe_id": pipe.id,
            "material_id": resolved_material.material_id,
            "material_selection_kind": resolved_material.selection_kind,
            "consumed_material_points": resolved_material.consumed_points.iter().map(|point| json!({
                "point_id": point.point_id, "temperature_k": optional_kelvin(point.temperature_k),
                "E_pa": point.pair.elastic_modulus_pa(), "nu": point.pair.poisson_ratio(),
                "retained_G_ignored": point.retained_g_ignored})).collect::<Vec<_>>(),
            "interpolation_fraction": resolved_material.interpolation_fraction,
            "applicability_reference": resolved_material.applicability_reference,
            "analysis_basis_override": resolved_material.analysis_basis_override.as_ref().map(|value| json!({"reason":value.reason,"provenance":value.provenance})),
            "selected_E_pa": pair.elastic_modulus_pa(),
            "selected_nu": pair.poisson_ratio(),
            "derived_G_pa": pair.shear_modulus_pa(),
            "G_basis": "E/[2(1+nu)] from the selected pair",
            "retained_G_ignored": resolved_material.retained_g_ignored,
            "operating_temperature_k": optional_kelvin(resolved_material.operating_temperature_k),
            "material_selection_temperature_k": optional_kelvin(resolved_material.selection_temperature_k),
            "reference_basis": match reference.basis { ReferenceBasisInput::TemperatureReference {..} => "temperature_reference", ReferenceBasisInput::DirectStrainReference {} => "direct_strain_reference" },
            "installation_temperature_k": optional_kelvin(installation),
            "thermal_definition": strain.definition,
            "expansion_law_id": law_id,
            "coefficient_datum_k": optional_kelvin(strain.datum_kelvin),
            // Consumed data entered the value; consulted data only established
            // coverage/positivity admissibility (dilation datum zero, path check).
            "consumed_law_point_indices": strain.consumed_point_indices,
            "consumed_law_segments": segments(&strain.consumed_segments),
            "consulted_law_point_indices": strain.consulted_point_indices,
            "consulted_law_segments": segments(&strain.consulted_segments),
            "installation_datum_stretch": strain.installation_datum_stretch,
            "operating_datum_stretch": strain.operating_datum_stretch,
            "thermal_strain": strain.thermal_strain,
            "thermal_stretch": strain.thermal_stretch,
            "fit_kind": fit_kind,
            "fit_input": fit_value,
            "reference_length_m": length,
            "fit_strain": strain.fit_strain,
            "fit_stretch": strain.fit_stretch,
            "total_eigenstrain": strain.total_eigenstrain,
            "eigenstrain_composition": "lambda_fit*lambda_thermal-1",
        }));
        contributions.push(json!({"source_id": format!("member_state:{}", pipe.id), "owner_kind": "resolved_member_state",
            "classification": "eigenstrain", "consumed_input_refs": [format!("{}:{}", configuration.id, pipe.id), format!("{}:element_state:{}", id, pipe.id)],
            "value": strain.total_eigenstrain}));
        pairs.insert(pipe.id.clone(), pair);
        members.push(ResolvedMember {
            pipe_index,
            pipe_id: pipe.id.clone(),
            material: resolved_material,
            strain,
        });
    }
    let node_index = model
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.as_str(), index))
        .collect::<HashMap<_, _>>();
    let mut prescribed = BTreeMap::new();
    let mut support_evidence = Vec::new();
    for support_state in &state.support_states {
        let Some(support) = model
            .supports
            .iter()
            .find(|support| support.id == support_state.support_ref)
        else {
            continue;
        };
        let Some(&node) = node_index.get(support.node.as_str()) else {
            continue;
        };
        let restrained = support
            .restraints
            .iter()
            .filter_map(|dof| parse_dof(dof).ok())
            .collect::<Vec<_>>();
        let rigid = !is_spring(support)
            && support.nonlinear.is_none()
            && !is_constant_effort_support(support);
        for motion in support_state.boundary_motion.iter().flatten() {
            let Ok(dof) = parse_dof(&motion.dof) else {
                continue;
            };
            if !rigid || !restrained.contains(&dof) {
                block(diagnostics, "LOAD_STATE_BOUNDARY_MOTION_UNRESTRAINED", &[id, &support.id, &motion.dof],
                    "boundary motion applies only to a DOF this support already restrains rigidly; motion on a free or spring DOF requires its own explicit law");
                continue;
            }
            let dimension = if matches!(dof, FrameDof::Ux | FrameDof::Uy | FrameDof::Uz) {
                Dimension::Displacement
            } else {
                Dimension::Rotation
            };
            let value = match normalized(&motion.value, dimension) {
                Ok(value) => value,
                Err(message) => {
                    block(diagnostics, "LOAD_STATE_BOUNDARY_MOTION_UNIT_INVALID", &[id, &support.id, &motion.dof],
                        format!("translations require a length unit and rotations an angle unit: {message}"));
                    continue;
                }
            };
            let global = node * DOF_PER_NODE + dof_index(dof);
            prescribed.insert(global, value);
            support_evidence.push(json!({"support_id": support.id, "node_id": support.node, "dof": motion.dof.to_ascii_uppercase(),
                "global_dof": global, "law_kind": "rigid_prescribed", "prescribed_value": value,
                "unit": if dimension == Dimension::Displacement { "m" } else { "rad" },
                "meaning": "absolute_reference_displacement", "physical_state_source": "support_state.boundary_motion"}));
            contributions.push(json!({"source_id": format!("support_state:{}:{}", support.id, motion.dof.to_ascii_uppercase()),
                "owner_kind": "support_state", "classification": "prescribed_boundary", "value": value}));
        }
    }
    let mut effective_case = case.clone();
    effective_case.primitive_loads = Vec::new();
    for source in &state.load_sources {
        let Some(load) = case
            .primitive_loads
            .iter()
            .find(|load| load.id == source.source_ref)
        else {
            continue;
        };
        let mut included = load.clone();
        included.magnitude.value *= source.factor;
        contributions.push(json!({"source_id": load.id, "owner_kind": "stored_primitive", "classification": "ordinary_applied",
            "factor": source.factor, "category": load.category, "dimension": load.dimension,
            "authored_normalized_magnitude": load.magnitude.value, "applied_magnitude": included.magnitude.value}));
        effective_case.primitive_loads.push(included);
    }
    for region in case.pressure_regions.iter().flatten() {
        contributions.push(json!({"source_id": format!("pressure_region:{}", region.id.as_deref().unwrap_or("")),
            "owner_kind": "pressure_region", "classification": "pressure_eigen_and_closure", "factor": Value::Null}));
    }
    let excluded = case
        .primitive_loads
        .iter()
        .filter(|load| !state.load_sources.iter().any(|source| source.source_ref == load.id))
        .map(|load| json!({"source_id": load.id, "owner_kind": "stored_primitive", "classification": "excluded",
            "category": load.category, "reason": "not listed in analysis_state.load_sources"}))
        .collect::<Vec<_>>();
    if diagnostics[before..]
        .iter()
        .any(|finding| finding.severity == "blocking")
        || members.len() != model.pipe_segments.len()
    {
        return None;
    }
    let evidence = json!({
        "load_case_id": id,
        "contract": LOAD_REFERENCE_STATE_CONTRACT,
        "profile": LOAD_STATE_PROFILE_ID,
        "reference_configuration_id": configuration.id,
        "reference_geometry": {"kind": "authored_model_geometry", "projection_sha256": reference_geometry_sha256(model)},
        "members": member_evidence,
        "support_components": support_evidence,
        "contributions": contributions,
        "excluded_sources": excluded,
        "history": {"kind": "independent_equilibrium"},
        "provenance": state.provenance,
    });
    Some(ResolvedCase {
        members,
        pairs,
        prescribed,
        effective_case,
        evidence,
    })
}
