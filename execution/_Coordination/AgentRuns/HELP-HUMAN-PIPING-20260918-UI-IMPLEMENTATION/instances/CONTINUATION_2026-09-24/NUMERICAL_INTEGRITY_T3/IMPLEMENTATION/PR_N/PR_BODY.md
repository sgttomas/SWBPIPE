## A correctly rounded Euclidean norm replaces libm `hypot` on published paths

`f64::hypot` is not correctly rounded, so published support magnitudes depended on the platform's libm. macOS and glibc differed in the last bit, and B1 needed per-platform pins. This PR adds `norm2` and `norm3` to frame_kernel (`correct_norm.rs`).
- They return RN(√(a² + b² [+ c²])) using IEEE operations, `sqrt` and `fma` only. Given a correctly rounded `fma` (the hardware's, or the platform's where there is none), every platform gives the same bits; a wasm32 build with software `fma` equals aarch64 bit for bit (RV126).
- Every magnitude formerly formed with libm `hypot` is now a correctly rounded norm; a 3-component one is the correctly rounded 3-norm, not a hypot chain. Two published magnitudes never used `hypot` and are unchanged: the selected-source support magnitude (`source_receipt::scaled_norm`) and the nodal `displacement_magnitude`. They are deterministic IEEE but not correctly rounded. (This corrects the code commit's message, which said every published 3-component magnitude is the 3-norm; RV126 S-1.)
- Special values follow C Annex F.
- They are verified against an exact integer-square-root oracle: 0 misrounded in 22,000,000 results, including the subnormal and overflow ranges and 1,000,000 adversarial near-midpoint triples. 1,200 vectors are committed.

**Call sites:** 30 of the 32 product `hypot` calls now use the norm: every one that reaches published bytes, a receipt or diagnostic text, plus the rank screen (admission) and `elastic_section` (no caller). The other 2 are in `performance_harness`, which is not a product dependency. That covers:
- the support and support-action magnitudes;
- the combination magnitudes;
- the intensified i·|M|/Z;
- the formation guard's q;
- the load-reference length;
- the pressure-region guard;
- the retained certificate's support projection and the retained support guard;
- the rigid-body rank screen, which is an admission decision (below).

stress_recovery includes the module by `#[path]`. No manifest, lock or reviewed input changes.

**The rank screen** now makes one admission decision on every platform. Compared with the libm screen on synthetic bodies (RV126 §2a):
- near the threshold the Restrained decision changes in both directions (59,600 and 66,967 of 1,045,305 probes), only within 1.5·10⁻³ relative of it, where the libm decision was already non-monotone and platform-dependent;
- at 10²⁰⁰ scale, 1 of 300,000 random bodies goes from MechanismWitnessed to NumericallyUnresolved, which changes the refusal's integrity code;
- a witnessed mechanism's published node motions (diagnostic text and retained wire) move by an ulp wherever its characteristic length L did (22,966 of 246,587 synthetic mechanisms).

No committed input lies in that band, no committed pin moves, and no true mechanism is admitted in either version. ROOT accepted these changes (RV126's B-1, option (i)).

**What moves:** among committed pins, one value by one ulp. (On the Mac, the outputs for `t13`'s and the runner's `load_reference` inputs also move by one ulp each, to the committed bytes, which were taken on Linux; the change record lists every one.) On macOS arm64, case C's `rigid:N0` support force magnitude goes from `1.6258317075882521e-12` to the correctly rounded `…523e-12`, which is glibc's value. Re-pinned with one pin for every platform:
- W-C2 dense (fixture and pins);
- (C, B, A) dense;
- u1's ordinary pin, now asserted on every target;
- the two reader-corpus cases and the PY 07n snapshot pins.

m08's expectation uses `norm2`. B1's glibc-only variants are retired. The ring check allows the absolute escape only for the near-zero residues (RV125 A1-N1). `t13` and the runner's two `load_reference` tests now pass on the Mac.

On glibc every committed pin holds. A value formed elsewhere from a hypot chain, or where a libm `hypot` was misrounded, can move by one ulp on either platform; the change record lists every moved value with its components.

**Evidence:**
- the 40 manifests on the Mac: 0 FAILED;
- glibc dispatches 37808190331 and 37820998162 (the re-pinned head, numerical cargo suite included): success;
- the full-SHA dispatch 37824479785 on `dab19291a8`: success;
- the corpus readers: the two PY files (1,079 passed) and the four TS files (1,314 passed), run by RV126 at `dab19291a8` and again by I109 at the repair commit `8ca80508b6`;
- at `8ca80508b6`: PP 743, FK 550 and `result_export` 199 passed, 0 failed.

**Repair round 01** (`8ca80508b6`, RV126's S-1 and notes): comment-only, line-neutral production edits (the S-1 wording above, the `fma` condition and two K5 comments) and two test edits (m08's header and exact constants for `support_hypot`). Its message states S-1's correction.

**Not in scope:** the remaining libm calls on product paths (sin, cos, atan2, asin, exp, exp_m1, mainly curved bends and logarithmic thermal strain). They stay platform-dependent and are a later round.

The evidence package, `IMPLEMENTATION/PR_N/` (CHANGE_RECORD.md), holds the moved values, the D1 call sites and the gates. Implemented by I109 (T3).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
