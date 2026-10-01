# A1 addendum 01 — unit coordinates, relative predicates and certificate lifetime

Status: correction proposed for RV28 backcheck; no implementation performed.
This supplements the sealed DESIGN_PROPOSAL.md (SHA256
926dea73178b0ecde07203fdf7fc85e5ce752b1a5f6ead7cf31a30673249b178), whose bytes and
eight-entry seal remain unchanged. Reviewed finding RV28-1 is in
R/design_review_RV28/REVIEW.md, SHA256
75e8a893403fdf51ad914357ed7d32652d6a2084094ef553d9000f4ad826f3bd.
Source remains d01ad98a754698631f927709d08284c272de85e8.

ROOT has disposed the bare-b conservative admission recommendation as within
the owner's already accepted bounded correction direction. Original §1's
suggestion that bare b itself requires a new owner choice is superseded.
Qualified intervals, outward **public** radii/scales, domain changes and
reversals of protected availability criteria remain unselected alternatives.
This addendum neither invents another approval checkpoint nor selects those
alternatives. Independent backcheck still precedes implementing reliance.

## 1. Superseding rules in brief

1. The SI kernel gate uses the reviewed `H=|x−v|+E`, its complete R7 error
   formula, existing gates and named rejection schedule. Bare-b means H≤b.
   In those formulas P is `report.precision=2p`; do not double report.precision.
2. Relative admission **requires**, rather than optionally uses,
   `H≤min(A_exact,A_f64)` and `10^9 H≤|x|`, with §2's exact definitions.
3. While the report is live, store a finite upward binary64 private radius
   `r_x=RU64(H)` for every eligible SI row. Store no report and no ExactWideSum
   per row. Move the publication and its parallel private radii into the selected
   RetainedSolve. §3 fixes the data shape and lifetimes for K6c.
4. Product publication/classification/receipt checks use the **SI-normalized
   value n** obtained from the actual raw row y in unit U by adopted D1 operations.
   The receipt's scale and bound bits are in that kind's SI unit.
5. S-I's absolute binding operand is explicitly **(n, SI_unit, b_SI)**. It forms
   its interval in SI, then converts endpoints outward to the rule's declared
   unit. It never adds b_SI to raw y in U. Relative/InputDerived point binding
   keeps the current raw `(y,U)` route. §4–6 specify the actual coordinates.

## 2. Executable relative predicates

For the kernel use x and S_x, both in the row kind's SI unit. For F2a's normalized
publication use n and S_n, again in SI. Write z for that comparison coordinate,
S for its final, reader-rederived scale, ε=2^-64, u=2^-53 and h=2^-1074.
All finite binary64 operands below are lifted to their exact real values.

    A_exact = ε max(|z|,S)(1+2^-21) + u|z| + h.

The **current protected comparator's operation order**, FK/tests/retained_k4/
models.rs:494–522, defines A_f64 as the exact value of these binary64 steps:

    a0 = RN64(ε · max(|z|,S))
    a1 = RN64(a0 · (1+2^-21))
    u0 = RN64(u · |z|)
    u1 = RN64(u0 + h)
    a2 = RN64(a1 + u1)
    A_f64 = exact_decode(a2).

No reassociation or fused multiply-add. Require each scale, intermediate and
allowance finite/nonnegative before use. Then admission requires **both**

    H ≤ min(A_exact,A_f64),       10^9 H ≤ |z|.

Compute the comparisons by exact sums/scalings/integer cross multiplication;
do not first round A_exact or H into binary64. This is preservation of the two
existing protected predicates and public decimal 1e-9 claim, not a new tolerance.
The radius r_x retained under §3 may be rounded slightly above H; acceptance
remains on exact H. A stored upper radius is not itself substituted for H in
these admission comparisons.

The normalized facade comparison is not a substitute for the raw row's public
relative claim: if an emitted y is described as relatively verified in U, also
require `10^9 H_U≤|y|` in U, as §4 derives. This additional conversion check can
refuse representation choices without changing SI G5b/c classification rules.

## 3. Concrete kernel data and lifetime strategy

The source confirms that RetainedSolve (adaptive.rs:2701–2710) retains no
VerificationReport, and finish_selected drops it after summary extraction
(:3295–3411). The original suggestion of borrowing that report later is
withdrawn. The selected **proposed implementation shape** is:

    RetainedSolve {
        // existing fields, including Publication
        publication_radius_bits: Box<[u64]>,
    }

This field is private producer state, not a public receipt/row field, persisted
artifact, registered new numerical radius contract or alternate class/bound.
Its length equals Publication.rows.len(); index, QuantityId, body, kind, selected
precision and source identity are bound when the pair is built and moved.
Every present radius is in the SI unit of that kernel row (m, rad, N or N·m).

- For every value-bearing, non-input-derived row, compute exact H **while its
  matching candidate, verification and report remain live**. Run the reviewed
  exact gate first. Form `r_x=RU64(H)` using the existing exact directed-ratio
  machinery on H/1, or an equivalently proved directed conversion. Store its
  nonnegative finite binary64 bits. H=0 gives canonical +0; 0<H<h gives h.
- For InputDerived and Unpublishable rows store a private reserved sentinel
  `0x7ff0000000000000`. This is a tagged **absence**, never an infinite numerical
  radius and never permission to publish. Accessors return None for these row
  classes and reject a sentinel/present/class mismatch. All other nonfinite,
  negative or negative-zero patterns are internal certificate errors. No caller
  may treat absent as zero or use the sentinel as a comparison allowance.
- Use one exactly sized boxed u64 slice, initialized to the sentinel. Fill only
  the current candidate's eligible rows. A typed private accessor validates row
  identity/class and exposes `(row identity, SI unit, finite radius)` to producer
  integration code; it does not borrow the discarded report. The eventual
  facade may need a narrow internal Rust integration accessor across a crate
  boundary; that visibility change supplies data, not new serialized semantics.
- The provisional Publication and radius slice form one owned draft. A
  successful gate moves both into RetainedSolve; finalization emits those same
  row bits. A failure/stop drops the draft and partial radii before reuse or
  escalation. Do not clone a second Publication merely to retain the radii.
  Existing RetainedSolve clones clone the boxed slice too; account for that.
- No report retention, later report recomputation, exact per-row H storage,
  per-row Wide storage, new factor/cache state or solver invocation is proposed.
  A combination performs its already-required fresh solve/certificate and owns
  its own radius slice; it cannot inherit another case's radii by index.

**Why binary64 r_x suffices.** Exact H bounds |x−q*|, and RU64(H)≥H. If the
absolute gate passed H≤b with finite binary64 b, monotonic directed rounding
also gives r_x≤b: storing r_x causes no extra absolute kernel refusal. For a
relative row H≤A_f64 with A_f64 finite binary64, r_x≤A_f64 similarly. Thus every
accepted kernel H has a finite representable upper radius; conversion failure
is a named certificate/arithmetic stop, never fallback to zero.

For normal H, 0≤r_x−H is less than one upward binary64 spacing; for subnormal
H it is less than h. It is exactly zero when H is representable. r_x can exceed
an exact non-binary64 relative threshold (A_exact or |x|/10^9) even though H
passed it. This does not invalidate acceptance: both the exact admission
predicate and the independently stored upper radius remain true. Downstream
may safely tighten its available radius by taking the exact minimum of r_x and
the already certified public/protected allowances:

    R_x = r_x                                      (absolute row; r_x≤b)
    R_x = min(r_x,A_exact,A_f64,|x|/10^9)           (relative row).

No receipt field is changed or relaxed by this private use of simultaneous
true upper bounds. A facade that uses r_x alone stays conservative and may
refuse more; the minimum avoids a gratuitous relative-boundary loss. Actual
conversion/derived rows may still refuse because their error budgets do not
fit the existing final b. Such refusals are measured, not excused by new b bits.

**K6c shape now fixed for this proposed kernel slice.** One additional boxed
slice header in RetainedSolve (two pointer-size words, 16 logical bytes on the
64-bit target, subject to actual struct padding) and 8Q logical payload bytes
per retained selected solve, Q=all publication rows. Sentinel entries cost the
same 8 bytes. No per-row Option/enum allocation and no BTreeMap are needed.
The draft radius allocation can overlap provisional publication/outcome
vectors, report, candidate, verification, states, cache and exact H/conversion
temporaries. Count actual allocator capacity/rounding and clone multiplicity;
do not call 8Q an allocator/RSS bound. Charge every exact H comparison and
RU conversion to the candidate publication/stop stage, case and invocation,
including failed/stopped paths. Existing fixed span and work budgets remain.
K6c must inspect the implementing type sizes and exact live ranges before its
final bound; no unresolved report-lifetime choice remains in this design.

## 4. Coordinate contract preserving SI G5b/G5c

For a kernel quantity let s* be intended exact source truth in its SI unit,
x the actual SI kernel binary64 publication, and R_x the available certified
radius from §3: `|x−s*|≤R_x`. For an emitted product row, distinguish:

| Symbol | Meaning and unit |
|---|---|
| U | Raw row's declared published unit from the closed D1 table |
| y | Actual emitted binary64 value in U; what the existing caller puts in SolverResultBinding.value |
| a_U | Exact positive linear map from U to the kind's SI unit; s=a_U t |
| t*=s*/a_U | Intended truth in U, for this same source quantity |
| n=N_U(y) | Reader's pinned binary64 normalization of actual y into SI |
| S_n,t_n,b_SI | SI body/derived scale, SI threshold and SI encoded A1 bound rederived from n rows |
| q_D2 | For absolute S-I binding, n in SI; for unchanged relative/InputDerived point binding, y in U before existing B2/B3 normalization |

The exact maps and reader operations are specifically:

| U | a_U | Pinned N_U(y), one RN64 operation |
|---|---:|---|
| m, rad, N, N·m, Pa | 1 | y×1 |
| mm | 1/1000 exactly | y÷1000 |
| kN, kN·m | 1000 exactly | y×1000 |
| MPa | 1000000 exactly | y×1000000 |

The mm operation must not be replaced by multiplication by the rounded
binary64 value of 0.001. The exact unit map is rational 1/1000 even though its
binary64 encoding is not exact. Other published units stay NotCovered under the
adopted table; this correction adds no unit or arbitrary conversion engine.

Readers and producer preflight apply adopted D1 item 3 to **all actual rows**,
then apply input-derived/O9 membership, maxima, original-operand coupling,
floor, stress-scale and class/bound rules on those normalized values. G5b/c
still compare exact bit patterns, including A1 b_row using |n| and S_n. In
particular, a mm y must never be compared directly with an m threshold. Neither
the kernel's S_x nor a rescaled copy of its class is substituted for S_n.

For an identity-derived kernel row, conservatively construct:

    e_pub_SI  = |a_U y − x|
    H_y_SI    = e_pub_SI + R_x             ≥ |a_U y − s*|
    H_U       = H_y_SI / a_U              ≥ |y − t*|
    e_norm_SI = |n − a_U y|
    H_n       = e_norm_SI + H_y_SI        ≥ |n − s*|.

These use exact decoded x/y/n and exact a_U. Thus H_n explicitly includes the
normalization discrepancy, even when raw publication used another floating
path or a rounded unit factor. The alternative tighter expression
`|n−x|+R_x` is also valid **for the same row/source identity**, as it combines
the signed publication and normalization differences before taking absolute
value; it may be used only by exact arithmetic, never by subtracting rounded
error estimates. Neither form assumes e_norm=0 because a round trip reproduced
x. The implementation may use the minimum of the two independently valid
bounds, but the finite test must report e_pub and e_norm separately.

Run the final certificate with H_n in SI against the **final normalized** class:
H_n≤b_SI for AbsoluteVerified, and §2's exact relative conjunction with z=n
for RelativeVerified. For a relatively verified raw y, also require
10^9 H_U≤|y|. Emit only after these and the identical G5a preflight pass.
Nonfinite/unsupported normalization, absent certificates or unsatisfied
predicates produce a named facade certificate/receipt refusal; no altered
scale/bound, input cutoff or silent NotCovered demotion repairs the failure.

## 5. Actual D2 binding coordinate and units

Current code has **no S-I interval field/path** at the source pin:
rule_check_runner/src/lib.rs:85–91 has SolverResultBinding `{value,unit}`;
:475–482 takes the caller's raw y/U; :530–539 invokes B2/B3
normalize_value_to_declared_unit; :971 onward returns unchanged value when
entered and declared units match. That is today's **point** binding path.
The proposed S-I path in D2 DESIGN §4.11 is not evidence that a radius is already
converted or attached in executable source.

The bounded clarification for the future absolute S-I path is:

1. Validate the receipt and read b_SI bits; take **q_D2=n, unit=the kind's SI
   unit**, using exactly the n already rederived for G5b/c. This makes
   D2's `[q_D2−b,q_D2+b]` dimensionally and numerically coherent without changing
   the encoded bound or SI G5b/c rule. Keep raw y/U as result provenance/display
   data, not the interval center used with b_SI.
2. If b_SI>0, form outward SI endpoints by D2's existing prescribed steps:
   `lo_SI=next_down(RN64(n−b_SI))`,
   `hi_SI=next_up(RN64(n+b_SI))`. If b_SI=0, the gate proved n=s*, so the SI
   operand is the exact point n with no initial widening. Overflow/nonfinite
   endpoints refuse binding conservatively.
3. Convert that SI interval/point to the rule declaration's unit V using the
   existing, accepted unit definitions and outward endpoint arithmetic. For the
   closed linear units above the exact target is `[lo_SI/a_V,hi_SI/a_V]`.
   A nonrepresentable converted point may become a narrow interval; it must not
   be asserted exact in V merely because b_SI was zero. The rule-interval path
   still owns outward conversion; this adds no new unit definition.
4. Feed those endpoints in V into S-I's existing three-valued plan. Do not feed
   `(n,U)` through the raw y/U normalizer, and do not transform b separately
   then center it on raw y without compensating for n−a_Uy. Preserve current
   relative/InputDerived point path `(y,U)` and its standing; no interval mode
   silently runs for those classes.
5. UI/diagnostic language that prints the encoded number must say `±b_SI SI_unit`
   around n. To present the certified interval in raw U, convert its SI endpoints
   outward to U. If a symmetric radius around raw y is desired for presentation,
   a sufficient radius is `RU64((b_SI+|n−a_Uy|)/a_U)` in U. This is a derived
   display quantity, **not** new receipt-bound bits; its derivation and unit
   must be retained. Showing raw y with the numeric b_SI labelled U is forbidden.

This resolves an ambiguity in an unimplemented absolute-binding path while
preserving adopted SI normalization and encoded bounds. The facade/reader
backcheck must confirm that the selected D2 slice interprets q this way. No
change to existing point-binding behavior, scale/class algorithm or public
radius encoding is selected by this clarification.

## 6. Derived rows, source-unit re-entry and proof boundary

For a covered derived SI truth s*=f(q*_1,...,q*_m,operands), use the private
kernel pairs `(x_i,R_i)` to form a conservative interval enclosure I_SI of that
**specified current formula** with all source/section operands interpreted by
their accepted exact-bit semantics or independently bounded uncertainty.
If y in U is the actual emitted result and n=N_U(y), certify both coordinates:

    H_n ≥ sup_{s∈I_SI}|n−s|,
    H_U ≥ sup_{s∈I_SI}|y−s/a_U|.

These formulas automatically include production recovery rounding and reader
normalization, provided I_SI encloses intended truth rather than merely reruns
the rounded producer. Equivalent Lipschitz/absolute-error propagation from
original §7 is acceptable when all operations/operands are bounded. Use exact
or directed outward comparisons, with named refusal for unsupported formulas,
span/work/range limits, or an interval division containing zero. Preserve the
existing closed stress factors, source pressure exclusions and G5b/c equations;
they define the **claim being tested**, not an assumed error bound for f.

G5a shape, zero, sanity, lower and summary checks still execute on the actual
n rows in SI, with the identical order/constants used by readers. RV28's
synthetic honest G5a failure confirms that its old closeness proof is not an
automatic-pass theorem. A G5a refusal remains conservative; a passing check
does not replace the final row certificate.

Source-unit re-entry is a different test from output conversion: converting
inputs and re-parsing can change the exact binary64 PrimitiveSource and hence
s*. Derive the oracle from the newly admitted primitive bits; do not assume
t*=old_s*/a_U for a changed source. For unchanged source output conversions the
equations in §4 do apply. A m→mm→m or N→kN→N bitwise round trip is useful
evidence but alone proves neither source equivalence nor zero conversion error.

Kernel implementation readiness: the minimum SI gate and private radius shape
are concrete, subject to this backcheck and source implementation review. No
facade callback, general interval engine, unit-engine expansion or report
recomputation is needed in that kernel slice. Remaining facade implementation:
actual row normalization/derived enclosures, G5a preflight, final certificates,
new failure mapping, corrected-policy reader recognition, S-I SI binding and
three-language parity. Those remain bounded pre-F2a obligations, not silently
closed by this additive design or by retaining private radius bits.

## 7. Finite discriminators and evidence

The addendum's standard-library checks establish design arithmetic only:

- RU64(H)≥H, exact zero, below-h H→h, representative normal upward rounding,
  and H≤representable b⇒RU64(H)≤b;
- exact normalized mm/kN maps and nonzero `|n−a_Uy|`, including round trips;
- wrong raw-mm/normalized-m classification and wrong-unit interval counterexamples;
- every certificate chain compared with exact truth in raw and normalized units;
- different A_exact/A_f64 at a rounding-sensitive value, where accepting only
  the larger one violates the explicit conjunction;
- point n with b_SI=0 may require outward endpoint conversion in V.

Implementation tests must additionally cover sentinel/class/identity mismatch,
partial-draft drop, moves/clones and allocation/work accounting; the existing
all-row and policy/gate obligations remain. Source-unit re-entry, native routes,
real derived stress certificates, and compiler/solver tests are unrun. No
private-radius availability or performance claim is inferred from arithmetic.

No change to original sealed files, maintained source, Rust/toolchain/host
configuration, Git/index or public receipt was made. The addendum seal and
provenance inventory accompany this correction for the same reviewer's backcheck.
