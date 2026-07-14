---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-03
package_id: PKG-01
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-028, SOW-048]
package_objective_refs: [OBJ-002]
---

# Scope of Work — DEL-01-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-01-03` in service of project scope [SOW-028, SOW-048] and package objectives [OBJ-002].

- **OUT-001** — A contributor certification workflow and template requirements for attestations, provenance fields, protected-content screening, review routing, quarantine or rejection, and disposition records.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-01-03 Contributor certification workflow

> #### Datasheet: DEL-01-03 Contributor certification workflow
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-01-03 |
> | Name | Contributor certification workflow |
> | Package ID | PKG-01 |
> | Package Name | Governance, IP Boundary, and Professional Responsibility |
> | Type | DOC_UPDATE |
> | Objective | OBJ-002 |
> | Scope Items | SOW-028, SOW-048 |
> | Source basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; approved `DAG-006`; `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/DIRECTIVE.md`; `docs/SPEC.md`; register rows for DEL-01-03, SOW-028, SOW-048 |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Workflow purpose | Define contributor attestations, provenance fields, review routing, and rejection rules for public data contributions. | `_CONTEXT.md` Description |
> | Anticipated repo artifacts | `CONTRIBUTING.md` and `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` exist as draft repo-level governance artifacts. | `_CONTEXT.md` Anticipated Artifacts; `CONTRIBUTING.md`; `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` |
> | Local artifact boundary | This current-basis refresh updates the DEL-01-03 local kit only; repo-level artifacts are read as evidence and remain draft governance surfaces. | Approved tranche `TP-DEL-01-03-CURRENT-BASIS-REFRESH-001` |
> | License decision | TBD. The project intends to be free/open-source, but exact license remains a human project authority decision. | `docs/CONTRACT.md` OPS-K-GOV-1; `docs/DIRECTIVE.md` section 6 |
> | Maintainer/release authority | TBD until recorded in public governance artifacts. | `docs/CONTRACT.md` OPS-K-GOV-2 |
> | Public contribution review gate | Source, provenance, redistribution rights, protected-content risk, private-data risk, and test evidence must be checked before merge. | `docs/DIRECTIVE.md` section 6; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 5 |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Required handling | Source |
> |---|---|---|
> | Suspected protected standards or proprietary content | Stop ingestion, mark as suspected protected content, quarantine outside public examples, record the issue, and request human/legal review. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; `docs/CONTRACT.md` OPS-K-IP-3 |
> | Missing source, license, or redistribution status | Treat as unresolved; do not accept as public data until reviewed. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/CONTRACT.md` OPS-K-IP-2 |
> | Legal or license conclusion needed | Record `TBD` and route to the human project authority/legal review. | `docs/CONTRACT.md` OPS-K-GOV-1; SOFTWARE_DECOMP OI-001, OI-003 |
> | Certification wording | Must not claim engineering certification, approval, sealing, legal clearance, or compliance for reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-AGENT-4 |
>

### CLM-005 — Construction

> ##### Construction
>

### CLM-006 — Contributor Certification Template Fields

> ###### Contributor Certification Template Fields
>
> | Field | Required value or status | Notes |
> |---|---|---|
> | contributor_name | Required | Person or organization submitting the data. |
> | contribution_description | Required | Short description of submitted data or artifact. |
> | source_name | Required for public data records | Matches `docs/IP_AND_DATA_BOUNDARY.md` required provenance fields. |
> | source_location | Required or `TBD` | URL, path, document identifier, or `TBD`. |
> | source_license | Required or `TBD` | Redistribution basis or unresolved. |
> | redistribution_status | Required | `public_permissive`, `private_only`, `unknown`, or `protected_suspected`. |
> | contributor_certification | Required | Statement that the contribution is original, permissively redistributable, or otherwise submitted with documented rights; no protected standards/proprietary content is copied unless rights are documented. |
> | protected_content_screen | Required | Records whether protected standards text/tables/figures/examples, copied formulas, protected dimensional tables, or proprietary catalog data appear suspected. |
> | private_data_screen | Required | Records whether user-private project, owner, rule-pack, material, or component data is present. |
> | review_status | Required | `pending`, `accepted`, `rejected`, or `quarantined`. |
> | reviewer | Required at disposition | Maintainer/reviewer identity; final role assignment remains `TBD`. |
> | disposition_notes | Required at disposition | Short rationale and links to evidence, issue, or quarantine record. |
>

### CLM-007 — Review Routing States

> ###### Review Routing States
>
> | State | Meaning |
> |---|---|
> | Intake pending | Contribution has not yet passed completeness/provenance screening. |
> | Provenance review | Source, license, redistribution status, and contributor attestation are being checked. |
> | Protected-content review | Contribution is blocked pending protected-content/private-data review. |
> | Quarantined | Suspected protected or private content has been isolated outside public examples. |
> | Accepted for public repo | Reviewer recorded public redistribution basis and no protected/private-data blocker. |
> | Rejected | Contribution cannot be accepted under current evidence. |
>

### CLM-008 — References

> ##### References
>
> - `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5.
> - `docs/CONTRACT.md` invariant index.
> - `docs/DIRECTIVE.md` sections 5-6.
> - `docs/SPEC.md` sections 6, 8, 10-11.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 rows DEL-01-03, SOW-028, SOW-048, OBJ-002, AB-00-01, AB-00-02, AB-00-06, AB-00-08.
> - `execution/_DAG/DAG-006/` approved active graph authority.
> - `CONTRIBUTING.md` and `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` draft repo-level contributor workflow artifacts.
> - `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, `ContextBudgetQA.csv` rows named in `_CONTEXT.md`.
>

### CLM-009 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Contributor certification templates and issue-intake surfaces now exist under the repository's PolyForm-Noncommercial-1.0.0 boundary. Current upstream authority is SOFTWARE_DECOMP revision 0.8 with DAG-007 coordination; contributor disposition remains subject to the recorded governance workflow.

## Completion and Reliance Basis — Epistemology

### CLM-010 — Specification: DEL-01-03 Contributor certification workflow

> #### Specification: DEL-01-03 Contributor certification workflow
>

### CLM-011 — Scope

> ##### Scope
>
> This deliverable defines the local requirements and evidence kit for a contributor certification workflow covering attestations, provenance fields, review routing, rejection rules, and quarantine handling for public data contributions.
>
> `CONTRIBUTING.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` exist as draft repo-level governance artifacts. The project license has been selected as `PolyForm-Noncommercial-1.0.0`. This current-basis refresh does not edit those repo-level artifacts, select the final contributor legal mechanism, assign final maintainer or reviewer authority, make legal conclusions, certify rights, or approve engineering reliance.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source |
> |---|---|---|
> | DEL-01-03-REQ-01 | The workflow shall require contributor-provided source, location, license or `TBD`, contributor identity, certification statement, redistribution status, and review status for public data records. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; OPS-K-IP-2 |
> | DEL-01-03-REQ-02 | The workflow shall reject or quarantine contributions with suspected protected standards text, tables, figures, examples, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data without redistribution rights, or private user/project/rule-pack data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 5; OPS-K-IP-1, OPS-K-IP-3 |
> | DEL-01-03-REQ-03 | The workflow shall route suspected protected content to human/legal review and shall not paraphrase protected tables or values into public data. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; OPS-K-IP-3; sealed brief hard stops |
> | DEL-01-03-REQ-04 | The workflow shall preserve `TBD` for unresolved contributor legal mechanism, maintainer authority, reviewer role, release authority, quorum, legal-review outcomes, and legal sufficiency of contribution instruments. | `docs/CONTRACT.md` OPS-K-GOV-1 and OPS-K-GOV-2; `docs/DIRECTIVE.md` section 6; `governance/MAINTAINERS.md` section 6 |
> | DEL-01-03-REQ-05 | The workflow shall separate contribution governance from professional engineering approval and shall not claim certification, sealing, code compliance, legal clearance, or professional reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-AGENT-4; `docs/DIRECTIVE.md` section 6 |
> | DEL-01-03-REQ-06 | The workflow shall record reviewer disposition as a project governance review only, with status values that distinguish pending, accepted, rejected, and quarantined contributions. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 5 |
> | DEL-01-03-REQ-07 | The workflow shall keep public/private data boundaries visible for future adapters, imports, rule packs, reports, and release gates. | AB-00-02, AB-00-06, AB-00-08; `docs/SPEC.md` sections 1, 7, 8 |
> | DEL-01-03-REQ-08 | The workflow shall maintain draft repo-level contributor workflow artifacts while keeping final contributor legal mechanism, legal sufficiency, reviewer authority, and human acceptance as separate governance decisions. | `_CONTEXT.md` Anticipated Artifacts; `CONTRIBUTING.md`; `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`; `governance/MAINTAINERS.md` |
>

### CLM-013 — Standards

> ##### Standards
>
> | Reference | Applicability | Status |
> |---|---|---|
> | `docs/CONTRACT.md` | Binding invariant catalog for IP, governance, authority, and agent behavior. | Accessible |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data boundary and provenance/quarantine rules. | Accessible |
> | `docs/DIRECTIVE.md` | Stop rules and governance baseline. | Accessible |
> | `docs/SPEC.md` | Architecture, provenance, diagnostics, reports, and acceptance semantics. | Accessible |
> | External legal standards or licenses | Legal sufficiency of certification language and any final contributor instrument. | TBD; human/legal review required |
>

### CLM-014 — Verification

> ##### Verification
>
> | Requirement | Verification approach |
> |---|---|
> | DEL-01-03-REQ-01 | Inspect template fields for all provenance and review fields from `docs/IP_AND_DATA_BOUNDARY.md` section 4. |
> | DEL-01-03-REQ-02 | Inspect rejection/quarantine rules for all public-repository exclusions from `docs/IP_AND_DATA_BOUNDARY.md` section 3. |
> | DEL-01-03-REQ-03 | Confirm suspected protected content routes to quarantine and human/legal review with no copied or paraphrased protected content. |
> | DEL-01-03-REQ-04 | Confirm unresolved contributor-governance decisions remain `TBD` while the selected project license remains recorded as `PolyForm-Noncommercial-1.0.0`. |
> | DEL-01-03-REQ-05 | Search the workflow for prohibited reliance claims such as certify, seal, approve, authenticate, legal clearance, or code compliant, except where listed as prohibited terms. |
> | DEL-01-03-REQ-06 | Confirm disposition statuses are present and reviewer authority is framed as governance review only. |
> | DEL-01-03-REQ-07 | Confirm protected-content/provenance warning paths are visible for downstream software gates. |
> | DEL-01-03-REQ-08 | Confirm `CONTRIBUTING.md` and `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` exist as draft repo-level artifacts and are not edited by this current-basis refresh. |
>

### CLM-015 — Documentation

> ##### Documentation
>
> The deliverable-local documentation set consists of:
>
> - `Datasheet.md` - structured fields and workflow states.
> - `Specification.md` - requirements, verification hooks, and exclusions.
> - `Guidance.md` - rationale, principles, trade-offs, assumptions, and human rulings needed.
> - `Procedure.md` - operational steps for intake, review, quarantine, disposition, and records.
> - `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` - semantic lens artifacts used as enrichment aids only.
> - `Dependencies.csv` and `_DEPENDENCIES.md` - local dependency register and index.
> - `CONTRIBUTING.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` - draft repo-level governance artifacts read as evidence by this deliverable.
>

### CLM-016 — Acceptance Criteria

> ##### Acceptance Criteria
>
> - The local kit contains the required four documents and generated semantic/dependency artifacts.
> - Repo-level contributor workflow artifacts exist as drafts, but this current-basis refresh does not modify them.
> - Final contributor legal mechanism, legal sufficiency, reviewer authority, and acceptance require recorded human project authority approval.
> - No protected standards/code data, copied tables, proprietary values, or legal conclusions are introduced.
> - Unknown contributor-governance/legal-review decisions remain `TBD`.
> - Human rulings needed are visible.
>

### CLM-017 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Contributor certification templates and issue-intake surfaces now exist under the repository's PolyForm-Noncommercial-1.0.0 boundary. Current upstream authority is SOFTWARE_DECOMP revision 0.8 with DAG-007 coordination; contributor disposition remains subject to the recorded governance workflow.

- **AC-001** — The workflow preserves the documented contributor fields, protected-content stop and quarantine rules, repository-governance-only disposition boundary, recorded human-gated decisions, and residual TBDs for SOW-028 and SOW-048 without adding legal or professional approval.

## Production and Verification Method — Praxeology

### CLM-018 — Procedure: DEL-01-03 Contributor certification workflow

> #### Procedure: DEL-01-03 Contributor certification workflow
>

### CLM-019 — Purpose

> ##### Purpose
>
> Define a repeatable local procedure for contributor certification intake, provenance review, protected-content screening, quarantine/rejection handling, and disposition records for public data contributions.
>

### CLM-020 — Prerequisites

> ##### Prerequisites
>
> - Assigned deliverable context: DEL-01-03 under PKG-01.
> - Current governing references: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7, and approved `DAG-006`.
> - Draft repo-level workflow surfaces exist at `CONTRIBUTING.md`, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`, and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`.
> - Human-owned governance decisions for exact license, final contributor legal mechanism, maintainer authority, reviewer role, release authority, and legal-review thresholds are `TBD`.
> - No protected standards, proprietary vendor data, or private project/rule-pack content may be copied into the public workflow records.
>

### CLM-021 — Steps

> ##### Steps
>
> 1. Receive contribution package.
>    - Record contributor identity and contribution description.
>    - Assign an intake record ID using the future project convention, or `TBD` if no convention exists.
>
> 2. Collect required provenance fields.
>    - `source_name`
>    - `source_location`
>    - `source_license`
>    - `contributor`
>    - `contributor_certification`
>    - `redistribution_status`
>    - `review_status`
>
> 3. Check completeness.
>    - If source, license, contributor certification, or redistribution status is missing, set review status to `pending` or `rejected` with a `TBD` note.
>    - Do not infer redistribution rights from usefulness or public availability.
>
> 4. Screen protected and private content.
>    - Look for protected standards text, tables, figures, examples, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor catalog data, commercial software examples/templates, private project data, private rule packs, owner standards, or company design bases.
>    - If suspected, stop intake and set `redistribution_status=protected_suspected` or equivalent note.
>
> 5. Quarantine suspected protected/private submissions.
>    - Do not reproduce the suspected content in public notes.
>    - Move or reference the artifact only through `quarantine/protected-content/` or a maintainer-approved equivalent.
>    - Treat quarantine access rule, escalation owner, and final legal disposition as `TBD` until human/legal authority records them.
>    - Record issue metadata, not protected content.
>    - Request human/legal review.
>
> 6. Route review.
>    - Provenance-complete and public-permissive contributions proceed to maintainer review.
>    - Unknown, private-only, or protected-suspected contributions do not enter public examples or libraries.
>    - Legal or license uncertainty remains `TBD` pending human/legal decision.
>
> 7. Record disposition.
>    - Use one of: `pending`, `accepted`, `rejected`, `quarantined`.
>    - Include reviewer, date, evidence reviewed, rationale, and limitations.
>    - State that disposition is repository governance review only and not engineering approval.
>
> 8. Maintain repo-level workflow evidence.
>    - Treat `CONTRIBUTING.md` and `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` as draft governance surfaces until human acceptance.
>    - Edit repo-level contributor artifacts only under an explicitly approved write scope.
>    - Carry the selected project license as `PolyForm-Noncommercial-1.0.0`; keep exact contributor legal mechanism, legal sufficiency, and reviewer/release authority `TBD` until the human project authority records them.
>

### CLM-022 — Verification

> ##### Verification
>
> | Check | Pass condition |
> |---|---|
> | Field completeness | All required provenance and certification fields are present or explicitly `TBD`. |
> | Protected-content stop rule | Suspected protected/private content is not reproduced and is routed to quarantine/human review. |
> | Authority boundary | Records do not claim certification, sealing, legal clearance, code compliance, or professional approval. |
> | Contributor-governance uncertainty | Contributor legal mechanism, legal sufficiency, and maintainer/release authority unresolved items remain `TBD`; selected project license remains recorded as `PolyForm-Noncommercial-1.0.0`. |
> | Local write scope | This current-basis refresh edits only DEL-01-03 local artifacts; repo-level artifacts are read as evidence unless separately approved for editing. |
>

### CLM-023 — Records

> ##### Records
>
> - Contributor certification intake record.
> - Provenance field checklist.
> - Protected/private content screen.
> - Quarantine issue record when applicable.
> - Reviewer disposition record.
> - Human/legal ruling reference when provided.
> - `CONTRIBUTING.md` and contributor certification template draft pointers.
>

### CLM-024 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. Contributor certification templates and issue-intake surfaces now exist under the repository's PolyForm-Noncommercial-1.0.0 boundary. Current upstream authority is SOFTWARE_DECOMP revision 0.8 with DAG-007 coordination; contributor disposition remains subject to the recorded governance workflow.

- **VER-001** — Compare the converted contract source markers and parity report against all four legacy source documents, then inspect the matrix and derived checklist for complete contributor-workflow coverage and preserved authority boundaries.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-01-03 Contributor certification workflow

> #### Guidance: DEL-01-03 Contributor certification workflow
>

### CLM-026 — Purpose

> ##### Purpose
>
> This workflow exists to make public contribution intake auditable before data enters the OpenPipeStress public repository. It supports OBJ-002 by separating open mechanics and permissibly redistributable public data from protected standards-body content, proprietary vendor data, private user/project data, and user-supplied code data.
>

### CLM-027 — Principles

> ##### Principles
>
> 1. Evidence first: accept public data only when source, provenance, license or redistribution status, contributor certification, and review disposition are recorded.
> 2. Stop on protected-content risk: suspected standards text, tables, figures, examples, code-derived formulas, protected dimensional tables, proprietary catalogs, or private project/rule-pack data are blockers, not cleanup tasks.
> 3. Human authority remains external: maintainers can review repository governance acceptance, but this workflow does not create legal clearance or professional engineering approval.
> 4. `TBD` is safer than invention: unresolved contributor legal mechanism, legal sufficiency, maintainer, release, or redistribution questions stay visible until decided by the appropriate human authority. The project license has already been selected as `PolyForm-Noncommercial-1.0.0`.
> 5. Local evidence first: this current-basis refresh reads the existing draft `CONTRIBUTING.md` section and certification template as evidence without editing repo-level policy files.
>

### CLM-028 — Considerations

> ##### Considerations
>
> The workflow should be strict enough to prevent accidental public redistribution of protected or private data, but simple enough for contributors to complete before maintainer review. A missing source or license should not be converted into a reviewer guess. A permissive-looking public web page is not automatically a redistribution grant.
>
> Reviewer notes should distinguish factual evidence from assumptions and proposals. When the contribution includes engineering values, reviewer disposition should focus on public-repository acceptability and provenance, not engineering adequacy for project reliance.
>

### CLM-029 — Trade-offs

> ##### Trade-offs
>
> | Topic | Trade-off | Preferred posture |
> |---|---|---|
> | Contributor friction | More fields slow contribution intake. | Keep fields minimal but mandatory for public data records. |
> | Source-rights uncertainty | Fast acceptance may be tempting when data looks useful. | Preserve `TBD` or reject/quarantine until rights are documented. |
> | Protected-content detection | Automated checks may miss paraphrased or reformatted protected data. | Use automated gates as evidence aids plus human review. |
> | Maintainer authority | Maintainers need a clear merge gate. | Treat maintainer disposition as repository governance only. |
>

### CLM-030 — Examples

> ##### Examples
>

### CLM-031 — Certification template text, draft

> ###### Certification template text, draft
>
> ```text
> I certify for repository-governance review that this contribution is my original work or is submitted with documented redistribution rights; that I have identified all sources, licenses, and provenance known to me; and that I have not copied protected standards text, tables, figures, examples, proprietary catalog data, private project data, or private rule-pack content into this public contribution except where explicit redistribution rights are documented for maintainer review.
> ```
>
> This text is a draft workflow artifact, not legal advice and not a final contributor license agreement.
>

### CLM-032 — Review disposition note, draft

> ###### Review disposition note, draft
>
> ```text
> Disposition: TBD / accepted / rejected / quarantined
> Basis: FACT / ASSUMPTION / PROPOSAL
> Evidence reviewed: source_name, source_location, source_license, redistribution_status, protected_content_screen
> Limits: Repository governance review only; no engineering approval or legal conclusion.
> ```
>

### CLM-033 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | C-001 | Project license is selected as `PolyForm-Noncommercial-1.0.0`, but final contributor legal mechanism and legal sufficiency are not selected. | `docs/CONTRACT.md` OPS-K-GOV-1 | `docs/DIRECTIVE.md` section 6; `governance/MAINTAINERS.md` section 6 | Specification Standards; Procedure Records | Carry selected project license as fact; defer contributor legal mechanism and legal sufficiency until human/legal review. | Human ruling 2026-06-04: defer until external contribution intake or public release readiness becomes relevant. |
> | C-002 | Legal review threshold and reviewer role for accepting public component/material data remain unresolved. | `governance/MAINTAINERS.md` section 6 | `docs/IP_AND_DATA_BOUNDARY.md` sections 4-5 | Procedure protected-content review and disposition | Treat suspected or uncertain rights as quarantine/reject until human/legal review. | TBD |
>

### CLM-034 — Human Rulings Needed

> ##### Human Rulings Needed
>
> - Whether a separate contributor license agreement, developer certificate of origin, or project-specific certification text will be used, and whether the final contributor instrument is legally sufficient.
> - Maintainer/reviewer authority model, quorum, and release-policy linkage.
> - Quarantine storage location and access rules for suspected protected/private submissions.
> - Whether automated protected-content/provenance lint gates are required before every contribution review or only before release.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-028 SOW-048 OBJ-002 | CLM-010 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
