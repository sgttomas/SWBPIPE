# Plan Completion Log

Archive ledger for landed items from the active completion plan (currently
[PLAN_2026-06-10_prd_completion.md](PLAN_2026-06-10_prd_completion.md)). When
a plan item lands, its row in the plan is compressed to one line and the full
narrative moves here, newest entry first. This file is history, not
authority: lifecycle state lives in deliverable `_STATUS.md` files, evidence
lives in `_run_records/**`, and rulings live in the decision register and
`SOFTWARE_DECOMP.md` decision log. Nothing here is a release, professional,
certification, or code-compliance claim.

---

## 2026-06-11 — A6 second sub-slice: results family selector (`TP-APP-R2-RESULTFAMILY-001`)

The results panel now exposes direct result-family filters with visible counts
for the solved preview envelope: displacement, reaction, force, moment, and
stress. Selecting a family resets pagination and constrains the existing
table groups, while the existing free-text filter still composes with the
selected family. The support-reaction rows and stress rows are therefore
selectable as dedicated table views without requiring text-search strings.

The fixture-backed app test asserts the family counts
(`15/9/180/180/263`), reaction-only pagination (`9 of 647`), stress-only
pagination (`263 of 647`), active `aria-pressed` state, and representative
reaction/stress rows. The Playwright R2 smoke covers the reaction family path
after solving. In-app browser smoke at `http://127.0.0.1:5175/` confirmed the
solved app showed `result_rows=647`, reaction/stress family selection,
visible non-overflowing selector buttons, all-family restoration, and zero
browser console errors.

Residuals remain in A6: true directional deformed shape once displacement
vectors exist, governing-ratio views once ratio rows exist in result
envelopes, and richer result-selection coupling.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-108 and
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-05_Results viewer/_run_records/WORKING_ITEMS_RUN_2026-06-11_results_family_selector.md`.

## 2026-06-11 — A5 second sub-slice: persisted edited-load solve regression (`TP-APP-R2-PERSISTEDSOLVE-001`)

The Tauri backend test suite now proves an edited load-data model can move
through the app's structured operation and local persistence seams before
being solved from the restored model payload. The regression applies an
explicit `update_load` operation to `load:L-100` primitive
`primitive_loads.1.magnitude.value`, persists the edited model into the local
SQLite project store, reloads it by project id, and solves the restored
payload through `solve_preview_mechanics`.

The test asserts the saved/reopened model retains the edited force magnitude,
the restored solve is bound to `project:edited-load-roundtrip`, mechanics
status is `MECHANICS_SOLVED`, and `result:disp:node-N-140` changes relative
to the original fixture solve. This is backend evidence for saved edited-load
data reaching the solve boundary; it does not claim that the later packaged
Tauri GUI smoke is complete.

Residuals remain in A5: full packaged-Tauri GUI smoke over a saved edited
project snapshot, UI polish for incomplete-model diagnostics, and broader
persisted solve coverage as new authoring surfaces grow.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-107;
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-11_persisted_edited_load_solve_regression.md`,
the same-named record under
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/`,
and the same-named record under
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_run_records/`.

## 2026-06-11 — A4 thirteenth sub-slice: combination creation editor (`TP-APP-R2-COMBCREATE-001`)

The Load Cases manager now exposes explicit creation for mechanics-basis load
combinations. The create form captures a new combination id, label, mechanics
basis, one existing load-case term, a finite dimensionless factor, provenance,
and rationale, then queues a structured `create_combination` operation with
`field_path=combinations`, `before=not_present`, unit `none`, and dimension
`dimensionless`.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`create_combination` payloads. Accepted intents must create a new
`Combination`, use basis `mechanics`, include at least one explicit term,
reference existing load cases, carry finite factors, avoid duplicate initial
operands, preserve structured-operation audit boundaries, and emit no
professional approval claim. Duplicate ids, missing load cases, empty terms,
invalid payloads, non-mechanics basis values, and invalid unit/dimension
metadata are blocked.

The app-level regression queues `op:load-manager-create-combination:C-300`,
applies it through `OperationApplyPanel`, verifies the manager summary changes
to two combinations, checks the new `combination:C-300` row with
`load:L-100 x 1`, selects the row into the property inspector, and confirms
zero pending operations with solve state reset to `not_started`. The
Playwright R2 smoke verifies the rendered create-combination preview without
applying so the solve/results/report path remains on the unchanged fixture
model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-create-combination:C-300` and confirmed two combinations,
`combination:C-300` with `basis=mechanics` and `load:L-100 x 1`, property
inspector selection, zero pending operations, `applied_operations=1`, solve
state `not_started`, and no browser console errors.

Residuals remain in A4: subtraction/range expression authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-106;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 twelfth sub-slice: combination term deletion editor (`TP-APP-R2-COMBTERMDELETE-001`)

The Load Cases manager now exposes explicit deletion for a selected existing
combination term. The delete control appears in the selected-term editor,
captures a rationale, and queues a structured `delete_combination_term`
operation with `field_path=terms.N`, `before=<load_case> x <factor>`,
`after=not_present`, unit `none`, and dimension `dimensionless`. The operation
removes one indexed term only; whole-term replacement, code/rule combinations,
and broader algebra authoring remain deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`delete_combination_term` payloads. Accepted intents must target an existing
`Combination`, address an existing term index, carry a before-value matching
the current term display, preserve structured-operation audit boundaries, and
emit no professional approval claim. Stale before-values, out-of-range
indices, invalid unit/dimension metadata, and missing combination `terms`
arrays are blocked.

The app-level regression selects `combination:C-OPER-ALT` term 1, queues
`op:load-manager-combination:C-OPER-ALT-term-1-delete`, applies it through
`OperationApplyPanel`, and verifies `load:L-200 x 0.5` is removed while
`load:L-100 x 1` remains. The Playwright R2 smoke verifies the rendered delete
preview without applying so the solve/results/report path remains on the
unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-combination:C-OPER-ALT-term-1-delete` and confirmed
`load:L-200 x 0.5` was no longer visible in the combination row, zero pending
operations, `applied_operations=1`, solve state `not_started`, and no browser
console errors.

Residuals remain in A4: broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-105;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_deletion_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 eleventh sub-slice: combination term creation editor (`TP-APP-R2-COMBTERMCREATE-001`)

The Load Cases manager now exposes explicit child-term creation for existing
load combinations. The create form selects an existing `Combination`, an
existing `LoadCase`, a finite dimensionless factor, and a rationale, then
queues a structured `create_combination_term` operation with
`field_path=terms`, `before=not_present`, unit `none`, and dimension
`dimensionless`. The operation appends one term only; whole-term replacement,
term deletion, code/rule combinations, and broader algebra authoring remain
deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`create_combination_term` payloads. Accepted intents must target an existing
`Combination`, reference an existing `LoadCase`, carry JSON
`{ "load_case": string, "factor": number }`, preserve structured-operation
audit boundaries, and emit no professional approval claim. Missing load cases,
invalid payloads, non-dimensionless metadata, and missing combination `terms`
arrays are blocked.

The app-level regression creates `load:L-300`, then appends it to
`combination:C-OPER-ALT` as `load:L-300 x 0.25` through
`OperationApplyPanel`, proving existing `load:L-100 x 1` and
`load:L-200 x 0.5` terms are preserved. The Playwright R2 smoke verifies the
rendered create-term preview without applying so the solve/results/report path
remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied the default
create-load-case intent, selected `load:L-300` in the combination-term form,
applied `op:load-manager-combination:C-OPER-ALT-term-2-create`, and confirmed
`load:L-300 x 1`, zero pending operations, `applied_operations=2`, solve
state `not_started`, and no browser console errors. The smoke avoided text
entry because the Browser plugin's virtual clipboard remained unavailable for
`fill`.

Residuals remain in A4: combination term deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke
over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-104;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 tenth sub-slice: combination basis editor (`TP-APP-R2-COMBBASIS-001`)

The Load Cases manager now exposes explicit basis editing for existing load
combinations. The selected-combination editor captures a replacement
`Combination.basis` text value and rationale, then queues a structured
`update_load` operation with `field_path=basis`, unit `none`, and dimension
`dimensionless`. The operation is limited to an existing combination record;
whole-term replacement, term creation/deletion, code/rule combinations, and
broader algebra authoring remain deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now treat
`Combination.basis` as an editable text field while retaining the explicit
deferred finding for whole `Combination.terms` edits. Accepted basis edits
must target an existing `Combination`, carry a current before-value, provide a
non-empty replacement value, and route through structured operations only. The
tests prove the edit changes only `basis` and preserves `terms` and
`provenance`.

The app-level test selects `combination:C-OPER-ALT`, changes basis from
`mechanics` to `mechanics_user_review`, queues
`op:load-manager-combination:C-OPER-ALT-basis`, applies it through
`OperationApplyPanel`, verifies the manager row and property inspector, and
confirms zero pending operations with solve state reset to `not_started`. The
Playwright R2 smoke checks the rendered basis preview without applying so the
solve/results/report path remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` keyed
`mechanics_user_review` into the basis field, applied
`op:load-manager-combination:C-OPER-ALT-basis`, and confirmed
`basis=mechanics_user_review`, zero pending operations,
`applied_operations=1`, and solve state `not_started`. The smoke used
single-key presses because the Browser plugin's virtual clipboard remained
unavailable for `fill`/`type`.

Residuals remain in A4: combination term creation/deletion, broader algebra
authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-103;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_basis_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 ninth sub-slice: imposed-displacement primitive-load creation editor (`TP-APP-R2-IMPOSED-001`)

The Load Cases manager now exposes explicit imposed-displacement primitive
load creation for existing load cases. The create form selects
`imposed_displacement`, captures the target load case, primitive-load id,
existing support target, support DOF, magnitude, and provenance, then queues a
structured `create_primitive_load` operation. Translational DOFs
`UX|UY|UZ` use the project length unit and dimension `displacement`.
Rotational DOFs `RX|RY|RZ` use the project angle unit and dimension
`rotation`. The invented preview model now records `project.units.angle =
"rad"` so rotational imposed-displacement previews and validation consume
explicit project metadata.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`imposed_displacement` primitive-load creation alongside the previously
landed concentrated-force, distributed-force, concentrated-moment, pressure,
and thermal paths. Accepted imposed-displacement intents must target an
existing `Load` with `field_path=primitive_loads`, `before=not_present`, a
globally unique primitive-load id, category `imposed_displacement`, target
`{ type: "support", support: <existing support id>, dof: <matching DOF> }`,
finite magnitude in the expected project unit, matching dimension, and
non-empty provenance. Duplicate primitive IDs, missing support targets,
invalid DOFs, target/DOF mismatches, and missing project unit metadata are
blocked.

The app-level test creates `load:L-100-I300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new support-targeted row
`load:L-100-I300; support:support:S-100; UZ; dimension=displacement`,
confirms the local receipt, and confirms stale solve state is reset. The
operation-service and Rust tests also apply a rotational `RX` payload with
dimension `rotation` and unit `rad`. The Playwright R2 smoke checks both the
translational preview and the rotational `RX` unit/dimension preview without
queueing so the solve/results/report path remains on the unchanged fixture
model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-I300-primitive` from a clean session
using the default finite magnitude `250 m`. The smoke confirmed `2 load
cases; 8 primitive loads; 1 combinations`, manager row
`load:L-100-I300; support:support:S-100; UZ; dimension=displacement`, zero
pending operations, `persistence=session_state_only_not_yet_saved`,
`professional_approval=false`, and solve state `not_started`. The browser
plugin could not type a replacement magnitude because its virtual clipboard
was unavailable; the negative translational magnitude path is covered by
Vitest.

Residuals remain in A4: combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-102;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_imposed_displacement_load_creation_editor.md`,
the same-named record under
`DEL-04-03_Linear support and restraint models/_run_records/`, the
same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 eighth sub-slice: pressure and thermal primitive-load creation editor (`TP-APP-R2-PRESSTEMP-001`)

The Load Cases manager now exposes explicit pressure and thermal primitive
load creation for existing load cases. The create form selects `pressure` or
`thermal`, captures the target load case, primitive-load id, existing pipe
target, global direction, magnitude, and provenance, then queues a structured
`create_primitive_load` operation. Pressure payloads are limited to target
`{ type: "element", pipe: <existing pipe id> }`, dimension `pressure`, and
the project pressure unit (`Pa` for the invented preview model). Thermal
payloads use the same element target contract with dimension
`temperature_interval` and the project temperature interval unit (`degC` for
the invented preview model). No gauge/absolute pressure conversion,
reference-pressure default, thermal absolute-temperature conversion, imposed
displacement, hidden default, unit conversion, or code-specific combination is
inferred.

The invented preview model now records `project.units.pressure = "Pa"` so
pressure creation consumes explicit project unit metadata rather than an
implicit pressure fallback. This remains a single-unit technical-preview
posture: D-01 has accepted future unit semantics, but Phase B unit conversion,
quantity-kind UI, reference-pressure handling, and unit picker/display
retirement remain outside this tranche.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`pressure` and `thermal` primitive-load creation alongside the previously
landed concentrated-force, distributed-force, and concentrated-moment paths.
Accepted pressure/thermal intents must target an existing `Load` with
`field_path=primitive_loads`, `before=not_present`, the corresponding project
unit metadata, matching dimension, a globally unique primitive-load id,
category `pressure` or `thermal`, an existing pipe target, global direction,
finite magnitude in the expected unit, and non-empty provenance. Duplicate
primitive IDs and missing pipe targets are blocked.

The app-level tests create `load:L-100-P300` and `load:L-100-T300` through
the manager, apply each through `OperationApplyPanel`, verify the manager
summary and `load:L-100` primitive count, check the new element-targeted rows,
confirm the local receipt, and confirm stale solve state is reset. The
Playwright R2 smoke checks both rendered create previews without queueing so
the solve/results/report path remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-P300-primitive` and
`op:load-manager-load:L-100-load:L-100-T300-primitive` in clean sessions. The
smoke confirmed `2 load cases; 8 primitive loads; 1 combinations`, pressure
row `load:L-100-P300; element:pipe:P-100; global_x; dimension=pressure`,
thermal row `load:L-100-T300; element:pipe:P-100; global_z;
dimension=temperature_interval`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: imposed-displacement authoring breadth, combination
basis editing, combination term creation/deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke
over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-101;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 seventh sub-slice: concentrated moment primitive-load creation editor (`TP-APP-R2-MOMENTCREATE-001`)

The Load Cases manager now exposes explicit concentrated nodal-moment
primitive load creation for existing load cases. The create form selects
`concentrated_moment`, captures the target load case, primitive-load id,
existing node target, rotational direction, magnitude, and provenance, then
queues a structured `create_primitive_load` operation. The payload is limited
to target `{ type: "node", node: <existing node id> }`, direction
`rotation_x|rotation_y|rotation_z`, dimension `moment`, and the project
force*length unit basis (`N*m` for the invented preview model). No
pressure/temperature primitives, imposed displacements, hidden defaults, unit
conversion, or code-specific combinations are inferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`concentrated_moment` primitive-load creation alongside the previously landed
`concentrated_force` and `distributed_force` paths. Accepted concentrated
moment intents must target an existing `Load` with `field_path=primitive_loads`,
`before=not_present`, project force and length unit metadata, dimension
`moment`, a globally unique primitive-load id, category
`concentrated_moment`, an existing node target, rotational direction, finite
magnitude in the expected unit, and non-empty provenance. Duplicate primitive
IDs and missing node targets are blocked.

The app-level test creates `load:L-100-M300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-M300; node:node:N-100; rotation_z; dimension=moment`, confirms
the local receipt, and confirms stale solve state is reset. The Playwright R2
smoke checks the rendered concentrated-moment create preview without queueing
so the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-M300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the concentrated-moment manager
row with `250 N*m`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: pressure/temperature primitive creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-100;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_moment_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 sixth sub-slice: distributed primitive-load creation editor (`TP-APP-R2-DISTLOAD-001`)

The Load Cases manager now exposes explicit distributed element-force
primitive load creation for existing load cases. The create form selects
`distributed_force`, captures the target load case, primitive-load id, existing
pipe target, global direction, magnitude, and provenance, then queues a
structured `create_primitive_load` operation. The payload is limited to target
`{ type: "element", pipe: <existing pipe id> }`, dimension
`force_per_length`, and the project force/length unit basis (`N/m` for the
invented preview model). No concentrated moments, pressure or temperature
primitives, imposed displacements, hidden defaults, unit conversion, or
code-specific combinations are inferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`distributed_force` primitive-load creation alongside the previously landed
`concentrated_force` path. Accepted distributed intents must target an
existing `Load` with `field_path=primitive_loads`, `before=not_present`,
project force and length unit metadata, dimension `force_per_length`, a
globally unique primitive-load id, category `distributed_force`, an existing
pipe target, direction `global_x|global_y|global_z`, finite magnitude in the
expected unit, and non-empty provenance. Duplicate primitive IDs and missing
pipe targets are blocked.

The app-level test creates `load:L-100-D300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-D300; element:pipe:P-100; global_y; dimension=force_per_length`,
confirms the local receipt, and confirms stale solve state is reset. The
Playwright R2 smoke checks the rendered distributed create preview without
queueing so the solve/results/report path remains on the unchanged fixture
model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-D300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the distributed-force manager row
with `250 N/m`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: concentrated moments, pressure/temperature primitive
creation, imposed-displacement authoring breadth, combination basis editing,
combination term creation/deletion, broader algebra authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-99;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_distributed_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 fifth sub-slice: concentrated primitive-load creation editor (`TP-APP-R2-PRIMCREATE-001`)

The Load Cases manager now exposes explicit concentrated nodal-force primitive
load creation for existing load cases. The form captures target load case,
primitive-load id, existing node target, global direction, magnitude, and
provenance, then queues a structured `create_primitive_load` operation. The
payload is limited to category `concentrated_force`, dimension `force`, and the
project force unit; no distributed loads, concentrated moments, pressure or
temperature primitives, imposed displacements, hidden defaults, unit
conversion, or code-specific combinations are inferred.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply `create_primitive_load`
intents. Accepted intents must target an existing `Load` with
`field_path=primitive_loads`, `before=not_present`, project force unit,
dimension `force`, a globally unique primitive-load id, category
`concentrated_force`, an existing node target, direction
`global_x|global_y|global_z`, finite magnitude, and non-empty provenance.
Duplicate primitive IDs and missing node targets are blocked.

The app-level test creates `load:L-100-F300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-F300; node:node:N-100; global_y; dimension=force`, confirms the
local receipt, and confirms stale solve state is reset. The Playwright R2 smoke
checks the rendered create preview without queueing so the solve/results/report
path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-F300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the concentrated-force manager row
with `250 N`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: distributed primitive-load creation, concentrated
moments, pressure/temperature primitive creation, imposed-displacement
authoring breadth, combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-98;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_primitive_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 fourth sub-slice: empty load-case creation editor (`TP-APP-R2-LOADCREATE-001`)

The Load Cases manager now exposes explicit empty load-case creation for the
invented preview model. The create form captures a user-supplied load-case id,
label, kind, status, and provenance, previews the structured create operation,
and queues only a load-case shell with `primitive_loads=0`. It does not create
primitive loads, imposed displacements, code combinations, hidden defaults, or
derived engineering values.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply `create_load_case`
intents. Accepted intents must target `Load` with `field_path=load_cases`,
`before=not_present`, unit `none`, dimension `dimensionless`, a non-duplicate
load-case id, matching JSON payload id, non-empty label/kind/status/
provenance, and absent or empty `primitive_loads`. Non-empty primitive payloads
are blocked so future primitive-load creation remains an explicit A4 tranche.

The app-level test creates `load:L-300` through the manager, applies it through
`OperationApplyPanel`, verifies the manager summary and row
`load:L-300; primitive_user_load; draft; primitives=0`, checks the property
inspector, confirms the local receipt, and confirms stale solve state is reset.
The Playwright R2 smoke checks the rendered create preview without queueing so
the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-create-load:L-300` and confirmed `3 load cases; 7 primitive
loads; 1 combinations`, the empty-shell manager row, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: arbitrary primitive-load creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-97;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 third sub-slice: combination term-factor editor (`TP-APP-R2-COMBFACTOR-001`)

The Load Cases manager now exposes existing combination term-factor editing
for the invented mechanics preview combination. The editor selects an
existing `Combination.terms.N.factor` row, queues a structured `update_load`
intent with explicit before/after scalar values, unit `none`, dimension
`dimensionless`, local-session audit boundaries, and no professional approval
claim. Whole `Combination.terms` replacement, combination `basis` editing,
term creation/deletion, code/rule combinations, and broader algebra authoring
remain explicitly out of scope.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply the dynamic
`terms.N.factor` path as a dimensionless numeric field. The app-level test
applies `combination:C-OPER-ALT` term 1 from `0.5` to `0.75` through
`OperationApplyPanel`, verifies the manager row, property inspector,
pending-operation count, applied-operation count, and stale-solve reset. The
Rust test proves `terms.1.factor` applies without mutating the input model and
that whole `terms` editing remains an explicit deferred finding. The
Playwright R2 smoke previews the rendered factor editor without queueing so
the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-combination:C-OPER-ALT-term-1-factor` and confirmed
`load:L-200 x 0.75`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: load-case creation, arbitrary primitive-load
creation, imposed-displacement authoring breadth, combination basis editing,
combination term creation/deletion, broader algebra authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-96;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_factor_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 second sub-slice: load-case metadata editor (`TP-APP-R2-LOADMETA-001`)

The Load Cases manager now exposes selected load-case metadata editing for
`status` and `kind`. The editor queues structured `update_load` intents for
`Load.status` or `Load.kind` with explicit before/after values, unit `none`,
dimension `dimensionless`, local-session audit boundaries, and no
professional approval claim. The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
those two load metadata fields; combination `basis`/`terms` editing remains
explicitly deferred.

The app-level test applies `load:L-100` status from `preview_only` to `TBD`
through `OperationApplyPanel`, verifies the manager row, property inspector,
pending-operation count, applied-operation count, and stale-solve reset, then
verifies the `kind` editor previews `primitive_user_load -> TBD` without
applying a second operation. The Playwright R2 smoke checks the rendered
status/kind metadata controls without queueing so the solve/results/report
path remains on the unchanged fixture model. An in-app browser smoke at
`http://127.0.0.1:5175/` applied the rendered status edit and confirmed the
row showed `status=TBD`, pending operations returned to zero, and solve state
remained `not_started`.

Residuals remain in A4: load-case creation, arbitrary primitive-load creation,
imposed-displacement authoring breadth, full combination editing/algebra
authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-95;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_metadata_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 seventh sub-slice: viewport pipe endpoint picking (`TP-APP-R2-PIPEPICK-001`)

The viewport pipe form now has explicit endpoint-pick controls. Arming
`Pick` for the `from` endpoint and selecting a rendered viewport node fills
the pipe form's `from` field, advances to `to` picking, and still updates the
normal viewport/model-tree selection. Selecting a second rendered node fills
the `to` field and clears pick mode. Picking only supplies node references;
material, section geometry, non-zero `y_reference`, and provenance remain
explicit user-entered fields before Queue pipe can be enabled.

The app-level test picks `node:N-100` and `node:N-140` from viewport targets,
fills the remaining explicit pipe fields, queues and applies
`pipe:P-151` through `OperationApplyPanel`, and verifies the created pipe is
selected in the model tree, viewport layer, and inspector. The Playwright R2
smoke covers the rendered endpoint-pick controls before the unchanged
solve/results/report flow. An in-app browser smoke at
`http://127.0.0.1:5175/` confirmed `from=node:N-100`,
`to=node:N-140`, pick-mode advancement/clearing, and Queue pipe remaining
disabled until the remaining explicit fields are supplied.

Residuals remain in A3: canvas gesture capture beyond node drafting and
endpoint picking, component/rigid authoring, and broader editor coverage as
new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-94;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_pipe_endpoint_picking.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and the
same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 sixth sub-slice: canvas node drafting (`TP-APP-R2-CANVASNODE-001`)

The viewport canvas now captures a primary pointer gesture into the explicit
node form before any operation is queued. In WebGL mode, the handler raycasts
from the Three.js camera to the `y=0` drafting plane; in fallback/test mode,
it maps the pointer into the same bounded model-drafting plane. The captured
draft fills a visible editable node id (`node:V-001` style), label, and finite
x/y/z coordinates. The existing Queue node and Apply Operations path remains
the only mutation path.

The app-level test applies a canvas-drafted node through the structured
operation seam and verifies model-tree/inspector selection. The Playwright R2
smoke now exercises the real browser canvas click path without queueing the
draft, so the solve/report smoke remains on the unchanged fixture model. An
in-app browser smoke at `http://127.0.0.1:5175/` confirmed a canvas click
drafted `node:V-001`, finite coordinates, `y=0`, and enabled Queue node.

Residuals remain in A3: canvas gesture capture beyond node drafting
(pipe/connectivity and component/rigid authoring), rigid/component authoring,
and broader editor coverage as new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-93;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_canvas_node_drafting.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and the
same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 fifth sub-slice: explicit straight-pipe creation (`TP-APP-R2-CONNECTPIPE-001`)

The viewport editor now has an explicit straight-pipe form for user-entered
pipe id, label, endpoint nodes, material, outside diameter, wall thickness,
non-zero local `y_reference`, and provenance. The form queues a structured
`connect_pipe_run` intent with `field_path=pipe_segments` and applies it
through the same operation review/acceptance path as other local-session
edits. Legacy one-click pipe-run viewport gestures remain blocked because
they still carry only underspecified `viewport.connect_pipe_run` data.

The Rust `core/model_operations/operation_applier` crate and browser local
operation mirror both validate the explicit pipe payload: matching id,
`before=not_present`, project length unit, `dimension=length`, non-duplicate
pipe id, existing endpoint nodes/material, positive OD/wall quantities, and
non-zero `y_reference`. Applying the intent appends a new `pipe_segments`
record to the returned session model without mutating the input model in
place. The app test confirms `pipe:P-150` is created, selected in the model
tree and viewport selection layer, visible in the property inspector, and
recorded with local-session acceptance only.

Residuals remain in A3: true canvas raycast/gesture geometry capture,
rigid/component authoring, and broader editor coverage as new authoring
surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-92;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_explicit_straight_pipe_connectivity.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 first sub-slice: load-case primitive magnitude manager (`TP-APP-R2-LOADMGR-001`)

The desktop app now has a right-rail Load Cases manager over the invented
preview model. It surfaces load-case counts, primitive-load rows, combination
terms, and a focused primitive-load magnitude editor. Selecting
`load:L-100-P` exposes `primitive_loads.2.magnitude.value`; changing the
magnitude queues `op:load-manager-load:L-100-load:L-100-P-magnitude` as a
structured `update_load` intent. Applying the queued operation uses the
existing OperationApplyPanel, records local-session acceptance, clears stale
solve results, and leaves persistence to the Save local path.

Residuals remain in A4: load-case creation, load status/kind editing,
arbitrary primitive-load creation, imposed-displacement authoring breadth,
full combination editing/algebra authoring, unit picker/display retirement
after Phase B, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-91;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_manager_primitive_magnitude.md`,
the same-named record under
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/`,
and the same-named record under
`DEL-05-02_Load-case algebra engine/_run_records/`.

## 2026-06-11 — A8 first sub-slice: Playwright R2 smoke harness (`TP-APP-R2-PLAYWRIGHT-001`)

The desktop workspace now has a Playwright harness and root script
`npm run test:e2e:desktop`. The first smoke test runs the technical-preview
fixture through initial shell checks, local-only boundary checks, a nonblank
and animated Three.js viewport assertion, mechanics preview solve,
`result_rows=647`, viewport displacement-overlay availability, result
filtering/detail inspection for `pipe:P-120`, and deterministic report-packet
export checks.

Vitest is scoped to `src/**/*.test.{ts,tsx}` so the unit suite and Playwright
suite remain separate. Playwright local output directories are ignored.

Residuals remain in A8: authored create/edit -> solve -> report automation,
full manual SMOKE checklist parity, packaged Tauri saved-project solve smoke,
and CI browser provisioning policy.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-90;
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-11_playwright_r2_smoke_harness.md`.

## 2026-06-11 — A6 first sub-slice: viewport displacement overlay (`TP-APP-R2-DEFORMEDVIEW-001`)

The Three.js viewport now consumes the current mechanics result and renders a
review-only shape overlay after a solved preview run. The first slice uses
available `displacement_magnitude` rows by node, draws a teal overlaid
centerline/marker set, and exposes a toolbar status with node count, maximum
reported displacement magnitude, and an explicit boundary:
`scale=normalized_display_offset_not_physical_length`,
`vector_direction=TBD`, and `professional_claim=false`.

When no result is present the overlay status is `not started`; when the
current mechanics result is incomplete, such as the browser edited-model guard,
the overlay status is `blocked` and no deformed overlay is rendered.

Residuals remain in A6: true directional deformed shape once displacement
vectors exist, support-reaction visualization, stress/governing-ratio views,
and richer result-selection coupling.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-89;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_deformation_overlay.md`
and the same-named record under
`DEL-07-05_Results viewer/_run_records/`.

## 2026-06-11 — A5 first sub-slice: model-bound solve guard (`TP-APP-R2-SOLVEBOUND-001`)

The preview mechanics service now refuses to reuse bundled solved-result rows
for an edited model in browser fixture mode. Edited browser-session models
return a `MODEL_INCOMPLETE` mechanics result with zero result rows and an
explicit `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL` blocking
diagnostic. The unchanged fixture model still returns the bundled solved
fixture for browser preview workflows.

The Tauri backend path was verified as model-bound: direct
`run_preview_mechanics(Some(model))` and the solve-job registry both solve the
supplied edited model payload and publish result envelopes bound to the edited
`project.id`, not the bundled fixture id.

Residuals remain in the A5 row: full packaged-Tauri GUI smoke over a saved
edited project snapshot, richer incomplete-model UI copy, and broader
persisted non-fixture solve coverage as authoring surfaces grow.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-88;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-11_edited_model_solve_binding.md`,
the same-named record under
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_run_records/`,
and the same-named record under
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/_run_records/`.

## 2026-06-10 — A3 fourth sub-slice: session undo/redo checkpoints (`TP-APP-R2-UNDOREDO-001`)

The Apply Operations panel now exposes local-session Undo/Redo controls for
applied structured operations. Applying an operation records a checkpoint for
the previous session model and selection; undo restores that checkpoint,
moves the current model to redo, and clears stale solve results; redo restores
the undone model and clears stale solve results again. The history summary is
explicitly labeled local-session-only and saved-project-mutated=false.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, and broader editor coverage as new
authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-87;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_session_undo_redo_checkpoints.md`
and the same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 third sub-slice: explicit node create operation (`TP-APP-R2-CREATENODE-001`)

The viewport editor now has an explicit node-geometry form for user-entered
node id, label, and finite x/y/z coordinates in the project length unit. The
form queues a structured `create_node` intent and applies it through the
existing operation seam; the browser local engine and Rust applier accept only
explicit node payloads, reject duplicate ids, and preserve the no-silent
conversion/default posture. The applied target becomes the active model-tree
and inspector selection. During browser smoke this tranche also fixed
viewport/operation-panel hit-test layout issues so the new form and apply
buttons are actionable in the live app.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, undo/redo, and broader editor coverage
as new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-86;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_explicit_node_create_operation.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 second sub-slice: property-inspector inline validation (`TP-APP-R2-INLINEVALID-001`)

The property inspector now exposes validate-only feedback for draft editor
intents before queue/apply. The UI calls the existing structured-operation
validation seam and displays application status, schema/unit/before-state
states, diff rows, diagnostics, and a no-mutation/professional-boundary note.
This landed as an A3 editor UX sub-slice; broader editor coverage, undo/redo,
and true geometry-capture workflows remain in the A3 row.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-85;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`
and
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`.

## 2026-06-10 — A3 first sub-slice: viewport selection binding (`TP-APP-R2-VIEWSELECT-001`)

Viewport entity selection controls for loaded nodes, straight pipes,
supports, and component markers now drive shared selection, model-tree
active state, property-inspector binding, and viewport active highlight.
Residuals (remain in the A3 row): true canvas raycast/gesture geometry
capture, node/straight-pipe creation tools with explicit
coordinates/connectivity, undo/redo, and inline validation messages.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-84;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_viewport_selection_binding.md`
and the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`.

## 2026-06-10 — A2 landed: model-document persistence (`TP-APP-R2-PERSIST-001`)

Implemented under `DEC-019` (D-08 ruling): in-document semver authority,
application-service transform chain (migrate-in-memory-on-open /
persist-on-save), refusal semantics for newer/unsupported documents, store
v9 evidence-only migration ledger with pre/post hashes, and
validation-preflight evidence replacing the prior TBD marker. Open residuals
(remain in the A2 row): compatibility-window size (human ruling), explicit
"Migrate project" operation, sibling JSON-slot coverage.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-83;
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-10_model_document_schema_migration.md`
(with same-day `TASK_RUN_2026-06-10_*.md` records in the same folder).

## 2026-06-10 — A1 landed: apply-operation command path (`TP-APP-R2-EDITLOOP-001`)

New `core/model_operations/operation_applier` crate plus the
`apply_model_operation` and `validate_model_operation` Tauri commands
(desktop bridge now 14 commands, two on the mutating path) and an Apply
Operations panel. Inspector modify intents apply to the session model;
viewport gesture intents block pending A3 geometry capture; unit conversion
blocks pending D-01/Phase B.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-82;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_apply_operation_command_path.md`.

## 2026-06-10 — Decision packets D-01 and D-08 prepared and ruled

Both packets drafted and ruled same day: `D-01` → `DEC-018` (SI-canonical
with dual display catalog, as the packet proposed; Phase B unblocked);
`D-08` → `DEC-019` (per-document semver transform chain,
migrate-in-memory-on-open / persist-on-save, as the packet proposed; A2
unblocked). `D-10` packet drafted the same day, `AWAITING_RULING`. Packets
and state: `execution/_Coordination/_DECISIONS/_REGISTER.md`; rulings:
`SOFTWARE_DECOMP.md` §12. This completed items 1–2 of the plan's original
"first three tranches" sequence; item 3 (A2) landed the same day, above.
