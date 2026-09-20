Inventory complete at `127677e1fff8b6c68044dfe0ed8475e3d1dd63d4`. **TASK Type2, gpt-6-astra / low**. No edits, tests, builds, UI, model calls, mutations or delegation.

Current live discussion contract is **revision 2**, SHA-256 `02c1e7982e919d2e6757c9ad24c7b8fec8c58bb88d64b8593f68f114a9bcefc2`. This differs from preserved revision 1’s `32277c…`; revision 2 incorporates CLI-first and the publication/idempotency conditions. Its inspected product basis remains `226b1db…`; this inventory uses the requested newer committed source. Brief hash matches `7dba40bc…`.

**Recommended bounded activation:** a session-lifetime JSON CLI/private socket bridge exposing inspect, preview, submit and status; one agent-authored node-coordinate proposal; existing human Apply and Undo. No direct external Apply, second model store, durable restart recovery, MCP or embedded Runtime.

### Controller and acknowledgement seam

The authoritative state lives in `src/features/workspace/workspaceSession.ts` and its state hooks:

- `currentModel.current`, `projectSessionGenerationRef`, `uiModelRevisionRef`, `modelRevision`, `modelPublicationGenerationRef`.
- `modelHashOwner` at lines 241–243 binds a computed hash to project generation/UI revision.
- `requestEpochRef` invalidates pending preparation; `operationRequest` supplies synchronous operation admission/response invalidation.
- `projectOperationOwner.current` is the synchronous project-operation ownership gate. Do not rely solely on rendered `projectBusy`.
- Queues, batch outcomes/receipts, applied operations and Undo/Redo remain React state in `operationsSessionState.ts`.

Current boundaries are insufficient externally:

- `handleQueueOperationBatch`, lines 971–992, captures basis **at invocation**, silently returns on invalidation, schedules queue state and returns no ticket.
- `handleRunOperationBatch`, 994–1065, guards engine responses and schedules history/model/result changes, but returns no publication acknowledgement.
- `commitModel`, 545–589, sets `modelAssignment.status="committed"` before React publication. That name is **not** sufficient external completion evidence.
- The model layout effect, 523–543, advances internal `modelRevision` and starts asynchronous canonical hashing. A response must not pair an old hash with the new model.

**Smallest workable change:** retain existing state/store ownership; add a controller-owned transaction registry and a committed-render publication effect.

1. Admit inspect/preview/submit against an explicit basis containing app instance, workspace identity, project identity/generation, UI revision and canonical hash. Capture typed selected target; later selection changes do not retarget it.
2. Return discriminated outcomes: `busy`, `stale`, `withdrawn`, `unavailable`, validation rejection, or acknowledged success. Use the same synchronous owner refs as UI operations.
3. For queue publication, store the pending transaction/key and schedule the existing queue setter. A `useLayoutEffect` observing committed queue state acknowledges the ticket only when the exact entry/payload/basis is present.
4. For Apply, retain existing validation/application/history/invalidation code; tag the pending publication with its transaction ID. A committed-render effect verifies the expected model identity/revision, matching batch receipt, Undo entry, cleared Redo, removed queue entry and invalidated result standing. Resolve committed only after those agree and the corresponding canonical hash is available.
5. Declare the effect after the existing model-invalidation effect and guard its asynchronous hash completion against generation/revision changes. Do not acknowledge an interrupted render.
6. Reset/unmount must invalidate admission and pending callbacks synchronously, settle pending requests explicitly, unregister transport readiness and retain already acknowledged outcomes for the declared session window.

This adds acknowledgement of the existing transaction rather than creating a parallel model or validator.

### Exact ownership proposal

Paths below are relative to `projects/chirality-piping/apps/desktop/`.

**Single shell/controller writer:**

- `src/features/workspace/workspaceSession.ts`
- `src/features/workspace/operationsSessionState.ts`
- New `src/features/workspace/liveControlController.ts`
- New `src/features/workspace/liveControlTypes.ts`
- New `src/services/liveControlBridge.ts`
- Focused `src/features/workspace/liveControlController.test.ts` and session integration tests
- `src/features/toolkit/BatchReviewPanel.tsx` only if ticket/actor presentation requires it

Keep `modelSessionState.ts`, `projectSessionState.ts`, `operationBatchService.ts`, domain `types.ts` and `App.tsx` unchanged unless implementation demonstrates a necessary field/export. The hook can own bridge registration without adding an App writer.

**Disjoint native/CLI worker:**

- New `src-tauri/src/live_control.rs`
- `src-tauri/src/lib.rs`: module/state/startup registration and reply/ready commands only
- New `src-tauri/src/bin/swbpipe-control.rs`
- `src-tauri/Cargo.toml` and lockfile only if necessary
- Native transport/CLI tests beside that module

Use a per-app Unix-domain socket in an owner-only directory, explicit app descriptor/capability and bounded JSON framing. Existing `std::os::unix::net`, serde/serde_json and getrandom dependencies appear sufficient for a small initial implementation; dependency changes are not established as necessary.

Native receives authorized requests, dispatches to the registered frontend controller, and correlates replies from that exact webview/registration. Tauri commands currently go frontend→backend; they do not already expose live workspace control. No suitable external live transport was found in the inspected desktop source. Existing batch Rust commands are stateless computations over supplied models.

**B4 overlap:** B4 already owns App/workspaceSession/styles and the table operation route. Do not assign concurrent writers to `workspaceSession.ts`. Finish/freeze B4’s shared-controller work, obtain a hash-bound handback, then integrate live control with that same shell writer—or explicitly transfer ownership. Native work can proceed from a frozen wire contract in disjoint paths. It must not duplicate the controller implementation.

### First supported mutation

Use existing `EditorOperationIntent`:

```text
operation_kind: modify
operation_status: proposed
target: { object_type: Node, ref: copied node ID }
change.change_kind: set_field
change.field_path: position.x
change.dimension: length
change.unit: model.project.units.length
```

The existing reusable builder is `makeRichIntent` in `src/features/rich-authoring/formSupport.tsx:100`. `workspaceSession.shell.test.tsx:179–191` constructs exactly this coordinate change, then supplies length unit/dimension.

Caution: the builder defaults to `author_type:"user"`, GUI provenance, deterministic IDs and user-authored rationale. The controller must replace these with trusted agent attribution and invocation-scoped operation/change IDs. It must not blindly forward those defaults. `ModelTree.tsx:1478`’s private `buildGridOperationIntent` uses the same typed `set_field` route; do not export/refactor it concurrently with B4 merely for this proof.

Wrap one intent in existing `OperationBatch`. Native and browser share semantics:

- Native: `operationBatchService.ts` invokes `validate_model_operation_batch` / `apply_model_operation_batch`.
- `src-tauri/src/lib.rs:1957–1964` calls `open_pipe_stress_operation_applier::{validate_operation_batch,apply_operation_batch}`.
- Browser: the service calls corresponding WASM batch exports.

Both routes return computed outcomes; neither alone publishes React model/history. Verify normalized diff/hash/application outcomes across routes, allowing their truthful route metadata to differ.

### Recovery, cancellation and reset rules

- **Idempotency first:** after authenticating explicit app/workspace generation, resolve existing key plus canonical payload association **before** checking whether the original model basis is now stale. Queue→lost reply→human Apply→same-key retry must return the original ticket/committed outcome.
- Reserve an in-flight key synchronously so two simultaneous submissions cannot create two queue entries. Different contents under the same key are a conflict.
- Admission must compare generation/revision as well as hash: Undo restoring equal bytes does not revive an old preview.
- Cancel accepted **before queue publication** leaves no ticket. It must remove/invalidate any scheduled entry before the publication effect can acknowledge it.
- After publication, RPC cancellation/disconnect only abandons waiting. It cannot clear the queue or reverse Apply. Existing UI Clear handles withdrawal.
- `handleClearReviewQueue:954–968` currently erases queue/outcomes and invalidates requests; add terminal ticket transitions before erasure.
- Project replacement/open/new and `adoptNormalizedPersistenceModel:1437–1468` clear receipts/history. Preserve registry terminal outcomes independently of those UI arrays for the promised same-process recovery window.
- App instance identity must change on restart; workspace identity must distinguish remounts and project generations. Existing numeric project/UI counters alone are insufficient globally.
- **Recommended explicit persistence boundary:** same running app/controller session reconnect recovery only. After restart, old app-instance references return expired/outcome-unknown and require reconciliation, never automatic replay. A stronger restart guarantee requires a durable receipt carrier and is a separate scope decision.
- Incoming author metadata is untrusted. Controller assigns `agent`; record actual bridge request identity without fabricated Codex thread/turn IDs. First Apply remains an actual human act; an agent-generated click cannot serve as that evidence.

### Verification scope

Focused tests should prove:

- coherent inspection/hash ownership; busy admission; original basis and frozen target;
- preview changes neither model nor history;
- no queue/commit success before committed-render acknowledgement;
- duplicate in-flight/reconnect recovery, conflicting key, and lost reply followed by human Apply and retry;
- cancellation on both sides of queue publication;
- Clear, project switch, same-ID replacement, Undo equal-hash, unmount and late engine/hash replies;
- partial/mismatched engine receipt never publishes success;
- socket authorization, explicit identity, framing limits, readiness loss, correlated replies and restart expiry.

Then one actual native journey: CLI inspect selected invented node → preview X change → submit → visible batch review → **human Apply** → CLI status receipt and visible coordinate/history/result invalidation → human Undo → stale-preview rejection. Include the lost-reply retry branch. This establishes native CLI→live-controller behavior; a Rust-only engine test does not.

Unresolved activation choices are limited to approving the concrete file ownership/handback, same-session-only recovery guarantee, and the actual human/native witness arrangement. Durable receipts, unattended Apply, MCP and Runtime qualification need not enter this first tranche.

### Consulted hashes

Committed source prefix: `projects/chirality-piping/apps/desktop/`.

```text
68feb8dc93c7d3c836fe45ab7e75611a7ebc61480b73770f2d918c54bbd67e3a src/features/workspace/workspaceSession.ts
8a27435e58fb5a5c24e3c78a2ec846fa000745a70ae36babdab9082c24e4a292 src/features/workspace/operationsSessionState.ts
3c9cba4677a2d9f0a2c8845780e3cc7afbe9965fa5ed00f419951f01c4785dff src/features/workspace/projectSessionState.ts
b689520597c9064a22881e8e6f4af52c6c999cb88908313b7183956ab2e40e76 src/features/workspace/modelSessionState.ts
66a4f562d1354ea86041858b678c7c92e477d071aedb1a5a9ad4cdc59b976ba5 src/features/toolkit/BatchReviewPanel.tsx
0cf84f7b8ced2786fa03729087fd1197bf8773afb6d15a9653cb86e8a0ecc52f src/services/operationBatchService.ts
a2ccaf863646c48c3f1c3303ad31ae87132311c28f72b343824f47ac58a83314 src/types.ts
6ea1c8f46a32edc2c5d69cb6f9a94ea64d53e18e3ede324ef5d4e8e9fd56c880 src-tauri/src/lib.rs
390a7a611509fce1c723d1fa198654df14f0a7651d5b18c53d0d7b04dd1ae246 src-tauri/Cargo.toml
0b2d6f9ffd3ae729b9106cb2188c0698c4796df133b3615d6d5e105d1dff0846 src/features/model-tree/ModelTree.tsx
4b546c8e5dd6bd056a13f1ac3afb4c271851dac1e19e1b9d0dc3cd1e439e40dd src/features/workspace/workspaceSession.shell.test.tsx
e7b07287b09af29a4214e1ca27d1e40e57ef8ce258522266bdbd168cc87bd973 src/features/rich-authoring/formSupport.tsx
```

No implementation activation or behavior qualification is implied.
