---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-09
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-077]
package_objective_refs: [OBJ-006, OBJ-015]
---

# Scope of Work — DEL-07-09 Interactive operation vocabulary and tool palette contract

## Purpose and Objective Traceability

This deliverable is the definition of what an engineer can do to a piping model by hand in the desktop, and of how those commands are arranged. It holds the vocabulary: the list of interactive model-building items, each classed as required now or as roadmap, and each bound to the structured operations that carry it out. It holds the one organization of the tool palette, so that the commands appear in one place under one grouping. And it holds the catalogue the palette is drawn from, which says by which entry and surface each required item is reached. It builds no editing surface itself. The result it contributes is that the command surface is complete against a ratified list, is organized once, and reaches the model only through structured operations.

It serves SOW-077 under OBJ-006 and OBJ-015 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-09), and PRD §14.1 and §5.3. The vocabulary is the annex `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`, with the ownership of its surfaces as amended by `execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/Vocabulary_Annex.md`. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It supports criterion 7, whose entry of the reference system uses the commands listed here.

- **OUT-001** — The vocabulary as this scope states it: each item, its class and the operations it binds to, with its coverage carried by the catalogue's routes and the owners the Boundary names.
- **OUT-002** — The palette organization and the catalogue the palette reads.

## Deliverable Definition — Ontology

**Terms the contract defines.**

- *Vocabulary item.* One kind of interactive model-building act. Its class is NORMATIVE-NOW, a coverage obligation, or ROADMAP, which is not one. Moving an item from ROADMAP to NORMATIVE-NOW is the owner's act.
- *Binding.* The operation kinds of the implemented operation taxonomy that carry an item out. An item that changes nothing in the model is stated as non-mutating.
- *Palette group.* One of the seven groups of the palette: Build, Supports, Properties, Loads, Edit, Select and View, Review.
- *Catalogue.* The machine-readable list of palette entries, each with an identifier, its group, the vocabulary items it serves and the surface that handles it. The palette is drawn from the catalogue and from nothing else.
- *Coverage.* For a NORMATIVE-NOW item: the route by which a user reaches it, the operations it produces and the deliverable that owns the surface. An item with no route is uncovered.

**The vocabulary.** NORMATIVE-NOW items, with their palette group:

| Row | Item | Group | Binding |
|---|---|---|---|
| 1 | Pipe run definition: routing and continuation | Build | create and delete a node, connect and delete a pipe run, set a field |
| 2 | Anchors and restraints by degree-of-freedom set | Supports | create, update and delete a support |
| 3 | Materials with temperature-dependent modulus, shear modulus and expansion | Properties | create a material, set a field |
| 4 | Temperature and pressure load cases | Loads | create and delete a load case, create a primitive load, set a field |
| 5 | Force, moment and displacement loads | Loads | create, update and delete a primitive load |
| 6 | Equivalent-static wind | Loads | update a load |
| 7 | Equivalent-static seismic acceleration | Loads | create a load case, update a load |
| 8 | Load-case combinations and range envelopes | Loads | create and delete a combination and a combination term, set a field |
| 9 | Typed selection identity | Select and View | non-mutating |
| 10 | Session undo and redo | Edit | restores a session checkpoint; produces no new operation |
| 11 | Agent proposal route: validate, preview, apply | Review | the batch route every change takes |
| 12 | Variable and constant-effort spring hangers | Supports | create and update a support |
| 13 | Nonlinear supports: one-way, lift-off, gap, friction | Supports | create and update a support |
| 14 | Expansion joints | Build | insert a component, set a field |
| 15 | Component creation: bends, tees, reducers, valves, flanges | Build | insert a component, set a field |
| 16 | Named support families line stop and vertical support, shown as themselves | Supports | create and update a support |
| 17 | Sections: create, delete and assign by reference | Properties | create, assign, detach and delete a section, set a field |
| 18 | Deletion of materials, sections and components | Edit | delete a material, a section, a component |
| 19 | Element insert and split | Build | split a pipe run |
| 20 | Copy, rotate and mirror of runs | Edit | transform a pipe run |
| 21 | Nozzle and equipment boundary conditions | Supports | create supports as one batch |
| 22 | Automatic self-weight case generation | Loads | create a load case and its distributed loads as one batch |
| 23 | Spring-hanger selection from user-imported hanger libraries | Properties | create or update a support |
| 24 | Project-wide unit switching | Select and View | non-mutating |

ROADMAP items: R1 node renumbering; R2 snubbers; R3 cold spring and cut-short.

The Review group also receives every pending change from every other group.

**Boundary.** This deliverable defines and checks coverage. It neither builds nor dispatches an editing surface. The surfaces are owned as follows:

- drawing the palette and routing its commands: DEL-07-01;
- editing in the tree, tables and inspector, including support creation and editing: DEL-07-02;
- load-case and combination editing, the self-weight plan, and the material, section, component and library editors: DEL-07-03;
- the rule-check run: DEL-07-04. Review, Apply and the ledger: DEL-07-08. The shell and session: DEL-07-11;
- the operation taxonomy, validation and application: DEL-16-01, DEL-16-02 and DEL-16-06;
- what each operation means for the mechanics: the engine deliverables.

**Not decided.** Whether R2, snubbers, becomes NORMATIVE-NOW now that the release target makes a dynamic-only snubber restraint a Must. Until the owner promotes the row it is no coverage obligation here.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The vocabulary lists every interactive model-building item with its class, and equals the ratified annex row for row. A ROADMAP item is no coverage obligation.
- **REQ-002** — Every NORMATIVE-NOW item binds to operation kinds of the implemented operation taxonomy, or is stated as non-mutating. No item binds to a kind that exists only in a schema enumeration.
- **REQ-003** — There is one route to the model. Every palette command that changes the model produces structured operations that pass validation, review and an explicit Apply. No command writes to the model directly. Routing each command so when it is used is DEL-07-01's, and this deliverable declares it as a need.
- **REQ-004** — There is one palette organization: the seven groups above. Every catalogue entry belongs to exactly one group, and every NORMATIVE-NOW item is served by at least one catalogue entry; an entry whose route cannot run states its held or unsupported state.
- **REQ-005** — The catalogue is the only source of palette entries. Each entry states its identifier, its group, the items it serves and the surface that handles it.
- **REQ-006** — Coverage is stated for each NORMATIVE-NOW item: the catalogue gives its entries and the surface that handles each, the vocabulary its operations, and the Boundary the owning deliverable. An item with no route is stated as uncovered, never as covered by a placeholder.
- **REQ-007** — Row 23 counts as covered only by a route that offers hanger records from libraries the user imported and from no other source. No hanger library is bundled.
- **REQ-008** — Row 24 counts as covered only by a route that changes no stored value: unit switching is a display choice.

### Acceptance criteria

- **AC-001** — The catalogue's entries together serve vocabulary rows 1 to 24 of the ratified annex and no other row, and its ROADMAP entries name R1 to R3 and no other item.
- **AC-002** — Every catalogue entry names one of the seven groups and at least one vocabulary item; and every NORMATIVE-NOW item is served by at least one entry.
- **AC-003** — Every operation kind a binding names exists in the implemented operation taxonomy.
- **AC-004** — Selecting entities and switching the project's display units leave the model document byte for byte unchanged.
- **AC-005** — The built product contains no hanger library, and the hanger selection offers no record until the user imports a library.

## Production and Verification Method — Praxeology

**Method choices.** The catalogue is one typed module that the palette imports, so an entry cannot be drawn without being listed.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the catalogue against the rows and classes of the ratified vocabulary annex): reads every entry's vocabulary rows and ROADMAP items and compares them with the annex.
- **VER-002** — Executable check (observes the catalogue): reads each entry's group and items and checks every NORMATIVE-NOW item against the entries.
- **VER-003** — Comparison with a named reference (the implemented operation taxonomy of DEL-16-01): compares every bound operation kind with the kinds the operation engine admits.
- **VER-004** — Executable check (observes a desktop session): selects entities and switches display units, and compares the model document's bytes before and after.
- **VER-005** — Executable check (observes the built product and a new session): scans the build for a hanger library and opens the hanger selection before any import.

## Governing Values and Decisions — Axiology

- **AX-001** — The list is fixed by the annex. Better is a vocabulary that equals the ratified annex, measured row for row, never an item added, dropped or reclassed because a surface happens to exist or not.
- **AX-002** — One palette, one route. Better is a command found in one group and reaching the model through the review queue, never a second toolbar or a shortcut around validation.
- **AX-003** — Coverage is a statement of fact. Better is an item stated as uncovered, never a route claimed for a surface that does not perform the bound operations.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-077 OBJ-006 OBJ-015 | REQ-001 REQ-006 | AC-001 | VER-001 | The ratified vocabulary annex and its ownership amendment, which this deliverable does not write. |
| OUT-002 | SOW-077 OBJ-006 | REQ-004 REQ-005 REQ-006 | AC-002 | VER-002 | The vocabulary table and the seven groups stated above are the reference for the catalogue. |
| OUT-001 | SOW-077 OBJ-015 | REQ-002 | AC-003 | VER-003 | Outside this deliverable: the operation kinds the engine admits (DEL-16-01, DEL-16-06). |
| OUT-001 | SOW-077 OBJ-006 | REQ-008 | AC-004 | VER-004 | The annex makes selection and unit switching non-mutating. The model document's bytes before the acts are the reference. |
| OUT-002 | SOW-077 OBJ-006 | REQ-007 | AC-005 | VER-005 | Hanger libraries are user-imported and none ships with the product. The build is scanned as built. |
