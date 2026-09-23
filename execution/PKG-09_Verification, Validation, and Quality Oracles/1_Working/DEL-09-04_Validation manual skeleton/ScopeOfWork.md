---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-04
package_id: PKG-09
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-027]
package_objective_refs: [OBJ-008, OBJ-011]
---

# Scope of Work — DEL-09-04

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open obligations remain in this Scope of Work and owning decisions, with execution in the selected graph; lifecycle remains in `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-09-04` in service of project scope [SOW-027] and package objectives [OBJ-008, OBJ-011].

- **OUT-001** — A validation-manual skeleton contract separating mechanics verification, intended-use workflow validation, user rule checks, and professional reliance across the required manual outline is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-09-04 Validation manual skeleton

> #### Datasheet: DEL-09-04 Validation manual skeleton
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-09-04 |
> | Deliverable name | Validation manual skeleton |
> | Package ID | PKG-09 |
> | Package name | Verification, Validation, and Quality Oracles |
> | Deliverable type | DOC_UPDATE |
> | Scope item | SOW-027 |
> | Objectives | OBJ-008; OBJ-011 |
> | Context envelope | M |
> | Lifecycle artifact | Draft setup document, not an issued validation manual |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value |
> |---|---|
> | Primary purpose | Define a validation manual structure that separates mechanics verification, workflow validation, user rule checks, and professional reliance. |
> | Required boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). |
> | Engineering authority | Human engineering judgment remains required for project-specific reliance. |
> | Code-specific data posture | User-supplied or private; not bundled into public validation examples. |
> | Benchmark source posture | Original, public-domain, or permissively licensed examples only. |
> | Protected-content posture | Protected standards text, tables, figures, examples, code formulas, material allowables, SIF/flexibility tables, and proprietary commercial data are excluded. |
> | Status vocabulary | `MODEL_INCOMPLETE`, `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `HUMAN_REVIEW_REQUIRED`, `HUMAN_APPROVED_FOR_PROJECT`; no automatic `CODE_COMPLIANT` status. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Source basis | Effect on the manual skeleton |
> |---|---|---|
> | Verification vs validation distinction is mandatory. | `docs/VALIDATION_STRATEGY.md` section 1; SOW-027 | The outline separates mechanics benchmark evidence from intended-use workflow evidence. |
> | Rule checks are user-defined computations. | `AGENTS.md`; `docs/TYPES.md`; `docs/SPEC.md` §7 Rule-pack evaluator | The outline treats rule-pack checks as private/user design-basis evaluation, not professional authentication. |
> | Professional reliance is outside software authority. | `docs/CONTRACT.md` OPS-K-AUTH-1; OBJ-011 | Every report-facing or validation-facing section must preserve the human review boundary. |
> | Public validation examples must avoid protected and proprietary sources. | `docs/IP_AND_DATA_BOUNDARY.md`; `docs/VALIDATION_STRATEGY.md` section 5 | Source/provenance review is part of the validation manual records. |
> | Unit-aware and diagnostic result boundaries remain relevant. | OPS-K-UNIT-1; `docs/SPEC.md` sections 7-9 | Manual sections include unit/schema verification and diagnostic/result-envelope checks where applicable. |
>

### CLM-006 — Construction

> ##### Construction
>
> The validation manual skeleton is organized as the following outline. Section content is intentionally skeletal until benchmark deliverables, GUI workflow evidence, report reproducibility evidence, and release gates mature.
>
> | Manual section | Purpose | Initial state |
> |---|---|---|
> | Product scope and limitations | Define what SWBPIPE assists with and what remains outside software authority. | Skeleton required |
> | Solver theory summary | Summarize open mechanics at a non-protected, public level. | Skeleton required |
> | Unit and schema verification | Record dimensional, schema, and invalid-input checks. | Skeleton required |
> | Element verification | Record frame/pipe element benchmark families and evidence links. | Skeleton required |
> | Load and stress recovery verification | Record load application and open mechanics stress-recovery benchmark evidence. | Skeleton required |
> | Nonlinear support verification | Record convergence, active-state, and unresolved non-convergence evidence. | Skeleton required |
> | Rule-pack evaluator verification | Record invented-value evaluator tests, missing-input behavior, unit mismatch behavior, and sandbox behavior. | Skeleton required |
> | GUI workflow validation | Record intended-use workflow evidence and warning presentation checks. | Skeleton required |
> | Report reproducibility validation | Record model hash, manifest, warning inclusion, checksum stability, and protected-content lint evidence. | Skeleton required |
> | Known limitations and open issues | Preserve risks, TBDs, exclusions, and human acceptance requirements. | Skeleton required |
>

### CLM-007 — References

> ##### References
>
> - `AGENTS.md`
> - `AGENTS.md`
> - `docs/DIRECTIVE.md`
> - `docs/CONTRACT.md`
> - `docs/TYPES.md`
> - `docs/SPEC.md`
> - `docs/IP_AND_DATA_BOUNDARY.md`
> - `docs/VALIDATION_STRATEGY.md`
> - `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
> - `execution/_Decomposition/SOFTWARE_DECOMP.md`
> - `docs/_Registers/Deliverables.csv`
> - `docs/_Registers/ScopeLedger.csv`
> - `docs/_Registers/ContextBudgetQA.csv`

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-09-04 Validation manual skeleton

> #### Specification: DEL-09-04 Validation manual skeleton
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — Scope

> ##### Scope
>
> Current deliverable obligation: Draft validation manual structure distinguishing verification, validation, user rule checks, and professional reliance. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle; this Scope of Work, owning decisions and the selected graph carry current work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> This deliverable defines the local skeleton for a validation manual. It covers the manual structure, required boundaries, evidence categories, and acceptance checks needed to distinguish:
>
> - mechanics verification;
> - intended-use workflow validation;
> - user rule-pack checks;
> - professional reliance and human acceptance.
>
> This deliverable does not edit `docs/VALIDATION_STRATEGY.md` and does not create issued validation evidence. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>
### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source basis | Acceptance hook |
> |---|---|---|---|
> | VAL-REQ-001 | The manual skeleton shall separate mechanics verification from workflow validation. | `docs/VALIDATION_STRATEGY.md` section 1; SOW-027 | `ScopeOfWork.md` verification checks confirm separate sections and terminology. |
> | VAL-REQ-002 | Mechanics verification sections shall describe software comparison against declared mechanics problems and tolerances, not code compliance. | `docs/VALIDATION_STRATEGY.md` section 1; `docs/TYPES.md` section 6 | Review confirms no automatic compliance wording appears. |
> | VAL-REQ-003 | Validation sections shall evaluate intended-use workflow fitness while preserving project-specific professional judgment. | `docs/VALIDATION_STRATEGY.md` section 1; OBJ-011 | Review confirms professional reliance language remains human-bound. |
> | VAL-REQ-004 | Rule-pack verification sections shall treat pass/fail results as user-supplied rule computations, not professional authentication. | `docs/SPEC.md` section 6; `docs/TYPES.md` section 4 | Review confirms statuses do not include automatic `CODE_COMPLIANT`. |
> | VAL-REQ-005 | The skeleton shall include the ten manual sections listed by the validation strategy. | `docs/VALIDATION_STRATEGY.md` section 3 | Datasheet outline and Procedure records include all ten sections. |
> | VAL-REQ-006 | Benchmark and validation examples referenced by the skeleton shall be original, public-domain, or permissively licensed, with protected and proprietary examples excluded. | `docs/VALIDATION_STRATEGY.md` section 5; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 | Review confirms no protected examples, code tables, or proprietary values are introduced. |
> | VAL-REQ-007 | The skeleton shall expose missing evidence, open risks, limitations, and `TBD` entries rather than filling gaps silently. | OPS-K-AGENT-1; OPS-K-AGENT-2; `docs/VALIDATION_STRATEGY.md` section 4 | Review confirms open items are visible in the manual outline. |
> | VAL-REQ-008 | Release-gate language shall describe software maturity and validation evidence only, not engineering approval. | `docs/VALIDATION_STRATEGY.md` section 4; OPS-K-AUTH-1 | Review confirms no certification, sealing, approval, authentication, or code-compliance claims (PRD §21.2). |
> | VAL-REQ-009 | Unit, schema, diagnostic, and result-envelope checks shall be represented where relevant to validation manual sections. | OPS-K-UNIT-1; `docs/SPEC.md` sections 7-9; AB-00-06 | Review confirms those check families have section slots and do not overclaim. |
> | VAL-REQ-010 | The skeleton shall preserve the public/private data boundary for rule packs, materials, component data, owner requirements, and project models. | OPS-K-IP-1 through OPS-K-IP-3; OPS-K-DATA-1 through OPS-K-DATA-3 | Review confirms user/private data is described as supplied by users or lawful private sources. |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code or standard is used as source authority for this skeleton. The governing sources are the SWBPIPE project documents and registers listed in `_REFERENCES.md`.
>
> Any future validation manual reference to an external standard, protected code, commercial benchmark, vendor data, or owner requirement is `TBD` until provenance, redistribution rights, and human/legal review are recorded.
>

### CLM-013 — Verification

> ##### Verification
>
> | Check | Method | Expected result |
> |---|---|---|
> | Scope of Work contract | Check the consolidated `ScopeOfWork.md` production format, required sections and source/verification traceability. | The current deliverable contract is present and internally consistent; earlier four-document artifacts remain historical provenance only. |
> | Protected-content boundary | Manual review against `docs/IP_AND_DATA_BOUNDARY.md` and OPS-K-IP invariants. | No protected standards data, proprietary values, or commercial examples introduced. |
> | Professional boundary | Search for certification, approval, sealing, authentication, and compliance claims (PRD §21.2). | No software/agent claim exceeds decision-support authority. |
> | Manual outline coverage | Compare Datasheet Construction against `docs/VALIDATION_STRATEGY.md` section 3. | Ten manual sections are represented. |
> | Dependency register schema | Run `python3 tools/validation/validate_dependencies_schema.py` on `Dependencies.csv`. | v3.1 schema is valid. |
>
### CLM-014 — Documentation

> ##### Documentation
>
> The setup artifact set for this deliverable consists of:
>
> - `ScopeOfWork.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_STATUS.md`
> - `_run_records/*`
>
> The repository-level `docs/VALIDATION_STRATEGY.md` remains read-only for this deliverable.
- **AC-001** — The contract preserves the ten manual sections, unit/schema/diagnostic/result-envelope evidence slots, public/private and protected-content boundaries, visible gaps and limitations, software-release versus project-reliance distinction, and human-owned professional judgment. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-09-04 Validation manual skeleton

> #### Procedure: DEL-09-04 Validation manual skeleton
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — Purpose

> ##### Purpose
>
> Use this procedure to produce and check the deliverable-local validation manual skeleton. The procedure is documentation-oriented and does not move any artifact to `ISSUED`.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> Current deliverable obligation: Draft validation manual structure distinguishing verification, validation, user rule checks, and professional reliance. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle; this Scope of Work, owning decisions and the selected graph carry current work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> | Prerequisite | Requirement |
> |---|---|
> | Sealed context | Confirm `DEL-09-04`, `PKG-09`, SOW-027, OBJ-008, and OBJ-011 from `_CONTEXT.md` and the registers. |
> | Governing sources | Read the local references listed in `_REFERENCES.md`, especially `docs/VALIDATION_STRATEGY.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
> | Data boundary | Do not introduce protected standards data, proprietary commercial data, or private user data into public setup artifacts. |
> | Professional boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). |
> | Write scope | Write only inside this deliverable folder. |
>
### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity and current lifecycle state in `_STATUS.md`.
> 2. Draft the `ScopeOfWork.md`:
>    - `ScopeOfWork.md` captures identity, source basis, boundaries, and the manual outline.
>    - `ScopeOfWork.md` captures requirements, exclusions, and acceptance checks.
>    - `ScopeOfWork.md` captures interpretation principles, trade-offs, and prohibited overclaims.
>    - `ScopeOfWork.md` captures production and verification steps.
> 3. Check that the validation manual outline includes the ten sections listed in `docs/VALIDATION_STRATEGY.md` section 3.
> 4. Check that mechanics verification, workflow validation, user rule checks, and professional reliance remain separate.
> 5. Check that release-gate language describes software quality evidence only.
> 6. Mark missing future evidence as `TBD` or as a visible open issue; do not invent benchmark results, source citations, or human rulings.
> 7. Generate semantic matrix and lensing artifacts as setup evidence.
> 8. Refresh dependency artifacts with conservative, evidence-cited rows.
> 9. Run local validation checks and record warnings in `_run_records`.
>
### CLM-020 — Verification

> ##### Verification
>
> | Check | Command or review method | Pass condition |
> |---|---|---|
> | Scope of Work contract | `python3 tools/scope_of_work/validate_scope_of_work.py <DELIVERABLE_PATH>/ScopeOfWork.md` | the Scope of Work contract are present. |
> | Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <deliverable-folder>/Dependencies.csv` | Schema validator reports `VALID`. |
> | Enum checks | `python3 tools/validation/validate_enum.py <enum> <value>` for emitted dependency enum values | Emitted values are accepted by the enum validator. |
> | Status check | Inspect `_STATUS.md`. | Current State is `SEMANTIC_READY` only after setup artifacts pass. |
> | Protected-content check | Manual review and targeted search for protected/compliance overclaims. | No protected data or software certification/compliance claims are present. |
>
### CLM-021 — Records

> ##### Records
>
> Required records for this setup run:
>
> - `ScopeOfWork.md`;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - refreshed `_DEPENDENCIES.md`;
> - `_STATUS.md`;
> - `_run_records/*` entries for the setup sequence and validations.
- **VER-001** — Validate the contract and review source parity, ten-section outline coverage, separation of verification/validation/rule checks/professional reliance, data and provenance boundaries, unit and diagnostic evidence slots, visible TBDs, and release/professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-09-04 Validation manual skeleton

> #### Guidance: DEL-09-04 Validation manual skeleton
>

### CLM-023 — Purpose

> ##### Purpose
>
> This guidance explains how to interpret the validation manual skeleton without blurring the SWBPIPE authority boundary. The manual is intended to organize verification and validation evidence for software quality. It is not an engineering seal, code-compliance ruling, or project-specific acceptance record.
>

### CLM-024 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Keep verification narrow. | Mechanics verification asks whether implemented mechanics match declared benchmark problems within declared tolerances. It does not decide whether a real project complies with a code. |
> | Keep validation contextual. | Validation asks whether the software workflow is fit for its intended support role, including warnings, reports, missing-data behavior, and reproducibility. |
> | Keep rule checks user-owned. | Rule packs use user-supplied or lawful private design-basis data. A software pass/fail result is a computation under that rule pack, not professional authentication. |
> | Keep professional reliance human-owned. | A competent human must accept the model, data, assumptions, rule basis, limitations, and report before project reliance. |
> | Keep public examples clean. | Public examples must be original, public-domain, or permissively licensed. Protected standards examples and commercial benchmark files are excluded unless rights are documented. |
> | Keep gaps visible. | Missing benchmark evidence, missing provenance, open risks, and unresolved source questions should remain explicit `TBD` or open issues. |
>

### CLM-025 — Considerations

> ##### Considerations
>
> The validation manual should be useful to maintainers, users, and reviewers, but each audience reads it with different authority:
>
> - Maintainers use it to decide whether software evidence is adequate for a release label.
> - Users use it to understand tested behavior, limitations, and evidence sources.
> - Reviewers use it to check traceability, reproducibility, and data-boundary conformance.
> - Professionals use it only as decision-support context, not as a substitute for judgment.
>
> The manual should avoid embedding protected standards text or code-derived examples. When a future benchmark needs a comparison basis, the source must be public, original, or permissively licensed. If a private licensed basis is used by a user, it belongs in private project records and should not be copied into public artifacts.
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Preferred handling |
> |---|---|
> | Specificity vs protected content risk | Prefer public mechanics statements and provenance over copying code language or examples. |
> | Automation vs human judgment | Automate repeatable checks, but keep professional acceptance as a human decision. |
> | Completeness vs false precision | Use `TBD` for unproduced evidence rather than implying benchmark coverage that does not exist yet. |
> | Release readiness vs project reliance | Release gates describe software quality only; project reliance requires separate professional review. |
>

### CLM-027 — Examples

> ##### Examples
>
> Acceptable example slots:
>
> - An original cantilever or frame mechanics benchmark with independent hand calculation notes.
> - An invented rule-pack test that demonstrates missing-input behavior without using protected code data.
> - A report reproducibility check that verifies hashes, warnings, assumptions, and rule-pack checksum fields.
>
> Excluded example slots:
>
> - Copied protected standards tables, figures, examples, equations, or commentary.
> - Commercial software benchmark files without documented redistribution rights.
> - Vendor or owner data without documented permission for public redistribution.
> - Any example implying software certification, engineering seal, official endorsement, or automatic code compliance (PRD §21.2).
>

### CLM-028 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in setup pass. | Not applicable | Not applicable | Not applicable | Not applicable | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-027 OBJ-008 OBJ-011 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-09-04:1** — Complete owner/maintainer review and MAINTAINER_REVIEWED promotion of the validation-manual case pages. Runner benchmark/regression and export-results report-package payload bindings are landed evidence; export-results is not a remaining runner stub (see `core/runner/headless/src/bin/openpipestress-runner.rs` and `validation/witness/generated/del1005_export_results_*.json`). DEC-080 establishes the evidence-bundle home `validation/evidence/reproduction/<run-id>/`; binding witnesses do not promote case-page review or engineering validation.
- **DEL-09-04:2** — Select final public-benchmark result-comparison values after the ruled V-D measurement/design derivative; DEC-046 C-B nonlinear convergence values are recorded at release scope as policy only (`0 count` relative field, `0 count` absolute floor, cap `4` for all five classes, inclusive final changed-support count zero), not as a release act (source: owner rulings 2026-08-11 / `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`)
