# N2-I1 implementation return

Status: `SUCCESS / ACCEPTED FOR REVIEW`

The shared invented projection is exactly matched by production TypeScript `buildReportPackageRequest` output for one present and one deliberately missing component-provenance case. Rust consumes the same projection through `ReportPackageRequest` and `assemble_wire_request`, and the canonical rendered HTML member contains both component IDs, the present source evidence, and the missing-provenance warning. An invalid provenance classification is rejected as `REPORT-PACKAGE-REPORT-DTO-INVALID` before package assembly. No production defect was exposed and no production function changed.

Frozen product/test paths and SHA-256:

- `apps/desktop/src/features/report/reportPackageRequest.test.ts` — `9cf36fec0b737b7f00d62ac1eb295cf75ce9e829d0f0efe9e0c4501a44b3d10a`
- `core/reporting/report_package/src/wire.rs` — `336c48775bd6217a9ab037e183e0345c48f7257d4ebdcd5d89ef3a07bc99f5a1`
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json` — `3b20700718c3e5ee87ddf242637f4bb997f38617115bb7ccc2cba89cbf51d110`

Evidence: `_run_records/TASK_RUN_2026-08-19_2347.md` (`42bef55510cd72afc52adf1b6145e6ac18c044b4699e419e9a964bda7f37b95d`). Focused Vitest PASS 6/6; report-package Cargo PASS 18/18 plus doc tests; Cargo fmt check PASS; scoped containment PASS; `git diff --check` PASS.

Residuals/blockers: none for N2-I1. N2-R1 fresh read-only review is released.
