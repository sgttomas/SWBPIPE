---
run-id: WORKING_ITEMS_RUN_2026-06-12_sweep_git_state_hardening
timestamp: 2026-06-12T08:40:00-06:00
persona: WORKING_ITEMS
tranche-id: TP-R2VERIFY-FIX-002
package-id: PKG-10
deliverable-id: DEL-10-04
run-status: SUCCESS
authority:
  - plans/VERIFICATION_2026-06-12_r2_exit_chain.md finding F-2
  - execution/_Decomposition/SOFTWARE_DECOMP.md DEC-025 (commit-bound sweep summary)
write-scope:
  - tools/release/run_evidence_sweep.py
  - tests/test_evidence_sweep.py
  - execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/MEMORY.md
  - execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_run_records/**
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-R2VERIFY-FIX-002

## Scope

Repair of verification finding F-2: `_capture()` converts any failed git
invocation into `None`, and `collect_git_state` defaulted that to an empty
dirty-path list — so a sweep summary could record
`working_tree_dirty: false` without having verified anything. Observed
context: the 2026-06-12 executing session's sweep summaries read clean during
a window in which an untracked file demonstrably existed in the worktree
(mechanism not fully determined; the failure path above is the confirmed
silent-clean route and is now closed).

## Changes

- `collect_git_state` records `status_capture_failed` explicitly;
  `working_tree_dirty` becomes `null` (never `false`) when the status capture
  fails.
- New `git_state_unverified` helper; `summary_filename` marks affected
  summaries `-gitunverified` instead of letting them look clean.
- `main --execute` exits nonzero with a named message when the summary cannot
  bind to a verified git state, because an unbound summary does not satisfy
  the DEC-025 commit-binding contract. Surface results in `overall_status`
  remain the surface truth.
- Summary `schema_version` bumped 1 → 2 for the new member.
- Tests: `test_collect_git_state_records_capture_failure_explicitly` and
  `test_summary_filename_marks_unverified_git_state` added; the
  all-surfaces-pass case now pins `schema_version == 2` and
  `status_capture_failed is False`.

## Validation

- `python3 -m pytest -q tests/test_evidence_sweep.py` — PASS, 16/16.
- Dry-run plan output unchanged.
- Five-surface DEC-025 sweep at session closeout HEAD exercises the modified
  collector live (summary committed with the closeout evidence commit).

## Boundary review

Tool-evidence hardening only. No release, CI-provider, lifecycle,
professional, certification, sealing, authentication, or code-compliance
claim; the DEC-025 ruling's gate semantics are unchanged — the tool now just
refuses to overstate them.

## Handoffs

- The historical false-clean mechanism in the 2026-06-12 session summaries
  remains not fully attributed; if it recurs with the new explicit fields, the
  recorded `status_capture_failed`/`-gitunverified` markers will localize it.
