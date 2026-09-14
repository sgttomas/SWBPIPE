# Actual bounded deliverable excerpts

## execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module
## Remaining

- The 2026-09-08 four-case pressure-reference investigation is complete as candidate evidence; read its frozen report with `SIGN_CONVENTION_CLARIFICATION_V2.md`. RP technical PASS validates that packet only. Production pressure-physics and public-schema adoption remain open to Owner rulings on Poisson/material authority, closure topology/defaults, typed force/result compatibility, curved and expansion-joint treatment, thresholds, and verification fixtures. No lifecycle, dependency, or acceptance change is implied.
- The 2026-09-09 endpoint section-cut, curved-frame metadata, and genuine-pressure eligibility repair passed focused and full `product_physics` validation plus the root-managed combined source-and-fixture review. The bounded repair has zero schema errors on affected endpoint and curved-station rows. The complete public envelope still has 196 unchanged baseline contract errors, so versioned public-result-contract work remains open.
- Fresh native validation and integration remain pending. Broader pressure behavior, connector treatment, public result-contract work, DEC-025, and CHANGE-owned Git closeout remain open. No lifecycle or DAG promotion is recorded.

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
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js v
---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-03
package_id: PKG-05
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-015]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-05-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-05-03` in service of project scope [SOW-015] and package objectives [OBJ-003].

- **OUT-001** — A fundamental stress-recovery contract covering mechanics-only recovery from element end and station resultants, axial, bending, torsional and shear components, section-property and modulus inputs, deterministic sweeps, units, findings, and result hooks is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-05-03 Fundamental stress recovery module

> #### Datasheet: DEL-05-03 Fundamental stress recovery module
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-05-03 |
> | Package ID | PKG-05 |
> | Package | Loads, Load Cases, and Stress Recovery |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-015 |
> | Objective | OBJ-003 |
> | Context envelope | M |
> | Anticipated artifacts | stress recovery module; hand-calc tests |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Current value |
> |---|---|
> | Module purpose | Recover code-neutral mechanics stresses from explicit force resultants, pressure-basis inputs, section properties, station resultants, a
nal rows are suppressed when closed-end pressure thrust is
  exact internal force diagrams, arbitrary station input, broader pressure or
  recovered outputs from axial/bending division, torsional recovery, pressure
  asymmetric optional pressure components, non-finite recovered pressure
  component range boundary and omitted-pressure behavior.
- Preserved open TBDs: canonical calculation unit basis and conversion
- Remaining TBDs: canonical calculation unit basis and conversions,
  established axial, bending, torsional, pressure, status, and finding
- Remaining TBDs: code/rule stress mappings, equivalent/principal stress,
  transverse shear stress, canonical unit conversions, production tolerance
- Remaining TBDs: code/rule stress mappings, equivalent/principal stress,
  transverse shear stress, canonical unit conversions, production tolerance
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from setup/future wording to current mechanics-only implementation evidence for axial, bending, torsional, pressure membrane, unit metadata, result-boundary, station, station-sweep, and mechanics-only range behavior.
- Preserved true TBDs for final application-service/result-envelope ownership, production section-property source authority, public report/rule-pack labels and sign conventions, unit conversion catalog, production tolerance policy, release benchmark scope, and professional reliance.
  private data, conversion constant, load default, or tolerance policy was
- Residual boundaries: Final application-service/result-envelope ownership, code/rule stress mappings, conversion catalog, production tolerance policy, release benchmark scope, and professional reliance remain explicit TBDs.
Straight interior section fields now use existing cut-equilibrium recovery with one documented j-side sign convention rather than interpolation of opposite nodal end actions. Endpoint action rows retain their existing convention. Supported straight mechanical distributed loads use matching fixed-end correction and station loading. The existing scalar stress expression receives internal span-boundary and stationary candidates of eight signed quadratics for constant-section piecewise-uniform straight loads. Public fixed stations remain; curved and combined continuous maxima are not claimed. Pressure constitutive/accounting policy remains held and unchanged.

## execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance
## Remaining

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
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-fa
---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-01
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-017]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-01` in service of project scope [SOW-017] and package objectives [OBJ-004].

- **OUT-001** — A material-library schema contract covering temperature-dependent properties, allowable slots, provenance, redistribution status, privacy, completeness, and explicit diagnostics is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-03-01 Material library schema with provenance

> #### Datasheet: DEL-03-01 Material library schema with provenance
>

### CLM-002 — PDU-024 Version Integration

> ##### PDU-024 Version Integration
>
> - Owning material schema remains `schemas/material.schema.yaml`.
> - Project-carried material data consumes DEL-02-05 model-document version handling, whose accepted current family is `0.2.0`.
> - No material fixture migration, material serializer, or new migration status is added by this tranche.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-03-01 |
> | Package ID | PKG-03 |
> | Package | Piping Components, Materials, and Library Data Model |
> | Type | DATA_MODEL_CHANGE |
> | Scope item | SOW-017 |
> | Objective | OBJ-004 |
> | Anticipated art
## 2026-07-12 - D41-R5-T2B-PDU024 downstream version integration
- Recorded the bounded downstream integration for `DEL-03-01-REQ-007`: project-carried material data consumes DEL-02-05's accepted `0.2.0` model-document family and explicit current/stale/unsupported/newer/failed version-check behavior.
  to 38 while conversion-witness targets remain two.
  catalog data, unit conversion behavior, private data, lifecycle transition,
  temperature interpolation policy, and allowable storage details remain `TBD`.
- Remaining TBDs from the sealed dispatch stay open: public material source
  catalog, public fixture value policy, temperature interpolation policy,
- Remaining `TBD` items include public material source catalog, public fixture value policy, temperature interpolation policy, allowable storage policy, dependency satisfaction, protected-content/redistribution review disposition, and human review dispositions.
- PKG-02 vocabulary evidence observed: `MaterialPropertyDimension` uses `density`, `stress`, `temperature`, `temperature_interval`, `thermal_expansion_coefficient`, `specific_heat`, `thermal_conductivity`, `dimensionless`, and `TBD`; retired aliases remain excluded by `tests/test_material_schema.py`.
- Remaining local review findings: `PKG03-DEL-03-01-PKG02-001` and `PKG03-DEL-03-01-PKG02-002` both remain `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`; no `Review_Findings.csv` edits were made.
- Remaining gates: `HumanDisposition` remains `TBD`; lifecycle remains `IN_PROGRESS`; dependency satisfaction, protected-content/redistribution review, public material source catalog, public fixture value policy, temperature interpolation policy, and allowable storage policy remain `TBD`.
  shear modulus, and thermal expansion coefficient.

## execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization
## Remaining
- The canonical `project.run_history` read path now reaches backend report sections, but explicit project-document migrate, the `.opsproj` multi-member container, compatibility-window semantics, and exact external/non-JSON payload partitioning remain open.
- Close FR-001 residuals: explicit migrate operation, compatibility-window semantics, and `.opsproj` multi-member container implementation per DEC-028 + DEC-057 naming rider (source: PRD plan §4 FR-001 row / DEC-028/DEC-057)
- H2 / F-5b: relocate DEC-019 migration evaluation into a wasm-compilable crate, replace `projectService.ts` `evaluateModelDocumentLocal`, and cover the migrated-bytes hash-integrity edge with cross-engine parity tests (source: PRD plan §3 hardening row H2). The three stale UI current-version comparisons were closed by DEC-074 R5 T2B/PDU-024 on 2026-07-12.

# Context: DEL-02-05

**Deliverable ID:** DEL-02-05
**Name:** Project persistence and round-trip serialization
**Package ID:** PKG-02
**Package Name:** Domain Model, Units, and Core Schemas
**Type:** DATA_MODEL_CHANGE

## Description
Implement create/open/save/versioned project persistence and deterministic model round-trip behavior for units, loads, rule-pack refs, and provenance metadata.

## Anticipated Artifacts
- project file schema
- round-trip tests
- persistence service contract

## Scope Coverage
- SOW-050
- SOW-041

## Objective Support
- OBJ-001
- OBJ-012

## Context Envelope
- **Envelope:** M
- **Envelope Notes:** Single data persistence surface; implement canonical JSON/JCS payloads over the SCA-003 local SQLite storage profile.

## Context Budget QA
- **Risk:** OK
- **Recommended Action:** Proceed with bounded Type 2 brief
- **Notes:** Single data persistence surface; implement canonical JSON/JCS payloads over the SCA-003 local SQLite storage profile.

## Package Reference
- **Package:** PKG-02 Domain Model, Units, and Core Schemas
- **Package Scope:** Defines the canonical software entities, unit system, persistence/serialization contracts, and extensibility boundaries.
- **Package Assigned Scope Items:** SOW-002, SOW-025, SOW-038, SOW-041, SOW-050
- **Package Exclusions:** Does not implement numerical solving or GUI views.

## Decomposition Reference
- **Decomposition:** execution/_Decomposition/SOFTWARE_DECOMP.md
- **Accepted Revision:** 0.7
- **Status:** current_basis

## Register References
- **Deliverables Register:** docs/_Registers/Deliverables.csv row DEL-02-05
- **Scope Ledger:** docs/_Registers/ScopeLedger.csv rows SOW-050,SOW-041
- **Context Budget QA:** docs/_Registers/ContextBudgetQA.csv row DEL-02-05


## Architecture Basis Injection
- **Scope Change:** SCA-001
- **Architecture Basis:** `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints for this sealed context. This does not mark PKG-00 as `ISSUED`.
- **Decomposition Revision:** execution/_Decomposition/SOFTWARE_DECOMP.md re
---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-05
package_id: PKG-02
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-050, SOW-041]
package_objective_refs: [OBJ-001, OBJ-012]
---

# Scope of Work — DEL-02-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-02-05` in service of project scope [SOW-050, SOW-041] and package objectives [OBJ-001, OBJ-012].

- **OUT-001** — A project-persistence and round-trip serialization contract covering create, open, save, versioned persistence, units, loads, rule-pack references, provenance, and reproducibility metadata is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet - DEL-02-05 Project Persistence and Round-Trip Serialization

> #### Datasheet - DEL-02-05 Project Persistence and Round-Trip Serialization
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS
onsume the DEC-019/DEC-033 browser evaluator rather than treating `0.1.0` as current.
- Current `0.2.0` documents report `current`; migratable `0.1.0` documents report `stale` at version-check grain and retain `migrated` migration evidence; unsupported, newer, and failed inputs retain established diagnostic/refusal states.
  save, validate, version-check, and migrate flows.
  explicit user mechanics combinations, pressure thrust, and fixed station-grid
## 2026-06-10 - TP-MAC-80 versioned store schema migration ledger
  a versioned `PRAGMA user_version` migration ledger
  (`versioned_sqlite_user_version_migration_ledger`,
  `STORE_SCHEMA_TARGET_VERSION=7`: v1 base project/FTS tables, v2–v7 the six
  snapshot JSON columns) instead of ad-hoc unversioned setup, and the
  `migration_framework`, `migration_status`, `store_schema_version`,
  `store_schema_target_version`, `migrations_applied_on_open`; the validation
- Authority basis: DEL-00-04 REQ-04-01/REQ-04-02 (versioned persistence;
  declared schema version and migration status). Model *document* schema
  `STORE_SCHEMA_TARGET_VERSION=8`) adds `project_envelope_hash_json`;
  versioning with in-document semver as the sole model-document version
## 2026-06-12 - TP-APP-R2-DOCVER-020-001 DEC-033 model document version 0.1.0 -> 0.2.0
  document supported schema version moved 0.1.0 -> 0.2.0 because
  transform; the chain walker stamps the version; the save-time ledger record
  version-gate; corpus suite green unchanged). `schemas/model.schema.yaml`
  pattern-validates schema_version and needed no edit.
- Residuals for H2: feature panels hard-code "0.1.0" version-check literals
  Same idempotent user_version ledger pattern; migration-evidence tests
  schema versioning (DEC-019/DEC-033) is unaffected.
  no unit conversion, target import compatibility, solver behavior,
  `conversion_performed=false`, and DEC-018/DEL-02-02/DEL-02-05 basis refs.
  canonicalization, schema versioning, unit conversion API, protected
  units and `conversion=false` alongside the existing round-trip status,
  checked-ref count/signature, and no-conversion policy.
  leaving target-format conversion-witness count at two.
  project-envelope schema, model hash canonicalization, unit-conversion API,
  project-envelope schema, model hash canonicalization, unit-conversion API,
  canonicalization, unit-conversion API, DEC-018 catalog constant, schema

## execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model
## Remaining

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
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/V
---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-06
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-010]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-06

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-06` in service of project scope [SOW-010] and package objectives [OBJ-004].

- **OUT-001** — An expansion-joint component-model contract covering supplied stiffnesses, effective area, movement limits, hardware fields, units, provenance, completeness, and diagnostics is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-03-06 Expansion joint component model

> #### Datasheet: DEL-03-06 Expansion joint component model
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh
  `DEL-03-06 - Expansion joint component model`.
- Added expansion-joint schema support as slots and diagnostics only.
- Accepted public expansion-joint source catalogs remain `TBD`.
- Public expansion-joint fixture value policy remains `TBD`.
- Concrete expansion-joint import formats remain `TBD`.
- Implemented evidence: commit `f15cbc6` (`schema: add expansion joint
- Preserved boundaries: expansion-joint stiffnesses, effective areas, movement
- Local audit artifacts are `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/_REVIEW.md` and `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/Review_Findings.csv`.
- Current evidence implements the `expansion_joint` component type,
  completeness evidence, and expansion-joint diagnostics.
- Movement-limit classes, hardware taxonomy, public expansion-joint source
## 2026-06-21 - TP-R4-D4-EJSTIFF-001 expansion-joint app absorption
  `component:C-150` now carries explicit pipe mapping, effective pressure area,
  movement limit, hardware/manufacturer references, pressure-thrust reference,
  expansion-joint area, movement, linear stiffness, and rotational stiffness
  quantities, emits `EXPANSION_JOINT_*` diagnostics, and appends four
  expansion-joint path.
  Playwright smoke now surface the D4 expansion-joint fields and review rows.
  data, pressure-thrust load generation, global nonlinear solve, lifecycle
## 2026-06-22 - TP-R4-D4-EJTHRUST-001 expansion-joint pressure-thrust load generation
- R4/D4 pressure-thrust follow-on landed for the invented preview path under
  `DEC-045`: pressure-thrust load evidence for `component:C-150` now uses the
  user-entered expansion-joint effective area on `pipe:P-130` plus explicit
  pressure load-case inputs for `L-100` and `L-200`.
- Product-physics preview output now records component pressure-thrust result
  load-case references, and `EXPANSION_JOINT_PRESSURE_THRUST_APPLIED`
- The report export packet now preserves the selected pressure-thrust result
  refs and component pressure-thrust evidence for the invented preview report.
  standards content, code-derived pressure-thrust coefficient, proprietary

## execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element
## Remaining

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
- **Resolved Baseline:** Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2
---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-02
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-006]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-04-02` in service of project scope [SOW-006] and package objectives [OBJ-003].

- **OUT-001** — A straight-pipe-element contract covering local stiffness, explicit section-property integration, weight hooks, boundary metadata, spanned loads and axial effects, unit-aware end/station resultant recovery, and deterministic solver verification is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-04-02 Straight pipe element

> #### Datasheet: DEL-04-02 Straight pipe element
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and f
-PKG04-PRODUCER-BINDING-001` v3.
- Canonical calculation unit basis and conversion constants remain `TBD`.
  end-i/end-j local result rows, thermal and pressure axial corrections, fixed
  metadata, pressure-longitudinal suppression for thrust-active cases, separate
  calculation unit basis/conversion constants, primitive weight application,
  stress, equivalent/principal stress, broader thermal/pressure behavior,
  basis/conversion constants, primitive weight application, downstream stress
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, primitive-load integration beyond explicit displacement
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, station envelopes beyond this slice, result-envelope
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, partial-span load recovery, station envelopes,
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, partial-span load recovery, station envelopes, release
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, final result-envelope/export/API integration, release
## 2026-05-17 TP-PHYS-008 Axial Effects For Thermal And Pressure
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, final result-envelope/export/API integration, release
- Remaining TBDs: sparse solver library, global tolerance policy, canonical
  unit conversions, final result-envelope/export/API integration, release
  `force_per_length` unit metadata while preserving the no-conversion and
  private data, unit conversion, load default, or tolerance policy was changed
  private data, conversion constant, load default, or tolerance policy was
This is bounded technical checkpoint acceptance, not lifecycle, engineering, release, or complete deliverable closure. State remains IN_PROGRESS and existing Remaining rows/Owner holds remain. Kernel targeted evidence197 PASS; P1 nonlinear22 PASS and product129 PASS with one enabled stale generated-fixture FAIL. Root separately owns final browser fixture regeneration after P5 integration; full P9/product/registered clean DEC025/native gates remain required. No global product PASS claimed. P5 receives exact frozen product source only after root accepts transfer.
