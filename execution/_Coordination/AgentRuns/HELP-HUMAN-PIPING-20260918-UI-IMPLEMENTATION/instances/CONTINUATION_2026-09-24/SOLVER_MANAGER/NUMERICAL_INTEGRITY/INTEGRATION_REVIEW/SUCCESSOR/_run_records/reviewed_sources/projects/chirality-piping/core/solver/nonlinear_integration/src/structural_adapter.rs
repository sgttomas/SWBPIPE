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
