# I35 — finite outward arithmetic recommendation

Revisable design for fresh independent math/source review. No maintained code or API is changed. Source basis is main `49034a940f3f8cd3f3da4d4cbc839943b808063d`; ROOT's conditional selection is `d85405edcd`, I33 is `9f1ef2693d`, RV45 is `d05bb825ff`. B1/B1C, corrected C1/C2, and their reviews remain governing premises. Paths below use FK = `projects/chirality-piping/core/solver/frame_kernel/src/structural/retained`.

## 1. Selection: existing arithmetic, fixed private certificate

Use the existing `wide::Wide<16>` value model at **exactly 1024 bits** and the existing `directed::{add_toward,sub_toward,mul_toward,div_toward}` with `ExactWideSum`. Reuse the actual integer arithmetic in `wide/multi.rs`; do not copy it into product_physics, introduce a generic public interval package, or add a precision retry. Certificate precision is independent of the selected public p and its P=2p verification factor. It does not change either.

A nonzero endpoint is `(-1)^sign M 2^(e-1023)`, with `2^1023 <= M < 2^1024`, sixteen little-endian u64 limbs, and **-2^62 <= e <= 2^62**. Zero is canonical +0 in the private certificate. There are no endpoint subnormals, infinities or NaNs. A signed source -0 can retain its provenance bits in its source record; its numerical endpoint is +0. Verified radii retain B1's stricter canonical nonnegative-bit validation. Public signed bits are not rewritten.

Finite binary64 lifts are exact: decode sign, the at-most-53-bit integer and its binary exponent, normalize into sixteen limbs. In particular 2^-1074, and c=D/2=2^-1075 when mathematically relevant, stay nonzero. Only the already validated finite actual source and radius values may be lifted; a sentinel infinity is never an endpoint. Constant integers 1,2,4,1000,10^6,10^9 and the two reviewed B1C pi endpoints are exact at this width. Pi constants have 514 significant bits, denominator 2^512, and are loaded as fixed constants; there is no runtime pi evaluation.

Every integer exponent intermediate is i128, checked before an i64 conversion or shift/index operation. The existing ±2^62 result limit remains. A factor-scale exponent is accepted only through the checked selected verification-factor view and representable `ONE.mul_pow2(s)`. No huge exponent difference is used to allocate or iterate: the exact-sum span check precedes alignment. The exact comparison span remains **8128 bits**, in each existing 8192-bit unsigned magnitude; do not enlarge it.

This format is conservative, not a universal-fit claim. A product, a tiny cancellation residual, or a directed predecessor may leave the exponent range even if some later expression would fit. Such a path refuses. A span above 8128 refuses even if a sticky-bit implementation could have computed a useful enclosure. There is no alternative runtime format or arbitrary-precision fallback.

## 2. Directed arithmetic and its finite proof

All facade scalar operations create a **fresh** `WideContext<16>(1024)` and fresh `ExactWideSum`, collect both on every exit, then discard them. Only endpoint values pass to subsequent operations. Do not reuse a cumulative saturating context/sum across an unbounded source-derived sequence. The facade aggregate is separately checked as specified in SCHEDULE_AND_COUNTS. These fresh locals give a bounded source proof for their existing counters; they do not qualify the existing kernel's cumulative counters.

The retained functions use nearest-even plus an exact side test and at most **one** adjacent 1024-bit step:

| Operation | Existing construction | Exact side relation |
|---|---|---|
| a+b or a-b | add the two exact endpoints to a fresh sum; nearest round; subtract nearest | sign(exact-nearest) determines the required step |
| a*b | existing TwoProduct gives s+r exactly; add both; nearest round; subtract nearest | same side test; both operands fit the fixed p |
| a/b, b>0 | existing nearest division q; exact TwoProduct q*b and subtract a | sign(q*b-a)=sign(q-a/b) |
| sqrt(a), a>=0 | **one new internal helper**, using existing nearest sqrt q; exact TwoProduct q*q and subtract a | for q,a>=0, sign(q*q-a)=sign(q-sqrt(a)) |

The sqrt helper belongs beside existing directed helpers and calls their existing private `step`; it is the sole added wide scalar operation. It does not use f64 sqrt or an optimizer. On a negative radicand it refuses. At exact zero it returns exact +0. For hypot of an interval pair, use the proven abs ranges, lower squares/sum/sqrt downward and upper squares/sum/sqrt upward. Never clamp an unproved negative radicand to zero. This supplies B2 arithmetic only, not B2's source-functional or maximum proof.

For a nonzero p-bit nearest value x, an outward step changes magnitude by `2^(e-p+1)` away from zero; a step toward zero uses that unit except at a power of two, where it uses `2^(e-p)`. The existing exact two-term step sum is therefore the adjacent p-bit value. Because nearest-even lies at one of the two bracketing representable values, the exact side test needs at most one step. Wide has no underflow-to-zero inside its range; a nearest zero is an exact-zero result. Every intermediate failure is returned, never converted into an apparently exact zero.

The integer core has these fixed bounds at L=16:

- Addition's alignment window and exact product are 32 limbs. Multiplication has exactly 16*16 schoolbook inner iterations, with u128 `xy+out+carry <= (2^64-1)^2+2(2^64-1)=2^128-1`.
- Division has 1025 restoring iterations, a 16-limb remainder plus its one-bit carry, and a 32-limb quotient. Invariant remainder < divisor holds after each iteration; the implicit carry implements a 1025-bit trial without losing it. Quotient has at most 1026 significant bits.
- Sqrt has exactly 1026 digit-pair iterations. Its root, remainder and trial use 17 active limbs in three existing 32-limb buffers. The radicand is at most 2051 bits; final root <2^1026, remainder <2*root+1 <2^1027, and trial/intermediate remainder <2^1028. Thus 1088 active bits suffice at every step. The radicand is addressed through input bits, not materialized as an exponent-sized integer.
- Exact-sum operations accept only span <=8128. In one directed operation there are at most five nonzero raw additions over its lifetime, separated by clears; no live value has more than three. The final allowance comparator below has at most five live terms. Relative to any admitted current anchor, a signed magnitude is <5*2^8128 <2^8131. This proves 61 spare bits, including lowered-anchor shifts and carry tails, without a lifetime term-count assumption. A carry tail takes at most 128 iterations. Its existing array safety depends on this restricted call grammar; arbitrary reuse is forbidden.
- A scaled final term `10^9 H` has at most 1054 bits and uses the existing 17-limb scaled buffer. No `add_product_of` or denominator-product expression tree is needed.
- Endpoint exponent arithmetic, sum anchors, high bits and differences are bounded by a small constant multiple of 2^62 plus 4096, far inside i128. Input lengths are compile-time 16/17 or bounded 128; no user slice length reaches the integer constructor through this facade.

These are source/mathematical bounds, not a compiled stack or allocator measurement. Runtime checked containment should replace any reliance on a debug assertion at the new facade boundary. A violating internal invariant is an arithmetic/accounting failure and invalidates the partial certificate.

## 3. Outward source geometry and material construction

B1C's exact symbolic formulas remain the truth. This design rounds each intermediate outward instead of retaining expanding rational numerators. For positive endpoint intervals, multiplication is `[mul_down(l1,l2),mul_up(u1,u2)]`; division is `[div_down(l1,u2),div_up(u1,l2)]`, requiring l2>0. Signed quotients and torsion use the corners below.

For actual normalized D,t, check exact 0<t<D/2 and all existing source admission. Form singleton c=D/2; outward intervals d=D-t and ri=c-t; then P=t*d, Q=c*c+ri*ri, G=P*Q. Form A=pi*P, I=pi*G/4, J=2I, Z=I/c with the reviewed pi endpoint corresponding to each direction. Require positive lower endpoints for every denominator. Because all true factors here are positive, interval monotonicity preserves the B1C enclosure despite intermediate rounding. This does not make actual stored A_hat/I_hat/J_hat/Z_hat the source quantities.

Use source E>0 and -1<nu<1/2 with checked selected-material custody. Form `[dlo,dhi]=2*(1+nu)` outward, then `Gsource=[div_down(E,dhi),div_up(E,dlo)]`. Require dlo>0; E_K must have the source E bits. Construct the four source C intervals and four exact K products as in I33. The K products of two binary64 operands need at most 106 bits, so they are exact in the chosen endpoint format unless invalid inputs were admitted.

For each C interval and its exact CK, compute `dC=max(absdiff_up(Clo,CK),absdiff_up(Chi,CK))`. `absdiff_up(a,b)` compares the exact endpoint values then subtracts smaller from larger upward; it preserves signed cancellation and never rounds a difference to nearest before measuring it.

For chord axes, enclose each exact binary64 coordinate difference by down/up subtraction. Its abs-range lower endpoint is zero if it crosses zero, otherwise min of the endpoint absolute values. Set ell_lo to the maximum of the three lower bounds, require ell_lo>0, and use invell=div_up(1,ell_lo). Since ell_lo<=max|exact chord|<=L, this is a valid majorant. A zero lower bound is a certificate refusal, not a zero-length verdict. It cannot weaken existing geometry admission.

Use Cupper/ell_lo and dC/ell_lo rounded upward to build I33's ten-entry D patterns; the integer factors 2,4 are checked exact exponent shifts. Bbar and Hbar use 1 and invell in their fixed 48/16 patterns. All row/block contractions are sequences of upward nonnegative multiply and add. Every stored row value remains a 1024-bit endpoint regardless of how many members contribute. The accumulated roundoff enlarges the majorant by induction; there is no denominator growth, hidden exact global matrix, or claim that long sums remain exact.

## 4. Strict alpha, tau, and source-action intervals

Borrow s_i and upward body B only from the selected verification build at P=2p, in its own free order and associated data blocks. Reproduce the exact data/zero-block predicate using retained source/ledger/state facts; do not infer it from observed output zeros. Preserve I33/RV45's common frame/load/constraint/operator-pattern premises and the factor of two.

Let beta=2*B by checked exact exponent shift. From the upward eta,v accumulations form:

    alpha_hi = mul_up(beta, eta_hi)
    require alpha_hi < 1
    d_lo = sub_down(1, alpha_hi)
    require d_lo > 0
    tau_hi = div_up(mul_up(beta, v_hi), d_lo)

The strict test is an exact endpoint comparison. Because alpha_true<=alpha_hi<1 and d_lo<=1-alpha_hi, the result bounds beta*v/(1-alpha_true). The extra d_lo>0 check is retained even though it follows mathematically for representable close endpoints when arithmetic succeeds. `alpha_hi==1` refuses even if the unknown true alpha is slightly below one. This is insufficiency of this finite certificate, never proof of singularity.

For free DOFs use M=add_up(abs(x),R_K) from the checked existing finite SI row radius; using R_K directly is conservative and avoids inventing a sharper source radius. T is tau times the exact s_i exponent shift. Prescriptions use the existing exact source law; product W1a's values are +0. Do not silently lift an exact combination prescription by rounded f64 if the scope is later broadened.

The source action error is I33's full `sum Ag*T + sum Da*M + |delta b|`; both coefficient-change and response-change terms are required. No-data blocks receive T=0 only under its exact homogeneous/uniqueness premises. Source interval endpoints are `sub_down(x,add_up(R_K,e_q))` and `add_up(x,add_up(R_K,e_q))`. Exact arithmetic zero remains zero. A missing row/radius/frame/load warrant is a typed refusal before arithmetic, not a zero operand.

## 5. Recipes, raw coordinates, and exact final predicates

For N/A or M/Z evaluate every one of the four signed endpoint quotients in both directions; retain min of downward values and max of upward values. This is at most 8 directed divisions and 6 exact endpoint comparisons. For T*c/J evaluate all eight endpoint triples; each uses down/up product and down/up division of that product interval by its positive endpoint J. This costs at most 16 products, 16 divisions, and 14 min/max comparisons. Corners are streamed; they are not retained as an array or expression tree. Actual producer T*c_hat/J_hat and its actual final y remain unchanged.

For a justified SI interval [l,u], compute Hn=max(absdiff_up(n,l),absdiff_up(n,u)). Convert the *truth interval*, not the public value, to raw units: identity; multiply by 1000 outward for mm; divide by 1000 outward for kN/kN*m; divide by 10^6 outward for MPa. Then HU is the maximum upward distance from actual y to those raw endpoints. Thus the unrepresentable rational 1/1000 is never treated as a singleton approximation.

The actual pinned n is still the existing RN64 normalization of y: one divide by 1000, one multiply by 1000 or 10^6, or identity. Verify its bits and unit spelling against the immutable final row. Keep every final row replacement, qualification, original-max coupling, InputDerived/O9 exclusion, p512 floor, actual A_hat/Z_hat stress-scale operation, class and b bits required by B1/C1. InputDerived gets its separate source proof. Unpublishable required rows cannot be removed to improve a scale. G5a remains an additional check.

All final inequalities use fresh **exact** sums, not rounded allowances. For nonnegative p-bit H and decoded finite binary64 values:

| Test | Exact sum whose sign must be nonnegative |
|---|---|
| absolute | b_SI - Hn |
| sharper exact | m*2^-64 + m*2^-85 + abs(n)*2^-53 + 2^-1074 - Hn, m=max(abs(n),S) |
| sharper binary64 | decode(A_f64) - Hn |
| SI decimal relative | abs(n) - 10^9*Hn |
| raw decimal relative | abs(y) - 10^9*HU |

Each is at most five direct/scaled Wide terms. No rational cross multiplication is needed after endpoint enclosure. A span refusal declines certification; an equality succeeds exactly. The sufficient enclosure can still reject a row whose true error fits, which is not a change to the predicates. At b=0, Hn=0 is required; outward underflow cannot manufacture it.

Reproduce A_f64 with the existing five separate RN64 operations in adaptive.rs:2981–2997, including each intermediate validity check and no FMA/reassociation. A_exact remains the exact four-term expression above. Do not substitute one for the other.

The small-scale public b must also retain its **exact existing bits**. Reuse `absolute_bound` for b0=RU64(2^-64 S) and its analogous fixed one-step scale-back test for r=RU64(2^-53 abs(n)). For 0<S<2^-988 obtain RU64(b0+r+h) with a second narrow new helper: **binary search the positive finite binary64 bit order**, comparing the exact three-term sum against each candidate with a fresh exact sum. Verify the largest finite candidate once; then at most 63 fixed-loop midpoint decisions find the least representable upper value. This is at most 64 comparisons, each four terms. S=0 is exactly b=0; other scales keep b0. An out-of-range sum refuses. No call to the existing open-ended `directed_ratio` or `CertificateMeter::round_up` correction loops is used by this facade. The helper is only for this closed public-bound expression; it is not a generic ratio service. Its comparison invariant proves the same RU64 value as the existing contract, including subnormal ties.

## 6. Refusal and review boundary

Refuse separately for missing/wrong source or row identity; absent or wrong-class radius/bound; unsupported truth/recipe; invalid positive geometry/material/denominator; unexpected pattern/coupling; exponent or 8128-bit span; negative sqrt; count/work admission or accounting failure; alpha>=1; failure of any named final predicate; or failed unchanged G5a/scale/coverage check. Preserve spent work and ordinary transaction routing. No refusal changes a covered row to NotCovered, relaxes a tolerance, starts another solve, or changes actual values/classes/scales/bounds.

New helpers are exactly (1) internal directed sqrt and (2) the closed small-bound RU64 search, plus ownership/meter/recipe orchestration around reused operations. No new transcendental, arbitrary rational, optimizer, general arithmetic API, or numerical engine is recommended. Stop for independent review before code. Complete aggregate memory, actual compiler/target/capacity behavior, source-map implementation, adopted facade work policy, I34/C1 reconciliation, B2/ordinary-route warrants, and actual availability remain separate obligations.
