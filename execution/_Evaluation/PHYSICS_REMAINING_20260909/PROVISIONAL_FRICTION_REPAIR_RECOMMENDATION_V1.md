# Provisional friction repair recommendation V1

## Status and evidence basis

This is an early, bounded implementation recommendation from EVALUATION P0 so that an independently reviewed repair can start before the remaining physics assessment closes. It is derivative evaluation evidence, not source authority or proof of validation against an external industry solver.

- Accepted source basis: `533332349a4607eee561d4ef90fb05a62d86519e`.
- `core/solver/nonlinear_supports/src/lib.rs`: SHA-256 `f6c6e62994564dddfc84e60df63f2e2f3a293b1d48a5950469c4368509733062`.
- `core/solver/nonlinear_integration/src/lib.rs`: SHA-256 `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46`.
- The reproducer below is an exact analytical/source trace pending the specialist's focused executable return. No runtime execution is claimed in this provisional record.
- Primary engineering basis: Abaqus documents the Coulomb stick/slip condition as `Phi = |Ft| - mu*Fn <= 0`, with sticking below the bound and sliding at the bound, and states that instantaneous slip direction is computed together with stick/slip determination: <https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-connfrictionbehav.htm>. Abaqus' theory documentation also changes reverse sliding to sticking when the slip increment and friction traction are inconsistent: <https://docs.software.vt.edu/abaqusv2025/English/SIMACAETHERefMap/simathe-c-coulombfric.htm>.

## Reproducible defect

For one tangential degree of freedom with stiffness `k = 100 N/m`, external force `F = +1 N`, and Coulomb limit `L = mu*N = 3 N`:

- A sticking seed produces the physically admissible static solution `u = 0`, with the support reaction inside the Coulomb cone.
- A sliding seed first obtains the free displacement `u = +0.01 m`. The next iteration uses that prior displacement to apply `f = -3 N`, and solves `u = (1 - 3)/100 = -0.02 m`.
- The current sliding persistence rule then retains `sliding` whenever displacement is nonzero. The solve can therefore converge with `f*u = (+0.06 J) > 0`: friction assists the final motion.

The defect is seed dependence plus acceptance of a state that violates final static Coulomb consistency. It is separate from the current-normal coupling repaired in PR 760 and does not reopen historical N7.

Current source cause:

- `nonlinear_supports/src/lib.rs:524-537` overrides a newly classified sticking state back to sliding whenever the prior state was sliding and trial displacement is merely nonzero.
- `nonlinear_integration/src/lib.rs:890-1007` selects the sliding direction solely from the previous iteration's displacement or reaction.
- `nonlinear_integration/src/lib.rs:447-460` can declare convergence from active-set residual, an unchanged state, a nondeferred force, and admissible derived-normal branches without verifying the final friction force against final motion.

## Selected engineering behavior

Keep the present solver explicitly **static and path independent**. Treat initial states as warm starts only; they must not define physical friction history. Accept a converged state only when the final solved quantities satisfy these conditions for each friction support:

1. Contact is inactive when the final compressive normal force is absent under the existing normal-force convention.
2. Sticking: tangential relative displacement is zero under the exact constraint formulation, and `|Rt| <= mu*N`.
3. Sliding: `|Ft| = mu*N`, `Ft = -mu*N*sign(ut)`, and therefore `Ft*ut < 0` for nonzero `ut`.
4. If an assumed sliding direction produces zero or reverse final motion, reject that branch. Re-evaluate the exact sticking candidate first; if it is outside the cone, solve the opposite sliding branch and require the same final checks.
5. At the exact Coulomb boundary, choose sticking deterministically. This follows the documented `Phi <= 0` stick admissibility and avoids inventing a new numerical threshold.
6. Preserve PR 760's same-iterate affine coupling when `N` is derived from a current support reaction. Branch acceptance must use that current normal force.

This makes the simple cases seed independent:

- `F = +/-1 N`, `L = 3 N`: `u = 0`, sticking.
- `F = +/-10 N`, `L = 3 N`: `u = +/-0.07 m`, sliding, friction `-/+3 N`.

True load-step friction history is a later capability. When introduced it needs an explicit committed step state and slip increment; active-set iteration history must not silently stand in for physical history.

## Exact implementation fence

The bounded repair should touch only:

- `projects/chirality-piping/core/solver/nonlinear_supports/src/lib.rs`
  - remove or replace unconditional sliding persistence;
  - expose a pure final stick/slide admissibility check if it naturally belongs with state classification;
  - keep existing input validation and contact-normal conventions.
- `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs`
  - determine/verify sliding direction from the candidate's final relative tangential displacement;
  - reject reverse or assisting branches before convergence;
  - preserve the current same-iterate derived-normal solve and diagnostic failure behavior;
  - add focused unit regressions alongside existing friction tests.

No schema, product-physics, application, governed-deliverable, or compatibility migration is needed for this static repair. Do not add a friction-history field or a new convergence tolerance in this tranche.

## Required independent refutation

An independent reviewer should attempt to falsify the repair with a focused matrix:

- sub-limit, exact-limit, and super-limit loads for both signs;
- sticking and sliding seeds for every case, requiring identical accepted physical results;
- explicit and derived normal forces, including PR 760's same-iterate coupling;
- dense-scrutiny and sparse-interactive modes with the same state, direction, and reported reactions;
- a mutation retaining the current persistence override, which the sub-limit sliding-seed case must kill;
- a mutation using prior rather than final displacement sign, which the assisting-friction case must kill;
- complementarity checks on final reported quantities, including `Ft*ut <= 0` and exact-limit deterministic sticking;
- unchanged nonconvergence diagnostics when the iteration cap prevents an admissible branch from being established.

External industry-solver comparison remains a later validation phase and must not be reported as complete by this repair.
