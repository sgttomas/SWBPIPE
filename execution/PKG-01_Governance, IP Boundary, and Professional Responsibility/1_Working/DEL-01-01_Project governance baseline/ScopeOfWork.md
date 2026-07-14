---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-01
package_id: PKG-01
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@c5abf91b717c0b3901d2a27c578e63976853f8de
project_scope_refs: [SOW-001, SOW-048]
package_objective_refs: [OBJ-001, OBJ-002]
---

# Scope of Work — DEL-01-01

## Purpose and Objective Traceability

This Scope of Work defines `DEL-01-01` in service of project scope [SOW-001, SOW-048] and package objectives [OBJ-001, OBJ-002].

- **OUT-001** — A project governance baseline covering the selected noncommercial license, public governance surfaces, protected-content and professional-authority boundaries, maintainer and release policy slots, and human-gated agent output is preserved for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-01-01 Project governance baseline

> #### Datasheet: DEL-01-01 Project governance baseline
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-01-01 |
> | Package ID | PKG-01 |
> | Package Name | Governance, IP Boundary, and Professional Responsibility |
> | Deliverable Name | Project governance baseline |
> | Type | DOC_UPDATE |
> | Scope Items | SOW-001, SOW-048 |
> | Objectives | OBJ-001, OBJ-002 |
> | Context Envelope | M |
> | Decomposition Basis | execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7 |
> | Coordination Basis | execution/_DAG/DAG-006/ approved active graph authority |
> | Status | IN_PROGRESS governance baseline refresh |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Product stance | Source-available noncommercial piping stress analysis platform | SOW-001; docs/DIRECTIVE.md Section 6 |
> | License selection | `PolyForm-Noncommercial-1.0.0` | Human project authority ruling on 2026-06-03; OPS-K-GOV-1; SOW-048 notes |
> | Governance surfaces | `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `governance/MAINTAINERS.md` | _CONTEXT.md anticipated artifacts |
> | Policy scope | Governance baseline, maintainer policy skeleton, release/governance boundary language | DEL-01-01 register row; SOW-048 |
> | Public data boundary | Protected standards text, tables, figures, examples, protected formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data are not public repository content | OPS-K-IP-1; docs/DIRECTIVE.md Sections 3 and 4 |
> | Professional authority boundary | Software and agents do not certify, seal, approve, authenticate, or declare engineering code compliance for reliance | OPS-K-AUTH-1; docs/DIRECTIVE.md Section 6 |
> | Agent authority boundary | Type 2 outputs are drafts/proposals until accepted by a human gate | OPS-K-AGENT-4; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Status |
> |---|---|
> | Repo-level governance surfaces exist and are explicitly in current-basis refresh scope | Applied |
> | No protected standards/code data is reproduced | Applied |
> | No legal conclusion beyond draft policy language is made | Applied |
> | Maintainer roster, release authority, quorum, and signing | TBD |
> | Public release maturity labels and validation disclosure format | TBD |
> | Human project authority for governance decisions | TBD |
>

### CLM-005 — Governance Decision Surface

> ##### Governance Decision Surface
>
> | Decision Surface | Current Value | Authority Needed |
> |---|---|---|
> | Project license | `PolyForm-Noncommercial-1.0.0` | Human project authority ruling recorded 2026-06-03 |
> | Maintainer roster | TBD | Human project authority |
> | Maintainer quorum or approval model | TBD | Human project authority |
> | Release authority | TBD | Human project authority |
> | Release signing or provenance process | TBD | Human project authority |
> | Legal review process for license/IP questions | TBD | Human project authority |
>

### CLM-006 — Construction

> ##### Construction
>
> This deliverable-local kit describes the baseline content and current-basis
> maintenance rules for public governance artifacts. It may update
> `docs/README.md` and `governance/MAINTAINERS.md` under explicit tranche write
> scope. The project license is resolved as `PolyForm-Noncommercial-1.0.0`;
> remaining human-governed decisions stay `TBD`.
>
> The baseline is constructed from:
>
> - scope item SOW-001: source-available noncommercial platform intent;
> - scope item SOW-048: license, governance, release, and maintainer policy obligation;
> - objective OBJ-001: auditable, inspectable, extensible platform;
> - objective OBJ-002: protected-standards and user-supplied-data separation;
> - contract invariants for hierarchy, identifiers, IP boundary, professional authority, governance, and agent execution.
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md`
> - `_REFERENCES.md`
> - `execution/_Decomposition/SOFTWARE_DECOMP.md`
> - `execution/_DAG/DAG-006/`
> - `docs/_Registers/Deliverables.csv`
> - `docs/_Registers/ScopeLedger.csv`
> - `docs/_Registers/ContextBudgetQA.csv`
> - `docs/CONTRACT.md`
> - `docs/DIRECTIVE.md`
> - `docs/TYPES.md`
> - `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-01-01 Project governance baseline

> #### Specification: DEL-01-01 Project governance baseline
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable specifies a draft governance baseline for OpenPipeStress. It covers the intended public/free/open-source posture, maintainer and release policy skeleton needs, and required policy boundaries for protected content, professional authority, and agent-generated outputs.
>
> This deliverable excludes product implementation, solver behavior, GUI
> behavior, legal advice, license selection, certification/approval language,
> release acceptance, and professional/code-compliance claims. It may update the
> visible docs index and maintainer policy skeleton when an approved tranche
> explicitly grants repo-level governance write scope.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | ID | Requirement | Basis | Verification |
> |---|---|---|---|
> | REQ-01-01-01 | The governance baseline must preserve the human-selected project license, `PolyForm-Noncommercial-1.0.0`, while avoiding legal advice and professional/code-compliance claims. | SOW-001; SOW-048; OPS-K-GOV-1 | Confirm the selected license is visible and no legal conclusion is asserted. |
> | REQ-01-01-02 | Governance language must distinguish project governance approval from professional engineering approval. | OPS-K-AUTH-1; docs/DIRECTIVE.md Section 6 | Check that maintainer/release approval is not described as certification, sealing, authentication, endorsement, or code-compliance approval. |
> | REQ-01-01-03 | Public contribution and release policy language must include protected-content, provenance, redistribution-rights, privacy, and review gates. | OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-GOV-4 | Check for policy slots covering source/provenance, redistribution status, contributor certification, review disposition, quarantine/escalation, and private-data risk. |
> | REQ-01-01-04 | Public release policy language must require disclosure of scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations. | OPS-K-GOV-3 | Check release policy skeleton for each disclosure category or an explicit `TBD` slot. |
> | REQ-01-01-05 | Maintainer authority, release authority, and public-governance decisions must be recorded in public governance artifacts before being treated as project policy. | OPS-K-GOV-2 | Check that draft language does not treat unrecorded roles, quorum, signing, or license choices as effective policy. |
> | REQ-01-01-06 | Agent-produced governance content must remain draft/proposal material until accepted by the human gate. | OPS-K-AGENT-4; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md | Check for draft status and absence of issued/final acceptance claims. |
> | REQ-01-01-07 | Unknown governance values must be marked `TBD`; the deliverable must not invent legal conclusions, maintainer identities, quorum rules, signing process, or compliance claims. | OPS-K-AGENT-1; docs/DIRECTIVE.md Section 6 | Inspect all governance-value fields and open policy choices. |
> | REQ-01-01-08 | The deliverable must preserve flat package/deliverable identity and stable IDs. | OPS-K-HIER-1; OPS-K-ID-1 | Confirm DEL-01-01, PKG-01, SOW, and OBJ IDs match registers. |
>

### CLM-011 — Acceptance Criteria

> ##### Acceptance Criteria
>
> | ID | Criterion | Evidence |
> |---|---|---|
> | AC-01-01-01 | Project license is recorded as `PolyForm-Noncommercial-1.0.0`; no legal conclusion is asserted. | Datasheet decision surface; Guidance conflict C-01-01-001 |
> | AC-01-01-02 | Contribution-review policy slots include source, provenance, redistribution status, contributor certification, review disposition, quarantine status, and private-data risk. | Specification REQ-01-01-03; future repo-level governance artifact review |
> | AC-01-01-03 | Release policy slots include scope, validation status, known limitations, data-boundary constraints, professional-responsibility limitations, and release maturity wording marked `TBD` until decided. | Specification REQ-01-01-04; Guidance trade-offs |
> | AC-01-01-04 | Maintainer/release authority values remain `TBD` until recorded by the human project authority. | Datasheet decision surface; Guidance conflict C-01-01-002 |
> | AC-01-01-05 | Run evidence records which repo-level governance artifacts were edited and confirms no protected standards/code content was reproduced. | `_run_records/TASK_RUN_*.md` |
>

### CLM-012 — Standards

> ##### Standards
>
> | Standard or Authority | Status |
> |---|---|
> | `docs/CONTRACT.md` invariant catalog | Accessible draft governance authority |
> | `docs/DIRECTIVE.md` founding directive | Accessible draft governance authority |
> | `docs/TYPES.md` identity and vocabulary | Accessible draft governance authority |
> | Project license | `PolyForm-Noncommercial-1.0.0`; see `LICENSE.md` and official PolyForm URL |
> | Protected standards/code requirements | Not reproduced; public data boundary only |
>

### CLM-013 — Verification

> ##### Verification
>
> Verification for this setup deliverable is document review:
>
> - compare DEL-01-01 identity, scope items, objectives, and anticipated artifacts against `_CONTEXT.md`, current decomposition revision `0.7`, `DAG-006`, and the registers;
> - confirm all applicable contract invariants are named or reflected;
> - confirm any repo-level governance edits stay within the authorized tranche write scope;
> - confirm no protected standards content, proprietary data, legal conclusion, or professional certification claim was introduced;
> - confirm `PolyForm-Noncommercial-1.0.0` is used for the selected project license and `TBD` is used for unresolved maintainer, release, quorum, signing, and human-authority choices;
> - confirm `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, and `_run_records` evidence exist after the setup workflow completes.
>

### CLM-014 — Documentation

> ##### Documentation
>
> Anticipated downstream artifacts from the register are:
>
> - `docs/CONTRACT.md`
> - `docs/DIRECTIVE.md`
> - `governance/MAINTAINERS.md`
>
> For current-basis refresh work, `docs/README.md` and
> `governance/MAINTAINERS.md` may be updated as governance surfaces. Changes to
> release acceptance and lifecycle state remain outside this tranche unless
> separately authorized.
>
> The release maturity label taxonomy, validation-status wording, release signing process, and governance acceptance record format remain `TBD`.

- **AC-001** — The contract preserves the exact issued governance baseline, including the selected PolyForm-Noncommercial-1.0.0 license, protected-content and professional-authority boundaries, human-gated agent output, contribution and release policy slots, and all unresolved maintainer, quorum, signing, release-authority, legal-review, and maturity-label values as TBD without adding legal, certification, compliance, release, or professional-reliance claims.

## Production and Verification Method — Praxeology

### CLM-015 — Procedure: DEL-01-01 Project governance baseline

> #### Procedure: DEL-01-01 Project governance baseline
>

### CLM-016 — Purpose

> ##### Purpose
>
> This procedure describes how to produce, refresh, and review the DEL-01-01
> governance baseline artifacts and authorized visible governance surfaces.
>

### CLM-017 — Prerequisites

> ##### Prerequisites
>
> - Sealed DEL-01-01 brief and write scope.
> - `_CONTEXT.md`, `_REFERENCES.md`, current decomposition revision `0.7`,
>   approved `DAG-006`, and register rows for DEL-01-01.
> - Access to `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/TYPES.md`, and `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`.
> - No protected standards/code data or proprietary contribution data.
>

### CLM-018 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity: DEL-01-01, PKG-01, SOW-001, SOW-048, OBJ-001, and OBJ-002.
> 2. Read the applicable contract invariants for hierarchy, IDs, IP boundary, professional authority, governance, and agent execution.
> 3. Draft or refresh deliverable-local `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`; edit repo-level governance targets only when the active tranche grants explicit write scope.
> 4. Record the selected project license as `PolyForm-Noncommercial-1.0.0`; mark remaining unresolved governance choices as `TBD`, including maintainer roster, quorum, release signing, release authority, and legal review process.
> 5. Check that draft language does not assert certification, sealing, endorsement, code compliance, legal opinion, or professional approval.
> 6. Generate `_SEMANTIC.md` as a question-shaping semantic lens after the four documents exist.
> 7. Generate `_SEMANTIC_LENSING.md` from `_SEMANTIC.md` and the four production documents.
> 8. Apply P3 enrichment only where the lensing register identifies warranted additions supported by local sources.
> 9. Generate `Dependencies.csv` and refresh `_DEPENDENCIES.md` using conservative dependency extraction.
> 10. Persist `_run_records` evidence for the setup or refresh sequence and leave `_STATUS.md` unchanged unless a separate human lifecycle gate authorizes a transition.
>

### CLM-019 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Scope boundary | Writes stay within the deliverable folder and any explicitly authorized governance surfaces. |
> | Repo-level targets | `docs/README.md` and `governance/MAINTAINERS.md` may be edited when explicitly authorized; `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, lifecycle files, release records, and DAG artifacts are not edited by this refresh. |
> | Protected content | No protected standards/code text, tables, examples, or proprietary data are reproduced. |
> | Professional boundary | No certification, approval, sealing, authentication, or compliance-for-reliance claim appears. |
> | Unknowns | Unresolved policy values are marked `TBD`. |
> | Setup artifacts | Four documents, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, and `_run_records` exist. |
>
> Release validation disclosures are communication controls. They must not be treated as professional reliance approval, code compliance, certification, sealing, endorsement, or authentication.
>
> The durable run record should explicitly state whether repo-level artifacts were edited, whether protected standards/code content was reproduced, and which unresolved governance decisions remain `TBD`.
>

### CLM-020 — Records

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

- **VER-001** — Validate the contract and independently review exact source parity, governance identity and objective traceability, license and policy boundaries, contribution and release checklist coverage, protected-content and professional-authority controls, human-gated agent status, and every unresolved governed value.

## Governing Values and Decisions — Axiology

### CLM-021 — Guidance: DEL-01-01 Project governance baseline

> #### Guidance: DEL-01-01 Project governance baseline
>

### CLM-022 — Purpose

> ##### Purpose
>
> This deliverable-local kit gives later human or Type 1 governance work a
> structured baseline for the public project governance surface. It is
> intentionally conservative: it records required policy boundaries and
> unresolved choices without asserting legal conclusions, release acceptance, or
> final project policy.
>

### CLM-023 — Principles

> ##### Principles
>
> - **Open mechanics, protected standards.** Governance should support a public, inspectable mechanics platform while preventing protected standards content or proprietary commercial data from entering public artifacts.
> - **Maintainer authority is project authority only.** Maintainers may govern repository process, releases, and contribution acceptance; they do not professionally approve piping calculations by maintaining or releasing software.
> - **Release labels are software labels.** A release may describe maturity, validation evidence, and known limitations; it must not imply engineering compliance, certification, endorsement, or sealing.
> - **Unknown policy choices remain TBD.** The project license is selected as `PolyForm-Noncommercial-1.0.0`; release quorum, maintainer roster, signing process, and legal review process remain human-governed `TBD`s.
> - **Drafts are not policy.** Agent outputs are proposals until accepted by the human gate and recorded in the appropriate public governance artifact.
>

### CLM-024 — Considerations

> ##### Considerations
>
> The current governing documents already contain much of the baseline intent.
> Repo-level governance edits should avoid duplicating or weakening existing
> invariants. `governance/MAINTAINERS.md` should point back to
> `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and
> the current decomposition/DAG authority rather than restating every invariant
> in full.
>
> Contribution review should be written as a gate with explicit evidence fields. At minimum, public data contributions need source, provenance, redistribution status, contributor certification, and review disposition. Suspected protected content should be quarantined and escalated rather than rewritten or paraphrased.
>
> Release review should include a checklist for scope disclosure, validation status, known limitations, data-boundary constraints, and professional-responsibility limits. These are governance and communication controls, not proofs of engineering adequacy for a project.
>
> The human project authority selected `PolyForm-Noncommercial-1.0.0` for the
> project license on 2026-06-03. The human project authority record, maintainer,
> release, and policy acceptance decisions remain unresolved; until recorded,
> those choices remain `TBD` and should be routed for human ruling before
> repo-level governance artifacts are changed.
>

### CLM-025 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Project license | Use `PolyForm-Noncommercial-1.0.0` as selected by human project authority. |
> | Central maintainer vs quorum model | Leave as `TBD`; record the selected model before treating it as policy. |
> | Public examples vs educational usefulness | Public examples must remain invented, permissively sourced, or public-domain with provenance. |
> | Automation vs human review | Automation can provide evidence, but protected-content, legal, and professional-boundary decisions remain human governance gates. |
> | Concise policy vs complete policy | Prefer concise public policy backed by explicit checklists and decision records; avoid vague assurances. |
>

### CLM-026 — Examples

> ##### Examples
>
> Acceptable draft phrasing:
>
> - "The project license is PolyForm Noncommercial 1.0.0."
> - "Maintainer approval is repository governance and is not professional engineering approval."
> - "Public releases disclose validation status and known limitations."
>
> Unacceptable draft phrasing:
>
> - "This release is code compliant."
> - "Maintainer approval certifies calculations."
> - "The project includes standards-derived allowables by default."
>

### CLM-027 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | C-01-01-001 | Project license was previously unresolved. | SOW-048; OPS-K-GOV-1 | docs/DIRECTIVE.md Section 6 | Datasheet Conditions; Specification Requirements; Procedure Steps | Use `PolyForm-Noncommercial-1.0.0` as selected by human project authority. | Resolved 2026-06-03 |
> | C-01-01-002 | Maintainer roster, quorum, release signing, and release authority are required governance choices but are not yet recorded. | OPS-K-GOV-2; docs/DIRECTIVE.md Section 6 | _CONTEXT.md anticipated artifacts | Specification Requirements; Procedure Records | Treat all role/quorum/signing values as `TBD` and do not treat them as policy. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-001 SOW-048 OBJ-001 OBJ-002 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
