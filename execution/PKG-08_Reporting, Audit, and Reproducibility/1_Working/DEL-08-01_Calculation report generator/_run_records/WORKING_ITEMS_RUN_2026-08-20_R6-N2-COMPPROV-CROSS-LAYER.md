# WORKING_ITEMS Run — R6 N2 component provenance cross-layer proof

Run: `HELP-HUMAN-PIPING-20260820-R6-ENGINEERING`
Instance: `WI-PKG08-DEL0801`
Package / deliverable: `PKG-08 / DEL-08-01`
Accepted basis: `357a58b56726feba49507534159c3fbc4656b818`, DAG-009, R5

## Result

Closed the exact Remaining item “Add the cross-layer TypeScript-to-Rust component-provenance test.” A shared invented projection is matched exactly by production `buildReportPackageRequest`, consumed by Rust through `ReportPackageRequest` and `assemble_wire_request`, and asserted in the canonical rendered HTML package member. The missing-provenance mutation occurs before mechanics, input-manifest, and analysis-run construction; the complete manifest model payload equals the report model and carries the same missing provenance; and the analysis run binds that manifest ref/hash.

The proof covers present component source evidence; missing provenance as `COMPONENT_PROVENANCE_MISSING`, `missing_data_finding=true`, warning/non-accepted behavior; preserved `private_only`, `pending`, and `private_project_data`; malformed provenance rejection as `REPORT-PACKAGE-REPORT-DTO-INVALID`; and fail-closed fixture schema/root drift. Integrated review v8 exposed a production identity defect: same-ID but different model payloads could pass against a verified manifest. Amendment 3 closes it by requiring equality of canonical, validated hashes for the supplied model and verified manifest model payload before rendering or assembly, with same-ID changed-payload rejection and insertion-order equivalence regressions.

## Product/test paths

- `apps/desktop/src/features/report/reportPackageRequest.ts`
- `apps/desktop/src/features/report/reportPackageRequest.test.ts`
- `core/reporting/report_package/src/wire.rs`
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json`

## Evidence

- Focused Vitest: PASS, 8/8.
- Registered desktop build/typecheck: PASS.
- Report-package Cargo: PASS, 19/19 plus doc tests.
- Cargo fmt check: PASS.
- Write containment and diff checks: PASS.
- Fresh read-only review attempt 1 found the fixture root/version drift gap; bounded amendment closed it.
- Fresh read-only review attempt 2: PASS, no actionable findings, 100% final diff coverage.
- Integrated review v4 found an exact-session evidence-basis mismatch; Amendment 2 rebuilt every dependent evidence object after the model mutation and added complete manifest-payload parity assertions.
- Fresh full-N2 review attempt 3: PASS, no actionable findings, 100% of the complete final three-file diff from original basis.
- Integrated review v8 found the production same-ID/different-model-payload acceptance defect; Amendment 3 added the canonical identity gate and focused regressions.
- Fresh full-N2 review attempt 4: PASS, no actionable findings, 100% of the complete final four-file diff from original basis.
- Detailed briefs, returns, attempts, hashes, and telemetry: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/`.

## Closure boundary

Only the component-provenance test residual is closed. DEL-08-01 remains `IN_PROGRESS`; the separate `.opsproj` compatibility-window/versioning policy residual is unchanged. No lifecycle, register, DAG, decision, decomposition, PRD, receipt, professional-approval, release, commit, push, or PR action occurred.
