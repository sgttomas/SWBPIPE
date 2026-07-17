---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-04
package_id: PKG-08
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-046]
package_objective_refs: [OBJ-007, OBJ-009]
---

# Scope of Work — DEL-08-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-08-04` in service of project scope [SOW-046] and package objectives [OBJ-007, OBJ-009].

- **OUT-001** — A schema-first machine-readable JSON result-envelope contract for review, regression comparison, report generation, GUI consumption, automation, and governed downstream adapters is produced while additional concrete export formats remain TBD.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-08-04 Result export format

> #### Datasheet: DEL-08-04 Result export format
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-04-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-08-04 |
> | Deliverable name | Result export format |
> | Package ID | PKG-08 |
> | Package name | Reporting, Audit, and Reproducibility |
> | Deliverable type | API_CONTRACT |
> | Scope item | SOW-046 |
> | Supported objectives | OBJ-007; OBJ-009 |
> | Setup status | Draft setup artifact; not implementation |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Draft setup value |
> |---|---|
> | Primary artifact family | Machine-readable result export contract for review, regression comparison, and downstream tooling. |
> | Baseline format | Schema-first JSON result envelopes. Source: `docs/_Registers/ScopeLedger.csv` row SOW-046; `_CONTEXT.md` Architecture Basis Injection. |
> | Additional export formats | TBD; this setup deliverable does not choose CSV, HDF5, neutral-file, spreadsheet, or external tool formats as final. |
> | Anticipated implementation artifacts | `schemas/results.schema.yaml`, exporter source, and tests. Source: `_CONTEXT.md` Anticipated Artifacts; not created or edited in this setup session. |
> | Envelope content categories | Result identity, model/run references, unit-aware value arrays, diagnostics, provenance, analysis status, warnings, and reproducibility references. Source: `docs/SPEC.md` sections 4.5, 7, 8, and 9; `execution/_Decomposition/SOFTWARE_DECOMP.md` architecture basis rows AB-00-03, AB-00-04, AB-00-06, and AB-00-07. |
> | Review boundary | Exports support review and comparison. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Source: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4. |
> | Unit boundary | Exported values must be unit-aware and dimensionally traceable; missing units are findings, not silent defaults. Source: `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2. |
> | Protected-data boundary | Public export contracts must not embed protected standards text, copied standards tables, proprietary formulas, private rule-pack payloads, or private project data by default. Source: `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - This session is setup/document production only; no schema file, exporter source, tests, docs outside this folder, or repository-level artifacts are modified.
> - The export baseline is a JSON result envelope contract. Any additional export format remains `TBD` until a later bounded decision or implementation brief.
> - Result exports must preserve the PKG-00 no-bypass baseline: adapters and downstream tools cannot bypass unit checks, diagnostics, provenance, public/private data boundaries, or professional-responsibility notices.
> - Mechanics results, user-rule-check results, and human review/approval states must remain distinguishable in exported data.
> - Exported diagnostics must carry enough structured information for review and regression triage without relying on prose-only warnings.
>

### CLM-006 — Construction

> ##### Construction
>
> The future result export contract is expected to include these setup-level components:
>
> | Component | Purpose | Boundary |
> |---|---|---|
> | Result envelope header | Identifies export schema version, result set ID, model/run reference, solver version reference, and creation context. | Does not imply validation, certification, or professional acceptance. |
> | Unit-aware result payloads | Carries displacement, rotation, force, moment, reaction, stress, ratio, or rule-check result values with unit metadata and dimensional categories. | No hidden unit defaults; code-specific values remain user/rule-pack supplied. |
> | Diagnostics block | Carries structured diagnostics with code, class, severity, source, affected object, message, remediation, and provenance. | Diagnostics must not be dropped by exporters or adapters. |
> | Provenance and reproducibility references | Links to model hash, manifest, rule-pack checksum, source/provenance notes, and input/run identifiers where available. | References private data safely; does not copy protected or private payloads into public artifacts. |
> | Analysis status fields | Distinguishes `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `HUMAN_REVIEW_REQUIRED`, and related statuses. | Must not emit automatic `CODE_COMPLIANT` or professional approval status. |
> | Regression comparison surface | Provides stable identifiers, deterministic ordering, and numeric/unit metadata suitable for comparison tests. | Exact comparison tolerances and tooling remain future validation work. |
> | Downstream tooling handoff | Provides a stable machine-readable contract for headless CLI, report generation, GUI results viewer, and adapters. | External transport and concrete non-JSON formats remain `TBD`. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, architecture basis IDs, and write-scope constraints.
> - `docs/_Registers/Deliverables.csv` row DEL-08-04 for artifact and objective mapping.
> - `docs/_Registers/ScopeLedger.csv` row SOW-046 for export baseline acceptance notes.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` rows AB-00-03, AB-00-04, AB-00-06, AB-00-07, and OI-004 for envelope/API/no-bypass and format-TBD constraints.
> - `docs/SPEC.md` sections 4.5, 7, 8, 9, and 11 for deterministic results, diagnostics, reporting, validation, and acceptance semantics.
> - `docs/TYPES.md` sections 4 and 8 for analysis-status vocabulary and `Result` object boundary.
> - `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 for public/private data and report/export boundaries.
> - `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-UNIT, OPS-K-RULE, OPS-K-PRIV, OPS-K-AUTH, and OPS-K-AGENT invariants.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-08-04 Result export format

> #### Specification: DEL-08-04 Result export format
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-04-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines the setup specification for a machine-readable result export contract. The baseline is a schema-first JSON result envelope suitable for review, regression comparison, and downstream tooling.
>
> This setup run does not implement exporter code, edit `schemas/results.schema.yaml`, create tests, choose additional export formats as final, or modify documentation outside this deliverable folder. Those remain future implementation work under bounded Type 2 briefs.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-08-04-R1 | The result export baseline shall be a schema-first JSON result envelope. | `docs/_Registers/ScopeLedger.csv` row SOW-046; `_CONTEXT.md` Architecture Basis Injection |
> | DEL-08-04-R2 | Additional concrete export formats shall remain `TBD` unless later approved and scoped; this deliverable shall not finalize CSV, spreadsheet, FEA, HDF5, or other external formats. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-004; sealed brief acceptance notes |
> | DEL-08-04-R3 | Result envelopes shall preserve units and dimensional metadata for exported values and shall not rely on hidden unit defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/SPEC.md` section 8 |
> | DEL-08-04-R4 | Result envelopes shall carry or reference provenance and reproducibility metadata, including model/run identity, solver version basis, rule-pack checksum where applicable, warnings, assumptions, and source notes. | `docs/SPEC.md` sections 4.5 and 8; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
> | DEL-08-04-R5 | Result envelopes shall carry structured diagnostics using the PKG-00 baseline fields: code, class, severity, source, affected object, message, remediation, and provenance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` section 7 |
> | DEL-08-04-R6 | Exported statuses shall distinguish mechanics solved, user-rule checked, rule-input incomplete, human-review required, and human-approved record states where present. | `docs/TYPES.md` sections 4 and 8; `docs/CONTRACT.md` OPS-K-AUTH-1 |
> | DEL-08-04-R7 | Result exports shall not claim certification, sealing, professional approval, authentication, or automatic engineering code compliance (PRD §21.2). | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4 |
> | DEL-08-04-R8 | Result exports and public examples shall not embed protected standards text, copied standards tables, proprietary formulas, private rule-pack payloads, or private project data by default. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 |
> | DEL-08-04-R9 | Exporters and adapters shall not bypass validation, unit checks, diagnostics, provenance handling, report controls, or public/private data-boundary checks. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-07; `docs/SPEC.md` section 1 |
> | DEL-08-04-R10 | Export ordering and identifiers should be deterministic enough for regression comparison of equivalent result sets. | `docs/SPEC.md` sections 4.5 and 9; `docs/_Registers/ScopeLedger.csv` row SOW-046 |
> | DEL-08-04-R11 | Missing solve-required, rule-check-required, provenance, or unit metadata shall be represented as diagnostics/findings, not silently filled with defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` section 7 |
> | DEL-08-04-R12 | The contract shall support downstream consumption by report generation, GUI results review, headless automation, and import/export adapters without creating a bypass around governed result envelopes. | `docs/_Registers/Deliverables.csv` rows DEL-07-05, DEL-08-01, DEL-10-02, DEL-10-05; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-07 |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or basis | Applicability | Status |
> |---|---|---|
> | JSON Schema 2020-12 | Baseline schema technology for public result envelope contracts. | Architecture basis; schema file editing is outside this setup write scope. |
> | Schema-first command/query/job result envelope baseline | Result export/API boundary and no-bypass contract. | Required by PKG-00 architecture basis referenced in `_CONTEXT.md`. |
> | Canonical JSON / JCS-compatible canonicalization | Relevant when exported JSON payloads or manifests are hashed for reproducibility. | Hash implementation belongs primarily to DEL-08-02 / future implementation; this deliverable shall preserve compatible references. |
> | OpenPipeStress invariant catalog | IP, data, units, rule-pack, privacy, professional-responsibility, and agent-boundary constraints. | Binding project governance draft. |
>
> No protected engineering code, standard clause text, standards table, commercial example, or proprietary rule content is used as an authority in this setup artifact.
>

### CLM-013 — Verification

> ##### Verification
>
> Future implementation acceptance should include the following checks:
>
> | Verification ID | Check | Expected result |
> |---|---|---|
> | V-1 | Validate a representative JSON result envelope against the result schema. | Schema validation passes and required envelope fields are present. |
> | V-2 | Export a result set with unit-bearing displacements, rotations, forces, moments, reactions, stresses, and ratios. | Each value family carries explicit unit/dimensional metadata or a blocking diagnostic. |
> | V-3 | Export a run with missing rule-pack input or missing provenance. | Export includes structured diagnostics/findings instead of silent defaults. |
> | V-4 | Export a run with rule-pack references. | Envelope records rule-pack identity/version/checksum/source status without copying private formulas or protected values. |
> | V-5 | Re-export an equivalent result set. | Stable identifiers and ordering allow deterministic regression comparison. |
> | V-6 | Route the export through an adapter or downstream tool. | Unit, provenance, diagnostics, and professional-boundary fields are preserved. |
> | V-7 | Scan public fixtures/templates for protected content and approval/compliance claims. | Protected/private data and certification/compliance claims are absent or blocked. |
> | V-8 | Generate a report or GUI result view from the export. | Consumer can distinguish mechanics, user-rule, diagnostics, and human-review-required statuses. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> This setup deliverable produces:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/*`
> - `_STATUS.md`
>
> Future implementation artifacts anticipated by the register are `schemas/results.schema.yaml`, exporter source, and tests; they are not created or edited in this setup session.

- **AC-001** — The contract preserves explicit units and dimensional metadata, structured diagnostics, provenance and reproducibility references, stable identifiers and ordering, mechanics/rule/human status distinctions, public/private content boundaries, and no-bypass or professional-authority constraints.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-08-04 Result export format

> #### Procedure: DEL-08-04 Result export format
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-04-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> This procedure describes how to maintain the setup artifact for the result export format and how a later implementation brief should convert the setup specification into a governed result export contract.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Why it matters | Source |
> |---|---|---|
> | Deliverable identity and scope for DEL-08-04 | Keeps the work bounded to SOW-046, OBJ-007, and OBJ-009. | `_CONTEXT.md` |
> | PKG-00 result-envelope/API no-bypass architecture basis | Establishes schema-first envelopes, diagnostics, unit/provenance boundaries, and adapter no-bypass constraints. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03, AB-00-06, AB-00-07 |
> | Unit system and dimensional-analysis core contract | Result exports need unit-aware values and dimensional checks. | `docs/CONTRACT.md` OPS-K-UNIT-1; upstream DEL-02-02 |
> | Canonical domain model and result object semantics | Result exports need stable model/result references and object vocabulary. | `docs/TYPES.md` section 8; upstream DEL-02-01 |
> | Audit manifest and model hash | Reproducible exports should carry or reference run/model manifest and hash metadata. | DEL-08-02; `docs/_Registers/ScopeLedger.csv` row SOW-039 |
> | Solver/load/stress result producers | Export format must consume mechanics outputs without redefining solver behavior. | PKG-04 and PKG-05 deliverables |
> | Rule-pack lifecycle and checksum handling | Rule-check exports need rule-pack identity/version/checksum without copying private/protected payloads. | DEL-06-04; `docs/SPEC.md` sections 6 and 8 |
> | Headless runner and adapter surfaces | Downstream automation and adapter workflows need the governed envelope contract. | DEL-10-02 and DEL-10-05 |
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm the current brief is limited to DEL-08-04 and does not authorize edits to `schemas/results.schema.yaml`, exporter source, tests, or documentation outside this deliverable folder.
> 2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md`, and the register rows for DEL-08-04 and SOW-046.
> 3. Treat the baseline export as a schema-first JSON result envelope. Record additional formats as `TBD` unless a later human-approved scope change chooses them.
> 4. Define future envelope responsibilities at setup level: result identity, model/run references, unit-aware values, diagnostics, provenance, analysis status, warnings, and reproducibility references.
> 5. Keep mechanics results, user-rule-check outcomes, missing-data findings, and human review/approval records separate in terminology and future fields.
> 6. Confirm that no public artifact embeds protected standards text, standards tables, proprietary formulas, private rule-pack payloads, private project data, or automatic compliance/approval claims.
> 7. Record dependency edges only when source documents state an explicit anchor, prerequisite, interface, or handoff. Do not add structural adjacency or coordination-only edges.
> 8. For a later implementation brief, create or edit schemas/exporter/tests only inside that later brief's write scope and run the applicable schema, unit, diagnostics, protected-content, and regression gates.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four-document setup kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist with stable default sections. |
> | Scope boundary | No files outside `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/**` are edited. |
> | Format boundary | JSON result envelope baseline is stated; additional export formats remain `TBD`. |
> | Unit/provenance/diagnostic boundary | Unit-aware values, provenance, diagnostics, and no-bypass adapter constraints are visible in the specification. |
> | Protected/private boundary | No protected standards text, copied tables, proprietary formulas, private rule-pack payloads, private project data, or certification/compliance claims are introduced. |
> | Dependency register | `Dependencies.csv` validates against v3.1 schema and `_DEPENDENCIES.md` counts match the CSV. |
> | Lifecycle state | `_STATUS.md` is `SEMANTIC_READY` only after the four docs, semantic matrix, lensing register, and dependency extraction setup gates pass. |
>

### CLM-021 — Records

> ##### Records
>
> The setup run should leave these records in the deliverable folder:
>
> - Four production documents: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
> - Semantic artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`
> - Dependency artifacts: `Dependencies.csv`, `_DEPENDENCIES.md`
> - Run records: `_run_records/TASK_RUN_2026-04-30_*.md`
> - Lifecycle status: `_STATUS.md`

- **VER-001** — Validate the contract and review source parity, schema-first JSON baseline, retained TBD format choices, unit and diagnostic completeness, deterministic comparison identity, provenance and rule-pack references, adapter no-bypass behavior, protected-content limits, and every governed residual.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-08-04 Result export format

> #### Guidance: DEL-08-04 Result export format
>

### CLM-023 — Purpose

> ##### Purpose
>
> The result export format exists to make solver and rule-check outputs reviewable, reproducible, comparable, and consumable by downstream tools without losing the project boundaries that make OpenPipeStress code-neutral. The setup baseline is a schema-first JSON result envelope; other export formats remain `TBD`.
>

### CLM-024 — Principles

> ##### Principles
>
> - Preserve the result envelope as the governed contract. Downstream tools may consume exports, but they should not bypass unit checks, diagnostics, provenance, public/private data handling, or report controls.
> - Keep the export format review-oriented. A result export can support professional review; acceptance and professional judgment remain with the responsible engineer.
> - Treat units as data, not context. Every exported numeric result needs explicit unit and dimensional meaning, or a diagnostic explaining why it is incomplete.
> - Keep diagnostics structured. Review and regression tools need stable diagnostic codes/classes/severities and affected-object references, not prose-only warnings.
> - Carry reproducibility references without copying private payloads. Model hashes, manifest IDs, solver versions, and rule-pack checksums are appropriate references; protected standards text, proprietary formulas, private rule-pack internals, and owner data are not public export payloads by default.
> - Prefer deterministic ordering and stable identifiers for regression comparison.
> - Use `TBD` for unresolved format decisions. Do not imply that a spreadsheet, CSV, FEA handoff, or other external export has been selected.
>

### CLM-025 — Considerations

> ##### Considerations
>
> | Topic | Guidance |
> |---|---|
> | Schema-first baseline | Start from JSON result envelopes because they align with the architecture basis and can be validated before exporter code exists. |
> | Results vs reports | Exports are machine-readable result contracts; reports are human-facing review artifacts. Reports may consume exports, but report wording and protected-content controls remain their own workflow. |
> | Mechanics vs rule checks | Mechanics result values and user-rule-check outcomes should remain separately identifiable. A user-rule check is not professional approval. |
> | Diagnostics | Export consumers should be able to filter by diagnostic class, severity, source, affected object, remediation, and provenance. |
> | Regression comparison | Stable object identifiers, result-station identifiers, load-case/combination references, deterministic ordering, and unit metadata are more important than display formatting. |
> | Interoperability | Downstream tools need a governed envelope first. Concrete external formats and transport protocols are later decisions. |
> | Privacy and protected data | Public fixtures should use invented or permissively licensed data only. User-private exports remain user controlled and should not become public defaults. |
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Setup position |
> |---|---|
> | Single JSON envelope vs many early formats | Use the JSON envelope as the baseline. Additional formats remain `TBD` to avoid premature commitments and governance bypasses. |
> | Verbose metadata vs compact exports | Preserve units, diagnostics, provenance, and status semantics even when compactness is desirable. Reviewability takes precedence over minimal payload size. |
> | Human-readable labels vs stable machine keys | Include stable identifiers for comparison and downstream tooling. Human-readable labels may supplement them but should not be the only references. |
> | Private rule-pack detail vs reproducibility references | Export identifiers, versions, checksums, source notes, and redistribution status; do not copy private formulas or protected values into public artifacts. |
> | Report convenience vs professional boundary | Avoid report/export wording that could be read as approval or compliance. Human review remains external to the software result. |
>

### CLM-027 — Examples

> ##### Examples
>
> The following are setup-level examples of export concerns, not final schema definitions:
>
> | Concern | Acceptable setup direction | Not acceptable in public baseline |
> |---|---|---|
> | A displacement result | Unit-aware value with node/result reference, load-case or combination basis, and source run metadata. | Numeric value without unit metadata or affected-object reference. |
> | A rule-check outcome | User-rule-check result with rule-pack ID/version/checksum and status distinguishing incomplete inputs, checked, or failed. | Automatic `CODE_COMPLIANT` status or copied protected formula text. |
> | A warning | Structured diagnostic with class, severity, source, affected object, message, remediation, and provenance. | Prose-only warning that cannot be traced or compared. |
> | A downstream export | Adapter consumes governed JSON envelope and preserves units/provenance/diagnostics. | Adapter-specific shortcut that drops diagnostics or private-data handling. |
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No unresolved source conflicts were found during this setup run. Open format choices remain `TBD` by scope, not a source conflict.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-046 OBJ-007 OBJ-009 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
