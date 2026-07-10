# WORKING_ITEMS Run Record - TP-PMM-P1-CURVEDBEND-001

Date: 2026-07-10
Agent: WORKING_ITEMS (orchestrated fable subagents; owner-directed session)
Deliverable: DEL-04-01 - 3D frame stiffness kernel
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-PMM-P1-CURVEDBEND-001
Target stage: R5 / physical-model mechanics program P1 sub-tranche 1
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-070`,
packet `execution/_Coordination/_DECISIONS/D-34_bend_flexibility_stiffness_realization.md` §7)

## Scope

First bounded slice of the D-34 O-B curved-bend macro-element realization:
the element formulation crate and the independent hand-calculated
expansion-loop reference. No integration into `core/product_physics`, no
schema change, no provenance change — those are the P1 assembly/recovery
and benchmark-wiring sub-tranches.

## Files Touched

- `core/solver/curved_bend/` (new crate `open_pipe_stress_curved_bend`:
  `Cargo.toml`, `README.md`, `src/lib.rs`, `.gitignore`)
- `validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md`
  (new draft witness; solver comparison values TBD until benchmark wiring)

## Implemented Evidence

- `CurvedBendMacroElement`: two-node circular-arc macro-element,
  center-parameterized with degenerate-geometry rejection; exact closed-form
  unit-load (Castigliano) end-flexibility integration over the arc (trig
  Gram contraction, no quadrature) covering in-plane bending, out-of-plane
  bending with arc bending-torsion coupling, torsion, and axial terms;
  user-entered `k_in`/`k_out` multiply only the bending strain-energy
  products (torsion and axial untouched, per the D-34 §7 realization);
  `K_jj = F^-1` via the kernel dense solve; full 12x12 by rigid equilibrium
  transfer; global transform via the kernel orientation conventions.
  Zero new dependencies (`DEC-023` posture); `frame_kernel` unmodified.
- In-crate tests (11): straight-limit convergence to the kernel straight
  element; quarter-circle in-plane and out-of-plane closed-form tip seeds
  at 1.0e-9 relative (independent longhand expectations through the
  assembled 12x12 + kernel reduce/solve path); exact k-scaling separation
  (affine in `k` at 1e-12, untouched blocks at 1e-15); rigid-body
  nullspace; symmetry/energy nonnegativity; frame invariance under
  composed rotation; constructor rejections; geometry accessors.
- Witness `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL`: plane anchor-to-anchor
  L-loop with quarter-circle elbow, force-method (bending-only flexibility,
  full axial thermal driving) closed-form coefficients with arc terms
  carrying `k`; reactions/displacements tabulated for k in {1, 5, 10, 20};
  governing anchor moment strictly monotonically decreasing in k (D-34
  k-sweep evidence shape); verified by antiderivative differentiation,
  200k-point quadrature cross-check (max 4.2e-12 relative), independent
  800-element displacement-method model (5.4e-6 relative), and equilibrium
  residuals at round-off. Invented fixture data only; no code content.

## Checks

- `cargo test` `core/solver/curved_bend`: 11 passed / 0 failed.
- `cargo test` `core/solver/frame_kernel`: 36 passed / 0 failed (crate
  untouched).
- `cargo fmt --check` clean on the new crate.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` five-surface sweep on the clean head: see the commit-bound
  summary under `validation/evidence/sweeps/` recorded with this tranche.

## Boundaries And Residuals

- The witness's 1.0e-9-class solver comparison presumes a bending-only
  comparison configuration (boosted axial rigidity ~1e3, not extreme, per
  the witness appendix conditioning note); the wiring tranche must choose
  and record the configuration (real-EA comparison cannot beat ~6.0e-3
  relative at k=1).
- Constructor geometry tolerances (radius match 1e-9 relative; included
  angle window (1e-9, pi-1e-9)) to be aligned with the DEC-024/026 policy
  at integration time if promoted.
- Interface notes for the assembly sub-tranche: product-side assembly can
  reuse public `element_dof_map`; `second_moment` is a single scalar
  (circular-pipe scope) mapped from `FrameSection` at integration time.
- No lifecycle transition, no release-readiness, professional,
  certification, or code-compliance claim.
