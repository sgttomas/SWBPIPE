# I23 R46 survivor diagnosis14

Source-only diagnosis complete. Start **2026-10-01 17:26:45 UTC**;
return **2026-10-01T17:30:18Z** (213.1 seconds, within10 minutes).

**The narrowed mapping lacks a discriminating assertion; the frozen fault is
active and non-equivalent.** Original R46 remains a survivor with zero kill
credit. Its A1 unit test includes q=3*2^-1060,S=2^-995: the upward branch is
active, correct bound bits0x8002 versus faulty0x8001, but both exceed the test's
RHS0x8000. Every exact-bit operand either bypasses the branch or has the same
nearest/upward result. Independent integer/Fraction binary64 arithmetic covers
all original operands; no production or mutated row_bound was run.

The historical RV19 record already shows this same unit test passing under
RV19-D2. The original kill came from classification set19, row20: published
rotation q=S with bits000e0eb8edf1a91a, correct bound_bits3 versus fault2.
That fixed row and expected value still exist in immutable40129. Preferred
follow-up is the existing classification bit-for-bit filter at its original
class-vector assertion, subject to ROOT/RV29 disposition and later run grant;
no new test/input is needed merely to recover the historical witness.

Optional focused proposals are distinguished: an exact-bit assertion0x8002
for the existing tiny loop operand, or the NEW smallest positive case q=S=h
(h=2^-1074), whose formula gives3h (bits3) versus faulty2h (bits2). Neither was
installed, patched or prepared. Old expectations/thresholds remain untouched.
No generic/earlier failure can qualify, and no new production-gate defect or
runtime result is claimed.

OPERAND_ARITHMETIC.json binds exact operands and rounding; SOURCE_AND_HISTORY
binds frozen source, unchanged patch, old review/log and current mapping;
DIAGNOSIS_AND_PROPOSAL gives the smallest bounded routes and exclusions.
No runtime/Rust/build/test/solver, patch/copy preparation, maintained edit,
Git/index or delegation occurred. No expectation coordination with the parallel
reviewer occurred. ROOT disposes the survivor after independent review; later
runtimes remain held. SHA256SUMS seals this additive packet.
