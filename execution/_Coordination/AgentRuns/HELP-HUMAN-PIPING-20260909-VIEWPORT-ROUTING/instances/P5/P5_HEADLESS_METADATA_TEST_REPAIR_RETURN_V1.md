# P5 headless metadata test repair return V1

Status: FROZEN FOR INDEPENDENT READ-ONLY REVIEW

## Authority and diagnosis

- Integrated candidate: `67f39cd498089dfb38aa61a787fa7fb217acf213`.
- C0 repair disposition: `INTEGRATION_REPAIR_DISPOSITION_V1.json`, SHA-256 `2ce3207e62cc2b450f0a5782f863f0aa865096e7080aa56c0921df7b2a48b46d`.
- Sealed pre-effect amendment: `P5_HEADLESS_METADATA_TEST_AMENDMENT_V1.md`, SHA-256 `39b2de6032477f0922add4e2efba045942663bca002add04e5f0d37e39b15619`.
- Diagnosis: the failing assertion was stale and overbroad. It required mechanical section-cut framing text on pressure-membrane rows even though those rows truthfully use `pipe_section` coordinates and explicit pressure sign conventions. No producer or schema regression was found.

## Test-only repair

The only source write was inside `straight_station_library_document_metadata_uses_canonical_schema_categories` in `core/runner/headless/src/result_envelope_binding.rs`.

- Pressure-hoop and pressure-longitudinal rows now assert their exact component, `pipe_section` coordinate system, stress-component recovery basis, and explicit pressure sign convention.
- All other primitive rows retain the five section-cut framing assertions, basis rule, `element_local` coordinate assertion, and no-interpolation assertion.
- The existing zero-pressure longitudinal-presence oracle remains, and nonzero-pressure cases now explicitly assert that the longitudinal pressure row is absent.
- Common required-field, string, enum, combination, disclosure, and exercised-family checks remain active.

The file changed from SHA-256 `2ed810b577d5db34824e65a36e9d696d3758dd14d8c4bdd0941c1cb797cf9c85` to frozen SHA-256 `fd965bb016daf9ab13df41e89057efaf84b8c4c92f87e973676fabddb917e1f0`. Rustfmt adjusted three pre-existing long lines within the same authorized test function; no other function changed.

## Validation

Attempt 1 ran:

```text
cargo fmt --manifest-path core/runner/headless/Cargo.toml -- --check && cargo test --manifest-path core/runner/headless/Cargo.toml straight_station_library_document_metadata_uses_canonical_schema_categories -- --nocapture
```

The formatter check exited 1 before Cargo executed the test, identifying only three formatting differences inside the authorized test function. After applying rustfmt to that file, the same command passed the formatter check and the focused test passed 1/1 with 38 filtered out.

The full command then ran:

```text
cargo test --manifest-path core/runner/headless/Cargo.toml
```

Results: library 39/39 PASS, `headless_preview_runner` 1/1 PASS, `openpipestress-runner` 15/15 PASS, and doc-tests 0 failures. The only emitted warning was the pre-existing unused-import warning in the mechanics benchmark crate. `git diff --check` passed for the changed headless file.

## Frozen boundaries

- Product physics source is unchanged at SHA-256 `dbfff6d5c2fa9241e972042fd2c3a6ce35c6d60f4a973dd5a083a7fe5dfb9c58`.
- Generated preview fixture is unchanged at SHA-256 `fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c`.
- No production behavior, schema, fixture, result family, tolerance, or evidence verdict changed.
- Source writes are frozen pending the root-routed independent read-only review and CHANGE-owned integrated rerun.
