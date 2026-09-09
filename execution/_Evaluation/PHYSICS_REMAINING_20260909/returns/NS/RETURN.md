# NS Remaining-Physics Return

## Execution identity

- Role: bounded ephemeral Agent 2 generalist; no delegation performed.
- Requested and exposed configuration: `gpt-5.6-sol`, `high`.
- Source basis: `533332349a4607eee561d4ef90fb05a62d86519e`.
- Scope: nonlinear supports, active-set integration, dense/sparse linear-policy interaction, and their product-facing diagnostics.
- Authority boundary: the equations below are small verification oracles derived directly from the implemented mechanics. No piping-design extracted equation was used as authority, and no industry-solver validation is claimed.

## Executive finding

The most consequential remaining defect is a seed-dependent static Coulomb result. A focused current-SHA Rust probe now confirms it through the public nonlinear-integration API. For a one-DOF tangential system with stiffness `k = 100 N/m`, external force `F = +1 N`, and friction limit `mu*N = 3 N`, a sticking seed converges to `u = 0`, `T = -1 N`, while a sliding seed converges to `u = -0.02 m`, `T = -3 N`. In the latter result, `T*u = +0.06 J`: the reported friction force assists the final displacement. The mechanism is current-source-visible: the integration loop chooses sliding direction from the previous iterate, while the classifier preserves a prior sliding state whenever the trial displacement is nonzero. The convergence gate does not check friction direction against the final displacement.

This defect is independent of the same-iterate derived-normal repair already present in the source. That repair should be retained.

Three further gaps remain: an initially inactive contact can make the first reduced system singular before an admissible active state is tried; both dense and sparse paths use an absolute `1e-12` pivot threshold that rejects a condition-one scalar problem; and nonpositive sparse pivots are recorded but are not routed into a blocking product diagnostic.

## Findings and selected dispositions

### NS-01 — Static Coulomb solve can be seed-dependent and non-dissipative

**Classification:** reproduced defect. **Severity:** high.

The current path obtains sliding direction from the previous displacement, falling back to previous reaction (`core/solver/nonlinear_integration/src/lib.rs:996-1007`). It then constructs the sliding candidate (`:890-950`). If the next classifier sees a force inside the cone but a nonzero trial displacement, a prior sliding state is forced to remain sliding (`core/solver/nonlinear_supports/src/lib.rs:524-537`). Convergence requires an unchanged active set and branch admissibility (`core/solver/nonlinear_integration/src/lib.rs:457-460`), but there is no final check that the friction force opposes the displacement used by this static, no-history slice.

The deterministic scalar replay is:

```text
k = 100, F = +1, mu*N = 3
sticking seed: u = 0, T = -1, |T| < 3
sliding seed:  u(1) = +0.01; choose T = -3 from sign(u(1));
               u(2) = (1 - 3)/100 = -0.02; T*u(2) = +0.06
```

The exact command and output are preserved in `RUNTIME_PROBE.md`. The current executable reports:

```text
sticking seed: converged=true, iterations=1, state=sticking, u=0, T=-1
sliding seed:  converged=true, iterations=2, state=sliding,  u=-0.02, T=-3, T*u=+0.06
```

Prior dense and sparse executable evidence records the same divergence in `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/M1/M1-N/OBSERVATIONS.json:418-444`. The new probe establishes the defect directly at the declared source SHA.

**Selected repair:** make the current no-history static solver use a deterministic stick-first Coulomb active-set rule.

1. Treat the supplied friction state as a numerical seed only.
2. Evaluate the sticking trial first. If `|T_trial| <= mu*N`, accept sticking.
3. When the trial exceeds the cone, derive impending-slip direction from the sticking trial reaction and solve on the Coulomb bound. Retain the current same-iterate affine coupling for derived normal force.
4. Require Coulomb complementarity and `f_t * delta_u_t <= 0` at the accepted iterate. In the current static slice, `delta_u_t` is measured from the zero/reference configuration. If the bounded solve reverses direction or returns inside the cone, re-stick and recompute; do not preserve sliding solely because displacement is nonzero.
5. Treat history-dependent or incremental friction as a separate future capability requiring explicit state/schema semantics.

This deliberately changes sliding-seeded, sub-limit models to the unique sticking solution. That is the intended correction for a solver that currently declares no friction history.

The rule is also consistent with the reference Coulomb algorithm described in the [Abaqus friction theory documentation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAETHERefMap/simathe-c-coulombfric.htm) (accessed 2026-09-09): below the critical shear stress there is no relative motion, and a reversed incremental slip direction returns the state to sticking. This is a design comparison, not an external validation result.

**Acceptance fence:** exact scalar cases for both force signs, below/equal/above the cone, both initial states, and dense/sparse solve modes; `f_t*delta_u_t <= 0`; `mu = 0`; `N = 0`; derived current normal; simultaneous multiple-friction coupling; the existing four-class nonlinear suite; and product-adapter propagation of the selected state.

### NS-02 — Singular inactive contact seed can prevent discovery of an admissible state

**Classification:** bounded capability defect. **Severity:** medium.

The API requires an initial nonlinear state (`core/solver/nonlinear_integration/src/lib.rs:1010-1050`; `core/product_physics/src/lib.rs:3070-3097`). The integration loop forms constraints from that state and attempts the linear solve before classification (`core/solver/nonlinear_integration/src/lib.rs:413-455`). A singular solve therefore exits before the classifier can engage an inactive support.

A two-node axial example is exact: with `k = 100 N/m`, `F = +10 N`, and an initially inactive one-way support at node 0, the unconstrained reduced matrix is `[[100,-100],[-100,100]]` with determinant zero. The admissible active-contact solve has `u_1 = 0.1 m` and support reaction `R_0 = -10 N`. The active seed solves; the inactive seed errors before classification. Prior executable evidence is preserved in `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/M1/M1-N/OBSERVATIONS.json` and its M1-N return.

**Selected repair:** on a singular first trial containing inactive frictionless unilateral supports, perform one deterministic all-contact recovery trial. Reclassify that result using the existing force and gap inequalities and continue the ordinary active-set loop. Bound the advertised capability to models for which this all-active recovery trial is algebraically admissible. If it is singular or inconsistent, preserve the structured block; do not add artificial stiffness or imply general subset enumeration.

This preserves existing nonsingular seeded behavior and adds deterministic recovery for contact-stabilized models. It does not claim to solve every combinatorial contact-feasibility problem.

**Acceptance fence:** one-contact load in each direction with active/inactive seeds and both solve modes; a no-admissible-support case that releases and blocks honestly; gap-clearance compatibility; multiple disjoint supports; repeated/contradictory constrained DOFs; and a product diagnostic that identifies use or failure of the recovery trial.

### NS-03 — Absolute pivot guard rejects well-conditioned scaled systems

**Classification:** numerical-policy defect. **Severity:** medium.

Both paths use an absolute `1e-12` pivot threshold (`core/solver/sparse_direct/src/lib.rs:17-21,418-423`; `core/solver/frame_kernel/src/lib.rs:21,935-936,965-966`). For the one-equation system `k = f = 2^-40 = 9.094947017729282e-13`, the exact solution is `x = 1` and the condition number is one, yet both guards reject the pivot. Sparse-to-dense fallback cannot repair this because the dense path applies the same absolute policy (`core/solver/nonlinear_integration/src/lib.rs:1262-1300`). Prior executable evidence also records the scaling defect in `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/M1/M1-L/raw_v2.json` and finding M1-L-004.

**Selected repair:** use symmetric diagonal equilibration for the expected elastic reduced stiffness, perform a positive-definite factorization on the equilibrated system, recover physical units, and evaluate an original-system componentwise backward residual. Use a machine-precision-relative arithmetic guard after equilibration. Keep engineering acceptance tolerances separate from factorization breakdown. A nonpositive pivot blocks a mechanics-solved result; a condition estimate can support a warning without becoming an arbitrary universal acceptance threshold.

This choice is narrower and more deterministic than adding a general rank-revealing indefinite backend. The [LAPACK DPOSVX documentation](https://www.netlib.org/lapack/explore-html/d6/d44/group__posvx_gae23dac18e7ec69c36fd2a2648a470dc3.html) (accessed 2026-09-09) provides a credible reference pattern combining equilibration, positive-definite factorization, condition estimation, refinement, and backward-error reporting. It is a reference design, not evidence that the implementation has passed LAPACK or industry-solver comparisons.

**Acceptance fence:** scalar scaling invariance over a declared binary-exponent range; diagonally heterogeneous SPD systems; exactly singular and indefinite systems; mixed translational/rotational frame models; dense/sparse agreement; and original-system backward-residual checks.

### NS-04 — Nonpositive sparse pivots do not become blocking product diagnostics

**Classification:** diagnostic nonconformance. **Severity:** medium.

The sparse factorization records nonpositive pivots (`core/solver/sparse_direct/src/lib.rs:424-454`). A conversion helper exists (`core/solver/diagnostics/src/lib.rs:608-634`), but no production caller routes it into nonlinear or product diagnostics. Nonlinear integration stores the count only as sparse-solve evidence (`core/solver/nonlinear_integration/src/lib.rs:1372-1394`), and product physics exposes solver metadata without converting this condition into a blocking result (`core/product_physics/src/lib.rs:2219-2240,2277-2320`). The currently validated product input classes make practical reachability narrower, but the contract gap remains.

**Selected repair:** implement this with NS-03. For the declared elastic-stiffness class, positive-definite factorization failure must block `MECHANICS_SOLVED`. If LDLT remains available internally, route every nonpositive pivot into a blocking mechanics diagnostic at the solve boundary. Do not rely on metadata inspection by downstream consumers.

**Acceptance fence:** negative scalar stiffness and a two-by-two indefinite matrix on direct sparse and dense/fallback routes; assert that product status cannot be `MECHANICS_SOLVED`; retain structured pivot index/value evidence.

## Reviewed repairs not reopened

- The same-iterate current-normal friction coupling is implemented in `core/solver/nonlinear_integration/src/lib.rs:700-887` and is exercised by focused source tests. The review found no new defect in that repair.
- Inclusive bearing at exact contact is present in `core/solver/nonlinear_supports/src/lib.rs:575-606`, addressing the earlier exact-gap chatter path.
- Zero-iteration, nonfinite-trial, and repeated-support guards are present in the current source, including `core/solver/nonlinear_integration/src/lib.rs:577-628`.
- Spring transfer now reaches the nonlinear solve through `solve_active_set_frame_with_mode_and_springs` (`core/solver/nonlinear_integration/src/lib.rs:379-405`) and the product call site (`core/product_physics/src/lib.rs:1913-1920`).

These are source-trace conclusions at the declared SHA. The only new executable work was the focused NS-01 evaluation-local runner documented in `RUNTIME_PROBE.md`; no repository-wide build or broader suite was run.

## Prioritized implementation and verification plan

1. **P0 — NS-01:** replace prior-iterate sliding persistence with stick-first, same-iterate direction/complementarity checks. Add the exact scalar oracle before changing the implementation, then run the focused nonlinear-support/integration/product tests in both solve modes.
2. **P1 — NS-02:** add the bounded first-iteration all-contact recovery and structured recovery diagnostics. Verify contact-only stabilization and honest failure outside the supported recovery class.
3. **P1 — NS-03/NS-04 together:** adopt equilibrated positive-definite factorization policy, original-unit residual reporting, and blocking nonpositive-pivot propagation. Verify scaling, singularity, indefiniteness, mixed units, and dense/sparse agreement.
4. Run an independent review of each physics-affecting change before acceptance, as required by the project doctrine. External solver comparison, if later desired, is a separate validation tranche and must report its actual cases and tolerances.

## Remaining uncertainty

- The focused runner establishes NS-01 through the current nonlinear-integration crate, but it does not execute the product adapter. Product-level propagation remains in the repair acceptance fence.
- The all-active contact recovery intentionally covers a bounded class; general multi-contact feasible-set discovery remains unclaimed.
- A practical engineering conditioning envelope and warning policy still require project-level units and model-scale decisions after the arithmetic guard is repaired.
- No comparison against an industry solver was performed.
