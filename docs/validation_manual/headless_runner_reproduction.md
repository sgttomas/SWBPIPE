---
doc_id: OPS-VALIDATION-MANUAL-HEADLESS-RUNNER-REPRODUCTION
doc_kind: governance.validation_manual_section
status: draft_evidence
created: 2026-07-05
refs:
  - rel: implements
    to: DEL-09-04
  - rel: exercises
    to: DEL-10-05
  - rel: decision_basis
    to: DEC-065
---

# Headless Runner Reproduction Slice

## Scope

This draft validation-manual slice records the current external-reproducibility
path for the local `openpipestress-runner` CLI implemented under `DEC-065`.
It uses invented public-metadata fixtures only. It is software-quality evidence,
not a release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Authority And Boundaries

| Item | Current record |
|---|---|
| Primary deliverable exercised | `DEL-10-05` |
| Manual deliverable | `DEL-09-04` |
| Decision basis | `D-33` / `DEC-065` |
| Runner policy | local foreground process; JSON stdin or `--input`; JSON stdout; optional explicit `--output`; no network, daemon, telemetry, hidden filesystem mutation, or direct SQL access |
| Evidence state | `DRAFT_EVIDENCE` |
| Public benchmark/release tolerance state | `TBD`; not settled by this slice |

## Fixture Set

| Case | Input fixture | Expected command | Expected exit | Expected evidence |
|---|---|---|---:|---|
| Valid invented preview solve | `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json` | `openpipestress-runner solve --input <fixture> --output <out>` | 0 | `runner_result.job.state` is `COMPLETED`; request/result validation diagnostics are empty; `runner_result.result_refs` is non-empty. |
| Invalid request metadata | `validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json` | `openpipestress-runner validate-input --input <fixture> --output <out>` | 1 | Request validation reports `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; no solver result is emitted. |
| Stable benchmark verb, downstream payload missing | `validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json` | `openpipestress-runner run-benchmark --input <fixture> --output <out>` | 1 | Runner diagnostics report `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`. |

The committed generated witness outputs are:

- `validation/witness/generated/tp_runner_015_final_cli_solve.json`
- `validation/witness/generated/tp_runner_015_final_cli_validation_blocking.json`
- `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`

## Reproduction Procedure

From `projects/chirality-piping`:

```bash
python3 validation/witness/inputs/generate_tp_runner_015_inputs.py
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- solve --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json --output /tmp/tp_runner_015_solve.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- validate-input --input validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json --output /tmp/tp_runner_015_validation_blocking.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json --output /tmp/tp_runner_015_benchmark_stub.json
```

The first command regenerates deterministic caller input fixtures from the
invented preview model fixture. The three runner commands write their JSON
result to stdout and to the explicit `--output` path. The two blocking cases
are expected to exit nonzero because they prove diagnostics, not successful
solve execution.

## Review Checks

Before treating this slice as maintainer-reviewed evidence, verify:

- `cargo test --manifest-path core/runner/headless/Cargo.toml` passes;
- `python3 tests/test_headless_runner_contract.py` passes;
- regenerated output preserves the expected command, operation, `DEC-065`
  policy fields, and diagnostic codes listed above;
- any comparison with committed generated outputs accounts only for intentional
  deterministic output-format changes reviewed in a bounded tranche.

## Remaining E2 Work

This slice does not close `DEL-09-04` or Phase E row E2. Remaining work includes
per-benchmark purpose/reference/tolerance/pass-fail records, public-benchmark
threshold disposition, broader benchmark/regression payload bindings, a clean
environment demonstration record, and any human-gated release-quality or
professional-boundary review required at the R5 exit.
