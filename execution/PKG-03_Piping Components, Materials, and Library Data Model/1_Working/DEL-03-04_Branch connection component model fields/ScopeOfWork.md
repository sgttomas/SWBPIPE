---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-04
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-008]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-04` in service of project scope [SOW-008] and package objectives [OBJ-004].

- **OUT-001** — A branch-connection component-model contract defining geometry, run and branch identities, section and stiffness inputs, units, provenance, completeness, and diagnostics is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-03-04 Branch connection component model fields

> #### Datasheet: DEL-03-04 Branch connection component model fields
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-03-04 |
> | Package ID | PKG-03 |
> | Package name | Piping Components, Materials, and Library Data Model |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope item | SOW-008 |
> | Objective | OBJ-004 |
> | Anticipated artifacts | branch component model; validation tests |
>

### CLM-003 — Attributes

> ##### Attributes
>
> The branch connection component model is now evidenced by the strict component
> library schema, invented component fixture, and schema test coverage:
>
> - `schemas/component.schema.yaml` defines branch component field kinds for
>   branch run size, branch header size, connection angle, connection type,
>   reinforcement area, reinforcement reference, branch geometry source
>   reference, user SIF value, and user flexibility-factor value.
> - `fixtures/component/invented_component_library_valid.json` carries a branch
>   component-family contract with geometry field kinds, rule-modifier field
>   kinds, source metadata field kinds, mechanics-interface metadata, and
>   `protected_value_policy: schema_slots_only`.
> - The same fixture carries an invented branch record whose public field slots
>   are intentionally `missing` and provenance-bearing, with schema-shape-only or
>   no-public-code-specific-value policies.
> - `tests/test_component_section_schema.py` asserts branch enum coverage,
>   branch family-contract coverage, branch fixture field coverage, branch
>   completeness diagnostics, component diagnostic `class`/`source` fields, and
>   protected-content denylist coverage.
>
> No protected branch connection tables, SIF tables, flexibility tables, code
> formulas, dimensional tables, copied engineering examples, proprietary catalog
> values, or bundled public branch defaults are included. Remaining `TBD` items
> are policy/source decisions rather than absence of the implemented schema
> surface.
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Source | Status |
> |---|---|---|
> | Component model shall support branch connections with user-entered reinforcement, SIF, flexibility, and local data fields. | docs/_Registers/ScopeLedger.csv row SOW-008; execution/_Decomposition/SOFTWARE_DECOMP.md rows SOW-008 and DEL-03-04 | In scope |
> | Public repository must not contain protected standards text, protected tables, copied formulas, or protected branch/SIF/flexibility data. | docs/CONTRACT.md OPS-K-IP-1, OPS-K-IP-3 | Constraint |
> | Code-specific values are user-supplied or lawfully imported private data, not bundled public defaults. | docs/CONTRACT.md OPS-K-DATA-1 | Constraint |
> | Missing solve-required or rule-check-required values are findings, not silent defaults. | docs/CONTRACT.md OPS-K-DATA-2 | Constraint |
> | Materials, components, SIFs, flexibility factors, allowables, and rule-pack values carry provenance fields. | docs/CONTRACT.md OPS-K-DATA-3 | Constraint |
> | Calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked. | docs/CONTRACT.md OPS-K-UNIT-1 | Constraint |
>

### CLM-005 — Construction

> ##### Construction
>
> The implemented evidence is schema-first and repository-level:
>
> - `schemas/component.schema.yaml` exposes a JSON Schema 2020-12 component
>   library contract with `component_library`, `component_family_contracts`,
>   `component_records`, `field_definitions`, `completeness_rules`,
>   `diagnostics`, and `open_decisions`.
> - Branch dimensional fields use `ComponentQuantityDimension` values that remain
>   constrained to the accepted unit dimension vocabulary exercised by the schema
>   test.
> - Branch diagnostics include `BRANCH_GEOMETRY_INCOMPLETE`,
>   `BRANCH_REINFORCEMENT_DATA_MISSING`, and `BRANCH_RULE_INPUT_MISSING`; the
>   component diagnostic shape requires `class`, `source`, affected reference,
>   message, remediation, and provenance.
> - The invented branch fixture records missing branch run/header/angle,
>   reinforcement reference, user SIF, and user flexibility-factor inputs as
>   blocking completeness evidence instead of silent defaults.
>
> This reconciliation records that implementation evidence. It does not edit
> repo-level schema, fixture, test, code, DAG, dependency, review-disposition, or
> lifecycle files.
>

### CLM-006 — Remaining TBDs

> ##### Remaining TBDs
>
> - Accepted public branch component source catalogs remain `TBD`.
> - Public branch fixture-value policy remains `TBD`; current public fixture
>   values are schema-shape-only or omitted.
> - Standard-specific branch interpretation, local-check methods, concrete import
>   formats, and GUI behavior remain `TBD`.
> - Dependency satisfaction and lifecycle state remain governed by their existing
>   local files and human/reconciliation process.
> - Review findings `PKG03-DEL-03-04-PKG02-001` and `-002` are
>   `ACCEPT_AS_IS` / `RESOLVED` in `Review_Findings.csv` under the recorded
>   2026-06-05 Gate C disposition. This documentation refresh does not alter
>   that formal review record.
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, accepted decomposition revision, architecture basis injection.
> - `_REFERENCES.md` - local governing reference list.
> - `docs/_Registers/Deliverables.csv` row DEL-03-04.
> - `docs/_Registers/ScopeLedger.csv` row SOW-008.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-03-04.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, rows for PKG-03, DEL-03-04, SOW-008, OBJ-004, and AB-00-01/02/04/06/07/08.
> - `docs/CONTRACT.md` invariants listed in this kit.
> - `schemas/component.schema.yaml` current component library schema evidence.
> - `fixtures/component/invented_component_library_valid.json` current invented
>   component fixture evidence.
> - `tests/test_component_section_schema.py` current schema/fixture validation
>   evidence.
> - `MEMORY.md`, `_REVIEW.md`, and `Review_Findings.csv` local reconciliation and
>   review evidence.

### CLM-008 — D-41 R5 T3 PDU-019 Evidence State

> ##### D-41 R5 T3 PDU-019 Evidence State
>
> Technical no-bypass evidence: present. Formal owning REVIEW sufficiency/disposition: held.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-03-04 Branch connection component model fields

> #### Specification: DEL-03-04 Branch connection component model fields
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies the branch connection component model fields now
> evidenced by `schemas/component.schema.yaml`,
> `fixtures/component/invented_component_library_valid.json`, and
> `tests/test_component_section_schema.py`. The implemented schema surface covers
> branch connection geometry, reinforcement references/area slots, user
> SIF/flexibility slots, source metadata, provenance, unit dimensions,
> completeness rules, and diagnostics without bundling protected engineering
> values.
>
> This evidence-reconciliation pass excludes:
>
> - product implementation code;
> - repo-level schema edits;
> - fixture or test edits;
> - dependency, review-finding, DAG, coordination, lifecycle, or status edits;
> - protected standards text, formulas, tables, figures, copied examples, or proprietary data;
> - bundled branch, SIF, flexibility, dimensional, material, or allowable defaults;
> - claims language — Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-011 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-03-04-RQ-001 | The component schema shall support branch connection data fields for geometry, reinforcement, user SIF/flexibility, and local branch data fields. | SOW-008; DEL-03-04 row | Validation tests inspect the model/schema for supported field groups. |
> | DEL-03-04-RQ-002 | Branch connection SIF, flexibility, reinforcement, and other code-specific values shall be user-supplied or lawfully imported private data, not bundled public defaults. | OPS-K-DATA-1; OPS-K-IP-1 | Protected-content and fixture review confirms no bundled protected tables/defaults/examples. |
> | DEL-03-04-RQ-003 | Missing solve-required or rule-check-required branch values shall be represented as explicit diagnostics/findings, not silent defaults. | OPS-K-DATA-2; AB-00-06 | Validation tests cover missing required data and expected diagnostic classes. |
> | DEL-03-04-RQ-004 | Branch component records shall carry provenance/source fields for component data, SIF values, flexibility factors, and imported values where applicable. | OPS-K-DATA-3; AB-00-04; AB-00-07 | Schema/model tests confirm provenance metadata fields and import boundary diagnostics. |
> | DEL-03-04-RQ-005 | Units associated with branch geometry, reinforcement, SIF/flexibility inputs, and exports shall be unit-aware and dimensionally checked where dimensions apply. | OPS-K-UNIT-1; AB-00-04 | Unit validation tests cover accepted, rejected, and missing unit metadata. |
> | DEL-03-04-RQ-006 | The implementation shall preserve public/private data boundaries and cannot bypass governance, validation, diagnostics, or provenance checks. | AB-00-02; AB-00-07; OPS-K-IP-3 | Architecture and service-boundary tests verify validation paths are used. |
> | DEL-03-04-RQ-007 | Validation tests shall be included for model completeness, provenance, unit handling, and protected-content boundary behavior. | AB-00-08; anticipated artifacts | Test inventory and CI-local validation confirm coverage. |
>

### CLM-012 — Current Evidence

> ##### Current Evidence
>
> - The component schema defines branch field kinds:
>   `branch_run_size`, `branch_header_size`, `branch_connection_angle`,
>   `branch_connection_type`, `branch_reinforcement_area`,
>   `branch_reinforcement_reference`, `branch_geometry_source_reference`,
>   `sif_user_value`, and `flexibility_factor_user_value`.
> - The branch component-family contract in the invented fixture lists branch
>   geometry, reinforcement, SIF/flexibility, source metadata, and
>   mechanics-interface slots with `protected_value_policy: schema_slots_only`.
> - The invented branch component record uses `public_schema_only`,
>   `redistribution_status: TBD`, missing value statuses, provenance, and a
>   `BRANCH_RULE_INPUT_MISSING` completeness finding.
> - Component diagnostics now require `class` and `source` fields in addition to
>   the earlier code, severity, affected reference, message, remediation, and
>   provenance fields.
> - The schema test asserts branch enum membership, branch fixture contract
>   membership, branch fixture field slots, branch completeness diagnostics,
>   diagnostic `class`/`source` requirements, fixture validation, and protected
>   public-data denylist behavior.
>

### CLM-013 — Standards

> ##### Standards
>
> No protected engineering standard text is locally available or reproduced here.
> Any standard-specific branch connection interpretation remains `TBD` and must
> be supplied through lawful private data, a licensed source, or a human-approved
> public abstraction that does not reproduce protected content.
>

### CLM-014 — Verification

> ##### Verification
>
> Current verification evidence includes:
>
> - schema/model presence checks for each field group named in SOW-008;
> - unit-aware validation for dimensional fields;
> - provenance completeness checks for user/imported values;
> - missing-data diagnostics for required analysis or rule-check inputs;
> - protected-content scans confirming no bundled branch/SIF/flexibility tables, formulas, or examples;
> - schema and fixture validation through `tests/test_component_section_schema.py`.
>
> Remaining verification `TBD`s include dependency satisfaction disposition,
> public fixture-value policy, accepted branch source catalog, persistence or
> round-trip evidence beyond the current strict schema fixture, adapter/import
> formats, and lifecycle acceptance.
>

### CLM-015 — Documentation

> ##### Documentation
>
> Current implementation evidence:
>
> - branch component schema surface: `schemas/component.schema.yaml`;
> - invented branch component fixture: `fixtures/component/invented_component_library_valid.json`;
> - schema/fixture validation: `tests/test_component_section_schema.py`.
>
> This deliverable-local kit records evidence reconciliation only. It is not a
> lifecycle promotion, release claim, professional/code-compliance claim, or human
> acceptance decision.

### CLM-016 — D-41 R5 T3 PDU-019 O7/E5 Evidence (2026-07-12)

> ##### D-41 R5 T3 PDU-019 O7/E5 Evidence (2026-07-12)
>
> Negative schema tests prove invalid privacy classes and unknown embedded payload fields are rejected. This bounded technical evidence does not determine formal REVIEW sufficiency or disposition.

- **AC-001** — The contract preserves accepted branch-connection requirements and limits, including explicit dimensional and provenance data, rights-safe fixtures, missing-value findings, and unresolved engineering choices without inferred values or compliance claims.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-03-04 Branch connection component model fields

> #### Procedure: DEL-03-04 Branch connection component model fields
>

### CLM-018 — Purpose

> ##### Purpose
>
> Describe the evidence-reconciliation procedure for maintaining DEL-03-04
> branch connection component model documentation against the implemented
> component schema, invented fixture, and schema test evidence without exceeding
> the authorized data boundary.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> - Use the sealed DEL-03-04 brief and deliverable-local context.
> - Apply SOW-008 and OBJ-004.
> - Apply architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
> - Apply CONTRACT invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-RULE-1, OPS-K-MECH-1, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
> - Read `schemas/component.schema.yaml`,
>   `fixtures/component/invented_component_library_valid.json`, and
>   `tests/test_component_section_schema.py` as current implementation evidence.
> - Treat absent engineering source data, policy decisions, dependency
>   satisfaction, human dispositions, and lifecycle acceptance as `TBD`.
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the write scope is limited to authorized DEL-03-04 narrative files
>    and the new reconciliation run record.
> 2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, the relevant decomposition/register rows, and applicable CONTRACT invariants.
> 3. Reconcile the four active docs and `MEMORY.md` against the implemented
>    branch evidence in the component schema, invented component fixture, schema
>    test, and local review findings.
> 4. Record implemented branch component field names where the schema now defines
>    them: `branch_run_size`, `branch_header_size`,
>    `branch_connection_angle`, `branch_connection_type`,
>    `branch_reinforcement_area`, `branch_reinforcement_reference`,
>    `branch_geometry_source_reference`, `sif_user_value`, and
>    `flexibility_factor_user_value`.
> 5. Preserve unresolved `TBD`s for public branch source catalogs, fixture-value
>    policy, standard-specific interpretation, concrete import formats,
>    dependency satisfaction, human review dispositions, and lifecycle state.
> 6. Preserve the formal Gate C record: `Review_Findings.csv` records
>    `PKG03-DEL-03-04-PKG02-001` and `-002` as `ACCEPT_AS_IS` / `RESOLVED`.
>    Documentation maintenance must not rewrite that review disposition.
> 7. Run relevant validation when feasible: `python3 -m pytest
>    tests/test_component_section_schema.py` and a stale-language search scoped to
>    `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
> 8. Do not introduce protected tables, formulas, examples, dimensional data,
>    material allowables, or certification/compliance claims.
>

### CLM-021 — Verification

> ##### Verification
>
> - Four-document kit exists and preserves Datasheet, Specification, Guidance, and Procedure sections.
> - Requirements trace to local context, decomposition/register rows, CONTRACT invariants, schema evidence, fixture evidence, and test evidence.
> - Implemented branch schema slots are no longer described as absent or future-only.
> - Unknown policy/source/dependency/human/lifecycle gates are still `TBD`; assumptions are labeled.
> - No protected branch/SIF/flexibility tables, formulas, or examples are present.
> - `python3 -m pytest tests/test_component_section_schema.py` passes when the
>   local test environment is available.
> - A stale-language search over the four active docs shows no remaining
>   setup-only or future-only implementation language that contradicts the
>   schema/fixture/test evidence.
>

### CLM-022 — Records

> ##### Records
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/TASK_RUN_*.md`
> - `_run_records/TASK_RUN_2026-06-05_DEL-03-04_evidence-reconciliation.md`

### CLM-023 — D-41 R5 T3 PDU-019 Check

> ##### D-41 R5 T3 PDU-019 Check
>
> Run strict ComponentRecord schema negatives for privacy classification and unknown payload keys. Route evidence to REVIEW without changing its outcome.

- **VER-001** — Validate the contract and review source parity, branch geometry and identity coverage, dimensional consistency, provenance and protected-content controls, missing-input diagnostics, mechanics handoff limits, and unresolved decisions.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-03-04 Branch connection component model fields

> #### Guidance: DEL-03-04 Branch connection component model fields
>

### CLM-025 — Purpose

> ##### Purpose
>
> This deliverable exists to keep branch connection component data explicit,
> unit-aware, provenance-bearing, and separated from protected standards content.
> The current component schema and invented fixture provide branch schema slots
> and missing-data diagnostics so users or lawful private libraries can supply
> branch-specific data without the public project embedding protected branch
> connection, SIF, flexibility, or reinforcement tables.
>

### CLM-026 — Principles

> ##### Principles
>
> - Treat branch connection data as user/imported input unless a public, rights-cleared source is documented.
> - Preserve the difference between data capture, solver mechanics, rule-pack evaluation, and human professional judgment.
> - Prefer explicit `TBD` or diagnostics over inferred defaults.
> - Keep provenance and redistribution status close to the values they qualify.
> - Make dimensional fields unit-aware; do not accept unitless dimensional values unless a later schema deliberately defines a safe representation.
> - Use the implemented branch field names in `schemas/component.schema.yaml` when
>   documenting current schema evidence; reserve `TBD` for unresolved policy,
>   source, lifecycle, dependency, or human-disposition questions.
>

### CLM-027 — Considerations

> ##### Considerations
>
> The branch connection model has more local field slots than simpler component
> families. Current evidence covers branch run/header size, connection angle/type,
> reinforcement area/reference, branch geometry source reference, user SIF, and
> user flexibility-factor field kinds. The decomposition still leaves specialized
> branch local-check methods, concrete import formats, and GUI behavior outside
> this reconciliation.
>
> Architecture basis implications:
>
> - AB-00-02 and AB-00-07 point toward domain contracts and validation boundaries rather than GUI or adapter shortcuts.
> - AB-00-04 points toward deterministic, provenance-preserving persistence when serialized.
> - AB-00-06 points toward structured diagnostics for blocking missing data and boundary warnings.
> - AB-00-08 points toward layered tests, including protected-content/provenance gates.
>

### CLM-028 — Trade-offs

> ##### Trade-offs
>
> | Topic | Guidance |
> |---|---|
> | Field specificity vs protected content | Capture field categories and user-supplied values; do not encode protected lookup content. |
> | Completeness vs implementation timing | Treat current branch schema field names as implemented evidence; keep specialized local-check needs, source catalogs, import formats, and lifecycle acceptance as `TBD`. |
> | Public examples vs validation | Use the existing invented non-engineering fixture as schema/diagnostic evidence only; do not treat its missing branch values as engineering examples. |
> | User flexibility vs diagnostics | Permit user-supplied branch data, but report missing or unverifiable values explicitly. |
>

### CLM-029 — Examples

> ##### Examples
>
> The current public fixture is an invented schema fixture, not an engineering
> example. It intentionally omits branch geometry, reinforcement, SIF, and
> flexibility values and emits `BRANCH_RULE_INPUT_MISSING` rather than supplying
> public defaults. Future value-bearing examples, if any, must be invented
> non-code values with clear non-engineering notices or must come from reviewed
> public-permissive/private sources; they must not reproduce protected tables,
> formulas, or code examples.
>

### CLM-030 — Review Posture

> ##### Review Posture
>
> Local review findings for unit-dependency satisfaction and component diagnostic
> envelope compatibility are `ACCEPT_AS_IS` / `RESOLVED` in
> `Review_Findings.csv` under the recorded 2026-06-05 Gate C disposition.
> Narrative evidence may cite the technical branch schema, fixture, and diagnostic
> updates, while dependency and lifecycle state remain governed separately.
>

### CLM-031 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | None identified | No conflict was detected between the current branch schema/fixture/test evidence and the local review/memory evidence. | `schemas/component.schema.yaml`; fixture; test | `MEMORY.md`; `_REVIEW.md`; `Review_Findings.csv` | N/A | Treat implementation evidence as current while preserving policy, dependency, human-disposition, and lifecycle gates. | TBD |

### CLM-032 — D-41 R5 T3 PDU-019 Boundary

> ##### D-41 R5 T3 PDU-019 Boundary
>
> Schema rejection evidence is not a formal review outcome and must not be represented as one.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-008 OBJ-004 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
