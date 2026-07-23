---
doc_id: R17-DEL1005-N4-RETURN-V3
doc_kind: coordination.agent_return
status: success_ready_for_fresh_n5
created: 2026-07-23
run_id: HELP-HUMAN-PIPING-20260723-DEL1005-EXPORT-RESULTS-R17
instance_id: N4
attempt: 2
candidate: CB-2026-07-23-DEL-10-05-EXPORT-RESULTS-001-v3
---

# N4 sealed v3 terminal return

N4 completed the exact two-file v3 remediation and froze the tree.

- A present `export_results` value is deferred as raw JSON to the
  export-results handler.
- Missing required fields, malformed field types, and explicit null now return
  exit `1` with `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID` and the
  existing `REPORT-PACKAGE-WIRE-INCOMPLETE` prefix.
- Those failures expose no report package and create no output file.
- The v3 delta is confined to the runner binary source and focused Python
  contract test.
- Focused, full registered, native, static, JSON, containment, diff, and
  staging gates passed.
- Exactly one authorized v3 replacement DEC-025 sweep passed:
  `validation/evidence/sweeps/SWEEP_20260723T100430Z_1f2ecc1d0637-dirty.json`
  (`sha256:ee50a166142468752b3e84631dc3568e1116918ab79610dacee22cf40026f3d1`).
- No implementation or witness file changed after the replacement sweep.

N4 does not claim `COMMIT-SAFE`. W3 remains blocked pending fresh N5.
