//! Private same-run diagnostic adapter. Public entrypoints never construct it.
#![allow(dead_code)]
use super::*;
use super::retained_receipt as trace;
use open_pipe_stress_frame_kernel::structural::retained_api as k;

#[derive(Debug)]
pub(super) enum MaterialSelector {
    Point(String),
    Temperature(u64),
}
#[derive(Debug)]
pub(super) struct MaterialSelection {
    pub case: String,
    pub material: String,
    pub material_ordinal: usize,
    pub selected: k::ProductMaterial,
    pub alpha: Option<f64>,
    pub selector: MaterialSelector,
    pub point_ids: (String, Option<String>),
    pub source_alpha: [Option<f64>; 2],
}
#[derive(Debug)]
pub(super) struct BasisRecord {
    pub case: String,
    pub selector: MaterialSelector,
    pub text: String,
}
#[derive(Debug)]
pub(super) struct TermIdentity {
    pub original: usize,
    pub occurrence: usize,
    pub canonical: usize,
    pub id: String,
    pub dof: usize,
    pub bits: u64,
}
#[derive(Debug)]
pub(super) struct MemberIdentity {
    pub model: usize,
    pub built: usize,
    pub id: String,
    pub material: usize,
    pub nodes: [usize; 2],
}
#[derive(Debug)]
pub(super) struct SpringIdentity {
    pub boundary: usize,
    pub support: usize,
    pub dof: usize,
    pub bits: u64,
}
#[derive(Debug)]
pub(super) struct ObservationValue {
    pub value_bits: u64,
    pub basis: String,
}
#[derive(Debug)]
pub(super) struct SolverObservations {
    pub case: String,
    pub mode: PreviewSolverMode,
    pub mode_row: ObservationValue,
    /// Independent producing-prefix fact, never inferred from the owned snapshot.
    pub parity_produced: bool,
    pub parity: Option<ObservationValue>,
}
/// B3b-P (B3-D P-1): the invocation's W1 route, decided once (`lib.rs` `w1_route`, in
/// `permitted_run` and the private driver) from the admitted model's namespace branch: the
/// preview successor over preview-physics-1, or the exact successor (`physics-retained-1`,
/// DEF-E) over physics-1 (schema 0.3.0, `exact_straight_pressure_v2`, explicitly empty
/// pressure regions). The route-specific steps (capture, observables, maxima, overlay and
/// the wire's identity) each call their own named function (I95's ruling 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum W1Route {
    #[default]
    Preview,
    Exact,
}
impl W1Route {
    /// The ordinary base publication's semantic contract on this route.
    pub(super) fn base_contract(self) -> &'static str {
        match self {
            Self::Preview => preview_physics::ID,
            Self::Exact => PHYSICS_SEMANTIC_CONTRACT_ID,
        }
    }
    /// The base publication's `contract_evidence` member that holds one entry per case.
    pub(super) fn evidence_cases(self) -> &'static str {
        match self {
            Self::Preview => "preview_cases",
            Self::Exact => "exact_cases",
        }
    }
}
/// B3b-P (B3-D §1.2, P-3): the exact route's constitutive basis of every used material.
pub(super) const EXACT_CONSTITUTIVE_BASIS: &str = "homogeneous_isotropic_E_nu_v1";
#[derive(Debug, Clone)]
pub(super) enum CaptureError {
    Association(String),
    CountRange(&'static str),
    Storage(&'static str),
    Accounting(AdapterFault),
    Source(k::SourceError),
    Origin(k::OriginError),
    NativeUnavailable,
    PreparedArithmetic(OperationalError),
    PreparedProof(k::ProductFailure),
    PreparedAttemptConsumed,
}
#[derive(Default)]
pub(super) struct ProductCapture {
    prepared_probe: bool,
    prepared_one_case_seen: bool,
    pub prepared_late_calls: usize,
    prepared_source_permit: bool,
    pub source_capture_entries: usize,
    #[cfg(test)]
    pub trace_fault:Option<trace::TraceFault>,
    pub prepared_capacity_bytes: [usize; 16],
    pub adapter: AdapterWork,
    pub invocation_calls: usize,
    pub invocation_mode: Option<PreviewSolverMode>,
    /// RV82-S2 (I61 U1 grant 2): the actual invocation's identity, its
    /// `source_blocks_invocation_v1` digest, so a serializer binds structurally to
    /// the invocation this capture observed (not only to its mode). Untracked by
    /// the adapter arrays (receipt bytes unchanged; counted by U4).
    pub invocation_digest: Option<String>,
    pub observation_calls: usize,
    pub observations: Option<SolverObservations>,
    /// Successfully allocated case/mode-basis/parity-basis capacities, even on later failure.
    pub observation_capacity_bytes: [usize; 3],
    #[cfg(test)]
    pub observation_storage_failure: Option<usize>,
    pub normalized_calls: usize,
    pub case_calls: usize,
    pub final_calls: usize,
    pub request_materials: bool,
    pub nodes: Vec<(String, [f64; 3])>,
    pub materials: Vec<(String, f64, f64)>,
    pub selections: Vec<MaterialSelection>,
    pub basis_record: Option<BasisRecord>,
    pub basis_record_calls: usize,
    /// Set from the actual source case, independently of successful text capture.
    pub basis_expected: bool,
    pub members: Vec<MemberIdentity>,
    pub terms: Vec<TermIdentity>,
    pub supports: Vec<(String, usize)>,
    pub spring_map: Vec<SpringIdentity>,
    pub support_fixed: Vec<[bool; 6]>,
    /// Actual capacity bytes: built-used, spring-used, rigid-owned, spring-map,
    /// source springs, group child lists (sum), fixed-component maps.
    pub support_capacity_bytes: [usize; 7],
    pub facts: Vec<k::ProductMemberFacts>,
    pub source: Option<k::PrimitiveSource>,
    pub case_id: String,
    pub error: Option<CaptureError>,
    pub verdicts: Vec<k::ProductRowVerdict>,
    pub numeric_failure: Option<k::ProductFailure>,
    pub numeric_pass: bool,
    pub observable_error: Option<CaptureError>,
    /// B1 SP (DESIGN_v2 T-8): this case's Run in the invocation's one call (per case).
    pub native: Option<k::RecordedCase>,
    /// B1 SP (DESIGN_v2 T-8): the invocation's one `RecordedInvocation` (its meter, call,
    /// sources, groups, builds and Runs), shared by the requested cases' Runs.
    pub native_invocation: Option<k::RecordedInvocation>,
    /// B1 SP (DESIGN_v2 T-6, T-9): this case's block of the envelope's rows, bound at custody
    /// when several cases are requested (per case; unused at c = 1, whose case owns every row).
    pub scope_rows: std::ops::Range<usize>,
    pub operational: Vec<OperationalSpent>,
    pub g5a_work: ScalarWork,
    pub g5a_error: Option<G5aFailure>,
    pub summary_coverage: Vec<k::ProductSummaryCoverage>,
    pub source_correction_calls: Option<k::WorkTotal>,
    pub work: String,
    pub capacities: Vec<(&'static str, usize)>,
    /// G-b/G-l (I61 U1): each solved case's actual ordinary attempt, captured
    /// typed at its sites in `solve_load_case_observed` (C2 §5; D39). Only an
    /// installed observer records it; the observer=None route is unchanged.
    pub ordinary: Vec<OrdinarySeed>,
    /// U3: the capture permit the facade moved into this observer (G-B, then G-C),
    /// and G-B's refusal if the late gate refused (the late capture is then skipped).
    permit: Option<super::retained_memory::CapturePermit>,
    late_refusal: Option<super::retained_memory::PhaseRefusal>,
    /// B1 seam (PLAN_v2 §2.1; RV107 SF-4): the running total of the requested cases'
    /// primitive loads at the late capture, added immediately before G-B, so that G-B at
    /// case k sees the sum over cases 0..=k. It records no adapter event (RV107 A1-N-2):
    /// the adapter's counts are receipt bytes. It saturates at `usize::MAX`, which G-B refuses
    /// (B1 SA's CaseLoadsTotal; RR ruling 2 on I89's SA).
    pub late_loads_total: usize,
    /// B1 SP (DESIGN_v2 T-2): the earlier requested cases' per-case capture, in request
    /// order. The capture's per-case fields hold the case being captured (or attempted); a
    /// later case's early hook parks them here. Empty at c = 1, where nothing is parked.
    parked: Vec<CaseSlot>,
    /// B3b-P (P-1): the invocation's route, fixed at construction (Preview by default).
    route: W1Route,
    /// B3b-P (P-3): on the exact route, each normalized material's Poisson's ratio, in
    /// `materials` order: `Some` for a material a member uses (its derived Ĝ checked), `None`
    /// otherwise. Allocated through the adapter on the exact route only; empty on the preview
    /// route (no event).
    pub material_nu: Vec<Option<f64>>,
    /// B2-P (B2-C §2.2; REVISION_01 S-1): the model's combinations, in authored order, captured
    /// at normalization through the adapter when the model has one (z = 0: empty, no event).
    pub combinations: Vec<CapturedCombination>,
    /// B2-P (T-6′; REVISION_01 S-2): the end of the case rows (the first combination row), and
    /// each combination's contiguous run of rows (a mechanics combination's freeze scope; empty
    /// for subtraction and range, whose rows bind by id), bound at custody when z ≥ 1.
    pub case_rows_end: usize,
    pub combination_rows: Vec<std::ops::Range<usize>>,
}
/// B2-P (C-1): whether a mechanics combination's terms name distinct load cases.
pub(super) fn distinct_terms(terms: &[(usize, f64)]) -> bool {
    terms.iter().enumerate().all(|(i, (case, _))| terms[..i].iter().all(|(other, _)| other != case))
}
/// B2-P: the T0R mechanics gates' codes (`preview_physics.rs` `gate_reason`; SCHEMA's
/// `base_withheld` reasons), in SCHEMA's order.
pub(super) const COMBINATION_GATE_CODES: [&str; 3] =
    ["NONLINEAR_COMBINATION_REQUIRES_SOLVE", "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE", "COMBINATION_MODULUS_BASIS_MIXED"];
/// B2-P (T-2′): one model combination as W1 captured it: its id, whether its basis is
/// `mechanics`, and a mechanics combination's terms as (load-case request index, factor), in
/// authored order (repeats kept).
#[derive(Debug, Clone, Default)]
pub(super) struct CapturedCombination {
    pub id: String,
    pub mechanics: bool,
    pub terms: Vec<(usize, f64)>,
}
/// B1 SP (DESIGN_v2 T-2): declares `CaseSlot`, the per-case part of `ProductCapture`, and the
/// swap that moves it in and out of the capture's own fields. Every per-case field of
/// `ProductCapture` is listed here; the other fields are the invocation's (one owner: the
/// adapter and its capacity records, the invocation, normalization, the seeds, the permit).
macro_rules! per_case_capture {
    ($($field:ident : $ty:ty),* $(,)?) => {
        /// One requested case's capture: its scope and custody counters, its solver
        /// observations, its late old-source capture with its facts and operational records,
        /// and its attempt's later state.
        #[derive(Default)]
        pub(super) struct CaseSlot { $(pub $field: $ty),* }
        impl ProductCapture {
            /// Exchange the capture's per-case fields with `slot`'s. Moves only: no event.
            fn swap_case(&mut self, slot: &mut CaseSlot) { $(std::mem::swap(&mut self.$field, &mut slot.$field);)* }
        }
    };
}
per_case_capture! {
    prepared_one_case_seen: bool,
    prepared_late_calls: usize,
    prepared_source_permit: bool,
    source_capture_entries: usize,
    observation_calls: usize,
    observations: Option<SolverObservations>,
    case_calls: usize,
    members: Vec<MemberIdentity>,
    terms: Vec<TermIdentity>,
    supports: Vec<(String, usize)>,
    spring_map: Vec<SpringIdentity>,
    support_fixed: Vec<[bool; 6]>,
    facts: Vec<k::ProductMemberFacts>,
    source: Option<k::PrimitiveSource>,
    case_id: String,
    error: Option<CaptureError>,
    verdicts: Vec<k::ProductRowVerdict>,
    numeric_failure: Option<k::ProductFailure>,
    numeric_pass: bool,
    observable_error: Option<CaptureError>,
    native: Option<k::RecordedCase>,
    scope_rows: std::ops::Range<usize>,
    operational: Vec<OperationalSpent>,
    g5a_work: ScalarWork,
    g5a_error: Option<G5aFailure>,
    summary_coverage: Vec<k::ProductSummaryCoverage>,
    source_correction_calls: Option<k::WorkTotal>,
    work: String,
    capacities: Vec<(&'static str, usize)>,
}
impl ProductCapture {
    /// B1 SP (T-2): the number of requested cases this capture has seen: the parked ones,
    /// plus the case in the capture's own fields once its early hook has run.
    pub(super) fn cases_seen(&self) -> usize { self.parked.len() + usize::from(self.prepared_one_case_seen) }
    /// B1 SP (T-2): the parked cases' slots, in request order (case `i` at index `i`). The
    /// last case seen is in the capture's own fields, not here.
    pub(super) fn parked_cases(&self) -> &[CaseSlot] { &self.parked }
    /// B1 SP (T-2): run `f` with request case `index` in the capture's own fields, then put
    /// every case back. The case already there (the last one seen) needs no move, so at
    /// c = 1 this is just `f(self)`.
    pub(super) fn with_case<R>(&mut self, index: usize, f: impl FnOnce(&mut Self) -> R) -> R {
        if index == self.parked.len() {
            return f(self);
        }
        let mut slot = std::mem::take(&mut self.parked[index]);
        self.swap_case(&mut slot);
        let result = f(self);
        self.swap_case(&mut slot);
        self.parked[index] = slot;
        result
    }
    /// B1 SP (T-8): the invocation and this case's Run, when the call recorded one.
    pub(super) fn native_pair(&self) -> Option<(&k::RecordedInvocation, &k::RecordedCase)> {
        self.native_invocation.as_ref().zip(self.native.as_ref())
    }
    /// Test access: move the invocation and this case's Run out.
    #[cfg(test)]
    pub(super) fn take_native(&mut self) -> Option<(k::RecordedInvocation, k::RecordedCase)> {
        self.native_invocation.take().zip(self.native.take())
    }
    /// B1 SP (T-8): request case `index`'s Run and prepared source, read where its slot is
    /// (parked, or in the capture's own fields as the last case seen). No move.
    fn case_native(&self, index: usize) -> Option<&k::RecordedCase> {
        match index.cmp(&self.parked.len()) {
            std::cmp::Ordering::Less => self.parked[index].native.as_ref(),
            std::cmp::Ordering::Equal => self.native.as_ref(),
            std::cmp::Ordering::Greater => None,
        }
    }
    fn case_prepared_source(&self, index: usize) -> Option<&k::PrimitiveSource> {
        match index.cmp(&self.parked.len()) {
            std::cmp::Ordering::Less => self.parked[index].source.as_ref(),
            std::cmp::Ordering::Equal => self.source.as_ref(),
            std::cmp::Ordering::Greater => None,
        }
    }
    /// B1 SP (T-9, T-11): request case `index`'s scope of the envelope. One requested case owns
    /// every row and the one evidence case (`CaseScope::WHOLE`, the one-case scope); with
    /// several, its block bound at custody, its own evidence case, and case-qualified ids after
    /// the first (lib.rs `qualified_load_case_result_id`).
    pub(super) fn case_scope(&self, index: usize) -> CaseScope {
        let cases = self.cases_seen();
        // B2-P (T-9′): with a combination, the one case's scope is its own block, not WHOLE.
        if cases == 1 && self.combinations.is_empty() {
            return CaseScope::whole(self.route);
        }
        let rows = match index.cmp(&self.parked.len()) {
            std::cmp::Ordering::Less => self.parked[index].scope_rows.clone(),
            std::cmp::Ordering::Equal => self.scope_rows.clone(),
            std::cmp::Ordering::Greater => 0..0,
        };
        CaseScope { rows: Some(rows), evidence: index, cases, qualified: index > 0, route: self.route }
    }
    /// B3b-P (P-1): the invocation's W1 route.
    pub(super) fn route(&self) -> W1Route { self.route }
    /// B1 SP (T-2): a later requested case's early hook parks the case in the capture's
    /// fields. The first park reserves room for every earlier case, through the adapter
    /// (one allocation, its capacity recorded); each park is one map write. Never at c = 1.
    fn park_case(&mut self, requested: usize) -> Result<(), CaptureError> {
        if self.parked.capacity() == 0 {
            self.parked = self.adapter.reserve(requested.checked_sub(1).ok_or(CaptureError::CountRange("parked cases"))?)?;
        }
        if self.parked.len() == self.parked.capacity() {
            return Err("parked case capacity".into());
        }
        self.capture_entry(AdapterEvent::MapWrite)?;
        let mut slot = CaseSlot::default();
        self.swap_case(&mut slot);
        self.parked.push(slot);
        Ok(())
    }
}
/// G-b/G-l (I61 U1): one case's ordinary attempt as the route actually ran it.
/// Diagnostic references are the ids of the diagnostics actually pushed; no
/// diagnostic text is read back. Payloads are owned copies made only while an
/// observer is installed (memory: counted by U4, not by the adapter arrays).
#[derive(Debug, Clone)]
pub(super) struct OrdinarySeed {
    pub case: String,
    /// None until the initial attempt's outcome is known (a case that returns
    /// before its solve keeps None: its not-attempted cause is not captured).
    pub initial: Option<InitialSeed>,
    pub w2: W2Seed,
    pub load_row_finding: Option<FindingSeed>,
    pub d5_diagnostic_ref: Option<String>,
    /// R-b' (`amend_integrity_report`) demoted the published verdict after the
    /// report: no wire member carries that finding, so the serializer declines.
    pub recovery_demoted: bool,
    pub legacy: Option<LegacySeed>,
}
/// C2 §5 `initial`, before any W2 consumption of the attempt.
#[derive(Debug, Clone)]
pub(super) enum InitialSeed {
    /// The actual successful ordinary solve: its integrity diagnostic as published
    /// (code after load-row/D5 demotion; R-b' tracked separately).
    Report { code: String, report_diagnostic_ref: String },
    /// An actual attempted solve failure; the reference is the diagnostic emitted
    /// for it before any W2, else None.
    StructuralFailure { error: StructuralError, diagnostic_ref: Option<String> },
    /// A deferred unformed basis (F1b): its formation `NumericalRange`.
    FormationFailure { error: FrameKernelError },
}
/// C2 §5 `w2`.
#[derive(Debug, Clone, Default)]
pub(super) enum W2Seed {
    #[default]
    NotTriggered,
    Published { trigger: RangeTrigger, force_scale_exponent: i32, report_diagnostic_ref: Option<String> },
    Failed { trigger: RangeTrigger, failure: ForceScalingFailure, diagnostic_ref: String },
}
/// The S11-G load-row finding and the diagnostic that discloses it, if any.
#[derive(Debug, Clone)]
pub(super) struct FindingSeed {
    pub sentence: String,
    pub fired: Vec<String>,
    pub diagnostic_ref: Option<String>,
}
/// G-l with D39 (ROOT, 2026-10-04): the legacy source route's actual branch.
#[derive(Debug, Clone)]
pub(super) enum LegacySeed {
    /// `!source_eligible`.
    NotEligible,
    /// `source_eligible && !needs_source_recovery`.
    NotRequired,
    /// A formation-guard or range-formation decline (no attempt; WorkReport 0/0/0).
    DeclinedWithoutAttempt { work: LegacyWork, diagnostic_ref: String },
    /// An actual `solve_ordinary` attempt that did not select.
    Unavailable { work: LegacyWork, diagnostic_ref: String },
    /// `Ok(recovery)`: exact-block selected; coexistence bypass (D-15), no successor.
    ExactSelected,
}
/// The typed `RecoveryFailure` stage and its actual `WorkReport`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LegacyWork {
    pub stage: &'static str,
    pub helper_stage: open_pipe_stress_frame_kernel::structural::exact_boundary::functionals::AttemptStage,
    pub charged: usize,
    pub rejected: usize,
    pub limit: usize,
}
impl ProductCapture {
    fn ordinary_seed(&mut self, case: &str) -> &mut OrdinarySeed {
        if self.ordinary.last().is_none_or(|seed| seed.case != case) {
            self.ordinary.push(OrdinarySeed {
                case: case.to_owned(), initial: None, w2: W2Seed::NotTriggered, load_row_finding: None,
                d5_diagnostic_ref: None, recovery_demoted: false, legacy: None,
            });
        }
        let last = self.ordinary.len() - 1;
        &mut self.ordinary[last]
    }
    /// G-b: the S11-G load-row finding, before the solve (its disclosure, if any, at the report).
    pub(super) fn ordinary_finding(&mut self, case: &str, finding: Option<&formation_guard::FormationFinding>) {
        let seed = self.ordinary_seed(case);
        seed.load_row_finding = finding.map(|f| FindingSeed { sentence: f.sentence.clone(), fired: f.fired.clone(), diagnostic_ref: None });
    }
    /// G-b: the initial attempt's failure, before W2 consumes it.
    pub(super) fn ordinary_initial_failure(&mut self, case: &str, failure: &OrdinaryFailure) {
        self.ordinary_seed(case).initial = Some(match failure {
            OrdinaryFailure::Structural(error) => InitialSeed::StructuralFailure { error: error.clone(), diagnostic_ref: None },
            OrdinaryFailure::Formation(error) => InitialSeed::FormationFailure { error: error.clone() },
        });
    }
    /// G-b: the initial structural failure's own integrity diagnostic (no W2).
    pub(super) fn ordinary_failure_diagnostic(&mut self, case: &str, diagnostic_ref: &str) {
        if let Some(InitialSeed::StructuralFailure { diagnostic_ref: slot, .. }) = &mut self.ordinary_seed(case).initial {
            *slot = Some(diagnostic_ref.to_owned());
        }
    }
    /// G-l: the typed `RecoveryFailure` at the attempt's `Err` arm (D39).
    pub(super) fn ordinary_legacy_failure(&mut self, case: &str, attempted: bool,
        failure: &source_recovery::RecoveryFailure, diagnostic_ref: &str) {
        let work = LegacyWork { stage: failure.stage, helper_stage: failure.helper_stage, charged: failure.work.charged,
            rejected: failure.work.rejected, limit: failure.work.limit };
        let diagnostic_ref = diagnostic_ref.to_owned();
        self.ordinary_seed(case).legacy = Some(if attempted {
            LegacySeed::Unavailable { work, diagnostic_ref }
        } else {
            LegacySeed::DeclinedWithoutAttempt { work, diagnostic_ref }
        });
    }
    /// G-l: the branches without a `RecoveryFailure` (D39 rows 1, 2 and 5).
    pub(super) fn ordinary_legacy_route(&mut self, case: &str, source_eligible: bool, needs_source_recovery: bool, exact_selected: bool) {
        let seed = self.ordinary_seed(case);
        if !source_eligible {
            seed.legacy = Some(LegacySeed::NotEligible);
        } else if !needs_source_recovery {
            seed.legacy = Some(LegacySeed::NotRequired);
        } else if exact_selected {
            seed.legacy = Some(LegacySeed::ExactSelected);
        }
    }
    /// G-b: W2 published at b != 0 (its report reference is set at the report).
    pub(super) fn ordinary_w2_published(&mut self, case: &str, trigger: &RangeTrigger, force_scale_exponent: i32) {
        self.ordinary_seed(case).w2 = W2Seed::Published { trigger: trigger.clone(), force_scale_exponent, report_diagnostic_ref: None };
    }
    /// G-b: W2's actual failure and the diagnostic it emitted, before the early return.
    pub(super) fn ordinary_w2_failed(&mut self, case: &str, trigger: &RangeTrigger, failure: &ForceScalingFailure, diagnostic_ref: &str) {
        self.ordinary_seed(case).w2 = W2Seed::Failed { trigger: trigger.clone(), failure: failure.clone(), diagnostic_ref: diagnostic_ref.to_owned() };
    }
    /// G-b: the integrity report actually published for the case's solve.
    pub(super) fn ordinary_report(&mut self, case: &str, record: &Diagnostic, finding_disclosed: bool, d5_line: bool) {
        let seed = self.ordinary_seed(case);
        match &mut seed.w2 {
            W2Seed::Published { report_diagnostic_ref, .. } => *report_diagnostic_ref = Some(record.id.clone()),
            _ if seed.initial.is_none() => {
                seed.initial = Some(InitialSeed::Report { code: record.code.clone(), report_diagnostic_ref: record.id.clone() });
            }
            _ => {}
        }
        if let (Some(finding), true) = (&mut seed.load_row_finding, finding_disclosed) {
            finding.diagnostic_ref = Some(record.id.clone());
        }
        if d5_line {
            seed.d5_diagnostic_ref = Some(record.id.clone());
        }
    }
    /// G-b: R-b' demoted the case's published integrity report after the report.
    pub(super) fn ordinary_recovery_demoted(&mut self, case: &str) {
        self.ordinary_seed(case).recovery_demoted = true;
    }
    pub(super) fn invocation(
        &mut self,
        capture: Option<&source_receipt::CapturedInvocation>,
        mode: PreviewSolverMode,
    ) {
        if self.error.is_some() { return; }
        let Some(calls) = self.invocation_calls.checked_add(1) else { self.fail_count("invocation calls"); return; };
        self.invocation_calls = calls;
        if calls != 1 { self.fail("duplicate invocation capture"); return; }
        if capture.is_none_or(|c| c.mode() != mode) {
            self.fail("missing/mismatched actual invocation capture");
            return;
        }
        if !self.adapter.enter(AdapterEvent::MapWrite, 1) { self.fail("invocation accounting"); return; }
        self.invocation_mode = Some(mode);
        self.invocation_digest = capture.map(|c| c.borrowed_digest().clone());
    }
    fn fail(&mut self, s: impl Into<String>) {
        if self.error.is_none() {
            self.error = Some(
                self.adapter
                    .fault
                    .get()
                    .map(CaptureError::Accounting)
                    .unwrap_or_else(|| CaptureError::Association(s.into())),
            );
        }
    }
    fn fail_count(&mut self, s: &'static str) {
        if self.error.is_none() {
            self.error = Some(CaptureError::CountRange(s));
        }
    }
    pub fn normalized(
        &mut self,
        model: &PreviewModel,
        materials: &[MaterialInput],
        request_materials: bool,
    ) {
        self.normalized_calls = self.normalized_calls.checked_add(1).expect("bounded calls");
        if self.normalized_calls != 1 {
            self.fail("capture reused across invocations");
            return;
        }
        if u32::try_from(model.nodes.len()).is_err()
            || model.nodes.len().checked_mul(6).is_none()
            || u32::try_from(model.pipe_segments.len())
                .ok()
                .and_then(|m| m.checked_mul(3))
                .is_none()
            || u32::try_from(model.supports.len()).is_err()
        {
            self.fail_count("source count range");
            return;
        }
        if !self.adapter.enter(AdapterEvent::LibraryBoundary, 1) {
            self.fail("normalization capture accounting");
            return;
        }
        self.request_materials = request_materials;
        // B3b-P (P-3): an exact model is in scope on the exact route only, and only there.
        // B2-P (T-2′, REVISION_01 S-1): combinations as D1.4 admits them (the predicate T-4's
        // re-check uses), and none on the exact route (B3-D §4.2).
        if pressure_runtime::is_exact(model) != (self.route == W1Route::Exact)
            || case_state::is_load_state(model)
            || !super::w1_model_combinations_admitted(model)
            || (self.route == W1Route::Exact && !model.combinations.is_empty())
            || !model.components.is_empty()
        {
            self.fail("outside private ordinary no-component/no-combination scope");
            return;
        }
        for (i, n) in model.nodes.iter().enumerate() {
            if model.nodes[..i]
                .iter()
                .any(|a| self.adapter.same(&a.id, &n.id))
            {
                self.fail("ambiguous node id");
                return;
            }
        }
        for (i, m) in materials.iter().enumerate() {
            if materials[..i]
                .iter()
                .any(|a| self.adapter.same(&a.id, &m.id))
            {
                self.fail("ambiguous material id");
                return;
            }
        }
        for (i, m) in model.pipe_segments.iter().enumerate() {
            if model.pipe_segments[..i]
                .iter()
                .any(|a| self.adapter.same(&a.id, &m.id))
            {
                self.fail("ambiguous member id");
                return;
            }
        }
        for (i, m) in model.supports.iter().enumerate() {
            if model.supports[..i]
                .iter()
                .any(|a| self.adapter.same(&a.id, &m.id))
            {
                self.fail("ambiguous support id");
                return;
            }
        }
        let captured = (|| -> Result<(), CaptureError> {
            self.nodes = self.adapter.reserve(model.nodes.len())?;
            for n in &model.nodes {
                if !self.adapter.enter(AdapterEvent::SourceVisit, 1)
                    || !self.adapter.enter(AdapterEvent::ValidationEntry, 1)
                {
                    return Err(CaptureError::Accounting(self.adapter.fault.get().unwrap()));
                }
                let id = self.adapter.copy(&n.id)?;
                if !self.adapter.enter(AdapterEvent::MapWrite, 1) {
                    return Err(CaptureError::Accounting(self.adapter.fault.get().unwrap()));
                }
                self.nodes
                    .push((id, [n.position.x, n.position.y, n.position.z]));
            }
            self.materials = self.adapter.reserve(materials.len())?;
            for m in materials {
                if !self.adapter.enter(AdapterEvent::SourceVisit, 1)
                    || !self.adapter.enter(AdapterEvent::ValidationEntry, 1)
                {
                    return Err(CaptureError::Accounting(self.adapter.fault.get().unwrap()));
                }
                let record = (
                    self.adapter.copy(&m.id)?,
                    m.elastic_modulus.value,
                    m.shear_modulus.as_ref().map_or(f64::NAN, |g| g.value),
                );
                if !self.adapter.enter(AdapterEvent::MapWrite, 1) {
                    return Err(CaptureError::Accounting(self.adapter.fault.get().unwrap()));
                }
                self.materials.push(record);
            }
            if self.route == W1Route::Exact {
                self.capture_exact_materials(model, materials)?;
            }
            if !model.combinations.is_empty() {
                self.capture_combinations(model)?;
            }
            Ok(())
        })();
        if let Err(e) = captured {
            self.error = Some(e);
        }
    }
    /// B3b-P (B3-D P-3; §1.2): on the exact route, each material's Poisson's ratio, in
    /// `materials` order. A material a member uses must carry the exact constitutive basis and a
    /// ratio, and its represented shear modulus (`resolve_base`'s Ĝ) must be a positive normal
    /// binary64 equal to RN64(E/(2·RN64(1+ν))), else a typed capture error (defensive: RV116
    /// N-1 proves the equality for every normal Ĝ). An unused material keeps `None`.
    /// B2-P (T-2′): the model's combinations, in authored order, through the adapter: each id
    /// (copied), whether its basis is `mechanics`, and a mechanics combination's terms with
    /// each term's load case resolved to its request index. A term naming no load case is a
    /// capture error (the ordinary route's validation refuses it first).
    fn capture_combinations(&mut self, model: &PreviewModel) -> Result<(), CaptureError> {
        self.combinations = self.adapter.reserve(model.combinations.len())?;
        for combination in &model.combinations {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            let id = self.adapter.copy(&combination.id)?;
            let mechanics = self.checked_same(&combination.basis, "mechanics")?;
            let mut terms = Vec::new();
            if mechanics {
                terms = self.adapter.reserve(combination.terms.len())?;
                for term in &combination.terms {
                    self.capture_entry(AdapterEvent::SourceVisit)?;
                    let mut case = None;
                    for (index, load_case) in model.load_cases.iter().enumerate() {
                        if self.checked_same(&load_case.id, &term.load_case)? {
                            case = Some(index);
                            break;
                        }
                    }
                    self.capture_entry(AdapterEvent::MapWrite)?;
                    terms.push((case.ok_or("combination term case")?, term.factor));
                }
            }
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.combinations.push(CapturedCombination { id, mechanics, terms });
        }
        Ok(())
    }
    fn capture_exact_materials(&mut self, model: &PreviewModel, materials: &[MaterialInput]) -> Result<(), CaptureError> {
        self.material_nu = self.adapter.reserve(materials.len())?;
        for m in materials {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            let mut used = false;
            for pipe in &model.pipe_segments {
                self.capture_entry(AdapterEvent::SourceVisit)?;
                if self.checked_same(&pipe.material, &m.id)? {
                    used = true;
                    break;
                }
            }
            let nu = if used { Some(self.exact_material_nu(m)?) } else { None };
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.material_nu.push(nu);
        }
        Ok(())
    }
    /// One used material's ν, with its basis and Ĝ checked (P-3).
    pub(super) fn exact_material_nu(&self, m: &MaterialInput) -> Result<f64, CaptureError> {
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        let basis = m.constitutive_basis.as_deref().ok_or("exact material constitutive basis")?;
        if !self.checked_same(basis, EXACT_CONSTITUTIVE_BASIS)? {
            return Err("exact material constitutive basis".into());
        }
        let nu = m.poisson_ratio.as_ref().ok_or("exact material Poisson ratio")?.value;
        let g = m.shear_modulus.as_ref().ok_or("exact material derived shear modulus")?.value;
        #[cfg(test)]
        let g = if crate::retained_tests_hooks::exact_capture_fault() { f64::from_bits(g.to_bits().wrapping_add(1)) } else { g };
        let derived = m.elastic_modulus.value / (2.0 * (1.0 + nu));
        if !(g.is_normal() && g > 0.0 && g.to_bits() == derived.to_bits()) {
            return Err("exact material derived shear modulus".into());
        }
        Ok(nu)
    }
    pub fn selection(
        &mut self,
        case: &PreviewLoadCase,
        materials: &[MaterialInput],
        material: &MaterialInput,
        ordinals: Option<(usize, Option<usize>)>,
        e: f64,
        g: f64,
        alpha: Option<f64>,
    ) {
        if !self.adapter.enter(AdapterEvent::SourceVisit, 1)
            || !self.adapter.enter(AdapterEvent::ValidationEntry, 1)
        {
            self.fail("selection accounting");
            return;
        }
        let Some(material_ordinal) = materials.iter().position(|m| std::ptr::eq(m, material))
        else {
            self.fail("material pointer");
            return;
        };
        let selected = match ordinals {
            Some((i, None)) => k::ProductMaterial::Point { ordinal: i, e, g },
            Some((lo, Some(hi))) => {
                let a = &material.temperature_points[lo];
                let b = &material.temperature_points[hi];
                k::ProductMaterial::Interpolated {
                    lower: lo,
                    upper: hi,
                    t_lo: a.temperature.as_ref().unwrap().value,
                    t: case.modulus_basis_temperature.as_ref().unwrap().value,
                    t_hi: b.temperature.as_ref().unwrap().value,
                    e_lo: a.elastic_modulus.as_ref().unwrap().value,
                    e_hi: b.elastic_modulus.as_ref().unwrap().value,
                    g_lo: a.shear_modulus.as_ref().unwrap().value,
                    g_hi: b.shear_modulus.as_ref().unwrap().value,
                    e_hat: e,
                    g_hat: g,
                }
            }
            None => {
                self.fail("missing selected point provenance");
                return;
            }
        };
        let provenance = (|| -> Result<_, CaptureError> {
            let (lo, hi) =
                ordinals.ok_or_else(|| CaptureError::Association("selected ordinals".into()))?;
            let point_ids = (
                self.adapter.copy(&material.temperature_points[lo].id)?,
                hi.map(|i| self.adapter.copy(&material.temperature_points[i].id))
                    .transpose()?,
            );
            let selector = match &load_case_selector(case) {
                Some(SelectorRef::Point(id)) => MaterialSelector::Point(self.adapter.copy(id)?),
                Some(SelectorRef::Temperature(bits)) => MaterialSelector::Temperature(*bits),
                None => return Err(CaptureError::Association("selected selector".into())),
            };
            let alpha_at = |i: usize| {
                material.temperature_points[i]
                    .thermal_expansion_coefficient
                    .as_ref()
                    .map(|a| a.value)
            };
            Ok((selector, point_ids, [alpha_at(lo), hi.and_then(alpha_at)]))
        })();
        let (selector, point_ids, source_alpha) = match provenance {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };
        if let Err(e) = self.adapter.push_room(&mut self.selections) {
            self.error = Some(e);
            return;
        }
        if !self.adapter.enter(AdapterEvent::MapWrite, 1)
            || !self.adapter.enter(AdapterEvent::LibraryBoundary, 2)
        {
            self.fail("selection accounting");
            return;
        }
        // // Existing String clone boundaries; their implementation cost is unqualified.
        self.selections.push(MaterialSelection {
            case: case.id.clone(),
            material: material.id.clone(),
            material_ordinal,
            selected,
            alpha,
            selector,
            point_ids,
            source_alpha,
        });
    }
    /// Observe only the actual successful aggregate resolver return, before case_source.
    /// This owned text is a presence-record warrant, never a numerical source law.
    pub fn successful_basis_record(&mut self, case: &PreviewLoadCase, text: &str) {
        if self.error.is_some() {
            return;
        }
        let result = (|| -> Result<BasisRecord, CaptureError> {
            self.adapter.enter(AdapterEvent::SourceVisit, 1);
            self.adapter.enter(AdapterEvent::ValidationEntry, 1);
            self.adapter.require()?;
            let calls = self
                .basis_record_calls
                .checked_add(1)
                .ok_or(CaptureError::CountRange("basis record calls"))?;
            self.adapter.enter(AdapterEvent::MapWrite, 1);
            self.adapter.require()?;
            self.basis_record_calls = calls;
            if calls != 1 || self.basis_record.is_some() {
                return Err("duplicate successful basis record".into());
            }
            self.validate_selected_case(case)?;
            let selector = match load_case_selector(case) {
                Some(SelectorRef::Point(id)) => MaterialSelector::Point(self.adapter.copy(id)?),
                Some(SelectorRef::Temperature(bits)) => MaterialSelector::Temperature(bits),
                None => return Err("unexpected base basis record".into()),
            };
            Ok(BasisRecord {
                case: self.adapter.copy(&case.id)?,
                selector,
                text: self.adapter.copy(text)?,
            })
        })();
        match result {
            Ok(record) => {
                if self.adapter.enter(AdapterEvent::MapWrite, 1) {
                    self.basis_record = Some(record);
                } else {
                    self.fail("basis record write accounting");
                }
            }
            Err(e) => self.error = Some(e),
        }
    }
    fn selector_matches(
        &self,
        selected: &MaterialSelector,
        case: &PreviewLoadCase,
    ) -> Result<bool, CaptureError> {
        self.adapter.enter(AdapterEvent::ValidationEntry, 1);
        self.adapter.require()?;
        let matches = match (selected, load_case_selector(case)) {
            (MaterialSelector::Point(a), Some(SelectorRef::Point(b))) => self.adapter.same(a, b),
            (MaterialSelector::Temperature(a), Some(SelectorRef::Temperature(b))) => *a == b,
            _ => false,
        };
        self.adapter.require()?;
        Ok(matches)
    }
    fn validate_selected_case(&self, case: &PreviewLoadCase) -> Result<(), CaptureError> {
        self.adapter.enter(AdapterEvent::ValidationEntry, 1);
        self.adapter.require()?;
        if load_case_selector(case).is_none() || self.selections.is_empty() {
            return Err("missing selected basis source".into());
        }
        for selection in &self.selections {
            self.adapter.enter(AdapterEvent::SourceVisit, 1);
            self.adapter.require()?;
            let same_case = self.adapter.same(&selection.case, &case.id);
            self.adapter.require()?;
            if !same_case || !self.selector_matches(&selection.selector, case)? {
                return Err("basis record source case/selector".into());
            }
        }
        Ok(())
    }
    fn basis_source_case(&mut self, case: &PreviewLoadCase) -> Result<(), CaptureError> {
        self.adapter.enter(AdapterEvent::ValidationEntry, 1);
        self.adapter.enter(AdapterEvent::MapWrite, 1);
        self.adapter.require()?;
        self.basis_expected = load_case_selector(case).is_some();
        match (self.basis_expected, &self.basis_record) {
            (false, None) if self.basis_record_calls == 0 => Ok(()),
            (true, Some(record)) if self.basis_record_calls == 1 => {
                self.validate_selected_case(case)?;
                let same_case = self.adapter.same(&record.case, &case.id);
                self.adapter.require()?;
                if !same_case || !self.selector_matches(&record.selector, case)? {
                    return Err("basis record producing case/selector".into());
                }
                Ok(())
            }
            _ => Err("basis record source presence".into()),
        }
    }
    fn validate_modulus_record(&self, row: &ResultItem) -> Result<(), CaptureError> {
        self.adapter.enter(AdapterEvent::ValidationEntry, 1);
        self.adapter.require()?;
        let record = self
            .basis_record
            .as_ref()
            .ok_or("missing successful basis record")?;
        let metadata = row.metadata.as_ref().ok_or("modulus record metadata")?;
        let basis = row.basis_ref.as_ref().ok_or("modulus record case basis")?;
        if !self.basis_expected
            || self.basis_record_calls != 1
            || row.value.to_bits() != 1.0f64.to_bits()
            || !row.source_result_refs.is_empty()
        {
            return Err("modulus record value/presence/references".into());
        }
        self.adapter.enter(AdapterEvent::LibraryBoundary, 1); // Existing fixed-id formatting; internals unqualified.
        self.adapter.require()?;
        let id = format!("result:modulus-basis:{}", stable_suffix(&self.case_id));
        let matches = self.adapter.same(&record.case, &self.case_id)
            && self.adapter.same(&row.kind, "modulus_basis_record")
            && self.adapter.same(&row.id, &id)
            && self.adapter.same(&row.unit, "record")
            && self.adapter.same(&row.entity_ref, &self.case_id)
            && self.adapter.same(&basis.ref_type, "load_case")
            && self.adapter.same(&basis.ref_id, &self.case_id)
            && self.adapter.same(&metadata.component, "material_modulus_basis")
            && self.adapter.same(&metadata.coordinate_system, "not_applicable")
            && self.adapter.same(&metadata.location, &self.case_id)
            && self.adapter.same(&metadata.sign_convention, "presence record; value 1.0 means the load case solved with the recorded user-entered property basis")
            && self.adapter.same(&metadata.basis, &record.text);
        self.adapter.require()?;
        if !matches {
            return Err("modulus record association".into());
        }
        Ok(())
    }
    fn observation_copy(&mut self, text: &str, slot: usize) -> Result<String, CaptureError> {
        std::alloc::Layout::array::<u8>(text.len())
            .map_err(|_| CaptureError::CountRange("observation layout"))?;
        let n =
            u64::try_from(text.len()).map_err(|_| CaptureError::CountRange("observation bytes"))?;
        self.capture_entry(AdapterEvent::AllocationRequest)?;
        self.adapter.enter(AdapterEvent::RequestedCopyBytes, n);
        self.adapter.require()?;
        self.capture_entry(AdapterEvent::MapWrite)?;
        self.capture_entry(AdapterEvent::LibraryBoundary)?;
        let mut value = String::new();
        #[cfg(test)]
        if self.observation_storage_failure == Some(slot) {
            return Err(CaptureError::Storage(
                "observation text (injected allocator refusal)",
            ));
        }
        value
            .try_reserve_exact(text.len())
            .map_err(|_| CaptureError::Storage("observation text"))?;
        // B1 SP (T-11's cumulative snapshot): summed over the requested cases (at c = 1, the one value).
        self.observation_capacity_bytes[slot] = self.observation_capacity_bytes[slot]
            .checked_add(value.capacity())
            .ok_or(CaptureError::CountRange("observation capacity total"))?;
        let bytes = u64::try_from(value.capacity())
            .map_err(|_| CaptureError::CountRange("observation capacity"))?;
        self.adapter.enter(AdapterEvent::RustCapacityBytes, bytes);
        self.adapter.require()?;
        self.capture_entry(AdapterEvent::LibraryBoundary)?;
        value.push_str(text);
        Ok(value)
    }
    fn observation_fields(
        &self,
        row: &ResultItem,
        case: &str,
        mode: PreviewSolverMode,
        parity: bool,
        final_row: bool,
        captured: Option<&ObservationValue>,
    ) -> Result<(), CaptureError> {
        self.observation_fields_of(row, case, mode, parity, final_row, false, captured)
    }
    /// `observation_fields`, where `qualified` marks the final row of a requested case after the
    /// first (B1 SP): the envelope qualifies its id with its case (`qualified_load_case_result_id`).
    #[allow(clippy::too_many_arguments)]
    fn observation_fields_of(
        &self,
        row: &ResultItem,
        case: &str,
        mode: PreviewSolverMode,
        parity: bool,
        final_row: bool,
        qualified: bool,
        captured: Option<&ObservationValue>,
    ) -> Result<(), CaptureError> {
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        let m = row.metadata.as_ref().ok_or("observation metadata")?;
        let (id, kind, unit, entity, component, location, sign) = if parity {
            ("result:sparse-live:dense-parity-relative-delta", "sparse_live_path_dense_parity_relative_delta",
             "unitless", "solver:sparse_direct", "sparse_live_path", "load_case",
             "unitless max absolute dense-sparse solution delta divided by max dense solution magnitude; no release threshold asserted")
        } else {
            ("result:solver-mode:linear-solve-basis", "linear_solver_mode_basis", "mode_code",
             "solver:linear_static_preview", "linear_solver_mode", case,
             "mode_code 1=sparse_interactive, 2=dense_scrutiny, 3=dense_fallback_after_sparse_failure")
        };
        let qualified_id = if qualified {
            // The envelope's existing case qualification of a row id (lib.rs).
            self.capture_entry(AdapterEvent::LibraryBoundary)?;
            Some(qualified_load_case_result_id(case, id))
        } else {
            None
        };
        let fixed = self.adapter.same(&row.id, qualified_id.as_deref().unwrap_or(id))
            && self.adapter.same(&row.kind, kind)
            && self.adapter.same(&row.unit, unit)
            && self.adapter.same(&row.entity_ref, entity)
            && self.adapter.same(&m.component, component)
            && self.adapter.same(&m.coordinate_system, "reduced_system")
            && self.adapter.same(&m.location, location);
        self.adapter.require()?;
        if !fixed || !row.source_result_refs.is_empty() {
            return Err("observation fixed fields".into());
        }
        let sign_matches = self.adapter.same(&m.sign_convention, sign);
        self.adapter.require()?;
        if !sign_matches {
            return Err(if parity {
                "observation parity sign"
            } else {
                "ordinary sparse mode sign"
            }
            .into());
        }
        if final_row {
            let basis = row.basis_ref.as_ref().ok_or("observation final basis")?;
            let same = self.adapter.same(&basis.ref_type, "load_case")
                && self.adapter.same(&basis.ref_id, case);
            self.adapter.require()?;
            if !same {
                return Err("observation final case".into());
            }
        } else if row.basis_ref.is_some() {
            return Err("observation producer basis".into());
        }
        if parity {
            if mode != PreviewSolverMode::DenseScrutiny || !row.value.is_finite() || row.value < 0.0
            {
                return Err("observation parity mode/value".into());
            }
        } else {
            let expected = match mode {
                PreviewSolverMode::SparseInteractive => 1.0f64,
                PreviewSolverMode::DenseScrutiny => 2.0f64,
            };
            if row.value.to_bits() != expected.to_bits() {
                return Err("observation mode value/fallback".into());
            }
        }
        if let Some(captured) = captured {
            let same = self.adapter.same(&m.basis, &captured.basis);
            self.adapter.require()?;
            if !same || row.value.to_bits() != captured.value_bits {
                return Err("observation captured value/text".into());
            }
        }
        Ok(())
    }
    pub(super) fn solver_observations(
        &mut self,
        case: &PreviewLoadCase,
        mode: PreviewSolverMode,
        produced_rows: &[ResultItem],
    ) {
        if self.error.is_some() {
            return;
        }
        if let Some(fault) = self.adapter.fault.get() {
            self.error = Some(CaptureError::Accounting(fault));
            return;
        }
        let result = (|| -> Result<(), CaptureError> {
            let calls = self
                .observation_calls
                .checked_add(1)
                .ok_or(CaptureError::CountRange("observation calls"))?;
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.observation_calls = calls;
            self.capture_entry(AdapterEvent::ValidationEntry)?;
            if calls != 1 || self.observations.is_some() {
                return Err("duplicate observation completion".into());
            }
            let same_case = self.adapter.same(&case.id, &self.case_id);
            self.adapter.require()?;
            if self.invocation_calls != 1
                || self.invocation_mode != Some(mode)
                || self.case_calls != 1
                || !same_case
            {
                return Err("observation invocation/case/mode".into());
            }
            let mut mode_row = None;
            let mut parity_row = None;
            for row in produced_rows {
                self.capture_entry(AdapterEvent::SourceVisit)?;
                let is_mode = self.adapter.same(&row.kind, "linear_solver_mode_basis");
                let is_parity = self
                    .adapter
                    .same(&row.kind, "sparse_live_path_dense_parity_relative_delta");
                self.adapter.require()?;
                if is_mode {
                    if mode_row.is_some() {
                        return Err("duplicate produced mode".into());
                    }
                    self.capture_entry(AdapterEvent::MapWrite)?;
                    mode_row = Some(row);
                }
                if is_parity {
                    if parity_row.is_some() {
                        return Err("duplicate produced parity".into());
                    }
                    self.capture_entry(AdapterEvent::MapWrite)?;
                    parity_row = Some(row);
                }
            }
            let mode_row = mode_row.ok_or("missing produced mode")?;
            self.observation_fields(mode_row, &case.id, mode, false, false, None)?;
            if let Some(row) = parity_row {
                self.observation_fields(row, &case.id, mode, true, false, None)?;
            }
            let parity_produced = parity_row.is_some();
            let captured_case = self.observation_copy(&case.id, 0)?;
            let captured_mode = ObservationValue {
                value_bits: mode_row.value.to_bits(),
                basis: self.observation_copy(&mode_row.metadata.as_ref().unwrap().basis, 1)?,
            };
            let parity = match parity_row {
                Some(row) => Some(ObservationValue {
                    value_bits: row.value.to_bits(),
                    basis: self.observation_copy(&row.metadata.as_ref().unwrap().basis, 2)?,
                }),
                None => None,
            };
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.observations = Some(SolverObservations {
                case: captured_case,
                mode,
                mode_row: captured_mode,
                parity_produced,
                parity,
            });
            Ok(())
        })();
        if let Err(error) = result {
            self.error = Some(error);
        }
    }
    pub(super) fn bind_observations(
        &self,
        envelope: &MechanicsEnvelope,
    ) -> Result<(), CaptureError> {
        self.bind_observations_in(&envelope.results, false)
    }
    /// `bind_observations` over one case's rows (B1 SP, T-9: its scope), where `qualified`
    /// marks a case after the first, whose row ids the envelope qualifies with the case.
    fn bind_observations_in(&self, rows: &[ResultItem], qualified: bool) -> Result<(), CaptureError> {
        self.adapter.require()?;
        if self.error.is_some() {
            return Err("prior observation capture refusal".into());
        }
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        let captured = self
            .observations
            .as_ref()
            .ok_or("missing observation completion")?;
        let same = self.adapter.same(&captured.case, &self.case_id);
        self.adapter.require()?;
        if self.invocation_calls != 1
            || self.case_calls != 1
            || self.observation_calls != 1
            || self.invocation_mode != Some(captured.mode)
            || !same
            || captured.parity_produced != captured.parity.is_some()
        {
            return Err("observation completion/presence custody".into());
        }
        let mut mode_count = 0usize;
        let mut parity_count = 0usize;
        for row in rows {
            self.capture_entry(AdapterEvent::RowVisit)?;
            let is_mode = self.adapter.same(&row.kind, "linear_solver_mode_basis");
            let is_parity = self
                .adapter
                .same(&row.kind, "sparse_live_path_dense_parity_relative_delta");
            self.adapter.require()?;
            if is_mode {
                mode_count = mode_count
                    .checked_add(1)
                    .ok_or(CaptureError::CountRange("final modes"))?;
                self.observation_fields_of(
                    row,
                    &captured.case,
                    captured.mode,
                    false,
                    true,
                    qualified,
                    Some(&captured.mode_row),
                )?;
            }
            if is_parity {
                parity_count = parity_count
                    .checked_add(1)
                    .ok_or(CaptureError::CountRange("final parity"))?;
                let snapshot = captured.parity.as_ref().ok_or("unexpected final parity")?;
                self.observation_fields_of(
                    row,
                    &captured.case,
                    captured.mode,
                    true,
                    true,
                    qualified,
                    Some(snapshot),
                )?;
            }
        }
        if mode_count != 1 || parity_count != usize::from(captured.parity_produced) {
            return Err("observation final presence".into());
        }
        Ok(())
    }
    /// B1 SP (T-2): request case `index`'s error, observations, captured id and custody
    /// counters, wherever its slot is (parked, or in the capture's own fields).
    fn case_observation_state(&self, index: usize) -> (Option<&CaptureError>, Option<&SolverObservations>, &str, usize, usize) {
        match self.parked.get(index) {
            Some(slot) => (slot.error.as_ref(), slot.observations.as_ref(), &slot.case_id, slot.case_calls, slot.observation_calls),
            None => (self.error.as_ref(), self.observations.as_ref(), &self.case_id, self.case_calls, self.observation_calls),
        }
    }
    /// B1 SP (DESIGN_v2 T-6; decision 19): every requested case's observations bound to the
    /// envelope in one pass over its rows, not one pass per case. Each case keeps
    /// `bind_observations`' checks: its completion and custody; exactly one final mode row
    /// and the parity rows its producing prefix made; each equal to its captured value and
    /// text. A mode or parity row is bound to the case whose captured id its basis names; a
    /// row that names no requested case is refused.
    pub(super) fn bind_observations_by_case(&self, envelope: &MechanicsEnvelope) -> Result<(), CaptureError> {
        self.adapter.require()?;
        let cases = self.cases_seen();
        for index in 0..cases {
            let (error, observations, case_id, case_calls, observation_calls) = self.case_observation_state(index);
            if error.is_some() {
                return Err("prior observation capture refusal".into());
            }
            self.capture_entry(AdapterEvent::ValidationEntry)?;
            let captured = observations.ok_or("missing observation completion")?;
            let same = self.adapter.same(&captured.case, case_id);
            self.adapter.require()?;
            if self.invocation_calls != 1
                || case_calls != 1
                || observation_calls != 1
                || self.invocation_mode != Some(captured.mode)
                || !same
                || captured.parity_produced != captured.parity.is_some()
            {
                return Err("observation completion/presence custody".into());
            }
        }
        // Per case: [final mode rows, final parity rows].
        let mut counts = self.adapter.reserve::<[usize; 2]>(cases)?;
        for _ in 0..cases {
            self.capture_entry(AdapterEvent::MapWrite)?;
            counts.push([0, 0]);
        }
        for row in &envelope.results {
            self.capture_entry(AdapterEvent::RowVisit)?;
            let is_mode = self.adapter.same(&row.kind, "linear_solver_mode_basis");
            let is_parity = self
                .adapter
                .same(&row.kind, "sparse_live_path_dense_parity_relative_delta");
            self.adapter.require()?;
            if !is_mode && !is_parity {
                continue;
            }
            let basis = row.basis_ref.as_ref().ok_or("observation final basis")?;
            let mut owner = None;
            for index in 0..cases {
                let captured = self.case_observation_state(index).1.ok_or("missing observation completion")?;
                let same = self.adapter.same(&basis.ref_id, &captured.case);
                self.adapter.require()?;
                if same {
                    owner = Some((index, captured));
                    break;
                }
            }
            let (index, captured) = owner.ok_or("observation final case")?;
            let slot = usize::from(is_parity);
            counts[index][slot] = counts[index][slot]
                .checked_add(1)
                .ok_or(CaptureError::CountRange("final observations"))?;
            let snapshot = if is_parity {
                captured.parity.as_ref().ok_or("unexpected final parity")?
            } else {
                &captured.mode_row
            };
            self.observation_fields_of(row, &captured.case, captured.mode, is_parity, true, index > 0, Some(snapshot))?;
        }
        for index in 0..cases {
            let captured = self.case_observation_state(index).1.ok_or("missing observation completion")?;
            if counts[index] != [1, usize::from(captured.parity_produced)] {
                return Err("observation final presence".into());
            }
        }
        Ok(())
    }
    fn capture_entry(&self, event: AdapterEvent) -> Result<(), CaptureError> {
        self.adapter.enter(event, 1);
        self.adapter.require()
    }
    pub(super) fn support_reserve<T>(
        &mut self,
        n: usize,
        slot: usize,
    ) -> Result<Vec<T>, CaptureError> {
        std::alloc::Layout::array::<T>(n)
            .map_err(|_| CaptureError::CountRange("support layout"))?;
        self.capture_entry(AdapterEvent::AllocationRequest)?;
        self.capture_entry(AdapterEvent::MapWrite)?; // capacity record entry before allocation
        let mut v = Vec::new();
        self.capture_entry(AdapterEvent::LibraryBoundary)?;
        v.try_reserve_exact(n)
            .map_err(|_| CaptureError::Storage("support vector"))?;
        let bytes = v
            .capacity()
            .checked_mul(std::mem::size_of::<T>())
            .ok_or(CaptureError::CountRange("support capacity bytes"))?;
        self.support_capacity_bytes[slot] = self.support_capacity_bytes[slot]
            .checked_add(bytes)
            .ok_or(CaptureError::CountRange("support capacity total"))?;
        let bytes = u64::try_from(bytes)
            .map_err(|_| CaptureError::CountRange("support capacity conversion"))?;
        self.adapter.enter(AdapterEvent::RustCapacityBytes, bytes);
        self.adapter.require()?;
        Ok(v)
    }
    /// Initialize an empty reserved bitmap with a finite owned loop. The shape
    /// guard enters once; each successful MapWrite immediately precedes one
    /// reserved-capacity push. There is no resize/bulk-library call to charge.
    pub(super) fn fill_support_bitmap(
        &self,
        bitmap: &mut Vec<bool>,
        count: usize,
    ) -> Result<(), CaptureError> {
        self.adapter.require()?;
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        if !bitmap.is_empty() || bitmap.capacity() < count {
            return Err("support bitmap reservation".into());
        }
        for _ in 0..count {
            self.capture_entry(AdapterEvent::MapWrite)?;
            bitmap.push(false);
        }
        Ok(())
    }
    pub(super) fn capture_supports(
        &mut self,
        model: &PreviewModel,
        built: &BuiltModel,
        restrained: &[usize],
        springs: &[SpringEntry],
        parts: &mut k::SourceParts,
    ) -> Result<(), CaptureError> {
        self.adapter.require()?;
        let n = model
            .nodes
            .len()
            .checked_mul(6)
            .ok_or(CaptureError::CountRange("support dofs"))?;
        for count in [
            springs.len(),
            model.supports.len(),
            built.supports.len(),
            restrained.len(),
        ] {
            self.capture_entry(AdapterEvent::ValidationEntry)?;
            u32::try_from(count).map_err(|_| CaptureError::CountRange("support count"))?;
        }
        model
            .supports
            .len()
            .checked_mul(6)
            .ok_or(CaptureError::CountRange("support slots"))?;
        let q = model
            .nodes
            .len()
            .checked_mul(7)
            .and_then(|v| {
                model
                    .pipe_segments
                    .len()
                    .checked_mul(30)
                    .and_then(|m| v.checked_add(m))
            })
            .and_then(|v| v.checked_add(springs.len()))
            .and_then(|v| v.checked_add(restrained.len()))
            .and_then(|v| {
                model
                    .supports
                    .len()
                    .checked_mul(2)
                    .and_then(|g| v.checked_add(g))
            })
            .ok_or(CaptureError::CountRange("support layout quantities"))?;
        u32::try_from(q).map_err(|_| CaptureError::CountRange("support layout quantities"))?;
        let mut built_used = self.support_reserve(built.supports.len(), 0)?;
        self.fill_support_bitmap(&mut built_used, built.supports.len())?;
        let mut spring_used = self.support_reserve(springs.len(), 1)?;
        self.fill_support_bitmap(&mut spring_used, springs.len())?;
        let mut rigid_owned = self.support_reserve(n, 2)?;
        self.fill_support_bitmap(&mut rigid_owned, n)?;
        self.spring_map = self.support_reserve(springs.len(), 3)?;
        parts.springs = self.support_reserve(springs.len(), 4)?;
        self.support_fixed = self.support_reserve(model.supports.len(), 6)?;
        for (i, authored) in model.supports.iter().enumerate() {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            let mut node = None;
            for (j, p) in model.nodes.iter().enumerate() {
                self.capture_entry(AdapterEvent::SourceVisit)?;
                let equal = self.adapter.same(&p.id, &authored.node);
                self.adapter.require()?;
                if equal {
                    node = Some(j);
                    break;
                }
            }
            let node = node.ok_or("support node")?;
            let mut matched = None;
            for (j, b) in built.supports.iter().enumerate() {
                self.capture_entry(AdapterEvent::SourceVisit)?;
                let equal = self.adapter.same(&b.support_id, &authored.id);
                self.adapter.require()?;
                if equal {
                    if matched.is_some() || built_used[j] {
                        return Err("duplicate built support".into());
                    }
                    matched = Some(j);
                }
            }
            let bi = matched.ok_or("missing built support")?;
            let b = &built.supports[bi];
            self.capture_entry(AdapterEvent::ValidationEntry)?;
            if b.node_index != node || b.imposed_displacement.is_some() {
                return Err("support build node/motion".into());
            }
            let is_spring = authored.family.as_deref() == Some("spring");
            let family = match authored.family.as_deref() {
                Some("spring") => SupportFamily::Spring,
                Some("anchor") => SupportFamily::Anchor,
                Some("guide") => SupportFamily::Guide,
                Some("line_stop") => SupportFamily::LineStop,
                Some("vertical_support") => SupportFamily::VerticalSupport,
                None if authored.restraints.len() == 6 => SupportFamily::Anchor,
                None => SupportFamily::Guide,
                _ => return Err("unsupported support family".into()),
            };
            if b.family != family || b.restrained_dofs.len() != authored.restraints.len() {
                return Err("support build family/axes".into());
            }
            let mut fixed = [false; 6];
            for (a, d) in authored.restraints.iter().zip(&b.restrained_dofs) {
                self.capture_entry(AdapterEvent::SourceVisit)?;
                self.capture_entry(AdapterEvent::LibraryBoundary)?;
                if parse_dof(a).map_err(CaptureError::from)? != *d {
                    return Err("support axis".into());
                }
                if !is_spring {
                    let axis = dof_index(*d);
                    let global = node
                        .checked_mul(6)
                        .and_then(|v| v.checked_add(axis))
                        .ok_or(CaptureError::CountRange("support global dof"))?;
                    if fixed[axis] || rigid_owned[global] {
                        return Err("ambiguous rigid ownership".into());
                    }
                    // The producing boundary is unique; also reject a malformed
                    // direct helper slice on its second match without counting up.
                    let mut hit = false;
                    for &g in restrained {
                        self.capture_entry(AdapterEvent::SourceVisit)?;
                        self.capture_entry(AdapterEvent::ValidationEntry)?;
                        if g == global {
                            if hit {
                                return Err("rigid boundary identity".into());
                            }
                            hit = true;
                        }
                    }
                    if !hit {
                        return Err("rigid boundary identity".into());
                    }
                    self.capture_entry(AdapterEvent::MapWrite)?;
                    fixed[axis] = true;
                    rigid_owned[global] = true;
                }
            }
            let mut children = self.support_reserve(usize::from(is_spring), 5)?;
            let mut found_spring = false;
            for (j, spring) in springs.iter().enumerate() {
                self.capture_entry(AdapterEvent::SourceVisit)?;
                let equal = self.adapter.same(&spring.support_id, &authored.id);
                self.adapter.require()?;
                if !equal {
                    continue;
                }
                self.capture_entry(AdapterEvent::ValidationEntry)?;
                if !is_spring || found_spring || spring_used[j] {
                    return Err("duplicate/extra support spring".into());
                }
                let input = authored
                    .stiffness
                    .as_ref()
                    .ok_or("authored spring stiffness")?;
                let stiffness = b.stiffness.as_ref().ok_or("built spring stiffness")?;
                self.capture_entry(AdapterEvent::LibraryBoundary)?;
                let dof = parse_dof(&input.dof).map_err(CaptureError::from)?;
                let dimension = if dof.is_translational() {
                    QuantityDimension::TranslationalStiffness
                } else {
                    QuantityDimension::RotationalStiffness
                };
                if b.restrained_dofs.as_slice() != [dof]
                    || spring.node_dof.node_index != node
                    || spring.node_dof.dof != dof
                    || stiffness.dimension != dimension
                    || spring.stiffness.dimension != dimension
                    || !input.value.value.is_finite()
                    || input.value.value.to_bits() != stiffness.value.to_bits()
                    || input.value.value.to_bits() != spring.stiffness.value.to_bits()
                {
                    return Err("spring source identity".into());
                }
                let global = spring.node_dof.global_index();
                self.capture_entry(AdapterEvent::MapWrite)?;
                spring_used[j] = true;
                found_spring = true;
                self.capture_entry(AdapterEvent::MapWrite)?;
                children.push(j as u32);
                self.capture_entry(AdapterEvent::MapWrite)?;
                parts.springs.push(k::Spring {
                    id: j as u32,
                    dof: k::Dof::from_global(global),
                    stiffness: spring.stiffness.value,
                });
                self.capture_entry(AdapterEvent::MapWrite)?;
                self.spring_map.push(SpringIdentity {
                    boundary: j,
                    support: i,
                    dof: global,
                    bits: spring.stiffness.value.to_bits(),
                });
            }
            if is_spring != found_spring || (!is_spring && b.stiffness.is_some()) {
                return Err("missing/foreign support stiffness".into());
            }
            let id = self.adapter.copy(&authored.id)?;
            self.capture_entry(AdapterEvent::MapWrite)?;
            built_used[bi] = true;
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.supports.push((id, node));
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.support_fixed.push(fixed);
            self.capture_entry(AdapterEvent::MapWrite)?;
            parts.supports.push(k::SupportGroup {
                id: i as u32,
                node: node as u32,
                restrained: fixed,
                springs: children,
                directional_springs: Vec::new(),
            });
        }
        for used in built_used.into_iter().chain(spring_used) {
            self.capture_entry(AdapterEvent::ValidationEntry)?;
            if !used {
                return Err("unconsumed built/boundary support".into());
            }
        }
        for &g in restrained {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            if !rigid_owned.get(g).copied().unwrap_or(false) {
                return Err("unowned rigid boundary".into());
            }
        }
        Ok(())
    }
    fn check_support_source(&self, source: &k::PrimitiveSource) -> Result<(), CaptureError> {
        self.check_support_maps(source.supports(), source.springs(), true)
    }
    pub(super) fn check_support_maps(
        &self,
        groups: &[k::SupportGroup],
        springs: &[k::Spring],
        canonical: bool,
    ) -> Result<(), CaptureError> {
        self.adapter.require()?;
        if groups.len() != self.supports.len()
            || springs.len() != self.spring_map.len()
            || self.support_fixed.len() != self.supports.len()
        {
            return Err("canonical support counts".into());
        }
        for (i, group) in groups.iter().enumerate() {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            if group.id != i as u32
                || group.node as usize != self.supports[i].1
                || group.restrained != self.support_fixed[i]
                || !group.directional_springs.is_empty()
            {
                return Err("canonical support identity".into());
            }
            let mut count = 0;
            for map in &self.spring_map {
                self.capture_entry(AdapterEvent::SourceVisit)?;
                if map.support != i {
                    continue;
                }
                count += 1;
                if group.springs.as_slice() != [map.boundary as u32] {
                    return Err("canonical support membership".into());
                }
                let mut found = None;
                for (index, spring) in springs.iter().enumerate() {
                    self.capture_entry(AdapterEvent::SourceVisit)?;
                    self.capture_entry(AdapterEvent::ValidationEntry)?;
                    if spring.id != map.boundary as u32 {
                        continue;
                    }
                    if found.is_some() || (canonical && index != map.boundary) {
                        return Err("canonical spring occurrence".into());
                    }
                    found = Some(spring);
                }
                let spring = found.ok_or("canonical spring")?;
                if spring.dof.global() != map.dof || spring.stiffness.to_bits() != map.bits {
                    return Err("canonical spring identity".into());
                }
            }
            if count != group.springs.len() {
                return Err("canonical support coverage".into());
            }
        }
        Ok(())
    }
    pub fn case_source(
        &mut self,
        model: &PreviewModel,
        built: &BuiltModel,
        materials: &[MaterialInput],
        case: &PreviewLoadCase,
        restrained: &[usize],
        springs: &[SpringEntry],
        application: &LoadApplication,
        thermal: &[ThermalElementLoad],
        pressure: &[PressureThrustLoad],
    ) {
        if self.prepared_probe {
            if self.error.is_some() {return;}
            if let Err(e)=self.prepared_case_seen(model,case) {self.error=Some(e);}
            return;
        }
        self.case_calls=self.case_calls.checked_add(1).expect("bounded cases");
        self.capture_case_source(model,built,materials,case,restrained,springs,application,thermal,pressure);
    }
    fn capture_case_source(&mut self, model:&PreviewModel, built:&BuiltModel,
        materials:&[MaterialInput], case:&PreviewLoadCase, restrained:&[usize],
        springs:&[SpringEntry], application:&LoadApplication, thermal:&[ThermalElementLoad],
        pressure:&[PressureThrustLoad]) {
        if self.error.is_some() {
            return;
        }
        let Some(entries)=self.source_capture_entries.checked_add(1) else {self.fail_count("source capture entries");return;};
        if let Err(e)=self.capture_entry(AdapterEvent::MapWrite) {self.error=Some(e);return;}
        self.source_capture_entries=entries;
        // B1 SP (T-2): the prepared probe captures each requested case in its own slot.
        if self.case_calls != 1 || (model.load_cases.len() != 1 && !self.prepared_probe) {
            self.fail("private witness has exactly one actual case");
            return;
        }
        // B3b-P (P-5; D1.5-exact, DEF-E `scope.materials`): the exact route's base common E/ν only.
        if self.route == W1Route::Exact && load_case_selector(case).is_some() {
            self.fail("exact route: base common E/nu selection only");
            return;
        }
        if let Err(e) = self.basis_source_case(case) {
            self.error = Some(e);
            return;
        }
        if !built.nonlinear_supports.is_empty()
            || !built.user_stiffness_elements.is_empty()
            || !built.curved_bend_elements.is_empty()
            || !thermal.is_empty()
            || !pressure.is_empty()
            || !application.element_uniform_loads.is_empty()
            || !application.imposed_displacements.is_empty()
            || case.equivalent_static.is_some()
            || case
                .pressure_regions
                .as_ref()
                .is_some_and(|v| !v.is_empty())
            || model.supports.iter().any(|s| s.hanger.is_some())
        {
            self.fail("unsupported producer present");
            return;
        }
        if !self.adapter.enter(AdapterEvent::LibraryBoundary, 1) {
            self.fail("case capture accounting");
            return;
        }
        let mut parts = k::SourceParts::default();
        let allocated = (|| -> Result<(), CaptureError> {
            self.case_id = self.adapter.copy(&case.id)?;
            parts.nodes = self.adapter.reserve(built.nodes.len())?;
            parts.members = self.adapter.reserve(model.pipe_segments.len())?;
            parts.stations = self.adapter.reserve(
                model
                    .pipe_segments
                    .len()
                    .checked_mul(3)
                    .ok_or(CaptureError::CountRange("stations"))?,
            )?;
            parts.constraints = self.adapter.reserve(restrained.len())?;
            parts.supports = self.adapter.reserve(model.supports.len())?;
            parts.loads = self.adapter.reserve(application.nodal_loads.len())?;
            self.facts = self.adapter.reserve(model.pipe_segments.len())?;
            self.members = self.adapter.reserve(model.pipe_segments.len())?;
            self.operational = self.adapter.reserve(model.pipe_segments.len())?;
            self.supports = self.adapter.reserve(model.supports.len())?;
            self.terms = self.adapter.reserve(application.nodal_loads.len())?;
            Ok(())
        })();
        if let Err(e) = allocated {
            self.error = Some(e);
            return;
        }
        for n in &built.nodes {
            if !self.adapter.enter(AdapterEvent::SourceVisit, 1)
                || !self.adapter.enter(AdapterEvent::MapWrite, 1)
            {
                self.fail("built-node accounting");
                return;
            }
            parts.nodes.push(n.coordinates);
        }
        if parts.nodes.len() != self.nodes.len()
            || parts
                .nodes
                .iter()
                .zip(&self.nodes)
                .any(|(a, b)| a.map(f64::to_bits) != b.1.map(f64::to_bits))
        {
            self.fail("normalized node/built identity");
            return;
        }
        for (i, pipe) in model.pipe_segments.iter().enumerate() {
            if !self.adapter.enter(AdapterEvent::SourceVisit, 1)
                || !self.adapter.enter(AdapterEvent::ValidationEntry, 1)
            {
                self.fail("member accounting");
                return;
            }
            let Some(bi) = built
                .pipes
                .iter()
                .position(|p| self.adapter.same(&p.element_id, &pipe.id))
            else {
                self.fail("missing built member");
                return;
            };
            let b = &built.pipes[bi];
            let Some(section) = built.sections.get(&pipe.id) else {
                self.fail("missing section");
                return;
            };
            let Some(mi) = self
                .materials
                .iter()
                .position(|m| self.adapter.same(&m.0, &pipe.material))
            else {
                self.fail("normalized material");
                return;
            };
            let Some(m) = materials
                .iter()
                .find(|m| self.adapter.same(&m.id, &pipe.material))
            else {
                self.fail("resolved material");
                return;
            };
            let id = match u32::try_from(i) {
                Ok(i) => i,
                Err(_) => {
                    self.fail_count("member count range");
                    return;
                }
            };
            let material =
                if case.modulus_basis_ref.is_none() && case.modulus_basis_temperature.is_none() {
                    match self.route {
                        W1Route::Preview => k::ProductMaterial::Base {
                            e: self.materials[mi].1,
                            g: self.materials[mi].2,
                        },
                        // B3b-P (P-5): the exact route's base common E/ν (B3-K's BaseENu); the
                        // source lane encloses G = E/(2(1+ν)) from E and ν (DEF-E `lanes`).
                        W1Route::Exact => match self.material_nu.get(mi).copied().flatten() {
                            Some(nu) => k::ProductMaterial::BaseENu { e: self.materials[mi].1, nu },
                            None => {
                                self.fail("exact material Poisson ratio");
                                return;
                            }
                        },
                    }
                } else {
                    let Some(s) = self.selections.iter().find(|s| {
                        self.adapter.same(&s.case, &case.id)
                            && self.adapter.same(&s.material, &pipe.material)
                    }) else {
                        self.fail("selection missing");
                        return;
                    };
                    s.selected
                };
            if m.elastic_modulus.value.to_bits() != b.section.elastic_modulus.to_bits()
                || m.shear_modulus.as_ref().unwrap().value.to_bits()
                    != b.section.shear_modulus.to_bits()
                || section.area.to_bits() != b.section.area.to_bits()
                || section.torsion_constant.to_bits() != b.section.torsion_constant.to_bits()
            {
                self.fail("resolved/built K mismatch");
                return;
            }
            let (Ok(ni), Ok(nj)) = (u32::try_from(b.node_i.index), u32::try_from(b.node_j.index))
            else {
                self.fail_count("node count range");
                return;
            };
            if !self.adapter.enter(AdapterEvent::MapWrite, 1) {
                self.fail("map-write accounting");
                return;
            }
            parts.members.push(k::StraightMember {
                id,
                node_i: ni,
                node_j: nj,
                elastic_modulus: b.section.elastic_modulus,
                shear_modulus: b.section.shear_modulus,
                area: b.section.area,
                second_moment_y: b.section.second_moment_y,
                second_moment_z: b.section.second_moment_z,
                torsion_constant: b.section.torsion_constant,
                y_reference: b.y_reference,
            });
            if !self.adapter.enter(AdapterEvent::MapWrite, 1) {
                self.fail("map-write accounting");
                return;
            }
            if !self.adapter.enter(AdapterEvent::RequestedCopyBytes,std::mem::size_of::<Option<u32>>() as u64){self.fail("operational identity accounting");return;}
            self.operational.push(evaluate_member_operational(id,
                [b.node_i.coordinates, b.node_j.coordinates],
                [
                    b.section.elastic_modulus,
                    b.section.shear_modulus,
                    b.section.area,
                    b.section.torsion_constant,
                ],
            ));
            if !self.adapter.enter(AdapterEvent::MapWrite, 1) {
                self.fail("map-write accounting");
                return;
            }
            self.facts.push(k::ProductMemberFacts {
                member: id,
                diameter: pipe.section.outside_diameter.value,
                effective_wall: section.wall_thickness,
                material,
                area: section.area,
                second_moment: section.second_moment,
                torsion_constant: section.torsion_constant,
                section_modulus: section.section_modulus,
                radius: section.torsion_radius,
            });
            self.adapter.enter(AdapterEvent::LibraryBoundary, 1);
            if !self.adapter.enter(AdapterEvent::MapWrite, 1) {
                self.fail("map-write accounting");
                return;
            }
            self.members.push(MemberIdentity {
                model: i,
                built: bi,
                id: pipe.id.clone(),
                material: mi,
                nodes: [b.node_i.index, b.node_j.index],
            });
            for (j, fraction) in [0.25, 0.5, 0.75].into_iter().enumerate() {
                let Some(station) = id.checked_mul(3).and_then(|v| v.checked_add(j as u32)) else {
                    self.fail_count("station count");
                    return;
                };
                if !self.adapter.enter(AdapterEvent::SourceVisit, 1)
                    || !self.adapter.enter(AdapterEvent::MapWrite, 1)
                {
                    self.fail("station-write accounting");
                    return;
                }
                parts.stations.push(k::Station {
                    id: station,
                    member: id,
                    fraction,
                });
            }
        }
        if built.pipes.len() != self.members.len() {
            self.fail("extra built member");
            return;
        }
        for &g in restrained {
            if !self.adapter.enter(AdapterEvent::SourceVisit, 1) {
                self.fail("constraint accounting");
                return;
            }
            if g >= parts.nodes.len() * 6 {
                self.fail("constraint index");
                return;
            }
            if !self.adapter.enter(AdapterEvent::MapWrite, 1) {
                self.fail("constraint-write accounting");
                return;
            }
            parts.constraints.push(k::Constraint {
                dof: k::Dof::from_global(g),
                value: 0.0,
            });
        }
        if let Err(e) = self.capture_supports(model, built, restrained, springs, &mut parts) {
            self.error = Some(e);
            return;
        }
        if let Err(e) = self.check_support_maps(&parts.supports, &parts.springs, false) {
            self.error = Some(e);
            return;
        }
        let mut used = match self.adapter.reserve(case.primitive_loads.len()) {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };
        used.resize(case.primitive_loads.len(), false);
        for (occurrence, l) in application.nodal_loads.iter().enumerate() {
            if !self.adapter.enter(AdapterEvent::SourceVisit, 1)
                || !self.adapter.enter(AdapterEvent::ValidationEntry, 1)
            {
                self.fail("term accounting");
                return;
            }
            let candidate=case.primitive_loads.iter().enumerate().position(|(i,p)| !used[i]&&self.adapter.same(&p.id,&l.load_id)
                && p.magnitude.value.to_bits()==l.value.to_bits()
                && matches!(&p.target,LoadTargetInput::Node{node} if node==&model.nodes[l.node_index].id)
                && parse_direction(&p.direction).is_ok_and(|d|d.dof_index()==l.global_dof%6));
            let Some(original) = candidate else {
                self.fail("individual load occurrence not authored");
                return;
            };
            if !self.adapter.enter(AdapterEvent::LibraryBoundary, 2)
                || !self.adapter.enter(AdapterEvent::MapWrite, 3)
            {
                self.fail("term-write accounting");
                return;
            }
            used[original] = true;
            self.terms.push(TermIdentity {
                original,
                occurrence,
                canonical: usize::MAX,
                id: l.load_id.clone(),
                dof: l.global_dof,
                bits: l.value.to_bits(),
            });
            parts.loads.push(k::NodalLoad {
                dof: k::Dof::from_global(l.global_dof),
                value: l.value,
                source_id: l.load_id.clone(),
            });
        }
        if used.iter().any(|u| !*u) {
            self.fail("unconsumed authored primitive");
            return;
        }
        if !self.adapter.enter(AdapterEvent::LibraryBoundary, 1)
            || self.adapter.fault.get().is_some()
        {
            self.fail("source construction accounting");
            return;
        }
        match k::PrimitiveSource::new(parts) {
            Ok(source) => {
                // Reconcile each canonical load occurrence to the producing record, not a rounded net.
                let mut used = match self.adapter.reserve(self.terms.len()) {
                    Ok(v) => v,
                    Err(e) => {
                        self.error = Some(e);
                        return;
                    }
                };
                used.resize(self.terms.len(), false);
                for (canonical, l) in source.loads().iter().enumerate() {
                    if !self.adapter.enter(AdapterEvent::SourceVisit, 1) {
                        self.fail("canonical accounting");
                        return;
                    }
                    let Some(i) = self.terms.iter().enumerate().position(|(i, t)| {
                        !used[i]
                            && self.adapter.same(&t.id, &l.source_id)
                            && t.dof == l.dof.global()
                            && t.bits == l.value.to_bits()
                    }) else {
                        self.fail("canonical term map");
                        return;
                    };
                    if !self.adapter.enter(AdapterEvent::MapWrite, 2) {
                        self.fail("canonical-write accounting");
                        return;
                    }
                    used[i] = true;
                    self.terms[i].canonical = canonical;
                }
                if let Err(e) = self.check_support_source(&source) {
                    self.error = Some(e);
                    return;
                }
                self.source = Some(source);
            }
            Err(e) => self.error = Some(CaptureError::Source(e)),
        }
        self.capacities = vec![
            ("nodes", self.nodes.capacity()),
            ("materials", self.materials.capacity()),
            ("selections", self.selections.capacity()),
            ("members", self.members.capacity()),
            ("terms", self.terms.capacity()),
            ("facts", self.facts.capacity()),
            ("supports", self.supports.capacity()),
            ("spring_map", self.spring_map.capacity()),
            ("support_fixed", self.support_fixed.capacity()),
        ];
    }
    pub fn finish(&mut self, envelope: &MechanicsEnvelope) {
        self.final_calls = self
            .final_calls
            .checked_add(1)
            .expect("bounded final hooks");
        if self.error.is_some() {
            return;
        }
        if self.adapter.fault.get().is_some() {
            self.fail("adapter accounting");
            return;
        }
        if self.final_calls != 1 {
            self.fail("final hook repeated");
            return;
        }
        if envelope.producer.semantic_contract_id != self.route.base_contract()
            || envelope.source_block_recovery.is_some()
            || envelope.status.mechanics != "MECHANICS_SOLVED"
        {
            self.fail("not final ordinary preview");
            return;
        }
        if self.prepared_probe {
            // B1 SP (T-2): every case seen, parked or not, completed its own late hook.
            if self.prepared_late_calls!=1 || !self.prepared_source_permit
                || !self.prepared_one_case_seen || self.source_capture_entries!=1 || self.source.is_none()
                || self.parked.iter().any(|slot|slot.prepared_late_calls!=1 || !slot.prepared_source_permit
                    || !slot.prepared_one_case_seen || slot.source_capture_entries!=1 || slot.source.is_none()) {
                self.fail("missing successful prepared late source hook");return;
            }
            let bound=if self.parked.is_empty() {self.bind_observations(envelope)} else {self.bind_observations_by_case(envelope)};
            if let Err(e)=bound {self.error=Some(e);}
            return;
        }
        let Some(source) = self.source.as_ref() else {
            self.fail("missing actual source");
            return;
        };
        let cap = match k::OriginCapacity::for_calls(&[1], &[]) {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(CaptureError::Origin(e));
                return;
            }
        };
        let mut invocation = match k::RecordedInvocation::new(60_000_000_000, cap) {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(CaptureError::Origin(e));
                return;
            }
        };
        self.adapter.enter(AdapterEvent::LibraryBoundary, 1);
        let mut cases = match invocation.solve_cases(
            std::slice::from_ref(source),
            k::CaseLimit::new(20_000_000_000),
        ) {
            Ok(v) => v,
            Err(e) => {
                self.error = Some(CaptureError::Origin(e));
                return;
            }
        };
        let case = cases.remove(0);
        let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
            self.error = Some(CaptureError::NativeUnavailable);
            self.native_invocation = Some(invocation);
            self.native = Some(case);
            return;
        };
        match self.bind_rows(envelope, owner) {
            Ok(rows) => {
                let spent = invocation.certify_product_case(case.run, owner, &self.facts, &rows);
                self.numeric_pass = spent.passed();
                self.verdicts = spent.verdicts().to_vec();
                self.summary_coverage = spent.summary_coverage().to_vec();
                self.source_correction_calls = spent.source_correction_calls();
                self.numeric_failure = spent.failure().cloned();
                self.work = format!(
                    "{:?}; native={:?}",
                    spent.work_summary(),
                    spent.native_work()
                );
                self.observable_error = self.observables(envelope).err();
                self.g5a_error = self.g5a(owner, &rows).err();
            }
            Err(e) => {
                self.error = Some(
                    self.adapter
                        .fault
                        .get()
                        .map(CaptureError::Accounting)
                        .unwrap_or(e),
                );
            }
        }
        self.native_invocation = Some(invocation);
        self.native = Some(case);
    }
    pub(super) fn bind_rows<'a>(
        &self,
        e: &'a MechanicsEnvelope,
        owner: &k::RetainedSolve,
    ) -> Result<Vec<k::ProductFinalRow<'a>>, CaptureError> {
        self.bind_rows_view(ProductCaseView::of(e,CaseScope::whole_ref(self.route)),owner)
    }
    /// B2-P: a mechanics combination's rows bound within its block (T-11's serializer).
    pub(super) fn bind_combination_rows_scoped<'a>(&self,e:&'a MechanicsEnvelope,scope:&'a CaseScope,owner:&k::RetainedSolve,combination:&str)
        -> Result<Vec<k::ProductFinalRow<'a>>,CaptureError> {
        self.bind_rows_subject(ProductCaseView::of(e,scope),owner,Some(combination))
    }
    /// `bind_rows` within one requested case's scope (B1 SP, T-11's serializer).
    pub(super) fn bind_rows_scoped<'a>(&self,e:&'a MechanicsEnvelope,scope:&'a CaseScope,owner:&k::RetainedSolve)
        -> Result<Vec<k::ProductFinalRow<'a>>,CaptureError> {
        self.bind_rows_view(ProductCaseView::of(e,scope),owner)
    }
    fn bind_rows_view<'a>(&self,view:ProductCaseView<'a>,owner:&k::RetainedSolve)
        -> Result<Vec<k::ProductFinalRow<'a>>,CaptureError> {
        self.bind_rows_subject(view,owner,None)
    }
    /// The rows of `view` bound to `owner`'s quantities: the case's (`combination` None) or,
    /// B2-P, a mechanics combination's own block (`Some(its id)`: basis `combination` naming it,
    /// combination metadata and ids, no mode, parity or record row; B2-C §2.4 (iii) 4).
    fn bind_rows_subject<'a>(&self,view:ProductCaseView<'a>,owner:&k::RetainedSolve,combination:Option<&str>)
        -> Result<Vec<k::ProductFinalRow<'a>>,CaptureError> {
        self.adapter.require()?;
        if self.error.is_some() {
            return Err("prior capture refusal".into());
        }
        if combination.is_none() {
            self.bind_observations_view(view)?;
        }
        self.adapter.enter(AdapterEvent::ValidationEntry, 1);
        self.adapter.require()?;
        if self.basis_expected != self.basis_record.is_some()
            || self.basis_record_calls != usize::from(self.basis_expected)
        {
            return Err("basis record capture presence".into());
        }
        let mut modulus_basis = false;
        let mut out = self.adapter.reserve(view.case_rows().len())?;
        for observed in view.rows() {
            let r=observed.original;let final_value=observed.value;
            if !self.adapter.enter(AdapterEvent::RowVisit, 1)
                || !self.adapter.enter(AdapterEvent::ValidationEntry, 1)
            {
                self.adapter.require()?;
            }
            let basis = r.basis_ref.as_ref().ok_or("final basis")?;
            let same_case = match combination {
                None => basis.ref_type == "load_case" && self.adapter.same(&basis.ref_id, &self.case_id),
                Some(id) => basis.ref_type == "combination" && self.adapter.same(&basis.ref_id, id),
            };
            self.adapter.require()?;
            if !same_case {
                return Err("case binding".into());
            }
            if combination.is_some() && matches!(r.kind.as_str(),"linear_solver_mode_basis"|"sparse_live_path_dense_parity_relative_delta"|"modulus_basis_record") {
                return Err("combination record row".into());
            }
            let node = self
                .nodes
                .iter()
                .position(|n| self.adapter.same(&n.0, &r.entity_ref));
            let member = self
                .members
                .iter()
                .position(|m| self.adapter.same(&m.id, &r.entity_ref));
            let support = self
                .supports
                .iter()
                .position(|s| self.adapter.same(&s.0, &r.entity_ref));
            self.adapter.require()?;
            let loc = r.metadata.as_ref().map(|m| m.location.as_str());
            let (recipe, body, unit) = if r.kind == "linear_solver_mode_basis" {
                if r.unit != "mode_code" {
                    return Err("mode unit".into());
                }
                (k::ProductRecipe::NonQuantity, 0, k::ProductUnit::Record)
            } else if r.kind == "sparse_live_path_dense_parity_relative_delta" {
                (k::ProductRecipe::DenseParityObservation, 0, k::ProductUnit::Record)
            } else if r.kind == "modulus_basis_record" {
                if modulus_basis || !self.basis_expected {
                    return Err("modulus record coverage".into());
                }
                self.validate_modulus_record(r)?;
                modulus_basis = true;
                (
                    k::ProductRecipe::ModulusBasisRecord,
                    0,
                    k::ProductUnit::Record,
                )
            } else if r.kind == "displacement_magnitude" {
                let n = node.ok_or("node")?;
                if r.unit != "mm" {
                    return Err("magnitude unit".into());
                }
                (
                    k::ProductRecipe::Native(k::QuantityId::DisplacementMagnitude(n as u32)),
                    owner.source().body_of_node(n as u32),
                    k::ProductUnit::Millimetre,
                )
            } else if let Some(c) = [
                "global_nodal_displacement_x",
                "global_nodal_displacement_y",
                "global_nodal_displacement_z",
                "global_nodal_rotation_x",
                "global_nodal_rotation_y",
                "global_nodal_rotation_z",
            ]
            .iter()
            .position(|s| *s == r.kind)
            {
                let n = node.ok_or("node")?;
                let unit = if c < 3 {
                    k::ProductUnit::Millimetre
                } else {
                    k::ProductUnit::Radian
                };
                if r.unit != if c < 3 { "mm" } else { "rad" } {
                    return Err("node unit".into());
                }
                (
                    k::ProductRecipe::Native(k::QuantityId::Displacement(k::Dof {
                        node: n as u32,
                        component: k::Component::ALL[c],
                    })),
                    owner.source().body_of_node(n as u32),
                    unit,
                )
            } else if r.kind.starts_with("support_reaction_") {
                let s = support.ok_or("support")?;
                let m = r.metadata.as_ref().ok_or("support metadata")?;
                let (recipe, unit) = if r.kind == "support_reaction_force_magnitude_v2" {
                    (k::ProductRecipe::Native(k::QuantityId::SupportForceMagnitude(s as u32)), k::ProductUnit::Newton)
                } else if r.kind == "support_reaction_moment_magnitude_v2" {
                    (k::ProductRecipe::Native(k::QuantityId::SupportMomentMagnitude(s as u32)), k::ProductUnit::NewtonMetre)
                } else if r.kind == "support_reaction_component_v2" {
                    let c = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"].iter()
                        .position(|v| *v == m.component).ok_or("support component")?;
                    (k::ProductRecipe::SupportComponent { support: s as u32, component: k::Component::ALL[c] },
                        if c < 3 { k::ProductUnit::Newton } else { k::ProductUnit::NewtonMetre })
                } else { return Err("unsupported support row".into()); };
                if r.unit
                    != if unit == k::ProductUnit::Newton {
                        "N"
                    } else {
                        "N*m"
                    }
                {
                    return Err("support unit".into());
                }
                (
                    recipe,
                    owner.source().body_of_node(self.supports[s].1 as u32),
                    unit,
                )
            } else {
                let m = member.ok_or("row member")?;
                let id = self.members[m].model as u32;
                let body = owner.source().body_of_node(self.members[m].nodes[0] as u32);
                if r.kind == "pipe_elastic_normal_stress_maximum_v2" {
                    if r.unit != "Pa" {
                        return Err("maximum unit".into());
                    }
                    (
                        k::ProductRecipe::CircularMaximum { member: id },
                        body,
                        k::ProductUnit::Pascal,
                    )
                } else {
                    let site = match loc {
                        Some("end_i") => k::ProductSite::End(k::End::I),
                        Some("end_j") => k::ProductSite::End(k::End::J),
                        Some("quarter_1") => k::ProductSite::Station(id * 3),
                        Some("midspan") => k::ProductSite::Station(id * 3 + 1),
                        Some("quarter_3") => k::ProductSite::Station(id * 3 + 2),
                        _ => return Err("row site".into()),
                    };
                    if let Some(c) = [
                        "element_local_axial_force",
                        "element_local_shear_force_y",
                        "element_local_shear_force_z",
                        "element_local_torsional_moment",
                        "element_local_bending_moment_y",
                        "element_local_bending_moment_z",
                    ]
                    .iter()
                    .position(|s| *s == r.kind)
                    {
                        let q = match site {
                            k::ProductSite::End(end) => k::QuantityId::EndAction {
                                member: id,
                                end,
                                component: k::Component::ALL[c],
                            },
                            k::ProductSite::Station(station) => k::QuantityId::StationAction {
                                station,
                                component: k::Component::ALL[c],
                            },
                        };
                        if r.unit != if c < 3 { "N" } else { "N*m" } {
                            return Err("action unit".into());
                        }
                        (
                            k::ProductRecipe::Native(q),
                            body,
                            if c < 3 {
                                k::ProductUnit::Newton
                            } else {
                                k::ProductUnit::NewtonMetre
                            },
                        )
                    } else {
                        let stress = match r.kind.as_str() {
                            "element_local_axial_normal_stress" => k::ProductStress::Axial,
                            "element_local_bending_normal_stress_y" => k::ProductStress::BendingY,
                            "element_local_bending_normal_stress_z" => k::ProductStress::BendingZ,
                            "element_local_torsional_shear_stress" => k::ProductStress::Torsion,
                            _ => {
                                return Err(CaptureError::Association(format!(
                                    "uncovered actual row {}",
                                    r.kind
                                )))
                            }
                        };
                        if r.unit != "MPa" {
                            return Err("stress unit".into());
                        }
                        (
                            k::ProductRecipe::Stress {
                                member: id,
                                site,
                                stress,
                            },
                            body,
                            k::ProductUnit::Megapascal,
                        )
                    }
                }
            };
            if recipe != k::ProductRecipe::ModulusBasisRecord {
                let subject=match combination {None=>RowSubject::Case{case:&self.case_id,qualified:view.scope.qualified},Some(id)=>RowSubject::Combination(id)};
                validate_final_metadata(r, recipe, subject, &self.adapter)?;
            }
            self.adapter.enter(AdapterEvent::MapWrite, 1);
            self.adapter.require()?;
            out.push(k::ProductFinalRow {
                id: &r.id,
                case_id: &basis.ref_id,
                value: final_value,
                unit,
                body,
                recipe,
            });
        }
        self.adapter.require()?;
        if modulus_basis != (self.basis_expected && combination.is_none()) {
            return Err("missing final modulus record".into());
        }
        Ok(out)
    }
    pub(super) fn observables(&self, e: &MechanicsEnvelope) -> Result<(), CaptureError> {
        self.observables_view(ProductCaseView::of(e,&CaseScope::whole(self.route)))
    }
    /// The case's observables: its route's evidence entry (B3-D P-6 on the exact route), then
    /// the shared checks of its `pipe_stress_extrema`, support magnitudes and headlines.
    fn observables_view(&self,view:ProductCaseView<'_>)->Result<(),CaptureError> {
        let extrema = match view.scope.route {
            W1Route::Preview => self.preview_case_evidence(view)?,
            W1Route::Exact => self.exact_case_evidence(view)?,
        };
        self.case_observables(view, extrema)
    }
    /// B3b-P (B3-D P-6): the exact route's closed physics-1 evidence and the case's entry:
    /// `{pressure: [], connector: [], exact_cases}`; the entry's eight keys (no
    /// `recovery_method`), its case, `profile_mode`, the base common E/ν basis, complete
    /// `stress_maximum_coverage`, `pressure_rhs_assembly` with no groups and all-zero vectors,
    /// and `pipe_sections` and `pipe_materials` covering every member, each bound to the
    /// captured normalized OD and effective wall, and to the captured E, ν and Ĝ. Returns the
    /// entry's `pipe_stress_extrema`.
    fn exact_case_evidence<'a>(&self,view:ProductCaseView<'a>)->Result<&'a [serde_json::Value],CaptureError> {
        let e=view.ordinary();
        self.adapter.require()?;
        self.adapter.enter(AdapterEvent::LibraryBoundary, 1); // Closed evidence inspection; serde/number internals remain unqualified.
        let evidence = e.contract_evidence.as_ref().ok_or("exact evidence")?;
        self.adapter.closed_keys(evidence, &["pressure", "connector", "exact_cases"], "evidence shape")?;
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        if evidence["pressure"].as_array().is_none_or(|v| !v.is_empty()) || evidence["connector"].as_array().is_none_or(|v| !v.is_empty()) {
            return Err("exact pressure/connector evidence".into());
        }
        let cases = evidence["exact_cases"].as_array().ok_or("exact cases")?;
        if cases.len() != view.scope.cases {
            return Err("evidence case".into());
        }
        let c = cases.get(view.scope.evidence).ok_or("evidence case")?;
        self.adapter.closed_keys(c, &["load_case_id", "profile_mode", "material_basis", "pressure_rhs_assembly", "pipe_sections",
            "pipe_stress_extrema", "stress_maximum_coverage", "pipe_materials"], "case shape")?;
        let case_id = c["load_case_id"].as_str().ok_or("evidence case")?;
        if !self.checked_same(case_id, &self.case_id)? {
            return Err("evidence case".into());
        }
        if !self.checked_same(c["profile_mode"].as_str().ok_or("exact profile mode")?, "exact_straight_pressure_v2")?
            || !self.checked_same(c["material_basis"].as_str().ok_or("exact material basis")?, "base_material_common_E_nu")? {
            return Err("exact profile/material basis".into());
        }
        self.adapter.closed_keys(&c["stress_maximum_coverage"], &["complete", "unavailable_pipe_ids"], "stress coverage shape")?;
        if c["stress_maximum_coverage"]["complete"] != true
            || c["stress_maximum_coverage"]["unavailable_pipe_ids"].as_array().is_none_or(|v| !v.is_empty()) {
            return Err("maximum coverage".into());
        }
        // No pressure region (D1.5-exact): no assembly group, and every assembled vector +0.
        let assembly = &c["pressure_rhs_assembly"];
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        if assembly["groups"].as_array().is_none_or(|v| !v.is_empty()) {
            return Err("exact pressure assembly groups".into());
        }
        for key in ["assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global"] {
            for v in assembly[key].as_array().ok_or("exact pressure assembly vector")? {
                self.capture_entry(AdapterEvent::RowVisit)?;
                if v.as_f64().is_none_or(|x| x.to_bits() != 0) {
                    return Err("exact pressure assembly vector".into());
                }
            }
        }
        let sections = c["pipe_sections"].as_array().ok_or("exact pipe sections")?;
        let materials = c["pipe_materials"].as_array().ok_or("exact pipe materials")?;
        if sections.len() != self.members.len() || materials.len() != self.members.len() {
            return Err("exact section/material coverage".into());
        }
        for (mi, member) in self.members.iter().enumerate() {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            let f = self.facts.get(mi).ok_or("exact section facts")?;
            let mut section = None;
            for s in sections {
                self.capture_entry(AdapterEvent::RowVisit)?;
                if self.checked_same(s["pipe_id"].as_str().ok_or("exact section id")?, &member.id)? {
                    if section.is_some() {
                        return Err("exact section identity".into());
                    }
                    section = Some(s);
                }
            }
            let s = section.ok_or("exact section identity")?;
            let bits = |key: &str| s[key].as_f64().map(f64::to_bits);
            if bits("outside_diameter_m") != Some(f.diameter.to_bits()) || bits("effective_wall_thickness_m") != Some(f.effective_wall.to_bits()) {
                return Err("exact section geometry".into());
            }
            let (_, e_pa, g_pa) = self.materials.get(member.material).ok_or("exact material")?;
            let nu = self.material_nu.get(member.material).copied().flatten().ok_or("exact material Poisson ratio")?;
            let mut material = None;
            for x in materials {
                self.capture_entry(AdapterEvent::RowVisit)?;
                if self.checked_same(x["pipe_id"].as_str().ok_or("exact material id")?, &member.id)? {
                    if material.is_some() {
                        return Err("exact material identity".into());
                    }
                    material = Some(x);
                }
            }
            let x = material.ok_or("exact material identity")?;
            let bits = |key: &str| x[key].as_f64().map(f64::to_bits);
            if bits("E_pa") != Some(e_pa.to_bits()) || bits("nu") != Some(nu.to_bits()) || bits("G_pa") != Some(g_pa.to_bits()) {
                return Err("exact material values".into());
            }
        }
        Ok(c["pipe_stress_extrema"].as_array().ok_or("extrema")?)
    }
    /// The preview route's closed preview-physics-1 evidence and the case's entry. Returns the
    /// entry's `pipe_stress_extrema`.
    fn preview_case_evidence<'a>(&self,view:ProductCaseView<'a>)->Result<&'a [serde_json::Value],CaptureError> {
        let e=view.ordinary();
        self.adapter.require()?;
        self.adapter.enter(AdapterEvent::LibraryBoundary, 1); // Closed evidence inspection; serde/number internals remain unqualified.
        let evidence = e.contract_evidence.as_ref().ok_or("preview evidence")?;
        // Existing preview-physics reader's closed namespace; this private scope has no combinations.
        self.adapter.closed_keys(
            evidence,
            &["preview_cases", "combination_gates"],
            "evidence shape",
        )?;
        self.adapter.enter(AdapterEvent::ValidationEntry, 1);
        self.adapter.require()?;
        let gates = evidence["combination_gates"]
            .as_array()
            .ok_or("combination gates")?;
        // B2-P (T-9′; REVISION_01 S-1): shape and consistency only. Exactly one entry per model
        // combination, in authored order, each closed over {combination_id, withheld, reason},
        // naming the model's combination; `withheld: false` with a null reason, or `withheld:
        // true` with one of the three gate codes. A withheld entry is not a case-freeze failure
        // (T-10a gives that combination `base_withheld`). With no combination: no entry, no event.
        if gates.len() != self.combinations.len() {
            return Err("excluded combination gates".into());
        }
        for (gate, combination) in gates.iter().zip(&self.combinations) {
            self.adapter.closed_keys(gate, &["combination_id", "withheld", "reason"], "combination gate shape")?;
            let id = gate["combination_id"].as_str().ok_or("combination gate identity")?;
            if !self.checked_same(id, &combination.id)? {
                return Err("combination gate identity".into());
            }
            match (&gate["withheld"], &gate["reason"]) {
                (serde_json::Value::Bool(false), serde_json::Value::Null) => {}
                (serde_json::Value::Bool(true), serde_json::Value::String(code)) if COMBINATION_GATE_CODES.contains(&code.as_str()) => {}
                _ => return Err("combination gate entry".into()),
            }
        }
        let cases = evidence["preview_cases"]
            .as_array()
            .ok_or("preview cases")?;
        if cases.len() != view.scope.cases {
            return Err("evidence case".into());
        }
        let c = cases.get(view.scope.evidence).ok_or("evidence case")?;
        self.adapter.closed_keys(
            c,
            &[
                "load_case_id",
                "pipe_stress_extrema",
                "stress_maximum_coverage",
                "support_attribution",
                "intensified_measures",
            ],
            "case shape",
        )?;
        let case_id = c["load_case_id"].as_str().ok_or("evidence case")?;
        let case_matches = self.adapter.same(case_id, &self.case_id);
        self.adapter.require()?;
        if !case_matches {
            return Err("evidence case".into());
        }
        self.adapter.closed_keys(
            &c["stress_maximum_coverage"],
            &[
                "complete",
                "unavailable_pipe_ids",
                "outside_domain_pipe_ids",
            ],
            "stress coverage shape",
        )?;
        self.adapter.closed_keys(
            &c["support_attribution"],
            &["attributed_support_ids", "withheld"],
            "support attribution shape",
        )?;
        if c["stress_maximum_coverage"]["complete"] != true
            || c["stress_maximum_coverage"]["unavailable_pipe_ids"]
                .as_array()
                .is_none_or(|v| !v.is_empty())
            || c["stress_maximum_coverage"]["outside_domain_pipe_ids"]
                .as_array()
                .is_none_or(|v| !v.is_empty())
            || c["intensified_measures"]
                .as_array()
                .is_none_or(|v| !v.is_empty())
        {
            return Err("maximum/SIF coverage".into());
        }
        let attributed = c["support_attribution"]["attributed_support_ids"]
            .as_array()
            .ok_or("support attribution")?;
        if attributed.len() != self.supports.len()
            || c["support_attribution"]["withheld"]
                .as_array()
                .is_none_or(|v| !v.is_empty())
        {
            return Err("support attribution coverage".into());
        }
        for (id, _) in &self.supports {
            if attributed
                .iter()
                .filter(|v| v.as_str() == Some(id.as_str()))
                .count()
                != 1
            {
                return Err("support attribution identity".into());
            }
        }
        Ok(c["pipe_stress_extrema"].as_array().ok_or("extrema")?)
    }
    /// The route-independent observables of a case, after its evidence entry: one
    /// `pipe_stress_extrema` record per member bound to its maximum row (midpoint), the support
    /// magnitude guard, and the two headlines (the case's aliases).
    /// The support coverage and guard of one case's (or, B2-P, one combination's) rows: per
    /// model support, exactly six `support_reaction_component_v2` rows, each component once, one
    /// force- and one moment-magnitude row, each magnitude within 64ε relative of hypot(hypot)
    /// of its three components (base G7's guard). Shared, unchanged, by the case observables
    /// and the combination observables stage (REVISION_01 §1.2 item 3).
    fn support_observables(&self,view:ProductCaseView<'_>)->Result<(),CaptureError> {
        for (id, _) in &self.supports {
            self.adapter.enter(AdapterEvent::RowVisit, 1);
            self.adapter.require()?;
            let mut rows=[None;6];let mut count=0usize;
            for row in view.rows().filter(|r| &r.entity_ref==id && r.kind=="support_reaction_component_v2") {
                self.capture_entry(AdapterEvent::RowVisit)?;
                if count<6 {rows[count]=Some(row);}
                count=count.checked_add(1).ok_or(CaptureError::CountRange("support rows"))?;
            }
            if count!=6{return Err("support coverage".into());}
            for (components, kind) in [
                (["Fx", "Fy", "Fz"], "support_reaction_force_magnitude_v2"),
                (["Mx", "My", "Mz"], "support_reaction_moment_magnitude_v2"),
            ] {
                let mut v = [0.0; 3];
                for (i, c) in components.into_iter().enumerate() {
                    let mut found=None;
                    for r in rows.iter().flatten().filter(|r|r.metadata.as_ref().is_some_and(|m|m.component==c)) {
                        self.capture_entry(AdapterEvent::RowVisit)?;
                        if found.is_some(){return Err("support component identity".into());}found=Some(*r.value);
                    }
                    v[i]=found.ok_or("support component identity")?;
                }
                let mut found=None;
                for r in view.rows().filter(|r| &r.entity_ref==id && r.kind==kind) {
                    self.capture_entry(AdapterEvent::RowVisit)?;
                    if found.is_some(){return Err("support magnitude identity".into());}found=Some(*r.value);
                }
                let y=found.ok_or("support magnitude identity")?;
                if (y - v[0].hypot(v[1]).hypot(v[2])).abs()
                    > 64.0 * f64::EPSILON * y.abs().max(f64::MIN_POSITIVE)
                {
                    return Err("support guard".into());
                }
            }
        }
        Ok(())
    }
    fn case_observables(&self,view:ProductCaseView<'_>,extrema:&[serde_json::Value])->Result<(),CaptureError> {
        if extrema.len() != self.members.len() {
            return Err("extrema coverage".into());
        }
        for (index, x) in extrema.iter().enumerate() {
            self.adapter.enter(AdapterEvent::RowVisit, 1);
            self.adapter.enter(AdapterEvent::ValidationEntry, 1);
            self.adapter.require()?;
            const KEYS: [&str; 13] = [
                "pipe_id",
                "result_id",
                "station_fraction",
                "span_index",
                "local_fraction",
                "value_lower_pa",
                "value_upper_pa",
                "global_upper_bound_pa",
                "certified_gap_pa",
                "subdivisions",
                "approximation",
                "coefficient_basis",
                "enclosure_scope",
            ];
            let object = x.as_object().ok_or("extrema object")?;
            if object.len() != KEYS.len() || object.keys().any(|k| !KEYS.contains(&k.as_str())) {
                return Err("extrema shape".into());
            }
            for key in ["station_fraction", "local_fraction"] {
                let n = view.number(index,key,&self.adapter)?.ok_or("extrema fraction")?;
                if !n.is_finite() || !(0.0..=1.0).contains(&n) {
                    return Err("extrema fraction".into());
                }
            }
            for (key, max) in [("span_index", f64::MAX), ("subdivisions", 131072.0)] {
                let n = view.number(index,key,&self.adapter)?.ok_or("extrema integer")?;
                if !n.is_finite() || n < 0.0 || n > max || n.fract() != 0.0 {
                    return Err("extrema integer".into());
                }
            }
            for key in [
                "value_lower_pa",
                "value_upper_pa",
                "global_upper_bound_pa",
                "certified_gap_pa",
            ] {
                if !view.number(index,key,&self.adapter)?.is_some_and(f64::is_finite) {
                    return Err("extrema finite".into());
                }
            }
            if x["approximation"]!="piecewise_quadratic_straight_section_statics"||x["coefficient_basis"]!="j_side_section_equilibrium_binary64"
                ||x["enclosure_scope"]!="supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate" {return Err("extrema scope".into());}
            if extrema[..index]
                .iter()
                .any(|a| a["pipe_id"] == x["pipe_id"] || a["result_id"] == x["result_id"])
            {
                return Err("extrema duplicate".into());
            }
            let id = x["result_id"].as_str().ok_or("maximum ref")?;
            let row = view.rows().find(|r| r.id == id).ok_or("maximum row")?;
            let lo = view.number(index,"value_lower_pa",&self.adapter)?.ok_or("lower")?;
            let hi = view.number(index,"value_upper_pa",&self.adapter)?.ok_or("upper")?;
            if !(lo >= 0.0
                && lo <= *row.value
                && *row.value <= hi
                && *row.value == lo + 0.5 * (hi - lo))
            {
                return Err("maximum midpoint".into());
            }
            if x["pipe_id"] != row.entity_ref || row.kind != "pipe_elastic_normal_stress_maximum_v2"
            {
                return Err("maximum binding".into());
            }
        }
        self.support_observables(view)?;
        for (headline, kind) in [
            (view.headline(false), "displacement_magnitude"),
            (
                view.headline(true),
                "pipe_elastic_normal_stress_maximum_v2",
            ),
        ] {
            let h = headline.ok_or("headline")?;
            let best = view.rows()
                .filter(|r| r.kind == kind)
                .max_by(|a, b| {
                    a.value
                        .total_cmp(&b.value)
                        .then_with(|| b.entity_ref.cmp(&a.entity_ref))
                })
                .ok_or("headline candidate")?;
            if h.result_ref != best.id
                || h.value.to_bits() != best.value.to_bits()
                || h.unit != best.unit
                || h.location_ref != best.entity_ref
            {
                return Err("headline alias".into());
            }
        }
        Ok(())
    }
}

/// B2-P: whose row `validate_final_metadata` checks: a load case's (its id, and whether its
/// ids are case-qualified), or a mechanics combination's (its id).
#[derive(Clone, Copy)]
enum RowSubject<'a> {
    Case { case: &'a str, qualified: bool },
    Combination(&'a str),
}
/// B2-P: a mechanics combination row's metadata basis and sign convention (preview_physics.rs
/// `combination_row`, from `result_metadata_basis` and `result_sign_convention`).
const COMBINATION_METADATA_BASIS: &str = "explicit_user_linear_combination";
const COMBINATION_SIGN_CONVENTION: &str = "positive value follows explicit user linear combination of matching source result sign conventions";

fn validate_final_metadata(
    row: &ResultItem,
    recipe: k::ProductRecipe,
    subject: RowSubject<'_>,
    work: &AdapterWork,
) -> Result<(), CaptureError> {
    work.require()?;
    work.enter(AdapterEvent::LibraryBoundary, 1); // Fixed expected-text construction, with formatting/allocator internals unqualified.

    // A case row has no source references; a combination row names its operands' rows (B2-P).
    let (case, qualified, combination) = match subject {
        RowSubject::Case { case, qualified } => {
            if !row.source_result_refs.is_empty() {
                return Err("unexpected final source references".into());
            }
            (case, qualified, None)
        }
        RowSubject::Combination(id) => {
            if row.source_result_refs.is_empty() {
                return Err("combination source references".into());
            }
            ("", false, Some(id))
        }
    };
    let suffix = stable_suffix(&row.entity_ref);
    let m = row.metadata.as_ref();
    let meta = |component: &str,
                coordinate: &str,
                location: &str,
                basis: &str,
                sign: &str|
     -> Result<(), CaptureError> {
        let m = m.ok_or("missing final metadata")?;
        let (basis, sign) = if combination.is_some() { (COMBINATION_METADATA_BASIS, COMBINATION_SIGN_CONVENTION) } else { (basis, sign) };
        if !work.same(&m.component, component)
            || !work.same(&m.coordinate_system, coordinate)
            || !work.same(&m.location, location)
            || !work.same(&m.basis, basis)
            || !work.same(&m.sign_convention, sign)
        {
            work.require()?;
            return Err(CaptureError::Association(format!(
                "final metadata mismatch: {}",
                row.id
            )));
        }
        Ok(())
    };
    let expected = match recipe {
        k::ProductRecipe::NonQuantity => {
            let m = m.ok_or("mode metadata")?;
            if row.kind != "linear_solver_mode_basis"
                || row.entity_ref != "solver:linear_static_preview"
                || m.component != "linear_solver_mode"
                || m.coordinate_system != "reduced_system"
                || m.location != case
                || !(row.value == 1.0 || row.value == 2.0)
            {
                return Err("ordinary mode record".into());
            }
            work.enter(AdapterEvent::ValidationEntry, 1);
            work.require()?;
            let sign_matches = work.same(
                &m.sign_convention,
                "mode_code 1=sparse_interactive, 2=dense_scrutiny, 3=dense_fallback_after_sparse_failure",
            );
            work.require()?;
            if !sign_matches {
                return Err("ordinary sparse mode sign".into());
            }
            "result:solver-mode:linear-solve-basis".to_string()
        }
        k::ProductRecipe::DenseParityObservation => {
            let m=m.ok_or("parity metadata")?;
            meta("sparse_live_path","reduced_system","load_case",&m.basis,
                "unitless max absolute dense-sparse solution delta divided by max dense solution magnitude; no release threshold asserted")?;
            "result:sparse-live:dense-parity-relative-delta".to_string()
        }
        k::ProductRecipe::Native(k::QuantityId::DisplacementMagnitude(_)) => {
            if m.is_some() {
                return Err("magnitude metadata".into());
            }
            format!("result:disp:{suffix}")
        }
        k::ProductRecipe::Native(k::QuantityId::Displacement(d)) => {
            let i = d.component.index();
            let axis = ["X", "Y", "Z"][i % 3];
            let component = [
                "nodal_displacement_x",
                "nodal_displacement_y",
                "nodal_displacement_z",
                "nodal_rotation_x",
                "nodal_rotation_y",
                "nodal_rotation_z",
            ][i];
            let sign = if i < 3 {
                format!("positive value follows the global cartesian {axis} axis displacement of the node")
            } else {
                format!("positive value follows the right-hand-rule rotation about the global cartesian {axis} axis")
            };
            meta(
                component,
                "global",
                "node",
                "solved_from_global_linear_system",
                &sign,
            )?;
            format!(
                "result:disp:{suffix}:{}",
                ["ux", "uy", "uz", "rx", "ry", "rz"][i]
            )
        }
        k::ProductRecipe::Native(k::QuantityId::EndAction { end, component, .. }) => {
            let i = component.index();
            let loc = if end == k::End::I { "end_i" } else { "end_j" };
            let sign = if end == k::End::I {
                "positive value follows the element-local DOF at the i-end force vector"
            } else {
                "positive value follows the element-local DOF at the j-end force vector"
            };
            meta(
                [
                    "axial_force",
                    "shear_force_y",
                    "shear_force_z",
                    "torsional_moment",
                    "bending_moment_y",
                    "bending_moment_z",
                ][i],
                "element_local",
                loc,
                SECTION_RESULTANT_BASIS,
                sign,
            )?;
            format!(
                "result:{}:{suffix}:{}{}",
                if i < 3 { "force" } else { "moment" },
                [
                    "axial",
                    "shear-y",
                    "shear-z",
                    "torsion",
                    "bending-y",
                    "bending-z"
                ][i],
                if end == k::End::J { ":end-j" } else { "" }
            )
        }
        k::ProductRecipe::Native(k::QuantityId::StationAction { component, .. }) => {
            let i = component.index();
            let loc = m.ok_or("station metadata")?.location.as_str();
            meta(["axial_force","shear_force_y","shear_force_z","torsional_moment","bending_moment_y","bending_moment_z"][i],
                "element_local",loc,SECTION_RESULTANT_BASIS,"positive value follows the j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction")?;
            format!(
                "result:{}:{suffix}:{}:{}",
                if i < 3 { "force" } else { "moment" },
                station_id_location(loc),
                [
                    "axial",
                    "shear-y",
                    "shear-z",
                    "torsion",
                    "bending-y",
                    "bending-z"
                ][i]
            )
        }
        k::ProductRecipe::SupportComponent { .. } | k::ProductRecipe::Native(
            k::QuantityId::SupportForceMagnitude(_)
            | k::QuantityId::SupportMomentMagnitude(_),
        ) => {
            let component = match recipe {
                k::ProductRecipe::SupportComponent { component, .. } => {
                    ["Fx", "Fy", "Fz", "Mx", "My", "Mz"][component.index()]
                }
                k::ProductRecipe::Native(k::QuantityId::SupportForceMagnitude(_)) => {
                    "force_magnitude"
                }
                k::ProductRecipe::Native(k::QuantityId::SupportMomentMagnitude(_)) => {
                    "moment_magnitude"
                }
                _ => unreachable!("the enclosing arm admits only these support recipes"),
            };
            meta(component,"global","node","recovered_from_assembled_support_law",
                "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate")?;
            match combination {
                // B2-P: a combination's support rows are not case-qualified (`append_combination_results`).
                Some(_) => format!("result:support-action:{}:{}:{}", row.entity_ref.len(), row.entity_ref, component),
                None => format!(
                    "result:support-action:{}:{}:{}:{}:{}",
                    case.len(),
                    case,
                    row.entity_ref.len(),
                    row.entity_ref,
                    component
                ),
            }
        }
        k::ProductRecipe::Stress { site, stress, .. } => {
            let (component, tail) = match stress {
                k::ProductStress::Axial => ("axial_normal_stress", "axial-normal"),
                k::ProductStress::BendingY => ("bending_normal_stress_y", "bending-normal-y"),
                k::ProductStress::BendingZ => ("bending_normal_stress_z", "bending-normal-z"),
                k::ProductStress::Torsion => ("torsional_shear_stress", "torsional-shear"),
            };
            let loc = m.ok_or("stress metadata")?.location.as_str();
            let (idloc, basis, sign) = if matches!(site, k::ProductSite::End(_)) {
                (
                    endpoint_id_location(loc),
                    SECTION_RESULTANT_BASIS,
                    STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION.to_string(),
                )
            } else {
                let prefix=match stress {
                    k::ProductStress::Axial=>"positive normal stress follows the section-equilibrium element-local axial resultant at this station",
                    k::ProductStress::BendingY=>"positive bending normal stress follows the section-equilibrium element-local y bending resultant at this station",
                    k::ProductStress::BendingZ=>"positive bending normal stress follows the section-equilibrium element-local z bending resultant at this station",
                    k::ProductStress::Torsion=>"positive torsional shear stress follows the section-equilibrium element-local torsional resultant at this station",
                };
                (station_id_location(loc),"recovered_from_open_mechanics_stress_components",format!("{prefix}; j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction"))
            };
            meta(component, "element_local", loc, basis, &sign)?;
            format!("result:stress:{suffix}:{idloc}:{tail}")
        }
        k::ProductRecipe::CircularMaximum { .. } => {
            meta("maximum_absolute_normal_stress","pipe_section","governing_station","recovered_from_open_mechanics_stress_components",
                "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim")?;
            format!(
                "result:elastic-maximum:{}:{}:{}:{}",
                case.len(),
                case,
                row.entity_ref.len(),
                row.entity_ref
            )
        }
        _ => return Err("unsupported metadata recipe".into()),
    };
    // B1 SP (T-9): a case after the first carries the envelope's case-qualified id, except
    // for the `_v2` kinds, whose ids already name the case (lib.rs, the preview's rows).
    let expected = if let Some(id) = combination {
        // B2-P: `qualified_combination_result_id` over the row's base id.
        work.enter(AdapterEvent::LibraryBoundary, 1);
        format!("result:combination:{}:{}", stable_suffix(id), super::result_tail(&expected))
    } else if qualified && !row.kind.ends_with("_v2") {
        work.enter(AdapterEvent::LibraryBoundary, 1);
        qualified_load_case_result_id(case, &expected)
    } else {
        expected
    };
    work.require()?;
    if !work.same(&row.id, &expected) {
        work.require()?;
        return Err(CaptureError::Association(format!(
            "final id association: {}",
            row.id
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScalarOperation {
    Add,
    Sub,
    Mul,
    Div,
    Sqrt,
}
#[derive(Debug, Clone, PartialEq)]
pub(super) enum OperationalError {
    MissingOrForeign,
    Input,
    Degenerate,
    NonFinite {
        operation: ScalarOperation,
        entered: u64,
    },
    CoefficientRange {
        coefficient: &'static str,
        operation: ScalarOperation,
    },
    Accounting,
}
#[derive(Debug, Default)]
pub(super) struct ScalarWork {
    pub entered: u64,
    pub checks: u64,
    pub lost: bool,
}
impl ScalarWork {
    fn check(&mut self) -> Result<(), OperationalError> {
        if self.lost {
            return Err(OperationalError::Accounting);
        }
        match self.checks.checked_add(1) {
            Some(n) => self.checks = n,
            None => {
                self.lost = true;
                return Err(OperationalError::Accounting);
            }
        }
        if self.lost {
            Err(OperationalError::Accounting)
        } else {
            Ok(())
        }
    }
    pub(super) fn op(
        &mut self,
        kind: ScalarOperation,
        a: f64,
        b: f64,
    ) -> Result<f64, OperationalError> {
        self.operation(kind, a, b, None)
    }
    fn operation(
        &mut self,
        kind: ScalarOperation,
        a: f64,
        b: f64,
        normal: Option<&'static str>,
    ) -> Result<f64, OperationalError> {
        if self.lost {
            return Err(OperationalError::Accounting);
        }
        let Some(n) = self.entered.checked_add(1) else {
            self.lost = true;
            return Err(OperationalError::Accounting);
        };
        self.entered = n;
        let value = match kind {
            ScalarOperation::Add => a + b,
            ScalarOperation::Sub => a - b,
            ScalarOperation::Mul => a * b,
            ScalarOperation::Div => a / b,
            ScalarOperation::Sqrt => a.sqrt(),
        };
        // Numeric result is formed before collection. Collection loss cannot erase it.
        let numeric = if let Some(coefficient) = normal {
            if !value.is_normal() {
                Err(OperationalError::CoefficientRange {
                    coefficient,
                    operation: kind,
                })
            } else {
                Ok(value)
            }
        } else if !value.is_finite() {
            Err(OperationalError::NonFinite {
                operation: kind,
                entered: n,
            })
        } else {
            Ok(value)
        };
        let collected = self.check().and_then(|()| {
            if normal.is_some() {
                self.check()
            } else {
                Ok(())
            }
        });
        match numeric {
            Err(e) => Err(e),
            Ok(v) => collected.map(|()| v),
        }
    }
    fn norm(&mut self, d: [f64; 3]) -> Result<f64, OperationalError> {
        let x = self.op(ScalarOperation::Mul, d[0], d[0])?;
        let y = self.op(ScalarOperation::Mul, d[1], d[1])?;
        let xy = self.op(ScalarOperation::Add, x, y)?;
        let z = self.op(ScalarOperation::Mul, d[2], d[2])?;
        let xyz = self.op(ScalarOperation::Add, xy, z)?;
        self.op(ScalarOperation::Sqrt, xyz, 0.0)
    }
    pub(super) fn coefficient(
        &mut self,
        a: f64,
        b: f64,
        l: f64,
        name: &'static str,
    ) -> Result<f64, OperationalError> {
        let p = self.operation(ScalarOperation::Mul, a, b, Some(name))?;
        self.operation(ScalarOperation::Div, p, l, Some(name))
    }
}
#[derive(Debug, Clone, Copy)]
pub(super) struct OperationalOperands {
    pub length: f64,
    pub axial: f64,
    pub torsion: f64,
    pub normalization_check: [f64; 3],
}
#[derive(Debug)]
pub(super) struct OperationalSpent {
    pub member:Option<u32>,
    pub inputs: [u64; 10],
    pub result: Result<OperationalOperands, OperationalError>,
    pub work: ScalarWork,
}
fn evaluate_member_operational(member:u32,nodes:[[f64;3];2],properties:[f64;4])->OperationalSpent {
    let mut spent=evaluate_operational(nodes,properties);spent.member=Some(member);spent
}
/// Newly evaluated operational expressions, never historical formation capture.
pub(super) fn evaluate_operational(nodes: [[f64; 3]; 2], properties: [f64; 4]) -> OperationalSpent {
    let [e, g, a, j] = properties;
    let input = [
        nodes[0][0],
        nodes[0][1],
        nodes[0][2],
        nodes[1][0],
        nodes[1][1],
        nodes[1][2],
        e,
        g,
        a,
        j,
    ];
    let mut work = ScalarWork::default();
    let result = (|| {
        for (i, &v) in input.iter().enumerate() {
            work.check()?;
            if !v.is_finite() || (i >= 6 && v <= 0.0) {
                return Err(OperationalError::Input);
            }
        }
        let mut d = [0.0; 3];
        for i in 0..3 {
            d[i] = work.op(ScalarOperation::Sub, nodes[1][i], nodes[0][i])?;
        }
        let magnitude = work.norm(d)?;
        work.check()?;
        if magnitude <= 1.0e-12 {
            return Err(OperationalError::Degenerate);
        }
        let inverse = work.op(ScalarOperation::Div, 1.0, magnitude)?;
        let mut normalization_check = [0.0; 3];
        for i in 0..3 {
            normalization_check[i] = work.op(ScalarOperation::Mul, d[i], inverse)?;
        }
        let length = work.norm(d)?;
        work.check()?;
        if length <= 0.0 {
            return Err(OperationalError::Input);
        }
        let axial = work.coefficient(e, a, length, "EA/L")?;
        let torsion = work.coefficient(g, j, length, "GJ/L")?;
        Ok(OperationalOperands {
            length,
            axial,
            torsion,
            normalization_check,
        })
    })();
    OperationalSpent {
        member:None,
        inputs: input.map(f64::to_bits),
        result,
        work,
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(super) enum G5aFailure {
    Accounting(AdapterFault),
    Shape(&'static str),
    Summary(&'static str),
    Zero {
        row: usize,
    },
    Sanity {
        body: u32,
        kind: usize,
    },
    Lower {
        member: u32,
        kind: usize,
    },
    Operational {
        member: usize,
        cause: OperationalError,
    },
    Arithmetic(OperationalError),
}
impl From<OperationalError> for G5aFailure {
    fn from(e: OperationalError) -> Self {
        Self::Arithmetic(e)
    }
}
impl ProductCapture {
    pub(super) fn full_case_passed(&self) -> bool {
        self.error.is_none()
            && self.adapter.fault.get().is_none()
            && self.final_calls == 1
            && self.numeric_pass
            && self.observable_error.is_none()
            && self.g5a_error.is_none()
            && !self.g5a_work.lost
            && self
                .operational
                .iter()
                .all(|o| !o.work.lost && o.result.is_ok())
    }
    pub(super) fn g5a(
        &mut self,
        owner: &k::RetainedSolve,
        rows: &[k::ProductFinalRow<'_>],
    ) -> Result<(), G5aFailure> {
        if let Some(fault) = self.adapter.fault.get() { return Err(G5aFailure::Accounting(fault)); }
        let source = owner.source();
        let evidence = owner.evidence();
        let nb = source.body_count() as usize;
        let nonnegative = |v: f64| v.is_finite() && v >= 0.0 && (v != 0.0 || v.to_bits() == 0);
        if self.verdicts.len() != rows.len() || self.operational.len() != self.facts.len() {
            return Err(G5aFailure::Shape("final/operational rows"));
        }
        validate_summary_shape(evidence, &self.summary_coverage, nb)?;
        // B presence/data ownership is also checked by the same producing SourceBridgeView.
        for b in 0..nb {
            let rs: Vec<_> = evidence
                .resolution_scale
                .iter()
                .filter(|v| v.0 == b as u32)
                .collect();
            if rs.len() != 1 {
                return Err(G5aFailure::Shape("resolution coverage"));
            }
            let resolution = [f64::from_bits(rs[0].1), f64::from_bits(rs[0].2)];
            if resolution.iter().any(|v| !nonnegative(*v)) {
                return Err(G5aFailure::Summary("resolution"));
            }
            let mut scales = [0.0f64; 4];
            let mut bounds: Option<([f64; 3], [f64; 3])> = None;
            for (i, node) in source.nodes().iter().enumerate() {
                if source.body_of_node(i as u32) != b as u32 {
                    continue;
                }
                let (mut lo, mut hi) = bounds.unwrap_or((*node, *node));
                for j in 0..3 {
                    lo[j] = lo[j].min(node[j]);
                    hi[j] = hi[j].max(node[j]);
                }
                bounds = Some((lo, hi));
            }
            let (lo, hi) = bounds.ok_or(G5aFailure::Shape("body nodes"))?;
            let mut delta = [0.0; 3];
            for j in 0..3 {
                delta[j] = self.g5a_work.op(ScalarOperation::Sub, hi[j], lo[j])?;
            }
            let extent = self.g5a_work.norm(delta)?;
            for (i, row) in rows.iter().enumerate() {
                if row.body != b as u32 {
                    continue;
                }
                let classified = match row.recipe {
                    k::ProductRecipe::Native(id) => {
                        let meta = owner.publish().rows.iter().find(|r| r.id == id)
                            .ok_or(G5aFailure::Shape("native row"))?;
                        let kind = match meta.kind { k::Kind::Translation => 0, k::Kind::Rotation => 1,
                            k::Kind::Force => 2, k::Kind::Moment => 3 };
                        Some((kind, meta.class == k::RowClass::InputDerived))
                    }
                    k::ProductRecipe::SupportComponent { support, component } => {
                        let mut group = None;
                        for g in source.supports() {
                            if !self.adapter.enter(AdapterEvent::RowVisit, 1)
                                || !self.adapter.enter(AdapterEvent::ValidationEntry, 1) {
                                return Err(G5aFailure::Accounting(self.adapter.fault.get().unwrap()));
                            }
                            if g.id == support { group = Some(g); break; }
                        }
                        let group = group.ok_or(G5aFailure::Shape("support group"))?;
                        if source.body_of_node(group.node) != row.body {
                            return Err(G5aFailure::Shape("support body"));
                        }
                        Some((if component.index() < 3 { 2 } else { 3 }, false))
                    }
                    _ => None,
                };
                if let Some((kind, input)) = classified {
                    let n = f64::from_bits(self.verdicts[i].normalized_bits);
                    if kind >= 2 && resolution[kind - 2] == 0.0 && n.to_bits() != 0 {
                        return Err(G5aFailure::Zero { row: i });
                    }
                    if !input { scales[kind] = scales[kind].max(n.abs()); }
                }
            }
            if extent != 0.0 {
                let [tr, ro, fo, mo] = scales;
                scales = [
                    tr.max(self.g5a_work.op(ScalarOperation::Mul, extent, ro)?),
                    ro.max(self.g5a_work.op(ScalarOperation::Div, tr, extent)?),
                    fo.max(self.g5a_work.op(ScalarOperation::Div, mo, extent)?),
                    mo.max(self.g5a_work.op(ScalarOperation::Mul, extent, fo)?),
                ];
            }
            let hats = if extent == 0.0 {
                resolution
            } else {
                [
                    resolution[0].max(self.g5a_work.op(
                        ScalarOperation::Div,
                        resolution[1],
                        extent,
                    )?),
                    resolution[1].max(self.g5a_work.op(
                        ScalarOperation::Mul,
                        extent,
                        resolution[0],
                    )?),
                ]
            };
            let mut upper = [0.0; 2];
            for j in 0..2 {
                upper[j] = self.g5a_work.op(
                    ScalarOperation::Mul,
                    hats[j],
                    f64::from_bits(0x3ff0000000001000),
                )?;
                if upper[j] < scales[j + 2] {
                    return Err(G5aFailure::Sanity {
                        body: b as u32,
                        kind: j,
                    });
                }
            }
            for (mi, m) in source.members().iter().enumerate() {
                if source.body_of_node(m.node_i) != b as u32 {
                    continue;
                }
                let op = &self.operational[mi];
                let [a, c] = [
                    source.nodes()[m.node_i as usize],
                    source.nodes()[m.node_j as usize],
                ];
                let inputs = [
                    a[0],
                    a[1],
                    a[2],
                    c[0],
                    c[1],
                    c[2],
                    m.elastic_modulus,
                    m.shear_modulus,
                    m.area,
                    m.torsion_constant,
                ]
                .map(f64::to_bits);
                if op.inputs != inputs {
                    return Err(G5aFailure::Operational {
                        member: mi,
                        cause: OperationalError::MissingOrForeign,
                    });
                }
                if op.work.lost {
                    return Err(G5aFailure::Operational {
                        member: mi,
                        cause: OperationalError::Accounting,
                    });
                }
                let values = op
                    .result
                    .as_ref()
                    .map_err(|cause| G5aFailure::Operational {
                        member: mi,
                        cause: cause.clone(),
                    })?;
                let mut endpoint_norms = [[0.0; 2]; 2];
                for (end, node) in [m.node_i, m.node_j].into_iter().enumerate() {
                    for kind in 0..2 {
                        let mut x = [0.0; 3];
                        for (j, v) in x.iter_mut().enumerate() {
                            let id = k::QuantityId::Displacement(k::Dof {
                                node,
                                component: k::Component::ALL[kind * 3 + j],
                            });
                            let matches: Vec<_> = rows
                                .iter()
                                .enumerate()
                                .filter(|(_, r)| r.recipe == k::ProductRecipe::Native(id))
                                .collect();
                            if matches.len() != 1 {
                                return Err(G5aFailure::Shape(
                                    "all nodal components including input-derived",
                                ));
                            }
                            *v = f64::from_bits(self.verdicts[matches[0].0].normalized_bits).abs();
                        }
                        let xy = self.g5a_work.op(ScalarOperation::Add, x[0], x[1])?;
                        let n = self.g5a_work.op(ScalarOperation::Add, xy, x[2])?;
                        endpoint_norms[end][kind] = n;
                    }
                }
                let norm = [
                    self.g5a_work.op(
                        ScalarOperation::Add,
                        endpoint_norms[0][0],
                        endpoint_norms[1][0],
                    )?,
                    self.g5a_work.op(
                        ScalarOperation::Add,
                        endpoint_norms[0][1],
                        endpoint_norms[1][1],
                    )?,
                ];
                for kind in 0..2 {
                    let threshold = self.g5a_work.op(
                        ScalarOperation::Mul,
                        f64::from_bits((1023u64 - 59) << 52),
                        scales[kind],
                    )?;
                    let lower = if norm[kind] <= threshold {
                        0.0
                    } else {
                        let margin = self.g5a_work.op(
                            ScalarOperation::Mul,
                            f64::from_bits((1023u64 - 60) << 52),
                            scales[kind],
                        )?;
                        let net = self.g5a_work.op(ScalarOperation::Sub, norm[kind], margin)?;
                        self.g5a_work.op(
                            ScalarOperation::Mul,
                            if kind == 0 {
                                values.axial
                            } else {
                                values.torsion
                            },
                            net,
                        )?
                    };
                    if upper[kind] < lower {
                        return Err(G5aFailure::Lower { member: m.id, kind });
                    }
                }
            }
        }
        Ok(())
    }
}

pub(super) fn validate_summary_shape(
    e: &k::RetainedEvidence,
    coverage: &[k::ProductSummaryCoverage],
    bodies: usize,
) -> Result<(), G5aFailure> {
    let nonnegative = |v: f64| v.is_finite() && v >= 0.0 && (v != 0.0 || v.to_bits() == 0);
    if coverage.len() != bodies || e.resolution_scale.len() != bodies || e.theta.len() != bodies {
        return Err(G5aFailure::Shape("body summaries"));
    }
    for (i, c) in coverage.iter().enumerate() {
        if c.body as usize != i {
            return Err(G5aFailure::Shape("coverage owner"));
        }
    }
    for (entries, bound, label) in [
        (&e.stop_rule, f64::from_bits((1023u64 - 64) << 52), "stop"),
        (&e.verification_estimate, 0.25, "estimate"),
        (&e.verification_charge, 1.0, "charge"),
    ] {
        for &(b, kind, value) in entries {
            if b as usize >= bodies
                || (!matches!(kind, k::Kind::Force | k::Kind::Moment) && label != "stop")
            {
                return Err(G5aFailure::Shape(label));
            }
            if !nonnegative(value) || value > bound {
                return Err(G5aFailure::Summary(label));
            }
        }
        for c in coverage {
            for (i, kind) in [
                k::Kind::Translation,
                k::Kind::Rotation,
                k::Kind::Force,
                k::Kind::Moment,
            ]
            .into_iter()
            .enumerate()
            {
                let wanted = if label == "stop" {
                    c.stop[i]
                } else if i < 2 {
                    false
                } else if label == "estimate" {
                    c.estimate[i - 2]
                } else {
                    c.charge[i - 2]
                };
                if entries
                    .iter()
                    .filter(|v| v.0 == c.body && v.1 == kind)
                    .count()
                    != usize::from(wanted)
                {
                    return Err(G5aFailure::Shape(label));
                }
            }
        }
    }
    for c in coverage {
        let mut theta=e.theta.iter().filter(|v|v.0==c.body);
        let first=theta.next();
        if first.is_none() || theta.next().is_some() || e.resolution_scale.iter().filter(|v|v.0==c.body).count()!=1 {
            return Err(G5aFailure::Shape("theta/resolution"));
        }
        if !nonnegative(first.unwrap().1) || first.unwrap().1>0.5{return Err(G5aFailure::Summary("theta"));}
        if e.certified_bound.iter().filter(|v|v.0==c.body).count()!=usize::from(c.has_data) {
            return Err(G5aFailure::Shape("B data coverage"));
        }
        for b in e.certified_bound.iter().filter(|v|v.0==c.body) {
            let value = f64::from_bits(b.1);
            if !value.is_finite() || value <= 0.0 {
                return Err(G5aFailure::Summary("B"));
            }
        }
    }
    if e.certified_bound.iter().any(|v| v.0 as usize >= bodies) {
        return Err(G5aFailure::Shape("B body"));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdapterEvent {
    SourceVisit = 0,
    RowVisit = 1,
    MapWrite = 2,
    ValidationEntry = 3,
    IdentityByteRead = 4,
    KeyProbe = 5,
    AllocationRequest = 6,
    LibraryBoundary = 7,
    RequestedCopyBytes = 8,
    RustCapacityBytes = 9,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdapterFault {
    Overflow(AdapterEvent),
}
#[derive(Debug, Default)]
pub(super) struct AdapterWork {
    pub counts: std::cell::Cell<[u64; 10]>,
    pub fault: std::cell::Cell<Option<AdapterFault>>,
}
impl AdapterWork {
    /// Closed borrowed key decoder. Charge each entered group, key visit and actual comparison.
    fn closed_keys(
        &self,
        value: &serde_json::Value,
        expected: &[&str],
        reason: &'static str,
    ) -> Result<(), CaptureError> {
        self.enter(AdapterEvent::ValidationEntry, 1);
        self.enter(AdapterEvent::LibraryBoundary, 1);
        self.require()?;
        let object = value.as_object().ok_or(reason)?;
        if object.len() != expected.len() {
            return Err(reason.into());
        }
        for key in object.keys() {
            self.enter(AdapterEvent::RowVisit, 1);
            self.require()?;
            let mut matched = false;
            for allowed in expected {
                let equal = self.same(key, allowed);
                self.require()?;
                if equal {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return Err(reason.into());
            }
        }
        Ok(())
    }
    fn enter(&self, event: AdapterEvent, amount: u64) -> bool {
        if self.fault.get().is_some() {
            return false;
        }
        let mut counts = self.counts.get();
        let Some(value) = counts[event as usize].checked_add(amount) else {
            self.fault.set(Some(AdapterFault::Overflow(event)));
            return false;
        };
        counts[event as usize] = value;
        self.counts.set(counts);
        true
    }
    fn require(&self) -> Result<(), CaptureError> {
        self.fault
            .get()
            .map_or(Ok(()), |f| Err(CaptureError::Accounting(f)))
    }
    /// Actual explicit comparison reads, stopping at the first mismatch.
    pub(super) fn same(&self, a: &str, b: &str) -> bool {
        if !self.enter(AdapterEvent::KeyProbe, 1) || !self.enter(AdapterEvent::ValidationEntry, 1) {
            return false;
        }
        if a.len() != b.len() {
            return false;
        }
        for i in 0..a.len() {
            if !self.enter(AdapterEvent::IdentityByteRead, 2) {
                return false;
            }
            if a.as_bytes()[i] != b.as_bytes()[i] {
                return false;
            }
        }
        true
    }
    pub(super) fn reserve<T>(&self, n: usize) -> Result<Vec<T>, CaptureError> {
        let layout = std::alloc::Layout::array::<T>(n)
            .map_err(|_| CaptureError::CountRange("adapter layout"))?;
        let _requested_layout = layout;
        if !self.enter(AdapterEvent::AllocationRequest, 1) {
            return Err(CaptureError::Accounting(self.fault.get().unwrap()));
        }
        let mut out = Vec::new();
        out.try_reserve_exact(n)
            .map_err(|_| CaptureError::Storage("adapter vector"))?;
        let bytes = out
            .capacity()
            .checked_mul(std::mem::size_of::<T>())
            .ok_or(CaptureError::CountRange("adapter capacity bytes"))?;
        let size = u64::try_from(bytes)
            .map_err(|_| CaptureError::CountRange("adapter capacity conversion"))?;
        if !self.enter(AdapterEvent::RustCapacityBytes, size) {
            return Err(CaptureError::Accounting(self.fault.get().unwrap()));
        }
        Ok(out)
    }
    fn copy(&self, s: &str) -> Result<String, CaptureError> {
        let n = u64::try_from(s.len()).map_err(|_| CaptureError::CountRange("identity bytes"))?;
        if !self.enter(AdapterEvent::AllocationRequest, 1)
            || !self.enter(AdapterEvent::RequestedCopyBytes, n)
        {
            return Err(CaptureError::Accounting(self.fault.get().unwrap()));
        }
        let mut out = String::new();
        out.try_reserve_exact(s.len())
            .map_err(|_| CaptureError::Storage("identity copy"))?;
        out.push_str(s);
        if !self.enter(AdapterEvent::RustCapacityBytes, out.capacity() as u64) {
            return Err(CaptureError::Accounting(self.fault.get().unwrap()));
        }
        Ok(out)
    }
    fn push_room<T>(&self, v: &mut Vec<T>) -> Result<(), CaptureError> {
        if v.len() == v.capacity() {
            if !self.enter(AdapterEvent::AllocationRequest, 1) {
                return Err(CaptureError::Accounting(self.fault.get().unwrap()));
            }
            v.try_reserve_exact(1)
                .map_err(|_| CaptureError::Storage("adapter growth"))?;
            let bytes = v
                .capacity()
                .checked_mul(std::mem::size_of::<T>())
                .ok_or(CaptureError::CountRange("adapter growth bytes"))?;
            if !self.enter(AdapterEvent::RustCapacityBytes, bytes as u64) {
                return Err(CaptureError::Accounting(self.fault.get().unwrap()));
            }
        }
        Ok(())
    }
}

impl From<&str> for CaptureError {
    fn from(s: &str) -> Self {
        Self::Association(s.to_string())
    }
}
impl From<String> for CaptureError {
    fn from(s: String) -> Self {
        Self::Association(s)
    }
}
impl std::fmt::Display for CaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Association(s) => f.write_str(s),
            _ => write!(f, "{self:?}"),
        }
    }
}

enum SelectorRef<'a> {
    Point(&'a str),
    Temperature(u64),
}
fn load_case_selector(case: &PreviewLoadCase) -> Option<SelectorRef<'_>> {
    if let Some(id) = case.modulus_basis_ref.as_deref() {
        Some(SelectorRef::Point(id))
    } else {
        case.modulus_basis_temperature
            .as_ref()
            .map(|t| SelectorRef::Temperature(t.value.to_bits()))
    }
}


// Private prepared-only custody. The actual ordinary inputs remain borrowed at
// the granted late callsite; no DeferredCaseInputs clone graph or bypass latch.
impl ProductCapture {
    fn checked_same(&self,a:&str,b:&str)->Result<bool,CaptureError> {
        let same=self.adapter.same(a,b);self.adapter.require()?;Ok(same)
    }
    fn prepared_case_seen(&mut self,model:&PreviewModel,case:&PreviewLoadCase)->Result<(),CaptureError> {
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        // B1 SP (T-2): a later requested case parks the earlier case first (never at c = 1).
        if self.prepared_one_case_seen && self.parked.len()+1<model.load_cases.len() {self.park_case(model.load_cases.len())?;}
        let index=self.parked.len();
        // B2-P (T-2′): combinations as D1.4 admits them (`w1_model_combinations_admitted`).
        if self.case_calls!=0 || self.prepared_one_case_seen || index>=model.load_cases.len() || !super::w1_model_combinations_admitted(model) {
            return Err("prepared case/no-combination source scope".into());
        }
        if !self.checked_same(&model.load_cases[index].id,&case.id)? {return Err("prepared early case identity".into());}
        let id=self.adapter.copy(&case.id)?;
        self.capture_entry(AdapterEvent::MapWrite)?;self.case_id=id;
        self.capture_entry(AdapterEvent::MapWrite)?;self.case_calls=1;
        self.capture_entry(AdapterEvent::MapWrite)?;self.prepared_one_case_seen=true;
        Ok(())
    }
    fn prepared_observation_custody(&self,case:&str)->Result<(),CaptureError> {
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        let observed=self.observations.as_ref().ok_or("missing successful solver observations")?;
        if self.invocation_calls!=1 || self.normalized_calls!=1 || self.observation_calls!=1
            || self.invocation_mode!=Some(observed.mode) || self.case_calls!=1
            || observed.parity_produced!=observed.parity.is_some()
            || (observed.mode==PreviewSolverMode::SparseInteractive && observed.parity_produced)
            || observed.mode_row.value_bits!=observed.mode.mode_code().to_bits() {
            return Err("prepared observation ownership/presence".into());
        }
        if !self.checked_same(&observed.case,case)? || !self.checked_same(&self.case_id,case)? {
            return Err("prepared observation case".into());
        }
        if let Some(parity)=&observed.parity {
            self.capture_entry(AdapterEvent::ValidationEntry)?;
            let v=f64::from_bits(parity.value_bits);
            if !v.is_finite() || v<0.0 {return Err("prepared observation parity value".into());}
        }
        Ok(())
    }
    pub(super) fn prepared_case_source(&mut self,source_selected:bool,model:&PreviewModel,
        built:&BuiltModel,materials:&[MaterialInput],case:&PreviewLoadCase,restrained:&[usize],
        springs:&[SpringEntry],application:&LoadApplication,thermal:&[ThermalElementLoad],pressure:&[PressureThrustLoad]) {
        // B1 (RV112 N-4): after a G-B refusal, a later case's late hook does nothing: the first
        // refusal stands, and no later G-B check, running total or late capture follows.
        if !self.prepared_probe || self.error.is_some() || self.late_refusal.is_some() {return;}
        let checked=(||->Result<(),CaptureError>{
            self.adapter.require()?;
            let count=self.prepared_late_calls.checked_add(1).ok_or(CaptureError::CountRange("prepared late hooks"))?;
            self.capture_entry(AdapterEvent::MapWrite)?;self.prepared_late_calls=count;
            self.capture_entry(AdapterEvent::ValidationEntry)?;
            // B1 SP (T-2): this case is request case `parked.len()`; its own counters.
            if count!=1 || !self.prepared_one_case_seen || self.case_calls!=1 || self.parked.len()>=model.load_cases.len()
                || !super::w1_model_combinations_admitted(model) {return Err("prepared late hook scope/presence".into());}
            if !self.checked_same(&model.load_cases[self.parked.len()].id,&case.id)? || !self.checked_same(&self.case_id,&case.id)? {
                return Err("prepared late case identity".into());
            }
            self.prepared_observation_custody(&case.id)?;
            if source_selected {return Err("exact-block selected: prepared source suppressed".into());}
            self.capture_entry(AdapterEvent::MapWrite)?;self.prepared_source_permit=true;
            Ok(())
        })();
        if let Err(e)=checked {self.error=Some(e);return;}
        // U3, G-B (I51 COMPOSITION §2): immediately before the late old-source
        // capture. A refusal skips the capture; the ordinary solve is unaffected.
        if let Some(permit)=self.permit.as_ref() { #[cfg(test)] crate::retained_tests_hooks::before_late_gate(&*self);
            // B1 seam: G-B's running load total, with no adapter event. It saturates (RR "I89's SA
            // verified and ruled…", ruling 2), recording no capture error: G-B then refuses typed on
            // CaseLoadsTotal, with exact bytes and no notice, as for any G-B refusal.
            self.late_loads_total = self.late_loads_total.saturating_add(case.primitive_loads.len());
            let facts=super::retained_memory::LateFacts{model,built,materials,case,restrained,springs,capture:&*self};
            if let Err(refusal)=permit.check_late(&facts) {self.late_refusal=Some(refusal);return;}
        }
        self.capture_case_source(model,built,materials,case,restrained,springs,application,thermal,pressure);
    }
}
fn prepared_reserve<T>(adapter:&AdapterWork,capacities:&mut [usize;16],slot:usize,n:usize)->Result<Vec<T>,CaptureError> {
    std::alloc::Layout::array::<T>(n).map_err(|_|CaptureError::CountRange("prepared layout"))?;
    adapter.enter(AdapterEvent::AllocationRequest,1);adapter.require()?;
    let mut v=Vec::new();v.try_reserve_exact(n).map_err(|_|CaptureError::Storage("prepared vector"))?;
    let bytes=v.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(CaptureError::CountRange("prepared capacity"))?;
    capacities[slot]=capacities[slot].checked_add(bytes).ok_or(CaptureError::CountRange("prepared capacity sum"))?;
    adapter.enter(AdapterEvent::RustCapacityBytes,u64::try_from(bytes).map_err(|_|CaptureError::CountRange("prepared capacity conversion"))?);
    adapter.require()?;Ok(v)
}
fn prepared_string(adapter:&AdapterWork,capacities:&mut [usize;16],text:&str)->Result<String,CaptureError> {
    let n=u64::try_from(text.len()).map_err(|_|CaptureError::CountRange("prepared string length"))?;
    adapter.enter(AdapterEvent::AllocationRequest,1);adapter.require()?;
    adapter.enter(AdapterEvent::RequestedCopyBytes,n);adapter.require()?;
    let mut s=String::new();s.try_reserve_exact(text.len()).map_err(|_|CaptureError::Storage("prepared string"))?;
    capacities[10]=capacities[10].checked_add(s.capacity()).ok_or(CaptureError::CountRange("prepared string capacity"))?;
    adapter.enter(AdapterEvent::RustCapacityBytes,s.capacity() as u64);adapter.require()?;
    adapter.enter(AdapterEvent::MapWrite,1);adapter.require()?;s.push_str(text);Ok(s)
}
#[derive(Debug)]
pub(super) struct PreparedAssociation {
    pub member:u32,
    /// E, G, A, Iy, Iz, J from the authenticated old PrimitiveSource.
    pub old_source:[u64;6],
    /// D, effective t, A, I, J, Z, c from its matched old product facts.
    pub old_facts:[u64;7],
    /// A, I, J, Z, c returned by the actual closed preparation.
    pub prepared:[u64;5],
}
pub(super) struct PreparedCase {
    ordinary: Option<MechanicsEnvelope>,
    proof_attempted: bool,
    overlay_work: ScalarWork,
    associations: Vec<PreparedAssociation>,
    capture: ProductCapture,
    pub preparations: Vec<k::PreparedAnnulus>,
    pub preparation_work: Vec<k::SectionPreparationWork>,
    pub old_operational: Vec<OperationalSpent>,
    pub trace:trace::PreparedTrace,
}
pub(super) struct PreparedCaseFailure {
    pub associations: Vec<PreparedAssociation>,
    pub ordinary: MechanicsEnvelope,
    pub capture: ProductCapture,
    pub preparations: Vec<k::PreparedAnnulus>,
    pub preparation_work: Vec<k::SectionPreparationWork>,
    pub preparation_error: Option<k::SectionPreparationError>,
    pub old_operational: Vec<OperationalSpent>,
    pub trace:trace::PreparedTrace,
}
/// What one preparation attempt allocates and returns, kept whether it succeeds or not.
#[derive(Default)]
pub(super) struct AttemptParts {
    pub preparations: Vec<k::PreparedAnnulus>,
    pub preparation_work: Vec<k::SectionPreparationWork>,
    pub associations: Vec<PreparedAssociation>,
    pub old_operational: Vec<OperationalSpent>,
    pub preparation_error: Option<k::SectionPreparationError>,
}
/// B1 SP (DESIGN_v2 T-7; C3): one case's product attempt. Its per-case capture (the prepared
/// source, the facts, the operational records, its Run, its error) stays in the capture's
/// slot for `request`; this holds the attempt's own trace, preparation records, proof state
/// and where it ended.
pub(super) struct CaseAttempt {
    /// The case's index in the request (`model.load_cases`).
    pub request: usize,
    /// The attempt's id: its position in actual start order (C3 `product_attempts[]`).
    pub attempt: usize,
    /// Whether preparation completed, so that the case has a prepared `CaseSource` and is
    /// submitted to the call (T-8).
    pub prepared: bool,
    pub parts: AttemptParts,
    pub trace: trace::PreparedTrace,
    /// T-9: whether its one proof has started, and its overlay's scalar work.
    proof_attempted: bool,
    overlay_work: ScalarWork,
    /// Where the attempt ended (T-7 to T-9).
    pub end: AttemptEnd,
}
/// B1 SP (DESIGN_v2 T-7 to T-9): where one product attempt is, or ended.
pub(super) enum AttemptEnd {
    /// T-7 refused: its error is in its case's slot, the helper's refusal in `parts`.
    Preparation,
    /// Prepared; not yet submitted to the call (T-8).
    Prepared,
    /// T-8: its Run did not end `selected`, or the call failed before any Run. The cause is
    /// `trace.native_error`.
    Native,
    /// T-8: its Run ended `selected`; T-9 has not run.
    Selected,
    /// T-9: frozen: certified, and every private gate passed.
    Frozen(FrozenCase),
    /// T-9 refused after the selected Run (`facade_certificate`).
    Candidate(RefusedCase),
}
/// T-9's frozen payload of one case (its rows' values, maxima patches and aliases) and its
/// certificate. The ordinary owner is not touched.
pub(super) struct FrozenCase { payload: PreparedPayload, certificate: k::CertifiedProductProof }
impl FrozenCase {
    pub(super) fn certificate(&self) -> &k::CertifiedProductProof { &self.certificate }
}
/// T-9's refusal of one case, with the certificate and values it kept.
pub(super) struct RefusedCase {
    pub error: PreparedCandidateError,
    certificate: Option<k::CertifiedProductProof>,
    pub values: Option<k::FrozenProductValues>,
}
impl RefusedCase {
    pub(super) fn certificate(&self) -> Option<&k::CertifiedProductProof> { self.certificate.as_ref() }
    /// The refused proof's own failure work, when a proof was started.
    pub(super) fn proof_failure(&self) -> Option<&k::ProductProofFailure> {
        match &self.error {
            PreparedCandidateError::Proof(e) => Some(e),
            PreparedCandidateError::Values { proof, .. } | PreparedCandidateError::Abandoned { proof, .. } => Some(proof),
            _ => None,
        }
    }
}
/// B1 SP (DESIGN_v2 T-6 to T-11): the invocation's transaction after custody: the one
/// ordinary owner (never touched by W1), the one capture, and the attempts over A in
/// request order (= start order).
pub(super) struct PreparedCases {
    pub ordinary: MechanicsEnvelope,
    pub capture: ProductCapture,
    pub attempts: Vec<CaseAttempt>,
    /// B2-P (T-10a, T-10b): every model combination's record, in authored order (none at z = 0).
    pub combinations: Vec<CombinationAttempt>,
    /// B2-P (C3a): the operand preparations, in first-need order.
    pub operand_preparations: Vec<OperandPreparation>,
    /// B2-P (T-10b (i)): the rebuilt sources of `unavailable` operand cases, one per case.
    pub rebuilt: Vec<(usize, k::PreparedCaseSource)>,
}
/// B1 SP (DESIGN_v2 T-6): invocation custody failed, so no case was attempted.
pub(super) struct CustodyFailure {
    pub ordinary: MechanicsEnvelope,
    pub capture: ProductCapture,
    pub error: CaptureError,
}
impl PreparedCases {
    /// B1 SP (DESIGN_v2 T-8): one `CaseBatchCall` over the prepared cases of A, in request
    /// order, through one `RecordedInvocation`. Every submitted case gets a Run: one ending
    /// `selected` is `Selected`; any other is `Native` (`NativeUnavailable`), with its
    /// terminal snapshot. A failure of the call itself before any Run makes every submitted
    /// case `Native` with that cause, so no case can be selected. At c = 1 this is the
    /// one-case `solve_native`, with the same trace and adapter events.
    pub(super) fn native(&mut self) {
        if !self.attempts.iter().any(|attempt| matches!(attempt.end, AttemptEnd::Prepared)) {
            return;
        }
        let (operands, prepared) = self.combination_capacity();
        let Self { capture, attempts, .. } = self;
        for attempt in attempts.iter_mut().filter(|attempt| matches!(attempt.end, AttemptEnd::Prepared)) {
            attempt.trace.enter(trace::Stage::Native);
        }
        let called = {
            let attempts = &*attempts;
            capture.native_call(|| attempts.iter().filter(|attempt| matches!(attempt.end, AttemptEnd::Prepared)).map(|attempt| attempt.request),
                &operands, prepared)
        };
        for attempt in attempts.iter_mut().filter(|attempt| matches!(attempt.end, AttemptEnd::Prepared)) {
            let outcome = match &called {
                Err(error) => Err(error.clone()),
                Ok(()) => match capture.case_native(attempt.request).map(|case| &case.outcome) {
                    Some(k::ExecutionOutcome::Selected(_)) => Ok(()),
                    _ => Err(CaptureError::NativeUnavailable),
                },
            };
            match outcome {
                Ok(()) => {
                    attempt.trace.completed(trace::Stage::Native);
                    attempt.end = AttemptEnd::Selected;
                }
                Err(error) => {
                    attempt.trace.fail_entered();
                    attempt.trace.native_error = Some(error);
                    attempt.trace.freeze(capture);
                    attempt.end = AttemptEnd::Native;
                }
            }
        }
    }
    /// B2-P (B2-C §2.2 T-8′): the combination Calls' declared maxima, for the invocation's one
    /// `OriginCapacity`: h for each mechanics combination, in authored order, that the gates do
    /// not withhold, whose terms name distinct cases, and that has a term in the batch; and the
    /// number of distinct `not_required` cases (outside A) among those combinations' terms, each
    /// at most one operand preparation. Maxima, not counts: a combination later `ordinary`, or
    /// decided before its Call, leaves its reservation unused. At z = 0: none.
    fn combination_capacity(&self) -> (Vec<usize>, usize) {
        let gates = self.ordinary.contract_evidence.as_ref().map(|evidence| &evidence["combination_gates"]);
        let submitted = |case: usize| self.attempts.iter().any(|attempt| attempt.request == case && matches!(attempt.end, AttemptEnd::Prepared));
        let attempted = |case: usize| self.attempts.iter().any(|attempt| attempt.request == case);
        let (mut operands, mut prepared) = (Vec::new(), Vec::new());
        for (index, combination) in self.capture.combinations.iter().enumerate() {
            let open = gates.and_then(|gates| gates[index]["withheld"].as_bool()) == Some(false);
            if !combination.mechanics || !open || !distinct_terms(&combination.terms)
                || !combination.terms.iter().any(|&(case, _)| submitted(case)) {
                continue;
            }
            operands.push(combination.terms.len());
            for &(case, _) in &combination.terms {
                if !attempted(case) && !prepared.contains(&case) {
                    prepared.push(case);
                }
            }
        }
        (operands, prepared.len())
    }
    /// B1 SP (DESIGN_v2 T-9): one freeze per selected Run, in request order, each on its
    /// case's own slot and scope against the one untouched ordinary owner: the dual-readout
    /// proof, certificate, observables and G5a. A refusal makes only that case unavailable
    /// (`facade_certificate`). Each attempt's snapshot is taken at its last proof stage.
    pub(super) fn freeze(&mut self) {
        let Self { ordinary, capture, attempts, .. } = self;
        let invocation = capture.native_invocation.take();
        for attempt in attempts.iter_mut().filter(|attempt| matches!(attempt.end, AttemptEnd::Selected)) {
            let scope = capture.case_scope(attempt.request);
            let frozen = capture.with_case(attempt.request, |capture| {
                let case = capture.native.take();
                let frozen = capture.freeze_case(&mut attempt.trace, &mut attempt.overlay_work, &mut attempt.proof_attempted, ordinary, &scope,
                    invocation.as_ref().zip(case.as_ref()));
                capture.native = case;
                frozen
            });
            attempt.end = match frozen {
                Ok((payload, certificate)) => AttemptEnd::Frozen(FrozenCase { payload, certificate }),
                Err(refused) => AttemptEnd::Candidate(refused),
            };
        }
        capture.native_invocation = invocation;
    }
    /// B1 SP (DESIGN_v2 T-11, staging): one copy of the ordinary envelope, with each frozen
    /// case's overlay (its rows' values and its maxima patches) on its own rows and evidence
    /// case, in request order. With several requested cases the summary's two headlines are
    /// then those of the staged rows (`stage_headlines`); with one, the frozen case's aliases,
    /// as before. The ordinary owner is untouched; on a fault the copy drops.
    pub(super) fn staged_envelope(&self) -> Result<MechanicsEnvelope, StagingFault> {
        let mut staged = self.ordinary.clone();
        for attempt in &self.attempts {
            if let AttemptEnd::Frozen(frozen) = &attempt.end {
                apply_prepared_overlay(&mut staged, &frozen.payload, &self.capture.case_scope(attempt.request))?;
            }
        }
        // B2-P (B2-C §2.7 staging 2): each retained_selected combination's values on its own
        // block, in authored order (no maxima, evidence or alias patch).
        for combination in &self.combinations {
            if let CombinationEnd::Frozen(frozen) = &combination.end {
                let rows = self.capture.combination_rows.get(combination.index).cloned().ok_or(StagingFault("combination results"))?;
                apply_combination_overlay(&mut staged, &frozen.values, rows)?;
            }
        }
        if self.capture.cases_seen() > 1 {
            stage_headlines(&mut staged, &self.ordinary)?;
        }
        Ok(staged)
    }
    /// T-12's detail placement: bit `k` is set when attempt `k` is selected (frozen).
    pub(super) fn selected_attempts(&self) -> u64 {
        self.attempts.iter().enumerate()
            .filter(|(_, attempt)| matches!(attempt.end, AttemptEnd::Frozen(_)))
            .fold(0u64, |bits, (k, _)| bits | 1u64.checked_shl(u32::try_from(k).unwrap_or(u32::MAX)).unwrap_or(0))
    }
    /// The one ordinary owner, untouched (the fallback, or the transfer's base).
    pub(super) fn into_ordinary(self) -> MechanicsEnvelope { self.ordinary }
    /// Test-only staging fault (RV85 N6): the first frozen case's first maxima patch names an
    /// evidence index past the envelope's extrema.
    #[cfg(test)]
    pub(super) fn test_break_overlay(&mut self) {
        if let Some(AttemptEnd::Frozen(frozen)) = self.attempts.iter_mut().map(|attempt| &mut attempt.end).find(|end| matches!(end, AttemptEnd::Frozen(_))) {
            frozen.payload.maxima.first_mut().expect("the milestone has maxima patches").evidence_index = usize::MAX;
        }
    }
    /// B3b-P (P-12) test-only staging fault: the first frozen case's first section patch names
    /// an index past its `pipe_sections` (exact route).
    #[cfg(test)]
    pub(super) fn test_break_section_overlay(&mut self) {
        if let Some(AttemptEnd::Frozen(frozen)) = self.attempts.iter_mut().map(|attempt| &mut attempt.end).find(|end| matches!(end, AttemptEnd::Frozen(_))) {
            frozen.payload.sections.first_mut().expect("an exact case has section patches").evidence_index = usize::MAX;
        }
    }
}
impl CaseAttempt {
    /// The attempt's typed C3 view (I57), over the capture with its case in the capture's own
    /// fields: its trace, preparation and operational records, overlay work and terminal.
    pub(super) fn typed_trace<'a>(&'a self, capture: &'a ProductCapture, costs: &mut trace::ProjectionWork)
        -> Result<trace::PreparedAttemptView<'a>, trace::TraceProjectionError> {
        let (work, old) = (&self.parts.preparation_work[..], &self.parts.old_operational[..]);
        match &self.end {
            AttemptEnd::Frozen(frozen) => trace::project(&self.trace, capture, work, old, &self.overlay_work,
                trace::ResultRef::Ready, Some(frozen.certificate.work()), None, None, costs),
            AttemptEnd::Candidate(refused) => {
                let failure = refused.proof_failure();
                let proof = failure.map(|f| f.work()).or_else(|| refused.certificate.as_ref().map(|c| c.work()));
                let values = match &refused.error { PreparedCandidateError::Values { failure, .. } => Some(failure), _ => None };
                trace::project(&self.trace, capture, work, old, &self.overlay_work,
                    trace::ResultRef::Unavailable(trace::FailureRef::Candidate(&refused.error)), proof, failure.map(|f| f.failure()), values, costs)
            }
            AttemptEnd::Native => trace::project(&self.trace, capture, work, old, &self.overlay_work,
                trace::ResultRef::Unavailable(trace::FailureRef::Native(self.trace.native_error.as_ref().ok_or(trace::TraceProjectionError::MissingFailure)?)),
                None, None, None, costs),
            AttemptEnd::Preparation => trace::project(&self.trace, capture, work, old, &self.overlay_work,
                trace::ResultRef::Unavailable(trace::FailureRef::Preparation {
                    capture: capture.error.as_ref().ok_or(trace::TraceProjectionError::MissingFailure)?, section: self.parts.preparation_error.as_ref() }),
                None, None, None, costs),
            AttemptEnd::Prepared | AttemptEnd::Selected => Err(trace::TraceProjectionError::MissingTerminal),
        }
    }
}
impl ProductCapture {
    /// B1 SP (DESIGN_v2 T-6): invocation custody, checked once for every requested case, before
    /// any attempt.
    /// - A prior capture error of any case, the first in request order, is the cause: the parked
    ///   cases' (0 to c − 2), then the last case's, in the capture's own fields (RV109 R3P-3).
    /// - The ordinary preconditions: one final hook, the route's base contract (preview-physics-1,
    ///   or physics-1 on the exact route: B3b-P), `MECHANICS_SOLVED`,
    ///   no exact-block selection, and no native work yet.
    /// - One complete late capture per requested case.
    /// - Each case's observations bound to the envelope (`bind_observations`, per case), and,
    ///   with several cases, each case's rows bound as its block (`bind_case_rows`).
    ///
    /// At c = 1 this is the old one-case prelude, with the same adapter events; the one-case
    /// `prepare_case` runs it too (RV109 R3P-9).
    fn prepared_custody(&mut self,ordinary:&MechanicsEnvelope,requested:usize,attempted:&[usize])->Result<(),CaptureError> {
        if let Some(slot)=self.parked.iter_mut().find(|slot|slot.error.is_some()) {
            return Err(slot.error.take().expect("observed prior case cause"));
        }
        if self.error.is_some() {
            return Err(self.error.take().expect("observed prior capture cause"));
        }
        self.adapter.require()?;
        if !self.prepared_probe || self.final_calls!=1
            || ordinary.source_block_recovery.is_some() || self.native.is_some() || self.native_invocation.is_some()
            || self.parked.iter().any(|slot|slot.native.is_some())
            || ordinary.producer.semantic_contract_id!=self.route.base_contract() || ordinary.status.mechanics!="MECHANICS_SOLVED" {
            return Err("prepared case custody/permit".into());
        }
        if self.cases_seen()!=requested {
            return Err("prepared case count".into());
        }
        // A's request indices: each in the request, strictly increasing (RV109 R3P-2).
        if attempted.iter().any(|&request|request>=requested) || attempted.windows(2).any(|pair|pair[0]>=pair[1]) {
            return Err("attempted cases outside the request or out of order".into());
        }
        if self.parked.is_empty() {
            self.bind_observations(ordinary)?;
            // B2-P (T-6′): with a combination, the one case's rows are its own block too.
            if self.combinations.is_empty() {
                return Ok(());
            }
            return self.bind_case_rows(ordinary);
        }
        self.bind_observations_by_case(ordinary)?;
        self.bind_case_rows(ordinary)
    }
    /// B1 SP (DESIGN_v2 T-6 for c ≥ 2; T-9's scope): each requested case's rows are one block
    /// of the envelope, in request order, as the ordinary run appends them (lib.rs), and no row
    /// names another case or none. One pass, comparing each row's basis with the current case
    /// and, at a block's end, the next. Each case's block is kept in its slot (`scope_rows`).
    ///
    /// B2-P (T-6′; REVISION_01 S-2): with a combination, the case blocks end at the first row
    /// whose basis `ref_type` is `combination` (`case_rows_end`), at c = 1 too, and the rows
    /// after them bind to their combinations (`bind_combination_rows`). With none, B1's pass.
    fn bind_case_rows(&mut self,ordinary:&MechanicsEnvelope)->Result<(),CaptureError> {
        let cases=self.cases_seen();
        let (mut current,mut start)=(0usize,0usize);
        let mut end=ordinary.results.len();
        for (index,row) in ordinary.results.iter().enumerate() {
            self.capture_entry(AdapterEvent::RowVisit)?;
            let basis=row.basis_ref.as_ref().ok_or("case row basis")?;
            if !self.combinations.is_empty() && self.checked_same(&basis.ref_type,"combination")? {
                end=index;
                break;
            }
            let same=self.adapter.same(&basis.ref_id,self.case_observation_state(current).2);
            self.adapter.require()?;
            if same {continue;}
            let next=current.checked_add(1).filter(|next|*next<cases && index>start).ok_or("case row block")?;
            let opens=self.adapter.same(&basis.ref_id,self.case_observation_state(next).2);
            self.adapter.require()?;
            if !opens {return Err("case row block".into());}
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.with_case(current,|capture|capture.scope_rows=start..index);
            (current,start)=(next,index);
        }
        if current.checked_add(1)!=Some(cases) || end==start {
            return Err("case row block".into());
        }
        self.capture_entry(AdapterEvent::MapWrite)?;
        self.with_case(current,|capture|capture.scope_rows=start..end);
        self.bind_combination_rows(ordinary,end)
    }
    /// B2-P (B2-C §2.2 T-6′; REVISION_01 S-2): after the case rows (from `end`), every row has
    /// basis `combination` and names a model combination; each mechanics combination's rows are
    /// one contiguous run, its freeze scope (empty when it has none). Subtraction and range rows,
    /// with the modulus-basis records that trail every combination's rows, bind to their
    /// combination by id, with no contiguity. A violation is a custody failure. With no
    /// combination there is no such row and nothing is recorded (no event).
    fn bind_combination_rows(&mut self,ordinary:&MechanicsEnvelope,end:usize)->Result<(),CaptureError> {
        if self.combinations.is_empty() {
            return Ok(());
        }
        let mut runs=self.adapter.reserve::<std::ops::Range<usize>>(self.combinations.len())?;
        for _ in 0..self.combinations.len() {
            self.capture_entry(AdapterEvent::MapWrite)?;
            runs.push(end..end);
        }
        for (index,row) in ordinary.results.iter().enumerate().skip(end) {
            self.capture_entry(AdapterEvent::RowVisit)?;
            let basis=row.basis_ref.as_ref().ok_or("combination row basis")?;
            if !self.checked_same(&basis.ref_type,"combination")? {
                return Err("combination row block".into());
            }
            let mut owner=None;
            for (k,combination) in self.combinations.iter().enumerate() {
                if self.checked_same(&basis.ref_id,&combination.id)? {
                    owner=Some(k);
                    break;
                }
            }
            let k=owner.ok_or("combination row block")?;
            if self.combinations[k].mechanics {
                let run=&mut runs[k];
                if run.start==run.end {
                    *run=index..index+1;
                } else if run.end==index {
                    run.end=index+1;
                } else {
                    return Err("combination row block".into());
                }
            }
        }
        self.capture_entry(AdapterEvent::MapWrite)?;
        self.case_rows_end=end;
        self.combination_rows=runs;
        Ok(())
    }
    /// B1 SP (DESIGN_v2 T-6, then T-7): custody once, then one product attempt per case in
    /// A (`attempted`: request indices, strictly increasing, validated by custody), each
    /// preparing its own case. An
    /// attempt's preparation failure makes only that case unavailable: it keeps no prepared
    /// source, its error is in its slot, and its trace's terminal snapshot is preparation's.
    /// The other cases continue. Attempt ids are the actual start order.
    pub(super) fn prepare_cases(mut self,ordinary:MechanicsEnvelope,requested:usize,attempted:&[usize])->Result<PreparedCases,CustodyFailure> {
        if let Err(error)=self.prepared_custody(&ordinary,requested,attempted) {
            return Err(CustodyFailure{ordinary,capture:self,error});
        }
        let mut attempts=Vec::with_capacity(attempted.len());
        for (attempt,&request) in attempted.iter().enumerate() {
            attempts.push(self.with_case(request,|capture|capture.prepare_attempt(request,attempt)));
        }
        Ok(PreparedCases{ordinary,capture:self,attempts,combinations:Vec::new(),operand_preparations:Vec::new(),rebuilt:Vec::new()})
    }
    /// One product attempt (T-7) on the case in the capture's own fields.
    fn prepare_attempt(&mut self,request:usize,attempt:usize)->CaseAttempt {
        #[cfg(test)] crate::retained_tests_hooks::before_case_preparation(self,request);
        let mut parts=AttemptParts::default();
        let mut trace=trace::PreparedTrace::default();trace.enter(trace::Stage::Preparation);
        let (prepared,end)=match self.prepare_active_case(&mut trace,&mut parts) {
            Ok(())=>(true,AttemptEnd::Prepared),
            Err(e)=>{trace.fail_entered();self.error=Some(e);trace.freeze(self);(false,AttemptEnd::Preparation)}
        };
        CaseAttempt{request,attempt,prepared,parts,trace,proof_attempted:false,overlay_work:ScalarWork::default(),end}
    }
    /// B1 SP (DESIGN_v2 T-8; C2 §4): one `CaseBatchCall` through one `RecordedInvocation`
    /// (W1-LME-20B-60B-v1: invocation limit 60e9, case limit 20e9 each) over the prepared
    /// sources of `submitted` (request indices, in request order). The kernel registers them in
    /// that order and names each by its batch ordinal. The invocation is kept once; each case's
    /// slot keeps its own Run. `Err` is a failure of the call itself before any Run.
    ///
    /// One submitted source is borrowed in place, as the one-case solve always did (no copy and
    /// no event). Several move into one reserved vector for the call (one allocation and a map
    /// write per move, through the adapter) and move back after it, whatever it returned.
    ///
    /// B2-P (T-8′): the capacity is `for_invocation(&[n], operands, prepared)` (KD §1.2) with the
    /// combination Calls' declared maxima (`PreparedCases::combination_capacity`); at z = 0
    /// `for_invocation(&[n], &[], 0)`, which is `for_calls(&[n], &[])` field for field (KD I2).
    fn native_call<I:Iterator<Item=usize>>(&mut self,submitted:impl Fn()->I,operands:&[usize],prepared:usize)->Result<(),CaptureError> {
        if self.native_invocation.is_some() || submitted().any(|request|self.case_native(request).is_some()) {
            return Err("duplicate prepared solve".into());
        }
        if submitted().any(|request|self.case_prepared_source(request).is_none()) {
            return Err("prepared source".into());
        }
        let count=submitted().count();
        let limit=k::CaseLimit::new(20_000_000_000);
        let cap=k::OriginCapacity::for_invocation(&[count],operands,prepared).map_err(CaptureError::Origin)?;
        let mut invocation=k::RecordedInvocation::new(60_000_000_000,cap).map_err(CaptureError::Origin)?;
        let cases=match (submitted().next(),count) {
            (Some(only),1)=>{
                let source=self.case_prepared_source(only).ok_or("prepared source")?;
                invocation.solve_cases(std::slice::from_ref(source),limit).map_err(CaptureError::Origin)?
            }
            _=>{
                let mut sources=self.adapter.reserve::<k::PrimitiveSource>(count)?;
                let moves=u64::try_from(count).ok().and_then(|n|n.checked_mul(2)).ok_or(CaptureError::CountRange("native source moves"))?;
                self.adapter.enter(AdapterEvent::MapWrite,moves);self.adapter.require()?;
                for request in submitted() {
                    if let Some(source)=self.with_case(request,|capture|capture.source.take()) {sources.push(source);}
                }
                let solved=if sources.len()==count {invocation.solve_cases(&sources,limit).map_err(CaptureError::Origin)}
                    else {Err("prepared source".into())};
                let mut back=sources.into_iter();
                for request in submitted() {
                    if let Some(source)=back.next() {self.with_case(request,|capture|capture.source=Some(source));}
                }
                solved?
            }
        };
        if cases.len()!=count {
            return Err("native case count".into());
        }
        self.native_invocation=Some(invocation);
        for (request,case) in submitted().zip(cases) {
            self.with_case(request,|capture|capture.native=Some(case));
        }
        Ok(())
    }
}
impl ProductCapture {
    pub(super) fn prepared_probe() -> Self { Self {prepared_probe:true,..Self::default()} }
    /// B3b-P (P-1): the private driver's observer on `route`.
    pub(super) fn prepared_probe_on(route:W1Route) -> Self { Self {prepared_probe:true,route,..Self::default()} }
    /// U3: the facade's observer, bound to its capture permit for G-B (the preview route).
    pub(super) fn permitted_probe(permit:super::retained_memory::CapturePermit) -> Self {
        Self::permitted_probe_on(permit,W1Route::Preview)
    }
    /// B3b-P (P-1): the facade's observer on the invocation's route, decided once in
    /// `permitted_run`.
    pub(super) fn permitted_probe_on(permit:super::retained_memory::CapturePermit,route:W1Route) -> Self {
        Self {prepared_probe:true,permit:Some(permit),route,..Self::default()}
    }
    pub(super) fn late_refusal(&self)->Option<&super::retained_memory::PhaseRefusal> {self.late_refusal.as_ref()}
    /// The facade's capture permit, owned by this observer (U3 grant 1b: linear), for G-C.
    pub(super) fn permit(&self)->Option<&super::retained_memory::CapturePermit> {self.permit.as_ref()}
    /// Preparation over the single actual ordinary run's capture (the private driver and tests):
    /// custody (T-6) and the one product attempt (T-7), through `prepare_cases` with the one
    /// requested case, so that there is one custody prelude (RV109 R3P-9). A custody refusal is
    /// the attempt's preparation failure: its trace enters and fails preparation, with its
    /// snapshot, and the cause is the capture's error.
    pub(super) fn prepare_case(self,ordinary:MechanicsEnvelope)->Result<PreparedCase,PreparedCaseFailure> {self.prepare_owned_case(ordinary)}
    fn prepare_owned_case(self, ordinary:MechanicsEnvelope) -> Result<PreparedCase,PreparedCaseFailure> {
        match self.prepare_cases(ordinary,1,&[0]) {
            Err(CustodyFailure{ordinary,mut capture,error})=>{
                let mut trace=trace::PreparedTrace::default();trace.enter(trace::Stage::Preparation);
                trace.fail_entered();capture.error=Some(error);trace.freeze(&capture);
                Err(PreparedCaseFailure{associations:Vec::new(),ordinary,capture,preparations:Vec::new(),preparation_work:Vec::new(),
                    preparation_error:None,old_operational:Vec::new(),trace})
            }
            Ok(PreparedCases{ordinary,capture,mut attempts,..})=>{
                let CaseAttempt{prepared,parts,trace,..}=attempts.pop().expect("one attempt");
                let AttemptParts{preparations,preparation_work,associations,old_operational,preparation_error}=parts;
                if prepared {
                    Ok(PreparedCase{ordinary:Some(ordinary),proof_attempted:false,overlay_work:ScalarWork::default(),associations,capture,
                        preparations,preparation_work,old_operational,trace})
                } else {
                    Err(PreparedCaseFailure{associations,ordinary,capture,preparations,preparation_work,preparation_error,old_operational,trace})
                }
            }
        }
    }
    /// The preparation of the case in the capture's own fields (DESIGN_v2 T-7, one attempt):
    /// the old source's custody, the closed annulus helper per member, and the prepared
    /// source. Run by each attempt of `prepare_cases` (and so by the one-case `prepare_case`); what it
    /// allocates and returns goes into `out`, whether it succeeds or not.
    fn prepare_active_case(&mut self,trace:&mut trace::PreparedTrace,out:&mut AttemptParts)->Result<(),CaptureError> {
        let old=self.source.as_ref().ok_or("old source validation missing")?;
        if old.members().len()!=self.facts.len() || old.members().len()!=self.operational.len() {
            return Err("old source/facts/operational coverage".into());
        }
        // Complete inventory/order is independent of later old-to-old section checks.
        for (m,op) in old.members().iter().zip(&self.operational) {
            trace.costs.record::<(u32,Option<u32>)>();
            if op.member!=Some(m.id){return Err("old operational member order".into());}
        }
        trace.costs.record::<trace::OldCoverage>();trace.old_coverage=trace::OldCoverage::Complete;
        #[cfg(test)] if self.trace_fault==Some(trace::TraceFault::AfterPrelude){return Err("trace control after prelude".into());}
        let mut parts=k::SourceParts::default();
        parts.nodes=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,0,old.nodes().len())?;
        parts.members=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,1,old.members().len())?;
        parts.constraints=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,2,old.constraints().len())?;
        parts.springs=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,3,old.springs().len())?;
        parts.stations=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,4,old.stations().len())?;
        parts.supports=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,5,old.supports().len())?;
        parts.loads=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,6,old.loads().len())?;
        out.preparations=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,7,old.members().len())?;
        out.preparation_work=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,8,old.members().len())?;
        out.associations=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,13,old.members().len())?;
        let new_operational=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,9,old.members().len())?;
        out.old_operational=std::mem::replace(&mut self.operational,new_operational);
        trace.costs.record::<bool>();trace.old_vector_swapped=true;
        trace.members=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,14,old.members().len())?;
        for x in old.nodes() { self.capture_entry(AdapterEvent::MapWrite)?; parts.nodes.push(*x); }
        for x in old.constraints() { self.capture_entry(AdapterEvent::MapWrite)?; parts.constraints.push(*x); }
        for x in old.springs() { self.capture_entry(AdapterEvent::MapWrite)?; parts.springs.push(*x); }
        for x in old.stations() { self.capture_entry(AdapterEvent::MapWrite)?; parts.stations.push(*x); }
        for x in old.supports() {
            let mut children=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,11,x.springs.len())?;
            for &id in &x.springs {self.capture_entry(AdapterEvent::MapWrite)?;children.push(id);}
            let mut directional=prepared_reserve(&self.adapter,&mut self.prepared_capacity_bytes,12,x.directional_springs.len())?;
            for &id in &x.directional_springs {self.capture_entry(AdapterEvent::MapWrite)?;directional.push(id);}
            self.capture_entry(AdapterEvent::MapWrite)?;
            parts.supports.push(k::SupportGroup{id:x.id,node:x.node,restrained:x.restrained,springs:children,directional_springs:directional});
        }
        for x in old.loads() {
            let id=prepared_string(&self.adapter,&mut self.prepared_capacity_bytes,&x.source_id)?;
            self.capture_entry(AdapterEvent::MapWrite)?;
            parts.loads.push(k::NodalLoad{dof:x.dof,value:x.value,source_id:id});
        }
        for (i,m) in old.members().iter().enumerate() {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            let f=self.facts.get(i).ok_or("prepared fact coverage")?;
            if f.member!=m.id || f.area.to_bits()!=m.area.to_bits()
                || f.second_moment.to_bits()!=m.second_moment_y.to_bits()
                || f.second_moment.to_bits()!=m.second_moment_z.to_bits()
                || f.torsion_constant.to_bits()!=m.torsion_constant.to_bits() {return Err("old-to-old section".into());}
            // Enter the reserved work-record write before the producer. Every
            // producing return is immediately owned, with no fallible gap.
            self.capture_entry(AdapterEvent::MapWrite)?;
            if trace.members.len()==trace.members.capacity() || out.preparation_work.len()==out.preparation_work.capacity(){return Err("prepared trace/work capacity".into());}
            trace.costs.record::<trace::PreparationEntry>();
            trace.members.push(trace::PreparationEntry {member:m.id,
                old_source:[m.elastic_modulus.to_bits(),m.shear_modulus.to_bits(),m.area.to_bits(),m.second_moment_y.to_bits(),m.second_moment_z.to_bits(),m.torsion_constant.to_bits()],
                old_facts:[f.diameter.to_bits(),f.effective_wall.to_bits(),f.area.to_bits(),f.second_moment.to_bits(),f.torsion_constant.to_bits(),f.section_modulus.to_bits(),f.radius.to_bits()],
                result:trace::PreparationResult::Entered,work_index:out.preparation_work.len()});
            let spent=k::prepare_product_annulus(f.diameter,f.effective_wall);
            let (result,w)=spent.into_parts();out.preparation_work.push(w); // reserved/precharged: no fallible return gap
            trace.costs.record::<trace::PreparationResult>();
            trace.members.last_mut().unwrap().result=match &result {
                Ok(p)=>trace::PreparationResult::Prepared(p.section_bits()),Err(e)=>trace::PreparationResult::Refused(e.clone())};
            #[cfg(test)] if self.trace_fault==Some(trace::TraceFault::AfterHelper){return Err("trace control after helper".into());}
            let prep=match result { Ok(p)=>p, Err(e)=>{
                out.preparation_error=Some(e); return Err("annulus preparation refused".into());
            }};
            if prep.input_bits()!=[f.diameter.to_bits(),f.effective_wall.to_bits()] {return Err("prepared input bits".into());}
            // Fixed infallible group: five returned bit copies plus nineteen record fields.
            self.adapter.enter(AdapterEvent::MapWrite,24);self.adapter.require()?;
            self.adapter.enter(AdapterEvent::RequestedCopyBytes,std::mem::size_of::<PreparedAssociation>() as u64);self.adapter.require()?;
            out.associations.push(PreparedAssociation{member:m.id,
                old_source:[m.elastic_modulus.to_bits(),m.shear_modulus.to_bits(),m.area.to_bits(),m.second_moment_y.to_bits(),m.second_moment_z.to_bits(),m.torsion_constant.to_bits()],
                old_facts:[f.diameter.to_bits(),f.effective_wall.to_bits(),f.area.to_bits(),f.second_moment.to_bits(),f.torsion_constant.to_bits(),f.section_modulus.to_bits(),f.radius.to_bits()],
                prepared:prep.section_bits().bits()});
            let [a,ii,j,z,c]=prep.section_bits().values();
            self.adapter.enter(AdapterEvent::RequestedCopyBytes,std::mem::size_of::<k::StraightMember>() as u64);self.adapter.require()?;
            self.capture_entry(AdapterEvent::MapWrite)?;let mut new=*m;
            self.adapter.enter(AdapterEvent::MapWrite,4);self.adapter.require()?;
            new.area=a;new.second_moment_y=ii;new.second_moment_z=ii;new.torsion_constant=j;
            self.capture_entry(AdapterEvent::MapWrite)?;parts.members.push(new);
            self.adapter.enter(AdapterEvent::MapWrite,5);self.adapter.require()?;
            let f=&mut self.facts[i];f.area=a;f.second_moment=ii;f.torsion_constant=j;f.section_modulus=z;f.radius=c;
            self.capture_entry(AdapterEvent::MapWrite)?;
            self.adapter.enter(AdapterEvent::RequestedCopyBytes,std::mem::size_of::<Option<u32>>() as u64);self.adapter.require()?;
            self.operational.push(evaluate_member_operational(m.id,[parts.nodes[m.node_i as usize],parts.nodes[m.node_j as usize]],
                [m.elastic_modulus,m.shear_modulus,a,j]));
            #[cfg(test)] if self.trace_fault==Some(trace::TraceFault::AfterEvaluation){return Err("trace control after evaluator".into());}
            self.capture_entry(AdapterEvent::MapWrite)?;out.preparations.push(prep);
        }
        self.adapter.require()?;
        #[cfg(test)] if self.trace_fault==Some(trace::TraceFault::SourceConstruction){parts.members[0].area=0.0;}
        let new=k::PrimitiveSource::new(parts).map_err(CaptureError::Source)?;
        self.check_support_source(&new)?;
        self.capture_entry(AdapterEvent::MapWrite)?;
        self.source=Some(new);trace.costs.record::<bool>();trace.source_ready=true;
        trace.completed(trace::Stage::Preparation);
        Ok(())
    }
}
impl PreparedCase {
    pub(super) fn capture(&self)->&ProductCapture {&self.capture}
    #[cfg(test)]
    pub(super) fn test_capture_mut(&mut self)->&mut ProductCapture {&mut self.capture}
    pub(super) fn prepare_observed(request:LinearStaticPreviewRequest,mode:PreviewSolverMode,
        capture:&source_receipt::CapturedInvocation)->Result<Self,PreparedCaseFailure> {
        // B3b-P (P-1, P-2): the route and its exact-block budget, as `permitted_run` decides them.
        let route=super::w1_route(&request.model).unwrap_or_default();
        let mut observer=ProductCapture::prepared_probe_on(route);
        let ordinary=run_linear_static_preview_observed(request,mode,Some(capture),&mut super::w1_budget(route),Some(&mut observer));
        observer.prepare_owned_case(ordinary)
    }
    pub(super) fn ordinary(&self)->&MechanicsEnvelope {self.ordinary.as_ref().expect("owned ordinary before attempt")}
    /// U3 fallback after a native refusal: the untouched ordinary envelope.
    pub(super) fn into_ordinary(mut self)->MechanicsEnvelope {self.ordinary.take().expect("owned ordinary before attempt")}
    /// The owned ordinary envelope, if this case still owns it (no panic).
    pub(super) fn owned_ordinary(&self)->Option<&MechanicsEnvelope> {self.ordinary.as_ref()}
    pub(super) fn solve_native(&mut self) -> Result<(),&CaptureError> {
        if self.trace.stages[trace::Stage::Native as usize]!=trace::StageState::NotEntered {
            return Err(&CaptureError::PreparedAttemptConsumed);
        }
        self.trace.enter(trace::Stage::Native);
        // The one-case call (B1 SP's T-8 with one submitted case: the same events).
        let result=(||->Result<(),CaptureError>{
        let o=&mut self.capture;
        let active=o.parked.len();
        o.native_call(||std::iter::once(active),&[],0)?;
        if matches!(o.native.as_ref().map(|case|&case.outcome),Some(k::ExecutionOutcome::Selected(_))) {Ok(())} else {Err(CaptureError::NativeUnavailable)}
        })();
        match result {Ok(())=>{self.trace.completed(trace::Stage::Native);Ok(())},
            Err(e)=>{self.trace.fail_entered();self.trace.native_error=Some(e);self.trace.freeze(&self.capture);Err(self.trace.native_error.as_ref().unwrap())}}
    }

}

// I51 frozen overlay. One ordinary owner remains untouched until the final move.
const PREPARED_MAX_KEYS:[&str;8]=["station_fraction","span_index","local_fraction","value_lower_pa",
    "value_upper_pa","global_upper_bound_pa","certified_gap_pa","subdivisions"];
struct PreparedMaximumPatch { evidence_index:usize, row:usize, member:u32, numbers:[serde_json::Number;8] }
/// B3b-P (B3-D §1.4, B3D-4): the exact route's regenerated section evidence of one member in
/// the owner case's `exact_cases` entry: `pipe_sections[evidence_index]`'s four values.
const PREPARED_SECTION_KEYS:[&str;4]=["As_m2","I_m4","J_m4","Z_m3"];
struct PreparedSectionPatch { evidence_index:usize, member:u32, numbers:[serde_json::Number;4] }
/// `sections` is empty on the preview route (no allocation, no event).
struct PreparedPayload { values:k::FrozenProductValues, maxima:Vec<PreparedMaximumPatch>,
    displacement:LocatedQuantity, stress:LocatedQuantity, sections:Vec<PreparedSectionPatch> }
#[derive(Clone,Copy)]
struct ProductRowView<'a> { original:&'a ResultItem, value:&'a f64 }
impl std::ops::Deref for ProductRowView<'_> {type Target=ResultItem;fn deref(&self)->&ResultItem{self.original}}
/// B1 SP (DESIGN_v2 T-9): the part of the one ordinary envelope that is one requested case's:
/// its rows, its `contract_evidence.preview_cases[]` entry (of `cases`), and whether its row
/// ids are case-qualified (a case after the first; lib.rs `qualified_load_case_result_id`).
/// One requested case owns the whole envelope (`WHOLE`: the one-case scope, unchanged).
/// B3b-P: `route` names the base evidence's per-case array (`preview_cases` or `exact_cases`)
/// and whether the exact route's section evidence is overlaid (B3-D P-6 to P-8).
#[derive(Debug,Clone,PartialEq,Eq)]
pub(super) struct CaseScope {rows:Option<std::ops::Range<usize>>,evidence:usize,cases:usize,qualified:bool,route:W1Route}
impl CaseScope {
    pub(super) const WHOLE:Self=Self{rows:None,evidence:0,cases:1,qualified:false,route:W1Route::Preview};
    /// The one-case scope on the exact route.
    pub(super) const WHOLE_EXACT:Self=Self{route:W1Route::Exact,..Self::WHOLE};
    /// The one-case scope on `route` (`WHOLE` on the preview route).
    pub(super) const fn whole(route:W1Route)->Self {Self{route,..Self::WHOLE}}
    /// The one-case scope on `route`, borrowed for any lifetime.
    pub(super) fn whole_ref(route:W1Route)->&'static Self {match route {W1Route::Preview=>&Self::WHOLE,W1Route::Exact=>&Self::WHOLE_EXACT}}
    /// B2-P: a combination's block of rows (its row binding; no evidence case).
    pub(super) fn block(rows:std::ops::Range<usize>,route:W1Route)->Self {Self{rows:Some(rows),evidence:0,cases:1,qualified:false,route}}
    /// The case's rows (all rows for `WHOLE`), or `None` when they are not the envelope's.
    pub(super) fn rows_of<'a>(&self,results:&'a [ResultItem])->Option<&'a [ResultItem]> {
        results.get(self.row_range(results.len()))
    }
    /// The case's row positions in an envelope of `len` rows (all of them for `WHOLE`).
    pub(super) fn row_range(&self,len:usize)->std::ops::Range<usize> {self.rows.clone().unwrap_or(0..len)}
}
#[derive(Clone,Copy)]
struct ProductCaseView<'a> {ordinary:&'a MechanicsEnvelope,payload:Option<&'a PreparedPayload>,scope:&'a CaseScope,
    /// B2-P: a frozen combination's values (no maxima or aliases), overlaid like a payload's.
    frozen:Option<&'a k::FrozenProductValues>}
impl<'a> ProductCaseView<'a> {
    /// The ordinary envelope's view of a case (no overlay).
    fn of(ordinary:&'a MechanicsEnvelope,scope:&'a CaseScope)->Self {Self{ordinary,payload:None,scope,frozen:None}}
    fn ordinary(self)->&'a MechanicsEnvelope {self.ordinary}
    /// The case's rows of the envelope (none when the scope does not fit it).
    fn case_rows(self)->&'a [ResultItem] {self.scope.rows_of(&self.ordinary.results).unwrap_or(&[])}
    fn rows(self)->impl Iterator<Item=ProductRowView<'a>> {
        self.case_rows().iter().enumerate().map(move |(i,r)|ProductRowView{original:r,value:match self.payload.map(|payload|&payload.values).or(self.frozen) {
            None=>&r.value,Some(values)=>values.value(i).expect("validated overlay length")}})
    }
    fn number(self,index:usize,key:&str,adapter:&AdapterWork)->Result<Option<f64>,CaptureError> {
        match self.payload {Some(payload)=>{
            let mut slot=None;
            for (i,k) in PREPARED_MAX_KEYS.iter().enumerate() {
                let same=adapter.same(k,key);adapter.require()?;
                if same{slot=Some(i);break;}
            }
            let Some(slot)=slot else{return Ok(None);};
            for m in &payload.maxima {
                adapter.enter(AdapterEvent::RowVisit,1);adapter.require()?;
                if m.evidence_index==index {
                    adapter.enter(AdapterEvent::ValidationEntry,1);adapter.require()?;
                    return Ok(m.numbers[slot].as_f64());
                }
            }
            Ok(None)
        },None=>Ok(self.ordinary.contract_evidence.as_ref().and_then(|e|e[self.scope.route.evidence_cases()][self.scope.evidence]["pipe_stress_extrema"][index][key].as_f64()))}
    }
    fn headline(self,stress:bool)->Option<&'a LocatedQuantity> {match self.payload {
        None=>if stress{self.ordinary.summary.max_open_formula_stress.as_ref()}else{self.ordinary.summary.max_displacement.as_ref()},
        Some(payload)=>Some(if stress{&payload.stress}else{&payload.displacement})}}
}
#[derive(Debug)]
pub(super) enum PreparedCandidateError {Capture(CaptureError),Proof(k::ProductProofFailure),
    Values{failure:k::ProductValuesFailure,proof:k::ProductProofFailure},Abandoned{cause:CaptureError,proof:k::ProductProofFailure},Numeric,Observable,G5a}
pub(super) struct PreparedCandidateRefusal {
    pub ordinary:MechanicsEnvelope,prepared:PreparedCase,pub error:PreparedCandidateError,
    certificate:Option<k::CertifiedProductProof>,pub values:Option<k::FrozenProductValues>,
}
pub(super) struct PrivatePreparedCandidate { envelope:MechanicsEnvelope,prepared:PreparedCase,
    certificate:k::CertifiedProductProof }
impl PrivatePreparedCandidate {
    pub(super) fn envelope(&self)->&MechanicsEnvelope{&self.envelope}
    /// U2: read-only; the serializer binds it to the selected owner structurally.
    pub(super) fn certificate(&self)->&k::CertifiedProductProof{&self.certificate}
    pub(super) fn capture(&self)->&ProductCapture{&self.prepared.capture}
    pub(super) fn local_work(&self)->(&ScalarWork,&[PreparedAssociation]){(&self.prepared.overlay_work,&self.prepared.associations)}
}
impl PreparedCandidateRefusal {
    /// U2 (failure path): read-only; a serialized refusal binds it structurally.
    pub(super) fn certificate(&self)->Option<&k::CertifiedProductProof>{self.certificate.as_ref()}
    /// The refused proof's own failure work, when a proof was started.
    pub(super) fn proof_failure(&self)->Option<&k::ProductProofFailure>{
        match &self.error {PreparedCandidateError::Proof(e)=>Some(e),
            PreparedCandidateError::Values{proof,..}|PreparedCandidateError::Abandoned{proof,..}=>Some(proof),_=>None}
    }
    pub(super) fn capture(&self)->&ProductCapture{&self.prepared.capture}
    pub(super) fn local_work(&self)->(&ScalarWork,&[PreparedAssociation]){(&self.prepared.overlay_work,&self.prepared.associations)}
}
impl ProductCapture {
    fn bind_observations_view(&self,view:ProductCaseView<'_>)->Result<(),CaptureError> {
        self.bind_observations_in(view.case_rows(),view.scope.qualified)?;
        for r in view.rows() {
            self.capture_entry(AdapterEvent::RowVisit)?;
            if matches!(r.kind.as_str(),"linear_solver_mode_basis"|"sparse_live_path_dense_parity_relative_delta"|"modulus_basis_record")
                && r.value.to_bits()!=r.original.value.to_bits() {return Err("overlay ancillary bits".into());}
        }
        Ok(())
    }
    fn prepared_verdict_copy(&mut self,verdicts:&[k::ProductRowVerdict],coverage:&[k::ProductSummaryCoverage])->Result<(),CaptureError> {
        self.verdicts=self.adapter.reserve(verdicts.len())?;
        for v in verdicts {self.capture_entry(AdapterEvent::MapWrite)?;self.verdicts.push(v.clone());}
        self.summary_coverage=self.adapter.reserve(coverage.len())?;
        for v in coverage {self.capture_entry(AdapterEvent::MapWrite)?;self.summary_coverage.push(*v);}
        Ok(())
    }
    fn prepared_specs<'a>(&self,rows:&[k::ProductFinalRow<'a>])->Result<Vec<k::ProductRowSpec<'a>>,CaptureError> {
        let mut specs=self.adapter.reserve(rows.len())?;
        for r in rows {
            self.capture_entry(AdapterEvent::RowVisit)?;
            let spec=match r.recipe {
                k::ProductRecipe::NonQuantity=>k::ProductRowSpec::mode(r.id,r.case_id,r.body,
                    if r.value.to_bits()==1f64.to_bits(){1}else if r.value.to_bits()==2f64.to_bits(){2}else{0}),
                k::ProductRecipe::DenseParityObservation=>k::ProductRowSpec::parity(r.id,r.case_id,r.body,r.value.to_bits()),
                k::ProductRecipe::ModulusBasisRecord=>Ok(k::ProductRowSpec::material_record(r.id,r.case_id,r.body)),
                _=>k::ProductRowSpec::mechanical(r.id,r.case_id,r.unit,r.body,r.recipe),
            }.map_err(CaptureError::PreparedProof)?;
            self.capture_entry(AdapterEvent::MapWrite)?;specs.push(spec);
        }
        Ok(specs)
    }
    fn prepared_row_index(&self,rows:&[k::ProductFinalRow<'_>],recipe:k::ProductRecipe)->Result<usize,CaptureError> {
        for (i,r) in rows.iter().enumerate() {self.capture_entry(AdapterEvent::RowVisit)?;if r.recipe==recipe{return Ok(i);}}
        Err("prepared row identity".into())
    }
    #[allow(clippy::too_many_arguments)]
    fn prepared_maxima(&self,e:&MechanicsEnvelope,scope:&CaseScope,owner:&k::RetainedSolve,rows:&[k::ProductFinalRow<'_>],
        values:&mut k::ProductValuesBuilder,work:&mut ScalarWork)->Result<(Vec<PreparedMaximumPatch>,Vec<k::ProductMaximumValue>),CaptureError> {
        #[cfg(test)] if self.trace_fault==Some(trace::TraceFault::Maxima){return Err(CaptureError::Storage("trace maximum allocation fault"));}
        let evidence=e.contract_evidence.as_ref().ok_or("prepared maximum evidence")?;
        // B3b-P (P-7): the route's per-case evidence entry (`exact_cases[c]` on the exact route).
        let extrema=evidence[scope.route.evidence_cases()][scope.evidence]["pipe_stress_extrema"].as_array().ok_or("prepared maximum records")?;
        if extrema.len()!=self.members.len(){return Err("prepared maximum complete domain".into());}
        let mut patches=self.adapter.reserve(self.members.len())?;
        let mut outputs=self.adapter.reserve(self.members.len())?;
        for (mi,m) in owner.source().members().iter().enumerate() {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            let f=&self.facts[mi];let id=&self.members[mi].id;
            let row=self.prepared_row_index(rows,k::ProductRecipe::CircularMaximum{member:m.id})?;
            let mut found=None;
            for (i,x) in extrema.iter().enumerate() {
                self.capture_entry(AdapterEvent::RowVisit)?;
                if self.checked_same(x["pipe_id"].as_str().ok_or("maximum pipe id")?,id)? && self.checked_same(x["result_id"].as_str().ok_or("maximum result id")?,rows[row].id)? {
                    if found.is_some(){return Err("prepared maximum identity".into());}
                    found=Some(i);
                }
            }
            let index=found.ok_or("prepared maximum identity")?;
            for key in PREPARED_MAX_KEYS {self.capture_entry(AdapterEvent::ValidationEntry)?;if extrema[index][key].as_number().is_none(){return Err("prepared maximum numeric slot".into());}}
            let mut ends=[0f64;12];
            for (j,v) in ends.iter_mut().enumerate() {
                self.capture_entry(AdapterEvent::RowVisit)?;
                let q=k::QuantityId::EndAction{member:m.id,end:if j<6{k::End::I}else{k::End::J},component:k::Component::ALL[j%6]};
                let at=self.prepared_row_index(rows,k::ProductRecipe::Native(q))?;
                *v=values.value(at).map_err(CaptureError::PreparedProof)?.ok_or("unprojected endpoint action")?;
            }
            let source=owner.source();
            let pipe=StraightPipeElement{element_id:self.adapter.copy(id)?,
                node_i:FrameNode{index:m.node_i as usize,coordinates:source.nodes()[m.node_i as usize]},
                node_j:FrameNode{index:m.node_j as usize,coordinates:source.nodes()[m.node_j as usize]},
                section:StraightPipeSectionProperties{elastic_modulus:m.elastic_modulus,shear_modulus:m.shear_modulus,
                    area:m.area,second_moment_y:m.second_moment_y,second_moment_z:m.second_moment_z,
                    torsion_constant:m.torsion_constant,mass_per_length:None},y_reference:m.y_reference};
            let section=DerivedSection{area:f.area,internal_area:0.0,second_moment:f.second_moment,
                torsion_constant:f.torsion_constant,section_modulus:f.section_modulus,torsion_radius:f.radius,
                membrane_radius:f.radius,wall_thickness:f.effective_wall};
            self.capture_entry(AdapterEvent::LibraryBoundary)?;
            let maximum=exact_straight_summary_extrema(&pipe,&ends,&[],&section,None).map_err(CaptureError::Association)?;
            let gap=work.op(ScalarOperation::Sub,maximum.value_upper,maximum.value_lower).map_err(CaptureError::PreparedArithmetic)?;
            let half=work.op(ScalarOperation::Mul,0.5,gap).map_err(CaptureError::PreparedArithmetic)?;
            let value=work.op(ScalarOperation::Add,maximum.value_lower,half).map_err(CaptureError::PreparedArithmetic)?;
            let n=|v:f64|{self.capture_entry(AdapterEvent::MapWrite)?;serde_json::Number::from_f64(v).ok_or_else(||CaptureError::Association("maximum number range".into()))};
            let nu=|v:u64|->Result<serde_json::Number,CaptureError>{self.capture_entry(AdapterEvent::MapWrite)?;Ok(serde_json::Number::from(v))};
            let numbers=[n(maximum.station)?,nu(u64::try_from(maximum.span_index).map_err(|_|CaptureError::CountRange("maximum span index"))?)?,n(maximum.local_fraction)?,
                n(maximum.value_lower)?,n(maximum.value_upper)?,n(maximum.upper_bound)?,n(maximum.certified_gap)?,
                nu(u64::try_from(maximum.subdivisions).map_err(|_|CaptureError::CountRange("maximum subdivisions"))?)?];
            let output=k::ProductMaximumValue::new(m.id,row,value).map_err(CaptureError::PreparedProof)?;
            self.capture_entry(AdapterEvent::MapWrite)?;outputs.push(output);
            self.capture_entry(AdapterEvent::MapWrite)?;patches.push(PreparedMaximumPatch{evidence_index:index,row,member:m.id,numbers});
        }
        Ok((patches,outputs))
    }
    /// B3b-P (B3-D P-8, §1.4): the exact route's section evidence regenerated for the owner case:
    /// for each member, its `pipe_sections` entry in the case's `exact_cases` entry (matched by
    /// `pipe_id`) takes the prepared A, I, J and Z (the facts after preparation). OD, wall, the
    /// radii, Ai, the basis and the order are unchanged.
    fn prepared_sections(&self,e:&MechanicsEnvelope,scope:&CaseScope)->Result<Vec<PreparedSectionPatch>,CaptureError> {
        let evidence=e.contract_evidence.as_ref().ok_or("prepared section evidence")?;
        let sections=evidence[scope.route.evidence_cases()][scope.evidence]["pipe_sections"].as_array().ok_or("prepared section records")?;
        if sections.len()!=self.members.len(){return Err("prepared section complete domain".into());}
        let mut patches=self.adapter.reserve(self.members.len())?;
        for (mi,member) in self.members.iter().enumerate() {
            self.capture_entry(AdapterEvent::SourceVisit)?;
            let f=self.facts.get(mi).ok_or("prepared section facts")?;
            let mut found=None;
            for (i,x) in sections.iter().enumerate() {
                self.capture_entry(AdapterEvent::RowVisit)?;
                if self.checked_same(x["pipe_id"].as_str().ok_or("prepared section pipe id")?,&member.id)? {
                    if found.is_some(){return Err("prepared section identity".into());}
                    found=Some(i);
                }
            }
            let index=found.ok_or("prepared section identity")?;
            for key in PREPARED_SECTION_KEYS {self.capture_entry(AdapterEvent::ValidationEntry)?;if sections[index][key].as_number().is_none(){return Err("prepared section numeric slot".into());}}
            let n=|v:f64|{self.capture_entry(AdapterEvent::MapWrite)?;serde_json::Number::from_f64(v).ok_or_else(||CaptureError::Association("section number range".into()))};
            let numbers=[n(f.area)?,n(f.second_moment)?,n(f.torsion_constant)?,n(f.section_modulus)?];
            self.capture_entry(AdapterEvent::MapWrite)?;patches.push(PreparedSectionPatch{evidence_index:index,member:f.member,numbers});
        }
        Ok(patches)
    }
    fn prepared_alias(&self,rows:&[ResultItem],values:&k::FrozenProductValues,kind:&str)->Result<LocatedQuantity,CaptureError> {
        let mut best=None;
        for (i,r) in rows.iter().enumerate() {
            self.capture_entry(AdapterEvent::RowVisit)?;
            if r.kind!=kind {continue;}
            let value=*values.value(i).ok_or("alias value")?;
            if best.is_none_or(|(j,v):(usize,f64)|value>v || (value==v && r.entity_ref<rows[j].entity_ref)) {best=Some((i,value));}
        }
        let (i,value)=best.ok_or("alias complete domain")?;let r=&rows[i];
        Ok(LocatedQuantity{value,unit:self.adapter.copy(&r.unit)?,location_ref:self.adapter.copy(&r.entity_ref)?,result_ref:self.adapter.copy(&r.id)?})
    }
}
impl PreparedCase {
    /// The private driver's all-in-one form: freeze, then commit the overlay into
    /// the owned ordinary envelope (unchanged behaviour and bytes).
    pub(super) fn project_candidate(self)->Result<PrivatePreparedCandidate,PreparedCandidateRefusal> {
        self.freeze_candidate().map(FrozenCandidate::commit_private)
    }
}
impl ProductCapture {
    /// B1 SP (DESIGN_v2 T-9): one case's freeze, on the case in the capture's own fields and
    /// within its `scope` of the one untouched ordinary owner, with that case's `native` Run
    /// in the invocation's call: the dual-readout proof, the projection, maxima, values and
    /// aliases, the certificate, observables and G5a, then the commit plan's precharge. The
    /// trace's snapshot is taken at its last stage. Shared by the one-case `freeze_candidate`
    /// (the whole envelope) and the n-case transaction (`PreparedCases::freeze`).
    #[allow(clippy::too_many_arguments)]
    fn freeze_case(&mut self,trace:&mut trace::PreparedTrace,overlay_work:&mut ScalarWork,proof_attempted:&mut bool,
        ordinary:&MechanicsEnvelope,scope:&CaseScope,native:Option<(&k::RecordedInvocation,&k::RecordedCase)>)
        ->Result<(PreparedPayload,k::CertifiedProductProof),RefusedCase> {
        let mut certificate=None;let mut saved_values=None;
        let result=(||->Result<PreparedPayload,PreparedCandidateError>{
            if *proof_attempted {return Err(PreparedCandidateError::Capture(CaptureError::PreparedAttemptConsumed));}
            let (invocation,case)=native.ok_or_else(||PreparedCandidateError::Capture("missing prepared native owner".into()))?;
            let k::ExecutionOutcome::Selected(owner)=&case.outcome else{return Err(PreparedCandidateError::Capture(CaptureError::NativeUnavailable));};
            let base_rows=self.bind_rows_view(ProductCaseView::of(ordinary,scope),owner).map_err(PreparedCandidateError::Capture)?;
            let specs=self.prepared_specs(&base_rows).map_err(PreparedCandidateError::Capture)?;
            self.capture_entry(AdapterEvent::MapWrite).map_err(PreparedCandidateError::Capture)?;
            *proof_attempted=true;
            trace.enter(trace::Stage::ProofStart);
            let draft=invocation.begin_prepared_product(case.run,owner,&self.facts,&specs).into_ready().map_err(PreparedCandidateError::Proof)?;
            trace.completed(trace::Stage::ProofStart);
            #[cfg(test)] println!("I51_DUAL_LANES {:?}",draft.lane_debug());
            trace.enter(trace::Stage::Projection);
            let (projected,mut builder)=draft.project().into_ready().map_err(PreparedCandidateError::Proof)?;
            trace.completed(trace::Stage::Projection);trace.enter(trace::Stage::Maxima);
            let (patches,maxima)=match self.prepared_maxima(ordinary,scope,owner,&base_rows,&mut builder,overlay_work) {
                Ok(v)=>v,Err(cause)=>return Err(PreparedCandidateError::Abandoned{cause,proof:projected.abandon_values(builder.abandon())})};
            // B3b-P (P-8): the exact route's section evidence, beside its maxima (same stage).
            let sections=match scope.route {
                W1Route::Preview=>Vec::new(),
                W1Route::Exact=>match self.prepared_sections(ordinary,scope) {
                    Ok(v)=>v,Err(cause)=>return Err(PreparedCandidateError::Abandoned{cause,proof:projected.abandon_values(builder.abandon())})},
            };
            trace.completed(trace::Stage::Maxima);trace.enter(trace::Stage::Values);
            #[cfg(test)] let maxima_input=if self.trace_fault==Some(trace::TraceFault::ValuesCompletion){&maxima[..0]}else{&maxima[..]};
            #[cfg(not(test))] let maxima_input=&maxima[..];
            let (values,value_work)=match builder.complete_maxima(maxima_input).into_ready() {
                Ok(v)=>v,Err(failure)=>return Err(PreparedCandidateError::Values{failure,proof:projected.abandon()})};
            trace.completed(trace::Stage::Values);trace.enter(trace::Stage::Aliases);
            let displacement=match self.prepared_alias(ProductCaseView::of(ordinary,scope).case_rows(),&values,"displacement_magnitude") {
                Ok(v)=>v,Err(cause)=>return Err(PreparedCandidateError::Abandoned{cause,proof:projected.abandon_values(value_work)})};
            let stress=match self.prepared_alias(ProductCaseView::of(ordinary,scope).case_rows(),&values,"pipe_elastic_normal_stress_maximum_v2") {
                Ok(v)=>v,Err(cause)=>return Err(PreparedCandidateError::Abandoned{cause,proof:projected.abandon_values(value_work)})};
            trace.completed(trace::Stage::Aliases);
            let payload=PreparedPayload{values,maxima:patches,displacement,stress,sections};
            let view=ProductCaseView{ordinary,payload:Some(&payload),scope,frozen:None};
            let rows=match self.bind_rows_view(view,owner) {
                Ok(v)=>v,Err(cause)=>return Err(PreparedCandidateError::Abandoned{cause,proof:projected.abandon_values(value_work)})};
            trace.enter(trace::Stage::Certificate);
            let certified=match projected.certify_final(&payload.values,&rows,value_work).into_ready() {
                Ok(v)=>{trace.checked(trace::Stage::Certificate,0,true);trace.costs.record::<bool>();trace.proof_ready=true;v},Err(failure)=>{
                    trace.checked(trace::Stage::Certificate,0,false);
                    self.numeric_pass=false;
                    self.numeric_failure=Some(failure.failure().clone());
                    self.source_correction_calls=failure.work().source_correction_calls();
                    #[cfg(test)]
                    {self.work=format!("{:?}; lanes={:?}",failure.work().work_summary(),failure.work().prepared_lane_work());}
                    if let Err(e)=self.prepared_verdict_copy(failure.work().verdicts(),failure.work().summary_coverage()) {self.error=Some(e);}
                    if self.error.is_none() && self.verdicts.len()==rows.len() {
                        trace.enter(trace::Stage::Observables);
                        self.observable_error=self.observables_view(view).err();
                        trace.checked(trace::Stage::Observables,1,self.observable_error.is_none());
                        trace.enter(trace::Stage::G5a);
                        self.g5a_error=self.g5a(owner,&rows).err();
                        trace.checked(trace::Stage::G5a,2,self.g5a_error.is_none());
                    }
                    #[cfg(test)] println!("I51_FROZEN_REFUSAL {}",serde_json::json!({"mode":self.invocation_mode.unwrap().as_str(),
                        "rows":view.rows().map(|r|serde_json::json!({"id":r.id,"kind":r.kind,"unit":r.unit,"value":r.value,"bits":format!("{:016x}",r.value.to_bits())})).collect::<Vec<_>>(),
                        "verdicts":self.verdicts.iter().map(|v|serde_json::json!({"row":v.row,"n":format!("{:016x}",v.normalized_bits),"scale":format!("{:016x}",v.scale_bits),"class":format!("{:?}",v.class),"passed":v.passed,"predicates":v.predicates})).collect::<Vec<_>>(),
                        "maxima":payload.maxima.iter().map(|m|serde_json::json!({"row":m.row,"member":m.member,"numbers":m.numbers})).collect::<Vec<_>>(),
                        "numeric_pass":false,"observables":format!("{:?}",self.observable_error),"g5a":format!("{:?}",self.g5a_error),"work":self.work}));
                    drop(rows);saved_values=Some(payload.values);return Err(PreparedCandidateError::Proof(failure));
                }
            };
            if !certified.matches_values(&payload.values){
                certificate=Some(certified);drop(rows);saved_values=Some(payload.values);
                return Err(PreparedCandidateError::Capture("frozen proof/value owner".into()));
            }
            if let Err(e)=self.prepared_verdict_copy(certified.verdicts(),certified.summary_coverage()) {
                certificate=Some(certified);drop(rows);saved_values=Some(payload.values);
                return Err(PreparedCandidateError::Capture(e));
            }
            self.numeric_pass=certified.passed();self.source_correction_calls=certified.work().source_correction_calls();
            #[cfg(test)]
            {self.work=format!("{:?}; lanes={:?}",certified.work().work_summary(),certified.work().prepared_lane_work());}
            trace.enter(trace::Stage::Observables);
            self.observable_error=self.observables_view(view).err();
            trace.checked(trace::Stage::Observables,1,self.observable_error.is_none());
            trace.enter(trace::Stage::G5a);
            self.g5a_error=self.g5a(owner,&rows).err();
            trace.checked(trace::Stage::G5a,2,self.g5a_error.is_none());
            #[cfg(test)] println!("I51_FROZEN_ROWS {}",serde_json::json!({"rows":view.rows().map(|r|serde_json::json!({"id":r.id,"kind":r.kind,"unit":r.unit,"value":r.value,"bits":format!("{:016x}",r.value.to_bits())})).collect::<Vec<_>>(),
                "verdicts":self.verdicts.iter().map(|v|serde_json::json!({"row":v.row,"n":format!("{:016x}",v.normalized_bits),"scale":format!("{:016x}",v.scale_bits),"class":format!("{:?}",v.class),"passed":v.passed,"predicates":v.predicates})).collect::<Vec<_>>(),
                "numeric_pass":self.numeric_pass,"observables":format!("{:?}",self.observable_error),"g5a":format!("{:?}",self.g5a_error),"work":self.work}));
            let pass=self.full_case_passed();certificate=Some(certified);
            drop(rows);drop(base_rows);drop(specs);
            if !pass {
                saved_values=Some(payload.values);
                return Err(if self.observable_error.is_some(){PreparedCandidateError::Observable}
                    else if self.g5a_error.is_some(){PreparedCandidateError::G5a}else{PreparedCandidateError::Numeric});
            }
            // The finite move plan is checked/charged before any ordinary mutation.
            // B3b-P: the exact route's section patches move four values each (none on the preview route).
            let section_moves=payload.sections.len().checked_mul(4).ok_or_else(||PreparedCandidateError::Capture(CaptureError::CountRange("commit sections")))?;
            let moves=payload.values.len().checked_add(payload.maxima.len().checked_mul(8).ok_or_else(||PreparedCandidateError::Capture(CaptureError::CountRange("commit maxima")))?)
                .and_then(|v|v.checked_add(section_moves))
                .and_then(|v|v.checked_add(2)).ok_or_else(||PreparedCandidateError::Capture(CaptureError::CountRange("commit moves")))?;
            self.adapter.enter(AdapterEvent::MapWrite,u64::try_from(moves).map_err(|_|PreparedCandidateError::Capture(CaptureError::CountRange("commit count")))?);
            self.adapter.require().map_err(PreparedCandidateError::Capture)?;
            trace.costs.record::<bool>();trace.private_commit_precharged=true;
            Ok(payload)
        })();
        match result {
            Err(error)=>{trace.fail_entered();trace.freeze(self);Err(RefusedCase{error,certificate,values:saved_values})},
            Ok(payload)=>{
                // The adapter snapshot is final here: no adapter event follows.
                trace.freeze(self);
                match certificate {
                    Some(certificate)=>Ok((payload,certificate)),
                    None=>{trace.fail_entered();Err(RefusedCase{error:PreparedCandidateError::Capture("frozen candidate without certificate".into()),
                        certificate:None,values:Some(payload.values)})}
                }
            }
        }
    }
}
impl PreparedCase {
    /// U3 (I51 frozen-candidate split): every private gate, proof and move-plan
    /// check runs (`ProductCapture::freeze_case` over the whole envelope), and the ordinary
    /// envelope is returned **untouched** beside the frozen payload. No ordinary mutation
    /// happens here.
    fn freeze_candidate(mut self)->Result<FrozenCandidate,PreparedCandidateRefusal> {
        let ordinary=self.ordinary.take().expect("closed owning preparation transition");
        let (invocation,case)=(self.capture.native_invocation.take(),self.capture.native.take());
        let frozen=self.capture.freeze_case(&mut self.trace,&mut self.overlay_work,&mut self.proof_attempted,&ordinary,&CaseScope::whole(self.capture.route),
            invocation.as_ref().zip(case.as_ref()));
        (self.capture.native_invocation,self.capture.native)=(invocation,case);
        match frozen {
            Ok((payload,certificate))=>Ok(FrozenCandidate{ordinary,payload,prepared:self,certificate}),
            Err(RefusedCase{error,certificate,values})=>Err(PreparedCandidateRefusal{ordinary,prepared:self,error,certificate,values}),
        }
    }
}

/// The frozen overlay of one case (its rows' values and maxima patches, and, when the case
/// owns the whole envelope, the summary aliases) applied to one envelope within the case's
/// `scope`. Shared by the private commit and the facade's staging copy, so both produce the
/// same rows. Typed (RV85 N6): a broken invariant names its site and the facade falls back
/// with the untouched ordinary owner; nothing here panics.
fn apply_prepared_overlay(envelope:&mut MechanicsEnvelope,payload:&PreparedPayload,scope:&CaseScope)->Result<(),StagingFault> {
    let rows=match &scope.rows {None=>&mut envelope.results[..],Some(rows)=>envelope.results.get_mut(rows.clone()).ok_or(StagingFault("results"))?};
    for (i,row) in rows.iter_mut().enumerate(){row.value=*payload.values.value(i).ok_or(StagingFault("values"))?;}
    let key=scope.route.evidence_cases();
    let cases=envelope.contract_evidence.as_mut().and_then(|e|e.as_object_mut()).and_then(|e|e.get_mut(key))
        .and_then(|c|c.as_array_mut()).ok_or(StagingFault(key))?;
    let case=cases.get_mut(scope.evidence).and_then(|c|c.as_object_mut()).ok_or(StagingFault("pipe_stress_extrema"))?;
    let extrema=case.get_mut("pipe_stress_extrema").and_then(|x|x.as_array_mut()).ok_or(StagingFault("pipe_stress_extrema"))?;
    for patch in &payload.maxima {
        let object=extrema.get_mut(patch.evidence_index).and_then(|x|x.as_object_mut()).ok_or(StagingFault("pipe_stress_extrema[]"))?;
        for (key,number) in PREPARED_MAX_KEYS.into_iter().zip(patch.numbers.iter()){
            *object.get_mut(key).ok_or(StagingFault("pipe_stress_extrema[].key"))?=serde_json::Value::Number(number.clone());
        }
    }
    if scope.route==W1Route::Exact {
        apply_exact_section_overlay(case,payload)?;
    }
    // The one requested case's aliases are the headlines (B2-P T-11′: at c = 1 its scope is its
    // own block when a combination follows; headlines cover load cases only).
    if scope.cases==1 {
        envelope.summary.max_displacement=Some(payload.displacement.clone());envelope.summary.max_open_formula_stress=Some(payload.stress.clone());
    }
    Ok(())
}
/// B3b-P (B3-D P-8, §1.4; DEF-E `evidence.pipe_sections`): in the owner case's `exact_cases`
/// entry only, each member's `pipe_sections` entry takes the prepared As, I, J and Z.
fn apply_exact_section_overlay(case:&mut serde_json::Map<String,serde_json::Value>,payload:&PreparedPayload)->Result<(),StagingFault> {
    let sections=case.get_mut("pipe_sections").and_then(|x|x.as_array_mut()).ok_or(StagingFault("pipe_sections"))?;
    for patch in &payload.sections {
        let object=sections.get_mut(patch.evidence_index).and_then(|x|x.as_object_mut()).ok_or(StagingFault("pipe_sections[]"))?;
        for (key,number) in PREPARED_SECTION_KEYS.into_iter().zip(patch.numbers.iter()){
            *object.get_mut(key).ok_or(StagingFault("pipe_sections[].key"))?=serde_json::Value::Number(number.clone());
        }
    }
    Ok(())
}
/// B1 SP (DESIGN_v2 T-11, several requested cases): the summary's two headlines over the
/// staged rows, each only where the ordinary envelope has one (a headline covers the whole
/// requested domain; lib.rs `maximum_across_cases`). The headline is the row of its kind with
/// the greatest value; a tie goes to the smaller case id, then the smaller location, as
/// `maximum_across_cases` orders the cases' headlines and each frozen case's aliases order its
/// rows. Unless a frozen case's rows hold it, this is the ordinary headline's row. Its result
/// reference is the row's own (case-qualified) id.
fn stage_headlines(staged:&mut MechanicsEnvelope,ordinary:&MechanicsEnvelope)->Result<(),StagingFault> {
    for (kind,stress) in [("displacement_magnitude",false),("pipe_elastic_normal_stress_maximum_v2",true)] {
        let present=if stress{ordinary.summary.max_open_formula_stress.is_some()}else{ordinary.summary.max_displacement.is_some()};
        if !present {continue;}
        let mut best:Option<(&ResultItem,&str)>=None;
        // B2-P (T-11′): load-case rows only, as `maximum_across_cases` and the base readers'
        // headline (a combination's magnitude is never a headline).
        for row in staged.results.iter().filter(|row|row.kind==kind) {
            let basis=row.basis_ref.as_ref().ok_or(StagingFault("results[].basis_ref"))?;
            if basis.ref_type!="load_case" {continue;}
            let case=basis.ref_id.as_str();
            if best.is_none_or(|(b,c)|row.value>b.value || (row.value==b.value && (case,row.entity_ref.as_str())<(c,b.entity_ref.as_str()))) {
                best=Some((row,case));
            }
        }
        let (row,_)=best.ok_or(StagingFault("summary"))?;
        let headline=LocatedQuantity{value:row.value,unit:row.unit.clone(),location_ref:row.entity_ref.clone(),result_ref:row.id.clone()};
        if stress {staged.summary.max_open_formula_stress=Some(headline);} else {staged.summary.max_displacement=Some(headline);}
    }
    Ok(())
}
/// RV85 N6: the overlay site whose invariant did not hold.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub(super) struct StagingFault(pub(super) &'static str);

/// U3 (I51 frozen-candidate split): a certified, private-gate-passed one-case candidate
/// whose ordinary envelope is still intact, for the private driver's commit. (The facade
/// stages its successor from `PreparedCases`, B1 SP T-11.)
pub(super) struct FrozenCandidate { ordinary:MechanicsEnvelope,payload:PreparedPayload,prepared:PreparedCase,
    certificate:k::CertifiedProductProof }
impl FrozenCandidate {
    /// The private driver's commit (unchanged bytes): the overlay moves into the
    /// owned ordinary envelope.
    fn commit_private(mut self)->PrivatePreparedCandidate {
        let mut envelope=self.ordinary;
        // The private driver only (tests): its bytes are unchanged and a broken
        // invariant still stops it here, as before the facade existed.
        apply_prepared_overlay(&mut envelope,&self.payload,&CaseScope::whole(self.prepared.capture.route)).expect("frozen overlay invariant");
        self.prepared.trace.costs.record::<bool>();self.prepared.trace.private_committed=true;self.prepared.trace.freeze(&self.prepared.capture);
        PrivatePreparedCandidate{envelope,prepared:self.prepared,certificate:self.certificate}
    }
}

#[cfg(test)]
impl PreparedCandidateRefusal {
    pub(super) fn test_reentry(mut self)->(Self,PreparedCandidateError) {
        let old=self.error;let counts=self.prepared.capture.adapter.counts.get();
        self.prepared.ordinary=Some(self.ordinary);
        let next=match self.prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("reentry succeeded")};
        assert_eq!(counts,next.capture().adapter.counts.get());
        assert!(matches!(&next.error,PreparedCandidateError::Capture(CaptureError::PreparedAttemptConsumed)));
        (next,old)
    }
}
#[cfg(test)]
impl PreparedCandidateRefusal {
    /// I61 U2 (failure path) control: exchange the refusal causes (and their proof
    /// work) of two refusals whose public facts are identical.
    pub(super) fn test_swap_error(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.error, &mut other.error);
    }
    /// The same for a commit refusal's surviving certified proof.
    pub(super) fn test_swap_certificate(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.certificate, &mut other.certificate);
    }
}
#[cfg(test)]
impl PrivatePreparedCandidate {
    /// I61 U2 control: exchange the certified proofs of two candidates whose public
    /// facts are identical, so each carries a proof of a foreign selected owner.
    pub(super) fn test_swap_certificate(&mut self, other: &mut Self) {
        std::mem::swap(&mut self.certificate, &mut other.certificate);
    }
    pub(super) fn test_reentry(mut self)->(PreparedCandidateRefusal,k::CertifiedProductProof) {
        let counts=self.prepared.capture.adapter.counts.get();
        self.prepared.ordinary=Some(self.envelope);
        let next=match self.prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("success reentry succeeded")};
        assert_eq!(counts,next.capture().adapter.counts.get());
        (next,self.certificate)
    }
}

#[cfg(test)]
pub(super) fn i51_overlay_layout() {
    macro_rules! layout {($t:ty)=>{println!("I51_OVERLAY_LAYOUT {} {} {}",stringify!($t),std::mem::size_of::<$t>(),std::mem::align_of::<$t>());};}
    layout!(PreparedCase);layout!(PreparedAssociation);layout!(PreparedMaximumPatch);layout!(PreparedPayload);layout!(ProductCaseView<'static>);
    layout!(ProductRowView<'static>);layout!(PrivatePreparedCandidate);layout!(PreparedCandidateRefusal);
    layout!(serde_json::Number);layout!(LocatedQuantity);layout!(ScalarWork);
}

// Closed terminal projections borrow the original producing owners. They never
// reconstruct entered work from successful-vector cardinality or Debug strings.
impl PreparedCaseFailure {
    pub(super) fn typed_trace<'a>(&'a self,costs:&mut trace::ProjectionWork)->Result<trace::PreparedAttemptView<'a>,trace::TraceProjectionError> {
        static ZERO:ScalarWork=ScalarWork{entered:0,checks:0,lost:false};
        trace::project(&self.trace,&self.capture,&self.preparation_work,&self.old_operational,&ZERO,
            trace::ResultRef::Unavailable(trace::FailureRef::Preparation{capture:self.capture.error.as_ref().ok_or(trace::TraceProjectionError::MissingFailure)?,section:self.preparation_error.as_ref()}),
            None,None,None,costs)
    }
}
impl PreparedCase {
    pub(super) fn native_refusal_trace<'a>(&'a self,costs:&mut trace::ProjectionWork)->Result<trace::PreparedAttemptView<'a>,trace::TraceProjectionError> {
        trace::project(&self.trace,&self.capture,&self.preparation_work,&self.old_operational,&self.overlay_work,
            trace::ResultRef::Unavailable(trace::FailureRef::Native(self.trace.native_error.as_ref().ok_or(trace::TraceProjectionError::MissingFailure)?)),None,None,None,costs)
    }
}
impl PrivatePreparedCandidate {
    pub(super) fn typed_trace<'a>(&'a self,costs:&mut trace::ProjectionWork)->Result<trace::PreparedAttemptView<'a>,trace::TraceProjectionError> {
        let p=&self.prepared;
        trace::project(&p.trace,&p.capture,&p.preparation_work,&p.old_operational,&p.overlay_work,
            trace::ResultRef::Ready,Some(self.certificate.work()),None,None,costs)
    }
}
impl PreparedCandidateRefusal {
    pub(super) fn typed_trace<'a>(&'a self,costs:&mut trace::ProjectionWork)->Result<trace::PreparedAttemptView<'a>,trace::TraceProjectionError> {
        let p=&self.prepared;
        let failure=match &self.error {PreparedCandidateError::Proof(e)=>Some(e),PreparedCandidateError::Values{proof,..}|PreparedCandidateError::Abandoned{proof,..}=>Some(proof),_=>None};
        let work=failure.map(|f|f.work()).or_else(||self.certificate.as_ref().map(|c|c.work()));
        let values=match &self.error{PreparedCandidateError::Values{failure,..}=>Some(failure),_=>None};
        trace::project(&p.trace,&p.capture,&p.preparation_work,&p.old_operational,&p.overlay_work,
            trace::ResultRef::Unavailable(trace::FailureRef::Candidate(&self.error)),work,failure.map(|f|f.failure()),values,costs)
    }
}

// ---- B2-P (B2-C §2.3–§2.7; REVISION_01 §1.2, §3.3; DEF-C): T-10a, T-10b and the combinations' staging ----

/// B2-P (B2-C §2.4 (i); KD §1.2): where one combination operand's source comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OperandSource {
    /// A selected case: its `RetainedSolve` (cache imports allowed).
    Selected,
    /// An `unavailable` case whose batch registered its CaseSource: a `PreparedCaseSource`
    /// rebuilt once from its prepared source (`PreparedCases::rebuilt[prepared]`), no import, at
    /// its batch-registered `source` id.
    Rebuilt { prepared: usize, source: usize },
    /// A `not_required` case: its operand preparation (`operand_preparations[preparation]`).
    Prepared { preparation: usize },
}
/// One operand of a retained combination, in authored term order.
#[derive(Debug, Clone, Copy)]
pub(super) struct CombinationOperand {
    /// The term's load case (request index) and factor.
    pub case: usize,
    pub factor: f64,
    pub source: OperandSource,
}
/// B2-P: the per-attempt result fields a combination's freeze writes. They are swapped into
/// the capture over operand 0's slot while the freeze or its projection runs (REVISION_01
/// N-9: the id maps and member facts are operand 0's; the outcome is the combination's own),
/// and swapped back after, so operand 0's own case keeps its results.
#[derive(Default)]
pub(super) struct CombinationResults {
    native: Option<k::RecordedCase>,
    error: Option<CaptureError>,
    verdicts: Vec<k::ProductRowVerdict>,
    numeric_failure: Option<k::ProductFailure>,
    numeric_pass: bool,
    observable_error: Option<CaptureError>,
    g5a_work: ScalarWork,
    g5a_error: Option<G5aFailure>,
    summary_coverage: Vec<k::ProductSummaryCoverage>,
    source_correction_calls: Option<k::WorkTotal>,
    work: String,
}
impl ProductCapture {
    /// Exchange the capture's per-attempt result fields with a combination's. Moves only.
    fn swap_results(&mut self, r: &mut CombinationResults) {
        std::mem::swap(&mut self.native, &mut r.native);
        std::mem::swap(&mut self.error, &mut r.error);
        std::mem::swap(&mut self.verdicts, &mut r.verdicts);
        std::mem::swap(&mut self.numeric_failure, &mut r.numeric_failure);
        std::mem::swap(&mut self.numeric_pass, &mut r.numeric_pass);
        std::mem::swap(&mut self.observable_error, &mut r.observable_error);
        std::mem::swap(&mut self.g5a_work, &mut r.g5a_work);
        std::mem::swap(&mut self.g5a_error, &mut r.g5a_error);
        std::mem::swap(&mut self.summary_coverage, &mut r.summary_coverage);
        std::mem::swap(&mut self.source_correction_calls, &mut r.source_correction_calls);
        std::mem::swap(&mut self.work, &mut r.work);
    }
}
/// B2-P (B2-C §2.3–§2.6): where one model combination ended.
pub(super) enum CombinationEnd {
    /// T-10a rule 1: its gate entry is withheld; the gate's code.
    BaseWithheld(String),
    /// T-10a rule 3: `ordinary`, `no_retained_mechanics`.
    Ordinary,
    /// T-10a rule 2: retained; T-10b has not decided it yet.
    Retained,
    /// T-10b (i): an operand has no usable source (C-4), the first such term.
    OperandSourceUnavailable { operand_index: usize },
    /// T-10b (iii) 1: a `not_required` operand's preparation was refused (the first such term).
    OperandPreparationFailure { operand_preparation: usize },
    /// T-10b (iii) 2: the Call refused before any source (`pre_source_refusal`).
    PreSource { stage: k::CombinationStage, reason: k::CombinationReason },
    /// T-10b (iii) 3: its Run did not end selected (`combination_unresolved`, phase `kernel`).
    Native,
    /// T-10b (iii) 4: frozen: certified, observables and G5a passed (`retained_selected`).
    Frozen(FrozenCombination),
    /// T-10b (iii) 4: the freeze refused after a selected Run (`facade_certificate`).
    Candidate(RefusedCase),
}
/// A frozen combination's values and certificate. The ordinary owner is not touched.
pub(super) struct FrozenCombination { values: k::FrozenProductValues, certificate: k::CertifiedProductProof }
impl FrozenCombination {
    pub(super) fn certificate(&self) -> &k::CertifiedProductProof { &self.certificate }
}
/// B2-P: one model combination's W1 record, in authored order.
pub(super) struct CombinationAttempt {
    /// Its authored index (`model.combinations`).
    pub index: usize,
    pub end: CombinationEnd,
    /// T-10b (i): its operands, in authored term order (a retained combination's).
    pub operands: Vec<CombinationOperand>,
    /// Its `MechanicsCombinationCall`'s id in the invocation, when it made one.
    pub call: Option<usize>,
    /// Its `CombinationAttempt`'s id in `product_attempts[]` (actual start order), when its Call
    /// returned a Run.
    pub attempt: Option<usize>,
    pub trace: trace::PreparedTrace,
    proof_attempted: bool,
    overlay_work: ScalarWork,
    results: CombinationResults,
}
impl CombinationAttempt {
    fn new(index: usize, end: CombinationEnd) -> Self {
        Self { index, end, operands: Vec::new(), call: None, attempt: None, trace: trace::PreparedTrace::default(),
            proof_attempted: false, overlay_work: ScalarWork::default(), results: CombinationResults::default() }
    }
    /// Its Run, when the Call returned one.
    pub(super) fn native(&self) -> Option<&k::RecordedCase> { self.results.native.as_ref() }
    /// Whether it is `retained_selected` or `retained_unavailable` (T-10a rule 2).
    pub(super) fn retained(&self) -> bool { !matches!(self.end, CombinationEnd::BaseWithheld(_) | CombinationEnd::Ordinary) }
    /// Its operand 0's case (its freeze's slot: REVISION_01 N-9).
    pub(super) fn representative(&self) -> Option<usize> { self.operands.first().map(|operand| operand.case) }
}
/// B2-P (C3a; DESIGN §4.2 P1): one `not_required` case's operand preparation: C3's preparation
/// stage on its own capture slot, shared by every retained combination that needs it.
pub(super) struct OperandPreparation {
    /// Its id: its position in first-need order (C3a-1).
    pub id: usize,
    /// The owner case (request index).
    pub owner: usize,
    /// The ascending authored indices of the combinations whose terms name the owner (C3a-1).
    pub requested_by: Vec<usize>,
    /// When prepared: its registered source id and kernel prep (C3a-2).
    pub source: Option<usize>,
    pub prepared: Option<k::PreparedCaseSource>,
    pub parts: AttemptParts,
    pub trace: trace::PreparedTrace,
}
/// B2-P (B2-C §2.6; REVISION_01 N-1, N-5): the whole successor is abandoned at T-10b
/// (`W1Fallback::CombinationCustody`): an `OriginError` or `OriginRefusal` from a combination's
/// custody, a `PreparedCaseSource::new` refusal after a completed operand preparation, or a
/// rebuild refusal. Never a capture error and never serialized (N-5). The site, for tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CombinationCustody(pub &'static str);
/// The kernel's case limit for each Run, a combination's included (C1 §2; W1-LME-20B-60B-v1).
const COMBINATION_CASE_LIMIT: u64 = 20_000_000_000;

impl PreparedCases {
    /// B2-P (B2-C §2.3, T-10a; decisions 5 and 20): each model combination's disposition, in
    /// authored order, after T-10's check: its gate entry withheld → `base_withheld` with the
    /// gate's code; a `mechanics` combination whose terms name distinct load cases (C-1), one of
    /// them selected (frozen) → retained; anything else → `ordinary`. No W1 work and no event.
    pub(super) fn dispositions(&mut self) {
        let gates = self.ordinary.contract_evidence.as_ref().map(|evidence| &evidence["combination_gates"]);
        let selected = |case: usize| self.attempts.iter().any(|attempt| attempt.request == case && matches!(attempt.end, AttemptEnd::Frozen(_)));
        let mut combinations = Vec::with_capacity(self.capture.combinations.len());
        for (index, combination) in self.capture.combinations.iter().enumerate() {
            let gate = gates.map(|gates| &gates[index]);
            let end = if gate.is_some_and(|gate| gate["withheld"] == serde_json::Value::Bool(true)) {
                CombinationEnd::BaseWithheld(gate.and_then(|gate| gate["reason"].as_str()).unwrap_or_default().to_owned())
            } else if combination.mechanics && distinct_terms(&combination.terms) && combination.terms.iter().any(|&(case, _)| selected(case)) {
                CombinationEnd::Retained
            } else {
                CombinationEnd::Ordinary
            };
            combinations.push(CombinationAttempt::new(index, end));
        }
        self.combinations = combinations;
    }
    /// B2-P (B2-C §2.4, T-10b): for the retained combinations, (i) the operand sources, (ii)
    /// the operand preparations and their registrations, in first-need order, and (iii) per
    /// combination in authored order its Call, Run and freeze. Each combination's outcome is its
    /// own; `Err` abandons the whole successor (N-1, N-5).
    pub(super) fn combine(&mut self) -> Result<(), CombinationCustody> {
        if !self.combinations.iter().any(|combination| matches!(combination.end, CombinationEnd::Retained)) {
            return Ok(());
        }
        let invocation = self.capture.native_invocation.take();
        let result = match invocation {
            Some(mut invocation) => {
                let result = self.combine_on(&mut invocation);
                self.capture.native_invocation = Some(invocation);
                result
            }
            // A selected case needs the batch's invocation (T-10 passed), so this cannot happen.
            None => Err(CombinationCustody("invocation")),
        };
        result
    }
    fn combine_on(&mut self, invocation: &mut k::RecordedInvocation) -> Result<(), CombinationCustody> {
        // (i) The operand sources, per retained combination in authored order. The first term
        // with no usable source decides the combination (`operand_source_unavailable`): it then
        // requests no operand preparation and makes no Call (C-4).
        let mut needs: Vec<(usize, usize)> = Vec::new(); // (owner case, combination), in first-need order
        for k in 0..self.combinations.len() {
            if !matches!(self.combinations[k].end, CombinationEnd::Retained) {
                continue;
            }
            let terms = self.capture.combinations[self.combinations[k].index].terms.clone();
            let mut operands = Vec::with_capacity(terms.len());
            let mut decided = None;
            for (operand_index, &(case, factor)) in terms.iter().enumerate() {
                let source = match self.attempts.iter().find(|attempt| attempt.request == case) {
                    Some(attempt) if matches!(attempt.end, AttemptEnd::Frozen(_)) => OperandSource::Selected,
                    // T-7 refused: no CaseSource.
                    Some(attempt) if !attempt.prepared => { decided = Some(operand_index); break; }
                    Some(_) => {
                        let run = self.capture.case_native(case).ok_or(CombinationCustody("operand run"))?;
                        if matches!(&run.outcome, k::ExecutionOutcome::Refused { refusal: k::Refusal::LedgerUnavailable(_), .. }) {
                            decided = Some(operand_index);
                            break;
                        }
                        let source = invocation.runs().get(run.run).map(|origin| origin.source).ok_or(CombinationCustody("operand source"))?;
                        let prepared = match self.rebuilt.iter().position(|(owner, _)| *owner == case) {
                            Some(prepared) => prepared,
                            None => {
                                let primitive = self.capture.case_prepared_source(case).ok_or(CombinationCustody("rebuild source"))?.clone();
                                let rebuilt = k::PreparedCaseSource::new(primitive).map_err(|_| CombinationCustody("rebuild"))?;
                                self.rebuilt.push((case, rebuilt));
                                self.rebuilt.len() - 1
                            }
                        };
                        OperandSource::Rebuilt { prepared, source }
                    }
                    None => {
                        needs.push((case, self.combinations[k].index));
                        OperandSource::Prepared { preparation: usize::MAX }
                    }
                };
                operands.push(CombinationOperand { case, factor, source });
            }
            if let Some(operand_index) = decided {
                self.combinations[k].end = CombinationEnd::OperandSourceUnavailable { operand_index };
                needs.retain(|&(_, combination)| combination != self.combinations[k].index);
            } else {
                self.combinations[k].operands = operands;
            }
        }
        // (ii) The operand preparations, one per owner case in first-need order, each listing
        // every still-retained combination whose terms name the owner (C3a-1).
        let mut owners: Vec<usize> = Vec::new();
        for &(owner, _) in &needs {
            if !owners.contains(&owner) {
                owners.push(owner);
            }
        }
        for owner in owners {
            let requested_by: Vec<usize> = self.combinations.iter()
                .filter(|combination| matches!(combination.end, CombinationEnd::Retained) && combination.operands.iter().any(|operand| operand.case == owner))
                .map(|combination| combination.index).collect();
            let id = self.operand_preparations.len();
            let mut preparation = self.capture.with_case(owner, |capture| capture.prepare_operand(id, owner, requested_by));
            if preparation.trace.source_ready {
                let primitive = self.capture.case_prepared_source(owner).ok_or(CombinationCustody("operand source"))?.clone();
                let prepared = k::PreparedCaseSource::new(primitive).map_err(|_| CombinationCustody("operand prep"))?;
                preparation.source = Some(invocation.register_prepared_source(&prepared).map_err(|_| CombinationCustody("registration"))?);
                preparation.prepared = Some(prepared);
            }
            self.operand_preparations.push(preparation);
            for combination in &mut self.combinations {
                for operand in &mut combination.operands {
                    if operand.case == owner {
                        operand.source = OperandSource::Prepared { preparation: id };
                    }
                }
            }
        }
        // (iii) Per retained combination, in authored order: decided without a Call, or its
        // Call, then its Run's freeze.
        let mut next_attempt = self.attempts.len();
        for k in 0..self.combinations.len() {
            if !matches!(self.combinations[k].end, CombinationEnd::Retained) {
                continue;
            }
            let refused = self.combinations[k].operands.iter().find_map(|operand| match operand.source {
                OperandSource::Prepared { preparation } => self.operand_preparations.get(preparation)
                    .filter(|record| record.source.is_none()).map(|record| record.id),
                _ => None,
            });
            if let Some(operand_preparation) = refused {
                self.combinations[k].end = CombinationEnd::OperandPreparationFailure { operand_preparation };
                continue;
            }
            self.combination_call(invocation, k, &mut next_attempt)?;
        }
        Ok(())
    }
    /// T-10b (iii) 2–4: one retained combination's `MechanicsCombinationCall` on the
    /// invocation's one meter, then, with a selected Run, its own freeze.
    fn combination_call(&mut self, invocation: &mut k::RecordedInvocation, k: usize, next_attempt: &mut usize) -> Result<(), CombinationCustody> {
        let index = self.combinations[k].index;
        #[cfg_attr(not(test), allow(unused_mut))]
        let mut factors: Vec<f64> = self.combinations[k].operands.iter().map(|operand| operand.factor).collect();
        // Test-only (B2-P hooks): the armed Call refuses before any source, through the kernel's
        // own `NoOperands` check (a non-finite factor).
        #[cfg(test)]
        if crate::retained_tests_hooks::combination_call_fault(index) {
            factors[0] = f64::NAN;
        }
        let outcome = {
            let mut operands: Vec<(f64, k::RecordedOperand<'_>)> = Vec::with_capacity(factors.len());
            for (operand, &factor) in self.combinations[k].operands.iter().zip(&factors) {
                let recorded = match operand.source {
                    OperandSource::Selected => match self.capture.case_native(operand.case).map(|case| &case.outcome) {
                        Some(k::ExecutionOutcome::Selected(solve)) => k::RecordedOperand::Selected(solve),
                        _ => return Err(CombinationCustody("selected operand")),
                    },
                    OperandSource::Rebuilt { prepared, source } => k::RecordedOperand::Prepared { source, prepared: &self.rebuilt[prepared].1 },
                    OperandSource::Prepared { preparation } => {
                        let record = self.operand_preparations.get(preparation).ok_or(CombinationCustody("operand preparation"))?;
                        match (record.source, record.prepared.as_ref()) {
                            (Some(source), Some(prepared)) => k::RecordedOperand::Prepared { source, prepared },
                            _ => return Err(CombinationCustody("operand preparation")),
                        }
                    }
                };
                operands.push((factor, recorded));
            }
            invocation.solve_combination_sources(&operands, k::CaseLimit::new(COMBINATION_CASE_LIMIT))
        };
        match outcome {
            Err(_) | Ok(k::RecordedKernelCombination::OriginRefusal { .. }) => Err(CombinationCustody("combination custody")),
            Ok(k::RecordedKernelCombination::PreSourceRefusal { call, stage, reason }) => {
                let combination = &mut self.combinations[k];
                combination.call = Some(call);
                combination.end = CombinationEnd::PreSource { stage, reason };
                Ok(())
            }
            Ok(k::RecordedKernelCombination::WithRun { call, case }) => {
                let selected = matches!(case.outcome, k::ExecutionOutcome::Selected(_));
                let Self { ordinary, capture, combinations, .. } = self;
                let combination = &mut combinations[k];
                combination.call = Some(call);
                combination.attempt = Some(*next_attempt);
                *next_attempt += 1;
                // Its attempt enters at its native stage (B2-C §4). The internal trace marks C3's
                // preparation stage completed and the source ready: the operands' preparations
                // are done (DEF-C `preparation.combination: not_entered`; never serialized).
                combination.trace.enter(trace::Stage::Preparation);
                combination.trace.completed(trace::Stage::Preparation);
                combination.trace.source_ready = true;
                // Its view reads operand 0's prepared operational records as new (section terms).
                combination.trace.old_vector_swapped = true;
                combination.trace.enter(trace::Stage::Native);
                combination.results.native = Some(case);
                let representative = combination.representative().ok_or(CombinationCustody("operands"))?;
                if !selected {
                    combination.trace.fail_entered();
                    combination.trace.native_error = Some(CaptureError::NativeUnavailable);
                    capture.with_case(representative, |capture| combination.trace.freeze(capture));
                    combination.end = CombinationEnd::Native;
                    return Ok(());
                }
                combination.trace.completed(trace::Stage::Native);
                let rows = capture.combination_rows.get(index).cloned().unwrap_or(0..0);
                let id = capture.combinations[index].id.clone();
                let scope = CaseScope { rows: Some(rows), evidence: index, cases: capture.cases_seen(), qualified: false, route: capture.route };
                let frozen = capture.with_case(representative, |capture| {
                    capture.swap_results(&mut combination.results);
                    let case = capture.native.take();
                    let frozen = capture.freeze_combination(&mut combination.trace, &mut combination.overlay_work, &mut combination.proof_attempted,
                        ordinary, &scope, &id, invocation, case.as_ref());
                    capture.native = case;
                    capture.swap_results(&mut combination.results);
                    frozen
                });
                combination.end = match frozen {
                    Ok((values, certificate)) => CombinationEnd::Frozen(FrozenCombination { values, certificate }),
                    Err(refused) => CombinationEnd::Candidate(refused),
                };
                Ok(())
            }
        }
    }
}
impl ProductCapture {
    /// B2-P (C3a-2, C3a-3): one `not_required` case's operand preparation, on its own slot in
    /// the capture's fields: C3's preparation stage from its T-2 capture, as a case attempt's
    /// (`prepare_active_case`), without a product attempt or native stage. Its snapshot is
    /// taken when the stage completes or fails. On refusal its error stays in the slot.
    fn prepare_operand(&mut self, id: usize, owner: usize, requested_by: Vec<usize>) -> OperandPreparation {
        #[cfg(test)] crate::retained_tests_hooks::before_operand_preparation(self, owner);
        let mut parts = AttemptParts::default();
        let mut trace = trace::PreparedTrace::default();
        trace.enter(trace::Stage::Preparation);
        match self.prepare_active_case(&mut trace, &mut parts) {
            Ok(()) => trace.freeze(self),
            Err(e) => { trace.fail_entered(); self.error = Some(e); trace.freeze(self); }
        }
        OperandPreparation { id, owner, requested_by, source: None, prepared: None, parts, trace }
    }
    /// B2-P (B2-C §2.4 (iii) 4; DEF-C; REVISION_01 §1.2, N-9): one combination's freeze over its
    /// own block, on operand 0's slot (its id maps and member facts) with the combination's own
    /// result fields swapped in: the dual-readout proof on its selected Run, the projection
    /// (B2-K forms its displacement magnitudes), maxima and aliases completing empty, the
    /// certificate, the combination observables stage and G5a, then the overlay's precharge.
    #[allow(clippy::too_many_arguments)]
    fn freeze_combination(&mut self, trace: &mut trace::PreparedTrace, overlay_work: &mut ScalarWork, proof_attempted: &mut bool,
        ordinary: &MechanicsEnvelope, scope: &CaseScope, combination: &str, invocation: &k::RecordedInvocation, case: Option<&k::RecordedCase>)
        -> Result<(k::FrozenProductValues, k::CertifiedProductProof), RefusedCase> {
        let _ = overlay_work;
        let mut certificate = None;
        let mut saved_values = None;
        let result = (|| -> Result<k::FrozenProductValues, PreparedCandidateError> {
            if *proof_attempted { return Err(PreparedCandidateError::Capture(CaptureError::PreparedAttemptConsumed)); }
            let case = case.ok_or_else(|| PreparedCandidateError::Capture("missing combination native owner".into()))?;
            let k::ExecutionOutcome::Selected(owner) = &case.outcome else { return Err(PreparedCandidateError::Capture(CaptureError::NativeUnavailable)); };
            let base_rows = self.bind_rows_subject(ProductCaseView::of(ordinary, scope), owner, Some(combination)).map_err(PreparedCandidateError::Capture)?;
            let specs = self.prepared_specs(&base_rows).map_err(PreparedCandidateError::Capture)?;
            self.capture_entry(AdapterEvent::MapWrite).map_err(PreparedCandidateError::Capture)?;
            *proof_attempted = true;
            trace.enter(trace::Stage::ProofStart);
            let draft = invocation.begin_prepared_product(case.run, owner, &self.facts, &specs).into_ready().map_err(PreparedCandidateError::Proof)?;
            trace.completed(trace::Stage::ProofStart);
            trace.enter(trace::Stage::Projection);
            let (projected, builder) = draft.project().into_ready().map_err(PreparedCandidateError::Proof)?;
            trace.completed(trace::Stage::Projection);
            // DEF-C `stages`: maxima and aliases complete empty (no maximum row; headlines cover
            // load cases only).
            trace.enter(trace::Stage::Maxima);
            trace.completed(trace::Stage::Maxima);
            trace.enter(trace::Stage::Values);
            let (values, value_work) = match builder.complete_maxima(&[]).into_ready() {
                Ok(v) => v,
                Err(failure) => return Err(PreparedCandidateError::Values { failure, proof: projected.abandon() }),
            };
            trace.completed(trace::Stage::Values);
            trace.enter(trace::Stage::Aliases);
            trace.completed(trace::Stage::Aliases);
            let view = ProductCaseView { ordinary, payload: None, frozen: Some(&values), scope };
            let rows = match self.bind_rows_subject(view, owner, Some(combination)) {
                Ok(v) => v,
                Err(cause) => return Err(PreparedCandidateError::Abandoned { cause, proof: projected.abandon_values(value_work) }),
            };
            trace.enter(trace::Stage::Certificate);
            let certified = match projected.certify_final(&values, &rows, value_work).into_ready() {
                Ok(v) => { trace.checked(trace::Stage::Certificate, 0, true); trace.costs.record::<bool>(); trace.proof_ready = true; v }
                Err(failure) => {
                    trace.checked(trace::Stage::Certificate, 0, false);
                    self.numeric_pass = false;
                    self.numeric_failure = Some(failure.failure().clone());
                    self.source_correction_calls = failure.work().source_correction_calls();
                    if let Err(e) = self.prepared_verdict_copy(failure.work().verdicts(), failure.work().summary_coverage()) { self.error = Some(e); }
                    if self.error.is_none() && self.verdicts.len() == rows.len() {
                        trace.enter(trace::Stage::Observables);
                        self.observable_error = self.combination_observables(view, combination).err();
                        trace.checked(trace::Stage::Observables, 1, self.observable_error.is_none());
                        trace.enter(trace::Stage::G5a);
                        self.g5a_error = self.g5a(owner, &rows).err();
                        trace.checked(trace::Stage::G5a, 2, self.g5a_error.is_none());
                    }
                    drop(rows);
                    saved_values = Some(values);
                    return Err(PreparedCandidateError::Proof(failure));
                }
            };
            if !certified.matches_values(&values) {
                certificate = Some(certified); drop(rows); saved_values = Some(values);
                return Err(PreparedCandidateError::Capture("frozen proof/value owner".into()));
            }
            if let Err(e) = self.prepared_verdict_copy(certified.verdicts(), certified.summary_coverage()) {
                certificate = Some(certified); drop(rows); saved_values = Some(values);
                return Err(PreparedCandidateError::Capture(e));
            }
            self.numeric_pass = certified.passed();
            self.source_correction_calls = certified.work().source_correction_calls();
            trace.enter(trace::Stage::Observables);
            self.observable_error = self.combination_observables(view, combination).err();
            trace.checked(trace::Stage::Observables, 1, self.observable_error.is_none());
            trace.enter(trace::Stage::G5a);
            self.g5a_error = self.g5a(owner, &rows).err();
            trace.checked(trace::Stage::G5a, 2, self.g5a_error.is_none());
            let pass = self.full_case_passed();
            certificate = Some(certified);
            drop(rows); drop(base_rows); drop(specs);
            if !pass {
                saved_values = Some(values);
                return Err(if self.observable_error.is_some() { PreparedCandidateError::Observable }
                    else if self.g5a_error.is_some() { PreparedCandidateError::G5a } else { PreparedCandidateError::Numeric });
            }
            // The overlay's move plan (its rows' values only), checked and charged before staging.
            self.adapter.enter(AdapterEvent::MapWrite, u64::try_from(values.len()).map_err(|_| PreparedCandidateError::Capture(CaptureError::CountRange("commit count")))?);
            self.adapter.require().map_err(PreparedCandidateError::Capture)?;
            trace.costs.record::<bool>();
            trace.private_commit_precharged = true;
            Ok(values)
        })();
        match result {
            Err(error) => { trace.fail_entered(); trace.freeze(self); Err(RefusedCase { error, certificate, values: saved_values }) }
            Ok(values) => {
                trace.freeze(self);
                match certificate {
                    Some(certificate) => Ok((values, certificate)),
                    None => { trace.fail_entered(); Err(RefusedCase { error: PreparedCandidateError::Capture("frozen candidate without certificate".into()), certificate: None, values: Some(values) }) }
                }
            }
        }
    }
    /// B2-P (REVISION_01 §1.2; DEF-C r2 `stages.observables`): the combination observables stage
    /// on its own block: the adapter (`require`, one `LibraryBoundary`); its gate entry not
    /// withheld (T-10a's consistency); the support coverage and guard (`support_observables`, base
    /// G7's `combination_magnitudes`); per model node exactly one displacement magnitude and one
    /// row of each translation, the magnitude within 64ε·max(|p|, MIN_POSITIVE) of
    /// hypot(hypot(x,y),z) of them; and no maximum or intensified row. No preview case evidence,
    /// maximum or headline check.
    fn combination_observables(&self, view: ProductCaseView<'_>, combination: &str) -> Result<(), CaptureError> {
        self.adapter.require()?;
        self.adapter.enter(AdapterEvent::LibraryBoundary, 1);
        #[cfg(test)]
        if crate::retained_tests_hooks::combination_freeze_fault() {
            return Err("test combination freeze fault".into());
        }
        let evidence = view.ordinary().contract_evidence.as_ref().ok_or("preview evidence")?;
        let gates = evidence["combination_gates"].as_array().ok_or("combination gates")?;
        let index = view.scope.evidence;
        self.capture_entry(AdapterEvent::ValidationEntry)?;
        let gate = gates.get(index).ok_or("combination gate entry")?;
        let id = gate["combination_id"].as_str().ok_or("combination gate identity")?;
        if !self.checked_same(id, combination)? || gate["withheld"] != serde_json::Value::Bool(false) {
            return Err("combination gate entry".into());
        }
        for row in view.rows() {
            self.capture_entry(AdapterEvent::RowVisit)?;
            if row.kind == "pipe_elastic_normal_stress_maximum_v2" || row.kind == super::preview_physics::INTENSIFIED_KIND {
                return Err("combination maximum or intensified row".into());
            }
        }
        self.support_observables(view)?;
        for (id, _) in &self.nodes {
            self.capture_entry(AdapterEvent::RowVisit)?;
            let mut v = [0.0f64; 3];
            for (slot, kind) in ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"].into_iter().enumerate() {
                let mut found = None;
                for r in view.rows().filter(|r| &r.entity_ref == id && r.kind == kind) {
                    self.capture_entry(AdapterEvent::RowVisit)?;
                    if found.is_some() { return Err("combination displacement identity".into()); }
                    found = Some(*r.value);
                }
                v[slot] = found.ok_or("combination displacement identity")?;
            }
            let mut found = None;
            for r in view.rows().filter(|r| &r.entity_ref == id && r.kind == "displacement_magnitude") {
                self.capture_entry(AdapterEvent::RowVisit)?;
                if found.is_some() { return Err("combination magnitude identity".into()); }
                found = Some(*r.value);
            }
            let p = found.ok_or("combination magnitude identity")?;
            if (p - v[0].hypot(v[1]).hypot(v[2])).abs() > 64.0 * f64::EPSILON * p.abs().max(f64::MIN_POSITIVE) {
                return Err("combination magnitude guard".into());
            }
        }
        Ok(())
    }
}
impl CombinationAttempt {
    /// The combination attempt's typed C3 view, over the capture with its operand 0's slot in
    /// place and its own result fields swapped in (`PreparedCases::with_combination`).
    pub(super) fn typed_trace<'a>(&'a self, capture: &'a ProductCapture, costs: &mut trace::ProjectionWork)
        -> Result<trace::PreparedAttemptView<'a>, trace::TraceProjectionError> {
        match &self.end {
            CombinationEnd::Frozen(frozen) => trace::project(&self.trace, capture, &[], &[], &self.overlay_work,
                trace::ResultRef::Ready, Some(frozen.certificate.work()), None, None, costs),
            CombinationEnd::Candidate(refused) => {
                let failure = refused.proof_failure();
                let proof = failure.map(|f| f.work()).or_else(|| refused.certificate.as_ref().map(|c| c.work()));
                let values = match &refused.error { PreparedCandidateError::Values { failure, .. } => Some(failure), _ => None };
                trace::project(&self.trace, capture, &[], &[], &self.overlay_work,
                    trace::ResultRef::Unavailable(trace::FailureRef::Candidate(&refused.error)), proof, failure.map(|f| f.failure()), values, costs)
            }
            CombinationEnd::Native => trace::project(&self.trace, capture, &[], &[], &self.overlay_work,
                trace::ResultRef::Unavailable(trace::FailureRef::Native(self.trace.native_error.as_ref().ok_or(trace::TraceProjectionError::MissingFailure)?)),
                None, None, None, costs),
            _ => Err(trace::TraceProjectionError::MissingTerminal),
        }
    }
}
impl PreparedCases {
    /// Run `f` with combination `k`'s operand 0's slot in the capture's fields and its own
    /// result fields swapped in, then restore both (its projection and serialization).
    pub(super) fn with_combination<R>(capture: &mut ProductCapture, combination: &mut CombinationAttempt, f: impl FnOnce(&mut ProductCapture, &CombinationAttempt) -> R) -> Option<R> {
        let representative = combination.representative()?;
        Some(capture.with_case(representative, |capture| {
            capture.swap_results(&mut combination.results);
            let result = f(capture, combination);
            capture.swap_results(&mut combination.results);
            result
        }))
    }
}
/// B2-P (B2-C §2.7 staging 2): a frozen combination's row values on its own block.
fn apply_combination_overlay(envelope: &mut MechanicsEnvelope, values: &k::FrozenProductValues, rows: std::ops::Range<usize>) -> Result<(), StagingFault> {
    let rows = envelope.results.get_mut(rows).ok_or(StagingFault("combination results"))?;
    for (i, row) in rows.iter_mut().enumerate() {
        row.value = *values.value(i).ok_or(StagingFault("combination values"))?;
    }
    Ok(())
}
