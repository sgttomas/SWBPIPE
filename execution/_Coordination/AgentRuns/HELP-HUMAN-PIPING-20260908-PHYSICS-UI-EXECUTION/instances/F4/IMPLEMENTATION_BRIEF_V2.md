# F4 current-normal friction implementation brief V2

Status: `FROZEN_CANDIDATE_PENDING_ROOT_SOURCE_AMENDMENT`
Supersedes: `IMPLEMENTATION_BRIEF_V1.md`, SHA256 `a6fb3b29a39188fd13e83d9a199ae23f9a876f922dd843551060ff1bd51a103b`; V1 bytes remain unchanged
Clarification scope: R04 exact zero/current-normal branch admissibility only; no source/test write and no new tolerance or convergence policy
Package / deliverable: `PKG-04 / DEL-04-04`
Source basis: `779dedb8670625b36af07b89fc5557470e47c50e`
Accepted mechanics basis: D-35 / DEC-067 bounded `+/- mu*N` opposing motion with the normal reaction from the current iterate
Policy fence: preserve DEC-046 controls, failure honesty, and public result semantics; do not select a load-history, reference, direction, or new numerical-threshold policy

## Reproduced defect and required postcondition

At source lines 427-433, `applied_sliding_friction_forces` is evaluated before the current linear solve. Its derived-normal path reads `previous.reactions` at lines 727-733. The solve/classification at lines 438-466 uses current displacement/reaction, while convergence at line 468 accepts an unchanged active-set state. A stable `Sliding` label can therefore return a force formed from the prior normal reaction.

For every returned converged sliding support, its applied tangential force must be formed from the normal reaction of the same returned linearized iterate:

`f_t = -direction * mu * abs(r_n(current))`.

The existing `sliding_direction(previous displacement, previous reaction)` rule remains unchanged. Explicit caller-supplied normal reactions remain constant inputs. Derived normals alone require coupled solution. A result that cannot establish current-normal consistency within the existing iteration cap returns `converged=false` with the existing `NonConvergence` failure envelope.

## Recommended exact algorithm: active-set iteration with an inner affine coupling solve

The frame problem is linear while the active boundary, sliding directions, and signs of derived normal reactions are fixed. Use that fact to solve the current-normal relation directly inside each outer active-set iteration, without introducing a force residual threshold.

1. Build the active boundary exactly as today. On a first iterate with no solved direction evidence, preserve the existing zero-force behavior and `sliding_force_deferred` gate.
2. For sliding supports with direction evidence, apply explicit-normal forces directly as today: `-direction * mu * abs(N_explicit)`.
3. For each sliding support with a derived normal, bind its tangential DOF, coefficient, existing direction, source reaction DOF, and an assumed signed-normal branch `s` in `{-1, 0, +1}`. Normalize both floating signed zeros to `s=0` by testing `r == 0.0` before any sign operation. Take `s` from the previous solved source reaction; if that reaction is zero, use the step-4 base current source reaction's normalized sign. This branch selection is an internal piecewise-linear solve detail; it does not change the public sliding-direction rule.
4. Solve the active-boundary frame once with base loads plus explicit-normal friction loads and no derived-normal friction loads. Call its reported reaction vector `r0`.
5. For each derived-normal sliding force `q_j`, solve the same frame system with the step-4 base loads plus a unit tangential load at its DOF, convert to the same reported-reaction convention as the final iteration, and subtract the step-4 base reaction. This base subtraction is required when active prescribed displacements or explicit-normal forces create an affine offset. At each derived normal source `i`, the reaction difference gives influence coefficient `H_ij`.
6. With `c_i = direction_i * mu_i * s_i`, solve the small dense system

   `(I + C H) q = -C r0`

   so `q_i = -direction_i * mu_i * s_i * (r0_i + sum_j H_ij q_j)`.

   Reuse `frame_kernel::solve_dense` for this internal coupling matrix. Validate all coefficients and results as finite. A singular coupling system remains an honest frame-kernel error; do not insert a regularization default.
7. Apply explicit and solved derived-normal forces and run the selected final frame solve in the caller's existing `LinearSolveMode`. Preserve that final solve's sparse/dense evidence. Report displacements, reactions, and `AppliedSlidingFrictionForce` through the existing public structures.
8. Check the assumed branch against the final current source reaction `r` by the exact domain of the absolute-value branch. For active friction (`mu > 0` and existing direction `d != 0`): `s=+1` is admissible when `r >= 0`; `s=-1` is admissible when `r <= 0`; `s=0` is admissible only when `r == 0`. Equivalently, a nonzero branch requires `s*r >= 0`, while the zero branch requires exact zero. Both `+0.0` and `-0.0` are zero. Strict equality between `s` and `sign(r)` is not the rule because `r=0` lies in both closed halfspaces.
9. If any active-friction derived-normal row is inadmissible, retain the current active-set result as iteration evidence, select the normalized sign of its final current reaction for the next outer iteration, and defer convergence through the existing loop. If the existing `max_iterations` cap is reached, return `converged=false` with the existing `NonConvergence` failure envelope. Do not add an inner retry budget, magnitude residual, epsilon, or tolerance.
10. When `mu == 0` or the existing direction is zero, the Coulomb row is identically `q=0`; normal-branch choice is irrelevant to current-normal force consistency and must not cause a branch retry. Existing first-iterate direction deferral remains unchanged. This exception does not permit an assumed zero branch with final nonzero reaction when `mu > 0` and `d != 0`.
11. When every active branch is admissible, the affine system enforces the current-normal relation to the linear solver's arithmetic. The fixture tests may reuse the crate's existing `1e-12` comparison scale; it is a verification comparison on an invented exact fixture, not a runtime branch or convergence tolerance.

## Exact branch-admissibility derivation and minimal cases

For a fixed existing direction `d`, coefficient `mu`, and assumed branch `s`, the inner system enforces `q=-d*mu*s*r`. It satisfies the required `q=-d*mu*abs(r)` exactly when `s*r=abs(r)`. This yields closed halfspaces for `s=+/-1` and the singleton zero set for `s=0`.

| Assumed `s` | Final current `r` | Admissible for active friction? | Proof/result |
|---:|---:|---|---|
| `+1` | `r>0` | yes | `s*r=r=abs(r)` |
| `+1` | `r=0` | yes | `s*r=0=abs(r)`; `q=0`; no retry |
| `+1` | `r<0` | no | `s*r<0`; retry with `s=-1` |
| `-1` | `r<0` | yes | `s*r=-r=abs(r)` |
| `-1` | `r=0` | yes | `s*r=0=abs(r)`; `q=0`; no retry |
| `-1` | `r>0` | no | `s*r<0`; retry with `s=+1` |
| `0` | `r=0` | yes | both enforced and required force are zero |
| `0` | `r!=0` | no when `mu>0` and `d!=0` | enforced `q=0` but required magnitude is `mu*abs(r)>0`; retry with normalized `sign(r)` |
| any | any finite `r` | branch-irrelevant when `mu=0` or `d=0` | both enforced and required force are identically zero |

This exact domain check is algebraic validation of the selected piecewise-linear branch. It does not add a convergence residual or acceptance threshold. A wrong nonzero branch, or an active zero branch followed by nonzero current reaction, cannot return converged; it retries under the already existing outer iteration count and fails visibly at the already existing cap. A nonzero assumed branch followed by exact zero is current-normal consistent and must not be rejected.

This formulation supports coupled multiple sliding supports. Every active derived-normal row must be branch-admissible in the same final iterate. Do not implement a sequential per-support update whose result depends on support ordering.

## Literal source surfaces

Primary production/test write, if root issues the versioned amendment:

- `core/solver/nonlinear_integration/src/lib.rs`
  - replace the prior-iterate derived-normal magnitude path in `applied_sliding_friction_forces` with an iteration-level coupled solve helper;
  - preserve `sliding_direction` unchanged;
  - preserve `NonlinearFrameSolveResult`, `NonlinearFrameIteration`, and `AppliedSlidingFrictionForce` field/type semantics;
  - extend the convergence expression now at line 468 only with first-force deferral and the exact closed-halfspace/zero branch validity above; use the existing outer retry/cap and add no tolerance;
  - extend `nonconverged_exit_diagnostic` cause/remediation text only as needed, retaining `SolverDiagnosticCode::NonConvergence`, Failure severity, and SolverIteration source;
  - correct the assembled-loop assumption/limitation text so it truthfully describes a same-iterate coupled current normal.

No write is justified in:

- `core/solver/nonlinear_supports/src/lib.rs`: the DEC-067 classifier and existing active-set count residual stay unchanged.
- `core/solver/frame_kernel/src/lib.rs`: existing assemble/reduce/solve interfaces are sufficient.
- `core/solver/diagnostics/src/lib.rs`: the existing diagnostic vocabulary represents capped failure.
- `core/units/src/lib.rs`: internal solver values remain canonical SI.

Conditional product test-only write, only if named by root's amendment:

- `core/product_physics/src/lib.rs`: add a boundary regression proving normalized product inputs reach the same corrected current-normal result while preserving selected-state/public result mapping. Do not change conversion constants, DTOs, checksums, diagnostic mappings, or production mechanics.

## Required focused verification

Before editing, preserve the unchanged M1-N-008 source hashes and raw baseline replay. After editing, rerun its exact source for both `Sticking`/`Sliding` seeds and `SparseInteractive`/`DenseScrutiny` modes. It must converge to the independently frozen positive branch:

- `u_x = 7/135`
- `N = 200/27`
- `R_f = -20/9`
- `R_f = -mu*N` from the same returned iterate.

Add a source-local mirror for negative applied tangential force, both seeds/modes:

- `u_x = -7/165`
- `N = 400/33`
- `R_f = 40/11`
- `R_f = +mu*N` from the same returned iterate.

Retain and run the existing explicit-normal, first-iteration-cap, general iteration-cap, TBD-policy, and sparse/dense parity tests. Add focused exact cases for assumed `+1` and `-1` with final zero (admissible without retry), assumed zero with final zero (admissible), assumed zero with final nonzero under active friction (retry, never false convergence), each nonzero sign flip (retry), `mu=0` with arbitrary finite normal (branch-irrelevant zero force), cap failure while a branch remains inadmissible, and two coupled derived-normal sliding supports demonstrating order independence. Any threshold-touch/reference/history case stays out unless separately adopted.

Run only the software-profile checks mapped by the final changed paths plus the always-run harness self-check. Fresh independent 100% frozen-diff review and the clean-source DEC-025 five-surface sweep are root-coordinated release gates, not F4 self-approval.

## Code-derived gate alternatives

| Option | Interaction with current code | Decision status |
|---|---|---|
| Inner affine coupling solve plus exact closed-halfspace/zero branch validation | Uses the existing linear frame response and outer active-set retry/cap; leaves `ConvergenceControl` as the state-change-count policy; no new quantitative threshold or criterion | Recommended for the selected narrow repair |
| Fixed-point predictor/corrector using prior/current `mu*N` | Needs a new force-consistency residual, an adopted tolerance/scale, and likely cap semantics because unchanged state labels do not certify the fixed point | Held; requires a separate convergence decision |
| Keep state-label convergence and only recompute the reported force | Leaves displacement/equilibrium inconsistent with the reported force | Rejected as incorrect |
| Add committed slip/load-step history or a new static reference | Changes mechanics and public state semantics beyond D-35's grant | Held for a future ruling |

## Acceptance and stop conditions

Accept the candidate only if the original and sign-extension fixtures match the frozen values in both modes and seeds, existing result/diagnostic semantics remain intact, focused mapped checks pass, and independent review finds no current/prior iterate mismatch. Stop and return evidence without a source claim if the inner coupling is singular, sign branches cycle to the existing cap, a public DTO/residual family must change, or any new tolerance/reference/history/direction choice becomes necessary.
