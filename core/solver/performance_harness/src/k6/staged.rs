//! The staged kernel sequence (K6 plan §2.3; ROOT's K6 ruling Q6(a) and N1).
//!
//! It is the product's linear entry, `SparseAssemblyEvidence::
//! solve_assembled_with_formation_check(…, selected = true)` (`SA:644-699`,
//! called at `PP@F1b:5388`), cut at its public-API boundaries, in SA's order:
//! assembly, evidence, ledger, reduce, geometry (with SA's pattern check),
//! [dense: densify], prepare, factor, [witness, after a refused factor],
//! finish, then recovery. `with_formation = false` gives the sibling of
//! `solve_assembled` (`SA:596-638`). Tests E pin the staged result, bit for
//! bit in `Debug`, to both SA entries, which the `entry_*` stages run as the
//! product calls them.
//!
//! Replicated private items of SA (ROOT's K6 ruling N5): `check_pattern`
//! (`SA:586-593`), the frame family's symmetry basis (`SA:1291-1297`), the
//! frames-only `formation_source` (`SA:1302-1357`), and the dense branch's
//! `solve_prepared` (`SA:1761-1778`), each from public calls.
//!
//! Bundled design stages (disclosed): the contribution audit is inside
//! `prepare`; rcond, the solve, the residual, the intended-action audit and
//! the formation check are inside `finish`.

use super::counts::FRAME_FAMILY_SYMMETRY_BASIS;
use super::lanes::reduced_entry_system;
use super::models::K6Model;
use super::{Fnv64, Mode};
use open_pipe_stress_frame_kernel::load_ledger::{AssembledForce, LoadLedger};
use open_pipe_stress_frame_kernel::structural::{
    assemble_sparse_stiffness, factor_sparse_structural_profile, factor_structural_cholesky,
    finish_sparse_structural, finish_structural, negative_pair_witness,
    prepare_assembled_sparse_structural, prepare_assembled_structural,
    prepare_formation_checked_sparse_structural, prepare_formation_checked_structural,
    reduce_assembled_sparse_system, sparse_negative_pair_witness, FormationSource,
    SparseAssemblyOptions, SparsePreparedSystem, SparseStiffness, SparseStructuralSystem,
    SparseSymmetryEvidence, StructuralError, StructuralSolution, StructuralSystem,
    SymmetryEvidence,
};
use open_pipe_stress_frame_kernel::{
    reduce_assembled_system, solve_dense, ForceScale, FrameElement,
};
use open_pipe_stress_nonlinear_integration::structural_adapter::SparseAssemblyEvidence;
use open_pipe_stress_nonlinear_integration::LinearSolveMode;
use open_pipe_stress_sparse_direct::solve_symmetric_system_from_entries;
use open_pipe_stress_sparse_direct::structural::order_sparse_structural;

/// A stage of the staged sequence or a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Assembly,
    Evidence,
    Ledger,
    Reduce,
    Geometry,
    Densify,
    Prepare,
    Factor,
    Witness,
    Finish,
    Recovery,
    EntryChecked,
    EntryPlain,
    LaneEntries,
    DensifyLu,
    LaneReduce,
    LaneSolve,
    /// K6b: the adapter and `PrimitiveSource::new`.
    W1Source,
    /// K6b: one `solve_case` call, the whole of K4's schedule.
    W1Solve,
    /// K6b: the j-th budget-truncated prefix call (Q1(c)), j ≥ 1.
    W1Prefix(u8),
}

impl Stage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Assembly => "assembly",
            Self::Evidence => "evidence",
            Self::Ledger => "ledger",
            Self::Reduce => "reduce",
            Self::Geometry => "geometry",
            Self::Densify => "densify",
            Self::Prepare => "prepare",
            Self::Factor => "factor",
            Self::Witness => "witness",
            Self::Finish => "finish",
            Self::Recovery => "recovery",
            Self::EntryChecked => "entry_checked",
            Self::EntryPlain => "entry_plain",
            Self::LaneEntries => "lane_entries",
            Self::DensifyLu => "densify_lu",
            Self::LaneReduce => "lane_reduce",
            Self::LaneSolve => "lane_solve",
            Self::W1Source => "w1_source",
            Self::W1Solve => "w1_solve",
            Self::W1Prefix(j) => match j {
                1 => "w1_prefix_1",
                2 => "w1_prefix_2",
                3 => "w1_prefix_3",
                4 => "w1_prefix_4",
                5 => "w1_prefix_5",
                6 => "w1_prefix_6",
                7 => "w1_prefix_7",
                8 => "w1_prefix_8",
                9 => "w1_prefix_9",
                10 => "w1_prefix_10",
                11 => "w1_prefix_11",
                _ => "w1_prefix",
            },
        }
    }
}

/// Receives stage boundaries (the binary times them and reads its allocator
/// there; tests pass `NoObserver`).
pub trait StageObserver {
    fn begin(&mut self, stage: Stage);
    fn end(&mut self, stage: Stage, ok: bool, error: Option<String>);
}

/// An observer that records nothing.
#[derive(Debug, Default)]
pub struct NoObserver;

impl StageObserver for NoObserver {
    fn begin(&mut self, _stage: Stage) {}
    fn end(&mut self, _stage: Stage, _ok: bool, _error: Option<String>) {}
}

/// A short, bounded description of a structural error (no direction vector),
/// for stage lines.
pub fn short_error(error: &StructuralError) -> String {
    match error {
        StructuralError::InvalidInput(reason) => format!("InvalidInput({reason})"),
        StructuralError::Range(reason) => format!("Range({reason})"),
        StructuralError::Asymmetric {
            row,
            col,
            relative_skew,
        } => format!("Asymmetric(row={row}, col={col}, relative_skew={relative_skew:e})"),
        StructuralError::NumericallyUnresolved { reason, global_dof } => {
            format!("NumericallyUnresolved({reason}; global_dof={global_dof:?})")
        }
        StructuralError::NegativeEnergy {
            direction,
            energy,
            allowance,
        } => format!(
            "NegativeEnergy(energy={energy:e}, allowance={allowance:e}, direction_len={})",
            direction.len()
        ),
        StructuralError::Mechanism { direction } => {
            format!("Mechanism(direction_len={})", direction.len())
        }
    }
}

/// The outcome class of a solve: the report's quality, or the error variant.
pub fn outcome_class(result: &Result<StructuralSolution, StructuralError>) -> &'static str {
    match result {
        Ok(solution) => match solution.report.quality {
            open_pipe_stress_frame_kernel::structural::SolveQuality::Passed => "Passed",
            open_pipe_stress_frame_kernel::structural::SolveQuality::Sensitive => "Sensitive",
        },
        Err(StructuralError::InvalidInput(_)) => "InvalidInput",
        Err(StructuralError::Range(_)) => "Range",
        Err(StructuralError::Asymmetric { .. }) => "Asymmetric",
        Err(StructuralError::NumericallyUnresolved { .. }) => "NumericallyUnresolved",
        Err(StructuralError::NegativeEnergy { .. }) => "NegativeEnergy",
        Err(StructuralError::Mechanism { .. }) => "Mechanism",
    }
}

/// A failure before the solver stages (formation, evidence, ledger or
/// reduction). None is expected on K6's models; it is recorded, never hidden.
#[derive(Debug, Clone, PartialEq)]
pub struct SetupError {
    pub stage: Stage,
    pub detail: String,
}

/// What the setup stages build and every later stage reads.
#[derive(Debug)]
pub struct Setup {
    pub k: SparseStiffness,
    pub evidence: SparseAssemblyEvidence,
    pub force: AssembledForce,
    pub free: Vec<usize>,
    pub prescribed: Vec<(usize, f64)>,
    pub restrained: Vec<usize>,
}

fn run<T, E: std::fmt::Debug>(
    obs: &mut dyn StageObserver,
    stage: Stage,
    f: impl FnOnce() -> Result<T, E>,
) -> Result<T, SetupError> {
    obs.begin(stage);
    let result = f();
    let detail = result.as_ref().err().map(|e| format!("{e:?}"));
    obs.end(stage, result.is_ok(), detail.clone());
    result.map_err(|_| SetupError {
        stage,
        detail: detail.unwrap_or_default(),
    })
}

/// The ledger of the model's nodal loads: one term per loaded DOF, ascending.
pub fn ledger(model: &K6Model) -> Result<AssembledForce, String> {
    let mut ledger = LoadLedger::new();
    for &(dof, value) in &model.loads {
        ledger.push(format!("load:{dof}"), dof, value);
    }
    ledger.finish(model.dofs()).map_err(|e| format!("{e:?}"))
}

/// Assembly, evidence, ledger and reduction.
pub fn setup(
    model: &K6Model,
    frames: &[FrameElement],
    obs: &mut dyn StageObserver,
) -> Result<Setup, SetupError> {
    let n_nodes = model.node_count();
    let k = run(obs, Stage::Assembly, || {
        assemble_sparse_stiffness(
            n_nodes,
            frames,
            &[],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        )
    })?;
    let evidence = run(obs, Stage::Evidence, || {
        SparseAssemblyEvidence::new(k.pattern(), n_nodes, frames, &[], &[], &[])
    })?;
    let force = run(obs, Stage::Ledger, || ledger(model))?;
    let restrained = model.restrained_dofs();
    let free = run(obs, Stage::Reduce, || {
        reduce_assembled_sparse_system(&k, &force, &restrained, None).map(|r| r.free_dofs)
    })?;
    let prescribed = restrained.iter().map(|&d| (d, 0.0)).collect();
    Ok(Setup {
        k,
        evidence,
        force,
        free,
        prescribed,
        restrained,
    })
}

/// The staged solve's result.
#[derive(Debug)]
pub struct StagedSolve {
    pub solution: Result<StructuralSolution, StructuralError>,
    /// The stage whose call returned the error, if any.
    pub failed_stage: Option<Stage>,
    /// The factor's own refusal, before the witness replaced or kept it.
    pub factor_error: Option<StructuralError>,
    /// Sparse: `order_sparse_structural`'s profile entries and half-bandwidth.
    pub ordering_counts: Option<(usize, usize)>,
}

impl StagedSolve {
    fn failed(stage: Stage, error: StructuralError) -> Self {
        Self {
            solution: Err(error),
            failed_stage: Some(stage),
            factor_error: None,
            ordering_counts: None,
        }
    }
}

fn end_result<T>(obs: &mut dyn StageObserver, stage: Stage, result: &Result<T, StructuralError>) {
    obs.end(
        stage,
        result.is_ok(),
        result.as_ref().err().map(short_error),
    );
}

/// The frames-only formation source, as SA's private `formation_source`
/// builds it from `SparseAssemblyEvidence::new`'s primitives (`SA:1302-1357`).
pub fn frames_formation_source(node_count: usize, frames: &[FrameElement]) -> FormationSource {
    FormationSource {
        node_count,
        frames: frames.to_vec(),
        users: Vec::new(),
        connectors: Vec::new(),
        curved: Vec::new(),
        springs: Vec::new(),
        unavailable: Vec::new(),
    }
}

/// Geometry through finish, in `mode` (`Sparse` or `Dense`), with or without
/// the formation source.
pub fn staged_solve(
    setup: &Setup,
    node_count: usize,
    frames: &[FrameElement],
    mode: Mode,
    with_formation: bool,
    obs: &mut dyn StageObserver,
) -> StagedSolve {
    obs.begin(Stage::Geometry);
    let geometry = if setup.k.pattern() != setup.evidence.pattern() {
        Err(StructuralError::InvalidInput(
            "stiffness pattern differs from the assembly evidence",
        ))
    } else {
        setup.evidence.geometry(&setup.prescribed)
    };
    end_result(obs, Stage::Geometry, &geometry);
    if let Err(error) = geometry {
        return StagedSolve::failed(Stage::Geometry, error);
    }
    match mode {
        Mode::Dense => dense_solve(setup, node_count, frames, with_formation, obs),
        _ => sparse_solve(setup, node_count, frames, with_formation, obs),
    }
}

fn sparse_solve(
    setup: &Setup,
    node_count: usize,
    frames: &[FrameElement],
    with_formation: bool,
    obs: &mut dyn StageObserver,
) -> StagedSolve {
    obs.begin(Stage::Prepare);
    let basis = FRAME_FAMILY_SYMMETRY_BASIS.to_string();
    let source = with_formation.then(|| frames_formation_source(node_count, frames));
    let symmetry = SparseSymmetryEvidence {
        absolute_roundoff: setup.evidence.absolute_roundoff(),
        operation_counts: setup.evidence.operation_counts(),
        basis: &basis,
    };
    let assembled = SparseStructuralSystem::assembled(
        &setup.k,
        &setup.force,
        &setup.free,
        &setup.prescribed,
        Some(setup.evidence.contributions()),
        Some(symmetry),
    );
    match &source {
        Some(source) => {
            let system = assembled.with_formation_source(source);
            let prepared = prepare_formation_checked_sparse_structural(&system);
            end_result(obs, Stage::Prepare, &prepared);
            match prepared {
                Ok(prepared) => sparse_factor_finish(&prepared, obs),
                Err(error) => StagedSolve::failed(Stage::Prepare, error),
            }
        }
        None => {
            let prepared = prepare_assembled_sparse_structural(&assembled);
            end_result(obs, Stage::Prepare, &prepared);
            match prepared {
                Ok(prepared) => sparse_factor_finish(&prepared, obs),
                Err(error) => StagedSolve::failed(Stage::Prepare, error),
            }
        }
    }
}

/// `solve_sparse_prepared` (`SD/structural.rs:88-98`) at its boundaries:
/// `factor_sparse_structural_ldlt` is `order_sparse_structural` then
/// `factor_sparse_structural_profile`.
fn sparse_factor_finish(
    prepared: &SparsePreparedSystem<'_>,
    obs: &mut dyn StageObserver,
) -> StagedSolve {
    obs.begin(Stage::Factor);
    let mut ordering_counts = None;
    let factor = match order_sparse_structural(prepared) {
        Ok(ordering) => {
            ordering_counts = Some((ordering.profile_entry_count, ordering.max_half_bandwidth));
            factor_sparse_structural_profile(prepared, &ordering.order, &ordering.first_columns)
        }
        Err(error) => Err(error),
    };
    end_result(obs, Stage::Factor, &factor);
    let factor = match factor {
        Ok(factor) => factor,
        Err(error) => {
            obs.begin(Stage::Witness);
            let witness = sparse_negative_pair_witness(prepared);
            end_result(obs, Stage::Witness, &witness);
            let solution = match witness {
                Ok(found) => Err(found.unwrap_or_else(|| error.clone())),
                Err(witness_error) => Err(witness_error),
            };
            return StagedSolve {
                solution,
                failed_stage: Some(Stage::Factor),
                factor_error: Some(error),
                ordering_counts,
            };
        }
    };
    obs.begin(Stage::Finish);
    let solution = finish_sparse_structural(&factor);
    end_result(obs, Stage::Finish, &solution);
    StagedSolve {
        failed_stage: solution.is_err().then_some(Stage::Finish),
        solution,
        factor_error: None,
        ordering_counts,
    }
}

fn dense_solve(
    setup: &Setup,
    node_count: usize,
    frames: &[FrameElement],
    with_formation: bool,
    obs: &mut dyn StageObserver,
) -> StagedSolve {
    obs.begin(Stage::Densify);
    let dense = setup.k.to_dense();
    let (roundoff, counts) = setup.evidence.dense_symmetry_view();
    obs.end(Stage::Densify, true, None);
    obs.begin(Stage::Prepare);
    let basis = FRAME_FAMILY_SYMMETRY_BASIS.to_string();
    let source = with_formation.then(|| frames_formation_source(node_count, frames));
    let system = StructuralSystem::assembled(
        &dense,
        &setup.force,
        &setup.free,
        &setup.prescribed,
        Some(setup.evidence.contributions()),
        Some(SymmetryEvidence {
            absolute_roundoff: &roundoff,
            operation_counts: &counts,
            basis: &basis,
        }),
    );
    let prepared = match &source {
        Some(source) => {
            let system = system.with_formation_source(source);
            let prepared = prepare_formation_checked_structural(&system);
            end_result(obs, Stage::Prepare, &prepared);
            match prepared {
                Ok(prepared) => return dense_factor_finish(prepared, obs),
                Err(error) => error,
            }
        }
        None => {
            let prepared = prepare_assembled_structural(&system);
            end_result(obs, Stage::Prepare, &prepared);
            match prepared {
                Ok(prepared) => return dense_factor_finish(prepared, obs),
                Err(error) => error,
            }
        }
    };
    StagedSolve::failed(Stage::Prepare, prepared)
}

/// SA's private dense `solve_prepared` (`SA:1761-1778`) at its boundaries.
fn dense_factor_finish(
    prepared: open_pipe_stress_frame_kernel::structural::PreparedSystem<'_>,
    obs: &mut dyn StageObserver,
) -> StagedSolve {
    obs.begin(Stage::Factor);
    let factor = factor_structural_cholesky(&prepared);
    end_result(obs, Stage::Factor, &factor);
    let factor = match factor {
        Ok(factor) => factor,
        Err(error) => {
            obs.begin(Stage::Witness);
            let witness = negative_pair_witness(&prepared);
            end_result(obs, Stage::Witness, &witness);
            let solution = match witness {
                Ok(found) => Err(found.unwrap_or_else(|| error.clone())),
                Err(witness_error) => Err(witness_error),
            };
            return StagedSolve {
                solution,
                failed_stage: Some(Stage::Factor),
                factor_error: Some(error),
                ordering_counts: None,
            };
        }
    };
    obs.begin(Stage::Finish);
    let solution = finish_structural(&factor);
    end_result(obs, Stage::Finish, &solution);
    StagedSolve {
        failed_stage: solution.is_err().then_some(Stage::Finish),
        solution,
        factor_error: None,
        ordering_counts: None,
    }
}

/// Recovery (b = 0): the E12 reactions from sparse rows (`SparseStiffness::
/// reactions`, as `PP@F1b` publishes them in both modes) and each frame's end
/// actions (`force_scaled_end_actions` at `ForceScale::UNSCALED`). Returns
/// whether every call succeeded and a digest of the value bits.
pub fn recovery(
    setup: &Setup,
    frames: &[FrameElement],
    displacements: &[f64],
    obs: &mut dyn StageObserver,
) -> (bool, u64) {
    obs.begin(Stage::Recovery);
    let mut digest = Fnv64::new();
    let mut ok = true;
    match setup.k.reactions(displacements, &setup.force) {
        Ok(reactions) => reactions.iter().for_each(|v| digest.update_f64(*v)),
        Err(error) => {
            ok = false;
            digest.update(short_error(&error).as_bytes());
        }
    }
    for frame in frames {
        match frame.force_scaled_end_actions(displacements, ForceScale::UNSCALED) {
            Ok(actions) => actions.iter().for_each(|a| digest.update_f64(a.value)),
            Err(error) => {
                ok = false;
                digest.update(format!("{error:?}").as_bytes());
            }
        }
    }
    obs.end(Stage::Recovery, ok, None);
    (ok, digest.finish())
}

/// SA's two entries, end to end, as the product calls them.
pub fn entry(
    setup: &Setup,
    mode: Mode,
    checked: bool,
    obs: &mut dyn StageObserver,
) -> Result<StructuralSolution, StructuralError> {
    let linear_mode = match mode {
        Mode::Dense => LinearSolveMode::DenseScrutiny,
        _ => LinearSolveMode::SparseInteractive,
    };
    let stage = if checked {
        Stage::EntryChecked
    } else {
        Stage::EntryPlain
    };
    obs.begin(stage);
    let result = if checked {
        setup.evidence.solve_assembled_with_formation_check(
            &setup.k,
            &setup.force,
            &setup.free,
            &setup.prescribed,
            linear_mode,
            &[],
            true,
        )
    } else {
        setup.evidence.solve_assembled(
            &setup.k,
            &setup.force,
            &setup.free,
            &setup.prescribed,
            linear_mode,
        )
    };
    end_result(obs, stage, &result);
    result
}

/// A lane's result: its counts and a digest of its solution bits.
#[derive(Debug, Clone, PartialEq)]
pub struct LaneOutcome {
    pub ok: bool,
    pub error: Option<String>,
    /// `lane-id`: (original profile, ordered profile, original half-bandwidth,
    /// ordered half-bandwidth, entries).
    pub profile: Option<(usize, usize, usize, usize, usize)>,
    pub solution_len: usize,
    pub solution_digest: u64,
}

fn digest_of(values: &[f64]) -> u64 {
    let mut digest = Fnv64::new();
    values.iter().for_each(|v| digest.update_f64(*v));
    digest.finish()
}

/// The identity-order lane: assembly and ledger (alive, as in the product),
/// the lane's entries, and `solve_symmetric_system_from_entries`.
pub fn lane_id(
    model: &K6Model,
    frames: &[FrameElement],
    obs: &mut dyn StageObserver,
) -> Result<LaneOutcome, SetupError> {
    let n_nodes = model.node_count();
    let _k = run(obs, Stage::Assembly, || {
        assemble_sparse_stiffness(
            n_nodes,
            frames,
            &[],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        )
    })?;
    let force = run(obs, Stage::Ledger, || ledger(model))?;
    let restrained = model.restrained_dofs();
    let system = run(obs, Stage::LaneEntries, || {
        reduced_entry_system(n_nodes, frames, force.values(), &restrained)
    })?;
    obs.begin(Stage::LaneSolve);
    let solved =
        solve_symmetric_system_from_entries(system.dimension, &system.entries, &system.force);
    let error = solved.as_ref().err().map(|e| format!("{e:?}"));
    obs.end(Stage::LaneSolve, solved.is_ok(), error.clone());
    Ok(match solved {
        Ok(result) => LaneOutcome {
            ok: true,
            error: None,
            profile: Some((
                result.original_profile_entry_count,
                result.ordered_profile_entry_count,
                result.original_max_half_bandwidth,
                result.ordered_max_half_bandwidth,
                system.entries.len(),
            )),
            solution_len: result.solution.len(),
            solution_digest: digest_of(&result.solution),
        },
        Err(_) => LaneOutcome {
            ok: false,
            error,
            profile: None,
            solution_len: 0,
            solution_digest: 0,
        },
    })
}

/// The dense LU lane: assembly and ledger, the dense view, the dense
/// reduction (the view dropped after it, as `PP@F1b:3849`), and `solve_dense`.
pub fn lane_lu(
    model: &K6Model,
    frames: &[FrameElement],
    obs: &mut dyn StageObserver,
) -> Result<LaneOutcome, SetupError> {
    let n_nodes = model.node_count();
    let k = run(obs, Stage::Assembly, || {
        assemble_sparse_stiffness(
            n_nodes,
            frames,
            &[],
            &[],
            &[],
            &SparseAssemblyOptions::new(),
        )
    })?;
    let force = run(obs, Stage::Ledger, || ledger(model))?;
    let restrained = model.restrained_dofs();
    obs.begin(Stage::DensifyLu);
    let dense = k.to_dense();
    obs.end(Stage::DensifyLu, true, None);
    let reduced = run(obs, Stage::LaneReduce, || {
        let reduced = reduce_assembled_system(&dense, &force, &restrained);
        drop(dense);
        reduced
    })?;
    obs.begin(Stage::LaneSolve);
    let solved = solve_dense(&reduced.stiffness, reduced.force.values());
    let error = solved.as_ref().err().map(|e| format!("{e:?}"));
    obs.end(Stage::LaneSolve, solved.is_ok(), error.clone());
    Ok(match solved {
        Ok(solution) => LaneOutcome {
            ok: true,
            error: None,
            profile: None,
            solution_len: solution.len(),
            solution_digest: digest_of(&solution),
        },
        Err(_) => LaneOutcome {
            ok: false,
            error,
            profile: None,
            solution_len: 0,
            solution_digest: 0,
        },
    })
}
