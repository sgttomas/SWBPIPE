# RV54 source trace

Reviewed candidate `38798e6ee477ce23bf9bd2a708a1b2e268def5b3`, base
`887b790c2a646a67f3ac28e63c3eb57275c1868b`; maintained predecessor `fdae294` is
byte-identical to base under P/core. P=projects/chirality-piping,
K=P/core/solver/frame_kernel/src/structural/retained. All eight maintained files
and the I40 return were read; instruction, contract and source hashes accompany
this note. No actionable defect found.

## Construction, counters and representation

`origins.rs:35–82,318–347,350–589`: OriginCapacity checks the aggregate case,
combination, operand, call and run counts and 4R/7R. Each of six global vector
reservations checks count*element_size <= isize::MAX and uses try_reserve_exact.
Case entry checks consumed calls/cases, then reserves owners/source/run/output
vectors before registering sources or running work. Combination entry checks
calls/combinations/operands and reserves its requested/selected-index and single
owner/source/run arrays before mutation. Those arrays are populated at most to
their checked capacities. Failed reservations leave the store and meter unchanged.
Subsequent encoding/preparation numerical allocations are explicitly outside a
qualified resource permit; this checkpoint does not claim to make them fallible.

The aggregate declaration permits different per-call lengths; it is not a retained
sequence of mandatory call shapes. This is safe: total cases <= C, combinations
<= Z, operands <= H and calls <= A. Thus sources/runs/selected/groups <= C+Z;
refused combinations consume quota but no source/run/group. Empty case and
combination calls still consume A. Each run has at most four solves: a solved
verification becomes the next candidate; failed verification advances c by two;
failed candidate advances c by one. Verification-shared runs at most three times.
Therefore builds <= 7R even for uncached budget retries across runs. Source-base
plus position and group-base plus local-index are bounded by these same global
counts before their unchecked additions. No source of externally supplied ordinal
or post-admission capacity growth was found.

## Actual builds and selected association

`adaptive.rs:3907–3950,4006–4040,4129–4178,4207–4238` captures the actual
obtain/obtain_verify built bit, stop, WorkTotal and stages immediately after their
return. Nonbudget failure and success occupy the native slot; Budget never does.
`origins.rs:620–683` stamps only built=true, uses the actual resident id otherwise,
and keeps two links per physical record. WorkTotal is copied, not reconstructed;
charge and numeric-prior handling remain in the inherited core. Zero-priced reuse
retains status via shared_total.mul(0), while its origin still names the old build.

`adaptive.rs:4599–4612,3680–3710` clones GroupCache at actual selection.
`origins.rs:687–780` freezes that run's before/after slots, actual terminal attempts
and work, then retains a strong Arc<CasePrep> only for Selected. Each offered case
and combination constructs a fresh prep once; the public surface cannot rerun one
or replace its prep/cache. The registry prevents address reuse while alive; Clone
preserves the selected prep/cache. Occupancy matching plus full source bytes is
therefore a check of the actual immutable selected owner, not arbitrary digest
acceptance. Public metadata clones cannot register a solve. Context drop cannot
cause the same solve to acquire provenance in a new context.

`origins.rs:439–574` finds local selected owners, preserves native validation
precedence, refuses missing custody before combined preparation, then imports each
slot from the first occupied selected snapshot in authored operand order. Native
GroupCache::merged makes the matching first-occupied choice. New builds belong to
the new group; imported ids keep their original group/run. Local combination
cache changes never write operand snapshots or a later combination's imports.

## Terminal and legacy custody

`adaptive.rs:4917–5007` preserves invocation-entry, group-preparation,
source-preparation and schedule branches. Exhaustion/accounting entry has a source
and run but no group/build. `CoreRun::idle` joins the prior status into zero actual
increment. Refusal attempts stay owned in ExecutionOutcome; the legacy projection
occurs before each append and allocates no origin inventory.

`origins.rs:477–505` preserves typed pre-source refusals with an unchanged actual
meter and no new source/run/group/import/build. Native order remains empty or
nonfinite, nested, compatibility, then preparation; foreign association is checked
after native compatibility and before preparation. Valid combined prep is created
once and its actual identity and ledger encoding are copied immediately into the
source registry, before run_core can return unavailable. This is real K4CMB/K4LED
custody, independent of a surviving RetainedSolve. Numerical WorkAccounting.prior
and CountRange remain typed; no new Debug parsing or arithmetic was introduced.

`combine.rs:130–214` extracts validation and preparation without changing their
order, operands or legacy outcome conversion. The extracted helpers drop stiffness
comparison bytes and temporary prep-reference arrays before the schedule; old
lexical scopes retained them longer. Recorded mode adds six capacity owners, nested
call arrays, source/stiffness/combined-ledger byte copies, index/output scratch and
one strong prep Arc per selected run. That Arc extends source/ledger/prescribed/
factor/identity/layout/extents lifetime, but adds no cache/factor/state clone.
Setup copies precede the exhausted kernel branch and still cost allocation/CPU.
Optional trace/projection stack layouts also changed. I40 truthfully requires P3
and other profile rebinding; no unchanged-memory or zero-setup-cost claim follows.
