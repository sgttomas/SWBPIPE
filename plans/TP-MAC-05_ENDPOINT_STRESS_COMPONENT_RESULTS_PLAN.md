---
doc_id: TP-MAC-05
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-05 Endpoint Stress Component Results

## Purpose

Build on TP-MAC-04 by exposing open-mechanics stress components at pipe end-i
and end-j in the product preview workflow.

The current stress-recovery crate already computes axial normal stress, bending
normal stress, torsional shear stress, and pressure membrane components from
explicit resultants, section properties, and optional pressure basis inputs.
The product preview currently exposes only a pipe-level stress summary. This
tranche promotes the available open mechanics components into inspectable
endpoint result rows while preserving the existing summary result.

## Scope

- Primary scope: `PKG-05 / DEL-05-03` stress recovery.
- Supporting surfaces: `DEL-07-05` results viewer, `DEL-08-04` result export
  metadata, and `DEL-14-02` analysis-run hashes.
- Recover stress twice per straight pipe: once from end-i local resultants and
  once from end-j local resultants.
- Preserve `result:stress:{pipe}` as the pipe-level
  `open_formula_stress_summary`.
- Emit endpoint stress component IDs using this pattern:
  - `result:stress:{pipe}:end-i:axial-normal`
  - `result:stress:{pipe}:end-i:bending-normal-y`
  - `result:stress:{pipe}:end-i:bending-normal-z`
  - `result:stress:{pipe}:end-i:torsional-shear`
  - `result:stress:{pipe}:end-i:pressure-hoop`
  - `result:stress:{pipe}:end-i:pressure-longitudinal`
  - and the same `end-j` variants.
- Emit pressure stress components only when explicit pressure basis exists for
  that pipe; do not invent zero-pressure component rows.
- Use `MPa` for product-preview stress component result values, rounded
  consistently with existing preview results.
- Extend desktop result interpretation so stress endpoint components get the
  same paired end-i/end-j comparison behavior as force/moment endpoint
  components.
- Extend report-packet selected result refs and DEL-14-02 analysis-run hashes
  to include deterministic endpoint stress component refs.
- Update the mechanics gap ledger so endpoint stress-component preview results
  are implemented while intermediate stations, shear force recovery, and
  pressure-to-frame conversion remain deferred.

## Boundaries

- Do not rename or remove existing force, moment, displacement, reaction, or
  stress summary result IDs.
- Do not implement intermediate station interpolation.
- Do not add shear force recovery.
- Do not convert pressure primitive loads into frame load vectors.
- Do not add equivalent stress, principal stress, code stress categories,
  allowables, rule checks, compliance checks, certification, sealing, approval,
  release-readiness, or production-readiness claims.
- Pressure membrane stress is open mechanics context only.

## Public Interfaces

- Extend result metadata component enums to include:
  - `axial_normal_stress`
  - `bending_normal_stress_y`
  - `bending_normal_stress_z`
  - `torsional_shear_stress`
  - `pressure_hoop_stress`
  - `pressure_longitudinal_stress`
- Add metadata basis `recovered_from_open_mechanics_stress_components`.
- Add coordinate-system enum `pipe_section` for pressure membrane stress
  components.
- Axial, bending, and torsional stress components use `element_local`
  coordinate-system metadata.
- The legacy pipe-level stress summary remains a summary result and does not
  need endpoint-pair metadata.

## Acceptance Criteria

- The computed preview result envelope contains endpoint stress component rows
  for each straight pipe where stress recovery succeeds.
- Selecting `result:stress:pipe-P-120:end-j:torsional-shear` in the desktop
  results panel shows stress component metadata, `end_j`, j-end context,
  `pipe:P-120`, and the paired end-i torsional-shear stress result.
- The existing `result:stress:pipe-P-120` summary result still exists.
- The report packet includes at least one deterministic endpoint stress
  component ref when available.
- DEL-14-02 analysis-run records include result-value hashes for endpoint
  stress component rows.
- The mechanics gap ledger marks endpoint stress-component preview results as
  implemented and keeps intermediate station recovery, shear force recovery,
  and pressure-to-frame conversion explicit as deferred or not implemented.

## Verification

- `npm run generate:product-preview-mechanics`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/loads/stress_recovery/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- Browser smoke using `apps/desktop/SMOKE.md`
- `git diff --check`

## Assumptions

- The current TP-MAC-04 local changes are the baseline and must not be
  reverted.
- This is a governed cross-package tranche, not a single sealed Type 2
  deliverable.
- Stress signs follow the existing `stress_recovery` crate and endpoint local
  force-vector sign conventions.
- Pressure primitive loads remain retained for stress recovery context but are
  not applied to the frame load vector in this tranche.

## Closeout

TP-MAC-05 is implemented for the current product preview workflow.

Delivered:

- end-i and end-j open-mechanics stress component rows for straight pipes;
- preserved pipe-level `result:stress:{pipe}` summary results;
- pressure membrane component rows only when explicit pressure basis exists;
- result metadata enums for stress components, `pipe_section`, and
  `recovered_from_open_mechanics_stress_components`;
- desktop endpoint-pair interpretation for selected endpoint stress rows;
- report-packet selected refs and DEL-14-02 result-value hashes for endpoint
  stress rows;
- mechanics gap ledger entry marking endpoint stress components implemented
  while station recovery, shear force recovery, and pressure-to-frame load
  conversion remain deferred.

Final verification was run on 2026-05-10:

- `npm run generate:product-preview-mechanics`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/loads/stress_recovery/Cargo.toml`
- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- browser smoke against `http://127.0.0.1:5173/`
- `git diff --check`

The desktop build retained the existing Vite chunk-size warning. Intermediate
stations, shear force recovery, pressure-to-frame conversion, equivalent or
principal stress, protected rule/code checks, and professional acceptance
remain deferred.
