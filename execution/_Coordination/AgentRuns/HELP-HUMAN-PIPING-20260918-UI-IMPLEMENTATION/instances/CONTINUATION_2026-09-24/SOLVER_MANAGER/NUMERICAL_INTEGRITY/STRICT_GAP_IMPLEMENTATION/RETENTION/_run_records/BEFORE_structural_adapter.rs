//! Caller-side provenance for the M03 passive structural gate.
//! Formation allowances are arithmetic estimates, never physical accuracy proofs.
use crate::{CurvedBendStiffnessElement, LinearSolveMode};
use open_pipe_stress_frame_kernel::rigid_body::{
    assess_rigid_body, ObjectiveFamily, RigidBodyStatus,
};
use open_pipe_stress_frame_kernel::structural::{
    self, StiffnessContribution, StructuralError, StructuralSolution, StructuralSystem,
    SymmetryEvidence,
};
use open_pipe_stress_frame_kernel::{
    element_dof_map, FrameElement, Matrix12, UserStiffnessElement,
};

#[derive(Debug, Clone)]
pub struct AssemblyEvidence {
    pub contributions: Vec<StiffnessContribution>,
    pub absolute_roundoff: Vec<Vec<f64>>,
    pub operation_counts: Vec<Vec<usize>>,
    coordinates: Vec<Option<[f64; 3]>>,
    edges: Vec<(usize, usize, bool)>,
    spring_ground: Vec<usize>,
}
impl AssemblyEvidence {
    pub fn new(
        node_count: usize,
        frames: &[FrameElement],
        users: &[UserStiffnessElement],
        curved: &[CurvedBendStiffnessElement],
        springs: &[(usize, f64)],
    ) -> Result<Self, StructuralError> {
        let n = node_count * 6;
        let mut result = Self {
            contributions: Vec::new(),
            absolute_roundoff: vec![vec![0.0; n]; n],
            operation_counts: vec![vec![0; n]; n],
            coordinates: vec![None; node_count],
            edges: Vec::new(),
            spring_ground: Vec::new(),
        };
        for element in frames {
            let local = element
                .local_stiffness()
                .map_err(|_| StructuralError::InvalidInput("frame local stiffness"))?;
            let t = element
                .orientation()
                .map_err(|_| StructuralError::InvalidInput("frame orientation"))?
                .transformation_matrix();
            let evidence = structural::transform_roundoff(&local, &t)?;
            result.node(element.node_i.index, element.node_i.coordinates)?;
            result.node(element.node_j.index, element.node_j.coordinates)?;
            result.element(
                element.node_i.index,
                element.node_j.index,
                &element
                    .global_stiffness()
                    .map_err(|_| StructuralError::InvalidInput("frame stiffness"))?,
                &evidence.absolute_roundoff,
                &evidence.operation_counts,
                true,
            )?;
        }
        for element in users {
            let local = element.local_stiffness();
            let t = element
                .orientation()
                .map_err(|_| StructuralError::InvalidInput("user orientation"))?
                .transformation_matrix();
            let evidence = structural::transform_roundoff(&local, &t)?;
            result.node(element.node_i.index, element.node_i.coordinates)?;
            result.node(element.node_j.index, element.node_j.coordinates)?;
            // Relative translation/rotation springs are not objective frame energy.
            result.element(
                element.node_i.index,
                element.node_j.index,
                &element
                    .global_stiffness()
                    .map_err(|_| StructuralError::InvalidInput("user stiffness"))?,
                &evidence.absolute_roundoff,
                &evidence.operation_counts,
                false,
            )?;
        }
        for element in curved {
            let zero = [[0.0; 12]; 12];
            let counts = [[0; 12]; 12];
            let (bounds, operations) = element
                .symmetry_formation
                .as_ref()
                .map(|e| (&e.0, &e.1))
                .unwrap_or((&zero, &counts));
            // Explicit slots have no silently inferred objective/nullspace contract.
            result.element(
                element.node_i,
                element.node_j,
                &element.global_stiffness,
                bounds,
                operations,
                false,
            )?;
        }
        for &(dof, value) in springs {
            if dof >= n || !value.is_finite() || value < 0.0 {
                return Err(StructuralError::InvalidInput("ground spring"));
            }
            result.contributions.push(StiffnessContribution {
                row: dof,
                col: dof,
                value,
            });
            result.operation_counts[dof][dof] += 1;
            if value > 0.0 {
                result.spring_ground.push(dof);
            }
        }
        // Sequential scatter adds are independent of transformation formation.
        let mut magnitudes = vec![vec![0.0; n]; n];
        let mut scatter_counts = vec![vec![0usize; n]; n];
        for c in &result.contributions {
            magnitudes[c.row][c.col] += c.value.abs();
            scatter_counts[c.row][c.col] += 1;
        }
        for i in 0..n {
            for j in 0..n {
                result.absolute_roundoff[i][j] +=
                    structural::gamma(scatter_counts[i][j]) * magnitudes[i][j];
                if !result.absolute_roundoff[i][j].is_finite() {
                    return Err(StructuralError::Range("assembly allowance"));
                }
            }
        }
        Ok(result)
    }
    fn node(&mut self, index: usize, point: [f64; 3]) -> Result<(), StructuralError> {
        let slot = self
            .coordinates
            .get_mut(index)
            .ok_or(StructuralError::InvalidInput("node index"))?;
        if slot.is_some_and(|old| old != point) {
            return Err(StructuralError::InvalidInput(
                "inconsistent node coordinates",
            ));
        }
        *slot = Some(point);
        Ok(())
    }
    fn element(
        &mut self,
        a: usize,
        b: usize,
        k: &Matrix12,
        bounds: &Matrix12,
        counts: &[[usize; 12]; 12],
        objective: bool,
    ) -> Result<(), StructuralError> {
        if a >= self.coordinates.len() || b >= self.coordinates.len() {
            return Err(StructuralError::InvalidInput("element endpoint"));
        }
        self.edges.push((a, b, objective));
        let map = element_dof_map(a, b);
        for i in 0..12 {
            for j in 0..12 {
                self.contributions.push(StiffnessContribution {
                    row: map[i],
                    col: map[j],
                    value: k[i][j],
                });
                self.absolute_roundoff[map[i]][map[j]] += bounds[i][j];
                self.operation_counts[map[i]][map[j]] += counts[i][j] + 1;
            }
        }
        Ok(())
    }
    /// Screen each actual connected body using only selected ground constraints.
    pub fn qualified_passive_family(&self) -> bool {
        !self.edges.is_empty() && self.edges.iter().all(|(_, _, qualified)| *qualified)
    }

    pub fn geometry(&self, prescribed: &[(usize, f64)]) -> Result<(), StructuralError> {
        let n = self.coordinates.len();
        let mut seen = vec![false; n];
        for seed in 0..n {
            if seen[seed] {
                continue;
            }
            let mut body = vec![seed];
            seen[seed] = true;
            let mut index = 0;
            while index < body.len() {
                let node = body[index];
                index += 1;
                for &(a, b, _) in &self.edges {
                    let other = if a == node {
                        Some(b)
                    } else if b == node {
                        Some(a)
                    } else {
                        None
                    };
                    if let Some(other) = other {
                        if !seen[other] {
                            seen[other] = true;
                            body.push(other);
                        }
                    }
                }
            }
            let qualified = self
                .edges
                .iter()
                .filter(|(a, _, _)| body.contains(a))
                .all(|(_, _, q)| *q);
            // An unqualified family is still checked by the matrix gate, but cannot
            // turn a geometric null vector into a physical mechanism assertion.
            if !qualified || body.iter().any(|&i| self.coordinates[i].is_none()) {
                continue;
            }
            let coordinates = body
                .iter()
                .map(|&i| self.coordinates[i].unwrap())
                .collect::<Vec<_>>();
            let ground = prescribed
                .iter()
                .map(|&(d, _)| d)
                .chain(self.spring_ground.iter().copied())
                .filter_map(|d| body.iter().position(|&i| i == d / 6).map(|i| 6 * i + d % 6))
                .collect::<Vec<_>>();
            let assessment = assess_rigid_body(
                &coordinates,
                &ground,
                ObjectiveFamily::WeldedUnreleasedElasticFrames,
            )?;
            match assessment.status {
                RigidBodyStatus::Restrained => {}
                RigidBodyStatus::MechanismWitnessed => {
                    let mut direction = vec![0.0; 6 * n];
                    for (&global, motion) in body.iter().zip(assessment.node_motion.unwrap()) {
                        direction[6 * global..6 * global + 6].copy_from_slice(&motion);
                    }
                    return Err(StructuralError::Mechanism { direction });
                }
                _ => {
                    return Err(StructuralError::NumericallyUnresolved {
                        reason: "rigid-restraint rank unresolved",
                        global_dof: None,
                    })
                }
            }
        }
        Ok(())
    }
    pub fn solve(
        &self,
        k: &[Vec<f64>],
        f: &[f64],
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<StructuralSolution, StructuralError> {
        self.geometry(prescribed)?;
        let family_basis = if self.edges.iter().all(|(_, _, qualified)| *qualified) {
            "objective welded unreleased straight-frame family"
        } else {
            "mixed or explicit-matrix family: physical rigid-null witness unqualified for bodies containing user/curved elements; matrix positivity remains mandatory"
        };
        let symmetry_basis = format!("represented local matrices; two-stage 12-term frame transforms plus directed scatter; curved H*K and (H*K)*H^T six-term stages when traced; inverse accuracy not claimed; {family_basis}");
        let system = StructuralSystem {
            stiffness: k,
            force: f,
            free_dofs: free,
            prescribed,
            contributions: Some(&self.contributions),
            symmetry: Some(SymmetryEvidence {
                absolute_roundoff: &self.absolute_roundoff,
                operation_counts: &self.operation_counts,
                basis: &symmetry_basis,
            }),
        };
        match mode {
            LinearSolveMode::DenseScrutiny => structural::solve_structural_dense(&system),
            LinearSolveMode::SparseInteractive => {
                open_pipe_stress_sparse_direct::structural::solve_structural_sparse(&system)
            }
        }
    }
}

/// Trace the curved source's *symmetry* formation: the explicitly symmetrized
/// represented tip inverse, H*K and (H*K)*H^T six-term dots, then T^T*K*T.
/// This does not bound inverse accuracy or qualify the curved physical nullspace.
pub fn curved_formation(
    element: &open_pipe_stress_curved_bend::CurvedBendMacroElement,
) -> Result<(Matrix12, [[usize; 12]; 12]), crate::NonlinearIntegrationError> {
    let invalid = |error: open_pipe_stress_curved_bend::CurvedBendError| {
        crate::NonlinearIntegrationError::InvalidInput {
            detail: format!("curved symmetry formation: {error}"),
        }
    };
    let local = element.local_stiffness().map_err(invalid)?;
    let t = element
        .orientation()
        .map_err(invalid)?
        .transformation_matrix();
    let geometry = element.geometry().map_err(invalid)?;
    let chord = [
        geometry.radius * (geometry.included_angle.cos() - 1.0),
        geometry.radius * geometry.included_angle.sin(),
        0.0,
    ];
    let mut h = [[0.0; 6]; 6];
    for i in 0..6 {
        h[i][i] = 1.0;
    }
    h[3][1] = -chord[2];
    h[3][2] = chord[1];
    h[4][0] = chord[2];
    h[4][2] = -chord[0];
    h[5][0] = -chord[1];
    h[5][1] = chord[0];
    let mut coupled = [[0.0; 6]; 6];
    let mut magnitude = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                let term = structural::checked_product(h[i][k], local[k + 6][j + 6])?;
                coupled[i][j] = structural::checked_value(coupled[i][j] + term)?;
                magnitude[i][j] = structural::checked_value(magnitude[i][j] + term.abs())?;
            }
        }
    }
    let g = structural::gamma(12);
    let mut local_bounds = [[0.0; 12]; 12];
    for i in 0..6 {
        for j in 0..6 {
            let first = structural::checked_quotient(
                structural::checked_product(g, magnitude[i][j])?,
                1.0 - g,
            )?;
            local_bounds[i][j + 6] = first;
            local_bounds[j + 6][i] = first;
            let mut inherited = 0.0;
            let mut second = 0.0;
            for k in 0..6 {
                let first_bound = structural::checked_quotient(
                    structural::checked_product(g, magnitude[i][k])?,
                    1.0 - g,
                )?;
                inherited = structural::checked_value(
                    inherited + structural::checked_product(h[j][k].abs(), first_bound)?,
                )?;
                second = structural::checked_value(
                    second + structural::checked_product(coupled[i][k], h[j][k])?.abs(),
                )?;
            }
            local_bounds[i][j] = structural::checked_product(
                structural::checked_value(inherited + structural::checked_product(g, second)?)?,
                1.0 + 64.0 * f64::EPSILON,
            )?;
        }
    }
    let mut bounds = structural::transform_roundoff(&local, &t)?.absolute_roundoff;
    for i in 0..12 {
        for j in 0..12 {
            let mut inherited = 0.0;
            for a in 0..12 {
                for b in 0..12 {
                    let term = structural::checked_product(
                        structural::checked_product(t[a][i].abs(), local_bounds[a][b])?,
                        t[b][j].abs(),
                    )?;
                    inherited = structural::checked_value(inherited + term)?;
                }
            }
            bounds[i][j] = structural::checked_value(
                bounds[i][j] + inherited / (1.0 - structural::gamma(432)),
            )?;
        }
    }
    // Longest formation-error path contributing to each global entry:
    // local anchor block 24, cross block 12, tip block 0; global stages 48.
    let counts = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            let mut longest = 48;
            for a in 0..12 {
                for b in 0..12 {
                    if t[a][i] != 0.0 && t[b][j] != 0.0 {
                        let local_count = if a < 6 && b < 6 {
                            24
                        } else if a < 6 || b < 6 {
                            12
                        } else {
                            0
                        };
                        longest = longest.max(48 + local_count);
                    }
                }
            }
            longest
        })
    });
    Ok((bounds, counts))
}

/// A first inactive-contact seed may lack rank before its bounded all-active
/// trial. This permits initialization only; the trial and final state still
/// pass the full gate. Assembly loss, skew, range and negative energy cannot
/// be rescued by changing contact state.
pub fn permits_contact_seed_trial(error: &StructuralError, qualified_passive_family: bool) -> bool {
    match error {
        StructuralError::Mechanism { .. } => true,
        StructuralError::NumericallyUnresolved { reason, .. } if qualified_passive_family => {
            matches!(
                *reason,
                "rigid-restraint rank unresolved"
                    | "zero original diagonal; no physical nullity inferred"
                    | "nonpositive or cancellation-unresolved structural pivot"
                    | "scaled condition estimate at working-precision boundary"
            )
        }
        _ => false,
    }
}

// Bounded strict-gap integration. All values/projections below refer to the
// declared represented contribution equations, not unrecorded primitive loads.
use open_pipe_stress_frame_kernel::structural::exact_boundary as exact;
#[derive(Debug, Clone, PartialEq)]
pub enum StrictGapEvidence {
    NotApplicable,
    Qualified(ExactGapReport),
    Unsupported { reason: String, work: ExactGapWork },
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExactGapWork {
    pub preparation_and_solve_charged: Option<usize>,
    pub charged: usize,
    pub rejected: usize,
    pub reserved_for_unobserved_failure: usize,
    pub attempts: Vec<(String, exact::WorkReport, Option<String>)>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GapDecision {
    pub support_id: String,
    pub global_dof: usize,
    pub sense: i8,
    pub gap: f64,
    pub prior: open_pipe_stress_nonlinear_supports::ActiveSetState,
    pub gap_sign: exact::Sign,
    pub reaction_sign: exact::Sign,
    pub state: open_pipe_stress_nonlinear_supports::ActiveSetState,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GapProjection {
    pub quantity: exact::Quantity,
    pub global_dof: usize,
    pub value: f64,
    pub interval: [f64; 2],
    pub absolute_error_bound: f64,
    pub relative_error_bound: f64,
    pub criterion: f64,
    pub basis: exact::ProjectionBasis,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExactGapReport {
    pub source_identity: String,
    pub source_level: &'static str,
    pub free_dofs: Vec<usize>,
    pub prescribed: Vec<(usize, f64)>,
    pub decisions: Vec<GapDecision>,
    pub projections: Vec<GapProjection>,
    pub ordinary_displacements: Vec<f64>,
    pub corrected_coordinate_count: usize,
    pub work: ExactGapWork,
}
const GAP_PROJECTION_RELATIVE_LIMIT: f64 = 1e-9;
pub(crate) const GAP_RUN_WORK_LIMIT: usize = 8_000_000;
fn gap_attempt<T>(
    attempt: exact::Attempt<T>,
    work: &mut ExactGapWork,
    label: String,
) -> Result<T, exact::Error> {
    work.charged = work.charged.saturating_add(attempt.work.charged);
    work.rejected = work.rejected.saturating_add(attempt.work.rejected);
    work.attempts.push((
        label,
        attempt.work,
        attempt.result.as_ref().err().map(|e| format!("{e:?}")),
    ));
    attempt.result
}
/// No extra contact-state solve occurs here. Exact ratios correct/project the
/// response of this iteration's ORIGINAL partition, then ordinary counting runs.
pub(crate) fn scrutinize_gaps(
    input: &crate::NonlinearFrameSolveInput,
    assembly: &AssemblyEvidence,
    stiffness: &[Vec<f64>],
    boundary: &crate::BoundaryState,
    prior: &[open_pipe_stress_nonlinear_supports::SupportStateRecord],
    iteration: usize,
    mode: crate::LinearSolveMode,
    solve: &mut crate::SlidingIterationSolve,
    remaining_run_work: &mut usize,
) -> StrictGapEvidence {
    use open_pipe_stress_nonlinear_supports::{
        ActiveSetState, GapDirection, NonlinearSupportBehavior,
    };
    let has_gap = input
        .nonlinear_supports
        .iter()
        .any(|s| matches!(s.behavior, NonlinearSupportBehavior::Gap { .. }));
    if !has_gap {
        return StrictGapEvidence::NotApplicable;
    }
    let mut work = ExactGapWork::default();
    let unsupported =
        |reason: String, work: ExactGapWork| StrictGapEvidence::Unsupported { reason, work };
    if !input
        .nonlinear_supports
        .iter()
        .all(|s| matches!(s.behavior, NonlinearSupportBehavior::Gap { .. }))
        || !input.user_stiffness_elements.is_empty()
        || !input.curved_bend_elements.is_empty()
        || !assembly.qualified_passive_family()
        || !input.friction_normal_reactions.is_empty()
        || !input.derived_friction_normal_reactions.is_empty()
        || !solve.applied_forces.is_empty()
    {
        return unsupported("strict-gap proof supports only gap-only objective straight-frame bodies with declared nodal loads and linear springs; mixed/curved/user/affine source capability remains unqualified".into(),work);
    }
    let mut gap_dofs = std::collections::HashSet::new();
    if input
        .nonlinear_supports
        .iter()
        .any(|support| !gap_dofs.insert(crate::node_dof_index(support.node_index, support.dof)))
    {
        return unsupported(
            "duplicate gap DOFs cannot acquire exact-law qualification".into(),
            work,
        );
    }
    let per_call = exact::Limits::default().operations.min(*remaining_run_work);
    if per_call == 0 {
        return unsupported("strict-gap undertaking work budget exhausted".into(), work);
    }
    let outcome = (|| -> Result<ExactGapReport, exact::Error> {
        let prescribed = boundary
            .dofs
            .iter()
            .copied()
            .zip(boundary.displacements.iter().copied())
            .collect::<Vec<_>>();
        let free = (0..input.force.len())
            .filter(|d| !boundary.dofs.contains(d))
            .collect::<Vec<_>>();
        let mut force = input.force.clone();
        crate::add_applied_forces(&mut force, &solve.applied_forces);
        let mut terms = input
            .force
            .iter()
            .enumerate()
            .map(|(dof, &value)| exact::ForceContribution {
                source: format!("declared input.force[{dof}]"),
                dof,
                value,
            })
            .collect::<Vec<_>>();
        for applied in &solve.applied_forces {
            terms.push(exact::ForceContribution {
                source: format!("selected affine {}", applied.support_id),
                dof: applied.global_dof,
                value: applied.force,
            });
        }
        let system = StructuralSystem {
            stiffness,
            force: &force,
            free_dofs: &free,
            prescribed: &prescribed,
            contributions: Some(&assembly.contributions),
            symmetry: Some(SymmetryEvidence {
                absolute_roundoff: &assembly.absolute_roundoff,
                operation_counts: &assembly.operation_counts,
                basis: "original represented assembly, no symmetry projection in exact proof",
            }),
        };
        let identity=format!("nonlinear original partition iteration={iteration}; mode={}; declared-force-plus-selected-affine-source",mode.as_str());
        let context = exact::Context::new(
            &system,
            &identity,
            exact::ForceBasis::IdentifiedContributions(&terms),
            exact::Limits {
                operations: per_call,
                ..exact::Limits::default()
            },
        )?;
        if !context.matches(
            &system,
            &identity,
            exact::ForceBasis::IdentifiedContributions(&terms),
        ) {
            return Err(exact::Error::Invalid("exact source binding"));
        }
        let response = context.solve()?;
        work.preparation_and_solve_charged = Some(response.operations());
        work.charged = response.operations();
        let mut decisions = Vec::new();
        for support in &input.nonlinear_supports {
            let state = prior
                .iter()
                .find(|s| s.support_id == support.support_id)
                .ok_or(exact::Error::Invalid("missing prior gap state"))?
                .state;
            if !matches!(state, ActiveSetState::Active | ActiveSetState::Inactive) {
                return Err(exact::Error::Invalid("unsupported gap state"));
            }
            let sense = match support.behavior {
                NonlinearSupportBehavior::Gap {
                    closes_when: GapDirection::PositiveDisplacement,
                } => 1,
                NonlinearSupportBehavior::Gap {
                    closes_when: GapDirection::NegativeDisplacement,
                } => -1,
                _ => return Err(exact::Error::Invalid("gap source family")),
            };
            let gap = support.gap.ok_or(exact::Error::Invalid("missing gap"))?;
            let dof = crate::node_dof_index(support.node_index, support.dof);
            let sign = gap_attempt(
                response.gap_sign_with_work(dof, sense, gap, per_call.saturating_sub(work.charged)),
                &mut work,
                format!("gap sign {}", support.support_id),
            )?;
            let reaction = gap_attempt(
                response.signed_reaction_with_work(
                    dof,
                    sense,
                    per_call.saturating_sub(work.charged),
                ),
                &mut work,
                format!("reaction sign {}", support.support_id),
            )?;
            let active = if state == ActiveSetState::Active {
                reaction != exact::Sign::Positive
            } else {
                sign != exact::Sign::Negative
            };
            decisions.push(GapDecision {
                support_id: support.support_id.clone(),
                global_dof: dof,
                sense,
                gap,
                prior: state,
                gap_sign: sign,
                reaction_sign: reaction,
                state: if active {
                    ActiveSetState::Active
                } else {
                    ActiveSetState::Inactive
                },
            });
        }
        let mut u = Vec::with_capacity(input.force.len());
        let mut reactions = Vec::with_capacity(input.force.len());
        let mut projections = Vec::new();
        for dof in 0..input.force.len() {
            for quantity in [exact::Quantity::Displacement, exact::Quantity::Reaction] {
                let remaining = per_call.saturating_sub(work.charged);
                let attempt = match quantity {
                    exact::Quantity::Displacement => {
                        response.project_displacement(dof, GAP_PROJECTION_RELATIVE_LIMIT, remaining)
                    }
                    exact::Quantity::Reaction => {
                        response.project_reaction(dof, GAP_PROJECTION_RELATIVE_LIMIT, remaining)
                    }
                };
                let p = gap_attempt(
                    attempt,
                    &mut work,
                    format!("projection {quantity:?} dof={dof}"),
                )?;
                if !p.is_for(&response, quantity, dof)
                    || p.relative_limit() != GAP_PROJECTION_RELATIVE_LIMIT
                    || p.relative_error_bound() > GAP_PROJECTION_RELATIVE_LIMIT
                {
                    return Err(exact::Error::ProjectionUnresolved(
                        "projection identity/criterion",
                    ));
                }
                match quantity {
                    exact::Quantity::Displacement => u.push(p.value()),
                    exact::Quantity::Reaction => reactions.push(p.value()),
                };
                projections.push(GapProjection {
                    quantity,
                    global_dof: dof,
                    value: p.value(),
                    interval: p.interval(),
                    absolute_error_bound: p.absolute_error_bound(),
                    relative_error_bound: p.relative_error_bound(),
                    criterion: p.relative_limit(),
                    basis: p.basis(),
                });
            }
        }
        let equilibrium = crate::product_equilibrium::evaluate(&system, &u)?;
        if !equilibrium.passed {
            return Err(exact::Error::ProjectionUnresolved(
                "projected exact response fails original represented equilibrium gate",
            ));
        }
        let ordinary = solve.linearized.displacements.clone();
        let corrected = ordinary
            .iter()
            .zip(&u)
            .filter(|(a, b)| a.to_bits() != b.to_bits())
            .count();
        solve.linearized.displacements = u;
        // Recover action from the SAME exact ratios, never from rounded u.
        solve.linearized.reactions = reactions.clone();
        solve.reactions = reactions;
        solve.linearized.max_abs_free_dof_force_residual = equilibrium
            .rows
            .iter()
            .filter(|r| r.global_dof % 6 < 3)
            .map(|r| r.residual.abs())
            .fold(0.0, f64::max);
        solve.linearized.max_abs_free_dof_moment_residual = equilibrium
            .rows
            .iter()
            .filter(|r| r.global_dof % 6 >= 3)
            .map(|r| r.residual.abs())
            .fold(0.0, f64::max);
        solve.linearized.max_abs_free_dof_work_residual = equilibrium
            .work_rows
            .iter()
            .map(|r| r.observed_work)
            .fold(0.0, f64::max);
        solve.linearized.product_equilibrium = equilibrium;
        Ok(ExactGapReport{source_identity:identity,source_level:"represented frame/spring contributions; declared aggregate nodal load entries; exact original Kfc and prescribed values",free_dofs:free,prescribed,decisions,projections,ordinary_displacements:ordinary,corrected_coordinate_count:corrected,work:work.clone()})
    })();
    // The helper exposes metered attempts and successful preparation/solve;
    // failed preparation/solve has no work return. Reserve that whole cap so
    // unknown consumed work cannot disappear from the undertaking budget.
    let debit = if work.preparation_and_solve_charged.is_some() {
        work.charged
    } else {
        work.reserved_for_unobserved_failure = per_call;
        per_call
    };
    *remaining_run_work = remaining_run_work.saturating_sub(debit);
    match outcome {Ok(report)=>StrictGapEvidence::Qualified(report),Err(error)=>unsupported(format!("strict-gap exact source/predicate/projection unresolved: {error:?}; unavailable preparation accounting reserves full {per_call} when applicable"),work)}
}

pub(crate) fn exact_gap_iteration(
    input: &crate::ActiveSetIterationInput,
    report: &ExactGapReport,
) -> Result<crate::ActiveSetIteration, crate::NonlinearIntegrationError> {
    use open_pipe_stress_nonlinear_supports::{ActiveSetState, SupportStateRecord};
    if report.decisions.len() != input.supports.len() {
        return Err(crate::NonlinearIntegrationError::InvalidInput {
            detail: "incomplete exact gap decision coverage".into(),
        });
    }
    let mut states = Vec::new();
    let mut changed = Vec::new();
    let mut ids = std::collections::HashSet::new();
    for support in &input.supports {
        let matches = report
            .decisions
            .iter()
            .filter(|d| d.support_id == support.support_id)
            .collect::<Vec<_>>();
        if matches.len() != 1 || !ids.insert(support.support_id.clone()) {
            return Err(crate::NonlinearIntegrationError::InvalidInput {
                detail: "duplicate or missing exact gap decision".into(),
            });
        }
        let decision = matches[0];
        let prior = input
            .prior_states
            .iter()
            .find(|p| p.support_id == support.support_id)
            .map(|p| p.state);
        if prior != Some(decision.prior)
            || !matches!(
                decision.state,
                ActiveSetState::Active | ActiveSetState::Inactive
            )
        {
            return Err(crate::NonlinearIntegrationError::InvalidInput {
                detail: "exact gap prior-state mismatch".into(),
            });
        }
        if decision.state != decision.prior {
            changed.push(support.support_id.clone());
        }
        states.push(SupportStateRecord::new(&support.support_id, decision.state));
    }
    states.sort_by(|a, b| a.support_id.cmp(&b.support_id));
    changed.sort();
    let residual = changed.len() as f64;
    let diagnostic = open_pipe_stress_solver_diagnostics::convergence_diagnostic(
        input.iteration,
        input.max_iterations,
        residual,
        input.tolerance,
    )
    .map_err(|e| crate::NonlinearIntegrationError::InvalidInput {
        detail: format!("exact gap count diagnostic: {e}"),
    })?;
    let mut diagnostics = Vec::new();
    if let Some(mut d) = diagnostic {
        d.message.push_str(&format!(
            "; exact gap decisions; changed_supports={changed:?}; states={states:?}"
        ));
        diagnostics.push(d);
    }
    Ok(crate::ActiveSetIteration {
        iteration: input.iteration,
        states,
        changed_supports: changed,
        residual_norm: residual,
        converged: residual <= input.tolerance,
        diagnostics,
    })
}
