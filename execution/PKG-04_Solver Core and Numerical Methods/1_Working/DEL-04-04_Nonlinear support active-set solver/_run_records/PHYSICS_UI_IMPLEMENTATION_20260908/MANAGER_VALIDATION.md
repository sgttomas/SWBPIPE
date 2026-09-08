# F4 manager validation

Status: `PASS_PENDING_FRESH_INDEPENDENT_REVIEW`

WORKING_ITEMS validated the complete live diff against the frozen V2 brief and the sealed I1 scope. The implementation uses reported base and base-plus-unit reactions, solves all active derived-normal rows simultaneously with `(I + C H)q = -C r0`, reconstructs public applied-force records in nonlinear-support input order, runs the selected final sparse/dense solve, and gates convergence with the exact reviewed branch domains. The existing sliding-direction rule, active-set state-count controls, outer cap, diagnostic code/severity/source, public structs, SI convention, and failure behavior remain intact. No runtime tolerance, new residual, history rule, direction rule, reference choice, public field, or inner iteration budget was added.

The manager reviewed all changed production lines and tests. The frozen diff is byte-identical to the live solver diff. Product-physics remained byte-identical and no optional product test was needed. The sole changed core path is `core/solver/nonlinear_integration/src/lib.rs`.

Manager verification used an isolated Cargo target:

- offline locked full nonlinear-integration crate: exit 0; 29 passed, 0 failed;
- `cargo fmt --check`: exit 0;
- isolated manager target cleanup and absence check: exit 0.

Child verification accepted at fan-in:

- positive and negative exact rational fixtures, both seeds and both solve modes: pass;
- unchanged M1-N-008 source witness: exit 0 with the accepted current-normal result in both seeds/modes;
- exact zero, assumed-zero/nonzero, both sign flips, cap failure, zero coefficient, and two-row reversed-order cases: pass;
- configured-interpreter piping pytest: 1012 passed;
- configured-interpreter harness self-check: exit 0;
- final child crate run: 29 passed, 0 failed; formatting: exit 0.

Preserved nonterminal evidence includes the 0-test incomplete `--exact` selector, the initial exact-order assertion failure with a measured `4.44e-16` floating difference, the formatting-only first check, the system-Python missing-PyYAML failure, and cleanup command corrections. The `1e-12` scale is confined to the existing fixture comparison; runtime branch and convergence logic uses exact zero and closed-halfspace comparisons.

The clean-source evidence sweep remains deferred to root while U7 writes concurrently. This candidate is ready for the required fresh independent 100% diff review; it is not release acceptance.
