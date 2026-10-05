# I36 — ordinary coefficient and readout instantiation

**Finite source/interval proof for backcheck; no implementation or availability
claim.** Native TASK `/root/i36_f2a_preview_truth`, parent `/root`. Receipt
2026-10-02 21:54:06 UTC; new-analysis cutoff 22:14:06; deadline 22:24:06.
Assignment/ROOT selection: `a2458ec6c7550d495a6f874d744c7cfd00042e7b`.
Maintained source: `49034a940f3f8cd3f3da4d4cbc839943b808063d`.

**Result.** Positive ordinary source E/G enclosures can be constructed with the
existing exact accumulator and directed rounding. For interpolation, check the
sign of the exact weighted numerator; a negative E endpoint is allowed when the
selected interpolation is positive. A nonpositive source result refuses this
certificate even when the actual rounded E_hat is positive. No model-domain
restriction, new bridge, arithmetic format, operator or output is introduced.
The coefficient intervals below instantiate I33 uniformly, and B1/B2 can then
form the selected private represented/source hull. The original seventeen-group checker had the exact-K
boundary defect recorded in RV47-C1; the fresh correction_03 checker passes
eighteen groups (see §5).

P = `projects/chirality-piping`; PP = `P/core/product_physics`; FK =
`P/core/solver/frame_kernel`; T3 =
`P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3`;
R = `T3/RESUME_2026-09-30`. Prior I36, RV47, B1/B1C, I33/RV45 and their source
warrants remain unchanged. Origins identify exact revisions and read sections.

## 1. Source values and positivity

All scalars below are decoded **exact normalized binary64 values** of the actual
ordinary invocation. This is not raw-decimal reconstruction. The route/basis view
must retain the accepted resolver's source selection and all its validity checks,
including alpha requirements even where this W1a proof does not numerically use
alpha. The exact profile's E/nu resolver is a different branch
(PP `lib.rs:8992–9000`) and cannot supply ordinary G.

### Base and exact-point selections

Bind E_s and G_s respectively to the actual normalized base values or selected
point values, not a same-named material elsewhere. Check exact E_s>0 and G_s>0,
and the bit identities E_K=E_hat=E_s and G_K=G_hat=G_s through the actual build.
Then `[E_s,E_s]` and `[G_s,G_s]` are positive exact enclosures. A failed identity
is an association failure, not an invitation to substitute another basis.
The source and build paths are PP `lib.rs:7410–7510,9038–9092,9241–9276,
9291–9306,6537–6563`; DEC-077/092 and RV47 preserve that selection rule.

### Interpolated selections

Bind adjacent selected point identities, normalized T_lo<T<T_hi, both E and G
endpoint values, and actual E_hat/G_hat returned by the resolver. Preserve its
existing no-duplicate-temperature, strictly bracketing, no-extrapolation and
no-base-fallback rules (`lib.rs:9096–9208`). For X=E or G define

    h = T_hi - T_lo > 0
    P_X = (T_hi-T) X_lo + (T-T_lo) X_hi
        = T_hi X_lo - T X_lo + T X_hi - T_lo X_hi
    X_s = P_X / h.

The equalities are exact algebra on the captured normalized operands. The actual
rounded expression at `lib.rs:9210–9238` remains a separate captured value.

**Positivity proof:** X_s>0 iff P_X>0, because h>0. Evaluate that exact sign.
The current source explicitly checks positive G endpoints (`:9184–9208`), so
positive weights also prove P_G>0. It checks resolved E (`:9241–9258`), which is
not a proof of exact P_E>0. Neither endpoint E_lo nor E_hi is required to be
positive by this certificate. For example E_lo=-1, E_hi=3 at the midpoint gives
E_s=1 and is admitted by this mathematical positivity test.

A useful **abstract scalar discriminator**, evaluated with exact simulated
separate RN64 operations, is

    T_lo=0, T=7, T_hi=25, E_lo=-7, E_hi=18.
    E_s=0, while RN64(E_lo + RN64(RN64(7/25)*RN64(18-(-7))))=2^-50>0.

This is not a product invocation, product-admission witness or discovered defect.
It proves why positive E_hat alone is insufficient. If P_E<=0 (or P_G<=0), return
`source_material_nonpositive` as a private W1 certificate refusal. Ordinary
behavior, output and standing follow the existing transaction. Do not add a
positive-endpoint gate to the model or reinterpret the target as E_hat.

### Finite outward realization using existing operations

Use I35's proposed fixed `Wide<16>`/1024 context and existing
`wide_sum::ExactWideSum`, `directed::round_toward`, `sub_toward`, `mul_toward`
and `div_toward`; this packet selects no second format or new numeric helper.
I35's format still needs its independent review and implementing qualification.

1. Lift the nine scalars T_lo,T,T_hi,E_lo,E_hi,G_lo,G_hi,E_hat,G_hat exactly.
   Require finite captured values and the actual accepted source association.
2. Form h_- and h_+ with downward/upward subtraction. Check h_->0.
3. For X, form the four products in the expanded P_X expression. Each product
   of two binary64 values has at most 106 significant bits, hence is exact in
   this context. A single directed multiplication in either direction suffices.
   Retain four product endpoints, with the displayed signs.
4. Add those four terms to one fresh ExactWideSum, inspect signum, and refuse if
   nonpositive. Round it downward using existing `round_toward`. Rebuild the
   same four-term sum and round upward (the helper consumes/clears its sum).
   This gives P_-<=P_X<=P_+. No interval evaluation of a cancelling expression
   substitutes for the exact sign.
5. Form

       X_- = div_down(P_-, h_+),  X_+ = div_up(P_+, h_-).

   Require X_->0 and X_-<=X_+. These are proved positive enclosures of X_s.
   Any arithmetic/range/accounting failure is a certificate refusal, not a zero.
6. Include the actual resolver bits without replacing the source target:

       H_X = [min(X_-,X_hat), max(X_+,X_hat)].

   Check finite X_hat>0 and equality X_hat=X_K for the actual mapped K member.
   H_X is positive and contains both exact X_s and actual X_hat. It creates no
   new public material definition. Base/point H_X is the singleton above.

**Finite-span argument.** Every nonzero binary64 product is an integer multiple
of 2^-2148 with magnitude <2^2048. The four original terms therefore span at most
4196 bit positions; each separated-sign magnitude and the net have magnitude
<2^2050, hence at most 4198 relevant positions. ExactWideSum removes trailing
zero bits (`wide_sum.rs:218–250`) and retains 64 carry bits above its 8128-bit
term-span limit. Four terms cannot consume that headroom. The rounded numerator
can reach 2^2050, but has no nonzero bit below 2^-2148; the extra nearest term in
`round_toward` gives a conservative 4199-position residual bound. After clear,
the helper's two-term adjacent step spans at most 1025 positions. No expanding
rational numerator, precision retry or source-dependent iteration is needed.

A positive exact P_X is at least 2^-2148. h lies between 2^-1074 and 2^1025.
These numerator/denominator bounds and any outward 1024-bit neighbors are deep
inside the existing exponent range. Correct outward rounding of their positive
values cannot turn them into zero in this Wide model. This proves the local
positivity construction; it does not prove that all later bridge/recipe sums fit.

## 2. Positive ordinary coefficient enclosure: I33 premise adaptation

Keep B1C's geometry from actual normalized D and **actual effective-wall bit**
t, with existing source admission and exact 0<t<D/2. It supplies positive
[A_-,A_+], [I_-,I_+], [J_-,J_+], [Z_-,Z_+] and c=D/2. Do not redo nominal-wall
subtraction in exact arithmetic or substitute rounded inner diameter/radii.

For each mapped member form, with outward positive multiplication,

    C1 = H_E * [A_-,A_+]      (EA)
    C2 = H_G * [J_-,J_+]      (GJ)
    C3 = H_E * [I_-,I_+]      (EI_z)
    C4 = H_E * [I_-,I_+]      (EI_y).

That is `[mul_down(left_lo,right_lo),mul_up(left_hi,right_hi)]` for each.
All lower endpoints must remain positive. Independently form the **exact**
admitted-bit products

    CK=(E_K A_K, G_K J_K, E_K I_zK, E_K I_yK).

Those products have at most 106 bits. Confirm K properties against the actual
built member; the ordinary circular build supplies identical Iy/Iz but no equality
with source I is assumed. For each j retain

    dC_j = max(absdiff_up(Cj_lo,CKj), absdiff_up(Cj_hi,CKj)).

These intervals contain every source coefficient tuple formed from the exact
ordinary selection, and also tuples with the actual resolved E_hat/G_hat and
source geometry. Ignoring correlations is conservative. CK need not lie in Cj:
dC bounds its difference from every enclosed source coefficient. Adding CK to
those intervals is unnecessary and would be a looser majorant, not new truth.

**Why I33 applies without a new operator proof.** I33 §3 needs positive bounds
on the four constitutive numerators and their differences from CK. Its use of
E/[2(1+nu)] establishes those inputs for the exact profile; it is not used by the
subsequent matrix inequalities. The ordinary construction above establishes the
same premises with independent E/G. For every actual coefficient tuple in this
positive rectangle, I33's same Bbar/Hbar and Dbar patterns majorize the source
operator and Delta. The envelope, inverse inequality and action inequality are
uniform over that rectangle. Correlation is not needed for this sufficient bound.

All other I33 premises remain unchanged: common exact normalized frame/chord,
individual nodal ledger, prescriptions and constraints, spring coefficients,
member/DOF/block maps, matching P=2p verification scaling, twice its associated
upward B, correct data/zero-block facts, and strict alpha<1. Positive coefficients
preserve the shared energy nullspace; no source/operator uniqueness is inferred
from a rounded positive factor. In particular, the source-action error still has
both `sum Ag*T` and `sum Da*M` (and the warranted delta-b term). Varying E/G does
not remove recovery-functional change. A changed frame/load/support law is an
association refusal outside this coefficient instantiation.

Thus the reviewed theorem supplies a source-action/displacement interval
`Q_S=[x-R_K-e_q,x+R_K+e_q]` outward, while the represented K row retains
`Q_K=[x-R_K,x+R_K]`. No source solve is computed. A strict-test failure proves
certificate insufficiency only. The zero-block and exact-static-identity
exceptions remain exactly those in I33, not observed-value shortcuts.

## 3. Represented/source readouts and final hull

Keep the two meanings identified separately until the private final cover:

- **Represented readout:** Q_K with actual positive post-processing A_hat,
  Z_hat, J_hat and c_hat, bound to the same member and final recipe. Check
  A_hat=A_K and J_hat=J_K in this ordinary build. For bending, also cover the
  formula interpretation I_K/c with **c=D/2 exactly**. Form its outward quotient
  interval with two directed divisions and take its hull with [Z_hat,Z_hat].
  Use the associated bending-axis I_K; never swap axes by row kind alone.
- **Source readout:** Q_S with B1C source A/Z/J and exact c. Its coefficient
  source is the ordinary selection above, not E/nu. B1 gives signed N/A, M/Z and
  T*c/J; B2 supplies its independently reviewed nonlinear recipes in their
  admitted source domain. No mixed `f(q_K,source section)` is passed off as the
  source operator's response.

Apply the represented recipe and source recipe separately, then take the hull
of their resulting SI intervals. This avoids unnecessary mixing of a K action
with a source denominator. A Z hull within the represented recipe is already a
conservative cover of the two represented bending interpretations; it selects
neither as exclusive public truth. Direct displacement/action rows need no
second numerical recipe because Q_K is contained in Q_S. InputDerived rows
retain their existing source classification and do not receive fabricated radii.

For the immutable actual final y/unit and normalized n, apply unchanged B1
endpoint-distance predicates to the hull. The maximum distance to a hull equals
the largest constituent endpoint distance, in SI and after the same positive
unit conversion. No public interval, new radius, relaxed bound, changed scale,
class or output value is created. Missing operands or failed covered recipes
refuse; they are not NotCovered reclassifications.

The old support-magnitude relation to published components, exact operation-
ordered maximum midpoint/location and supplied-coefficient enclosure scope
remain separate observable checks. B2 must compose its source/represented
recipes with them. Preserve surviving rows and retired-row exclusions; headlines
remain result_ref aliases, not a new all-case source-maximum guarantee (ROOT
`a2458ec6c7`). No combination maximum is restored.

## 4. Private interface, controls and return

The proposed source view is described concretely in `DEPENDENCIES.md`. It binds
capture/route/case/basis, point ids and normalized operands, actual resolver
outputs and K products, D/effective-wall, geometry/section operands and final
row maps. A borrowed object alone is not proof of its source identity. No caller-
constructed loose interval record, serialized private endpoint, second source
copy or per-member coefficient cache is required.

The preserved `_run_records/exact_controls.py` historically reported 17 passing
groups, but its rounded-CK group is not credited as exact-K evidence. The fresh
`_run_records/correction_03/exact_controls.py` covers base/point values,
independent G, the four-term identity, nonpositive endpoints with positive E,
positive E_hat with exact E=0, zero/negative refusal, positive cancellation,
G convexity, extreme abstract source spans, coefficient/delta inclusion, a
closed scalar specialization of the already reviewed bridge, represented/source
stress hulls, torsion zero/signs, SI/raw hull distance and unchanged-bound refusal.
It checks exact arithmetic implications only; no product code is imported or run.

ROOT/I35/M1 still owe the existing arithmetic review, concrete validating
constructor/maps, actual source/row/state lifetimes, full metering/status/drop
integration, actual layouts/capacities/caller overlap, code review, protected
checks and real availability. The new finite operation and scratch inputs are
in `DEPENDENCIES.md`. No new owner decision is needed merely to implement this
faithful proof. Prior packets are untouched. Stop for RV47/math backcheck.

## 5. RV47-C1 evidence correction (2026-10-02)

RV47 at `dfd7a07711` found that the original checker wrapped all four CK
products in `bits(...)`, rounding the products back to binary64. The theorem in
§2 already requires the exact product of admitted primitives and is unchanged.
The historical 17-group replay was a reproducibility check, not a correction
or valid exact-K product witness. Original raw checker/results/origins remain
byte-unchanged at the frozen `f5ca39a1a2` basis.

ROOT authorized the narrow correction at `8ed7a1d5c5`. The fresh checker separately
lifts each primitive operand, then multiplies the Fractions with no outer
rounding. **All four CK values and all four corresponding endpoint delta bounds
changed** in this control; exact before/after values are printed in
`_run_records/correction_03/exact_controls.json`. The corrected control passes.

The added rounding-sensitive control applies the same corrected constructor to
`(1+2^-52)^2`. With the source singleton equal to RN64 of that product, exact CK
requires `dC=2^-104`; the historical rounded-product substitution gives zero and
fails that requirement. This is an abstract arithmetic discriminator, not a
realized solver or admitted-product failure.

The fresh run passes **18 groups**. The other 16 original groups retain their
control statements byte-for-byte and pass. Original group order is retained,
with the new discriminator added after the repaired group; all pre-existing
JSON fields except group count/list are unchanged. Thirty-two named unaffected
exact-value states were also compared. The original JSON did not serialize CK
or its deltas; this correction explicitly reconstructs and reports them from
the preserved checker, rather than claiming an old raw field changed.

`_run_records/correction_03/CORRECTION.json` records source/repair/command/result
hashes, comparisons, and the exact allowed write set. Informational inventory
is updated; old raw evidence and reviews are not overwritten. Mathematical
source promises, coefficient theorem, operation/storage dependencies, output
predicates and qualification limits are unchanged. Stop for RV47 confirmation.
