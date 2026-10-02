# I22 implementation checkpoint 0 — concrete SI certificate plan

Status: **PLAN ONLY**. ROOT selected bare-b SI admission and the relative
conjunction in the committed brief. The routed addendum's private-radius and
unit-coordinate design is incorporated with RV28's verified additive backcheck.
No maintained code, test, oracle, host configuration or Git/index edit was made.
No Rust, solver, build, Python check, mutant or host job ran. No child was created.

TASK /root/t3_recovery_manager/i22_a1 reports only through its recovery manager
to ROOT. Existing Root/TASK/Piping/COMMON/diagnosis-skill basis remains ../BASIS.json.
Instruction grant: COORD 2ecdde3d73f8e4264c4398c8225af9193d8e30f0, SHA256
84ec0d5e59d2524006fcf5d2f650f1b4497f3b3dea387cf5e88fbe61b0a3ab52.
Original proposal/review/addendum hashes and source-read evidence are in BASIS.json.
Their seals verify (8/8/6 files); the routed addendum backcheck also verifies
all eight entries. BACKCHECK.md SHA256 is
1a71f9b5d4adb9fa85ad26a850e117f1f8f4d306e4b7529a4ca0f19d21172247,
seal ed74a957e74df78d590df6fb3788c906d4ba40d5e54e2a685efcbe8391298fcc.
RV28-1 is closed at specification level; RV28-N1's directed-conversion
scratch/delta accounting is an implementation obligation. Actual A1 HEAD observed during this plan was
d294b8e8a62d5ebeae56e126c28b9921cf7847a7, after ROOT's records integration.
The complete FK diff from pinned product basis d01ad98a754698631f927709d08284c272de85e8
is empty. This does not silently rebind the numerical source.

Aliases: P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
H=P/core/solver/performance_harness;
VR=P/validation/benchmarks/numerical_robustness; R is the resumed run root.
All proposed edits below need a subsequent explicit ROOT source grant and host
work needs separately granted slots. The records DEC-025 job is not I22's job.

## 1. Small write set and explicit exclusions

Request these four maintained files for the minimum kernel implementation:

| Path | Exact purpose |
|---|---|
| FK/src/structural/retained/adaptive.rs | Add typed certificate reasons/predicate enums, bound verification pairing, metered provisional publication/H/RU64 helpers, strict gate after old R7, owned accepted draft, private radius slice/accessor, policy constant, new test-module inclusion. |
| FK/src/structural.rs | Re-export only the new public attempt-reason payload enums needed by existing retained_api consumers; no public radius/facade API. |
| FK/tests/retained_k4/publication_tests.rs (new) | Finite pure-certificate, lifecycle, schedule, budget, C17/frozen-B and fixed source controls; embed immutable primitive bit inputs and independently established expectations with provenance. |
| H/tests/k6b_export.rs | Compile/import smoke coverage for the new typed reason payloads and corrected policy token through the existing external public API. |

Keep implementation in adaptive.rs to avoid expanding sibling-module visibility
or adding a second numerical implementation. verify.rs already has every required
error field; its formation/error formulas, factor/solve/bound loops, source
admission, recover.rs and exact_sum.rs remain unchanged in this plan.
Existing protected tests/oracle files, fixtures, expected-unresolved lists and
historical hash-bound evidence remain unchanged. No blanket write glob is proposed.

ROOT-owned integration may separately authorize the exact VR observation refresh
in §7, additive policy/D1/D2 registration and K6c changes. Those are not implicitly
included in the four-file grant. If compilation or a discovered necessary change
needs another path, return the exact dependency before editing it.

## 2. API and flow

Retain existing public solve_case/solve_cases/RetainedCombination::solve and
Publication/PublishedRow field contracts. Proposal for private plumbing:

- A bound verification bundle holds the existing VerificationState report,
  an Arc<CasePrep> and a clone of the matching PrecisionState Arc. It is created
  only by verify_precision from the same active prep/state. compare/certify check
  prep Arc identity, verification state Arc identity, exact precision pair and
  report.precision; the extra Arcs do not copy state arrays. This supplies an
  actual pairing invariant instead of trusting equal vector lengths.
- A private PublicationDraft owns Publication plus Box<[u64]> radius bits and
  its candidate precision/source/prep binding. It is created after old R7 gates
  pass. A separate private CertifiedPublication wrapper can only be constructed
  by successful certification and final radius/class validation.
- A private certificate function dispatches through the same three supported
  pairs as compare_states: 128/256, 256/512, 512/1024. Its row worker takes the
  final PublishedRow, matching verification Wide value, report fields and P.
  It returns a spent result: accepted draft, numerical row rejection, or typed
  terminal error, with AttemptWork/SumWork/total on every path.
- finish_selected consumes CertifiedPublication by value and moves its exact
  Publication and radius box into RetainedSolve. It does not call published(),
  publish_prescribed() or classify_rows_floored() a second time.
- RetainedSolve gains the addendum's private publication_radius_bits: Box<[u64]>.
  A crate-private indexed accessor checks expected QuantityId/body/kind,
  selected precision/policy/source binding and class/sentinel consistency,
  returning Result<Option<SiRadius>,CertificateIssue>. SiRadius is a private
  typed identity/kind-SI-unit/finite-nonnegative-bits view. No borrowed report
  escapes. No public serialized radius or cross-crate facade accessor is added.

Before any zip/indexing, check exact lengths of layout, candidate/verification
values, publication, radii and report e_rows/w/charge/w_plus, expected body and
floor inventories, supported precision pair and canonical unique row identities.
Check each Publication row's id/body/kind against the same prep layout.
InputDerived is only a prescribed/constrained displacement component; verify its
source-prescription entry exists, then preserve exact_publication's existing
single-round result. No missing entry may fall back to the p-rounded value.

For each eligible non-input-derived value row, require the full expected
nonnegative fields: W_plus for free components/magnitudes; e_rows, W and C for
force/moment. Zero is valid only as actual present zero. Validate closed row
families and field presence; negative, mismatched, absent or malformed data is
a named certificate error, never an unwrap_or(0). Wide values are finite by
construction; reject any nonfinite binary64 extent/scale/floor/allowance/bound,
negative or noncanonical negative-zero metadata. The private infinity sentinel
is handled only as a tag, never supplied as a numeric allowance.

Preserve candidate-based O9 membership and exact input-derived publication.
InputDerived/Unpublishable rows get sentinel bits 0x7ff0000000000000. They do not
acquire an H=0 certificate. Displacement magnitudes still need their own W_plus
even when components are prescribed. Existing R7 checks still run on every row
they currently check; the new gate is never substituted for those checks.

## 3. Exact arithmetic and accepted predicates

P is report.precision=2p, not twice that field again. Construct one exact H
from final binary64 x and retained verification v:

| Row family | E added to |x-v| |
|---|---|
| Free translation/rotation component | W_plus |
| Displacement magnitude | W_plus + 2^(1-P)|v| |
| Every force/moment family (end/station/global or directional spring/reaction/support magnitude) | 69·2^-P E_q + W + 2^-P W + C |

Use ExactWideSum::add_binary64/add_wide/add_wide_scaled/add_scaled,
make_absolute and exact sign comparison. No f64 subtraction or rounded H.
Scalar e_rows is E_q, not the complete error. Preserve 69, W·2^-P, C and the
magnitude term. No source matrix refactor, new norm or inverse estimate is needed.

Absolute: subtract H from exactly decoded b and require nonnegative, including
b=0. Relative: in stable order require public, sharper-exact, sharper-binary64:
1. |x| − 1,000,000,000 H >= 0.
2. epsilon*m + epsilon*2^-21*m + 2^-53*|x| + h − H >= 0,
   m=max(|x|,S), epsilon=2^-64, h=2^-1074.
3. A_f64 − H >= 0, where A_f64 is exact decoding of these separately rounded
   operations, with no reassociation/FMA:
   a0=RN64(epsilon*m); a1=RN64(a0*(1+2^-21));
   u0=RN64(2^-53*|x|); u1=RN64(u0+h); a2=RN64(a1+u1).
   Every intermediate is finite/nonnegative; A_f64=decode(a2).

This is H<=min(A_exact,A_f64) AND 10^9H<=|x|. It is not permission to pick a
passing larger bound. Values/classes/scales, A1's small-scale branch, threshold
tie direction and force/moment-only p512 floors stay bit-identical.

After the exact gate succeeds, compute RU64(H) with metered exact directed
H/1 rounding. Store +0 for exact H=0 and at least h for positive H. Failure to
obtain a finite nonnegative canonical radius is terminal, never zero. The
addendum proves representability from H<=finite b or H<=finite A_f64.
Acceptance uses exact H, not RU64(H); radius rounding must not add an unwarranted
relative rejection. Validate r<=b on absolute rows and r<=A_f64 on relative rows.
No public bound is enlarged.

## 4. Reasons, policy and schedule

Propose exact new Rust payloads, names finalized by ROOT registration before
implementation is relied upon:

- PublicationPredicate::{AbsoluteBound, PublicRelative, SharperExact,
  SharperBinary64}, in that comparison order for each row.
- AttemptReason::PublicationEnclosure { quantity, body, kind, predicate }.
  Internal Rejection adds the matching layout-index/predicate form.
  Canonical semantic reason string: publication_enclosure.
- CertificateIssue for Shape, PairIdentity, Precision, RowIdentity, MissingField,
  NegativeField, NonFinite, NonCanonicalZero, RadiusClassMismatch, with field
  identity where useful; no generic successful fallback.
- AttemptStop::PublicationCertificate { index: Option<usize>, issue } maps to
  UnresolvedReason::PublicationCertificate with a preserved failed attempt.
  Semantic reason: publication_certificate. It does not escalate.

Keep METHOD_TOKEN=contribution_preserving_multiprecision_v1 because formation,
solve and recovery algorithms are unchanged. Propose POLICY=M03-INTEGRITY-MP-v2
for the strictly stronger acceptance contract. Current source registration is
adaptive.rs's exported const plus RetainedEvidence.policy; scoped source search
found no separate maintained policy-token registry. H/VR currently serialize
attempt enums through Debug strings. Do not retrofit old v1 evidence to v2.
ROOT's additive ruling/policy registration and D2's later recognition are still
explicit integration decisions; this plan does not register a token by prose.

Order remains R7 (a), (b), (c), (d), then certificate. Existing R7 summary fields
continue to summarize their old tests; they do not claim to encode H. Mark
Accepted/Verified only after the certified draft exists. Numeric H failure is
Rejected(PublicationEnclosure) at the first failing layout row/predicate;
release draft/radii and reuse the solved verification as next candidate through
the existing pending path. At 512 it becomes Unresolved(Ceiling) with the
specific rejected attempt; no precision extension.

Budget/Span/Exponent/other existing arithmetic failures keep existing terminal
mapping. A certificate integrity error is terminal Unresolved, preserving
attempt evidence rather than routing through Refusal::Structure and losing it.
Preserve StageGuard's Case-before-Invocation precedence, and prior R7 failure
precedence. Finish_selected's existing nonfinite certified-B finalization guard
remains effective. Combinations re-enter run_schedule and receive their own
certificate/radius box; operand standing/radii are never reused by row index.

## 5. Exact work, legacy boundary and replay

Keep the current 19 StageWork fields. The entire acceptance decision remains
inclusive stop_rule_work/stages.stop_rule. Charge new production certificate
work to the candidate's AttemptWork+SumWork, case total and invocation exactly
once, even on H rejection or terminal stop. Do not charge it to verification,
shared cache or both. Old R7 decision work is charged before the new stage;
construct a fresh remaining-room StageGuard for the new stage. This avoids
subtracting/adding old work twice. Existing H segments label the combined phase
decide_<p>, so their schema and chronological boundaries remain valid.

Meter all new/repeated operations within the existing LME definition:
- H construction, exact sign/absolute checks, all three relative comparisons.
- RU64(H), including cloned ExactWideSum netting/rounding and product-reaches
  correction scratch; the existing directed_ratio helper currently hides those
  local SumWork counters.
- Exact small-A1 bound formation and its WideContext operations when provisional
  classification calls it; the existing pure row_bound helper hides that work.

Use shared internal arithmetic implementations with an optional spent-work sink
so existing public pure helpers retain signatures and bits. New metered helpers
return errors and work before propagating failure. When cloning a sum with
inherited counters, merge only the newly incurred delta, not the old H work
again; max_span evidence is max, not a subtractable work charge. Capture every
scratch accumulator/context once, including stopped loop paths. Check the guard
after bounded primitive groups/each row and inside directed correction loops;
before publishing success or a numeric rejection, perform the final work check.
A terminal arithmetic error keeps its existing observed precedence. No claimed
budget pass is based on a post-error zero counter. Existing span remains 8128 bits.

ROOT explicitly retains the legacy ExactAccumulator boundary. The helper
exact_publication (adaptive.rs:990; exact_sum.rs:205–295) has no work counter;
its binary64-pair products, net/round and ordinary binary64 publication/scaling
do not report AttemptWork/SumWork today. Preserve its one-round value algorithm.
Moving prescribed publication before H means one call per input-derived row
for each candidate that reaches certification, including candidates subsequently
rejected: at most three candidate passes, rather than only the old final accepted
pass. These repeated legacy calls are disclosed and remain outside the inherited
LME instrumentation under ROOT's routed direction. No invented per-call price,
exact_sum.rs instrumentation or total primitive CPU-work claim is proposed.
If implementation exposes a conflicting governing metric requirement, return the
exact conflict before extension; do not erase or misprice those calls.

Replay means rerunning the source through the unchanged schedule plus new gate
and comparing policy, outcome/reasons, publication/class/floor/radius bits, state
identity and exact work. No existing standalone retained receipt replay API was
found in the bounded source; do not advertise one. Private tests use fresh,
cache-reused and combination runs to prove deterministic equivalence. Source-
matched frozen external oracle comparisons remain independent of these replays.

## 6. Reviewed lifetime/phase contract for K6c

Incorporating CORRECTION.md and its verified RV28 backcheck:
- Exactly one radius box per selected solve, logical 8Q payload bytes plus a
  two-pointer-size header (16 bytes on this target before struct padding).
  Q includes InputDerived/Unpublishable sentinel slots.
- Allocate the logical Q-slice initialized to sentinel. A safe Vec-filled-then-
  boxed implementation must expose actual capacity/shrink/reallocation overlap
  to K6c; do not label logical 8Q an allocator/RSS bound.
- Draft values Vec<Binary64Outcome>, Publication rows, body maxima/coupled scales,
  body_scales, existing decision floor/summary vectors and radius allocation
  coexist with prep/group, states, candidate/verification and report.
- Old R7 trackers are dropped when rule returns; bound by the maximum of that
  phase and certificate phase, not an assumed sum or omission. H/RU scratch is
  a finite number of ExactWideSum/term buffers and WideContext<16>, not Q exact
  sums. Record their actual live ranges and nested directed-ratio stack overlap.
- On success drop provisional value/scaling temporaries and move Publication/
  radii once. On rejection/stop drop both draft and partial box before escalation.
  Verification reports remain transient; no report retention or recomputation.
- Derived Clone of RetainedSolve clones Publication and the boxed radius slice;
  prep/group/state Arcs remain shared. Count clone multiplicity and actual struct/
  enum size changes (AttemptReason/Stop/Record), not just Q payload.
- Combination certificates own a fresh box and cannot inherit operand radii.

ROOT should send this settled ownership information to the K6c owner after
implementation freeze. I22 does not resume or edit I21.
H/src/k6/w1/counts.rs report+decide/end maxima, W1SizeFacts and VR/src/scale.rs
ported estimate all require final source binding. H/VR admission/backstop,
admission replay and post-KF3 W1-T4 remain outstanding. No E_max or memory bound
is established by this plan.

## 7. Mechanical consumers and exact conditional observation paths

No source edit is expected for these consumers if the 19-stage choice is kept;
their validation is nevertheless required:
- H/src/k6/w1/staged.rs: stage_fields, own_total, charged_by, stage_sum,
  work_closes, stages_equal_totals, work_by_precision, segments and prefix_matches.
- H/src/bin/k6_observe/w1.rs: Debug attempt output, stop_rule_work and dynamic
  stage_fields. New terminal errors must retain attempts through the generic path.
- VR/src/records.rs: existing Debug outcome plus own_work, exact_sum_work,
  stages.stop_rule, stop_rule_work, invocation_charged and possibly selected/
  verification precision, attempt role/reasons and corrections when H escalates.
  No new JSON field or vk-case-record schema change is proposed.
- VR/src/lane.rs and tests/lane.rs compare protected standing/availability and
  deterministic observations. tests/files.rs pins the expected-unresolved list.
- VR/runner/vk_scale_runner.py and H/VR counts/backstop consume memory estimates,
  not the new scalar radius directly. Their mechanical updates belong to K6c.

The exact generated snapshot paths potentially requiring ROOT-authorized refresh:
VR/observations/kernel_lane/{rf_chain,rf_skew,rf_weak,rf_large,rf_invariance,
rf_range,rf_zero,rf_finite,rf_mech,rf_cancel}.json;
VR/observations/kernel_lane/invariance.json;
VR/observations/kernel_lane/parity.json;
VR/observations/kernel_lane/SHA256SUMS.
Generator is VR/examples/vk_records.rs with --write only after explicit decision.
First produce read-only output and a field-level delta. Expected new work-only
differences may refresh after ROOT disposition; changed precision/class/outcome,
availability, parity or invariance must be individually reconciled.
Protected inputs cases/*.jsonl, cases/expected_unresolved.json, references,
class/not-covered expectations, tolerance and independently frozen truth do not
change. Preserve all old execution packets. Do not bless failure by regenerating
records. No current source grant for these 13 files is inferred.

## 8. Public contract and SI unit boundary

The minimum correction changes admission/policy and typed attempt reasons, not
published row value/scale/class/bound formats. Private radii are not
receipt fields. F2a's future cross-crate data accessor, producer finalization,
receipt encoding and D2/Rust-Python-TS recognition remain separate work.

The routed addendum fixes the coordinate contract for that later work:
raw y in U, exact positive a_U mapping U to SI, normalized n=N_U(y), and
S_n/b_SI reconstructed in SI by existing D1/G5b/c. mm uses RN(y/1000),
not multiplication by rounded 0.001; kN/kN-m use RN(y*1000), MPa RN(y*10^6),
identity SI units preserve the prescribed operation. Unsupported table units
remain outside covered kinds. An SI certificate is not a unit-conversion test.

Future absolute S-I binding is (n,SI_unit,b_SI), outward SI endpoints, then
outward conversion to the rule's unit; it never adds SI b to raw y in U.
Relative/InputDerived point binding retains (y,U). Product admission must
include actual normalization discrepancy and the raw-unit relative check,
retain real G5a shape/zero/sanity/lower/summary preflight, and bound every covered
derived formula. No reliance on a dropped report, epsilon*S_pub shortcut,
old closeness proof, or observed-zero exemption is allowed.
RV28 has verified those coordinate/binding specifications; per-formula and
runtime qualification remain open. No facade/interval-engine or unit
implementation is included in this kernel write set.

## 9. Finite verification and next decisions

TEST_AND_MUTANT_PLAN.md gives bounded fixtures, exact source controls,
discriminators and acceptance. All tests/mutants remain UNRUN. Sequence after
grant: targeted certificate tests, exact C17 regression, frozen B controls and
separately admitted new controls, old R7/A1/A2 suites/mutants, H/VR parity and
field-level observation disposition, independent complete-diff review, then
required actual-candidate CI/DEC-025/GEN-8/T9/both-entry and practitioner/receipt
gates. No build overlaps the current records sweep or another exclusive slot.
Use existing M5 guard, tool-managed supervision, 1.97.1/offline/locked/-j4,
RUST_TEST_THREADS=2, private target and ROOT-bound VENV when ROOT grants jobs.

Exact decisions still needed:
1. Independent implementation review must verify the already-backchecked
   radius/sentinel/lifetime and RV28-N1 work/scratch accounting in actual code;
   there is no remaining RV28-1 specification hold.
2. ROOT registration of policy v2 and typed reason semantics, including terminal
   PublicationCertificate mapping; no policy decision is inferred from this file.
3. Explicit four-file implementation grant and later finite runtime/mutant slots;
   confirm whether source-level pairing wrapper above is the desired small
   integrity boundary. No host slot is held by this plan.
4. Independent exact expectations for the three newly specified source fixtures;
   ROOT routes oracle work. Existing B/C17 truth and oracles stay fixed.
5. Explicit VR snapshot-regeneration scope after measured deltas and protected-
   criterion disposition, and separately owned final K6c estimate/integration.
6. Any observed protected availability conflict returns to ROOT/owner; honest
   refusal alone does not authorize reversing a protected selected outcome.

The implementation direction is concrete enough for code review after those
bounded dispositions. No source repair, design acceptance beyond ROOT's supplied
direction, product qualification, release or closure of A1 is claimed.
