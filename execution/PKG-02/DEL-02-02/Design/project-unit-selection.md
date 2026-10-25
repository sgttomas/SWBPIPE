# Project unit selection (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23) for §1–§6; slice (b)'s
additions (§2.7 refusals, §2.8, §3, §4) at `25b7df82ae` (2026-10-25). Slice (a)
(§5) is implemented in W-17 (`apps/desktop/src/features/project-units/`). Slice
(b), `set_project_unit` and the Project units section, is designed here and
implemented with it. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).

Requirement: PRD §10.1 step 1, "User creates a project and selects units". Unit
policy: DEC-018 (D-01 accepted 2026-06-10: SI-canonical calculation, a dual
SI/US entered and display catalog, entered units preserved).

## 1. Where things stood before slice (a)

This section records the starting point at the design basis. Since slice (a),
blank projects open the units dialog and every new project carries all seven
keys, including `angle` (§5); the rest still applies.


- **Blank projects fix SI.** `buildBlankLocalModelDocument` writes
  `project.units` = `{length: m, force: N, moment: N*m, pressure: Pa, stress: Pa,
  temperature: degC}` (`apps/desktop/src/services/projectService.ts:395-402`).
  The blank load/reference builder inherits it (`:439-449`). "New blank", "Blank
  load/reference model" and the `file.new-blank` command create it at once, with
  no choice offered (`App.tsx:843-850,1181`; `workspaceSession.ts:1864-1872,2536`).
- **No `angle` key is written.** The applier refuses a rotational imposed
  displacement when `project.units.angle` is absent
  (`operation_applier/src/lib.rs:5229,5272-5280`, `OP-UNIT-METADATA-MISSING`).
  The desktop meanwhile assumes `rad` (`PropertyInspector.tsx:127,1776`;
  `PipeViewport.tsx:748`). On a blank project that load cannot be created.
- **Only node positions depend on the project unit.** They are the only bare
  numbers: `UnitSource::ProjectUnits("length")` covers `position.x/y/z` alone
  (`lib.rs:384-404`). Every other quantity is `{value, unit}`, stored in its
  entered unit.
  - **create_node** converts the entered unit into `project.units.length`
    (`lib.rs:3230-3275`, `length_value_in_unit` `:8505`).
  - **set_field** does the same for positions (`lib.rs:7026-7050`).
  - **The solver** reads only `project.units.length`, and converts positions to
    metres at its boundary (`core/product_physics/src/lib.rs:8815-8876`).
- **The other keys are preconditions and defaults.**
  - **Applier preconditions.** The applier requires the key to be present and
    of the right dimension before it accepts a value: `pressure` for material
    moduli (`lib.rs:4301-4325`), `temperature` for expansion and thermal loads
    (`:4446`, `:5228`), and `force`, `length` and `angle` for primitive loads
    (`:5224-5280`). It never converts a value into these keys: material moduli
    keep their entered unit (`:4436`).
  - **Desktop defaults.** The desktop uses the keys as the default unit of new
    fields (`componentIntent.ts:266-268`, `PipeViewport.tsx:435`).
  - **Composed defaults.** Some defaults are composed from the keys rather than
    read from them:
    - **load authoring** composes a concentrated moment as `<force>*<length>`
      and a distributed force as `<force>/<length>`; the moment ignores
      `project.units.moment` (`load-cases/primitiveLoadVocabulary.ts:71-81`,
      used at `LoadCaseManagerPanel.tsx:319,461`);
    - **the inspector** composes linear stiffness as `<force>/<length>`, unless
      a `linear_stiffness` key is present, and thermal expansion as
      `1/<temperature>` (`PropertyInspector.tsx:3072-3083`).

    Each composition is valid in the catalog for SI-metre and US-inch keys.
    Other valid choices compose symbols the catalog lacks: `mm` with `N` gives
    the moment `N*mm`, and `in` with `N` gives `N/in`. The new-load form then
    refuses its own default.
  - **Manifests.** Exports copy `project.units` into their unit manifests
    (`exportUnitDisclosure.ts:49`, `HandoffPanel.tsx:201,223`,
    `LocalFeaHandoffPanel.tsx:508-520`).
- **Display units are separate and session-only.** The Entered/SI/US selector
  converts readouts through the Rust unit engine, and "Editing uses entered
  units" (`features/display-units/index.tsx:50-72`, `targets.ts`). It is not
  stored. `App.tsx:149` mounts the provider with the default `entered`.

## 2. Decisions

1. **Where the choice lives.** `project.units` stays the one record: a map from
   dimension key to catalog symbol. No unit-system name is persisted. A preset
   is UI shorthand, and the map is what readers use. The schema already admits
   the seven keys (`schemas/model.schema.yaml` `ProjectUnits`), so neither S1b nor
   this item needs a schema change.
2. **Every new project carries seven keys:** `length`, `force`, `moment`,
   `pressure`, `stress`, `temperature`, `angle`. Each is a DEC-018 catalog symbol
   accepted for that key's dimension.
   - **Canonical symbols only, checked explicitly.** The chosen string must equal
     a catalog entry's `symbol` for the key's dimension. `unit_by_symbol` alone
     is not enough: it also accepts the legacy `C` for temperature
     (`core/units/src/lib.rs:870-876`), and `C` is refused for a new selection.
     The alias namespace is still open (DEL-02-02:3).
   - **Existing documents** keep a legacy `C`; readers still accept it.
   - Adding `angle` closes the rotational-displacement gap in §1.
3. **Presets.**
   - SI: `m, N, N*m, Pa, Pa, degC, rad`. This is today's set plus `angle`, and
     it is the default.
   - US customary: `in, lbf, lbf*in, psi, psi, degF, deg`.
   - Custom: any accepted catalog symbol per key, chosen from the unit catalog
     the desktop already loads (`unitCatalogService.ts`).
4. **Composed defaults are checked against the catalog.**
   - **Moment** defaults use `project.units.moment` whenever that key is
     present; a present key is never replaced by a composition. Only when the
     key is absent does the default fall back to `<force>*<length>`, checked
     against the catalog like the compositions below.
   - **Distributed force, linear stiffness and thermal expansion** defaults keep
     their compositions, and each composed symbol is checked against the catalog
     for its dimension. A composition the catalog lacks gives no default. The
     field starts unselected and says why ("`N/in` is not a catalog unit;
     choose one"). No other unit is substituted.
   - **The creation dialog** lists the composed defaults of a Custom selection
     and marks any the catalog lacks. That is a notice, not a refusal.
5. **Changing units after entities exist.**
   - **`length`** may change only while the document has no nodes. Once a node
     exists, a change is refused (`OP-PROJECT-UNIT-LENGTH-LOCKED`). Changing the
     key alone would rescale every position. Converting the positions would
     rewrite every entered coordinate in binary64, change the model hash and
     invalidate results, and the user gains nothing from it: positions can
     already be entered in any length unit (create_node converts), and readouts
     can be shown in SI or US.
   - **The other six keys** may change at any time. No stored value is
     converted, and existing quantities keep their entered units. The edit is a
     model edit like any other: it changes the hash and clears the current
     solve.
   - **Removing a key** is refused: the applier's preconditions need it.
   - **Display units** never touch the document. The selector stays session
     state; persisting it per user is outside this item.
6. **Entered units are preserved.** Nothing in this item converts a stored
   value. The solver's boundary normalization is unchanged.
7. **The edit is an operation.** After creation, units change only through the
   applier: one `set_project_unit` change per key. It is shown as a diff and
   undone through the batch checkpoint (`workspaceSession.ts:1192-1223`).
   - **Target:** object type `Project`, with `ref` equal to the document's
     `project.id`. This is a singleton: any other `ref` is refused.
   - **Field path:** `project.units.<key>`, for one of the seven keys.
   - **Before:** the current symbol, or `not_present` when the key is absent; a
     key's absence is the only representation of "missing". A `before` that no
     longer matches is refused as stale (`OP-STALE-BEFORE-VALUE`, the applier's
     existing pattern, `lib.rs:7631-7656`).
   - **After:** the canonical catalog symbol (§2.2). There is no remove form.
   - **Validation:** canonical symbol, dimension and the length lock.
   - **Wire.** Operation kind `modify`, change kind `set_project_unit`, as
     `set_model_setting` is wired (`operation_applier/src/model_settings_authoring.rs`).
     It returns one diff row `{object_type: Project, field_path, before, after}`
     and changes no other byte of the document.
   - **Canonical check in Rust.** `unit_by_symbol` resolves aliases and falls
     back across compatible dimensions, so the applier does not use it here.
     The check is the desktop's (`projectUnitSelection.ts`
     `canonicalCatalogEntry`): an entry of `catalog_definitions()` whose
     `symbol` equals the string exactly, whose review status is accepted, and
     whose dimension is the key's. The key's dimension is `length`, `force`,
     `moment`, `pressure`, `stress`, `temperature` or `angle`, matched as the
     desktop's `unitEntryMatchesDimension` matches it. `core/units` is not
     changed.
   - **Refusals, by name.** Each refusal leaves the document unchanged and
     queues nothing.

     | Condition | Code |
     |---|---|
     | Target type is not `Project`, or a member is missing or not a string | `OP-PROJECT-UNIT-SHAPE-INVALID` |
     | `ref` is not the document's `project.id` | `OP-TARGET-NOT-FOUND` |
     | Document kind or version not the current model document | `OP-PROJECT-UNIT-DOCUMENT-INVALID` |
     | Field path is not `project.units.<key>` for one of the seven keys | `OP-PROJECT-UNIT-UNKNOWN-KEY` |
     | `after` is absent, null or `not_present` (a removal) | `OP-PROJECT-UNIT-REMOVAL-REFUSED` |
     | `after` is not an exact accepted catalog symbol (unknown, an alias such as `C`, or a different spelling) | `OP-PROJECT-UNIT-NOT-CANONICAL` |
     | `after` is a canonical symbol of another dimension | `OP-UNIT-DIMENSION-MISMATCH` |
     | `before` does not match the stored value | `OP-STALE-BEFORE-VALUE` |
     | `after` equals the stored value | `OP-PROJECT-UNIT-UNCHANGED` |
     | Key `length`, and the document has at least one node | `OP-PROJECT-UNIT-LENGTH-LOCKED` |

     The checks run in the table's order, so a stale `before` is reported
     before the length lock.
   - **Settled in implementation** (`project_units_authoring.rs`):
     - The change's `unit` must equal `after` (`OP-PROJECT-UNIT-SHAPE-INVALID`),
       and its `dimension` must be the key (`OP-UNIT-DIMENSION-MISMATCH`).
     - The applier's generic missing-member check gives way to the table's
       codes for this change kind. Through an atomic batch, the batch's own
       shape check still requires string `before` and `after`, so a removal
       sent that way is refused by the batch.
     - A `project.units` that is not an object is
       `OP-PROJECT-UNIT-DOCUMENT-INVALID`; an absent map is created holding
       the one key.
     - The `stress` key takes the catalog's pressure-dimension entries (`Pa`,
       `MPa`, `psi`), as the desktop's dimension match does.
8. **What a change after creation affects.** The edit changes one string in
   `project.units`. Everything else follows from the existing readers
   (survey at `25b7df82ae`):
   - **Stored values.** None is converted. Positions stay in the length unit,
     which cannot change while nodes exist; every `{value, unit}` quantity keeps
     its entered unit. The solver reads only `project.units.length`
     (`core/product_physics/src/lib.rs`, position normalization), so a change
     to any other key cannot move a solver result.
   - **The model document.** The schema is unchanged: model document 0.7.0
     already admits the seven keys. Changing a present key replaces one string.
     Setting an absent key adds that key, and setting a key when the map itself
     is absent adds the map holding it. The document hash changes, as for any
     edit.
   - **Defaults and preconditions.** New fields default to the new key's unit
     (§1 "Desktop defaults", §2.4); fields already authored keep theirs. The
     applier's preconditions read the new key on the next operation.
   - **Current results.** The edit goes through the same commit as every
     operation (`workspaceSession.ts` `commitModel`). That clears the current
     solve, analysis run, input manifest and rule-check aggregate. Undo and redo
     restore the checkpoint and clear them the same way. The user solves again.
   - **Saved states.** A state is the canonical text of the whole document, so
     a state saved before the change keeps its units. Nothing rewrites it.
     Comparing it with a later state shows one field at `project.units.<key>`
     on the `model` row (`stateComparison.ts`): `changed` when a present key
     was replaced, `added` when an absent key was set. It is not a
     `unit_changed` row, because that kind is for `{value, unit}` quantities.
   - **Recorded runs.** A run keeps its input manifest, whose
     `unit_basis.project_units` records the units at the run, and nothing
     rewrites it. Result rows carry the solver's own units, so run comparison
     compares rows of runs made before and after a change. It adds no context
     delta for the units: no result value depends on a non-length key, and the
     length key cannot change while nodes exist.
   - **Reports and exports.** They read the current document and the current
     result. The edit clears the current result, so a report cannot pair the
     new units with a result solved under the old ones. The report-package save
     already requires the report's units to equal the model text's
     (`report_package_save.rs`). Export unit manifests copy the current units;
     length conversion in exports uses the length key, which is locked.
   - **Rule checks.** The rule-check runner and the desktop rule-check service
     do not read `project.units`; inputs convert to each rule's declared unit.
     Only the cleared aggregate is affected.
   - **Display units** are session state and do not read `project.units`.

## 3. Desktop UX

- **Units at creation.** "New blank", "Blank load/reference model" and
  `file.new-blank` open a small dialog: a preset (SI selected), the seven keys
  with their symbols, a Custom choice per key, and Create. The builders take the
  chosen map; with no argument they produce the SI preset. The dialog text says:
  values are stored in the units you enter, and the solver converts at its
  boundary. It also says the length unit is fixed once nodes exist.
- **A "Project units" section on the Project page,** for an open project.
  - It shows the seven keys and allows the §2.5 edits. The length selector is
    disabled while nodes exist, with the reason shown.
  - A document opened without one of the keys shows a "missing project unit"
    notice naming the key.
    - **Missing `length` with nodes** is a hard lock. The notice is a re-author
      diagnostic: the positions have no stated unit, and assigning one would be
      an interpretation. No fix is offered.
    - **Any other missing key,** or `length` with no nodes, gets an offer to set
      it. The fix is a `set_project_unit` operation, never a silent default.
  - **Placement.** The section sits in the property inspector's project view,
    beside Model settings (`PropertyInspector.tsx`, selection type `project`).
  - **Editing.** It follows Model settings (`ModelSettingsSection.tsx`):
    - Each key offers the accepted catalog symbols of its dimension, the same
      list as the creation dialog's Custom choice.
    - Choosing one builds the intent and validates it against the current
      model hash. A refusal is shown by its code and message, and nothing is
      queued. Otherwise the change is queued for Review and Apply.
    - A second queued change for the same key is refused.
    - Choosing a unit validates and queues at once; there is no separate
      Queue button.
    - A key holding a legacy alias (`C`) is shown as stored, with a note that
      a new choice must be a catalog symbol.
- **The display selector is unchanged** and keeps its note.

## 4. Tests

- **Vitest, builders.** Each preset gives exactly seven keys, each a catalog
  symbol for its dimension. The default equals today's set plus `angle: rad`.
  Existing blank-document assertions (`projectService.test.ts:137,156,177`)
  follow.
- **Vitest, dialog.** Preset and custom selection, Create, the `file.new-blank`
  route, and refusal of `C` and every other non-canonical symbol.
- **Vitest, composed defaults:**
  - `mm` with `N`: the distributed default is `N/mm` (a catalog unit), and the
    moment default is the moment key, never `N*mm`;
  - a mixed SI/US selection (`in` with `N`): the distributed and stiffness
    fields start unselected with the notice;
  - both presets: every composed default is a catalog unit.
- **Applier.**
  - `set_project_unit` succeeds on a document with no nodes.
  - A length change is refused once a node exists, with the named code.
  - A non-length change leaves every other byte of the document unchanged.
  - An unknown symbol, a non-canonical symbol (`C`), a dimension mismatch, a
    removal, a stale `before` and a `ref` other than the project id are each
    refused.
  - Setting a key absent from an opened document succeeds with
    `before: not_present`. `length` with nodes is refused.
  - Undo restores the previous map.
  - Every code in the §2.7 table is reached by at least one test.
  - A legacy `C` temperature changes to `degC` with `before: "C"`.
  - Through an atomic batch, a unit change applies, and a later failing
    operation rolls it back with the document bytes unchanged.
  - The change-kind exemption from the generic shape check reaches no other
    path: a wrong operation kind, target type or field path is refused with
    nothing applied.
- **Through the apply command (src-tauri).** A force-key change applied through
  the apply command changes `project.units.force` and nothing else, and the
  same model solves to the same rows.
- **Vitest, section.** The seven keys shown; the length selector disabled with
  the reason while nodes exist; each missing-key notice (hard lock for `length`
  with nodes, an offer otherwise); a refusal shown with nothing queued; a
  second queued change for one key refused; undo and redo through the
  checkpoint.
- **Vitest, Project page without the map.** The property inspector's project
  view renders a document with no `project.units`, or with keys missing, with
  and without nodes. It shows the notices, offers the fix with
  `before: not_present`, and leaves the document unchanged. A model change
  during validation queues nothing and asks for a re-queue.
- **Applier, angle.** A rotational imposed displacement is accepted on a new
  blank document.
- **e2e.** Specs that click "New blank" gain the dialog's Create click.
- **Unchanged solver results.** No solver test changes. A US-preset blank model
  with nodes entered in inches solves to the same canonical positions as the
  same model in metres; this is a test of the existing normalization
  (DEL-02-02:2 already covers m/mm/in).

## 5. Files, overlap with W-13 and sequencing

| File | This item | W-13 S1b / S2 | Collision |
|---|---|---|---|
| `apps/desktop/src/services/projectService.ts` | blank builders take a units map; `angle` added | S1b edits the version constants and blank name (`:55,437,447`) | Same file, different hunks; no |
| `apps/desktop/src/features/workspace/workspaceSession.ts` | blank handlers pass the dialog's map | S1b's literal-version guard edits the `:1871` message | Same file, different hunks; no |
| `apps/desktop/src/features/load-cases/primitiveLoadVocabulary.ts`, `LoadCaseManagerPanel.tsx` | composed defaults (§2.4), slice (a) | none named in W-13 §6 | no |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | composed defaults (`:3072-3083`), slice (a) | S2 adds Tags, Notes and the Provenance record section | same file, different functions; textual only |
| `apps/desktop/src/App.tsx` | dialog trigger; Project units section | S2 may mount new panels | Low, textual only |
| new `apps/desktop/src/features/project-units/*` | dialog and section | none | no |
| `core/model_operations/operation_applier/src/lib.rs` | `set_project_unit` on `Project` | S2 adds `Project` to `collection_for` (`:869-880`) for project tags and notes; S1b edits version literals | **yes (S2, same match arms)** |
| `schemas/model.schema.yaml` | none | S1b rewrites it | no |
| D3-pinned files (`core/product_physics/**`, `core/units/**`, …) | none | S1b's D3 chain | no |

**Slices.** The work is two slices:
- **Slice (a): selecting units at creation.**
  - Scope: the blank-project dialog and the seven keys including `angle`
    (§2.1-2.4, §2.6), the composed defaults, and the builder, dialog,
    default and angle tests.
  - It touches no applier or pinned file.
- **Slice (b): editing units after creation.**
  - Scope: `set_project_unit` (§2.5, §2.7, §2.8), the Project units section
    with its missing-key handling, and the applier, src-tauri and section tests.
  - It uses the `Project` singleton target that W-13 S2 added for project tags
    and notes and that `set_model_setting` uses (`model_data_authoring.rs`,
    `model_settings_authoring.rs`), rather than adding a second one.
  - It touches no D3-pinned file and no schema.

## 6. Review findings resolved

The Codex review of `1f1b0d7fa9` found these, and the design above resolves
them:
1. **Composed defaults could fall outside the catalog (P2).** The moment default
   now uses the moment key, and every composed default is checked against the
   catalog, with tests (§1, §2.4, §4).
2. **`set_project_unit` was underspecified.** Its singleton target, stale
   `before`, missing-key form and canonical-symbol check are now defined
   (§2.2, §2.7).
3. **Missing `length` with existing nodes.** This is now a hard lock with a
   re-author diagnostic, and no repair is offered (§3).
4. **Moment fallback without a moment key** (found in implementation, not by
   that review). The composition is kept as the fallback because the bundled
   sample model has no `moment` key (§2.4).
