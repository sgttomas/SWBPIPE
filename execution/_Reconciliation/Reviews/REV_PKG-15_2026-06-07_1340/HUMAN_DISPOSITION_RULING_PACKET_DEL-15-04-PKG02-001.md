# Human Disposition Ruling Packet: DEL-15-04-PKG02-001

## Ruling Requested

| Field | Value |
|---|---|
| Finding | `DEL-15-04-PKG02-001` |
| Deliverable | `DEL-15-04` - External prover boundary metadata |
| Current severity | `BLOCKER` |
| Current `HumanDisposition` | `TBD` |
| Current `Status` | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` |
| Recommended human disposition | `ACCEPT_AS_IS` |
| Recommended status after ruling | `RESOLVED` |

Recommended ruling: accept the implemented technical remediation as sufficient
to resolve the blocker, while preserving all normal boundaries: no lifecycle
transition, no release claim, no professional approval, no certification, no
sealing, no authentication, and no code-compliance claim is made by this ruling.

## Original Blocker

The May 16 PKG-02 downstream compatibility audit found that `DEL-15-04`
external-prover metadata allowed `notes` and `tags` to carry prohibited
authority wording without diagnostics, even though both are explicit in-scope
metadata categories.

Source evidence:

- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`
  - Overall package verdict was `BLOCKER`.
  - `DEL-15-04` was the only blocker deliverable.
  - Required disposition: screen `notes` and `tags` for prohibited
    authority/lifecycle terms or constrain them to a separately validated
    non-authoritative vocabulary.
- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-04_External prover boundary metadata/Review_Findings.csv`
  - Finding row `DEL-15-04-PKG02-001` records the same blocker.

## Technical Remediation Evidence

The current implementation now includes `tags` and `notes` in the deterministic
boundary-term diagnostic loop:

- `core/handoff/external_prover/metadata.py`
  - `diagnostics_for_external_prover_metadata()` checks `"tags"` and `"notes"`
    along with the other metadata fields before returning sorted diagnostics.
  - `_boundary_term_diagnostics()` emits
    `EPM-PROHIBITED-AUTHORITY-TERM` with severity `blocking` when prohibited
    authority or lifecycle wording is detected.

Focused regression evidence:

- `tests/test_external_prover_boundary_metadata.py`
  - `test_notes_and_tags_authority_wording_are_blocking_diagnostics()` mutates
    `tags` with authority wording and `notes` with certification wording.
  - The test asserts that `EPM-PROHIBITED-AUTHORITY-TERM` is emitted.
  - The test asserts that both `tags` and `notes` are affected fields.
  - The test asserts that at least one emitted diagnostic has severity
    `blocking`.
  - The resulting metadata record still validates against
    `schemas/external_prover_metadata.schema.json`.

Direct diagnostic probe run on 2026-06-07 produced:

```text
EPM-PROHIBITED-AUTHORITY-TERM blocking notes
EPM-PROHIBITED-AUTHORITY-TERM blocking tags
```

Focused validation command run on 2026-06-07:

```text
python3 tests/test_external_prover_boundary_metadata.py
```

Result: passed.

## Review And Fan-In Evidence

- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-04_External prover boundary metadata/_run_records/TASK_RUN_2026-06-07_DEL-15-04_post-remediation-readiness.md`
  - Records `RUN_STATUS: SUCCESS`.
  - States `RF-001` is technically addressed.
  - Preserves that `DEL-15-04-PKG02-001` remains
    `HumanDisposition=TBD` with status `TECHNICALLY_ADDRESSED_PENDING_HUMAN`.
- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/_run_records/WORKING_ITEMS_RUN_2026-06-07_PKG15_POST_REMEDIATION_READINESS_FANIN.md`
  - Records all four PKG-15 post-remediation workers as `SUCCESS`.
  - Records that focused validation passed, including
    `python3 tests/test_external_prover_boundary_metadata.py`.
- `execution/_Reconciliation/Reviews/REV_PKG-15_2026-06-07_1340/RUN_SUMMARY.md`
  - Holds `DEL-15-04` only because this blocker-class finding still has
    `HumanDisposition=TBD`.
  - States that the technical content and validation evidence are sound.

## Boundary Checks

The proposed disposition does not approve or claim any external prover result.
It only accepts that the metadata boundary now blocks prohibited authority terms
in the previously missing `notes` and `tags` fields.

Preserved boundaries:

- External prover metadata remains non-authoritative.
- Attachment payloads remain references only; embedded payloads are rejected by
  existing tests.
- Software authority flags remain blocked when set to authority-granting values.
- External tool execution and commercial result ingestion remain out of scope.
- Human acceptance records, certification, sealing, authentication, code
  compliance, and professional reliance remain outside software authority.

## Recommended Ruling Text

```text
Approved disposition for DEL-15-04-PKG02-001:

Accept the technical remediation as sufficient. The finding's required
disposition has been satisfied because external-prover metadata `notes` and
`tags` are now screened for prohibited authority/lifecycle wording and emit
blocking `EPM-PROHIBITED-AUTHORITY-TERM` diagnostics. Focused regression
testing passed on 2026-06-07. Set HumanDisposition=ACCEPT_AS_IS and
Status=RESOLVED for this finding.

This ruling is a review disposition only. It does not advance lifecycle state,
approve a release, certify or seal engineering work, authenticate an external
prover result, or make any code-compliance/professional-acceptance claim.
```

## If The Human Declines

Recommended alternate disposition: `DEFER`.

Impact of declining or deferring:

- `DEL-15-04` should remain held from `CHECKING`.
- No additional content remediation is identified by the current evidence.
- The remaining action would be a human request for additional independent
  review evidence, broader authority-term test coverage, or a different
  metadata vocabulary policy.
