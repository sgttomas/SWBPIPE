# RV65 — frozen named-support implementation

**FINDINGS: one blocking local-accounting defect (RV65-I1).** The functional
support and dense-observation corrections are implemented correctly at the
reviewed boundary, and the fresh numerical evidence is sound for both fixed
named captures. Do not fan this candidate into NUM until the bitmap initialization
accounting and its discriminating prefix control are repaired and backchecked.
The tests passing does not close that known source defect.

Candidate `8104a4fedd0fd575f20b3d723cc0cdd722d73ba1`, base
`d0daa18717f8243a7232e898c9ef9b4f4d18d9e4`, review brief
`59a85f6165b7187d743745bd4ddaed0100cf0a60`. P=projects/chirality-piping;
PP=P/core/product_physics; FK=P/core/solver/frame_kernel. The complete seven-file
diff was read independently. CHECKS.json binds source, selected inputs, fresh
runtime, author packet and baseline checks; BULK_MANIFEST.json binds new external
logs and per-row exact evidence. No maintained source was edited.

## RV65-I1 — meter the three bitmap fills before entering them

**P2, blocking this component's fan-in.** Location:
PP/src/retained_product.rs:830–835 (`capture_supports`). `built_used.resize`,
`spring_used.resize` and `rigid_owned.resize` initialize newly allocated maps
without an entered initialization/library boundary or charged element writes.
`support_reserve` at754–778 charges the allocation and its capacity record;
its MapWrite comment explicitly names that capacity record and its LibraryBoundary
precedes `try_reserve_exact`. Those charges do not cover the later resize/fill.
There is no selected local accounting warrant bundling the later initialization
into reserve. RustCapacityBytes records storage, not entered fill work.

The actual named case initializes4 built flags,3 spring flags and12 rigid flags
through those unmetered calls. The same omission exists on ordinary prior cases.
It also permits those writes to occur after the remaining map-write capacity is
exhausted: seed MapWrite at MAX-1 in a fresh support seam. The first reserve's
capacity-record charge consumes the last event, then its bitmap is filled; the
next allocation request is entered before the following MapWrite detects loss.
The intended first failure must precede the initialization writes and any later
allocation. This is a local entered-work/first-prefix defect, not a numerical
false PASS, measured RSS excess, or a demand to complete the whole resource profile.

Repair within the existing PP implementation/test paths: add a checked bitmap
initialization path that records its library entry and each actual initialization
write before entry (or an explicitly selected equivalent finite fill rule),
stops before the first unchargeable write, and retains successful reserve capacity
on later failure. Cover all three maps, including empty length without pretending
it performed element writes. Do not charge nonexistent successful writes after
a stopped collector.

Add a discriminator that lets the first reservation succeed but exhausts the
counter immediately before filling. Assert the exact successful prefix,
including **no second allocation request**, retained first capacity, no committed
support/source map and sticky first error. Include a bounded multi-element fill
whose counter expires mid-initialization if the chosen implementation can stop
per element. The current test at retained_product_tests.rs:272–319 seeds every
event at MAX and checks typed error plus stability on a repeat; it stops before
the relevant later fill and never asserts the independently expected successful
prefix. It therefore cannot kill this defect. The current passing suite must
not be presented as proof that every new map write was accounted.

## ROOT question (b): hits range and its precise warrant

`let mut hits = 0` at retained_product.rs:900 defaults to i32; the comparisons
and increment at901–908 impose no wider type. **The u32 length preflight alone
does not prove its range safe for arbitrary malformed slices.** A directly
supplied slice with more than i32::MAX repeats can overflow before the final
`hits != 1` rejection. No enormous input was allocated or run.

For the **actual production contract reviewed here**, there is a stronger owning
source warrant: PP lib.rs:2326 obtains the boundary from `prepare_boundary`,
whose `add_restrained_dof` (linear_supports/src/lib.rs:634–651) tests membership
and returns without pushing a repeated DOF. Blocking boundary findings stop PP
before its case loop. The same borrowed boundary vector is passed into
solve_load_case_observed and its sole production `case_source` hook. Thus each
global DOF occurs at most once and this counter is in{0,1}, independently of
vector length. I do **not** classify an i32 overflow as a reachable production
defect for that proven route.

The limit must remain explicit: this does not qualify arbitrary direct helper
inputs under merely a u32 count rule. A small hardening improvement in the same
repair would use an explicitly bounded usize counter or return immediately on
a second match, with a two-repeat malformed-boundary control. That removes the
implicit caller premise without a giant model. It is non-blocking for the actual
producer route; no source-level blanket malformed-input safety claim follows
from the existing `hits` code. The `count` in `check_support_maps` differs: its
comparison to `group.springs.len()` infers usize and its increments are bounded
by the traversed Vec length.

## Prior conditions and functional review

- **RV65-1 confirmed at functional scope.** Every SupportComponent enters FK's
  force/moment final maxima, is classified with input=false, and uses unchanged
  extent/floor/gate logic. PP G5a classifies it by component and checked source
  body, reads the actual normalized verdict bits, includes it in mechanical
  maxima and preserves the uncoupled-resolution +0 requirement and subsequent
  sanity/lower guards. The isolated empty -0 control passes its interval gate
  and fails G5a; largest-force and largest-moment controls affect both scans.
  A nonzero empty-law value is evaluated against its actual allowance, not an
  invented unconditional rejection. Local accounting remains subject to I1.
- **RV65-2 confirmed.** All PP component bindings, including old rigid/base and
  material cases, use SupportComponent. Independent6g slots detect lost or
  duplicate empty rows. A singleton consumes exactly its native Reaction or
  SpringAction; replacing it with Native misses the attributed slot, while adding
  a direct Native duplicate fails contributor coverage. Native API rows retain
  their independent behavior and cannot alias a support slot. Native magnitudes,
  all other native quantities,21m derivatives and mandatory mode remain covered.
- Builder/boundary support ids, family, node, axis, dimension and k bits are
  checked against authored input. Actual boundary ordinal supplies spring id;
  source order is reconciled after canonicalization. Duplicate/missing/extra
  entries, same-node equal-k axis swaps and unowned rigid DOFs refuse. Duplicated
  group membership is checked before Source::new deduplication. Zero k is retained
  and reaches NonPositiveSpring. Scalar spring axes never become rigid flags.
  Empty/singleton laws use the exact source group and represented/source native
  enclosures; multiple contributors/directional groups remain outside this seam.
- **Dense custody confirmed.** The lib.rs change is exactly the reviewed optional
  three-line terminal call after mode/parity production and before later mutation.
  Actual invocation mode, one completed case hook, independent parity_produced
  and owned optional snapshot are checked. Missing capture never means absence;
  loss of both snapshot and final row still refuses. Actual value bits and dynamic
  basis bytes are owned and compared, alongside all fixed fields/case binding.
  Mode1/2 is mandatory; fallback3 remains refused privately. Sparse parity,
  duplicate completion, foreign case/mode, one-bit/dynamic-text mutations and
  missing/extra rows refuse. Synthetic completed absence is labelled synthetic.
  Parity has distinct at-most-one typed coverage, no mechanical scale/G5a slot,
  class=None and preserved >=0 semantics including captured negative zero.
- New count/cast paths bind s/g/Q to u32, checked products/layouts precede owned
  storage, fallible reserves propagate storage errors, and successful temporary
  capacities are retained in the scoped records. Observer text allocation failure
  cannot create completed absence. Prior stopped-collector and first numerical
  error precedence remains. The known I1 omission prevents a clean local-work
  verdict; storage/capacity reporting is not itself its remedy.

The new controls exercise real discriminator locations: duplicate empty rows use
a distinct id, Native replacement and addition distinguish slot from contributor
coverage, equal-k swaps do not rely on numerical inequality, and direct typed
parity mutations bypass PP identity rejection to exercise FK coverage. The old
missing-mode FK test is preserved by removing the descriptor after a valid PP
bind; PP's new earlier presence refusal is tested separately. Old numerical
expectations and the public both-entry Sensitive test were not weakened.

## Independent full named-row result

The independent checker was written from source/ledger/statics before reading
the author's full oracle. It imports no product or author checker. It decodes
the entire canonical K4SRC/K4LED bytes, checks complete consumption and actual
input/source association, and constructs the complete97-row logical roster
independently. Exact chord d=(1,2,2), L=3, y-reference and symbolic sqrt2 axes are
orthonormal with the source handedness. The three exact ledger moments are
alpha*d; equilibrium gives root rotations M_i/k_i and tip rigid translation
theta_root cross d. Relative tip rotation is3*M_i/(GJ), and local torque T=3alpha.
These satisfy the same positive beam/spring law for source and represented K.
End-i nodal torque is -T, end-j/station torque +T; section torsional stress uses
positive T*c/J. All force/bending/axial/normal-maximum truths are zero in this
fixed pure-torsion problem, irrespective of positive axial/bending coefficients.

`independent_rows.py` uses a separate Machin identity for pi with exact rational
alternating remainder bounds, outward dyadic endpoints, and integer-square-root
norm bounds. It includes exact base E/G, source annulus, represented J and both
represented-Z readings (their bending numerator is zero here). Positive reciprocal
monotonicity covers the full relevant source coefficient interval, not just
sampled corners. It reconstructs actual binary64 normalization/scales/classes,
all raw/SI and absolute predicates, G5a zero/sanity/lower guards, support norms,
extrema midpoint and headline aliases. Upper bounds prove PASS; lower bounds
prove misses; no unresolved interval comparison is counted as a pass.

| Fresh capture | Quantity rows | Candidate PASS predicates | Passing rows | Actual truth-miss rows | Truth-pass/certificate-refusal rows |
|---|---:|---:|---:|---:|---:|
| sparse |97|68|32|60|5|
| dense |97|67|31|61|5|

All194 quantity rows and135 candidate PASS predicates are independently covered,
with **zero false PASS and zero unresolved comparisons**. The first actual
numerical refusal remains displacement magnitude N1, row2 sparse/row3 dense.
G5a and observables independently pass. Fresh debug and optimized per-row truth
results agree; named request/envelope/source/native/facts/verdicts/observation
captures agree with the frozen author records. This establishes sound finite
refusals for these fixed outputs, not first F2a publication.

RV66's accepted torsional-shear obstruction remains restricted to its captured
ordinary and algebraically projected fixed-scale scenarios. It was not used as
a replacement for the all-row check, and this review adds no arbitrary future
scale/projection theorem, source reinterpretation or publication exception.

## Fresh checks, evidence and return

All commands used absolute manifests, --locked --offline, four build jobs/two
test threads, separate RV65 PP/FK targets and1200-second command walls. One Cargo
process ran at a time; all were reaped. Results:

| Fresh check | Result |
|---|---:|
| PP retained_product_tests, debug |18/18|
| PP retained_product_tests, optimized |18/18|
| FK private product_certificate tests |17/17|
| Exact named both-entry/mode formation test |1/1|
| PP exact s11f_site_test target |11/11|
| FK exact s11_site_table target |3/3|
| Independent named-row reference, debug/optimized |both complete;135/135 PASS predicates proved|
| Old base/material semantic comparison, debug and optimized |104 semantic records identical in each build, six cases total|

The last compatibility row means the two original base cases plus four original
selected-material cases, each compared in debug and optimized builds; it does
not claim the original sparse-only specimens were newly executed in two solver
modes. The existing sparse helper remains the sparse wrapper.

Independently verified the seven source paths, all ten packet files including
seal, all73 manifested author bulk files,950 unchanged baseline files, exact
2747-byte fixture extraction and only the approved literal replacement in the
public formation test. Removing the three hook lines restores lib.rs to the base
byte-for-byte. CODE is clean and equals the full candidate pin. Historical failed
runs, first dense prefix and prior seals were preserved.

Runtime ended at07:40:16UTC and its lane was released to ROOT at07:41. A later
process check found only the existing guard PID5387 and its sleep, with no
Cargo/rustc. The guard was untouched. No source/Git/index/API write, descendant,
installation, broad sweep or native UI run occurred. ROOT owns the bounded I1
repair, same-reviewer backcheck and any later integration. No main/publication,
resource-profile, availability, product acceptance or T3 closure follows.
