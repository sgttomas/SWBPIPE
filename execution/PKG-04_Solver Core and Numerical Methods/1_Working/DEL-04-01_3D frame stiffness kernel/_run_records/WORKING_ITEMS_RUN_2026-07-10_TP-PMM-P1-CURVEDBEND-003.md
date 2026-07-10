# WORKING_ITEMS Run Record - TP-PMM-P1-CURVEDBEND-003

Date: 2026-07-10
Agent: WORKING_ITEMS (claude worker; owner-adopted tranche, branch
`claude/piping-p2-nonlinear-repair`)
Deliverable: DEL-04-01 - 3D frame stiffness kernel
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-PMM-P1-CURVEDBEND-003
Target stage: R5 / physical-model mechanics program P1 residual closure
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; rulings `DEC-067`
authority via D-35, residual fenced by `DEC-070`)

## Scope

Close the named P1 residual from TP-PMM-P1-CURVEDBEND-002: give the
assembled nonlinear active-set loop an explicit curved-bend macro-element
stiffness slot so the loop assembles the DEC-070 arc stiffness, then retire
the blocking `CURVED_BEND_NONLINEAR_LOOP_UNSUPPORTED` branch in
`core/product_physics` and wire realized bends into the nonlinear loop input.

## Files Touched

- `core/solver/nonlinear_integration/{Cargo.toml, Cargo.lock, README.md, src/lib.rs}`
- `core/product_physics/{Cargo.lock, src/lib.rs}`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/MEMORY.md`

## Implemented Evidence

- New `CurvedBendStiffnessElement` slot on `NonlinearFrameSolveInput`:
  element id, node indices, and the validated 12x12 global arc stiffness,
  constructible directly from the `open_pipe_stress_curved_bend`
  `CurvedBendMacroElement` (`from_macro_element`) or from the build-time
  validated matrix. Finiteness and node-index validation; no straight-chord
  fallback and no derived defaults.
- `solve_active_set_frame_with_mode` scatter-adds the curved-bend slots
  beside frame and user-stiffness assembly once per solve; the DEC-053
  sparse evidence lane consumes the same assembled matrix
  (`direct_reduced_profile_entries`), so dense/sparse parity holds on the
  arc path without a second assembly convention.
- `core/product_physics::append_nonlinear_support_loop_results` passes the
  build-time `CurvedBendMacroBuild` matrices into the loop input; the
  blocking `CURVED_BEND_NONLINEAR_LOOP_UNSUPPORTED` recorded residual is
  retired (grep-clean outside historical run records).
- Tests: crate-level
  `curved_bend_slot_matches_linear_arc_path_when_support_stays_released`
  (nonlinear loop with a curved bend + released one-way support equals the
  direct frame-kernel reduction/solve of the same arc stiffness, delta
  <= 1.0e-12) and product-level
  `curved_bend_macro_element_solves_assembled_nonlinear_loop` (preview model
  with a realized bend + inactive one-way support converges; nonlinear-loop
  tip displacement matches the linear curved-bend row and the direct
  macro-element oracle within 1.0e-6 mm).

## Checks

- `cargo test` `core/solver/nonlinear_integration`: 14 passed.
- `cargo test` `core/product_physics`: 56 passed.
- `cargo test` `validation/benchmarks/nonlinear`: 19 passed;
  `validation/benchmarks/mechanics`: 27 passed (unchanged crate, sanity).
- `python3 -m pytest -q tests`: 369 passed.
- `cargo fmt --check` clean on touched crates.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` sweep recorded with this tranche set (see companion DEL-04-04
  run records for the shared sweep summary).

## Boundaries And Residuals

- Arc interior-station results and consistent distributed-load end moments
  for macro spans remain recorded residuals with diagnostics (unchanged by
  this tranche).
- The slot consumes caller-supplied stiffness only; pressure-thrust load
  generation, vendor defaults, and compliance checks remain outside the
  loop.
- No lifecycle transition, no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.
