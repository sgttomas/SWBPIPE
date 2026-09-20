**PASS — no actionable finding in the three-path CI repair.**

Reviewed frozen `3861b614362d8da925e148d7506ecc9902c7f808` against `002dff0f244976f98b36517d920b3761f6f88704`.

- Agent reason placement now clears the page Close button. The transparent gap bridge preserves hoverability.
- Escape dismissal runs after child handlers, respects their consumed events, and prevents the window-level shell action. The event-specific WeakSet allows simultaneous visible reasons to dismiss together.
- Hidden reasons leave Escape alone; focus/pointer reentry restores visibility. All listeners are removed on unmount.
- Described-by text remains mounted, and dismissal does not move focus.
- Existing assertions remain unchanged. New regressions use ordinary pointer/keyboard actions; no forced click, skip, timeout, or weakened oracle was added. The component-consumed-event test deliberately installs a one-shot `preventDefault` listener.

The sealed brief and implementer return hashes match. All **3 source hashes and 89 evidence entries** match manifest `3489c677ef4a483003cfed55ab4371f2b3d5de9b9bdebb3e1571b54b3a8a847a`.

Retained pinned-Chromium evidence shows:

- Original Close interception reproduced in **2/2 cases**.
- Geometry-only repair: **6/6 passed**.
- Pre-repair Escape regressions: **4/4 failed**.
- Final focused checks: **10/10 passed**; final description/new-regression checks: **6/6 passed**.
- Close’s unchanged center changes from tooltip ownership to actual button ownership in both profiles.

`git diff --check` passes. Tracked files match the frozen revision; untracked material is the repair evidence directory. I performed no tests, UI, writes, Git mutations, or delegation.

This delta is suitable for code fan-in. Manager’s broader affected source/dist checks, rebuilt native witness, and ROOT’s final sweep/CI/merge gates remain separate and pending. No broader accessibility, usability, or native acceptance is claimed.

Independent TASK, parent ROOT; existing Astra/xhigh attribution and telemetry limits apply. Standard F-PIP-2 / DEC-081 claim fence applies.
