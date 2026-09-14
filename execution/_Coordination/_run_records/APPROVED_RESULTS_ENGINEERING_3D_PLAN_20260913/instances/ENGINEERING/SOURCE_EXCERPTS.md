# Bounded live deliverable context

## execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance

### _STATUS.md
## Remaining


### _CONTEXT.md
# Context: DEL-03-01

**Deliverable ID:** DEL-03-01
**Name:** Material library schema with provenance
**Package ID:** PKG-03
**Package Name:** Piping Components, Materials, and Library Data Model
**Type:** DATA_MODEL_CHANGE

## Description
Define temperature-dependent material property records, source notes, redistribution status, and completeness flags.

## Anticipated Artifacts
- schemas/material.schema.yaml
- material editor fixtures

## Scope Coverage
- SOW-017

## Objective Support
- OBJ-004

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Schema plus tests; no proprietary data.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Schema plus tests; no proprietary data.

## Package Reference
- **Package:** PKG-03 Piping Components, Materials, and Library Data Model
- **Package Scope:** Defines material/component/section/library models and public/private data governance at the data-object level.
- **Package Assigned Scope Items:** SOW-007, SOW-008, SOW-009, SOW-010, SOW-017, SOW-018, SOW-019, SOW-044, SOW-051
- **Package Exclusions:** Does not implement the rule evaluator or global solver.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-03-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-017
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-03-01


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
- Remaining TBDs from the sealed dispatch stay open: public material source
- Remaining `TBD` items include public material source catalog, public fixture value policy, temperature interpolation policy, allowable storage policy, dependency satisfaction, protected-content/redistribution review disposition, and human review dispositions.
- Remaining local review findings: `PKG03-DEL-03-01-PKG02-001` and `PKG03-DEL-03-01-PKG02-002` both remain `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`; no `Review_Findings.csv` edits were made.
- Remaining gates: `HumanDisposition` remains `TBD`; lifecycle remains `IN_PROGRESS`; dependency satisfaction, protected-content/redistribution review, public material source catalog, public fixture value policy, temperature interpolation policy, and allowable storage policy remain `TBD`.

## execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model

### _STATUS.md
## Remaining


### _CONTEXT.md
# Context: DEL-03-06

**Deliverable ID:** DEL-03-06
**Name:** Expansion joint component model
**Package ID:** PKG-03
**Package Name:** Piping Components, Materials, and Library Data Model
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement manufacturer-data-driven expansion joint fields: stiffnesses, effective area, limits, hardware flags.

## Anticipated Artifacts
- expansion joint model
- validation tests

## Scope Coverage
- SOW-010

## Objective Support
- OBJ-004

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Specialized but bounded component type.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Specialized but bounded component type.

## Package Reference
- **Package:** PKG-03 Piping Components, Materials, and Library Data Model
- **Package Scope:** Defines material/component/section/library models and public/private data governance at the data-object level.
- **Package Assigned Scope Items:** SOW-007, SOW-008, SOW-009, SOW-010, SOW-017, SOW-018, SOW-019, SOW-044, SOW-051
- **Package Exclusions:** Does not implement the rule evaluator or global solver.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-03-06
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-010
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-03-06


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
  `DEL-03-06 - Expansion joint component model`.
- Implemented evidence: commit `f15cbc6` (`schema: add expansion joint
- Local audit artifacts are `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/_REVIEW.md` and `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/Review_Findings.csv`.
  `component:C-150` now carries explicit pipe mapping, effective pressure area,
  movement limit, hardware/manufacturer references, pressure-thrust reference,
- Product-physics preview validation now normalizes and validates
  data, pressure-thrust load generation, global nonlinear solve, lifecycle
## 2026-06-22 - TP-R4-D4-EJTHRUST-001 expansion-joint pressure-thrust load generation
- R4/D4 pressure-thrust follow-on landed for the invented preview path under
  `DEC-045`: pressure-thrust load evidence for `component:C-150` now uses the
  pressure load-case inputs for `L-100` and `L-200`.
- Product-physics preview output now records component pressure-thrust result
  load-case references, and `EXPANSION_JOINT_PRESSURE_THRUST_APPLIED`
- The report export packet now preserves the selected pressure-thrust result
  refs and component pressure-thrust evidence for the invented preview report.
  standards content, code-derived pressure-thrust coefficient, proprietary

## execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module

### _STATUS.md
## Remaining

- The 2026-09-08 four-case pressure-reference investigation is complete as candidate evidence; read its frozen report with `SIGN_CONVENTION_CLARIFICATION_V2.md`. RP technical PASS validates that packet only. Production pressure-physics and public-schema adoption remain open to Owner rulings on Poisson/material authority, closure topology/defaults, typed force/result compatibility, curved and expansion-joint treatment, thresholds, and verification fixtures. No lifecycle, dependency, or acceptance change is implied.
- The 2026-09-09 endpoint section-cut, curved-frame metadata, and genuine-pressure eligibility repair passed focused and full `product_physics` validation plus the root-managed combined source-and-fixture review. The bounded repair has zero schema errors on affected endpoint and curved-station rows. The complete public envelope still has 196 unchanged baseline contract errors, so versioned public-result-contract work remains open.
- Fresh native validation and integration remain pending. Broader pressure behavior, connector treatment, public result-contract work, DEC-025, and CHANGE-owned Git closeout remain open. No lifecycle or DAG promotion is recorded.


### _CONTEXT.md
# Context: DEL-05-03

**Deliverable ID:** DEL-05-03
**Name:** Fundamental stress recovery module
**Package ID:** PKG-05
**Package Name:** Loads, Load Cases, and Stress Recovery
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement mechanics stress recovery from axial force, bending, torsion, pressure, and section properties.

## Anticipated Artifacts
- stress recovery module
- hand-calc tests

## Scope Coverage
- SOW-015

## Objective Support
- OBJ-003

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Does not encode code stress equations.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Does not encode code stress equations.

## Package Reference
- **Package:** PKG-05 Loads, Load Cases, and Stress Recovery
- **Package Scope:** Implements primitive loads, concentrated/distributed user loads, load-case algebra, mechanical stress recovery, and analysis-status semantics.
- **Package Assigned Scope Items:** SOW-013, SOW-014, SOW-015, SOW-047, SOW-052
- **Package Exclusions:** Does not contain proprietary code load combinations or allowables.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-05-03
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-015
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-05-03


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
## 2026-09-09 - Endpoint section-cut, curved-frame, and pressure-eligibility repair
The same tranche added one shared genuine-pressure eligibility predicate for thrust and pressure stress. It excludes category or dimension mismatches while preserving every genuine pressure contribution and their sum. It does not adopt new Poisson behavior, closure topology, pressure-result schemas, connector mechanics, or longitudinal-pressure redesign.
The public full-envelope witness still reports 196 unchanged baseline result-contract errors, with zero errors on repair-affected endpoint and curved-station metadata. Root accepted the bounded repair criterion and kept the broader versioned public-result-contract repair open. Fresh native validation, broader pressure and connector work, DEC-025, and Git integration remain pending. The deliverable stays `IN_PROGRESS`; no lifecycle or DAG promotion is implied.
## 2026-09-08 - Candidate four-case pressure-reference investigation
P5 completed the candidate-only free-closed, restrained-closed, remote-closure/open-boundary-compensation, and thermal-plus-pressure investigation in the frozen [reference report](_run_records/PHYSICS_UI_EXECUTION_20260908/INVESTIGATION_REPORT.md), read with its additive [V2 node-on-element sign clarification](_run_records/PHYSICS_UI_EXECUTION_20260908/SIGN_CONVENTION_CLARIFICATION_V2.md). The independent [RP final review](../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/RP/RETURN.md) returned technical PASS, and [root's final notice](../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/notices/RP_REVIEW_FINAL_20260908.json) accepted that review scope. This completes investigation evidence only: Poisson/material authority, closure topology/defaults, public wall/effective/action/cut schema and migration, curved/expansion-joint treatment, thresholds, and verification fixtures remain Owner-gated production choices. No pressure physics, public schema, lifecycle, dependency, or acceptance state was adopted.
  pressure membrane components from explicit inputs.
- Reported missing resultants, missing section or pressure inputs, non-finite
  bending-y/z, torsional-shear, pressure-hoop, and pressure-longitudinal
- Pressure and thermal evidence: TP-MAC-06/09 and current product physics
  apply explicit invented thermal expansion and pressure-thrust axial
  corrections before stress recovery; pressure hoop rows remain, while
  pressure-longitudinal rows are suppressed when closed-end pressure thrust is
  exact internal force diagrams, arbitrary station input, broader pressure or
- Hardened `core/loads/stress_recovery/src/lib.rs` by rejecting non-finite
  recovered outputs from axial/bending division, torsional recovery, pressure
  infinite results into components or summaries.
  asymmetric optional pressure components, non-finite recovered pressure
  output, and non-finite range differences.
  component range boundary and omitted-pressure behavior.
- The helper performs finite-input validation and preserves code-neutral
  non-finite resultant inputs.
- Remaining TBDs: canonical calculation unit basis and conversions,
- Added `ForceResultants::from_station_resultants` as a finite-validated
  established axial, bending, torsional, pressure, status, and finding
  of non-finite or invalid station fields.
- Remaining TBDs: code/rule stress mappings, equivalent/principal stress,
- Remaining TBDs: code/rule stress mappings, equivalent/principal stress,
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from setup/future wording to current mechanics-only implementation evidence for axial, bending, torsional, pressure membrane, unit metadata, result-boundary, station, station-sweep, and mechanics-only range behavior.
## 2026-09-05 — Physics audit bounded product repairs (R08 and R11)
Straight interior section fields now use existing cut-equilibrium recovery with one documented j-side sign convention rather than interpolation of opposite nodal end actions. Endpoint action rows retain their existing convention. Supported straight mechanical distributed loads use matching fixed-end correction and station loading. The existing scalar stress expression receives internal span-boundary and stationary candidates of eight signed quadratics for constant-section piecewise-uniform straight loads. Public fixed stations remain; curved and combined continuous maxima are not claimed. Pressure constitutive/accounting policy remains held and unchanged.
Evidence: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P5/children/review/MANIFEST.json`, manager replay at `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P5/manager_replay/MANIFEST.json`, and package record `_run_records/WORKING_ITEMS_RUN_2026-09-05_PHYSICS_AUDIT_REPAIRS.md`. Root owns final fixture generation, independent P9 binding, clean DEC-025/native/CI checks and Git closeout. State remains IN_PROGRESS; broader lifecycle, release and professional reliance are not advanced. D01–D06 held decisions remain.

## execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application

### _STATUS.md
## Remaining


### _CONTEXT.md
# Context: DEL-05-05

**Deliverable ID:** DEL-05-05
**Name:** Concentrated and distributed user load application
**Package ID:** PKG-05
**Package Name:** Loads, Load Cases, and Stress Recovery
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement concentrated forces, concentrated moments, and distributed user loads with unit-aware application and result recovery hooks.

## Anticipated Artifacts
- load application module
- load tests
- result hooks

## Scope Coverage
- SOW-052
- SOW-013

## Objective Support
- OBJ-003
- OBJ-012

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Keeps general user loads separate from code-specific combinations.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Keeps general user loads separate from code-specific combinations.

## Package Reference
- **Package:** PKG-05 Loads, Load Cases, and Stress Recovery
- **Package Scope:** Implements primitive loads, concentrated/distributed user loads, load-case algebra, mechanical stress recovery, and analysis-status semantics.
- **Package Assigned Scope Items:** SOW-013, SOW-014, SOW-015, SOW-047, SOW-052
- **Package Exclusions:** Does not contain proprietary code load combinations or allowables.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-05-05
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-052,SOW-013
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-05-05


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
- Remaining TBDs: arbitrary-orientation global-to-local load mapping beyond
- Remaining TBDs: final result-envelope/API integration, production tolerance
- Remaining TBDs: partial-span consistent loads, final
  `ElementOutOfRange`, nonfinite axial force emits `NonFiniteAxialEffect`, and
- Remaining TBDs: primitive axial-effect provenance beyond `load_id`,
- True remaining TBDs: final result-envelope/API/persistence/GUI/CLI/report
- Hardened straight-pipe distributed-load recovery so nonfinite or
- Remaining TBDs: final result-envelope/API/persistence/GUI/CLI/report
  unit, finite magnitude, and provenance.
- Remaining DEL-05-05 residuals include distributed-load GUI integration,
  project force/length unit `N/m`, finite magnitude, and provenance.
- Remaining DEL-05-05 residuals include concentrated moments, element-station
  authoring, pressure/temperature primitive creation, imposed displacements,
  force*length unit `N*m`, finite magnitude, and provenance.
- Remaining DEL-05-05 residuals include pressure/temperature primitive
## 2026-06-11 - TP-APP-R2-PRESSTEMP-001 pressure and thermal primitive-load creation editor
  pressure/thermal load design authority in the desktop Load Cases manager.
- The GUI and operation seam now accept explicit pressure and thermal
  dimensions `pressure` and `temperature_interval`, project units `Pa` and
  `degC`, finite magnitude, and provenance.
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`
  Playwright smoke 1/1, and in-app browser pressure/thermal smoke.
- Remaining DEL-05-05 residuals include imposed displacements, final
  distributed force, moment, pressure, temperature, and imposed-displacement
  `lbf` concentrated force, `kPa` pressure, and incompatible `mm` rejection
- Remaining DEL-05-05 residuals include existing primitive-load magnitude-edit
- Remaining DEL-05-05 residuals include final result-envelope/API/
## 2026-09-05 — Physics audit bounded product repairs (R11)
Evidence: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P5/children/review/MANIFEST.json`, manager replay at `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P5/manager_replay/MANIFEST.json`, and package record `_run_records/WORKING_ITEMS_RUN_2026-09-05_PHYSICS_AUDIT_REPAIRS.md`. Root owns final fixture generation, independent P9 binding, clean DEC-025/native/CI checks and Git closeout. State remains IN_PROGRESS; broader lifecycle, release and professional reliance are not advanced. D01–D06 held decisions remain.

## execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel

### _STATUS.md
## Remaining
- Verify mechanics-program §5 completion: assessment gap rows G1/G2/G4 and M2/M3 methods defects closed or explicitly re-dispositioned by owner ruling (see also DEL-04-04, DEL-05-01, DEL-05-02) (gated: owner re-disposition where not closed by evidence) (source: mechanics plan §5 / DEC-066–070)


### _CONTEXT.md
# Context: DEL-04-01

**Deliverable ID:** DEL-04-01
**Name:** 3D frame stiffness kernel
**Package ID:** PKG-04
**Package Name:** Solver Core and Numerical Methods
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement the global 3D frame stiffness assembly, coordinate transforms, boundary conditions, and sparse solve interface.

## Anticipated Artifacts
- core/solver/frame_kernel
- unit tests

## Scope Coverage
- SOW-005
- SOW-035

## Objective Support
- OBJ-003

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** Central solver kernel; large but single domain and foundational.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** Central solver kernel; large but single domain and foundational.

## Package Reference
- **Package:** PKG-04 Solver Core and Numerical Methods
- **Package Scope:** Implements global 3D centerline/frame mechanics, straight pipe behavior, supports, nonlinear support logic, diagnostics, and performance harnesses.
- **Package Assigned Scope Items:** SOW-005, SOW-006, SOW-011, SOW-012, SOW-035, SOW-053
- **Package Exclusions:** Does not decide code compliance; produces mechanical results.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-04-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-005,SOW-035
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-04-01


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
  temporary verification interface until the sparse numerical library is
- The crate validates finite/positive numeric inputs, degenerate axes,
  conversion constants, solver tolerances, sparse solver library, or canonical
  non-finite input rejection.
- Accepted sparse numerical library remains `TBD`.
  reliance/compliance claims were introduced; sparse solver library, tolerance
  zero-pivot singularity handling, invalid dense matrix/non-finite entry
- Preserved open TBDs: sparse solver library, tolerance policy, canonical unit
  prescribed DOFs, and invalid prescribed displacement vector length/non-finite
- Preserved open TBDs: sparse solver library, tolerance policy, canonical unit
- No sparse solver, result envelope, units policy, release thresholds,
  stricter boundary metadata ID normalization, and documentation that the dense
- Preserved open TBDs: sparse solver library, solver tolerance policy,
- No sparse solver, tolerance policy, protected standards text/tables,
- No solver algorithm, sparse solver, tolerance policy, lifecycle state,
- Residual boundaries: Sparse solver library, production tolerance policy, release thresholds, final result-envelope integration, and professional reliance remain explicit downstream or human-governed TBDs.
## 2026-06-11 - DEC-023 sparse skyline solver, first bounded slice (TP-D03-SPARSE-001)
  `core/solver/sparse_direct` with deterministic reverse Cuthill-McKee
  `sparse_solver_tbd_diagnostic` reworded to record that selection is resolved
- Performance harness extended: sparse path measured alongside dense on the
  same reduced systems (`SparseSolveObservation`: profile/bandwidth, pivots,
  timing), new invented grid-frame fixture, suite-level sparse aggregates.
- Validation green: sparse_direct 18 tests; diagnostics 24; performance
  `_run_records/TASK_RUN_2026-06-11_TP-D03-SPARSE-001.md`, including the
  remaining assembly-side sparsity follow-up for PRD §20 thousands-of-nodes
- Residuals remain explicit: sparse live-path adoption is gated by `D-17`,
## 2026-06-21 - TP-R4-D7-SPARSELIVE-001 sparse evidence lane adoption
  current dense solve path remains default, while `core/solver/sparse_direct`
- `core/solver/nonlinear_integration` now records sparse evidence per iteration:
  sparse delta, sparse residual, and non-blocking unavailable status when the
  sparse sidecar cannot solve.
  `sparse_live_path_dense_parity_relative_delta` result rows for the invented
- Residuals remain explicit: profile-direct sparse assembly and sparse-default
## 2026-06-22 - TP-R4-D7-SPARSEPROFILE-001 product direct-profile sparse evidence
- Added a direct explicit-entry sparse solve surface in `core/solver/sparse_direct`
- `core/product_physics` sparse evidence rows now record
  `default_sparse_promotion=follow_on`.
- Regenerated the invented product-preview mechanics fixture and updated sparse
- Residuals remain explicit: dense remains default; default sparse promotion,
  nonlinear/core profile-direct sparse promotion, external validation
  sparse evidence lane consumes the same assembled matrix, so dense/sparse
  1.0e-9 relative tier (measured normalized deviation 3.6e-10, limited by the
- Pressure thrust keeps the recorded straight-chord axial treatment on macro
## 2026-07-19 - R14-W1-T3 arc pressure-thrust complete self-equilibrated system
- The recorded straight-chord axial pressure-thrust treatment on
  `consistent_radial_pressure_nodal_loads` (force method on the plain
  `arc_section_resultants_with_radial_pressure` (far-segment wall actions:
- `core/product_physics`: `add_pressure_thrust_loads` branches on
  pressure correction is retired for macro spans); stations include the wall
  load's far-segment actions; the `include_pressure_longitudinal` gating
  `pressure_thrust_treatment=arc_end_cap_tangent_pair_plus_consistent_radial_wall_load`.
  `validation/hand_calcs/mechanics/curved_bend_pressure_thrust_arc.md`
  `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` at the DEC-026 analytic-class
  normalized deviation ~1.5e-11). Fixture count 22 -> 23; one additive
## 2026-09-05 — Physics audit repair checkpoint: R01/R06 finite kernel and selected-state integration
Fallible element/global assembly, prescribed-force and dense/sparse solve paths reject computed nonfinite arithmetic through existing channels. Product ordinary mechanics now consumes one unrounded selected state, retaining ground springs in nonlinear assembly. Existing physics/pivot/convergence policies remain unchanged.
Evidence: `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/KERNEL_CHECKPOINT_V1/MANIFEST.json` (ff28049d1ff3ea27feb8f8ec9759cef10335792797a01a4dede03b897de5eae1), `PRODUCT_CHECKPOINT_V1/MANIFEST.json` (6541b981c4a9b2dc5568f1d47531d67049b129db5392147a99b1bfe8aef34bed), fresh KR plus backcheck and PR review packets, and D1 current derivative manifest663f30bafbfa9841a99baa1da79943d9f4811fc24b7c2040382d4ba7ae7ba766 under that P4 root. Manager orchestration is in `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P4/`.
This is bounded technical checkpoint acceptance, not lifecycle, engineering, release, or complete deliverable closure. State remains IN_PROGRESS and existing Remaining rows/Owner holds remain. Kernel targeted evidence197 PASS; P1 nonlinear22 PASS and product129 PASS with one enabled stale generated-fixture FAIL. Root separately owns final browser fixture regeneration after P5 integration; full P9/product/registered clean DEC025/native gates remain required. No global product PASS claimed. P5 receives exact frozen product source only after root accepts transfer.

## execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element

### _STATUS.md
## Remaining


### _CONTEXT.md
# Context: DEL-04-02

**Deliverable ID:** DEL-04-02
**Name:** Straight pipe element
**Package ID:** PKG-04
**Package Name:** Solver Core and Numerical Methods
**Type:** BACKEND_FEATURE_SLICE

## Description
Implement straight pipe local stiffness, section-property integration, weight hooks, and element force recovery.

## Anticipated Artifacts
- straight pipe element
- solver tests

## Scope Coverage
- SOW-006

## Objective Support
- OBJ-003

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Single element type.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Single element type.

## Package Reference
- **Package:** PKG-04 Solver Core and Numerical Methods
- **Package Scope:** Implements global 3D centerline/frame mechanics, straight pipe behavior, supports, nonlinear support logic, diagnostics, and performance harnesses.
- **Package Assigned Scope Items:** SOW-005, SOW-006, SOW-011, SOW-012, SOW-035, SOW-053
- **Package Exclusions:** Does not decide code compliance; produces mechanical results.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-04-02
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-006
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-04-02


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
- The governed analysis-run producer now creates the solver-to-envelope integration this deliverable's Remaining row named: `core/runner/headless/src/result_envelope_binding.rs` builds a validated DEL-08-04 result-export envelope document from a completed preview solve (RunnerRequest identity + RunnerResult + MechanicsEnvelope) and `run_preview_in_memory*` attaches it to a new serde-excluded `PreviewRunnerOutput` field, fail-closed on structural production/validation failure (`HEADLESS_RUNNER_RESULT_ENVELOPE_PRODUCTION_FAILED`). Brief `CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001` v3.
  end-i/end-j local result rows, thermal and pressure axial corrections, fixed
  metadata, pressure-longitudinal suppression for thrust-active cases, separate
  stress, equivalent/principal stress, broader thermal/pressure behavior,
  non-finite/overflowing weight-hook values, non-finite mass-per-length,
  extraction, non-finite recovery displacements, and required global-model
- Preserved open TBDs: sparse solver library, tolerance policy, canonical unit
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  validation for non-finite or out-of-range station fractions; they do not
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
## 2026-05-17 TP-PHYS-008 Axial Effects For Thermal And Pressure
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
- Residual boundaries: Sparse solver library, production tolerance policy, release thresholds, final result-envelope integration, and professional reliance remain explicit downstream or human-governed TBDs.
## 2026-09-05 — Physics audit repair checkpoint: R02 direct station point-input validation
Direct station recovery now validates point-load location fractions and finite force before location-based accumulation. Existing valid scalar/point-jump behavior remains. Later R08/R11 station-sign and consistent-distributed integration are PKG05 obligations; this entry does not claim their completion.
Evidence: `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/KERNEL_CHECKPOINT_V1/MANIFEST.json` (ff28049d1ff3ea27feb8f8ec9759cef10335792797a01a4dede03b897de5eae1), `PRODUCT_CHECKPOINT_V1/MANIFEST.json` (6541b981c4a9b2dc5568f1d47531d67049b129db5392147a99b1bfe8aef34bed), fresh KR plus backcheck and PR review packets, and D1 current derivative manifest663f30bafbfa9841a99baa1da79943d9f4811fc24b7c2040382d4ba7ae7ba766 under that P4 root. Manager orchestration is in `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P4/`.
This is bounded technical checkpoint acceptance, not lifecycle, engineering, release, or complete deliverable closure. State remains IN_PROGRESS and existing Remaining rows/Owner holds remain. Kernel targeted evidence197 PASS; P1 nonlinear22 PASS and product129 PASS with one enabled stale generated-fixture FAIL. Root separately owns final browser fixture regeneration after P5 integration; full P9/product/registered clean DEC025/native gates remain required. No global product PASS claimed. P5 receives exact frozen product source only after root accepts transfer.

## execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor

### _STATUS.md
## Remaining
- Broader viewport gesture/editor UX beyond the landed explicit component-creation tools, the 2026-09-05 persistent-workspace presentation, and the 2026-09-09 bounded orthogonal route-capture tranche remains separately selectable. Vocabulary row 15 component-symbol authoring is complete; no lifecycle promotion is inferred.
- Richer routing gestures, attached engineering transforms, finite component insert-in-run, and other still-listed tool-palette and 3D authoring work remain open. DAG-002-E0482 through E0485 remain unconsumed here; all formal dependency-row dispositions remain owner-gated and retain their recorded statuses. Historical row 15 closure and accepted N7 review history are preserved. See DEL-07-09 current `Capability_Comparison.csv` and `Palette_Operation_Routing.md`.


### _CONTEXT.md
# Context: DEL-07-01

**Deliverable ID:** DEL-07-01
**Name:** 3D viewport and centerline editor
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Type:** UX_UI_SLICE

## Description
Implement initial viewport interactions for nodes, pipe runs, bends, and simple component symbols.

## Anticipated Artifacts
- GUI viewport
- interaction tests

## Scope Coverage
- SOW-020

## Objective Support
- OBJ-006

## Context Envelope
- **Envelope:** L
- **Envelope Notes:** GUI surface is broad but bounded to viewport/editor.

## Context Budget QA
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands
- **Notes:** GUI surface is broad but bounded to viewport/editor.

## Package Reference
- **Package:** PKG-07 Graphical User Interface and Engineering Workflow
- **Package Scope:** Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.
- **Package Assigned Scope Items:** SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055
- **Package Exclusions:** Does not silently supply missing code data.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-07-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-020
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-07-01


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container remain implementation-level decisions unless this deliverable explicitly resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## PREPARATION Notes
- Structural scaffold only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
## 2026-09-09 - bounded viewport route capture and continuation
- Broader palette and 3D scope remains open, including finite component insertion in a run, attached engineering transforms, richer routing gestures, and other still-listed tools. Lifecycle stays `IN_PROGRESS`; dependency and formal row statuses are unchanged.
## 2026-09-08 - bounded native straight authoring implementation and validation
- The isolated packaged native walkthrough passed node-only save/reopen, atomic route Apply, explicit model inputs, two model-bound solve tuples, selected-property Apply and result invalidation, undo, 1024 x 768 presentation, and guarded store cleanup. Its historical successor `FINAL_MANIFEST.json` and `RETURN.md` have SHA-256 `372dfd3eec4f76315b2a580ce13207ffb44318e6e6f9a1e3bd0256ca144716d1` and `b25ce242d5923979c2a4b5a7389fdbc14705b480d15c03ed96ee75cfa386be44`. These R3/native hashes are predecessor bindings; the separately managed evidence-only correction must be cited with them once sealed to resolve normalized active records through lossless original archives.
## Remaining TBDs
- Remaining product-level items remain downstream or broader-program scope:
  to the viewport editor for user-entered node id, label, and finite x/y/z
  with normalized display offsets.
  `scale=normalized_display_offset_not_physical_length`,
## 2026-09-05 — bounded workspace design implementation
- Source snapshot: `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W7/snapshots/SOURCE_V2/`, manifest SHA256 `c53fd038fd753ea39049b37de98343d6427abb11446439fd1b41b95703eda5f2`. Selected evidence: `_run_records/WORKING_ITEMS_RUN_2026-09-05_WORKSPACE_DESIGN.md`; manager `RETURN.md` and current independent `children/R1/RETURN_V2.md`.
- Remaining was narrowed only to exclude this landed presentation slice. Broader routing/attachment gestures, full specialized property widgets, report MAP-031 inline human Add/Apply, modulus-basis/unit-picker/hardening work where already listed, analytical integration, durable history and D-58 remain under their existing owners and holds. Current Queue → Review → Apply behavior remains. Historical N7 records are preserved; this tranche does not disposition them.
## 2026-09-05 — browser path and diagnostic pointer repair supplement
- Source manifest SHA256 `cf2cc3df5746b76652974df700a78ea0f876ca232ab9835ecf019ec0d5d18daf` supersedes only the applicable live source binding; previous SOURCE_V2, FINAL_SNAPSHOT and run records remain immutable historical evidence. New local evidence: `_run_records/WORKING_ITEMS_RUN_2026-09-05_BROWSER_POINTER_REPAIR.md`; manager `E2E_REPAIR_RETURN.md` and R2 `RETURN_FINAL_V3.json`.
- No further Remaining narrowing: this repair belongs to the bounded presentation slice already excluded from broad residuals. IN_PROGRESS, ScopeOfWork, engineering holds, MAP031 inline Apply, full typed widgets, D58 and PDU045/046 are unchanged.

## execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-09_Interactive operation vocabulary and tool palette contract

### _STATUS.md
## Remaining

- Production implementation remains outside DEL-07-09. Coverage work is governed by the accepted SCA-009 Vocabulary Annex and lands only in the deliverables named in its `Implementation lands in` column.
- Historical selected rows 14–16 retain their recorded landed/closed status. Lifecycle review or promotion of DEL-07-09 itself remains separately human-gated; current implementation evidence does not move its `OPEN` state.
- Current reconciliation explicitly records all 24 NORMATIVE-NOW rows and R1–R3 with implemented UI routes, PKG-16 linkage and bounded residuals. Original historical cells for rows 14–16 are preserved; added current evidence is implementation/check evidence only, with final N7 integration review pending. D58 live provider and DEL-16-03 durable accepted-history residual remain held/open; R1–R3 remain deferred. No whole-toolkit or lifecycle closure is claimed.
- N7 F1/F2/F3 current coverage amendment records canonical family mapping, shared duplicate-stiffness rejection and DOF-correct display readouts; original row14–16 historical cells remain unchanged. Postrepair desktop build / 43 files / 732 tests and host dist 1 test PASS; root N7 final rereview remains pending. See DEL-07-09 Palette_Operation_Routing.md N7 repair amendment; no lifecycle closure.


### _CONTEXT.md
# Context: DEL-07-09

**Deliverable ID:** DEL-07-09
**Name:** Interactive operation vocabulary and tool palette contract
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Type:** UX_UI_SLICE

## Description

Own the ratified two-class interactive operation vocabulary coverage contract and the single tool-palette surface organization over the viewport and tree/inspector surfaces, routed through the structured model operation layer.

## Anticipated Artifacts

- vocabulary coverage ledger
- palette organization contract
- palette-to-operation routing map

## Scope Coverage

- SOW-077

## Scope Detail

- SOW-077: The GUI shall provide a ratified interactive model-building operation vocabulary and a governing tool-palette contract that organizes the human command surface for building and modifying the piping model, with every palette command routed through structured model operations.

## Objective Support

- OBJ-006
- OBJ-015

## Context Envelope

- **Envelope:** L
- **Risk:** WATCH
- **Recommended Action:** Confirm scope and split if it expands.
- **Notes:** Contract/coverage slice only. Implementation lands in the deliverables named in the SCA-009 Vocabulary Annex; editor implementation does not land here.

## Package Reference

- **Package:** PKG-07 Graphical User Interface and Engineering Workflow
- **Package Scope:** Implements the interactive modeler, editors, warning UX, solve-execution UX, and results views.
- **Package Assigned Scope Items:** SOW-020, SOW-021, SOW-022, SOW-023, SOW-036, SOW-055, SOW-076, SOW-077
- **Package Exclusions:** Does not silently supply missing code data.

## Accepted Authority

- **Decomposition:** `execution/_Decomposition/SOFTWARE_DECOMP.md`
- **Accepted Revision:** 0.12
- **Scope Change:** `execution/_ScopeChange/SCA-009_2026-08-20_0000/`
- **Decision:** DEC-094
- **Vocabulary Contract:** `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`
- **Boundary:** DEL-07-09 owns the coverage contract and palette organization only; it never dispatches or receives the implementation routed by the annex.

## Register References

- **Deliverables Register:** `docs/_Registers/Deliverables.csv` row DEL-07-09
- **Scope Ledger:** `docs/_Registers/ScopeLedger.csv` row SOW-077
- **Context Budget QA:** `docs/_Registers/ContextBudgetQA.csv` row DEL-07-09

## PREPARATION Notes

- Structural scaffold and source-grounded metadata only.
- Lifecycle is OPEN; no production document or implementation artifact is created by PREPARATION.


## execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-01_Structured model operation schema

### _STATUS.md
## Remaining
- Obtain owning human dispositions for `PKG16-DEL1601-PKG02-001` and `PKG16-DEL1601-PKG02-002`; both remain `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`, notwithstanding current model-basis/hash and fixture evidence (PDU-060).


### _CONTEXT.md
# Context: DEL-16-01

**Deliverable ID:** DEL-16-01
**Name:** Structured model operation schema
**Package ID:** PKG-16
**Package Name:** Model Operation and Agent Proposal Framework
**Type:** DATA_MODEL_CHANGE

## Description
Define structured add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge operations.

## Anticipated Artifacts
- schemas/model_operation.schema.json
- operation fixtures

## Scope Coverage
- SOW-069

## Scope Detail
- SOW-069: All GUI and agent edits shall be represented as structured model operations that pass schema validation, constraint validation, diff preview, and controlled application through the model engine.
## Objective Support
- OBJ-015

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Operations are the only model mutation route for GUI and agent proposals.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Operations are the only model mutation route for GUI and agent proposals.

## Package Reference
- **Package:** PKG-16 Model Operation and Agent Proposal Framework
- **Package Scope:** Implements structured model operations, validation/diff preview, user acceptance/audit trail, and agent rationale/professional-boundary controls.
- **Package Assigned Scope Items:** SOW-069, SOW-070
- **Package Exclusions:** Does not allow hidden model mutations or autonomous engineering acceptance.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis_for_control_surface_refresh

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-16-01
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-069
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-16-01

## Architecture Basis Injection
- **Scope Change:** SCA-001 architecture basis, carried into SCA-002 revision 0.7 coordination.
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark `PKG-00` as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7
- **Applicable Basis IDs:** AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.
- **Still TBD:** Exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, physical project package/container, and package-specific implementation choices remain decision-gated unless this deliverable later resolves one under human approval.
- **Dispatch Rule:** Future TASK execution must apply only the applicable architecture-basis constraints and must not copy full PKG-00 prose into deliverable artifacts.

## SCA-002 Control-Surface Refresh Note
- This control surface was created by PREPARATION on 2026-05-03 from the accepted revision 0.7 decomposition and companion registers.
- This pass did not dispatch Type 2 work, produce implementation artifacts, promote candidate edges, refresh the blocker queue, materialize local `Dependencies.csv`, or promote the quarantined Chirality corpus.

## PREPARATION Notes
- Structural scaffold and metadata context only.
- No Type 2 implementation artifacts are drafted in this folder by PREPARATION.


### MEMORY.md
## Remaining TBDs
- Expansion joints persist only explicit geometry and four-axis stiffness inputs; no pipe role, mechanics interface, or engineering value is inferred.
