---
doc_id: R17-DEL1005-OWNER-ADOPTION-V3
doc_kind: coordination.owner_authorization
status: adopted_execution_released
created: 2026-07-23
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v3
---

# Owner adoption — sealed v3 remediation

Owner authorization, relayed by HELP_HUMAN:

> I AUTHORIZE the sealed v3 remediation in
> `AMENDMENT_V3_PROPOSAL.md`, limited to
> `core/runner/headless/src/bin/openpipestress-runner.rs` and
> `tests/test_headless_runner_contract.py`: preserve a present
> `export_results` member as raw JSON until the export-results handler; map
> its DTO deserialization failures to exit 1 with
> `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`, an existing
> `REPORT-PACKAGE-WIRE-INCOMPLETE` message prefix, no report-package payload,
> and no output file; add missing-required-field and malformed-type subprocess
> coverage. No shared-wire, schema, redaction, binding, verb,
> benchmark/regression, desktop, Cargo, witness, decomposition, lifecycle,
> stage, release, issuance, or claims change is authorized. After
> affected/full/native gates pass, I AUTHORIZE exactly one v3 replacement
> DEC-025 sweep followed by fresh N5 verification. W3 remains gated on
> COMMIT-SAFE.

This adoption releases the existing N4 as sole serialized implementation owner
for the exact two-file remediation and managed run evidence. It does not
release W3, CHANGE, commit, PR, merge, lifecycle, stage, release, or issuance.
