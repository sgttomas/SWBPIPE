# Reference-entry journey (RT-G2/RT-G3)

**Basis:** `main` at `20614594c` (#130's merge, after #128); run sheet DEL-09-01 `Work/reference-system-run-sheet.md`
§2–§3.14; fixture `fixtures/reference_system/reference_system.json`. Exit criterion 7
(`docs/RELEASE_TARGET_v1.0.md`) is context only.

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

Each route is one of two kinds:

- **Typed UI:** a person's entry in a rendered control.
- **UI function:** the same function a control calls.

No step is blocked.

Selections use the session's `handleSelectEntity`, the function the Model tree and the
load-case list call.

| Step | Route | Records matched |
|---|---|---|
| §2.1 Blank load/reference model, SI | UI function `requestBlankProject("load_state")` (the Project page button's handler); the units dialog typed (Preset SI, Create) | `schema_version`, `document_kind`, `project.units` |
| §3.1 Pressure profile | typed UI | `pressure_contract` |
| §3.2 Material RS-M1, four temperature points, expansion law | typed UI | `material:RS-M1` |
| §3.3 Sections | typed UI (New section, three times) | 3 sections |
| §3.4/§3.5 Run sheet, 26 rows | Header and column units: draft setup (the sheet's draft state is set directly, not through the header controls, so a defect in those controls is outside this evidence). The 20 other rows: UI function, the sheet's own commit functions, as `referenceBendSifs.test.tsx` does. Typed in the grid: bend rows 8, 10 and 25 and valve/flange rows 3–5, 105 cells with their SIFs, Component details and three node labels. Queue rows, Validate batch and Apply entire batch: typed UI | 27 nodes, 26 pipes, `RS-B1`–`B3`, `RS-F1`, `RS-V1`, `RS-F2`, 10 supports |
| §3.4 RS-050 X = 1.3 | typed UI (Task, X position, Add, Apply) | `node:RS-050` |
| §3.6.1 Expansion joint RS-EJ1 | typed UI (New component, expansion_joint, tie rods, pressure model) | `component:RS-EJ1` |
| §3.6.2 Tee RS-T1, two SIFs, modifier source | typed UI (New component; three Task edits) | `component:RS-T1` |
| §3.7 Variable spring RS-VS1 | typed UI: select RS-080, the inspector's support form (no Restrain box; Include hanger data; variable spring hanger; Add hanger stiffness, UZ, 180000 N/m; loads, travel, source, consumption); Queue support creation, Apply (below) | `support:RS-VS1` |
| §3.8 Reference configuration | typed UI, one member typed and 25 filled | `reference:RS-INSTALLED` |
| §3.9 Self-weight, three cases | typed UI (Generate, Queue complete plan, Validate batch, Apply entire batch) | `load:RS-SUS`, `RS-OPE`, `RS-SAM` with 28 primitives each |
| §3.10 Pressure regions | typed UI: five regions for RS-SUS; Copy for RS-OPE and RS-SAM | each case's `pressure_regions` |
| §3.11 Analysis states | typed UI with the fills (the support-state fill adds RS-VS1's `active_model_device` state with the other nine); RS-SAM through Copy and two motions | each case's `analysis_state`, with one RS-VS1 support state each |
| §3.12 Combinations | typed UI | `combination:RS-EXP`, `combination:RS-dSAM` |

**Totals.** All 82 of the fixture's 82 records match, each with every member compared. There
are 27 queue/apply cycles: the run sheet's §5 count of 27.

## RS-VS1 and the blocked list

The blocked list is empty, and the test asserts that it is. No record or nested entry is
excluded from the comparison.

**§3.7 RS-VS1** was blocked at #128: `create_support` refused its empty restraint set with
`OP-RESTRAINT-SET-EMPTY`. #130 is the enabling change. The applier now admits `restraints: []`
on `create_support` and on direct restraint replacement for a support whose `family` is exactly
`variable_spring_hanger` and whose `hanger.stiffness.dof` is exactly `UX`, `UY` or `UZ`
(`core/model_operations/operation_applier/src/lib.rs:9718-9733`,
`variable_spring_hanger_admits_empty_restraints`). Every other empty set is still refused.
The fixture is entered as recorded, with `restraints: []` and `hanger.stiffness.dof: "UZ"`.

The support form sends the record as typed. The test asserts that the queued `create_support`
intent's payload equals the fixture's `support:RS-VS1` exactly. The form adds no restraint and
does not change the DOF's case, pad it or default it. The payload the form sent:

```json
{"id":"support:RS-VS1","label":"Variable spring VS1 (user stiffness)","node":"node:RS-080","restraints":[],"family":"variable_spring_hanger","provenance":"invented reference-system fixture input (DEL-09-01 Work/reference-system.md); not standard, supplier, commercial or project data","hanger":{"hanger_type":"variable_spring_hanger","stiffness":{"dof":"UZ","value":{"value":180000,"unit":"N/m"}},"installed_load":{"value":8000,"unit":"N"},"cold_load":{"value":8000,"unit":"N"},"hot_load":{"value":7400,"unit":"N"},"travel_range":{"value":50,"unit":"mm"},"source_reference":"invented user value, DEL-09-01 Work/reference-system.md §5","mechanics_consumption":"linear_spring_primitive_user_stiffness"}}
```

After Apply, the model's RS-VS1 equals that payload.

**Not blocked.** The other candidate gaps were all entered:

- the hanger form;
- the expansion joint;
- self-weight generation;
- the tee;
- the RS-050 correction.

Skew and the spec §10 pending families are not part of tranche 1's fixture.

## Comparison allowances

Each allowance below is applied by member path. The test checks that the entered value is the
stated derived value before it sets the fixture's value in its place. For `source_model_hash`,
the entered value must equal the hash of the model each case was generated from, captured in the
journey; the fixture's value comes from its generator's capture model, so only its form is checked.
RS-VS1's nested entries, which #128 excluded, are asserted exactly (review r1 of #128). Each of the
three cases holds exactly one RS-VS1 support state in the fixture, and the entered document holds
the same one.

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
| Support | RS-VS1's creation through the support form |
| Load/reference state | RS-INSTALLED |
| Combination | RS-dSAM |

Save and reopen use the session's `handleSaveProject` and `handleOpenProject`. The reopened
document equals the entered one, and the comparison is repeated on it: 82 of 82 again.

## Observations for the run sheet's owners

These are reported, not changed here:

1. **§3.7 is no longer refused.** Since #130 it applies as written; see above.
2. **§3.6 field order loses the label.** §3.6 lists ID and Label before Kind. Choosing a kind
   replaces the label with a derived one (`component-creation/componentIntent.ts:335`), so the
   journey chooses Kind, node and pipe first.
3. **The self-weight plan keeps its pipe choice.** After a plan is queued, its 25 pipes stay
   selected for the next case.
4. **The session keeps 25 Undo checkpoints.** The journey's 27 cycles exceed this depth
   (`workspace/workspaceSession.ts:1602`).
5. **The browser has no unit catalog.** The catalog comes from the desktop. As
   `rigidQuantityUnits.test.tsx` does, the test supplies the committed
   `test-support/unitCatalogFixture.ts`; without it, the section form offers only the project's
   `m`.

## Shared helpers

The 26 rows, the header and the valve/flange Component details moved from
`runSheet/referenceBendSifs.test.tsx` to `src/test-support/referenceRunSheetEntry.ts`. That
test's assertions are unchanged.
