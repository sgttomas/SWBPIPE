Recommend one Sections increment after the current merge: **Name, pipe-only Type, OD, wall and string Provenance in the common direct/review table**, retaining the existing operation route and explicit Queue workflow. No implementation started; wt3 remains clean at `dd8be5928c10000c4f93aaa13c572f576366ddd3`, desktop source `15186ef0973c0ba7f654743ed0f19913b0361c57`.

- **Fields and units:** ID stays read-only. Direct numeric editing requires an existing finite quantity and its actual compatible sibling unit; OD and wall may use different units. Remove Section’s legacy project-unit fallback from the new projection—missing units must remain missing. Headers should say “per-row entered unit”; cells/editors show their actual unit. Preserve canonical converted readouts beside raw review drafts. Structured provenance stays read-only; direct text requires an existing string. Numeric blank/TBD remains raw review staging subject to engine rejection, not direct quantity authoring. Compare direct numeric no-ops numerically, and text against the normalized proposal without normalizing away the canonical before-value.

- **Coupled geometry:** Every Section operation validates the resulting whole Section, including Name/Type/Provenance edits. The existing requirement is pipe geometry with `0 < wall < OD/2`, evaluated after conversion. Shared Section edits propagate to bound pipes; stale binding caches or local mill tolerance can reject the whole operation. Atomic batches simulate operations **in submitted order**, stop on failure and publish only the completed final model. They do not validate a simultaneous OD/wall replacement. For example, changing `100/40 mm` to `60/20 mm` requires wall first; OD first fails its intermediate state. This is source-derived, not a newly executed test. Keep this behavior and its diagnostics; do not add automatic reordering or a new paired-field operation. Today Grid Queue produces individual intents in row/column order, not an atomic batch.

- **Type:** Adopt a small editable combobox with completion for the sole supported token `pipe`. Preserve an unsupported current token until an explicit valid choice; opening/dismissing must not repair it implicitly. UX §3.1 explicitly requires typed completion. Existing `CompactSelect` is select-only, so using it unchanged would not satisfy that clause. Add an opt-in enum editor to the common core, leaving Node/Materials unchanged. No owner ruling or PRD amendment is needed for this implementation choice. PRD §18.2’s additional library fields remain later scope; this slice must not imply their completion.

- **Reuse and ownership:** Manager remains the sole writer of `ModelTree.tsx`. Reuse `EngineeringTable`’s persistent host for both Section modes, `modelTableAdapter`, `quantitySortProjection`, `displayQuantityService` and the existing readout path. Preserve exact per-cell eligibility during unrelated conversion refreshes, whole-sort current-basis invalidation, captured before/unit/generation, truthful Apply versus Keep outcomes, natural Keep-on-blur, ordered review selection and exact `aria-owns` ownership. Keep visible-only Queue removal and explicit all-draft Clear. Do not bind lifted drafts to a newly invented unit policy; reject obsolete active-editor callbacks without rebasing retained text.

Proposed production write set is `ModelTree.tsx`, `table/modelTableAdapter.ts`, `table/tableState.ts`, `table/EngineeringTable.tsx`, and a small colocated enum editor if separation improves clarity. A narrowly scoped style addition may be needed for its popup. Existing conversion/core/VirtualList/native/controller services should remain dependencies, not write targets. Tests belong in the corresponding table files, affected `ModelTree`/`App` tests, a new Sections browser spec, and focused existing Rust operation/batch test modules if adding the coupled-order regression there. ROOT retains CI mapping/integration ownership.

Acceptance should cover:

1. Direct success/no-op/rejection, exact before/unit ownership, mixed OD/wall units, structured provenance, missing/TBD quantities and unsupported Type.
2. Shared-pipe propagation and complete rollback on local-tolerance failure; both valid and invalid coupled batch order, with no intermediate publication.
3. Raw review staging, active-editor Queue/Clear ordering, canonical readout, current-unit Queue payloads and filter/family/page draft retention.
4. Delayed conversion with Apply→Tab→actual next character; physical sorting with opposing raw/converted order; real row movement with stable input identity, caret and one text Undo. Include virtualization, hidden/inert reveal and external focus ownership.
5. Enum typed completion, explicit pointer choice, passive dismissal, first/second Escape, same-value no-op and source/options changes.
6. Both configured browser profiles, preserving existing geometry oracles. Later native witness: invented in-memory Sections, pointer Apply/Cancel, text Undo versus model Undo, sorted review Keep/Clear, enum cancellation, coupled rejection, baseline restoration and live handback. No stored-row operations or broad menu replay.

The existing short-drawer and narrow Both/Inspector reachability gaps need not precede this bounded adapter increment if the qualified full-height Table route remains usable and those limitations stay explicit. They remain prerequisites to claiming complete B4 fit. No new owner-held conflict was found. Exact enum popup containment and native interaction remain implementation checks, not facts established by this preparation.

Consulted hashes follow. Paths are relative to `projects/chirality-piping`; `CONT` means ROOT’s continuation directory. These bind inspected portions/searches, not a claim of whole-file review.

```text
35c1943905973ad33d315df68d824da68a3a8a77f7dd2cf1d88cdcfb43b3af57  CONT/B4_4_SECTIONS_PREPARATION_BRIEF.md
301cd8d0ed6dfec1007479be379bcda61d450db8a1afdbbfb09c667683c44783  CONT/_run_records/B4_3_FAMILY_PREPARATION_RETURN.md
62f578a3c7ec2d5d729eae45c4880c2c584c18ef91b72783277f78354103c514  apps/desktop/src/features/model-tree/ModelTree.tsx
64b2d89f25f4912d63ab5a2d65d068a1c5321755f631001d57eb38969f74c48c  apps/desktop/src/features/model-tree/ModelTree.test.tsx
341184fe52c4a7880c41e0140842c4aa840a92cca7251da7a412e6368dad082e  apps/desktop/src/features/workspace/table/EngineeringTable.tsx
bac7e5c64be5a9db5c6083cda532326f97e87c95611c728a98b1e31142a64368  apps/desktop/src/features/workspace/table/tableState.ts
4361222ca36b83354ac0b019e10c006c7e29db51f3d13bee9f5009c258f2adcc  apps/desktop/src/features/workspace/table/modelTableAdapter.ts
84addabae0ea1e65df20de711694b90c99623bd1d306f32f832ca9a90b4a7eba  apps/desktop/src/features/workspace/table/quantitySortProjection.ts
a0c160c24da5b472c7e60c028308d4db1bdfc9675b0157e99a12be693f1767a2  apps/desktop/src/features/workspace/CompactSelect.tsx
fe0f9eea5a991d965dc097553e73e8c4c23c7968ed1743a8709af94f02dab20d  apps/desktop/src/features/workspace/workspaceSession.ts
66a4f562d1354ea86041858b678c7c92e477d071aedb1a5a9ad4cdc59b976ba5  apps/desktop/src/features/toolkit/BatchReviewPanel.tsx
ae7c0074c9ddd62800469eba4718972d1110c58388a4fe87ce8491ea0f5416ff  apps/desktop/src/features/operations/OperationApplyPanel.tsx
65a9557c32a628e3481242ac2cb26a51438ee91b39e492f892d15fefed172cef  apps/desktop/src/services/displayQuantityService.ts
dda696031a605a7b15bbf7940e2134b7a1023b8cf88a67dbf76d3675e47da2a8  apps/desktop/src/services/unitConversion.ts
0cf84f7b8ced2786fa03729087fd1197bf8773afb6d15a9653cb86e8a0ecc52f  apps/desktop/src/services/operationBatchService.ts
5e16c0d91fe9d489aa523b179bcdc9bf79f78faa0b23f7ef6a72dc8f88a964d4  apps/desktop/src/App.test.tsx
22062ea42fd135a9dbefa0f7e09cf3a2feccc5193ff6028818fd1c6221a3f355  core/model_operations/operation_applier/src/lib.rs
f73a763cd9d273e8f802304f1150c316ff5470739692fb41c95dc19ff2450aa6  core/model_operations/operation_applier/src/atomic_batch.rs
424cba3f1e6292d75233bd403b60c922e2e56365c7c7b15b463c6efe58d9fd72  core/model_operations/operation_applier/src/section_bindings.rs
cd6297af2ed9588f6da24c0665d32c74ee33d3e7f9115e733b47244fd75b4611  docs/PRD.md
2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682  execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/UX-SPEC/UX_SPEC_V1.md
```

`unitConversion.ts` was inspected only to distinguish the older display helper; the proposed physical-sort dependency is `displayQuantityService` through `quantitySortProjection`. No writes, tests, builds, UI, network or delegation were performed.
