# Condition-set authoring in the tree, inspector and tables (S3b)

Model document 0.9.0, slice S3b (DEL-02-01 Work `model-document-0.9.0.md` §1.2, §4.1,
§4.2, §11, §12), for DEL-07-02:18 and the condition-set parts of DEL-07-02:13 and :14.
It serves RT-G5, RT-G2 and RT-G3. The load manager's part is in DEL-07-03 Work
`condition-use-authoring.md`. This note describes the implementation; it records no
backcheck or acceptance.

## 1. One route

Every surface builds an operation of S3a's wire (§11) and adds no route of its own. The
forms check the operation validate-only with the operation engine and the current model
hash, then queue it for Review/Apply (`features/condition-sets/engineCheckedQueue.tsx`,
as the load-state editor does). The bulk change and the run sheet queue batches through
their existing batch routes. The person applies through Review/Apply or batch review, and
the session's checkpoint undoes. Nothing writes the model or the store directly.

| Surface | Operations | Producer |
|---|---|---|
| Model tree, Condition sets section | `create_condition_set`, `update_condition_set`, `delete_condition_set` (object type `ConditionSet`, field path `condition_sets`, the whole record as `after`) | `ConditionSetsSection.tsx` |
| Pipe inspector, Condition set field | `assign_condition_set` (Element, `condition_set_ref`), clear with `after` `not_present` | `PipeConditionSetField.tsx` |
| Engineering table, pipes, Condition set column | the same, from the grid's queued cells (`buildGridOperationIntent`) | `ModelTree.tsx` `conditionSetGridColumn`, `modelTableAdapter.ts` |
| Bulk change, Assign condition set | one `assign_condition_set` per affected pipe, one batch | `bulk-change/bulkChange.ts` |
| Run sheet, Condition set column | one `assign_condition_set` per run row with a present effective value | `runSheet/runSheetBatch.ts` (Work `run-sheet-entry.md` §1.2, §2, §3.3) |

The operation builders are in `conditionSetIntents.ts`. A delete's diff row names
`condition_sets[<index>]`. Its inverse is a create at that path with the row's `before`,
and a test applies it.

## 2. The set form

- **Members.** Id (on create), label, provenance (required), installation temperature,
  the conditions table (key, label, temperature, pressure, contents density), design and
  peak values, added weight (magnitude with its `force_per_length` unit and a
  `global_x|y|z` direction, both entered) and exposure (wind, snow and ice, wave).
- **Units.** Each quantity is a value with an explicit unit. Where the catalog is loaded
  (`unitCatalogService`), its accepted units of the dimension on main (`temperature`,
  `pressure`, `density`, `force_per_length`) are offered as suggestions. The engine checks
  the unit and dimension either way, and names a mismatch,
  `OPERATING_CONDITION_QUANTITY_INVALID`. Text becomes a number through `parseQuantities`.
- **Nothing is entered for the person.** Empty cells read "not stated", and nothing is
  prefilled or suggested. Only entered members are written. A half-entered quantity, or an
  added weight with only one of its two members, is refused in the form by name, and
  nothing is queued.
- **Exposure marks.** Each mark has three states: not stated (absent), marked (`true`) and
  not marked (`false`), so a stored `false` is shown and kept, never dropped.
- **Members the solve does not read.** Design, peak, snow and ice, wave and each
  condition's contents density carry "(not used by the solve in 0.9.0)".
- **Pressure.** On a pressure-free document the Pressure column states that no solve of
  the document can use it, naming `LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT`.
  It stays editable, because the validator admits a condition's pressure in a set; a test
  stores one.
- **Retained members.** Members the form does not edit (tags, notes and any other) are
  listed as "Retained as authored" and written back unchanged.

## 3. Refusals and findings in place

The engine's outcome is listed under the control that queued it, each finding with its
code, message and references:

- **Refused.** Blocking findings, and nothing is queued. These include
  `OP-CONDITION-SET-IN-USE` with every pipe that carries the set (the section also names
  those pipes before the attempt), the `OP-DATA-*` codes (`OP-DATA-ID-COLLISION`,
  `OP-DATA-UNCHANGED`, …) and the document validator's codes.
- **Staged.** Warnings: the solve refuses the case until it is resolved.
- **Carried.** Info: a refusal the document already carried before the edit, outside the
  edit's scope; the solve still refuses it.

Review/Apply then shows the applied outcome's findings, as it does for every operation.

## 4. Mounting

The Condition sets section is mounted by `ModelTree.tsx` in its tree-records area, above
the Lines section, whenever the tree has the operation route. It reads pending intents
through an optional `queuedIntents` prop, which App does not yet pass. Without them, a
second pending change to the same record is still refused at apply as stale (`before`
mismatch), but not before it is queued. App's one-line hunk is listed in §6. The
inspector field sits after the section assignment. The table column follows Material in
the pipes grid. The inspector's All properties list shows a pipe's Condition set, or "not
stated".

## 5. Evidence

`apps/desktop/src/features/condition-sets/`:

- `conditionSets.engine.test.tsx`:
  - the end-to-end path: form create, inspector assignment and case use, each applied
    by the wasm engine;
  - every operation with its inverse, restoring the canonical document;
  - the in-use refusal naming P1 and P3;
  - the ID-collision, unchanged and quantity refusals;
  - the form's own refusals;
  - carried info and staged warnings;
  - the not-used labels, "not stated" and a stored pressure;
  - retained members;
  - the exact-route refusal;
  - the engine's ΔT records.
- `conditionSets.session.test.tsx`:
  - create, bulk assignment and use through the session, each undone through its
    checkpoint to the prior model hash;
  - a saved and reopened model, edited again.
- `conditionSetSurfaces.engine.test.tsx`:
  - the table column's assign and clear, applied;
  - the tree section and the inspector field mounted;
  - the bulk-change panel.
- `../model-tree/runSheet/conditionSetColumn.test.ts`:
  - inheritance, `-`, the F rule and the unknown-id refusal;
  - emission order;
  - the bend rule;
  - the projection in later bends' `before`, with the engine applying the chain.
- `services/operationContractCorpus.test.ts`: S3a's corpus in the browser lanes. One
  case is recorded as a reported divergence (§6).

## 6. Open

- **App hunk (not written; held for PR #59).** Pass the pending intents to the tree, so
  the section can refuse a second pending change before queueing:
  `<ModelTree … queuedIntents={editorIntents} …>` in `App.tsx`.
- **Reported to the applier's owner, not repaired here.** `OP-DATA-UNCHANGED` compares
  serde values, which distinguish `20` from `20.0`. A record whose `after` text is
  float-form, applied to a model that reached the engine through JS (integer-form),
  reads as changed and is applied as a no-op. Corpus case `s3a_14` shows it in the
  browser lanes. The desktop's own `after` text is integer-form, so its unchanged edits
  are refused.
- **Not built.** Support `direction`, contents and hydrotest, and EX resolution stay
  refused and offer no input.
- **Verification still open.** Native and timed walkthroughs.
