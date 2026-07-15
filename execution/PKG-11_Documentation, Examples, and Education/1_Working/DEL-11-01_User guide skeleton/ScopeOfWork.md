---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-01
package_id: PKG-11
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-011]
---

# Scope of Work — DEL-11-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-11-01` in service of project scope [SOW-033] and package objectives [OBJ-001, OBJ-011].

- **OUT-001** — A user-guide skeleton contract for safe installation, model creation, solve, result review, troubleshooting, and professional-responsibility navigation is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-11-01 User guide skeleton

> #### Datasheet: DEL-11-01 User guide skeleton
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-01-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-11-01 |
> | Deliverable name | User guide skeleton |
> | Package ID | PKG-11 |
> | Package name | Documentation, Examples, and Education |
> | Deliverable type | DOC_UPDATE |
> | Scope item | SOW-033 |
> | Objectives | OBJ-001; OBJ-011 |
> | Context envelope | M |
> | Lifecycle artifact | Draft setup document, not the repository user guide |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value |
> |---|---|
> | Primary purpose | Define a user guide structure for project setup, modeling, solving, rule checks, reports, and limitations. |
> | Target artifact | `docs/user_guide/index.md` is the anticipated downstream artifact, but this setup run does not edit it. |
> | Guide posture | Help users understand an open, auditable piping stress workflow without hiding missing data, private-data requirements, or professional responsibility limits. |
> | Required sections | Product scope, setup, project creation, model building, solving, rule checks, results review, reports, limitations, troubleshooting, glossary. |
> | Data boundary | Public guide content must not include protected standards text, protected examples, proprietary tables, private rule packs, private libraries, or owner data. |
> | Rule-check boundary | User rule checks are computations using user-supplied or lawful private rule packs; they are not professional authentication. |
> | Professional boundary | The software assists analysis and reporting; competent human review remains required before project reliance. |
> | Implementation maturity | Exact packaging, public API transport, import/export formats, solver library, expression grammar, and physical project container remain `TBD`. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Source basis | Effect on the guide skeleton |
> |---|---|---|
> | User documentation is in scope for PKG-11. | `docs/_Registers/Deliverables.csv` row DEL-11-01; `docs/_Registers/ScopeLedger.csv` row SOW-033 | The skeleton covers the user-facing workflow, not source code or examples. |
> | Open mechanics and protected standards data must remain separated. | `INIT.md`; `docs/DIRECTIVE.md` sections 1-4; OPS-K-IP-1 through OPS-K-IP-3 | Guide sections must describe user/private data requirements without copying or paraphrasing protected standards content. |
> | Mechanics solve, user rule check, and professional approval are separate states. | `docs/TYPES.md` sections 4 and 6; OPS-K-AUTH-1 | The guide outline must explain statuses and warnings without implying automatic code compliance. |
> | Primary global analysis is a 3D centerline/frame model. | `docs/DIRECTIVE.md` section 3; `docs/SPEC.md` sections 1 and 4 | Modeling sections emphasize centerline nodes, elements, components, supports, loads, and unit-aware fields. |
> | Reports must be auditable and boundary-aware. | `docs/SPEC.md` section 8; OPS-K-REPORT-1 and OPS-K-AUTH-1 | Reporting sections include provenance, warnings, hashes/checksums, limitations, and human review notice. |
> | Future implementation details are unresolved. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 11 | Setup/install and advanced integration content uses `TBD` rather than inventing product behavior. |
>

### CLM-006 — Construction

> ##### Construction
>
> The user guide skeleton is organized as a documentation outline. Section content is intentionally skeletal until product features, UI behavior, reports, examples, packaging, and release gates mature.
>
> | Guide section | Purpose | Initial state |
> |---|---|---|
> | Scope and authority boundary | Explain OpenPipeStress as decision-support software with open mechanics and user-supplied design-basis data. | Skeleton required |
> | Installation and project setup | Reserve setup instructions for supported builds, local-first storage, project files, units, and privacy posture. | `TBD` where packaging is unresolved |
> | Creating a project | Cover unit selection, project metadata, model identity, provenance expectations, and private rule-pack/library references. | Skeleton required |
> | Building the centerline model | Cover nodes, elements, pipe runs, components, supports/restraints, load cases, combinations, and missing-data warnings. | Skeleton required |
> | Solving mechanics | Explain mechanics-only solve status, solver diagnostics, singular/nonconverged states, and result envelopes. | Skeleton required |
> | Running user rule checks | Explain private rule packs, required inputs, unit checks, missing rule data, checksums, and rule-check statuses. | Skeleton required |
> | Reviewing results | Reserve structure for displacements, rotations, forces, moments, reactions, equipment loads, stresses, ratios, warnings, and assumptions. | Skeleton required |
> | Generating reports | Cover input manifest, model hash, solver version, rule-pack checksum, provenance, limitations, and public-template prohibitions. | Skeleton required |
> | Limitations and professional responsibility | State global centerline vs local FEA handoff limits, validation status, known risks, and human review requirements. | Skeleton required |
> | Troubleshooting and warnings | Organize solve blockers, rule-check blockers, provenance warnings, assumption warnings, nonlinear warnings, and IP-boundary warnings. | Skeleton required |
> | Glossary and status vocabulary | Define user-facing terms from `docs/TYPES.md` without creating new engineering authority. | Skeleton required |
>

### CLM-007 — References

> ##### References
>
> - `INIT.md`
> - `AGENTS.md`
> - `docs/README.md`
> - `docs/DIRECTIVE.md`
> - `docs/CONTRACT.md`
> - `docs/TYPES.md`
> - `docs/SPEC.md`
> - `docs/IP_AND_DATA_BOUNDARY.md`
> - `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
> - `execution/_Decomposition/SOFTWARE_DECOMP.md`
> - `docs/_Registers/Deliverables.csv`
> - `docs/_Registers/ScopeLedger.csv`
> - `docs/_Registers/ContextBudgetQA.csv`

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-11-01 User guide skeleton

> #### Specification: DEL-11-01 User guide skeleton
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-01-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines the deliverable-local skeleton for the OpenPipeStress user guide. It covers the guide structure needed for project setup, centerline modeling, mechanics solving, user rule checks, result review, report generation, limitations, and professional responsibility notices.
>
> This deliverable does not edit `docs/user_guide/index.md`, does not create tutorials or example models, does not document protected standards content, and does not claim certification, endorsement, approval, sealing, authentication, or engineering code compliance.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source basis | Acceptance hook |
> |---|---|---|---|
> | UG-REQ-001 | The guide skeleton shall include sections for project setup, modeling, solving, rule checks, reports, and limitations. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row DEL-11-01 | Procedure verification confirms all required section slots exist. |
> | UG-REQ-002 | Setup content shall preserve unresolved implementation details as `TBD` where packaging, dependency versions, public API transport, import/export formats, solver library, expression grammar, or project container are not yet decided. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` open issues | Review confirms no invented setup commands or product behavior are asserted. |
> | UG-REQ-003 | Modeling content shall frame the global model as a unit-aware 3D centerline/frame model with explicit nodes, elements, components, supports, loads, and provenance-bearing fields. | `docs/DIRECTIVE.md` sections 2-3; `docs/SPEC.md` sections 1, 3, 4, and 7 | Datasheet Construction and Guidance Principles cover model-building categories. |
> | UG-REQ-004 | Solving content shall distinguish mechanics solve status from rule-check status and human professional acceptance. | `docs/TYPES.md` sections 4 and 6; OPS-K-MECH-2; OPS-K-AUTH-1 | Search and review confirm no automatic `CODE_COMPLIANT` status or professional approval claim appears. |
> | UG-REQ-005 | Rule-check content shall state that code-specific values, allowables, combinations, formulas, SIFs, flexibility factors, owner requirements, and proprietary data are supplied by users through lawful private data or rule packs. | `INIT.md`; `docs/DIRECTIVE.md` sections 1 and 3; OPS-K-DATA-1; OPS-K-RULE-1 and OPS-K-RULE-3 | Guidance examples exclude protected and proprietary source material. |
> | UG-REQ-006 | Report content shall include auditable-report slots for software/solver version, model hash, input manifest, units, warnings, assumptions, source/provenance notes, rule-pack checksum, results, and limitations. | `docs/SPEC.md` section 8; OPS-K-REPORT-1 | Datasheet Construction includes a reporting section and Procedure checks the slots. |
> | UG-REQ-007 | Limitation content shall cover professional responsibility, validation status, public/private data boundaries, local FEA handoff, missing data, and unresolved `TBD` items. | `docs/DIRECTIVE.md` sections 3-6; `docs/VALIDATION_STRATEGY.md` section 1; OPS-K-AUTH-1 | Guidance and Procedure include boundary checks. |
> | UG-REQ-008 | Public guide content shall not reproduce protected standards text, tables, figures, examples, protected dimensional data, proprietary commercial data, or private user/project data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-7; OPS-K-IP-1 through OPS-K-IP-3 | Protected-content review finds no prohibited content. |
> | UG-REQ-009 | The skeleton shall use canonical status and vocabulary terms where user-facing status semantics are referenced. | `docs/TYPES.md` sections 4-7 | Procedure verification checks canonical status terms. |
> | UG-REQ-010 | The skeleton shall expose unknowns and assumptions rather than filling gaps with silent defaults. | OPS-K-AGENT-1; OPS-K-AGENT-2; OPS-K-DATA-2 | Review confirms remaining unknowns are `TBD` or explicit limitations. |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code or standard is used as source authority for this user guide skeleton. The governing sources are the OpenPipeStress project documents and registers listed in `_REFERENCES.md`.
>
> Any future guide content that references an external code, standard, vendor catalog, owner requirement, commercial software example, or licensed data source is `TBD` until provenance, redistribution rights, and human/legal review are recorded.
>

### CLM-013 — Verification

> ##### Verification
>
> | Check | Method | Expected result |
> |---|---|---|
> | Four-document presence | Run `tools/validation/check_four_documents.sh` on the deliverable folder. | All four setup documents are present. |
> | Required guide slots | Compare Datasheet Construction against UG-REQ-001. | Required setup, modeling, solving, rule-check, report, and limitation sections are present. |
> | Protected-content boundary | Manual review against `docs/IP_AND_DATA_BOUNDARY.md` and OPS-K-IP invariants. | No protected standards data, proprietary examples, or private user data are introduced. |
> | Professional boundary | Search for certification, approval, sealing, authentication, endorsement, and compliance overclaims. | Any hits are prohibitions or boundary statements, not product claims. |
> | Dependency register schema | Run `python3 tools/validation/validate_dependencies_schema.py` on `Dependencies.csv`. | v3.1 schema is valid. |
> | Lensing coverage | Count matrix lens coverage rows in `_SEMANTIC_LENSING.md`. | 96 rows for matrices A, B, C, F, D, X, and E. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> The setup artifact set for this deliverable consists of:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_STATUS.md`
> - `_run_records/*`
>
> The repository-level `docs/user_guide/index.md` remains read-only for this deliverable.

- **AC-001** — The contract preserves the source-defined guide structure, current implementation declarations, unit and missing-data visibility, diagnostics and result interpretation boundaries, invented-example posture, protected/private-data controls, accessibility intent, visible unresolved documentation decisions, and the prohibition on treating software output as professional approval or code compliance.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-11-01 User guide skeleton

> #### Procedure: DEL-11-01 User guide skeleton
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-016 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-01-DECL-004`.
>

### CLM-017 — Purpose

> ##### Purpose
>
> Use this procedure to produce and check the deliverable-local user guide skeleton. The procedure is documentation-oriented and does not edit `docs/user_guide/index.md` or move any artifact to `ISSUED`.
>

### CLM-018 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Requirement |
> |---|---|
> | Sealed context | Confirm `DEL-11-01`, `PKG-11`, SOW-033, OBJ-001, and OBJ-011 from `_CONTEXT.md` and the registers. |
> | Governing sources | Read the local references listed in `_REFERENCES.md`, especially `INIT.md`, `docs/DIRECTIVE.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
> | Data boundary | Do not introduce protected standards data, proprietary commercial data, or private user/project data into public setup artifacts. |
> | Professional boundary | Do not certify, endorse, approve, seal, authenticate, or declare engineering code compliance. |
> | Write scope | Write only inside this deliverable folder. |
>

### CLM-019 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity and current lifecycle state in `_STATUS.md`.
> 2. Draft the four-document kit:
>    - `Datasheet.md` captures identity, source basis, boundaries, and the guide outline.
>    - `Specification.md` captures guide skeleton requirements, exclusions, and acceptance checks.
>    - `Guidance.md` captures interpretation principles, trade-offs, and prohibited overclaims.
>    - `Procedure.md` captures production and verification steps.
> 3. Check that the guide outline covers project setup, project creation, centerline modeling, mechanics solving, user rule checks, results review, reports, limitations, troubleshooting, and glossary/status vocabulary.
> 4. Check that guide language preserves these separations:
>    - mechanics solve vs user rule check;
>    - user rule check vs human professional acceptance;
>    - open mechanics vs protected standards data;
>    - global centerline analysis vs local FEA handoff.
> 5. Mark unresolved product behavior as `TBD`; do not invent installation commands, UI behavior, external formats, solver-library choices, rule expression grammar, or project container behavior.
> 6. Generate semantic matrix and lensing artifacts as setup evidence.
> 7. Refresh dependency artifacts with conservative, evidence-cited rows.
> 8. Run local validation checks and record warnings in `_run_records`.
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Command or review method | Pass condition |
> |---|---|---|
> | Four-document kit | `tools/validation/check_four_documents.sh <deliverable-folder>` | All four documents are present. |
> | Required guide sections | Review `Datasheet.md` Construction and `Specification.md` UG-REQ-001. | Required user guide section slots are present. |
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
> - four-document kit;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - refreshed `_DEPENDENCIES.md`;
> - `_STATUS.md`;
> - `_run_records/*` entries for the setup sequence and validations.

- **VER-001** — Validate the contract and review source parity, user-journey and guide-structure coverage, current-versus-planned declarations, units and diagnostics, protected/private-data and accessibility controls, retained conflicts and TBDs, and professional-responsibility limits.

## Governing Values and Decisions — Axiology

### CLM-022 — Guidance: DEL-11-01 User guide skeleton

> #### Guidance: DEL-11-01 User guide skeleton
>

### CLM-023 — Purpose

> ##### Purpose
>
> This guidance explains how to interpret the user guide skeleton without blurring the OpenPipeStress data, rule-check, and professional responsibility boundaries. The user guide should help a user move through setup, modeling, solving, rule checking, reports, and limitations while making missing data and human decision points visible.
>

### CLM-024 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Start with the authority boundary. | The guide should explain what OpenPipeStress computes and what remains user-supplied or human-reviewed before it teaches button-by-button workflows. |
> | Keep setup factual. | Packaging, install steps, storage containers, external transports, and dependency versions remain `TBD` until accepted elsewhere. Do not invent setup commands. |
> | Teach the centerline model. | Modeling guidance should orient users around projects, units, nodes, elements, components, supports, loads, combinations, and provenance. |
> | Keep mechanics and rule checks separate. | A mechanics solve produces displacements, forces, moments, reactions, stresses, and diagnostics. A user rule pack can evaluate those results only when required private/user data is present. |
> | Keep private data private. | Rule packs, owner requirements, licensed code values, vendor data, and private libraries belong in user-controlled/private paths unless redistribution rights are documented. |
> | Keep reports auditable. | Report guidance should emphasize manifests, model hashes, solver versions, warnings, assumptions, provenance, checksums, results, limitations, and human review notices. |
> | Keep limitations visible. | The guide should state when a result is incomplete, when local analysis may be needed, and when validation or professional review is outside software authority. |
>

### CLM-025 — Considerations

> ##### Considerations
>
> The user guide has to serve multiple audiences without making the product sound more mature or authoritative than the current evidence supports:
>
> - New users need a clear workflow from project creation through reports.
> - Experienced engineers need explicit status, warning, provenance, and limitation semantics.
> - Maintainers need a guide structure that can grow as implementation and validation evidence mature.
> - Educators need invented or public/permissive examples that do not rely on protected standards data.
>
> The guide should avoid presenting a future GUI, CLI, or report behavior as already implemented unless that behavior is supported by an accepted implementation deliverable. Where behavior is planned but unresolved, use `TBD` or "future section" language.
>

### CLM-026 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Preferred handling |
> |---|---|
> | Helpfulness vs false precision | Provide a section slot and the governing boundary; leave exact commands or screenshots as `TBD` until implemented. |
> | Engineering clarity vs protected content risk | Explain open mechanics and data responsibilities without copying code text, protected examples, or proprietary tables. |
> | Workflow simplicity vs warning visibility | Do not hide missing data, provenance gaps, rule-check blockers, or nonlinear diagnostics to make the guide look simpler. |
> | Public examples vs realism | Use invented or permissively sourced examples and label them as educational, non-code examples. |
> | Report confidence vs professional reliance | Reports can support review; they do not replace competent human acceptance for project use. |
>

### CLM-027 — Examples

> ##### Examples
>
> Acceptable guide examples:
>
> - A mechanics-only walkthrough using invented material and section values, clearly marked as non-code educational data.
> - A rule-pack workflow that uses invented demonstration values and shows missing-input blockers.
> - A report review checklist that names provenance, warnings, hashes, checksums, and limitations without quoting protected source material.
> - A troubleshooting example that explains `RULE_CHECK_BLOCKING` or `PROVENANCE_WARNING` without supplying proprietary values.
>
> Excluded guide examples:
>
> - Copied protected standards tables, figures, examples, formulas, or commentary.
> - Commercial software examples, manuals, screenshots, or benchmark files without redistribution permission.
> - Vendor catalog or owner data without documented public redistribution rights.
> - Any wording that implies software certification, endorsement, engineering seal, official approval, or automatic code compliance.
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
| OUT-001 | SOW-033 OBJ-001 OBJ-011 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
