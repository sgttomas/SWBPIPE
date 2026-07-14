---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-03
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-007]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-03` in service of project scope [SOW-007] and package objectives [OBJ-004].

- **OUT-001** — A bend-and-elbow component-model contract defining geometry, section, flexibility, stiffness, mass, unit, provenance, and completeness fields is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-03-03 Bend and elbow component model fields

> #### Datasheet: DEL-03-03 Bend and elbow component model fields
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-03-03 |
> | Package ID | PKG-03 |
> | Package | Piping Components, Materials, and Library Data Model |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope Item | SOW-007 |
> | Objective | OBJ-004 |
> | Decomposition Basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 |
> | Evidence Artifacts | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py` |
>

### CLM-003 — Attributes

> ##### Attributes
>
> This deliverable concerns data-model fields for bends and elbows. The current repository evidence supports:
>
> | Attribute Group | Field Intent | Source |
> |---|---|---|
> | Component identity | Distinguish bend/elbow records from other component families. | DEL-03-03 context; SOW-007 |
> | Bend geometry | Store user-entered centerline bend geometry through schema field kinds including `bend_centerline_radius`, `bend_included_angle`, and `bend_plane_orientation`. | SOW-007 |
> | SIF inputs | Store user-entered stress intensification factors without bundled protected tables or formulas. | SOW-007; OPS-K-IP-1; OPS-K-DATA-1; OPS-K-DATA-3 |
> | Flexibility inputs | Store user-entered flexibility factors without bundled protected tables or formulas. | SOW-007; OPS-K-IP-1; OPS-K-DATA-1; OPS-K-DATA-3 |
> | Provenance/source metadata | Record source, provenance, redistribution status where applicable, and evidence references for user-supplied or imported data. | OPS-K-DATA-3; AB-00-04; AB-00-07 |
> | Validation state | Represent missing or unresolved required inputs explicitly rather than silently defaulting values. | OPS-K-DATA-2; AB-00-06 |
>

### CLM-004 — Conditions

> ##### Conditions
>
> - The public repository must not contain protected SIF or flexibility tables, protected standards text, copied examples, or code-derived formulas.
> - Code-specific values for SIFs, flexibility factors, bend geometry, and related rule-pack inputs are user-supplied or lawfully imported private data.
> - Units must be explicit and dimensionally checked for geometry and any numeric factors that participate in calculation or export.
> - Diagnostics must identify missing solve-required or rule-check-required bend/elbow values as findings.
> - The active evidence is schema/fixture/test evidence only; it does not close dependency satisfaction or human review dispositions.
>

### CLM-005 — Construction

> ##### Construction
>
> Current evidence categories:
>
> | Category | Draft Field Concept | Status |
> |---|---|---|
> | Identity | `bend` and `elbow` are explicit `ComponentType` values, aligned with `schemas/model.schema.yaml`. | Implemented evidence |
> | Geometry | Bend geometry slots include centerline radius, included angle, tangent/orientation fields, and geometry source references. | Implemented evidence |
> | Factors | User SIF and flexibility factor slots are present as user/private inputs with no public code-specific values. | Implemented evidence |
> | Source metadata | Component fields require provenance, redistribution status, contributor certification, and review status. | Implemented evidence |
> | Validation | Missing bend geometry/rule inputs are represented through `BEND_GEOMETRY_INCOMPLETE` and `BEND_RULE_INPUT_MISSING`. | Implemented evidence |
> | Persistence | Strict component fixture validates as a standalone `component.schema.yaml` instance; combined fixture points to the strict component fixture. | Implemented evidence |
>

### CLM-006 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, objective, scope, and architecture-basis injection.
> - `docs/_Registers/Deliverables.csv` row `DEL-03-03`.
> - `docs/_Registers/ScopeLedger.csv` row `SOW-007`.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 rows for `DEL-03-03`, `SOW-007`, `OBJ-004`, `PKG-03`, and applicable architecture basis IDs.
> - `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1..4.

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-03-03 Bend and elbow component model fields

> #### Specification: DEL-03-03 Bend and elbow component model fields
>

### CLM-008 — Scope

> ##### Scope
>
> This deliverable specifies the bend/elbow component model slice as currently evidenced by component schema fields, strict component fixture records, and validation tests. The implemented evidence is limited to bend/elbow geometry slots, user-entered SIF/flexibility slots, source metadata, completeness diagnostics, and model-enum compatibility.
>
> Out of scope:
>
> - Solver implementation.
> - Rule-pack evaluator implementation.
> - B31J or other protected SIF/flexibility tables, formulas, examples, or paraphrased derivations.
> - Public bundled engineering values or default code data.
> - Certification, approval, sealing, or compliance claims.
>

### CLM-009 — Requirements

> ##### Requirements
>
> | ID | Requirement | Basis | Verification Hook |
> |---|---|---|---|
> | R01 | The model must represent bend/elbow component identity separately from other component families. | DEL-03-03; PKG-03 | Validation tests cover bend/elbow record identity. |
> | R02 | The model must provide fields for user-entered bend geometry. | SOW-007 | Schema/model tests confirm required geometry slots and unit metadata. |
> | R03 | The model must provide fields for user-entered SIFs. | SOW-007 | Tests confirm values are accepted only as user/import inputs with provenance. |
> | R04 | The model must provide fields for user-entered flexibility factors. | SOW-007 | Tests confirm values are accepted only as user/import inputs with provenance. |
> | R05 | The model must carry source/provenance metadata for SIFs, flexibility factors, and imported or library-sourced bend data. | OPS-K-DATA-3; AB-00-04; AB-00-07 | Schema tests require provenance/source fields on component records and field slots. |
> | R06 | The implementation must not bundle or encode protected SIF/flexibility tables, copied formulas, or protected examples. | OPS-K-IP-1; OPS-K-IP-3; SOW-007 note | Protected-content gate and review checklist reject bundled protected content. |
> | R07 | Missing solve-required or rule-check-required values must produce explicit diagnostics, not silent defaults. | OPS-K-DATA-2; AB-00-06 | Validation tests assert missing values become findings. |
> | R08 | Numeric geometry inputs must be unit-aware and dimensionally checked. | OPS-K-UNIT-1; AB-00-04 | Unit tests cover dimension validation and serialization. |
> | R09 | Public rule-pack or example data associated with this model must use invented non-code values and non-engineering notices. | OPS-K-RULE-1 | Example/test fixture review confirms no code-derived values. |
> | R10 | The model must remain compatible with the 3D centerline/frame analysis model, with local FEA handoff only as a downstream path. | OPS-K-MECH-1 | Model documentation and tests avoid shell/solid assumptions in the core component record. |
> | R11 | Adapters/plugins must not bypass unit, provenance, redistribution, validation, or diagnostic boundaries. | AB-00-02; AB-00-07 | This deliverable records the constraint; adapter/API proof remains outside DEL-03-03 scope. |
>

### CLM-010 — Standards

> ##### Standards
>
> | Reference | Status |
> |---|---|
> | SOW-007 | Available via decomposition/register text. |
> | B31J or other protected SIF/flexibility sources | Not bundled, not paraphrased, and not used as public source text in this deliverable. |
> | Project invariant catalog | Available in `docs/CONTRACT.md`. |
>

### CLM-011 — Verification

> ##### Verification
>
> - Confirm the four-document kit contains no protected standards text, SIF/flexibility tables, copied formulas, or invented engineering values.
> - Confirm `schemas/component.schema.yaml` and `schemas/model.schema.yaml` both include `bend` and `elbow` component types.
> - Confirm strict component fixture evidence includes the bend/elbow family contract, missing-value bend record, provenance, and diagnostics.
> - Confirm validation tests cover bend/elbow identity, field kinds, user/private value policy, missing-value diagnostics, and strict component fixture validation.
> - Confirm review findings remain pending until accepted by a human gate.
>

### CLM-012 — Documentation

> ##### Documentation
>
> Required local artifacts:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/`

- **AC-001** — The contract preserves accepted bend and elbow modeling requirements, explicit units and provenance, protected-data boundaries, diagnostic treatment of missing inputs, and unresolved engineering-policy choices without inventing defaults.

## Production and Verification Method — Praxeology

### CLM-013 — Procedure: DEL-03-03 Bend and elbow component model fields

> #### Procedure: DEL-03-03 Bend and elbow component model fields
>

### CLM-014 — Purpose

> ##### Purpose
>
> Define the evidence-reconciliation procedure for the bend/elbow component model deliverable without crossing the protected-data or professional-authority boundaries.
>

### CLM-015 — Prerequisites

> ##### Prerequisites
>
> - Use the sealed context for `DEL-03-03`.
> - Apply `SOW-007`, `OBJ-004`, and applicable architecture basis IDs `AB-00-01`, `AB-00-02`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
> - Apply contract invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1..4.
> - Treat dependency satisfaction, lifecycle transitions, review dispositions, downstream solver behavior, and downstream adapter/GUI behavior as unresolved unless authorized by their owning workflow or a later human gate.
>

### CLM-016 — Steps

> ##### Steps
>
> 1. Confirm the assessment task is still scoped to `DEL-03-03` and the bend/elbow component family.
> 2. Read the active component schema, model schema, strict component fixture, combined fixture pointer, tests, review findings, status, memory, and relevant run records.
> 3. Reconcile current evidence against the three local review findings without editing `Review_Findings.csv`.
> 4. Refresh only active DEL-03-03 documentation when stale setup/future language conflicts with implemented schema/fixture/test evidence.
> 5. Run focused validation for the component/section schema contract.
> 6. Record the readiness verdict in `MEMORY.md` and a local run record.
> 7. Keep all protected standards content, tables, copied formulas, and non-public commercial data out of public artifacts.
>

### CLM-017 — Verification

> ##### Verification
>
> - Active documents distinguish implemented DEL-03-03 evidence from unresolved dependency, lifecycle, and downstream behavior.
> - No protected SIF/flexibility table, formula, standard text, or invented engineering value appears in the deliverable kit.
> - Focused validation confirms schema/fixture/test evidence for bend/elbow identity, field slots, provenance, and diagnostics.
> - `_STATUS.md` and `Review_Findings.csv` are not changed by this TASK.
>

### CLM-018 — Records

> ##### Records
>
> - Active four-document kit.
> - Review findings and review notes.
> - Focused validation output.
> - Local run record for this evidence-reconciliation pass.

- **VER-001** — Validate the contract and review source parity, bend and elbow field coverage, dimensional consistency, provenance and protected-content boundaries, missing-input diagnostics, mechanics handoff limits, and unresolved decisions.

## Governing Values and Decisions — Axiology

### CLM-019 — Guidance: DEL-03-03 Bend and elbow component model fields

> #### Guidance: DEL-03-03 Bend and elbow component model fields
>

### CLM-020 — Purpose

> ##### Purpose
>
> This deliverable records bounded schema/fixture/test evidence for a bend/elbow component model slice. The important boundary is that OpenPipeStress can store and validate user-entered or lawfully imported bend/elbow data, but must not publish protected SIF/flexibility tables, formulas, examples, or default code-derived values.
>

### CLM-021 — Principles

> ##### Principles
>
> - Treat bend/elbow SIFs and flexibility factors as user/private/library inputs with provenance, not public defaults.
> - Make absent required values visible through validation findings.
> - Keep unit metadata close to numeric geometry fields.
> - Keep source metadata close to any data that may have licensing, redistribution, or protected-content implications.
> - Preserve the distinction between model data, solver mechanics, rule-pack evaluation, and human engineering judgment.
>

### CLM-022 — Considerations

> ##### Considerations
>
> The current component schema and strict component fixture expose bend/elbow identity, centerline geometry field kinds, user SIF/flexibility slots, provenance, and missing-input diagnostics without crossing the protected-data boundary. Later solver, rule-pack, adapter, and GUI workflows must consume those fields without treating public schema slots as code-specific values.
>
> Diagnostics should be able to distinguish missing geometry from missing SIF/flexibility inputs and from missing provenance. This matters because the later response may be different: solve-blocking, rule-check-blocking, provenance warning, assumption warning, or IP-boundary warning.
>

### CLM-023 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Convenience defaults vs protected-data safety | Prefer no default over any value that could imply bundled code data. |
> | Flexible generic component record vs bend-specific fields | Use bend-specific slots where validation, units, or provenance differ materially from generic components. |
> | Public examples vs realistic engineering examples | Use invented non-code values only, and label them as non-engineering examples. |
> | Schema evidence vs downstream behavior | Treat current schema/fixture/test evidence as DEL-03-03 evidence only; leave solver, rule-pack, adapter, and GUI behavior to their own deliverables. |
>

### CLM-024 — Examples

> ##### Examples
>
> - Acceptable statement: "The model stores user-entered SIFs with provenance."
> - Not acceptable: a copied or paraphrased table of SIF/flexibility factors, code formulas, or standard examples.
> - Acceptable test fixture posture: missing/schema-shape values used only to test schema shape, provenance, and diagnostics.
>

### CLM-025 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Description | Contenders | Human Ruling |
> |---|---|---|---|
> | None | No source conflict identified in this evidence-reconciliation pass. | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-007 OBJ-004 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
