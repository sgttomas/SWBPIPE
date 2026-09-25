//! Private, complete-source recovery for the bounded straight-frame family.
//!
//! The certificate concerns the declared binary64 element/load operands. It is
//! neither a primitive-geometry certificate nor an ordinary factorization report.
//! Selection and public admission belong to the product facade.
use super::{
    dof_index, is_constant_effort_support, parse_category, parse_direction, parse_dof,
    parse_load_dimension, support_stiffness_input, AssemblyEvidence, BuiltModel, LoadDimension,
    LoadTargetInput, Matrix12, PressureThrustLoad, PreviewLoadCase, PreviewModel,
    PrimitiveLoadCategory, Quantity, SpringEntry, SupportFamily, ThermalElementLoad, DOF_PER_NODE,
    ELEMENT_DOF,
};
use exact::functionals::{
    AffineTerm, AttemptBudget, AttemptStage, FunctionalConvention, FunctionalDescriptor,
    FunctionalKey, FunctionalPlan, FunctionalQuantity, FunctionalUnit, MemberEnd,
    RetainedFunctionalSet,
};
use open_pipe_stress_frame_kernel::element_dof_map;
use open_pipe_stress_frame_kernel::structural::{exact_boundary as exact, StructuralSystem};
use open_pipe_stress_primitive_loads::LoadApplication;
use std::collections::{HashMap, HashSet};

pub(super) const RELATIVE_LIMIT: f64 = 1.0e-9;
pub(super) const STATIONS: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
const END_CONVENTION: FunctionalConvention = FunctionalConvention::NodeOnElement;

/// Every slice is the actual invocation's source, before reduction/projection.
/// Unsupported producers are checked from both authored and built inventories.
pub(super) struct Input<'a> {
    pub model: &'a PreviewModel,
    pub built: &'a BuiltModel,
    pub stiffness: &'a [Vec<f64>],
    pub force: &'a [f64],
    pub free: &'a [usize],
    pub prescribed: &'a [(usize, f64)],
    pub spring_entries: &'a [SpringEntry],
    pub load_case: &'a PreviewLoadCase,
    pub load_application: &'a LoadApplication,
    pub thermal_loads: &'a [ThermalElementLoad],
    pub pressure_thrust_loads: &'a [PressureThrustLoad],
}

#[derive(Debug, Clone, PartialEq)]
pub(super) enum RecoveryError {
    Unsupported(&'static str),
    SourceMismatch(&'static str),
    Exact(exact::Error),
}
impl From<exact::Error> for RecoveryError {
    fn from(value: exact::Error) -> Self {
        Self::Exact(value)
    }
}
#[derive(Debug, Clone)]
pub(super) struct RecoveryFailure {
    pub stage: &'static str,
    pub helper_stage: AttemptStage,
    pub error: RecoveryError,
    pub work: exact::WorkReport,
}

#[derive(Debug, Clone)]
pub(super) struct SpringAction {
    pub support_id: String,
    pub global_dof: usize,
    pub value: f64,
    pub functional_index: usize,
}
#[derive(Debug, Clone)]
pub(super) struct SupportActions {
    pub support_id: String,
    pub node_index: usize,
    /// Fx, Fy, Fz [N], Mx, My, Mz [N*m], support-on-pipe in global axes.
    pub values: [f64; 6],
    pub functional_indices: [usize; 6],
}
#[derive(Debug, Clone)]
pub(super) struct MemberRecovery {
    pub member_id: String,
    /// All twelve node-on-element actions, in local i then local j order.
    pub end_forces: [f64; 12],
    pub end_functional_indices: [usize; 12],
    /// J-side section actions at STATIONS, including both endpoints.
    pub sections: [[f64; 6]; 5],
    pub section_functional_indices: [[usize; 6]; 5],
}
#[derive(Debug, Clone)]
pub(super) struct RecoverySummary {
    pub dofs: usize,
    pub stiffness_contributions: usize,
    pub identified_load_contributions: usize,
    pub blocks: usize,
    pub largest_block: usize,
    pub member_end_components: usize,
    pub section_components: usize,
    pub spring_actions: usize,
    pub support_components: usize,
    pub published_nodal_components: usize,
    pub functional_count: usize,
    pub dof_projection_count: usize,
    pub max_absolute_projection_error: f64,
    pub max_relative_projection_error: f64,
    pub relative_limit: f64,
    pub work: exact::WorkReport,
}

/// No public/import constructor: all projections and the immutable companion
/// are created together from one response under one decreasing work budget.
#[derive(Debug)]
pub(super) struct SelectedSourceRecovery {
    displacements: Vec<f64>,
    reactions: Vec<f64>,
    published_nodal: Vec<f64>,
    published_nodal_indices: Vec<usize>,
    members: Vec<MemberRecovery>,
    springs: Vec<SpringAction>,
    supports: Vec<SupportActions>,
    retained: RetainedFunctionalSet,
    summary: RecoverySummary,
    // Retaining the live attempt ledger lets explicit expected-invocation replay
    // continue the same budget; replay never resets the selection's work cap.
    budget: AttemptBudget,
}
impl SelectedSourceRecovery {
    pub fn displacements(&self) -> &[f64] {
        &self.displacements
    }
    pub fn reactions(&self) -> &[f64] {
        &self.reactions
    }
    /// Translational components are certified directly in mm, rotations in rad.
    pub fn published_nodal_components(&self) -> &[f64] {
        &self.published_nodal
    }
    pub fn published_nodal_functional_indices(&self) -> &[usize] {
        &self.published_nodal_indices
    }
    pub fn members(&self) -> &[MemberRecovery] {
        &self.members
    }
    pub fn spring_actions(&self) -> &[SpringAction] {
        &self.springs
    }
    pub fn support_actions(&self) -> &[SupportActions] {
        &self.supports
    }
    pub fn retained(&self) -> &RetainedFunctionalSet {
        &self.retained
    }
    pub fn summary(&self) -> &RecoverySummary {
        &self.summary
    }

    /// Final row binding and receipt hashing are part of the same bounded attempt.
    pub fn charge_finalization(&mut self, operations: usize) -> Result<(), RecoveryFailure> {
        let result = self.budget.charge(AttemptStage::Replay, operations);
        self.summary.work = self.budget.report();
        result.map_err(|error| failure("receipt finalization", error.into(), &self.budget))
    }

    /// Complete current-source equality after the independent captured-source
    /// replay; does not repeat solve/projection arithmetic or reset its ledger.
    pub fn check_binding_against(&mut self, input: Input<'_>) -> Result<(), RecoveryFailure> {
        let result = (|| {
            let source = prepare_sources(&input, &mut self.budget)?;
            self.retained.check_binding_with_budget(
                &source.system(&input), &source.identity,
                exact::ForceBasis::IdentifiedContributions(&source.force_terms),
                &source.descriptors, &mut self.budget,
            ).result?;
            Ok::<(), RecoveryError>(())
        })();
        self.summary.work = self.budget.report();
        result.map_err(|error| failure("current source binding", error, &self.budget))
    }

    /// Rebuild the adapter's ownership/closure/plan from the *current* invocation
    /// before helper replay. Replaying the stored plan against itself is avoided.
    pub fn replay_against(&mut self, input: Input<'_>) -> Result<(), RecoveryFailure> {
        let mut stage = "replay source closure";
        let outcome = (|| {
            let source = prepare_sources(&input, &mut self.budget)?;
            stage = "expected invocation replay";
            self.retained
                .replay_against(
                    &source.system(&input),
                    &source.identity,
                    exact::ForceBasis::IdentifiedContributions(&source.force_terms),
                    &source.descriptors,
                    &[],
                    RELATIVE_LIMIT,
                    &mut self.budget,
                )
                .result?;
            Ok::<(), RecoveryError>(())
        })();
        self.summary.work = self.budget.report();
        outcome.map_err(|error| failure(stage, error, &self.budget))
    }
}

struct Sources {
    assembly: AssemblyEvidence,
    force_terms: Vec<exact::ForceContribution>,
    descriptors: Vec<FunctionalDescriptor>,
    identity: String,
    member_ids: Vec<String>,
    spring_ids: Vec<(String, usize)>,
    support_ids: Vec<(String, usize)>,
    nodal_start: usize,
    spring_start: usize,
    support_start: usize,
}
impl Sources {
    fn system<'a>(&'a self, input: &'a Input<'_>) -> StructuralSystem<'a> {
        StructuralSystem {
            stiffness: input.stiffness,
            force: input.force,
            free_dofs: input.free,
            prescribed: input.prescribed,
            contributions: Some(&self.assembly.contributions),
            symmetry: None,
        }
    }
}
fn failure(stage: &'static str, error: RecoveryError, budget: &AttemptBudget) -> RecoveryFailure {
    RecoveryFailure {
        stage,
        helper_stage: budget.stage(),
        error,
        work: budget.report(),
    }
}
fn charge(budget: &mut AttemptBudget, count: usize) -> Result<(), RecoveryError> {
    budget
        .charge(AttemptStage::SourceClosure, count)
        .map_err(Into::into)
}
fn mismatch(message: &'static str) -> RecoveryError {
    RecoveryError::SourceMismatch(message)
}
fn unsupported(message: &'static str) -> RecoveryError {
    RecoveryError::Unsupported(message)
}
fn unit(component: usize) -> FunctionalUnit {
    if component % 6 < 3 {
        FunctionalUnit::Newton
    } else {
        FunctionalUnit::NewtonMetre
    }
}
fn key(
    case: &str,
    quantity: FunctionalQuantity,
    component: usize,
    convention: FunctionalConvention,
) -> FunctionalKey {
    FunctionalKey {
        case_id: case.to_owned(),
        quantity,
        unit: unit(component),
        convention,
    }
}

/// Sufficient exact transform closure: each actual matrix row and column is a
/// signed unit selector. No tolerance, coordinate test or magnitude exception.
fn signed_permutation(t: &Matrix12) -> Result<[(usize, f64); 12], RecoveryError> {
    let mut selected = [(0, 0.0); 12];
    let mut used = [false; 12];
    for (row, values) in t.iter().enumerate() {
        let mut column = None;
        for (col, &value) in values.iter().enumerate() {
            if value == 0.0 {
                continue;
            }
            if !matches!(value, -1.0 | 1.0) || column.is_some() || used[col] {
                return Err(unsupported("actual transform is not a signed permutation"));
            }
            column = Some((col, value));
        }
        let (col, sign) = column.ok_or_else(|| unsupported("incomplete transform selector"))?;
        used[col] = true;
        selected[row] = (col, sign);
    }
    Ok(selected)
}

// Length-delimited strings and normalized operand bits preserve invocation
// identity without treating an opaque hash or caller label as mechanical proof.
struct Identity {
    names: Vec<String>,
    bits: Vec<u64>,
}
impl Identity {
    fn new() -> Self {
        Self {
            names: vec!["straight-source-recovery-v1".into()],
            bits: Vec::new(),
        }
    }
    fn name(&mut self, name: &str, budget: &mut AttemptBudget) -> Result<(), RecoveryError> {
        charge(budget, name.len().saturating_mul(8).saturating_add(8))?;
        self.names.push(name.to_owned());
        Ok(())
    }
    fn quantity(&mut self, q: &Quantity, budget: &mut AttemptBudget) -> Result<(), RecoveryError> {
        self.name(&q.unit, budget)?;
        self.scalars([q.value]);
        Ok(())
    }
    fn scalars(&mut self, values: impl IntoIterator<Item = f64>) {
        self.bits.extend(values.into_iter().map(f64::to_bits));
    }
    fn indices(&mut self, values: impl IntoIterator<Item = usize>) {
        self.bits.extend(values.into_iter().map(|n| n as u64));
    }
    fn finish(self, budget: &mut AttemptBudget) -> Result<String, RecoveryError> {
        charge(
            budget,
            self.bits
                .len()
                .saturating_mul(24)
                .saturating_add(self.names.len().saturating_mul(8)),
        )?;
        serde_json::to_string(&(self.names, self.bits))
            .map_err(|_| mismatch("source identity encoding"))
    }
}

fn prepare_sources(
    input: &Input<'_>,
    budget: &mut AttemptBudget,
) -> Result<Sources, RecoveryError> {
    charge(budget, 64)?;
    let n = input
        .built
        .nodes
        .len()
        .checked_mul(DOF_PER_NODE)
        .ok_or(exact::Error::Budget)?;
    let member_count = input.built.pipes.len();
    let limits = budget.limits();
    let source_count = member_count
        .checked_mul(144)
        .and_then(|v| v.checked_add(input.spring_entries.len()))
        .and_then(|v| v.checked_add(input.load_application.nodal_loads.len()))
        .ok_or(exact::Error::Budget)?;
    if n > 256 || n > limits.dofs || source_count > 16_384 || source_count > limits.source_terms {
        return Err(exact::Error::Budget.into());
    }
    if input.force.len() != n
        || input.stiffness.len() != n
        || input.stiffness.iter().any(|row| row.len() != n)
        || input.model.nodes.len() != input.built.nodes.len()
        || input.model.pipe_segments.len() != member_count
        || input.built.frame_elements.len() != member_count
        || input.built.supports.len() != input.model.supports.len()
        || input.free.len() > n
        || input.prescribed.len() > n
    {
        return Err(mismatch("actual model/source dimensions"));
    }
    // Precharge formation, fixed-size matrix closure, maps, descriptor generation,
    // source copies and output construction before those operations allocate.
    charge(
        budget,
        n.saturating_mul(n)
            .saturating_mul(16)
            .saturating_add(member_count.saturating_mul(32_000))
            .saturating_add(
                input
                    .model
                    .supports
                    .len()
                    .saturating_mul(n.saturating_mul(24).saturating_add(512)),
            )
            .saturating_add(input.load_case.primitive_loads.len().saturating_mul(128))
            .saturating_add(input.spring_entries.len().saturating_mul(128)),
    )?;
    if member_count == 0 {
        return Err(unsupported("no straight frame family"));
    }
    if !input.built.nonlinear_supports.is_empty()
        || !input.built.nonlinear_initial_states.is_empty()
        || !input.built.nonlinear_friction_normal_reactions.is_empty()
        || !input
            .built
            .nonlinear_derived_friction_normal_reactions
            .is_empty()
        || input.model.supports.iter().any(|s| s.nonlinear.is_some())
    {
        return Err(unsupported("nonlinear/contact source family"));
    }
    if !input.built.user_stiffness_elements.is_empty()
        || !input.built.curved_bend_elements.is_empty()
        || !input.model.components.is_empty()
    {
        return Err(unsupported(
            "component, curved, user-matrix or release source family",
        ));
    }
    if input.load_case.equivalent_static.is_some()
        || !input.load_application.element_uniform_loads.is_empty()
        || !input.load_application.imposed_displacements.is_empty()
        || !input.thermal_loads.is_empty()
        || !input.pressure_thrust_loads.is_empty()
        || input.model.supports.iter().any(|s| {
            is_constant_effort_support(s)
                || s.hanger.as_ref().is_some_and(|h| h.constant_load.is_some())
        })
    {
        return Err(unsupported("non-nodal load producer present"));
    }
    if input.load_application.is_blocked() {
        return Err(mismatch("blocked load application"));
    }
    if input.load_case.id.is_empty() || input.model.project.id.is_empty() {
        return Err(mismatch("empty invocation identity"));
    }
    let mut identity = Identity::new();
    identity.name(&input.model.project.id, budget)?;
    identity.name(&input.load_case.id, budget)?;
    let descriptor_count = member_count
        .saturating_mul(42)
        .saturating_add(n)
        .saturating_add(input.spring_entries.len())
        .saturating_add(input.model.supports.len().saturating_mul(6));
    charge(
        budget,
        descriptor_count.saturating_mul(input.load_case.id.len().saturating_add(32)),
    )?;
    identity.name(
        input.load_case.modulus_basis_ref.as_deref().unwrap_or(""),
        budget,
    )?;
    if let Some(t) = &input.load_case.modulus_basis_temperature {
        identity.indices([1]);
        identity.scalars([t.value]);
        identity.name(&t.unit, budget)?;
    } else {
        identity.indices([0]);
    }
    let mut node_ids = HashMap::new();
    for (index, (authored, built)) in input.model.nodes.iter().zip(&input.built.nodes).enumerate() {
        charge(budget, authored.id.len().saturating_mul(16))?;
        if authored.id.is_empty()
            || node_ids.insert(authored.id.as_str(), index).is_some()
            || built.index != index
            || built
                .coordinates
                .iter()
                .zip([
                    authored.position.x,
                    authored.position.y,
                    authored.position.z,
                ])
                .any(|(a, b)| a.to_bits() != b.to_bits())
        {
            return Err(mismatch("node identity/order/coordinate closure"));
        }
        identity.name(&authored.id, budget)?;
        identity.scalars(built.coordinates);
    }

    let mut force_terms = Vec::new();
    let mut authored_loads = HashMap::new();
    for load in &input.load_case.primitive_loads {
        charge(budget, load.id.len().saturating_mul(4))?;
        if load.id.is_empty() || authored_loads.insert(load.id.as_str(), load).is_some() {
            return Err(mismatch("duplicate/empty authored load identity"));
        }
        let category = parse_category(&load.category).map_err(|_| unsupported("load category"))?;
        if matches!(
            category,
            PrimitiveLoadCategory::Thermal
                | PrimitiveLoadCategory::Pressure
                | PrimitiveLoadCategory::ImposedDisplacement
        ) || !matches!(
            parse_load_dimension(&load.dimension),
            Ok(LoadDimension::Force | LoadDimension::Moment)
        ) || !matches!(load.target, LoadTargetInput::Node { .. })
        {
            return Err(unsupported(
                "authored non-nodal load family, including zero-valued inputs",
            ));
        }
        identity.name(&load.id, budget)?;
        identity.name(&load.category, budget)?;
        identity.name(&load.direction, budget)?;
        identity.name(&load.dimension, budget)?;
        identity.name(&load.magnitude.unit, budget)?;
        identity.scalars([load.magnitude.value]);
    }
    let mut seen_loads = HashSet::new();
    let mut folded_force = vec![0.0; n];
    for load in &input.load_application.nodal_loads {
        charge(budget, load.load_id.len().saturating_mul(4))?;
        let authored = authored_loads
            .get(load.load_id.as_str())
            .ok_or_else(|| mismatch("unowned nodal load contribution"))?;
        let LoadTargetInput::Node { node } = &authored.target else {
            return Err(unsupported("element load"));
        };
        let node_index = *node_ids
            .get(node.as_str())
            .ok_or_else(|| mismatch("load node identity"))?;
        let direction =
            parse_direction(&authored.direction).map_err(|_| mismatch("load direction"))?;
        let dimension =
            parse_load_dimension(&authored.dimension).map_err(|_| mismatch("load dimension"))?;
        if !seen_loads.insert(load.load_id.as_str())
            || load.node_index != node_index
            || load.global_dof != node_index * 6 + direction.dof_index()
            || load.global_dof >= n
            || !load.value.is_finite()
            || load.value.to_bits() != authored.magnitude.value.to_bits()
            || (direction.is_rotational() && dimension != LoadDimension::Moment)
            || (!direction.is_rotational() && dimension != LoadDimension::Force)
        {
            return Err(mismatch("identified load operand/target/coverage"));
        }
        folded_force[load.global_dof] += load.value;
        identity.name(&load.load_id, budget)?;
        identity.indices([load.node_index, load.global_dof]);
        identity.scalars([load.value]);
        force_terms.push(exact::ForceContribution {
            source: load.load_id.clone(),
            dof: load.global_dof,
            value: load.value,
        });
    }
    if seen_loads.len() != authored_loads.len()
        || folded_force
            .iter()
            .zip(input.force)
            .any(|(a, b)| !a.is_finite() || a.to_bits() != b.to_bits())
    {
        return Err(mismatch(
            "complete ordered load fold differs from actual force",
        ));
    }

    let mut authored_supports = HashMap::new();
    for support in &input.model.supports {
        charge(
            budget,
            support
                .id
                .len()
                .saturating_mul(16)
                .saturating_add(support.node.len().saturating_mul(4)),
        )?;
        if support.restraints.len() > 6 {
            return Err(mismatch("support restraint count"));
        }
        if support.id.is_empty()
            || authored_supports
                .insert(support.id.as_str(), support)
                .is_some()
        {
            return Err(mismatch("duplicate/empty support identity"));
        }
        identity.name(&support.id, budget)?;
        identity.name(&support.node, budget)?;
        identity.name(support.family.as_deref().unwrap_or(""), budget)?;
        for dof in &support.restraints {
            identity.name(dof, budget)?;
        }
    }
    let mut support_ids = Vec::new();
    let mut support_seen = HashSet::new();
    let mut rigid_owner = vec![None; n];
    let mut expected_prescribed = vec![None; n];
    let mut expected_springs = Vec::new();
    for support in &input.built.supports {
        charge(budget, support.support_id.len().saturating_mul(16))?;
        if support.restrained_dofs.is_empty() || support.restrained_dofs.len() > 6 {
            return Err(mismatch("built support restraint count"));
        }
        let authored = authored_supports
            .get(support.support_id.as_str())
            .ok_or_else(|| mismatch("built support lacks authored owner"))?;
        if !support_seen.insert(support.support_id.as_str())
            || support.node_index >= input.built.nodes.len()
            || node_ids.get(authored.node.as_str()).copied() != Some(support.node_index)
        {
            return Err(mismatch("support identity/node closure"));
        }
        let owner_index = support_ids.len();
        support_ids.push((support.support_id.clone(), support.node_index));
        identity.name(&support.support_id, budget)?;
        identity.indices([
            support.node_index,
            support.family as usize,
            support.restrained_dofs.len(),
        ]);
        for &dof in &support.restrained_dofs {
            let global = support.node_index * 6 + dof_index(dof);
            identity.indices([global]);
            if support.family == SupportFamily::Spring {
                let stiffness = support
                    .stiffness
                    .as_ref()
                    .ok_or_else(|| mismatch("spring stiffness missing"))?;
                let authored_stiffness = support_stiffness_input(authored)
                    .ok_or_else(|| mismatch("authored spring stiffness missing"))?;
                if parse_dof(&authored_stiffness.dof).ok() != Some(dof)
                    || authored_stiffness.value.value.to_bits() != stiffness.value.to_bits()
                {
                    return Err(mismatch("authored/built spring operand closure"));
                }
                identity.name(&authored_stiffness.dof, budget)?;
                identity.name(&authored_stiffness.value.unit, budget)?;
                expected_springs.push((
                    support.support_id.as_str(),
                    global,
                    stiffness.value.to_bits(),
                ));
            } else {
                if rigid_owner[global].replace(owner_index).is_some() {
                    return Err(unsupported(
                        "ambiguous coincident ideal-constraint ownership",
                    ));
                }
                let imposed = if support.family == SupportFamily::ImposedDisplacement {
                    support
                        .imposed_displacement
                        .as_ref()
                        .ok_or_else(|| mismatch("prescribed value missing"))?
                        .value
                } else {
                    0.0
                };
                expected_prescribed[global] = Some(imposed);
                identity.scalars([imposed]);
            }
        }
    }
    if support_seen.len() != authored_supports.len() {
        return Err(mismatch("authored support not represented"));
    }
    let mut seen_partition = vec![false; n];
    for &(dof, value) in input.prescribed {
        if dof >= n
            || seen_partition[dof]
            || !value.is_finite()
            || expected_prescribed[dof].is_none_or(|v| v.to_bits() != value.to_bits())
        {
            return Err(mismatch(
                "actual prescribed partition differs from owned boundary",
            ));
        }
        seen_partition[dof] = true;
    }
    for &dof in input.free {
        if dof >= n || seen_partition[dof] || expected_prescribed[dof].is_some() {
            return Err(mismatch("actual free partition"));
        }
        seen_partition[dof] = true;
    }
    if seen_partition.iter().any(|v| !*v) {
        return Err(mismatch("incomplete partition"));
    }
    if expected_springs.len() != input.spring_entries.len() {
        return Err(mismatch("spring coverage count"));
    }
    let mut spring_seen = HashSet::new();
    let mut spring_ids = Vec::new();
    let mut springs = Vec::new();
    for (spring, expected) in input.spring_entries.iter().zip(expected_springs) {
        charge(budget, spring.support_id.len().saturating_mul(8))?;
        let dof = spring.node_dof.global_index();
        let k = spring.stiffness.value;
        if dof >= n
            || !k.is_finite()
            || k <= 0.0
            || expected != (spring.support_id.as_str(), dof, k.to_bits())
            || !spring_seen.insert((spring.support_id.as_str(), dof))
        {
            return Err(mismatch("positive distinct ordered spring ownership"));
        }
        identity.name(&spring.support_id, budget)?;
        identity.indices([dof]);
        identity.scalars([k, 0.0]);
        spring_ids.push((spring.support_id.clone(), dof));
        springs.push((dof, k));
    }
    let assembly = AssemblyEvidence::new(
        input.built.nodes.len(),
        &input.built.frame_elements,
        &[],
        &[],
        &springs,
    )
    .map_err(|e| RecoveryError::Exact(exact::Error::Arithmetic(e)))?;
    if !assembly.qualified_passive_family() {
        return Err(unsupported("unqualified passive family"));
    }
    // Actual exact source positivity is established later by Context, not by a
    // geometry count or the rounded ordinary matrix's factorization outcome.
    let mut folded_stiffness = vec![vec![0.0; n]; n];
    for term in &assembly.contributions {
        folded_stiffness[term.row][term.col] += term.value;
    }
    if folded_stiffness
        .iter()
        .flatten()
        .zip(input.stiffness.iter().flatten())
        .any(|(a, b)| !a.is_finite() || a.to_bits() != b.to_bits())
    {
        return Err(mismatch(
            "complete ordered stiffness fold differs from actual aggregate",
        ));
    }
    let mut descriptors = Vec::new();
    let mut member_ids = Vec::new();
    let mut member_seen = HashSet::new();
    for (index, ((pipe, frame), authored)) in input
        .built
        .pipes
        .iter()
        .zip(&input.built.frame_elements)
        .zip(&input.model.pipe_segments)
        .enumerate()
    {
        charge(
            budget,
            pipe.element_id
                .len()
                .saturating_mul(48)
                .saturating_add(authored.id.len().saturating_mul(4)),
        )?;
        if pipe.element_id != authored.id
            || !member_seen.insert(pipe.element_id.as_str())
            || node_ids.get(authored.from.as_str()).copied() != Some(pipe.node_i.index)
            || node_ids.get(authored.to.as_str()).copied() != Some(pipe.node_j.index)
            || pipe.node_i != frame.node_i
            || pipe.node_j != frame.node_j
        {
            return Err(mismatch("member identity/scatter ownership"));
        }
        identity.name(&authored.id, budget)?;
        identity.name(&authored.from, budget)?;
        identity.name(&authored.to, budget)?;
        identity.name(&authored.material, budget)?;
        identity.name(authored.section_ref.as_deref().unwrap_or(""), budget)?;
        identity.quantity(&authored.section.outside_diameter, budget)?;
        identity.quantity(&authored.section.wall_thickness, budget)?;
        for quantity in [
            &authored.section.mill_tolerance,
            &authored.section.material_density,
            &authored.section.contents_density,
            &authored.section.insulation_thickness,
            &authored.section.insulation_density,
        ] {
            identity.indices([usize::from(quantity.is_some())]);
            if let Some(quantity) = quantity {
                identity.quantity(quantity, budget)?;
            }
        }
        identity.scalars([
            frame.section.elastic_modulus,
            frame.section.shear_modulus,
            frame.section.area,
            frame.section.second_moment_y,
            frame.section.second_moment_z,
            frame.section.torsion_constant,
        ]);
        identity.scalars(frame.y_reference);
        let local = frame
            .local_stiffness()
            .map_err(|_| mismatch("frame local matrix"))?;
        let pipe_local = pipe
            .local_stiffness()
            .map_err(|_| mismatch("pipe local matrix"))?;
        let transform = frame
            .orientation()
            .map_err(|_| mismatch("frame orientation"))?
            .transformation_matrix();
        let pipe_transform = pipe
            .frame_element()
            .map_err(|_| mismatch("pipe frame"))?
            .orientation()
            .map_err(|_| mismatch("pipe orientation"))?
            .transformation_matrix();
        if local
            .iter()
            .flatten()
            .zip(pipe_local.iter().flatten())
            .any(|(a, b)| a.to_bits() != b.to_bits())
            || transform
                .iter()
                .flatten()
                .zip(pipe_transform.iter().flatten())
                .any(|(a, b)| a.to_bits() != b.to_bits())
        {
            return Err(mismatch("actual pipe/frame recovery operands differ"));
        }
        let selector = signed_permutation(&transform)?;
        let map = element_dof_map(frame.node_i.index, frame.node_j.index);
        let length = frame.length().map_err(|_| mismatch("member span"))?;
        if !length.is_finite() || length <= 0.0 {
            return Err(mismatch("positive member span"));
        }
        identity.indices([index, index * 144, 144]);
        identity.indices(map);
        identity.scalars(local.iter().flatten().copied());
        identity.scalars(transform.iter().flatten().copied());
        identity.scalars([length]);
        // A signed permutation has exactly one product per global matrix entry:
        // no rounded dot product is used as the source-closure oracle.
        for a in 0..12 {
            for b in 0..12 {
                let (i, si) = selector[a];
                let (j, sj) = selector[b];
                let expected = si * local[a][b] * sj;
                let actual = &assembly.contributions[index * 144 + i * 12 + j];
                if actual.row != map[i]
                    || actual.col != map[j]
                    || actual.value != expected
                    || !expected.is_finite()
                {
                    return Err(mismatch("exact local/transform/scatter assembly closure"));
                }
            }
        }
        let mut rows = Vec::new();
        for row in 0..12 {
            let terms = (0..12)
                .filter(|&p| local[row][p] != 0.0)
                .map(|p| {
                    let (j, sign) = selector[p];
                    // Multiplication by the exact unit selector cannot round.
                    AffineTerm {
                        dof: map[j],
                        products: vec![vec![sign * local[row][p]]],
                    }
                })
                .collect();
            rows.push(FunctionalDescriptor {
                key: key(
                    &input.load_case.id,
                    FunctionalQuantity::MemberEnd {
                        member: pipe.element_id.clone(),
                        end: if row < 6 { MemberEnd::I } else { MemberEnd::J },
                        row: (row % 6) as u8,
                    },
                    row,
                    END_CONVENTION,
                ),
                offset: vec![],
                terms,
            });
        }
        descriptors.extend(rows.iter().cloned());
        for fraction in STATIONS {
            for component in 0..6 {
                let mut row = scaled_row(&rows[component], -1.0);
                // j-side My = -(Mi_y + Vi_z*x), Mz = -(Mi_z - Vi_y*x).
                // Retain fraction and actual span as separate product operands.
                if component == 4 || component == 5 {
                    let shear = if component == 4 { 2 } else { 1 };
                    let sign = if component == 4 { -1.0 } else { 1.0 };
                    append_scaled_products(&mut row, &rows[shear], &[sign, fraction, length]);
                }
                row.key = key(
                    &input.load_case.id,
                    FunctionalQuantity::MemberSection {
                        member: pipe.element_id.clone(),
                        station_bits: fraction.to_bits(),
                        component: component as u8,
                    },
                    component,
                    FunctionalConvention::SectionLocal,
                );
                descriptors.push(row);
            }
        }
        member_ids.push(pipe.element_id.clone());
    }
    let nodal_start = descriptors.len();
    for (node, authored) in input.model.nodes.iter().enumerate() {
        for component in 0..6 {
            let dof = node * 6 + component;
            descriptors.push(FunctionalDescriptor {
                key: FunctionalKey {
                    case_id: input.load_case.id.clone(),
                    quantity: FunctionalQuantity::NodeDisplacement {
                        node: authored.id.clone(),
                        dof,
                    },
                    unit: if component < 3 {
                        FunctionalUnit::Millimetre
                    } else {
                        FunctionalUnit::Radian
                    },
                    convention: FunctionalConvention::DeclaredAffine,
                },
                offset: vec![],
                terms: vec![AffineTerm {
                    dof,
                    products: vec![vec![if component < 3 { 1000.0 } else { 1.0 }]],
                }],
            });
        }
    }
    let spring_start = descriptors.len();
    for spring in input.spring_entries {
        let dof = spring.node_dof.global_index();
        descriptors.push(FunctionalDescriptor {
            key: key(
                &input.load_case.id,
                FunctionalQuantity::GroundSpring {
                    support: spring.support_id.clone(),
                    dof,
                },
                dof,
                FunctionalConvention::SpringOnStructure,
            ),
            offset: vec![],
            terms: vec![AffineTerm {
                dof,
                products: vec![vec![-spring.stiffness.value]],
            }],
        });
    }
    let support_start = descriptors.len();
    for (owner, (id, node)) in support_ids.iter().enumerate() {
        for component in 0..6 {
            let dof = node * 6 + component;
            let mut row = FunctionalDescriptor {
                key: key(
                    &input.load_case.id,
                    FunctionalQuantity::SupportAction {
                        support: id.clone(), node: *node, component: component as u8,
                    },
                    component,
                    FunctionalConvention::SupportOnStructure,
                ),
                offset: vec![],
                terms: vec![],
            };
            if rigid_owner[dof] == Some(owner) {
                for term in assembly
                    .contributions
                    .iter()
                    .filter(|t| t.row == dof && t.value != 0.0)
                {
                    row.terms.push(AffineTerm {
                        dof: term.col,
                        products: vec![vec![term.value]],
                    });
                }
                for force in force_terms
                    .iter()
                    .filter(|f| f.dof == dof && f.value != 0.0)
                {
                    row.offset.push(vec![-force.value]);
                }
            }
            for spring in input
                .spring_entries
                .iter()
                .filter(|s| s.support_id == *id && s.node_dof.global_index() == dof)
            {
                row.terms.push(AffineTerm {
                    dof,
                    products: vec![vec![-spring.stiffness.value]],
                });
            }
            descriptors.push(row);
        }
    }
    let identity = identity.finish(budget)?;
    Ok(Sources {
        assembly,
        force_terms,
        descriptors,
        identity,
        member_ids,
        spring_ids,
        support_ids,
        nodal_start,
        spring_start,
        support_start,
    })
}

fn scaled_row(row: &FunctionalDescriptor, factor: f64) -> FunctionalDescriptor {
    let mut out = FunctionalDescriptor {
        key: row.key.clone(),
        offset: vec![],
        terms: vec![],
    };
    append_scaled_products(&mut out, row, &[factor]);
    out
}
fn append_scaled_products(
    out: &mut FunctionalDescriptor,
    row: &FunctionalDescriptor,
    factors: &[f64],
) {
    for product in &row.offset {
        let mut product = product.clone();
        product.extend_from_slice(factors);
        out.offset.push(product);
    }
    for term in &row.terms {
        let products = term
            .products
            .iter()
            .map(|product| {
                let mut product = product.clone();
                product.extend_from_slice(factors);
                product
            })
            .collect();
        out.terms.push(AffineTerm {
            dof: term.dof,
            products,
        });
    }
}

pub(super) fn solve(
    input: Input<'_>,
    limits: exact::Limits,
) -> Result<SelectedSourceRecovery, RecoveryFailure> {
    let mut budget = AttemptBudget::new(limits);
    let mut stage = "source closure";
    let outcome = (|| {
        let source = prepare_sources(&input, &mut budget)?;
        stage = "exact context";
        let context = exact::Context::prepare_with_budget(
            &source.system(&input),
            &source.identity,
            exact::ForceBasis::IdentifiedContributions(&source.force_terms),
            &mut budget,
        )
        .result?;
        stage = "exact source solve";
        let response = context.solve_with_budget(&mut budget).result?;
        stage = "complete affine plan";
        let plan = FunctionalPlan::new(&context, &source.descriptors, &mut budget).result?;
        stage = "unprojected affine recovery";
        let functionals = response.evaluate_functionals(&plan, &mut budget).result?;
        // No projected displacement participates in any member/device recovery.
        stage = "functional projection";
        let mut functional_projections = Vec::new();
        for index in 0..functionals.len() {
            functional_projections.push(
                functionals
                    .project(index, RELATIVE_LIMIT, &mut budget)
                    .result?,
            );
        }
        stage = "global displacement/reaction projection";
        let mut dof_projections = Vec::new();
        for quantity in [exact::Quantity::Displacement, exact::Quantity::Reaction] {
            for dof in 0..input.force.len() {
                dof_projections.push(
                    response
                        .project_with_budget(quantity, dof, RELATIVE_LIMIT, &mut budget)
                        .result?,
                );
            }
        }
        stage = "retention";
        let retained = functionals
            .retain(&[], &dof_projections, &functional_projections, &mut budget)
            .result?;
        stage = "selected output construction";
        charge(
            &mut budget,
            functional_projections
                .len()
                .saturating_mul(32)
                .saturating_add(dof_projections.len().saturating_mul(16)),
        )?;
        let n = input.force.len();
        let displacements = dof_projections[..n].iter().map(|p| p.value()).collect();
        let reactions = dof_projections[n..].iter().map(|p| p.value()).collect();
        let value = |index: usize| functional_projections[index].value();
        let members = source
            .member_ids
            .iter()
            .enumerate()
            .map(|(index, id)| {
                let start = index * 42;
                MemberRecovery {
                    member_id: id.clone(),
                    end_forces: std::array::from_fn(|i| value(start + i)),
                    end_functional_indices: std::array::from_fn(|i| start + i),
                    sections: std::array::from_fn(|s| {
                        std::array::from_fn(|c| value(start + 12 + s * 6 + c))
                    }),
                    section_functional_indices: std::array::from_fn(|s| {
                        std::array::from_fn(|c| start + 12 + s * 6 + c)
                    }),
                }
            })
            .collect();
        let springs = source
            .spring_ids
            .iter()
            .enumerate()
            .map(|(i, (id, dof))| SpringAction {
                support_id: id.clone(),
                global_dof: *dof,
                value: value(source.spring_start + i),
                functional_index: source.spring_start + i,
            })
            .collect();
        let supports = source
            .support_ids
            .iter()
            .enumerate()
            .map(|(i, (id, node))| SupportActions {
                support_id: id.clone(),
                node_index: *node,
                values: std::array::from_fn(|c| value(source.support_start + i * 6 + c)),
                functional_indices: std::array::from_fn(|c| source.support_start + i * 6 + c),
            })
            .collect();
        let summary = RecoverySummary {
            dofs: n,
            stiffness_contributions: source.assembly.contributions.len(),
            identified_load_contributions: source.force_terms.len(),
            blocks: retained.response().block_witnesses().len(),
            largest_block: retained
                .response()
                .block_witnesses()
                .iter()
                .map(|b| b.dofs().len())
                .max()
                .unwrap_or(0),
            member_end_components: source.member_ids.len() * 12,
            section_components: source.member_ids.len() * 30,
            spring_actions: source.spring_ids.len(),
            support_components: source.support_ids.len() * 6,
            published_nodal_components: n,
            functional_count: functional_projections.len(),
            dof_projection_count: dof_projections.len(),
            max_absolute_projection_error: functional_projections
                .iter()
                .map(|p| p.absolute_error_bound())
                .chain(dof_projections.iter().map(|p| p.absolute_error_bound()))
                .fold(0.0, f64::max),
            max_relative_projection_error: functional_projections
                .iter()
                .map(|p| p.relative_error_bound())
                .chain(dof_projections.iter().map(|p| p.relative_error_bound()))
                .fold(0.0, f64::max),
            relative_limit: RELATIVE_LIMIT,
            work: budget.report(),
        };
        Ok::<_, RecoveryError>((
            displacements,
            reactions,
            (0..n).map(|i| value(source.nodal_start + i)).collect(),
            (0..n).map(|i| source.nodal_start + i).collect(),
            members,
            springs,
            supports,
            retained,
            summary,
        ))
    })();
    match outcome {
        Ok((
            displacements,
            reactions,
            published_nodal,
            published_nodal_indices,
            members,
            springs,
            supports,
            retained,
            summary,
        )) => Ok(SelectedSourceRecovery {
            displacements,
            reactions,
            published_nodal,
            published_nodal_indices,
            members,
            springs,
            supports,
            retained,
            summary,
            budget,
        }),
        Err(error) => Err(failure(stage, error, &budget)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture {
        model: PreviewModel,
        built: BuiltModel,
        stiffness: Vec<Vec<f64>>,
        force: Vec<f64>,
        free: Vec<usize>,
        prescribed: Vec<(usize, f64)>,
        springs: Vec<SpringEntry>,
        loads: LoadApplication,
    }
    impl Fixture {
        fn model() -> PreviewModel {
            serde_json::from_str(include_str!(
                "../../../fixtures/product_preview/numerical_sensitive_torsion_model.json"
            ))
            .unwrap()
        }
        fn new(model: PreviewModel) -> Self {
            let mut diagnostics = Vec::new();
            let built =
                super::super::build_model(&model, &model.materials, &mut diagnostics).unwrap();
            let boundary = super::super::prepare_boundary(built.nodes.len(), &built.supports);
            let springs = boundary.springs;
            let mut stiffness = super::super::assemble_global_stiffness_with_user_elements(
                built.nodes.len(),
                &built.frame_elements,
                &built.user_stiffness_elements,
            )
            .unwrap();
            for spring in &springs {
                let dof = spring.node_dof.global_index();
                stiffness[dof][dof] += spring.stiffness.value;
            }
            let primitives = super::super::build_load_case_primitive_loads(
                &model,
                &model.load_cases[0],
                &mut diagnostics,
            );
            let loads =
                super::super::prepare_loads(built.nodes.len(), built.pipes.len(), &primitives);
            let force = loads.global_load_vector(built.nodes.len());
            let prescribed: Vec<_> = boundary
                .restrained_dofs
                .into_iter()
                .map(|d| (d, 0.0))
                .collect();
            let free = (0..force.len())
                .filter(|d| !prescribed.iter().any(|(p, _)| p == d))
                .collect();
            Self {
                model,
                built,
                stiffness,
                force,
                free,
                prescribed,
                springs,
                loads,
            }
        }
        fn input(&self) -> Input<'_> {
            Input {
                model: &self.model,
                built: &self.built,
                stiffness: &self.stiffness,
                force: &self.force,
                free: &self.free,
                prescribed: &self.prescribed,
                spring_entries: &self.springs,
                load_case: &self.model.load_cases[0],
                load_application: &self.loads,
                thermal_loads: &[],
                pressure_thrust_loads: &[],
            }
        }
    }

    #[test]
    fn source_plan_preserves_absorbed_loads_and_distinct_colocated_springs() {
        let mut model = Fixture::model();
        let template = model.load_cases[0].primitive_loads[0].clone();
        model.load_cases[0].primitive_loads = [1.0e16, 1.0, -1.0e16]
            .into_iter()
            .enumerate()
            .map(|(i, value)| {
                let mut load = template.clone();
                load.id = format!("ordered-tail-{i}");
                load.magnitude.value = value;
                load
            })
            .collect();
        let mut spring = model.supports[1].clone();
        spring.id = "second-independent-spring".into();
        model.supports.push(spring);
        let fixture = Fixture::new(model);
        assert_eq!(fixture.force[9], 0.0);
        let sources = prepare_sources(
            &fixture.input(),
            &mut AttemptBudget::new(exact::Limits::default()),
        )
        .unwrap();
        assert_eq!(
            sources
                .force_terms
                .iter()
                .map(|f| f.value)
                .collect::<Vec<_>>(),
            vec![1.0e16, 1.0, -1.0e16]
        );
        assert_eq!(sources.spring_ids.len(), 2);
        assert_eq!(sources.spring_ids[0].1, sources.spring_ids[1].1);
        assert_ne!(sources.spring_ids[0].0, sources.spring_ids[1].0);
        assert_eq!(sources.descriptors.len(), 42 + 12 + 2 + 18);
    }

    #[test]
    fn omitted_zero_load_producer_and_source_mutations_are_rejected() {
        let mut fixture = Fixture::new(Fixture::model());
        fixture.model.load_cases[0].primitive_loads[0].dimension = "temperature_change".into();
        fixture.model.load_cases[0].primitive_loads[0].category = "thermal".into();
        fixture.model.load_cases[0].primitive_loads[0]
            .magnitude
            .value = 0.0;
        assert!(matches!(
            prepare_sources(
                &fixture.input(),
                &mut AttemptBudget::new(exact::Limits::default())
            ),
            Err(RecoveryError::Unsupported(_))
        ));
        let mut fixture = Fixture::new(Fixture::model());
        fixture.loads.nodal_loads[0].value = -fixture.loads.nodal_loads[0].value;
        assert!(matches!(
            prepare_sources(
                &fixture.input(),
                &mut AttemptBudget::new(exact::Limits::default())
            ),
            Err(RecoveryError::SourceMismatch(_))
        ));
        let mut fixture = Fixture::new(Fixture::model());
        fixture.stiffness[3][3] = fixture.stiffness[3][3] + 1.0;
        assert!(matches!(
            prepare_sources(
                &fixture.input(),
                &mut AttemptBudget::new(exact::Limits::default())
            ),
            Err(RecoveryError::SourceMismatch(_))
        ));
    }

    #[test]
    fn coincident_ideal_ownership_is_not_copied_to_multiple_supports() {
        let mut model = Fixture::model();
        let mut duplicate = model.supports[0].clone();
        duplicate.id = "ambiguous-anchor".into();
        model.supports.push(duplicate);
        let fixture = Fixture::new(model);
        assert!(matches!(
            prepare_sources(
                &fixture.input(),
                &mut AttemptBudget::new(exact::Limits::default())
            ),
            Err(RecoveryError::Unsupported(
                "ambiguous coincident ideal-constraint ownership"
            ))
        ));
    }

    #[test]
    fn selection_failure_preserves_denied_whole_attempt_work() {
        let fixture = Fixture::new(Fixture::model());
        let failure = solve(
            fixture.input(),
            exact::Limits {
                operations: 0,
                ..exact::Limits::default()
            },
        )
        .unwrap_err();
        assert_eq!(failure.stage, "source closure");
        assert_eq!(failure.work.charged, 0);
        assert!(failure.work.rejected > 0);
        assert_eq!(failure.error, RecoveryError::Exact(exact::Error::Budget));
    }

    #[test]
    fn signed_permutation_uses_all_actual_entries_without_tolerance() {
        let mut t = [[0.0; 12]; 12];
        for i in 0..12 {
            t[i][11 - i] = if i % 2 == 0 { -1.0 } else { 1.0 };
        }
        assert!(signed_permutation(&t).is_ok());
        t[0][0] = f64::MIN_POSITIVE;
        assert!(signed_permutation(&t).is_err());
        t[0][0] = 0.0;
        t[0][11] = 1.0 - f64::EPSILON;
        assert!(signed_permutation(&t).is_err());
    }
}
