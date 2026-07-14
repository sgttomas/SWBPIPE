---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-01
package_id: PKG-06
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-016, SOW-042]
package_objective_refs: [OBJ-005]
---

# Scope of Work — DEL-06-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-06-01` in service of project scope [SOW-016, SOW-042] and package objectives [OBJ-005].

- **OUT-001** — A rule-pack-schema contract covering identity and versioning, provenance and redistribution status, canonical payload checksums, required inputs, declarative formula and allowable slots, check criteria, diagnostics, units, and professional-boundary metadata is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-06-01 Rule-pack schema

> #### Datasheet: DEL-06-01 Rule-pack schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-01-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-06-01 |
> | Package ID | PKG-06 |
> | Package | Rule Packs and User-Supplied Code Check Engine |
> | Type | DATA_MODEL_CHANGE |
> | Scope items | SOW-016; SOW-042 |
> | Objective | OBJ-005 |
> | Anticipated artifacts | `schemas/rule_pack.schema.yaml`; `docs/SPEC.md` update |
> | Current evidence status | Setup evidence only; no product implementation |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Required setup meaning | Source |
> |---|---|---|
> | Rule-pack artifact scope | Private user-defined stress checks, allowables, formulas, required inputs, and pass/fail criteria. | ScopeLedger rows SOW-016, SOW-042 |
> | Schema baseline | Future public schema/interchange work must align with JSON Schema 2020-12. | ContextBudgetQA row DEL-06-01; AB-00-04 |
> | Versioning | Rule packs must carry stable identity and version metadata. | SOW-042; OPS-K-RULE-3 |
> | Checksum basis | JSON rule-pack payloads use canonical JSON with JCS-compatible hashing where checksums are computed. | AB-00-04; SOW-039 |
> | Provenance and source notes | Rule-pack values, formulas, and source basis records must carry source/provenance metadata. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-RULE-3 |
> | Redistribution status | Rule packs must be explicitly marked public/private and redistribution status must be recorded. | SOW-042; OPS-K-RULE-3 |
> | Private-data boundary | Private rule packs are user-owned data and should not be committed or transmitted publicly by default. | OPS-K-PRIV-1; SOW-042 notes |
> | Protected-content boundary | Public artifacts must not reproduce protected standards text, tables, examples, copied formulas, or proprietary allowables. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-RULE-1 |
> | Unit handling | Rule-pack inputs, formulas, and allowables must be unit-aware and dimensionally checked by later implementation work. | OPS-K-UNIT-1 |
> | Missing data behavior | Missing rule-check-required values become explicit findings, not silent defaults. | OPS-K-DATA-2 |
> | Evaluator boundary | Formula representation must remain declarative and sandbox-compatible; exact expression grammar/library remains `TBD`. | SOW-045 note; OI-006 |
> | Professional boundary | A rule-pack pass/fail result is not professional certification, sealing, or engineering approval. | OPS-K-AUTH-1; OPS-K-MECH-2 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - This setup pass does not create or edit `schemas/rule_pack.schema.yaml`, `docs/SPEC.md`, examples, evaluator code, registry code, or public/private storage controls.
> - No protected standards text, protected formulas, proprietary commercial rules, code tables, material allowables, SIF/flexibility tables, or project-specific engineering values are included.
> - Any future public example rule pack must use invented non-code values and carry an explicit non-engineering notice.
> - Private rule packs are user-supplied artifacts. Public repository content may define schema slots and validation behavior, but not the user's protected design basis.
> - Exact expression grammar/library, public API transport, physical project package/container, private rule-pack encryption defaults, and detailed storage controls remain `TBD` unless later ruled by the human project authority.
>

### CLM-006 — Construction

> ##### Construction
>
> The future rule-pack schema should be evaluated for these descriptive record groups:
>
> | Record group | Fields or slots to resolve later |
> |---|---|
> | Identity | Rule-pack ID, display name, namespace, schema version, rule-pack version, lifecycle status, author/contributor role, and generated/modified timestamps. Exact naming rules are `TBD`. |
> | Provenance | Source type, source note, citation pointer, contributor certification pointer, review disposition, import path, and source hash where available. |
> | Redistribution | Public/private marker, redistribution status, license/reference pointer, quarantine marker, and public-example safety marker. |
> | Checksum | Canonical payload hash algorithm, hash value, hash scope, manifest reference for non-JSON assets, and invalidation behavior on content change. |
> | Required inputs | Input IDs, dimensional categories, unit constraints, source requirements, missing-value behavior, and applicability conditions. |
> | Variables/results | Bindings to solver result fields, load-case or stress-result categories, units, and unavailable-result diagnostics. |
> | Formula declarations | Declarative expression slots, variable bindings, dimensional expectations, evaluation status, and sandbox compatibility flags. Actual protected formulas are excluded. |
> | Allowable slots | User-supplied allowable placeholders with units, provenance, source note, redistribution status, and completeness status. Actual allowable values are excluded from setup evidence. |
> | Checks and criteria | Check IDs, applicability conditions, formula references, limit references, comparison operator categories, pass/fail/incomplete statuses, and diagnostic outputs. |
> | Diagnostics | Structured findings compatible with AB-00-06 for missing inputs, unit mismatch, provenance warning, redistribution warning, protected-content warning, evaluator error, and rule-check blocking. |
> | Professional boundary | Reportable status wording that distinguishes mechanics solved, user-rule checked, and human professional approval. |
>

### CLM-007 — References

> ##### References
>
> - `docs/_Registers/Deliverables.csv` row DEL-06-01
> - `docs/_Registers/ScopeLedger.csv` rows SOW-016 and SOW-042
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-06-01
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
> - `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-RULE-1, OPS-K-RULE-3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-MECH-2, OPS-K-PRIV-1, OPS-K-AGENT-1..4

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-06-01 Rule-pack schema

> #### Specification: DEL-06-01 Rule-pack schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-01-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines setup evidence for a future rule-pack schema covering metadata, required inputs, formula declarations, user-supplied allowables, pass/fail/incomplete criteria, checksum fields, provenance/source notes, redistribution status, and professional-boundary markings.
>
> This setup pass does not implement `schemas/rule_pack.schema.yaml`, update `docs/SPEC.md`, create public examples, define an expression grammar, implement a sandboxed evaluator, publish protected rule content, or claim engineering code compliance.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Evidence basis | Verification approach |
> |---|---|---|---|
> | REQ-06-01-001 | The future schema shall use JSON Schema 2020-12 conventions for public schema/interchange artifacts. | ContextBudgetQA row DEL-06-01; AB-00-04 | Schema meta-validation in a later implementation pass. |
> | REQ-06-01-002 | The future schema shall include rule-pack identity, schema version, rule-pack version, lifecycle/status, and source-note metadata. | SOW-042; OPS-K-RULE-3 | Required-field schema validation. |
> | REQ-06-01-003 | The future schema shall require provenance metadata for rule-pack values, allowable slots, formulas, and source bases. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-RULE-3 | Positive and negative provenance fixtures. |
> | REQ-06-01-004 | The future schema shall record public/private classification and redistribution status for each rule pack and public-example candidate. | SOW-042; OPS-K-RULE-3; OPS-K-PRIV-1 | Redistribution-status fixture validation. |
> | REQ-06-01-005 | The future schema shall include checksum metadata identifying algorithm, hash scope, hash value, and canonicalization basis for JSON payloads. | SOW-042; AB-00-04 | Canonical JSON/JCS-compatible hash fixture validation. |
> | REQ-06-01-006 | The future schema shall support required-input declarations with units, dimensional categories, source/provenance requirements, and missing-value findings. | SOW-016; OPS-K-DATA-2; OPS-K-UNIT-1 | Missing-input and unit-mismatch fixtures. |
> | REQ-06-01-007 | The future schema shall support formula declaration slots without embedding protected standards text, protected formulas, copied code equations, or arbitrary executable code. | SOW-016; OPS-K-IP-1; OPS-K-IP-3; OPS-K-RULE-2 | Protected-content review and evaluator-boundary review. |
> | REQ-06-01-008 | The future schema shall support user-supplied allowable slots with units, provenance, source notes, redistribution status, and completeness status. | SOW-016; OPS-K-DATA-1; OPS-K-DATA-3 | Allowable-slot negative fixture validation. |
> | REQ-06-01-009 | The future schema shall represent pass/fail/incomplete criteria for user-rule checks while preserving the distinction between user-rule checked and professionally approved. | OBJ-005; OPS-K-MECH-2; OPS-K-AUTH-1 | Status-model and report-wording review. |
> | REQ-06-01-010 | The future schema shall emit or support structured diagnostics for rule-check blocking, provenance warning, unit mismatch, protected-content warning, evaluator error, and redistribution warning conditions. | AB-00-06; OPS-K-DATA-2; OPS-K-IP-3 | Diagnostic fixture validation. |
> | REQ-06-01-011 | The future schema shall require public examples to use invented non-code values and explicit non-engineering notices. | OPS-K-RULE-1; SOW-016 note | Public-example protected-content gate. |
> | REQ-06-01-012 | The future schema shall not treat agent-generated setup text as engineering authority, standards authority, legal conclusion, or professional approval. | OPS-K-AGENT-1; OPS-K-AGENT-4 | Review checklist. |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or policy source | Use in this setup evidence |
> |---|---|
> | OpenPipeStress CONTRACT | Governs protected-content, provenance, privacy, unit, rule-pack, professional-boundary, and agent-output constraints. |
> | SOFTWARE_DECOMP revision 0.7 | Provides package/deliverable scope, accepted architecture basis, open issues, and remaining `TBD` boundaries. |
> | JSON Schema 2020-12 | Required baseline for future public schema/interchange artifacts. Exact file layout and code-generation tooling remain `TBD`. |
> | Canonical JSON / JCS-compatible hashing | Required checksum basis where JSON payloads are hashed. Exact implementation library remains `TBD`. |
> | External engineering standards | May be referenced by user-owned private rule packs, but their protected text, tables, examples, formulas, and values are not public project content. |
>

### CLM-013 — Verification

> ##### Verification
>
> Future implementation verification should include:
>
> - JSON Schema 2020-12 meta-validation for the rule-pack schema artifact.
> - Positive fixtures for identity/version metadata, checksum metadata, required input declarations, provenance/source notes, redistribution status, unit metadata, and rule-check criteria.
> - Negative fixtures for missing units, missing provenance, missing source notes, missing redistribution status, missing required inputs, checksum mismatch, and private/public classification gaps.
> - Protected-content review confirming public fixtures do not contain standards text, copied protected formulas, proprietary allowables, protected tables, or project-specific engineering values.
> - Canonical JSON/JCS-compatible hash verification for JSON rule-pack payloads and manifest hash behavior for any non-JSON assets.
> - Status wording review confirming no output claims certification, sealing, engineering approval, or code compliance for reliance.
>

### CLM-014 — Documentation

> ##### Documentation
>
> Expected future product artifacts remain:
>
> - `schemas/rule_pack.schema.yaml`
> - `docs/SPEC.md` rule-pack schema section update
> - invented non-code fixture notes, if examples are later authorized
> - checksum/provenance validation notes
> - protected-content and redistribution review evidence

- **AC-001** — The contract preserves the accepted public/private and protected-content boundaries, declarative sandbox-compatible posture, explicit missing-data behavior, canonical JSON/JCS-compatible hash basis, and retained grammar, storage, encryption, and packaging TBDs without inventing protected formulas, allowables, code content, defaults, or professional approval.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-06-01 Rule-pack schema

> #### Procedure: DEL-06-01 Rule-pack schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-01-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and verify the future rule-pack schema artifact from the current setup evidence. It is operational guidance only; this setup run does not create product schema files or modify repo-level product documentation.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> - Read the sealed deliverable context in `_CONTEXT.md`.
> - Confirm scope coverage for SOW-016 and SOW-042 in `docs/_Registers/ScopeLedger.csv`.
> - Confirm the deliverable row and context note for DEL-06-01 in `docs/_Registers/Deliverables.csv` and `docs/_Registers/ContextBudgetQA.csv`.
> - Apply applicable invariants from `docs/CONTRACT.md`, especially protected-content, provenance, rule-pack, unit, privacy, and professional-boundary invariants.
> - Preserve the architecture basis for JSON Schema 2020-12, canonical JSON/JCS-compatible hashing, diagnostics, no-bypass validation, and layered validation gates.
> - Stop and escalate if future work requires protected standards text, protected formulas, proprietary allowables, or private rule-pack contents to be committed publicly.
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm that the work is limited to rule-pack schema structure and supporting documentation.
> 2. Define the schema envelope for rule-pack identity, schema version, rule-pack version, lifecycle/status, source notes, and contributor/review metadata.
> 3. Define provenance and redistribution record groups for every rule-pack value class, including formulas, allowable slots, required inputs, and public-example candidates.
> 4. Define checksum metadata fields for algorithm, hash scope, canonicalization basis, hash value, and invalidation behavior.
> 5. Define required-input declarations with dimensional category, allowed unit metadata, source/provenance requirement, missing-value behavior, and applicability conditions.
> 6. Define declarative formula slots and variable bindings without specifying protected formulas or arbitrary executable code.
> 7. Define user-supplied allowable slots with units, provenance, source notes, redistribution status, and completeness flags, without bundling actual protected values.
> 8. Define check criteria fields for applicability, formula references, limit references, comparison category, pass/fail/incomplete outcome, and diagnostic emission.
> 9. Define diagnostic categories for missing inputs, missing provenance, missing units, checksum mismatch, redistribution warning, protected-content warning, evaluator error, and rule-check blocking.
> 10. Add explicit public/private and professional-boundary wording so user-rule checked status is not confused with human professional approval.
> 11. Build invented non-code fixtures only if a later brief authorizes example work; otherwise keep fixture content `TBD`.
> 12. Verify that no repo-level product artifacts outside the assigned deliverable folder are edited by setup/document production work.
>

### CLM-020 — Verification

> ##### Verification
>
> Use these checks for a future implementation pass:
>
> - Schema artifact validates against JSON Schema 2020-12.
> - Required metadata, provenance, redistribution, checksum, units, input, formula, allowable, check, and diagnostic fields are present or conditionally required as specified.
> - Negative fixtures produce explicit findings for missing values, missing provenance, missing units, checksum mismatch, and redistribution gaps.
> - Protected-content review finds no standards text, copied protected formulas, protected tables, proprietary allowables, or real code-derived public examples.
> - Checksum fixtures demonstrate canonical JSON/JCS-compatible behavior for JSON rule-pack payloads.
> - Report/status wording preserves the mechanics-solved, user-rule-checked, and human-approved distinction.
>

### CLM-021 — Records

> ##### Records
>
> Retain these records within the deliverable or later implementation evidence:
>
> - Updated setup documents: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
> - Semantic evidence: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`
> - Dependency evidence: `Dependencies.csv`, `_DEPENDENCIES.md`
> - Run records under `_run_records/`
> - Future schema validation logs and protected-content/provenance review records when implementation begins

- **VER-001** — Validate the contract and review source parity, schema record groups, identity/version/checksum/provenance fields, required-input and unit handling, evaluator and redistribution boundaries, diagnostics, invented-example constraints, and every retained governed residual.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-06-01 Rule-pack schema

> #### Guidance: DEL-06-01 Rule-pack schema
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-023 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-01-DECL-003`.
>

### CLM-024 — Purpose

> ##### Purpose
>
> The rule-pack schema exists to let users evaluate solver results against user-owned design bases without placing protected standards content or proprietary engineering values in the public project. The schema should describe the artifact envelope, metadata, provenance, units, required inputs, formula declarations, allowables, criteria, checksums, and statuses needed for later rule evaluation.
>

### CLM-025 — Principles

> ##### Principles
>
> - Keep public mechanics and public schema structure separate from private code-specific rule content.
> - Treat rule packs as user-supplied data artifacts. The public repository may define slots, validation behavior, and example-safe patterns, but not real protected rule content.
> - Require source notes, provenance, redistribution status, and checksum metadata before a rule pack can be treated as traceable.
> - Treat missing required inputs, missing allowables, missing units, missing provenance, and unresolved redistribution status as explicit findings.
> - Preserve unit awareness and dimensional checking through the schema shape even before the evaluator is implemented.
> - Preserve the professional boundary: a rule-pack result can support review, but it is not certification, sealing, or professional approval.
> - Mark exact expression grammar/library, private storage/encryption behavior, and physical project packaging as `TBD` unless a later human-approved brief resolves them.
>

### CLM-026 — Considerations

> ##### Considerations
>
> The schema will sit between solver/stress results and the later rule evaluator. It should therefore avoid coupling to a specific commercial code while still being structured enough for deterministic validation. The schema should identify variables, dimensional expectations, required inputs, allowable slots, and check criteria, but should not encode protected equations or source text.
>
> Checksum handling should bind to the content actually evaluated. For JSON payloads, the accepted architecture basis points to canonical JSON with JCS-compatible hashing. If future rule packs include non-JSON assets, those assets should be covered by manifest hashes rather than silently excluded.
>
> Provenance should be more than a free-text note where possible. Future schema work should distinguish source type, source pointer, contributor/reviewer role, review disposition, license or redistribution status, and quarantine status for suspected protected content.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Schema flexibility vs. evaluator safety | Favor a declarative structure that can later be sandboxed and unit checked. Arbitrary executable code is outside the rule-pack boundary. |
> | Public examples vs. data-boundary risk | Public examples should use invented non-code values only. Real code-derived examples belong outside the public repository unless lawful redistribution is proven and accepted. |
> | Minimal metadata vs. auditability | Favor explicit metadata and checksum fields because reports and audit manifests depend on traceable rule-pack identity. |
> | Immediate completeness vs. unresolved architecture choices | Record unresolved grammar, storage, encryption, and packaging details as `TBD` rather than embedding premature choices. |
> | Pass/fail convenience vs. professional responsibility | Use pass/fail/incomplete as user-rule check statuses only. Do not imply code compliance or professional acceptance. |
>

### CLM-028 — Examples

> ##### Examples
>
> Public example rule packs, when later authorized, should:
>
> - use invented rule names, invented source labels, and invented non-engineering values;
> - include a clear non-engineering notice;
> - avoid standards names, clause numbers, copied examples, copied equations, protected tables, and real material allowables;
> - demonstrate schema mechanics such as provenance, units, missing-input diagnostics, and checksum fields without suggesting technical adequacy for design use.
>
> Private user rule packs may contain user-owned or lawfully licensed design basis content, but those private artifacts are not public repository content and should be handled under the private-data controls owned by later PKG-12 work.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No source conflict was found during setup drafting. Open decisions remain `TBD` rather than conflicts:
>
> | Item | Open decision | Current handling |
> |---|---|---|
> | OI-006 | Exact rule-pack expression grammar/library | `TBD`; schema should remain declarative and sandbox-compatible. |
> | OI-010 | Private rule-pack encryption default | `TBD`; defer to security/privacy work and human ruling. |
> | OI-011 | Physical project package/container | `TBD`; checksum schema should not assume a container format. |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-016 SOW-042 OBJ-005 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
