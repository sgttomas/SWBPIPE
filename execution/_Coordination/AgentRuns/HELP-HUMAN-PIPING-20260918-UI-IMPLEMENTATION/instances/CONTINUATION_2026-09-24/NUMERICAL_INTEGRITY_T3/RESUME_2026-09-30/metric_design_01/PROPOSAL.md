# K6c metric/base composition — bounded source proposal

Status: **conditional composition established; no complete numeric E_max,
admission acceptance, implementation or measurement authorized.**

## Recommendation and tradeoff

Keep one shared source-derived kernel phase ledger in H, with H-stage and
VR-global consumer compositions. Retain the existing fixture domains, requested-
heap windows, move-accounting metric and admission rule. Do not replace an
unproved runtime term by a measured constant from another process.

The smallest useful observation option is a scalar cut in the **same run**:
`max(observed prefix peak, observed live bytes + proved future envelope)`.
It can remove the need to derive allocations that occurred wholly before that
cut. It cannot discharge future ownership, make a no-op/counts-only baseline
universal, or supply the existing runner's before-launch estimate automatically.
Under ROOT's current direction, no admission/record-contract change is selected.
Therefore use this option only as an explicitly conditional per-run composition
proposal; retain source-derived prelaunch terms wherever the existing admission
contract requires a value before those observations exist. Return any missing
term rather than start a universal runtime-source audit.

This is a smaller design than a whole-binary source derivation, but it is **not
a complete substitute for that derivation under unchanged prelaunch admission**.
The alternative that preserves the existing pure estimate interface requires
proved source upper bounds for its caller/runtime terms, with the exact finite
argv/input/build domain. Those bounds remain unavailable here.

## The cut theorem and ownership rule

Let `t0` be a quiescent allocation boundary in one process. Read the existing
allocator's `current`, `peak`, and `peak_move` without intervening allocation.
Let C0 be current requested bytes; P0 and P0m the process prefix peaks. Let
F(t0,D) and Fm(t0,D) bound, for every permitted continuation of descriptor D,
all future requested ownership above a conservatively retained C0, including
reallocation accounting. Then:

```
Eglobal  = max(P0,  C0 + F(t0,D))
Eglobalm = max(P0m, C0 + Fm(t0,D))
```

This is a conditional bound for every run satisfying D and the read-boundary
premises. It is not one universal number derived from sampled C0/P0. P0 captures
already completed peaks; C0 captures current ownership. Using C0 alone loses a
freed prefix peak. Using P0 as a live base is safe but unnecessarily pessimistic.
A final measured peak substituted after the solve is a tautological comparison,
not a predictive future envelope.

The simplest composition makes **no subtraction from C0**. Keep all allocations
present at the cut notionally alive; in the future ledger omit only the explicit
owners already prepaid by that cut, and add all later distinct allocations and
any growth of prepaid ones. Do not add the old H/VR `fixed` term wholesale: it
mixes caller, input source, source clone, case and group. At the proposed pre-solve
cuts the caller exists but the kernel source/prep/group generally does not.

Optional tightening uses a proved partition: subtract the **actual requested
bytes**, or a justified lower bound, of identified live modeled allocations A0
from C0, then add a complete future upper for those same allocation identities.
Subtracting an upper estimate of A0 is unsafe. An Arc clone adds no second payload;
a deep clone adds its own children. Future growth/lazy initialization of any
unmodeled baseline owner still needs a bound. Sequentially-consistent counters
do not themselves make three reads an atomic snapshot in an allocating
multithreaded process; bind quiescence or the equivalent accounting premise.

For a prepaid old buffer of r bytes growing to n, in-place current grows by
n-r, while the existing move peak adds n above the frozen old-buffer base
(old plus new). For a fresh buffer use its full source-derived move envelope.
A conservative full old-plus-new term may overcount the prepaid old buffer but
must be labeled; do not silently subtract a modeled capacity. Shrinking realloc
has no old-plus-new increment in these particular counters. Requested bytes do
not include System allocator overhead, size-class slack, stacks, RSS or macOS
footprint. The move model is a synthetic growing-realloc counter, not proof of
physical simultaneous allocation or actual allocator movement.

## Exact consumer windows and candidate boundaries

References here use K6C `82cc9fa9d5`, with H=`P/core/solver/performance_harness`
and VR=`P/validation/benchmarks/numerical_robustness`.

| Consumer | Existing metric and smallest sound composition |
|---|---|
| H timed repeats | `main.rs:280-317`: stage_begin output drops, then stage_reset at287 and current_begin at288. Observer::end snapshots at298-305 **before** stage-line construction. For each source/solve stage s, `C_s + F_s` bounds the stage from reset to that snapshot. Max across the same stages/repeats bounds repeats_heap_peak; use move F for repeats_heap_peak_move. The existing stage line already reports current_begin. A process-global prefix maximum must not be relabeled as H's measured stage maximum. |
| H budget prefixes | `main.rs:946-971`: the prefix adapter runs at956 before the fresh solve-stage reset. w1_solve returns after Observer::end emits its stage line. The prefix metric then re-reads stage_peak at965-966, so F_prefix **includes that stage-line construction/emission**. It ends before prefix_line at967. The next prefix's adapter again runs outside the fresh reset. Retain saved full attempts, prefix-limit vector/labels and the input source in each boundary base. No prefix or its pre-check overrun/unwind is removed. |
| H common pre-admission composition | Candidate tH is after counts/file-read scopes and counts output finish, immediately before admission_estimate_bytes at659 (sizes at647; counts-only branch already handled). C_H includes model, frames, Args, counts and runtime owners. Future source/prep/group/solve allocations are additional. An admission-time H bound needs the union of later saved-attempt children, prefix controls, source/error formatting and any persistent caller growth through every later observed window. Per-stage C_s observations alone arrive too late for that prelaunch/admission bound. |
| VR global | Candidate tV is after drop(source) at399 and sizes at400, immediately before estimate at401 and the counts emit at402. C_V includes selected Case/Model, Args/id/hash/runtime/retained count owners; kernel source is gone and future lane source is additional. Keep P0/P0m from process start, not Phase::stage_peak. F_V includes counts-line output/backstop paths, lane control/source/solve/comparison/record, report/output clones, expected-list initialization, RCM, sparse binary64 parity, and summary construction through its peak sample at511. Bounding the full final summary emission is a permissible conservative extension, not a change to the reported sample. |

H source-stage and solve-stage reasons are formatted before Observer::end
(`w1/staged.rs:59-63,77-87`), so their finite Debug grammar remains a future
term. In a solve-stage C_s, the original PrimitiveSource is already paid; its
CasePrep clone is not. Previous-repeat attempts are already paid only at that
particular stage cut; they are future owners at tH. Prefix_line transients are
outside the prefix sample, but any persistent effects must be present in the
next boundary or bounded from tH.

VR's old family parsing, large input read, JSON parse and counts helper peaks
can be covered by P0 once tV is reached. This does not make their *execution
before the cut* safe under an unproved prelaunch estimate, nor delete them from
the existing global metric. Existing cap/watchdog behavior remains distinct
from the estimate. Post-tV JSON/format/library scratch cannot be paid by P0.
In particular V10-V12 and the sparse Binary64Envelope in source_01 remain.

## Admission and replay consequences

The existing runner makes its decision from separate counts-only process
records before launching the measured process: VR runner154-175,181-192,292-304;
H runner496-570. It uses the same-family ascent and rho calibration, footprint
projection, projected RSS <=0.8C, named exclusions/approvals and binary half-heap-
cap backstop. Keep all those predicates and their historical denominators bound.
The no-op RSS/footprint subtraction used to calibrate rho is **not** a requested-
heap baseline that may be subtracted from these heap observations.

1. **Source-only prelaunch option:** with proved source caller/runtime maxima,
   compose the pure shared estimator and keep the current timing/record meaning.
   No new metric/domain/admission rule is needed. Missing maxima are genuine
   remaining proof cells, not a permission to insert a fixed allowance.
2. **Observation added for evidence:** bind actual-run C/P at the stated cuts
   and retain existing prelaunch admission separately. This can validate a
   conditional per-run heap envelope without changing admission. It does not
   complete the universal prelaunch E_max obligation by itself. Additional
   record fields/write paths need an implementation grant; none is made here.
3. **Observation replaces admission input:** moving the final decision to the
   same live process, using an in-process preflight/resume arrangement, or
   treating a newly refused actual run as merely deferred changes the current
   orchestration/record contract. Even unchanged arithmetic is insufficient:
   today a binary refusal of an admitted row is a stop. A prelaunch P*/C* bound
   plus same-run conformance can avoid relying on a prior measured value, but
   proving P*/C* returns the missing source-envelope obligation. ROOT/owner
   disposition is required before selecting a changed admission contract.

Replay must carry binary/source/library/layout/descriptor identities, metric
and cut IDs, actual argv/input/counts hashes, cap/options/repeats/prefix flags,
C/P/Pm and their process identity, future phase terms, and checked arithmetic.
Recompute the **same predicate** in chronological family/size order, including
rho denominators and all earlier records. Keep H and VR bases separate. A new
counts-only run does not reconstruct an old measured process's C0/P0.
Historical records without the chosen cut observations must be marked unavailable
for this option unless an independently justified conservative substitute exists.
Do not rewrite old sealed evidence or pretend a later measurement is its baseline.

## Exact remaining bindings; no new research programme

- Finish I21's existing kernel owner union, source_01 K01-K20, with source_02's
  bound capacity operators and actual final-A1 type/Arc/tree layouts. The cut
  does not cover future kernel allocations. Preserve all33 H fixture shapes,
  VR's193 factored test cases, actual spring/directional/body/load multiplicity,
  the named RF-LARGE scale schedule, repeats and prefixes.
- Apply source_03 A1 owner changes: sequential canonical layouts; fallible
  publication capacity; distinct radius8q; moved publication/radius on accepted
  finish; failed certificate drop; actual changed aggregate strides. Its retained
  source tree at3cf296e36645d97e4c657c8ad1a6322bc4163f16 has **no diff** against
  assigned frozen A1 cb13fcf3ea560c0d07ad9ac02dac78e9d2e06e00. This verifies source
  continuity, not the correctness of the owner proof or later acceptance.
- H future cells: exact source/error grammar, saved-attempt child clone and
  prefix-control construction/lifetimes; stage-line envelope for prefix samples;
  later runtime/caller growth if composing from tH. Existing staged windows do
  not require a generic whole-process H Line allowance solely for repetitions.
- VR future cells: source_01 V05-V12, actual comparison descriptors and finite
  strings, consumer/type/tree/serde ownership, post-cut JSON/emit scratch,
  expected-list file binding, RCM and sparse helper maxima. Prefix observation
  may replace V01-V04 source allocation proof **only for the per-run global
  theorem**, while their hashes and already-live objects stay bound.
- Bind candidate/toolchain/features and quiescent observation placement;
  unavailable/overflow input cannot silently become zero. Shared H-library
  dependency/API proposal remains source_01's, with binary-specific caller facts;
  no allocator redesign or new observer framework is needed for this theorem.
- ROOT must name implementation/write/record scope if proceeding (including
  H main.rs, which the original I21 write list did not name). Independent
  complete-bound review, required suites/mutants, additive historical admission
  replay and settled-A1 W1-T4 remain outstanding. No numeric E_max is supplied.
