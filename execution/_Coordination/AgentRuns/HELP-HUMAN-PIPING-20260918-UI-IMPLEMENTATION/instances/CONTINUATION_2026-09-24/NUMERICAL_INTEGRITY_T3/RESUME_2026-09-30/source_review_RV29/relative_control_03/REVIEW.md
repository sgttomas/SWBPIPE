# RV29 independent relative-radius control check

**VERIFIED as an isolated arithmetic/certificate fixture.** This is not a new
primitive-source model, expected solver selection, executed Rust result or final
implementation acceptance. ROOT's supplied549b894e98 proposal was checked using
independent integer nearest-even rounding; no hardware floating operation, solver
output or production helper supplied an expected answer.

For x=S bits3ff0000000000401, the exact value is(2^52+1025)/2^52. With
H=x(2^-64+2^-85+2^-53), independent multiplication gives
H=19352257851307810204156929/2^137. The odd numerator has85 significant bits
and leading exponent−53, so P256 represents H exactly. CHECKS.json supplies
its normalized little-endian Wide4 limbs.

The five prescribed binary64 steps independently round to:

| Step | Bits |
|---|---|
| a0=RN64(2^-64*x) | 3bf0000000000401 |
| a1=RN64(a0*(1+2^-21)) | 3bf0000080000401 |
| u0=RN64(2^-53*x) | 3ca0000000000401 |
| u1=RN64(u0+h) | 3ca0000000000401 |
| A_f64=RN64(a1+u1) | 3ca0020000100402 |

Here h=2^-1074 and A_exact=H+h. H lies2149581825/4294967296 of the way
from predecessor3ca0020000100401 to its successor. This fraction is strictly
above1/2, so RN64(H)=RU64(H)=3ca0020000100402. Exact rational comparison proves

    predecessor < H < A_exact < A_f64 = RU64(H).

Also10^9*H<x. The scale exceeds2^-988, and the actual rounded classification
threshold has bits3dd0000000000401, strictly below x. Thus RelativeVerified is
the real classification, not a synthetic forced class. Every operand and allowance
is finite/nonnegative. The disclosed initial3ff0000000000400 trial remains
rejected and was not substituted for the proposed control.

## Compact certify_rows fixture requirements

Use candidate precision128, verification/report precision256 and an eligible
free Translation or Rotation *component*, with x=v equal to the supplied bits
and W_plus exactly H. Do not choose DisplacementMagnitude as the target: it
adds2^(1-P)|v| and would change H. Keep canonical layout, matching row metadata,
report lengths and required error fields; obtain final class/scales through the
real publication path and assert the target S bits and RelativeVerified class.
This is a controlled private certificate input, not a claim that a real solve
produced these chosen verification/error fields.

A compact reuse of the existing two-node/one-free-Ux zero-pair scaffolding is
possible without introducing a new primitive model: set supplied publication
and verification values of free D:6 and its M:1 magnitude to x, keep prescribed
rows zero, and retain the canonical full report shape. Give D:6 W_plus=H; keep
M:1 W_plus=0 and other required fields valid zero values. With the existing
unit extent, the target translation scale is exactly x. Other rows must still
pass normally rather than being skipped or misclassified.

In that consistent magnitude companion, the mandatory magnitude term gives
H_M=x*2^-255, exactly representable binary64 bits3000000000000401. It passes
its relative allowances and supplies a second distinct positive private radius,
useful for the already identified pairing/permutation check. This companion
calculation is independently checked in CHECK.py; it is optional fixture reuse,
not an additional source witness.

Exercise certify_rows through successful return and assert target radius bits
3ca0020000100402 with the same target publication/class/identity. Independently
assert radius>A_exact while H<A_exact; exact admission must succeed. A mutation
that substitutes the upward stored radius for H in the sharper-exact admission
comparison must fail this fixture. It does not isolate public-decimal-boundary
rejection, because the public predicate has slack; preserve that separate check.

The control does not replace required malformed/late-failure/schedule checks.
I22 must bind the actual constructed values and report fields to these independent
constants, then supply the future frozen test delta and authorized runtime result.
No maintained code or Rust execution was performed by RV29.
