---
doc_id: R17-DEL1005-N4-REMEDIATION-V3-EVIDENCE
doc_kind: coordination.implementation_evidence
status: pre_sweep_pass
created: 2026-07-23
attempt: 2
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v3
---

# N4 sealed v3 remediation evidence

This is run-local implementation evidence. It is not independent verification,
acceptance, release, issuance, or a `COMMIT-SAFE` decision.

## Containment and behavior

The v3 product/test delta is confined to:

1. `core/runner/headless/src/bin/openpipestress-runner.rs`
2. `tests/test_headless_runner_contract.py`

No shared wire, report-package, schema, redaction, binding, verb,
benchmark/regression, desktop, Cargo/lock, witness, decomposition, lifecycle,
stage, release, issuance, claims, or W3 closeout surface was modified by this
attempt.

The outer CLI input now preserves a present `export_results` member as raw
JSON, including an explicit JSON `null`, until the export-results handler. The
handler deserializes that raw value to the existing shared
`ReportPackageRequest`. A DTO deserialization failure returns exit 1 with
`HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`; its message begins
`REPORT-PACKAGE-WIRE-INCOMPLETE: ` and appends the concrete serde detail. It
exposes no report-package payload and creates no `--output` file. An absent
member retains `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING`.

## Final affected gates

All commands ran after the final v3 implementation edit:

| Command | Result |
|---|---|
| `cargo fmt --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml -- --check` | PASS, exit 0 |
| `cargo test --manifest-path projects/chirality-piping/core/reporting/report_package/Cargo.toml` | PASS, 16 tests |
| `cargo test --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml` | PASS, 46 tests |
| `python3 -m pytest -q projects/chirality-piping/tests/test_headless_runner_contract.py` | PASS, 17 tests |

The two added subprocess cases cover a missing required `package_id` and a
malformed string in place of the `source_basis_refs` sequence.

## Native runner proof

Direct execution of the final debug runner produced:

| Case | Exit | Diagnostic/message | Package exposed | Output file |
|---|---:|---|---|---|
| absent member | 1 | `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING` | no | none |
| missing required `package_id` | 1 | `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`; `REPORT-PACKAGE-WIRE-INCOMPLETE: missing field package_id` | no | none |
| malformed `source_basis_refs` type | 1 | `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`; concrete serde `invalid type: string` detail | no | none |
| explicit JSON `null` | 1 | `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`; concrete serde null/struct detail | no | none |
| valid invented success | 0 | none | yes | caller-named structured JSON only |

The valid success package remains byte-identical: 29,126 bytes, six members,
and SHA-256
`8abef0f82ce8be9a1ce5b0c427d66951fdf81f984b0854e524cbe6209bc062ab`.
The computed hash equals the reported hash and the output JSON equals stdout.

## Registered full union

The final v3 registered invocation is normalized at
`instances/N4/evidence/registered_pre_sweep_v3.json`.

- desktop Vitest: 29 files, 516 tests passed;
- desktop production build: PASS;
- piping pytest: 548 passed;
- practitioner-harness pytest: 311 passed;
- repository self-check: PASS under its registered deterministic contract.

Overall registered status: PASS, exit 0.

## Static gates

| Gate | Result |
|---|---|
| claims-language validator | PASS; 268 files |
| path-anchor validator | PASS; 984 files; zero findings |
| piping receipt validator | PASS; frozen through Receipt-44 |
| parse all R17 JSON/JSONL | PASS; 6 files |
| exact v3 write containment | PASS |
| `git diff --check` | PASS |
| `git diff --cached --check` | PASS |
| staged paths | none |

## Replacement-sweep posture

There are 305 pre-existing sweep artifacts, including the superseded v2
attempt. No v3 replacement sweep has been invoked at this evidence freeze.
Every v3 pre-sweep gate is green, so the next permitted action is exactly one
registered replacement DEC-025 sweep. The implementation tree freezes at that
invocation.
