# Exact source and qualification manifest

Recommendation only. P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
K=FK/src/structural/retained; H=P/core/solver/performance_harness;
VR=P/validation/benchmarks/numerical_robustness. Every path below is exact; grouped
names expand only within the stated directory. No unrelated source or historical
record refresh is included. The paired P1 scalar-admission and F2a producer/reader
work remains part of ROOT's complete grant, not waived by this counter plan.

## A. Kernel maintained paths

| Exact path(s) | Selected changes and complete caller handling |
|---|---|
| K/work.rs (new), K/mod.rs | WorkFault/Status/Total, non-ZST WorkStream and borrowed WorkSnapshot; checked state algebra and public exact accessors; register module. New unit test module uses FK/tests/retained_k4/work_tests.rs. |
| K/wide/multi.rs | WidthWork status/transactional charge and checked costs; AttemptWork checked reduction/merge; WideContext charge failures map to WideError::WorkAccounting. Clean price/getter/Debug compatibility. |
| K/wide_sum.rs | SumWork status/checked aggregate, ExactWideSum poison and pending-base/carry/shift checks, fallible sign/is_zero/absolute/net, donor checks, private CloneWork operations/deltas and compatibility Debug. |
| K/wide.rs | Only add WorkAccounting variant/import/Display. Do not modify K3a WorkCounter, WideArith algorithms or L2 prices. |
| K/adaptive.rs | All stage/cohort totals become WorkTotal/WorkSnapshot; Stage::StopRule; checked StageWork methods; Spent/StopDecision/CertificateMeter/PublicationSpent finalizers join error status. Replace free sum_work_delta with CloneWork. All dynamic raw observations use `?`. Tracker collapse/offer/finish terminally propagate accounting errors before pruning. New stop/terminal reason, record accessors/state, case/meter charging, typed caches and CoreRun/RunWork/C2 recorded adapters. Preserve every partial/error/budget/selected path. |
| K/verify.rs | VerifyShared/VerifySpent totals; cohort snapshots and compound stage differences; local t/r/rr/inf/one/au/d fallible observations; error finalization after all owners and refusal vectors captured. No new local LME price. |
| K/assemble.rs | gram_exponent signum uses `?`; borrowed work owners and existing guards propagate WorkAccounting. Existing form/assembly pricing and values unchanged. |
| K/factor.rs | pivot predicate/negative_pair signum use `?`; closed determinant exception uses is_zero().ok()? under API_PLAN §5 bound. Factor/condition uses checked context/sum guards without altered numerical loop order. |
| K/bound.rs | shift_needed signum uses `?`; refusable/reset retains E Span/Exponent behavior and propagates WorkAccounting; all borrowed operations already use Result. No accounting error becomes a per-block refusal. |
| K/directed.rs | signum uses `?` in round_toward/div_toward/binary64_up; preserve clears and numerical steps. |
| K/source.rs | chord_checks is_zero().expect with proved fresh-helper bound; no panicking dynamic owner introduced. P1 separately closes constructor/count/encoding guards in this same source. |
| K/combine.rs | C2 PreSourceRefusal/WithRun additive path and unchanged legacy projection; typed meter snapshots, actual RunWork, first-slot imports/status; no source/Run on early failure. |
| FK/src/structural.rs | Re-export WorkFault/WorkStatus/WorkTotal, RunWork and C2 recorded types/functions. Existing legacy CaseOutcome shape stays unchanged. |

K/recover.rs and K/ledger.rs need **no work-signature source edit**: their borrowed
add/round operations already propagate Result; no ExactWideSum sign/is_zero mutation
is swallowed there. They remain required source-review and regression targets.
FK/src/exact_sum.rs and structural/formation_check.rs need no edit: the former
has its own checked carry error, the latter accepts/forwards generic WideError
without an exhaustive variant map. Test-only seeded.rs remains unchanged; its K3a
counters do not become W1 work. Scalar admission from P1 can require its own
additional edits, which are not silently folded into this manifest.

## B. Necessary legacy consumers, chosen behavior

H/src/k6/w1/staged.rs changes W1Solve.charged from u64 to WorkTotal, initialized
from meter.checked_charged(). No additional status field. work_closes accepts
WorkTotal and returns false on non-E/overflow/inconsistent sums. stage equality/
complete predicates first check statuses; add_stages calls public StageWork::merge.
work_by_precision, segments and prefix_limits return Result<existing return type,
WorkFault>, using exact() followed by checked integer addition/subtraction. They
cannot saturate differences or manufacture a valid segment end. prefix_matches
returns false on either segment error. Plain own_total/charged_by/stage_fields
compatibility views remain but are never sufficient proof; writers use the
checked helpers and reject non-E before extracting those views.

H/src/bin/k6_observe/w1.rs makes repeat_digest, outcome_line, attempt_lines,
parity_lines and prefix_line return Result<old return,WorkFault>. Validate the whole
relevant input slice/meter before emitting any work line, then use exact amounts.
No new valid-case field or formatting change. On non-E no exact charge, parity,
segment or deterministic-work digest is emitted. H/src/bin/k6_observe/main.rs
propagates this typed error to its existing failing-run exit path before saving
attempts or emitting subsequent work/prefix observations. budget_reached first requires an exact meter; a non-E exhausted flag is reported
as accounting failure, never merely budget_reached. Its existing error reporting prints the new
bounded static fault text; no new numerical observation schema is introduced.

VR/src/records.rs makes case_record return Result<Value,WorkFault>; it checks meter,
all raw/stage/record totals before formatting. VR/src/lane.rs changes CaseRun.record
to Option<Value>, wraps existing source-refusal records in Some, and on accounting
failure records a harness failure plus None. It never fabricates a numerical case
record. VR/examples/vk_records.rs and examples/vk_scale.rs must stop with a failed
run before printing or writing a missing record/corpus; ordinary E records remain
byte-identical. VR/tests/lane.rs unwraps only after asserting no accounting failure.
No historical JSON record is rewritten. VR/src/invariance.rs needs no edit: it
reads other CaseRun fields, not record. Changing Result/Option consumer signatures
is preferable to outputting an invalid record under the old schema.

These are necessary **checked-work** additions to C2's earlier statement that its
terminal-only additive API required no H/VR edits. C2's unchanged Refused patterns
and empty legacy Refused attempts still need no field-pattern repair.

## C. Exact test files

New FK/tests/retained_k4/work_tests.rs covers state/lineage/raw-near-limit mutation
controls. Existing files needing expectation/signature adaptation and targeted
controls are:

- FK/tests/retained_wide_k3/k3_tests.rs;
- FK/tests/retained_k4/wide_sum_tests.rs, directed_tests.rs, assemble_tests.rs,
  factor_tests.rs, bound_tests.rs, ledger_tests.rs, recover_tests.rs,
  adaptive_tests.rs, kf1_tracker_tests.rs, kf3_tests.rs, publication_tests.rs,
  combine_tests.rs, method_tests.rs;
- FK/tests/s11_site_table.rs (review changed accounting arithmetic-site inventory);
- H/tests/k6b_w1.rs, k6b_export.rs, k6c_envelope.rs, k6c_h_envelope.rs;
- VR/tests/lane.rs and VR/tests/k6c_envelope.rs.

FK/tests/retained_wide/wide_tests.rs, FK/src/structural/formation_check_tests.rs and
FK/tests/retained_k4/references_tests.rs are read-only regression checks: shared error
compatibility and unchanged legacy Refused shape. Ordinary fixtures, trusted
vectors, reference values, tolerances and old record bytes are not edited.

## D. Memory/profile paths and exact consequences

Edit H/src/k6/w1/counts.rs, envelope.rs and h_envelope.rs, and VR/src/envelope.rs
only against new independently reviewed candidate layout/owner facts. Retain the
old Source40129Rust1971Aarch64V1 mathematical profile with its historical meaning;
add the proposed CheckedWorkV1Rust1971Aarch64 basis/constructor in kernel/H/VR
profiles. It becomes usable only with actual candidate/source/toolchain/target
correspondence, not by calling a host size factory. No profile is claimed qualified
by this plan. H/VR use one canonical memory core; do not port or duplicate formulas.

Parameterize the new profile by reviewed constants S_SumWork, S_WidthWork,
S_AttemptWork, S_StageWork, S_WorkTotal, S_ExactWideSum, S_AttemptRecord,
S_AttemptStop, S_UnresolvedReason, S_Evaluated, S_Shared[p], S_VerifyShared[p],
S_GroupCache, S_RetainedSolve, S_CoreRun, S_RunWork and S_W1Solve, with alignments
and actual container/node request rules. All are private immutable facts in that
profile, not caller inputs. WorkStatus and WorkFault are one-byte representations;
WorkTotal repr(C) includes target u64 alignment/padding. No compiled size is asserted.

Re-derive 2144 sum, 4304 tracker/fallback stride, 40 evaluated-entry stride, tree-node
1168/200 facts, 720 Shared base, VERIFY_ARC, 1464 call, 3328 attempt buffers and 1976
finish; these numbers remain only in the old profile until checked correspondence.
Each new flag can change parent padding/niches even if a particular allocation
happens to keep the same size. New WorkAccounting prior payload affects
UnresolvedReason and containing outcome/cached enum unions. WorkSnapshot borrows a
stack WorkStream plus WorkTotal; at most the source's t0…t7 snapshots coexist per
scope. CloneWork replaces the existing cloned accumulator rather than adding a
second live clone, but its private wrapper/padding still needs a layout witness.
Raw pending charge is stack state; no heap count/lineage registry is introduced.

Successful cache Arc payloads are counted once per actual allocation; Arc handles,
cache enums and deep-cloned failure StageWork/refusal vectors are separate owners.
CoreRun/RunWork add fixed owned evidence and C2 recorded refused-attempt retention;
legacy mode drops those vectors at the existing per-case adapter before pushing
its legacy outcome. F2a P1–P5 must compose batch/combination outputs, imported
snapshots, partial/error owners, logical graphs, caller ordinary/W1 drafts and
completion buffers with the added typed totals/flags. Changed layouts and extended
lifetimes both matter. VR Option<Value> can change CaseRun layout/niche and failed
record ownership; H W1Solve's WorkTotal replaces its 8-byte scalar. Revisit H/VR
record/diagnostic and saved-attempt profile terms even when E serialization is
unchanged. No byte allowance, OOM guarantee or elapsed-time bound follows.

## E. C1/C2 integration paths and controlled implementation order

The exact existing prospective C1 integration paths affected by this decision are:
P/core/product_physics/src/retained_receipt.rs and retained_source.rs (new), plus
lib.rs and preview_physics.rs; P/core/reporting/result_export/src/retained_precision.rs
(new); P/core/analysis_runs/retained_precision.py (new);
P/apps/desktop/src/features/results/retainedPrecision.ts (new);
P/schemas/retained_precision_mp_v2.schema.json (new); and
P/fixtures/results/retained_precision_cases.json (new). Use API_PLAN §6's exact
state precondition/diagnostic/defensive Reason additions. The existing C1 carrier
and async registration integration set remains required, without new status fields
in a successful numeric-only receipt. Do not edit historical C1/C2 packets; ROOT
records adoption and the maintained contract constants/schema embody it.

Corresponding exact prospective tests are P/core/product_physics/tests/retained_routing.rs;
P/core/reporting/result_export/tests/retained_precision_contract.rs;
P/tests/test_retained_precision_contract.py and test_retained_precision_schema.py;
P/apps/desktop/src/features/results/retainedPrecision.test.ts,
retainedPrecisionIntegration.test.tsx;
P/apps/desktop/src/services/retainedPrecisionAnalysisRun.test.ts.

Finite checkpoints:

1. RV46 backcheck of this selected API, helper proofs and manifest; ROOT freezes
   this with P1 scalar interfaces and existing C1/C2 complete integration set.
2. Implement work.rs/raw/context signatures, fail-fast error transfers, snapshots
   and exact Debug compatibility; qualify unit boundaries before caller integration.
3. Convert every listed stage/cache/schedule/C2 recorded transfer atomically; no
   naked production u64 accounting operation remains on the enumerated streams.
4. Integrate H/VR no-false-record gating, C1 producer/readers and diagnostic mapping;
   qualify new layout/profile/P1–P5 facts and clean observation identity.
5. Fresh full-candidate review and required checks before merge; no code authority
   or acceptance is conferred by completing this design checkpoint.

Decisive tests in debug/overflow-checking and optimized configurations: exact MAX
versus prospective overflow; every component and weighted/aggregate boundary;
foreign equal-valued WorkSnapshots; CloneWork cannot accept foreign snapshots;
MAX−MAX remains O; missing/zero-delta and underflow; base reservation survives
shift/carry increments; an aborted uncommitted event is not charged; partial
compound insertions poison and clear invokes full reset; reset preserves flags;
accounting errors cannot be pruned or A2-swallowed; original numeric prior remains;
non-budget failed cache/built=false imports state without price; budget failure
uncached; later bad case/combination blocks exact invocation receipt; source refusal
has no fake Run; all clean price, bits/classes, Debug digests, old H/VR bytes,
threshold precedence/overshoot and C1 B/T partition remain identical. Mutate each
status-transfer boundary and require a targeted failure rather than a general
fixture change. None of these implementation/build tests ran in this packet.
