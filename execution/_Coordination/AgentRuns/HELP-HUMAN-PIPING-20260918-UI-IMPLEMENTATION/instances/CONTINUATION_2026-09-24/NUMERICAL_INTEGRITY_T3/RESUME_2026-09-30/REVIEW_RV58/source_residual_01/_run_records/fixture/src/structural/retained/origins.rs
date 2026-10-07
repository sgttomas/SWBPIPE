//! Native execution custody, not a serialized receipt or memory admission permit.
//!
//! A context owns the one meter and issues all identities. Capacity is a finite
//! representation reservation derived from submitted call cardinalities. Future
//! P1/M1 admission must wrap construction and the source/record owners before
//! execution; successful allocation alone qualifies neither memory nor a caller.
use super::adaptive::{
    self, AttemptStop, CaseLimit, CasePrep, CoreRun, ExecutionOutcome, GroupCache, InvocationMeter,
    Refusal, RetainedSolve, RunWork, StageWork,
};
use super::combine::{self, CombinationReason};
use super::source::PrimitiveSource;
use super::work::WorkTotal;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OriginError {
    CountRange(&'static str),
    Capacity,
    Allocation,
    MissingSelectedOrigin { operand: usize },
}

/// Exact maxima for a declared finite collection of calls. No default allowance.
#[derive(Debug, Clone, Copy)]
pub struct OriginCapacity {
    calls: usize,
    cases: usize,
    combinations: usize,
    operands: usize,
    runs: usize,
    builds: usize,
}
impl OriginCapacity {
    pub fn for_calls(
        case_batch_lengths: &[usize],
        combination_operand_lengths: &[usize],
    ) -> Result<Self, OriginError> {
        let mut cases = 0usize;
        for &n in case_batch_lengths {
            cases = cases
                .checked_add(n)
                .ok_or(OriginError::CountRange("case runs"))?;
        }
        let mut operands = 0usize;
        for &n in combination_operand_lengths {
            operands = operands
                .checked_add(n)
                .ok_or(OriginError::CountRange("operands"))?;
        }
        let combinations = combination_operand_lengths.len();
        let calls = case_batch_lengths
            .len()
            .checked_add(combinations)
            .ok_or(OriginError::CountRange("calls"))?;
        let runs = cases
            .checked_add(combinations)
            .ok_or(OriginError::CountRange("runs"))?;
        // Four physical links are embedded per Run; seven possible builds per Run.
        runs.checked_mul(4)
            .ok_or(OriginError::CountRange("physical records"))?;
        let builds = runs
            .checked_mul(7)
            .ok_or(OriginError::CountRange("builds"))?;
        Ok(Self {
            calls,
            cases,
            combinations,
            operands,
            runs,
            builds,
        })
    }
}

fn reserved<T>(count: usize) -> Result<Vec<T>, OriginError> {
    count
        .checked_mul(std::mem::size_of::<T>())
        .filter(|&bytes| bytes <= isize::MAX as usize)
        .ok_or(OriginError::CountRange("record allocation"))?;
    let mut out = Vec::new();
    out.try_reserve_exact(count)
        .map_err(|_| OriginError::Allocation)?;
    Ok(out)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginSlot {
    S128,
    S256,
    S512,
    S1024,
    V256,
    V512,
    V1024,
}
impl OriginSlot {
    pub const ALL: [Self; 7] = [
        Self::S128,
        Self::S256,
        Self::S512,
        Self::S1024,
        Self::V256,
        Self::V512,
        Self::V1024,
    ];
    pub(crate) fn solve(p: u32) -> Self {
        match p {
            128 => Self::S128,
            256 => Self::S256,
            512 => Self::S512,
            1024 => Self::S1024,
            _ => unreachable!("schedule precision"),
        }
    }
    pub(crate) fn verify(p: u32) -> Self {
        match p {
            256 => Self::V256,
            512 => Self::V512,
            1024 => Self::V1024,
            _ => unreachable!("verification precision"),
        }
    }
    pub(crate) fn index(self) -> usize {
        self as usize
    }
}
/// Fixed slot order is OriginSlot::ALL. These references are only issued here.
pub type SlotSnapshot = [Option<usize>; 7];
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RecordBuildLinks {
    pub shared: Option<usize>,
    pub verification_shared: Option<usize>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildPhase {
    Shared,
    VerificationShared,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildState {
    Success,
    NonbudgetFailure,
    BudgetFailure,
}
#[derive(Debug, Clone)]
pub struct BuildOrigin {
    pub id: usize,
    pub group: usize,
    pub slot: OriginSlot,
    pub call: usize,
    pub run: usize,
    pub physical_record: usize,
    pub phase: BuildPhase,
    pub state: BuildState,
    pub reason: Option<AttemptStop>,
    pub work: WorkTotal,
    pub stages: StageWork,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeOwner {
    Case(usize),
    Combination(usize),
}
#[derive(Debug, Clone)]
pub struct SourceOrigin {
    pub id: usize,
    pub owner: NativeOwner,
    /// Full existing K4SRC or K4CMB bytes, never a substituted digest.
    pub identity: Vec<u8>,
    pub stiffness: Vec<u8>,
    /// Exists only after actual combined preparation, including unavailable runs.
    pub combination_ledger: Option<Vec<u8>>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunPhase {
    InvocationEntry,
    GroupPreparation,
    SourcePreparation,
    Schedule,
}
#[derive(Debug, Clone)]
pub struct RunOrigins {
    pub id: usize,
    pub call: usize,
    pub position: usize,
    pub source: usize,
    pub owner: NativeOwner,
    pub group: Option<usize>,
    pub phase: RunPhase,
    pub cache_before: SlotSnapshot,
    pub cache_after: SlotSnapshot,
    /// Only the first physical_records entries describe actual records.
    pub records: [RecordBuildLinks; 4],
    pub physical_records: usize,
    pub work: RunWork,
}
#[derive(Debug, Clone)]
pub struct RecordedCase {
    pub outcome: ExecutionOutcome,
    pub run: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheImport {
    pub operand_index: usize,
    pub selected_run: usize,
    pub slot: OriginSlot,
    pub build: usize,
}
#[derive(Debug, Clone)]
pub enum GroupPreparation {
    Ready,
    Refused(Refusal),
}
#[derive(Debug, Clone)]
pub struct GroupOrigin {
    pub id: usize,
    pub call: usize,
    pub first_source: usize,
    pub preparation: GroupPreparation,
    /// Group membership is the RunOrigins entries referencing this id.
    pub imports: [Option<CacheImport>; 7],
    slots: SlotSnapshot,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombinationStage {
    OperandValidation,
    CombinedPreparation,
}
#[derive(Debug, Clone)]
pub enum CallResult {
    Runs,
    PreSourceRefusal {
        stage: CombinationStage,
        reason: CombinationReason,
    },
    /// Native association failure; closed serialized mapping is a later interface.
    OriginRefusal(OriginError),
}
#[derive(Debug, Clone)]
pub enum CallKind {
    CaseBatch,
    MechanicsCombination,
}
#[derive(Debug, Clone, Copy)]
pub struct RequestedOperand {
    /// None truthfully denotes a legacy/foreign solve with no local origin.
    pub source: Option<usize>,
    /// Retains even a nonfinite native factor; no finite wire value is invented.
    pub factor_bits: u64,
}
#[derive(Debug, Clone)]
pub struct CallOrigin {
    pub id: usize,
    pub kind: CallKind,
    pub owners: Vec<NativeOwner>,
    pub requested_operands: Vec<RequestedOperand>,
    pub sources: Vec<usize>,
    pub runs: Vec<usize>,
    pub invocation_before: WorkTotal,
    pub invocation_after: WorkTotal,
    pub result: CallResult,
}
#[derive(Debug, Clone)]
pub enum RecordedKernelCombination {
    PreSourceRefusal {
        call: usize,
        stage: CombinationStage,
        reason: CombinationReason,
    },
    OriginRefusal {
        call: usize,
        error: OriginError,
    },
    WithRun {
        call: usize,
        case: RecordedCase,
    },
}

// Holding the Arc prevents address reuse after a solve is dropped. Every offered
// entry constructs a fresh CasePrep once, and no offered API can rerun a prep.
// RetainedSolve::clone preserves this same immutable selected owner and snapshot.
struct SelectedOrigin {
    prep: Arc<CasePrep>,
    source: usize,
    run: usize,
    slots: SlotSnapshot,
}
pub(crate) struct OriginStore {
    calls: Vec<CallOrigin>,
    sources: Vec<SourceOrigin>,
    groups: Vec<GroupOrigin>,
    builds: Vec<BuildOrigin>,
    runs: Vec<RunOrigins>,
    selected: Vec<SelectedOrigin>,
}
/// Owns the sole actual meter; no mutable meter escape, reset, or refund API.
/// Source/selected owner ordinals are native identities. PP must bind its actual
/// request/map owners separately; they are not authored request indices here.
pub struct RecordedInvocation {
    meter: InvocationMeter,
    capacity: OriginCapacity,
    used_cases: usize,
    used_combinations: usize,
    used_operands: usize,
    store: OriginStore,
}
impl RecordedInvocation {
    pub fn new(invocation_limit: u64, capacity: OriginCapacity) -> Result<Self, OriginError> {
        Ok(Self {
            meter: InvocationMeter::new(invocation_limit),
            capacity,
            used_cases: 0,
            used_combinations: 0,
            used_operands: 0,
            store: OriginStore {
                calls: reserved(capacity.calls)?,
                sources: reserved(capacity.runs)?,
                groups: reserved(capacity.runs)?,
                builds: reserved(capacity.builds)?,
                runs: reserved(capacity.runs)?,
                selected: reserved(capacity.runs)?,
            },
        })
    }
    pub fn meter(&self) -> &InvocationMeter {
        &self.meter
    }
    pub fn calls(&self) -> &[CallOrigin] {
        &self.store.calls
    }
    pub fn sources(&self) -> &[SourceOrigin] {
        &self.store.sources
    }
    pub fn groups(&self) -> &[GroupOrigin] {
        &self.store.groups
    }
    pub fn builds(&self) -> &[BuildOrigin] {
        &self.store.builds
    }
    pub fn runs(&self) -> &[RunOrigins] {
        &self.store.runs
    }

    pub fn solve_cases(
        &mut self,
        sources: &[PrimitiveSource],
        limit: CaseLimit,
    ) -> Result<Vec<RecordedCase>, OriginError> {
        let next = self
            .used_cases
            .checked_add(sources.len())
            .ok_or(OriginError::CountRange("case ordinal"))?;
        if self.store.calls.len() >= self.capacity.calls || next > self.capacity.cases {
            return Err(OriginError::Capacity);
        }
        // Reserve per-call reference arrays before any source preparation or work.
        let mut owners = reserved(sources.len())?;
        let mut source_refs = reserved(sources.len())?;
        let run_refs = reserved(sources.len())?;
        let out = reserved(sources.len())?;
        let source_base = self.store.sources.len();
        for (position, source) in sources.iter().enumerate() {
            let owner = NativeOwner::Case(self.used_cases + position);
            let id = self.store.sources.len();
            self.store.sources.push(SourceOrigin {
                id,
                owner,
                identity: source.encoding(),
                stiffness: source.stiffness_encoding(),
                combination_ledger: None,
            });
            owners.push(owner);
            source_refs.push(id);
        }
        self.used_cases = next;
        let call = self.store.calls.len();
        let before = self.meter.checked_charged();
        self.store.calls.push(CallOrigin {
            id: call,
            kind: CallKind::CaseBatch,
            owners,
            requested_operands: Vec::new(),
            sources: source_refs,
            runs: run_refs,
            invocation_before: before,
            invocation_after: before,
            result: CallResult::Runs,
        });
        let group_base = self.store.groups.len();
        let recording = BatchRecording {
            store: &mut self.store,
            call,
            source_base,
            group_base,
        };
        let out = adaptive::solve_cases_projected(
            sources,
            limit,
            &mut self.meter,
            Some(recording),
            out,
            |core, run| RecordedCase {
                outcome: core.outcome,
                run: run.expect("recorded case has actual run"),
            },
        );
        self.store.calls[call].invocation_after = self.meter.checked_charged();
        Ok(out)
    }

    pub fn solve_combination(
        &mut self,
        operands: &[(f64, &RetainedSolve)],
        limit: CaseLimit,
    ) -> Result<RecordedKernelCombination, OriginError> {
        let next = self
            .used_operands
            .checked_add(operands.len())
            .ok_or(OriginError::CountRange("operand ordinal"))?;
        if self.store.calls.len() >= self.capacity.calls
            || self.used_combinations >= self.capacity.combinations
            || next > self.capacity.operands
        {
            return Err(OriginError::Capacity);
        }
        let mut requested = reserved(operands.len())?;
        let mut selected_indices = reserved(operands.len())?;
        let mut owners = reserved(1)?;
        let source_refs = reserved(1)?;
        let run_refs = reserved(1)?;
        for &(factor, solve) in operands {
            let selected = self
                .store
                .selected
                .iter()
                .position(|s| Arc::ptr_eq(&s.prep, &solve.prep));
            requested.push(RequestedOperand {
                source: selected.map(|i| self.store.selected[i].source),
                factor_bits: factor.to_bits(),
            });
            selected_indices.push(selected);
        }
        let owner = NativeOwner::Combination(self.used_combinations);
        owners.push(owner);
        self.used_combinations = self
            .used_combinations
            .checked_add(1)
            .ok_or(OriginError::CountRange("combination ordinal"))?;
        self.used_operands = next;
        let call = self.store.calls.len();
        let before = self.meter.checked_charged();
        self.store.calls.push(CallOrigin {
            id: call,
            kind: CallKind::MechanicsCombination,
            owners,
            requested_operands: requested,
            sources: source_refs,
            runs: run_refs,
            invocation_before: before,
            invocation_after: before,
            result: CallResult::Runs,
        });
        // Keep native validation precedence before the additional custody check.
        if let Err(reason) = combine::validate_operands(operands) {
            return Ok(self.pre_source(call, CombinationStage::OperandValidation, reason));
        }
        for (operand, (_, solve)) in operands.iter().enumerate() {
            let valid = selected_indices[operand].is_some_and(|i| {
                let selected = &self.store.selected[i];
                self.store.sources[selected.source].identity == solve.prep.identity
                    && solve.cache.matches_origins(&selected.slots)
            });
            if !valid {
                let error = OriginError::MissingSelectedOrigin { operand };
                self.store.calls[call].result = CallResult::OriginRefusal(error.clone());
                return Ok(RecordedKernelCombination::OriginRefusal { call, error });
            }
        }
        let prep = match combine::prepare_operands(operands) {
            Ok(p) => p,
            Err(reason) => {
                return Ok(self.pre_source(call, CombinationStage::CombinedPreparation, reason))
            }
        };
        let source = self.store.sources.len();
        self.store.sources.push(SourceOrigin {
            id: source,
            owner,
            identity: prep.identity.clone(),
            stiffness: prep.source.stiffness_encoding(),
            combination_ledger: Some(prep.ledger.encoding()),
        });
        self.store.calls[call].sources.push(source);
        let mut imports = [None; 7];
        let mut slots = [None; 7];
        for (operand_index, selected) in selected_indices.iter().enumerate() {
            let selected = &self.store.selected[selected.expect("association checked")];
            for slot in OriginSlot::ALL {
                let i = slot.index();
                if slots[i].is_none() {
                    if let Some(build) = selected.slots[i] {
                        slots[i] = Some(build);
                        imports[i] = Some(CacheImport {
                            operand_index,
                            selected_run: selected.run,
                            slot,
                            build,
                        });
                    }
                }
            }
        }
        let group = self.store.groups.len();
        self.store.groups.push(GroupOrigin {
            id: group,
            call,
            first_source: source,
            preparation: GroupPreparation::Ready,
            imports,
            slots,
        });
        let mut cache = GroupCache::merged(operands.iter().map(|o| &o.1.cache));
        assert!(
            cache.matches_origins(&slots),
            "actual merged slot inventory"
        );
        let run_id = self.store.runs.len();
        let mut trace = RunTrace::new(&mut self.store.builds, call, run_id, group, slots);
        let run = adaptive::run_core_with_origins(
            prep,
            operands[0].1.group.clone(),
            &mut cache,
            limit,
            &mut self.meter,
            Some(&mut trace),
        );
        let capture = trace.finish();
        let id = self.store.finish_run(
            call,
            0,
            source,
            Some(group),
            RunPhase::Schedule,
            &run,
            capture,
        );
        self.store.calls[call].invocation_after = self.meter.checked_charged();
        Ok(RecordedKernelCombination::WithRun {
            call,
            case: RecordedCase {
                outcome: run.outcome,
                run: id,
            },
        })
    }
    fn pre_source(
        &mut self,
        call: usize,
        stage: CombinationStage,
        reason: CombinationReason,
    ) -> RecordedKernelCombination {
        self.store.calls[call].result = CallResult::PreSourceRefusal {
            stage,
            reason: reason.clone(),
        };
        self.store.calls[call].invocation_after = self.meter.checked_charged();
        RecordedKernelCombination::PreSourceRefusal {
            call,
            stage,
            reason,
        }
    }
}

pub(crate) struct RunCapture {
    before: SlotSnapshot,
    after: SlotSnapshot,
    links: [RecordBuildLinks; 4],
}
impl RunCapture {
    fn idle(slots: SlotSnapshot) -> Self {
        Self {
            before: slots,
            after: slots,
            links: [RecordBuildLinks::default(); 4],
        }
    }
}
/// Only the private schedule can request these stamps. No external BuildRef is accepted.
pub(crate) struct RunTrace<'a> {
    builds: &'a mut Vec<BuildOrigin>,
    call: usize,
    run: usize,
    group: usize,
    capture: RunCapture,
}
impl<'a> RunTrace<'a> {
    fn new(
        builds: &'a mut Vec<BuildOrigin>,
        call: usize,
        run: usize,
        group: usize,
        slots: SlotSnapshot,
    ) -> Self {
        Self {
            builds,
            call,
            run,
            group,
            capture: RunCapture::idle(slots),
        }
    }
    pub(crate) fn requested(
        &mut self,
        slot: OriginSlot,
        physical_record: usize,
        built: bool,
        reason: Option<&AttemptStop>,
        work: WorkTotal,
        stages: &StageWork,
    ) {
        let index = slot.index();
        let phase = if index < 4 {
            BuildPhase::Shared
        } else {
            BuildPhase::VerificationShared
        };
        let id = if built {
            let id = self.builds.len();
            // The call preflight reserved 7 per possible run; this is an internal
            // schedule invariant, not a fallback allocation or memory permit.
            assert!(id < self.builds.capacity(), "reserved build inventory");
            let state = match reason {
                None => BuildState::Success,
                Some(AttemptStop::Budget(_)) => BuildState::BudgetFailure,
                Some(_) => BuildState::NonbudgetFailure,
            };
            self.builds.push(BuildOrigin {
                id,
                group: self.group,
                slot,
                call: self.call,
                run: self.run,
                physical_record,
                phase,
                state,
                reason: reason.cloned(),
                work,
                stages: stages.clone(),
            });
            if state != BuildState::BudgetFailure {
                self.capture.after[index] = Some(id);
            }
            id
        } else {
            self.capture.after[index].expect("resident slot has an actual recorded build")
        };
        let links = &mut self.capture.links[physical_record];
        match phase {
            BuildPhase::Shared => links.shared = Some(id),
            BuildPhase::VerificationShared => links.verification_shared = Some(id),
        }
    }
    pub(crate) fn finish(self) -> RunCapture {
        self.capture
    }
}
impl OriginStore {
    fn finish_run(
        &mut self,
        call: usize,
        position: usize,
        source: usize,
        group: Option<usize>,
        phase: RunPhase,
        run: &CoreRun,
        capture: RunCapture,
    ) -> usize {
        let id = self.runs.len();
        let physical_records = match &run.outcome {
            ExecutionOutcome::Selected(s) => s.evidence().attempts.len(),
            ExecutionOutcome::Refused { attempts, .. }
            | ExecutionOutcome::Unresolved { attempts, .. } => attempts.len(),
        };
        assert!(physical_records <= 4, "four-record schedule");
        if let Some(group) = group {
            self.groups[group].slots = capture.after;
        }
        if let ExecutionOutcome::Selected(solve) = &run.outcome {
            assert_eq!(
                self.sources[source].identity, solve.prep.identity,
                "selected source is the actual registered full byte identity"
            );
            assert!(
                solve.cache.matches_origins(&capture.after),
                "selected finish inventory"
            );
            self.selected.push(SelectedOrigin {
                prep: solve.prep.clone(),
                source,
                run: id,
                slots: capture.after,
            });
        }
        self.runs.push(RunOrigins {
            id,
            call,
            position,
            source,
            owner: self.sources[source].owner,
            group,
            phase,
            cache_before: capture.before,
            cache_after: capture.after,
            records: capture.links,
            physical_records,
            work: run.work,
        });
        self.calls[call].runs.push(id);
        id
    }
}

pub(crate) struct BatchRecording<'a> {
    store: &'a mut OriginStore,
    call: usize,
    source_base: usize,
    group_base: usize,
}
impl BatchRecording<'_> {
    pub(crate) fn group(&mut self, position: usize, refusal: Option<&Refusal>) {
        let id = self.store.groups.len();
        self.store.groups.push(GroupOrigin {
            id,
            call: self.call,
            first_source: self.source_base + position,
            preparation: refusal.map_or(GroupPreparation::Ready, |r| {
                GroupPreparation::Refused(r.clone())
            }),
            imports: [None; 7],
            slots: [None; 7],
        });
    }
    pub(crate) fn trace(&mut self, group_index: usize) -> RunTrace<'_> {
        let group = self.group_base + group_index;
        let slots = self.store.groups[group].slots;
        RunTrace::new(
            &mut self.store.builds,
            self.call,
            self.store.runs.len(),
            group,
            slots,
        )
    }
    pub(crate) fn finish(
        &mut self,
        position: usize,
        group_index: Option<usize>,
        phase: RunPhase,
        run: &CoreRun,
        capture: Option<RunCapture>,
    ) -> usize {
        let group = group_index.map(|i| self.group_base + i);
        let capture = capture.unwrap_or_else(|| {
            RunCapture::idle(group.map_or([None; 7], |g| self.store.groups[g].slots))
        });
        self.store.finish_run(
            self.call,
            position,
            self.source_base + position,
            group,
            phase,
            run,
            capture,
        )
    }
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/origins_tests.rs"]
mod tests;
