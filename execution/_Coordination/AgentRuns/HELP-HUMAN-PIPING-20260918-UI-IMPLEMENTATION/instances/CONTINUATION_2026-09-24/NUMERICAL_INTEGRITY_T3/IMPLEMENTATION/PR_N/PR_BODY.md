## A correctly rounded Euclidean norm replaces libm `hypot` on published paths

`f64::hypot` is not correctly rounded, so published support magnitudes depended on the platform's libm. macOS and glibc differed in the last bit, and B1 needed per-platform pins. This PR adds `norm2` and `norm3` to frame_kernel (`correct_norm.rs`).
- They return RN(√(a² + b² [+ c²])) using IEEE operations, `sqrt` and `fma` only, so every platform gives the same bits.
- A 3-component magnitude is the correctly rounded 3-norm, not a hypot chain.
- Special values follow C Annex F.
- They are verified against an exact integer-square-root oracle: 0 misrounded in 22,000,000 results, including the subnormal and overflow ranges and 1,000,000 adversarial near-midpoint triples. 1,200 vectors are committed.

**Call sites:** every product `hypot` that reaches published bytes, a receipt or diagnostic text now uses the norm. That covers:
- the support and support-action magnitudes;
- the combination magnitudes;
- the intensified i·|M|/Z;
- the formation guard's q;
- the load-reference length;
- the pressure-region guard;
- the retained certificate's support projection and the retained support guard;
- the rigid-body rank screen, which is an admission decision.

stress_recovery includes the module by `#[path]`. No manifest, lock or reviewed input changes.

**What moves:** among committed pins, one value by one ulp. (On the Mac, the outputs for `t13`'s and the runner's `load_reference` inputs also move by one ulp each, to the committed bytes, which were taken on Linux; the change record lists every one.) On macOS arm64, case C's `rigid:N0` support force magnitude goes from `1.6258317075882521e-12` to the correctly rounded `…523e-12`, which is glibc's value. Re-pinned with one pin for every platform:
- W-C2 dense (fixture and pins);
- (C, B, A) dense;
- u1's ordinary pin, now asserted on every target;
- the two reader-corpus cases and the PY 07n snapshot pins.

m08's expectation uses `norm2`. B1's glibc-only variants are retired. The ring check allows the absolute escape only for the near-zero residues (RV125 A1-N1). `t13` and the runner's two `load_reference` tests now pass on the Mac.

On glibc every committed pin holds. A value formed elsewhere from a hypot chain, or where a libm `hypot` was misrounded, can move by one ulp on either platform; the change record lists every moved value with its components.

**Evidence:**
- the 40 manifests on the Mac: 0 FAILED;
- glibc dispatches 37808190331 (success) and 37820998162;
- the PY and TS readers that read the corpus: all pass.

**Not in scope:** the remaining libm calls on product paths (sin, cos, atan2, asin, exp, exp_m1, mainly curved bends and logarithmic thermal strain). They stay platform-dependent and are a later round.

The evidence package, `IMPLEMENTATION/PR_N/` (CHANGE_RECORD.md), holds the moved values, the D1 call sites and the gates. Implemented by I109 (T3).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
