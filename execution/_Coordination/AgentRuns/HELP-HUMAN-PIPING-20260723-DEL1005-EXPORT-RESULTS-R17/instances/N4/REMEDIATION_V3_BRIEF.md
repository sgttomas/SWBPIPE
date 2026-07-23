---
doc_id: R17-DEL1005-N4-REMEDIATION-V3-BRIEF
doc_kind: coordination.agent_brief
status: dispatched
created: 2026-07-23
run_id: HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17
instance_id: N4
attempt: 2
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v3
---

# N4 sealed v3 remediation brief

N4 remains the sole serialized implementation owner. N4 does not delegate.

## Exact writable product/test paths

1. `core/runner/headless/src/bin/openpipestress-runner.rs`
2. `tests/test_headless_runner_contract.py`

N4 may also write managed evidence below this R17 run root and exactly one new
replacement `validation/evidence/sweeps/SWEEP_*.json` after every pre-sweep
gate passes.

## Required behavior

- Preserve a present `export_results` member as raw JSON until the
  export-results handler.
- A missing member remains exit `1` with
  `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING`.
- Deserialize a present member to the existing shared
  `ReportPackageRequest` inside the handler.
- A DTO deserialization failure returns exit `1` with
  `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`.
- Its message begins with existing stable
  `REPORT-PACKAGE-WIRE-INCOMPLETE` and includes the concrete serde detail.
- The failure exposes no report-package payload and writes no output file.
- Add subprocess coverage for a missing required field and a malformed field
  type.

## Frozen exclusions

Do not modify shared wire/report-package code, schemas, redaction, cross-binding
semantics, runner verbs or other verb paths, benchmark/regression, desktop,
Cargo/lock, witnesses, deliverable closeout, decomposition, dependency,
lifecycle, stage, release, issuance, or claims.

## Gates and terminal sweep

After the final remediation edit:

1. run Rust formatting, report-package tests, runner tests, and focused Python
   contract tests;
2. prove missing-field and malformed-type behavior, success/failure
   regression, no payload, and no output file;
3. run the complete registered pre-sweep union and applicable native runner
   proof;
4. run claims/path/receipt/JSON/containment/diff checks;
5. only if all pass, invoke exactly one v3 replacement DEC-025 sweep;
6. make no product/test/schema/Cargo/witness edit after the sweep;
7. return terminal evidence without claiming `COMMIT-SAFE`.

Any excluded-path need or failed gate returns `BLOCK`. W3 remains held for a
fresh N5.
