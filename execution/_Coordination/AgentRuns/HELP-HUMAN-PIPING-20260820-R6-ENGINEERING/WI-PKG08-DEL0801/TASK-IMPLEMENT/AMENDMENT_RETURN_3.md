---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-IMPLEMENT
node: N2-I1
amendment: 3
attempt: 4
status: SUCCESS
finding_state: FULLY_REMEDIATED_PENDING_FRESH_REVIEW
accepted_basis: 357a58b56726feba49507534159c3fbc4656b818
---

# N2-I1 amendment return 3 — production model/manifest payload identity gate

## Remediation

- `buildReportPackageRequest` retains the existing internal input-manifest verification and manifest/model/result reference check, then hashes both the supplied model and the verified manifest's `model_basis.model_payload` through the existing WASM-backed `canonicalSha256Hex` service.
- Both canonical hashes must be exact lowercase SHA-256 values and equal before `buildRenderableReportInput` or any report/package assembly runs.
- A same-project-ID payload difference now fails closed with stable code `REPORT-PACKAGE-INPUT-MANIFEST-MODEL-PAYLOAD-MISMATCH`.
- The supplied-model hash is reused by the existing result-envelope, audit-manifest, and state-comparison projections; no second canonicalization implementation or new dependency was introduced.
- Focused regression coverage rejects a same-ID model with a changed payload, proves reordered object keys have different ordinary JSON byte order but equal canonical hashes and pass the live production gate, and retains the ordinary and missing-provenance positive paths.
- Amendments 1–2 root/version, exact current-session basis, shared fixture, Rust typed/fail-closed, classification, and canonical HTML package-output gates remain intact.

Production-defect disposition: **FIXED IN AUTHORIZED SEAM**, pending fresh governed read-only review over the complete amended N2 diff.

## Amendment 3 changed paths

- `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.ts`
- `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.test.ts`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/TASK-IMPLEMENT/AMENDMENT_RETURN_3.md`

No earlier TASK record, DEL-08-01/manager/handoff/FAN_IN, N1/N3, Rust, fixture, schema, dependency, policy, Git, receipt, or PR surface was modified by this amendment.

## Final complete N2 product/test freeze

Basis: Git `357a58b56726feba49507534159c3fbc4656b818`.

- `apps/desktop/src/features/report/reportPackageRequest.ts` — SHA-256 `514b0582eb56a233b7445ca53b67bef0e903bde4b36acca6f8f0491d3dd0db99` (`11` additions, `2` deletions from basis).
- `apps/desktop/src/features/report/reportPackageRequest.test.ts` — SHA-256 `c3fe9583ae9329761a186eb8e6209491bec09c584298189ad9d87ee323593437` (`109` additions, `2` deletions from basis).
- `core/reporting/report_package/src/wire.rs` — SHA-256 `4a04fbde7b50515b00d55dde7438497fa7c5fe1d433bb7fc7d9ff5ee771489e1` (`136` additions from basis).
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json` — SHA-256 `3b20700718c3e5ee87ddf242637f4bb997f38617115bb7ccc2cba89cbf51d110` (`104` additions from basis).

## Checks

- `npm test --workspace apps/desktop -- --run src/features/report/reportPackageRequest.test.ts` — PASS: 1 file, 8 tests.
- `npm run build:desktop` — PASS: registered desktop `tsc -b` typecheck and Vite production build completed; only the existing non-blocking chunk-size advisory was emitted.
- `cargo test --manifest-path core/reporting/report_package/Cargo.toml` — PASS: 7 unit tests, 12 container integration tests, and doc tests with 0 failures.
- `cargo fmt --manifest-path core/reporting/report_package/Cargo.toml -- --check` — PASS.
- `python3 tools/software_workflow/validate_change_scope.py ... --path <four complete N2 product/test paths>` — PASS: zero violations.
- `git diff --check 357a58b... -- <four complete N2 product/test paths>` — PASS.

## Containment and blockers

- All implementation and evidence writes remain within the original sealed write targets.
- Blockers: none.
- Rerun trigger: any change to manifest verification/model payload, the canonical hash service, report-package request construction, the shared projection, Rust wrappers/DTO, or assembly path requires the focused TypeScript and Rust suites, registered desktop build, and fresh full-N2 review again.
