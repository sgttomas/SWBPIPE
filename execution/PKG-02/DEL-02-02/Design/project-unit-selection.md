# Project unit selection (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23). Design before
implementation. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).

Requirement: PRD §10.1 step 1, "User creates a project and selects units". Unit
policy: DEC-018 (D-01 accepted 2026-06-10: SI-canonical calculation, a dual
SI/US entered and display catalog, entered units preserved).

## 1. Where things stand

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
   accepted for that key's dimension (`unit_by_symbol`). Catalog symbols only;
   no aliases (the alias namespace is still open, DEL-02-02:3). Adding `angle`
   closes the rotational-displacement gap in §1.
3. **Presets.**
   - SI: `m, N, N*m, Pa, Pa, degC, rad`. This is today's set plus `angle`, and
     it is the default.
   - US customary: `in, lbf, lbf*in, psi, psi, degF, deg`.
   - Custom: any accepted catalog symbol per key, chosen from the unit catalog
     the desktop already loads (`unitCatalogService.ts`).
4. **Changing units after entities exist.**
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
5. **Entered units are preserved.** Nothing in this item converts a stored
   value. The solver's boundary normalization is unchanged.
6. **The edit is an operation.** After creation, units change only through the
   applier: one `set_project_unit` change per key, on target object type
   `Project` with field path `project.units.<key>`. It is validated (catalog
   symbol, dimension, the length lock), shown as a diff and undoable through the
   batch undo checkpoint (`workspaceSession.ts:1192-1223`).

## 3. Desktop UX

- **Units at creation.** "New blank", "Blank load/reference model" and
  `file.new-blank` open a small dialog: a preset (SI selected), the seven keys
  with their symbols, a Custom choice per key, and Create. The builders take the
  chosen map; with no argument they produce the SI preset. The dialog text says:
  values are stored in the units you enter, and the solver converts at its
  boundary. It also says the length unit is fixed once nodes exist.
- **A "Project units" section on the Project page,** for an open project.
  - It shows the seven keys and allows the §2.4 edits. The length selector is
    disabled while nodes exist, with the reason shown.
  - A document opened without one of the keys shows a "missing project unit"
    notice naming the key and offers to set it. The fix is a `set_project_unit`
    operation, never a silent default.
- **The display selector is unchanged** and keeps its note.

## 4. Tests

- **Vitest, builders.** Each preset gives exactly seven keys, each a catalog
  symbol for its dimension. The default equals today's set plus `angle: rad`.
  Existing blank-document assertions (`projectService.test.ts:137,156,177`)
  follow.
- **Vitest, dialog.** Preset and custom selection, Create, and the
  `file.new-blank` route.
- **Applier.**
  - `set_project_unit` succeeds on a document with no nodes.
  - A length change is refused once a node exists, with the named code.
  - A non-length change leaves every other byte of the document unchanged.
  - An unknown symbol, a dimension mismatch and a key removal are each refused.
  - Undo restores the previous map.
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
| `apps/desktop/src/App.tsx` | dialog trigger; Project units section | S2 may mount new panels | Low, textual only |
| new `apps/desktop/src/features/project-units/*` | dialog and section | none | no |
| `core/model_operations/operation_applier/src/lib.rs` | `set_project_unit` on `Project` | S2 adds `Project` to `collection_for` (`:869-880`) for project tags and notes; S1b edits version literals | **yes (S2, same match arms)** |
| `schemas/model.schema.yaml` | none | S1b rewrites it | no |
| D3-pinned files (`core/product_physics/**`, `core/units/**`, …) | none | S1b's D3 chain | no |

**Sequencing** (coordinator, 2026-10-23). The work is two slices:
- **Slice (a): selecting units at creation. It lands now.**
  - Scope: the blank-project dialog and the seven keys including `angle`
    (§2.1-2.3, §2.5), plus the builder, dialog and angle tests.
  - It touches no applier or pinned file. If S1b merges first, this slice
    rebases onto it.
- **Slice (b): editing units after creation. It waits for W-13 S2.**
  - Scope: `set_project_unit` (§2.4, §2.6), the Project units section, and the
    applier tests.
  - It reuses the `Project` target that S2 adds in applier `lib.rs`, rather than
    adding a second one.
