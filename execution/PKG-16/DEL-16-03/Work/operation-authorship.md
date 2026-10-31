# Operation authorship on agent and imported routes

Status: implementation design for RT-E1, 2026-10-31. Decided by Agent 0, who
placed DEL-16-03's offline intake with the RT-E1 implementation manager.

## Why authorship matters here

The operation applier refuses an agent-authored write of a value the person
enters: the bend flexibility factor k (`OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED`)
and, from RT-E1 M2-D1, every other component modifier
(`OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED`: SIFs, stiffness scaling and
stiffness, and the modifier source). The owner's ruling is that users enter
their own k and SIF. The refusal reads `author_type`, which the operation
boundary cannot verify. So the refusal holds only if every route that carries
agent or imported content stamps `agent` itself, rather than passing a claimed
author through.

## Offline proposal intake

Offline intake used to queue a pasted or opened batch exactly as written,
including the author and source it claimed. A file claiming `author_type: user`
could therefore satisfy both refusals once the person applied the batch.

Now `stampImportedBatch` (`apps/desktop/src/services/operationAuthorship.ts`)
gives every imported operation `author_type: agent` and a host source:
`source_channel: offline_intake`, `source_role: imported`, and a `source_ref`
naming the file's batch and the operation's position. Every other field the
file asserts is kept exactly; the shared engine still checks it. The author and
source the file claimed are kept beside the batch as display-only import
context, never in an intent. Batch review shows an imported batch as
"Imported; attribution unverified" and lists the claimed attribution under it.
An imported batch can therefore write no k, SIF or other component modifier;
the person enters those in the component inspector.

## Requeue from a reopened project

Saved records keep their intent but no batch context, so a requeued record's
author is read from the record itself (`requeueRetainedRecord`):

- A record that is agent-authored, or whose source names the embedded agent or
  an offline import, is re-stamped `agent`.
- A `user` record that writes a value only the person enters is refused,
  because its author cannot be verified after reopening. Records saved before
  this change may be imports that claimed `user`. The person enters the value
  again in the inspector.
- Any other `user` record is requeued unchanged. Its author changes no refusal.
  Most desktop editors emit `user` with no source, so refusing every such
  record would refuse nearly all requeues without protecting anything.
- A record naming no recognizable author is refused.

## Trust boundary

The bundled webview is trusted: any of its code can assert `author_type: user`
on an operation it sends to the applier. A source claim from the webview would
prove no more, so the applier does not add a source-role check. This is the
same boundary as the embedded agent's live session. A library or hanger record
the person explicitly applies is the person's own act and counts as
user-entered; its library provenance keeps the value's source traceable, and
the embedded agent cannot route a library application (DEL-16-06).

## Evidence

- `core/model_operations/operation_applier/tests/user_entered_modifiers.rs`:
  the applier refuses agent-authored SIF, modifier-source and stiffness-scaling
  edits, a component created with modifiers, and a library record copying a
  SIF; k keeps its own code; the refusal reads only the author claim.
- `apps/desktop/src/services/operationAuthorship.test.ts`: route recognition,
  the requeue rules, and a file claiming `user` whose stamped k and SIF edits
  are refused by the desktop batch service on the wasm engine.
- `apps/desktop/src/features/offline-proposal-intake/workflows.test.tsx` and
  `features/toolkit/BatchReviewPanel.test.tsx`: the stamped batch, the
  display-only import context and its review display.
