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
It uses invented public-metadata fixtures only. It is development verification
and screening evidence, not a release-readiness record; validation occurs in
the user's accepted professional tools, and acceptance and professional
judgment remain with the responsible engineer.

As of 2026-07-19 this slice has two parts: the frozen E1 `tp_runner_015`
procedure (with a dated historical note on case 3), and the bound per-case
`run-benchmark` / `run-regression` reproduction path landed by PR #287
(`DEL-10-05`, implementation commit `60841413a`, merged at `45ec0524d`).

## Authority And Boundaries

| Item | Current record |
|---|---|
| Primary deliverable exercised | `DEL-10-05` |
| Manual deliverable | `DEL-09-04` |
| Decision basis | `D-33` / `DEC-065` |
| Runner policy | local foreground process; JSON stdin or `--input`; JSON stdout; optional explicit `--output`; no network, daemon, telemetry, hidden filesystem mutation, or direct SQL access |
| Exit-code policy (`DEC-065`) | exit 0 only when no blocking diagnostic is present; exit 1 with blocking diagnostics; exit 2 reserved for usage/malformed input |
| Evidence state | `DRAFT_EVIDENCE` |
| Public benchmark/release tolerance state | `TBD`; not settled by this slice |

## Part 1 — Frozen E1 Procedure (`tp_runner_015`)

### Fixture Set

| Case | Input fixture | Expected command | Expected exit | Expected evidence |
|---|---|---|---:|---|
| Valid invented preview solve | `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json` | `openpipestress-runner solve --input <fixture> --output <out>` | 0 | `runner_result.job.state` is `COMPLETED`; request/result validation diagnostics are empty; `runner_result.result_refs` is non-empty. |
| Invalid request metadata | `validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json` | `openpipestress-runner validate-input --input <fixture> --output <out>` | 1 | Request validation reports `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; no solver result is emitted. |
| Benchmark verb on the frozen payload-less input (historical stub case) | `validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json` | `openpipestress-runner run-benchmark --input <fixture> --output <out>` | 1 | On sources at or after PR #287: runner diagnostics report `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` (the frozen input carries no `benchmark` payload). Historical, pre-#287 sources: `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`, as recorded in the committed witness. |

**Dated note (2026-07-19).** Case 3's stub expectation is historical for
sources at or after PR #287 (implementation commit `60841413a`, merged at
`45ec0524d`). PR #287 bound the `run-benchmark` and `run-regression` payloads
(`DEL-10-05`); on such sources the stub diagnostic
`HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` is emitted only
by `export-results`, and the same frozen case 3 command exits 1 with
`HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` because the frozen input contains
only a `request` object and no `benchmark` payload. The committed
`validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`
witness and prior reproduction bundles — including
`validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`
— remain truthful for their pinned pre-#287 commits and are not edited. The
three frozen input fixtures, their generator, and the three committed
generated witnesses remain byte-identical frozen surfaces.

The committed generated witness outputs are:

- `validation/witness/generated/tp_runner_015_final_cli_solve.json`
- `validation/witness/generated/tp_runner_015_final_cli_validation_blocking.json`
- `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`

### Reproduction Procedure (frozen E1)

From `projects/chirality-piping`:

```bash
python3 validation/witness/inputs/generate_tp_runner_015_inputs.py
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- solve --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json --output <scratch-output>/tp_runner_015_solve.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- validate-input --input validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json --output <scratch-output>/tp_runner_015_validation_blocking.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json --output <scratch-output>/tp_runner_015_benchmark_stub.json
```

The first command regenerates deterministic caller input fixtures from the
invented preview model fixture. The three runner commands write their JSON
result to stdout and to the explicit `--output` path. The two blocking cases
are expected to exit nonzero because they prove diagnostics, not successful
solve execution. On sources at or after PR #287 the third command's expected
diagnostic is `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`, per the dated note
above.

## Part 2 — Bound Per-Case Benchmark/Regression Reproduction (post-PR #287)

PR #287 (`CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001`) bound the
`run-benchmark` and `run-regression` downstream payloads to the committed
suite crates (`DEL-09-01` mechanics, `DEL-09-02` stress recovery, `DEL-09-03`
nonlinear support). `export-results` is the only runner verb that remains a
structured stub (bounded `DEL-10-05` work). The deterministic input source is
the committed generator
`validation/witness/inputs/generate_del1005_payload_binding_inputs.py`; it
writes only `del1005_payload_binding_*` input files and does not touch the
frozen `tp_runner_015` surfaces.

### Bound Fixture Set

| Case | Input fixture (`validation/witness/inputs/`) | Expected command | Expected exit | Expected evidence (committed witness) |
|---|---|---|---:|---|
| Benchmark, single named case | `del1005_payload_binding_benchmark_single_case_input.json` | `openpipestress-runner run-benchmark --input <fixture> --output <out>` | 0 | Suite `mechanics` (`DEL-09-01`); `requested_case_count` 1; case `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` `executed_and_matched` (1/1); `diagnostics` empty. |
| Benchmark, multiple named cases | `del1005_payload_binding_benchmark_multi_case_input.json` | `openpipestress-runner run-benchmark --input <fixture> --output <out>` | 0 | Suite `stress` (`DEL-09-02`); the three cases `STRESS-AXIAL-NORMAL-ORIGINAL`, `STRESS-RANGE-MECHANICS-ORIGINAL`, `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` all `executed_and_matched` (3/3); `diagnostics` empty. |
| Regression, whole-suite default | `del1005_payload_binding_regression_full_suite_input.json` | `openpipestress-runner run-regression --input <fixture> --output <out>` | 0 | Suite `nonlinear` (`DEL-09-03`); the input omits the `cases` list, so `whole_suite_default_applied` is `true`; 5/5 cases `executed_and_matched`; `diagnostics` empty. |
| Benchmark, payload missing | `del1005_payload_binding_benchmark_payload_missing_input.json` | `openpipestress-runner run-benchmark --input <fixture> --output <out>` | 1 | Input contains only a `request` object; blocking diagnostic `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`. |
| Regression, payload missing | `del1005_payload_binding_regression_payload_missing_input.json` | `openpipestress-runner run-regression --input <fixture> --output <out>` | 1 | Input contains only a `request` object; blocking diagnostic `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`. |

The three success witnesses serialize `"diagnostics": []` (an empty array —
no blocking diagnostic, hence exit 0 under the `DEC-065` exit policy). The
recorded exit codes for the five commands are 0, 0, 0, 1, 1 in table order,
as recorded in the R12 implementation evidence
(`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N3/RETURN.md`
§3) and reproduced against the committed witnesses.

The committed generated witness outputs are:

- `validation/witness/generated/del1005_payload_binding_benchmark_single_case.json`
- `validation/witness/generated/del1005_payload_binding_benchmark_multi_case.json`
- `validation/witness/generated/del1005_payload_binding_regression_full_suite.json`
- `validation/witness/generated/del1005_payload_binding_benchmark_payload_missing.json`
- `validation/witness/generated/del1005_payload_binding_regression_payload_missing.json`

### Reproduction Procedure (bound path)

From `projects/chirality-piping`:

```bash
python3 validation/witness/inputs/generate_del1005_payload_binding_inputs.py
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input validation/witness/inputs/del1005_payload_binding_benchmark_single_case_input.json --output <scratch-output>/del1005_benchmark_single_case.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input validation/witness/inputs/del1005_payload_binding_benchmark_multi_case_input.json --output <scratch-output>/del1005_benchmark_multi_case.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-regression --input validation/witness/inputs/del1005_payload_binding_regression_full_suite_input.json --output <scratch-output>/del1005_regression_full_suite.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --input validation/witness/inputs/del1005_payload_binding_benchmark_payload_missing_input.json --output <scratch-output>/del1005_benchmark_payload_missing.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-regression --input validation/witness/inputs/del1005_payload_binding_regression_payload_missing_input.json --output <scratch-output>/del1005_regression_payload_missing.json
```

Expected exits are 0, 0, 0, 1, 1 respectively. The two payload-missing cases
prove the fail-closed diagnostics, not successful suite execution.

### Per-Case Reporting and Fail-Closed Semantics

A bound suite run reports a `suite_run` object recording the suite, its
suite deliverable, the claim posture, the comparison basis, whether the
whole-suite default was applied (`cases` omitted), the requested case count,
and per-case entries with `executed_and_matched` / `executed_and_mismatched` /
`blocked` tallies. Cases whose recorded comparison basis is not reusable
through a crate-encoded predicate fail closed with
`HEADLESS_RUNNER_BENCHMARK_CASE_COMPARISON_BASIS_NOT_REUSABLE` /
`HEADLESS_RUNNER_REGRESSION_CASE_COMPARISON_BASIS_NOT_REUSABLE`, and a
whole-suite run containing blocked cases exits 1 — never a silent partial
skip. On the R12 implementation head (`60841413a`), whole-suite coverage
recorded: mechanics 11/21 `executed_and_matched` + 10 blocked; stress 12/15
`executed_and_matched` + 3 blocked; nonlinear 5/5 `executed_and_matched`
(R12 N3 return §2).

Per-case match/fail is regression evidence for current solver behavior
against the suite crates' recorded comparison values only. Blocked cases are
not a defect record and no figure above is a release judgment or an
acceptance criterion. Release thresholds, final tolerance policy, CI gate
policy, and professional reliance remain `TBD`, owner-gated (`DEC-046`
promotion untouched).

## Rerun Consequence

Per `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001` §8, this 2026-07-19 refresh
changes the documented reproduction surface: any subsequent clean-checkout
reproduction executes from a post-#287 source commit under a fresh run ID and
a new immutable bundle under `validation/evidence/reproduction/<run-id>/`
(`DEC-080`). Completed bundles — including
`REPRO_DEL0904_20260719T202023Z_23eeaabc9040/` — are not edited,
reinterpreted, or invalidated; they remain truthful for their pinned commits.

## Review Checks

Before treating this slice as maintainer-reviewed evidence, verify:

- `cargo test --manifest-path core/runner/headless/Cargo.toml` passes;
- `python3 tests/test_headless_runner_contract.py` passes;
- regenerated output preserves the expected command, operation, `DEC-065`
  policy fields, and diagnostic codes listed above;
- any comparison with committed generated outputs accounts only for intentional
  deterministic output-format changes reviewed in a bounded tranche.

## Recorded Reproduction Deltas

- 2026-07-10 (`TP-E2-VALMANUAL-001`): the three commands above were re-run at
  the then-current head. Exit codes and diagnostic codes matched this slice
  exactly. The regenerated caller input fixtures were byte-identical to the
  committed ones. The live `solve` output reported 830 `result_refs` versus
  the 822 recorded in the committed
  `validation/witness/generated/tp_runner_015_final_cli_solve.json` (solver
  result surface grew since the `TP-RUNNER-015` head); this slice's stated
  expectation (`result_refs` non-empty) still holds, and refreshing the
  committed generated witness outputs remains bounded `DEL-10-05` work.

## Remaining E2 Work

Per-case purpose/reference/tolerance/pass-fail records now live under
`docs/validation_manual/cases/` (see the [manual index](index.md)). This slice
still does not close `DEL-09-04` or Phase E row E2. Still open:
public-benchmark threshold disposition (owner-gated), `MAINTAINER_REVIEWED`
case-page promotion, GUI-workflow validation evidence, the `export-results`
payload binding (the only remaining structured runner stub; bounded
`DEL-10-05` work), and any human-gated release-quality or
professional-boundary review required at the R5 exit. Landed: the
`run-benchmark` / `run-regression` payload bindings (PR #287, `DEL-10-05`)
and the actor-neutral clean-checkout demonstration
(`validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`,
`INTERNALLY_VERIFIED`; reproduction acceptance remains an owner gate).
