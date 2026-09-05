# B2 rich forms — bounded implementation return

Status: implementation complete and source frozen for independent review; package integration/real-engine application verification remain parent work. Agent 2 ephemeral generalist under N1_WI_PKG07, delegated-harness-native with instruction+config asserted non-delegation. Inherited model identifier not exposed. No children created.

## Basis and scope

Sealed LAUNCH_BRIEF.md; root/project AGENTS, AGENT_TASK base guidance; software-bounded-implementation version 1 and all three companion files found/read; software-workflow.json; DEL-07-02/03 ScopeOfWork, docs/CONTRACT.md; Phase A accepted palette snapshot; N2_WI_PKG16/B2_ACCEPTED_SNAPSHOT.json exact rich field wire. TASK was read as ephemeral base guidance, not a managed TASK-shell role requiring an environment override. Applicable invariants: DATA-1/2/3, UNIT-1, AGENT-1..4, AUTH-1, PRIV-1, IP-1..3. No engineering catalogs/equations/default constants added.

Source freeze: SOURCE_MANIFEST.json hashes all five module/test files, with accepted upstream snapshots and sealed brief hash. This is derivative run evidence, not decomposition truth. No lifecycle, release, physics acceptance, receipts, registry or governance state changed.

## Outputs

- `apps/desktop/src/features/support-configuration/SupportConfigurationForm.tsx`: explicit creation and full configuration replacement for existing support; named family selection, DOF restraints, top-level and hanger stiffness, optional hanger quantities/references, nonlinear behavior/state/contact direction, gap/friction/reaction quantities, and derived normal source restricted to eligible existing linear support and restrained translational DOF. Optional blocks can be removed explicitly; current aliases are preserved. Creating requires explicit unique ID, label, node and provenance. Queueing does not imply solver readiness.
- `apps/desktop/src/features/material-temperature/MaterialTemperatureForm.tsx`: add/remove stable UI rows with authored point IDs, optional temperature/E/G/alpha quantities and provenance. Blank fields remain absent; partial unselected records are allowed for engine review. Existing-array canonical before or not_present; whole-array replacement.
- `apps/desktop/src/features/wind-exposure/WindExposureForm.tsx`: whole-pipe checkboxes and partial-span rows with pipe choice and explicit fraction values/units; atomic whole/partial replacement; overlap/mixed-whole/partial and fraction-bound checks; exact existing before projection.
- `apps/desktop/src/features/rich-authoring/formSupport.tsx`: structured controls, explicit dimensional cues, blank/finite quantity checks, standard intent envelope, Rust-backed canonical before values, immutable capture across await, selection/model/queue change guards, busy/error feedback. Quantity parsing preserves unknown fields for authoritative engine rejection instead of silently dropping them.
- `apps/desktop/src/features/rich-authoring/richForms.test.tsx`: 10 focused DOM tests. All fixture data synthetic/original.

## Integration contract

Each exported form accepts `{model, selection, queuedIntents?, onQueueIntent, operationBusy?}` exactly as frozen. No direct durable model writes.

| Form | Selection precondition | Focus anchor |
|---|---|---|
| SupportConfigurationForm | project/node for create; existing support for edit and optional create-new mode | `#rich-support-form` |
| MaterialTemperatureForm | existing material | `#temperature-table` |
| WindExposureForm | existing load with existing equivalent_static.wind object | `#wind-exposure-form` |

Anchors are focusable sections with tabIndex=-1. Forms return null outside their selection scope. The existing wind configuration controls must create wind before the exposure form appears. Source modules are frozen; B0 owns App/inspector integration and all shared files.

Shared type additions requested early and supplied by B0: materials.temperature_points optional exact array, supports.nonlinear optional exact shape, and wind.exposed_spans optional exact array. No further shared type change is required.

Wire: create_support/field supports/before not_present; update_support/field configuration canonical projection of family/restraints/stiffness/hanger/nonlinear/provenance; set_field/Material.temperature_points; update_load/Load.equivalent_static.wind.exposure. Rich envelope unit none, dimension dimensionless; nested quantities preserve explicit entered unit. Missing optional after keys intentionally clear their owned support members. No type or schema tokens invented.

## Verification

- CHECKS.json: desktop-test PASS, 31 files/590 tests including all 10 new rich-form tests; initial build caught synthetic fixture missing diagnostics. Fixture fixed to actual typed shape.
- CHECKS_V2.json: desktop-build PASS. Desktop-test 589/590 PASS, including all 10 rich-form tests; sole failure is concurrent B0-owned App.deadControls test: enabled `cancel-pipe-draft` produced no observable DOM change. Routed to parent; not modified outside scope.
- ANCHOR_BUILD.json: final build PASS after adding requested focusable anchors. Anchor addition only changes IDs/tabIndex; final behavior test rerun belongs integration fan-in.
- SCOPE_CHECK.json: source write-scope PASS for all five files. Registered checks invoked via tools/software_workflow/run_registered_checks.py; scope via validate_change_scope.py. File edits used local Python/TypeScript AST formatting; no install, network, dependencies, Wasm rebuild, Git or shared source edits.

Tests cover exact nested quantities/intent routing, missing optional fields, removal and exact before projection, invalid half-entered/duplicate data, selection change during canonicalization, atomic wind exposure and overlap rejection, source reaction selection, collisions/pending-change guard, and no model mutation.

## Handoff and remaining work

Bounded implementation verdict: READY_FOR_INDEPENDENT_REVIEW. Accepted upstream snapshots remain those named above. Required parent reruns: fresh read-only review over source hashes, B0 integration and toolkit focus routes, remedy/recheck cancel-pipe-draft audit, shared Wasm rebuild and real-engine validation/preview/apply tests, final frozen-diff checks. No broad UI/toolkit, semantic parity, solver or lifecycle closure claimed. Unknown units and incomplete engineering configurations are sent for authoritative engine validation; this UI does not supply defaults or duplicate the unit conversion engine.
