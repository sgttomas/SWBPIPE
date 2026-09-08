# RF independent F4 product-diff review

Verdict: `CHANGES_REQUIRED`

Production assessment: no production defect was found. At source SHA-256 `da4cc3f5d21f1e1841d375dd2525f3bb2d3f0e5d3e960bd2389d32cef3a8b320`, the simultaneous affine solve implements `(I + C H)q = -C r0` with the reviewed signs and dimensions. Base reaction subtraction uses the public reported-reaction convention and correctly includes prescribed-displacement and explicit-normal affine offsets. The final solve preserves the caller's sparse/dense mode evidence; the internal coupling solve reuses the existing dense solver and propagates singularity. Exact signed-zero normalization, closed nonzero halfspaces, active zero-branch rejection, finite checks, first-iterate direction deferral, the unchanged public sliding-direction rule, outer cap, diagnostic vocabulary, public structures, canonical SI, history behavior, and product result mapping were preserved. Multiple derived rows are assembled and solved simultaneously.

## RF-F4-001 — required active-zero retry proof does not exercise an assumed-zero row

Location: `core/solver/nonlinear_integration/src/lib.rs:2247` (`active_zero_branch_with_coupled_nonzero_current_normal_retries`), with the only direct zero-branch predicate assertions at lines 2086-2095.

Impact: the frozen V2 acceptance case “active assumed zero with final nonzero under active friction retries and never falsely converges” remains unverified by the committed test. A regression that bypasses or misroutes the zero branch in `solve_iteration_with_sliding_friction` could pass this test.

Evidence: RF instrumented the exact fixture through the public solve. For F-X, the iteration sequence was `Inactive @ 0.0`, `Sticking @ +0.26829268292682973`, `Sliding @ -0.7317073170731703`, then coupled forces `+0.0537660687228116` and `-0.05355143371793012` on iterations 4 and 5. When F-X first becomes a coupled derived candidate, its previous normal is nonzero, so the helper assumes `-1`, then `+1`; it never assumes `0`. The test assertions at lines 2261-2269 observe only eventual convergence, a nonzero second-iteration reaction, and the final Coulomb magnitude. The predicate-only checks at lines 2091-2093 establish the boolean truth table but do not exercise branch selection, affine-force construction, or the outer convergence gate. The full lossless trace is `_run_records/EXTERNAL_WITNESS_FINAL.txt`.

Remediation direction: add a direct helper-level fixture with a synthetic prior sliding iteration whose derived source reaction is exactly `+0.0` and `-0.0`, a base response with nonzero current normal, and active `mu`/direction. Assert that the solved derived force remains the zero-branch force and `derived_normal_branches_admissible == false`; retain an explicit assertion that the caller's convergence gate cannot accept that iteration. Also mirror the line-2101 affine solve for an assumed `-1` branch ending at exact zero; the current affine solve fixture covers only assumed `+1`, while the `-1` case is predicate-only.

## Other reviewed behavior

- Independent rational derivation for the 45-degree fixture gives `K=[[150,-50],[-50,150]]`. Positive motion solves `135u=7`, hence `u=7/135`, `N=200/27`, `Rf=-20/9`; negative motion solves `165u=-7`, hence `u=-7/165`, `N=400/33`, `Rf=40/11`. Source tests and the unchanged M1-N-008 witness matched these values for both seeds and modes.
- Independent two-row replay verified each final force against its own `mu*abs(current normal)`. Reversing row order produced zero displacement delta and a maximum reaction/force delta of `4.440892098500626e-16` in both modes. The corrected `1e-12` comparison changes floating comparison only; it does not hide meaningful order sensitivity.
- Independent public fixtures verified explicit-normal loads in the affine base (`qx=-2.9540918163672658`, `qy=-10`, `N=14.770459081836327`) and a nonzero prescribed displacement offset (`q=-3.8078156312625233`, `N=-19.039078156312627`). Both satisfy the current-normal relation.
- A singular one-row coupling propagated `FrameKernelError::SingularSystem`; an overflowing finite-input coupling right-hand side returned `InvalidInput` naming the nonfinite right-hand side. Existing cap failures retained `NonConvergence`, Failure severity, and SolverIteration source.
- No public type, field, solve signature, conversion constant, dependency, schema, migration, history surface, or Git state changed in F4. Stable `frame_kernel`, `nonlinear_supports`, `diagnostics`, and `product_physics` callers are byte-unchanged from base.

Residual risk: the current production hash passed all independent mechanics checks, but the missing frozen branch proof is a regression-evidence gap and blocks this review. This review performs no lifecycle, engineering, dependency, integration, or release acceptance.
