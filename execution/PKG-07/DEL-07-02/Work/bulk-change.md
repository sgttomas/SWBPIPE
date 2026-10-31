# Bulk change and replace over a selection (B2)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Adopted
under the owner's 2026-10-27 delegation (survey item B2, FR-GUI-004,
FR-MOD-004). A bulk change is a way to write many existing operations at once.
It adds no operation, no model-document member and no view state. It builds one
ordinary operation batch (DEL-16-06) and hands it to the existing review queue,
so validation, the per-step diff preview, the single all-or-nothing Apply and
Undo are the queue's own. Nothing is inferred: every value written is one the
user chose or entered, or the entity's own current value.

## Changes offered

Each change maps to an operation the applier already resolves, with the same
wire fields its existing editor writes. The canonical `before` is the one that
operation checks for staleness.

| Change | Entities it writes | Operation (`change_kind`, `target.object_type`, `field_path`) | `before` | `after` | Existing producer of the same operation |
|---|---|---|---|---|---|
| Assign a shared section | pipes | `assign_section`, Element, `section_ref` | canonical JSON of the whole pipe record | `{"section_ref": <id>}` | `toolkit/SectionAssignment.tsx` |
| Reassign the material | pipes | `set_field`, Element, `material` | the pipe's current material id | the chosen material id | the inspector's Material field |
| Replace a support kind | supports | `update_support`, Support, `configuration` | canonical JSON of the support's configuration projection (`family`, `restraints`, `stiffness`, `hanger`, `nonlinear`, `provenance`, those present) | the same projection with `family` and, when entered, `restraints` replaced | `support-configuration/SupportConfigurationForm.tsx` |
| Remove a support kind | supports | `delete_support`, Support, `supports` | the support's trimmed label | `not_present` | `model-tree/removalIntents.ts` `buildDeleteSupportIntent` (reused) |
| Set the bend radius | bend and elbow components | `set_field`, Component, `geometry.bend_radius.value` | the current value, or `TBD` when absent | `{"value": <n>, "unit": <unit>}`, with the intent's `unit` and dimension `length` | the inspector's Bend radius field |
| Add a tag | every entity type that takes tags | the family's annotation kind (`set_field`; `update_support` for supports; `update_load` for load cases), `tags` | canonical JSON of the current tags, or `not_present` | the current tags with the new tag appended | `model-data/modelDataIntents.ts` `annotationIntent` (reused) |

Canonical JSON comes from the operation engine (`canonicalJsonString`), as the
existing editors use it. The builder is `features/bulk-change/bulkChange.ts`.
Each intent's `source.source_ref` names that module; the change itself is the
existing producer's.

## Scope

The scope is the current ordered selection. A run range is a range of rows in
the model tree or the bulk grid: Shift+click, or Shift+Space in the tree,
extends the selection from its anchor, and Ctrl or Cmd adds or removes one row,
as those surfaces already do. The viewport's box selection works as well. A
run-sheet grid (B1) is not on main; when it lands, its row ranges are
selections too and need nothing further here.

Each change reads the scope in its own family:

- **Section and material** act on the selected pipes.
- **Support changes** act on the selected supports, the supports at the
  selected nodes, and the supports at both end nodes of the selected pipes.
  This lets a range of a run reach the supports along it.
- **Bend radius** acts on the selected components, the components at the
  selected nodes and pipe end nodes, and the bends whose `bend_pipe_ref` names
  a selected pipe.
- **Add a tag** acts on each selected entity.

Each entity reached once is listed once, in selection order.

Keyboard focus on an engineering-table cell makes that cell's row the table's
selection, as the table already does. "Change selected…" opened from a cell
therefore acts on that one row. The row headers, the tree, the bulk grid and
the viewport leave a multiple selection as it is.

## Family-aware validation

Every entity reached falls into one of four lists:

- **Affected:** it changes, and one operation is written for it.
- **Unchanged:** it already has the value. Examples are a pipe already bound
  to the chosen section, a support already of the target kind with the same
  restraints, or an entity that already carries the tag. No operation is
  written, because the applier refuses a no-op annotation and a no-op adds
  nothing to review.
- **Not matched:** it is a support or component whose kind is not the one
  chosen, such as an anchor when guides are replaced. It is reached but
  outside the request.
- **Refused:** the change does not apply to it. The reason names the entity:
  - Section and material: a selected entity that is not a pipe.
  - Support changes: a selected entity that is not a support, node or pipe.
  - Bend radius: a selected entity that is not a component, node or pipe.
  - Add a tag: an entity type without tags (a diagnostic).
  - Replace a support kind: a matched support that carries `stiffness`,
    `hanger`, non-null `nonlinear`, `properties` or a `boundary_association`.
    Replacing its kind would discard data that the Support configuration form
    and Boundary authoring own, so it is changed there, one support at a time.
  - Remove a support kind: a matched support with a boundary association, or
    one that something still references. The references are the set the
    applier's `delete_support` refuses on (`OP-SUPPORT-DELETE-REFERENCED`), in
    its order:
    - primitive loads that target the support, sorted by the bytes of their
      ids with repeats dropped, as the applier sorts them;
    - supports that name it as their normal-reaction source;
    - load-case `analysis_state.support_states` entries that name it.

    The last two keep model order, as in the applier.

    The reason names each reference. `supportDeleteReferences` in
    `bulkChange.ts` mirrors `resolve_delete_support` in the applier's `lib.rs`
    and `support_references` in `load_state_authoring.rs`, so a change to one
    needs the same change to the other. It reads the model the plan was made
    from, so a reference is refused even when the referencing entity is
    removed in the same request.
  - Bend radius: a realized bend (`mechanics_interface.solver_consumption`
    `curved_bend_macro_element`). Its radius forms the arc between fixed
    tangent nodes, so it is changed in the inspector, one bend at a time.

The request itself is checked first: a section must be a pipe section of the
model, a material must exist, a radius must be a finite positive number with
a unit, and a tag must be non-empty, trimmed and at most 64 characters (the
document's Tags rule).

The applier still judges every operation when the batch is validated or
applied, for example a stale shared-section cache. Its findings appear in Batch review. The
desktop checks above are a first refusal, not a replacement.

## No inferred restraints

A support-kind replacement writes restraints in one of two ways:

- **Keep each support's restraints.** The `restraints` member is copied
  unchanged, and only `family` changes.
- **Write these restraints.** The user ticks the degrees of freedom: UX, UY,
  UZ, RX, RY and RZ. None are ticked to begin with, and nothing is filled in
  from the target kind. Queuing is refused until at least one is ticked.

The preview shows every affected support's kind and restraints before and
after. The restraints the user sees and confirms there are the ones written
(DEL-07-09 routing record: "No inferred restraints or stiffness"). The target
kinds are anchor, guide, line stop and vertical support. These need no
stiffness, hanger or nonlinear data, which a bulk change does not enter.

## Preview

The panel recomputes the plan from the current model and selection on every
change. It shows the number of affected entities, and the number unchanged,
not matched and refused. The affected list shows each entity's id with its
before and after values. The refused list shows each id with its reason.

## Queueing: one batch, all or nothing

"Queue as one batch" is enabled only when at least one entity is affected,
none is refused, the request is valid, and no other operation is running.
Otherwise it is disabled, with the first reason stated. It then:

- builds the batch (`batch_id` `bulk-<change>-<n>`), with one operation per
  affected entity, in list order;
- checks that the model and the selection are the ones the plan was made from;
  if either changed, it queues nothing and says so;
- hands the batch to the session's existing `handleQueueOperationBatch`.

The session binds the batch to the current model hash and opens Batch review.
A refusal queues nothing, and so does a failure to build any operation. The
batch is applied whole or not at all ("Apply entire batch"), and the batch-
apply path is unchanged.

## Undo

Applying the batch creates the queue's single undo checkpoint. One Undo
restores the model as it was before the whole bulk change.

## Routes

| Command | Button or menu | Keyboard |
|---|---|---|
| Open Bulk change on the selection | "Change selected…" in the entity context menu (tree rows, bulk-grid rows, engineering-table cells and row headers, viewport labels and canvas); the "Bulk change" tab under Editing and review tools | Shift+F10 or the ContextMenu key, then the item; Tab to the tab button, then Enter |
| Choose the change and its values | the panel's own controls | native controls; Tab moves between them |
| Queue as one batch | the panel's button | Enter or Space on it |
| Validate, Apply, Undo | Batch review and the existing Undo | as today |

"Change selected…" follows the B9 selection rule: it is disabled with the
existing reason when the menu's entity is not in the selection. It opens the
Operations section on the Bulk change tab and moves focus to the panel. The
panel shows the scope as the selection count.

## Out of scope

- **Operating-condition sets.** They arrive with C1 and a model-document
  version step. The model is 0.8.0 and has no such member, so nothing is
  reassigned.
- **Element-kind replacement** (for example valve to flange). The applier
  defers `Component.kind` (`lib.rs` `DEFERRED_FIELDS`: "component-kind editing
  is deferred to the component editor scope"). A kind change also moves
  geometry between family members (`bend_*`, `branch_*`, `rigid_*`), which no
  existing operation expresses as one exact change. It needs a new applier
  operation with its own review.
- **Bend radius classes.** No document member records a class. A class
  expressed as a multiple of diameter would be a derived value, which is not
  an entered one. The bend radius is offered as an entered value only.
- **Kinds that need data.** Spring, hanger, constant-effort and nonlinear
  targets need stiffness, loads or travel. They are configured per support.
- **Removing a tag, and notes.** They are not in the survey item. The
  inspector's annotation editor remains the route.
- **The command palette.** Its operations routing lives in the session file
  that another change is reordering, and the menu and the tab already give
  both routes.

## Evidence

- `features/bulk-change/bulkChange.test.ts`:
  - for each change, the generated batch equals a hand-built batch;
  - the family refusals, and the unchanged and not-matched lists;
  - the scope's reach through nodes and pipes, and the request checks;
  - no batch is built while any entity is refused.
- `features/bulk-change/BulkChangePanel.test.tsx`:
  - the preview count and the per-entity before and after list;
  - no restraint ticked to begin with;
  - nothing queued while any entity is refused;
  - queuing hands over exactly one batch;
  - a model change during building queues nothing.
- `features/bulk-change/bulkChange.session.test.tsx`: each change runs
  through the session and the wasm engine. It is queued, validates with no
  blocking finding, applies once, and one Undo gives back the original model
  hash.
- `features/bulk-change/bulkChange.edges.session.test.tsx`, through the wasm
  engine:
  - a radius entered in millimetres or inches is written and stored as
    entered, unconverted, in the inspector field's shape;
  - an absent radius has the `TBD` before and applies;
  - a normal-reaction source, and a support a primitive load targets, are
    refused before queuing, with the reference named;
  - the same removal queued by another route is blocked by the applier
    (`OP-SUPPORT-DELETE-REFERENCED`), leaving the model unchanged;
  - "Keep each support's restraints" writes them exactly.
- `features/bulk-change/BulkChangeMenus.test.tsx`: Change selected… in the
  menus of the tree (from the keyboard), the bulk grid (with the B9 disabled
  reason), an engineering-table cell and the viewport. It is absent where
  nothing is wired.
- `App.bulkChange.test.tsx`: through the shell, from the tree's menu on a
  Shift range to the focused panel, then Batch review, Apply and Undo. The
  checkpoint restores the primary selection, as every Undo does. Keyboard alone (Shift+F10 or the ContextMenu key, then
  Enter) from a table cell, a table row header, a viewport label and the
  canvas opens the panel on the selection. Keyboard focus on a table cell
  already makes its row the selection, so that route acts on one row.
