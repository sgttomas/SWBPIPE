---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-02
package_id: PKG-01
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-003, SOW-028]
package_objective_refs: [OBJ-002]
---

# Scope of Work — DEL-01-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-01-02` in service of project scope [SOW-003, SOW-028] and package objectives [OBJ-002].

- **OUT-001** — A copyright and protected-data boundary policy and contribution-review checklist requirements defining allowed public content, prohibited public content, private or user-controlled content, provenance, quarantine, and human/legal escalation.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-01-02 Copyright and protected-data boundary policy

> #### Datasheet: DEL-01-02 Copyright and protected-data boundary policy
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-01-02-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-01-02 |
> | Deliverable name | Copyright and protected-data boundary policy |
> | Package ID | PKG-01 |
> | Package name | Governance, IP Boundary, and Professional Responsibility |
> | Type | DOC_UPDATE |
> | Scope items | SOW-003, SOW-028 |
> | Objective | OBJ-002 |
> | Anticipated repo-level artifacts | docs/IP_AND_DATA_BOUNDARY.md; contribution review checklist |
> | Repo-level artifact status | `docs/IP_AND_DATA_BOUNDARY.md` and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` exist as draft governance artifacts |
> | Local artifact role | Deliverable-local evidence and review kit for the protected-data boundary policy |
> | Lifecycle authority | Draft/proposal until accepted by human gate |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary boundary | Public repository must not redistribute protected standards/code/vendor data | execution/_Decomposition/SOFTWARE_DECOMP.md §Scope of Work; docs/CONTRACT.md §1 |
> | Contributor governance need | Repository must include IP controls and review procedures | execution/_Decomposition/SOFTWARE_DECOMP.md §Scope of Work; docs/CONTRACT.md §1 |
> | Quarantine posture | Stop ingestion, mark suspected protected content, quarantine outside public examples, record issue, request human/legal review | docs/IP_AND_DATA_BOUNDARY.md §5 |
> | Public data prerequisites | Source, provenance, license or redistribution status, contributor certification, and review disposition | docs/CONTRACT.md §1; docs/IP_AND_DATA_BOUNDARY.md §4 |
> | Legal conclusion authority | Human/legal review required; this deliverable is not a legal opinion | docs/_Registers/ContextBudgetQA.csv row DEL-01-02; docs/README.md §Status |
> | Public examples posture | Invented, public-domain, or permissively licensed examples only | docs/DIRECTIVE.md §3; docs/IP_AND_DATA_BOUNDARY.md §2 |
>

### CLM-005 — Conditions

> ##### Conditions
>
> - The setup workflow may discuss `docs/IP_AND_DATA_BOUNDARY.md` and the contribution checklist only within this deliverable-local kit.
> - Protected standards text, tables, figures, examples, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data, copied commercial examples, private rule packs, owner standards, and private project data are excluded from public repository content.
> - Unknown license, provenance, redistribution status, certification mechanism, or review disposition is `TBD` until resolved by human project authority or legal review.
> - Software and agent outputs remain draft/proposal artifacts until accepted by a human gate.
>

### CLM-006 — Construction

> ##### Construction
>
> The intended repo-level policy artifact should be constructed from:
>
> 1. Boundary categories: allowed public content, prohibited public content, private/user-controlled content, and suspected protected content.
> 2. Required contribution metadata: source, location, license or redistribution basis, contributor, certification, redistribution status, review status, and disposition.
> 3. Review actions: intake screening, provenance check, redistribution-rights check, protected-content scan, quarantine decision, human/legal escalation, and final accept/reject/quarantine outcome.
> 4. Non-claims: no legal opinion, no code-compliance claim, no professional engineering approval, no certification/seal/endorsement claim.
>

### CLM-007 — References

> ##### References
>
> - AGENTS.md — OpenPipeStress agent index and Type 2 dispatch rule.
> - docs/DIRECTIVE.md — founding intent, public/private boundary, stop rules.
> - docs/CONTRACT.md — invariant catalog.
> - docs/TYPES.md — canonical vocabulary and provenance labels.
> - docs/SPEC.md — technical implementation context, reports, rule packs, and warning classes.
> - docs/IP_AND_DATA_BOUNDARY.md — draft repo-level policy artifact.
> - governance/CONTRIBUTION_REVIEW_CHECKLIST.md — draft repo-level contribution review checklist.
> - docs/AGENTIC_DEVELOPMENT_WORKFLOW.md — Type 2 execution and review expectations.
> - execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7 — DEL-01-02, SOW-003, SOW-028, OBJ-002, AB-00-01, AB-00-02, AB-00-06, AB-00-08.
> - execution/_DAG/DAG-006/ — approved active graph authority.
> - docs/_Registers/Deliverables.csv, ScopeLedger.csv, ContextBudgetQA.csv — register evidence.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-01-02 Copyright and protected-data boundary policy

> #### Specification: DEL-01-02 Copyright and protected-data boundary policy
>

### CLM-009 — Scope

> ##### Scope
>
> This deliverable-local specification defines requirements for the repo-level copyright and protected-data boundary policy and contribution review checklist. It covers public-repository data boundaries, contributor provenance controls, suspected protected-content quarantine, and review evidence. It does not implement product code, decide the project license, provide legal advice, certify compliance, or approve engineering use.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | ReqID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-01-02-R1 | The policy shall state that the public repository must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. | docs/CONTRACT.md §1 OPS-K-IP-1; execution/_Decomposition/SOFTWARE_DECOMP.md SOW-003 | Review policy prohibited-content section and protected-content checklist entries. |
> | DEL-01-02-R2 | The policy shall require public data contributions to include source, provenance, license or redistribution status, contributor certification, and review disposition. | docs/CONTRACT.md §1 OPS-K-IP-2; docs/IP_AND_DATA_BOUNDARY.md §4 | Confirm contribution checklist has each metadata field. |
> | DEL-01-02-R3 | The policy shall define suspected protected-content handling as stop, quarantine, issue record, and human/legal escalation. | docs/CONTRACT.md §1 OPS-K-IP-3; docs/IP_AND_DATA_BOUNDARY.md §5 | Confirm quarantine path/process and escalation fields are present; exact path is `TBD`. |
> | DEL-01-02-R4 | The policy shall distinguish open mechanics from user-supplied code-specific and proprietary data. | INIT.md; docs/DIRECTIVE.md §1, §3; docs/CONTRACT.md OPS-K-DATA-1 | Review allowed/prohibited/private categories. |
> | DEL-01-02-R5 | The policy shall state that missing, unknown, or undocumented provenance/redistribution status blocks public acceptance until resolved. | docs/CONTRACT.md OPS-K-DATA-2, OPS-K-IP-2; docs/TYPES.md §5, §7 | Checklist includes `TBD`, `UNKNOWN_SOURCE`, and rejection/quarantine outcomes. |
> | DEL-01-02-R6 | The policy shall require public rule-pack examples and public report templates/examples to avoid protected standards content and use invented or permissively sourced data. | docs/CONTRACT.md OPS-K-RULE-1, OPS-K-REPORT-2; docs/SPEC.md §6, §8 | Review public-example and report-template review rows. |
> | DEL-01-02-R7 | The policy shall make contributor review a maintainer gate for IP, provenance, privacy, and protected-content risks before accepting public contributions. | docs/CONTRACT.md OPS-K-GOV-4; docs/DIRECTIVE.md §6 | Checklist includes review disposition and responsible reviewer field. |
> | DEL-01-02-R8 | The policy shall not claim legal advice, professional engineering approval, certification, sealing, code compliance, or standards-body endorsement. | docs/CONTRACT.md OPS-K-AUTH-1, OPS-K-AGENT-4; docs/DIRECTIVE.md §4.2, §6 | Review notices and forbidden-claim checks. |
> | DEL-01-02-R9 | The policy shall preserve stable IDs and traceability to DEL-01-02, PKG-01, SOW-003, SOW-028, and OBJ-002 in the deliverable-local kit. | docs/CONTRACT.md OPS-K-HIER-1, OPS-K-ID-1; docs/TYPES.md §1-2 | Confirm IDs appear consistently in this kit. |
> | DEL-01-02-R10 | Architecture-facing references to diagnostics, result envelopes, tests, and gates shall remain constraints for downstream implementation only unless this deliverable explicitly resolves them. | execution/_Decomposition/SOFTWARE_DECOMP.md AB-00-06, AB-00-08; sealed brief | Confirm policy text uses non-implementation wording and records downstream handoff. |
> | DEL-01-02-R11 | The repo-level checklist location is `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`; reviewer role and final legal/governance mechanism shall remain `TBD` until assigned by the human project authority. | docs/IP_AND_DATA_BOUNDARY.md §8; governance/CONTRIBUTION_REVIEW_CHECKLIST.md; docs/CONTRACT.md OPS-K-GOV-2, OPS-K-GOV-4 | Confirm the checklist path exists while reviewer and final legal mechanism remain unresolved. |
>

### CLM-011 — Standards

> ##### Standards
>
> No protected standards text or clause-level standard requirements are used as source material. Standards-body content is treated as excluded public-repository content unless explicit redistribution rights are documented and human/legal review accepts it. Specific legal requirements, licenses, and quarantine storage paths are `TBD`.
>

### CLM-012 — Verification

> ##### Verification
>
> - Four-document kit exists in the deliverable folder.
> - Default sections are present in Datasheet, Specification, Guidance, and Procedure.
> - Requirements cite local project sources rather than protected standard text.
> - Unknowns are marked `TBD`.
> - No product implementation, protected standards/code data, legal conclusion, certification, sealing, approval, or compliance-for-reliance claim is introduced.
> - The contribution checklist verification remains field-by-field; path existence is resolved at `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`, while reviewer role and final legal mechanism remain `TBD`.
> - Dependency register validates against v3.1 schema after extraction.
>

### CLM-013 — Documentation

> ##### Documentation
>
> Repo-level artifacts are `docs/IP_AND_DATA_BOUNDARY.md` and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`. This deliverable-local kit records their current-basis requirements and unresolved governance decisions.

- **AC-001** — The policy and contribution-review checklist preserve the documented prohibited-content boundary, provenance and contributor-certification fields, quarantine and human/legal escalation path, unresolved TBD decisions, and non-claim limitations for SOW-003 and SOW-028.

## Production and Verification Method — Praxeology

### CLM-014 — Procedure: DEL-01-02 Copyright and protected-data boundary policy

> #### Procedure: DEL-01-02 Copyright and protected-data boundary policy
>

### CLM-015 — Purpose

> ##### Purpose
>
> This procedure describes how to maintain and review the repo-level protected-data boundary policy and contribution checklist without introducing protected content or legal conclusions.
>

### CLM-016 — Prerequisites

> ##### Prerequisites
>
> - Sealed deliverable context for DEL-01-02.
> - Local sources listed in `_REFERENCES.md`.
> - Applicable invariants from `docs/CONTRACT.md`.
> - Human/legal availability for unresolved legal wording, license, contributor certification, and quarantine-path decisions.
> - Checklist path: `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`.
> - Human project authority assignment of reviewer role and final governance/legal mechanism: `TBD`.
> - No protected standards/code/vendor data in working examples or drafts.
>

### CLM-017 — Steps

> ##### Steps
>
> 1. Confirm scope is limited to DEL-01-02, SOW-003, SOW-028, and OBJ-002.
> 2. Read governing local sources: AGENTS.md, docs/DIRECTIVE.md, docs/CONTRACT.md, docs/TYPES.md, docs/SPEC.md, docs/IP_AND_DATA_BOUNDARY.md, governance/CONTRIBUTION_REVIEW_CHECKLIST.md, docs/AGENTIC_DEVELOPMENT_WORKFLOW.md, decomposition, and registers.
> 3. Draft or review the policy sections for allowed public content, prohibited public content, private/user-controlled content, required provenance, quarantine, reports, and contributor review.
> 4. Draft or review the contribution checklist fields:
>    - contribution description;
>    - source name and source location;
>    - source license or redistribution basis;
>    - contributor identity and certification;
>    - redistribution status;
>    - protected-content risk;
>    - private-data risk;
>    - reviewer disposition;
>    - quarantine/escalation record, if applicable.
> 5. If suspected protected content appears, stop ingestion, avoid reproducing it, mark the item `PROTECTED_CONTENT_SUSPECTED`, quarantine outside public examples, record the issue, and request human/legal review.
> 6. Replace unknowns with `TBD` rather than inventing license status, legal conclusions, provenance, examples, formulas, or engineering values.
> 7. Verify the policy avoids claims of legal advice, code compliance, professional engineering approval, certification, sealing, endorsement, or release fitness.
> 8. Perform a field-by-field checklist acceptance review against Specification R2 and R11, treating reviewer role and final governance/legal mechanism as `TBD` until assigned by the human project authority.
> 9. Route the repo-level policy/checklist for REVIEW and then human acceptance before treating it as project policy.
>

### CLM-018 — Verification

> ##### Verification
>
> - The four-document kit stays deliverable-local.
> - Repo-level artifacts remain draft governance surfaces unless accepted by a human gate.
> - Prohibited-content categories align with OPS-K-IP-1 and SOW-003.
> - Provenance and contributor-review fields align with OPS-K-IP-2 and OPS-K-GOV-4.
> - Quarantine language aligns with OPS-K-IP-3 and remains non-legal-conclusive.
> - Unknown values and unresolved policy decisions remain `TBD`.
> - The dependency register is schema-valid after extraction.
>

### CLM-019 — Records

> ##### Records
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/TASK_RUN_*.md`

- **VER-001** — Compare the converted contract source markers and parity report against all four legacy source documents, and review the output/evaluation matrix and derived checklist for complete source-grounded coverage.

## Governing Values and Decisions — Axiology

### CLM-020 — Guidance: DEL-01-02 Copyright and protected-data boundary policy

> #### Guidance: DEL-01-02 Copyright and protected-data boundary policy
>

### CLM-021 — Purpose

> ##### Purpose
>
> The policy exists to protect standards-body and vendor intellectual property while allowing OpenPipeStress to remain a public, educational, auditable implementation of open mechanics. It should give maintainers and contributors a conservative intake path: accept only content with documented rights, keep private/project data private, and escalate suspected protected content before it reaches the public repository.
>

### CLM-022 — Principles

> ##### Principles
>
> - Open mechanics are public; code-specific values, proprietary libraries, protected standards tables, and owner/project data are not public defaults.
> - Provenance is part of the data, not an optional note.
> - `TBD`, `UNKNOWN_SOURCE`, and `PROTECTED_CONTENT_SUSPECTED` are valid findings; they should not be silently converted into public content.
> - Contributor review is a gate, not a courtesy label.
> - Quarantine is protective and procedural; it is not a finding of infringement or a legal conclusion.
> - Public examples should be original, invented, public-domain, or permissively licensed, with a clear non-engineering notice.
>

### CLM-023 — Considerations

> ##### Considerations
>
> The existing draft `docs/IP_AND_DATA_BOUNDARY.md` provides categories for allowed public content, prohibited public content, provenance fields, quarantine behavior, private user data, reports, and the contribution review checklist. The draft `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` provides the current checklist surface. Later governance work may still need human decisions for final legal mechanism, license status vocabulary, reviewer roles, and escalation owner.
>
> Architecture-basis items AB-00-01, AB-00-02, AB-00-06, and AB-00-08 matter as downstream constraints: protected-content and provenance gates should be testable, diagnostics should include `IP_BOUNDARY_WARNING` where implementation later touches contribution/report flows, and adapters/plugins should not bypass provenance or data-boundary controls. This document does not implement those controls.
>

### CLM-024 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Usability vs. protected-content risk | Prefer conservative rejection or quarantine when redistribution status is unclear. |
> | Educational examples vs. accidental copying | Use invented examples and avoid code-derived values, table shapes, copied wording, and proprietary benchmark files. |
> | Automation vs. legal judgment | Automated scans can flag risk but cannot be the sole legal control. Human/legal review remains required for suspicious or high-impact contributions. |
> | Public transparency vs. private data protection | Record policy and review outcomes publicly where safe, but do not expose private rule packs, owner standards, project models, or quarantined content. |
>

### CLM-025 — Examples

> ##### Examples
>
> - Acceptable pattern: a small invented example model with original values, a non-engineering notice, and provenance marked `PUBLIC_DOMAIN_OR_ORIGINAL`.
> - Review-required pattern: manufacturer data with unclear redistribution terms; record `source_license=TBD` and block public acceptance until resolved.
> - Quarantine pattern: a contribution that appears copied from a standards table; stop ingestion, mark `PROTECTED_CONTENT_SUSPECTED`, move outside public examples, record issue, and request human/legal review.
>

### CLM-026 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
> |---|---|---|---|---|---|---|
> | C-001 | Exact quarantine storage path is now specified as `quarantine/protected-content/` or maintainer-approved equivalent, but no maintainer-approved alternative path or owner is assigned. | docs/IP_AND_DATA_BOUNDARY.md §5 | governance/CONTRIBUTION_REVIEW_CHECKLIST.md §8 | Procedure §Steps; Specification R3 | Treat `quarantine/protected-content/` as the default unless a maintainer-approved equivalent is recorded. | TBD |
> | C-002 | Contributor certification mechanism is required by policy intent but exact wording/mechanism remains unresolved. | docs/CONTRACT.md OPS-K-IP-2 | docs/DIRECTIVE.md §6 | Specification R2; Procedure checklist | Human/legal review should approve final attestation language. | TBD |
> | C-003 | The contribution checklist path exists, but reviewer role and final governance/legal mechanism remain unresolved. | docs/IP_AND_DATA_BOUNDARY.md §8; governance/CONTRIBUTION_REVIEW_CHECKLIST.md | docs/CONTRACT.md OPS-K-GOV-2, OPS-K-GOV-4 | Specification R11; Procedure §Prerequisites | Human project authority should assign reviewer role and final governance/legal mechanism. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-003 SOW-028 OBJ-002 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
