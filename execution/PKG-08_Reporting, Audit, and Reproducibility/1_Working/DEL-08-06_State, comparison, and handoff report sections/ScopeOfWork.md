---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-06
package_id: PKG-08
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-024]
package_objective_refs: [OBJ-007, OBJ-016, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-08-06

## Purpose and Objective Traceability

This Scope of Work defines `DEL-08-06` in service of project scope [SOW-024] and package objectives [OBJ-007, OBJ-016, OBJ-017, OBJ-018].

- **OUT-001** — A state, comparison, and handoff report-section contract covering immutable model-state and analysis-run records, deterministic comparisons, handoff manifests, warnings, assumptions, limitations, units, hashes, provenance, and unsupported-target disclosures is produced without implying professional validation or acceptance.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-08-06 State, comparison, and handoff report sections

> #### Datasheet: DEL-08-06 State, comparison, and handoff report sections
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-06-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-08-06 | `_CONTEXT.md` |
> | Name | State, comparison, and handoff report sections | `_CONTEXT.md` |
> | Package ID | PKG-08 | `_CONTEXT.md` |
> | Package name | Reporting, Audit, and Reproducibility | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-08 |
> | Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-08-06 |
> | Scope coverage | SOW-024 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-024 |
> | Objective support | OBJ-007, OBJ-016, OBJ-017, OBJ-018 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective map |
> | Implementation evidence | TBD | No product code was inspected or created in this setup pass. |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current source-grounded value | Source |
> |---|---|---|
> | Deliverable purpose | Implement report sections generated from model states, analysis runs, comparisons, and handoff manifests without implying professional validation. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-08-06 |
> | Anticipated artifacts | State/run report sections; comparison report section; handoff manifest report section. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-08-06 |
> | Report content basis | Auditable calculation reports include inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations. | SOW-024 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv`; `docs/SPEC.md` section 9 |
> | State/run basis | Immutable model states, analysis runs, and deterministic comparisons are first-class product records for design iteration and review. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-016; rows DEL-14-01 through DEL-14-05 |
> | Handoff basis | Handoff packages support downstream modeling and professional validation workflows without automatic professional approval states. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-017; rows DEL-15-01, DEL-15-03, DEL-15-04 |
> | Professional boundary | Reports are decision support and must not declare code compliance, certification, sealing, approval, authentication, endorsement, or professional reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 9 |
> | IP/data boundary | Public report artifacts must not copy protected standards text, protected tables, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, or real secrets. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-REPORT-2; `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 7 |
> | Hash/provenance basis | JSON payload hashes use the accepted JCS-compatible canonical JSON basis where applicable; reports preserve stable references, checksums, source notes, privacy classification, review state, and provenance. | `_CONTEXT.md` Architecture Basis; `docs/SPEC.md` sections 4.4 and 9 |
> | Upstream dependency mirror | 22 ACTIVE approved DAG-006 rows are present as local evidence. | `Dependencies.csv`; `_DEPENDENCIES.md` |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Value | Source |
> |---|---|---|
> | Lifecycle state at four-doc pass start | OPEN | `_STATUS.md` |
> | Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row DEL-08-06 |
> | Package exclusion | PKG-08 does not authenticate or certify engineering work. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-08 |
> | Public/private content posture | Private rule packs, material libraries, component libraries, owner requirements, project values, and private templates remain user-controlled unless intentionally exported or contributed with documented rights. | `docs/IP_AND_DATA_BOUNDARY.md` sections 6 and 7 |
> | Missing engineering values | Missing solve-required or rule-check-required values remain explicit findings, not defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4.3 and 9 |
> | Runtime/code locations | TBD | No deliverable-specific implementation files are defined by the accessible sources. |
>

### CLM-006 — Construction

> ##### Construction
>
> | Construct | Description | Source |
> |---|---|---|
> | State/run report sections | Report-facing sections that reference immutable model states and analysis runs, including hashes, warnings, assumptions, diagnostics, source notes, and relevant result/report payload references. | `_CONTEXT.md`; `docs/SPEC.md` section 9; `Dependencies.csv` rows DAG-002-E0861 and DAG-002-E0862 |
> | Comparison report section | Report-facing section for deterministic model-state and/or analysis-run comparison records using stable IDs, mappings where required, units, diagnostics, and tolerance/profile references when available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-073; `Dependencies.csv` rows DAG-002-E0863 through DAG-002-E0865 |
> | Handoff manifest report section | Report-facing section that references handoff package manifest data, units, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags, and external-prover boundary metadata when available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-074 and SOW-075; `Dependencies.csv` rows DAG-002-E0866 through DAG-002-E0868 |
> | Integration surface | ASSUMPTION: this deliverable consumes report generator, audit manifest, report-section, result export, protected-content linter, redaction/export, state/run/comparison, and handoff records through schema-first service boundaries. | `Dependencies.csv`; `_CONTEXT.md` Architecture Basis; exact API names are TBD. |
> | Test surface | TBD: expected tests should verify section assembly, boundary wording, protected-content avoidance, provenance/checksum preservation, unit metadata handling, and missing-data reporting. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08; concrete test paths are TBD. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, objectives, architecture-basis injection, and package boundaries.
> - `_REFERENCES.md` - accessible governing reference list for this folder.
> - `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-006 local mirror and evidence surface.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 package, deliverable, objective, scope, and architecture-basis entries.
> - `docs/_Registers/Deliverables.csv` - row DEL-08-06.
> - `docs/_Registers/ScopeLedger.csv` - row SOW-024 and related source rows SOW-071 through SOW-075.
> - `docs/_Registers/ContextBudgetQA.csv` - row DEL-08-06.
> - `docs/CONTRACT.md` - invariants for IP/data, professional authority, reports, units, rules, and agents.
> - `docs/SPEC.md` - section 4.3 analysis boundary, section 4.4 persistence/hash basis, and section 9 reporting and audit.
> - `docs/DIRECTIVE.md` - founding intent, non-negotiable product principles, scope, and stop rules.
> - `docs/IP_AND_DATA_BOUNDARY.md` - public/private data and report boundary policy.
> - `docs/TYPES.md` - schema/object registry entries for report, report section, result export, audit manifest, checksum, states/runs, comparisons, and handoff-related references.
>

### CLM-008 — D-41 R5 T4 current implementation

> ##### D-41 R5 T4 current implementation
>
> | Item | Current bounded state |
> |---|---|
> | Application-service input | Canonical `project.run_history.model_state_records` and `analysis_run_records` |
> | Preserved report context | Warnings, assumptions, limitations, solver context, hashes/references, analysis status, and provenance |
> | Mutations/execution | None; project mutation and solver execution are recorded false |
> | Authority boundary | No human approval, code-compliance, validation, release, or professional-reliance inference |
> | Held scope | External/non-JSON payload partitioning, rendered layout, concrete adapter/runtime hooks, and broader workflow integration |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-08-06 State, comparison, and handoff report sections

> #### Specification: DEL-08-06 State, comparison, and handoff report sections
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-06-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> This deliverable covers backend report sections for state/run records, deterministic comparison records, and handoff manifest records. The sections must support auditable calculation reports and preserve professional, IP, privacy, provenance, checksum, unit, and missing-data boundaries.
>
> This deliverable excludes final report styling/layout, GUI presentation, CLI runtime behavior, API transport, arbitrary project-file reading, solver-internal execution, protected-content linting implementation, private redaction/export control implementation, and any automatic professional approval or code-compliance status. These exclusions are source-grounded in `docs/SPEC.md` section 9 and the approved upstream dependency mirror.
>
> Implementation file paths, concrete API names, exact schema fragments, and dependency versions are TBD unless later resolved by human-approved implementation work.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-08-06-R1 | The deliverable shall provide report-section behavior for model states, analysis runs, comparisons, and handoff manifests. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-08-06 | Tests or review evidence shall show all three anticipated artifact families are represented. |
> | DEL-08-06-R2 | Report sections shall support SOW-024 content: inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations. | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-024; `docs/SPEC.md` section 9 | Section fixtures or schema tests shall include fields or references for each SOW-024 content category, or explicit TBD diagnostics where unavailable. |
> | DEL-08-06-R3 | Report sections shall preserve the professional boundary and shall not declare code compliance, certification, sealing, approval, authentication, endorsement, or professional reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 9; `docs/DIRECTIVE.md` sections 2.2 and 3 | Boundary-wording tests or protected phrase checks shall reject automatic professional-approval claims. |
> | DEL-08-06-R4 | Software-generated analysis statuses used by report sections shall not include `HUMAN_APPROVED_FOR_PROJECT` or equivalent automatic human approval/professional reliance labels. | `docs/SPEC.md` sections 4.3 and 9; `docs/TYPES.md` companion schema notes | Status serialization tests shall verify only permitted software-generated status values are emitted. |
> | DEL-08-06-R5 | Missing solve-required or rule-check-required values shall remain explicit findings with diagnostics and provenance, not hidden defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4.3 and 9 | Tests shall verify missing inputs produce explicit report-facing findings. |
> | DEL-08-06-R6 | Numeric values included through result/report references shall carry unit and dimensional metadata or explicit diagnostics from source envelopes. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` section 9 | Unit/dimension validation tests shall reject unqualified numeric report values unless paired with diagnostics. |
> | DEL-08-06-R7 | Report sections shall preserve stable references, checksums, source notes, privacy classification, review state, and provenance for input/report/result payloads where available. | `docs/SPEC.md` sections 4.4 and 9; `_CONTEXT.md` Architecture Basis AB-00-04 | Tests shall compare stable identifiers and checksum/provenance fields across source envelope and rendered section records. |
> | DEL-08-06-R8 | Public report sections, templates, fixtures, and examples shall not copy protected standards text, protected tables, protected examples, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, or real secrets. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-REPORT-2; `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 7 | Protected-content/provenance gates shall inspect configured public surfaces; human/legal review remains required when uncertain. |
> | DEL-08-06-R9 | Report sections may reference private rule packs, libraries, owner requirements, and project values only by identifier, checksum, source note, privacy class, and review state when public/private boundaries require it. | `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` sections 6 and 7 | Fixture review shall confirm private payload contents are not copied into public artifacts. |
> | DEL-08-06-R10 | State/run report sections shall consume immutable model-state and analysis-run records through their stable record identities, hashes, warnings, assumptions, diagnostics, solver version, settings, units, load cases, results, rule/library refs, and result hashes where available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-071 and SOW-072; `Dependencies.csv` rows DAG-002-E0861 and DAG-002-E0862 | Integration tests shall use approved or invented fixtures with explicit references to state/run source records. |
> | DEL-08-06-R11 | Comparison report sections shall preserve deterministic comparison semantics, including stable IDs, manual mappings where required, unit-normalized result deltas, tolerance/profile references, unmatched classifications, diagnostics, and settings where available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-073 and rows DEL-14-03 through DEL-14-05; `Dependencies.csv` rows DAG-002-E0863 through DAG-002-E0865 | Comparison fixture tests shall verify deterministic ordering and boundary wording; exact threshold policy is TBD. |
> | DEL-08-06-R12 | Handoff manifest report sections shall preserve handoff package manifest data, model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags, and external-prover boundary metadata where available. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-074 and SOW-075; `Dependencies.csv` rows DAG-002-E0866 through DAG-002-E0868 | Handoff fixture tests shall verify manifest references and unsupported-target/professional-boundary disclosures. |
> | DEL-08-06-R13 | Report-section behavior shall remain behind schema-first service boundaries and shall not bypass governance, validation, diagnostics, privacy, protected-content, report, solver, rule, or human-acceptance boundaries. | `_CONTEXT.md` Architecture Basis AB-00-02, AB-00-03, AB-00-06, AB-00-07; `docs/SPEC.md` sections 4.5 and 9 | Architecture or module-boundary review shall verify the section builder depends on source envelopes/contracts, not on bypass paths. |
> | DEL-08-06-R14 | Tests shall be layered and include report-relevant protected-content/provenance gates where applicable. | `_CONTEXT.md` Architecture Basis AB-00-08; `docs/CONTRACT.md` enforcement map | Test-plan review shall identify Cargo/validation/protected-content gates or mark unavailable gates as TBD. |
>

### CLM-013 — Standards

> ##### Standards
>
> | Standard or governing basis | Status for this deliverable | Source |
> |---|---|---|
> | JSON Schema 2020-12 contracts | Applicable as architecture basis; exact schema fragments for this deliverable are TBD. | `_CONTEXT.md` Architecture Basis; `docs/TYPES.md` schema registry |
> | JCS-compatible canonical JSON hash basis | Applicable where JSON payloads are hashed; non-JSON/binary partitioning remains TBD. | `_CONTEXT.md` Architecture Basis AB-00-04; `docs/SPEC.md` section 4.4 |
> | Project invariant catalog | Applicable for IP/data, report, authority, unit, and agent constraints. | `docs/CONTRACT.md` |
> | Public/private report boundary policy | Applicable for public report sections and examples. | `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
> | Protected standards or code-specific engineering standards text | Not accessible and not to be reproduced. Clause-level engineering requirements are TBD unless supplied through authorized user/private sources. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/DIRECTIVE.md` section 4.2 |
>

### CLM-014 — Verification

> ##### Verification
>
> | Verification topic | Minimum check | Evidence state |
> |---|---|---|
> | Scope coverage | Confirm state/run, comparison, and handoff report-section artifacts exist or are explicitly represented in tests. | TBD |
> | SOW-024 content | Confirm inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations are represented. | TBD |
> | Professional boundary | Confirm generated wording cannot imply professional approval, certification, sealing, authentication, endorsement, code compliance, or reliance. | TBD |
> | IP/protected content | Confirm public sections/templates/fixtures are scanned or reviewed for protected/private content risk. | TBD |
> | Units and numeric values | Confirm numeric values carry unit/dimension metadata or diagnostics. | TBD |
> | Provenance/checksums | Confirm source notes, checksum refs, review state, privacy class, and stable refs survive assembly. | TBD |
> | Determinism | Confirm comparison and section ordering are deterministic for identical source envelopes. | TBD |
> | Missing data | Confirm missing source values become explicit findings, warnings, limitations, or unresolved TBDs. | TBD |
>

### CLM-015 — Documentation

> ##### Documentation
>
> Expected deliverable-local records and downstream artifacts:
>
> - State/run report sections.
> - Comparison report section.
> - Handoff manifest report section.
> - Tests or review notes proving boundary wording, SOW-024 coverage, deterministic assembly, provenance/checksum preservation, and protected-content avoidance.
> - Source notes that identify upstream record contracts consumed by the sections.
> - Any unresolved implementation/API/schema decisions recorded as TBD rather than inferred.
>

### CLM-016 — D-41 R5 T4 persisted-run reporting seam

> ##### D-41 R5 T4 persisted-run reporting seam
>
> The bounded `project.run_history` application-service handoff may assemble
> existing model-state and analysis-run records into backend report sections.
> It shall preserve warnings, unresolved assumptions, explicit limitations,
> solver context, provenance, and only the governed software analysis-status
> vocabulary. It shall not mutate the project, execute a solver, infer
> `HUMAN_APPROVED_FOR_PROJECT` or `CODE_COMPLIANT`, redefine external/non-JSON
> payload partitioning, or imply rendered-report, runtime-adapter, or whole-
> workflow closure.

- **AC-001** — The contract preserves stable record identities, source-envelope diagnostics, deterministic comparison mappings and tolerance references when available, handoff traceability, missing-data visibility, protected/private-content boundaries, and professional-authority limits while retaining unresolved implementation, schema, layout, and external-payload decisions as source-grounded TBDs.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-08-06 State, comparison, and handoff report sections

> #### Procedure: DEL-08-06 State, comparison, and handoff report sections
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-018 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-06-DECL-004`.
>

### CLM-019 — Purpose

> ##### Purpose
>
> Define an operational procedure for producing and verifying the DEL-08-06 report-section artifacts without inventing implementation evidence, engineering values, standards text, or product code. This procedure is scoped to future execution of state/run, comparison, and handoff manifest report sections.
>

### CLM-020 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Required state | Source |
> |---|---|---|
> | Deliverable context | `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, and `Dependencies.csv` are available in the deliverable folder. | Local folder |
> | Report generator basis | Report sections integrate with report generator conventions. | `Dependencies.csv` row DAG-002-E0855; `docs/SPEC.md` section 9 |
> | Audit manifest/hash basis | Audit manifest and model-hash conventions are available. | `Dependencies.csv` row DAG-002-E0856; `docs/SPEC.md` sections 4.4 and 9 |
> | Warnings/provenance basis | Warnings, assumptions, provenance notes, limitations, unresolved TBDs, and human-review-needed findings are represented. | `Dependencies.csv` row DAG-002-E0857; `docs/SPEC.md` section 9 |
> | Result export basis | Result export envelopes are available for report consumption. | `Dependencies.csv` row DAG-002-E0858; `docs/SPEC.md` section 9 |
> | Protected-content controls | Protected-content linter controls and private redaction/export controls are available or explicitly marked TBD. | `Dependencies.csv` rows DAG-002-E0859 and DAG-002-E0860 |
> | State/run/comparison sources | Immutable model states, analysis runs, state comparisons, run comparisons, and comparison export contracts are available or explicitly marked TBD. | `Dependencies.csv` rows DAG-002-E0861 through DAG-002-E0865 |
> | Handoff sources | Handoff package manifests, export workflow records, and external-prover boundary metadata are available or explicitly marked TBD. | `Dependencies.csv` rows DAG-002-E0866 through DAG-002-E0868 |
> | Professional/IP boundary | Boundary wording and public/private content controls are applied. | `docs/CONTRACT.md`; `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` |
>

### CLM-021 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity is DEL-08-06 and the package is PKG-08.
> 2. Read the current source contracts for report generation, audit manifests, report sections, result exports, protected-content linting, state/run records, comparison records, handoff package manifests, export workflows, and external-prover metadata.
> 3. Define the state/run report-section inputs as references to immutable model-state and analysis-run records. Include stable IDs, model/run basis, hashes, solver version stamps, settings, units, load cases, diagnostics, warnings, assumptions, rule/library references, result hashes, and source/provenance notes where available.
> 4. Define the comparison report-section inputs as references to deterministic state/run comparison records. Include stable IDs, manual mappings where required, unmatched classifications, unit-normalized deltas, tolerance/profile references, diagnostics, settings, and source/provenance notes where available.
> 5. Define the handoff manifest report-section inputs as references to handoff package records. Include model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags, export workflow records, and external-prover boundary metadata where available.
> 6. For every field that carries an engineering value, confirm the value has unit/dimensional metadata, provenance, review state, privacy classification, and source envelope evidence where required. If evidence is absent, emit an explicit finding or TBD rather than a default.
> 7. Apply professional-boundary wording. The generated sections may support human professional review but must not state or imply software-generated code compliance, approval, certification, sealing, authentication, endorsement, or professional reliance.
> 8. Apply public/private and protected-content boundaries. Public sections, templates, examples, and fixtures must avoid protected standards text, protected tables, protected examples, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, and real secrets.
> 9. Preserve deterministic ordering and stable references so repeated generation from identical source envelopes produces repeatable output.
> 10. Record unresolved implementation decisions as TBD. Current TBDs include concrete code paths, exact schema fragments, API names, report layout, final notice wording, release thresholds, and external transport/export details.
>

### CLM-022 — Verification

> ##### Verification
>
> | Check | Expected result | Evidence |
> |---|---|---|
> | Scope completeness | State/run, comparison, and handoff report-section artifacts are represented. | Tests or review notes, TBD |
> | SOW-024 coverage | Inputs, sources, warnings, assumptions, results, rule-pack checksums, and limitations are present or explicitly marked TBD. | Tests or fixture review, TBD |
> | Boundary wording | No automatic professional approval, certification, sealing, authentication, endorsement, code-compliance, or reliance claim is emitted. | Boundary tests, TBD |
> | Missing-data behavior | Missing solve-required or rule-check-required values become explicit findings, warnings, limitations, unresolved TBDs, or human-review-needed findings. | Tests, TBD |
> | Unit/provenance behavior | Numeric values carry unit/dimensional metadata and provenance or explicit diagnostics. | Schema/unit tests, TBD |
> | Protected-content behavior | Public report surfaces avoid protected/private content and preserve only permitted references/checksums/source notes. | Protected-content/provenance gate, TBD |
> | Determinism | Repeated section generation from identical source envelopes is stable. | Snapshot or round-trip tests, TBD |
> | Dependency mirror | Approved DAG-002 rows in `Dependencies.csv` remain ACTIVE and unmodified unless a later human-approved reconciliation changes the mirror. | Local dependency validation and final report |
>

### CLM-023 — Records

> ##### Records
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` for setup context.
> - `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` for semantic lensing, not engineering authority.
> - `Dependencies.csv` and `_DEPENDENCIES.md` as the approved DAG-006 local mirror/evidence surface.
> - Future implementation tests and review notes for section assembly, protected-content avoidance, professional-boundary wording, provenance/checksum preservation, deterministic output, and missing-data findings.
>

### CLM-024 — D-41 R5 T4 persisted-run procedure

> ##### D-41 R5 T4 persisted-run procedure
>
> 1. Accept a canonical project persistence envelope.
> 2. Read only embedded model-state and analysis-run records from
>    `project.run_history`.
> 3. Assemble backend sections with the existing DEL-08-06 path.
> 4. Verify warnings, assumptions, limitations, solver context, provenance, and
>    permitted analysis statuses survive.
> 5. Verify no project mutation, solver execution, human approval,
>    code-compliance result, or external/non-JSON partition decision is emitted.
> 6. Keep producer-side solver/rule/adapter bindings and rendered-report layout
>    as explicit residuals.

- **VER-001** — Validate the contract and review source parity, state/run/comparison/handoff coverage, SOW-024 report content, stable identities and hashes, units and provenance, deterministic ordering, missing-data findings, unsupported-target disclosures, protected-content and professional-authority boundaries, and every retained TBD or governed residual.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-08-06 State, comparison, and handoff report sections

> #### Guidance: DEL-08-06 State, comparison, and handoff report sections
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-06-DECL-003`.
>

### CLM-027 — Purpose

> ##### Purpose
>
> This deliverable exists to make state/run records, deterministic comparisons, and handoff manifests visible in auditable calculation reports while preserving the project boundary that software output is decision support, not professional validation. The source basis is `_CONTEXT.md`, SOW-024, OBJ-007, OBJ-016, OBJ-017, OBJ-018, `docs/SPEC.md` section 9, and the local approved DAG-002 dependency mirror.
>

### CLM-028 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Report from records, not hidden state | Treat model states, analysis runs, comparisons, and handoff manifests as source records referenced by stable IDs, hashes, checksums, provenance, review state, and privacy classification where available. | `docs/SPEC.md` section 9; `docs/TYPES.md`; `Dependencies.csv` |
> | Keep missing data visible | Missing solve-required or rule-check-required values should become explicit findings, warnings, limitations, unresolved TBDs, or human-review-needed findings. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4.3 and 9 |
> | Do not turn reports into approval records | State/run, comparison, and handoff sections may support professional review workflows but must not claim certification, sealing, approval, authentication, endorsement, code compliance, or professional reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` sections 4.3 and 9 |
> | Preserve source envelopes | Prefer references to source envelopes and manifests over copying private payloads or engineering values into public artifacts. | `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` sections 6 and 7 |
> | Separate public examples from private data | Public fixtures should be original or invented. Private project values, private rule-pack payloads, private library content, protected standards text, and proprietary formulas should not be copied into public report artifacts. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2, 3, and 7; `docs/SPEC.md` section 9 |
> | Treat handoff as traceability, not acceptance | Handoff manifests can support downstream modeling and professional validation workflows, but unsupported-target flags and external-prover metadata do not create automatic professional acceptance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-074 and SOW-075; `Dependencies.csv` rows DAG-002-E0866 through DAG-002-E0868 |
>

### CLM-029 — Considerations

> ##### Considerations
>
> - The report generator, audit manifest, warnings/provenance sections, result export, protected-content linter, private redaction/export controls, state/run records, comparison records, handoff package records, export workflow records, and external-prover metadata are all upstream evidence surfaces in `Dependencies.csv`. This setup pass preserves those edges as ACTIVE and does not reclassify them.
> - ASSUMPTION: the future implementation should assemble sections from schema-first records rather than from unstructured text scraping. This follows the architecture-basis injection and `docs/SPEC.md` section 9, but exact code interfaces are TBD.
> - Any comparison wording should remain diagnostic/audit-oriented. A deterministic comparison can show changes and deltas; it does not prove external validation or acceptance.
> - Any handoff wording should expose target mapping metadata and unsupported-target flags when available. It should not imply that a downstream external tool or human has approved the model.
> - Boundary notices should be stable enough for tests, but exact wording remains TBD until the professional-claims policy deliverable and report templates are accepted.
> - Protected-content linter output is review evidence only; a clean finding is not legal clearance or professional clearance.
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative direction |
> |---|---|
> | Detail vs protected/private leakage | Prefer identifiers, checksums, source notes, review state, and privacy classification over copying formulas, tables, private values, or private payloads. |
> | Human readability vs machine traceability | Preserve stable structured fields first; rendered prose can summarize but should not drop provenance or warning semantics. |
> | Comparison completeness vs false authority | Present deterministic comparison findings and unresolved mappings without declaring acceptance, validation, or compliance. |
> | Handoff usefulness vs unsupported-target risk | Expose unsupported or approximate target behavior instead of hiding loss of fidelity. |
> | Early implementation speed vs report safety | Keep exact APIs and layout TBD until source contracts are available; do not infer product code from decomposition text alone. |
>

### CLM-031 — Examples

> ##### Examples
>
> No authoritative examples are available in the accessible source set for this deliverable. Public examples or fixtures for later implementation should be invented or otherwise permitted, include provenance/review notes, and avoid protected standards text, protected tables, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, and real secrets.
>

### CLM-032 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | None | No source conflicts were identified during this P1/P2 setup pass. | N/A | N/A | N/A | N/A | TBD |
>

### CLM-033 — D-41 R5 T4 guidance

> ##### D-41 R5 T4 guidance
>
> Treat the persisted-run helper as a narrow read-side application-service
> binding. It proves that canonical records can reach backend report sections
> without dropping governed review context. It does not turn a stored record
> into an approval, select an external payload partition, render a report, or
> close solver/rule/adapter producer integrations.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-024 OBJ-007 OBJ-016 OBJ-017 OBJ-018 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
