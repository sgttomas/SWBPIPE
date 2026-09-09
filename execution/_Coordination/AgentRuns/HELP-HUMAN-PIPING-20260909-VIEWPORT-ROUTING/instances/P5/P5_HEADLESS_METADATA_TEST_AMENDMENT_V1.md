# P5 headless metadata test amendment V1

Status: SEALED FOR TEST-ONLY IMPLEMENTATION

## Trigger and diagnosis

Integrated DEC-025 gate G0 at candidate `67f39cd498089dfb38aa61a787fa7fb217acf213` ran the headless crate with 38 passing tests and one failure: `straight_station_library_document_metadata_uses_canonical_schema_categories`. The failure reported that a primitive quarter-1 pressure-hoop row did not contain `j-side section action`; its actual convention was the explicit pressure membrane hoop convention.

The product producer is correct. Straight station section resultants and mechanical stress components retain the five section-action, element-frame, equilibrium, recovered-action, and distributed-load details. Pressure membrane stress is a distinct `pipe_section` category whose basis and sign are the explicit pipe pressure. Genuine nonzero pressure emits hoop stress while pressure thrust suppresses the separate longitudinal row. A genuine zero-valued pressure input emits both zero hoop and zero longitudinal membrane rows; the longitudinal kind remains explicitly disclosed because the export map does not adopt that family.

The headless test is stale because its primitive-row branch requires all five mechanical section-cut phrases for every stress kind after accepting both `element_local` and `pipe_section` coordinate systems. Schema required-field, string, enum, combination, disclosure, and exercised-family checks are otherwise sound.

## Authorized repair

Write only the existing test in `core/runner/headless/src/result_envelope_binding.rs`, whose sealed pre-edit SHA-256 is `2ed810b577d5db34824e65a36e9d696d3758dd14d8c4bdd0941c1cb797cf9c85`.

Retain the common schema checks and explicit-combination assertions. For primitive pressure-hoop and pressure-longitudinal rows, assert `pipe_section`, `recovered_from_open_mechanics_stress_components`, and the exact category-specific explicit-pressure sign convention. For every other exercised primitive row, assert `element_local`, preserve the existing basis rule, and require all five section-cut phrases plus the no-interpolation assertion. Retain the zero-pressure longitudinal presence/disclosure oracle and pressure-hoop coverage in every model.

Do not change production behavior, product physics, fixtures, schemas, export mappings, result identities, result families, tolerances, or evidence verdicts.

## Validation and handoff

Run the focused failing test, then the full headless crate if focused validation passes, using the assigned serial Cargo slot. Record the exact commands and results. Freeze the one-file test diff, verify the product source and fixture hashes remain `dbfff6d5c2fa9241e972042fd2c3a6ce35c6d60f4a973dd5a083a7fe5dfb9c58` and `fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c`, and route the final diff for fresh read-only review before integration resumes.
