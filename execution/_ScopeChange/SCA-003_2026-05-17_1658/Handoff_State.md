---
amendment_id: SCA-003
doc_kind: scope_change.handoff_state
package_role: snapshot / handoff artifact
created: 2026-05-17
status: executed
closure_verdict: OPEN_PENDING_DERIVATIVE_CLOSURE
---

# SCA-003 Handoff State

## Accepted Snapshot

|Field|Value|
|---|---|
|Snapshot|`execution/_ScopeChange/SCA-003_2026-05-17_1658/`|
|Decomposition Revision|`0.6`|
|Docs Authority|`docs/_ScopeChange/SCA-003_2026-05-17_1658/Authority.md`|
|Latest Pointers|Updated for execution/docs SCA state and decomposition latest state.|

## Authoritative Truth Changed

|Surface|State|
|---|---|
|`execution/_Decomposition/SOFTWARE_DECOMP.md`|CURRENT_REVISION_0.6|
|`docs/_Registers/ScopeLedger.csv`|CURRENT_FOR_SCA_003|
|`docs/_Registers/Deliverables.csv`|CURRENT_FOR_SCA_003|
|`docs/_Registers/ContextBudgetQA.csv`|CURRENT_FOR_SCA_003|
|Affected `_CONTEXT.md` files|UPDATED_WITH_SCA_003_DISPATCH_CONSTRAINTS|

## Derivative Package State

|Package / Area|Owner|Status|Next Required Action|
|---|---|---|---|
|Persistence schema|TASK / domain-schema|STALE_REBUILD_REQUIRED|Update physical-container and external artifact reference schema.|
|Persistence service and tests|TASK / persistence|STALE_REBUILD_REQUIRED|Implement SQLite profile and round-trip/migration/sidecar tests.|
|Local-first storage policy|TASK / security-privacy|STALE_REBUILD_REQUIRED|Replace TBD wording with SCA-003 local-only profile when sealed.|
|Architecture persistence contract|TASK / architecture/docs|STALE_REBUILD_REQUIRED|Update physical-container language when sealed.|
|Desktop packaging|TASK / interop-build|STALE_REBUILD_REQUIRED|Verify SQLite/FTS5 bundled/offline posture.|
|GUI save/open UX|TASK / gui-workflow|STALE_REBUILD_REQUIRED|Add local create/open/save flow through services.|
|States/runs/handoff/operation audit|TASK / PKG-14/15/16|STALE_REBUILD_REQUIRED|Persist metadata through storage services without DB-as-contract.|

## Closure Verdict

`OPEN_PENDING_DERIVATIVE_CLOSURE`

SCA-003 is closed for controlled decomposition amendment purposes, but downstream derivative packages remain stale until their owning workflows update schemas, docs, code, tests, and UX surfaces.

## Remaining Blockers

- Migration framework/tooling and DB migration details remain TBD.
- Portable export/copy behavior for large external files remains TBD.
- Optional NumPy retrieval-cache promotion requires later evidence and approval.
- No runtime storage implementation exists yet.

## Next Owning Workflow

ORCHESTRATOR should plan bounded downstream TASK work from this accepted SCA-003 snapshot.
