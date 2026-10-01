# RV28 backcheck — A1 addendum 01

**VERIFIED.** The correction resolves RV28-1 at specification level and makes
the protected relative conjunction mandatory. No residual blocking or
SHOULD-FIX finding in the correction.

The minimum SI kernel design, including private upward-rounded radii, is ready
for a bounded implementation and independent source review. The F2a/D2
**coordinate and binding specification** is now ready for implementation.
Complete per-formula facade enclosures and implementation qualification remain
outstanding as the addendum states; this return does not qualify F2a, certify
an implemented kernel, measure availability, or select a new contract/domain.

Reviewed correction:
`R/design_a1/addendum_01/CORRECTION.md`, SHA256
`30c907589e80a60407acc139a5805d9941b5df1a5b9dd5a2f1dc845159bae80a`,
seal `d1593a77f61ff7bd76e91248cfa224be01f0aaaaa75e2e677ff311ec5f370151`.
Original proposal and original RV28 review seals remain intact. All three
manifests and their entries were independently verified before the check.
Source remains `d01ad98a754698631f927709d08284c272de85e8`.

Role TASK `/root/rv28_a1_design`; parent `/root`; same reviewer on native
followup_task, same instruction/skill basis as original RV28 BASIS.json.
No delegation. Aliases match the original review.

## RV28-1 closure: exact coordinates and both claims

Addendum §4 (lines 157–218) now names raw y in U, exact a_U, intended
SI truth s*, raw truth t*=s*/a_U, rounded reader n=N_U(y), and SI S/class/b.
It preserves adopted D1 item 3's operation: mm division by 1000,
kN/kN·m multiplication by 1000, MPa multiplication by 1000000, identity for
SI units. It does not replace division by a rounded 0.001 multiplication.
All maxima/coupling/floor/class comparisons use the normalized data and
original coupling operands. The encoded bound remains b_SI.

Given |x-s*|<=R_x, independent triangle inequalities give

    |a_U*y-s*| <= |a_U*y-x|+R_x = H_y_SI
    |y-t*| <= H_y_SI/a_U = H_U
    |n-s*| <= |n-a_U*y|+H_y_SI = H_n.

All a_U are positive. The alternative |n-x|+R_x is also valid for the
same source row, by direct triangle inequality; it is no greater than the
two-step radius. Exact cancellation in n-x is legitimate. A bitwise round trip
does not justify dropping either conversion discrepancy from its separate
ledger or prove equivalence of changed source inputs.

For a relative output, §2 and §4 expressly require both the normalized
predicate and 10^9 H_U<=|y|. Therefore accuracy of n is not silently substituted
for the public raw y claim. The exact checks isolate a case where the normalized
public 1e-9 predicate alone holds while the raw predicate fails. That check
isolates those two predicates; it is **not** a claim that all protected
relative gates pass in a reachable source case.

For derived rows, §6 changes neither truth nor the contract to a replay of
rounded production arithmetic: I_SI must enclose intended f(q*,operands).
Distances to that enclosure certify n and y separately. This is a sound
requirement. Formula-specific implementation still must establish that I_SI
encloses every covered intended result, including section operands and
nonlinear extrema; a general interval formula in this document is not that
implementation evidence.

## D2 binding and outward conversion

Addendum §5 (lines 220–264) unambiguously supplies absolute S-I binding with
(n, SI_unit, b_SI). It preserves raw (y,U) as provenance/display data and
preserves the existing relative/InputDerived point path.

If H_n<=b_SI, s* lies in [n-b_SI,n+b_SI]. D2's next_down/next_up after RN64
subtraction/addition gives an outer interval whenever the finite-endpoint check
succeeds. At b_SI=0, H_n=0 proves n=s*, so the initial SI point is exact.
Converting even that point to a nonrepresentable rule-unit value must produce
an outward interval; nearest conversion alone would destroy exactness.
For positive a_V, dividing both endpoints by a_V preserves interval order.
The exact tests include a zero-bound 1 SI unit converted to a unit with
a_V=1000, whose exact endpoint 1/1000 is not binary64.

The optional raw-centered display radius is also sound:

    |y-s*/a_U| <= (|n-a_U*y|+b_SI)/a_U.

Its upward-rounded value is a presentation quantity, not a replacement for
the receipt's radius. An unencodable display radius must be refused/omitted
rather than displayed as a finite certified number. Using outward-converted
endpoints avoids needing that symmetric form.

The cited current point-only binding path was independently verified at
P/core/rules/rule_check_runner/src/lib.rs:85–90, 475–480, 530–539 and 971–1027.
S-I interval wiring is not present there. The correction thus specifies the
future absolute path without claiming the current code already implements it.

## Mandatory relative conjunction

Addendum §2 (lines 38–72) now pins

    H <= min(A_exact,A_f64), and 10^9 H <= |z|,

with A_f64 decoded from the exact existing sequence:
RN(epsilon*M), RN(previous*(1+2^-21)), RN(u*|z|),
RN(previous+h), then RN(sum of those branches).
That agrees with FK/tests/retained_k4/models.rs:494–522.
Each operation must remain unfused and unreassociated.

Independent exact checks found both rounding directions: cases where A_f64
is smaller than A_exact and cases where it is larger. In either case an H
strictly between them would pass a larger-only test and fail the required
conjunction. Exact cross multiplication preserves the decimal 1/10^9
predicate. The new finite/nonnegative checks prevent an infinite allowance
from making the predicate vacuous.

## Private RU64(H): proof, shape and lifetime

For nonnegative H, r_x=RU64(H) is an upper bound. If exact admission established
H<=b with finite binary64 b, least-upper rounding gives r_x<=b.
For a relative row the same argument applies to finite A_f64. Therefore an
accepted H always has a finite binary64 upper radius. This result does not say
the existing conversion implementation cannot encounter a span/exponent/work
refusal; those remain recorded stops.

Zero maps to canonical +0, strictly positive H<h maps to h, and a
mathematical H above f64::MAX has no finite RU64 result. The latter cannot
follow successful admission against the finite allowances, but the converter
and caller must still reject it. Underflow is never an excuse to store zero.
The exact tests cover subnormal values, normal boundaries, f64::MAX and
overflow, and check 0<=H<=RU64(H)<=b at representative finite allowances.

Acceptance stays on exact H. A stored r_x can exceed the exact non-dyadic
decimal allowance while H passes it; that is not a false certificate.
For relative rows each of r_x, A_exact, A_f64 and |x|/10^9 is independently an
upper bound on the same error, so their exact minimum remains an upper bound.
If later code uses that minimum, it must compare or represent the rational
terms exactly or outward; replacing an exact minimum by nearest-rounded
binary64 could understate it. Using r_x alone is conservative as specified.

The boxed slice is private and parallel to the immutable certified publication.
Its prescribed absence marker is a tagged +infinity bit pattern, never a
numerical radius. Eligible rows require finite nonnegative bits; absence is
allowed only for InputDerived/Unpublishable rows. Canonical zero is present,
not absent; negative zero/negative values/NaNs/other infinity are errors.
The independent modeled access checks discriminate those cases. Production
accessors must also check length/index/QuantityId/body/kind/precision/source
binding as specified; the synthetic test does not prove that wiring exists.

The source has the needed report alive at comparison/finalization
(adaptive.rs:3172–3245), and currently drops it after building RetainedSolve
(:3295–3411). Computing the slice while live, then moving it with the exact
provisional publication, resolves the old impossible-borrow suggestion.
RetainedSolve's existing Clone (2699–2710) will clone the new Box payload.
Combinations need their own fresh certificate, not another solve's radius
indices. Partial drafts must be dropped before escalation, including
conversion/validation stops.

No public radius field or new reader-attested private H is introduced.
Replay must regenerate the gate/private state; serialization must not leak
the sentinel or silently reinterpret it as an allowed bound.

## Implementation obligation RV28-N1: count directed-conversion work and storage explicitly

This is a source-review requirement for the upcoming implementation, **not a
new mathematical defect or required author correction**: §3 already requires
all work to be charged.

FK adaptive.rs:458–517's existing directed_ratio returns only f64. It clones
num and den (464–468), rounds those clones, and calls product_reaches
(437–451), which creates another ExactWideSum. Those local accumulators'
SumWork is not returned by that API. Observing the outer H accumulator and
WideContext alone does not demonstrate accounting for all new RU conversion
work. Cloned accumulators also carry their existing counters: summing their
whole counters could bill inherited work twice.

The new call path must expose/aggregate actual newly performed work, including
local accumulator deltas and every failed/stopped conversion, and perform its
budget checks at the defined stage boundaries. A bounded accounting-aware
helper or equivalently proved conversion is acceptable. Do not broaden this
assignment into an unrelated helper rewrite. The implementation review must
verify the chosen path; no correct accounting is inferred from helper reuse.

K6c must include simultaneous H/denominator/directed-conversion clones and
comparison scratch, not just the final Box. The addendum correctly calls 8Q
**logical payload**, not capacity/allocator/RSS. The Box header is two machine
words on the named 64-bit target, subject to actual layout/padding verification.
A Vec-to-Box construction may have transient allocation/capacity implications.
Count the actual construction, live report/states/cache/publication overlap,
clone multiplicity, combinations and rejected draft disposal. No O(1) extra
memory, final E_max, timing, throughput, or availability claim has been proved.

## Remaining pre-F2a work and evidence boundary

No further correction is needed to close RV28-1. Remaining implementation
obligations are explicit and unchanged in authority:

- Implement/review the kernel gate, paired publication/radii, stop reasons,
  accounting and corrected identity; exercise all-row, range, zero, floor,
  threshold, mismatch and required mutant cases.
- Settle per-formula derived enclosures and actual bounded arithmetic in F2a;
  implement raw and normalized certificates, G5a preflight, protected point
  behavior and named refusals.
- Implement S-I's SI-centered binding and outward rule-unit conversion,
  corrected-policy reader recognition and Rust/Python/TS parity.
- Reconcile K6c to actual sizes/lifetimes/work, then perform admission and
  required native/source-unit/ordinary-availability checks.

These are not closed by this review. Root's previously accepted conservative
refusal direction covers faithful implementation; changing scale/bound
semantics, domain, or protected availability remains unselected.

## Actual execution

The existing VENV ran fresh backcheck.py once, using only the standard library:
**72 checks passed**, exit 0, empty stderr. CHECK.stdout.json, EXACT_CHECKS.json,
BASIS.json and COMMANDS.json retain results, input identities and the actual
read-only Git commands. No designer checks were imported or executed.
No Rust/solver/native job, source edit, Git/index mutation, delegation or
host-tool change occurred.

One source lookup initially used the mistaken core/solver/rule_check_runner
path and returned “No such file”; rg located core/rules/rule_check_runner.
The correct source was then read and independently matched to the pinned Git
object. No conclusion depends on the failed lookup.

Only this additive backcheck subtree was written. Original instruction
origins/hashes remain in the sealed parent BASIS.json; no extra skill, workflow
or role was activated. SHA256SUMS seals this backcheck's files and excludes
itself. Later implementation backchecks must preserve these bytes.

