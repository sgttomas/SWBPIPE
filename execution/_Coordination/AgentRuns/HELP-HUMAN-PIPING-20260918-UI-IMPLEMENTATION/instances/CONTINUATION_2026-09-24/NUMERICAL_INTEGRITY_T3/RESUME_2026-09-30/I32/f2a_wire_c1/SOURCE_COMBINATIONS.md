# I32 C1 — source interface, finite seams and dependencies

Proposal only, using main49034a940f. Paths below are P-relative (`P=projects/chirality-piping`). This document bounds the I32 contribution to the later atomic F2a/S-G1 manifest; it grants no writes. The complete prospective producer/three-reader set remains I30/SOURCE_MAP.md, reconciled by ROOT with routing, certification and memory returns.

## 1. Current source and the smallest mixed-source extension

`core/solver/frame_kernel/src/structural/retained/combine.rs:1–129` defines an own combined-ledger solve. It checks finite nonempty factors, nonnested operands, equal full stiffness encoding, QuantityMeta layout, station declarations and support groups; constructs CasePrep::combination; merges actual operand cache snapshots; then runs its own schedule. It does not read operand u, published rows or private radii. `adaptive.rs:889–955` encodes the combination as K4CMB\x01 with authored factor order and operand source encodings; exact loads and prescriptions are combined before the one rounding at p. Public prescribed rows use the exact combination rounded once to binary64, not the p-rounded value.

Proposed additive API (names provisional):

```rust
// Fields private; no Serialize and no caller-supplied identity/cache constructor.
pub struct PreparedCaseSource { prep: Arc<CasePrep> }
impl PreparedCaseSource {
    pub fn new(source: PrimitiveSource) -> Result<Self, PreparationFailure>;
    pub fn source(&self) -> &PrimitiveSource;
}
pub enum CombinationOperand<'a> {
    Retained(&'a RetainedSolve),
    Prepared(&'a PreparedCaseSource),
}
impl RetainedCombination {
    pub fn solve_sources(
        operands: &[(f64, CombinationOperand<'_>)],
        case_limit: CaseLimit,
        meter: &mut InvocationMeter,
    ) -> CombinationOutcome;
}
```

The prepared wrapper calls CasePrep::new once after source admission. It contains source, exact ledger, constraints, layout, extents and original encoding, **no solved state, factor, verification report, private radius or cache**. It is allowed for an ordinary `not_required` operand whose source is inside W1a. Preparation is counted in memory/preparation cost even though inherited LME is zero. Do not call solve_case/solve_cases merely to make such a wrapper. Do not expose arbitrary CasePrep construction, arrays of pre-rounded equivalents, or arbitrary cache injection.

`solve_sources` resolves each operand to `&CasePrep` without cloning a RetainedSolve. It checks the same four equalities as today's solve, in the same order; the prepared constructor has empty factors, while retained operands with factors remain NestedCombination. The checks concern exact source identity, not just public row shape. A same station id at a changed fraction and a same support id with changed membership must both fail, as today. Source construction's existing normality/finiteness/zero-length/exact orientation rules remain; no new physical cutoff or directional-spring product entry.

Use a selected operand's existing GroupPrep if one is available, after those equalities. The selected operand need not be first in the authored term order; the combined CasePrep still uses the first operand as today. If there is no selected operand, preparing one GroupPrep from the common source is a legitimate source-only API path, but production F2a leaves ordinary-only combinations on their existing ordinary path. GroupPrep construction can refuse before schedule; preserve its actual geometry/reason and empty schedule. This is not an operand solve.

Merge only actual `RetainedSolve.cache` snapshots in **authored operand order**, exactly `GroupCache::merged`: first nonempty s128/s256/s512/s1024/v256/v512/v1024 wins, including cached non-budget failures. A prepared ordinary operand contributes no slots. Shared costs count fully to this combination's CaseLimit; the invocation is charged only when this call actually builds a slot. Identical sources from two separate solve_case calls do not imply a shared cache. A new combination cache is local to that solve/result; do not mutate the borrowed operands or carry a newly populated cache into later combinations through a hidden global map.

For F2a provenance, add a compact immutable slot inventory method on RetainedSolve, or attach equivalent typed observations to the invocation-owned outcome as it is produced. It exposes only occupancy, success/non-budget-failure and charged stage totals, with provenance assigned by the actual invocation orchestrator; no factor payload or mutable cache access. The orchestrator associates a built slot with the actual run/record where built_here=true and binds later snapshot reuse to that slot identity. The implementation must verify actual snapshot occupancy, not assume that every earlier case sharing a stiffness identity contributed its final cache. Selected earlier cases retain the snapshot at their own finish; a later batch case may extend the group cache without updating that earlier snapshot. Repeat sources/factors keep separate operand provenance even when snapshots share an Arc.

Current `solve(operands:&[(f64,&RetainedSolve)],...)` remains as a compatibility wrapper to the same checked implementation. All-selected behavior, source/ledger/state bytes, selected values, policy, precision, refusal order and work must be identical. No maintained H/VR work rebaseline follows from the API extension.

## 2. Mechanics versus subtraction/range and fallback

Product `lib.rs:12457–12587` distinguishes Mechanics, ResultStateSubtraction and RangeEnvelope. Mechanics operand order is authored terms; subtraction is minuend then subtrahend; range sorts explicit ids. `preview_physics.rs:766–791` applies nonlinear, constant-effort factor-sum and modulus-basis gates **only to mechanics**. `preview_physics_evidence.rs:695–703` explicitly says ranges mode-select and are not a combined state. Treating all three as a shared-stiffness solve would change existing meaning and availability.

| Product expression | F2a behavior |
|---|---|
| Mechanics with at least one retained-selected operand, every operand source W1a eligible and common source/layout/station/support identity | Own source-ledger solve through solve_sources. Ordinary operands contribute prepared sources without becoming selected or changing their ordinary numerical_quality. Combination owns its complete receipt, final certificate, class and work. |
| Mechanics with ordinary-only operands | Existing base semantics; no W1 attempt/token solely because a receipt exists elsewhere. Separate ordinary coverage entry. |
| Mechanics already withheld by nonlinear/constant-effort/modulus gate | Preserve the exact base gate, diagnostic and row absence; do not override it with a successful numerical source solve. |
| Subtraction | Preserve labelled result-state difference semantics, including cases on differing modulus bases. It is not automatically a same-K mechanics combination and does not gain a retained certificate by using ±1 factors. No new W1 token/own-scale promise for this ordinary algebra branch. |
| Range envelope | Preserve mode-selected source semantics and deterministic order/ties. It is not K applied to a combined RHS; no combined-ledger solve, combined maximum or invented certificate. |

Both retained and ordinary publication preserve no combination maxima and no intensified combination rows (`preview_physics.rs:991–1005`); headlines continue to cover load cases only. Preserve support identity/attribution, source-result references, mechanics/subtraction magnitude rules and range-specific magnitude selection. A selected case's token cannot be copied into an ordinary algebra row by cloning; G6 strips/refuses accidental propagation.

**Finite routing dependency, not a licence to withhold:** an ordinary operand can be outside W1a (e.g. unsupported producer/pressure); same visible row ids do not make a supported PrimitiveSource. Likewise a W1 candidate can be declined by facade certification or work. Do not combine its rounded rows and label that a retained mechanics result. Before final W1 publication, I30 must freeze the dependency/rollback table for these branches. Proposed conservative completion is the preserved whole ordinary transaction with an explicit W1 decline, keeping already publishable ordinary combinations and spent W1 evidence. This proposal never silently removes previously published ordinary rows. If that rollback cannot preserve required successful recovery or the base evidence/standing, return the exact affected request/branch to ROOT; no implementation relies on an unchosen withholding policy. An ordinary algebra result keeps only its existing base standing; it receives no label-only W1 upgrade.

This is a faithful kernel mixed-source extension, not a claim to certify all possible product combination expressions. Broader subtraction/range certification, W1b formed-load sources and an explicit availability loss would need their actual contract decisions.

## 3. Refused-terminal evidence seam

At `adaptive.rs:3556–3568`, CaseOutcome::Refused currently lacks attempts. At `finish_terminal:4168–4182`, NegativeEnergy/Structure pass through `terminal` to Refused and drop the supplied Vec. The invocation meter still contains spent work. A receipt cannot both preserve all failed records and use that API unchanged.

Minimal proposed change: add `attempts:Vec<AttemptRecord>` to Refused, pass the owned vector through finish_terminal, use an explicitly empty vector on prepare_group/ledger pre-schedule refusals, and propagate the vector through CombinationOutcome::Unresolved when its reason is Refused. Preserve the same refusal reason/geometry/numerical outcome and schedule; no extra computation or solver retry. If downstream exhaustive matches make a direct field addition too broad, introduce an additive F2a-only `CaseExecution { outcome, terminal_attempts }` return wrapper internally before loss occurs, with existing public callers projected unchanged. ROOT selects one API shape after the actual consumer inventory; the source must retain evidence **before** the current discard, not attempt recovery later.

Meter snapshots belong to the invocation orchestrator and each complete run, captured before and after the existing call. Snapshot subtraction is a consistency check, not a replacement for attempts. Saturation/overflow or missing evidence declines receipt production honestly. Wire translation retains every current reason's typed payload and does not invent an exact lost work count.

## 4. Exact prospective source seams

The following is the proposed minimal **I32-owned contribution to a later grant**, not today's write scope. Names marked new are proposals.

| Files | Change |
|---|---|
| `core/solver/frame_kernel/src/structural/retained/adaptive.rs` | PreparedCaseSource wrapper at CasePrep; nonnumerical terminal evidence preservation; narrow immutable cache inventory if chosen; no changed stop rule/prices/ceiling. |
| `core/solver/frame_kernel/src/structural/retained/combine.rs` | CombinationOperand/solve_sources and faithful compatibility wrapper; preserve Refused attempts; same exact ledger, group/cache checks and output. |
| `core/solver/frame_kernel/src/structural.rs` | Only required public type/function reexports. |
| `core/solver/frame_kernel/tests/retained_k4/combine_tests.rs` | Source-prepared mixed API, identity/cache scope, own-combination failure and operand unchanged controls. |
| `core/solver/frame_kernel/tests/retained_k4/adaptive_tests.rs` | Existing schedule test home: terminal records, checked charge closure and stop precedence controls; no golden weakening. |
| `core/product_physics/src/retained_receipt.rs` (new) | Physical→logical projection, closed stable policies/domains/serialization, checked counters and fallback error; no shared raw radius serialization. |
| `core/product_physics/src/retained_source.rs` (new), `lib.rs`, `preview_physics.rs` | I30 integration owner supplies prepared operand sources/id maps and combination branch/ordinary transaction, work snapshots and actual snapshot provenance. I32 changes do not duplicate routing. |
| `core/product_physics/tests/retained_routing.rs` (new) | Mixed/ordinary/subtraction/range/fallback controls, no forced ordinary operand solve. |
| `core/reporting/result_export/src/retained_precision.rs` (new); `semantic_contract.rs`, `derivative.rs`, `physics_source.rs`, `lib.rs` | Rust independent G0–G8, ordinary projection and carrier retention/binding. |
| `core/analysis_runs/retained_precision.py` (new); `compatibility.py`, `physics_source.py` | Python independently checked G0–G8 and AnalysisRun retention. |
| `apps/desktop/src/features/results/retainedPrecision.ts` (new); `numericalResultQuality.ts`, `resultSemantics.ts`, `knownSemanticLimitations.ts`, `KnownSemanticNotices.tsx`, `physicsSourceRecovery.ts`; `services/analysisRunCompatibility.ts`, `services/previewService.ts`; `types.ts` | TS full async validation/registration plus bound synchronous standing; retained class disclosures; shared stable table policies. Exact native caller/memory files are I30/I29 dependencies, not satisfied here. |
| `schemas/retained_precision_mp_v2.schema.json` (new); `schemas/results.v0.3.schema.yaml`, `analysis_run.v0.3.schema.json`, `stress_neutral_export.v0.3.schema.json` | Closed versioned receipt and identity-located carrier branches; transport never eligible; no export-authority expansion. |
| `fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json`, `semantic_contract_v0_3_physics_retained_1.json`, `retained_precision_cases.json` (all new) | ROOT-reserved tables and one shared finite reader corpus. All producer-positive raws generated later only under runtime authorization, never by editing old fixture meaning. |

Reader tests remain exactly I30's proposed new files: Rust `core/reporting/result_export/tests/retained_precision_contract.rs`; Python `tests/test_retained_precision_contract.py`, `test_retained_precision_schema.py`; TS `features/results/retainedPrecision.test.ts`, `retainedPrecisionIntegration.test.tsx`, `services/retainedPrecisionAnalysisRun.test.ts`; T6 `features/results/retainedPrecisionOutputRefusal.test.tsx` plus existing output adapters from I30. Broader source-preparation/resource-count changes are memory design dependencies, not implicit additional files in this table.

The bounded source search for `CaseOutcome::Refused` finds these additional existing consumers: FK tests `factor_tests.rs`, `kf1_tracker_tests.rs`, `publication_tests.rs`, `method_tests.rs`, `references_tests.rs`; H `core/solver/performance_harness/src/k6/w1/staged.rs`, `src/bin/k6_observe/w1.rs`; VR `validation/benchmarks/numerical_robustness/src/lane.rs`, `src/records.rs`. This is an exact candidate inspection list, not proof each needs editing: patterns with `..` already tolerate a field. At implementation freeze inspect those pattern sites, add only necessary mechanical changes to the explicit grant, preserve their numerical outputs/old observation bytes and do not refresh records. Prefer an additive F2a-only return wrapper if it keeps this unrelated consumer surface unchanged without losing evidence.

## 5. Finite fixture and mutation inventory (unrun)

The run-record arithmetic examples are evidence of projection, not producer-complete positive receipt fixtures: VR omits some native numerical summaries, and H prefixes omit attempts. The future shared corpus must use complete producer outputs after the new evidence seam. Keep one raw/control per distinct branch rather than a combinatorial all-model programme.

| ID | Discriminator / required result |
|---|---|
| W01 | Selected128 physical Candidate/Accepted then Verification/Verified → one logical Accepted; all 19 stage totals and charges preserved. Delete verification → work failure. |
| W02 | THIN-style rejected128/reused256/reused512/completed1024 ceiling → three logical rejected candidates; same source count 4,442,076 for the existing record. Add Q or D again → work failure. |
| W03 | Candidate Pivot/Condition/ResidualGate failure versus verification solve failure → one-slot versus two-slot advancement. A failed verification is never reused. |
| W04 | Verification-pass partial failure with Uc/S refusal; retain reason, bound_refusals and all partial shared/own work. Drop refusal or mark built false → failure. |
| W05 | Terminal schedule Refused after nonzero work → vector survives CaseOutcome and CombinationOutcome; pre-schedule geometry refusal retains empty vector and correct zero inherited LME. |
| W06 | Shared slot built in earlier same-batch case, reused in later case and mixed combination; independent solve_case calls rebuild and charge. Mutate global digest dedup or reverse first-slot winner → failure. |
| W07 | Cached non-budget failure vs budget failure: only former reused; both preserve spent work. Do not reset meter on facade fallback or per case. |
| W08 | At threshold, one unit over, Case and Invocation simultaneously over, subsequent invocation-exhausted case with no attempts. Honest overshoot unavailable accepted as well-formed; clipping rejected. No hard overshoot bound asserted. |
| W09 | Require upstream no-wrap admission bound to actual source/count/build profile, including cumulative components, aggregates, stages and maximum unguarded work. Missing or mismatched proof declines W1 before execution; mutually consistent wrapped-small values cannot qualify as exact evidence. With that premise, 2^53−1 count accepted; 2^53, negative, fraction, bool, -0, u64::MAX/overflow/inconsistent sum cannot emit selected receipt. Saturated diagnostic is labelled lower-bound. |
| W10 | Kernel Selected with facade unit/recipe/G5a/receipt rejection → public unavailable, no row token/selected diagnostic, intact kernel Accepted and spent work. Removing facade distinction must fail. |
| C01 | All-selected old API vs source API identical source/ledger/state digest, publication/work/refusal; mixed Prepared+Retained selects without any ordinary operand solve; retain ordinary quality unchanged. |
| C02 | Change one of stiffness bits, layout meta, station fraction, support membership independently → OperandsDiffer before schedule. An identical ID alone does not pass. |
| C03 | Cancellation A+B−A2 and (P,epsilon)−P: combined exact ledger retained; mutate to sum published rows or retained u → unchanged protected truth catches loss. No new oracle/tolerance. |
| C04 | Combination escalation/stop leaves operand values/evidence/cache snapshot unchanged; later combination does not reuse first combination's newly built slots by accident. |
| C05 | Mechanics nonlinear/constant-effort/modulus base gates survive; subtraction across different modulus bases remains labelled difference; range mode/ties preserved; no maxima or intensified rows. |
| C06 | Ordinary operand outside W1a or facade-refused required combination: explicit routing fallback preserves previously publishable ordinary output; no source construction from rounded force nets and no silent disappearance. |
| R00–R08 | Shared corpus mutates one branch per gate: wrong v1/profile/table/policy; stale/rehashed receipt/publication; invalid Bits/count; case/combination order or missing row; diagnostic conflict; schedule/work/summary; misplaced token; malformed base evidence; wrong actual mode/material/source mapping. Assert identical first failure code in Rust/Python/TS. Rehash semantic mutations so G1 does not mask later checks. |
| R09 | Stop one ulp above2^-64, estimate above1/4, charge above1, theta above1/2, missing/extra B body, E=0 with nonzero SI action, wrong small-scale b, wrong floor precision, mm multiplication by0.001, section/member/DOF/hanger/pressure mismatch: each specifically rejected. |
| T01 | Missing invocation → needs_recompute; carrier without rows never eligible; copy/restore receipt unchanged; drop class disclosure or stale async-validation fingerprint → refusal. Label alone cannot register Current. |
| T02 | Absolute/not_covered quantity or headline binding uses the two adopted codes, no pass; raw y±b_SI unit error rejected; relative/InputDerived retain existing point semantics. T6 export remains refused. |

## 6. Cross-packet dependencies and review boundary

I30 must freeze actual request/mode custody, failed ordinary-attempt evidence, exact-block coexistence, invocation execution order, prepared source/id/material maps, explicit combination fallback and no newly reachable unadmitted ordinary work. This contract does not solve early-terminal arbitration or native registration.

I31 must freeze identity-bound certificate views and every covered final-row/combination recipe after actual rendering/qualification. Kernel acceptance alone never supplies a selected receipt. The per-source map, layout and section truths used here must be identical to its proofs; unsupported recipes do not get silently reclassified to escape a failed certificate. G5a preflight is required even for a mathematically conservative certificate.

Memory/cost P1–P5 must count PreparedCaseSource/CasePrep source+ledger+layout+extents, combination source and exact prescription vectors, GroupPrep, actual Arc/cache snapshots and partial failures, retained solves/radii, physical trace plus logical index graph, source-binding arrays, receipt draft/hash buffers, base fallback transaction, plus PP/headless/native ownership/copies. It must independently establish and bind the upstream no-wrap premise specified in WIRE_CONTRACT §2 before numeric execution: cumulative SumWork components, aggregate/stage additions and maximum unguarded/whole-attempt work, including reuse and stopped/error paths. This is a separate arithmetic proof obligation; a byte allowance or intermittent LME threshold is not its proof. Preparation/trace/hash/certificate work must have finite separate limits before allocation/growth. No 3.75-GiB value is enacted here. Execution-order and cache-origin inventories are real storage, not free telemetry.

Fresh independent reviewer checks the frozen wire and source-interface packet, exact arithmetic, reachable terminal cases and all decision boundaries. ROOT then reserves identities/policies, freezes the actual maintained write set and reconciles I30/I31/memory. No schema/producer-only grant, kernel method change, F3 selection, release or runtime follows from this return.
