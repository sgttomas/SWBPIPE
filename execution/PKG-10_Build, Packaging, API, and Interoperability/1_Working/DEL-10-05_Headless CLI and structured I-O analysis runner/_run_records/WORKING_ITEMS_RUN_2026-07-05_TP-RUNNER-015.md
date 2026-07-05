# WORKING_ITEMS_RUN - TP-RUNNER-015 Final Local Headless CLI

**Date:** 2026-07-05
**Package:** PKG-10 - Build, Packaging, API, and Interoperability
**Primary deliverable:** DEL-10-05 - Headless CLI and structured I/O analysis runner
**Decision basis:** `DEC-065` / D-33 O-A
**Scope:** Implement the final local `openpipestress-runner` CLI/process policy
needed by Phase E row E1, without release packaging, hosted/public transport,
CI activation, lifecycle issuance, professional approval, certification,
sealing, authentication, or code-compliance claim.

## Implemented

- Added explicit Cargo binary target `openpipestress-runner`.
- Preserved `headless_preview_runner` as the DEC-064 compatibility witness.
- Added stable local CLI verbs:
  - `solve`
  - `validate-input`
  - `export-results`
  - `run-benchmark`
  - `run-regression`
- Added schema-first CLI input handling:
  - stdin by default or caller-named `--input`;
  - structured JSON stdout by default;
  - optional explicit `--output` path;
  - no shorthand engineering-data flags.
- Added DEC-065 local policy output metadata:
  - foreground local process;
  - no network;
  - no daemon;
  - no telemetry;
  - no hidden filesystem mutation;
  - no direct SQL/SQLite bypass.
- Added stable downstream-operation stubs for `export-results`,
  `run-benchmark`, and `run-regression`; these return structured blocking
  diagnostics until later tranches supply the relevant payload bindings.
- Updated schema/tests/docs/app-facing packet mirrors to replace the old final
  CLI/process `TBD` posture with `SETTLED_DEC_065` where D-33 settled the
  local surface.

## Evidence Commands

Focused validation:

```bash
cargo fmt --manifest-path core/runner/headless/Cargo.toml --check
cargo test --manifest-path core/runner/headless/Cargo.toml
python3 tests/test_headless_runner_contract.py
```

Generated CLI evidence:

```bash
openpipestress-runner solve
```

- Input: invented preview-model payload embedded in a schema-first CLI request.
- Output: `validation/witness/generated/tp_runner_015_final_cli_solve.json`
- Exit: 0
- Summary: `COMPLETED`; request diagnostics 0; result diagnostics 0; runner
  diagnostics 0; result refs 822.

```bash
openpipestress-runner validate-input
```

- Input: invented request with missing `load_basis_refs`.
- Output:
  `validation/witness/generated/tp_runner_015_final_cli_validation_blocking.json`
- Exit: 1
- Summary: request diagnostics 1; no solver execution.

```bash
openpipestress-runner run-benchmark
```

- Input: invented benchmark request metadata.
- Output:
  `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`
- Exit: 1
- Summary: stable verb accepted; one structured diagnostic records that
  downstream benchmark payload binding remains future bounded work.

## Boundaries Preserved

- No release packaging, signing, notarization, publication, or release-readiness
  claim.
- No hosted CI activation, public transport protocol, external adapter format,
  persisted-project storage-root selection, or direct SQL/SQLite access.
- No protected standards content, proprietary values, private project data, or
  private rule-pack payload committed.
- No lifecycle issuance, professional approval, certification, sealing,
  authentication, or code-compliance acceptance.
- No tier-0 profile update, app-dev F3 opening, MCP/source-type binding, or live
  embedded-agent act.

## Remaining Work

- Persisted project input and storage-service binding.
- Full result-export payload execution for `export-results`.
- Benchmark/regression payload execution for `run-benchmark` and
  `run-regression`.
- Any package-script additions as development/test conveniences only.
- DEC-025 five-surface sweep before branch closeout.
