# WORKING_ITEMS_RUN - TP-VALIDMANUAL-RUNNERREPRO-001

**Date:** 2026-07-05
**Package:** PKG-09 - Verification, Validation, and Quality Oracles
**Primary deliverable:** DEL-09-04 - Validation manual skeleton
**Supporting deliverable:** DEL-10-05 - Headless CLI and structured I-O analysis runner
**Decision basis:** `DEC-065` / D-33 O-A
**Phase basis:** Phase E row E2 validation manual, following E1 merge

## Scope

Create a draft validation-manual reproduction slice for the current local
`openpipestress-runner` CLI evidence without changing runner behavior,
settling release thresholds, opening hosted/public transport, or making
release/professional claims.

## Implemented

- Added `docs/validation_manual/headless_runner_reproduction.md`.
- Added `validation/witness/inputs/generate_tp_runner_015_inputs.py`.
- Generated:
  - `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json`
  - `validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json`
  - `validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json`
- Updated `docs/validation_manual/index.md` and this deliverable `MEMORY.md`
  to point to the draft evidence slice.

## Evidence Commands

```bash
python3 validation/witness/inputs/generate_tp_runner_015_inputs.py
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- solve --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json --output /tmp/tp_runner_015_solve.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- validate-input --input validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json --output /tmp/tp_runner_015_validation_blocking.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json --output /tmp/tp_runner_015_benchmark_stub.json
```

Observed outcomes: solve exited 0; validation-blocking exited 1 with the
expected request diagnostic; benchmark-stub exited 1 with the expected
downstream-payload diagnostic.

## Boundaries Preserved

- No DEL-09-04 lifecycle transition.
- No public benchmark or release threshold settlement.
- No clean-environment R5 exit demonstration claim.
- No release readiness, release packaging/signing/publication, hosted CI,
  public transport, protected-content/legal clearance, professional approval,
  certification, sealing, authentication, or code-compliance claim.
