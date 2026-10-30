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
the user's accepted professional tools.

As of 2026-07-19 this slice has two parts: the frozen E1 `tp_runner_015`
procedure (with dated historical notes on cases 1 and 3), and the bound per-case
`run-benchmark` / `run-regression` reproduction path landed by PR #287
(`DEL-10-05`, implementation commit `60841413a`, merged at `45ec0524d`).

As of 2026-10-23 the [current runnable procedure](#current-runnable-procedure-2026-10-23)
below is the one to follow on today's sources. Part 1 is kept verbatim as a
historical frozen procedure; Part 2's bound commands are reused by the current
procedure unchanged.

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

## Current Runnable Procedure (2026-10-23)

Written from the runner source and its tests at `origin/main` `62c1efd75b`:
`core/runner/headless/src/bin/openpipestress-runner.rs`,
`core/runner/headless/src/redaction_binding.rs` and
`tests/test_headless_runner_contract.py`. The expected exits and diagnostics
below are what that source and those tests state. No run of this procedure
is recorded here; a recorded run is the reproduction bundle described at the
end of this section.

### Runner interface

```text
openpipestress-runner <solve|validate-input|export-results|run-benchmark|run-regression>
    [--input <request.json>|-] [--output <result.json>]
    [--explicit-local-private-intent]
    [solve only: --solver-mode <sparse_interactive|dense_scrutiny>]
openpipestress-runner read <reader> [--input <reader-input.json>|-]
```

- **Input.** One JSON object, from `--input` or stdin. It carries a `request`
  object whose `operation` matches the verb, plus the verb's payload: `solve`
  needs `solve.preview_model` (`{"model": …, "materials": […]}`),
  `run-benchmark` needs `benchmark.suite` and `run-regression` needs
  `regression.suite` (each with an optional `cases` list; omitted means the
  whole suite), and `export-results` needs `export_results`.
  `validate-input` needs only `request`.
- **Output wrapper.** Once the input is read, stdout is the local-private
  export control object: `payload`, `decisions`, `findings`, `blocked`,
  `summary`. The runner's own output document is `payload`. Argument errors
  print a message and the usage text to stderr and exit 2; an unreadable
  input file prints only the read error to stderr and exits 2. Neither
  writes stdout.
- **Local-private intent.** Without `--explicit-local-private-intent`, an
  output that carries values classified private is blocked: `blocked` is
  `true`, `payload` is `null`, the findings carry
  `LOCAL_PRIVATE_INTENT_REQUIRED`, and the exit is 1. Every command in this
  procedure passes the flag except the step that shows this control.
- **`--output`.** The file holds the same JSON as stdout. It is written
  only when the export is not blocked, and for `export-results` only on
  exit 0.
- **Exit codes.** 0 when no blocking diagnostic is present and the export
  is not blocked; 1 with a blocking diagnostic or a blocked export; 2 for a
  usage error, an unreadable input file, input that is not a JSON object of
  the expected shape (`HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID`), an
  unsupported verb (`HEADLESS_RUNNER_CLI_UNSUPPORTED_COMMAND`) or an
  unwritable `--output` path.
- **`read`.** Returns one product reader's verdict as JSON on stdout. It
  exits 0 when the reader passes the statement, 1 when it refuses it, and 2
  when the invocation or input is unusable. It takes no `--output` or intent
  flag. The reader names are in the usage text (`--help`, exit 2) and
  `core/runner/headless/src/reader_binding.rs`.

### Setup

From `projects/chirality-piping`, with a scratch directory outside the
checkout:

```bash
SCRATCH=$(mktemp -d)
RUNNER=$(python3 tools/runner/build_runner.py --target-dir "$SCRATCH/target" | tail -n 1)
I=validation/witness/inputs
```

`tools/runner/build_runner.py` runs `cargo build --locked --release --bin
openpipestress-runner` and prints the binary path. A binary already built at
the same commit may be used instead (`RUNNER=<path>`).

Two inputs are composed in the scratch directory; nothing is written to the
checkout. The success-path solve wraps the model the current product solves,
`fixtures/model_operations/exact_pressure_authoring_model.json`, in the
frozen Part 1 `solve` request (the 2026-10-19 dated note's "current success
path"):

```bash
python3 - "$SCRATCH/current_solve_input.json" <<'EOF'
import json, sys
frozen = json.load(open("validation/witness/inputs/tp_runner_015_final_cli_solve_input.json"))
model = json.load(open("fixtures/model_operations/exact_pressure_authoring_model.json"))
body = {"request": frozen["request"], "solve": {"preview_model": {"model": model, "materials": []}}}
json.dump(body, open(sys.argv[1], "w"))
EOF
```

The `del1005_export_results_*` inputs are version 2.0.0 package requests.
`core/runner/headless/tests/del1005_package_witness.rs` generates the success
input and the alias overlay (the alias input as leaf replacements over the
success input) from a real headless solve against the product-physics
component the runner links, so they record its identity.
`validation/witness/inputs/generate_del1005_export_results_inputs.py` derives
the committed missing-payload input from the success input, and, with
`--out "$SCRATCH"`, also the binding-mismatch and producer-blocked inputs, one
edit each; those two are not committed, and the contract tests make the same
edits in memory. The runner refuses any other
identity. The contract tests assert that the committed identity is the linked
one, so a component version change fails them until the generator is re-run.
The result document is self-hashed and validated against its source, so its
identity cannot be rewritten in place. The stale-identity control C12 changes
only the audit manifest's identity:

```bash
python3 - "$SCRATCH/export_stale_identity_input.json" <<'EOF'
import json, sys
body = json.load(open("validation/witness/inputs/del1005_export_results_success_input.json"))
body["export_results"]["audit_manifest"]["solver_version"]["solver_version"] = "0.1.0"
json.dump(body, open(sys.argv[1], "w"))
EOF
```

### Steps

| Step | Command (after setup) | Exit | Expected evidence | Basis |
|---|---|---:|---|---|
| C1 | `"$RUNNER" validate-input --explicit-local-private-intent --input $I/tp_runner_015_final_cli_validation_blocking_input.json --output "$SCRATCH/c01.json"` | 1 | Request validation reports `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; no solver result. The output file is written, because the export is not blocked. | Part 1 case 2 |
| C2 | `"$RUNNER" solve --explicit-local-private-intent --input "$SCRATCH/current_solve_input.json" --output "$SCRATCH/c02.json"` | 0 | `blocked` is `false`; `runner_result.job.state` is `COMPLETED`; `mechanics_envelope.status.mechanics` is `MECHANICS_SOLVED`; `runner_result.result_refs` is non-empty; no blocking request or result validation diagnostic. | `test_final_runner_subprocess_covers_every_active_verb_and_stdout`; 2026-10-19 note |
| C3 | `"$RUNNER" solve --input "$SCRATCH/current_solve_input.json" --output "$SCRATCH/c03.json"` (no intent flag) | 1 | `blocked` is `true`, `payload` is `null`, findings carry `LOCAL_PRIVATE_INTENT_REQUIRED`; no `c03.json` is written. | `test_final_runner_subprocess_blocking_exit_one_writes_no_file` |
| C4 | `"$RUNNER" solve --explicit-local-private-intent --input $I/tp_runner_015_final_cli_solve_input.json --output "$SCRATCH/c04.json"` | 1 | The legacy-content refusal control (Part 1 case 1): job state `COMPLETED`, mechanics `MODEL_INCOMPLETE`, blocking `PRESSURE_MODEL_REAUTHOR_REQUIRED` for each legacy nonzero pressure load and `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`, result validation `HEADLESS_RUNNER_RESULT_REFS_MISSING`, empty `result_refs`. | `test_final_runner_refuses_legacy_pressure_demo_as_explicit_control`; 2026-10-19 note |
| C5 | `"$RUNNER" run-benchmark --explicit-local-private-intent --input $I/tp_runner_015_final_cli_benchmark_stub_input.json --output "$SCRATCH/c05.json"` | 1 | `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` (Part 1 case 3). | Part 1 case 3; source |
| C6–C10 | The five Part 2 commands, with `"$RUNNER"` in place of `cargo run … --` and outputs under `"$SCRATCH"` | 0, 0, 0, 1, 1 | As the Part 2 fixture table states. | Part 2 |
| C11 | `"$RUNNER" export-results --explicit-local-private-intent --input $I/del1005_export_results_success_input.json --output "$SCRATCH/c11.json"` | 0 | `payload.report_package` is present, with `container_bytes` whose SHA-256 equals `container_sha256_hex`; a repeat run gives the same bytes. | `test_final_runner_subprocess_export_results_success_is_deterministic_and_zip_exact` |
| C12 | `"$RUNNER" export-results --explicit-local-private-intent --input "$SCRATCH/export_stale_identity_input.json" --output "$SCRATCH/c12.json"` | 1 | The stale identity is refused: `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID` with `REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH` in its message; no `report_package`; no `c12.json`. | `test_final_runner_export_results_refuses_unlinked_solver_identity` |
| C13 | `printf '{' \| "$RUNNER" validate-input --explicit-local-private-intent` | 2 | `HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID`. | `test_final_runner_subprocess_malformed_input_exits_two` |

The other `export-results` inputs (`$SCRATCH` copies from the generator's
`--out`) behave as follows. Missing payload gives
`HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING`, binding mismatch gives
`HEADLESS_RUNNER_EXPORT_RESULTS_BINDING_MISMATCH`, and producer blocked gives
`HEADLESS_RUNNER_EXPORT_RESULTS_PACKAGE_BLOCKED`. All three exit 1 and write
no file. The alias input, whose request names the model by another reference,
succeeds like C11 (`export_results_binds_the_request_alias_not_a_second_actual_identity`
applies the overlay).

A whole-suite `run-benchmark` or `run-regression` (input without `cases`)
exits 1 whenever any case is blocked, under the fail-closed semantics below.
This procedure records no whole-suite figure for today's sources; the
figures in Part 2 are pinned to their commits.

### Review checks

```bash
cargo test --manifest-path core/runner/headless/Cargo.toml
OPENPIPESTRESS_NO_CARGO_BUILD=1 OPENPIPESTRESS_RUNNER_BIN="$RUNNER" \
  OPENPIPESTRESS_PREVIEW_RUNNER_BIN=<headless_preview_runner built at the same commit> \
  python3 -m pytest -q tests/test_headless_runner_contract.py
```

The session authorities have their own presets (`OPENPIPESTRESS_UNITS_BIN`,
`OPENPIPESTRESS_CHECKED_JSON_BIN`; see `tests/conftest.py`). Under
`OPENPIPESTRESS_NO_CARGO_BUILD=1` a test whose binary is absent is skipped
with the reason, so a reproduction run reports skips as well as passes.
Without that variable the pytest run builds missing binaries itself.

### Fresh reproduction bundle (to do, heavy lane)

The last bundle is
`validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/`,
pinned to `a5235340aae3`. It ran the Part 1 and Part 2 commands under their
then-current expectations. No reproduction has run since 2026-07-20. A fresh
run against the PRD §24 R6 reproduction exit criterion (as amended by
`DEC-080`; historically cited as v0.1 R5) needs:

- a clean local clone detached at one pinned commit, with the runner, the
  `headless_preview_runner` and the session authorities built at that
  commit (offline cargo, target directory outside the checkout);
- the setup and steps C1–C13 above, then the review checks, each with
  recorded argv, working directory, environment, tool versions, stdout,
  stderr, exit code and output SHA-256, and clean `git status` before and
  after;
- a new immutable bundle `validation/evidence/reproduction/REPRO_DEL0904_<UTC>_<commit12>/`
  in the layout of the last one (`README.md`, `manifest.json`,
  `commands.jsonl`, `environment.txt`, `exit_codes/`, `stdout/`, `stderr/`,
  `outputs/`, `SHA256SUMS.txt`, `source_commit.txt`, `procedure.sha256`);
- the brief that governs the run, from the manager. Reproduction acceptance
  remains the owner's.

Earlier bundles are not edited. They remain truthful for their pinned
commits.

## Part 1 — Frozen E1 Procedure (`tp_runner_015`)

> **Historical frozen procedure.** Part 1 is kept verbatim for the E1
> record and its committed witnesses; do not run it as written on current
> sources. Its commands omit `--explicit-local-private-intent`, and its case 1
> solve is refused (dated notes below). The
> [current runnable procedure](#current-runnable-procedure-2026-10-23) covers
> the same three inputs as steps C1, C4 and C5.

### Fixture Set

| Case | Input fixture | Expected command | Expected exit | Expected evidence |
|---|---|---|---:|---|
| Invented preview solve (historical success case; now a legacy-content refusal) | `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json` | `openpipestress-runner solve --input <fixture> --output <out>` | 1 (historically 0) | On current sources, per the dated 2026-10-19 note below: the solve is refused by name with blocking `PRESSURE_MODEL_REAUTHOR_REQUIRED` and `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED` diagnostics and `HEADLESS_RUNNER_RESULT_REFS_MISSING`; `runner_result.result_refs` is empty. Historical, as recorded in the committed witness: `runner_result.job.state` is `COMPLETED`; request/result validation diagnostics are empty; `runner_result.result_refs` is non-empty; on sources at or after R14-W1 T2, the solve result envelope additionally carries one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning for `support:CE-120`, per the dated 2026-07-20 note below. |
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
three committed generated witnesses and the generator remain byte-identical
frozen surfaces. The input fixtures are re-authored when the model document
version changes: the solve input's model `schema_version` token moved to
0.5.0 at B7a, to 0.6.0 at T4-S1, to 0.7.0 at W-13 S1b and to 0.8.0 at W-15
slice c, and the generator
reproduces the current inputs byte for byte from the invented preview model
fixture.

**Dated note (2026-07-20).** Case 1's committed-witness byte expectation is
historical for sources at or after R14-W1 T2 (implementation commit
`faee4faed`, on main at `581a15b1c`, PR #292, Receipt-61). On such sources
the same frozen solve command still exits 0 with `runner_result.job.state`
`COMPLETED`, empty request/result validation diagnostics, and non-empty
`runner_result.result_refs`, and the solve result envelope additionally
carries one non-blocking warning diagnostic
`SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` for `support:CE-120` (whose frozen
`restraints` list is empty — a deliberately non-consuming shape) plus
updated sign-convention disclosure text on its two
`constant_effort_user_input_review` rows; therefore regenerated output no
longer byte-matches the committed
`validation/witness/generated/tp_runner_015_final_cli_solve.json` witness,
which — like the prior reproduction bundles — remains truthful for its
pinned pre-T2 commits and is not edited. The non-consumption warning is
review evidence about a non-consuming user-entered constant-effort data
shape; it is not a defect, not a solve error, and not a new acceptance
criterion.

**Dated note (2026-10-19).** Case 1's success expectation is historical. On
current sources the frozen solve command exits 1, and the refusal is correct.
Re-run on `main` at `084ff961f8`:

- *Local-private intent.* Since `9457565c2c` (2026-07-22) the runner wraps its
  CLI output in the local-private export control described in Part 2. Without
  `--explicit-local-private-intent`, each of the three frozen commands exits 1
  with blocking `LOCAL_PRIVATE_INTENT_REQUIRED` findings. The payload is
  withheld and no `--output` file is written. With the flag, cases 2 and 3
  reproduce their expected exit and diagnostic above.
- *Case 1 refusal.* With the flag, case 1 still exits 1. The job state is
  `COMPLETED`, but mechanics status is `MODEL_INCOMPLETE` and
  `runner_result.result_refs` is empty. The mechanics envelope carries blocking
  diagnostics:
  - four `PRESSURE_MODEL_REAUTHOR_REQUIRED`, for the legacy pressure primitives
    `L-100-P`, `L-100-P-EJ`, `L-200-P` and `L-200-P-EJ`;
  - one `LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED`, for the legacy four-rate
    expansion joint `component:C-150` on `pipe:P-130`.

  Result validation adds the blocking `HEADLESS_RUNNER_RESULT_REFS_MISSING`
  diagnostic.
- *Why.* The input's model `schema_version` is the current `0.8.0`, so this is
  not a version refusal. The generator copies the bundled legacy demo
  `fixtures/product_preview/invented_preview_model.json`, whose *content* uses
  retired contracts. Legacy nonzero pressure loads have been refused since
  `22452ecd14` (2026-09-25). T3 U3 (`8a12de28db`, 2026-10-08) retired the legacy
  pressure contract product-wide, with zero values included. T4-U3
  (`a601e3706d`, 2026-10-10) refuses legacy finite-span joints. Under the
  owner's no-compatibility direction (DEL-04-08 `Work/b7-plan.md`, "Decisions
  applied"), retired content is refused by name and not migrated.
- *What stays unchanged.* `tests/test_headless_runner_contract.py` keeps this
  demo as the explicit legacy-pressure refusal control and asserts this exit-1
  refusal. The case 1 input is therefore not re-authored. The generator, the
  three inputs, the committed generated witnesses and the prior reproduction
  bundles remain truthful for their pinned commits and are not edited.
- *Current success path.* The model the current product solves for a
  success-path `solve` is
  `fixtures/model_operations/exact_pressure_authoring_model.json`. The same
  test uses it unchanged. Wrapped in this procedure's `solve` request with
  `--explicit-local-private-intent`, it exits 0 at `084ff961f8` with job state
  `COMPLETED`, empty request and result validation diagnostics, and non-empty
  `result_refs`. This slice commits no input for that run.

The committed generated witness outputs are:

- `validation/witness/generated/tp_runner_015_final_cli_solve.json`
- `validation/witness/generated/tp_runner_015_final_cli_validation_blocking.json`
- `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`

### Reproduction Procedure (frozen E1)

From the repository root:

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
above. On sources at or after `9457565c2c`, add `--explicit-local-private-intent`
to each runner command to see its payload. On current sources the first
command is refused with exit 1, per the dated 2026-10-19 note.

## Part 2 — Bound Per-Case Benchmark/Regression Reproduction (post-PR #287)

PR #287 (`CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001`) bound the
`run-benchmark` and `run-regression` downstream payloads to the committed
suite crates (`DEL-09-01` mechanics, `DEL-09-02` stress recovery, `DEL-09-03`
nonlinear support). `export-results` was later bound to the report-package
payload (f82bb28e25; witnesses
`validation/witness/generated/del1005_export_results_*.json`, except the
success output, which embeds the whole package and is reproduced by C11
rather than committed). The
deterministic input source is the committed generator
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

From the repository root:

```bash
python3 validation/witness/inputs/generate_del1005_payload_binding_inputs.py
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --explicit-local-private-intent --input validation/witness/inputs/del1005_payload_binding_benchmark_single_case_input.json --output <scratch-output>/del1005_benchmark_single_case.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --explicit-local-private-intent --input validation/witness/inputs/del1005_payload_binding_benchmark_multi_case_input.json --output <scratch-output>/del1005_benchmark_multi_case.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-regression --explicit-local-private-intent --input validation/witness/inputs/del1005_payload_binding_regression_full_suite_input.json --output <scratch-output>/del1005_regression_full_suite.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-benchmark --explicit-local-private-intent --input validation/witness/inputs/del1005_payload_binding_benchmark_payload_missing_input.json --output <scratch-output>/del1005_benchmark_payload_missing.json
cargo run --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner -- run-regression --explicit-local-private-intent --input validation/witness/inputs/del1005_payload_binding_regression_payload_missing_input.json --output <scratch-output>/del1005_regression_payload_missing.json
```

Expected exits are 0, 0, 0, 1, 1 respectively. The two payload-missing cases
prove the fail-closed diagnostics, not successful suite execution.

The runner now wraps its CLI output in the local-private export control
(`payload`, `decisions`, `findings`, `blocked`, `summary`). Without
`--explicit-local-private-intent` that control blocks the export
(`LOCAL_PRIVATE_INTENT_REQUIRED`, exit 1) and no `--output` file is written.
With the flag, each output's `payload` member equals the committed witness
document as JSON (checked 2026-10-08 at the U3 branch head; the committed files
predate the wrapper and keep their own key order).

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

**Dated currency note (2026-07-20).** The whole-suite figures above are
pinned to the R12 implementation head (`60841413a`) and remain truthful for
that commit. At the R14 head (post-wave-2 main base `e315fb840`) the
mechanics suite carries 24 fixtures (readiness assertion
`fixtures.len() == 24`; the three R14 additions are
`MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD` (W1 T2),
`MECH-CURVED-BEND-PRESSURE-THRUST-ARC` (W1 T3), and
`MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` (W2 T4)), and the committed
derivative evidence bundle
`validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`
records the head whole-suite `run-benchmark` result: 24 cases, 11
`executed_and_matched` + 13 `blocked`, exit 1 under the `DEC-065`
fail-closed policy. That bundle is regression evidence under the recorded
claim posture; blocked cases are not a defect record, and no figure in this
note is a release judgment or an acceptance criterion — release thresholds,
final tolerance policy, and CI gate policy remain `TBD`, owner-gated. The
stress (12/15 `executed_and_matched` + 3 blocked) and nonlinear (5/5)
pinned figures are unchanged by the R14 waves (both suites untouched since
`60841413a`; the five committed `del1005_payload_binding_*` witnesses are
byte-identical at head).

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
case-page promotion, GUI-workflow validation evidence, and any human-gated
release-quality or professional-boundary review required at the PRD §24 R6
exit (historical v0.1 R5 token, resolved through the D-21 Annex A
crosswalk), and a reproduction bundle under the
current runnable procedure (to do, above).
Landed: the `run-benchmark` / `run-regression` payload bindings (PR #287,
`DEL-10-05`), the `export-results` binding (f82bb28e25) and the actor-neutral
clean-checkout demonstration
(`validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`,
rerun at the R14 head as
`validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/`;
both `INTERNALLY_VERIFIED`; reproduction acceptance remains an owner gate).
