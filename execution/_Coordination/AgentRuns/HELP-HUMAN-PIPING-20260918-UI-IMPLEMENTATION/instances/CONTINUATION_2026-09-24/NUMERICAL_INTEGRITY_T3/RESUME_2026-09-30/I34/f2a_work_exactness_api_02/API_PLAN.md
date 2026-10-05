# I34 API-02 — selected concrete implementation plan

Recommendation for ROOT and RV46; no code grant. Basis: numerical source
49034a940f, reviewed design 49d7dee436, RV46 review 221adf0202 and ROOT disposition
a130003526. K denotes frame_kernel/src/structural/retained. The API decisions below
are selected as one plan; compiled sizes and implementation witnesses are not claimed.

## 1. Representation and accessors

Add K/work.rs, register it in retained/mod.rs, and re-export the three public
read-only types through structural::retained_api. No dependency, heap registry,
thread-local state, second numeric implementation or new work price.

```rust
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkFault { Overflow = 1, Inconsistent = 2, Both = 3 }
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct WorkStatus(u8);                 // private; only 0..=3 constructible
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct WorkTotal { amount: u64, status: WorkStatus } // private fields
```

Public accessors: WorkStatus::is_exact(), fault()->Option<WorkFault>;
WorkTotal::status(), exact()->Result<u64,WorkFault>. The public API has no constructor
from an arbitrary claimed exact count. Crate-private constructors `zero()` and
`exact_count(u64)` apply only at source-proven constants/counters. Crate-private
`add`, `mul(u64)`, `remainder`, `join_status` return WorkTotal and never panic.
Each arithmetic operation joins input status before checked arithmetic. Overflow
sets O and a diagnostic amount MAX; underflow sets I and diagnostic amount 0.
On already-invalid operands the diagnostic amount is immaterial, and status never
clears. `legacy_saturated()` returns the actual amount for E and MAX otherwise.
It is crate-private except through existing legacy numeric getters. No unavailable
amount is described as a saturated lower bound. Zero multiplication preserves flags.

Insert one private `status: WorkStatus` field into SumWork, WidthWork and StageWork.
AttemptWork derives its status from its three WidthWork slots and checked weighted
aggregation, without another field. Keep existing public numerical getters and
StageWork's 19 public numeric fields for compatibility; add `checked_lme()` to
SumWork/AttemptWork, `checked_lme(limbs)` to WidthWork, and `checked_total()` to
StageWork. Add public StageWork::merge(&mut self, other:&Self), using the same
checked implementation as internal stage merges; H no longer duplicates 19 raw adds.
Existing scalar getters use legacy_saturated. Production accounting
uses checked accessors exclusively. SumWork/WidthWork merges update component
payloads with checked arithmetic, union all flags and test their aggregate.
WidthWork checked_lme/charge/merge remain crate-private; their limbs argument is
checked against 4/8/16 before price arithmetic, with any other value yielding I.
SumWork/AttemptWork checked_lme and StageWork checked_total/merge are public read
and checked-composition methods. Direct external struct-literal construction of
newly private-status owners is no longer supported; actual repository consumers
use Default, getters or returned records. This source-API consequence is explicit,
while existing legacy result variants and clean output representations stay intact.

Replace private totals in Shared, VerifyShared, Spent, VerifySpent, StopDecision,
PublicationSpent, CertificateMeter and CaseBudget with WorkTotal. StageGuard's
base/case_room/invocation_room also become WorkTotal. InvocationMeter keeps limit:u64
and changes charged to WorkTotal; `checked_charged()->WorkTotal` is public, existing
charged()->u64 is the legacy getter. `exhausted()` is true on non-E or exact charged≥limit.
Room clamps legitimate exact overshoot to 0; non-E room remains non-E. Limit/getters
and CaseLimit remain unchanged.

Add one private `work_status: WorkStatus` to AttemptRecord. It captures detached
shared/verification/stop totals and propagated scope errors; clone preserves it.
Provide public `work_status`, `checked_own_work`, `checked_shared_work`,
`checked_verification_shared_work`, `checked_verification_work`,
`checked_stop_rule_work`, `checked_case_charge`, `checked_invocation_increment`.
Each checks relevant components and joins the record's status; invocation increment
joins dependency status even when built_here=false contributes zero new amount.
Keep existing scalar record fields as compatibility views. No caller-created or
mutated public count/record is an instrumented-origin warrant.

Implement manual Debug for changed existing structs so E formatting is byte-for-byte
its current derived representation, omitting the new field. Non-E formatting adds
`work_status` and is never an exact observation. WorkTotal's Debug formats E exactly as the prior bare u64 and marks invalid
payloads unavailable. WorkStatus Debug names Exact/Overflow/Inconsistent/Both. Unchanged existing error variants retain their Debug/Display.
New size diagnostics must tell the truth under a new reference profile; preserving
an old size literal is not a compatibility technique.

## 2. Snapshots and same-owner enforcement

Use two concrete private interfaces, each for its existing purpose.

For cumulative build/solve/verification stage snapshots, define:

```rust
pub(crate) struct WorkStream { anchor: std::cell::Cell<u8> } // non-ZST; !Clone, !Copy
#[derive(Clone, Copy)]
pub(crate) struct WorkSnapshot<'s> {
    stream: &'s WorkStream,
    total: WorkTotal,
}
// WorkStream::new(); stream.snapshot(total) -> Result<WorkSnapshot<'_>, WorkFault>
// after.delta_since(before) -> WorkTotal
```

WorkSnapshot fields/direct construction are private to work.rs; WorkStream::new,
snapshot and WorkSnapshot::delta_since are crate-private methods. The Cell is a fixed
identity anchor, never a serial counter; new() is non-const and never resets or
clones an anchor. Interior mutability excludes constant promotion/merging of
identity anchors; no snapshot crosses threads. delta_since first checks
`std::ptr::eq` of the borrowed non-ZST stream, then checked after−before. Foreign
streams set I even when their amounts match. Rust borrowing prevents moving a
stream while a snapshot refers to it; no Pin/unsafe/heap/serial-number counter is
needed. Define exactly one local WorkStream per cumulative context/sum cohort in
build_shared, solve_case_at, compare_states, build_verify_shared and verify_state.
Snapshot creation rejects non-E immediately; every capture uses `?`, so a delayed
compound stage assignment cannot postpone detection. Their single local `spent` closure always includes that cohort's same contexts
and sum, including initially zero secondary contexts. t0…t7 are snapshots; compound
stage differences add WorkTotals. A stream is never reused for another build or
new cohort. This private call mapping is part of source review; token equality
alone cannot make a caller's deliberately mislabelled arbitrary value truthful.
The guard's `t3−ctx−sum` is checked subset remainder, not a chronological delta.

Replace free `sum_work_delta(after,before)` and caller-supplied `before` arguments
with private K/wide_sum.rs `CloneWork { sum: ExactWideSum }`. `CloneWork::new(&sum)`
clones once with ancestry/status. Only `signum()` and generic `round(&mut WideContext<L>)`
are exposed, returning `(Result<T,SumRefusal>, SumWork)` for that operation's delta.
Each saves its own before and after inside the method; callers cannot supply a
snapshot, replace/reset the sum, obtain &mut ExactWideSum or Deref it. The same
CloneWork persists across certificate sign and round, preserving old clone lifetime
counts. Delta flags join before/after, component underflow is I, max_span is copied
from after. CertificateMeter `collect_clone(result,delta)` merges that delta even
on error. All other inherited clones execute fallible raw operations and propagate
errors without charging their previously unpriced counters.

Run/meter snapshots are private WorkTotals captured by the actual core call that
holds the one &mut InvocationMeter. Run increment is accumulated from actual charge
events, never reconstructed by subtracting after−before. C2's private call/run
constructors bind these snapshots to that invocation and actual origins. Conservation
is an additional checked test; no external arbitrary before/after pair is accepted.

## 3. Errors, early returns and stage closure

Add `WideError::WorkAccounting(WorkFault)` in wide.rs, with Display
`retained work accounting {overflow|inconsistent|both}`. WideArith/K3a never
constructs it; its old WorkCounter and arithmetic stay unchanged. The shared enum's
new variant is necessary to keep existing WideContext result signatures and `?`
conversions, rather than adding a second arithmetic-result hierarchy.

Add SumRefusal::WorkAccounting(WorkFault), AttemptStop::WorkAccounting(WorkFault),
and UnresolvedReason::WorkAccounting { fault:WorkFault, prior:Option<AttemptStop> }.
No box or string allocation: AttemptStop does not contain UnresolvedReason.
From<WideError>/From<SumRefusal> flatten this new variant to AttemptStop::WorkAccounting;
no mapping to Span, Exponent or Budget. WorkFault is non-escalating. Bound
refusal_kind already propagates non-Span/non-Exponent errors; keep that behavior.

WideContext's numerical method signatures remain Result<_,WideError>. WidthWork
`charge(kind,limbs)->Result<(),WorkFault>` checks the prospective per-kind count and its
unchanged width-weighted total BEFORE invoking numerical arithmetic. On failure,
mark status and do not commit the count or execute the operation. On success,
commit the one count before arithmetic, exactly as today; numerical error still
costs that operation. Every context passes its const L; WidthWork::merge also
accepts that slot width, supplied by AttemptWork record/merge via L/SLOT_LIMBS.
No mutable or caller-guessed width is introduced. Checked weighted totals use the existing eight prices.

ExactWideSum changes `signum()->Result<i8,SumRefusal>`,
`is_zero()->Result<bool,SumRefusal>`, `make_absolute()->Result<(),SumRefusal>` and
private `net()->Result<(bool,Magnitude,usize),SumRefusal>`. Existing add/round
Result signatures remain. Negation becomes private infallible swap invoked only
after successful signum; no other production caller needs it; the existing child-module unit test can still
exercise that private swap directly on a valid sum. Every consumer adds `?`, except
explicit closed fresh helper exceptions in §5. Quiet Debug sign inspection remains
uncharged and must not inspect a poisoned value: display `value_poisoned` there.

Fail-fast Result propagation replaces the need for a global scratch-status sink.
All priced owners also retain state. At each Spent/VerifySpent/StopDecision/
PublicationSpent exit, compute checked totals, join stage status and any
WorkAccounting error into total status. Thus an unpriced local fault surviving only
in Err cannot vanish. If the numerical result was Ok and total is non-E, replace
it with WorkAccounting and discard the unusable success. If a numerical Err already
exists, preserve it; keep accounting state in total. CertificateMeter always merges
local/delta work before applying this rule, including errors.

At solve_cases/run_schedule entry, check meter status before its ordinary exhausted
comparison: a bad meter returns accounting-unresolved without preparation/builds,
not Budget. Before cache success, numerical escalation, next precision, certification or final
selection, the schedule checks recorded/budget/meter state. Non-E terminal outcome
is Unresolved(WorkAccounting {fault,prior}), with prior holding any original non-work
stop (including Pivot/Condition/Span/Structure/NegativeEnergy); the physical record
retains that original Failed reason. No lost numerical error, accidental escalation,
or unreachable terminal(Condition) call. On a first work error prior=None. Existing
numeric-error-versus-budget precedence stays exactly as today on E paths.

StageWork::set(Stage,WorkTotal), add_to and add latch state before copying an amount.
Add Stage::StopRule to eliminate direct stop_rule additions. set/add_to/merge
latch state and return Result<(),WorkFault>; stage-local calls use `?` immediately.
The outer record/charge bundle explicitly collects merge errors until both case and
invocation ledgers are latched, then performs the accounting-terminal check before
any subsequent operation. Add From<WorkFault> conversions into the three error types.
StageGuard::with_base returns Result<Self,AttemptStop> and checks all supplied work
state before entry to a new stage; its calls use `?`. A non-E value's legacy
field is MAX; it is never validated as exact. close_stopped joins total/stage status,
then fills only an E nonnegative remainder; otherwise preserves partial fields and
adds I for proved total<stages. StageGuard::test accepts WorkTotal, refuses non-E
before comparisons, then preserves exact `used > room` and Case-before-Invocation.

Tracker collapse becomes Result<(),AttemptStop>. It returns WorkAccounting immediately;
only old numerical refusals become Evaluated::Refused. offer, TrackerSet::offer and
finish propagate it before prune/drop. No separate tracker flag or per-row flag is
needed: stored sums carry state and accounting errors cannot enter a prunable table.
All selected-input raw donor checks precede zero/empty/factor-zero shortcuts.

## 4. Raw mutation, pending charge and poison

Add `poisoned: bool` to ExactWideSum. This is numeric-value validity, separate from
SumWork.status. All numeric observations/mutations first reject poisoned state;
misuse without required reset marks I and returns WorkAccounting(Inconsistent).
Clone preserves poison. New creates false. `clear()` has no return and calls full
reset when poisoned, otherwise retains its current used-prefix fast path; reset
zeros both entire magnitudes, clears numeric poison, and retains every work count
and flag. Accounting poison therefore remains terminal even after reset.

For nonzero add_raw: compute term bounds/span without mutation; existing immediate
Span refusal has no charge and does not poison an untouched value. Before anchor,
shift or magnitude mutation, check term counter t+b and complete raw LME with
pending b=limbs+1. No charge is committed by this reservation. Each shift and carry
check includes the pending b in both term and aggregate remaining capacity. A
failed precheck latches O, leaves that event uncharged and does not perform its
mutation. Earlier committed events remain counted; the failed prospective event
is not retrospectively called actual spent work or a proved MAX lower bound.

Shift: check shift increment and all source/destination containment first, then
perform the existing shift and commit exactly 2*new_used. The main limb-add loop
checks index bounds before every access. Carry tail checks index AND the prospective
one-unit carry charge with pending b before writing, then commits that one unit at
its existing point. At successful completion commit b; no fixed 256 reserve/price.
Any error after a value mutation sets poisoned=true before returning. Compound
add_scaled/add_product/add_product_of track whether an earlier insertion mutated
the destination and poison it if a later insertion refuses, even if that last
add_raw failed before its own first write. An impossible
headroom/containment failure sets I, not Span. No out-of-range indexing, discarded
nonzero top limb, returned partial value or reliance on debug_assert.

Scaled multiplications check their existing M or other.used charge before their
local loop, commit it where currently charged, then call add_raw. A subsequent
Span retains that already incurred scaling cost. net/round/sign charge before
observation; all failed charge checks leave that event unperformed. add_product
retains context TwoProduct and successful first expansion term if the later term
fails. add_product_of checks donor state before b.net and each scaled insertion.

The reviewed proof T≤t, t+b≤2^64−1 ⇒ T+1<2^64 remains valid. Failed partial values
cannot re-enter this induction: only full reset clears them. Source/count/index
admission still precedes6n, offsets, residual m/64m, tracker sequences and r*r.
This plan neither selects new scalar limits nor substitutes flags for the separately
assigned P1 count guards; the final code grant must include that closed interface.

## 5. Complete helper/drop disposition

Dynamic locals use fallible operations and `?`: residual/fallback r,d,absolute,num,
den,t; decision difference/allowance and tracker ratios; verification prescribed t,
r/rr,inf/one/au,d; recovery/directed/bound/assembly/factor borrowed accumulators.
No unpriced local's numerical Result is converted to an Option or boolean default.
CertificateMeter preserves all collected prices and CloneWork deltas. A numerical
Span/Exponent caught by A2 can reset and continue its block policy only while work
is E; WorkAccounting propagates. No numeric failure is reset into successful work.

The only accounting-impossible conversions retained are these fresh finite helpers.
Bounds use I29's independently reviewed raw512/add_scaled1280/round384 atoms;
all numerical domain errors retain their existing Option/fallback behavior.

| Helper | Complete local work argument / selected handling |
|---|---|
| source::chord_checks | Each d owner: 2 binary64 adds+sign≤1152. Each c:≤2 add_scaled+sign≤2688. No ancestry except these d donors. Keep finite/in-span expects; new is_zero uses expect with this invariant. |
| factor::determinant_nonzero | sum ≤24 raw adds+sign≤12416; c128 has 6 TwoProducts and c192 has 12 (price32 each). Fresh per call, no reused context. New is_zero().ok()?; other existing .ok conversions stay. |
| adaptive::intensified_k | Fresh sum≤3 raw adds+sign≤1664; one width4 TwoProduct=32. New signum().ok()?; existing fallback remains. |
| adaptive::row_bound/classify | num 3 adds≤1536, den 1≤512. directed_ratio's inherited clones start at these bounds; one sign and one round per clone add≤512. Context only two rounds+one division≤17506 at L16. Each correction scratch is fresh,≤2 add_scaled+sign≤2688, independent of iteration count. Existing .ok/∞ fallback remains; no cumulative context operation in correction loops. |
| directed::binary64_up | Fresh sum 2 adds+sign≤1152; select explicit `?` transfer (already returns AttemptStop), not a swallowed exception. |
| new supported WideContext::new expects | Factory only constructs zero work after validating the constant supported precision; no charge transition can fail. |

Legacy ExactAccumulator in ledger/prescriptions is unmetered and unchanged, including
its checked carry refusal; its signum is not the fallible ExactWideSum method.
K3a WorkCounter remains outside W1. Private helpers above cannot receive a seeded
near-MAX owner; near-MAX tests target the dynamic interfaces, not their proven fresh
constructors. Tests must keep these operation-count ceilings as source regressions.

## 6. C2 recorded/legacy paths and C1 projection

Keep C2's additive ExecutionOutcome three variants and unchanged legacy CaseOutcome
shape. The new accounting stop is its Unresolved variant with the new typed reason.
Select a private `CoreRun { outcome:ExecutionOutcome, work:RunWork }`, where
`RunWork { case, invocation_before, invocation_increment, invocation_after }` has
four private WorkTotals and read-only accessors. One charge helper updates case,
actual increment and InvocationMeter before any error is returned. Reuse adds full
case work, zero new invocation amount, and unconditionally joins origin status.

RecordedCase becomes `{outcome,origins,work}`. C2 RecordedCombination remains
PreSourceRefusal versus WithRun: its before/after are WorkTotals; WithRun additionally
holds RunWork. Early operand/ledger failures still have no source, group, import or
Run; unchanged numerical meter snapshots may nevertheless inherit prior non-E status.
No fabricated zero or exact equality follows from empty attempts. Nonrecorded mode
uses the same CoreRun, allocates no origin inventories, and erases only Refused
attempts at C2's existing compatibility adapter. Accounting terminal status remains
in the Unresolved reason and meter even after any compatibility projection.

Cache success/failure slots replace their detached total:u64 with WorkTotal.
Success is installed only after E finalization. Non-budget failure retains total
and original stop; budget failure remains uncached. A bad cached origin defensively
causes the new accounting terminal on reuse even when built_here=false. Finish-time
snapshots and first-filled imported slots preserve this typed state; no digest cache,
backfill or changed numerical cache order. C2 Build.work is taken from exact() only.

C1/C2 need this specific normative reconciliation: qualified checked-source/carry
mechanism plus closed pre-execution scalar admission is the alternative to the
unprovided global lifetime no-wrap theorem. Every Run/Build/Call amount must be E
before projection, then satisfy the existing conservation and safe-JSON checks.
Native exact MAX is distinguishable but still unencodable (>2^53−1). Numeric-only
successor Work/Stages schema need not grow status fields: any O/I aborts successor
finalization to the preserved ordinary base, with unchanged spent ledger.

Select diagnostic details under existing RETAINED_PRECISION_UNAVAILABLE,
reason=receipt_encoding: `work_counter_overflow` for O, `work_counter_inconsistent`
for I/OI, `work_counter_unknown` for missing qualification, `work_counter_range`
for E outside JSON range. Private run evidence retains the full OI bits and typed
prior stop. No `saturated_lower_bound` text for these states. C2's finite defensive
Reason map adds stop/work_accounting{fault}, unresolved/work_accounting{fault,prior:
null|Stop}, and WideError work_accounting{fault}; fault=overflow|inconsistent|both.
These typed mappings do not permit non-E charge fields into a successor receipt.
G5 rejects any work_accounting cause in a purported successor even if its numeric
fields have been forged into a small internally consistent ledger.
An unknown legacy-only count view must decline before execution under the admission
contract, not gain qualification from these getters. Earlier selected numerical
standing remains unchanged when later work invalidates the aggregate receipt.
