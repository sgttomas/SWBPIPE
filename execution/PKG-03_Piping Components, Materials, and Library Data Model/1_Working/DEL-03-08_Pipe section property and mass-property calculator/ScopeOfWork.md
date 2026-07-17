---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-08
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-051, SOW-018]
package_objective_refs: [OBJ-004, OBJ-012]
---

# Scope of Work — DEL-03-08

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-08` in service of project scope [SOW-051, SOW-018] and package objectives [OBJ-004, OBJ-012].

- **OUT-001** — A pipe section-property and mass-property calculator contract covering explicit dimensional and material inputs, units, provenance, section and mass outputs, diagnostics, protected-data boundaries, and solver handoff limits is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-03-08 Pipe section property and mass-property calculator

> #### Datasheet: DEL-03-08 Pipe section property and mass-property calculator
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-03-08 |
> | Package ID | PKG-03 |
> | Package | Piping Components, Materials, and Library Data Model |
> | Deliverable type | BACKEND_FEATURE_SLICE |
> | Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 |
> | Scope items | SOW-051, SOW-018 |
> | Objectives | OBJ-004, OBJ-012 |
> | Context envelope | M |
> | Lifecycle state | `IN_PROGRESS`; implementation evidence exists, but this is not ISSUED, accepted, released, or approved for professional reliance |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value |
> |---|---|
> | Calculator purpose | Calculate pipe section and mass-property outputs from user-entered dimensions and material data with unit checks. |
> | Permitted data source posture | User-entered or lawfully imported private/project data only. |
> | Public bundled data posture | No protected pipe dimensional tables, material allowables, contents defaults, insulation defaults, corrosion allowances, or proprietary component data. |
> | Unit posture | Inputs and outputs carry explicit unit and dimension metadata. Mixed units are rejected; approved conversion constants and public unit catalog remain `TBD`. |
> | Provenance posture | Calculator inputs require provenance metadata, and derived outputs state that they were calculated from user-entered dimensions. Exact private-library record linkage remains `TBD`. |
> | Solver boundary | This deliverable prepares section and mass properties; it does not implement the global solver, code compliance, or rule-pack evaluation. |
>

### CLM-004 — Conditions

> ##### Conditions
>
> Implementation evidence exists in `core/section_properties/calculator.py` for user-entered pipe outside diameter, wall thickness, optional corrosion allowance, optional insulation thickness, and optional material, contents, and insulation density inputs. The calculator derives inside diameter, metal/cross-section area, second moment of area, section modulus, torsional constant, contents volume per length, and optional mass-per-length contributors when matching density inputs are supplied.
>
> The implementation rejects missing required dimensions, missing provenance, incompatible dimensions, mixed units, and non-physical geometry through blocking diagnostics. It does not provide pipe schedule tables, material defaults, unit conversion constants, protected dimensional tables, contents defaults, insulation defaults, corrosion defaults, SIF/flexibility values, code-specific values, or proprietary catalog values.
>
> Exact allowed public unit catalog, conversion constants, accepted schema field placement, dependency satisfaction, fixture-value policy, public source catalog, lifecycle disposition, and downstream solver/GUI/report integration remain `TBD`.
>
> Mass-property tests use invented synthetic values in `tests/test_section_properties.py`. No protected published pipe tables, material tables, or vendor proprietary data may be encoded as test data.
>

### CLM-005 — Construction

> ##### Construction
>
> | Construction item | Status |
> |---|---|
> | Section property calculator artifact | Implemented at `core/section_properties/calculator.py`. |
> | Calculator README | Implemented at `core/section_properties/README.md`. |
> | Mass property tests | Implemented in `tests/test_section_properties.py` using invented synthetic values; formal fixture-value policy remains `TBD`. |
> | Schema-like input mapping | Implemented through `quantity_from_mapping`; accepted schema field placement and dependency satisfaction remain `TBD`. |
> | Diagnostic envelope fields | Implemented with code, diagnostic class, severity, field, source, affected object, message, remediation, and provenance; downstream result-envelope mapping remains `TBD`. |
> | Private library linkage | Provenance is required at the calculator boundary; exact private-library record linkage remains `TBD`. |
>

### CLM-006 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, objectives, artifact list, and architecture basis injection.
> - `_REFERENCES.md` for governing local references.
> - `docs/_Registers/Deliverables.csv` row DEL-03-08.
> - `docs/_Registers/ScopeLedger.csv` rows SOW-051 and SOW-018.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-03-08.
> - `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-SOLVER-1, and OPS-K-AGENT-1..4.
> - `core/section_properties/calculator.py` and `core/section_properties/README.md` for implemented calculator behavior.
> - `tests/test_section_properties.py` for current unit and negative-test evidence.
> - `fixtures/results/invented/tp_phys_015_section_property_stress_evidence_envelope.json` for invented downstream section-property evidence context.
> - `Review_Findings.csv` for human-gated technical finding status; statuses remain `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`.
>

### CLM-007 — Open Questions

> ##### Open Questions
>
> Current implementation-form note: `PipeSectionInput` accepts an optional
> dimensional `mill_tolerance`, and the calculator uses `wall_thickness -
> corrosion_allowance - mill_tolerance` as effective wall. The input is
> provenance-stamped and validated like the other dimensional slots; a fractional
> or catalog-derived tolerance policy remains `TBD`.
>
> | Question | Needed from |
> |---|---|
> | Which approved unit catalog and conversion constants, if any, may this calculator call? | DEL-02-02 / human architecture ruling |
> | Which schema record owns calculator inputs, outputs, and library linkage? | DEL-03-02 / schema owner |
> | What downstream result-envelope mapping is required before solver, persistence, GUI, or report integration? | PKG-02 / PKG-08 / human architecture ruling |
> | What public source catalog and fixture-value policy is acceptable for section and mass-property tests? | Validation/QA owner |
> | When may the technically addressed review findings move beyond human-gated `TBD` disposition? | Human project authority |
>

### CLM-008 — D-41 R5 T2B PDU-047 Evidence State

> ##### D-41 R5 T2B PDU-047 Evidence State
>
> | Path | Inputs | Bound outputs |
> |---|---|---|
> | Production `calculate_pipe_section_properties` → TP-PHYS-015 formal oracle → governed result envelope | Invented OD 2.0 m; wall 0.25 m | Area `m^2`/`area`; section modulus `m^3`/`section_modulus`; torsional constant `m^4`/`second_moment_area`, checked with existing witness tolerances. |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-03-08 Pipe section property and mass-property calculator

> #### Specification: DEL-03-08 Pipe section property and mass-property calculator
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies the bounded backend feature slice that calculates pipe section and mass properties from user-entered dimensions and material data with unit checks. It also records the private-library and provenance constraints that apply when those inputs are sourced from pipe section, component, or material libraries.
>
> Current implementation evidence is `core/section_properties/calculator.py`, `core/section_properties/README.md`, and `tests/test_section_properties.py`. This evidence reconciliation does not edit code, repo-level schemas, fixtures, tests, lifecycle records, dependency registers, review dispositions, or DAG/coordination files, and it does not introduce protected dimensional, material, contents, insulation, corrosion, code, or vendor data.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source basis | Verification hook |
> |---|---|---|---|
> | DEL-03-08-RQ-001 | The calculator shall accept only explicit user-entered or lawfully imported private/project input values for dimensions, material density, contents, insulation, corrosion basis, and dimensional mill tolerance. | SOW-051; OPS-K-DATA-1; OPS-K-IP-1 | `tests/test_section_properties.py` uses explicit invented inputs; public source catalog and fixture-value policy remain `TBD`. |
> | DEL-03-08-RQ-002 | The calculator shall reject or flag missing solve-required values instead of applying silent defaults. | OPS-K-DATA-2; SOW-051 | `tests/test_section_properties.py` covers missing wall, missing provenance, mixed units, and invalid geometry. Optional mass-contributor requiredness policy remains `TBD`. |
> | DEL-03-08-RQ-003 | All input quantities, intermediate calculations, and outputs shall be unit-aware and dimensionally checked. | SOW-051; OPS-K-UNIT-1; OBJ-012 | Tests assert canonical output dimensions and rejection of mixed units. Approved unit conversion constants and dependency satisfaction remain `TBD`. |
> | DEL-03-08-RQ-004 | Library-sourced inputs shall carry provenance and redistribution status through calculator schema hooks. | SOW-018; OPS-K-DATA-3 | `Quantity` and `quantity_from_mapping` require provenance. Accepted schema field placement and private-library record linkage remain `TBD`. |
> | DEL-03-08-RQ-005 | Public repository fixtures shall not encode protected pipe dimensional tables, protected material data, proprietary commercial data, or paraphrased protected tables. | OPS-K-IP-1; OPS-K-IP-3 | Protected-content review and fixture review. |
> | DEL-03-08-RQ-006 | Calculator outputs intended for solver consumption shall remain code-neutral and shall not claim certification, code compliance, or professional approval (PRD §21.2). | OPS-K-AGENT-4; AB-00-06 | Review of diagnostics/result envelopes and report-facing text. |
> | DEL-03-08-RQ-007 | The calculator shall be isolated from global solver implementation and rule-pack compliance logic. | PKG-03 exclusion; OPS-K-SOLVER-1 | Module boundary review confirms `core/section_properties` is outside global solver and rule-pack logic; downstream solver integration policy remains `TBD`. |
>

### CLM-012 — Standards

> ##### Standards
>
> No protected standard text or table is available in this deliverable-local context. Any standard, code, or vendor basis must be introduced only as a licensed/private input or as a non-protected pointer with provenance. Clause-level requirements are `TBD`.
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification area | Current evidence and remaining gate |
> |---|---|
> | Unit safety | Tests demonstrate canonical dimensions and mixed-unit rejection without hidden conversion. Approved conversion support remains `TBD`. |
> | Missing input behavior | Tests demonstrate explicit blocking findings for missing required wall thickness and missing provenance; optional contributor policy remains `TBD`. |
> | Provenance | Tests demonstrate provenance requirements at quantity construction/mapping boundaries and derived-output provenance. Library record linkage remains `TBD`. |
> | IP/data boundary | Tests use invented synthetic values; formal source catalog and fixture-value policy remain `TBD`. |
> | Solver boundary | Calculator outputs remain data/service outputs, not solver certification or code-compliance claims. Downstream integration remains `TBD`. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Current implementation artifacts are:
>
> - `core/section_properties/calculator.py`;
> - `core/section_properties/README.md`;
> - `tests/test_section_properties.py`;
> - schema-like quantity mapping through `quantity_from_mapping`.
>
> The current input form includes optional dimensional `mill_tolerance`. When
> present, effective wall is `wall_thickness - corrosion_allowance -
> mill_tolerance`; absence means no reduction and is not a hidden zero default.
> Negative tolerance or a corrosion-plus-tolerance reduction that consumes the
> wall is blocking. Fractional/catalog conversion policy remains `TBD` and no
> catalog value or default is supplied.
>
> Accepted schema ownership, dependency satisfaction, downstream result-envelope mapping, lifecycle disposition, and human disposition for review findings remain `TBD`.
>

### CLM-015 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | PKG03-DEL-03-08-PKG02-001 | Dimension vocabulary compatibility finding has technical evidence in calculator/tests but awaits human disposition. | `Review_Findings.csv`; `tests/test_section_properties.py` | `TBD` |
> | PKG03-DEL-03-08-PKG02-002 | Input provenance finding has technical evidence in calculator/tests but awaits human disposition. | `Review_Findings.csv`; `tests/test_section_properties.py` | `TBD` |
> | PKG03-DEL-03-08-PKG02-003 | Diagnostic envelope field finding has technical evidence in calculator/tests but awaits human disposition. | `Review_Findings.csv`; `tests/test_section_properties.py` | `TBD` |
>

### CLM-016 — D-41 R5 T2B PDU-047 Evidence Binding (2026-07-12)

> ##### D-41 R5 T2B PDU-047 Evidence Binding (2026-07-12)
>
> `tests/test_calculation_witness.py` now executes `calculate_pipe_section_properties` with the existing rights-safe TP-PHYS-015 outside diameter and wall thickness, then compares the produced area, section modulus, and torsional constant to both the formal oracle and `tp_phys_015_section_property_stress_evidence_envelope.json` using the witness's existing tolerances. It also asserts the emitted units and dimensions. This is bounded validation evidence, not an engineering-validation or lifecycle disposition.

- **AC-001** — The contract preserves accepted calculator requirements and boundaries, including user-entered or lawfully imported inputs, dimensional checks, no silent defaults, invented fixtures, unresolved conversion, schema, contributor, dependency, and human-review decisions. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-03-08 Pipe section property and mass-property calculator

> #### Procedure: DEL-03-08 Pipe section property and mass-property calculator
>

### CLM-018 — Purpose

> ##### Purpose
>
> Define the procedure for reconciling and maintaining implementation evidence for the pipe section property and mass-property calculator without introducing protected data or unapproved repo-level changes.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status |
> |---|---|
> | Sealed deliverable context for DEL-03-08 | Available in `_CONTEXT.md`. |
> | Unit-system and dimensional-analysis contract | Calculator requires explicit unit/dimension metadata and rejects mixed units; approved conversion API/catalog remains `TBD`. |
> | Pipe section/component library schema contract | Schema-like mapping exists through `quantity_from_mapping`; accepted owner fields and dependency satisfaction remain `TBD`. |
> | Material library provenance model | Calculator quantities require provenance; private-library record linkage remains `TBD`. |
> | Diagnostic/result envelope contract | Calculator diagnostics carry class, source, affected object, and provenance; downstream result-envelope mapping remains `TBD`. |
> | Synthetic or cleared fixture policy | Current tests use invented synthetic values; formal source catalog and fixture-value policy remain `TBD`. |
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the task is sealed to DEL-03-08 and that write scope is limited to the authorized deliverable folder or separately authorized implementation paths.
> 2. Read current implementation evidence in `core/section_properties/calculator.py`, `core/section_properties/README.md`, and `tests/test_section_properties.py`.
> 3. Confirm calculator inputs remain explicit user-entered or lawfully imported values. Do not introduce bundled public dimensional, material, contents, insulation, or corrosion defaults.
> 4. Confirm validation behavior remains explicit for missing required values, missing provenance, incompatible dimensions, mixed units, non-positive required values, and non-physical geometry.
> 5. Confirm output shape preserves magnitude, unit, dimension, and provenance for section properties and mass properties.
> 6. Confirm tests use synthetic or cleared values only and do not encode protected/reference-table content.
> 7. Run `python3 -m pytest tests/test_section_properties.py` when feasible.
> 8. Run a stale-language search over `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`; reconcile implementation evidence while preserving unresolved `TBD` items.
> 9. Record unresolved policy, source catalog, fixture-value, dependency satisfaction, human disposition, lifecycle, and downstream integration inputs as `TBD` and route them to the responsible schema/unit/diagnostic/human owner.
>

### CLM-021 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Protected data check | No protected pipe tables, material tables, code tables, copied formulas, or proprietary fixtures are introduced. |
> | Unit check | Inputs and outputs carry explicit units/dimensions; mixed units are rejected until conversion support is approved. |
> | Missing value check | Missing required values produce explicit blocking diagnostics, not defaults. |
> | Provenance check | Inputs require provenance and outputs preserve calculated provenance; exact library linkage remains `TBD`. |
> | Review-finding check | Technical evidence remains aligned with `TECHNICALLY_ADDRESSED_PENDING_HUMAN`; `HumanDisposition` remains `TBD` until human action. |
> | Boundary check | Calculator remains outside global solver and rule-pack compliance logic. |
>

### CLM-022 — Records

> ##### Records
>
> - Updated four-document kit in this folder.
> - `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` historical semantic artifacts.
> - `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
> - `_run_records/TASK_RUN_*.md` records for each TASK sequence step.
> - Current evidence reconciliation run record: `_run_records/TASK_RUN_2026-06-05_DEL-03-08_evidence-reconciliation.md`.
>

### CLM-023 — D-41 R5 T2B PDU-047 Check

> ##### D-41 R5 T2B PDU-047 Check
>
> Run `pytest -p no:cacheprovider tests/test_section_properties.py tests/test_calculation_witness.py`. Acceptance of this bounded witness requires the production calculator values, units, and dimensions to agree with the existing formal oracle and governed result envelope within the already-recorded witness tolerances. Do not substitute a re-derived test-only formula or select a new tolerance.

- **VER-001** — Validate the contract and review source parity, section and mass-property coverage, dimensional consistency, missing-input behavior, provenance, protected-content controls, calculator and solver separation, bounded witness evidence, and unresolved human-review items.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-03-08 Pipe section property and mass-property calculator

> #### Guidance: DEL-03-08 Pipe section property and mass-property calculator
>

### CLM-025 — Purpose

> ##### Purpose
>
> This deliverable provides a bounded, unit-aware calculation surface for pipe section and mass properties while preserving OpenPipeStress data-boundary rules. The key distinction is that calculation from explicit user-entered values is allowed, but bundled protected pipe tables, material tables, proprietary data, hidden unit conversion, and silent defaults are not.
>

### CLM-026 — Principles

> ##### Principles
>
> Use user-entered dimensions and material data as the source of truth. If a required value is missing, incompatible, unprovenanced, or not licensed for redistribution, record a blocking diagnostic rather than substituting a default.
>
> Keep the calculator code-neutral. Section and mass properties can support solver and reporting workflows; acceptance and professional judgment remain with the responsible engineer.
>
> Keep unit behavior explicit. The calculator currently requires matching units at the calculation boundary and rejects mixed units; approved unit conversion support remains a separate `TBD`.
>
> Keep provenance attached. Calculator inputs require provenance metadata, and calculated outputs identify that they were derived from user-entered dimensions. Exact private-library record linkage and downstream envelope mapping remain `TBD`.
>

### CLM-027 — Considerations

> ##### Considerations
>
> The most important implementation risk is accidentally turning public fixtures or defaults into a protected data table. Synthetic examples may be used for tests, but they must not be copied from protected standards, commercial catalogs, or proprietary project records.
>
> The second major risk is ambiguity about optional contributors such as contents, insulation, and corrosion basis. Current code treats these contributors as optional explicit inputs; downstream policy still must define when each contributor is required, optional, explicitly not applicable, or pending user input.
>
> The third risk is unit drift between schema, UI, calculator, and solver-facing outputs. The calculator emits canonical dimension labels in current tests, but accepted schema ownership, dependency satisfaction, and downstream result-envelope integration remain `TBD`.
>

### CLM-028 — Trade-offs

> ##### Trade-offs
>
> | Decision area | Conservative posture |
> |---|---|
> | Public default data | Do not ship defaults where provenance or redistribution rights are unclear. |
> | Test fixtures | Use synthetic values designed for dimensional behavior, not copied industry tables. |
> | Calculator scope | Calculate section/mass properties only; leave code checks and solver verification to their packages. |
> | Schema timing | Treat hook names and exact field placement as `TBD` until schema contracts are accepted. |
> | Review findings | Treat technical evidence as addressed pending human disposition; do not mark findings resolved without human action. |
>

### CLM-029 — Examples

> ##### Examples
>
> Current numerical examples live in `tests/test_section_properties.py` and use invented synthetic values to verify section properties, mass-per-length contributors, mixed-unit rejection, missing provenance, and invalid geometry. Additional examples must remain synthetic or clearly licensed/user-provided and must include units, provenance posture, and expected diagnostic behavior. Formal fixture-value policy remains `TBD`.
>

### CLM-030 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Current disposition | Human ruling |
> |---|---|---|---|
> | PKG03-DEL-03-08-PKG02-001 | Dimension-vocabulary compatibility has technical evidence but no human disposition. | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | `TBD` |
> | PKG03-DEL-03-08-PKG02-002 | Input-provenance handling has technical evidence but no human disposition. | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | `TBD` |
> | PKG03-DEL-03-08-PKG02-003 | Diagnostic-envelope fields have technical evidence but no human disposition. | `TECHNICALLY_ADDRESSED_PENDING_HUMAN` | `TBD` |
>

### CLM-031 — P3 Enrichment Notes

> ##### P3 Enrichment Notes
>
> Semantic lensing identified that diagnostic taxonomy, schema hook names, and optional mass contributors needed explicit treatment. The calculator now emits blocking diagnostics with diagnostic class, source, affected object, and provenance, and tests cover optional mass contributors when explicit densities are supplied. Accepted schema hook names, optional contributor requiredness policy, source catalog, fixture-value policy, dependency satisfaction, lifecycle disposition, human disposition, and downstream integration remain `TBD`.
>

### CLM-032 — D-41 R5 T2B PDU-047 Boundary

> ##### D-41 R5 T2B PDU-047 Boundary
>
> The TP-PHYS-015 binding is deliberately narrow: it demonstrates that selected existing rights-safe section-property oracle values traverse the actual Python calculator and an existing governed result envelope. It does not approve catalogs, conversions, thresholds, broader mechanics suitability, or professional reliance.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-051 SOW-018 OBJ-004 OBJ-012 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
