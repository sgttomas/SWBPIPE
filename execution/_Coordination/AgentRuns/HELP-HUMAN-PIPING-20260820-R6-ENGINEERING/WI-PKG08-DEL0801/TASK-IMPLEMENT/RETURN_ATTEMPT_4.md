# N2-I1 implementation return — Amendment 3 / attempt 4

Status: `SUCCESS / PRODUCTION IDENTITY DEFECT FIXED / ACCEPTED FOR FRESH FULL-N2 REVIEW`

`buildReportPackageRequest` now computes the existing WASM-backed canonical SHA-256 for both the separately supplied model and the verified manifest's `model_basis.model_payload`, validates both digests, and requires equality before `buildRenderableReportInput` or package assembly. Same-ID different payloads fail closed with `REPORT-PACKAGE-INPUT-MANIFEST-MODEL-PAYLOAD-MISMATCH`; the supplied model hash is reused in the existing package evidence.

Complete final N2 product/test freeze from original basis `357a58b56726feba49507534159c3fbc4656b818`:

- `apps/desktop/src/features/report/reportPackageRequest.ts` — `514b0582eb56a233b7445ca53b67bef0e903bde4b36acca6f8f0491d3dd0db99`
- `apps/desktop/src/features/report/reportPackageRequest.test.ts` — `c3fe9583ae9329761a186eb8e6209491bec09c584298189ad9d87ee323593437`
- `core/reporting/report_package/src/wire.rs` — `4a04fbde7b50515b00d55dde7438497fa7c5fe1d433bb7fc7d9ff5ee771489e1`
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json` — `3b20700718c3e5ee87ddf242637f4bb997f38617115bb7ccc2cba89cbf51d110`

Full diff: 360 additions, 4 deletions. Manager rerun: focused Vitest PASS 8/8; registered desktop typecheck/build PASS; report-package Cargo PASS 19/19 plus doc tests; Cargo fmt PASS. Child containment and basis-scoped diff check PASS. No blocker remains for implementation.
