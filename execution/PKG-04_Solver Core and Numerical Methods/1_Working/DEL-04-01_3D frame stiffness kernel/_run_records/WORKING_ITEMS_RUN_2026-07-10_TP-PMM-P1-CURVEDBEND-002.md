# WORKING_ITEMS Run Record - TP-PMM-P1-CURVEDBEND-002

Date: 2026-07-10
Agent: WORKING_ITEMS (orchestrated fable subagents; owner-directed session)
Deliverable: DEL-04-01 - 3D frame stiffness kernel
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-PMM-P1-CURVEDBEND-002
Target stage: R5 / physical-model mechanics program P1 sub-tranches 2-3
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-070`)

## Scope

Second and third bounded slices of the D-34 O-B realization, following
TP-PMM-P1-CURVEDBEND-001 (formulation crate + witness): (a) product
consumption of the curved-bend macro-element in `core/product_physics`;
(b) the D-34 §5 expansion-loop benchmark wired into the mechanics
benchmark crate against the hand-calculated witness, with fixture
inventory registration.

## Files Touched

- `core/product_physics/{Cargo.toml, Cargo.lock, src/lib.rs, src/validation.rs}`
- `schemas/component.schema.yaml` (one enum value:
  `curved_bend_macro_element` in `ComponentMechanicsInterface.solver_consumption`)
- `validation/benchmarks/mechanics/{Cargo.toml, Cargo.lock, README.md, src/lib.rs}`
- `validation/hand_calcs/mechanics/README.md` (fixture-inventory row)

## Implemented Evidence

- Product realization mode `curved_bend_macro_element` (DEC-070): bends
  with a user flexibility factor, user bend radius, and a `bend_pipe_ref`
  span mapping are assembled as `CurvedBendMacroElement` global stiffness
  in the dense preview path and both sparse evidence lanes; the replaced
  span's straight chord is not assembled (double-stiffness guarded by
  test). Arc center derived from chord + user radius + pipe `y_reference`
  plane, convention recorded in provenance; blocking diagnostics for
  missing/inconsistent user geometry (no silent default, PRD §6.2).
- No k double-counting: in the new mode the component stress-review
  multiplier applies the user SIF only, and provenance states the
  flexibility factor enters the assembled stiffness — retiring the
  legacy "base frame stiffness unchanged" wording for this mode only.
  Legacy `mechanics_geometry_only` proven byte-identical (fixture-parity
  regeneration equals the committed invented envelope; wording
  regression test).
- Loads/recovery for macro spans: exact uniform-thermal free-expansion
  identity (equivalent load `K*u_free`, recovery subtracts `u_free`;
  anchored-free arc yields zero reactions by test); arc-length 50/50
  weight lumping with recorded-residual diagnostic; chord-axial pressure
  thrust; end-force recovery rotated to the chord frame; arc interior
  stations suppressed with recorded-residual diagnostic; dedicated
  review row per realized bend.
- Benchmark `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` wired in the
  mechanics crate directly on kernel + curved-bend APIs: witness
  reactions/displacements reproduced for k in {1, 5, 10, 20} at 5.0e-7
  relative tolerance (measured max deviation 5.8e-8; limiting factor is
  the recorded bending-only axial-rigidity boost 1.0e5, chosen by a
  1e3/1e5/1e7 study that confirmed the witness conditioning warning);
  strict k-sweep monotonicity of the governing anchor moments asserted
  (D-34 evidence shape); free-expansion stress-free self-check;
  whole-body equilibrium; fixture registered in `fixture_inventory()`
  (19) and the hand-calc README.

## Checks

- `cargo test` `core/product_physics`: 56 passed (45 pre-existing + 11 new).
- `cargo test` `validation/benchmarks/mechanics`: 27 passed.
- `python3 -m pytest -q tests/product_preview`: 9 passed; full
  `python3 -m pytest -q tests`: 369 passed.
- `cargo fmt --check` clean on touched crates.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` five-surface sweep on the clean head: commit-bound summary
  under `validation/evidence/sweeps/` recorded with this tranche.

## Boundaries And Residuals

- **Nonlinear active-set loop + curved bend is a blocking recorded
  residual** (`CURVED_BEND_NONLINEAR_LOOP_UNSUPPORTED`):
  `NonlinearFrameSolveInput` carries no explicit-stiffness element slot;
  unblocking is a named follow-on sub-tranche in the solver crate
  (deliberately not a silent arc-free or chord-fallback solve).
- Arc interior-station results and consistent distributed-load end
  moments for macro spans: recorded residuals with diagnostics.
- `bend_pipe_ref` is a new preview-model field (EJ `expansion_joint_pipe_ref`
  precedent); GUI/editors do not yet emit it.
- Witness solver-comparison tolerance recorded at 5.0e-7 relative with
  the limiting factor named; the witness's aspirational 1.0e-9 class is
  not attainable under any finite axial-rigidity comparison
  configuration and the deviation basis is documented in the crate.
- No lifecycle transition, no release-readiness, professional,
  certification, or code-compliance claim.
