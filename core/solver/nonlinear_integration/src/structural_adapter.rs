//! Caller-side provenance for the M03 passive structural gate.
//! Formation allowances are arithmetic estimates, never physical accuracy proofs.
use crate::{CurvedBendStiffnessElement, LinearSolveMode};
use open_pipe_stress_curved_bend::CurvedBendMacroElement;
use open_pipe_stress_frame_kernel::load_ledger::{AssembledForce, ForceTerm};
use open_pipe_stress_frame_kernel::rigid_body::{
    assess_constrained_bodies, assess_rigid_body, objective_sub_bodies, user_element_tie,
    ConstrainedAssessment, ConstrainedGround, ObjectiveFamily, RigidBodyStatus, TieRefusal,
};
use open_pipe_stress_frame_kernel::structural::{
    self, assemble_sparse_stiffness, CurvedFormation, ForceScaleReason, ForceScaledError,
    ForceScaledSolution, ForceScalingRefusal, FormationSource, RangeTrigger, SparseAssemblyOptions,
    SparsePattern, SparseStiffness, SparseStructuralSystem, SparseSymmetryEvidence, StiffnessBlock,
    StiffnessContribution, StructuralError, StructuralSolution, StructuralSystem, SymmetryEvidence,
};
use open_pipe_stress_frame_kernel::{
    element_dof_map, force_scaled_matrix, force_scaled_value, ForceScale, ForceScaleCensus,
    FrameElement, FrameKernelError, Matrix12, UserStiffnessElement,
};
use open_pipe_stress_sparse_direct::structural::solve_sparse_prepared;

#[derive(Debug, Clone)]
pub struct AssemblyEvidence {
    pub contributions: Vec<StiffnessContribution>,
    pub absolute_roundoff: Vec<Vec<f64>>,
    pub operation_counts: Vec<Vec<usize>>,
    coordinates: Vec<Option<[f64; 3]>>,
    edges: Vec<(usize, usize, bool)>,
    spring_ground: Vec<usize>,
    /// Identified ledger terms for the load-fidelity audit (S11-K; dormant).
    force_terms: Option<Vec<ForceTerm>>,
    /// K-D5: the primitives the D-5 formation check re-forms from.
    formation: FormationPrimitives,
    /// K2b (D1 §4.7): the force scale 2^b the evidence was formed at;
    /// `UNSCALED` for `new`. Never part of `StructuralReport`.
    force_scale: ForceScale,
}

/// The formation source recorded by `AssemblyEvidence::new` (K-D5): the
/// frame, user and spring primitives as supplied, and for each curved slot
/// what `solve_assembled_with_formation_check` needs to match it to its macro
/// element.
#[derive(Debug, Clone, Default)]
struct FormationPrimitives {
    node_count: usize,
    frames: Vec<FrameElement>,
    users: Vec<UserStiffnessElement>,
    curved: Vec<CurvedSlot>,
    springs: Vec<(usize, f64)>,
}

#[derive(Debug, Clone)]
struct CurvedSlot {
    element_id: String,
    node_i: usize,
    node_j: usize,
    global_stiffness: Matrix12,
    /// Built by `CurvedBendStiffnessElement::new` (an explicit matrix with no
    /// traced macro-element source): never re-formable.
    explicit: bool,
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
        let parts = EvidenceParts::new(
            node_count,
            frames,
            users,
            curved,
            springs,
            DenseEvidenceStore {
                absolute_roundoff: vec![vec![0.0; n]; n],
                operation_counts: vec![vec![0; n]; n],
                magnitudes: vec![vec![0.0; n]; n],
                scatter_counts: vec![vec![0usize; n]; n],
            },
        )?;
        Ok(Self {
            contributions: parts.contributions,
            absolute_roundoff: parts.store.absolute_roundoff,
            operation_counts: parts.store.operation_counts,
            coordinates: parts.coordinates,
            edges: parts.edges,
            spring_ground: parts.spring_ground,
            force_terms: None,
            formation: parts.formation,
            force_scale: ForceScale::UNSCALED,
        })
    }

    /// K2b (D1 §4.7, formation-time scaling): `new` on the primitives formed
    /// at 2^b: frames with E and G times 2^b, user and spring stiffnesses
    /// times 2^b, and each curved slot's global matrix and formation
    /// allowances times 2^b, all exactly. The formation allowances, the
    /// contributions and K-D5's formation primitives are then those of the
    /// scaled system. A value that cannot stay normal is
    /// `Range("force-scaled formation outside the normal range")`. With
    /// `UNSCALED` it is `new`.
    pub fn new_force_scaled(
        node_count: usize,
        frames: &[FrameElement],
        users: &[UserStiffnessElement],
        curved: &[CurvedBendStiffnessElement],
        springs: &[(usize, f64)],
        force_scale: ForceScale,
    ) -> Result<Self, StructuralError> {
        if force_scale.is_unscaled() {
            return Self::new(node_count, frames, users, curved, springs);
        }
        let (frames, users, curved, springs) =
            force_scaled_primitives(frames, users, curved, springs, force_scale)?;
        let mut evidence = Self::new(node_count, &frames, &users, &curved, &springs)?;
        evidence.force_scale = force_scale;
        Ok(evidence)
    }

    /// K2b: the force scale this evidence was formed at.
    pub fn force_scale(&self) -> ForceScale {
        self.force_scale
    }

    /// Screen each actual connected body using only selected ground constraints.
    pub fn qualified_passive_family(&self) -> bool {
        qualified_passive_family(&self.edges)
    }

    pub fn geometry(&self, prescribed: &[(usize, f64)]) -> Result<(), StructuralError> {
        BodyEvidence {
            coordinates: &self.coordinates,
            edges: &self.edges,
            spring_ground: &self.spring_ground,
            w4: None,
        }
        .geometry(prescribed)
    }

    /// K5 (D1 §4.9, W4; ROOT's K5 ruling Q1(b)): `geometry` with the
    /// constrained-body witness for mixed bodies. Called only in the four
    /// selected branches of the formation-checked entries.
    fn constrained_geometry(
        &self,
        prescribed: &[(usize, f64)],
        curved_sources: &[CurvedBendMacroElement],
    ) -> Result<(), StructuralError> {
        BodyEvidence {
            coordinates: &self.coordinates,
            edges: &self.edges,
            spring_ground: &self.spring_ground,
            w4: Some(W4Context {
                formation: &self.formation,
                curved_sources,
                force_scale: self.force_scale,
            }),
        }
        .geometry(prescribed)
    }

    /// Attaches the case's identified ledger terms. `solve` then also runs
    /// the kernel's load-fidelity audit against the given force vector (the
    /// C3-detect entry point): a flagged row makes the case Sensitive with a
    /// `LoadFidelityReport`, never a refusal. The solve itself is unchanged.
    pub fn with_force_terms(mut self, terms: &[ForceTerm]) -> Self {
        self.force_terms = Some(terms.to_vec());
        self
    }

    fn symmetry_basis(&self) -> String {
        symmetry_basis(&self.edges)
    }

    /// The `&[f64]` linear solve (C3-detect when `with_force_terms` is set).
    /// S11-F narrowed it to `pub(crate)`: the product calls the typed
    /// `solve_assembled`, and no caller outside this crate remains. Its
    /// callers are this crate's tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn solve(
        &self,
        k: &[Vec<f64>],
        f: &[f64],
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<StructuralSolution, StructuralError> {
        unscaled_evidence(self.force_scale)?;
        self.geometry(prescribed)?;
        let symmetry_basis = self.symmetry_basis();
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
        if let Some(terms) = &self.force_terms {
            let prepared = structural::prepare_structural_with_force_terms(&system, terms)?;
            return solve_prepared(prepared, mode);
        }
        match mode {
            LinearSolveMode::DenseScrutiny => structural::solve_structural_dense(&system),
            LinearSolveMode::SparseInteractive => {
                open_pipe_stress_sparse_direct::structural::solve_structural_sparse(&system)
            }
        }
    }

    /// K-D5's linear entry (D1 §4.3.1): `solve_assembled` (the ledger-built
    /// `AssembledForce`, S11-F), plus the D-5 formation check before a Passed
    /// result is published; the check's residual ρ uses the ledger terms.
    /// Called only from the product's linear route
    /// (`solve_preview_reduced_system`); the nonlinear loop keeps the unchanged
    /// `solve_binary64` (pinned in `s11k_tests`).
    ///
    /// - `curved_sources` are the macro elements the curved slots were formed
    ///   from. Each slot is matched by its node indices and by bitwise equality
    ///   of `global_stiffness()` with the slot's matrix, in any order. A slot
    ///   with no match, or an explicit slot, cannot be re-formed, and the case
    ///   is demoted with `formation_check_unavailable` (fail closed).
    /// - `selected` is false for an invocation with any nonlinear support: ROOT's
    ///   ruling that such a case is never selected. It then runs the unchanged
    ///   `solve_assembled` and keeps its ordinary result and standing exactly as
    ///   today.
    ///
    /// Values are never changed; a demoted case differs only in `quality`.
    #[allow(clippy::too_many_arguments)]
    pub fn solve_assembled_with_formation_check(
        &self,
        k: &[Vec<f64>],
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
        curved_sources: &[CurvedBendMacroElement],
        selected: bool,
    ) -> Result<StructuralSolution, StructuralError> {
        if !selected {
            return self.solve_assembled(k, f, free, prescribed, mode);
        }
        unscaled_evidence(self.force_scale)?;
        self.constrained_geometry(prescribed, curved_sources)?;
        let symmetry_basis = self.symmetry_basis();
        let source = self.formation_source(curved_sources);
        let system = StructuralSystem::assembled(
            k,
            f,
            free,
            prescribed,
            Some(&self.contributions),
            Some(SymmetryEvidence {
                absolute_roundoff: &self.absolute_roundoff,
                operation_counts: &self.operation_counts,
                basis: &symmetry_basis,
            }),
        )
        .with_formation_source(&source);
        solve_prepared(
            structural::prepare_formation_checked_structural(&system)?,
            mode,
        )
    }

    /// The formation source for this assembly, with each curved slot matched
    /// to its macro element (or named as not re-formable).
    fn formation_source(&self, curved_sources: &[CurvedBendMacroElement]) -> FormationSource {
        formation_source(&self.formation, curved_sources)
    }

    /// Named, unchanged binary64 variant of `solve` for the nonlinear active-set
    /// loop's closed-gap prescribed solves (ROOT option (c) on the S11-K stop
    /// report): the kernel's right-hand side and refinement residual fold in
    /// binary64 as before S11-K. Linear callers use `solve` (exact KS1-KS3).
    pub fn solve_binary64(
        &self,
        k: &[Vec<f64>],
        f: &[f64],
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<StructuralSolution, StructuralError> {
        unscaled_evidence(self.force_scale)?;
        self.geometry(prescribed)?;
        let symmetry_basis = self.symmetry_basis();
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
        solve_prepared(structural::prepare_structural_binary64(&system)?, mode)
    }

    /// Typed sibling of `solve` (S11-K; dormant until S11-F): the force is the
    /// ledger-built `AssembledForce`, so the kernel's right-hand side (KS1),
    /// refinement residual (KS3) and load-fidelity audit use its terms.
    /// ```compile_fail
    /// use open_pipe_stress_nonlinear_integration::structural_adapter::AssemblyEvidence;
    /// use open_pipe_stress_nonlinear_integration::LinearSolveMode;
    /// let evidence = AssemblyEvidence::new(1, &[], &[], &[], &[]).unwrap();
    /// let k = vec![vec![1.0; 6]; 6];
    /// let force: Vec<f64> = vec![0.0; 6];
    /// let _ = evidence.solve_assembled(&k, &force, &[0], &[], LinearSolveMode::DenseScrutiny);
    /// ```
    /// The positive twin (RV1-N8): the same call compiles with a ledger-built
    /// `AssembledForce`, so the failure above is the refused `Vec<f64>`.
    /// ```
    /// use open_pipe_stress_frame_kernel::load_ledger::LoadLedger;
    /// use open_pipe_stress_nonlinear_integration::structural_adapter::AssemblyEvidence;
    /// use open_pipe_stress_nonlinear_integration::LinearSolveMode;
    /// let evidence = AssemblyEvidence::new(1, &[], &[], &[], &[]).unwrap();
    /// let k = vec![vec![1.0; 6]; 6];
    /// let force = LoadLedger::new().finish(6).unwrap();
    /// let _ = evidence.solve_assembled(&k, &force, &[0], &[], LinearSolveMode::DenseScrutiny);
    /// ```
    pub fn solve_assembled(
        &self,
        k: &[Vec<f64>],
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<StructuralSolution, StructuralError> {
        unscaled_evidence(self.force_scale)?;
        self.geometry(prescribed)?;
        let symmetry_basis = self.symmetry_basis();
        let system = StructuralSystem::assembled(
            k,
            f,
            free,
            prescribed,
            Some(&self.contributions),
            Some(SymmetryEvidence {
                absolute_roundoff: &self.absolute_roundoff,
                operation_counts: &self.operation_counts,
                basis: &symmetry_basis,
            }),
        );
        solve_prepared(structural::prepare_assembled_structural(&system)?, mode)
    }

    /// K2b (D1 §4.7): the force-scaled sibling of `solve_assembled`.
    /// - `k` is the stiffness formed at this evidence's 2^b (the kernel's
    ///   assembly with `SparseAssemblyOptions::with_force_scale`, or its dense
    ///   view). A matrix at another scale fails M03's contribution audit.
    /// - `f` is the case's **unscaled** ledger force; its terms are taken at
    ///   2^b here (`AssembledForce::force_scaled`), exactly.
    /// - The result is unscaled for publication
    ///   (`structural::unscale_structural_solution`: each residual-record
    ///   field with its explicit outcome), and a `StructuralError` has its
    ///   payload unscaled. With `UNSCALED` the solution and errors are
    ///   `solve_assembled`'s, unchanged.
    pub fn solve_force_scaled(
        &self,
        k: &[Vec<f64>],
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<ForceScaledSolution, ForceScaledError> {
        let owned;
        let force = if self.force_scale.is_unscaled() {
            f
        } else {
            owned = f
                .force_scaled(self.force_scale)
                .map_err(ForceScaledError::refused)?;
            &owned
        };
        let solved = self.geometry(prescribed).and_then(|()| {
            let symmetry_basis = self.symmetry_basis();
            let system = StructuralSystem::assembled(
                k,
                force,
                free,
                prescribed,
                Some(&self.contributions),
                Some(SymmetryEvidence {
                    absolute_roundoff: &self.absolute_roundoff,
                    operation_counts: &self.operation_counts,
                    basis: &symmetry_basis,
                }),
            );
            solve_prepared(structural::prepare_assembled_structural(&system)?, mode)
        });
        force_scaled_outcome(solved, self.force_scale, f)
    }

    /// K2b (D1 §4.7): the force-scaled sibling of K-D5's linear entry
    /// `solve_assembled_with_formation_check`, with the same selection rule.
    /// `k`, `f` and the result are as in `solve_force_scaled`. The formation
    /// source holds the primitives at 2^b (the frames, users and springs this
    /// evidence was formed from; each curved slot matched to its macro element
    /// by the bits of the macro element's global matrix times 2^b, with E and
    /// G times 2^b), so K-D5's re-formation, its ρ and its record (in
    /// displacement units) are those of the scaled system. With `UNSCALED` it
    /// is `solve_assembled_with_formation_check`, unchanged.
    #[allow(clippy::too_many_arguments)]
    pub fn solve_force_scaled_with_formation_check(
        &self,
        k: &[Vec<f64>],
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
        curved_sources: &[CurvedBendMacroElement],
        selected: bool,
    ) -> Result<ForceScaledSolution, ForceScaledError> {
        if !selected {
            return self.solve_force_scaled(k, f, free, prescribed, mode);
        }
        let owned;
        let force = if self.force_scale.is_unscaled() {
            f
        } else {
            owned = f
                .force_scaled(self.force_scale)
                .map_err(ForceScaledError::refused)?;
            &owned
        };
        let screened = self.constrained_geometry(prescribed, curved_sources);
        let solved = screened.and_then(|()| {
            let symmetry_basis = self.symmetry_basis();
            let source =
                force_scaled_formation_source(&self.formation, curved_sources, self.force_scale);
            let system = StructuralSystem::assembled(
                k,
                force,
                free,
                prescribed,
                Some(&self.contributions),
                Some(SymmetryEvidence {
                    absolute_roundoff: &self.absolute_roundoff,
                    operation_counts: &self.operation_counts,
                    basis: &symmetry_basis,
                }),
            )
            .with_formation_source(&source);
            solve_prepared(
                structural::prepare_formation_checked_structural(&system)?,
                mode,
            )
        });
        force_scaled_outcome(solved, self.force_scale, f)
    }
}

/// K1 (T3 D1 revision 5a.2 §4.8): the sparse `AssemblyEvidence`. It is built
/// by the same code as `AssemblyEvidence::new` (`EvidenceParts::new`), with
/// the formation allowances and operation counts indexed by the stiffness
/// pattern instead of n×n, so every allowance is the dense one bit for bit.
/// The contributions, S11-K's force terms, K-D5's formation primitives and
/// the geometry, qualified-family and rigid-body evidence are unchanged in
/// content.
///
/// Its solve entries take the pattern (`SparseStiffness`). In
/// `SparseInteractive` mode they run the M03 gate over the pattern; in
/// `DenseScrutiny` mode they materialize the dense view of the same values
/// and run today's dense Cholesky path, bit for bit.
#[derive(Debug, Clone)]
pub struct SparseAssemblyEvidence {
    contributions: Vec<StiffnessContribution>,
    pattern: SparsePattern,
    absolute_roundoff: Vec<f64>,
    operation_counts: Vec<usize>,
    coordinates: Vec<Option<[f64; 3]>>,
    edges: Vec<(usize, usize, bool)>,
    spring_ground: Vec<usize>,
    force_terms: Option<Vec<ForceTerm>>,
    formation: FormationPrimitives,
    /// K2b: as `AssemblyEvidence`'s.
    force_scale: ForceScale,
}

impl SparseAssemblyEvidence {
    /// `AssemblyEvidence::new` on the pattern of the assembled stiffness
    /// (`assemble_sparse_stiffness` of the same elements and springs).
    pub fn new(
        pattern: &SparsePattern,
        node_count: usize,
        frames: &[FrameElement],
        users: &[UserStiffnessElement],
        curved: &[CurvedBendStiffnessElement],
        springs: &[(usize, f64)],
    ) -> Result<Self, StructuralError> {
        if Some(pattern.dimension()) != node_count.checked_mul(6) {
            return Err(StructuralError::InvalidInput("stiffness pattern dimension"));
        }
        let nnz = pattern.entry_count();
        let parts = EvidenceParts::new(
            node_count,
            frames,
            users,
            curved,
            springs,
            SparseEvidenceStore {
                pattern: pattern.clone(),
                absolute_roundoff: vec![0.0; nnz],
                operation_counts: vec![0; nnz],
                magnitudes: vec![0.0; nnz],
                scatter_counts: vec![0; nnz],
            },
        )?;
        Ok(Self {
            contributions: parts.contributions,
            pattern: parts.store.pattern,
            absolute_roundoff: parts.store.absolute_roundoff,
            operation_counts: parts.store.operation_counts,
            coordinates: parts.coordinates,
            edges: parts.edges,
            spring_ground: parts.spring_ground,
            force_terms: None,
            formation: parts.formation,
            force_scale: ForceScale::UNSCALED,
        })
    }
    /// K2b: `AssemblyEvidence::new_force_scaled` on the pattern of the
    /// assembly formed at 2^b (the pattern does not depend on b).
    #[allow(clippy::too_many_arguments)]
    pub fn new_force_scaled(
        pattern: &SparsePattern,
        node_count: usize,
        frames: &[FrameElement],
        users: &[UserStiffnessElement],
        curved: &[CurvedBendStiffnessElement],
        springs: &[(usize, f64)],
        force_scale: ForceScale,
    ) -> Result<Self, StructuralError> {
        if force_scale.is_unscaled() {
            return Self::new(pattern, node_count, frames, users, curved, springs);
        }
        let (frames, users, curved, springs) =
            force_scaled_primitives(frames, users, curved, springs, force_scale)?;
        let mut evidence = Self::new(pattern, node_count, &frames, &users, &curved, &springs)?;
        evidence.force_scale = force_scale;
        Ok(evidence)
    }
    /// K2b: the force scale this evidence was formed at.
    pub fn force_scale(&self) -> ForceScale {
        self.force_scale
    }
    /// `AssemblyEvidence::with_force_terms` (S11-K's C3-detect terms).
    pub fn with_force_terms(mut self, terms: &[ForceTerm]) -> Self {
        self.force_terms = Some(terms.to_vec());
        self
    }
    pub fn qualified_passive_family(&self) -> bool {
        qualified_passive_family(&self.edges)
    }
    pub fn geometry(&self, prescribed: &[(usize, f64)]) -> Result<(), StructuralError> {
        BodyEvidence {
            coordinates: &self.coordinates,
            edges: &self.edges,
            spring_ground: &self.spring_ground,
            w4: None,
        }
        .geometry(prescribed)
    }
    /// K5: `AssemblyEvidence::constrained_geometry` (the four selected
    /// branches only).
    fn constrained_geometry(
        &self,
        prescribed: &[(usize, f64)],
        curved_sources: &[CurvedBendMacroElement],
    ) -> Result<(), StructuralError> {
        BodyEvidence {
            coordinates: &self.coordinates,
            edges: &self.edges,
            spring_ground: &self.spring_ground,
            w4: Some(W4Context {
                formation: &self.formation,
                curved_sources,
                force_scale: self.force_scale,
            }),
        }
        .geometry(prescribed)
    }
    pub fn pattern(&self) -> &SparsePattern {
        &self.pattern
    }
    /// The contributions, both triangles, in assembly order (as
    /// `AssemblyEvidence::contributions`).
    pub fn contributions(&self) -> &[StiffnessContribution] {
        &self.contributions
    }
    /// The formation allowance of each pattern entry.
    pub fn absolute_roundoff(&self) -> &[f64] {
        &self.absolute_roundoff
    }
    /// The operation count of each pattern entry.
    pub fn operation_counts(&self) -> &[usize] {
        &self.operation_counts
    }
    /// Deterministic storage counts: pattern entries and contributions.
    pub fn storage_counts(&self) -> SparseEvidenceCounts {
        SparseEvidenceCounts {
            pattern_entries: self.pattern.entry_count(),
            contributions: self.contributions.len(),
        }
    }
    /// The dense (n×n) view of the allowances and operation counts, as
    /// `AssemblyEvidence` holds them (0 where nothing is stored).
    pub fn dense_symmetry_view(&self) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
        let n = self.pattern.dimension();
        let mut roundoff = vec![vec![0.0; n]; n];
        let mut counts = vec![vec![0; n]; n];
        for row in 0..n {
            for index in self.pattern.row_range(row) {
                let col = self.pattern.column(index);
                roundoff[row][col] = self.absolute_roundoff[index];
                counts[row][col] = self.operation_counts[index];
            }
        }
        (roundoff, counts)
    }
    fn check_pattern(&self, k: &SparseStiffness) -> Result<(), StructuralError> {
        if k.pattern() != &self.pattern {
            return Err(StructuralError::InvalidInput(
                "stiffness pattern differs from the assembly evidence",
            ));
        }
        Ok(())
    }

    /// The pattern-taking sibling of `AssemblyEvidence::solve_assembled`.
    pub fn solve_assembled(
        &self,
        k: &SparseStiffness,
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<StructuralSolution, StructuralError> {
        unscaled_evidence(self.force_scale)?;
        self.check_pattern(k)?;
        self.geometry(prescribed)?;
        let symmetry_basis = symmetry_basis(&self.edges);
        match mode {
            LinearSolveMode::DenseScrutiny => {
                let dense = k.to_dense();
                let (roundoff, counts) = self.dense_symmetry_view();
                let system = StructuralSystem::assembled(
                    &dense,
                    f,
                    free,
                    prescribed,
                    Some(&self.contributions),
                    Some(SymmetryEvidence {
                        absolute_roundoff: &roundoff,
                        operation_counts: &counts,
                        basis: &symmetry_basis,
                    }),
                );
                solve_prepared(structural::prepare_assembled_structural(&system)?, mode)
            }
            LinearSolveMode::SparseInteractive => {
                let system = SparseStructuralSystem::assembled(
                    k,
                    f,
                    free,
                    prescribed,
                    Some(&self.contributions),
                    Some(self.sparse_symmetry(&symmetry_basis)),
                );
                solve_sparse_prepared(structural::prepare_assembled_sparse_structural(&system)?)
            }
        }
    }

    /// The pattern-taking sibling of
    /// `AssemblyEvidence::solve_assembled_with_formation_check` (K-D5's linear
    /// entry): the same selection rule and the same formation source.
    #[allow(clippy::too_many_arguments)]
    pub fn solve_assembled_with_formation_check(
        &self,
        k: &SparseStiffness,
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
        curved_sources: &[CurvedBendMacroElement],
        selected: bool,
    ) -> Result<StructuralSolution, StructuralError> {
        if !selected {
            return self.solve_assembled(k, f, free, prescribed, mode);
        }
        unscaled_evidence(self.force_scale)?;
        self.check_pattern(k)?;
        self.constrained_geometry(prescribed, curved_sources)?;
        let symmetry_basis = symmetry_basis(&self.edges);
        let source = formation_source(&self.formation, curved_sources);
        match mode {
            LinearSolveMode::DenseScrutiny => {
                let dense = k.to_dense();
                let (roundoff, counts) = self.dense_symmetry_view();
                let system = StructuralSystem::assembled(
                    &dense,
                    f,
                    free,
                    prescribed,
                    Some(&self.contributions),
                    Some(SymmetryEvidence {
                        absolute_roundoff: &roundoff,
                        operation_counts: &counts,
                        basis: &symmetry_basis,
                    }),
                )
                .with_formation_source(&source);
                solve_prepared(
                    structural::prepare_formation_checked_structural(&system)?,
                    mode,
                )
            }
            LinearSolveMode::SparseInteractive => {
                let system = SparseStructuralSystem::assembled(
                    k,
                    f,
                    free,
                    prescribed,
                    Some(&self.contributions),
                    Some(self.sparse_symmetry(&symmetry_basis)),
                )
                .with_formation_source(&source);
                solve_sparse_prepared(structural::prepare_formation_checked_sparse_structural(
                    &system,
                )?)
            }
        }
    }

    /// The pattern-taking sibling of the crate's `&[f64]` `solve` (C3-detect
    /// when `with_force_terms` is set). Its callers are this crate's tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn solve(
        &self,
        k: &SparseStiffness,
        f: &[f64],
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<StructuralSolution, StructuralError> {
        unscaled_evidence(self.force_scale)?;
        self.check_pattern(k)?;
        self.geometry(prescribed)?;
        let symmetry_basis = symmetry_basis(&self.edges);
        match mode {
            LinearSolveMode::DenseScrutiny => {
                let dense = k.to_dense();
                let (roundoff, counts) = self.dense_symmetry_view();
                let system = StructuralSystem {
                    stiffness: &dense,
                    force: f,
                    free_dofs: free,
                    prescribed,
                    contributions: Some(&self.contributions),
                    symmetry: Some(SymmetryEvidence {
                        absolute_roundoff: &roundoff,
                        operation_counts: &counts,
                        basis: &symmetry_basis,
                    }),
                };
                if let Some(terms) = &self.force_terms {
                    let prepared = structural::prepare_structural_with_force_terms(&system, terms)?;
                    return solve_prepared(prepared, mode);
                }
                structural::solve_structural_dense(&system)
            }
            LinearSolveMode::SparseInteractive => {
                let system = SparseStructuralSystem::new(
                    k,
                    f,
                    free,
                    prescribed,
                    Some(&self.contributions),
                    Some(self.sparse_symmetry(&symmetry_basis)),
                );
                match &self.force_terms {
                    Some(terms) => solve_sparse_prepared(
                        structural::prepare_sparse_structural_with_force_terms(&system, terms)?,
                    ),
                    None => solve_sparse_prepared(structural::prepare_sparse_structural(&system)?),
                }
            }
        }
    }

    /// K2b: the pattern-taking sibling of `AssemblyEvidence::solve_force_scaled`
    /// (the force-scaled `solve_assembled`): `k` formed at this evidence's
    /// 2^b, `f` the unscaled ledger force, the result unscaled for
    /// publication. With `UNSCALED` it is `solve_assembled`, unchanged.
    pub fn solve_force_scaled(
        &self,
        k: &SparseStiffness,
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
    ) -> Result<ForceScaledSolution, ForceScaledError> {
        let owned;
        let force = if self.force_scale.is_unscaled() {
            f
        } else {
            owned = f
                .force_scaled(self.force_scale)
                .map_err(ForceScaledError::refused)?;
            &owned
        };
        let solved = self
            .check_pattern(k)
            .and_then(|()| self.geometry(prescribed))
            .and_then(|()| {
                let symmetry_basis = symmetry_basis(&self.edges);
                match mode {
                    LinearSolveMode::DenseScrutiny => {
                        let dense = k.to_dense();
                        let (roundoff, counts) = self.dense_symmetry_view();
                        let system = StructuralSystem::assembled(
                            &dense,
                            force,
                            free,
                            prescribed,
                            Some(&self.contributions),
                            Some(SymmetryEvidence {
                                absolute_roundoff: &roundoff,
                                operation_counts: &counts,
                                basis: &symmetry_basis,
                            }),
                        );
                        solve_prepared(structural::prepare_assembled_structural(&system)?, mode)
                    }
                    LinearSolveMode::SparseInteractive => {
                        let system = SparseStructuralSystem::assembled(
                            k,
                            force,
                            free,
                            prescribed,
                            Some(&self.contributions),
                            Some(self.sparse_symmetry(&symmetry_basis)),
                        );
                        solve_sparse_prepared(structural::prepare_assembled_sparse_structural(
                            &system,
                        )?)
                    }
                }
            });
        force_scaled_outcome(solved, self.force_scale, f)
    }

    /// K2b: the pattern-taking sibling of
    /// `AssemblyEvidence::solve_force_scaled_with_formation_check`, with the
    /// same selection rule and the same force-scaled formation source. With
    /// `UNSCALED` it is `solve_assembled_with_formation_check`, unchanged.
    #[allow(clippy::too_many_arguments)]
    pub fn solve_force_scaled_with_formation_check(
        &self,
        k: &SparseStiffness,
        f: &AssembledForce,
        free: &[usize],
        prescribed: &[(usize, f64)],
        mode: LinearSolveMode,
        curved_sources: &[CurvedBendMacroElement],
        selected: bool,
    ) -> Result<ForceScaledSolution, ForceScaledError> {
        if !selected {
            return self.solve_force_scaled(k, f, free, prescribed, mode);
        }
        let owned;
        let force = if self.force_scale.is_unscaled() {
            f
        } else {
            owned = f
                .force_scaled(self.force_scale)
                .map_err(ForceScaledError::refused)?;
            &owned
        };
        let solved = self
            .check_pattern(k)
            .and_then(|()| self.constrained_geometry(prescribed, curved_sources))
            .and_then(|()| {
                let symmetry_basis = symmetry_basis(&self.edges);
                let source = force_scaled_formation_source(
                    &self.formation,
                    curved_sources,
                    self.force_scale,
                );
                match mode {
                    LinearSolveMode::DenseScrutiny => {
                        let dense = k.to_dense();
                        let (roundoff, counts) = self.dense_symmetry_view();
                        let system = StructuralSystem::assembled(
                            &dense,
                            force,
                            free,
                            prescribed,
                            Some(&self.contributions),
                            Some(SymmetryEvidence {
                                absolute_roundoff: &roundoff,
                                operation_counts: &counts,
                                basis: &symmetry_basis,
                            }),
                        )
                        .with_formation_source(&source);
                        solve_prepared(
                            structural::prepare_formation_checked_structural(&system)?,
                            mode,
                        )
                    }
                    LinearSolveMode::SparseInteractive => {
                        let system = SparseStructuralSystem::assembled(
                            k,
                            force,
                            free,
                            prescribed,
                            Some(&self.contributions),
                            Some(self.sparse_symmetry(&symmetry_basis)),
                        )
                        .with_formation_source(&source);
                        solve_sparse_prepared(
                            structural::prepare_formation_checked_sparse_structural(&system)?,
                        )
                    }
                }
            });
        force_scaled_outcome(solved, self.force_scale, f)
    }

    fn sparse_symmetry<'e>(&'e self, basis: &'e str) -> SparseSymmetryEvidence<'e> {
        SparseSymmetryEvidence {
            absolute_roundoff: &self.absolute_roundoff,
            operation_counts: &self.operation_counts,
            basis,
        }
    }
}

/// Deterministic storage counts of a `SparseAssemblyEvidence`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SparseEvidenceCounts {
    pub pattern_entries: usize,
    pub contributions: usize,
}

/// K1: where `EvidenceParts::new` records its formation allowances and
/// operation counts: n×n arrays (the dense `AssemblyEvidence`) or one slot per
/// pattern entry (`SparseAssemblyEvidence`). The arithmetic is the builder's,
/// in `new` and `element`, unchanged.
trait EvidenceStore {
    /// The (allowance, operation count) slots of entry (row, col).
    fn slot(&mut self, row: usize, col: usize) -> Result<(&mut f64, &mut usize), StructuralError>;
    /// The scatter (magnitude, count) slots of entry (row, col).
    fn scatter_slot(
        &mut self,
        row: usize,
        col: usize,
    ) -> Result<(&mut f64, &mut usize), StructuralError>;
    /// Calls `f(allowance, magnitude, scatter count)` for every entry, in
    /// row-major order, stopping at the first error.
    fn visit(
        &mut self,
        f: &mut dyn FnMut(&mut f64, f64, usize) -> Result<(), StructuralError>,
    ) -> Result<(), StructuralError>;
}

struct DenseEvidenceStore {
    absolute_roundoff: Vec<Vec<f64>>,
    operation_counts: Vec<Vec<usize>>,
    magnitudes: Vec<Vec<f64>>,
    scatter_counts: Vec<Vec<usize>>,
}

impl EvidenceStore for DenseEvidenceStore {
    fn slot(&mut self, row: usize, col: usize) -> Result<(&mut f64, &mut usize), StructuralError> {
        Ok((
            &mut self.absolute_roundoff[row][col],
            &mut self.operation_counts[row][col],
        ))
    }
    fn scatter_slot(
        &mut self,
        row: usize,
        col: usize,
    ) -> Result<(&mut f64, &mut usize), StructuralError> {
        Ok((
            &mut self.magnitudes[row][col],
            &mut self.scatter_counts[row][col],
        ))
    }
    fn visit(
        &mut self,
        f: &mut dyn FnMut(&mut f64, f64, usize) -> Result<(), StructuralError>,
    ) -> Result<(), StructuralError> {
        let n = self.absolute_roundoff.len();
        for i in 0..n {
            for j in 0..n {
                f(
                    &mut self.absolute_roundoff[i][j],
                    self.magnitudes[i][j],
                    self.scatter_counts[i][j],
                )?;
            }
        }
        Ok(())
    }
}

struct SparseEvidenceStore {
    pattern: SparsePattern,
    absolute_roundoff: Vec<f64>,
    operation_counts: Vec<usize>,
    magnitudes: Vec<f64>,
    scatter_counts: Vec<usize>,
}

impl SparseEvidenceStore {
    fn index(&self, row: usize, col: usize) -> Result<usize, StructuralError> {
        self.pattern
            .find(row, col)
            .ok_or(StructuralError::InvalidInput(
                "assembly entry outside the stiffness pattern",
            ))
    }
}

impl EvidenceStore for SparseEvidenceStore {
    fn slot(&mut self, row: usize, col: usize) -> Result<(&mut f64, &mut usize), StructuralError> {
        let index = self.index(row, col)?;
        Ok((
            &mut self.absolute_roundoff[index],
            &mut self.operation_counts[index],
        ))
    }
    fn scatter_slot(
        &mut self,
        row: usize,
        col: usize,
    ) -> Result<(&mut f64, &mut usize), StructuralError> {
        let index = self.index(row, col)?;
        Ok((&mut self.magnitudes[index], &mut self.scatter_counts[index]))
    }
    /// Pattern entries are row-major. An unstored entry of the dense store
    /// has allowance 0 and scatter count 0, which the pass leaves at 0.
    fn visit(
        &mut self,
        f: &mut dyn FnMut(&mut f64, f64, usize) -> Result<(), StructuralError>,
    ) -> Result<(), StructuralError> {
        for index in 0..self.absolute_roundoff.len() {
            f(
                &mut self.absolute_roundoff[index],
                self.magnitudes[index],
                self.scatter_counts[index],
            )?;
        }
        Ok(())
    }
}

/// What `AssemblyEvidence::new` builds, for either store (K1).
struct EvidenceParts<S> {
    contributions: Vec<StiffnessContribution>,
    coordinates: Vec<Option<[f64; 3]>>,
    edges: Vec<(usize, usize, bool)>,
    spring_ground: Vec<usize>,
    formation: FormationPrimitives,
    store: S,
}

impl<S: EvidenceStore> EvidenceParts<S> {
    fn new(
        node_count: usize,
        frames: &[FrameElement],
        users: &[UserStiffnessElement],
        curved: &[CurvedBendStiffnessElement],
        springs: &[(usize, f64)],
        store: S,
    ) -> Result<Self, StructuralError> {
        let n = node_count * 6;
        let mut result = Self {
            contributions: Vec::new(),
            coordinates: vec![None; node_count],
            edges: Vec::new(),
            spring_ground: Vec::new(),
            formation: FormationPrimitives {
                node_count,
                frames: frames.to_vec(),
                users: users.to_vec(),
                curved: curved
                    .iter()
                    .map(|e| CurvedSlot {
                        element_id: e.element_id.clone(),
                        node_i: e.node_i,
                        node_j: e.node_j,
                        global_stiffness: e.global_stiffness,
                        explicit: e.symmetry_formation.is_none(),
                    })
                    .collect(),
                springs: springs.to_vec(),
            },
            store,
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
            *result.store.slot(dof, dof)?.1 += 1;
            if value > 0.0 {
                result.spring_ground.push(dof);
            }
        }
        // Sequential scatter adds are independent of transformation formation.
        for c in &result.contributions {
            let (magnitude, count) = result.store.scatter_slot(c.row, c.col)?;
            *magnitude += c.value.abs();
            *count += 1;
        }
        result.store.visit(&mut |roundoff, magnitude, count| {
            *roundoff += structural::gamma(count) * magnitude;
            if !roundoff.is_finite() {
                return Err(StructuralError::Range("assembly allowance"));
            }
            Ok(())
        })?;
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
                let (bound, count) = self.store.slot(map[i], map[j])?;
                *bound += bounds[i][j];
                *count += counts[i][j] + 1;
            }
        }
        Ok(())
    }
}

/// The connected-body evidence `geometry` screens (K1: shared by both
/// evidences).
struct BodyEvidence<'e> {
    coordinates: &'e [Option<[f64; 3]>],
    edges: &'e [(usize, usize, bool)],
    spring_ground: &'e [usize],
    /// K5 (W4): `Some` only in the four selected branches.
    w4: Option<W4Context<'e>>,
}

impl BodyEvidence<'_> {
    fn geometry(&self, prescribed: &[(usize, f64)]) -> Result<(), StructuralError> {
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
                for &(a, b, _) in self.edges {
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
            // K5 (W4, the four selected branches only): a mixed body whose user
            // elements are ties and whose curved elements match their macro
            // source is assessed by `assess_constrained_bodies`; any other mixed
            // body is left to the matrix gate, as today.
            if !qualified {
                if let Some(w4) = &self.w4 {
                    w4.screen(self, &body, prescribed)?;
                }
            }
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
}

// ------------------------------------------------------------------ K5 (W4)

/// K5 (D1 §4.9, W4): what the selected branches pass to `BodyEvidence`: the
/// formation primitives (edge order: frames, users, curved), the macro
/// elements the curved slots were formed from, and the evidence's 2^b.
struct W4Context<'e> {
    formation: &'e FormationPrimitives,
    curved_sources: &'e [CurvedBendMacroElement],
    force_scale: ForceScale,
}

/// K5: W4's record for one mixed body. The reason for an unqualified body is
/// carried here, never in `StructuralReport` (D5C-3).
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum W4Body {
    Unqualified(W4Unqualified),
    /// `nodes`: the body's global nodes, ascending (local node k is nodes[k]).
    Assessed {
        nodes: Vec<usize>,
        assessment: ConstrainedAssessment,
    },
}

/// K5: why a mixed body is not assessed by W4 (the matrix gate still runs).
#[allow(dead_code)] // F2a API: the reason carrier; read by tests today.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum W4Unqualified {
    /// A user element that is no tie (Q5(a)): `element` indexes the users.
    UserTie { element: usize, refusal: TieRefusal },
    /// A curved slot built by `CurvedBendStiffnessElement::new` (no source).
    CurvedExplicit { element_id: String },
    /// A curved slot matched to no macro element (Q2(b)).
    CurvedUnmatched { element_id: String },
    /// A matched macro element whose node coordinates differ from those
    /// recorded for the same node (a frame, a user or another curved slot).
    CurvedCoordinates { element_id: String, node: usize },
    /// `assess_constrained_bodies` refused the input. Unreachable from built
    /// evidence (I14's RETURN derives it); the body is left to the matrix gate.
    Geometry(StructuralError),
}

impl W4Context<'_> {
    /// W4's outcome for one mixed body, mapped as the frame screen maps its own
    /// (a witnessed mechanism with its direction over the global DOFs, or the
    /// unresolved rank); an unqualified body passes to the matrix gate.
    fn screen(
        &self,
        evidence: &BodyEvidence<'_>,
        body: &[usize],
        prescribed: &[(usize, f64)],
    ) -> Result<(), StructuralError> {
        let W4Body::Assessed { nodes, assessment } = self.body(evidence, body, prescribed) else {
            return Ok(());
        };
        match assessment.status {
            RigidBodyStatus::Restrained => Ok(()),
            RigidBodyStatus::MechanismWitnessed => {
                let mut direction = vec![0.0; 6 * evidence.coordinates.len()];
                let motion = assessment.node_motion.unwrap_or_default();
                for (&global, motion) in nodes.iter().zip(motion) {
                    direction[6 * global..6 * global + 6].copy_from_slice(&motion);
                }
                Err(StructuralError::Mechanism { direction })
            }
            _ => Err(StructuralError::NumericallyUnresolved {
                reason: "constrained-body rank unresolved",
                global_dof: None,
            }),
        }
    }

    /// The per-body record: every edge of the body qualified (frames link,
    /// user elements tie under `user_element_tie`, curved slots link when
    /// matched to their macro source with consistent coordinates), then
    /// `assess_constrained_bodies` over the body's nodes in ascending order,
    /// with the prescribed and positive-spring grounds as today.
    pub(crate) fn body(
        &self,
        evidence: &BodyEvidence<'_>,
        body: &[usize],
        prescribed: &[(usize, f64)],
    ) -> W4Body {
        let mut nodes = body.to_vec();
        nodes.sort_unstable();
        let local = |global: usize| nodes.binary_search(&global).ok();
        let frames = self.formation.frames.len();
        let users = frames + self.formation.users.len();
        let mut curved_points: Vec<Option<[f64; 3]>> = vec![None; nodes.len()];
        let mut links = Vec::new();
        let mut ties = Vec::new();
        for (index, &(a, b, _)) in evidence.edges.iter().enumerate() {
            let (Some(la), Some(lb)) = (local(a), local(b)) else {
                continue;
            };
            if index < frames {
                links.push([la, lb]);
            } else if index < users {
                let element = index - frames;
                match user_element_tie(&self.formation.users[element]) {
                    Ok(_) => ties.push([la, lb]),
                    Err(refusal) => {
                        return W4Body::Unqualified(W4Unqualified::UserTie { element, refusal })
                    }
                }
            } else {
                let slot = &self.formation.curved[index - users];
                let element_id = slot.element_id.clone();
                if slot.explicit {
                    return W4Body::Unqualified(W4Unqualified::CurvedExplicit { element_id });
                }
                let Some(source) = w4_curved_source(slot, self.curved_sources, self.force_scale)
                else {
                    return W4Body::Unqualified(W4Unqualified::CurvedUnmatched { element_id });
                };
                for (l, global, point) in [
                    (la, a, source.node_i.coordinates),
                    (lb, b, source.node_j.coordinates),
                ] {
                    let recorded = evidence.coordinates[global].or(curved_points[l]);
                    if recorded.is_some_and(|old| old != point) {
                        return W4Body::Unqualified(W4Unqualified::CurvedCoordinates {
                            element_id,
                            node: global,
                        });
                    }
                    curved_points[l] = Some(point);
                }
                links.push([la, lb]);
            }
        }
        let mut points = Vec::with_capacity(nodes.len());
        for (l, &global) in nodes.iter().enumerate() {
            match evidence.coordinates[global].or(curved_points[l]) {
                Some(point) => points.push(point),
                None => {
                    return W4Body::Unqualified(W4Unqualified::Geometry(
                        StructuralError::InvalidInput("constrained geometry"),
                    ))
                }
            }
        }
        let sub_bodies = match objective_sub_bodies(nodes.len(), &links) {
            Ok(sub_bodies) => sub_bodies,
            Err(error) => return W4Body::Unqualified(W4Unqualified::Geometry(error)),
        };
        let grounds = prescribed
            .iter()
            .map(|&(d, _)| d)
            .chain(evidence.spring_ground.iter().copied())
            .filter_map(|d| local(d / 6).map(|l| ConstrainedGround::Dof(6 * l + d % 6)))
            .collect::<Vec<_>>();
        match assess_constrained_bodies(&points, &sub_bodies, &ties, &grounds) {
            Ok(assessment) => W4Body::Assessed { nodes, assessment },
            Err(error) => W4Body::Unqualified(W4Unqualified::Geometry(error)),
        }
    }
}

/// K5 (Q2(b)): the macro element a curved slot was formed from, by K-D5's
/// predicate (`formation_source`; `force_scaled_formation_source` at 2^b):
/// the node indices, and the bits of the macro element's global matrix
/// (times 2^b) equal to the slot's; the first match in `curved_sources`.
/// A matched element's moduli need not scale normally (W4 uses its nodes).
fn w4_curved_source<'m>(
    slot: &CurvedSlot,
    curved_sources: &'m [CurvedBendMacroElement],
    force_scale: ForceScale,
) -> Option<&'m CurvedBendMacroElement> {
    curved_sources.iter().find(|m| {
        m.node_i.index == slot.node_i
            && m.node_j.index == slot.node_j
            && m.global_stiffness()
                .ok()
                .and_then(|g| {
                    if force_scale.is_unscaled() {
                        Some(g)
                    } else {
                        force_scaled_matrix("curved entry*2^b", &g, force_scale).ok()
                    }
                })
                .is_some_and(|g| {
                    g.iter()
                        .flatten()
                        .zip(slot.global_stiffness.iter().flatten())
                        .all(|(a, b)| a.to_bits() == b.to_bits())
                })
    })
}

fn qualified_passive_family(edges: &[(usize, usize, bool)]) -> bool {
    !edges.is_empty() && edges.iter().all(|(_, _, qualified)| *qualified)
}

fn symmetry_basis(edges: &[(usize, usize, bool)]) -> String {
    let family_basis = if edges.iter().all(|(_, _, qualified)| *qualified) {
        "objective welded unreleased straight-frame family"
    } else {
        "mixed or explicit-matrix family: physical rigid-null witness unqualified for bodies containing user/curved elements; matrix positivity remains mandatory"
    };
    format!("represented local matrices; two-stage 12-term frame transforms plus directed scatter; curved H*K and (H*K)*H^T six-term stages when traced; inverse accuracy not claimed; {family_basis}")
}

/// The formation source for an assembly, with each curved slot matched to
/// its macro element (or named as not re-formable).
fn formation_source(
    primitives: &FormationPrimitives,
    curved_sources: &[CurvedBendMacroElement],
) -> FormationSource {
    let mut source = FormationSource {
        node_count: primitives.node_count,
        frames: primitives.frames.clone(),
        users: primitives.users.clone(),
        curved: Vec::new(),
        springs: primitives.springs.clone(),
        unavailable: Vec::new(),
    };
    for slot in &primitives.curved {
        if slot.explicit {
            source
                .unavailable
                .push(format!("curved_bend_explicit_matrix:{}", slot.element_id));
            continue;
        }
        let matched = curved_sources.iter().find(|m| {
            m.node_i.index == slot.node_i
                && m.node_j.index == slot.node_j
                && m.global_stiffness().is_ok_and(|g| {
                    g.iter()
                        .flatten()
                        .zip(slot.global_stiffness.iter().flatten())
                        .all(|(a, b)| a.to_bits() == b.to_bits())
                })
        });
        match matched {
            Some(m) => source.curved.push(CurvedFormation {
                node_i: m.node_i.index,
                node_j: m.node_j.index,
                coordinates_i: m.node_i.coordinates,
                coordinates_j: m.node_j.coordinates,
                center: m.center,
                elastic_modulus: m.elastic_modulus,
                shear_modulus: m.shear_modulus,
                area: m.area,
                second_moment: m.second_moment,
                torsion_constant: m.torsion_constant,
                in_plane_flexibility_factor: m.in_plane_flexibility_factor,
                out_of_plane_flexibility_factor: m.out_of_plane_flexibility_factor,
            }),
            None => source
                .unavailable
                .push(format!("curved_bend_source_unmatched:{}", slot.element_id)),
        }
    }
    source
}

// ------------------------------------------------------------------ K2b

/// K2b: the existing entries refuse an evidence formed at 2^b ≠ 1 (fail
/// closed); only its force-scaled entries unscale for publication. Every
/// evidence built by `new` is unscaled, so this never refuses one.
fn unscaled_evidence(force_scale: ForceScale) -> Result<(), StructuralError> {
    if force_scale.is_unscaled() {
        Ok(())
    } else {
        Err(StructuralError::InvalidInput(
            "force-scaled assembly evidence: use its force-scaled entry",
        ))
    }
}

type ForceScaledPrimitives = (
    Vec<FrameElement>,
    Vec<UserStiffnessElement>,
    Vec<CurvedBendStiffnessElement>,
    Vec<(usize, f64)>,
);

/// K2b: the evidence primitives times 2^b, exactly (E and G of each frame;
/// each user stiffness; each curved slot's global matrix and formation
/// allowances; each spring), in their given order.
fn force_scaled_primitives(
    frames: &[FrameElement],
    users: &[UserStiffnessElement],
    curved: &[CurvedBendStiffnessElement],
    springs: &[(usize, f64)],
    force_scale: ForceScale,
) -> Result<ForceScaledPrimitives, StructuralError> {
    let range = |_: FrameKernelError| {
        StructuralError::Range("force-scaled formation outside the normal range")
    };
    let frames = frames
        .iter()
        .map(|element| element.force_scaled(force_scale).map_err(range))
        .collect::<Result<Vec<_>, _>>()?;
    let users = users
        .iter()
        .map(|element| element.force_scaled(force_scale).map_err(range))
        .collect::<Result<Vec<_>, _>>()?;
    let curved = curved
        .iter()
        .map(|slot| {
            let symmetry_formation = match &slot.symmetry_formation {
                Some((bounds, counts)) => Some((
                    force_scaled_matrix("curved allowance*2^b", bounds, force_scale)
                        .map_err(range)?,
                    *counts,
                )),
                None => None,
            };
            Ok(CurvedBendStiffnessElement {
                element_id: slot.element_id.clone(),
                node_i: slot.node_i,
                node_j: slot.node_j,
                global_stiffness: force_scaled_matrix(
                    "curved entry*2^b",
                    &slot.global_stiffness,
                    force_scale,
                )
                .map_err(range)?,
                symmetry_formation,
            })
        })
        .collect::<Result<Vec<_>, StructuralError>>()?;
    let springs = springs
        .iter()
        .map(|&(dof, stiffness)| {
            force_scaled_value("spring stiffness*2^b", stiffness, force_scale)
                .map(|scaled| (dof, scaled))
                .map_err(range)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((frames, users, curved, springs))
}

/// K2b: the formation source of an evidence formed at 2^b. Its frames, users
/// and springs are already the scaled primitives. Each curved slot (formed at
/// 2^b) is matched to the macro element whose global matrix, times 2^b, has
/// its bits, and is re-formed with E and G times 2^b (K-D5's Wide<2>
/// re-formation then scales exactly: its exponent is 64-bit). A slot with no
/// match, an explicit slot, or a matched element whose E or G cannot be
/// scaled normally is named as not re-formable (fail closed). With
/// `UNSCALED` it is `formation_source`.
fn force_scaled_formation_source(
    primitives: &FormationPrimitives,
    curved_sources: &[CurvedBendMacroElement],
    force_scale: ForceScale,
) -> FormationSource {
    if force_scale.is_unscaled() {
        return formation_source(primitives, curved_sources);
    }
    let mut source = FormationSource {
        node_count: primitives.node_count,
        frames: primitives.frames.clone(),
        users: primitives.users.clone(),
        curved: Vec::new(),
        springs: primitives.springs.clone(),
        unavailable: Vec::new(),
    };
    for slot in &primitives.curved {
        if slot.explicit {
            source
                .unavailable
                .push(format!("curved_bend_explicit_matrix:{}", slot.element_id));
            continue;
        }
        let matched = curved_sources.iter().find(|m| {
            m.node_i.index == slot.node_i
                && m.node_j.index == slot.node_j
                && m.global_stiffness()
                    .ok()
                    .and_then(|g| force_scaled_matrix("curved entry*2^b", &g, force_scale).ok())
                    .is_some_and(|g| {
                        g.iter()
                            .flatten()
                            .zip(slot.global_stiffness.iter().flatten())
                            .all(|(a, b)| a.to_bits() == b.to_bits())
                    })
        });
        let Some(m) = matched else {
            source
                .unavailable
                .push(format!("curved_bend_source_unmatched:{}", slot.element_id));
            continue;
        };
        let moduli = force_scaled_value("E*2^b", m.elastic_modulus, force_scale).and_then(|e| {
            force_scaled_value("G*2^b", m.shear_modulus, force_scale).map(|g| (e, g))
        });
        match moduli {
            Ok((elastic_modulus, shear_modulus)) => source.curved.push(CurvedFormation {
                node_i: m.node_i.index,
                node_j: m.node_j.index,
                coordinates_i: m.node_i.coordinates,
                coordinates_j: m.node_j.coordinates,
                center: m.center,
                elastic_modulus,
                shear_modulus,
                area: m.area,
                second_moment: m.second_moment,
                torsion_constant: m.torsion_constant,
                in_plane_flexibility_factor: m.in_plane_flexibility_factor,
                out_of_plane_flexibility_factor: m.out_of_plane_flexibility_factor,
            }),
            Err(_) => source
                .unavailable
                .push(format!("curved_bend_source_unscalable:{}", slot.element_id)),
        }
    }
    source
}

/// K2b: a force-scaled solve's result unscaled for publication, or its error
/// with the payload unscaled.
fn force_scaled_outcome(
    solved: Result<StructuralSolution, StructuralError>,
    force_scale: ForceScale,
    f: &AssembledForce,
) -> Result<ForceScaledSolution, ForceScaledError> {
    match solved {
        Ok(solution) => Ok(structural::unscale_structural_solution(
            solution,
            force_scale,
            f,
        )),
        Err(error) => Err(ForceScaledError::Structural(
            structural::unscale_structural_error(error, force_scale),
        )),
    }
}

/// K2b: the evidence representation `solve_with_force_scaling` uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceRepresentation {
    /// `AssemblyEvidence` on the dense view of the kernel's assembly (the
    /// product's representation today).
    Dense,
    /// `SparseAssemblyEvidence` on the pattern (K1).
    Pattern,
}

/// K2b: one linear case as the facade forms it, for `solve_with_force_scaling`.
#[derive(Clone, Copy)]
pub struct ForceScalingCase<'a> {
    pub node_count: usize,
    pub frames: &'a [FrameElement],
    pub users: &'a [UserStiffnessElement],
    /// Realized curved bends formed at b = 0, added to the stiffness as
    /// blocks in this order (after frames and users, before springs).
    pub curved: &'a [CurvedBendStiffnessElement],
    /// The macro elements of the curved slots, for K-D5's re-formation.
    pub curved_sources: &'a [CurvedBendMacroElement],
    pub springs: &'a [(usize, f64)],
    /// The case's ledger force, unscaled.
    pub force: &'a AssembledForce,
    /// Every boundary DOF with its value (0.0 for a restrained DOF); every
    /// other DOF is free.
    pub prescribed: &'a [(usize, f64)],
    pub mode: LinearSolveMode,
    /// K-D5's selection (false for an invocation with a nonlinear support).
    pub selected: bool,
    pub representation: EvidenceRepresentation,
}

/// K2b: the result of `solve_with_force_scaling`.
#[derive(Debug, Clone, PartialEq)]
pub struct ForceScalingOutcome {
    /// Unscaled for publication; `solution.force_scale` is the b used (0 when
    /// the evaluation at b = 0 succeeded).
    pub solution: ForceScaledSolution,
    /// The stiffness the solve used, formed at that 2^b (for the published
    /// reactions, `SparseStiffness::force_scaled_reactions`).
    pub stiffness: SparseStiffness,
}

/// One evaluation of a case at one scale.
enum Evaluation {
    /// A range trigger (D1 §4.7 step 1): K2a's `NumericalRange` at formation,
    /// or `StructuralError::Range` from the evidence or the M03 solve.
    Range(RangeTrigger),
    /// Any other outcome, as today.
    Failed(ForceScaledError),
}

/// The census of a case (steps 2–3's inputs): every frame, user element,
/// curved slot, spring and load term.
fn force_scale_census(case: &ForceScalingCase<'_>) -> Result<ForceScaleCensus, FrameKernelError> {
    let mut census = ForceScaleCensus::new();
    for element in case.frames {
        census.frame(element)?;
    }
    for element in case.users {
        census.user(element);
    }
    for slot in case.curved {
        census.matrix(&slot.global_stiffness);
    }
    for &(_, stiffness) in case.springs {
        census.spring(stiffness);
    }
    for term in case.force.terms() {
        census.load_term(term);
    }
    Ok(census)
}

/// One evaluation at 2^b: the kernel's assembly formed at 2^b, the evidence
/// formed at 2^b in the case's representation, and the force-scaled
/// formation-checked solve, unscaled for publication.
fn evaluate_force_scaled(
    case: &ForceScalingCase<'_>,
    force_scale: ForceScale,
) -> Result<ForceScalingOutcome, Evaluation> {
    let blocks: Vec<StiffnessBlock> = case
        .curved
        .iter()
        .map(|slot| StiffnessBlock {
            node_i: slot.node_i,
            node_j: slot.node_j,
            stiffness: slot.global_stiffness,
        })
        .collect();
    let stiffness = assemble_sparse_stiffness(
        case.node_count,
        case.frames,
        case.users,
        &blocks,
        case.springs,
        &SparseAssemblyOptions::new().with_force_scale(force_scale),
    )
    .map_err(|error| match error {
        FrameKernelError::NumericalRange { .. } => {
            Evaluation::Range(RangeTrigger::Formation(error))
        }
        error => Evaluation::Failed(ForceScaledError::Formation(error)),
    })?;
    let mut boundary = vec![false; stiffness.dimension()];
    for &(dof, _) in case.prescribed {
        if let Some(slot) = boundary.get_mut(dof) {
            *slot = true;
        }
    }
    let free: Vec<usize> = (0..boundary.len()).filter(|&dof| !boundary[dof]).collect();
    let solved = match case.representation {
        EvidenceRepresentation::Dense => AssemblyEvidence::new_force_scaled(
            case.node_count,
            case.frames,
            case.users,
            case.curved,
            case.springs,
            force_scale,
        )
        .map_err(ForceScaledError::Structural)
        .and_then(|evidence| {
            evidence.solve_force_scaled_with_formation_check(
                &stiffness.to_dense(),
                case.force,
                &free,
                case.prescribed,
                case.mode,
                case.curved_sources,
                case.selected,
            )
        }),
        EvidenceRepresentation::Pattern => SparseAssemblyEvidence::new_force_scaled(
            stiffness.pattern(),
            case.node_count,
            case.frames,
            case.users,
            case.curved,
            case.springs,
            force_scale,
        )
        .map_err(ForceScaledError::Structural)
        .and_then(|evidence| {
            evidence.solve_force_scaled_with_formation_check(
                &stiffness,
                case.force,
                &free,
                case.prescribed,
                case.mode,
                case.curved_sources,
                case.selected,
            )
        }),
    };
    match solved {
        Ok(solution) => Ok(ForceScalingOutcome {
            solution,
            stiffness,
        }),
        Err(ForceScaledError::Structural(StructuralError::Range(reason))) => Err(
            Evaluation::Range(RangeTrigger::Evaluation(StructuralError::Range(reason))),
        ),
        Err(ForceScaledError::Refused(ForceScalingRefusal {
            reason: ForceScaleReason::ScaledEvaluation,
            ..
        })) => Err(Evaluation::Range(RangeTrigger::Evaluation(
            StructuralError::Range("force-scaled load terms outside the normal range"),
        ))),
        Err(error) => Err(Evaluation::Failed(error)),
    }
}

/// K2b (T3 D1 revision 5a.2 §4.7, W2, with ROOT's K2b rulings): one linear
/// case through the normative b-rule.
/// 1. **Evaluate at b = 0**, as today: the kernel's assembly, the evidence
///    and the formation-checked solve. A result, or any error other than a
///    range trigger (K2a's `NumericalRange` at formation;
///    `StructuralError::Range` from the evidence or the solve), is returned
///    as it is: the solution is then the unscaled entry's, byte for byte.
/// 2. **The census** (`ForceScaleCensus`): a subnormal input is refused.
/// 3. **The window** and the even b (`ForceScaleCensus::force_scale`).
/// 4. **Evaluate once at that b.** A range trigger again is refused as
///    "range: scaled evaluation outside normal range". There is no third
///    attempt.
/// 5. **Unscaling for publication** (inside the force-scaled entries): the
///    report's records are unscaled, each residual-record field with its
///    explicit outcome (ROOT's K2b checkpoint-A ruling B). The published
///    reactions (`SparseStiffness::force_scaled_reactions`, on the returned
///    stiffness) and actions (`structural::unscale_for_publication`) refuse a
///    value that would underflow or overflow, as "range: publication outside
///    binary64", never flushed.
///
/// Every refusal carries the step-1 trigger, so K2a's names survive.
pub fn solve_with_force_scaling(
    case: &ForceScalingCase<'_>,
) -> Result<ForceScalingOutcome, ForceScaledError> {
    let trigger = match evaluate_force_scaled(case, ForceScale::UNSCALED) {
        Ok(outcome) => return Ok(outcome),
        Err(Evaluation::Range(trigger)) => trigger,
        Err(Evaluation::Failed(error)) => return Err(error),
    };
    let refuse = |reason: ForceScaleReason| {
        ForceScaledError::Refused(ForceScalingRefusal {
            reason,
            trigger: Some(trigger.clone()),
        })
    };
    let force_scale = force_scale_census(case)
        .map_err(ForceScaledError::Formation)?
        .force_scale()
        .map_err(refuse)?;
    match evaluate_force_scaled(case, force_scale) {
        Ok(outcome) => Ok(outcome),
        Err(Evaluation::Range(_)) => Err(refuse(ForceScaleReason::ScaledEvaluation)),
        Err(Evaluation::Failed(ForceScaledError::Refused(refusal))) => Err(refuse(refusal.reason)),
        Err(Evaluation::Failed(error)) => Err(error),
    }
}

/// Named, unchanged binary64 variant of `sparse_direct::structural::
/// solve_structural_sparse` for the nonlinear active-set loop (ROOT option (c)):
/// the same prepare, skyline LDL, witness and completion sequence, on the
/// binary64 kernel path.
pub fn solve_structural_sparse_binary64(
    system: &StructuralSystem<'_>,
) -> Result<StructuralSolution, StructuralError> {
    solve_prepared(
        structural::prepare_structural_binary64(system)?,
        LinearSolveMode::SparseInteractive,
    )
}

fn solve_prepared(
    prepared: structural::PreparedSystem<'_>,
    mode: LinearSolveMode,
) -> Result<StructuralSolution, StructuralError> {
    let factor = match mode {
        LinearSolveMode::DenseScrutiny => structural::factor_structural_cholesky(&prepared),
        LinearSolveMode::SparseInteractive => {
            open_pipe_stress_sparse_direct::structural::factor_structural_ldlt(&prepared)
        }
    };
    let factor = match factor {
        Ok(factor) => factor,
        Err(error) => return Err(structural::negative_pair_witness(&prepared)?.unwrap_or(error)),
    };
    structural::finish_structural(&factor)
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
    retained: exact::RetainedResponse,
    admitted_limits: exact::Limits,
    ordinary_state: crate::LinearizedSolve,
    selected_state: SelectedProjectionState,
    decision_sources: Vec<GapDecision>,
    work_snapshot: ExactGapWork,
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
/// The selected projected state and its actual floating equilibrium report are
/// distinct from ordinary precondition state/report and exact retained ratios.
#[derive(Debug, Clone, PartialEq)]
struct SelectedProjectionState {
    displacements: Vec<f64>,
    reactions: Vec<f64>,
    equilibrium: crate::product_equilibrium::ProductEquilibriumReport,
}
impl ExactGapReport {
    pub fn retained(&self) -> &exact::RetainedResponse {
        &self.retained
    }
    pub fn ordinary_reactions(&self) -> &[f64] {
        &self.ordinary_state.reactions
    }
    pub fn ordinary_structural_report(&self) -> &structural::StructuralReport {
        &self.ordinary_state.structural_report
    }
    pub fn ordinary_equilibrium_report(
        &self,
    ) -> &crate::product_equilibrium::ProductEquilibriumReport {
        &self.ordinary_state.product_equilibrium
    }
    pub fn selected_displacements(&self) -> &[f64] {
        &self.selected_state.displacements
    }
    pub fn selected_reactions(&self) -> &[f64] {
        &self.selected_state.reactions
    }
    pub fn selected_equilibrium_report(
        &self,
    ) -> &crate::product_equilibrium::ProductEquilibriumReport {
        &self.selected_state.equilibrium
    }
    fn summaries_match(&self) -> bool {
        let bits = |a: &[f64], b: &[f64]| {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
        };
        let source = self.retained.source_system();
        if self.admitted_limits != self.retained.limits()
            || self.work != self.work_snapshot
            || self.source_identity != self.retained.source_identity()
            || self.source_level != GAP_SOURCE_LEVEL
            || self.free_dofs != source.free_dofs
            || self.prescribed.len() != source.prescribed.len()
            || !self
                .prescribed
                .iter()
                .zip(source.prescribed)
                .all(|((i, a), (j, b))| i == j && a.to_bits() == b.to_bits())
            || !bits(
                &self.ordinary_displacements,
                &self.ordinary_state.displacements,
            )
            || self.decisions != self.decision_sources
            || self.decisions.len() != self.retained.contacts().len()
            || self.projections.len() != self.retained.projections().len()
        {
            return false;
        }
        for (d, c) in self.decisions.iter().zip(self.retained.contacts()) {
            if d.global_dof != c.dof()
                || d.sense != c.sense()
                || d.gap.to_bits() != c.gap().to_bits()
                || d.gap_sign != c.penetration_sign()
                || d.reaction_sign != c.reaction_sign()
            {
                return false;
            }
            use open_pipe_stress_nonlinear_supports::ActiveSetState;
            let active = if d.prior == ActiveSetState::Active {
                c.reaction_sign() != exact::Sign::Positive
            } else if d.prior == ActiveSetState::Inactive {
                c.penetration_sign() != exact::Sign::Negative
            } else {
                return false;
            };
            if (d.state == ActiveSetState::Active) != active
                || !matches!(d.state, ActiveSetState::Active | ActiveSetState::Inactive)
            {
                return false;
            }
        }
        for (p, r) in self.projections.iter().zip(self.retained.projections()) {
            if p.quantity != r.quantity()
                || p.global_dof != r.dof()
                || p.value.to_bits() != r.value().to_bits()
                || !bits(&p.interval, &r.interval())
                || p.absolute_error_bound.to_bits() != r.absolute_error_bound().to_bits()
                || p.relative_error_bound.to_bits() != r.relative_error_bound().to_bits()
                || p.criterion.to_bits() != r.relative_limit().to_bits()
                || p.criterion != GAP_PROJECTION_RELATIVE_LIMIT
                || p.basis != r.basis()
            {
                return false;
            }
            let values = match p.quantity {
                exact::Quantity::Displacement => &self.selected_state.displacements,
                exact::Quantity::Reaction => &self.selected_state.reactions,
            };
            if values.get(p.global_dof).map(|v| v.to_bits()) != Some(p.value.to_bits()) {
                return false;
            }
        }
        self.corrected_coordinate_count
            == self
                .ordinary_state
                .displacements
                .iter()
                .zip(&self.selected_state.displacements)
                .filter(|(a, b)| a.to_bits() != b.to_bits())
                .count()
    }
    /// Reserve the complete metadata comparison before inspecting string bytes.
    /// Both public and private sides count, so mutated larger summaries are also
    /// bounded. First accept/debit the O(1) container-length sizing prefix;
    /// then visit admitted entries to reserve dynamic comparison bytes. A late
    /// byte/overflow rejection retains the prefix charge and rejects only the
    /// unexecuted byte reservation (usize::MAX marks its saturated size).
    fn summary_comparison_charge(&self, limit: usize) -> Result<usize, exact::WorkReport> {
        fn add(total: &mut usize, count: usize, width: usize, limit: usize) -> Result<(), usize> {
            let amount = count.checked_mul(width).ok_or(usize::MAX)?;
            *total = total.checked_add(amount).ok_or(usize::MAX)?;
            if *total > limit {
                return Err(*total);
            }
            Ok(())
        }
        let source = self.retained.source_system();
        let mut total = 256;
        if total > limit {
            return Err(exact::WorkReport {
                charged: 0,
                rejected: total,
                limit,
            });
        }
        // Fixed scalar fields, length checks, repeated numeric summary loops,
        // and construction of the expected-contact triples are covered here.
        for count in [
            self.free_dofs.len(),
            source.free_dofs.len(),
            self.prescribed.len(),
            source.prescribed.len(),
            self.ordinary_displacements.len(),
            self.ordinary_state.displacements.len(),
            self.selected_state.displacements.len(),
            self.selected_state.reactions.len(),
            self.decisions.len(),
            self.decision_sources.len(),
            self.retained.contacts().len(),
            self.projections.len(),
            self.retained.projections().len(),
            self.work.attempts.len(),
            self.work_snapshot.attempts.len(),
        ] {
            add(&mut total, count, 64, limit).map_err(|rejected| exact::WorkReport {
                charged: 0,
                rejected,
                limit,
            })?;
        }
        // The complete sizing traversal is now conservatively charged, even if
        // it stops early. Dynamic comparison bytes have not yet been accepted.
        let prefix = total;
        let remaining = limit - prefix;
        let late_rejection = |rejected| exact::WorkReport {
            charged: prefix,
            rejected,
            limit,
        };
        let mut dynamic = 0;
        for bytes in [
            self.source_identity.len(),
            self.retained.source_identity().len(),
            self.source_level.len(),
            GAP_SOURCE_LEVEL.len(),
        ] {
            add(&mut dynamic, bytes, 1, remaining).map_err(late_rejection)?;
        }
        for work in [&self.work, &self.work_snapshot] {
            for (label, _, error) in &work.attempts {
                add(&mut dynamic, label.len(), 1, remaining).map_err(late_rejection)?;
                add(
                    &mut dynamic,
                    error.as_ref().map_or(0, String::len),
                    1,
                    remaining,
                )
                .map_err(late_rejection)?;
            }
        }
        for decisions in [&self.decisions, &self.decision_sources] {
            for decision in decisions {
                add(&mut dynamic, decision.support_id.len(), 1, remaining)
                    .map_err(late_rejection)?;
            }
        }
        // Bounds above establish prefix + dynamic <= limit without overflow.
        Ok(prefix + dynamic)
    }
    /// Private-owned retained evidence survives dropping Context. This verifies
    /// summaries and exact replay against independently supplied source; it does
    /// not emit a public method receipt or establish a passing case policy.
    pub fn replay_retained_against(
        &self,
        expected: &StructuralSystem<'_>,
        identity: &str,
        force_basis: exact::ForceBasis<'_>,
        work_limit: usize,
    ) -> exact::Attempt<exact::ReplayCheck> {
        let limit = work_limit.min(self.retained.limits().operations);
        let charge = match self.summary_comparison_charge(limit) {
            Ok(charge) => charge,
            Err(work) => {
                return exact::Attempt {
                    result: Err(exact::Error::Budget),
                    work,
                }
            }
        };
        if !self.summaries_match() {
            return exact::Attempt {
                result: Err(exact::Error::Invalid("retained adapter summary mismatch")),
                work: exact::WorkReport {
                    charged: charge,
                    rejected: 0,
                    limit,
                },
            };
        }
        let contacts = self
            .decision_sources
            .iter()
            .map(|d| (d.global_dof, d.sense, d.gap))
            .collect::<Vec<_>>();
        let mut attempt = self.retained.replay_against(
            expected,
            identity,
            force_basis,
            &contacts,
            GAP_PROJECTION_RELATIVE_LIMIT,
            self.admitted_limits,
            limit - charge,
        );

        attempt.work.charged = attempt.work.charged.saturating_add(charge);
        attempt.work.limit = limit;
        attempt
    }
}
const GAP_SOURCE_LEVEL:&str="represented frame/spring contributions; declared aggregate nodal load entries; exact original Kfc and prescribed values";
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
// Conservative copy reservations in bytes-as-work-units, including nested
// dynamic rows/strings. This is not a hardware operation or wall-time count.
fn state_retention_charge(
    ordinary: &crate::LinearizedSolve,
    projected: &crate::product_equilibrium::ProductEquilibriumReport,
    decisions: &[GapDecision],
    work: &ExactGapWork,
) -> usize {
    let mut n = std::mem::size_of_val(ordinary)
        .saturating_add(ordinary.displacements.len().saturating_mul(32))
        .saturating_add(ordinary.reactions.len().saturating_mul(24));
    let r = &ordinary.structural_report;
    for bytes in [
        std::mem::size_of_val(r.scale_exponents.as_slice()),
        std::mem::size_of_val(r.pivots.as_slice()),
        std::mem::size_of_val(r.residual_rows.as_slice()),
        std::mem::size_of_val(r.intended_residual_rows.as_slice()),
        std::mem::size_of_val(r.contribution_rounding.as_slice()),
    ] {
        n = n.saturating_add(bytes);
    }
    for c in &r.contribution_rounding {
        n = n.saturating_add(
            c.accumulated_expansion
                .len()
                .saturating_add(c.difference_expansion.len())
                .saturating_mul(8),
        );
    }
    n = n.saturating_add(r.symmetry_basis.as_ref().map_or(0, String::len));
    for e in [&ordinary.product_equilibrium, projected] {
        n = n
            .saturating_add(std::mem::size_of_val(e.rows.as_slice()))
            .saturating_add(std::mem::size_of_val(e.work_rows.as_slice()));
    }
    let sparse = &ordinary.sparse_evidence;
    for text in [
        &sparse.policy_ref,
        &sparse.solution_basis,
        &sparse.assembly_basis,
    ] {
        n = n.saturating_add(text.len());
    }
    n = n.saturating_add(sparse.failure_message.as_ref().map_or(0, String::len));
    for d in decisions {
        n = n
            .saturating_add(std::mem::size_of_val(d))
            .saturating_add(d.support_id.len());
    }
    for (label, _, error) in &work.attempts {
        n = n
            .saturating_add(label.len())
            .saturating_add(error.as_ref().map_or(0, String::len))
            .saturating_add(64);
    }
    n.saturating_mul(2).saturating_add(256)
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
        let mut contact_proofs = Vec::new();
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
            let proof = gap_attempt(
                response.gap_proof_with_work(
                    dof,
                    sense,
                    gap,
                    per_call.saturating_sub(work.charged),
                ),
                &mut work,
                format!("gap operands {}", support.support_id),
            )?;
            let sign = proof.contact().penetration_sign();
            let reaction = proof.contact().reaction_sign();
            contact_proofs.push(proof);
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
        let mut projection_proofs = Vec::new();
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
                projection_proofs.push(p);
            }
        }
        let retained = gap_attempt(
            response.retain_with_work(
                &contact_proofs,
                &projection_proofs,
                per_call.saturating_sub(work.charged),
            ),
            &mut work,
            "retain source/minors/ratios/contact/projection".into(),
        )?;
        let equilibrium = crate::product_equilibrium::evaluate(&system, &u)?;
        if !equilibrium.passed {
            return Err(exact::Error::ProjectionUnresolved(
                "projected exact response fails original represented equilibrium gate",
            ));
        }
        let state_charge =
            state_retention_charge(&solve.linearized, &equilibrium, &decisions, &work);
        let remaining = per_call.saturating_sub(work.charged);
        gap_attempt(
            exact::Attempt {
                result: if state_charge <= remaining {
                    Ok(())
                } else {
                    Err(exact::Error::Budget)
                },
                work: exact::WorkReport {
                    charged: if state_charge <= remaining {
                        state_charge
                    } else {
                        0
                    },
                    rejected: if state_charge <= remaining {
                        0
                    } else {
                        state_charge
                    },
                    limit: remaining,
                },
            },
            &mut work,
            "retain ordinary/projected states and reports".into(),
        )?;
        let ordinary_state = solve.linearized.clone();
        let selected_state = SelectedProjectionState {
            displacements: u.clone(),
            reactions: reactions.clone(),
            equilibrium: equilibrium.clone(),
        };
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
        Ok(ExactGapReport {
            admitted_limits: retained.limits(),
            retained,
            ordinary_state,
            selected_state,
            decision_sources: decisions.clone(),
            work_snapshot: work.clone(),
            source_identity: identity,
            source_level: GAP_SOURCE_LEVEL,
            free_dofs: free,
            prescribed,
            decisions,
            projections,
            ordinary_displacements: ordinary,
            corrected_coordinate_count: corrected,
            work: work.clone(),
        })
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
    if !report.summaries_match() || report.decisions.len() != input.supports.len() {
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

#[cfg(test)]
#[path = "structural_adapter/kd5_tests.rs"]
pub(crate) mod kd5_tests;

#[cfg(test)]
#[path = "structural_adapter/k1_tests.rs"]
mod k1_tests;

#[cfg(test)]
#[path = "structural_adapter/k2b_tests.rs"]
mod k2b_tests;

#[cfg(test)]
#[path = "structural_adapter/k5_tests.rs"]
mod k5_tests;

#[cfg(test)]
mod retention_tests {
    use super::*;
    use crate::{ConvergenceControl, ConvergencePolicyStatus, NonlinearFrameSolveInput};
    use open_pipe_stress_frame_kernel::{FrameDof, FrameNode, FrameSection};
    use open_pipe_stress_nonlinear_supports::{
        ActiveSetState, GapDirection, NonlinearSupport, SupportStateRecord,
    };
    fn adjacent_input() -> NonlinearFrameSolveInput {
        let section = FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0).unwrap();
        let elements = (0..2)
            .map(|i| {
                FrameElement::new(
                    FrameNode::new(i, [i as f64, 0.0, 0.0]).unwrap(),
                    FrameNode::new(i + 1, [(i + 1) as f64, 0.0, 0.0]).unwrap(),
                    section,
                    [0.0, 1.0, 0.0],
                )
                .unwrap()
            })
            .collect();
        let mut force = vec![0.0; 18];
        force[12] = 12.5;
        NonlinearFrameSolveInput {
            node_count: 3,
            elements,
            user_stiffness_elements: vec![],
            curved_bend_elements: vec![],
            force,
            base_restrained_dofs: (0..18).filter(|&i| i != 6 && i != 12).collect(),
            nonlinear_supports: vec![
                NonlinearSupport::gap(
                    "g1",
                    1,
                    FrameDof::Ux,
                    f64::from_bits(0.125_f64.to_bits() - 1),
                    GapDirection::PositiveDisplacement,
                )
                .unwrap(),
                NonlinearSupport::gap(
                    "g2",
                    2,
                    FrameDof::Ux,
                    0.25,
                    GapDirection::PositiveDisplacement,
                )
                .unwrap(),
            ],
            initial_states: vec![
                SupportStateRecord::new("g1", ActiveSetState::Active),
                SupportStateRecord::new("g2", ActiveSetState::Inactive),
            ],
            friction_normal_reactions: vec![],
            derived_friction_normal_reactions: vec![],
            convergence: ConvergenceControl::new(
                "DEC-046-fixture-active-set-count-tightening",
                ConvergencePolicyStatus::Accepted,
                0.0,
                0.0,
                4,
            )
            .unwrap(),
        }
    }
    #[test]
    fn ret01_long_id_and_nested_error_are_reserved_before_summary_comparison() {
        let mut comparison_charges = Vec::new();
        for id in ["g1".to_string(), "x".repeat(4096)] {
            let mut input = adjacent_input();
            input.nonlinear_supports[0].support_id = id.clone();
            input.initial_states[0].support_id = id;
            let mode = LinearSolveMode::DenseScrutiny;
            let solved = crate::solve_active_set_frame_with_mode(&input, mode).unwrap();
            let iteration = solved.iterations.last().unwrap();
            let report = match &iteration.strict_gap {
                StrictGapEvidence::Qualified(r) => r,
                other => panic!("{other:?}"),
            };
            let stiffness = crate::assemble_global_stiffness_with_user_elements(
                input.node_count,
                &input.elements,
                &[],
            )
            .unwrap();
            let assembly =
                AssemblyEvidence::new(input.node_count, &input.elements, &[], &[], &[]).unwrap();
            let fixed = (0..18)
                .filter(|&d| d != 12)
                .map(|d| {
                    (
                        d,
                        if d == 6 {
                            f64::from_bits(0.125_f64.to_bits() - 1)
                        } else {
                            0.0
                        },
                    )
                })
                .collect::<Vec<_>>();
            let terms = input
                .force
                .iter()
                .enumerate()
                .map(|(dof, &value)| exact::ForceContribution {
                    source: format!("declared input.force[{dof}]"),
                    dof,
                    value,
                })
                .collect::<Vec<_>>();
            let source = StructuralSystem {
                stiffness: &stiffness,
                force: &input.force,
                free_dofs: &[12],
                prescribed: &fixed,
                contributions: Some(&assembly.contributions),
                symmetry: Some(SymmetryEvidence {
                    absolute_roundoff: &assembly.absolute_roundoff,
                    operation_counts: &assembly.operation_counts,
                    basis: "original represented assembly, no symmetry projection in exact proof",
                }),
            };
            let identity=format!("nonlinear original partition iteration={}; mode={}; declared-force-plus-selected-affine-source",iteration.iteration,mode.as_str());
            let replay = |r: &ExactGapReport, limit| {
                r.replay_retained_against(
                    &source,
                    &identity,
                    exact::ForceBasis::IdentifiedContributions(&terms),
                    limit,
                )
            };
            let charge = report.summary_comparison_charge(usize::MAX).unwrap();
            comparison_charges.push(charge);
            // RET-01's old fixed prefix admitted 1088 and compared long strings.
            // The repaired reservation rejects before summaries_match instead.
            let tight = replay(report, 1088);
            assert!(matches!(tight.result, Err(exact::Error::Budget)));
            assert_eq!(tight.work.charged, 0);
            assert!(tight.work.rejected > 1088);
            let before_comparison = replay(report, charge - 1);
            assert!(matches!(
                before_comparison.result,
                Err(exact::Error::Budget)
            ));
            assert!(before_comparison.work.charged > 0);
            assert!(before_comparison.work.charged < charge);
            assert_eq!(
                before_comparison.work.charged + before_comparison.work.rejected,
                charge
            );
            // Prefix accepted; only unexecuted dynamic bytes were rejected.
            let sizing_prefix = before_comparison.work.charged;
            let sufficient = replay(report, usize::MAX);
            assert!(sufficient.result.is_ok(), "{:?}", sufficient.result);
            assert!(sufficient.work.charged > charge);
            let one_short = replay(report, sufficient.work.charged - 1);
            assert!(matches!(one_short.result, Err(exact::Error::Budget)));
            assert!(one_short.work.rejected > 0);
            assert!(one_short.work.charged >= charge);
            // Public failed-attempt strings are compared even though this honest
            // successful report had no errors. Mutated metadata must be sized
            // before rejecting its mismatch with the immutable work snapshot.
            let mut changed = report.clone();
            changed.work.attempts[0].2 = Some("e".repeat(4096));
            assert_eq!(
                changed.summary_comparison_charge(usize::MAX).unwrap(),
                charge + 4096
            );
            let denied = replay(&changed, charge);
            assert!(matches!(denied.result, Err(exact::Error::Budget)));
            assert_eq!(denied.work.charged, sizing_prefix);
            assert!(denied.work.rejected > charge - sizing_prefix);
            assert!(matches!(
                replay(&changed, usize::MAX).result,
                Err(exact::Error::Invalid("retained adapter summary mismatch"))
            ));
        }
        // Both decision IDs and work labels have public/private copies.
        assert!(comparison_charges[1] >= comparison_charges[0] + 4 * (4096 - 2));
    }
    #[test]
    fn actual_adapter_retains_adjacent_response_and_separate_ordinary_projected_states() {
        for mode in [
            LinearSolveMode::DenseScrutiny,
            LinearSolveMode::SparseInteractive,
        ] {
            let input = adjacent_input();
            let solved = crate::solve_active_set_frame_with_mode(&input, mode).unwrap();
            let iteration = solved.iterations.last().unwrap();
            let report = match &iteration.strict_gap {
                StrictGapEvidence::Qualified(r) => r,
                other => panic!("{other:?}"),
            };
            // scrutinize_gaps' temporary Context has already been dropped.
            let retained = report.retained();
            assert_eq!(retained.displacements().len(), 18);
            assert_eq!(retained.reactions().len(), 18);
            assert_eq!(report.selected_displacements()[12], 0.25);
            assert_eq!(report.selected_reactions()[6], -25.0 * 2.0_f64.powi(-54));
            assert_eq!(
                retained.contacts()[1].penetration_sign(),
                exact::Sign::Negative
            );
            assert_eq!(
                report.ordinary_structural_report(),
                &iteration.structural_report
            );
            assert_eq!(
                report.selected_equilibrium_report(),
                &iteration.product_equilibrium
            );
            assert!(!std::ptr::eq(
                report.ordinary_equilibrium_report(),
                report.selected_equilibrium_report()
            ));
            let stiffness = crate::assemble_global_stiffness_with_user_elements(
                input.node_count,
                &input.elements,
                &[],
            )
            .unwrap();
            let assembly =
                AssemblyEvidence::new(input.node_count, &input.elements, &[], &[], &[]).unwrap();
            let fixed = (0..18)
                .filter(|&d| d != 12)
                .map(|d| {
                    (
                        d,
                        if d == 6 {
                            f64::from_bits(0.125_f64.to_bits() - 1)
                        } else {
                            0.0
                        },
                    )
                })
                .collect::<Vec<_>>();
            let terms = input
                .force
                .iter()
                .enumerate()
                .map(|(dof, &value)| exact::ForceContribution {
                    source: format!("declared input.force[{dof}]"),
                    dof,
                    value,
                })
                .collect::<Vec<_>>();
            let source = StructuralSystem {
                stiffness: &stiffness,
                force: &input.force,
                free_dofs: &[12],
                prescribed: &fixed,
                contributions: Some(&assembly.contributions),
                symmetry: Some(SymmetryEvidence {
                    absolute_roundoff: &assembly.absolute_roundoff,
                    operation_counts: &assembly.operation_counts,
                    basis: "original represented assembly, no symmetry projection in exact proof",
                }),
            };
            let identity=format!("nonlinear original partition iteration={}; mode={}; declared-force-plus-selected-affine-source",iteration.iteration,mode.as_str());
            let replay = report.replay_retained_against(
                &source,
                &identity,
                exact::ForceBasis::IdentifiedContributions(&terms),
                usize::MAX,
            );
            assert!(replay.result.is_ok(), "{:?}", replay.result);
            assert!(replay.work.charged > 0);
            let mut changed = report.clone();
            changed.projections[12].value += 1.0;
            assert!(changed
                .replay_retained_against(
                    &source,
                    &identity,
                    exact::ForceBasis::IdentifiedContributions(&terms),
                    usize::MAX
                )
                .result
                .is_err());
            let mut changed = report.clone();
            changed.decisions[0].gap_sign = exact::Sign::Positive;
            assert!(!changed.summaries_match());
            let mut changed = report.clone();
            changed.ordinary_displacements[12] = 0.5;
            assert!(!changed.summaries_match());
            let mut changed = report.clone();
            changed.work.charged += 1;
            assert!(!changed.summaries_match());
            assert!(report
                .replay_retained_against(
                    &source,
                    &identity,
                    exact::ForceBasis::IdentifiedContributions(&terms),
                    0
                )
                .result
                .is_err());
        }
    }
}
