# F4-I1 candidate return

Status: `CANDIDATE_COMPLETE_PENDING_FRESH_INDEPENDENT_REVIEW`

Implemented the reviewed simultaneous affine current-normal correction only in `core/solver/nonlinear_integration/src/lib.rs`. Active derived-normal sliding rows now use reported base reactions and reported base-plus-unit reaction differences to form `H`, solve `(I + C H)q = -C r0` together, and then run the caller-selected final solve. Public applied-force records retain nonlinear-support input order. `sliding_direction`, DEC-046 state-count controls and outer cap, public structs, canonical SI, anti-chatter behavior, and sparse/dense final-solve evidence remain unchanged.

Runtime branch validation is exact: signed zero is normalized by `r == 0.0`; nonzero branches use closed halfspaces; an active zero branch accepts only exact zero; zero coefficient or zero existing direction is branch-irrelevant. No runtime tolerance, residual family, inner retry budget, public field, history, reference, or direction choice was added.

## Source and diff binding

- Before SHA-256: `fb02a52273637f844c63ac2e1545bc7fbe8ab79b58d7f5e259829e46be4b1439`
- After SHA-256: `da4cc3f5d21f1e1841d375dd2525f3bb2d3f0e5d3e960bd2389d32cef3a8b320`
- Diff size: 726 insertions, 50 deletions; `git diff --check` exit 0
- Complete diff: `projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/implementation/CANDIDATE_DIFF.patch`
- Diff SHA-256: `8080a8c242931409863771aba963897a72b055c39aee1d61fd567eee4bb3801b`; byte comparison against the live source diff exited 0
- Historical M1-N-008 witnesses stayed unchanged: Cargo.lock `ac1abea2355eafb839ee15c8dbde2cb500710b66217106c9b036b59d54461cd4`; Cargo.toml `326677deba0692d4eb2bebbb4c4bd009c805fc0a742197798a072afaf4924558`; main.rs `5b4671282466232902d42daeb6ec12391654381c067c9b9276cc6dbd6253ed2d`.

Changed production symbols/lines: loop integration at line 427; `solve_iteration_with_sliding_friction` at 704; `sliding_friction_candidates` at 890; `reported_reactions` at 961; `normalized_reaction_sign` at 973; `derived_normal_branch_admissible` at 981; `nonconverged_exit_diagnostic` at 1705. Tests begin at lines 2048, 2101, 2146, 2178, 2247, and 2274.

## Verification

- Offline locked compile-only: exit 0.
- Frozen positive/negative rational oracle test: exit 0, 1 test / 8 seed-mode-sign cases. Positive: `u=7/135`, `N=200/27`, `Rf=-20/9`. Negative: `u=-7/165`, `N=400/33`, `Rf=40/11`.
- Historical M1-N-008 source rerun: exit 0; both modes/seeds returned the corrected positive result (`u=0.051851851851851864`, `N=7.407407407407408`, `Rf=-2.222222222222222` at printed precision).
- Focused exact-domain, both sign-flip retries, cap failure, two-row order, active-zero retry, zero-coefficient, and nonzero-branch/final-zero tests: all exit 0.
- Final `cargo fmt --check`: exit 0.
- Final offline locked full crate: exit 0; 29 passed, 0 failed, 0 ignored.
- Profile selector: exit 0; selected `evidence-sweep`, `harness-self-check`, and `piping-pytest`.
- Configured-interpreter piping pytest: exit 0; 1012 passed.
- Configured-interpreter harness self-check: exit 0; existing repository findings were reported.
- Full evidence sweep deferred to root while U7 concurrent writes remain, as directed by F4 manager.

Preserved nonterminal failures: the first focused invocation used `--exact` with an incomplete test path and exited 0 with 0 selected; the corrected invocation passed 1. The initial bitwise order assertion exited 101 because one force/reaction differed by `4.44e-16`; it was replaced by the reviewed fixture scale `1e-12` and passed, while runtime branch logic remains exact. Initial `cargo fmt --check` exited 1 with formatting-only diffs, then formatter and recheck exited 0. System Python self-check exited 2 because PyYAML was unavailable; the configured interpreter rerun exited 0. A broad `rm -rf` cleanup attempt was tool-rejected; an initial `cargo clean --target-dir` without a manifest exited 101; the manifest-bound retry exited 0 and removed 2626 files / 170.9 MiB. The private target absence check exited 0.

## Scope and residual risk

Only the exclusively assigned nonlinear-integration source plus this child evidence and the authorized DEL-04-04 implementation diff were written. No product-physics or other source, decision, lifecycle, dependency, requirement, status, memory, pointer, Git, or sibling artifact was edited. Unrelated concurrent worktree changes were left untouched.

Residual risk is limited to fresh independent review and root-coordinated clean-source evidence sweep. Singular affine coupling still returns the existing frame-kernel error; an inadmissible branch still retries only through the existing outer cap and returns the existing NonConvergence failure envelope at the cap.

This is candidate source for F4 validation. It is not an acceptance or release claim.
