//! Private, complete-source recovery for the bounded straight-frame family.
//!
//! The certificate concerns the declared binary64 element/load operands. It is
//! neither a primitive-geometry certificate nor an ordinary factorization report.
//! Selection and public admission belong to the product facade.
use super::case_state::resolve::ResolvedCase;
use super::{
    dof_index, is_constant_effort_support, parse_category, parse_direction, parse_dof,
    parse_load_dimension, support_stiffness_input, AssemblyEvidence, BuiltModel, LoadDimension,
    LoadTargetInput, Matrix12, PreviewLoadCase, PreviewModel,
    PreviewPrimitiveLoad, PrimitiveLoadCategory, Quantity, SpringEntry, SupportFamily,
    ThermalElementLoad, DOF_PER_NODE,
};
use exact::functionals::{
    AffineTerm, AttemptBudget, AttemptStage, FunctionalConvention, FunctionalDescriptor,
    FunctionalKey, FunctionalPlan, FunctionalQuantity, FunctionalUnit, MemberEnd, Products,
    RetainedFunctionalSet,
};
use open_pipe_stress_frame_kernel::element_dof_map;
use open_pipe_stress_frame_kernel::load_ledger::{AssembledForce, LoadLedger};
use open_pipe_stress_frame_kernel::structural::{
    exact_boundary as exact, StiffnessContribution, StructuralSystem,
};
use open_pipe_stress_frame_kernel::ForceScale;
use open_pipe_stress_primitive_loads::LoadApplication;
use std::collections::{HashMap, HashSet};

pub(super) const RELATIVE_LIMIT: f64 = 1.0e-9;
/// The largest system (global DOFs) the method accepts; above it `prepare_sources`
/// refuses by budget before any read of `Input::stiffness` (F1b: the product
/// builds its dense view only up to this size; ROOT Q9(a)).
pub(super) const DENSE_SOURCE_DOF_LIMIT: usize = 256;
pub(super) const STATIONS: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
const END_CONVENTION: FunctionalConvention = FunctionalConvention::NodeOnElement;

/// Every slice is the actual invocation's source, before reduction/projection.
/// Unsupported producers are checked from both authored and built inventories.
pub(super) struct Input<'a> {
    pub model: &'a PreviewModel,
    pub built: &'a BuiltModel,
    pub stiffness: &'a [Vec<f64>],
    /// The ledger-built case force (S11 section 4.3).
    pub force: &'a AssembledForce,
    pub free: &'a [usize],
    pub prescribed: &'a [(usize, f64)],
    pub spring_entries: &'a [SpringEntry],
    pub load_case: &'a PreviewLoadCase,
    pub load_application: &'a LoadApplication,
    pub thermal_loads: &'a [ThermalElementLoad],
    /// The one resolved case of a 0.4.0 load/reference-state invocation. It is
    /// the only admitted owner of eigen element loads and nonzero prescribed
    /// support motion; absent for every pre-0.4 invocation.
    pub load_state: Option<&'a ResolvedCase>,
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

/// S11-G: the unsupported-family reason of a formation-guard decline.
const FORMATION_GUARD_DECLINE: &str = "the load-row formation guard fired for this case";

/// S11-G revision 2.2 with ROOT's D22-1 condition: a case the ordinary route
/// would not attempt (report Passed, no `Err`) whose load-row guard fired is
/// declined without running an attempt, so the invocation ledger equals the
/// unguarded one: zero work {limit 0, charged 0, rejected 0}, the same stage,
/// helper stage and error as `decline_formation`.
pub(super) fn formation_decline_without_attempt() -> RecoveryFailure {
    RecoveryFailure {
        stage: "formation guard",
        helper_stage: AttemptStage::SourceClosure,
        error: RecoveryError::Unsupported(FORMATION_GUARD_DECLINE),
        work: exact::WorkReport::new(0, 0, 0),
    }
}

/// F1b (ROOT OQ2, option B): the reason of the decline of a case whose
/// basis's formation left the binary64 range.
const RANGE_FORMATION_DECLINE: &str =
    "the ordinary stiffness was not formed (range at formation); no retained-source attempt";

/// F1b (ROOT OQ2, option B): a case whose basis's formation left the range has
/// no assembled stiffness to recover from, and `prepare_sources` would refuse
/// it at the same frames' formation (`AssemblyEvidence::new`; F1b's derivation
/// D2), so it is declined without an attempt, with zero work, as
/// `formation_decline_without_attempt` declines.
pub(super) fn range_formation_decline_without_attempt() -> RecoveryFailure {
    RecoveryFailure {
        stage: "range formation",
        helper_stage: AttemptStage::SourceClosure,
        error: RecoveryError::Unsupported(RANGE_FORMATION_DECLINE),
        work: exact::WorkReport::new(0, 0, 0),
    }
}

/// F1b (T3 D1 §4.7 with ROOT Q2(a) and OQ3): the reason of the scaled-evidence
/// refusal.
const FORCE_SCALED_EVIDENCE: &str =
    "force-scaled ordinary evidence (b != 0) is not a retained-source input";

/// F1b (ROOT OQ3): the product's only entry into the method. `attempt_scale`
/// is the force scale 2^b of the ordinary evidence the input carries; an
/// evidence formed at b != 0 is refused as `Unsupported` before any work is
/// charged. The product runs exact-block before W2 and passes `UNSCALED`, so
/// this is a defensive guard (F1b's derivation D3).
pub(super) fn solve_ordinary(
    input: Input<'_>,
    limits: exact::Limits,
    attempt_scale: ForceScale,
) -> Result<SelectedSourceRecovery, RecoveryFailure> {
    if !attempt_scale.is_unscaled() {
        let budget = AttemptBudget::new(limits);
        return Err(failure(
            "source closure",
            unsupported(FORCE_SCALED_EVIDENCE),
            &budget,
        ));
    }
    solve(input, limits)
}

/// No longer built: the selection keeps no spring actions. The registered memory
/// profile still names this type (`s(SpringAction)`) until B7a's D3 re-registration.
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
    supports: Vec<SupportActions>,
    retained: RetainedFunctionalSet,
    summary: RecoverySummary,
    // Retaining the live attempt ledger lets explicit expected-invocation replay
    // continue the same budget; replay never resets the selection's work cap.
    budget: AttemptBudget,
    retained_identity_digest: Option<String>,
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
    pub fn support_actions(&self) -> &[SupportActions] {
        &self.supports
    }
    pub fn retained(&self) -> &RetainedFunctionalSet {
        &self.retained
    }
    pub fn summary(&self) -> &RecoverySummary {
        &self.summary
    }

    /// Lazily hash this immutable owned response once; never accept a caller's
    /// claimed digest. Repeated derived recipes reserve only the returned copy.
    pub fn retained_identity_digest(&mut self) -> Result<String, RecoveryFailure> {
        use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
        use sha2::{Digest, Sha256};
        if self.retained_identity_digest.is_some() {
            self.charge_finalization(128)?;
            return Ok(self
                .retained_identity_digest
                .as_ref()
                .expect("owned digest")
                .clone());
        }
        self.charge_finalization(
            self.retained
                .response()
                .source_identity()
                .len()
                .saturating_mul(12)
                .saturating_add(1024),
        )?;
        let result=(|| {
            let text=serde_json::to_string(&serde_json::json!({"domain":"physics_source_retained_identity_v1","payload":self.retained.response().source_identity()})).map_err(|_|mismatch("identity digest serialization"))?;
            let canonical=canonical_json_checked_v1_text(&text).map_err(|_|mismatch("identity digest checked JSON"))?;
            Ok::<String,RecoveryError>(format!("{:x}",Sha256::digest(canonical.as_bytes())))
        })().map_err(|error|failure("retained identity digest",error,&self.budget))?;
        self.retained_identity_digest = Some(result.clone());
        Ok(result)
    }

    /// ROOT CP3 SF-1 screen, on both exact routes (pre-0.4 failure
    /// unification §3). Captured replay continues this attempt's own ledger and repeats its
    /// source closure and exact solve, so its work is close to the live
    /// charge. Selection first reserves an amount equal to that charge;
    /// otherwise the attempt is declined as a budget refusal whose rejected
    /// work is the unreserved replay. This is a screen, not a guarantee:
    /// derived recipes and the finalization reservation are charged before
    /// replay, and replay can cost slightly more than the live charge. A
    /// selected source that still cannot finalize is published on the ordinary
    /// route (CP4 review SF-1R).
    pub fn reserve_captured_replay(self, limit: usize) -> Result<Self, RecoveryFailure> {
        let charged = self.summary.work.charged;
        if charged <= limit.saturating_sub(charged) {
            return Ok(self);
        }
        Err(RecoveryFailure {
            stage: "captured replay reservation",
            helper_stage: AttemptStage::Replay,
            error: RecoveryError::Exact(exact::Error::Budget),
            work: exact::WorkReport::new(charged, charged, limit),
        })
    }

    /// ROOT CP3 SF-1, on both exact routes: the invocation's selected join (its
    /// source composite) could not finalize, so its republication declines
    /// every successful attempt. The executed work stays charged; nothing is
    /// selected or published from this response.
    pub fn decline_withheld(self) -> RecoveryFailure {
        RecoveryFailure {
            stage: "invocation join withheld",
            helper_stage: AttemptStage::Replay,
            error: RecoveryError::Unsupported(
                "selected join could not finalize for this invocation",
            ),
            work: self.summary.work,
        }
    }

    /// S11-G revision 2.2 G-3: a selection declined because the case's
    /// load-row formation guard fired (a guard-fired case is never selected or
    /// qualified). Mirrors `decline_withheld`: the executed work stays charged;
    /// the receipt entry is `failed` (`source_validation`, `unsupported_family`).
    pub fn decline_formation(self) -> RecoveryFailure {
        RecoveryFailure {
            stage: "formation guard",
            helper_stage: AttemptStage::SourceClosure,
            error: RecoveryError::Unsupported(FORMATION_GUARD_DECLINE),
            work: self.summary.work,
        }
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
            self.retained
                .check_binding_with_budget(
                    &source.system(&input),
                    &source.identity,
                    exact::ForceBasis::IdentifiedContributions(&source.force_terms),
                    &source.congruence,
                    &source.descriptors,
                    &mut self.budget,
                )
                .result?;
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
                    &source.congruence,
                    &source.descriptors,
                    Some(&|index| source.hint(index)),
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
    /// RF design §3.3: one entry per member whose transform is not a signed
    /// permutation, in member order; empty when every member is one. The same
    /// list is given to the functional entry, its binding check and its replay.
    congruence: Vec<exact::CongruenceMember>,
    force_terms: Vec<exact::ForceContribution>,
    descriptors: Vec<FunctionalDescriptor>,
    identity: String,
    member_ids: Vec<String>,
    spring_ids: Vec<(String, usize)>,
    support_ids: Vec<(String, usize)>,
    nodal_start: usize,
    support_start: usize,
}
impl Sources {
    fn system<'a>(&'a self, input: &'a Input<'_>) -> StructuralSystem<'a> {
        StructuralSystem {
            stiffness: input.stiffness,
            force: input.force.values(),
            free_dofs: input.free,
            prescribed: input.prescribed,
            contributions: Some(&self.assembly.contributions),
            contribution_groups: None,
            symmetry: None,
        }
    }
    /// R-A's B1 hint rule (DEL-04-08 Design `source-work-nondyadic-inputs.md` §3.1). The
    /// member rows come first, each member's 12 end rows then its 6 × STATIONS section
    /// rows, so a section row's candidate is its member's i-end row of the same
    /// component. Every other row has none. The kernel charges the rule before it
    /// builds the hints, in the plan and in the replay, and verifies every hint before
    /// use, so a wrong one takes the long path.
    fn hint(&self, index: usize) -> Option<u32> {
        const MEMBER_ROWS: usize = 12 + 6 * STATIONS.len();
        let row = index % MEMBER_ROWS;
        if index >= self.nodal_start || row < 12 {
            return None;
        }
        u32::try_from(index - row + (row - 12) % 6).ok()
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
/// `None` for any other transform, whose member is a congruence member (RF
/// design §3.1) and must pass `block_rotation` and its formation closure.
fn signed_permutation(t: &Matrix12) -> Option<[(usize, f64); 12]> {
    let mut selected = [(0, 0.0); 12];
    let mut used = [false; 12];
    for (row, values) in t.iter().enumerate() {
        let mut column = None;
        for (col, &value) in values.iter().enumerate() {
            if value == 0.0 {
                continue;
            }
            if !matches!(value, -1.0 | 1.0) || column.is_some() || used[col] {
                return None;
            }
            column = Some((col, value));
        }
        let (col, sign) = column?;
        used[col] = true;
        selected[row] = (col, sign);
    }
    Some(selected)
}

/// RF design §3.2 (1): the decline of a transform that is neither a signed
/// permutation nor a block rotation.
pub(super) const BLOCK_ROTATION_DECLINE: &str = "actual transform is not a block rotation";

/// RF design §3.2 (1): a congruence member's transform must be blockdiag(R, R,
/// R, R): its four 3×3 diagonal blocks bit-identical and every off-block entry
/// +0.0 or −0.0. The decline names "transform", so the receipt keeps the code
/// `unsupported_source_closure` (`source::failure_fields`).
fn block_rotation(t: &Matrix12) -> Result<(), RecoveryError> {
    for (row, values) in t.iter().enumerate() {
        for (col, &value) in values.iter().enumerate() {
            let closed = if row / 3 == col / 3 {
                value.to_bits() == t[row % 3][col % 3].to_bits()
            } else {
                value == 0.0
            };
            if !closed {
                return Err(unsupported(BLOCK_ROTATION_DECLINE));
            }
        }
    }
    Ok(())
}

/// RF design §3.2 (2), formation determinism: a congruence member's 144
/// assembly contributions are its `global_stiffness()` bits at its DOF map, in
/// local row-major order. This ties the ordinary aggregate (whose ordered fold
/// the coverage checks bind) to the same declared operands.
fn close_congruence_formation(
    global: &Matrix12,
    map: &[usize; 12],
    contributions: Option<&[StiffnessContribution]>,
) -> Result<(), RecoveryError> {
    let closed = contributions.is_some_and(|c| {
        c.len() == 144
            && c.iter().enumerate().all(|(e, c)| {
                c.row == map[e / 12]
                    && c.col == map[e % 12]
                    && c.value.to_bits() == global[e / 12][e % 12].to_bits()
            })
    });
    if !closed {
        return Err(mismatch(
            "member global formation differs from its assembly contributions",
        ));
    }
    Ok(())
}

/// How a member's exact end rows are formed (RF design §3.1 and §3.4).
enum MemberBasis {
    /// A signed permutation: one exact product per entry.
    Selector([(usize, f64); 12]),
    /// A congruence member's declared transform T.
    Congruence(Matrix12),
}

/// RF design §3.4: the p, ascending, of the nonzero pairs
/// [K_local[row][p], T[p][j]] of a congruence member's end row `row` at the
/// local column j of T.
fn end_row_pairs<'a>(
    local: &'a Matrix12,
    transform: &'a Matrix12,
    row: usize,
    j: usize,
) -> impl Iterator<Item = usize> + 'a {
    (0..12).filter(move |&p| local[row][p] != 0.0 && transform[p][j] != 0.0)
}

/// RF design §3.4: the (k, l), ascending in k then l, of the nonzero triple
/// products [T[k][i], K_local[k][l], T[l][j]] of a congruence member's exact
/// entry G[i][j] = Σ_k Σ_l T[k][i]·K_local[k][l]·T[l][j].
fn exact_entry_pairs(
    member: &exact::CongruenceMember,
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (k_local, t) = (&member.local_stiffness, &member.transform);
    (0..12).filter(move |&k| t[k][i] != 0.0).flat_map(move |k| {
        (0..12)
            .filter(move |&l| k_local[k][l] != 0.0 && t[l][j] != 0.0)
            .map(move |l| (k, l))
    })
}

/// The congruence member whose rounded contribution is `index`, with that
/// contribution's local (row, column).
fn congruence_entry(
    congruence: &[exact::CongruenceMember],
    index: usize,
) -> Option<(&exact::CongruenceMember, usize, usize)> {
    congruence
        .iter()
        .find(|m| m.contributions.contains(&index))
        .map(|m| {
            let e = index - m.contributions.start;
            (m, e / 12, e % 12)
        })
}

/// One member end row's displacement terms.
/// - A signed permutation: one term per nonzero K_local[row][p], in ascending
///   p, whose one product is K_local[row][p] times its exact unit selector.
/// - A congruence member (RF design §3.4): rows[r] = Σ_p K_local[r][p]·(T·u_e)_p,
///   merged per global DOF. One term per local column j of T, in ascending j
///   (the DOF map's order), holding every nonzero pair [K_local[r][p], T[p][j]]
///   as a product, in ascending p; a column without such a pair has no term.
fn end_row_terms(
    local: &Matrix12,
    basis: &MemberBasis,
    map: &[usize; 12],
    row: usize,
) -> Vec<AffineTerm> {
    match basis {
        MemberBasis::Selector(selector) => (0..12)
            .filter(|&p| local[row][p] != 0.0)
            .map(|p| {
                let (j, sign) = selector[p];
                // Multiplication by the exact unit selector cannot round.
                AffineTerm {
                    dof: map[j],
                    products: vec![vec![sign * local[row][p]]],
                }
            })
            .collect(),
        MemberBasis::Congruence(transform) => (0..12)
            .filter_map(|j| {
                let products: Products = end_row_pairs(local, transform, row, j)
                    .map(|p| vec![local[row][p], transform[p][j]])
                    .collect();
                (!products.is_empty()).then(|| AffineTerm {
                    dof: map[j],
                    products,
                })
            })
            .collect(),
    }
}

/// RF design §3.5: the structure `descriptors_charge` counts for one
/// descriptor. Its units are 1 (the descriptor), plus each offset product and
/// its atoms, plus each term, each term product and its atoms.
#[derive(Clone, Copy, Default)]
struct RowShape {
    offsets: usize,
    offset_atoms: usize,
    terms: usize,
    products: usize,
    product_atoms: usize,
}
impl RowShape {
    /// The units of this row's offsets and terms with `factors` more atoms in
    /// every product (as `append_scaled_products` appends them), without the
    /// descriptor's own unit.
    fn units_with(&self, factors: usize) -> usize {
        self.offsets
            + self.offset_atoms
            + factors * self.offsets
            + self.terms
            + self.products
            + self.product_atoms
            + factors * self.products
    }
}
/// A nodal displacement row: one term with one one-atom product.
const NODAL_ROW_UNITS: usize = 4;
/// A ground-spring row: one term with one one-atom product.
const SPRING_ROW_UNITS: usize = 4;

/// The structure of the end row `end_row_terms` and its eigen offset form.
fn end_row_shape(local: &Matrix12, basis: &MemberBasis, row: usize, eigen: f64) -> RowShape {
    let offsets = usize::from(eigen != 0.0 && (row == 0 || row == 6));
    let (terms, products, atoms) = match basis {
        MemberBasis::Selector(_) => {
            let terms = (0..12).filter(|&p| local[row][p] != 0.0).count();
            (terms, terms, 1)
        }
        MemberBasis::Congruence(transform) => (
            (0..12)
                .filter(|&j| end_row_pairs(local, transform, row, j).next().is_some())
                .count(),
            (0..12)
                .flat_map(|j| end_row_pairs(local, transform, row, j))
                .count(),
            2,
        ),
    };
    RowShape {
        offsets,
        offset_atoms: offsets,
        terms,
        products,
        product_atoms: atoms * products,
    }
}

/// RF design §3.5: one member's descriptor units, counted from its structure
/// as `descriptors_charge` counts the rows `prepare_sources` builds: its 12
/// end rows, and its 30 section rows (each end row of its component with one
/// more factor, plus for My and Mz the shear row with three more).
fn member_descriptor_units(local: &Matrix12, basis: &MemberBasis, eigen: f64) -> usize {
    let shapes: [RowShape; 12] =
        std::array::from_fn(|row| end_row_shape(local, basis, row, eigen));
    let mut units = 0usize;
    for shape in &shapes {
        units = units.saturating_add(1 + shape.units_with(0));
    }
    for _ in STATIONS {
        for component in 0..6 {
            units = units.saturating_add(1 + shapes[component].units_with(1));
            if component == 4 || component == 5 {
                let shear = if component == 4 { 2 } else { 1 };
                units = units.saturating_add(shapes[shear].units_with(3));
            }
        }
    }
    units
}

/// RF design §3.5: the support rows' descriptor units, counted from their
/// structure as `descriptors_charge` counts the rows `prepare_sources` builds
/// (a rounded contribution's term has one one-atom product; a congruence
/// entry's term has its triple products; each load offset is one one-atom
/// product; each spring term one one-atom product).
fn support_units(
    input: &Input<'_>,
    support_ids: &[(String, usize)],
    rigid_owner: &[Option<usize>],
    contributions: &[StiffnessContribution],
    congruence: &[exact::CongruenceMember],
    force_terms: &[exact::ForceContribution],
) -> usize {
    let mut units = 0usize;
    for (owner, (id, node)) in support_ids.iter().enumerate() {
        for component in 0..6 {
            let dof = node * 6 + component;
            units = units.saturating_add(1);
            if rigid_owner[dof] == Some(owner) {
                for (index, term) in contributions.iter().enumerate().filter(|(_, t)| t.row == dof) {
                    let (products, atoms) = match congruence_entry(congruence, index) {
                        Some((member, i, j)) => (exact_entry_pairs(member, i, j).count(), 3),
                        None => (usize::from(term.value != 0.0), 1),
                    };
                    if products > 0 {
                        units = units.saturating_add(1 + products * (1 + atoms));
                    }
                }
                let loads = force_terms
                    .iter()
                    .filter(|f| f.dof == dof && f.value != 0.0)
                    .count();
                units = units.saturating_add(2 * loads);
            }
            let springs = input
                .spring_entries
                .iter()
                .filter(|s| s.support_id == *id && s.node_dof.global_index() == dof)
                .count();
            units = units.saturating_add(3 * springs);
        }
    }
    units
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
    if n > DENSE_SOURCE_DOF_LIMIT
        || n > limits.dofs
        || source_count > 16_384
        || source_count > limits.source_terms
    {
        return Err(exact::Error::Budget.into());
    }
    // D7 (DEL-04-07 `pressure-contract.md`, "T3 coexistence and B7"): each
    // family a T4 unit admits under v3 that this straight source method does
    // not extend is declined by its own name, ahead of the dimension closure
    // (a realized arc or a connector-replaced span has no straight frame, so
    // that closure would otherwise decline it as a source mismatch).
    if !input.built.curved_bend_elements.is_empty() {
        return Err(unsupported(
            "realized curved bend (curved_bend_macro_element) is not a straight source family",
        ));
    }
    if !input.built.connectors.is_empty() {
        return Err(unsupported("objective connector is not a straight source family"));
    }
    // O4 (DEL-04-08 Design `o4-source-extension.md` §3.1, §3.2): a
    // timoshenko_annulus_energy_v1 document and a nonzero mill tolerance or
    // corrosion allowance are admitted. The declared operand is each frame's
    // own binary64 local matrix (closed against the pipe element and the
    // assembly below), and the stress rows are published on each section
    // basis from the same retained functionals.
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
    // A resolved case is required for, and admitted only with, a 0.4.0 model:
    // no pre-0.4 invocation inherits it and no 0.4.0 invocation omits it.
    let load_state = match (
        input.load_state,
        super::case_state::is_load_state(input.model),
    ) {
        (None, false) => None,
        (Some(state), true) => Some(state),
        _ => return Err(mismatch("resolved load/reference-state ownership")),
    };
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
    if !input.built.connectors.is_empty()
        || !input.built.curved_bend_elements.is_empty()
        || !input.model.components.is_empty()
    {
        return Err(unsupported(
            "component, curved, user-matrix or release source family",
        ));
    }
    // Presence is the producer inventory: a zero-valued pressure region is
    // still unsupported. Exact source cases require an explicitly empty list.
    if super::pressure_runtime::is_exact(input.model) {
        if !input
            .load_case
            .pressure_regions
            .as_ref()
            .is_some_and(Vec::is_empty)
        {
            return Err(unsupported(
                "exact source recovery requires explicitly empty pressure regions",
            ));
        }
        if input.built.exact_sections.len() != member_count {
            return Err(mismatch("exact source geometry inventory"));
        }
    } else if !crate::model_format::is_pressure_free(input.model)
        || input.load_case.pressure_regions.is_some()
    {
        return Err(unsupported("preview source-blocks namespace requires a pressure-free model without pressure contract or regions"));
    }
    if input.load_case.equivalent_static.is_some()
        || !input.load_application.element_uniform_loads.is_empty()
        || !input.load_application.imposed_displacements.is_empty()
        || (load_state.is_none() && !input.thermal_loads.is_empty())
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
    // S11 section 4.5: the same nodal and eigen terms as the case ledger, in a
    // ledger; `finish()`'s correctly rounded nets must equal the actual force.
    let mut folded_force = LoadLedger::new();
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
        folded_force.push(&load.load_id, load.global_dof, load.value);
        identity.name(&load.load_id, budget)?;
        identity.indices([load.node_index, load.global_dof]);
        identity.scalars([load.value]);
        force_terms.push(exact::ForceContribution {
            source: load.load_id.clone(),
            dof: load.global_dof,
            value: load.value,
        });
    }
    let eigen_axial = match load_state {
        Some(state) => close_load_state(
            input,
            state,
            &authored_loads,
            &mut folded_force,
            &mut force_terms,
            &mut identity,
            budget,
        )?,
        None => vec![0.0; member_count],
    };
    let folded_force = folded_force
        .finish(n)
        .map_err(|_| mismatch("complete ordered load fold differs from actual force"))?;
    if seen_loads.len() != authored_loads.len()
        || folded_force.len() != input.force.len()
        || folded_force
            .values()
            .iter()
            .zip(input.force.values())
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
                let imposed = if let Some(state) = load_state {
                    // The resolved support state is the sole motion owner.
                    if support.family == SupportFamily::ImposedDisplacement {
                        return Err(unsupported(
                            "legacy imposed-displacement support with resolved support state",
                        ));
                    }
                    state.prescribed.get(&global).copied().unwrap_or(0.0)
                } else if support.family == SupportFamily::ImposedDisplacement {
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
    if let Some(state) = load_state {
        for (&dof, &value) in &state.prescribed {
            if dof >= n
                || !value.is_finite()
                || expected_prescribed[dof].is_none_or(|v| v.to_bits() != value.to_bits())
            {
                return Err(mismatch(
                    "resolved support motion lacks one rigid boundary owner",
                ));
            }
            identity.indices([dof]);
            identity.scalars([value]);
        }
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
    #[cfg(test)]
    let assembly = tests::perturbed_assembly(assembly);
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
    // RF design §3.2: every member closes before descriptor #1 is built. A
    // member whose transform is a signed permutation closes exactly as before.
    // Any other member is a congruence member (§3.1): its transform must be a
    // block rotation and its rounded contributions its own formation. Each
    // member's descriptor units, counted from its structure, enter the running
    // total of the unit precheck below.
    let mut congruence = Vec::new();
    let mut member_units = 0usize;
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
        #[cfg(test)]
        let transform = tests::perturbed_transform(index, transform);
        let selector = signed_permutation(&transform);
        if selector.is_none() {
            block_rotation(&transform)?;
        }
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
        let basis = match selector {
            Some(selector) => {
                // A signed permutation has exactly one product per global matrix
                // entry: no rounded dot product is used as the source-closure oracle.
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
                MemberBasis::Selector(selector)
            }
            None => {
                // RF design §3.2 (2): the rounded contributions, which stay in the
                // coverage fold, are this member's own formation of its declared
                // operands; its exact source is their congruence (§3.3).
                let global = frame
                    .global_stiffness()
                    .map_err(|_| mismatch("frame global matrix"))?;
                let range = index * 144..index * 144 + 144;
                close_congruence_formation(&global, &map, assembly.contributions.get(range.clone()))?;
                congruence.push(exact::CongruenceMember {
                    dofs: map,
                    local_stiffness: local,
                    transform,
                    contributions: range,
                });
                MemberBasis::Congruence(transform)
            }
        };
        member_units = member_units.saturating_add(member_descriptor_units(
            &local,
            &basis,
            eigen_axial[index],
        ));
    }
    // RF design §3.2 (3) and §3.5: with a congruence member present, the units
    // descriptor #1 will hold, counted from structure exactly as the functional
    // plan counts them (`descriptors_charge`), are checked against the
    // unchanged unit law before any descriptor is built. A model whose members
    // are all signed permutations takes no new check.
    if !congruence.is_empty() {
        let units = member_units
            .saturating_add(n.saturating_mul(NODAL_ROW_UNITS))
            .saturating_add(input.spring_entries.len().saturating_mul(SPRING_ROW_UNITS))
            .saturating_add(support_units(
                input,
                &support_ids,
                &rigid_owner,
                &assembly.contributions,
                &congruence,
                &force_terms,
            ));
        let limit = limits.source_terms.min(16_384);
        #[cfg(test)]
        let limit = tests::unit_precheck(units, limit);
        if units > limit {
            return Err(exact::Error::Budget.into());
        }
    }
    #[cfg(test)]
    tests::descriptor_construction_started();
    let mut descriptors = Vec::new();
    let mut member_ids = Vec::new();
    let mut congruence_members = congruence.iter().peekable();
    for (index, (pipe, frame)) in input
        .built
        .pipes
        .iter()
        .zip(&input.built.frame_elements)
        .enumerate()
    {
        // The operands closed above, re-formed by the same deterministic calls.
        let local = frame
            .local_stiffness()
            .map_err(|_| mismatch("frame local matrix"))?;
        let map = element_dof_map(frame.node_i.index, frame.node_j.index);
        let length = frame.length().map_err(|_| mismatch("member span"))?;
        let basis = match congruence_members.next_if(|m| m.contributions.start == index * 144) {
            Some(member) => MemberBasis::Congruence(member.transform),
            None => MemberBasis::Selector(
                signed_permutation(
                    &frame
                        .orientation()
                        .map_err(|_| mismatch("frame orientation"))?
                        .transformation_matrix(),
                )
                .ok_or_else(|| mismatch("exact local/transform/scatter assembly closure"))?,
            ),
        };
        descriptors.extend(member_descriptors(
            &input.load_case.id,
            &pipe.element_id,
            &local,
            &basis,
            &map,
            length,
            eigen_axial[index],
        ));
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
                        support: id.clone(),
                        node: *node,
                        component: component as u8,
                    },
                    component,
                    FunctionalConvention::SupportOnStructure,
                ),
                offset: vec![],
                terms: vec![],
            };
            if rigid_owner[dof] == Some(owner) {
                for (index, term) in assembly
                    .contributions
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.row == dof)
                {
                    // RF design §3.4: a congruence member's exact entry, as its
                    // triple products, in place of its rounded contribution.
                    if let Some((member, i, j)) = congruence_entry(&congruence, index) {
                        let products: Products = exact_entry_pairs(member, i, j)
                            .map(|(k, l)| {
                                vec![
                                    member.transform[k][i],
                                    member.local_stiffness[k][l],
                                    member.transform[l][j],
                                ]
                            })
                            .collect();
                        if !products.is_empty() {
                            row.terms.push(AffineTerm {
                                dof: term.col,
                                products,
                            });
                        }
                    } else if term.value != 0.0 {
                        row.terms.push(AffineTerm {
                            dof: term.col,
                            products: vec![vec![term.value]],
                        });
                    }
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
        congruence,
        force_terms,
        descriptors,
        identity,
        member_ids,
        spring_ids,
        support_ids,
        nodal_start,
        support_start,
    })
}

/// Force-contribution owner of one member's resolved eigen axial load.
pub(super) fn eigen_source_id(pipe_id: &str) -> String {
    format!("load_state_eigenstrain:{}:{pipe_id}", pipe_id.len())
}

/// Closes the resolved case against the actual invocation: every member's
/// pair reached formation, every eigen element load is exactly
/// `E_member*A_s*eps*` once, and its equivalent nodal action is pushed as the
/// product route's ledger terms. Returns each member's eigen axial load.
fn close_load_state(
    input: &Input<'_>,
    state: &ResolvedCase,
    authored_loads: &HashMap<&str, &PreviewPrimitiveLoad>,
    folded_force: &mut LoadLedger,
    force_terms: &mut Vec<exact::ForceContribution>,
    identity: &mut Identity,
    budget: &mut AttemptBudget,
) -> Result<Vec<f64>, RecoveryError> {
    let member_count = input.built.pipes.len();
    charge(
        budget,
        member_count
            .saturating_mul(512)
            .saturating_add(state.prescribed.len().saturating_mul(64)),
    )?;
    if state.members.len() != member_count
        || state.pairs.len() != member_count
        || input.thermal_loads.len() > member_count
    {
        return Err(mismatch("resolved member inventory"));
    }
    identity.name("load-reference-state-v1", budget)?;
    let mut eigen_axial = vec![0.0; member_count];
    let mut expected_loads = Vec::new();
    for (index, ((member, pipe), frame)) in state
        .members
        .iter()
        .zip(&input.built.pipes)
        .zip(&input.built.frame_elements)
        .enumerate()
    {
        let pair = member.material.pair;
        let owned = state.pairs.get(&member.pipe_id);
        let section = input.built.sections.get(&member.pipe_id);
        let geometry = input.built.exact_sections.get(&member.pipe_id);
        let (Some(owned), Some(section), Some(geometry)) = (owned, section, geometry) else {
            return Err(mismatch("resolved member source ownership"));
        };
        let strain = &member.strain;
        let bits = |a: f64, b: f64| a.to_bits() == b.to_bits();
        if member.pipe_index != index
            || member.pipe_id != pipe.element_id
            || !bits(owned.elastic_modulus_pa(), pair.elastic_modulus_pa())
            || !bits(owned.poisson_ratio(), pair.poisson_ratio())
            || !bits(owned.shear_modulus_pa(), pair.shear_modulus_pa())
            || !bits(frame.section.elastic_modulus, pair.elastic_modulus_pa())
            || !bits(frame.section.shear_modulus, pair.shear_modulus_pa())
            || !bits(frame.section.area, section.area)
            || !bits(section.area, geometry.nominal().wall_area_m2())
            || !strain.total_eigenstrain.is_finite()
        {
            return Err(mismatch(
                "resolved member pair/section did not reach formation",
            ));
        }
        identity.name(&member.pipe_id, budget)?;
        identity.name(&member.material.material_id, budget)?;
        identity.name(member.material.selection_kind, budget)?;
        identity.name(strain.definition, budget)?;
        identity.indices([index]);
        identity.scalars([
            pair.elastic_modulus_pa(),
            pair.poisson_ratio(),
            pair.shear_modulus_pa(),
            strain.thermal_strain,
            strain.thermal_stretch,
            strain.fit_strain,
            strain.fit_stretch,
            strain.total_eigenstrain,
        ]);
        if strain.total_eigenstrain != 0.0 {
            // Same operation order as the product route's eigen load.
            let axial = pair.elastic_modulus_pa() * section.area * strain.total_eigenstrain;
            if !axial.is_finite() {
                return Err(mismatch("resolved eigen axial load range"));
            }
            eigen_axial[index] = axial;
            expected_loads.push((index, axial, strain.total_eigenstrain));
        }
    }
    if expected_loads.len() != input.thermal_loads.len()
        || expected_loads
            .iter()
            .zip(input.thermal_loads)
            .any(|(&(index, axial, strain), load)| {
                load.element_index != index
                    || load.axial_load.to_bits() != axial.to_bits()
                    || load.thermal_strain.to_bits() != strain.to_bits()
            })
    {
        return Err(mismatch(
            "actual eigen element loads differ from the resolved case",
        ));
    }
    for load in input.thermal_loads {
        let pipe = &input.built.pipes[load.element_index];
        let source = eigen_source_id(&pipe.element_id);
        if authored_loads.contains_key(source.as_str()) {
            return Err(mismatch(
                "eigen source identity collides with a primitive load",
            ));
        }
        let local_x = pipe
            .frame_element()
            .map_err(|_| mismatch("eigen member frame"))?
            .orientation()
            .map_err(|_| mismatch("eigen member orientation"))?
            .local_axes[0];
        let i_base = pipe.node_i.index * DOF_PER_NODE;
        let j_base = pipe.node_j.index * DOF_PER_NODE;
        identity.name(&source, budget)?;
        identity.scalars([load.axial_load]);
        identity.scalars(local_x);
        // Mirror `add_thermal_equivalent_loads`'s ledger terms exactly.
        for axis in 0..3 {
            let value = load.axial_load * local_x[axis];
            folded_force.push(&source, i_base + axis, -value);
            folded_force.push(&source, j_base + axis, value);
            if value != 0.0 {
                force_terms.push(exact::ForceContribution {
                    source: source.clone(),
                    dof: i_base + axis,
                    value: -value,
                });
                force_terms.push(exact::ForceContribution {
                    source: source.clone(),
                    dof: j_base + axis,
                    value,
                });
            }
        }
    }
    // The published resolver ledger binds excluded sources, temperatures,
    // selections and provenance that are not stiffness/load operands.
    let evidence = serde_json::to_string(&state.evidence)
        .map_err(|_| mismatch("resolved evidence encoding"))?;
    identity.name(&evidence, budget)?;
    Ok(eigen_axial)
}

/// One member's descriptors, in order: its 12 end rows (local i then j), then
/// its 30 j-side section rows (STATIONS, then components). `eigen` is the
/// member's resolved eigen axial load (zero without one).
fn member_descriptors(
    case: &str,
    member: &str,
    local: &Matrix12,
    basis: &MemberBasis,
    map: &[usize; 12],
    length: f64,
    eigen: f64,
) -> Vec<FunctionalDescriptor> {
    let mut rows = Vec::new();
    for row in 0..12 {
        let terms = end_row_terms(local, basis, map, row);
        // The resolved eigen axial load is removed once from the axial end
        // actions, exactly as the ordinary route's axial-effect correction.
        let offset = match row {
            0 if eigen != 0.0 => vec![vec![eigen]],
            6 if eigen != 0.0 => vec![vec![-eigen]],
            _ => vec![],
        };
        rows.push(FunctionalDescriptor {
            key: key(
                case,
                FunctionalQuantity::MemberEnd {
                    member: member.to_owned(),
                    end: if row < 6 { MemberEnd::I } else { MemberEnd::J },
                    row: (row % 6) as u8,
                },
                row,
                END_CONVENTION,
            ),
            offset,
            terms,
        });
    }
    let mut descriptors = rows.clone();
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
                case,
                FunctionalQuantity::MemberSection {
                    member: member.to_owned(),
                    station_bits: fraction.to_bits(),
                    component: component as u8,
                },
                component,
                FunctionalConvention::SectionLocal,
            );
            descriptors.push(row);
        }
    }
    descriptors
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
            &source.congruence,
            &mut budget,
        )
        .result?;
        stage = "exact source solve";
        let response = context.solve_with_budget(&mut budget).result?;
        stage = "complete affine plan";
        let plan = FunctionalPlan::new_with_hints(&context, &source.descriptors, &|index| source.hint(index), &mut budget).result?;
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
            supports,
            retained,
            summary,
        )) => Ok(SelectedSourceRecovery {
            displacements,
            reactions,
            published_nodal,
            published_nodal_indices,
            members,
            supports,
            retained,
            summary,
            budget,
            retained_identity_digest: None,
        }),
        Err(error) => Err(failure(stage, error, &budget)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LinearStaticPreviewRequest;
    use serde_json::{json, Value};
    use std::cell::Cell;

    thread_local! {
        static UNIT_PRECHECK: Cell<Option<usize>> = const { Cell::new(None) };
        static UNIT_LIMIT_RAISED: Cell<bool> = const { Cell::new(false) };
        static DESCRIPTORS_STARTED: Cell<bool> = const { Cell::new(false) };
    }
    /// The unit precheck's observation point (RF-13, RF-13b): records the
    /// count. Only RF-13b raises the limit, to build the descriptors of a model
    /// above the law and compare them with the count.
    pub(super) fn unit_precheck(units: usize, limit: usize) -> usize {
        UNIT_PRECHECK.with(|c| c.set(Some(units)));
        if UNIT_LIMIT_RAISED.with(Cell::get) {
            usize::MAX
        } else {
            limit
        }
    }
    pub(super) fn descriptor_construction_started() {
        DESCRIPTORS_STARTED.with(|c| c.set(true));
    }
    thread_local! {
        static PERTURB_TRANSFORM: Cell<Option<(usize, usize, usize)>> = const { Cell::new(None) };
        static PERTURB_CONTRIBUTION: Cell<Option<usize>> = const { Cell::new(None) };
    }
    /// One ulp away from `x` (the smallest subnormal from a zero).
    fn one_ulp(x: f64) -> f64 {
        if x == 0.0 {
            f64::from_bits(1)
        } else {
            f64::from_bits(x.to_bits() + 1)
        }
    }
    /// RF-9's fault: one entry of one member's actual transform, changed by one
    /// ulp after the frame/pipe operand identity check.
    pub(super) fn perturbed_transform(member: usize, mut t: Matrix12) -> Matrix12 {
        if let Some((m, row, col)) = PERTURB_TRANSFORM.get() {
            if m == member {
                t[row][col] = one_ulp(t[row][col]);
            }
        }
        t
    }
    /// RF-12's fault: one rounded assembly contribution, changed by one ulp.
    pub(super) fn perturbed_assembly(mut assembly: AssemblyEvidence) -> AssemblyEvidence {
        if let Some(index) = PERTURB_CONTRIBUTION.get() {
            let value = &mut assembly.contributions[index].value;
            *value = one_ulp(*value);
        }
        assembly
    }
    struct Fixture {
        model: PreviewModel,
        built: BuiltModel,
        stiffness: Vec<Vec<f64>>,
        force: AssembledForce,
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
            let mut stiffness = super::super::assemble_global_stiffness_with_connectors(
                built.nodes.len(),
                &built.frame_elements,
                &built.connectors,
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
            let force = super::super::nodal_and_eigen_case_force(&loads, &[], &built).unwrap();
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
                load_state: None,
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
        // S11-F: the binary64 fold absorbs the 1.0 (the paths differ), and
        // the case force is now the correctly rounded net of the three terms.
        assert_eq!([1.0e16, 1.0, -1.0e16].iter().fold(0.0, |s, v| s + v), 0.0);
        assert_eq!(fixture.force.values()[9], 1.0);
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

    /// D7 (DEL-04-07 `pressure-contract.md`, "T3 coexistence and B7";
    /// HELP_HUMAN 2026-10-10): the v3 family T4-U2 admits, the realized bend
    /// under `3.0.0/exact_pressure_v3`, never reaches a retained source
    /// solve, since T4-U2 does not extend the source method. In
    /// `prepare_sources` it fails closed before any source is formed, by its
    /// own family name and ahead of the actual-dimensions check (frames =
    /// members), which a realized arc or a connector-replaced span would
    /// otherwise fail as a source mismatch; T4-U3's objective connector is
    /// declined the same way. A straight v3 model is declined by the
    /// not-joined name.
    #[test]
    fn d7_realized_bend_v3_family_never_reaches_a_source_solve() {
        let documents: serde_json::Value = serde_json::from_str(include_str!(
            "../../../validation/references/t4_i7/u2_document_sketches.json"
        ))
        .unwrap();
        // B7b: the frozen sketch's copy is re-authored to the current 0.8.0 form.
        let mut sketch = documents["sketches"]["U2-L-ANCH-PTW-K2"]["0.3.0"]["document"]["model"].clone();
        crate::b7b_admission_tests::reauthor_current_model(&mut sketch);
        let mut model: PreviewModel = serde_json::from_value(sketch).unwrap();
        // The product's own preparation: units, then the exact route's derived G.
        let (mut materials, mut diagnostics) = (model.materials.clone(), Vec::new());
        super::super::normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        super::super::pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        model.materials = materials;
        let refuse = |fixture: &Fixture| {
            prepare_sources(&fixture.input(), &mut AttemptBudget::new(exact::Limits::default())).err()
        };
        let bend = Fixture::new(model.clone());
        assert!(!bend.built.curved_bend_elements.is_empty(), "precondition: a realized bend");
        assert_eq!(bend.built.frame_elements.len() + bend.built.curved_bend_elements.len(), bend.built.pipes.len());
        const BEND: &str = "realized curved bend (curved_bend_macro_element) is not a straight source family";
        assert_eq!(refuse(&bend), Some(RecoveryError::Unsupported(BEND)));
        // The family decline precedes the dimension closure: with the
        // stiffness dimension broken too, the family name still decides.
        let mut broken = Fixture::new(model.clone());
        broken.stiffness.pop();
        assert_eq!(refuse(&broken), Some(RecoveryError::Unsupported(BEND)));
        // T4-U3's objective connector: its replaced span has no straight frame.
        let references: serde_json::Value = serde_json::from_str(include_str!(
            "../../../validation/references/t4_i12/u3_reference_cases.json"
        ))
        .unwrap();
        let mut connector_document =
            references["cases"]["U3-SYS-DEMO-CONNECTOR-001"]["inputs"]["document_v3_0.3.0"]["model"].clone();
        crate::b7b_admission_tests::reauthor_current_model(&mut connector_document);
        let mut connector: PreviewModel = serde_json::from_value(connector_document).unwrap();
        let (mut materials, mut diagnostics) = (connector.materials.clone(), Vec::new());
        super::super::normalize_model_units(&mut connector, &mut materials, &mut diagnostics);
        super::super::pressure_material::resolve_base(&connector, &mut materials, &mut diagnostics);
        connector.materials = materials;
        let joint = Fixture::new(connector);
        assert!(!joint.built.connectors.is_empty(), "precondition: an objective connector");
        assert!(joint.built.frame_elements.len() < joint.built.pipes.len(), "precondition: a replaced span");
        assert_eq!(
            refuse(&joint),
            Some(RecoveryError::Unsupported("objective connector is not a straight source family"))
        );
        // The pressure-source join (b7-plan): a straight v3 model is joined, so
        // no family decline applies and its sources prepare.
        let mut straight_model = Fixture::model();
        straight_model.pressure_contract = model.pressure_contract.clone();
        straight_model.schema_version = model.schema_version.clone();
        straight_model.load_cases[0].pressure_regions = Some(Vec::new());
        let straight = Fixture::new(straight_model);
        assert_eq!(refuse(&straight), None);
    }

    // RV127 N-4: `validate_profile` refuses every one of these documents before
    // a solve, so this gate is defence in depth for the non-exact namespace;
    // it is pinned here at unit level (U3).
    #[test]
    fn non_exact_source_blocks_gate_refuses_a_later_version_a_contract_or_regions() {
        const GATE: &str = "preview source-blocks namespace requires a pressure-free model without pressure contract or regions";
        let refuse = |fixture: &Fixture| {
            prepare_sources(
                &fixture.input(),
                &mut AttemptBudget::new(exact::Limits::default()),
            )
            .err()
        };
        let fixture = Fixture::new(Fixture::model());
        assert_eq!(fixture.model.schema_version, crate::MODEL_SCHEMA_VERSION);
        assert_eq!(refuse(&fixture), None);
        // Another version (an exact one, or a retired one) is outside the pressure-free namespace.
        for version in ["0.3.0", "0.2.0", "0.1.0"] {
            let mut later = Fixture::new(Fixture::model());
            later.model.schema_version = version.into();
            assert_eq!(refuse(&later), Some(RecoveryError::Unsupported(GATE)), "{version}");
        }
        for (version, mode) in [
            ("1.0.0", "legacy_pressure_v1"),
            ("2.0.0", "exact_straight_pressure_v2"),
        ] {
            let mut contract = Fixture::new(Fixture::model());
            contract.model.pressure_contract = Some(super::super::PressureContractInput {
                version: Some(version.into()),
                mode: Some(mode.into()),
                shear_model: Some("euler_bernoulli".into()),
            });
            assert_eq!(
                refuse(&contract),
                Some(RecoveryError::Unsupported(GATE)),
                "{version}/{mode}"
            );
        }
        let mut regions = Fixture::new(Fixture::model());
        regions.model.load_cases[0].pressure_regions = Some(Vec::new());
        assert_eq!(refuse(&regions), Some(RecoveryError::Unsupported(GATE)));
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
        assert!(failure.work.rejected.exact().unwrap() > 0);
        assert_eq!(failure.error, RecoveryError::Exact(exact::Error::Budget));
    }

    /// The committed current (0.8.0, v3) n05 source control.
    fn pressure_source_fixture() -> Fixture {
        let raw: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
        )).unwrap();
        let request: LinearStaticPreviewRequest = serde_json::from_value(raw).unwrap();
        let mut model = request.model;
        let mut diagnostics = Vec::new();
        let mut materials = model.materials.clone();
        super::super::pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        assert!(!super::super::has_blocking(&diagnostics), "{diagnostics:?}");
        model.materials = materials;
        Fixture::new(model)
    }

    #[test]
    fn pressure_source_empty_inventory_prepares_without_the_old_v3_gate() {
        let fixture = pressure_source_fixture();
        let prepared = prepare_sources(&fixture.input(), &mut AttemptBudget::new(exact::Limits::default()));
        assert!(prepared.is_ok(), "{:?}", prepared.err());
    }

    /// O4 (DEL-04-08 Design `o4-source-extension.md` §3.1, §3.2, §6; replaces
    /// `o4_prepare_sources_declines_by_name`): `prepare_sources` no longer
    /// declines a timoshenko_annulus_energy_v1 document, a nonzero mill
    /// tolerance or a nonzero corrosion allowance. A Timoshenko document's
    /// declared operand is each frame's own Timoshenko local matrix, closed
    /// against the pipe element and the assembly; it selects, and its public
    /// envelope publishes pressure-source-1, read eligible with its captured
    /// invocation in both modes. The budget/DOF refusal still comes first, D7's
    /// bend decline keeps its name, and the pressure-free route keeps its one
    /// t - c_m section.
    #[test]
    fn o4_timoshenko_source_case_selects_and_publishes() {
        use open_pipe_stress_result_export::reader_verdict as rv;
        const TIMOSHENKO: &str = "timoshenko_annulus_energy_v1";
        let length = |value: f64| Some(Quantity { value, unit: "m".into() });
        let prepare = |fixture: &Fixture, limits: exact::Limits| {
            prepare_sources(&fixture.input(), &mut AttemptBudget::new(limits)).err()
        };
        let refuse = |fixture: &Fixture| prepare(fixture, exact::Limits::default());
        let control = pressure_source_fixture();
        assert_eq!(refuse(&control), None, "control: the n05 v3 source prepares");
        // Each document fact, authored before the build (the built model then
        // carries its Timoshenko leaf and its D-5 bases).
        let rebuilt = |edit: &dyn Fn(&mut PreviewModel)| {
            let mut model = pressure_source_fixture().model;
            edit(&mut model);
            Fixture::new(model)
        };
        let timoshenko = || rebuilt(&|m| m.pressure_contract.as_mut().unwrap().shear_model = Some(TIMOSHENKO.into()));
        let mill = || rebuilt(&|m| m.pipe_segments[0].section.mill_tolerance = length(0.0005));
        let allowance = || rebuilt(&|m| m.pipe_segments[0].section.corrosion_allowance = length(0.001));
        let all = || rebuilt(&|m| {
            m.pressure_contract.as_mut().unwrap().shear_model = Some(TIMOSHENKO.into());
            m.pipe_segments[0].section.mill_tolerance = length(0.0005);
            m.pipe_segments[0].section.corrosion_allowance = length(0.001);
        });
        for (label, fixture) in [("timoshenko", timoshenko()), ("mill", mill()), ("allowance", allowance()), ("all", all())] {
            assert_eq!(refuse(&fixture), None, "{label}: prepares");
            // The budget/DOF refusal is unchanged and still first.
            assert_eq!(prepare(&fixture, exact::Limits { dofs: 0, ..exact::Limits::default() }),
                Some(RecoveryError::Exact(exact::Error::Budget)), "{label}: budget first");
            // The dimension closure now names a broken stiffness dimension.
            let mut broken = fixture;
            broken.stiffness.pop();
            assert_eq!(refuse(&broken), Some(RecoveryError::SourceMismatch("actual model/source dimensions")), "{label}");
        }
        // The declared operand is the frame's own Timoshenko local matrix: its
        // bending entries differ from the Euler-Bernoulli twin's, and the
        // selected source declares exactly the matrix the ordinary route
        // assembles (the closure above), so the solve selects.
        let (euler, timo) = (control, timoshenko());
        for (e, t) in euler.built.frame_elements.iter().zip(&timo.built.frame_elements) {
            assert!(t.section.shear_area().is_some() && e.section.shear_area().is_none());
            let (le, lt) = (e.local_stiffness().unwrap(), t.local_stiffness().unwrap());
            assert_ne!(le[5][11].to_bits(), lt[5][11].to_bits(), "the bending k2 entry is the Timoshenko one");
            assert_eq!(le[0][0].to_bits(), lt[0][0].to_bits(), "the axial entry is shared");
        }
        for (label, fixture) in [("timoshenko", timo), ("all", all())] {
            let selected = solve(fixture.input(), exact::Limits { operations: crate::PHYSICS_SOURCE_WORK_LIMIT, ..exact::Limits::default() });
            assert!(selected.is_ok(), "{label}: {:?}", selected.err().map(|f| f.error));
        }
        // The public route: the Timoshenko twin of the committed n05 control
        // publishes pressure-source-1 and reads eligible with its invocation.
        let mut request: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
        )).unwrap();
        request["model"]["pressure_contract"]["shear_model"] = json!(TIMOSHENKO);
        for mode in MODES {
            let envelope = serde_json::to_value(crate::run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
            assert_eq!(envelope["producer"]["semantic_contract_id"], json!(crate::exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID), "{mode:?}");
            assert!(!envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"), "{mode:?}");
            let section = &envelope["contract_evidence"]["exact_cases"][0]["pipe_sections"][0];
            assert_eq!(section["beam_theory"]["model"], json!(TIMOSHENKO), "{mode:?}");
            let invocation = json!({"request": request, "solver_mode": mode.as_str()});
            let verdict = rv::validate("pressure-source", &envelope, Some(&request["model"]), Some(&invocation));
            assert_eq!(verdict, json!({"ok": true, "eligible": true, "findings": []}), "{mode:?}");
        }
        // After the D7 family declines: a realized bend with mill tolerance in
        // a Timoshenko document is declined as a bend.
        let documents: Value = serde_json::from_str(include_str!(
            "../../../validation/references/t4_i7/u2_document_sketches.json"
        ))
        .unwrap();
        let mut sketch = documents["sketches"]["U2-L-ANCH-PTW-K2"]["0.3.0"]["document"]["model"].clone();
        crate::b7b_admission_tests::reauthor_current_model(&mut sketch);
        let mut model: PreviewModel = serde_json::from_value(sketch).unwrap();
        for pipe in &mut model.pipe_segments {
            pipe.section.mill_tolerance = length(0.0005);
        }
        model.pressure_contract.as_mut().unwrap().shear_model = Some(TIMOSHENKO.into());
        let (mut materials, mut diagnostics) = (model.materials.clone(), Vec::new());
        super::super::normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        super::super::pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        model.materials = materials;
        let bend = Fixture::new(model);
        assert!(!bend.built.curved_bend_elements.is_empty(), "precondition: a realized bend");
        assert_eq!(refuse(&bend), Some(RecoveryError::Unsupported(
            "realized curved bend (curved_bend_macro_element) is not a straight source family")));
        // The pressure-free route: nonzero mill tolerance keeps its one
        // t - c_m section and prepares as before.
        let mut preview = Fixture::model();
        preview.pipe_segments[0].section.mill_tolerance = length(0.0005);
        let preview = Fixture::new(preview);
        assert!(crate::model_format::is_pressure_free(&preview.model));
        assert_eq!(refuse(&preview), None, "pressure-free mill tolerance");
    }

    #[test]
    fn pressure_source_preparation_declines_every_present_pressure_region() {
        const EMPTY_REQUIRED: &str = "exact source recovery requires explicitly empty pressure regions";
        for pressure in [0.0, 1.0, -1.0] {
            let mut fixture = pressure_source_fixture();
            fixture.model.load_cases[0].pressure_regions = Some(vec![serde_json::from_value(json!({
                "id":"invented:source-family-region", "pressure":{"value":pressure,"unit":"Pa"}
            })).unwrap()]);
            assert_eq!(prepare_sources(&fixture.input(), &mut AttemptBudget::new(exact::Limits::default())).err(), Some(RecoveryError::Unsupported(EMPTY_REQUIRED)));
        }
        let mut fixture = pressure_source_fixture();
        fixture.model.load_cases[0].pressure_regions = None;
        assert_eq!(prepare_sources(&fixture.input(), &mut AttemptBudget::new(exact::Limits::default())).err(), Some(RecoveryError::Unsupported(EMPTY_REQUIRED)));
    }

    #[test]
    fn pressure_source_denied_attempt_keeps_the_existing_work_boundary() {
        let fixture = pressure_source_fixture();
        let failure = solve(fixture.input(), exact::Limits { operations: 0, ..exact::Limits::default() }).unwrap_err();
        assert_eq!(failure.stage, "source closure");
        assert_eq!(failure.work.charged, 0);
        assert!(failure.work.rejected.exact().unwrap() > 0);
        assert_eq!(failure.error, RecoveryError::Exact(exact::Error::Budget));
    }

    #[test]
    fn signed_permutation_uses_all_actual_entries_without_tolerance() {
        let mut t = [[0.0; 12]; 12];
        for i in 0..12 {
            t[i][11 - i] = if i % 2 == 0 { -1.0 } else { 1.0 };
        }
        assert!(signed_permutation(&t).is_some());
        t[0][0] = f64::MIN_POSITIVE;
        assert!(signed_permutation(&t).is_none());
        t[0][0] = 0.0;
        t[0][11] = 1.0 - f64::EPSILON;
        assert!(signed_permutation(&t).is_none());
    }

    // ---- RF source-method design §3 and §7 (DEL-04-08 `rf-source-method-extension.md`) ----

    const RF_PROVENANCE: &str = "invented_rf_source_method_test_no_external_project_data";
    const MODES: [crate::PreviewSolverMode; 2] = [
        crate::PreviewSolverMode::SparseInteractive,
        crate::PreviewSolverMode::DenseScrutiny,
    ];
    /// The invented order-4 request of RF-15 (kept for J2's later base): a
    /// two-member axis-aligned chain whose only twist restraint is a soft root
    /// spring, explicit `euler_bernoulli`, zero mill tolerance and corrosion
    /// allowance. Its exact free blocks have orders 3, 2, 4 and 4.
    const ORDER_FOUR_REQUEST: &str =
        include_str!("../tests/fixtures/rf_source_method/order_four_sensitive.request.json");

    /// An invented current exact request on the committed n05 control's
    /// document: its nodes, straight members (from, to, y reference),
    /// supports and nodal loads replaced.
    fn invented_request(
        id: &str,
        nodes: &[(&str, [f64; 3])],
        members: &[(&str, &str, [f64; 3])],
        supports: Vec<Value>,
        loads: Vec<Value>,
    ) -> Value {
        let mut request: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
        ))
        .unwrap();
        let model = &mut request["model"];
        let segment = model["pipe_segments"][0].clone();
        model["project"]["id"] = json!(format!("synthetic:rf-source-method:{id}"));
        model["nodes"] = json!(nodes
            .iter()
            .map(|(id, p)| json!({"id": id, "label": format!("Invented {id}"), "provenance": RF_PROVENANCE,
                "position": {"x": p[0], "y": p[1], "z": p[2]}}))
            .collect::<Vec<_>>());
        model["pipe_segments"] = json!(members
            .iter()
            .enumerate()
            .map(|(i, (from, to, y))| {
                let mut s = segment.clone();
                s["id"] = json!(format!("member-{i}"));
                s["label"] = json!(format!("Invented member-{i}"));
                s["provenance"] = json!(RF_PROVENANCE);
                s["from"] = json!(from);
                s["to"] = json!(to);
                s["y_reference"] = json!({"x": y[0], "y": y[1], "z": y[2]});
                s["section"]["mill_tolerance"] = json!({"unit": "m", "value": 0.0});
                s
            })
            .collect::<Vec<_>>());
        model["supports"] = json!(supports);
        model["load_cases"][0]["primitive_loads"] = json!(loads);
        request
    }
    fn rigid(id: &str, node: &str, restraints: &[&str]) -> Value {
        json!({"family": "anchor", "id": id, "label": format!("Invented {id}"), "node": node,
            "provenance": RF_PROVENANCE, "restraints": restraints})
    }
    fn rotational_spring(id: &str, node: &str, dof: &str, value: f64) -> Value {
        json!({"family": "spring", "id": id, "label": format!("Invented {id}"), "node": node,
            "provenance": RF_PROVENANCE, "restraints": [dof],
            "stiffness": {"dof": dof, "value": {"unit": "N*m/rad", "value": value}}})
    }
    /// A nodal moment ("RX", "RY", "RZ") or force ("global_x", ...).
    fn nodal_load(id: &str, node: &str, direction: &str, value: f64) -> Value {
        let (category, dimension, unit) = if direction.starts_with('R') {
            ("concentrated_moment", "moment", "N*m")
        } else {
            ("concentrated_force", "force", "N")
        };
        json!({"category": category, "dimension": dimension, "direction": direction, "id": id,
            "magnitude": {"unit": unit, "value": value}, "provenance": RF_PROVENANCE,
            "target": {"node": node, "type": "node"}})
    }
    fn request_fixture(request: &Value) -> Fixture {
        let request: LinearStaticPreviewRequest = serde_json::from_value(request.clone()).unwrap();
        let mut model = request.model;
        let mut diagnostics = Vec::new();
        let mut materials = model.materials.clone();
        super::super::pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        assert!(!super::super::has_blocking(&diagnostics), "{diagnostics:?}");
        model.materials = materials;
        Fixture::new(model)
    }
    /// One (3,4,0) member between a root whose rotations about x and y are held
    /// only by soft springs and a pinned tip: the member's rigid twist about
    /// its own axis is nearly free, so the ordinary route is sensitive. Exact
    /// free blocks {RX, RY at both ends} and {RZ at the tip}.
    fn skew_pin() -> Value {
        invented_request(
            "skew-pin",
            &[("root", [0.0, 0.0, 0.0]), ("tip", [1.5, 2.0, 0.0])],
            &[("root", "tip", [0.0, 0.0, 1.0])],
            vec![
                rigid("root-pin", "root", &["UX", "UY", "UZ", "RZ"]),
                rotational_spring("root-spring-x", "root", "RX", 1.0e-4),
                rotational_spring("root-spring-y", "root", "RY", 1.0e-4),
                rigid("tip-pin", "tip", &["UX", "UY", "UZ"]),
            ],
            vec![
                nodal_load("tip-twist", "tip", "RX", 1.0),
                nodal_load("tip-moment-z", "tip", "RZ", -0.5),
            ],
        )
    }
    /// A (3,4,0) congruence member from an anchor to a pin, then an
    /// axis-aligned (signed-permutation) member to a pin with a rotational
    /// spring. Exact free blocks of orders 4 and 2.
    fn mixed_pin() -> Value {
        invented_request(
            "mixed-pin",
            &[("a", [0.0, 0.0, 0.0]), ("b", [1.5, 2.0, 0.0]), ("c", [3.5, 2.0, 0.0])],
            &[("a", "b", [0.0, 0.0, 1.0]), ("b", "c", [0.0, 1.0, 0.0])],
            vec![
                rigid("a-anchor", "a", &["UX", "UY", "UZ", "RX", "RY", "RZ"]),
                rigid("b-pin", "b", &["UX", "UY", "UZ"]),
                rigid("c-pin", "c", &["UX", "UY", "UZ"]),
                rotational_spring("c-spring", "c", "RX", 50.0),
            ],
            vec![
                nodal_load("b-twist", "b", "RX", 1.0),
                nodal_load("b-moment-z", "b", "RZ", -2.0),
                nodal_load("b-push", "b", "global_x", 10.0),
                nodal_load("c-moment-y", "c", "RY", 0.5),
            ],
        )
    }
    /// `congruent` congruence members in a pinned chain from an anchor, in
    /// turning directions, then one axis-aligned member to a pin with a spring.
    fn skew_chain(congruent: usize) -> Value {
        const DIRECTIONS: [[f64; 3]; 5] =
            [[3.0, 4.0, 0.0], [1.0, 2.0, 2.0], [1.0, 2.0, -2.0], [2.0, 1.0, 2.0], [2.0, -2.0, -1.0]];
        let names: Vec<String> = (0..congruent + 2).map(|i| format!("p{i}")).collect();
        let mut point = [0.0; 3];
        let mut nodes = vec![(names[0].as_str(), point)];
        for i in 0..=congruent {
            let d = if i < congruent { DIRECTIONS[i % 5] } else { [2.0, 0.0, 0.0] };
            point = [point[0] + 0.5 * d[0], point[1] + 0.5 * d[1], point[2] + 0.5 * d[2]];
            nodes.push((names[i + 1].as_str(), point));
        }
        let members: Vec<_> = (0..=congruent)
            .map(|i| {
                let y = if i < congruent { [0.0, 0.0, 1.0] } else { [0.0, 1.0, 0.0] };
                (names[i].as_str(), names[i + 1].as_str(), y)
            })
            .collect();
        let mut supports = vec![rigid("anchor", &names[0], &["UX", "UY", "UZ", "RX", "RY", "RZ"])];
        for name in &names[1..] {
            supports.push(rigid(&format!("{name}-pin"), name, &["UX", "UY", "UZ"]));
        }
        let last = &names[congruent + 1];
        supports.push(rotational_spring("end-spring", last, "RX", 50.0));
        invented_request(
            &format!("skew-chain-{congruent}"),
            &nodes,
            &members,
            supports,
            vec![
                nodal_load("first-twist", &names[1], "RX", 1.0),
                nodal_load("first-push", &names[1], "global_y", 3.0),
                nodal_load("last-moment", last, "RZ", 2.0),
            ],
        )
    }
    fn prepare(fixture: &Fixture) -> Result<Sources, RecoveryError> {
        prepare_sources(&fixture.input(), &mut AttemptBudget::new(exact::Limits::default()))
    }
    fn generous() -> exact::Limits {
        exact::Limits { operations: 50_000_000, ..exact::Limits::default() }
    }
    /// The units `descriptors_charge` (frame kernel `functionals.rs`) counts for
    /// built descriptors: one per descriptor, and per offset product and term
    /// product its own unit and its atoms, and one per term.
    fn charged_units(descriptors: &[FunctionalDescriptor]) -> usize {
        let products = |p: &Products| p.len() + p.iter().map(Vec::len).sum::<usize>();
        descriptors
            .iter()
            .map(|d| {
                1 + products(&d.offset)
                    + d.terms.len()
                    + d.terms.iter().map(|t| products(&t.products)).sum::<usize>()
            })
            .sum()
    }
    /// The kernel's own `descriptors_charge`, where the source's Context
    /// prepares: the functional plan of these descriptors under `source_terms`.
    fn plan_under(sources: &Sources, input: &Input<'_>, source_terms: usize) -> Option<Result<(), exact::Error>> {
        let mut budget = AttemptBudget::new(exact::Limits { source_terms, ..generous() });
        let context = exact::Context::prepare_with_budget(
            &sources.system(input),
            &sources.identity,
            exact::ForceBasis::IdentifiedContributions(&sources.force_terms),
            &sources.congruence,
            &mut budget,
        )
        .result
        .ok()?;
        Some(FunctionalPlan::new(&context, &sources.descriptors, &mut budget).result.map(|_| ()))
    }

    /// RF-9 (design §3.2 (1)): a congruence member's transform must be
    /// blockdiag(R, R, R, R): four bit-identical diagonal blocks and ±0
    /// elsewhere. Checked on the block-rotation closure itself and through
    /// `prepare_sources`, with one entry of the actual transform changed.
    #[test]
    fn congruence_requires_identical_rotation_blocks() {
        let fixture = request_fixture(&skew_pin());
        let t = fixture.built.frame_elements[0].orientation().unwrap().transformation_matrix();
        assert!(signed_permutation(&t).is_none(), "precondition: a (3,4,0) member");
        assert_eq!(block_rotation(&t), Ok(()));
        let decline = Err(RecoveryError::Unsupported(BLOCK_ROTATION_DECLINE));
        let mut signed_zero = t;
        signed_zero[0][3] = -0.0;
        signed_zero[7][2] = -0.0;
        assert_eq!(block_rotation(&signed_zero), Ok(()), "a signed zero off the blocks");
        for block in 0..4 {
            for (row, col) in [(0, 0), (0, 2), (1, 1), (2, 2), (2, 0)] {
                let mut changed = t;
                let entry = &mut changed[block * 3 + row][block * 3 + col];
                *entry = one_ulp(*entry);
                assert_eq!(block_rotation(&changed), decline, "block {block} entry ({row}, {col})");
            }
        }
        for (row, col) in [(0, 3), (3, 0), (11, 0), (5, 9), (6, 5), (2, 11)] {
            let mut changed = t;
            changed[row][col] = f64::from_bits(1);
            assert_eq!(block_rotation(&changed), decline, "off-block entry ({row}, {col})");
        }
        // Through the closure: the unchanged member is a congruence member.
        let sources = prepare(&fixture).unwrap();
        assert_eq!(sources.congruence.len(), 1);
        assert_eq!(sources.congruence[0].transform, t);
        assert_eq!(sources.congruence[0].contributions, 0..144);
        // One entry of a later block, or one off-block entry, is declined by
        // name before any formation closure or descriptor.
        for (row, col) in [(4, 4), (9, 10), (0, 6), (8, 1)] {
            PERTURB_TRANSFORM.set(Some((0, row, col)));
            DESCRIPTORS_STARTED.set(false);
            let outcome = prepare(&fixture).err();
            PERTURB_TRANSFORM.set(None);
            assert_eq!(outcome, decline.clone().err(), "actual transform entry ({row}, {col})");
            assert!(!DESCRIPTORS_STARTED.get());
        }
    }

    /// RF-10, product part: a single (3,4,0) member selects through the source
    /// method as a congruence member (exact blocks of order 4 and 1), and its
    /// selection replays and binds against the current invocation.
    #[test]
    fn congruence_member_selects_through_the_source_method() {
        let fixture = request_fixture(&skew_pin());
        let mut selected = solve(
            fixture.input(),
            exact::Limits { operations: crate::PHYSICS_SOURCE_WORK_LIMIT, ..exact::Limits::default() },
        )
        .unwrap();
        let response = selected.retained().response();
        assert_eq!(response.congruence_members().len(), 1);
        let orders: Vec<usize> = response.block_witnesses().iter().map(|b| b.dofs().len()).collect();
        assert_eq!(orders, [4, 1]);
        assert!(selected.summary().work.charged <= crate::PHYSICS_SOURCE_WORK_LIMIT);
        selected.replay_against(fixture.input()).unwrap();
        selected.check_binding_against(fixture.input()).unwrap();
    }

    /// RF-11 (design §3.4): with a congruence member beside a signed
    /// permutation, at every node DOF the members' exact global end actions
    /// Tᵀ·rows (T the frame's own transform) equal the support actions (rigid
    /// and spring) plus the loads exactly. Each residual is a declared
    /// functional of the exact response; its exact value is zero.
    #[test]
    fn congruence_end_actions_equilibrate_exactly() {
        let fixture = request_fixture(&mixed_pin());
        let input = fixture.input();
        let mut budget = AttemptBudget::new(generous());
        let sources = prepare_sources(&input, &mut budget).unwrap();
        assert_eq!(sources.congruence.len(), 1, "precondition: one congruence member");
        let context = exact::Context::prepare_with_budget(
            &sources.system(&input),
            &sources.identity,
            exact::ForceBasis::IdentifiedContributions(&sources.force_terms),
            &sources.congruence,
            &mut budget,
        )
        .result
        .unwrap();
        let response = context.solve_with_budget(&mut budget).result.unwrap();
        let n = input.force.len();
        let extend = |out: &mut FunctionalDescriptor, row: &FunctionalDescriptor, factor: f64| {
            let scaled = |p: &Vec<f64>| p.iter().copied().chain([factor]).collect::<Vec<_>>();
            out.offset.extend(row.offset.iter().map(scaled));
            out.terms.extend(row.terms.iter().map(|t| AffineTerm { dof: t.dof, products: t.products.iter().map(scaled).collect() }));
        };
        let mut residuals = Vec::new();
        for d in 0..n {
            let mut residual = FunctionalDescriptor {
                key: FunctionalKey {
                    case_id: input.load_case.id.clone(),
                    quantity: FunctionalQuantity::DeclaredAffine { label: format!("equilibrium:{d}") },
                    unit: FunctionalUnit::Dimensionless,
                    convention: FunctionalConvention::DeclaredAffine,
                },
                offset: vec![],
                terms: vec![],
            };
            for (m, frame) in input.built.frame_elements.iter().enumerate() {
                let t = frame.orientation().unwrap().transformation_matrix();
                let map = element_dof_map(frame.node_i.index, frame.node_j.index);
                for a in (0..12).filter(|&a| map[a] == d) {
                    for r in (0..12).filter(|&r| t[r][a] != 0.0) {
                        extend(&mut residual, &sources.descriptors[m * 42 + r], t[r][a]);
                    }
                }
            }
            // Every support's row, rigid or spring: a spring support's row is its
            // spring's action on the structure (the ground-spring rows repeat it).
            for (owner, (_, node)) in sources.support_ids.iter().enumerate() {
                if node * 6 <= d && d < node * 6 + 6 {
                    extend(&mut residual, &sources.descriptors[sources.support_start + owner * 6 + d % 6], -1.0);
                }
            }
            residual.offset.extend(sources.force_terms.iter().filter(|f| f.dof == d).map(|f| vec![-f.value]));
            residuals.push(residual);
        }
        let plan = FunctionalPlan::new(&context, &residuals, &mut budget).result.unwrap();
        let set = response.evaluate_functionals(&plan, &mut budget).result.unwrap();
        for d in 0..n {
            assert_eq!(set.ratio(d).unwrap().sign(), exact::Sign::Zero, "equilibrium at DOF {d}");
        }
        // Not vacuous: the congruence member's end rows act.
        let plan = FunctionalPlan::new(&context, &sources.descriptors[..12], &mut budget).result.unwrap();
        let ends = response.evaluate_functionals(&plan, &mut budget).result.unwrap();
        assert!((0..12).any(|r| ends.ratio(r).unwrap().sign() != exact::Sign::Zero));
    }

    /// RF-12 (design §3.2 (2)): a congruence member's 144 rounded
    /// contributions must be its own `global_stiffness()` bits at its map. A
    /// contribution changed by one ulp is refused by that closure alone: the
    /// actual aggregate is re-folded from the changed contributions, so the
    /// ordered-fold coverage still holds.
    #[test]
    fn congruence_closure_refuses_a_changed_rounded_contribution() {
        const FORMATION: &str = "member global formation differs from its assembly contributions";
        let fixture = request_fixture(&skew_pin());
        let frame = &fixture.built.frame_elements[0];
        let global = frame.global_stiffness().unwrap();
        let map = element_dof_map(frame.node_i.index, frame.node_j.index);
        let springs: Vec<_> = fixture.springs.iter().map(|s| (s.node_dof.global_index(), s.stiffness.value)).collect();
        let assembly = |changed: Option<usize>| {
            let mut a = AssemblyEvidence::new(fixture.built.nodes.len(), &fixture.built.frame_elements, &[], &[], &springs).unwrap();
            if let Some(i) = changed {
                a.contributions[i].value = one_ulp(a.contributions[i].value);
            }
            a
        };
        let own = assembly(None);
        assert_eq!(close_congruence_formation(&global, &map, own.contributions.get(0..144)), Ok(()));
        let refused = Err(RecoveryError::SourceMismatch(FORMATION));
        for i in [0, 13, 77, 143] {
            let changed = assembly(Some(i));
            assert_eq!(close_congruence_formation(&global, &map, changed.contributions.get(0..144)), refused, "value {i}");
            let mut moved = own.contributions[..144].to_vec();
            moved[i].col = map[(i + 1) % 12];
            assert_eq!(close_congruence_formation(&global, &map, Some(&moved)), refused, "column {i}");
        }
        assert_eq!(close_congruence_formation(&global, &map, own.contributions.get(0..143)), refused);
        assert_eq!(close_congruence_formation(&global, &map, None), refused);
        // Through `prepare_sources`.
        for i in [0, 13, 77, 143] {
            let mut changed = request_fixture(&skew_pin());
            let folded = assembly(Some(i));
            let n = changed.stiffness.len();
            let mut aggregate = vec![vec![0.0; n]; n];
            for term in &folded.contributions {
                aggregate[term.row][term.col] += term.value;
            }
            changed.stiffness = aggregate;
            PERTURB_CONTRIBUTION.set(Some(i));
            let outcome = prepare(&changed).err();
            PERTURB_CONTRIBUTION.set(None);
            assert_eq!(outcome, refused.clone().err(), "contribution {i}");
        }
        assert!(prepare(&fixture).is_ok());
    }

    /// RF-13 (design §3.2 (3), §3.5): with congruence members, a model whose
    /// descriptors would pass the unchanged 16,384-unit law is declined by
    /// `Budget` in the source closure, before descriptor #1 is built. One
    /// member fewer, it builds; a signed-permutation model takes no precheck.
    #[test]
    fn congruence_unit_precheck_declines_before_descriptor_construction() {
        let above = request_fixture(&skew_chain(6));
        UNIT_PRECHECK.set(None);
        DESCRIPTORS_STARTED.set(false);
        assert_eq!(prepare(&above).err(), Some(RecoveryError::Exact(exact::Error::Budget)));
        let units = UNIT_PRECHECK.take().expect("the precheck ran");
        assert!(units > 16_384, "{units}");
        assert!(!DESCRIPTORS_STARTED.get(), "declined before descriptor #1");
        let failure = solve(above.input(), exact::Limits { operations: crate::PHYSICS_SOURCE_WORK_LIMIT, ..exact::Limits::default() }).unwrap_err();
        assert_eq!((failure.stage, failure.helper_stage), ("source closure", AttemptStage::SourceClosure));
        assert_eq!(failure.error, RecoveryError::Exact(exact::Error::Budget));
        let within = request_fixture(&skew_chain(5));
        DESCRIPTORS_STARTED.set(false);
        let sources = prepare(&within).unwrap();
        let units = UNIT_PRECHECK.take().expect("the precheck ran");
        assert!(units <= 16_384, "{units}");
        assert_eq!(units, charged_units(&sources.descriptors));
        assert!(DESCRIPTORS_STARTED.get());
        // A model whose members are all signed permutations takes no precheck.
        prepare(&pressure_source_fixture()).unwrap();
        assert_eq!(UNIT_PRECHECK.take(), None);
    }

    /// RF-13b (design §3.5; D3's memory bound rests on it): the precheck's
    /// count equals what `descriptors_charge` counts on the descriptors
    /// `prepare_sources` then builds: for every census case with a congruence
    /// member (the census's own requests and actual inputs, both modes), for
    /// constructed models of 1 to 5 congruence members beside a signed
    /// permutation (above the law too, with the limit raised for the
    /// comparison only), and member by member with an eigen axial offset.
    /// Where the source's Context prepares, the kernel's own count admits the
    /// descriptors at exactly that many units and refuses them at one fewer.
    #[test]
    fn congruence_unit_precheck_equals_descriptors_charge() {
        use super::j7_rf_census::{current_request, inputs, Owned};
        UNIT_LIMIT_RAISED.set(true);
        let mut kernel_checked = std::collections::BTreeSet::new();
        let mut check = |label: &str, input: Input<'_>| -> bool {
            UNIT_PRECHECK.set(None);
            let prepared = prepare_sources(&input, &mut AttemptBudget::new(exact::Limits::default()));
            let Some(units) = UNIT_PRECHECK.take() else {
                return false;
            };
            let sources = prepared.unwrap_or_else(|e| panic!("{label}: {e:?}"));
            assert!(!sources.congruence.is_empty(), "{label}");
            assert_eq!(units, charged_units(&sources.descriptors), "{label}");
            eprintln!("RF-13b {label}: {units} units, {} congruence members", sources.congruence.len());
            if units <= 16_384 {
                if let Some(admitted) = plan_under(&sources, &input, units) {
                    assert_eq!(admitted, Ok(()), "{label}");
                    assert_eq!(plan_under(&sources, &input, units - 1), Some(Err(exact::Error::Budget)), "{label}");
                    kernel_checked.insert(label.to_owned());
                }
            }
            true
        };
        let mut census = std::collections::BTreeSet::new();
        for case in inputs()["cases"].as_array().unwrap() {
            let id = case["id"].as_str().unwrap();
            let request = current_request(case);
            for mode in MODES {
                let Ok(owned) = Owned::actual(&request, mode) else { continue };
                if check(&format!("{id} {}", mode.as_str()), owned.input()) {
                    census.insert(id.to_owned());
                }
            }
        }
        // The 27 census transform cases (design §1.3) all reach the precheck;
        // RF-RANGE-SKEW-E-1000 stops at formation (`checked_value_range`).
        assert_eq!(census.len(), 27, "{census:?}");
        for congruent in 1..=6 {
            let fixture = request_fixture(&skew_chain(congruent));
            assert!(check(&format!("chain {congruent}"), fixture.input()));
        }
        for request in [skew_pin(), mixed_pin()] {
            let fixture = request_fixture(&request);
            assert!(check(request["model"]["project"]["id"].as_str().unwrap(), fixture.input()));
        }
        UNIT_LIMIT_RAISED.set(false);
        // The kernel's own count, where the Context prepares (blocks of order at
        // most 4): the census's four selectable transform cases in both modes,
        // and the constructed models that stay within order 4.
        let mut expected: std::collections::BTreeSet<String> = ["RF-SKEW-T-PIN-OFF-345-r1e-04", "RF-SKEW-T-PIN-OFF-345-r1e-08",
            "RF-SKEW-T-PIN-OFF-345-r1e-12", "RF-MECH-LINE345-RX-COMPANION"]
            .iter()
            .flat_map(|id| MODES.map(|mode| format!("{id} {}", mode.as_str())))
            .collect();
        expected.extend(["chain 1", "synthetic:rf-source-method:skew-pin", "synthetic:rf-source-method:mixed-pin"].map(String::from));
        assert_eq!(kernel_checked, expected);
        // Member by member, with and without an eigen axial offset.
        let fixture = request_fixture(&mixed_pin());
        for frame in &fixture.built.frame_elements {
            let local = frame.local_stiffness().unwrap();
            let t = frame.orientation().unwrap().transformation_matrix();
            let map = element_dof_map(frame.node_i.index, frame.node_j.index);
            let basis = match signed_permutation(&t) {
                Some(selector) => MemberBasis::Selector(selector),
                None => MemberBasis::Congruence(t),
            };
            for eigen in [0.0, -3.25e4] {
                let rows = member_descriptors("case", "member", &local, &basis, &map, 2.5, eigen);
                assert_eq!(rows.len(), 42);
                assert_eq!(member_descriptor_units(&local, &basis, eigen), charged_units(&rows));
            }
        }
        // O4 (§3.1): a Timoshenko member at phi = 2 exactly, where
        // k_tt' = (2 - phi)EI/(L(1 + phi)) is an exact zero in both bending
        // planes, on a signed-permutation and on a congruence orientation:
        // phi = ((12E)I)/((G As)L^2) = 12/6 at L = 1, and 12/(0.24 x 25) at L = 5.
        use open_pipe_stress_frame_kernel::{FrameElement, FrameNode, FrameSection};
        for (to, shear_area) in [([1.0, 0.0, 0.0], 6.0), ([3.0, 4.0, 0.0], 0.24)] {
            let section = FrameSection::timoshenko(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, shear_area).unwrap();
            let frame = FrameElement::new(
                FrameNode { index: 0, coordinates: [0.0; 3] },
                FrameNode { index: 1, coordinates: to },
                section,
                [0.0, 0.0, 1.0],
            )
            .unwrap();
            let local = frame.local_stiffness().unwrap();
            assert_eq!((local[4][10], local[5][11]), (0.0, 0.0), "{to:?}: the structurally zero k_tt' entries");
            let t = frame.orientation().unwrap().transformation_matrix();
            let basis = match signed_permutation(&t) {
                Some(selector) => MemberBasis::Selector(selector),
                None => MemberBasis::Congruence(t),
            };
            assert_eq!(matches!(basis, MemberBasis::Congruence(_)), to[1] != 0.0, "{to:?}");
            let map = element_dof_map(0, 1);
            for eigen in [0.0, -3.25e4] {
                let rows = member_descriptors("case", "member", &local, &basis, &map, frame.length().unwrap(), eigen);
                assert_eq!(rows.len(), 42);
                assert_eq!(member_descriptor_units(&local, &basis, eigen), charged_units(&rows), "{to:?}");
            }
        }
    }

    /// RF-14 (design §2.9), restated by the pre-0.4 failure unification
    /// (DEL-04-08 Design `pre04-failure-unification.md` §7). RF-14 pinned the
    /// routes' asymmetry ("Pre-0.4 publication behaviour changed (for example a
    /// replay screen added)"), which that unit removes: both routes now screen
    /// selection by the captured replay's reservation and republish ordinarily
    /// when a selected source still cannot finalize. Its mutant row is restated:
    /// "The routes' selection or failure behaviour diverge (a screen or a
    /// republication missing on either route)". A congruence selection under a
    /// private case limit of c + c/2 is declined at the screen: nothing is
    /// selected, one attempt is charged and the invocation publishes ordinarily.
    /// At 2c it passes the screen, its finalization cannot fit, and the
    /// invocation republishes with the case's own replay refusal as its cause
    /// (measured, both modes: c = 1,933,252).
    #[test]
    fn pre04_selection_takes_the_replay_screen() {
        use crate::source_receipt::load_state_fallback_tests::{assert_pre04_republication, unavailable, WITHHELD};
        let request = skew_pin();
        for mode in MODES {
            let fixture = request_fixture(&request);
            let c = solve(fixture.input(), exact::Limits { operations: crate::PHYSICS_SOURCE_WORK_LIMIT, ..exact::Limits::default() })
                .unwrap()
                .summary()
                .work
                .charged;
            assert_eq!(c, 1_933_252, "{mode:?}");
            let run = |limit: usize| {
                let (typed, capture) = crate::source_receipt::CapturedInvocation::parse(request.clone(), mode).unwrap();
                let mut budget = crate::SourceRecoveryBudget { per_case_limit: limit, ..Default::default() };
                let envelope = crate::run_linear_static_preview_captured(typed, mode, Some(&capture), &mut budget);
                (envelope, budget)
            };
            // The screen declines at c + c/2.
            let limit = c + c / 2;
            let (envelope, budget) = run(limit);
            assert_pre04_republication(&envelope, &request, mode);
            let message = unavailable(&envelope, "case");
            assert!(
                message.contains(&format!("stage: \"captured replay reservation\", helper_stage: Replay, error: Exact(Budget), work: WorkReport {{ charged: {c}, rejected: {c}, limit: {limit} }}")),
                "{mode:?}: {message}"
            );
            assert!(!message.contains("could not finalize"), "{mode:?}: {message}");
            assert_eq!(budget.attempts, 1);
            assert!(budget.join_withheld.is_none());
            // At 2c the screen passes; finalization's captured replay cannot fit.
            let (envelope, budget) = run(2 * c);
            assert_pre04_republication(&envelope, &request, mode);
            let cause = budget.join_withheld.clone().expect("republished");
            assert!(cause.starts_with("case case: captured source replay: "), "{mode:?}: {cause}");
            assert!(cause.contains("Exact(Budget)"), "{mode:?}: {cause}");
            let message = unavailable(&envelope, "case");
            assert!(message.contains("stage: \"invocation join withheld\""), "{mode:?}: {message}");
            assert!(message.ends_with(&format!("{WITHHELD}{cause}")), "{mode:?}: {message}");
            assert_eq!(budget.attempts, 2);
            assert!(budget.charged <= budget.invocation_limit);
        }
    }

    /// RF-15 (design §7): the producer publishes free blocks of up to 4 DOFs.
    /// The invented order-4 request (sensitive on the ordinary route) publishes
    /// `pressure-source-1` with blocks of orders 3, 2, 4 and 4, and the Rust
    /// reader accepts it with its captured invocation.
    #[test]
    fn order_four_selection_publishes_a_pressure_source_receipt() {
        use open_pipe_stress_result_export::physics_source as ps;
        let request: Value = serde_json::from_str(ORDER_FOUR_REQUEST).unwrap();
        assert_eq!(request["model"]["pressure_contract"]["shear_model"], "euler_bernoulli");
        for segment in request["model"]["pipe_segments"].as_array().unwrap() {
            assert_eq!(segment["section"]["mill_tolerance"]["value"], 0.0);
            assert_eq!(segment["section"]["corrosion_allowance"]["value"], 0.0);
        }
        for mode in MODES {
            let envelope = serde_json::to_value(crate::run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
            assert_eq!(envelope["producer"]["semantic_contract_id"], crate::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID, "{mode:?}");
            assert_eq!(envelope["numerical_quality"]["status"], "sensitive", "{mode:?}");
            let case = &envelope["source_block_recovery"]["body"]["cases"][0];
            assert_eq!(case["outcome"], "qualified", "{mode:?}");
            assert_eq!(case["source"]["level"], "complete_identified_represented_contributions");
            let orders: Vec<usize> = case["source"]["free_blocks"].as_array().unwrap().iter().map(|b| b.as_array().unwrap().len()).collect();
            assert_eq!(orders, [3, 2, 4, 4], "{mode:?}");
            let invocation = json!({"request": request, "solver_mode": mode.as_str()});
            assert_eq!(ps::validate_in(&envelope, Some(&invocation), ps::SourceContract::PressureV3), Ok(true), "{mode:?}");
        }
    }

    /// RF-17 (design §7): a publication with a congruence member declares
    /// `complete_identified_declared_member_congruence` and its normalized
    /// source names the member; a signed-permutation publication keeps
    /// `complete_identified_represented_contributions` and no such member.
    #[test]
    fn congruence_receipt_declares_declared_member_congruence() {
        use open_pipe_stress_result_export::physics_source as ps;
        let control: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
        ))
        .unwrap();
        for (request, level, members) in [
            (skew_pin(), "complete_identified_declared_member_congruence", Some(json!(["member-0"]))),
            (control, "complete_identified_represented_contributions", None),
        ] {
            for mode in MODES {
                let _ = crate::source_receipt::take_observed_congruence_members();
                let envelope = serde_json::to_value(crate::run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
                let label = format!("{} {mode:?}", request["model"]["project"]["id"]);
                assert_eq!(envelope["producer"]["semantic_contract_id"], crate::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID, "{label}");
                let source = &envelope["source_block_recovery"]["body"]["cases"][0]["source"];
                assert_eq!(source["level"], level, "{label}");
                let observed = crate::source_receipt::take_observed_congruence_members();
                assert!(!observed.is_empty() && observed.iter().all(|o| *o == members), "{label}: {observed:?}");
                let invocation = json!({"request": request, "solver_mode": mode.as_str()});
                assert_eq!(ps::validate_in(&envelope, Some(&invocation), ps::SourceContract::PressureV3), Ok(true), "{label}");
            }
        }
    }
}

#[cfg(test)]
#[path = "source_recovery/j7_t4_consequences.rs"]
mod j7_t4_consequences;

#[cfg(test)]
#[path = "source_recovery/j7_rf_census.rs"]
pub(crate) mod j7_rf_census;
