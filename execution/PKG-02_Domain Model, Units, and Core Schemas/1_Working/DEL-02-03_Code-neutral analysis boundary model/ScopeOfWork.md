---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-03
package_id: PKG-02
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-002]
package_objective_refs: [OBJ-001, OBJ-011]
---

# Scope of Work — DEL-02-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-02-03` in service of project scope [SOW-002] and package objectives [OBJ-001, OBJ-011].

- **OUT-001** — A code-neutral analysis-boundary model defining states and interfaces that separate mechanics solving, user-rule checking, and human professional approval is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-02-03 Code-neutral Analysis Boundary Model

> #### Datasheet: DEL-02-03 Code-neutral Analysis Boundary Model
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | `DEL-02-03` | `_CONTEXT.md` |
> | Name | Code-neutral analysis boundary model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row `DEL-02-03` |
> | Package | `PKG-02` Domain Model, Units, and Core Schemas | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 7 |
> | Deliverable type | `DATA_MODEL_CHANGE` | `_CONTEXT.md`; `docs/TYPES.md` section 3 |
> | Scope item | `SOW-002` | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row `SOW-002` |
> | Objectives | `OBJ-001`, `OBJ-011` | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 |
> | Anticipated artifacts | `analysis_status` enum; `docs/SPEC.md` state model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row `DEL-02-03` |
> | Context envelope | `S`; risk `OK` | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row `DEL-02-03` |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Draft value | Source / notes |
> |---|---|---|
> | Boundary purpose | Separate mechanics solve, user-supplied rule-pack check, and project-specific human professional acceptance. | `INIT.md` agent rule; `docs/DIRECTIVE.md` section 2.2; `execution/_Decomposition/SOFTWARE_DECOMP.md` sections 1 and 7 |
> | Code-neutral scope | Solver computes mechanics; user-supplied rule packs evaluate acceptability. | `SOW-002`; `docs/PRD.md` section 6.1; `docs/INTENT.md` "Rule-pack intent" |
> | Public data boundary | Public artifacts must not contain protected standards text, tables, examples, code-derived formulas, allowables, SIF/flexibility tables, or proprietary commercial data. | `docs/CONTRACT.md` `OPS-K-IP-1`; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 |
> | Status vocabulary basis | `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`, `HUMAN_APPROVED_FOR_PROJECT`. | `docs/TYPES.md` section 4 |
> | Prohibited automatic status | `CODE_COMPLIANT` must not be used as an automatic software status. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1` |
> | Authority levels | Software finding, solver result only, rule-pack finding, software computation using user data, always-required professional review, and human record only. | `docs/TYPES.md` section 4 |
> | Result-envelope basis | Commands, queries, jobs, reproducibility metadata, diagnostics, and result envelopes must preserve the mechanics/rule/human distinction. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 `AB-00-03`; `docs/SPEC.md` sections 1 and 11 |
> | Diagnostics basis | Diagnostics/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 `AB-00-06` |
> | Schema basis | Public schemas/interchange use JSON Schema 2020-12 where schema artifacts are produced. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | Human acceptance binding | Human acceptance records, if used, bind to specific model/rule/report hashes and do not survive content changes without re-review. | `docs/CONTRACT.md` `OPS-K-AUTH-2` |
>

### CLM-004 — Conditions

> ##### Conditions
>
> - The model applies only to the semantic and data-interface boundary for `DEL-02-03`; it does not implement numerical solving, rule-pack evaluation, GUI views, reports, or human-acceptance workflow screens. Source: `_CONTEXT.md` Package Exclusions and anticipated artifacts.
> - Missing solve-required or rule-check-required values must be explicit findings, not silent defaults. Source: `docs/CONTRACT.md` `OPS-K-DATA-2`; `docs/PRD.md` section 6.2.
> - A mechanics-only solve may exist before a rule-pack check, but it must not be represented as a code pass/fail state. Source: `docs/PRD.md` section 6.2.
> - A user rule-pack result is a software computation using user data; it is not professional authentication. Source: `docs/TYPES.md` section 4; `docs/DIRECTIVE.md` section 3.
> - Human professional acceptance is outside solver authority and may only be represented as a project-specific human record. Source: `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1`.
> - ASSUMPTION: `analysis_status` is a coarse boundary enum for serialized result/state surfaces. Detailed rule-check outcomes, diagnostic arrays, and human acceptance record fields may be separate fields to avoid overloading the enum. Exact schema file path and field layout are TBD.
>

### CLM-005 — Construction

> ##### Construction
>
> Minimum draft status fields for downstream schema design:
>
> | Field | Purpose | Status |
> |---|---|---|
> | `analysis_status` | One value from the status vocabulary basis. | Required by anticipated artifact; exact serialization TBD. |
> | `status_authority` | Indicates software finding, solver result, rule-pack finding/computation, or human record. | ASSUMPTION, derived from `docs/TYPES.md` section 4. |
> | `status_evidence_ref` | References solver result, rule-pack evaluation, missing-input diagnostic, or human acceptance record. | TBD. |
> | `diagnostics` | Carries blocking/warning information with provenance. | Required in principle by `AB-00-06`; exact schema TBD. |
> | `rule_pack_ref` | Identifies user rule-pack ID, version, checksum, source/provenance, and redistribution status when a rule check is attempted. | Supported by `docs/SPEC.md` section 6 and `docs/PRD.md` section 12.2. |
> | `human_acceptance_ref` | Optional pointer to a human acceptance record outside solver core. | Governed by `OPS-K-AUTH-2`; exact storage location TBD. |
> | `model_or_result_hash` | Binds status/evidence to specific content when hashing is available. | Supported by `AB-00-04`; exact hash scope TBD. |
>
> Open storage and stale-state questions:
>
> | Question | Current disposition | Source / notes |
> |---|---|---|
> | Where is a human acceptance record stored? | TBD. The pointer may reference an external project record, report/audit manifest entry, or future persistence object; this deliverable does not choose the storage location. | `docs/CONTRACT.md` `OPS-K-AUTH-2`; `docs/TYPES.md` section 4; `_CONTEXT.md` "Still TBD" implementation choices. |
> | How is human acceptance marked stale after bound evidence changes? | TBD. The invariant is that the record does not survive model/rule/report content changes without re-review; the exact stale-state field or invalidation mechanism remains unresolved. | `docs/CONTRACT.md` `OPS-K-AUTH-2`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 hash/persistence TBDs. |
>
> Minimum boundary interfaces:
>
> | Interface | Inputs | Outputs | Boundary rule |
> |---|---|---|---|
> | Mechanics solve boundary | Unit-aware model data and solver settings | Mechanical result envelope or solve-blocking diagnostic | May produce `MECHANICS_SOLVED` but not rule pass/fail or human acceptance. |
> | Rule-pack check boundary | Mechanical result envelope plus user-supplied rule pack and required inputs | Rule-check result, missing-input finding, or failed-check finding | May produce user-rule statuses only; no professional acceptance or automatic code-compliance claim. |
> | Human acceptance boundary | Specific model/result/rule/report evidence and external human review record | Optional `HUMAN_APPROVED_FOR_PROJECT` record | Human record only; not emitted by solver core. |
>

### CLM-006 — References

> ##### References
>
> - `_CONTEXT.md` revision 0.7 for sealed deliverable identity, scope, artifacts, and architecture-basis injection.
> - `docs/_Registers/Deliverables.csv` row `DEL-02-03`.
> - `docs/_Registers/ScopeLedger.csv` row `SOW-002`.
> - `docs/_Registers/ContextBudgetQA.csv` row `DEL-02-03`.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 sections 5, 7, 8, and 9.
> - `docs/CONTRACT.md` section 1 invariants `OPS-K-IP-*`, `OPS-K-DATA-*`, `OPS-K-AUTH-*`, `OPS-K-MECH-2`, `OPS-K-AGENT-*`.
> - `docs/TYPES.md` sections 4, 5, 6, and 8.
> - `docs/DIRECTIVE.md` sections 2, 3, 4, and 5.
> - `docs/SPEC.md` sections 1, 6, 7, 8, 9, and 11.
> - `docs/PRD.md` sections 6.1, 6.2, 12, 17.3, and 17.4.
> - `docs/INTENT.md` "Rule-pack intent" and mechanics/rule-pack boundary notes.
> - `docs/IP_AND_DATA_BOUNDARY.md` sections 2-7.
>

### CLM-007 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The code-neutral analysis-boundary schema is implemented and verified against its current contract. External hash-bound acceptance remains human-owned where recorded; implementation evidence does not itself confer acceptance or validation.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-02-03 Code-neutral Analysis Boundary Model

> #### Specification: DEL-02-03 Code-neutral Analysis Boundary Model
>

### CLM-009 — Scope

> ##### Scope
>
> This specification defines the draft data-model boundary for `DEL-02-03`: states and interfaces that distinguish mechanics solve, user-rule check, and project-specific human professional acceptance.
>
> In scope:
>
> - `analysis_status` vocabulary and authority semantics.
> - Boundary interfaces between mechanics solving, rule-pack checking, and human acceptance records.
> - Minimum provenance, diagnostics, and evidence hooks needed to avoid status ambiguity.
> - Documentation guidance for a future `docs/SPEC.md` state-model section.
>
> Out of scope:
>
> - Numerical solver implementation.
> - Rule-pack expression grammar or evaluator implementation.
> - GUI workflow behavior.
> - Report rendering.
> - External code, standard, or certification logic.
> - Protected standards/code data, code tables, allowables, SIF/flexibility factors, or proprietary examples.
>
> Primary governing sources are `_CONTEXT.md`, `SOW-002`, `docs/TYPES.md` section 4, `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/PRD.md` sections 6.1/6.2/12/17.4, and `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | DEL-02-03-R01 | The boundary model shall represent the status vocabulary from `docs/TYPES.md` section 4: `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`, and `HUMAN_APPROVED_FOR_PROJECT`. | `docs/TYPES.md` section 4 |
> | DEL-02-03-R02 | The model shall not include `CODE_COMPLIANT` as an automatic software status and shall not claim certification, sealing, approval, authentication, or engineering code compliance for reliance (PRD §21.2). | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1`; `INIT.md` Agent rule |
> | DEL-02-03-R03 | The mechanics solve boundary shall communicate that solver output is a mechanics result only. | `docs/CONTRACT.md` `OPS-K-MECH-2`; `docs/PRD.md` section 6.1; `docs/INTENT.md` mechanics/rule-pack boundary |
> | DEL-02-03-R04 | The rule-check boundary shall require user-supplied rule-pack data for acceptability checks and shall not bundle proprietary or protected code content into the public model. | `SOW-002`; `docs/PRD.md` sections 6.1 and 12.1; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 |
> | DEL-02-03-R05 | Missing solve-required and rule-check-required values shall surface as explicit statuses or diagnostics, never as silent defaults. | `docs/CONTRACT.md` `OPS-K-DATA-2`; `docs/PRD.md` section 6.2 |
> | DEL-02-03-R06 | The model shall keep user-rule pass/fail outcomes distinct from human professional acceptance. | `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/TYPES.md` section 4 |
> | DEL-02-03-R07 | A human acceptance record, if represented, shall be outside solver authority and bound to the specific model/rule/report evidence that was reviewed. | `docs/CONTRACT.md` `OPS-K-AUTH-2`; `docs/TYPES.md` section 4 |
> | DEL-02-03-R08 | Status envelopes and diagnostics shall carry provenance sufficient to identify the source/evidence for the status. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.1 `AB-00-06`; `docs/SPEC.md` sections 7-8 |
> | DEL-02-03-R09 | Where a public schema/interchange artifact is produced, it shall align with the JSON Schema 2020-12 baseline from SCA-001. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | DEL-02-03-R10 | The model shall preserve application-service and API boundary separation; adapters/plugins shall not bypass governance, validation, diagnostics, or public/private data boundaries. | `AB-00-02`; `AB-00-07`; `docs/SPEC.md` section 1 |
> | DEL-02-03-R11 | The model shall leave exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, and physical project container as implementation-level TBD unless a later human project-authority decision resolves them. | `_CONTEXT.md` Still TBD; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
> | DEL-02-03-R12 | ASSUMPTION: If `analysis_status` remains a single enum, `USER_RULE_FAILED` represents an evaluated user-defined rule failure and `USER_RULE_CHECKED` represents completed user-rule evaluation without a blocking missing-input condition. A richer outcome field may be needed; exact split is TBD. | Inference from `docs/TYPES.md` section 4 and anticipated artifact `analysis_status enum` |
>

### CLM-011 — Status Authority and Evidence Model

> ###### Status Authority and Evidence Model
>
> The following draft maps refine R01-R08 for downstream schema work. Exact schema names and storage paths remain `TBD`; the obligations below describe the minimum boundary evidence that must not be lost.
>
> Minimum provenance sufficiency:
>
> | Evidence area | Minimum contents | Source |
> |---|---|---|
> | Any emitted status | `analysis_status`, the authority/producer basis for that status, and a reference to the evidence that justifies it. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-1`, `OPS-K-MECH-2` |
> | Diagnostic-backed status | Diagnostic code, class, severity, source, affected object, message, remediation, and provenance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-06`; `docs/SPEC.md` section 7 |
> | User-rule status | Rule-pack identity, version/checksum, source/provenance note, redistribution/private status, and evaluation or missing-input evidence. | `docs/SPEC.md` section 6; `docs/PRD.md` section 12.2; `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
> | Human acceptance status | Pointer to a project-specific human acceptance record outside solver core and to the reviewed model/rule/report evidence. Storage location and stale-state field remain `TBD`. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` `OPS-K-AUTH-2` |
> | Reproducibility binding | Model/result hash or input manifest when available; exact hash scope remains `TBD`. | `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-04`; `docs/SPEC.md` section 8 |
>
> Field-level obligation map:
>
> | Field | Obligation | Applies when | Notes |
> |---|---|---|---|
> | `analysis_status` | Required. | Every boundary status record. | Values limited to the `docs/TYPES.md` section 4 vocabulary unless the human project authority later amends it. |
> | `status_authority` | Required or derivable without ambiguity. | Every boundary status record. | Authority basis follows the status vocabulary: software finding, solver result only, rule-pack finding/computation, always-required review, or human record only. |
> | `status_evidence_ref` | Required. | Every emitted status. | References the diagnostic, solver result envelope, rule-pack evaluation, missing-input finding, or human acceptance record; exact pointer format is `TBD`. |
> | `diagnostics` | Required when a missing, blocking, warning, assumption, nonconvergence, IP-boundary, or failed-check condition is present. | Incomplete, blocking, warning, assumption, and failure conditions. | Must preserve provenance and affected-object context; clean states may still carry informational diagnostics. |
> | `rule_pack_ref` | Required for rule-check statuses. | `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`; optional/TBD where a rule pack is selected but not yet run. | Must identify at least name/id, version or checksum, source/provenance, and private/redistribution status when available. |
> | `human_acceptance_ref` | Required only for `HUMAN_APPROVED_FOR_PROJECT`; prohibited as a solver-generated result. | Human acceptance record represented in project data. | Storage location, permissions, and stale-state representation are `TBD`. |
> | `model_or_result_hash` | Required when binding a human record or producing audit/reproducibility output with available hashes. | Human acceptance, audit/report, and reproducibility contexts. | Exact hash scope remains `TBD`; JSON payload hash basis follows the SCA-001 architecture baseline. |
>
> Producer authority and evidence map:
>
> | Status | Authorized producer / actor | Authority level | Minimum evidence |
> |---|---|---|---|
> | `MODEL_INCOMPLETE` | Model validation or solve precheck. | Software finding. | Solve-blocking diagnostic identifying missing physical data. |
> | `MECHANICS_SOLVED` | Mechanics solver through result envelope. | Solver result only. | Solver result envelope and model/result evidence reference. |
> | `RULE_INPUTS_INCOMPLETE` | Rule-pack completeness checker/evaluator. | Rule-pack finding. | Selected rule-pack reference plus missing rule-input diagnostic. |
> | `USER_RULE_CHECKED` | Rule-pack evaluator using user-supplied data. | Software computation using user data. | Rule-pack reference, evaluation evidence, and any nonblocking diagnostics. |
> | `USER_RULE_FAILED` | Rule-pack evaluator using user-supplied data. | Software computation using user data. | Rule-pack reference, failed-check evidence, and diagnostic/result context. |
> | `HUMAN_REVIEW_REQUIRED` | Boundary/status model or reporting surface. | Always true for professional use. | Status presentation or report notice that professional reliance requires competent human review. |
> | `HUMAN_APPROVED_FOR_PROJECT` | Human reviewer or project record outside solver core. | Human record only. | Human acceptance record pointer bound to reviewed model/rule/report evidence; not emitted by solver or rule-pack code. |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code or protected standard is a governing source for this deliverable. The governing standards for this draft are project-internal governance and architecture basis documents:
>
> | Source | Applicable content |
> |---|---|
> | `docs/CONTRACT.md` | Binding invariants for protected content, user-supplied data, status authority, solver/rule separation, and agent boundaries. |
> | `docs/TYPES.md` | Analysis-status vocabulary, epistemic labels, domain terms, and lifecycle states. |
> | `docs/DIRECTIVE.md` | Product boundaries, no silent defaults, human authority, and stop rules. |
> | `docs/SPEC.md` | Layered architecture, rule-pack evaluator requirements, warning classes, report/audit boundaries, and acceptance semantics. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | Accepted revision 0.7 decomposition, `SOW-002`, `DEL-02-03`, objectives, and SCA-001 architecture basis. |
> | `docs/PRD.md` | Open mechanics/private code data, no silent engineering defaults, rule-pack requirements, private data handling, and disclaimer requirements. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data boundary, provenance fields, quarantine rule, and report boundary. |
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification ID | Requirement(s) | Check |
> |---|---|---|
> | V01 | R01, R02 | Confirm the status vocabulary matches `docs/TYPES.md` section 4 and excludes automatic `CODE_COMPLIANT`. |
> | V02 | R03, R04, R06 | Confirm the mechanics, rule-pack, and human acceptance interfaces are documented separately. |
> | V03 | R05, R08 | Confirm missing values are represented as explicit findings/statuses/diagnostics with provenance hooks. |
> | V04 | R07 | Confirm human acceptance is modeled as a human record tied to specific evidence, not a solver output. |
> | V05 | R09, R10 | Confirm future schema/API work references JSON Schema 2020-12 and no-bypass adapter constraints without selecting unresolved implementation details. |
> | V06 | R11 | Confirm all unresolved implementation-level choices remain `TBD`. |
> | V07 | All | Confirm no protected standards text, tables, proprietary data, or code-compliance/certification claims appear in the artifact. |
> | V08 | R02, R07 | Confirm `HUMAN_APPROVED_FOR_PROJECT` is presented only as a project-specific human record and never as software approval, certification, or code compliance (PRD §21.2). |
> | V09 | R01, R05, R08 | Confirm each status has the required actor/authority level, evidence reference, and diagnostic or rule-pack hook from the producer authority and evidence map. |
> | V10 | R08 | Confirm minimum provenance contents identify status evidence, rule-pack source/version/checksum where applicable, diagnostic provenance where present, and hash/input-manifest binding when available. |
> | V11 | R10 | Confirm every adapter, plugin, API surface, or schema surface that can observe or mutate analysis status routes through validation, diagnostics/provenance, and public/private data-boundary controls; no unvalidated status write path is permitted. |
>
> Per-status verification coverage:
>
> | Status | Acceptance check |
> |---|---|
> | `MODEL_INCOMPLETE` | Missing physical data is represented by a solve-blocking diagnostic, not a silent default. |
> | `MECHANICS_SOLVED` | Result presentation says mechanics only and does not imply a rule-pack pass/fail or human acceptance. |
> | `RULE_INPUTS_INCOMPLETE` | Selected rule-pack identity is present and missing rule-pack inputs are explicit. |
> | `USER_RULE_CHECKED` | Rule-pack evaluation evidence is present and language avoids external code-compliance claims. |
> | `USER_RULE_FAILED` | Failed-check evidence is present and remains a user-rule result, not professional authentication. |
> | `HUMAN_REVIEW_REQUIRED` | Presentation treats competent human review as required before professional reliance. |
> | `HUMAN_APPROVED_FOR_PROJECT` | A human acceptance record is referenced, bound to reviewed evidence, and clearly outside solver/rule-pack authority. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required documentation outputs for this deliverable tranche:
>
> - `Datasheet.md` describing the boundary model attributes and construction.
> - `Specification.md` defining requirements and verification hooks.
> - `Guidance.md` explaining conservative use and trade-offs.
> - `Procedure.md` giving operational steps for producing/updating the model.
> - Future integration target: a `docs/SPEC.md` state-model section. This run does not edit `docs/SPEC.md` because the sealed write scope is deliverable-local.
> - Future schema target: `analysis_status` enum/schema location TBD.
>

### CLM-015 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The code-neutral analysis-boundary schema is implemented and verified against its current contract. External hash-bound acceptance remains human-owned where recorded; implementation evidence does not itself confer acceptance or validation.

- **AC-001** — The contract preserves the accepted source distinctions among software-computed mechanics, user-supplied rule evaluation, and authorized human approval without creating automatic compliance or professional-reliance meaning.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-02-03 Code-neutral Analysis Boundary Model

> #### Procedure: DEL-02-03 Code-neutral Analysis Boundary Model
>

### CLM-017 — Purpose

> ##### Purpose
>
> Define a repeatable procedure for producing and checking the `DEL-02-03` boundary model. The procedure is for deliverable drafting and later implementation planning; it does not perform mechanics solving, rule-pack evaluation, GUI development, or human professional acceptance.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> - Read `_CONTEXT.md` revision 0.7 for deliverable identity, scope, artifacts, and SCA-001 architecture-basis injection.
> - Confirm `_STATUS.md` allows editing under the active task brief. Reread the current state for each run; do not carry forward stale prior-run state claims.
> - Read `_REFERENCES.md` and use accessible local sources.
> - Read `docs/_Registers/Deliverables.csv` row `DEL-02-03`, `docs/_Registers/ScopeLedger.csv` row `SOW-002`, and `docs/_Registers/ContextBudgetQA.csv` row `DEL-02-03`.
> - Read `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 sections for objectives, `PKG-02`, `DEL-02-03`, SCA-001 basis, and `SOW-002`.
> - Read governing source slices: `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/PRD.md` sections 6.1/6.2/12/17.4, `docs/INTENT.md` rule-pack/mechanics boundary, and `docs/IP_AND_DATA_BOUNDARY.md`.
> - Confirm no human-owned upstream dependency list blocks the work. `_DEPENDENCIES.md` states dependencies are coordinated externally and not tracked in this folder.
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm scope.
>    - Use `DEL-02-03`, `PKG-02`, `SOW-002`, and anticipated artifacts from `_CONTEXT.md` and the registers.
>    - Do not expand into solver, GUI, report, or rule-pack implementation deliverables.
>
> 2. Establish the boundary actors.
>    - Mechanics solver: computes mechanical results only.
>    - Rule-pack evaluator: evaluates user-supplied acceptability checks.
>    - Human reviewer: records project-specific acceptance outside solver authority.
>
> 3. Establish the status vocabulary.
>    - Start from `docs/TYPES.md` section 4.
>    - Include only the listed statuses unless the human project authority later amends the vocabulary.
>    - Do not introduce `CODE_COMPLIANT` as an automatic status.
>
> 4. Define status evidence.
>    - For `MODEL_INCOMPLETE`, require solve-blocking missing-data evidence.
>    - For `MECHANICS_SOLVED`, require a solver result envelope.
>    - For `RULE_INPUTS_INCOMPLETE`, require rule-pack missing-input evidence.
>    - For `USER_RULE_CHECKED` or `USER_RULE_FAILED`, require rule-pack identity, version/checksum, and evaluation evidence.
>    - For `HUMAN_APPROVED_FOR_PROJECT`, require a human acceptance record bound to specific model/rule/report evidence.
>
> 5. Apply data-boundary rules.
>    - Do not include protected standards/code text, tables, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data, or commercial software examples.
>    - Mark unknowns as `TBD`.
>    - Label inferences as `ASSUMPTION`.
>
> 6. Apply architecture-basis constraints.
>    - Preserve layer responsibilities and inward dependency direction from `AB-00-02`.
>    - Preserve command/query/job/result-envelope separation from `AB-00-03`.
>    - Use JSON Schema 2020-12 where a public schema artifact is later produced, per `AB-00-04` and `_CONTEXT.md`.
>    - Carry diagnostics/provenance per `AB-00-06`.
>    - Preserve no-bypass adapter/plugin boundaries per `AB-00-07`.
>    - Plan verification hooks per `AB-00-08`.
>
> 7. Draft or update artifacts.
>    - Draft `analysis_status` enum semantics.
>    - Draft the `docs/SPEC.md` state-model text in deliverable-local form only unless a later task authorizes editing `docs/SPEC.md`.
>    - Keep implementation-level choices marked `TBD`: exact schema file path, exact field layout, expression grammar/library, public API transport, and physical persistence container.
>
> 8. Run cross-document consistency checks.
>    - Confirm Datasheet attributes are reflected in Specification requirements.
>    - Confirm Specification requirements have Guidance rationale and Procedure verification hooks.
>    - Confirm terminology is consistent: mechanics solve, user-rule check, human professional acceptance, user-supplied rule pack, protected standards data, and `analysis_status`.
>    - Confirm no numeric engineering values or protected source content were introduced.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Scope check | All content remains about `DEL-02-03` and does not implement adjacent deliverables. |
> | Status vocabulary check | Status names match `docs/TYPES.md` section 4; `CODE_COMPLIANT` is absent as an automatic status. |
> | Authority check | Solver, rule-pack, and human acceptance authority are separately described. |
> | Data-boundary check | No protected standards/code data, code tables, copied formulas, proprietary examples, or certification/compliance claims are introduced. |
> | TBD/ASSUMPTION check | Unknown implementation details remain `TBD`; inferred design choices are labeled `ASSUMPTION`. |
> | Architecture-basis check | Applicable SCA-001 basis IDs are referenced as constraints without copying full PKG-00 prose. |
> | Cross-document check | Datasheet, Specification, Guidance, and Procedure use consistent terms and status meanings. |
>

### CLM-021 — Records

> ##### Records
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_STATUS.md` safe transition record, when state permits.
> - `_run_records/TASK_RUN_*.md`
> - Future record target: authorized update to `docs/SPEC.md` state model, if authorized by a separate task or change process.

- **VER-001** — Validate the contract and review source parity, allowed state and interface distinctions, authority labels, diagnostics, and absence of automatic code-compliance or approval claims.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-02-03 Code-neutral Analysis Boundary Model

> #### Guidance: DEL-02-03 Code-neutral Analysis Boundary Model
>

### CLM-023 — Purpose

> ##### Purpose
>
> This guidance helps future implementers use the `DEL-02-03` boundary model without blurring three different authorities:
>
> - mechanics produced by the solver;
> - acceptability checks produced by user-supplied rule packs;
> - project-specific human professional acceptance.
>
> The distinction is required by `SOW-002`, the `docs/TYPES.md` analysis-status vocabulary, `docs/CONTRACT.md` invariants `OPS-K-AUTH-1` and `OPS-K-MECH-2`, and the SCA-001 `AB-00-03` architecture basis.
>

### CLM-024 — Principles

> ##### Principles
>
> 1. Mechanics results are not rule checks.
>    Source: `docs/PRD.md` section 6.1 and `docs/INTENT.md` mechanics/rule-pack boundary.
>
> 2. User-rule checks are not human professional acceptance.
>    Source: `docs/DIRECTIVE.md` section 2.2 and `docs/TYPES.md` section 4.
>
> 3. Missing data is a finding, not an implementation gap to hide.
>    Source: `docs/CONTRACT.md` `OPS-K-DATA-2` and `docs/PRD.md` section 6.2.
>
> 4. Protected standards and proprietary data do not belong in public boundary-model artifacts.
>    Source: `docs/CONTRACT.md` `OPS-K-IP-1`; `docs/IP_AND_DATA_BOUNDARY.md` section 3.
>
> 5. Human acceptance, if represented, is an external record bound to reviewed evidence.
>    Source: `docs/CONTRACT.md` `OPS-K-AUTH-2`.
>
> 6. Status labels should communicate authority, not just workflow progress.
>    Source: `docs/TYPES.md` section 4 authority levels.
>

### CLM-025 — Vocabulary Note

> ##### Vocabulary Note
>
> Use `human acceptance record` as the preferred term for the data object or pointer that may support `HUMAN_APPROVED_FOR_PROJECT`. Use `professional approval` only for the human governance concept described in `docs/TYPES.md` section 6, not for a software action. Accepted aliases such as human professional approval, project-specific human acceptance, and human approval must resolve to a human record outside solver authority. UI, API, and report text should not shorten `HUMAN_APPROVED_FOR_PROJECT` to "approved" without the project-specific human-record qualifier (PRD §21.2).
>
> Source: `docs/TYPES.md` sections 4 and 6; `docs/CONTRACT.md` `OPS-K-AUTH-1` and `OPS-K-AUTH-2`; `INIT.md` Agent rule.
>

### CLM-026 — Considerations

> ##### Considerations
>
> - A single `analysis_status` enum is easy to serialize, but it may be too coarse to represent concurrent facts such as "mechanics solved" and "human review required." ASSUMPTION: use the enum as the coarse public boundary and keep detailed rule results, diagnostics, and human acceptance evidence in separate fields where needed. Exact field split is TBD.
> - ASSUMPTION: Keep `analysis_status` coarse while downstream consumers need one headline boundary state and sibling evidence fields can carry rule-pack, diagnostic, hash, and human-record detail. Propose separate solve/rule/human axes only when a schema, API, UI, or report must preserve simultaneous facts that a single enum cannot represent without hiding evidence. The split decision requires human project authority because it changes the anticipated `analysis_status enum` artifact and may affect adjacent packages. Source basis: `_CONTEXT.md` anticipated artifacts, `docs/TYPES.md` section 4, and `execution/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-03`.
> - `HUMAN_REVIEW_REQUIRED` is defined as always true for professional use in `docs/TYPES.md` section 4. Implementers should avoid treating it as a transient workflow step that disappears after a rule-pack check.
> - `HUMAN_APPROVED_FOR_PROJECT` should not be generated by solver or rule-pack code. It should only point to a project-specific human record, and that record should be invalidated or marked stale when bound model/rule/report evidence changes.
> - `USER_RULE_CHECKED` and `USER_RULE_FAILED` are user-rule outcomes, not code-compliance declarations. Public UI/report language should identify the user rule pack and its provenance/checksum instead of implying external endorsement.
> - `RULE_INPUTS_INCOMPLETE` should be used when mechanics may be available but selected rule-pack inputs are missing. This supports no-silent-defaults behavior required by `docs/PRD.md` section 6.2.
> - Schema and API work should preserve `AB-00-02`, `AB-00-03`, `AB-00-06`, and `AB-00-07`: no layer, adapter, or plugin should bypass validation, diagnostics, provenance, or public/private data boundaries.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Option | Benefit | Risk | Conservative guidance |
> |---|---|---|---|
> | Single `analysis_status` enum | Simple interchange and clear artifact match. | Can imply one linear lifecycle when statuses are partly orthogonal. | Use only for coarse boundary state; add explicit evidence and diagnostic fields. |
> | Separate status axes for solve/rule/human | More accurate authority modeling when concurrent facts must be retained. | Larger schema surface; may exceed this deliverable's limited implementation surface and affect adjacent packages. | Mark as future refinement unless human project authority expands scope or accepts the split. |
> | Store human acceptance in solver result | Easy to display near results. | Blurs software authority and professional acceptance. | Store or reference it outside solver core and bind it to hashes/evidence. |
> | Include rule-pack formulas in public reports/examples | Easier for demos. | May expose protected standards or proprietary data. | Public examples use invented non-code values only; private user reports remain user responsibility. |
> | Treat a successful user-rule check as "approved" | Shorter UX language. | Violates professional-responsibility boundary. | Use "user rule checked" or equivalent non-certifying language (PRD §21.2). |
>

### CLM-028 — Examples

> ##### Examples
>
> These are invented status-flow examples only; they are not engineering examples and contain no code-specific values.
>
> | Scenario | Appropriate boundary state | Notes |
> |---|---|---|
> | Required model data for solving is missing. | `MODEL_INCOMPLETE` | Pair with solve-blocking diagnostics. |
> | Solver produced mechanical displacements/forces/stresses, but no rule pack was run. | `MECHANICS_SOLVED` | Still not a rule check or human acceptance. |
> | Solver result exists, but selected rule pack lacks required user-supplied inputs. | `RULE_INPUTS_INCOMPLETE` | Do not report pass/fail. |
> | User rule pack evaluated and at least one user-defined check failed. | `USER_RULE_FAILED` | Identify rule pack, version, checksum, and evidence. |
> | User rule pack evaluated without a blocking missing-input condition. | `USER_RULE_CHECKED` | Does not mean external code compliance or professional acceptance. |
> | Responsible human records project-specific acceptance after review. | `HUMAN_APPROVED_FOR_PROJECT` record | Human record only; bind to model/rule/report evidence. |
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No unresolved source conflicts remain. Pass 3 resolved the stale run-state trace item by requiring `_STATUS.md` to be reread for each run rather than carrying a prior run state into `Procedure.md`. Current unresolved items are implementation-level `TBD` items, not source conflicts:
>
> | Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | None | None identified. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-002 OBJ-001 OBJ-011 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
