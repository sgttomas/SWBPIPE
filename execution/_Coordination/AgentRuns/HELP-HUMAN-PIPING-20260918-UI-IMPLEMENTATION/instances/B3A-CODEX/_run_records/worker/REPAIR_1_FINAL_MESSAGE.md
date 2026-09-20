P2 repaired and frozen. Landed same-session writes now update the marker despite later failed/missing Open requests. Verified response order prevents delayed hashing from overwriting a newer baseline; existing UI adoption and integrity guards remain.

Checks: **33 project-handler tests passed**, **7 existing persistence tests passed**, TypeScript and diff checks passed. Fail-before evidence retained.

Only `workspaceSession.ts` and `App.projectHandlers.test.tsx` changed. Evidence: `worker/repair-1-*`; source-manifest SHA-256 `7518e507ff1a1244c43cfb458be30eb1286e64877c14042b34e413767cdcf9ec`.

Ordering uses observed service fulfillment; no backend commit sequence exists. Native/browser reassessment remains manager-owned. No resources retained or Git actions performed.
