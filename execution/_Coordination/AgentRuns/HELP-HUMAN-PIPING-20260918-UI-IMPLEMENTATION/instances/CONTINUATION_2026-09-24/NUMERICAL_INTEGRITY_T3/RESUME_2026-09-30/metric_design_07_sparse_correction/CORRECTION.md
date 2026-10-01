# RV30-SP13-F1 — additive HFactor correction

**Correction supplied for RV30 backcheck.** This addendum supersedes only the
first HFactor arm in metric_design_04_sparse/DERIVATION.md. Both that packet
and metric_design_06_closure_assessment remain sealed and unchanged. This is
not closure of the remaining V-SPARSE/E_max premises or review acceptance.

On the unchanged notation, n is the global DOF count and f the free dimension:

```
Build = Fct+16f+f+8f+8f+8f+epsilon*O_G(PivotEvidence,f)
old: HFactor = max(n, Build)
new: HFactor = max(n+16f, Build)
```

No other term, capacity, lifetime, phase window, domain, record or admission
rule changes. epsilon still distinguishes ordinary requested bytes from the
existing growing-realloc move counter. The added16f is retained ownership in
both metrics; it is not a new allocation or active-old surcharge at validation.

## Source lifetime warrant

Immutable40129a225d73860ac2a53da9a2fa73869df668f3 remains the source basis.
DS=projects/chirality-piping/core/solver/sparse_direct;
FK=projects/chirality-piping/core/solver/frame_kernel.

- DS/src/structural.rs70-75 returns the owned order and first_columns Vecs.
  The already reviewed constructors give two exact f-element usize allocations,
  jointly16f on the declared target.
- DS/src/structural.rs82-83 keeps that `ordering` local alive and borrows both
  arrays into factor_sparse_structural_profile. They have not dropped or moved
  into the factor before validation.
- FK/src/structural/sparse.rs1748 first calls validate_sparse_prepared.
  That calls validate_sparse at1622; validate_sparse's seen Vec at1087 has
  global length n (force.len()). Caller order/first coexist with those n bytes.
- The subsequent local `n=prepared.dimension()` at1749 denotes f. Its later
  seen[f], position[f], factor buffers and factorizer scratch already belong
  to Build and are unchanged. The two different source-local `n` variables must
  not be conflated in the estimator's global-n/free-f notation.

Thus the validation arm must explicitly cover n+16f before factor construction.
The corrected helper remains valid without assuming the build arm happens to
be larger. This repairs its independent source-completeness claim.

## Local correction versus composed-bound implications

The sealed sparse_13 review found **no demonstrated fixed-roster or composed
undercount**. It supplied two distinct dominance checks:

1. The original outer HSolver has `P+Fct+PivotCopy+FinishExtra`.
   For nonnegative descriptors, Fct>=16f and FinishExtra>=n, so this term is
   >=P+n+16f. Consequently replacing the first HFactor arm does not enlarge the
   complete displayed HSolver maximum under those same existing inequalities.
   This outer padding does not justify leaving a separately reusable helper's
   validation ownership implicit or incomplete.
2. The review's metadata check covers all213 cases and finds f=0 or n<=49f.
   On successful profile construction hs>=f, so Fct>=24f even without assigning
   a Vec-header stride. Build>=65f then covers n+16f for those cases; when f=0
   the correction adds0. This addendum **reuses the review's sealed roster
   check**; it did not rerun that metadata derivation or create a new case.

These are conditional source inequalities, not a measured-memory result, proof
of an actual overflow, or whole-process E_max/admission acceptance. The direct
n+16f arm is selected for clarity; future consumers need not carry the roster's
n<=49f premise just to reuse HFactor.

## Unchanged residuals and return

All other source04/source11 cells remain: allowance-map node requests, valid
stored/value-sensitive sparse descriptors or proved uppers (including a useful
hs bound), final public/Expansion/library/feature/build correspondence,
class/Debug/expect/panic/IO terms, and checked owner-union integration. No new
proof programme or runtime authority follows. Canonical K0 assembly follows the
active proof reviews. **No contract alternative is selected.**

RV30 should backcheck this exact replacement and its preserved limits. The
review finding is addressed by this addendum but is not marked independently
closed by its author. No maintained source or estimator was edited.
