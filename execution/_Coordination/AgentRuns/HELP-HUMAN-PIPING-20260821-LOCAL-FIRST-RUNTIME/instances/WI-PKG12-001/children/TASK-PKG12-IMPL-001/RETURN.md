# TASK-PKG12-IMPL-001 Accepted Return

- Status: `ACCEPTED_AFTER_AMENDMENT_v2`
- Coverage: DEL-12-01 Remaining item 1, current applicable controlled adapter/result/report export and report-package persistence routes.
- ProductEffect: canonical Python local-first storage guard now supplies metadata-only route admission to every core `control_route_export` consumer; the desktop mirror gates every `controlRouteExport` / `ControlledExportLink` consumer; report-package save rejects missing/blocked evidence in both TypeScript and the native Rust command before assembly/file-picker access.
- Compatibility: known public/shared/downstream routes retain redaction behavior; `public_example` remains supported; local-private routes require wrapper-owned explicit intent; unknown route IDs fail closed.
- Boundaries: no payload inspection by the local-first route decision; no cloud/network/telemetry operation; no root/container selection; no storage-service or plugin-runtime selection.
- ChangedProductTestPaths: 12 paths listed in `../../FROZEN_HASH_INVENTORY.md`.
- FocusedChecks: Python security 31 passed; desktop focused 48 passed; desktop build PASS; Rust local-first validator 3 passed; writer-focused Python 9 passed; cargo fmt, diff check, and write containment PASS.
- RegisteredChecks: desktop-test PASS; desktop-build PASS; harness-pytest PASS; harness-self-check PASS. `piping-pytest` and `evidence-sweep` were executed and failed only because the host Python lacks required `jsonschema>=4,<5`; evidence sweep completed all preceding non-Python surfaces before the dependency-only Python collection failure. Exact normalized evidence is under `../../_run_records/`.
- Residual: terminal fresh read-only 100%-diff review is pending; full Python/evidence sweep must rerun on a dependency-complete surface or after the owning tranche supplies its standard dev dependency environment.
- ModelAttribution: `Codex, GPT-5 family`; no more-specific runtime slug exposed.
