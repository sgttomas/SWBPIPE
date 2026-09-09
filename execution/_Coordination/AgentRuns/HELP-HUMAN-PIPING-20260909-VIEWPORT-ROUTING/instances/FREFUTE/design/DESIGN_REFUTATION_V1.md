# Friction design refutation V1

## Identity and verdict

- Role: fresh bounded Agent 2 independent engineering reviewer; parent `/root`; no delegation.
- Requested and exposed configuration: `gpt-5.6-sol`, high reasoning.
- Source basis: `533332349a4607eee561d4ef90fb05a62d86519e` (observed `HEAD` match).
- Reviewed proposal: `PROVISIONAL_FRICTION_REPAIR_RECOMMENDATION_V1.md`, SHA-256 `e24d93d740fedd32a2caedfa5d99d9626a496a9714d91372ddaa0b0cdd4b80a8`.
- Verdict on the provisional proposal: **CHANGES_REQUIRED**. The static, path-independent, final-Coulomb-admissibility model is defensible, and the two-file product-source fence is sufficient, subject to the corrections and algorithm conditions below.
- Source effect: not authorized by this review. External industry-solver validation remains future work and is not claimed.

## Exact reviewed inventory

| Surface | SHA-256 / basis | Review use |
|---|---|---|
| `AGENTS.md` | `377a93c13dc8e727c2fb38b6ace5c0dd62833fff3ec50753ebe58d57937a9fc3` | root runtime doctrine |
| `projects/chirality-piping/AGENTS.md` | `33f5e5f01ac80971e853094b72a3297c9002e8537593292648b34e5eb90d4d5b` | project instructions |
| `SHARED_LAUNCH_CONTRACT_V1.md` | `c65fd8e058ae104364360fdd2990351d06d67c8801288955b98adf0726d70c42` | shared constraints |
| `OWNER_ENGINEERING_DIRECTION_V2.md` | `34f63986512e53b3e574563a61e4411b14bb27815cd30c18b3d5907dacd32c18` | engineering-selection authority |
| `FRICTION_DESIGN_REFUTATION_LAUNCH_BRIEF_V1.md` | `23911596636d7b4362b8e234bf2de0dc0b99d4a6ea404ffd6cb3b2a77f18355d` | sealed brief |
| `WORK_GRAPH_FRICTION_REVIEW_AMENDMENT_V4.json` | `80d59ada63a81d63c01551d39c0269a63c68890887b1b8ed6aace089bd13b388` | accepted graph amendment |
| `PROVISIONAL_FRICTION_REPAIR_RECOMMENDATION_V1.md` | `e24d93d740fedd32a2caedfa5d99d9626a496a9714d91372ddaa0b0cdd4b80a8` | proposal under refutation |
| P0 `returns/NS/RETURN.md` | `d0ad55de23da0f0ca7965acfb4dc6e7e5f17b472203199a6754a0cb8b3ddc515` | corroborating source trace and exact replay; no current-SHA build claimed |
| `core/solver/nonlinear_supports/src/lib.rs` | `f6c6e62994564dddfc84e60df63f2e2f3a293b1d48a5950469c4368509733062` | classifier, persistence rule, focused tests |
| `core/solver/nonlinear_integration/src/lib.rs` | `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46` | assembled loop, force/reaction convention, current-normal coupling, diagnostics, focused tests |

The official Abaqus 2025 connector-friction and Coulomb-friction theory pages were read as primary reference comparisons. They support the Coulomb cone, zero-friction no-effect behavior, exact sticking as a constraint option, and return from slip to stick when slip and traction are inconsistent. They do not constitute validation of this implementation.

## Source-visible defect and sign trace

The defect is confirmed. `nonlinear_supports/src/lib.rs:524-537` changes a friction row classified `Sticking` back to `Sliding` whenever its prior state is `Sliding` and displacement is merely nonzero. `nonlinear_integration/src/lib.rs:890-1008` selects sliding direction from the previous iterate, and `:457-460` can accept an unchanged state without checking the applied support force against final displacement.

The source sign convention is coherent when used correctly. `solve_linearized_system` forms structural reactions as `K u - P_total` (`nonlinear_integration/src/lib.rs:1236-1240`). At a released sliding DOF this residual is zero; `reported_reactions` then adds the applied friction load (`:959-970`), so the reported tangential reaction is the friction support force on the structure. Thus:

- `F=+10`, `L=3`: `u=+0.07`, `F_t=-3`;
- `F=-10`, `L=3`: `u=-0.07`, `F_t=+3`;
- `F=+1`, sticking: `u=0`, `R_t=-1`;
- `F=-1`, sticking: `u=0`, `R_t=+1`.

The current mutation path for `k=100`, `F=+1`, `L=3` is exactly `u_1=+0.01`, `F_t=-3`, `u_2=-0.02`, giving `F_t u_2=+0.06`. Friction assists final motion and the result depends on the seed.

## Required corrections to the selected model

1. Replace the proposal's unconditional strict inequality `F_t u_t < 0`. General scalar Coulomb admissibility is `F_t u_t <= 0`. Require strict opposition only when `L=mu N>0` and `u_t!=0`. With `mu=0`, `N>0`, and `u_t!=0`, the existing contract has a frictionless released/sliding row with `F_t=0` and product zero. With explicit `N<=0`, retain the existing inactive/no-contact classification and no tangential force.
2. Keep exact-boundary sticking as a deterministic application convention when the exact constrained candidate has `u_t=0` and `|R_t|<=L`. Do not attribute that tie-break to Abaqus as an equality-is-stick rule: the cited connector page states stick for `Phi<0` and sliding at `Phi=0`, although the admissible cone is inclusive and slip can occur at equality. The app's tie-break is defensible for a no-history static solve but must be identified as its own deterministic convention.
3. Treat initial states only as numerical warm starts. They may change iteration count, but a converged result must pass the same final branch checks for all seeds. Do not carry iteration displacement or state as physical slip history.
4. Preserve same-iterate current-normal coupling. For derived normals, branch force magnitude must continue to come from the simultaneous affine solve in `nonlinear_integration/src/lib.rs:700-887`; final acceptance must use the same candidate's current reported normal reaction and must retain the exact signed-normal branch check.
5. Do not claim universal uniqueness for arbitrary derived-normal multi-contact systems. The `N=|R_n|` dependence is piecewise and solution-dependent; final admissibility is necessary, but multiple admissible equilibria or active-set nonconvergence can exist. Require deterministic ordering and seed/mode/order agreement on the declared fixtures, and preserve an honest `NonConvergence` result at the cap outside that demonstrated class.

## Corrected bounded algorithm

The smallest compatible repair can retain the current state enum and public inputs.

1. Solve an exact sticking candidate for a friction row when it is sticking or when a sliding candidate is inconsistent. The constrained DOF is exactly zero. With current normal `N`, accept `Sticking` when contact is active and `|R_t|<=mu N`, including exact equality.
2. If the sticking reaction is outside the cone, choose impending-motion direction as `-sign(R_t)` under the existing reaction convention, release the DOF, and apply `F_t=-mu N sign(u_assumed)`. For a derived normal, use the existing simultaneous same-iterate affine coupling and signed-normal branch validation.
3. A positive-limit sliding candidate is admissible only if final `u_t!=0`, the applied branch force is at the current bound by construction, and the reported support-force direction opposes final displacement. If it returns `u_t=0`, reverses motion, assists motion, or loses contact, do not accept convergence. Re-enter the exact sticking candidate first; if that candidate lies outside the cone, the following released solve derives the opposite direction from the sticking reaction.
4. A zero-limit contacted row with nonzero displacement is frictionless released/sliding with `F_t=0`. An explicit noncompressive normal row is inactive. These cases must never receive an absolute-valued explicit friction load on an accepted iterate.
5. For simultaneous rows, update all inconsistent rows as one active-set state vector and solve again. Keep stable input/support-ID ordering. This is a bounded active-set method, not proof that every combinatorial contact problem has one equilibrium. If no admissible stable vector is reached within the existing cap, return the existing visible failure diagnostic.

Implementation should avoid a new force or displacement tolerance. State selection uses exact zero, sign, and inclusive cone comparisons. Sliding-force magnitude is guaranteed by the branch construction, while existing linear-solve residual evidence continues to report numerical equilibrium quality; do not introduce a separate hidden friction threshold to compare floating results.

## Smallest coherent fence

The product-source and focused-regression fence is exactly:

- `projects/chirality-piping/core/solver/nonlinear_supports/src/lib.rs`: replace the displacement-only persistence override with direction-aware final classification; correct assumption text; replace the persistence unit test with valid-slide, assisting-slide, zero-limit, exact-limit, and no-contact classifier cases.
- `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs`: gate convergence on final branch consistency; re-stick inconsistent branches before choosing the opposite direction; preserve same-iterate derived-normal coupling and cap diagnostics; add the scalar, mode, seed, multi-row, and mutation-killing regressions.

No public schema, state variant, history field, product-physics adapter, or compatibility migration is required. Existing benchmark/hand-calculation prose that names the old unconditional anti-chatter rule is derivative evidence that becomes stale after source effect and should be regenerated or explicitly deferred at closure; it need not expand the product-source repair fence.

## Oracle and mutation matrix

| Case | Required final result | Mutation killed |
|---|---|---|
| `k=100`, `L=3`, `F=+/-1`, seeds Stick/Slide, both modes | `u=0`, `R_t=-F`, Stick | old displacement-only persistence |
| same, `F=+/-3` | `u=0`, `R_t=-F`, Stick | exact-boundary `<` or slide tie-break |
| same, `F=+/-10` | `u=+/-0.07`, `F_t=-/+3`, Slide, product negative | wrong force/reaction sign |
| one ULP below / at / above `L`, both signs | below and at Stick; above Slide; no epsilon band | hidden tolerance or wrong equality comparator |
| `mu=0`, `N>0`, `F=+/-1` | `u=+/-0.01`, `F_t=0`, released/sliding, product zero | erroneous strict-product gate |
| explicit `N=0` and negative-normal no-contact cases | inactive, no friction force, free response | unconditional `abs(N)` force on explicit no-contact |
| known assisting branch `F=+1`, assumed `F_t=-3` | reject `u=-0.02`; re-stick to `u=0`, `R_t=-1` | prior-displacement direction without final check |
| derived-normal PR-760 fixture, both load signs/seeds/modes | existing `u`, current `N`, and `F_t` oracles unchanged | lagged-normal reintroduction |
| two scalar friction rows with one sub-limit and one super-limit; swapped input order | one Stick, one Slide; same physical result | per-row/order-dependent retry |
| positive-limit candidate with `F_t u_t>=0` | cannot converge | state-only convergence gate |
| `max_iterations=1` sliding seed and capped derived branch | `converged=false`, visible `NonConvergence` failure | silent capped exit |

Also retain the existing gap, lift-off, one-way, derived-normal signed-branch, zero-coefficient, sparse/dense, and invalid-input tests. A focused test of the two crates is needed after implementation; no repository build or full sweep was run in this review.

## Conditional release criteria

The design becomes acceptable for source effect only when the F4 plan binds the corrected zero-limit rule, app-owned equality tie-break, direction-aware final acceptance, simultaneous-row retry semantics, preserved current-normal branch solve, bounded nonuniqueness statement, exact two-file fence, and the oracle/mutation matrix above. The future implementation still requires fresh 100% frozen-diff review and the project validation gates before publish.

## Final reconciliation

F4 revised its plan and matrix to incorporate every condition above, including the explicit next-state override before residual/diagnostic formation, the first-iterate deferred warm-start exception needed to preserve the existing two-iteration super-limit fixture, adjacent-float boundary cases, zero-limit branch evidence without an exact free-DOF residual assertion, and a separate PR-760 derived-normal sign-branch retry. The reconciled F4 design verdict is **PASS**; see `F4_RECONCILIATION_V1.md`. This does not change the **CHANGES_REQUIRED** verdict on the uncorrected provisional proposal and does not authorize source effect.
