# F4 friction-plan reconciliation V1

## Verdict

**PASS** for the reconciled design at source basis `533332349a4607eee561d4ef90fb05a62d86519e`.

Reviewed immutable inputs:

- `instances/F4/FRICTION_REPAIR_IMPLEMENTATION_PLAN_V1.md` — SHA-256 `8651bad68ea72425ee00d17fea200d3c1883404fdd65fe52cbb7e5f4b4c38566`.
- `instances/F4/FRICTION_REPAIR_VALIDATION_MATRIX_V1.md` — SHA-256 `91a2789b856cf5244dc942fb414e86753332bbc452b1859238e512ad23f4251b`.
- P0 `returns/NS/RUNTIME_PROBE.md` — SHA-256 `44ba3704f668f36c3aabe4fe9bc22919a9b4f461aad8ba3e614586466dbaa087`.
- P0 `returns/NS/VALIDATION.json` — SHA-256 `ef0dab886b0ee858edee3d7c9b94e486362fc575338af542841175ed6b015948`.

## Acceptance basis

The revised F4 method resolves all actionable findings from the provisional review:

1. It applies `F_t u_t <= 0` generally and reserves strict opposition for positive-limit sliding, preserving contacted `mu=0` release and explicit no-contact behavior.
2. It records exact-boundary sticking at zero displacement as the project's deterministic static convention and tests the immediately adjacent `f64` loads without a tolerance band.
3. It keeps the initially sliding, no-force first iterate only as an explicitly nonconvergent warm-start direction trial, preserving the established two-iteration super-limit fixture without adding physical history.
4. After a bounded-force trial, it overrides every tangentially invalid sliding row to `Sticking` before changed-state, residual, convergence, and diagnostic formation, even if roundoff made the base cone classifier return `Sliding`. The separate defensive consistency gate cannot become a same-state retry loop.
5. It distinguishes an inadmissible derived-normal sign assumption from a bad tangential branch. A normal-branch-only mismatch remains `Sliding` and retries with the observed current normal sign, preserving PR 760's same-iterate affine coupling and established three-iteration sign-flip path.
6. It updates simultaneous invalid rows in one deterministic state vector, makes no global-uniqueness claim for solution-dependent multi-contact systems, and preserves visible nonconvergence at the existing cap.
7. Its two-file source fence is coherent with existing external benchmarks because valid super-limit sliding seeds retain their established iteration count. No schema, history state, convergence tolerance, or product adapter change is required.

The current-SHA Rust probe independently confirms the pre-repair counterexample through the real solver API: sticking seed gives `u=0`, `R_t=-1`; sliding seed gives `u=-0.02`, applied/reported `F_t=-3`, and `F_t u=+0.06`, with both paths incorrectly reporting convergence before repair.

## Remaining gates

This PASS covers design readiness only. Implementation must stay inside the two source files, satisfy every F4/FREFUTE oracle and mutation check, retain existing downstream benchmark/product tests, receive fresh read-only review of 100% of the frozen diff after any remediation, and complete the project validation/CHANGE sequence. No external industry-solver comparison has run.
