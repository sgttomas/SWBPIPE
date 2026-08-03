# Owner Ruling — Piping harvest slate — 2026-08-02

Status: `OWNER RULING OF RECORD`

This record preserves the owner's in-session ruling on
`CANDIDATE_HARVEST_REPORT_2026-08-02.md`. It authorizes only the Task
Management row and closeout acts stated below. It does not make either owner
intent record scope and does not authorize product work.

## Verbatim ruling

<!-- BEGIN OWNER RULING VERBATIM -->
RULING — Piping harvest slate 2026-08-02.

1. PROMOTE HC-PIP-20260802-001 as recommended: Status DEFERRED,
   cross-related to TM-PIP-001 and root TM-ROOT-102. Trigger: the
   owner-initiated Piping product-basis act. Neither intent record is
   scope.
2. PROMOTE HC-PIP-20260802-002. The root-owned repair has since merged:
   PR #482, merge commit 4a162adb1 (tools/taskmgmt/taskmgmt.py now keys
   dedup on loop identity, with a regression fixture proving same-named
   notices in different loops stay distinct). Verify independently by
   rerunning `taskmgmt scan` and confirming this loop's four
   NOTICE_2026-08-02_* files appear as distinct candidates. On
   verification, close RESOLVED_WITH_CHANGE citing the merge commit and
   fixture. Replace the planned elevation draft with an informational
   notice to Root recording the elevation, the repair evidence, and
   this closure, with reciprocal citations.
3. DECLINE promotion of HC-PIP-20260802-003 through -014 with the
   dispositions exactly as proposed; owning source surfaces unchanged.

Then deferral review, `taskmgmt archive`, closeout under my gate.
<!-- END OWNER RULING VERBATIM -->

## Bounded interpretation

- `HC-PIP-20260802-001` becomes `TM-PIP-025`, `DEFERRED`, with the exact
  product-basis trigger stated above.
- `HC-PIP-20260802-002` becomes `TM-PIP-026` and may close
  `RESOLVED_WITH_CHANGE` only if the independent scan and regression test
  pass.
- `HC-PIP-20260802-003..014` create no rows and do not change their sources.
- The Root communication is a local draft informational notice. It creates no
  foreign row and is not written to the Root coordination surface before the
  owner's closeout gate.
