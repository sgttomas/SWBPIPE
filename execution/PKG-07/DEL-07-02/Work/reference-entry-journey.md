# Reference-entry journey (RT-G2/RT-G3)

**Basis:** `main` at `874378dde`; run sheet DEL-09-01 `Work/reference-system-run-sheet.md`
§2–§3.14; fixture `fixtures/reference_system/reference_system.json`. Exit criterion 7
(`docs/RELEASE_TARGET_v1.0.md` §4) is context only.

**What this is.** A maintained browser test,
`apps/desktop/src/features/workspace/referenceEntryJourney.session.test.tsx`, that enters the
reference system from a blank load/reference model in the run sheet's order. It uses one
`useWorkspaceSession` with the real wasm operation engine and the desktop's own surfaces:

- the units dialog;
- the pressure, material, temperature-point, expansion-law, section, component and support
  forms in the inspector;
- the run sheet's grid;
- the load manager, with its fills and copies;
- the self-weight plan;
- batch review and Review/Apply.

At the end it compares the document with the fixture record by record, keyed by id. It then
saves through the session, reopens through the session and compares again.

This is functional authoring-path evidence only. It is not a human timing result, a native
usability result or a physics result. No solve runs: the browser refuses one
(`services/previewService.ts:357-358`), so §3.13 is outside this test. The run sheet's
durations stay estimates.

## Steps and routes

Each route is one of three kinds:

- **Typed UI:** a person's entry in a rendered control.
- **UI function:** the same function a control calls.
- **Blocked.**

Selections use the session's `handleSelectEntity`, the function the Model tree and the
load-case list call.

| Step | Route | Records matched |
|---|---|---|
| §2.1 Blank load/reference model, SI | UI function `requestBlankProject("load_state")` (the Project page button's handler); the units dialog typed (Preset SI, Create) | `schema_version`, `document_kind`, `project.units` |
| §3.1 Pressure profile | typed UI | `pressure_contract` |
| §3.2 Material RS-M1, four temperature points, expansion law | typed UI | `material:RS-M1` |
| §3.3 Sections | typed UI (New section, three times) | 3 sections |
| §3.4/§3.5 Run sheet, 26 rows | Header and the 20 other rows: UI function, the sheet's own commit functions, as `referenceBendSifs.test.tsx` does. Typed in the grid: bend rows 8, 10 and 25 and valve/flange rows 3–5, 105 cells with their SIFs, Component details and three node labels. Queue rows, Validate batch and Apply entire batch: typed UI | 27 nodes, 26 pipes, `RS-B1`–`B3`, `RS-F1`, `RS-V1`, `RS-F2`, 10 supports |
| §3.4 RS-050 X = 1.3 | typed UI (Task, X position, Add, Apply) | `node:RS-050` |
| §3.6.1 Expansion joint RS-EJ1 | typed UI (New component, expansion_joint, tie rods, pressure model) | `component:RS-EJ1` |
| §3.6.2 Tee RS-T1, two SIFs, modifier source | typed UI (New component; three Task edits) | `component:RS-T1` |
| §3.7 Variable spring RS-VS1 | **blocked** (below); after the refusal, Clear pending changes through the UI function `handleClearReviewQueue`, because the button is disabled while no batch is queued | none |
| §3.8 Reference configuration | typed UI, one member typed and 25 filled | `reference:RS-INSTALLED` |
| §3.9 Self-weight, three cases | typed UI (Generate, Queue complete plan, Validate batch, Apply entire batch) | `load:RS-SUS`, `RS-OPE`, `RS-SAM` with 28 primitives each |
| §3.10 Pressure regions | typed UI: five regions for RS-SUS; Copy for RS-OPE and RS-SAM | each case's `pressure_regions` |
| §3.11 Analysis states | typed UI with the fills; RS-SAM through Copy and two motions | each case's `analysis_state` |
| §3.12 Combinations | typed UI | `combination:RS-EXP`, `combination:RS-dSAM` |

**Totals.** 81 of the fixture's 82 records match, each with every member compared. There are
26 queue/apply cycles plus RS-VS1's refused one: the run sheet's §5 count of 27.

## Blocked and unbuilt

**Blocked: §3.7 RS-VS1.**

- **Missing:** an engine operation that accepts a support with no restraints. `create_support`
  normalizes the restraint list and refuses an empty set with `OP-RESTRAINT-SET-EMPTY`. The
  fixture's spring has `restraints: []`, and §3.7 checks no Restrain box.
- **Evidence:** `core/model_operations/operation_applier/src/lib.rs:5478` (create_support's
  restraints) and `:9735-9744` (the refusal).
- **Unmatched in the fixture:** `support:RS-VS1`, and with it the RS-VS1 entry in each case's
  `analysis_state.support_states`.

The test asserts this refusal, and it asserts the blocked list. If either changes, the test
fails.

**Not blocked.** The other candidate gaps were all entered:

- the hanger form;
- the expansion joint;
- self-weight generation;
- the tee;
- the RS-050 correction.

Skew and the spec §10 pending families are not part of tranche 1's fixture.

## Comparison allowances

Each allowance below is applied by member path. The test checks that the entered value is the
stated derived value before it sets the fixture's value in its place.

From §3.14:

- `project.id`, `name` and `description`;
- pipe labels, which are the id's token;
- the derived bend labels on the six tangent nodes, three arcs and three bend components;
- RS-EJ1's `objective_connector.stiffness.provenance`, which is the connector's own source,
  measurement and transform references;
- `terms: []` on the two subtractions;
- `source_model_hash` inside each self-weight primitive's provenance;
- array order.

Array order is allowed in these places only:

- each collection, compared by id;
- `member_references`, by `pipe_ref`;
- `element_states`, by `pipe_ref`;
- `support_states`, by `support_ref`;
- each region's `member_pipe_ids`, as a set.

The nested lists follow the model's pipe and support order. That order is the bend append and
row-order cause that §3.14 gives.

**Not in §3.14.** These are the blank's own document members, and the test asserts each is
exactly the blank's:

- `analysis_status`;
- `data_boundary`, which is absent in the fixture;
- `diagnostics`, which holds the blank's blocking `BLANK_PROJECT_AUTHORING_TARGET` and is absent
  in the fixture.

The test also types the fixture's node labels in the Label cell, as §3.4 allows.

## Undo/Redo and save/reopen

Each Undo/Redo pair goes through Review/Apply's buttons. Undo gives the prior document exactly,
and Redo gives the applied document exactly. One step per family:

| Family | Step |
|---|---|
| Geometry and supports | the run sheet's batch, which creates the ten sheet supports |
| Component | RS-T1's creation |
| Load/reference state | RS-INSTALLED |
| Combination | RS-dSAM |

There is no separate support-form step to undo, because RS-VS1 is blocked.

Save and reopen use the session's `handleSaveProject` and `handleOpenProject`. The reopened
document equals the entered one, and the comparison is repeated on it.

## Observations for the run sheet's owners

These are reported, not changed here:

1. **§3.7 is refused**, as described above.
2. **§3.6 field order loses the label.** §3.6 lists ID and Label before Kind. Choosing a kind
   replaces the label with a derived one (`component-creation/componentIntent.ts:335`), so the
   journey chooses Kind, node and pipe first.
3. **The self-weight plan keeps its pipe choice.** After a plan is queued, its 25 pipes stay
   selected for the next case.
4. **The session keeps 25 Undo checkpoints.** The journey's 26 cycles exceed this depth
   (`workspace/workspaceSession.ts:1602`).
5. **The browser has no unit catalog.** The catalog comes from the desktop. As
   `rigidQuantityUnits.test.tsx` does, the test supplies the committed
   `test-support/unitCatalogFixture.ts`; without it, the section form offers only the project's
   `m`.

## Shared helpers

The 26 rows, the header and the valve/flange Component details moved from
`runSheet/referenceBendSifs.test.tsx` to `src/test-support/referenceRunSheetEntry.ts`. That
test's assertions are unchanged.
