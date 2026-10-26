# Project unit selection (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23) for §1–§6; slice (b)'s
additions (§2.7 refusals, §2.8, §3, §4) at `25b7df82ae` (2026-10-25); slice (c)
(§7) at `9d51a4e744` (2026-10-26). Slice (a) (§5) is implemented in W-17
(`apps/desktop/src/features/project-units/`). Slice (b), `set_project_unit` and
the Project units section, is designed here and implemented with it. Slice (c),
the remaining fixed load-field defaults and the export panels' unit fallbacks,
is designed in §7 and not yet implemented. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).

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
       the one key. The desktop never presents a document without the map
       (§3, "The map is required at open"), so this applies only to callers
       of the applier outside the desktop.
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
- **The map is required at open.** Model document 0.7.0's schema allows
  `project.units` to be absent (`Project` requires only `id`), but the desktop
  refuses such a document by name, `MODEL-FILE-SHAPE` at `project.units`, on
  every path into a session:
  - Open Model Document (`modelDocumentShape.ts`);
  - Open local project, in the browser preview and natively
    (`projectService.ts`);
  - Open saved state (`useModelStateHistory.ts`).
  The bundled demo carries the map; the blank builders carry all seven keys.
  Undo, redo, applied operations and Save as new project start from a session
  model. Missing individual keys remain allowed.
  - **Recovery export.** A refused stored project or saved state is offered
    "Save the stored document to a file…": the Save Model Document dialog with
    its own explicit intent, raised by the refused open of a stored project
    and offered as a History panel button for a refused saved state. The
    native command (`stored_model_document_export.rs`) writes the stored text
    unchanged (a project's `model_json`, a state's canonical text), screens its
    dispositions as the save does, and uses the save's chooser and writer.
    Nothing is adopted. The store is opened read-only and never migrated: a
    store at another schema version is refused
    (`MODEL-DOC-EXPORT-STORE-NOT-CURRENT`). The command takes its arguments
    (the row's ids and the intent) as the `args-json-v1` raw body the history
    commands take, within their 256 KiB budget: a longer body is refused
    before it is decoded (`INGRESS-ARGS-TOO-LARGE`), and typed (JSON)
    arguments are refused. The read is admitted by the memory pool as the
    saved-state reader's is (`stored_read`), counted together with the
    ingress lease against the per-invocation budget, and the reservation of
    the text is held through the write (`STORE-READ-MEMORY-UNAVAILABLE`). Each
    library the screening resolves is read in the same transaction under its
    own reservations (SQLite's working set for the library keys, then the
    measured load and the parse of its document), held through the
    screening; a library the pool does not admit refuses the export by the
    same name. The shared disposition screener is not itself metered, so
    what it builds (a copied path for every site it records, a diagnostic for
    every finding and unresolved reference, and a path for every object of a
    hanger library) is bounded from the parsed model and each parsed library
    and reserved before it runs: the model's bound before the screening, each
    library's in its lookup. The screening itself is unchanged: every member
    is screened and a blocking disposition still blocks. The outcome is
    serialized within the same accounting while every reservation (the
    lease, the screening bounds, the text) is still held: its exact length is
    reserved, and a Saved response's bound is reserved before the chooser, so
    no response is refused after a file is written. A refusal whose
    diagnostics do not fit is sent as one named refusal that gives their
    count and quotes the first. A row above the model-document file cap is
    refused from its measured length before it is loaded
    (`MODEL-DOC-SAVE-TOO-LARGE`). A text that holds an
    object member twice is refused before screening
    (`MODEL-DOC-EXPORT-DUPLICATE-MEMBER`), so the document screened is the
    document written. The user adds the map and opens the file with Open
    Model Document.
- **Entity editors with a missing key** refuse by name; nothing is assumed
  (`features/project-units/missingProjectUnit.ts`).
  - **Fields with their own unit selector:** the unit starts unselected, and
    Add, Review and Apply are disabled. The notice reads: "Missing project
    unit: <key>. This field has no default unit, and nothing is assumed.
    Choose its unit here, or set <key> in Project units on the Project page."
    A unit the user picks there lifts the refusal and is kept.
  - **Centre of gravity** is the exception. Its vector field has its own
    editable unit selector, but with its default key missing the field keeps
    the `required` refusal: picking a unit there does not lift it, unlike the
    rule just above. It reads: "Missing project unit: <key>. This field takes
    its unit from project.units.<key>, and nothing is assumed. Set <key> in
    Project units on the Project page before editing it." A centre of gravity
    that already carries its own unit is not refused.
  - **Node positions without `length`** show the hard lock above, and picking a
    unit does not lift it.
  - **The table** refuses to queue such a cell: "Not queued: <column> of <id>
    has no unit." followed by the field's notice.
  - **The viewport's creation forms** show the same notices: component
    creation for each unselected unit, node creation for a missing `length`
    (the hard lock when nodes exist), and pipe creation while its unit is unset.
  - The former defaults `m`, `rad`, `N` and `Pa` are removed from the property
    inspector, the component draft, the viewport and the table.
  - The load fields' fixed defaults `K` and `m/s^2` did not come from
    `project.units` and were left by slice (b); slice (c) settles them (§7).
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
- **Vitest, entity editors and open paths.** A node, pipe, component and load
  each open on a document with every key, unchanged, and with each relevant
  key missing: no throw, the named refusal, no assumed unit and nothing queued
  (`model-tree/missingProjectUnitEditors.test.tsx`). A document without the
  map is refused by name when opened as a model document
  (`modelDocumentOpen.test.tsx`), as a local project in each runtime, and as a
  saved state (`compareStates.test.tsx`). The viewport's forms name the
  missing key (`viewport/PipeViewport.missingProjectUnit.test.tsx`).
- **Recovery export.** The refused open offers the export behind the explicit
  intent, sends only the row's identity and adopts nothing
  (`model-file/storedDocumentExport.test.tsx`, `compareStates.test.tsx`). The
  written file holds the stored bytes and the store file is unchanged; a
  blocked disposition, a duplicate member, a refused memory admission (of the
  row, or of a referenced library larger than the pool or than what remains
  of the budget), a row above the cap and a store at another schema version
  each refuse by name, show no chooser and write nothing, and the store is
  left unchanged; every reservation is released afterwards. An argument body
  above the budget, a JSON body and a body without the header are refused at
  ingress; the ingress lease counts against the budget; a well-formed
  envelope exports. The screening's allocations are refused by name under a
  pool below their peak (a hanger library with a 32 KiB key over 160 objects,
  and 150 unresolved references under a 32 KiB key) and export under an
  ordinary pool; the counting allocator checks the screening's bound against
  the screener's measured peak. One byte below the smallest budget that sends
  the whole response, the export is refused by naming the response, with no
  chooser and nothing written (`src-tauri/src/stored_model_document_export.rs`). The
  webview sends the `args-json-v1` body (`storedDocumentExport.test.tsx`).
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

## 7. Slice (c): load-field defaults and export unit fallbacks

Basis `9d51a4e744`. Agent 0 set the unit of work (2026-10-26): "The fixed `K` and
`m/s^2` load-field defaults, and the export panels' unit fallbacks ... in one
DEL-02-02 design-then-implement unit. Settle how temperature and acceleration
fields get their units under the project-unit design: whether `project.units`
covers those dimensions, and what a missing one refuses by name. Find the
export panels' fallbacks and bring them under the same rule."

The rule is §3's: a default unit comes from `project.units`, directly or by a
catalog-checked composition (§2.4). A missing key is refused by name, and no
other unit takes its place. Items marked **Open** wait for Agent 0 before they
are built. The rest is settled.

### 7.1 Inventory

Survey of `apps/desktop/src` (non-test) for `units.<key> ?? "<unit>"` and for
literal unit defaults:

| Site | Today | Section |
|---|---|---|
| `PropertyInspector.tsx:2237`, load case "Modulus basis solve temperature" | `?? "K"` | 7.2 |
| `ModelTree.tsx:1545-1553`, load grid "Solve temperature" | `"K"` | 7.2 |
| `PropertyInspector.tsx:2247`, "Seismic gravity acceleration" | `?? "m/s^2"` | 7.3 |
| `ModelTree.tsx:1554-1562`, load grid "Seismic gravity" | `"m/s^2"` | 7.3 |
| `CaepipeMbfExportPanel.tsx:331,405,596` | length `?? "m"` | 7.4 |
| `CaepipeMbfExportPanel.tsx:382-383` | force `?? "N"`, temperature `?? "C"` | 7.4 |
| `PcfExportPanel.tsx:416` (witness), `:356` and `:530` (payload and diagnostic, no fallback) | length `?? "m"` | 7.4 |
| `LocalFeaHandoffPanel.tsx:540-546` | `?? "m"`, `?? "N"`, `?? "MPa"`, `?? "degC"` | 7.4 |
| `convertLengthToMillimeters` in `CaepipeMbfExportPanel.tsx` and `PcfExportPanel.tsx` | factor 1 for every unit except `m` | 7.5 |
| Component modifier fields: `PropertyInspector.tsx:2074,2156,2166` (`N/m`), `:2084,2176,2186` (`N*m/rad`), `:2109` (`m^2`); the grid's `ModelTree.tsx:1450,1458,1473,1503,1511,1519,1527` | literal | 7.6 |

The survey also found these literal units, which are outside this rule:
- `LibraryManagerPanel.tsx` form defaults: a library document has no
  `project.units`.
- The rigid connector's `rotation_scale` and `coefficient_unit`
  (`componentIntent.ts:227-228`): fixed by the `scaled_work_coefficients_v1`
  representation.
- The node-proximity tolerance's default value (`modelSettingsIntents.ts:18`).
- The display targets (`display-units/targets.ts`).
- The CAEPIPE external harness's declared CSV units
  (`CaepipeExternalHarnessPanel.tsx:162-163`).
- The review geometry's metre target, which already blocks any length other
  than `m` without a substitute (`ReviewGeometryPanel.tsx:678-690`).

### 7.2 Temperature: settled

- **The key exists.** `project.units.temperature` is one of the seven keys
  (`schemas/model.schema.yaml` `ProjectUnits`; §2.2). Model document 0.7.0
  needs no change.
- **Inspector.** "Modulus basis solve temperature" defaults to the key when
  the quantity is absent. The key is read as the thermal primitive load reads it
  (`primitiveLoadVocabulary.ts` `projectTemperatureUnit`), so a legacy `C`
  becomes `degC` and a new field never takes the alias. The two readers share one
  helper.
  - The field has its own unit selector, and the applier needs no key for it
    (`operation_applier/src/lib.rs:815-821`, an `OptionalQuantity` with an
    explicit unit). With the key missing, it is
    `projectUnitDefault(..., "temperature", "choose")`: the unit starts
    unselected with the `choose` notice, and a unit picked there lifts the
    refusal.
- **Grid.** The "Solve temperature" column's fallback becomes the same reading,
  or `""`. Its dimension `temperature` is a project key, so the existing
  `missingProjectUnitCellRefusal` refuses an empty cell by name with the
  `required` text. A cell whose quantity already carries a unit keeps it
  (`quantityUnitValue`).
- **Numerical meaning.** None changes.
  - The solver normalizes the entered quantity into kelvin
    (`core/product_physics/src/lib.rs`, the `normalize_quantity` loop over
    `modulus_basis_temperature`; `retained_precision.rs` `unit_value`).
  - Stored values and results do not move.
  - What changes is the unit a new entry starts with. In a `degC` project,
    typing 400 now proposes 400 degC, not 400 K. The selector shows the unit
    before Review.

### 7.3 Acceleration: Open choice A

**What exists today:**
- `project.units` has no acceleration key.
- The schema's open map admits an `acceleration` entry
  (`additionalProperties: string`), but it is not one of the seven known keys.
  `set_project_unit` refuses it (`OP-PROJECT-UNIT-UNKNOWN-KEY`), and the
  dialog does not write it.
- The catalog has one acceleration unit, `m/s^2` (`core/units/src/lib.rs`
  `MeterPerSecondSquared`), and one time unit, `s`.
- The applier takes the field as an explicit-unit `OptionalQuantity`
  (`lib.rs:828-833`).
- The solver normalizes it to `m/s^2` before seismic generation
  (`product_physics/src/lib.rs`, the `gravity_acceleration` normalization).
  So no option below changes a stored value or a result.

**Options:**
- **(i) Add an `acceleration` key.** Reserved: it changes what a model document
  carries, and 0.8.0 is planned in the solver lane.
  - It extends the known keys, presets, dialog, `set_project_unit`'s key list
    and the schema description.
  - With one catalog unit, both presets would write `m/s^2`.
- **(ii) Derive from `length` (recommended).** Add the composition
  `acceleration: <length>/s^2` to §2.4's `COMPOSITIONS` and `COMPOSED_DEFAULTS`,
  checked against the catalog like the others.
  - **Results by length key:**

    | Length key | Default | Field |
    |---|---|---|
    | `m` | `m/s^2` | as today |
    | `mm` | none | unselected, notice "`mm/s^2` is not a catalog unit; choose one" |
    | `in` | none | unselected, notice "`in/s^2` is not a catalog unit; choose one" |
    | `ft` | none | unselected, notice "`ft/s^2` is not a catalog unit; choose one" |
    | missing | none | `choose` refusal naming `length` |

  - **Lifting the refusal:** in each unselected case the user picks `m/s^2`,
    and that lifts the refusal.
  - **Creation dialog:** its composed-defaults notice, shown for a Custom
    selection, gains a row, marked when the composition is not a catalog unit.
  - **Grid:** an empty cell is refused by name. The refusal names the
    composition's notice or the missing `length`, because `acceleration` is
    not a key.
  - **Cost and benefit:** no schema or applier change, and no new principle.
    The default follows a later catalog addition (`in/s^2`, `ft/s^2`) without
    another design change. Adding those units is a `core/units` change and
    lies outside this item.
- **(iii) Always explicit.** No default. The unit starts unselected everywhere,
  and the only choice is `m/s^2`.
  - The self-weight plan's gravity unit already works this way
    (`SelfWeightPlanPanel.tsx:38`).
  - It is the simplest form, but every metre project picks the unit by hand.
- **(iv) Default to a dimension's only accepted catalog unit.** `m/s^2` would
  stay the default in every project. No assumption is made today, because no
  alternative exists. When the catalog gains a second unit, the default
  disappears.
  - This is a new rule, independent of `project.units`.

Recommendation: (ii). It applies §2.4 unchanged and keeps today's default for
metre projects. Elsewhere it states why the field starts unselected.

### 7.4 Export panels: settled, with Open choice B

- **Inputs come from `project.units`.** A panel takes every unit it writes from
  `project.units`. If any key it reads is missing (`projectUnitMissing`), the
  export is refused by name:
  - the download controls are not rendered, and a status line with test id
    `<prefix>-project-unit-refusal` takes their place;
  - no packet is built with a substitute.
  - The load-reference output gate keeps precedence. Its refusal shows first,
    and the unit refusal sits inside it.
- **Refusal text.** `<CODE>: Missing project unit: <key>. This export writes
  <what> from project.units.<key>, and nothing is assumed. Set <key> in Project
  units on the Project page.` Missing `length` with nodes gives `<CODE>:`
  followed by §3's hard-lock text (`missingProjectUnitText(model, "length",
  "positions")`).
- **One helper.** The key check and text live in one helper beside
  `missingProjectUnit.ts`. Each panel passes its code and its required keys.

| Export | What the file needs | Keys read | Missing key refuses as | In #1639's files |
|---|---|---|---|---|
| CAEPIPE MBF (`CaepipeMbfExportPanel.tsx`) | Node coordinates converted to `mm` from the length key; a `UNIT` record of target length `mm`, then force and temperature symbols as stored (a legacy `C` is written as stored) | `length`, `force`, `temperature` | `MBF-PROJECT-UNIT-MISSING` | no |
| PCF (`PcfExportPanel.tsx`) | Node coordinates converted to `MM` from the length key; witnesses name the same source unit as the payload (today the witness says `m` while the payload uses the raw key) | `length` | `PCF-PROJECT-UNIT-MISSING` | no |
| Local FEA handoff (`LocalFeaHandoffPanel.tsx`) | `units_manifest`, whose seven members the schema requires non-empty (`local_fea_handoff.schema.yaml` `UnitsManifest`) | `length`, `force`, `stress`, `temperature` | `LOCAL-FEA-PROJECT-UNIT-MISSING` | no |

Each of the four fallbacks is removed. The MBF diagnostic at `:596` reads the
key itself, so a missing length is refused rather than treated as metres.

- **Open choice B: what the local FEA manifest declares.**
  - Its other three members are literals: `moment_unit: "N*m"`,
    `displacement_unit: "mm"`, `rotation_unit: "rad"`. They equal the solver's
    result-row units: result rows carry `mm`, `N`, `N*m`, `MPa` and `rad`.
  - `force_unit` and `stress_unit` come from the project keys. In an SI-preset
    project (`stress: Pa`) the manifest therefore says `Pa`, while the stress
    rows it references are in `MPa`.
  - **Options:**
    - (B1) Take `force_unit` and `stress_unit` from the project keys, as today,
      with the fallbacks removed and the three literals unchanged. This item
      does not change the manifest's meaning.
    - (B2) Declare model units (`coordinate_unit`, `temperature_unit`) from
      the keys, and result units (`force`, `moment`, `displacement`,
      `rotation`, `stress`) from the referenced result rows. This changes
      `force_unit` and `stress_unit` wherever the keys differ from the
      solver's units. It is a DEL-10-03 contract question.
  - **Recommendation:** B1 in this slice, and B2 raised with DEL-10-03.

### 7.5 The hidden conversion fallback: Open choice C

**The defect.** Both panels' `convertLengthToMillimeters` multiply by 1000 for
`m` and by 1 for every other unit. The text output is wrong in these cases:
- **Coordinates.** With a project length of `in` or `ft`, the MBF and PCF text
  carries unconverted coordinates labelled `mm`/`MM`. Each package records a
  blocking diagnostic (`MBF-NODE-COORDINATE-UNIT-TBD`,
  `PCF-COORDINATE-UNIT-TBD`), but the download links are still offered.
- **PCF section dimensions.** OD and wall thickness are converted from their
  own entered units. One entered in `in` is written unconverted as `MM`, and
  its witness says `source_unit_matches_target_unit`. No diagnostic is raised.
  The US preset makes this reachable on every such project.

**Options:**
- **(C1) Refuse (recommended for this slice).**
  - The helper returns no value for any unit other than `m` and `mm`.
  - A coordinate or section length it cannot convert refuses the export the way
    §7.4 does: the existing codes for coordinates, and a new
    `PCF-SECTION-UNIT-TBD` for a section dimension, naming the element and
    unit.
  - Output for `m` and `mm` is unchanged.
- **(C2) Convert `in`, `ft` and `cm` exactly through the unit engine.** This
  changes numerical export meaning, so it needs targeted regression checks and
  independent scrutiny (Root `AGENTS.md`). It belongs in a later item.
- **(C3) Leave it.** The text stays wrong for non-metric projects.

C1 changes no output for a project in `m` or `mm`. Because it affects what an
export contains, independent scrutiny of the implementation is still warranted.

### 7.6 Component modifier literals: Open choice D

The component modifiers' linear stiffness (`N/m`), rotational stiffness
(`N*m/rad`) and effective area (`m^2`) are the same kind of fixed default.
Agent 0's unit does not name them.

**Proposal, the §2.4 compositions:**
- **Linear stiffness:** `declaredOrComposedDefault(linear_stiffness)`, as the
  support draft already uses.
- **Rotational stiffness:** `<moment>/<angle>`, with the moment read as
  declared or composed.
- **Area:** `<length>^2`.

**Effect:**
- The SI preset and the bundled demo, which has no `moment` key, keep `N/m`,
  `N*m/rad` and `m^2`.
- The US preset gives `lbf/in` and `in^2`. Rotational stiffness starts
  unselected, because `lbf*in/deg` is not a catalog unit.

Options: build D in this slice, or as the next slice. Recommendation: in this
slice. The mechanism and the tests are the same, and the code is in the same
two files.

### 7.7 Ownership, overlap and sequencing

- **ScopeOfWork.** No commitment or acceptance criterion changes under the
  recommended options. The edits carry out existing commitments:
  - DEL-02-02: U-006, U-013 and "No silent defaults".
  - DEL-17-07: "Missing or ambiguous units | Diagnostic or blocking loss entry;
    no silent default for unit-bearing physical values"; "Units | Require
    explicit unit policy. Do not use dimensionless or missing unit metadata as
    fallback."
  - DEL-10-03: "Units or dimensional basis are missing | Emit `SOLVE_BLOCKING`
    or export-blocking diagnostics where applicable; do not silently default."
  - DEL-17-04: AC "blocking diagnostics for incomplete packages".

  On implementation, the DEL-02-02 Design pointer adds slice (c). That pointer
  is a description, not a commitment. Option (i) is reserved for its
  model-document consequence. It would change no ScopeOfWork text.
- **Native smoke.** No `apps/desktop/native-smoke/expected.json` value moves:
  - its fixtures carry `length`, `force`, `stress` and `temperature`;
  - the exact route's MBF check reads the load-reference refusal, which keeps
    precedence;
  - no expectation reads a solve temperature, gravity or export unit.
- **Open PR #1639.** None of the code files is in its file list.
  `docs/COMPLETENESS_MAP.md` and `docs/user_guide/index.md` are. Any
  reconciliation edit to them at implementation is sequenced through Agent 0.
- **Files:**
  - `PropertyInspector.tsx`, `ModelTree.tsx`
  - `project-units/projectUnitSelection.ts`, `missingProjectUnit.ts`
  - `load-cases/primitiveLoadVocabulary.ts` (the shared temperature reader)
  - the three export panels
  - tests

  No applier, schema, `core/units` or solver file changes.

### 7.8 Tests

- **Vitest, inspector (load case).**
  - In the demo (`temperature: degC`), the solve temperature starts at `degC`.
    Queueing 400 emits `{value: 400, unit: "degC"}`. The expectation in
    `schemaSlotEmission.test.tsx:175` changes from `K`, the reason being
    §7.2.
  - A legacy `C` key starts at `degC`.
  - A missing key starts unselected with the `choose` notice, and nothing is
    queued. Picking `K` lifts the refusal and queues `K`.
  - A stored quantity keeps its own unit.
- **Vitest, acceleration (option ii).**
  - With length `m`, the default is `m/s^2`; `schemaSlotEmission.test.tsx:179`
    is unchanged.
  - With `mm`, `in` or `ft`, the field starts unselected with the named notice,
    and picking `m/s^2` lifts it.
  - With `length` missing, the `choose` refusal names `length`.
  - With the catalog not loaded, the composition stays unchecked.
  - `composedDefaultsFor` lists the acceleration composition for both presets'
    maps. The US map's entry is marked as not in the catalog. The dialog test
    for a Custom selection follows.
- **Vitest, grid.**
  - The "Solve temperature" column takes the key. An empty cell with the key
    missing is refused with "Not queued: Solve temperature of <id> has no
    unit.", and nothing is queued.
  - A gravity cell is refused by name in each unselected case.
  - Stored units are shown as stored.
- **Vitest, export panels.** For each panel:
  - Each required key missing in turn: the named code and text are shown, no
    `*-export-link` or `*-text-link` is rendered, and no builder output
    carries a substitute symbol.
  - Missing `length` with nodes shows the hard-lock text.
  - With every key present, the demo's packets are unchanged:
    - the MBF `UNIT` record is `mm`, `N`, `degC`;
    - the PCF witness count stays 23 (`App.test.tsx`);
    - the local FEA manifest is `m`, `N`, `MPa`, `degC`.
  - A legacy `C` is written as stored in the MBF `UNIT` record.
  - On the exact route with a key missing, the load-reference refusal is shown
    and the unit refusal is not.
  - `LocalFeaHandoffPanel.test.tsx`'s model has no `stress` key; today it gets
    the `MPa` fallback. It gains the key, and a separate case asserts the
    refusal.
- **Vitest, option C1.**
  - `convertLengthToMillimeters` gives no value for anything except `m`
    (×1000) and `mm` (×1).
  - Length `in` withholds both panels' links under the existing codes.
  - A PCF section OD in `in` refuses with `PCF-SECTION-UNIT-TBD`.
  - Output for `m` and `mm` is unchanged.
- **Vitest, option D (if built).**
  - The SI preset and the demo keep `N/m`, `N*m/rad` and `m^2`.
  - The US preset gives `lbf/in` and `in^2`, and rotational stiffness starts
    unselected with the notice.
  - Missing parts are refused by name in the inspector and in the grid.
- **e2e.** In `r2-smoke.spec.ts`, "DEC-077 solve temperature queues an
  explicit unit-bearing operation" expects the `degC` default and
  `{"value":400,"unit":"degC"}` (the demo's key). The reason is §7.2.
- **Unchanged.** No Rust, applier, solver or validation-reference test changes,
  and no native-smoke expectation changes.
