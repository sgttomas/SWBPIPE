---
doc_id: R17-DEL1005-N5-RETURN
doc_kind: coordination.agent_return
status: block
created: 2026-07-23
run_id: HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17
instance_id: N5
frozen_git_basis: 1f2ecc1d06375c01a409041b8380e4d65b2a9f9a
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v2
---

# N5 terminal return — BLOCK

Branch, frozen basis, changed-path containment, shared wire extraction,
aggregate redaction behavior, package projection, cross-bindings, witnesses,
native proof, collateral, and the sole passing DEC-025 sweep independently
verified. W3 paths remain untouched.

## Blocking finding

`CliInput.export_results` deserializes directly to `ReportPackageRequest`.
Removing the required `export_results.package_id` from the accepted success
witness therefore fails in the global input parser before the export-results
handler runs.

Observed:

- exit `2`;
- `HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID`;
- no output file.

Adopted v2 requires a DTO/conversion failure to return:

- exit `1`;
- `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`;
- the underlying stable `REPORT-PACKAGE-*` code in the message;
- no package payload or output file.

## Minimum remediation

- Preserve `export_results` as an independently parsed raw JSON value until
  the export-results handler.
- Map its DTO deserialization failure to the adopted payload-invalid
  diagnostic and exit.
- Add malformed-required-field subprocess contract coverage.
- Obtain explicit post-sweep remediation and one replacement DEC-025 sweep
  authority, repeat the affected/full/native gates, and dispatch a fresh N5.

No other implementation defect was found. The unchanged desktop Rust 74/75
stale-notice residual remains outside scope and is not waived.

Verdict: `BLOCK`. W3 remains blocked.
