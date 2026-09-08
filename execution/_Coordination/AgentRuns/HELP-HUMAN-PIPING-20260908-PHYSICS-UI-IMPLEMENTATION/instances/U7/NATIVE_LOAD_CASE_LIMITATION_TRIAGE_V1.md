# U7 Native Load-Case Limitation Triage

Status: derivative read-only triage evidence; this does not reopen R3 PASS or authorize a repair.

The first-create native workflow exposed a pre-existing controlled-state defect in `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`. The file is byte-identical at base `55df51ac3201456e0f181823e3aefefef47a73bb` and the live R3 cut, SHA-256 `391756508c612c0b030c510e3401e9e1b420fd071c4e1e9a2a49e2390233a64e`. Lines 154-156 initialize `primitiveLoadDraft.loadCaseId` only once; lines 559-568 bind the controlled case select to that value; lines 1508-1511 reject an empty or stale nonmember case and therefore disable Queue. These seams originate in commit `e67c3c8ad` (2026-06-11), before this tranche.

When the model gains its first case after the form has mounted, the browser can display the sole case option while React state remains empty or stale. Selecting the sole visible option does not produce a value change; category changes preserve the stale case. The current in-session GUI workaround is to create a temporary second synthetic case, select it, switch back to the intended case, and delete the temporary case before solve.

The owning GUI deliverable is `DEL-07-02_Model tree and property inspector`; no producer defect is implicated. The minimal future repair seam is `LoadCaseManagerPanel.tsx` plus an App-level regression covering blank project creation, first load-case creation, and primitive-load queue enablement. `LoadCaseManagerPanel.tsx` is outside U7's released nine-path fence, and DEL-07-02 writes are explicitly prohibited by the current U7 source release. No implementation, deliverable state, dependency status, finding register, lifecycle, or Git state changed through this triage.
