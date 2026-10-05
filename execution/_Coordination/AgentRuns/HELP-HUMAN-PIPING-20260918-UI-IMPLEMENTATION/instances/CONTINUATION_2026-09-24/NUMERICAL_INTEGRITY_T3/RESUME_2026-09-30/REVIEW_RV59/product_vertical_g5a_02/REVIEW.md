# RV59 G5a operational operand follow-up

**The proposed correction is sufficient for the private ordinary witness, subject to the concrete conditions below.** It is a new evaluation of the existing operational expressions, not a new numerical truth contract and not capture of historical intermediates. Until the implementation supplies and checks these operands, it must return an explicit missing-operational-operands refusal and withhold complete G5a/case success.

This narrows the earlier RV59 CLEAR: the earlier statement that A/Z and body extent suffice is correct for final row scales, but did not discharge G5a item 4. The original I45 and RV59 packets remain unchanged. The correction does not reopen the direct/recipe hull theorem, original K gates, source residual proof or final ordinary hook.

## Owning contracts and the omitted prerequisite

P=projects/chirality-piping; FK=P/core/solver/frame_kernel; SP=P/core/solver/straight_pipe; T3 is NUMERICAL_INTEGRITY_T3 in the governed run; R=T3/RESUME_2026-09-30. Maintained source below is independently hash-checked against 6ba653451f9fd27cdb852b7974753a3921f1c483 through NUM. No in-progress CODE source was read.

- T3/DESIGN_NUMERICS/DESIGN.md:501–508 fixes section terms A/Z/L/k_a/k_t as the operational product bits, with source identity. Its :868 explicitly specifies k_a=fl(fl(E*A)/L), k_t=fl(fl(G*J)/L), formed as FK forms its coefficients. The full receipt's custody/reader claim remains a separate obligation.
- T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md:817–850, §6.3 G5a item 4, uses k_a/k_t to lower-bound resolution scales from normalized final nodal displacement/rotation rows, including InputDerived. L is needed to form those coefficients; body extent L_b is not a substitute.
- I45 RETURN:33 limits A/Z sufficiency to row scales but :71 promises G5a; the missing operational evaluation means the complete gate was not yet specified. This is the narrow blocker against relying on an uncorrected implementation, not a defect in the unchanged predicates.

## Exact new evaluation

Bind the actual member ordinal, endpoints and full source/case/basis owner first. Read actual built FrameNode coordinates and E/G/A/J from the same StraightPipeElement section that supplied K. SP lib.rs:414–423 copies the six section values directly to FrameSection; :449–457 copies its nodes/section/y-reference into FrameElement. No material lookup, interpolation, geometry/section construction or alternate K primitive is needed. All current member/source/map/eligibility checks remain mandatory.

The literal existing length routine is FK lib.rs:586–590, with normalize/norm/dot/subtract/scale at :1864–1898:

1. Compute each delta component as RN64(x_j[a]-x_i[a]), in x/y/z order.
2. Compute norm by three separate RN64 squares, then RN64(RN64(dx²+dy²)+dz²), then the existing binary64 sqrt. Do not use hypot, powi, extended precision, a fused multiply-add, exact coordinate differences, or right-associated addition.
3. Apply normalize's actual `magnitude <= 1.0e-12` degeneracy check, using the same binary64 tolerance. On its success path, normalize evaluates RN64(1/magnitude) and the three component products. These values are discarded by FrameElement::length, but they are part of the literal source schedule.
4. FrameElement::length then computes norm(delta) again in the same order and returns it. Preserve finite positive length as FrameProperties::new requires (:484–486). Successful deterministic evaluations of the same norm yield the same bits; no result from a retained exact-coordinate source norm is interchangeable with this operational L.
5. Compute EA=RN64(E*A), then k_a=RN64(EA/L); separately GJ=RN64(G*J), then k_t=RN64(GJ/L), in the order of FK lib.rs:742–751. Never use E*(A/L), an exact product divided once, or a scaled product as an unnoticed replacement.

A direct PP helper can evaluate this fixed scalar sequence from captured fields without reconstructing a FrameElement or forming stiffness. In particular, do not call SP StraightPipeElement::length or frame_element: those methods rebuild a FrameElement and rerun properties/orientation (:445–457). Borrowing the already produced actual FrameElement and calling only its length method is another scalar evaluation, but all its entered work still belongs to this new check. No full local/global stiffness call is needed; do not compute its unrelated L²/L³ and bending coefficients merely to extract two scalars.

This gives a deterministic operational equality: with the same admitted built operand bits, same RN64/sqrt operations and same order, each newly computed scalar equals the corresponding expression's value. It does not prove that the scalar was historically stored, observed during an earlier matrix formation, or covered by a section_terms receipt. Name the records as newly evaluated operational-check operands and keep this provenance distinct. If actual ordinary formation used force scaling or a different coefficient expression, do not relabel these unscaled expressions as observed W2 coefficients. This derivation supplies only the defined unscaled operational check values. Any broader route claim needs its own association/operation-order warrant; otherwise refuse that claim and report operand range insufficiency without changing ordinary behavior.

## Checks and spent-work constraints

Require finite actual coordinates and finite positive E/G/A/J with correct owner/bit association. The actual built-element admission remains the source of geometric eligibility. New finite/range checks are private witness refusals and do not change ordinary model admission or outputs.

FK checked_formation_value (:819–854) rejects zero, subnormal or nonfinite product/quotient results from finite nonzero operands. Preserve that boundary for EA, k_a, GJ and k_t individually. A final normal k does not repair an intermediate overflow/underflow; do not rescue it by reassociation or scaling. Missing/foreign operands, failed length/denominator, range refusal and lost accounting remain distinct. No fallback to zero, body extent, source real length or caller-supplied scalar is permitted.

The literal successful length schedule has 19 named binary64 source operations: 3 subtractions; two norms, each with 3 multiplications, 2 additions and 1 sqrt; one reciprocal and three normalization products. Both coefficients add 4 operations, totaling 23. This is a source-level operation count, not machine instructions or target timing. Count actual field/identity checks, finite/range checks and storage separately. A degeneracy return occurs after the first 9 arithmetic operations; an axial product refusal adds only its entered product after a successful length. New explicit finite guards may stop sooner, but must record that actual new prefix and reason, not claim a later operation ran. If an implementation deliberately eliminates repeated/dead arithmetic, it must state the equivalence and record its actual reduced schedule rather than booking 23 as spent.

Collect work from the producing new evaluation once on every exit. Do not replay it to manufacture counts, charge earlier ordinary formation again, label the new work free because the original builder ran, or reset a fault. Join the new work status with the rest of the certificate before success. There is still no selected tariff, facade admission or whole-host profile.

## G5a consumes these operands unchanged

Use the actual final row bindings after their normal raw-to-SI conversion, not the private source center or native rows substituted for product rows. Resolve all six nodal components uniquely per endpoint; include InputDerived. Keep the original source-associated body resolution scale/estimate/charge/theta/B facts.

For each endpoint use Nt=RN64(RN64(abs(ux)+abs(uy))+abs(uz)) and the same Nr sequence for rotations. Add endpoint Nt/Nr once each. Per kind compare N with RN64(2^-59*S). Only the prescribed small-N branch returns zero; otherwise form RN64(k*RN64(N-RN64(2^-60*S))). Compare the member maximum to RN64(e_hat*c), with c bits 0x3ff0000000001000. Preserve G5a shape, +0 zero rule, coupled-scale sanity, summary bounds and all other gates. No new normal-only test is imposed on G5a's threshold/margin arithmetic: its specified binary64 zero/subnormal behavior remains intact, while nonfinite required check results refuse. A finite G5a failure on actual ordinary output remains a truthful private witness result, not permission to change output or relax a predicate.

## Controls and verdict boundary

Five standard-library bit/exact groups pass. The oracle uses Fraction and bounded binary64 bit searches, including an exact midpoint-square test for sqrt rounding. It imports no product code and performs no model, builder, stiffness or solver run.

- Norm reassociation changes the result from 0x1.0000000000001p+0 to 0x1.0000000000000p+0 for delta=(1,0x1.6cp-27,0x1.6cp-27).
- Coefficient reassociation changes 0x1.3ff6019418a0ap+14 to 0x1.3ff6019418a09p+14 for the preserved operands.
- Product overflow, product subnormal and quotient subnormal refuse at their actual scalar stage; the first two cannot be justified by a representable reassociated final value.
- Unit conversion and inclusion of InputDerived change the G5a lower bound as required; all-zero lower-bound input takes its exact zero branch.

These establish operation-order sensitivity and the finite scalar derivation, not Rust target parity or successful W0/W1 product admission. The implementation still needs actual source association, failure-prefix accounting, bit parity and G5a controls on the exact eventual candidate. ROOT may select this narrow correction within retained_product.rs; no new public truth/predicate decision is required. No source change is granted or performed by this review. The separately reported tenth-path S11 test inventory grant is not reviewed or enlarged here.
