# WORKING_ITEMS Run Record - TP-PMM-P1-CURVEDBEND-004

Date: 2026-07-10
Agent: WORKING_ITEMS (claude worker; owner-adopted tranche, branch
`claude/piping-p1-arc-consistency`)
Deliverable: DEL-04-01 - 3D frame stiffness kernel
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-PMM-P1-CURVEDBEND-004
Target stage: R5 / physical-model mechanics program P1 residual closure
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-070`,
residuals named in TP-PMM-P1-CURVEDBEND-002/003)

## Scope

Close the two remaining DEC-070 arc residuals from TP-PMM-P1-CURVEDBEND-002:
(a) replace the 50/50 arc-tributary end lumping of uniform distributed loads
on curved-bend macro spans with consistent fixed-end forces/moments from
exact integration along the arc, formed in `core/solver/curved_bend` and
consumed by `core/product_physics`; (b) evaluate interior result stations
along the arc from the assembled macro-element by segment equilibrium.
Retire the `CURVED_BEND_DISTRIBUTED_LOAD_LUMPED` and
`CURVED_BEND_INTERIOR_STATIONS_NOT_EVALUATED` diagnostics and the
`distributed_load_treatment=arc_tributary_end_lumped` /
`interior_stations=recorded_residual` provenance strings only with the live,
tested replacements.

## Files Touched

- `core/solver/curved_bend/src/lib.rs`
- `core/product_physics/src/lib.rs`
- `validation/hand_calcs/mechanics/curved_bend_distributed_load_fixed_end.md`
  (new witness)
- `validation/hand_calcs/mechanics/README.md` (fixture-inventory row)
- `validation/benchmarks/mechanics/{src/lib.rs, README.md}`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D
  frame stiffness kernel/MEMORY.md`

## Implemented Evidence

- Crate `open_pipe_stress_curved_bend`:
  `consistent_uniform_nodal_loads(intensity_global)` — consistent equivalent
  nodal loads for a constant-direction uniform load per unit arc length, by
  exact closed-form unit-load integration (extended
  `{1, cos, sin, theta, theta cos, theta sin}` basis Gram, no quadrature),
  force-method redundants against the element's own end flexibility (user
  flexibility factors enter the bending curvature exactly as in
  `end_flexibility`, so load and stiffness stay consistent), and a
  rigid-equilibrium node-i share; `arc_section_resultants(fraction,
  node_j_force_global, intensity_global)` — true section resultants at any
  arc fraction from segment equilibrium, reported in the arc section frame
  (x tangent toward node j, z bend-plane normal, y toward the arc center).
  Crate tests (16 total, 5 new): straight-limit convergence to the classical
  straight fixed-end vector `[qL/2, qL^2/12]` with first-order convergence
  and a documented 2.0e-3 tolerance at 1e-3 rad (cancellation floor noted);
  rigid-equilibrium force/moment balance on translated arc geometry;
  independent longhand quarter-circle tip deflections under combined
  in-plane/out-of-plane intensities for (k_in, k_out) in {(1,1), (2,1.75)}
  at 1.0e-9 relative; station identities (fraction 1 equals the node-j end
  force, fraction 0 the negated node-i force, free-tip interior equals the
  longhand distributed segment actions); input validation rejections.
- `core/product_physics`: uniform element loads on realized bend spans
  assemble the crate's consistent load vector (blocking `LOAD_INPUT_INVALID`
  on an inapplicable load — no silent drop or lumped fallback); recovery
  subtracts the equivalent loads so recovered end forces are the true member
  forces of the continuously loaded arc (mirroring the exact thermal
  free-expansion subtraction); interior stations at the straight-span
  fractions (quarter_1/midspan/quarter_3) are emitted for macro spans from
  arc segment equilibrium with result-metadata basis
  `arc_section_equilibrium_from_assembled_end_forces` and coordinate system
  `arc_section_frame` (straight spans keep
  `interpolated_from_endpoint_resultants` byte-identically); station stress
  rows recover from the arc resultants. Review-row basis updated to
  `distributed_load_treatment=arc_consistent_fixed_end_integration` and
  `interior_stations=arc_section_equilibrium_stations`. Both retired
  diagnostics and both retired provenance strings are grep-clean outside
  historical run records. Product tests: consistent-weight end forces match
  the direct macro-element oracle (anchored end carries the full distributed
  resultant — the lumping carried only half — and the free tip carries
  none); arc station rows match the direct `arc_section_resultants` oracle;
  dense/sparse parity test extended with the arc uniform load
  (`result:sparse-live:dense-parity-relative-delta` <= 1.0e-9 and identical
  station rows on both lanes).
- Witness `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END`
  (`validation/hand_calcs/mechanics/curved_bend_distributed_load_fixed_end.md`):
  clamped-clamped quarter-circle arc (R = 1.2 m, invented section/material)
  under uniform in-plane (-1500 N/m) and out-of-plane (-800 N/m) loads for
  k in {1, 2}; closed-form force-method free-tip deflections, fixed-end
  reactions at both ends, and interior-station bending/torsion distribution;
  verified by antiderivative differentiation, 200k-point quadrature
  (max 4.0e-12 / 1.1e-11 relative), an independent 4000-element
  displacement-method model (reactions <= 2.4e-5, stations <= 7.8e-5
  relative, discretization-limited), round-off equilibrium residuals, and
  exact quarter-arc symmetry checks (equal end shears, antisymmetric torsion
  with exact midspan zero). Invented fixture data only; no code content.
- Benchmark fixture `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` wired in
  `validation/benchmarks/mechanics` directly on the curved-bend crate APIs:
  42 dimensioned expected values (reactions + stations, both cases, both k),
  compared at the DEC-026 analytic-class 1.0e-9 relative tier with a 1.0e-3
  near-zero absolute scale floor (exact-zero midspan torsion row); measured
  normalized deviation 3.6e-10 (limited by the 10-significant-figure witness
  transcription; tighten-only). Registered in `fixture_inventory()` (21) and
  both README inventories; the stale benchmark-README note claiming the
  expansion-loop fixture was "not yet registered" was corrected in passing
  (it has been registered since TP-PMM-P1-CURVEDBEND-002).

## Checks

- `cargo test` `core/solver/curved_bend`: 16 passed.
- `cargo test` `core/product_physics`: 71 passed.
- `cargo test` `validation/benchmarks/mechanics`: 33 passed.
- `cargo test` `core/solver/nonlinear_integration`: 14 passed;
  `core/solver/frame_kernel`: 36 passed; `validation/benchmarks/nonlinear`:
  19 passed (dependent/adjacent crates, sanity).
- `cargo fmt --check` clean on touched crates.
- `python3 -m pytest -q tests` (project): 387 passed.
- Repo-root `PYTHONDONTWRITEBYTECODE=1 python3
  tools/practitioner_harness/harness.py self-check`: exit 0.
- Full practitioner-harness pytest (`tools/practitioner_harness/`):
  263 passed, 1 skipped.
- `DEC-025` five-surface sweep retry chain: the first sweep at the clean
  code head `50f230b09` failed at surface `desktop_vitest` (`vitest: command
  not found` — fresh worktree without node dev dependencies;
  `SWEEP_20260711T020214Z_50f230b09885.json`, environment failure, not a
  code failure); `npm ci` installed the committed lockfile dependencies; the
  immediate re-run passed all five surfaces but recorded `-dirty` because
  the first summary sat untracked
  (`SWEEP_20260711T020414Z_50f230b09885-dirty.json`); both artifacts are
  committed as the retry chain and the final clean-committed-head sweep
  summary is committed on top (named in the closing commit).

## Boundaries And Residuals

- Pressure thrust keeps the recorded straight-chord axial end-force
  treatment on macro spans (`pressure_thrust_treatment=
  straight_chord_axial_end_forces`); arc stations inherit that recovery
  decision and their basis string names the inheritance.
- The consistent-load path covers constant-direction uniform intensities
  (the only element uniform loads the preview accepts); distributed moments
  and varying intensities remain out of scope for the preview load model as
  a whole, not as an arc-specific residual.
- All flexibility factors remain user-entered; no k/SIF equation, catalog
  value, or default ships (DEC-066/DEC-070 fence). D-38 temperature
  interpolation semantics untouched (AWAITING_RULING).
- No threshold loosening anywhere; new tolerances are measured-and-recorded
  under the DEC-026 tighten-only convention.
- No lifecycle transition, release-readiness, professional, certification,
  sealing, authentication, or code-compliance claim.
