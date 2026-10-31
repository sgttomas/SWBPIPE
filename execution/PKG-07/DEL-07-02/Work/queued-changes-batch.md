# Review all queued changes as one batch (G9)

Serves RT-G2 and exit criterion 7. It closes gap G9 of the reference-system run sheet
(DEL-09-01 `Work/reference-system-run-sheet.md` §6), where every queued single change was applied
on its own. DEL-07-02 owns it (Agent 0, 2026-10-31), and DEL-16-03 was consulted on the acceptance
and receipt record. Desktop paths are relative to `apps/desktop/src/`.

## Behaviour

- **Control.** When at least two of the person's own changes wait in *Review changes*
  (`features/operations/OperationApplyPanel.tsx`), **Review all N queued changes as one batch**
  moves them into batch review as one batch (`handleReviewQueuedIntentsAsBatch`,
  `features/workspace/workspaceSession.ts`). They move in the order they were queued, through the
  same `registerOperationBatch` every other batch producer uses.
- **Nothing is applied by the move.** Nothing applies until the person's **Apply entire batch**
  (`features/toolkit/BatchReviewPanel.tsx`). It validates and applies the batch on the operation
  engine's atomic batch route (`apply_operation_batch`): all changes apply, or none do.
  **Validate batch** checks the batch first without applying, and is required beforehand only for
  a batch holding an operation that needs a preview, which waits for a clean preview of that exact
  batch.
- **No adjustment.** Each change keeps the `before` it was queued with. A change whose `before` an
  earlier change in the batch makes stale is refused by name in validation, for example
  `OP-STALE-BEFORE-VALUE` for a field edit, and nothing applies. It is never rebased. The person
  can clear the batch and queue the change again on the current model.
- **Shared operation ids.** Changes queued from the same field of the same form carry the same
  operation id, and one batch cannot tell them apart. The move is refused before anything moves,
  and the message names the shared id and how many changes carry it. Every change stays queued
  singly.
- **Authorship.** Only changes with `author_type: "user"` move. A change another author queued
  stays single, and the message says how many stayed.
- **Registration failure.** If the batch cannot be registered, for example because the model
  changed meanwhile, the changes stay queued singly.

## Acceptance and receipts (DEL-16-03)

A batch applied this way is recorded like every other batch:
- one acceptance record (`acceptance_basis: user_initiated_apply_in_local_session`, not a
  professional approval);
- one entry in "Applied this session";
- one batch receipt that holds every operation;
- one undo checkpoint.

So undo reverses the whole batch, not one change of it. Saved records keep each intent, as for any
batch. No receipt or acceptance field changes.

## Evidence

`features/operations/queuedChangesBatch.test.tsx` runs against the workspace session and the wasm
engine. It covers:
- queue order;
- no change to the model before apply;
- one validated atomic apply, with one undo checkpoint and one applied entry;
- a stale dependent change refused by name, with the model unchanged;
- two queued edits sharing an operation id, refused by name before the move, with the queue
  unchanged;
- no batch from fewer than two changes;
- the panel control's count, accessible description and busy state.

## Estimate effect

The run sheet's 48 apply cycles become one cycle per review session for the single-intent forms:
material, sections, components, supports, bend realizations and per-case load/reference states.
Changes that depend on each other still need their own cycles, for example creating a material and
then adding its temperature points, because a form can only be filled once its target exists.
