# Verification Results

Snapshot: `TP-INTEGRATED-VERIFY-002_2026-05-31`
Date: 2026-05-31

## Result Matrix

| Check | Command | Result | Evidence |
|---|---|---:|---|
| DEV-001 coordination derivative freshness | `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check` | PASS | Output: `VALID: DEV-001 coordination derivatives for DAG-005`. |
| Production and dev dependency audit | `npm audit --audit-level=moderate` | PASS | Output: `found 0 vulnerabilities`. |
| Release readiness profile | `python3 tools/release/check_release_readiness.py --profile all --execute` | PASS | 29 planned checks ran successfully. Includes DAG dependency schema validation, release readiness script tests, repository Python tests, coordination tool tests, security/privacy tests, and crate-local Cargo tests for 24 discovered manifests. |
| Desktop tests | `npm run test:desktop` | PASS | Vitest `4.1.7` ran 1 test file and 5 tests; all passed. |
| Desktop build | `npm run build:desktop` | PASS | TypeScript build and Vite `7.3.3` production build completed; Vite transformed 1601 modules and built in 2.27s. |
| Whitespace check | `git diff --check` | PASS | No whitespace errors reported. |
| Working tree intake state | `git status --short` | PASS | No output before snapshot creation. |

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

The prior desktop workspace bootstrap and dependency-maintenance closeouts
removed the previously observed local execution blocker:

- `npm run test:desktop` now runs successfully and passes 5 tests.
- `npm run build:desktop` now runs successfully and produces a Vite production
  build.
- `npm audit --audit-level=moderate` reports zero vulnerabilities.

## Interpretation

The May 31 bootstrap and dependency-maintenance gaps did not recur. The current
integrated verification tranche observed no open readiness gap in the executed
checks. This is not a release, acceptance, professional-reliance, or
code-compliance claim.
