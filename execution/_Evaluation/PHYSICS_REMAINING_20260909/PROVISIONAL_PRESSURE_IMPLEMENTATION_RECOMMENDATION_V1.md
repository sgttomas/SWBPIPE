# Provisional pressure implementation recommendation V1

Status: **engineering selection ready for independent refutation; P0 final fan-in pending**. This is a bounded implementation recommendation, not source authority, project-contract adoption, or completed industry-solver validation.

## Consequential finding

Current straight-pipe pressure behavior is internally inconsistent. The solver applies a closed-end `[-P,+P]` pressure pair using `P=pAi`, then subtracts that pair from straight recovered endpoint forces, suppresses longitudinal pressure stress whenever pressure thrust is active, and has no Poisson ratio or pressure-boundary topology in the runtime input. Endpoint and station paths therefore do not expose one coherent wall-force field, and pressure-induced axial strain is omitted. The mismatch is visible in current source at `core/product_physics/src/lib.rs:6610-6647,6714-6741,6847-6865,1660-1666` and `core/loads/stress_recovery/src/lib.rs:926-956` and is quantified by the accepted candidate four-case investigation in `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_run_records/PHYSICS_UI_EXECUTION_20260908/INVESTIGATION_REPORT.md`.

## Selected engineering model

For the first straight-pipe pressure tranche, select the exact circular-annulus, homogeneous-isotropic, small-strain generalized-plane-strain reduction used by the 2026-09-08 four-case candidate:

- `Nw` is the physical tensile wall resultant; axial wall stress is `Nw/As` exactly once.
- `S=Nw+peAe-piAi` is a separately named effective axial force. Never reuse a generic axial row for both meanings.
- Use actual bore and wall areas from the effective pipe geometry. Do not mix mean-radius cap area with exact annulus wall area.
- Add explicit dimensionless Poisson ratio at the same selected temperature basis as `E`, `G`, and `alpha`; do not silently infer it from `E/G`.
- Validate the isotropic thermodynamic range `-1 < nu < 0.5`. Publish the computed consistency residual against `G=E/[2(1+nu)]` as diagnostic evidence; do not hide an inconsistent `E/G/nu` triple behind a default.
- Add explicit pressure wall-transfer topology. Use `closed_transferred` when attached closures transfer end thrust to the pipe wall and `separate_closure` when closure thrust bypasses the pipe wall. A truly vented/open terminal is outside a uniform-pressure segment and must not be simulated by relabeling a pressurized free barrel.
- Preserve legacy documents by a versioned migration that records the existing behavior as `closed_transferred_legacy_inference`; require explicit topology for newly authored pressure loads. Do not silently change already published row meanings.
- Emit additive typed results for node-on-element endpoint actions, tension-positive section cuts, wall axial force, effective axial force, and exact inner/outer hoop/radial plus wall-average axial stress. Keep legacy rows only as explicitly deprecated compatibility rows with their old basis named.
- Limit this tranche to straight circular pipes with uniform pressure, homogeneous isotropic linear elasticity, and small displacement away from end discontinuities. Curved-bend Poisson coupling, expansion-joint effective area, pressure stiffening, ovalization, large displacement, plasticity, and code allowables remain separate work.

The engineering basis is the exact annulus/Lamé and generalized Hooke-law treatment in MIT 22.314 lecture note L.4 (`https://ocw.mit.edu/courses/22-314j-structural-mechanics-in-nuclear-power-technology-fall-2006/137e6469e37e9d347b7b3b69292da2f3_l4_2.pdf`, checked 2026-09-09). Abaqus' publisher documentation independently confirms that pipe-pressure load semantics must distinguish closed-end loading and open-end compensation and that effective axial force is a separately named output (`https://docs.software.vt.edu/abaqusv2025/English/SIMACAEPRCRefMap/simaprc-c-loaddistributed.htm`, checked 2026-09-09; `https://docs.software.vt.edu/abaqusv2025/English/SIMACAEOUTRefMap/simaout-c-std-elementsectionvariables.htm`, checked 2026-09-09). These sources support the selected model and terminology; Chirality should keep its own typed conventions.

## Proposed bounded implementation fence

One WORKING_ITEMS tranche should own only these source/data surfaces, with the exact subset finalized after independent design refutation:

- `projects/chirality-piping/core/product_physics/src/lib.rs` — runtime input, pressure topology, annulus/constitutive assembly, typed force/stress result production, diagnostics, focused unit tests.
- `projects/chirality-piping/core/loads/stress_recovery/src/lib.rs` — replace the ambiguous mean-radius pressure pair for the selected exact-pressure path with an explicitly typed exact annulus result basis; retain old helper only for versioned compatibility if still consumed.
- `projects/chirality-piping/schemas/model.schema.yaml` and `projects/chirality-piping/schemas/results.schema.yaml` — additive topology/property/result vocabulary and versioned compatibility semantics. The separate material library schema already admits `poisson_ratio`; do not duplicate a competing meaning.
- `projects/chirality-piping/core/model_transform/physical_to_analytical/contract.py` and `_solver_boundary_adapter.py` only to carry the selected Poisson/topology fields without inference; keep physical-to-analytical runtime invocation as a separately testable bridge concern.
- Focused fixtures/tests under `projects/chirality-piping/fixtures/product_preview/**`, `projects/chirality-piping/tests/test_physical_to_analytical_transform.py`, `projects/chirality-piping/tests/test_analytical_solver_boundary_adapter.py`, and a new bounded PKG-09 reference location under `projects/chirality-piping/validation/**`.

No app/UI source is required for the first physics implementation. App authoring of the new fields can follow as a separate consumer tranche after the source contract is frozen.

## Frozen refutation and validation requirements

Before source effect, a fresh reviewer should attempt to falsify:

1. the four free-body/constitutive cases: free closed, restrained closed, free barrel with separately supported closures, and combined thermal/pressure;
2. endpoint reversal and `Ncut_i=-Fi_x`, `Ncut_j=+Fj_x` sign invariance;
3. `p=0`, `nu=0`, and `DeltaT=0` reductions, plus two collinear segments whose internal equivalent loads cancel;
4. mutation cases that omit or double-count `P`, use the wrong area, or mix wall and effective force;
5. missing/invalid `nu`, missing new topology, and legacy migration behavior;
6. additive result-schema compatibility and exact source-result provenance.

After implementation and independent code review, run focused analytic oracles in both dense and sparse modes. Then compare the four frozen cases and a small multi-element model with at least one industry-standard solver in the later validation phase. That external comparison is planned and has not yet occurred.

## Remaining limitation

This recommendation does not yet select a pressure-gradient/open-terminal model, curved-pipe constitutive extension, expansion-joint pressure-area interaction, or a code-based thin-wall applicability threshold. None is needed to implement the bounded exact straight-pipe tranche above.
