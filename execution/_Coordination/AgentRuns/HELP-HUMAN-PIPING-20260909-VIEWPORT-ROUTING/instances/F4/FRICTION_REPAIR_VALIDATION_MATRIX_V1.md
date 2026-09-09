# F4 friction repair validation matrix V1

All new assertions use exact analytical values for the scalar fixture. Existing crate comparisons may retain their already accepted `1e-12` test-only checks; this repair adds no solver tolerance.

| ID | Fixture / variation | Required oracle |
|---|---|---|
| FR-01 | `k=100`, `N=10`, `mu=.3`, `F=+/-1`; sticking/sliding seeds; sparse/dense | Converged final state `Sticking`; `u=0`; `R_t=-F`; no final applied sliding force; all seeds and modes have identical physical result. Sliding-seed wrong branch (`u=-/+0.02`, assisting force) is absent. |
| FR-02 | Same, load magnitude one representable `f64` immediately below, exactly at, and immediately above the computed positive `mu*N`; both signs | Below and at: `Sticking`, `u=0`, `R_t=-F`, no final applied force. Above: `Sliding`, nonzero displacement, bounded force opposing motion. Construct neighbors with `f64::to_bits/from_bits`; add no epsilon band. |
| FR-03 | Same, `F=+/-10` | `Sliding`; `u=+/-0.07`; support/applied force `- / +3`; `F_t*u_t=-0.21`; seeds and modes match physically. Both seeds retain the established two-iteration bounded-sliding result. |
| FR-04 | Same, `mu=0`, `N=10`, nonzero `F`, both seeds/modes | Converged released/sliding state; `u=F/k`; branch construction has no applied-force record; product is zero within existing equilibrium evidence; do not assert an exactly zero numerical free-DOF reaction or add a tolerance. |
| FR-05 | Same, explicit `N=0` and negative explicit normal, both seeds/modes | Final `Inactive`; no applied sliding force; free displacement agrees with the linear solve. |
| FR-06 | Existing derived current-normal signed fixture and `derived_normal_sign_flips_retry_before_convergence_and_fail_honestly_at_cap`, both load signs, seeds/modes | Preserve existing rational displacement, current normal and friction-force oracles; `|F_t|=mu*|N_current|` under the affine construction; `F_t*u_t<0`. A normal-branch-only sign mismatch retains `Sliding`, blocks convergence, and retries with the observed current normal sign; the established 3-iteration result remains. |
| FR-07 | Two friction rows with one sub-limit and one super-limit load, plus existing coupled derived-normal rows; swapped support order, both seeds/modes | Final one-row `Sticking`/one-row `Sliding` result is order independent; existing simultaneous derived-normal rational oracles remain stable; all inconsistent rows update in one state vector. |
| FR-08 | Derived normal reaches exact zero; signed `+0/-0`; both modes | Preserve exact-zero branch behavior and absence of a friction load; final contact classification and branch-admissibility evidence remain deterministic. |
| FR-09 | Positive-limit post-force candidate forced to assisting, same-sign, reverse-motion, `u=0`, or roundoff-shaped `|R_t|>L` conditions | Candidate explicitly changes to `Sticking` before residual/convergence calculation. Opposing actual force with nonzero `u` retains sliding. |
| FR-10 | `max_iterations=1` sliding seed, sub-limit sliding seed with cap 2/3, and capped derived branch | Iteration 1 remains the deferred warm-start slide and cannot converge. Valid super-limit seed still converges in 2 iterations. Sub-limit slide is `Sticking` after iteration 2 and converges only on iteration 3; a cap of 2 returns one visible blocking `NonConvergence`. |
| FR-11 | Existing one-way, gap, lift-off, four-class, curved-bend and spring-transfer tests | Byte-unchanged regression suites pass; friction repair does not change non-friction support mechanics. |
| FR-12 | Existing product nonlinear tests | State code, displacement, reaction, normal evidence and diagnostic propagation remain compatible; no product/schema edit. |

## Mutation checks

| Mutation | Killing cases |
|---|---|
| Restore unconditional `prior Sliding && u_t != 0` persistence | FR-01 both signs; produces the known assisting result. |
| Use prior displacement/reaction without checking final force direction | FR-01 and FR-09. |
| Re-stick the explicitly deferred positive-limit/no-force warm-start iterate | FR-03 loses its established two-iteration result. |
| Retain a positive-limit missing-force row after the deferred iterate | FR-09. |
| Use only a convergence boolean without explicitly changing an inconsistent next state | FR-09 roundoff-shaped case repeats the bad branch until cap. |
| Remove the `mu*N==0 && F_t==0` degenerate branch | FR-04 oscillates or fails to converge. |
| Make exact `u_t==0`, `|R_t|==mu*N` classify sliding | FR-02. |
| Use prior rather than current derived normal | FR-06 existing rational oracles and FR-07 coupling/order checks. |
| Re-stick solely on derived signed-normal branch mismatch | FR-06 sign-flip retry changes state/path and breaks the established 3-iteration current-normal result. |
| Accept absent/negative normal as active friction contact | FR-05 and FR-08. |
| Drop final branch consistency from convergence | FR-01/FR-09 and FR-10. |

## Planned validation commands

The implementation child may run focused test filters in an isolated target only after root assigns a build slot. After the diff is frozen and reviewed, the manager requests slots for the complete crate and downstream checks.

```text
cargo test --manifest-path projects/chirality-piping/core/solver/nonlinear_supports/Cargo.toml
cargo test --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml
cargo test --manifest-path projects/chirality-piping/validation/benchmarks/nonlinear/Cargo.toml
cargo test --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml friction_preview
cargo test --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml mixed_nonlinear_preview_bundle_converges_and_emits_each_support_state
```

The project profile also selects `piping-pytest`, `evidence-sweep`, and the always-check `harness-self-check` for a `core/**` diff. Root/CHANGE owns the heavy build/integration schedule; exact registered invocations and results must be recorded at execution time. No command in this matrix is reported as run by F4 preparation.
