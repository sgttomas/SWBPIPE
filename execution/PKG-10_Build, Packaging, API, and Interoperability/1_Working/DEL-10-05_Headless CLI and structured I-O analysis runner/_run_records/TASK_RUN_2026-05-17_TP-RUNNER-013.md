---
doc_id: TASK-RUN-TP-RUNNER-013
doc_kind: deliverable.task_run
status: completed
created: 2026-05-17
task: TP-RUNNER-013 Headless Full Envelope Validation
deliverable_id: DEL-10-05
package_id: PKG-10
---

# TASK RUN - TP-RUNNER-013 Headless Full Envelope Validation

## Request

Add headless runner tests proving reference-level result-envelope pass-through
and optional full result-envelope payload validation against the accepted
DEL-08-04 shape. Keep final CLI syntax, process policy, public transport,
package scripts, CI provider, release matrix, and other public runtime surfaces
as `TBD`.

## Loaded Truth Set

- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_CONTEXT.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_STATUS.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_REFERENCES.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_DEPENDENCIES.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/Dependencies.csv`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/MEMORY.md`
- Primary artifacts: `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`
- Accepted DEL-08-04 serialized fixture from TP-RESULT-017.

## Changes

- Added `validate_result_with_optional_envelope_payload` in
  `core/runner/headless`.
- Kept existing `validate_result` reference-level behavior intact; full
  envelope payload validation is only applied when a caller supplies a payload.
- Added checks for DEL-08-04 schema wrapper identity, serialized envelope ID
  matching `result_envelope_ref`, `HUMAN_REVIEW_REQUIRED`, nonempty result
  sets, deterministic ordering evidence, and a matching result-envelope
  checksum reference.
- Added runner tests for:
  - valid TP-PHYS-015 full payload validation;
  - reference-level validation without a payload;
  - envelope-reference mismatch;
  - missing result-envelope checksum.

## Validation

- `python3 tests/test_headless_runner_contract.py` - passed.
- `cargo test --manifest-path core/runner/headless/Cargo.toml` - passed 10 tests.

## Boundaries Preserved

- No final CLI command syntax, package script, process/network/filesystem
  policy, public transport, external format, CI provider, release matrix,
  GUI/report runtime, adapter runtime, local FEA package, lifecycle state,
  dependency register, DAG/blocker file, review disposition, release record, or
  acceptance record was changed.
- The new validation is an in-memory test/support boundary only. It does not
  make professional, certification, sealing, approval, authentication, or
  code-compliance claims.

## Remaining Follow-Up

- Checksum/canonicalization vocabulary still needs a cross-deliverable ruling
  before changing schema wording across DEL-08-02, DEL-08-04, and DEL-10-05.
- Diagnostic vocabulary alignment remains a separate DEL-00-06 / DEL-08-04 /
  DEL-10-05 ruling topic.
