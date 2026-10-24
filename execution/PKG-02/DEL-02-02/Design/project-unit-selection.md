# Project unit selection (W-17)

**Basis:** `origin/main` at `356a9be374` (2026-10-23). Slice (a) (§5) is
implemented in W-17 (`apps/desktop/src/features/project-units/`); slice (b),
`set_project_unit` and the Project units section, is not implemented. Standard claim fence applies (F-PIP-2; claims taxonomy per
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
  - Scope: `set_project_unit` (§2.5, §2.7), the Project units section with its
    missing-key handling, and the applier tests.
  - It reuses the `Project` target that S2 adds in applier `lib.rs`, rather than
    adding a second one.

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
