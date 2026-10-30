# Project unit selection (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23) for §1–§6; slice (b)'s
additions (§2.7 refusals, §2.8, §3, §4) at `25b7df82ae` (2026-10-25); slice (c)
(§7) at `9d51a4e744` (2026-10-26), with §7.5 from #1646. Slice (a) (§5) is
implemented in W-17 (`apps/desktop/src/features/project-units/`). Slice (b),
`set_project_unit` and the Project units section, is designed here and
implemented with it. Slice (c), the remaining fixed load-field defaults and the
export panels' unit fallbacks, is designed in §7 and implemented. That includes
B as B-c (§7.7), whose B-a and B2 remain deferred, and §7.5's export length
conversion, built in #1646. The §7.3 catalog entries also remain deferred.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

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
     (`report_package_save.rs`). Export unit manifests copy the current units.
     Exports convert node coordinates from the length key, which is locked,
     and other lengths from their own units (§7.5).
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
    **Object form (W-17).** #1650 (DEL-08-01 §2.6) recorded that the screen
    visited only object-valued provenance records. Probed at `e2f9677175`,
    the gap was reachable. A stored project or state whose
    `provenance_record` was an array (the positional form
    `["private_only", "quarantined", "Invented source"]`), text or null, or
    whose `rule_pack_refs[]` entry or `library_ref` was positional, was written
    out, and so was a record with an array-valued disposition. So was a
    reference to a stored library record whose disposition was an array or an
    object, which the ordinary library writer stores (#1651 Codex r1). The rows reach
    the store three ways. Save State keeps a snapshot with blocking slot
    findings as `MODEL_INCOMPLETE` (W-12 §2), so a session request carrying
    such a record is stored. Project saves before `a4e5e1103e` (2026-10-23)
    did not run the slot validator. The store is also a local SQLite file the
    user can edit; the export opens it with no validator, by design. Project
    saves now refuse such rows through the slot validator
    (`MODEL-PROVENANCE-RECORD-INVALID`, `MODEL-SLOT-SHAPE`).
    Recovery does no typed decode, so the fix is in the shared keyed screen
    (DEL-15-01 `canonical-model-export.md`, "Disposition screening"): such a
    record now refuses by name, `MODEL-DISPOSITION-RECORD-MALFORMED`, with no
    chooser and nothing written. A positional member outside the screened
    records, such as a node or the units map, still recovers. The fix does not
    depend on #1650's validator.
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
  Object form (§3): a screened record that is not an object, positional or
  otherwise, or one with an array-valued disposition, is refused
  `MODEL-DISPOSITION-RECORD-MALFORMED` on a raw project row and a raw state
  row. The same refusal applies to:
  - a positional `library_ref`;
  - a stored library it names whose library object is positional;
  - a stored library object or matching record holding a positional
    `provenance`, or an array- or object-valued `review_status` or
    `redistribution_status` at any depth. The ordinary library writer stores
    such members.
  - a positional record in a state that Save State kept.

  A whole stored record in positional form cannot be matched by its id, so its
  reference is `MODEL-LIBRARY-REF-UNRESOLVED`, a warning, and the document is
  written, as for an absent record. A positional node or units map still
  recovers. Each refusal test failed with the document written, on
  `e2f9677175` or, for the stored members, on `20bde6a3a3`
  (`stored_model_document_export.rs`).
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

**Rulings.**
- The owner, 2026-10-26: "units conversions that are routinely done in
  engineering applications are permissible as a solution."
- Agent 0 chose, on the options first presented in this section:
  - **A:** revised (ii);
  - **B:** the declared stress unit must agree with the referenced results;
  - **C:** C2, exact conversion, in #1646;
  - **D:** in this slice.
- A Codex review of the first version of this section (`66c21f1092`) is folded
  in below.

**The rule** is §3's.
- A default unit for a project-derived quantity comes from `project.units`,
  either directly or by a catalog-checked composition (§2.4).
- A missing key is refused by name, and no other unit takes its place.
  "Missing" means absent, empty or whitespace-only.
- A unit that is already stored stays as stored. Changing a key later moves only
  the starting unit of a new entry: stored quantities, queued or captured
  operations, and undo/replay keep their units (§2.5, §2.8).

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
| `LocalFeaHandoffPanel.tsx:540-546` | `?? "m"`, `?? "N"`, `?? "MPa"`, `?? "degC"` | 7.4, 7.7 |
| `convertLengthToMillimeters` in `CaepipeMbfExportPanel.tsx` and `PcfExportPanel.tsx` | factor 1 for every unit except `m` (fixed by #1646) | 7.5 |
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

### 7.2 Temperature (settled)

- **The key.** `project.units.temperature` is one of the seven keys
  (`schemas/model.schema.yaml` `ProjectUnits`; §2.2), so model document 0.7.0
  needs no change.
- **One shared reader.** The project temperature default is read by one helper
  in `projectUnitSelection.ts`, `projectTemperatureDefault`. It returns:
  - the stored symbol, with a legacy `C` given as `degC`;
  - or `""` (no unit) when the key is absent, empty or whitespace-only.
  The thermal primitive load maps `""` to its own `"TBD"`. The editors keep
  `""`.
- **Inspector.** "Modulus basis solve temperature" defaults to that reading
  when the quantity is absent.
  - The field has its own unit selector, and the applier needs no project key
    for it (`operation_applier/src/lib.rs:815-821`, an `OptionalQuantity` with
    an explicit unit).
  - A missing key gives `projectUnitDefault(..., "temperature", "choose")`. The
    unit starts unselected, and the `choose` notice is shown. A unit picked in
    the selector lifts the refusal.
- **Grid.** The "Solve temperature" column uses the same reading.
  - The grid has no unit selector. An empty cell is refused ("Not queued:
    Solve temperature of <id> has no unit."), and the refusal sends the user to
    the inspector: "Missing project unit: temperature. This cell has no default
    unit, and nothing is assumed. Choose its unit in the property inspector, or
    set temperature in Project units on the Project page."
  - A cell whose quantity already carries a unit keeps it.
- **Numerical meaning** is unchanged.
  - The solver normalizes the entered quantity into kelvin
    (`core/product_physics/src/lib.rs`, the `normalize_quantity` loop over
    `modulus_basis_temperature`).
  - Stored values and results do not move.
  - Only a new entry's starting unit changes. In a `degC` project, typing 400 now
    proposes 400 degC, not 400 K, and the selector shows the unit before Review.

### 7.3 Acceleration (Agent 0: (ii), as built)

**The settled design:**
- `project.units` has no acceleration key, and none is added; option (i) stays
  reserved.
- The seismic gravity default is the composition `acceleration:
  <length>/s^2` (§2.4 `COMPOSITIONS`), checked against the catalog, for every
  project length unit.
- The solver normalizes the entered quantity to `m/s^2` before seismic
  generation (`product_physics/src/lib.rs`, the `gravity_acceleration`
  normalization), so no default changes a stored value or a result.

**The catalog requirement.** Today the catalog has one acceleration unit,
`m/s^2` (`core/units/src/lib.rs` `MeterPerSecondSquared`).
- Revised (ii) needs `mm/s^2`, `in/s^2` and `ft/s^2` as derived entries with
  exact factors to `m/s^2`: 0.001, 0.0254 and 0.3048.
- They are `core/units` additions. No schema enumerates catalog symbols, so
  they change no model-document format or schema version.
- They are numerical code with a CI routing digest. Agent 0 (2026-10-26)
  recorded them as a **deferred improvement** that needs a D3 turn in the
  numerical lane's queue. No Rust is changed in this slice.
- The desktop needs no further change when they land: the composition finds
  them in the catalog.

**As built (until the entries land):**

| Length key | Default | Field |
|---|---|---|
| `m` | `m/s^2` | as today |
| `mm`, `in`, `ft` | none | unselected, notice "`<length>/s^2` is not a catalog unit; choose one" |
| absent | none | `choose` refusal naming `length` |
| empty or whitespace-only | none | "project.units.length is empty; choose one" |

- A unit picked in the inspector lifts the refusal.
- **A catalog that arrives late.** Opening the field before the catalog loads
  starts it at the unchecked composition. When the catalog arrives, an
  inherited default is recomputed and replaced. A unit the user picked, and a
  stored unit, are kept. The inspector holds the user's choice apart from the
  field's default; the default follows the field (Codex review round 1, P2).
- In the grid, an empty cell is refused by name. The refusal names the missing
  `length` or the unavailable composed symbol, and sends the user to the
  inspector.
- Where no catalog is loaded (the browser preview), the composition is kept
  unchecked, as in §2.4. The applier's unit validation still applies.
- The creation dialog's composed-defaults list does not gain the acceleration
  row until the entries land. §4's "every composed default is a catalog unit"
  for both presets would otherwise fail on the US preset.

### 7.4 Export panels' missing project keys (built)

These refusals cover units the exports take from `project.units`. Other units
in the same files are not affected:
- the target units (`mm`, `MM`) come from the target profile;
- the local FEA result-unit declarations are covered by §7.7.

- **The refusal.** If a key that a panel reads is missing (`projectUnitMissing`:
  absent, empty or whitespace-only), the export is refused by name:
  - the .mbf and PCF packets record a blocking `*-PROJECT-UNIT-MISSING`
    diagnostic per missing key, and write `""` where the key would go, never a
    substitute;
  - the local FEA packet is not built;
  - the download controls are not rendered, and a status line with test id
    `<prefix>-project-unit-refusal` takes their place;
  - the summary reads `refused`.
  The load-reference output gate keeps precedence: its refusal is shown first,
  and the unit refusal sits inside it.
- **Text:** "`<CODE>`: Missing project unit: `<key>`. This export writes
  `<what>` from project.units.`<key>`, and nothing is assumed. Set `<key>` in
  Project units on the Project page."
  - Missing `length` with nodes present gives `<CODE>:` followed by §3's
    hard-lock text (the Project units section's notice). Its remediation
    agrees: "Re-author the model with a length unit before exporting <text>;
    the positions have no stated unit, and no fix is offered." Any other
    missing key's remediation is to set the key in Project units on the
    Project page.
- **One helper.** The key check, the text and the remediation are in
  `features/project-units/exportProjectUnits.ts`.
- **Precedence over §7.5.** For a missing length key,
  `MBF-PROJECT-UNIT-MISSING` or `PCF-PROJECT-UNIT-MISSING` is recorded instead
  of `*-LENGTH-UNIT-UNSUPPORTED (missing)`, not beside it. The panel shows the
  project-unit status line, and the length status line is not shown. A length
  that is present but has no routine conversion (`cm`) keeps §7.5's refusal.
  `exportProjectUnitRefusal.test.tsx` tests the order. #1646's
  `exportLengthConversion.test.tsx` keeps only its `cm` case for coordinates.

| Export | Project-derived source units | Missing key refuses as |
|---|---|---|
| CAEPIPE MBF (`CaepipeMbfExportPanel.tsx`) | node-coordinate source unit (`length`); the `UNIT` record's force and temperature symbols, written as stored | `MBF-PROJECT-UNIT-MISSING` |
| PCF (`PcfExportPanel.tsx`) | node-coordinate source unit (`length`), the same in the payload and the witnesses | `PCF-PROJECT-UNIT-MISSING` |
| Local FEA handoff (`LocalFeaHandoffPanel.tsx`) | `coordinate_unit` (`length`), `temperature_unit`, and `force_unit` or `stress_unit` where no result row of that family is referenced (§7.7) | `LOCAL-FEA-PROJECT-UNIT-MISSING` |

- **The fallbacks.** The `??` fallbacks are removed.
- **Unchanged output.** With every key present, the .mbf and PCF texts and
  whole packets for the demo in `m`, `mm`, `in` and `ft` hash identically
  before and after (checked against `origin/main` `b9fa486209`).
- **Shared with #1639.** None of these files is in #1639.

### 7.5 Export length conversion (C2)

**Ruling.** The owner, 2026-10-26: "units conversions that are routinely done in
engineering applications are permissible as a solution." Agent 0 chose C2, exact
conversion, with no interim refusal for units that convert routinely.

**The defect.** Before this change, both exports' `convertLengthToMillimeters`
multiplied by 1000 for `m` and by 1 for every other unit:
- 1 in and 1 ft were written as 1 mm in the .mbf and PCF text.
- A PCF OD of 6 in and a wall of 0.25 in were written as `6` and `0.25` MM. Their
  witnesses recorded `conversion_factor_to_target: 1` and
  `source_unit_matches_target_unit`, and no section-unit diagnostic was raised.
- This happened with a project length of `m` too, because section quantities
  carry their own units.

**Exposure before the fix.** With the real `ControlledExportLink`, the text
controls have no download href, and the downloadable package JSON redacts
geometry and embedded text. But the package's structural allowlist keeps
`conversion_factor_to_target` (`redactionExportControls.ts`, the PCF and .mbf
structural public paths), so the false conversion evidence was downloadable.

**Rule.**
- **Exact factors, in one place.** `apps/desktop/src/features/exportLengthConversion.ts`
  holds the only table, in millimetres per unit: `m` 1000, `mm` 1, `in` 25.4
  and `ft` 304.8. These are exact by definition (1 in = 25.4 mm, 1 ft =
  304.8 mm). They are the four lengths the engine registers
  (`core/units/src/lib.rs` `UnitId::Meter`, `Millimeter`, `Inch`, `Foot`); the
  engine has no `cm`. The factor is applied as written, not as 0.0254 × 1000.
  The value keeps the exports' six-decimal normalisation,
  `Number((value * factor).toFixed(6))`. For example, 6 × 25.4 is
  152.39999999999998 in binary64 and is written as 152.4.
- **Per quantity.** Each quantity converts by its own stored unit. Node
  coordinates use `project.units.length`. PCF OD and wall thickness use their
  own section units, whatever the project length.
- **Witnesses tell the truth.** `conversion_factor_to_target` is the factor
  used. `source_unit_matches_target_unit` is recorded only when the source unit
  is `mm`.
- **No routine conversion: refused by name.** Any other length unit is refused
  by name, for example `cm`, `MM`, `TBD` or an empty section unit. A project
  length key that is absent, empty or whitespace-only is not refused here:
  §7.4's `*-PROJECT-UNIT-MISSING` replaces it. This sentence was edited when
  slice (c) built that refusal; #1646's text named the missing key here. The
  export records a blocking `MBF-LENGTH-UNIT-UNSUPPORTED` or
  `PCF-LENGTH-UNIT-UNSUPPORTED` diagnostic for the coordinates or for each
  section quantity concerned. Each diagnostic names the unit, and for a section
  quantity the segment and field.
  - Both download links are withheld. A status line takes their place
    (`caepipe-mbf-length-unit-refused`, `pcf-export-length-unit-refused`), and
    the summary reads `refused`. The load-reference output gate keeps
    precedence: the length refusal sits inside it.
  - What a refused quantity carries:
    - Its witness target and factor are `null`, with
      `refused_no_routine_exact_conversion`.
    - A refused node coordinate is `null` in the payload and
      `TBD_LENGTH_REFUSED` in the text.
    - A refused PCF OD or wall is the placeholder string
      `TBD_OD_UNIT_UNSUPPORTED` or `TBD_WALL_UNIT_UNSUPPORTED`.
    - Nothing records a conversion that did not happen.
  - These replace the earlier `MBF-NODE-COORDINATE-UNIT-TBD` and
    `PCF-COORDINATE-UNIT-TBD`, which blocked `in` and `ft` but still offered
    the links. For a missing project length key, §7.4's
    `MBF-PROJECT-UNIT-MISSING` or `PCF-PROJECT-UNIT-MISSING` is recorded
    instead of the unsupported-unit refusal, and its status line is shown.
    This sentence was edited when slice (c) was built.
- **Out of range: refused by name.** A source value that is not finite (`NaN`,
  `Infinity`, `-Infinity`) is refused, and so is one whose converted value is
  not finite, such as 1e308 in overflowing after × 25.4. The helper checks the
  source and the product with `Number.isFinite`.
  - The export records a blocking `MBF-LENGTH-OUT-OF-RANGE` or
    `PCF-LENGTH-OUT-OF-RANGE` diagnostic. There is one per node coordinate
    concerned, read from the actual coordinates, and one per PCF section
    quantity. Each names the node and axis, or the segment and field, with the
    value and unit.
  - The links are withheld and the status line shows the refusal, as above.
  - The witness is `null` and `refused_out_of_range`. The payload is `null` for
    a coordinate, and `TBD_OD_OUT_OF_RANGE` or `TBD_WALL_OUT_OF_RANGE` for a
    PCF section quantity.
- **Rounding and very large values (engineering judgment, Agent 0 as HELP_HUMAN
  under the owner's 2026-10-26 delegation).** Six-decimal millimetre
  normalisation of the binary64 product is within normal engineering practice,
  and it is kept.
  - **How it was measured.** Independent scrutiny of #1646 ran three bounded
    sweeps of 100,000 inputs each, at 1e-8 increments. The worst difference from
    the correctly rounded exact decimal product was 0.000000552 mm (0.552 nm):
    1000000.00044176 ft writes 304800000.134649, while the exact product rounds
    to 304800000.134648.
  - **Not a global bound.** At extreme magnitudes precision degrades, and values
    that overflow are refused.
  - **Exponent notation.** From 1e21 mm upwards, JavaScript writes the value in
    exponent notation: a coordinate of 1e20 ft is written `3.048e+22`, and it
    agrees with its witness. No exponent prohibition was found in the PCF or
    .mbf references, but whether the target tools accept it has not been
    checked.
  - **Practical resolution.** Such magnitudes, 1e15 km and beyond, are far
    outside any piping model, so no further range limit is added.
- **Unchanged.** Output for `m` and `mm` is byte-identical.
- **Scope.** No other export converts lengths to millimetres. The local FEA
  handoff preserves source units (`conversion=false`), and its unit manifest is
  a separate question.
- **Checks.** `exportLengthConversion.test.tsx` checks the conversions against
  hand-computed millimetres for coordinates in `in` and `ft` and for OD and wall
  in `in` and `ft` under a project length of `m`. It also checks the witness
  factors through the real export controls, and both refusals with their links
  withheld: unsupported units, and out-of-range values (1e308 in, `NaN`,
  `Infinity` and `-Infinity` coordinates, and PCF OD and wall).

### 7.6 Component modifier defaults (settled; Agent 0: in this slice)

The component modifiers' fixed defaults become §2.4 compositions with the §3
refusal pattern. The affected fields are:
- linear, axial and lateral stiffness (`N/m`);
- rotational, angular and torsional stiffness (`N*m/rad`);
- the expansion joint's effective area (`m^2`).
They appear in the inspector and in the component grid.

- **Linear stiffness:** `declaredOrComposedDefault(linear_stiffness)`. This is
  the support draft's default: a declared `linear_stiffness` key, otherwise
  `<force>/<length>`.
- **Rotational stiffness:** `<moment>/<angle>`. The moment is read as declared
  or composed (§2.4).
- **Area:** `<length>^2`.
- **Missing parts and unavailable symbols.**
  - A missing part names its key. In the inspector the refusal is `choose`; in
    the grid the refusal points to the inspector.
  - A composed symbol the catalog lacks gives the §2.4 notice.
  - A stored unit is kept.
- **Effect by project:**
  - The SI preset and the bundled demo (no `moment` key) keep `N/m`, `N*m/rad`
    and `m^2`.
  - The US preset gives `lbf/in` and `in^2`. Rotational stiffness starts
    unselected, because `lbf*in/deg` is not a catalog unit.
  - Adding rotational-stiffness entries such as `lbf*in/deg` would be the same
    kind of `core/units` addition as §7.3's. They are not requested here and
    are recorded as an improvement.

### 7.7 Local FEA manifest result units (Agent 0's B-c, built)

**The problem.**
- The handoff refers to results and preserves their units. It carries no stress
  values: stress rows appear only as ids.
- `force_unit` and `stress_unit` came from the project keys.
- The bundled demo references two stress rows:
  - the governing summary maximum,
    `result:elastic-maximum:10:load:L-100:10:pipe:P-100`, in `Pa`;
  - `result:stress:pipe-P-120:end-j:torsional-shear`, in `MPa`.
- The manifest said `MPa`, so its label disagreed with the governing row.

**The decision.** Agent 0, 2026-10-26, under the engineering-judgment
delegation, chose B-c. It needs no schema change and touches only the
DEL-10-03 export builder (`buildLocalFeaHandoffPacket` and its panel).

**The rule** (`referencedResultUnits`), for each of `stress_unit` and
`force_unit`:
- **The governing row decides.** For stress, that is the summary maximum. With
  no governing row among the candidates, the candidates' shared unit is used.
- **Only rows in the declared unit are referenced.** A candidate in another unit
  is dropped from the result ids, stations and transfer lists. A non-blocking
  `LOCAL-FEA-RESULT-ROW-UNIT-OMITTED` diagnostic names it: "Result row <id>
  (<unit>) is not referenced: this handoff declares <family>_unit <declared>,
  from the governing <family> result, and references only <family> rows in that
  unit; nothing is converted." The diagnostic is appended after the standing
  findings, so their indices are kept.
- **Candidates that disagree, with none governing,** refuse the handoff:
  `LOCAL-FEA-RESULT-UNIT-INCONSISTENT`, shown at
  `local-fea-result-unit-refusal`, with no download. It is checked before the
  §7.4 missing-key refusal.
- **No row of a family referenced:** the member is the project key (§7.4).

**Results.**
- The demo now declares `stress_unit: Pa`.
- It no longer references the MPa torsional-shear row, and shows one omitted-row
  diagnostic for it. Its diagnostic count goes from 4 to 5.
- Its witnesses (displacement, force, moment) are unchanged at 3, and
  `force_unit` stays `N`.
- No `expected.json` value reads the handoff.

**Unchanged.** The schema, the candidate list (`selectedResultRefs`), the witness
policy, and the literals `moment_unit`, `displacement_unit` and `rotation_unit`
are not touched. On the routes the panel is open to, the literals equal the
referenced rows' units.

**Deferred** (recorded in DEL-10-03 `Work/manifest-result-units.md`):
- **B-a:** stress witnesses with values converted to the declared unit, so that
  rows in other units can be referenced. This is a DEL-10-03 schema change.
- **B2:** every result-unit member taken from the rows.

### 7.8 Ownership, overlap and sequencing

- **ScopeOfWork.** No commitment or acceptance criterion changes; the edits
  carry out existing text:
  - DEL-02-02: U-006, U-013 and "No silent defaults".
  - DEL-17-07: "no silent default for unit-bearing physical values" and "Do
    not use ... missing unit metadata as fallback".
  - DEL-10-03: "do not silently default".
  - DEL-17-04: AC "blocking diagnostics for incomplete packages".
  On implementation, the DEL-02-02 Design pointer adds slice (c). That pointer
  is a description, not a commitment.
- **Native smoke.** No `apps/desktop/native-smoke/expected.json` value moves:
  - its fixtures carry `length`, `force`, `stress` and `temperature`;
  - the exact route's MBF check reads the load-reference refusal, which keeps
    precedence;
  - no expectation reads a solve temperature, gravity, modifier or export unit,
    or the local FEA handoff.
- **Shared files.**
  - #1646 owns the MBF and PCF length conversion and this file's §7.5.
  - The completeness-map edits go to #1642.
  - `docs/user_guide/index.md` waits for #1639.
- **Files:**
  - `PropertyInspector.tsx`, `ModelTree.tsx` and `modelTableAdapter.ts`;
  - `project-units/projectUnitSelection.ts`, `missingProjectUnit.ts` and
    `exportProjectUnits.ts`;
  - `load-cases/primitiveLoadVocabulary.ts`;
  - the three export panels (§7.4, §7.7);
  - DEL-10-03 `Work/manifest-result-units.md`;
  - tests.
  No applier, schema, solver or `core/units` file changes. The §7.3 catalog
  entries are a deferred improvement.

### 7.9 Tests

- **Inspector, load case.**
  - In the demo (`temperature: degC`), the solve temperature starts at `degC`,
    and queueing 400 emits `{value: 400, unit: "degC"}`. This changes
    `schemaSlotEmission.test.tsx`'s and the e2e "DEC-077 solve temperature"
    expectation from `K`, for the reason in §7.2. The tests that store `K`
    explicitly are kept.
  - A legacy `C` starts at `degC`.
  - A key that is absent, empty or whitespace-only starts unselected with the
    `choose` notice, and nothing is queued. Picking `K` lifts the refusal and
    queues `K`.
  - A stored quantity keeps its own unit.
- **A project-key change.** After `temperature` changes from `degC` to `degF`:
  - a stored `K` quantity still reads `K`;
  - an operation captured before the change still carries `degC`;
  - a new entry starts at `degF`.
- **Acceleration.**
  - `m` gives `m/s^2`.
  - `mm`, `in` and `ft` start unselected with the notice naming the symbol.
    Picking `m/s^2` lifts the refusal.
  - An absent `length` gives a `choose` refusal naming `length`. An empty or
    whitespace-only key gives the empty-key notice.
  - With the catalog not loaded, the composition is unchecked.
  - The fixture test records `mm/s^2`, `in/s^2` and `ft/s^2` as absent from
    the core catalog. That expectation moves when the entries land.
- **Component modifiers.**
  - The SI preset and the demo keep `N/m`, `N*m/rad` and `m^2`.
  - The US preset gives `lbf/in` and `in^2`. Rotational stiffness starts
    unselected with the notice.
  - A missing part is named in the inspector and in the grid.
  - A stored unit is kept.
- **Grid.** An empty solve-temperature, gravity or modifier cell is refused by
  name, the refusal points to the inspector, and nothing is queued.
- **A catalog that arrives late** (`sliceCUnitDefaults.test.tsx`).
  - Gravity in an inch project, and US rotational stiffness: once the catalog
    loads, the field is unselected with the notice and nothing is queued.
  - A stored unit is kept.
  - A chosen unit is kept when the inherited default changes, while an inherited
    one follows.
  - The first two and the last fail on the inspector before the fix.
- **Local FEA (§7.7, `LocalFeaHandoffPanel.test.tsx`).**
  - An SI-preset key (`Pa`) and a US-preset key (`psi`, `lbf`) against rows in
    `MPa` and `N`: the declared units are the rows' units, and every referenced
    stress and force row carries them.
  - A row in another unit than the governing maximum is not referenced, and the
    non-blocking diagnostic names it.
  - Stress candidates in two units with none governing are refused by name,
    with no download.
  - The bundled demo declares `Pa`, references no MPa stress row, names the
    omitted row, and keeps 3 witnesses.
  - All of these fail on `origin/main`'s builder.
- **Export panels (§7.4, `exportProjectUnitRefusal.test.tsx`).**
  - A length key that is absent, empty or whitespace-only: `.mbf` and PCF
    record `*-PROJECT-UNIT-MISSING` and not `*-LENGTH-UNIT-UNSUPPORTED`. The
    hard-lock text is shown and the links are withheld.
  - The remediation says to re-author under the hard lock. With no nodes,
    both the text and the remediation say to set the key.
  - `cm` keeps §7.5's refusal.
  - MBF with force or temperature missing: refused by name, and `N` and `C`
    are not substituted.
  - A legacy `C` is written as stored.
  - Local FEA with each written key missing: refused, with no link.
  - With every key present, nothing changes.
- **Unchanged.** No Rust, applier, solver or native-smoke expectation changes.
