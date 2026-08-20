# AMENDMENT 7 — desktop fixed-status hit-testing remediation

Version: 7

Authority: HELP_HUMAN standing direction to amend in-scope N1 discoveries;
mandatory clean DEC-025 evidence sweep after CHANGE node commit
`a9b1fbef90f3bb9a894054c22f6fc77572fedd0d`.

Reason: `DESKTOP_FIXED_STATUS_LAYER_INTERCEPTS_WORKSPACE_CLICKS`.

The clean commit-bound sweep
`validation/evidence/sweeps/SWEEP_20260820T214858Z_a9b1fbef90f3.json`
(SHA-256 `79301f7651bce52a492562a001d65b959c368b8d02cdf896bc3927d911c3a078`)
passed cargo, Python 902 tests, and desktop Vitest 533 tests, but the desktop
Playwright surface returned 20 pass / 2 fail. At desktop width, the fixed
workspace status/Solve proof layer intercepted clicks intended for a result row
and the Render report control. Compact variants passed. Production build was
not run after Playwright failed.

The earlier sweep
`validation/evidence/sweeps/SWEEP_20260820T214752Z_a9b1fbef90f3.json`
(SHA-256 `c66786be239b46f5d8fdc16c754c31f9eaff55de99f684a1b7d7970d4173e7df`)
is preserved as environment-attempt history only: bare Python lacked
`jsonschema`; it is not classified as a product failure.

Authorized bounded correction:

- preserve both sweep summaries as commit-bound evidence;
- diagnose and repair the fixed desktop status layer's hit-testing/layout so
  informational pills cannot occlude or intercept unrelated workspace
  controls, while status actions remain operable and accessible;
- do not weaken or bypass Playwright click checks;
- add durable regression coverage for desktop result-row and Render report
  clicks with the Solve proof/status layer present;
- rerun focused desktop interaction checks, full registered desktop tests,
  desktop build, and the affected Playwright surfaces;
- update the existing DEL run record and exact R8 manager/control evidence;
- freeze the adjacent remediation diff against node commit
  `a9b1fbef90f3bb9a894054c22f6fc77572fedd0d` and dispatch a seventh different
  fresh read-only non-delegating software-code-review over 100%.

Allowed writes expand only to the two named sweep summaries already emitted,
the existing `apps/desktop/**`, DEL-07-06 status/memory/run-record surfaces,
and this exact R8 AgentRuns subtree. No receipt, register, DAG/decomposition,
root-governance, Git commit, push, PR, lifecycle, release, or professional-
reliance write is authorized. Reviewers 1-6 and all prior evidence remain
preserved.
