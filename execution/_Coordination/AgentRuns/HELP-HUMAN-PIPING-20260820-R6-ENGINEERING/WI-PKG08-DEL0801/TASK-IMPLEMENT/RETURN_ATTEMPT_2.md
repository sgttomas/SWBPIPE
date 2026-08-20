# N2-I1 implementation return — remediation attempt 2

Status: `SUCCESS / REVIEW FINDING REMEDIATED / ACCEPTED FOR FRESH REVIEW`

The shared fixture root is now closed against silent drift. TypeScript asserts schema `1.0.0` and compares the complete constructed root with the complete fixture. Rust deserializes typed `deny_unknown_fields` root and present/missing case wrappers, validates the exact version, and has explicit fail-closed cases for an unsupported version and an unexpected root field.

Frozen product/test paths and SHA-256:

- `apps/desktop/src/features/report/reportPackageRequest.test.ts` — `c7536feb61c2ae02471156d0b360cc3664c3a15cde68934a7de426d30b2f3f5b`
- `core/reporting/report_package/src/wire.rs` — `4a04fbde7b50515b00d55dde7438497fa7c5fe1d433bb7fc7d9ff5ee771489e1`
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json` — `3b20700718c3e5ee87ddf242637f4bb997f38617115bb7ccc2cba89cbf51d110`

Manager rerun: focused Vitest PASS 6/6; report-package Cargo PASS 19/19 plus doc tests; Cargo fmt check PASS. Child containment and diff checks PASS. No production function or fixture content changed in remediation.

Residuals/blockers: none for N2-I1. Fresh N2-R1 attempt 2 is released.
