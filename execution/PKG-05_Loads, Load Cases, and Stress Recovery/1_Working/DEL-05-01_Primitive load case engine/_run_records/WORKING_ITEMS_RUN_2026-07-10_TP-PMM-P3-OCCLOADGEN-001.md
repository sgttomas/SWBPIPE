# WORKING_ITEMS Run Record - TP-PMM-P3-OCCLOADGEN-001

Date: 2026-07-10
Agent: WORKING_ITEMS (bounded Type-2 implementation worker; owner-directed session)
Deliverable: DEL-05-01 - Primitive load case engine
Package: PKG-05 - Loads, Load Cases, and Stress Recovery
Tranche: TP-PMM-P3-OCCLOADGEN-001
Target stage: R5 / physical-model mechanics program P3 tranche (b)
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-068`,
D-36 item 2)

## Scope

Second bounded slice of the D-36 P3 workflow-physics set:
static-equivalent occasional-load generation. Seismic: user-entered
g-factors per global axis and a user-entered gravity acceleration
generate distributed loads from the model's own computed mass
distribution (metal + contents + insulation over the tranche (c)
mill-tolerance effective wall). Wind: user-entered pressure/shape
parameters project onto the exposed diameter of user-marked spans only.
Pure mechanics from user inputs; blocking diagnostics on missing inputs
(PRD section 6.2); no dynamics (disposition stays with D-12).

## Files Touched

- `core/loads/primitive_loads/src/lib.rs` (new generators
  `generate_seismic_equivalent_static_loads` /
  `generate_wind_equivalent_static_loads` with basis/element input
  structs; `FindingCode` gains `MissingGenerationInput` /
  `InvalidGenerationInput`; `prepare_equivalent_static_loads` unchanged —
  it remains the pass-through boundary the generated loads flow into)
- `core/solver/diagnostics/src/lib.rs` (finding-code mapping
  exhaustiveness for the two new codes)
- `core/product_physics/src/lib.rs` (preview `PipeSectionInput` gains
  optional `material_density` / `contents_density` /
  `insulation_thickness` / `insulation_density`; `PreviewLoadCase` gains
  optional `equivalent_static` with seismic/wind user inputs; DEC-018
  unit normalization for all new quantities; per-pipe
  `compute_pipe_mass_per_length` mirroring
  `core/section_properties/calculator.py`;
  `append_equivalent_static_generated_loads` wired into
  `build_load_case_primitive_loads` so `solve_load_case` consumes
  generated loads through the existing uniform-load machinery)
- `core/product_physics/src/validation.rs` (equivalent-static-only load
  cases satisfy the at-least-one-load requirement; generation inputs
  require public-preview provenance)
- `schemas/model.schema.yaml` (`LoadCase.equivalent_static_generation`
  slot; new `$defs`: `EquivalentStaticGeneration`,
  `SeismicEquivalentStaticInput` (required user-entered
  `gravity_acceleration` + per-axis `g_factors`, `minProperties: 1`),
  `WindEquivalentStaticInput` (required `pressure`, `shape_factor`,
  `direction`, `exposed_element_refs`), plus `AccelerationQuantity` /
  `PressureQuantity` / `DimensionlessQuantity`)
- `tests/test_model_schema.py`, `tests/test_physical_to_analytical_transform.py`
- `validation/hand_calcs/mechanics/tp_pmm_p3_occloadgen_equivalent_static.md`
  (witness) and `validation/hand_calcs/mechanics/README.md`
- `validation/benchmarks/mechanics/{src/lib.rs, README.md}` (fixture
  `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC`, family registration,
  inventory count, tests)

## Recorded Design Choices

- **Gravity is a user-entered input, not an embedded constant.** The
  ruling's "g-factors" multiply mass; converting mass to force needs an
  acceleration. To keep the no-defaults posture strict, the seismic
  input requires an explicit user-entered `gravity_acceleration`
  (blocking when absent) instead of embedding a standard-gravity value.
- **Seismic mass basis covers every pipe segment** (the model's own mass
  distribution); any pipe lacking a user-entered `material_density` is a
  blocking diagnostic, never a defaulted mass. Contents density is
  optional alone; insulation thickness/density must be supplied as a
  pair (a lone half is blocking, not silently skipped).
- **Wind acts on user-marked spans only** (`exposed_pipe_refs` on the
  preview input; `exposed_element_refs` in the canonical schema);
  unknown references are blocking. Exposed diameter is outside diameter
  plus twice the insulation thickness when insulation is supplied.
- Generated loads carry categories `seismic`/`wind` (already
  equivalent-static categories) and flow through the existing
  distributed-load, curved-bend-lumping, and result machinery unchanged.

## Implemented Evidence

- Rust generator unit tests: intensity closed forms, deterministic load
  ids, blocking findings for missing/invalid inputs, boundary-helper
  compatibility (5 new tests).
- Product integration tests: generated seismic and wind cases produce
  envelopes identical (1e-9 relative) to authored distributed loads of
  the hand-computed intensity; missing density / gravity / marked spans
  and partial insulation pairs are blocking with
  `EQUIVALENT_STATIC_INPUT_MISSING` (6 new tests).
- Benchmark `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` reproduces the
  witness mass distribution, generated intensities, and 50/50 lumped end
  forces at the 1.0e-9 internal epsilon (3 new tests). Tolerance posture
  per `DEC-024`/`DEC-026` (governed values `TBD`; tighten-only).

## Checks

- `cargo test` `core/loads/primitive_loads`: 45 passed (40 + 5 new).
- `cargo test` `core/solver/diagnostics`: 24 passed.
- `cargo test` `core/product_physics`: 66 passed (60 + 6 new).
- `cargo test` `validation/benchmarks/mechanics`: 30 passed (27 + 3 new).
- `python3 -m pytest -q tests`: 376 passed (374 + 2 new).
- `cargo fmt --check` clean on touched crates.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` five-surface sweep recorded on the clean head with the final
  tranche of this run.

## Boundaries And Residuals

- No dynamics content (response spectra, time histories, modal
  analysis); that disposition stays with `D-12`.
- No code-content wind/seismic coefficients, exposure categories,
  importance factors, or catalogs; all values user-entered.
- Wind marking is whole-span (per pipe segment); sub-span wind marking
  is a possible future extension, deliberately not invented here.
- The canonical `model.schema.yaml` slot and the preview-model input are
  parallel surfaces (per the `bend_pipe_ref` precedent); GUI/editors do
  not yet emit either.
- Schema files carry no literal version constant to bump (instance
  `schema_version` remains `0.1.0`); recorded as an
  instructions-vs-live-tree delta.
- No lifecycle transition, no release-readiness, professional,
  certification, or code-compliance claim.
