---
doc_id: TP-MAC-06
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-06 Uniform Thermal Axial Preview

## Purpose

Add the first governed thermal mechanics slice to the product-preview workflow:
uniform axial temperature-change behavior for straight-pipe preview elements.

TP-MAC-05 is the closed endpoint stress-component baseline, TP-RUN-01 is the
closed runtime baseline, and TP-PER-01 is the closed persistence baseline.
TP-MAC-06 should extend mechanics behavior without changing runtime,
persistence, desktop save/open, CLI, container, external execution, protected
rule/check, or professional-acceptance scope.

## Scope

- Add optional preview material input
  `thermal_expansion_coefficient: { value, unit: "1/degC" }`.
- Require `thermal_expansion_coefficient` only when a thermal primitive load
  targets a pipe using that material.
- Use the existing primitive load shape for thermal input:
  `category: "thermal"`, element target, `dimension: "temperature_change"`,
  and unit `degC`.
- Treat the thermal load `direction` as legacy syntactic input that must not
  affect thermal magnitude or sign.
- For each targeted straight pipe, compute `epsilon_th = alpha * delta_T` and
  `N_th = E * A * epsilon_th`.
- Assemble thermal equivalent nodal loads along the element local x axis:
  `-N_th` at end-i and `+N_th` at end-j.
- Add the thermal fixed-end correction to recovered local axial forces:
  `+N_th` at end-i UX and `-N_th` at end-j UX.
- Feed corrected axial resultants into the existing endpoint stress recovery.
- Preserve existing force, reaction, displacement, stress, report-ref, and
  analysis-run result ID patterns; thermal effects may change computed values,
  not IDs.
- Update invented preview fixtures, generated mechanics results, report-packet
  refs, analysis-run hashes, and persistence fixture hashes only as needed to
  reflect explicit invented thermal inputs and changed computed values.
- Update the mechanics gap ledger so uniform axial thermal preview behavior is
  implemented while broader thermal behavior remains deferred.

## Boundaries

- No temperature-dependent material interpolation.
- No thermal load-case algebra, user combinations, sustained/operating code
  combinations, or protected rule/check criteria.
- No expansion-joint behavior, bend flexibility, SIF/flexibility factors,
  code stress categories, allowables, owner criteria, or private rule packs.
- No pressure-to-frame load conversion, intermediate station recovery, shear
  force recovery, equivalent stress, or principal stress.
- No external execution, public transport, FEA/prover invocation, or external
  result ingestion.
- No desktop save/open UX, physical project containers, final CLI syntax,
  migrations, external storage, or durable storage claims.
- No compliance, certification, sealing, professional approval,
  release-readiness, production-readiness, or professional-acceptance claims.
- Use invented or explicitly cleared data only.

## Public Interfaces

- Preview material records may carry
  `thermal_expansion_coefficient: { value, unit: "1/degC" }`.
- Thermal primitive loads continue to use the existing preview primitive-load
  object shape with an element target, `category: "thermal"`,
  `dimension: "temperature_change"`, and unit `degC`.
- The Tauri and headless runtime request shape remains the existing
  `LinearStaticPreviewRequest`; any schema or type change is limited to the
  optional preview material field.
- Existing TP-MAC-05 result IDs are preserved. The implementation must not
  rename or remove force, moment, stress, displacement, reaction, report, or
  analysis-run refs.

## Diagnostics

- Missing `thermal_expansion_coefficient` for a material used by a thermally
  loaded pipe returns a structured diagnostic.
- Non-finite or unsupported thermal expansion values return structured
  diagnostics.
- Invalid thermal load unit, unsupported thermal target, and unsupported
  thermal dimension return structured diagnostics.
- Thermal loads must not silently fall through the existing uniform
  force-per-length load path.
- Unsupported broader thermal paths remain explicit in diagnostics or the
  mechanics gap ledger.

## Acceptance Criteria

- A valid invented preview model can include a thermal primitive load and an
  explicit invented thermal expansion coefficient.
- The product-preview solve applies uniform axial thermal equivalent loads for
  straight pipes targeted by thermal temperature-change loads.
- Recovered endpoint axial force and endpoint stress values include the
  thermal fixed-end correction.
- Existing result IDs and runtime/persistence interfaces are preserved.
- Missing thermal expansion coefficient and invalid thermal inputs produce
  structured diagnostics rather than hidden defaults.
- The mechanics gap ledger marks uniform axial thermal preview behavior
  implemented and keeps broader thermal behavior deferred.
- Existing TP-MAC-05 endpoint stress behavior, TP-RUN-01 runtime routing, and
  TP-PER-01 persistence behavior continue to pass their focused checks.

## Verification

Plan-opening verification:

- `git diff --check`

Implementation verification for closing TP-MAC-06:

- `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml`
- `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `git diff --check`

## Assumptions

- The first thermal behavior slice is uniform axial temperature-change
  behavior for straight-pipe preview elements only.
- Public examples remain invented or cleared.
- Existing TP-MAC-05 result IDs and mechanics behavior remain the baseline
  unless TP-MAC-06 explicitly changes computed values through thermal input.
- No protected standards data, allowables, code criteria, owner data, private
  project data, release claims, or professional claims are introduced.

## Closeout

TP-MAC-06 is implemented for the current product-preview workflow.

Delivered:

- optional preview material support for
  `thermal_expansion_coefficient: { value, unit: "1/degC" }`;
- blocking diagnostics for thermally loaded materials missing explicit thermal
  expansion coefficients and for invalid thermal coefficient input;
- thermal primitive-load handling limited to element-targeted
  `temperature_change` loads with unit `degC`;
- legacy thermal `direction` input preserved syntactically but ignored for
  thermal magnitude and sign;
- uniform axial thermal equivalent nodal loads for targeted straight pipes;
- thermal fixed-end correction before endpoint force and stress result
  recovery;
- regenerated invented preview mechanics fixture and updated persistence
  fixture hashes for explicit invented thermal input;
- desktop mechanics gap ledger marking uniform axial thermal preview behavior
  implemented while broader thermal behavior remains deferred.

Final verification was run on 2026-05-10:

- `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml`
- `cargo test --manifest-path core/solver/straight_pipe/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml`
- `npm run generate:product-preview-mechanics`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py tests/test_project_persistence_service.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- `git diff --check`

The desktop build retained the existing Vite chunk-size warning. Broader
thermal behavior, temperature-dependent material interpolation, thermal
load-case algebra, protected rule/code checks, station recovery,
equivalent/principal stress, release/professional claims, and professional
acceptance remain deferred.
