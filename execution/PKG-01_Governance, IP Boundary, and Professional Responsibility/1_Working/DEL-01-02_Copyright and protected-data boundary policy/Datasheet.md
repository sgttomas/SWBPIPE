# Datasheet: DEL-01-02 Copyright and protected-data boundary policy

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-01-02-DECL-002`.

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-01-02 |
| Deliverable name | Copyright and protected-data boundary policy |
| Package ID | PKG-01 |
| Package name | Governance, IP Boundary, and Professional Responsibility |
| Type | DOC_UPDATE |
| Scope items | SOW-003, SOW-028 |
| Objective | OBJ-002 |
| Anticipated repo-level artifacts | docs/IP_AND_DATA_BOUNDARY.md; contribution review checklist |
| Repo-level artifact status | `docs/IP_AND_DATA_BOUNDARY.md` and `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` exist as draft governance artifacts |
| Local artifact role | Deliverable-local evidence and review kit for the protected-data boundary policy |
| Lifecycle authority | Draft/proposal until accepted by human gate |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Primary boundary | Public repository must not redistribute protected standards/code/vendor data | execution/_Decomposition/SOFTWARE_DECOMP.md §Scope of Work; docs/CONTRACT.md §1 |
| Contributor governance need | Repository must include IP controls and review procedures | execution/_Decomposition/SOFTWARE_DECOMP.md §Scope of Work; docs/CONTRACT.md §1 |
| Quarantine posture | Stop ingestion, mark suspected protected content, quarantine outside public examples, record issue, request human/legal review | docs/IP_AND_DATA_BOUNDARY.md §5 |
| Public data prerequisites | Source, provenance, license or redistribution status, contributor certification, and review disposition | docs/CONTRACT.md §1; docs/IP_AND_DATA_BOUNDARY.md §4 |
| Legal conclusion authority | Human/legal review required; this deliverable is not a legal opinion | docs/_Registers/ContextBudgetQA.csv row DEL-01-02; docs/README.md §Status |
| Public examples posture | Invented, public-domain, or permissively licensed examples only | docs/DIRECTIVE.md §3; docs/IP_AND_DATA_BOUNDARY.md §2 |

## Conditions

- The setup workflow may discuss `docs/IP_AND_DATA_BOUNDARY.md` and the contribution checklist only within this deliverable-local kit.
- Protected standards text, tables, figures, examples, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data, copied commercial examples, private rule packs, owner standards, and private project data are excluded from public repository content.
- Unknown license, provenance, redistribution status, certification mechanism, or review disposition is `TBD` until resolved by human project authority or legal review.
- Software and agent outputs remain draft/proposal artifacts until accepted by a human gate.

## Construction

The intended repo-level policy artifact should be constructed from:

1. Boundary categories: allowed public content, prohibited public content, private/user-controlled content, and suspected protected content.
2. Required contribution metadata: source, location, license or redistribution basis, contributor, certification, redistribution status, review status, and disposition.
3. Review actions: intake screening, provenance check, redistribution-rights check, protected-content scan, quarantine decision, human/legal escalation, and final accept/reject/quarantine outcome.
4. Non-claims: no legal opinion, no code-compliance claim, no professional engineering approval, no certification/seal/endorsement claim.

## References

- AGENTS.md — OpenPipeStress agent index and Type 2 dispatch rule.
- docs/DIRECTIVE.md — founding intent, public/private boundary, stop rules.
- docs/CONTRACT.md — invariant catalog.
- docs/TYPES.md — canonical vocabulary and provenance labels.
- docs/SPEC.md — technical implementation context, reports, rule packs, and warning classes.
- docs/IP_AND_DATA_BOUNDARY.md — draft repo-level policy artifact.
- governance/CONTRIBUTION_REVIEW_CHECKLIST.md — draft repo-level contribution review checklist.
- docs/AGENTIC_DEVELOPMENT_WORKFLOW.md — Type 2 execution and review expectations.
- execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7 — DEL-01-02, SOW-003, SOW-028, OBJ-002, AB-00-01, AB-00-02, AB-00-06, AB-00-08.
- execution/_DAG/DAG-006/ — approved active graph authority.
- docs/_Registers/Deliverables.csv, ScopeLedger.csv, ContextBudgetQA.csv — register evidence.
