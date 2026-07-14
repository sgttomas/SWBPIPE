---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-04
package_id: PKG-05
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-047]
package_objective_refs: [OBJ-005, OBJ-011]
---

# Scope of Work — DEL-05-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-05-04` in service of project scope [SOW-047] and package objectives [OBJ-005, OBJ-011].

- **OUT-001** — An analysis-status-semantics contract defining explicit model-incomplete, mechanics-solved, rule-inputs-incomplete, user-rule checked or failed, human-review-required, and externally recorded human-approval states with schema-first result interfaces is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-05-04 Analysis status semantics

> #### Datasheet: DEL-05-04 Analysis status semantics
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-05-04 |
> | Name | Analysis status semantics |
> | Package | PKG-05 Loads, Load Cases, and Stress Recovery |
> | Type | DATA_MODEL_CHANGE |
> | Scope items | SOW-047 |
> | Objectives | OBJ-005, OBJ-011 |
> | Context envelope | S |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Evidence value | Source |
> |---|---|---|
> | Primary subject | Explicit result/status semantics that separate mechanics solve state, user rule-pack check state, missing input state, and human acceptance state. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` row DEL-05-04 |
> | Mechanics status | `MECHANICS_SOLVED` describes completed numerical mechanics computation for a model snapshot; it is not a code-compliance statement. | `docs/TYPES.md` section 4; `docs/architecture/analysis_status_semantics.md` Usage Rules |
> | Incomplete-data statuses | `MODEL_INCOMPLETE` and `RULE_INPUTS_INCOMPLETE` distinguish solve-required physical data gaps from rule-pack-required user/code data gaps. | `docs/TYPES.md` section 4; OPS-K-DATA-2 |
> | Rule-pack statuses | `USER_RULE_CHECKED` and `USER_RULE_FAILED` describe user-rule-pack evaluation outcomes using user data and rule definitions. | `docs/TYPES.md` section 4; `docs/architecture/analysis_status_semantics.md` Authority Boundary |
> | Human review status | `HUMAN_REVIEW_REQUIRED` is reportable for professional use; `HUMAN_APPROVED_FOR_PROJECT` is human-record-only and not emitted automatically by software. | `docs/TYPES.md` section 4; OPS-K-AUTH-1; OPS-K-AUTH-2 |
> | Result-envelope interface | Status fields belong with schema-first command/query/job result envelopes and diagnostics, preserving mechanics/rule/human distinctions. Implemented evidence exists in `schemas/analysis_status.schema.yaml`, `schemas/analysis_boundary.schema.yaml`, `schemas/results.schema.yaml`, `api/api_boundary_contract.yaml`, and downstream result/report/headless surfaces. | AB-00-03; AB-00-06; `tests/test_analysis_status_schema.py`; `tests/test_analysis_boundary_schema.py`; `tests/test_results_schema.py`; `tests/test_api_boundary_contract.py` |
> | Human acceptance workflow | TBD. External human acceptance records and storage/presentation ownership remain outside this deliverable-local kit. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-007; `docs/architecture/analysis_status_semantics.md` Remaining TBDs |
>

### CLM-004 — Conditions

> ##### Conditions
>
> - Status records describe what software computed or found; they must not claim certification, sealing, approval, authentication, or professional code compliance. Source: OPS-K-AUTH-1 and OPS-K-MECH-2.
> - Missing solve-required or rule-check-required values must be explicit findings and never silent defaults. Source: OPS-K-DATA-2.
> - Human acceptance records, if used later, must bind to specific model, rule-pack, result, and report hashes and must not survive content changes without re-review. Source: OPS-K-AUTH-2.
> - Reports must disclose solver/version, rule-pack checksum, warnings, assumptions, limitations, source/provenance notes, and professional-boundary notices. Source: OPS-K-REPORT-1.
> - Setup artifacts are drafts/proposals until accepted by a human gate. Source: OPS-K-AGENT-4.
>

### CLM-005 — Construction

> ##### Construction
>
> | Artifact | Description | Evidence status |
> |---|---|---|
> | Analysis status enum | Enum vocabulary for model incomplete, mechanics solved, rule inputs incomplete, user rule checked/failed, human review required, and human-approved record references. | Implemented in `schemas/analysis_status.schema.yaml`; automatic status enum excludes `HUMAN_APPROVED_FOR_PROJECT`. |
> | API/result fields | Result-envelope fields that separate software status from external human acceptance record state. | Implemented evidence exists in `schemas/results.schema.yaml`, `api/api_boundary_contract.yaml`, `schemas/analysis_run.schema.json`, and `schemas/headless_runner.schema.yaml`; final integration ownership remains broader-deliverable scope. |
> | Diagnostics linkage | Diagnostic class/status wiring for solve blocking, rule-check blocking, provenance, assumptions, and reportable limitations. | Implemented evidence exists in `schemas/analysis_boundary.schema.yaml`, `core/solver/diagnostics/src/lib.rs`, `core/rules/completeness_checker/src/lib.rs`, and report/result export modules. |
> | Tests | Tests prove mechanics/rule/human statuses are distinct and no automatic compliance/approval status is emitted. | Evidence includes `tests/test_analysis_status_schema.py`, `tests/test_analysis_boundary_schema.py`, `tests/test_results_schema.py`, and `tests/test_api_boundary_contract.py`. |
>

### CLM-006 — Status Vocabulary Register

> ##### Status Vocabulary Register
>
> | Status | Status meaning | Authority level |
> |---|---|---|
> | `MODEL_INCOMPLETE` | Solve-required physical inputs are missing. | Software finding |
> | `MECHANICS_SOLVED` | Numerical mechanics solve completed for a specific snapshot. | Solver result only |
> | `RULE_INPUTS_INCOMPLETE` | Mechanics may be solved, but rule-pack-required user/code values are missing. | Rule-pack finding |
> | `USER_RULE_CHECKED` | User-defined rule pack evaluated the result. | Software computation using user data |
> | `USER_RULE_FAILED` | User-defined rule pack produced a failing result. | Software computation using user data |
> | `HUMAN_REVIEW_REQUIRED` | Professional use requires competent human review. | Report/professional-boundary notice |
> | `HUMAN_APPROVED_FOR_PROJECT` | A human recorded project-specific acceptance outside solver authority. | Human record only; automatic software emission prohibited |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md`
> - `_REFERENCES.md`
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
> - `docs/_Registers/Deliverables.csv` row DEL-05-04
> - `docs/_Registers/ScopeLedger.csv` row SOW-047
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-05-04
> - `docs/CONTRACT.md`
> - `docs/TYPES.md`
> - `docs/SPEC.md`
> - `docs/architecture/analysis_status_semantics.md`
> - `schemas/analysis_status.schema.yaml`
> - `schemas/analysis_boundary.schema.yaml`
> - `schemas/results.schema.yaml`
> - `api/api_boundary_contract.yaml`
> - `tests/test_analysis_status_schema.py`
> - `tests/test_analysis_boundary_schema.py`
> - `tests/test_results_schema.py`
> - `tests/test_api_boundary_contract.py`
>

### CLM-008 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Analysis-status semantics are implemented as a code-neutral status vocabulary with current verification evidence. External acceptance remains human-owned, and stale-hash negative behavior remains held where explicitly recorded.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-05-04 Analysis status semantics

> #### Specification: DEL-05-04 Analysis status semantics
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies the analysis-status semantics used by result envelopes and related boundaries. It covers status distinctions among mechanics solved, rule-pack checked or blocked by incomplete rule inputs, incomplete solve data, and human-approved/not-human-approved records. Implemented evidence exists for the schema vocabulary, result/API fields, boundary tests, and downstream consumers. Lifecycle promotion, release readiness, GUI acceptance workflow ownership, report rendering acceptance, and professional approval remain outside this deliverable-local kit.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ReqID | Requirement | Source |
> |---|---|---|
> | REQ-05-04-001 | Status semantics shall clearly distinguish mechanics-solved state, rule-pack evaluation state, incomplete-data findings, and human acceptance state. | SOW-047; AB-00-03 |
> | REQ-05-04-002 | `MECHANICS_SOLVED` shall mean mechanics outputs were computed for a specific model snapshot and shall not imply rule-pack completeness or code compliance. | `docs/TYPES.md` section 4; OPS-K-MECH-2 |
> | REQ-05-04-003 | `MODEL_INCOMPLETE` shall represent missing solve-required physical inputs and shall block mechanics solving for the affected subject. | `docs/architecture/analysis_status_semantics.md` Usage Rules; OPS-K-DATA-2 |
> | REQ-05-04-004 | `RULE_INPUTS_INCOMPLETE` shall represent missing rule-pack-required user/code/project data and may coexist with `MECHANICS_SOLVED`. | `docs/TYPES.md` section 4; `docs/architecture/analysis_status_semantics.md` Usage Rules |
> | REQ-05-04-005 | `USER_RULE_CHECKED` and `USER_RULE_FAILED` shall describe user-rule-pack computation outcomes without declaring professional code compliance. | `docs/architecture/analysis_status_semantics.md` Authority Boundary; OPS-K-AUTH-1 |
> | REQ-05-04-006 | `HUMAN_REVIEW_REQUIRED` shall be available for reportable result envelopes to preserve the professional-review boundary. | `docs/architecture/analysis_status_semantics.md` Usage Rules; OPS-K-REPORT-1 |
> | REQ-05-04-007 | Software shall not automatically emit `HUMAN_APPROVED_FOR_PROJECT`; that value is reserved for a separate human acceptance record. | `docs/architecture/analysis_status_semantics.md` Authority Boundary; OPS-K-AUTH-1 |
> | REQ-05-04-008 | Any future human acceptance record shall bind to specific reviewed model, result, rule-pack, report, and manifest hashes and shall not survive content changes. | OPS-K-AUTH-2; `docs/architecture/analysis_status_semantics.md` Human Acceptance Records |
> | REQ-05-04-009 | Result/status envelopes shall preserve diagnostic provenance needed for rule-pack references, warnings, assumptions, limitations, and report audit. | AB-00-06; OPS-K-REPORT-1 |
> | REQ-05-04-010 | Tests and future release gates shall prove mechanics solved, rule checked, rule inputs incomplete, human review required, and human-approved record states cannot collapse into a single automatic compliance flag. | OPS-K-AUTH-1; OPS-K-MECH-2; AB-00-08 |
> | REQ-05-04-011 | Public examples and deliverable artifacts shall not embed protected standards text, protected tables, code formulas, proprietary values, private rule values, or professional approval claims. | OPS-K-IP-1; `docs/architecture/analysis_status_semantics.md` Protected Data Boundary |
> | REQ-05-04-012 | Status semantics shall use `TBD` for unresolved human acceptance workflow ownership, storage location, UI presentation, and non-JSON hash edge cases. | OPS-K-AGENT-1; `docs/architecture/analysis_status_semantics.md` Remaining TBDs |
> | REQ-05-04-013 | Result-envelope design shall identify at least the software status, diagnostic references, human-review notice, and optional external human acceptance record reference without implying approval. | `_SEMANTIC_LENSING.md` F-002; AB-00-03; AB-00-06; `schemas/analysis_status.schema.yaml`; `schemas/results.schema.yaml` |
> | REQ-05-04-014 | Future release gates shall include tests and wording checks that prevent automatic approval, automatic code-compliance claims, and stale human-acceptance reuse after content changes. | `_SEMANTIC_LENSING.md` A-001 and X-001; OPS-K-AUTH-1; OPS-K-AUTH-2 |
>

### CLM-012 — Standards

> ##### Standards
>
> No external protected standard text is introduced by this deliverable-local kit. Governing local standards are the project invariant catalog, type vocabulary, technical specification, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.
>

### CLM-013 — Verification

> ##### Verification
>
> | Requirement | Verification approach |
> |---|---|
> | REQ-05-04-001 | `tests/test_analysis_status_schema.py` and `tests/test_analysis_boundary_schema.py` confirm separate automatic software status, authority-model, and external human-record structures. |
> | REQ-05-04-002 | Schema and downstream tests confirm `MECHANICS_SOLVED` is an automatic status and does not require rule-pack checked status. |
> | REQ-05-04-003 | Boundary/result schemas and downstream UX tests preserve `MODEL_INCOMPLETE` as a missing-input status; product-specific emitting workflows remain separate deliverable scope. |
> | REQ-05-04-004 | `tests/test_analysis_boundary_schema.py`, `core/rules/completeness_checker/src/lib.rs`, and downstream result/run tests preserve `RULE_INPUTS_INCOMPLETE` alongside mechanics status. |
> | REQ-05-04-005 | Rule-pack/evaluator and result/report tests preserve checked/failed user-rule outcomes without compliance claims. |
> | REQ-05-04-006 | `tests/test_results_schema.py`, `tests/test_report_generator_contract.py`, `tests/test_report_sections_contract.py`, and runner/report modules require or surface `HUMAN_REVIEW_REQUIRED`. |
> | REQ-05-04-007 | `tests/test_analysis_status_schema.py`, `tests/test_analysis_boundary_schema.py`, `tests/test_results_schema.py`, and `tests/test_api_boundary_contract.py` exclude `HUMAN_APPROVED_FOR_PROJECT` from automatic/API/result statuses. |
> | REQ-05-04-008 | `schemas/analysis_status.schema.yaml` defines hash-bound `HumanAcceptanceRecord`; workflow ownership, storage, and UI presentation remain TBD. |
> | REQ-05-04-009 | `schemas/analysis_boundary.schema.yaml`, `schemas/results.schema.yaml`, report/result export modules, and tests preserve diagnostics, provenance, and boundary notices. |
> | REQ-05-04-010 | Focused schema/API tests prevent `CODE_COMPLIANT`, certification, sealing, approval, or equivalent automatic statuses. |
> | REQ-05-04-011 | Protected-content and product-claims review of examples, docs, and report-facing wording. |
> | REQ-05-04-012 | Open-item review requiring unresolved human workflow details to remain explicit `TBD`s. |
> | REQ-05-04-013 | Schema/API review and tests confirm status-envelope fields remain distinct and do not imply approval. |
> | REQ-05-04-014 | Negative tests for automatic approval/compliance plus stale-hash acceptance reuse. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required local artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, `MEMORY.md`, `_REVIEW.md`, `Review_Findings.csv`, and `_run_records/`. These local artifacts summarize and cite implemented evidence; they are not lifecycle approval records.
>

### CLM-015 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No conflict identified during current evidence alignment. | N/A | N/A | N/A | N/A | TBD |

- **AC-001** — The contract preserves the accepted separation of numerical computation, user-rule evaluation, missing data, professional review, and human-owned project acceptance, including hash-bound external acceptance residuals, without automatic certification, sealing, compliance, approval, or stale-record reuse.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-05-04 Analysis status semantics

> #### Procedure: DEL-05-04 Analysis status semantics
>

### CLM-017 — Purpose

> ##### Purpose
>
> Define the procedure for maintaining DEL-05-04 analysis status semantics evidence after bounded implementation. Product code remains outside this deliverable folder, but the local kit may cite implemented schemas, code, tests, and downstream consumers as evidence.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Source |
> |---|---|
> | Sealed DEL-05-04 context with write scope limited to this deliverable folder. | `_CONTEXT.md`; OPS-K-AGENT-3 |
> | Decomposition revision 0.7 and register rows for DEL-05-04, SOW-047, OBJ-005, and OBJ-011. | `execution/_Decomposition/SOFTWARE_DECOMP.md`; registers |
> | Applicable architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08. | `_CONTEXT.md` Architecture Basis Injection |
> | Invariant catalog slices for authority, mechanics/rule separation, missing data, reports, and agent boundaries. | `docs/CONTRACT.md` |
> | Analysis status vocabulary, architecture note, schemas, and tests. | `docs/TYPES.md`; `docs/architecture/analysis_status_semantics.md`; `schemas/analysis_status.schema.yaml`; `tests/test_analysis_status_schema.py` |
>

### CLM-019 — Steps

> ##### Steps
>
> | Step | Action | Evidence/record |
> |---|---|---|
> | 1 | Read the sealed context and confirm the deliverable is DEL-05-04 / PKG-05 / SOW-047. | `_CONTEXT.md`; run record |
> | 2 | Read governing vocabulary and invariants for analysis statuses, authority boundaries, missing data, report provenance, and human acceptance records. | `docs/TYPES.md`; `docs/CONTRACT.md`; architecture note |
> | 3 | Maintain the four-document kit as deliverable-local evidence for the analysis-status state model. | `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` |
> | 4 | Build the semantic matrix lens as question-shaping evidence, not engineering authority. | `_SEMANTIC.md` |
> | 5 | Build the semantic lensing register and capture warranted gaps/questions with `HumanRuling=TBD`. | `_SEMANTIC_LENSING.md` |
> | 6 | Apply only source-supported Pass 3 refinements, preserving unresolved items as `TBD`. | Four documents and run record |
> | 7 | Extract dependency anchors and information-flow constraints conservatively. | `Dependencies.csv`; `_DEPENDENCIES.md` |
> | 8 | Validate dependency/schema evidence as needed and keep `_STATUS.md` unchanged unless a human explicitly authorizes a lifecycle gate. | Validation output; `_STATUS.md` |
>

### CLM-020 — Implementation Evidence Fixture Selection

> ##### Implementation Evidence Fixture Selection
>
> When maintaining or extending implementation evidence, select tests that demonstrate:
>
> - `MECHANICS_SOLVED` can coexist with `RULE_INPUTS_INCOMPLETE`;
> - missing physical solve inputs emit `MODEL_INCOMPLETE`;
> - rule-pack evaluation can emit checked or failed outcomes without compliance claims;
> - `HUMAN_REVIEW_REQUIRED` appears for reportable results;
> - `HUMAN_APPROVED_FOR_PROJECT` cannot be emitted by ordinary software execution and cannot survive bound-content hash changes.
>

### CLM-021 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
> | Local write boundary | No schema, API, Rust, TypeScript, GUI, or test implementation files are created or edited inside this deliverable-local maintenance tranche. |
> | Status distinction preserved | Mechanics, rule-pack, incomplete-data, and human-acceptance statuses remain distinct. |
> | Human approval boundary preserved | Automatic software approval/compliance claims are absent; human acceptance remains external and `TBD` where unresolved. |
> | Dependency register valid | `Dependencies.csv` conforms to v3.1 required columns and `_DEPENDENCIES.md` counts match. |
> | Lifecycle safe | `_STATUS.md` remains under human lifecycle authority and is not changed by evidence alignment. |
>

### CLM-022 — Records

> ##### Records
>
> - `_run_records/TASK_RUN_2026-04-30_1530_four-documents-p1-p2.md`
> - `_run_records/TASK_RUN_2026-04-30_1530_semantic-matrix-build.md`
> - `_run_records/TASK_RUN_2026-04-30_1530_lens-register.md`
> - `_run_records/TASK_RUN_2026-04-30_1530_four-documents-p3.md`
> - `_run_records/TASK_RUN_2026-04-30_1530_dependency-extract.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_REVIEW.md`
> - `Review_Findings.csv`
>

### CLM-023 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Analysis-status semantics are implemented as a code-neutral status vocabulary with current verification evidence. External acceptance remains human-owned, and stale-hash negative behavior remains held where explicitly recorded.

- **VER-001** — Validate the contract and review source parity, status vocabulary and transitions, automatic-versus-human authority boundaries, result and diagnostic interfaces, missing-input behavior, stale-hash obligations, tests, and professional-reliance controls.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-05-04 Analysis status semantics

> #### Guidance: DEL-05-04 Analysis status semantics
>

### CLM-025 — Purpose

> ##### Purpose
>
> This deliverable exists to prevent users, reports, APIs, and future agents from treating a mechanics solve, a user-rule-pack check, and professional acceptance as the same thing. The local kit should make implemented status evidence legible while preserving the boundary that lifecycle promotion and human/project acceptance require separate authority.
>

### CLM-026 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Separate computation from judgment | Mechanics and rule-pack statuses describe computations and findings; professional compliance remains human judgment. | OPS-K-MECH-2; OPS-K-AUTH-1 |
> | Missing data is a result, not a default | Missing solve-required and rule-check-required values must be reported explicitly. Do not invent values or silently bypass a check. | OPS-K-DATA-2 |
> | Human acceptance is external | Human acceptance, if later supported, should be modeled as a separate hash-bound record rather than a solver-emitted status. | OPS-K-AUTH-2; `docs/architecture/analysis_status_semantics.md` Human Acceptance Records |
> | Rule checks use user data | User rule packs may evaluate results, but public software does not bundle protected standards formulas or private project values. | `docs/SPEC.md` section 6; OPS-K-IP-1 |
> | Reports preserve limitation context | Report-facing statuses should carry warnings, assumptions, rule-pack references, provenance, and limitations. | OPS-K-REPORT-1 |
> | Local kit is not lifecycle approval | This folder records evidence, acceptance criteria, review history, and residual TBDs. It may cite implemented schema/code/test evidence, but it does not promote lifecycle state, release readiness, or professional acceptance. | `_CONTEXT.md`; `_STATUS.md`; OPS-K-AGENT-4 |
>

### CLM-027 — Considerations

> ##### Considerations
>
> - `MECHANICS_SOLVED` and `RULE_INPUTS_INCOMPLETE` can both be true for the same snapshot when mechanics outputs exist but user-rule-pack data is missing.
> - `USER_RULE_CHECKED` and `USER_RULE_FAILED` are user-rule computation outcomes. They should not be renamed or displayed as automatic code compliance.
> - `HUMAN_REVIEW_REQUIRED` is a software/report-facing boundary notice, not a failure.
> - `HUMAN_APPROVED_FOR_PROJECT` is reserved for external human acceptance records. Implemented and future ordinary solve, rule-check, import, and report-generation code paths should prevent this value from appearing as automatic software status.
> - `CODE_COMPLIANT` is not an allowed automatic status per `docs/TYPES.md`.
> - Implemented schema/API/result surfaces should continue to expose only automatic software statuses in ordinary software output; external human acceptance remains a hash-bound reference or record.
>

### CLM-028 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Single enum vs status dimensions | A single enum is simple, but multiple statuses may truthfully coexist. Implemented and future surfaces should preserve coexistence where sources allow it. |
> | Human approval label vs separate record | A label is easy to display, but a separate record is needed to bind acceptance to reviewed content hashes and prevent stale reliance. |
> | Rule-pack checked vs rule-pack passed | The sources define checked and failed outcomes; pass semantics should be treated carefully and tied to user-defined rule outputs, not professional acceptance. |
>

### CLM-029 — Vocabulary Notes

> ##### Vocabulary Notes
>
> - Use `mechanics solved` for open-mechanics computation completion only.
> - Use `rule inputs incomplete` when a rule pack cannot evaluate because user/code/project data is missing.
> - Use `user rule checked` or `user rule failed` for user-rule-pack computation outcomes.
> - Use `human review required` as a report/professional-boundary notice.
> - Use `human approved for project` only as an external human acceptance record reference, never as an automatically emitted software result.
> - Reports may reference rule-pack identity and checksums, but public artifacts must not reproduce protected formulas or private values.
>

### CLM-030 — Examples

> ##### Examples
>
> | Scenario | Appropriate status interpretation |
> |---|---|
> | Missing pipe section dimensions prevent solving. | `MODEL_INCOMPLETE`; do not invent defaults. |
> | Displacements/reactions/stresses are computed, but a rule pack needs a missing owner allowable. | `MECHANICS_SOLVED` plus `RULE_INPUTS_INCOMPLETE`. |
> | A user rule pack runs and returns at least one failing check. | `USER_RULE_FAILED` with rule-pack identity/version/checksum and diagnostics. |
> | A calculation report is exported for professional use. | Include `HUMAN_REVIEW_REQUIRED` and professional-boundary notice. |
> | A competent human later accepts a specific report package. | External hash-bound human acceptance record; not automatic software approval. |
>

### CLM-031 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified during current evidence alignment. | N/A | N/A | N/A | N/A | TBD |
>

### CLM-032 — Open Questions

> ##### Open Questions
>
> | Open item | Status |
> |---|---|
> | Exact ownership of all schema/API/result integration points across downstream deliverables. | Partly implemented; integration ownership remains deliverable-specific |
> | Storage owner and UI presentation for external human acceptance records. | TBD |
> | Canonicalization edge cases for non-JSON payload hashes referenced by human records. | TBD |
> | Whether a positive user-rule outcome needs an explicit `USER_RULE_PASSED` value or remains represented by `USER_RULE_CHECKED` plus check details. | TBD |
> | Human acceptance workflow details remain TBD because automatic software authority is prohibited and any accepted record must bind to reviewed hashes. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-047 OBJ-005 OBJ-011 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
