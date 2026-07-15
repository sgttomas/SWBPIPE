---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-05
package_id: PKG-11
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-002]
---

# Scope of Work — DEL-11-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-11-05` in service of project scope [SOW-033] and package objectives [OBJ-001, OBJ-002].

- **OUT-001** — A contributor tutorial and onboarding contract for safe repository setup, bounded changes, tests, evidence, review, and governance-aware contribution is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-11-05 Contributor tutorial and onboarding

> #### Datasheet: DEL-11-05 Contributor tutorial and onboarding
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-05-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-11-05 |
> | Package ID | PKG-11 |
> | Package | Documentation, Examples, and Education |
> | Type | DOC_UPDATE |
> | Scope item | SOW-033 |
> | Objectives | OBJ-001; OBJ-002 |
> | Context envelope | S |
> | Lifecycle target for setup | SEMANTIC_READY after setup gates pass |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Source | Value |
> |---|---|---|
> | Primary purpose | _CONTEXT.md; Deliverables.csv row DEL-11-05 | Create an onboarding path for new contributors using package/deliverable decomposition and governance docs. |
> | Production surface | Human brief | Deliverable-local setup/document production only. Repo-level `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, documentation outside this deliverable, source code, and repo-level artifacts are excluded from this session. |
> | Intended audience | docs/AGENTIC_DEVELOPMENT_WORKFLOW.md section 1; AGENTS.md project-specific TASK profiles | New contributors and Type 2 TASK workers who need a safe first path through OpenPipeStress governance, decomposition, and bounded execution. |
> | Documentation targets | _CONTEXT.md; Deliverables.csv row DEL-11-05 | Future CONTRIBUTING tutorial material and future AGENTIC_DEVELOPMENT_WORKFLOW onboarding material, staged here only. |
> | Data boundary | INIT.md; docs/DIRECTIVE.md sections 3-5; docs/IP_AND_DATA_BOUNDARY.md sections 2-6 | Public onboarding must not introduce protected standards text, copied tables, protected examples, proprietary vendor data, private rule packs, owner standards, or company design bases. |
> | Professional boundary | docs/CONTRACT.md OPS-K-AUTH-1; docs/TYPES.md section 4; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md section 4 | Onboarding must not state or imply that software, agents, maintainers, or contributors certify, approve, seal, authenticate, or declare engineering code compliance for reliance. |
> | Agent boundary | AGENTS.md dispatch rule; docs/SPEC.md sections 10-11 | Type 2 work uses one sealed deliverable, explicit write scope, applicable invariants, acceptance criteria, and evidence. |
> | Architecture-basis context | _CONTEXT.md Architecture Basis Injection | Applicable architecture basis IDs are AB-00-01, AB-00-02, AB-00-06, AB-00-07, and AB-00-08; they inform contributor routing but do not mark PKG-00 as ISSUED. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Handling |
> |---|---|
> | No protected examples | Use invented, public-domain, or permissively licensed examples only; otherwise record `TBD` or quarantine per policy. |
> | No public defaults for code data | Contributor material must preserve user-supplied/private rule-pack and project-data boundaries. |
> | No certification claims | Use "human review", "maintainer review", "acceptance gate", or "draft/proposal" language as appropriate; avoid code-compliance certification claims. |
> | No out-of-scope edits | This setup deliverable does not edit repo-level onboarding files. Future integration must be a separate approved task or human action. |
> | Unknowns | Record `TBD`; do not invent policy, legal conclusions, engineering values, or source citations. |
>

### CLM-006 — Construction

> ##### Construction
>
> The onboarding path should be constructed as a contributor journey with the following local drafting components:
>
> | Component | Role | Boundary |
> |---|---|---|
> | Orientation | Explains OpenPipeStress intent, agent roles, decomposition hierarchy, and sealed-deliverable execution. | Must remain descriptive and source-grounded. |
> | Safe contribution checklist | Lists protected-data, provenance, privacy, and professional-responsibility checks before public contribution. | Does not replace legal or professional review. |
> | Type 2 task walkthrough | Shows how a contributor reads `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, and register rows before working. | Does not authorize cross-deliverable edits. |
> | Evidence and review handoff | Describes expected outputs, validation evidence, warnings, and review readiness. | Does not mark any artifact accepted or ISSUED. |
>

### CLM-007 — References

> ##### References
>
> | Reference | Sections used | Use |
> |---|---|---|
> | INIT.md | Required reading order; agent rule | Bootstrap and stop-rule context. |
> | AGENTS.md | Agent posture; primary agents; dispatch rule | Role and dispatch boundaries. |
> | docs/README.md | Document map; agent use package | Source navigation. |
> | docs/DIRECTIVE.md | Sections 1-6 | Project intent, boundaries, and stop rules. |
> | docs/CONTRACT.md | Invariant index | Binding invariants. |
> | docs/TYPES.md | Sections 1-9 | IDs, statuses, epistemic labels, authority vocabulary. |
> | docs/SPEC.md | Sections 1, 6-11 | Architecture boundary, public examples, reports, agent mechanics, acceptance semantics. |
> | docs/IP_AND_DATA_BOUNDARY.md | Sections 2-7 | Public/private data and quarantine policy. |
> | docs/VALIDATION_STRATEGY.md | Sections 1, 4-5 | Validation boundary and permitted benchmark/example sources. |
> | docs/AGENTIC_DEVELOPMENT_WORKFLOW.md | Sections 1-6 | Type 1/Type 2 workflow and review checklist. |
> | execution/_Decomposition/SOFTWARE_DECOMP.md | Revision 0.7; SOW-033; DEL-11-05 | Scope and objectives. |
> | docs/_Registers/Deliverables.csv | Row DEL-11-05 | Deliverable identity. |
> | docs/_Registers/ScopeLedger.csv | Row SOW-033 | Scope item basis. |
> | execution/_ScopeChange/SCA-001_2026-04-30_0045/Handoff_State.md | Explicit holds | Architecture-basis handling and ISSUED hold. |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-11-05 Contributor tutorial and onboarding

> #### Specification: DEL-11-05 Contributor tutorial and onboarding
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-05-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable defines a contributor onboarding tutorial draft inside the DEL-11-05 working folder. It covers how contributors should orient to OpenPipeStress governance, the package/deliverable decomposition, sealed Type 2 execution, evidence production, review handoff, and protected-data boundaries.
>
> This setup run does not edit repo-level `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, documentation outside this deliverable, source code, examples, schemas, or release artifacts. Future publication of the tutorial into repo-level onboarding surfaces requires a separate approved task or human action.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | REQ-11-05-01 | The tutorial draft shall start contributors from root bootstrap and governance reading before any implementation or documentation work. | INIT.md required reading order; docs/README.md document map | Procedure includes onboarding read sequence. |
> | REQ-11-05-02 | The tutorial draft shall explain the flat package/deliverable hierarchy and stable ID model used by OpenPipeStress. | docs/TYPES.md sections 1-2; SOFTWARE_DECOMP revision 0.7 | Datasheet and Procedure use PKG/DEL identity consistently. |
> | REQ-11-05-03 | The tutorial draft shall describe Type 1 and Type 2 agent responsibilities without expanding TASK authority beyond a sealed deliverable. | AGENTS.md primary agents and dispatch rule; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md sections 1-4 | Guidance and Procedure distinguish routing, execution, review, and human authority. |
> | REQ-11-05-04 | The tutorial draft shall require contributors to preserve protected standards, vendor IP, private data, and provenance controls. | docs/CONTRACT.md OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-PRIV-1; docs/IP_AND_DATA_BOUNDARY.md sections 2-6 | Guidance includes stop rules; dependencies record governing constraints. |
> | REQ-11-05-05 | The tutorial draft shall preserve the rule-pack boundary: public examples use invented non-code values and user/code data remains user supplied. | docs/CONTRACT.md OPS-K-RULE-1 and OPS-K-RULE-3; docs/SPEC.md section 6; ScopeLedger.csv SOW-033 | Guidance and Procedure prohibit protected or private rule-pack examples. |
> | REQ-11-05-06 | The tutorial draft shall preserve professional responsibility boundaries and avoid certification, code-compliance, approval, sealing, or reliance claims by agents/software. | docs/CONTRACT.md OPS-K-AUTH-1; docs/TYPES.md section 4; docs/DIRECTIVE.md sections 3 and 6 | Text scan confirms no software/agent compliance certification claim. |
> | REQ-11-05-07 | The tutorial draft shall tell contributors to use architecture-basis constraints only as dispatch context and not as ISSUED product authority. | _CONTEXT.md Architecture Basis Injection; SCA-001 Handoff_State.md Explicit Holds | Procedure includes architecture-basis handling. |
> | REQ-11-05-08 | The tutorial draft shall include evidence expectations: changed paths, validation commands/results, warnings, open issues, and review handoff. | docs/AGENTIC_DEVELOPMENT_WORKFLOW.md sections 4-5; docs/SPEC.md section 11 | Procedure includes final handoff checklist. |
> | REQ-11-05-09 | The setup artifacts shall include four documents, semantic matrix, lensing register, dependency register, run records, and final status only if setup gates pass. | Human brief; skills/four-documents/SKILL.md; skills/semantic-matrix-build/SKILL.md; skills/lens-register/SKILL.md; skills/dependency-extract/SKILL.md | Local validation commands pass before final `_STATUS.md` is set to SEMANTIC_READY. |
>

### CLM-012 — Standards

> ##### Standards
>
> No engineering design standard content is used as source material for this contributor tutorial. Governing standards for this deliverable are the project governance and workflow documents listed in `_REFERENCES.md`. If a future contributor wants to include any standard, vendor, owner, or commercial software material in onboarding examples, the correct action is to stop and route the material through protected-content/provenance review rather than copy or paraphrase it into public documentation.
>

### CLM-013 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four-document presence | `tools/validation/check_four_documents.sh <deliverable>` returns PASS. |
> | Minimum metadata presence | `tools/validation/check_min_viable_fileset.sh <deliverable>` returns PASS. |
> | Dependency schema | `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv` returns VALID. |
> | Lifecycle enum | `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY` returns VALID. |
> | Protected-data boundary scan | Local review finds no protected standards tables, code examples, proprietary vendor data, private rule packs, or commercial software examples introduced. |
> | Professional-claims scan | Local review finds no automatic compliance, certification, approval, sealing, or professional reliance claim by software or agents. |
> | Semantic setup | `_SEMANTIC.md` has Audit Result PASS and `_SEMANTIC_LENSING.md` includes coverage rows for matrices A, B, C, F, D, X, and E. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required setup outputs for this deliverable are:
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
> Repo-level publication targets remain references only in this setup session.
>

### CLM-015 — Required Invariants

> ##### Required Invariants
>
> | Invariant | Application |
> |---|---|
> | OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3 | Public onboarding content must exclude protected standards/vendor/commercial/private material and must require provenance review for public data. |
> | OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3 | Code-specific and proprietary engineering data are user supplied or private; missing values stay explicit. |
> | OPS-K-RULE-1, OPS-K-RULE-3 | Public rule-pack examples are invented/non-code and rule packs carry version, checksum, source notes, and public/private markers where referenced. |
> | OPS-K-AUTH-1 | No software or agent certification, approval, sealing, authentication, or code-compliance reliance claim. |
> | OPS-K-PRIV-1 and OPS-K-PRIV-2 | Private project/code/rule/component data and telemetry boundaries stay visible. |
> | OPS-K-AGENT-1 through OPS-K-AGENT-4 | Contributors and agents do not invent facts; conflicts are surfaced; Type 2 work is sealed; outputs are drafts until accepted. |
> | No-bypass adapter/API baseline | Contributors touching plugins/adapters/API topics must preserve units, provenance, diagnostics, rule sandboxing, report controls, and data-boundary checks. |
>

### CLM-016 — Acceptance Criteria

> ##### Acceptance Criteria
>
> This setup deliverable is ready for review when the listed setup artifacts exist, validation commands pass, no protected-data or certification-claim warning is found in the local artifacts, and `_STATUS.md` records `Current State: SEMANTIC_READY`.

- **AC-001** — The contract preserves the source-defined contributor path, architecture-basis handling, schema and unit invariants, deterministic checks, protected/private-data and licensing boundaries, documentation and review records, safe-versus-unsafe examples, visible unresolved onboarding decisions, and the distinction between contribution acceptance and professional engineering approval.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-11-05 Contributor tutorial and onboarding

> #### Procedure: DEL-11-05 Contributor tutorial and onboarding
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-018 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-11-05-DECL-004`.
>

### CLM-019 — Purpose

> ##### Purpose
>
> This procedure defines how to produce and verify the contributor onboarding tutorial draft for DEL-11-05 inside the deliverable working folder.
>

### CLM-020 — Prerequisites

> ##### Prerequisites
>
> - The contributor has read the assigned brief and confirmed the write scope is only this deliverable folder.
> - `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, and register rows for DEL-11-05 and SOW-033 are available.
> - Governing references listed in `_REFERENCES.md` are accessible for reading.
> - No protected standards examples, proprietary vendor data, private rule packs, owner standards, or commercial software examples are imported.
> - The current lifecycle state allows setup refresh.
>

### CLM-021 — Steps

> ##### Steps
>
> 1. Confirm deliverable identity.
>    - Read `_CONTEXT.md`.
>    - Verify `Deliverable ID: DEL-11-05`, `Package ID: PKG-11`, `Type: DOC_UPDATE`, `Scope Coverage: SOW-033`, and `Objective Support: OBJ-001; OBJ-002`.
>
> 2. Confirm source authority.
>    - Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and the decomposition/register rows listed in `_REFERENCES.md`.
>    - Use source locations in the document kit where requirements or constraints are stated.
>
> 3. Draft the contributor tutorial concept in the four-document kit.
>    - Put descriptive identity, boundaries, and references in `Datasheet.md`.
>    - Put normative onboarding requirements, invariants, and acceptance criteria in `Specification.md`.
>    - Put rationale, trade-offs, safe/unsafe examples, and source-boundary advice in `Guidance.md`.
>    - Put contributor execution and validation steps in this `Procedure.md`.
>
> 4. Preserve hard boundaries.
>    - Do not edit repo-level `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, source code, examples, schemas, or documentation outside this deliverable.
>    - Do not introduce protected standards text, copied standards tables, code-derived formulas, commercial software examples, proprietary vendor data, private rule packs, owner standards, or company design bases.
>    - Do not claim software, agents, contributors, or maintainers certify, approve, seal, authenticate, or declare engineering code compliance.
>
> 5. Generate setup artifacts in sequence.
>    - Run `four-documents` with `RUN_PASSES=P1_P2`.
>    - Run `semantic-matrix-build`.
>    - Run `lens-register`.
>    - Run `four-documents` with `RUN_PASSES=P3_ONLY`.
>    - Run `dependency-extract`.
>
> 6. Validate setup gates.
>    - Run `tools/validation/check_min_viable_fileset.sh <deliverable>`.
>    - Run `tools/validation/check_four_documents.sh <deliverable>`.
>    - Run `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`.
>    - Run `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY`.
>    - Scan the local artifacts for protected-data risk and certification/compliance-claim language.
>
> 7. Set status only after gates pass.
>    - If all setup gates pass, update `_STATUS.md` to `SEMANTIC_READY`.
>    - If any setup gate fails, leave status below `SEMANTIC_READY`, record the failure in `_run_records`, and surface open issues.
>
> 8. Return the handoff.
>    - Report status, changed paths, validation commands/results, warnings, and open issues.
>

### CLM-022 — Verification

> ##### Verification
>
> The setup is valid when all required files exist, dependency schema validation passes, the semantic lens and lensing register exist, run records are present, no out-of-scope file is modified, and `_STATUS.md` is set to `SEMANTIC_READY` only after those checks pass.
>

### CLM-023 — Records

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
> - `_STATUS.md`
> - `_run_records/TASK_RUN_*.md`

- **VER-001** — Validate the contract and review source parity, onboarding-step and contributor-path coverage, architecture and invariant handling, test/evidence/review expectations, protected/private-data and licensing controls, retained conflicts and TBDs, and professional-responsibility limits.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-11-05 Contributor tutorial and onboarding

> #### Guidance: DEL-11-05 Contributor tutorial and onboarding
>

### CLM-025 — Purpose

> ##### Purpose
>
> The contributor tutorial should make the first hour of contribution work predictable: read the governing documents, locate the active deliverable, understand the data and authority boundaries, work only inside the assigned scope, produce evidence, and hand off for review. The tutorial is a path through the existing governance system, not a replacement for `CONTRIBUTING`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, or human review.
>

### CLM-026 — Principles

> ##### Principles
>
> 1. Start with boundaries before mechanics. Contributors need the OpenPipeStress rule that public code may implement open mechanics while protected standards and proprietary engineering data remain outside public defaults.
> 2. Treat the decomposition as the work identity. A contributor works on one `DEL-XX-YY` inside one `PKG-XX` unless a human changes scope.
> 3. Keep authority labels clear. Type 2 outputs are drafts/proposals until accepted; deterministic tools provide evidence, not judgment; humans resolve scope changes and acceptance gates.
> 4. Prefer `TBD` over invention. Missing policy choices, engineering values, source citations, or legal conclusions are surfaced rather than guessed.
> 5. Keep examples clean. Public examples are invented, original, public-domain, or permissively licensed with provenance. They do not copy protected standards examples or commercial software cases.
>

### CLM-027 — Considerations

> ##### Considerations
>

### CLM-028 — Contributor Path

> ###### Contributor Path
>
> A contributor-facing tutorial should guide the reader through this sequence:
>
> 1. Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and the active decomposition/register rows relevant to the assigned deliverable.
> 2. Confirm the sealed deliverable identity, package, scope items, objectives, invariants, acceptance criteria, and explicit write scope.
> 3. Inspect `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_STATUS.md` before changing production artifacts.
> 4. Make only scoped edits and record evidence through validation commands, warnings, and open issues.
> 5. Return a handoff with changed paths, validation results, data-boundary notes, and review readiness.
>

### CLM-029 — Protected-Data Boundary

> ###### Protected-Data Boundary
>
> Contributor onboarding should be explicit that the public repository may contain schemas, mechanics, workflows, blank templates, and invented or permissively licensed examples, but not protected standards text/tables/examples, copied code formulas, proprietary vendor catalogs without rights, private owner standards, private rule packs, or company design bases. If a source appears protected, the tutorial should instruct the contributor to stop, mark the concern, and escalate through the project review path rather than paraphrase it.
>

### CLM-030 — Professional Responsibility Boundary

> ###### Professional Responsibility Boundary
>
> The tutorial should avoid language that sounds like certification. A maintainer can accept a development artifact, but that is not a professional engineering approval of a piping calculation. A user rule-check result is not a professional code-compliance declaration. Software output remains decision support until a competent human accepts it for a particular project.
>

### CLM-031 — Architecture-Basis Handling

> ###### Architecture-Basis Handling
>
> SCA-001 allows `PKG-00` `SEMANTIC_READY` content to be used as an architecture-basis candidate for sealed brief injection. Contributors should use applicable basis IDs as context constraints, especially the no-bypass API/adapter baseline, without treating PKG-00 as `ISSUED` or copying full PKG-00 prose into new artifacts.
>

### CLM-032 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Helpful tutorial vs. over-specific process | Provide a concrete path through existing files, but keep repo-level publication and policy decisions separate from this deliverable. |
> | Contributor confidence vs. certification language | Explain what maintainers review and what tools check, but avoid any statement that software or agents approve engineering work. |
> | Rich examples vs. IP risk | Use invented examples and provenance notes rather than real protected code examples or commercial software files. |
> | Automation vs. human authority | Use scripts for evidence and repeatability; preserve human gates for scope, acceptance, legal/professional questions, and ambiguous protected content. |
>

### CLM-033 — Examples

> ##### Examples
>

### CLM-034 — Safe Onboarding Step

> ###### Safe Onboarding Step
>
> > Confirm `DeliverableID`, `PackageID`, `Scope Coverage`, `Objective Support`, write scope, and applicable invariants from `_CONTEXT.md` before editing.
>
> This is a process example only. It does not introduce engineering values or standards material.
>

### CLM-035 — Unsafe Onboarding Step

> ###### Unsafe Onboarding Step
>
> > Copy a code example from a standard or commercial report and adapt it into public docs.
>
> This violates the protected-content boundary unless documented redistribution rights and human/legal review explicitly allow it.
>

### CLM-036 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No source conflicts were detected during setup. If future repo-level onboarding asks this deliverable to edit `CONTRIBUTING` or `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` directly, that is a scope change from this setup session and must be routed for human approval.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-033 OBJ-001 OBJ-002 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
