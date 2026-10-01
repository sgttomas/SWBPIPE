# Source14 — finite binary64 caller premise

TASK I21; actual start2026-10-01 16:49:20 UTC; deadline17:09:20 UTC. Source-only, frozen40129. The exact allocation derivation and exact_11 review remain sealed. This packet does not execute a solver/model, generate values, or treat prior passing runs as proof.

Aliases: VR=projects/chirality-piping/validation/benchmarks/numerical_robustness; L=VR/src/lane.rs; F=VR/src/floor.rs; C=VR/src/compare.rs; X=VR/src/exact.rs; V=VR/examples/vk_scale.rs; K=projects/chirality-piping/core/solver/frame_kernel/src/structural/retained.

## Actual entry sites and branch order

| Exact site | Actual producer | Current premise |
|---|---|---|
| C40 scalar | L::value_of direct displacement/reaction/spring/axial/torque values, or L162-169 twist/extension quotient | Direct production Normal/Subnormal values and mapped Underflow ±0 are finite. Quotients require a separate proof; tags describe the numerator, not the division. |
| X480 magnitude | L155-159 two separately published bending components | Both production tags are finite; exact magnitude_predicate converts them before multiplying as Exact values. No f64 square/hypot is formed in this caller. Overflow/missing tags produce Unavailable instead. |
| X510 floor | F::row_scale selects coupled kind or per-member mo/kt or fo/ka | Reference maxima are bounded decimal inputs, but coordinate norms, coupling and quotient results have no final finite guard. This is the primary derived floor premise. |
| Value controls L56-65 | Decimal E/S/O parsing | No from_f64 call; already validated decimal descriptors do not require this premise. |

Actual twist/extension observation is a division of the published j-end torque/axial action. It does not subtract two displacement/rotation values. The relevant f64 differences are coordinate xj-xi in member_length and hi-lo in body_extent. Finite coordinates do not alone prove those differences, squared norms or later divisions finite.

L266-280 only compares selected outcomes when case.refuse is false; refused/source-refused/unresolved lane alternatives do not run this row loop. But the global V458-460 full-maxima floor pass is independent of that outcome, and C53-57 calls holds even when covered=false. Neither NotCovered nor a recorded kernel refusal is a universal guard for every later global from_f64 call. Full fixed-domain coverage remains213 cases (193 factored obligations,8 declared refusals,12 external), with no fixture removed.

X290 asserts x.is_finite before bits extraction or Nat::from_u64. A nonfinite value therefore creates no Nat for that operand, but the already live caller/Exact owners persist into an unbounded assertion/runtime branch. The successful-path allocation recurrences cannot be relabeled as a bound on that branch. No panic-library investigation is authorized here.

## Source checks and exact remaining inequalities

The following final source proposal distinguishes closed raw/tag, decimal-maxima and fixed-input floor inequalities from the remaining selected-output arithmetic bound. No numerical success assumption or changed comparison criterion is introduced.

## Closed production tag and decimal premises

K/wide/multi.rs:666-680 defines the production conversion tags. to_binary64:714-755 returns Normal ±0, normal bit encodings with leading exponent<=1023, or subnormal bit encodings; larger leading exponent returns Overflow, with no f64 payload. Underflow similarly has no payload. K/recover.rs:479-489 uses exactly that conversion (or literal Normal(0.0)); adaptive.rs:1035-1049 also maps failed/nonrepresentable conversions to Overflow. These are the actual production-tag contracts, not a claim that arbitrary code cannot manually construct the public enum with invalid data.

L81-88 value_of passes Normal/Subnormal payloads, maps Underflow to signed zero and rejects Overflow/missing. Thus direct scalar observations are finite, including all displacement/reaction/spring/axial/torque target variants. BendingEnd/Station reads two such components and passes them separately; X480 converts before Exact multiplication/addition, so finite components cannot overflow a floating square at this caller. StructuralZero/Unavailable performs no from_f64.

The accepted exact descriptor/review validates all213 cases,27752 rows and5085 value-control entries, with reference/selected-scale magnitude strictly below10^298. Consequently F135's decimal reference parse has finite output (possibly signed zero through underflow); max(abs(...)) of finite nonnegative values remains finite. MODEL_SCALARS.json additionally checks all12 committed s_full maps: strict magnitude<10^3, valid decimal grammar. Their V320 parse therefore also supplies finite maxima. No control decimal is routed through from_f64.

## Fixed-input interval proof (proposed, independently reviewable)

FIXED_INEQUALITIES_ALL.json is the final descriptor set. It preserves all213 case identities, every one of68977 member identities,373 groups of identical member bounds, and all138 explicitly open observation-row identities. Ten committed family-file hashes match metric_design_03_exact. The12 external raw files were read only at the sealed I23 paths and freshly matched both expected/actual raw model hashes. No source encoding, model generation, float calculation, floor/Exact algorithm or solver was run. The script uses integer decoding of the stored binary64 bits and integer decimal lexemes solely to prove power-of-two inequalities.

The initial embedded-only FIXED_INEQUALITIES.json is explicitly superseded. It could not close THIN-B using its coarse chord interval. The final proof retains that case and tightens its actual one-axis power-of-two chord; it does not assume that the case ends unresolved.

Let d be floor(log2(max absolute exact coordinate difference)) for a member chord or body span, computed as an integer from the stored dyadic bits. For the actual inputs, body d lies in[-239,244], member d in[-240,241]. All input coordinates are finite, and each model has nodes. For a nonzero chord/span:
- Every rounded difference has absolute value<=2^(d+1); at least one is>=2^d. These endpoints are exactly representable and far from underflow/overflow.
- Each square is<=2^(2d+2), while at least one square is>=2^(2d). With d>=-511 and2d+4<=1023, the bounding operations stay in normal range. Adding three nonnegative squares and taking sqrt gives L in[2^d,2^(d+2)]. Zero components/squares do not invalidate the lower bound from the maximum component.
- If exactly one difference is nonzero and its exact absolute value is a power of two, subtraction, its square, additions with zero and sqrt produce L=2^d exactly. The final descriptors record this tightening.
- A zero body extent takes F119's explicit return of the original finite maxima. No zero member chord occurs in the final fixed-input member proof.

These are mathematical bounds for the ordinary binary64 operation order in F26-29/F105-112, under the same normal-production target arithmetic basis. No observed geometry output is substituted. Unused design-coefficient intermediates may overflow; the required bound concerns values that actually reach from_f64.

For positive normal a,b with binary exponents ea,eb (the actual E/G/A/J bitfields), F33-37's successful design_coefficient branch already checks both p and q normal. In the fallback:
- s=-(ea+eb); scale2 uses factors whose exponents are clamped to[-1000,1000], within pow2's explicit assertion.
- Scaled b has exponent -ea. For every fixed member this and all monotone intermediate scales stay normal; a*scaled_b is between1 and4, inclusive after rounding.
- With the chord interval above, q is bounded by[2^(-d-2),2^(2-d)], or[2^(-d),2^(2-d)] for the exact-axis chord.
- Unscaling gives a coefficient lower exponent ell=ea+eb-d-2 (ell=ea+eb-d for exact-axis) and upper exponent h=ea+eb-d+2. Both bounds are recorded for kt and ka.
- Every actual member satisfies normal-intermediate checks, ell>=-1022 and h<=1023. Actual final minimum ell is-1021 and maximum h is991. Therefore every fixed kt/ka is strictly positive and finite, regardless of whether the fast or prescaled branch is taken. F's design product may itself overflow/underflow and select fallback; that does not create a from_f64 assertion.

For a reference kind with strict exact-decimal magnitude exponent e, the rounded maximum is conservatively bounded by a representable power2^s. Subnormal rounding is covered by flooring that upper exponent at-1074; all-zero inputs have a separate zero marker. If nonzero body extent is in[2^d,2^(d+2)], F117-128's four coupled upper exponents are:
  max(s_tr,d+2+s_ro),
  max(s_ro,s_tr-d),
  max(s_fo,s_mo-d),
  max(s_mo,d+2+s_fo).
Terms with zero numerators are omitted as zero. All actual row-coupled exponents are<=993, so each product/division/max result is finite. Dividing those moment/force bounds by kt/ka's positive lower2^ell gives member-floor upper exponents. All are<=996, hence finite. The independent s_full computation passes the same test for all12 external cases (full coupled upper<=24).

Result: the final fixed-input inequalities propose closure of F::row_scale -> X510 for all213 cases and all12 s_full paths, including declared-refusal, disconnected, zero, cancellation, directional-spring and extreme-range data. No successful kernel outcome is assumed for this floor proof. The scalar inequalities preserve the specified operation order and use no sampled output.

## Remaining observation premise:138 rows in18 cases

For an observed numerator v supplied by the finite production tag and k>0:
- if k>=1, RN(v/k) is finite for every finite v;
- if v is signed zero, the quotient is finite for every proved-positive k;
- otherwise the finite tag alone supplies no upper small enough when k<1.

Every actual twist/extension row in24 RF-LARGE cases passes the k>=1 sufficient test. Together with the direct/magnitude/floor proofs, this closes the from_f64-finiteness premise for the actual vk_scale RF-LARGE24 comparison routes on these hash-bound inputs, as a source proposal subject to review. It is not a whole global-memory bound or a new input admission rule.

For the broader fixed lane roster,138 actual twist/extension rows in18 cases retain the missing selected-output bound. OBSERVATION_GAPS.json supplies each case count/first row; FIXED_INEQUALITIES_ALL.json retains every row index/key/member and coefficient interval. The cases are:
- RF-WEAK-W-AX-rho1e-08 and rho1e-12;
- RF-WEAK-W-3D-rho1e-08 and rho1e-12;
- RF-RANGE-CHAIN, RF-RANGE-SKEW and RF-RANGE-CONT, each at L-240, E-1000, LEF-small and SIM-b;
- RF-RANGE-THIN-A and RF-RANGE-THIN-B.

All138 affected fixed coefficients are proved below1 by their upper interval. For example RF-WEAK-W-AX-rho1e-08 row67 tw.C has kt in[2^-6,2^-4]. The exact missing premise is a bound on the magnitude of that member's actual selected published j-end torque/axial numerator strong enough that its quotient remains below binary64 overflow. A sufficient source inequality is |v|<=f64::MAX*2^ell, interpreted mathematically/checked exactly, not evaluated by an overflowing floating multiplication. The exact round-to-nearest overflow condition is |v/k|>=2^1024-2^970.

A finite-tag-level counterexample condition exists: v=f64::MAX and any of these proved-small k gives an infinite quotient. This is NOT a demonstrated value returned by the certified solver for the accepted fixture. Reference/control numbers and older passing runs do not prove a selected-output amplitude bound. No numerical criterion is weakened and no fixture is excluded. Selection/row-certificate consequences that could constrain these138 numerators have not been proved in this bounded caller tranche.

The fixed coefficient proof excludes zero/NaN/Infinity denominators for this roster, so the remaining observation concern is signed-Infinity from quotient overflow, not0/0. Underflow to signed zero remains finite. The floor decision does not screen it: C53-57 still calls holds when !covered. Nor does the expected-unresolved list enforce the outcome: L251-256 records a mismatch but does not bypass the Selected row branch; THIN-A/B remain in this conditional obligation.

## Allocation consequence and exact handoff

At a remaining scalar failure, L165-169 constructs Observed::Value(infinity), C40 calls X289-290, and the assertion fires before that operand's Nat::from_u64 allocation. Its absence is not a heap upper for the error path. E/S have already been parsed and remain alive; the selected Outcome, publication/body/floor owners, retained encoding, controls and earlier diagnostics from source11 also remain. Panic formatting/unwind/termination is not assigned zero or investigated here. The successful scalar recurrence cannot cover this branch merely because from_f64 itself has not allocated its8-byte Nat yet.

The floor assertion branch is source-excluded only for the specifically hash-bound fixed-input inequalities just proved; changed files/models require revalidation. Raw magnitude/direct sites are covered by their production tag contract. The broad lane allocation bound therefore remains conditional on the138 selected-numerator obligations or a separately authorized/defined nonfinite outcome/error-path treatment. ROOT must dispose that numerical/contract question; no API, record, oracle, criterion, admission or source change is proposed as already accepted.

The public parse-view stride remains size_of::<&[u8]>() for future checked implementation. All other V-EXACT equations, serializer/sparse/runtime cells, source12/13 private requests, H metrics and final-A1/target correspondence remain unchanged. No full E_max, W1/F2a or admission acceptance follows.
