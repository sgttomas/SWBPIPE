# WORKING_ITEMS package return — PKG-08 / DEL-08-01

Run: `HELP-HUMAN-PIPING-20260820-R6-ENGINEERING`
Instance: `WI-PKG08-DEL0801`
Parent graph node: `N2`
Status: `COMPLETE / VALID FOR AGENT 0 FAN-IN`

## Coverage and accepted outputs

The exact DEL-08-01 Remaining item “Add the cross-layer TypeScript-to-Rust component-provenance test” is closed. Final product/test paths:

- `apps/desktop/src/features/report/reportPackageRequest.ts` (`514b0582eb56a233b7445ca53b67bef0e903bde4b36acca6f8f0491d3dd0db99`)
- `apps/desktop/src/features/report/reportPackageRequest.test.ts` (`c3fe9583ae9329761a186eb8e6209491bec09c584298189ad9d87ee323593437`)
- `core/reporting/report_package/src/wire.rs` (`4a04fbde7b50515b00d55dde7438497fa7c5fe1d433bb7fc7d9ff5ee771489e1`)
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json` (`3b20700718c3e5ee87ddf242637f4bb997f38617115bb7ccc2cba89cbf51d110`)

Production TypeScript output is exactly bound to the shared projection. The missing-provenance mutation precedes mechanics, manifest, and analysis-run construction; all dependent evidence derives from that model; the complete manifest model payload equals the report model and carries the exact missing provenance; and the analysis run binds that manifest ref/hash. Rust consumes the shared projection through the wire/package path and the canonical HTML member contains present source evidence and missing-provenance evidence. Malformed provenance, unsupported fixture versions, and unexpected fixture-root fields fail closed. Private/pending classifications are preserved. A production seam defect exposed by integrated review v8 is closed: before rendering or assembly, the producer now requires equality of canonical, validated hashes for the supplied model and verified manifest model payload, including same-ID changed-payload rejection and insertion-order equivalence.

## Validation

- Focused Vitest: PASS, 8/8.
- Registered desktop build/typecheck: PASS.
- Report-package Cargo: PASS, 19/19 plus doc tests.
- Cargo fmt check: PASS.
- Write containment and diff checks: PASS.
- Fresh review attempt 1: FAIL, one fixture-root/version drift finding.
- Bounded Amendment 1: PASS; finding fully remediated.
- Fresh review attempt 2: PASS, no actionable findings, 100% final diff.
- Integrated review v4: FAIL, exact session-basis mismatch found.
- Bounded Amendment 2: PASS; model mutation moved before all dependent evidence and exact manifest payload parity asserted.
- Fresh full-N2 review attempt 3: PASS, no actionable findings, 100% of the complete final three-file diff from original basis.
- Integrated review v8: FAIL, production accepted a same-ID changed model against a verified manifest payload.
- Bounded Amendment 3: PASS; canonical model/manifest payload identity gate and positive/negative/canonicalization regressions added.
- Fresh full-N2 review attempt 4: PASS, no actionable findings, 100% of the complete final four-file diff from original basis.
- Runtime summary: `RUNTIME_SUMMARY.json` — PASS, eight complete sessions, zero unmatched; exact token/context occupancy unavailable and recorded as a measurement limitation.

## Deliverable effect and residuals

DEL-08-01 `_STATUS.md`, `MEMORY.md`, and its run record were updated at fan-in. Lifecycle remains `IN_PROGRESS`. The separate `.opsproj` compatibility-window/versioning policy Remaining item is unchanged and excluded. No notices, waivers, human decisions, cross-package writes, or rerun requirements remain for N2.

Derivative status: the shared invented projection and AgentRuns evidence are derivative proof artifacts bound to accepted source/product state; they do not replace deliverable/decomposition truth.

Requested Agent 0 action: accept N2 into integrated tranche fan-in and route the scoped changes/evidence to CHANGE. No Git action was taken by this manager.
