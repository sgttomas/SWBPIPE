# NOTICE — 2026-08-03 — Task Management launcher remediation (root tranche)

Coordination, not authority; no project-local act is created or required.

Root tranche ROOT-TM-LAUNCHER-REMEDIATION-20260803 amended this loop's
`init/taskmgmt-init-prompt.md` (and the shared helper) ahead of the next
Task Management generation:

- Step 4 now names the explicit per-loop
  `python3 tools/taskmgmt/taskmgmt.py archive --register <this loop's register>`
  command; the helper's `archive` verb now requires `--register` and no
  longer defaults to the root register, so a child session can never
  archive a foreign register by omission.
- Step 5 and the write-scope sentence now name the mandatory closeout
  receipt appended to this loop's receipts surface per
  `agents/AGENT_TASK_MANAGEMENT.md` §Closeout (rule of 2026-08-02,
  commit 02117f6c4; the launchers predated it by hours).
- Harvest adds a manual sweep of marker classes the deterministic helper
  does not implement (e.g. `TM-CANDIDATE:` lines); steer examples now
  include `triage of the open rows`.

Follow-on for this loop: none required now. The next TASK_MANAGEMENT
session for this register simply follows the amended launcher. The loop
adopts, amends, or declines under its own instruments (agent-index
change-notice rule, AGENTS.md).
