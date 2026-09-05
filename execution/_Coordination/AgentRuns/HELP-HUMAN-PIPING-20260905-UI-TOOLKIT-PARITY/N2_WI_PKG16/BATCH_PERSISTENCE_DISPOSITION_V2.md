# Batch provenance persistence — final owning disposition V2

Parent direction accepted; supersedes V1 queue-handling ambiguity. Read-only source disposition. Exact original members may use existing editor_intents persistence only as **stored proposed operation context**, never as durable accepted history. Full SOW-070 closure is not claimed.

## Required renderer separation

Maintain `restoredContext: EditorOperationIntent[]` separately from the live pending queue. Naming is illustrative; no durable field is added. On open, put every saved editor_intent in stored/restored context, show acceptance unknown, and leave live pending empty. Explicit requeue plus fresh validation is required before a restored record can be applied. Never silently reconstruct acceptance, grouping or pending execution from the saved array.

On successful generation/hash-guarded apply publication, append exact original members to stored context and remove those members from pending. Preserve author/source/rationale/change/before/after/flags, with operation_status still proposed. Failed/stale/validation-only responses do not append applied context. Clearing pending must not clear stored context.

For Save, take one coherent snapshot of model and combined stored-context plus pending records, preserving deterministic order and original values. Use **that same combined array** for `computeProjectEnvelopeHash({editor_intents: combined,...})` and `saveLocalProject(model, combined,...)`. Native `normalized_editor_intents` passes the array unchanged and `upsert_project` persists it in the same transaction as model. Avoid deduplication by identity that discards conflicting content.

Confirmed actual save response handling can leave live queue untouched: current App.tsx handleSaveProject unconditionally calls `setEditorIntents(saved.editor_intents ?? [])` after updating summary. That setter is a renderer convenience, not a native contract requirement. Remove/rework that queue setter for Save; retain local pending/context separation and update envelope metadata/summary from response. Do not replace pending with the returned combined array. Guard stale asynchronous Save metadata responses against later open/new/edit state so a response cannot overwrite newer queue/context or mislabel its hash as current. The native response array may be checked against the submitted snapshot without becoming the queue.

Batch grouping/application receipts remain session-only and may be exported through an explicitly supported session artifact; no persisted receipt/group/acceptance schema is introduced. Disclosure: member source/rationale is saved as review context; reopening does not restore an acceptance record. Retained proposed records do not prove current application after undo.

## Verification and limits

Test apply -> save -> reopen exact member source/author/rationale preservation, restored acceptance unknown/live queue empty, explicit requeue path, and no receipts restored. Test save response leaves current live pending/context unchanged, combined-array envelope hash matches saved payload, and pending clear does not erase retained records. Keep V1 full basis and native line references as source evidence.

This closes the concrete total-loss risk for batch member provenance only. Durable accepted/rejected decisions, timestamp/hash-bound receipts, assumptions and group reconstruction remain an explicit DEL-16-03/SOW-070 residual. Historical storage-TBD or empty status Remaining does not waive those requirements. No source writes, native schema additions or second issue plan.
