# I34 — checked work exactness, bounded design

Status: design for fresh independent review. No maintained implementation, API,
contract identifier, byte allowance, or release is selected. Basis: main
49034a940f3f8cd3f3da4d4cbc839943b808063d; I29 partial at 4c2c1e9857;
RV44 completed limited review at bdfd7ae75c; corrected C1 §2/RV43-F1; ROOT's
3d848df33a disposition. Source anchors use FK=projects/chirality-piping/core/solver/
frame_kernel/src/structural/retained. Source hashes and read coverage are recorded
under `_run_records`; this is neither a new global no-wrap proof nor an allegation
of overflow on a current workload.

## 1. One numeric core, explicit evidence state

Retain the existing numerical algorithms, operation sequence, prices, precision
schedule, cache selection and work ownership. Replace loss-prone *accounting*
transitions with checked transitions. Carry one small state with each independently
movable work owner and one enclosing scope state for faults from temporary,
unpriced owners. This is not a second numerical engine or a new price model.

Abstract state is a two-bit set `{O,I}`: E={}, O={overflow}, I={inconsistent},
OI={both}. Join is set union; no error precedence erases a bit. O means a defined
counter/price/total update was not representable, including a next update stopped
before execution. I means a required difference/ownership/chronology invariant
failed. Neither means the numeric payload is an exact charge or necessarily a
saturated lower bound. Numeric overflow of a mechanics value remains its existing
numerical error, separate from work evidence. Missing/unqualified provenance is
an admission/projection failure, not an invented E, O, or chronology finding.

An abstract charge is `(amount,state)`. Amount is available as an exact charge
only in E. Tests here use `None` for unavailable amounts; that is a specification
notation, not a Rust representation choice. An implementation may retain an exact
prefix or saturating diagnostic payload, but no accounting consumer can extract
it as exact when state is non-E. Do not advertise a lower bound without a separate
proof for that *particular* payload. An O caused by prechecking an unexecuted
operation does not prove that actual spent work exceeded MAX.

| Transition | Required result |
|---|---|
| New independent owner | E, zero counters; complete instrumented provenance required. |
| Add nonnegative charges / merge / record context | Join all input states; checked add every component and aggregate actually formed; add O on unrepresentable result. |
| Weighted cost | Preserve the existing eight width prices; checked count×price and checked sum. Join status even for zero coefficients. |
| Chronological delta after−before | Only E, same cumulative stream/clone ancestry, and componentwise after≥before yields an exact difference. Otherwise propagate input loss; add I for demonstrably bad chronology/underflow. Never saturating-subtract accounting deltas. |
| Exact equal snapshots | E zero is valid, including MAX−MAX if both are proved exact and correctly paired. |
| Lost snapshots, including clipped MAX−MAX | Non-E; cannot become exact zero. |
| Value clear / refusal reset | Numeric reset as today; keep cumulative counters AND all evidence state. Refusal reset still clears full magnitudes. |
| Clone/copy/move, cached success/failure, snapshot | Preserve state with the object and any retained total; a copy is not a new accounting origin. |
| Numeric operand transfer with historically unpriced local work | Join state only, with zero new LME. Do not add donor work or retrospectively price local arithmetic. |
| Error/early return / discard / prune | Join all incurred local status before dropping owners or choosing a numerical branch; retain original error and partial work. |
| Fresh case / next precision / fallback | Fresh *local* counters may start E; parent invocation history and dependency states cannot reset. |
| Budget room `max(limit−used,0)` | Legitimate clamped room if used is exact; overshoot is not I. If used is non-E, room is unavailable and execution cannot continue using a fabricated room. |
| Projection | Exact native status plus complete provenance, checked conservation and safe-JSON conversion; otherwise no successor receipt. |

The delta's same-stream premise is substantive. A mere after≥before comparison
cannot detect swapping equal-valued snapshots from different owners. Snapshot
pairing must be constrained by the private operation's ownership/call structure
or an opaque lineage mechanism; no global incrementing lineage ID is required
by this design. `sum_work_delta` only subtracts the four charge components;
max_span remains the after snapshot's maximum, never a price/difference.

A scalar MAX sentinel cannot meet this table. MAX is a valid exact u64 value;
MAX−MAX erases saturation; local drop/terminal omission erases the sentinel; and
one sentinel cannot distinguish overflow from underflow. A sentinel plus an
unerasable tagged result is already explicit state. No complete smaller
sentinel-only design was found.

## 2. Minimal ownership and transfer discipline

SumWork owns raw four-component state, including status inherited when values
are cloned or used by another accumulator. WidthWork owns its eight counts'
state. AttemptWork can derive state from its three WidthWork values plus checked
cross-width aggregation; a duplicate persistent flag is unnecessary unless an
implementation stores an otherwise unrecoverable aggregate fault. StageWork
needs state because stages/deltas/closure can fail even with exact raw components.
Its 19 numbers are exact as a group only when that state is E.

Each detached scalar total/snapshot travels with its status, or is usable only
while its status-owning parent is borrowed. This applies to t0…t7, guard base,
Spent/VerifySpent/StopDecision/PublicationSpent totals, Shared/VerifyShared totals,
shared/verification/stop fields in AttemptRecord, CaseBudget and InvocationMeter.
A scalar u64 copied out and later reclassified from its magnitude is forbidden.
These are logical owners: state may live in a private wrapper around an unchanged
legacy public count view; this document does not choose field insertion. A legacy
count view cannot be re-imported as qualified E history merely because it is small.
Use an enclosing scope status to collect unpriced scratch/clone faults without
merging their prices. That scope has no independent counter model and requires
no heap registry or thread-local/global flag. Its lifetime follows the build,
solve, verification or decision being recorded.

All reductions check intermediates, not just their final value: raw LME,
width×price, AttemptWork LME, W+K, stage totals, compound stage differences,
shared-stage S+V, case totals, actual invocation increments and projection sums.
For the existing source this covers all ordinary additions listed in the source
manifest, including StageWork::add and decision/certificate stop-rule accumulation.
`close_stopped` only fills an exact nonnegative remainder; total<stages is I,
and an unavailable total/stage sum cannot be repaired into equality. Retain
partial stages and the original stop, with non-E closure status.

Every local owning accumulator must be consumed by one of three audited routes:
(1) priced work and state collected exactly as today; (2) only state joined to
its enclosing owner; (3) a closed fresh constant-size helper with a source proof
that work cannot become non-E. Route3 is limited to helpers such as source chord
checks, determinant checks and binary64-up scratch, whose inputs are plain
finite values and whose raw owners are freshly created. It is not available to
an inherited clone, arbitrary-length prescribed/residual loop, or cumulative
certificate context. If route3's proof or source changes, join status instead.

In particular, approximate_ratio/directed_ratio clones preserve ancestry; their
historically unpriced SumWork is not added to LME. Their state must be checked
before early zero/infinity returns. Tracker offer/collapse/prune/finish joins
state before it drops lazy/evaluated rows, even when the associated ratio/refusal
would be pruned by a later key. Storing state only inside `Evaluated::Refused`
is insufficient. Work invalidity is not a numerically ignorable tracker refusal.
The same rule covers verification's per-term t, per-row r, inf/one/au, prescribed
d, residual/fallback r/d/absolute/num/den, and stop-rule scratch. A successful
copy of a numeric value cannot erase the work fault of its producing scope.

## 3. Raw accumulator safety before mutation

A receipt-only status after value corruption is unacceptable. The current
wide_sum.rs span of 8128 bits leaves 64 carry bits in each 128-limb magnitude;
`add_raw`'s carry tail currently has a debug assertion before direct indexing.
The following local invariant avoids a new lifetime call-count theorem or a
second term-count field, provided the implementation enforces it at the raw owner:

Let T be the number of completed nonzero raw additions in the currently live
value, including a clone's inherited value. Let t be this owner's exact cumulative
term_limbs. Each completed nonzero add_raw charges b=limbs+1≥2; scaling/carry
charges only increase t. Clear/reset retain t; a clone retains both value and t.
Therefore T≤t. No merged aggregate is installed as a fresh raw owner's history.

After computing and checking the proposed term's span, but **before changing
anchor, shifting magnitudes or adding a limb**, require E raw history and checked
t+b≤M, M=2^64−1. This is an availability check, not a new charge. Then
T+1≤t+1≤M<2^64. Every term measured at the prospective anchor is <2^8128,
so each signed magnitude is <2^64·2^8128=2^8192. The same bound protects a
lowered-anchor shift: `high` covers the largest term leading bit and the span
check applies at the new anchor. This protects current value headroom independently
of future lifetime work or how rarely StageGuard runs.

During an add, check each carry-work increment with the still-pending base b
included in its remaining-capacity check; never allow the carry tail to consume
the final base reservation. Commit only the charges at their existing logical
charge points. Check shift_limbs before shift writes, scaling charges before
scaled work is consumed, and net/rounded charges before returning their observations.
No conservative fixed 256 reservation is charged or used to reject an operation
whose *actual* defined accounting fits. Zero raw inputs keep their existing
zero-add behavior; their dependency status is still observed.

Also make carry/index containment an explicit checked condition before any
out-of-array access or truncating shift, rather than relying solely on a debug
assertion. A violated internal carry invariant is an I/safety stop, not an exact
numeric result. Any partly mutated value is unusable until the existing full
reset discipline runs; `clear`'s used-prefix clearing is not a substitute after
an interrupted add. Work state remains non-E through that reset and cannot resume
this retained attempt. A failed numerical Span/Exponent with otherwise E work
keeps today's block-local reset behavior. Accounting failure must not be mapped
to Span/Exponent and swallowed by bound::refusable.

This proof relies on source-enforced raw ownership and completed-add accounting.
It does not follow from an arbitrary fabricated t or a reconstructed final total.
Fresh independent review must check both the pre-mutation ordering and all failure
edges. The finite reduced-width control is evidence about this invariant, not a
Rust array/mutation witness. An explicit checked live-term field is a fallback
representation only if implementation cannot preserve the existing-counter proof;
its layout/reset/clone consequences must then be added before qualification.

## 4. Stops, caches, terminals and invocation custody

Detect counter overflow at the increment; detect weighted/aggregate overflow at
the first evaluation; detect bad subtraction before use. A non-E result cannot
be used for a budget comparison, numerical acceptance branch, further raw sum,
cache success or selected finalization. It terminally invalidates W1 at the next
owned operation return, before a subsequent stage/schedule step. Arithmetic
failures retain their current precedence/reason and charged prefix; status is
orthogonal evidence, so a simultaneous error cannot hide it. A first accounting
failure needs a distinct terminal accounting reason; its API spelling is unselected.
It must not impersonate Case/Invocation overshoot, Pivot/Condition escalation,
or a per-block Span/Exponent.

Caches retain the complete originating state with both successful Shared/
VerifyShared and the non-budget failure tuples. Obtain transfers it even when
built_here=false and the actual new invocation charge is 0. Case price still
includes the full shared cost; invocation price includes only actual new builds.
The *receipt-validity state* joins dependency state unconditionally, including
coefficient-zero reuse, without charging reused work again. This conservative
join can make an invocation receipt unusable even where its new physical charge
alone is known. Never falsely describe that as a new build or loss of prior
selected numerical standing. In a correctly stopped execution, no newly tainted
success becomes a cache success; defensive transfer checks still reject such a
slot. Budget failures remain uncached. Invalid evidence, if retained as a
non-budget failure, stays invalid on every reuse.

Finish-time cache snapshots and first-filled combination slots retain state,
origin and alias identity. GroupCache::merged does not merge counters from equal
sources or change first-available slot selection. Every actual call/retry and
combination uses the same parent meter as stipulated by C1. New case state cannot
clear it. When any participating scope becomes non-E, the invocation's receipt
state becomes non-E before the scope can return; later runs cannot restore it.
No refund, fabricated exact before/after delta, or new work at guessed zero room.

The recorded entry must own a terminal envelope for every exit: actual outcome,
actual physical records (possibly empty only when no schedule started), run state,
and the invocation snapshots/state. Build errors before a physical record is
complete still update that record/scope; pre-source refusal must not invent a
source or Run. NegativeEnergy/Structure through finish_terminal and combinations
must retain the vector before current Refused adapters discard it. A later
inexact run cannot leave body.work.charged labelled exact merely because an
omitted terminal vector makes previous records close.

Legacy and recorded entry points must share this core. Legacy clean calls retain
the existing values, classes, reasons, counts, Debug/serialized bytes and public
observation order. A legacy shape that intentionally omits Refused attempts may
remain as a compatibility projection only *after* the recorded core has latched
run/invocation status; that projection is not complete exact-charge evidence.
A caller exposing only a u64 charged getter cannot qualify an exact receipt.
Whether this uses additional fields or an additive recorded wrapper/accessor is
an explicit later API choice. If a layout change alters an emitted size diagnostic,
that diagnostic must truthfully change under new profile correspondence; literal
byte identity of size reports cannot be promised alongside unchosen new layouts.
Prefer unchanged legacy observation views and separately qualified new layout facts;
never preserve an old size number by printing a false one. No duplicate solve or second counting pass is allowed.

## 5. C1 replacement and remaining upfront safety

Once independently reviewed instrumented source and affected consumers implement
this design, exactness follows by induction: fresh qualified owners are E with
correct counters; every E update uses the unchanged exact price; every loss marks
state before use; union, snapshots, caches, error and terminal custody cannot erase
loss. Thus an E terminal run with complete lineage has exact native charges.
This replaces the *unprovided global cumulative no-wrap bound* for receipts. It
does not make retrospective equality alone a proof. C1 §2 must explicitly accept
this qualified mechanism as an alternative premise; until then its current
pre-execution no-wrap requirement remains the governing blocker.

With that premise, C1 still checks own=W+K, stages, D+Q≤O, shared stages=S+V,
physical/logical B/T conservation, build flags, execution order, before/after and
safe-JSON range. Exact native MAX can now be distinguished from saturation, but is
still >2^53−1 and cannot be a successor JSON count. Unknown, I or O evidence uses
the existing transactional ordinary fallback, with diagnostic wording distinguishing
unknown/inconsistent/overflow from proved exact values. C1's current suggestion
of `saturated_lower_bound` for otherwise unknown native counts needs narrowing;
this design supplies no general lower-bound claim. No incomplete `run` is encoded
with fabricated zeroes. Earlier selected rows' numerical standing is not rewritten
by a later case; the aggregate successor transaction still cannot finalize an
inexact work receipt. Preserve the ordinary base and spent ledger.

Separate checked pre-execution admission remains necessary, before affected
construction, against actual immutable raw counts, accepted widths 4/8/16,
precisions 128/256/512/1024, feature set, target integer widths and source identities:

- Node×6 and all DOF/index endpoints; node/body/member/spring/station/support and
  raw child counts; u32 IDs/encoded lengths and total encoding capacities; source
  constructors use node×6 and narrowing casts before derived counts exist.
- Pattern/contribution uppers 78m+s+6d and144m+s+9d, dense N²/profile F(F+1)/2,
  counts/prefix sums/adjacency/RCM/profile offsets, row/header/allocation lengths;
  evaluate checked uppers first, then validate actual same-source counts.
- Layout upper 7n+12m+6t+s+3d+r+2g, loaded/prescribed/combination operand and support
  membership multiplicities, all address/capacity expressions. Admission cannot
  borrow H/VR's no-support/zero-prescription assumptions for product sources.
- Residual/fallback count→m=2count+2→64m; factor operations and64·operations;
  tracker offered sequence and held−before+after; scale/exponent/index products.
  I29's sufficient 128(Z+1)<M and 128F<M guards remain usable after binding counts.
- ceil_sqrt's usize→u64 and binary64→integer path and r*r: I29's sufficient
  F≤2^32−2 restriction protects each block n_c≤F and (F+1)²<M. Widening/replacing
  this scalar algorithm is a separate implementation choice, not supplied here.

A work flag cannot repair already-wrapped mechanics inputs. Counter checks also
supply no bound on elapsed time, unmetered source work, allocations, peak RSS,
transport, OOM or caller overlap. P1–P5 memory and caller qualification remain.

## 6. Concrete closure required before implementation

The source/consumer/layout/test manifest is in SOURCE_MANIFEST.md. Abstract controls
in `_run_records/controls.py` exercise this state's rules, preserving exact finite
prices; they import no product, solver, numerical model or third-party package.
They are not implementation witnesses. Outstanding choices are: exact Rust state
placement/accessors and terminal reason; ownership-constrained snapshot interface;
recorded-versus-legacy API adapter; partial-write failure mechanics; diagnostic
encoding and profile revisions. These are a finite design integration list, not
a request to broaden numerical policy. No code/API is chosen by this packet.

Before reliance: fresh independent source/design re-derivation; authorized frozen
maintained manifest; code review of every mutation and drop edge; reduced-capacity
or seeded near-MAX Rust tests in checked and optimized builds; clean-case byte,
price, classification, budget/overshoot and cache differential checks; exact
candidate K6c/H/VR layout/source/profile correspondence and F2a P1–P5 recalculation;
C1 reader/producer/terminal tests across Rust/Python/TS; normal required gates.
Historical evidence stays historical. No runtime/host qualification was run here.
