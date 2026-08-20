# WORKING_ITEMS Run — R6 N2 component provenance cross-layer proof

Run: `HELP-HUMAN-PIPING-20260820-R6-ENGINEERING`
Instance: `WI-PKG08-DEL0801`
Package / deliverable: `PKG-08 / DEL-08-01`
Accepted basis: `357a58b56726feba49507534159c3fbc4656b818`, DAG-009, R5

## Result

Closed the exact Remaining item “Add the cross-layer TypeScript-to-Rust component-provenance test.” A shared invented projection is matched exactly by production `buildReportPackageRequest`, consumed by Rust through `ReportPackageRequest` and `assemble_wire_request`, and asserted in the canonical rendered HTML package member.

The proof covers present component source evidence; missing provenance as `COMPONENT_PROVENANCE_MISSING`, `missing_data_finding=true`, warning/non-accepted behavior; preserved `private_only`, `pending`, and `private_project_data`; malformed provenance rejection as `REPORT-PACKAGE-REPORT-DTO-INVALID`; and fail-closed fixture schema/root drift. No production defect was exposed and no production function changed.

## Product/test paths

- `apps/desktop/src/features/report/reportPackageRequest.test.ts`
- `core/reporting/report_package/src/wire.rs`
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json`

## Evidence

- Focused Vitest: PASS, 6/6.
- Report-package Cargo: PASS, 19/19 plus doc tests.
- Cargo fmt check: PASS.
- Write containment and diff checks: PASS.
- Fresh read-only review attempt 1 found the fixture root/version drift gap; bounded amendment closed it.
- Fresh read-only review attempt 2: PASS, no actionable findings, 100% final diff coverage.
- Detailed briefs, returns, attempts, hashes, and telemetry: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/`.

## Closure boundary

Only the component-provenance test residual is closed. DEL-08-01 remains `IN_PROGRESS`; the separate `.opsproj` compatibility-window/versioning policy residual is unchanged. No lifecycle, register, DAG, decision, decomposition, PRD, receipt, professional-approval, release, commit, push, or PR action occurred.
