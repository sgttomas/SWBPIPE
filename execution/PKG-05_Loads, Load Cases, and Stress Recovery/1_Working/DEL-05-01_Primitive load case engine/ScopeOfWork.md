---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-01
package_id: PKG-05
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-013]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-05-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-05-01` in service of project scope [SOW-013] and package objectives [OBJ-003].

- **OUT-001** — A primitive-load-case contract covering explicit weight, pressure, thermal, imposed-displacement, hydrotest, wind, seismic, and occasional mechanics categories, unit-aware preparation, load-case records, diagnostics, and deterministic solver-boundary contributions is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-05-01 Primitive load case engine

> #### Datasheet: DEL-05-01 Primitive load case engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-05-01-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-05-01 |
> | Name | Primitive load case engine |
> | Package | PKG-05 Loads, Load Cases, and Stress Recovery |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope items | SOW-013 |
> | Objectives | OBJ-003 |
> | Context envelope | M |
>

### CLM-004 — Evidence Summary

> ##### Evidence Summary
>
> | Evidence surface | Current evidence |
> |---|---|
> | Implementation crate | `core/loads/primitive_loads` |
> | Crate scope | Code-neutral primitive load records and deterministic mechanics-boundary preparation for weight, pressure, thermal, imposed displacement, hydrotest, wind, seismic, and occasional categories. |
> | Readme evidence | `core/loads/primitive_loads/README.md` documents implemented categories, storage-neutral primitive load-case records, boundary quantity metadata, diagnostic bridge records, lumping, and exclusions. |
> | Rust API evidence | `core/loads/primitive_loads/src/lib.rs` defines primitive category/type records, load quantities, target/direction records, boundary metadata, load-case records, diagnostic records, preparation helpers, lumping, axial-effect helpers, and solver-vector assembly helpers. |
> | Local memory evidence | `MEMORY.md` records implementation, verification, and preserved boundaries from the original crate implementation through the 2026-06-05 foundational hardening tranche and parent fan-in. |
>

### CLM-005 — Attributes

> ##### Attributes
>
> | Attribute | Current value | Source |
> |---|---|---|
> | Primary subject | Primitive load definitions and mechanics-boundary preparation for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional categories. | `_CONTEXT.md`; `core/loads/primitive_loads/README.md`; `src/lib.rs` |
> | Analysis model boundary | Primitive loads are prepared for a 3D centerline/frame mechanics model through nodal, element-uniform, imposed-displacement, lumped-equivalent nodal, axial-effect, and solver-vector assembly surfaces. | `docs/CONTRACT.md` OPS-K-MECH-1; `src/lib.rs` |
> | Mechanics/rule boundary | The crate defines mechanics load categories and deterministic findings; it does not evaluate user-rule acceptability, code compliance, professional judgment, code combinations, or allowables. | `docs/CONTRACT.md` OPS-K-MECH-2; `core/loads/primitive_loads/README.md` |
> | Unit boundary | Load magnitudes carry explicit dimensions; boundary metadata maps accepted PKG-02 dimension vocabulary including `force_per_length` and rejects retired aliases. No conversion constants are supplied. | OPS-K-UNIT-1; `CanonicalDimension`; `QuantityUnitMetadata` |
> | Boundary metadata | `BoundaryRecordRef`, `BoundaryQuantityRecord`, and `PrimitiveLoadCaseRecord` carry schema binding, target/basis refs, JCS payload refs, payload-hash refs, unit metadata, provenance refs, and canonical round-trip key material. | `src/lib.rs` |
> | Load-case record boundary | `PrimitiveLoadCaseRecord` binds one primitive category to canonical model `LoadCase` metadata, sorted load IDs, provenance, payload refs, and hash refs. Mixed-category algebra remains downstream DEL-05-02 scope. | `PrimitiveLoadCaseRecord`; `_DEPENDENCIES.md` |
> | Diagnostic bridge | `LoadDiagnosticRecord` maps primitive-load validation and load-case assembly findings to code, class, blocking severity, source, affected object, message, remediation, and provenance ref for later result-envelope transport. | `LoadDiagnosticRecord`; `diagnostic_records_from_load_findings`; `diagnostic_records_from_load_case_assembly_findings` |
> | Missing values | Missing solve-required load targets, magnitudes, spans, properties, or provenance become explicit findings/errors rather than silent defaults. | OPS-K-DATA-2; `FindingCode`; `BoundaryMetadataError` |
> | Dynamic scope | Wind, seismic, and occasional loads are represented only as explicit equivalent mechanics loads; equivalent-static preparation requires caller-supplied basis/provenance refs. Dynamic procedure generation, response spectra, code coefficients, and environmental defaults remain `TBD` and out of this deliverable. | `_CONTEXT.md`; `README.md`; tests in `src/lib.rs` |
>

### CLM-006 — Primitive Load Category Register

> ##### Primitive Load Category Register
>
> | Category | Implemented mechanics evidence | Preserved boundary / open item |
> |---|---|---|
> | Weight | Element uniform `ForcePerLength` loads; eligible for lumped equivalent nodal conversion when caller supplies span/connectivity. | Mass-source, gravity-vector, density provenance, and unit-conversion policy remain `TBD`. |
> | Pressure | Element pressure or equivalent line-load records; straight-pipe pressure thrust helper computes `p * internal_area` from caller-supplied properties. | Pressure stress formulas, code stress categories, pressure design rules, and property/default sourcing remain outside this deliverable. |
> | Thermal | Element temperature-change records; straight-pipe thermal axial helper computes `E * A * alpha * DeltaT` from caller-supplied properties. | Reference temperature, material property provenance, expansion policy, and broader thermal behavior remain bounded to future sealed scopes. |
> | Imposed displacement | Support-target imposed displacement records preserve translational displacement or rotational DOF boundary. | Support/restraint implementation and boundary-condition ownership remain interface-dependent. |
> | Hydrotest | Element pressure or equivalent line-load category is represented in primitive mechanics preparation. | Hydrotest procedure defaults, test-fluid assumptions, and code requirements are not invented. |
> | Wind | Explicit equivalent nodal force or element `ForcePerLength` mechanics loads are accepted. | Wind coefficients, profiles, exposure rules, and code factors are user/rule-supplied `TBD`. |
> | Seismic | Explicit equivalent nodal force or element `ForcePerLength` mechanics loads are accepted. | Dynamic treatment, response parameters, seismic code procedure generation, and acceleration-style placeholders remain out of scope. |
> | Occasional | Explicit equivalent nodal force/moment or element `ForcePerLength` mechanics loads are accepted. | Event definition and user/rule mapping remain downstream/user-governed. |
>

### CLM-007 — Implemented Surfaces

> ##### Implemented Surfaces
>
> | Surface | Current status |
> |---|---|
> | Primitive category model | Implemented by `PrimitiveLoadCategory` and `PrimitiveLoadCaseKind`. |
> | Primitive load records | Implemented by `PrimitiveLoad`, `LoadTarget`, `LoadDirection`, and `LoadQuantity`. |
> | Boundary quantity metadata | Implemented by `CanonicalDimension`, `QuantityUnitMetadata`, `BoundaryRecordRef`, and `BoundaryQuantityRecord`. |
> | Primitive load-case records | Implemented by `PrimitiveLoadCaseRecord`; validates model `LoadCase` schema binding, non-empty refs, non-empty unique load IDs, and single-category membership. |
> | Load preparation | Implemented by `prepare_loads` for nodal, element-uniform, and imposed-displacement contributions with deterministic sorting. |
> | Equivalent-static preparation | Implemented by `EquivalentStaticMechanicsBasis` and `prepare_equivalent_static_loads` for wind, seismic, and occasional explicit mechanics loads with caller-supplied basis/provenance refs. |
> | Lumped equivalent nodal conversion | Implemented by `prepare_lumped_nodal_loads` for explicit translational/global `ForcePerLength` element loads with caller-supplied spans/connectivity. |
> | Axial effects | Implemented by `prepare_straight_pipe_axial_effects` for thermal axial force and pressure thrust from caller-supplied properties. |
> | Solver load-vector assembly | Implemented by `assemble_solver_load_vector`; sorts/sums valid nodal contributions and returns findings instead of partial vectors when invalid data exists. |
> | Diagnostic bridge | Implemented by diagnostic conversion helpers for primitive-load and load-case assembly findings. |
>

### CLM-008 — Verification Evidence

> ##### Verification Evidence
>
> | Evidence | Result |
> |---|---|
> | Historical run records | `MEMORY.md` records successful cargo formatting/testing tranches for primitive implementation, lumping, solver-vector assembly, axial effects, boundary dimension update, load-case records, and diagnostic bridge. |
> | Current crate tests | `src/lib.rs` contains 40 tests for all primitive categories, boundary metadata, retired dimension aliases, equivalent-static handling, load-case records, deterministic sorting, lumping, axial effects, diagnostic records, missing/invalid inputs, and solver-vector assembly findings. |
> | Current validation evidence | `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml --locked` passed with 40 tests in the 2026-06-05 foundational hardening tranche; doc-alignment closeout should still run diff hygiene for touched files. |
>

### CLM-009 — Conditions

> ##### Conditions
>
> - Primitive load categories remain separate from code-specific load combinations and allowables. Source: `_CONTEXT.md`; PKG-05 exclusions; SOW-014 note.
> - The downstream load-case algebra/user-combination interface remains DEL-05-02. Source: `Specification.md` REQ-05-01-007; `_DEPENDENCIES.md`.
> - The crate exposes storage-neutral boundary records and diagnostic records, not the final application-service result envelope/API.
> - Unknown coefficients, default magnitudes, conversion constants, jurisdictional values, and professional acceptance remain `TBD`. Source: OPS-K-AGENT-1; OPS-K-DATA-1; OPS-K-IP-1.
> - Agent-authored deliverable evidence remains draft/proposal material until accepted by the human workflow authority. Source: OPS-K-AGENT-4.
>

### CLM-010 — References

> ##### References
>
> - `_CONTEXT.md`
> - `_REFERENCES.md`
> - `_DEPENDENCIES.md`
> - `MEMORY.md`
> - `core/loads/primitive_loads/README.md`
> - `core/loads/primitive_loads/src/lib.rs`
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
> - `docs/_Registers/Deliverables.csv` row DEL-05-01
> - `docs/_Registers/ScopeLedger.csv` row SOW-013
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-05-01
> - `docs/CONTRACT.md`
> - `docs/SPEC.md`
> - `docs/TYPES.md`

## Completion and Reliance Basis — Epistemology

### CLM-011 — Specification: DEL-05-01 Primitive load case engine

> #### Specification: DEL-05-01 Primitive load case engine
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-012 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-05-01-DECL-001`.
>

### CLM-013 — Scope

> ##### Scope
>
> This deliverable specifies the implemented evidence boundary for the primitive load case engine under `core/loads/primitive_loads`. The slice defines code-neutral primitive load categories, storage-neutral primitive load-case records, boundary quantity metadata, deterministic mechanics preparation, equivalent-static mechanics handling, lumped equivalent nodal conversion, straight-pipe axial-effect helpers, solver load-vector assembly helpers, and diagnostic bridge records for later result-envelope transport.
>
> This deliverable does not define code-specific load combinations, bundle coefficients or default magnitudes, set jurisdictional factors, implement rule-pack evaluation, implement final result-envelope/API integration, or claim engineering compliance. Mixed-category load-case algebra and user-defined combinations remain downstream DEL-05-02 scope.
>

### CLM-014 — Requirements

> ##### Requirements
>
> | ReqID | Requirement | Source |
> |---|---|---|
> | REQ-05-01-001 | The deliverable shall preserve primitive load category identity for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional loads. | SOW-013; DEL-05-01; `PrimitiveLoadCategory` |
> | REQ-05-01-002 | Primitive load records shall remain compatible with the 3D centerline/frame mechanics analysis boundary through explicit node, element, and support targets. | OPS-K-MECH-1; `LoadTarget`; `LoadDirection` |
> | REQ-05-01-003 | The primitive load engine shall not encode code-specific load combinations, allowables, factors, coefficients, procedure generators, default environmental values, or certification claims. | PKG-05 exclusion; OPS-K-DATA-1; OPS-K-IP-1; OPS-K-MECH-2 |
> | REQ-05-01-004 | Unit-bearing load inputs shall preserve explicit dimension intent and reject invalid or retired boundary metadata where implemented. | OPS-K-UNIT-1; `LoadDimension`; `CanonicalDimension`; `BoundaryMetadataError` |
> | REQ-05-01-005 | Missing or invalid solve-required load data shall become explicit findings/errors rather than silent defaults. | OPS-K-DATA-2; `FindingCode`; `BoundaryMetadataError` |
> | REQ-05-01-006 | Load validation and load-case assembly findings shall be convertible to diagnostic records carrying code, class, blocking severity, source, affected object, message, remediation, and provenance reference. | AB-00-06; `LoadDiagnosticRecord` |
> | REQ-05-01-007 | Primitive load-case records shall bind one primitive category to canonical model `LoadCase` metadata, provenance, payload reference, payload-hash reference, and sorted load IDs; mixed-category algebra remains DEL-05-02 scope. | SOW-013; SOW-014; DEL-05-02; `PrimitiveLoadCaseRecord`; `_DEPENDENCIES.md` |
> | REQ-05-01-008 | Wind, seismic, and occasional equivalent-static handling shall accept only explicit equivalent mechanics loads with caller-supplied basis/provenance refs unless a later sealed scope authorizes dynamic procedure generation. | SOW-013 note; `_CONTEXT.md`; `README.md`; `EquivalentStaticMechanicsBasis` |
> | REQ-05-01-009 | Deterministic primitive-load tests shall cover category representation, supported/unsupported target and dimension boundaries, boundary metadata, equivalent-static handling, load-case records, diagnostic bridge records, lumping, axial effects, and solver-vector assembly behavior before release use. | OPS-K-SOLVER-1; `src/lib.rs` tests |
> | REQ-05-01-010 | Primitive load inputs, load-case records, boundary quantity records, and diagnostic records shall retain provenance references sufficient to distinguish explicit user/lawful source inputs from unresolved `TBD` state. | OPS-K-DATA-1; OPS-K-DATA-2; AB-00-06 |
> | REQ-05-01-011 | Lumped equivalent nodal conversion shall remain limited to explicit translational/global `ForcePerLength` element loads with caller-supplied element span and node connectivity. | `prepare_lumped_nodal_loads`; `ElementLoadSpan` |
> | REQ-05-01-012 | Straight-pipe axial-effect helpers shall remain mechanics-only helpers for thermal axial force and pressure thrust from caller-supplied properties and shall not introduce pressure stress formulas or material/default catalogs. | `prepare_straight_pipe_axial_effects`; PKG-05 exclusions |
>

### CLM-015 — Standards

> ##### Standards
>
> No external protected standard text is introduced by this deliverable. Governing local standards are the project invariant catalog, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.
>

### CLM-016 — Verification

> ##### Verification
>
> | Requirement | Verification evidence / approach |
> |---|---|
> | REQ-05-01-001 | Unit tests and enum inspection confirm all eight primitive categories exist and map to primitive load-case kinds without invented coefficients. |
> | REQ-05-01-002 | Unit tests cover node, element, and support target preparation for accepted mechanics boundaries. |
> | REQ-05-01-003 | Documentation and crate boundary review confirm absence of code combinations, allowables, protected standard values, and professional/compliance claims. |
> | REQ-05-01-004 | Unit tests cover canonical dimension parsing, `force_per_length`, dimension mismatch, and retired alias rejection. |
> | REQ-05-01-005 | Unit tests cover missing targets, missing magnitudes, invalid targets, invalid dimensions, invalid directions, invalid spans/connectivity, invalid physical properties, non-finite inputs, and blocked outputs. |
> | REQ-05-01-006 | Unit tests cover conversion from primitive-load findings and load-case assembly findings into diagnostic records with provenance. |
> | REQ-05-01-007 | Unit tests cover load-case schema binding, schema load-type mapping, sorted load IDs, missing metadata, duplicate IDs, empty loads, and mixed-category rejection. |
> | REQ-05-01-008 | Unit tests cover wind/seismic/occasional explicit equivalent mechanics loads, required equivalent-static basis/provenance refs, and rejection of acceleration/dynamic-placeholder dimensions. |
> | REQ-05-01-009 | Current crate evidence is the focused 40-test primitive-load suite; broader release gates remain under project validation/review authority. |
> | REQ-05-01-010 | Unit tests cover missing provenance rejection for boundary records and diagnostic records. |
> | REQ-05-01-011 | Unit tests cover lumped nodal splitting, accepted translational directions, unsupported pressure/thermal/dynamic inputs, missing/invalid spans, invalid indices, and unchanged base `prepare_loads` behavior. |
> | REQ-05-01-012 | Unit tests cover thermal and pressure axial force preparation, finding fan-in, missing/invalid properties, non-finite magnitude, and overflowing computed force rejection. |
>

### CLM-017 — Remaining TBDs

> ##### Remaining TBDs
>
> - Canonical unit conversion constants and production unit conversion policy.
> - Final result-envelope/API integration and application-service command/query surface.
> - Production tolerance policy and release thresholds.
> - Wind/seismic dynamic treatment, occasional-event mapping, and any future lawful procedure generators.
> - Broader material/property sourcing policy, default prohibition enforcement, and human acceptance gates.
> - Professional reliance and code-compliance acceptance remain human/project authority decisions, not crate behavior.
>

### CLM-018 — Documentation

> ##### Documentation
>
> Required local deliverable artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `MEMORY.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`. Implementation evidence is in `core/loads/primitive_loads/README.md` and `core/loads/primitive_loads/src/lib.rs`.

- **AC-001** — The contract preserves the accepted primitive-load mechanics, unit, provenance, diagnostic, and rule-separation boundaries, including explicit missing-input findings, caller-supplied equivalent-static bases, and unresolved production policies without inventing code factors, combinations, allowables, defaults, or professional approval.

## Production and Verification Method — Praxeology

### CLM-019 — Procedure: DEL-05-01 Primitive load case engine

> #### Procedure: DEL-05-01 Primitive load case engine
>

### CLM-020 — Purpose

> ##### Purpose
>
> Describe bounded steps for maintaining and verifying the primitive load case engine evidence without introducing code-specific load combinations, protected standards data, final result-envelope/API behavior, or compliance claims.
>

### CLM-021 — Prerequisites

> ##### Prerequisites
>
> - Sealed deliverable scope for DEL-05-01 and SOW-013.
> - Applicable architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
> - Invariants OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-AGENT-1 through OPS-K-AGENT-4, and OPS-K-IP-1.
> - Current implementation evidence in `core/loads/primitive_loads/README.md` and `core/loads/primitive_loads/src/lib.rs`.
> - Explicit human approval before any future scope adds code-specific combinations, coefficients, allowables, default environmental factors, dynamic procedure generation, final compliance claims, or professional reliance claims.
>

### CLM-022 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity, package, scope item, objective, architecture basis, and current lifecycle state from `_CONTEXT.md` and `_STATUS.md`.
> 2. Read the deliverable-local truth set before edits: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
> 3. Read the implementation evidence in `core/loads/primitive_loads/README.md` and `core/loads/primitive_loads/src/lib.rs`.
> 4. Confirm all SOW-013 primitive categories remain represented: weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional.
> 5. Confirm primitive mechanics boundaries: nodal, element-uniform, imposed-displacement, equivalent-static preparation, lumped equivalent nodal conversion, straight-pipe axial effects, and solver load-vector assembly are documented only to the extent implemented.
> 6. Preserve the boundary between primitive load definitions/load-case records in DEL-05-01 and mixed-category load-case algebra/user combinations in DEL-05-02.
> 7. Preserve the boundary between mechanics solving, diagnostic record transport, rule/compliance evaluation, and human professional judgment.
> 8. Keep unknown coefficients, default magnitudes, conversion constants, dynamic treatment, property/default sourcing, production tolerance policy, final API integration, release thresholds, and professional reliance as `TBD` unless supplied by lawful source material and sealed scope.
> 9. Run focused validation after documentation or crate-affecting changes: `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` and `git diff --check` scoped to touched files where possible.
> 10. Record durable closeout evidence in `MEMORY.md` and `_run_records/TASK_RUN_*.md` without editing `_STATUS.md`, dependency registers, DAG artifacts, schemas, repo-level governance, or core code unless explicitly authorized by a later brief.
>

### CLM-023 — Verification

> ##### Verification
>
> | Check | Expected evidence |
> |---|---|
> | Scope check | Documents reference DEL-05-01, PKG-05, SOW-013, OBJ-003, and the current architecture basis consistently. |
> | Category check | All eight primitive load categories appear in implementation-aligned evidence. |
> | Boundary check | No code-specific load combinations, allowables, factors, coefficients, default magnitudes, dynamic procedure generators, or certification claims are introduced. |
> | Unit/boundary metadata check | Documents preserve explicit unit/dimension/provenance/hash-ref boundaries and do not claim conversion constants. |
> | Load-case record check | Documents describe single-category primitive load-case records and keep mixed-category algebra in DEL-05-02. |
> | Diagnostic bridge check | Documents describe local diagnostic bridge records without claiming a shared enum or final result-envelope/API implementation. |
> | Equivalent-static check | Documents limit wind, seismic, and occasional equivalent-static handling to explicit mechanics loads with caller-supplied basis/provenance refs and no dynamic procedure generation. |
> | Lumping check | Documents limit lumping to explicit translational/global `ForcePerLength` element loads with caller-supplied spans/connectivity. |
> | Axial-effect check | Documents limit axial effects to thermal and pressure mechanics helpers from caller-supplied properties without code stress/compliance claims. |
> | Test check | `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` passes or any failure is recorded with boundaries. |
> | Diff hygiene check | `git diff --check` passes for touched files or any failure is recorded. |
> | Status check | `_STATUS.md` remains unchanged unless a later brief explicitly authorizes a lifecycle update. |
>

### CLM-024 — Records

> ##### Records
>
> - Four-document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
> - Working memory: `MEMORY.md`
> - Semantic evidence: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`
> - Dependency evidence: `Dependencies.csv`, `_DEPENDENCIES.md`
> - Implementation evidence: `core/loads/primitive_loads/README.md`, `core/loads/primitive_loads/src/lib.rs`
> - Run evidence: `_run_records/`
> - Lifecycle evidence: `_STATUS.md`
>

### CLM-025 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The primitive-load slice now includes explicit loads and the DEC-068 equivalent-static wind, seismic, and occasional generators. Dynamic loading and code-prescribed generation are outside this bounded slice unless separately implemented and evidenced.

- **VER-001** — Validate the contract and review source parity, category and record coverage, unit/provenance metadata, preparation and solver-vector interfaces, deterministic findings, protected-content and professional boundaries, and every surviving governed residual.

## Governing Values and Decisions — Axiology

### CLM-026 — Guidance: DEL-05-01 Primitive load case engine

> #### Guidance: DEL-05-01 Primitive load case engine
>

### CLM-027 — Purpose

> ##### Purpose
>
> This deliverable frames primitive load categories as explicit mechanics inputs for the OpenPipeStress solver boundary. The current implementation evidence is the bounded `core/loads/primitive_loads` crate, which preserves primitive category identity, deterministic findings, boundary metadata, load-case record shape, diagnostic bridge records, equivalent-static mechanics handling, lumped equivalent nodal conversion, and straight-pipe axial-effect helpers without adding code-specific combinations or compliance logic.
>

### CLM-028 — Principles

> ##### Principles
>
> - Treat primitive load categories as mechanics-boundary records and validation surfaces, not as a finished load-combination or rule-pack engine.
> - Keep primitive category, primitive load, primitive load case, load-case algebra, code combination, and rule-pack evaluation as separate concepts.
> - Preserve unit/dimension intent and provenance references for load quantities, boundary records, load-case records, and diagnostic records.
> - Use explicit findings/errors for missing targets, magnitudes, spans, properties, provenance, invalid dimensions, invalid directions, invalid topology, and non-finite values.
> - Keep wind, seismic, and occasional loads as explicit equivalent mechanics inputs with explicit basis/provenance refs unless a later sealed scope authorizes dynamic methods or lawful procedure generation.
> - Avoid copying or deriving protected standard content, tables, examples, coefficients, jurisdictional factors, or proprietary defaults.
>

### CLM-029 — Considerations

> ##### Considerations
>
> | Topic | Guidance |
> |---|---|
> | Weight | Use explicit mechanics quantities such as element `ForcePerLength`; any mass-source, gravity-vector, density, and conversion policy must come from lawful upstream data or future sealed scope. |
> | Pressure | Pressure can be carried as an element primitive load and interpreted for straight-pipe pressure thrust only with caller-supplied properties; pressure stress formulas and code stress categories belong elsewhere. |
> | Thermal | Thermal can be carried as element temperature change and interpreted for straight-pipe axial force only with caller-supplied material/section properties; reference temperature and material provenance remain open policy. |
> | Displacement | Imposed displacement is a support-target mechanics input; support/restraint behavior remains an interface with the owning solver/support deliverables. |
> | Hydrotest | Hydrotest is represented as a primitive mechanics category; do not imply hydrotest procedure defaults, code requirements, or test-fluid assumptions without sourced input. |
> | Wind/seismic/occasional | Current implemented behavior is explicit equivalent nodal force/moment or element `ForcePerLength` where supported; `prepare_equivalent_static_loads` requires `EquivalentStaticMechanicsBasis`, and acceleration/dynamic placeholders are not treated as dynamic procedure support. |
> | Load-case records | Use `PrimitiveLoadCaseRecord` as a storage-neutral single-category boundary record. Mixed-category algebra and user-defined combinations remain DEL-05-02. |
> | Boundary metadata | Boundary records should carry schema binding, target/basis refs, JCS payload refs, payload-hash refs, unit metadata, and provenance refs; `TBD` is rejected where a concrete boundary ref is required. |
> | Diagnostics | Diagnostic bridge records preserve local finding codes and local classes for result-envelope handoff; they do not define a shared diagnostic enum or final application API. |
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Current position |
> |---|---|
> | Category coverage vs. behavior breadth | Cover all SOW-013 primitive categories, but only implement bounded mechanics preparation and validation behavior evidenced in `core/loads/primitive_loads`. |
> | Public defaults vs. user-supplied data | Do not bundle protected, jurisdictional, or catalog values; require explicit lawful/user-supplied inputs and provenance references. |
> | Static equivalent inputs vs. dynamic methods | Wind, seismic, and occasional behavior remains equivalent mechanics input in this slice with caller-supplied basis/provenance refs; dynamic procedure generation remains `TBD`. |
> | Primitive load case vs. load algebra | The primitive record binds one category and sorted load IDs; mixed cases, load algebra, and combinations belong to DEL-05-02. |
> | Diagnostic bridge vs. final envelope | The crate can produce storage-neutral diagnostic records; final result-envelope/API integration remains downstream. |
> | Axial helper formulas vs. code stress evaluation | Thermal and pressure axial-effect helpers compute mechanics forces from explicit properties only; they are not pressure stress formulas, code checks, or allowables. |
>

### CLM-031 — Evidence Use

> ##### Evidence Use
>
> - Use `core/loads/primitive_loads/README.md` for the current crate boundary statement.
> - Use `core/loads/primitive_loads/src/lib.rs` for implemented API names and unit-test evidence.
> - Use `MEMORY.md` for historical tranche evidence, preserved boundaries, and remaining TBDs.
> - Use `_DEPENDENCIES.md` for the active downstream interface to DEL-05-02.
> - Do not promote lifecycle status or claim release/professional acceptance from these documents alone.
>

### CLM-032 — Examples

> ##### Examples
>
> No normative numeric load examples, code-specific combinations, coefficient examples, or jurisdictional examples are introduced here. Test values in `src/lib.rs` are invented mechanics fixtures for unit-test coverage only and are not design defaults.
>

### CLM-033 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict detected between the deliverable-local truth set and current primitive-load crate evidence. | N/A | N/A | N/A | N/A | N/A |
>

### CLM-034 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The primitive-load slice now includes explicit loads and the DEC-068 equivalent-static wind, seismic, and occasional generators. Dynamic loading and code-prescribed generation are outside this bounded slice unless separately implemented and evidenced.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-013 OBJ-003 | CLM-011 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
