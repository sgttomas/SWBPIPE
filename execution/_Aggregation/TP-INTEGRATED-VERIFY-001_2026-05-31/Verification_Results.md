# Verification Results

Snapshot: `TP-INTEGRATED-VERIFY-001_2026-05-31`
Date: 2026-05-31

## Result Matrix

| Check | Command | Result | Evidence |
|---|---|---:|---|
| DEV-001 coordination derivative freshness | `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check` | PASS | Output: `VALID: DEV-001 coordination derivatives for DAG-005`. |
| Release readiness profile | `python3 tools/release/check_release_readiness.py --profile all --execute` | PASS | Planned 29 checks ran successfully. Includes DAG dependency schema validation, release readiness script tests, all repository Python tests, coordination tool tests, security/privacy tests, and crate-local Cargo tests for 24 discovered manifests. |
| Desktop tests | `npm run test:desktop` | BLOCKED | Fails before tests execute: `sh: vitest: command not found`. Root and desktop `package.json` are present; `node_modules` and `apps/desktop/node_modules` are absent. |
| Desktop build | `npm run build:desktop` | BLOCKED | Fails before build executes: `sh: tsc: command not found`. Root and desktop `package.json` are present; `node_modules` and `apps/desktop/node_modules` are absent. |

## Release Readiness Profile Detail

The release readiness script discovered 24 Cargo manifests and planned 29
checks. The run completed with all planned checks passing:

- DAG dependency schema validation: PASS.
- `tests/test_release_readiness_script.py`: 5 passed.
- Repository Python contract tests: 269 passed.
- `tests/test_coordination_maintenance.py`: 4 passed.
- `tests/security`: 31 passed.
- Cargo crate-local tests: all discovered manifests passed.

The Cargo surfaces covered:

- `core/gui/viewport_editor`
- `core/loads/load_case_algebra`
- `core/loads/primitive_loads`
- `core/loads/stress_recovery`
- `core/loads/user_loads`
- `core/product_physics`
- `core/reporting/audit_manifest`
- `core/reporting/protected_content_linter`
- `core/reporting/report_generator`
- `core/reporting/report_sections`
- `core/reporting/result_export`
- `core/rules/completeness_checker`
- `core/rules/expression_evaluator`
- `core/rules/rule_pack_lifecycle`
- `core/runner/headless`
- `core/solver/diagnostics`
- `core/solver/frame_kernel`
- `core/solver/linear_supports`
- `core/solver/nonlinear_supports`
- `core/solver/performance_harness`
- `core/solver/straight_pipe`
- `validation/benchmarks/mechanics`
- `validation/benchmarks/nonlinear`
- `validation/benchmarks/stress`

## Desktop Supplemental Detail

The desktop package is declared under the root npm workspace:

- Root package: `package.json`
- Desktop package: `apps/desktop/package.json`
- Root lockfile: `package-lock.json`

The desktop package declares `vitest` and `typescript` in `devDependencies`,
but local `node_modules` directories were not present during this verification
run. The supplemental desktop commands therefore failed at shell command
resolution before exercising application tests or builds.

## Interpretation

The prior DEL-11-04 schema-example residual appears locally remediated under
the provider-neutral readiness profile. The remaining observed gap is not a
domain/schema failure; it is a local desktop toolchain/bootstrap gap required
before desktop tests and build can provide evidence.
