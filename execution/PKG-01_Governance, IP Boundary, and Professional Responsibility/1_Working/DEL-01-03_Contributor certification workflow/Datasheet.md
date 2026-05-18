# Datasheet: DEL-01-03 Contributor certification workflow

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-01-03 |
| Name | Contributor certification workflow |
| Package ID | PKG-01 |
| Package Name | Governance, IP Boundary, and Professional Responsibility |
| Type | DOC_UPDATE |
| Objective | OBJ-002 |
| Scope Items | SOW-028, SOW-048 |
| Source basis | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md`; `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/DIRECTIVE.md`; `docs/SPEC.md`; register rows for DEL-01-03, SOW-028, SOW-048 |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Workflow purpose | Define contributor attestations, provenance fields, review routing, and rejection rules for public data contributions. | `_CONTEXT.md` Description |
| Anticipated repo artifacts | `CONTRIBUTING.md` section; contributor certification template. | `_CONTEXT.md` Anticipated Artifacts |
| Local artifact boundary | This deliverable drafts the workflow kit only; repo-level artifacts are not edited in this setup run. | Sealed brief |
| License decision | TBD. The project intends to be free/open-source, but exact license remains a human project authority decision. | `docs/CONTRACT.md` OPS-K-GOV-1; `docs/DIRECTIVE.md` section 6 |
| Maintainer/release authority | TBD until recorded in public governance artifacts. | `docs/CONTRACT.md` OPS-K-GOV-2 |
| Public contribution review gate | Source, provenance, redistribution rights, protected-content risk, private-data risk, and test evidence must be checked before merge. | `docs/DIRECTIVE.md` section 6; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 5 |

## Conditions

| Condition | Required handling | Source |
|---|---|---|
| Suspected protected standards or proprietary content | Stop ingestion, mark as suspected protected content, quarantine outside public examples, record the issue, and request human/legal review. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; `docs/CONTRACT.md` OPS-K-IP-3 |
| Missing source, license, or redistribution status | Treat as unresolved; do not accept as public data until reviewed. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/CONTRACT.md` OPS-K-IP-2 |
| Legal or license conclusion needed | Record `TBD` and route to the human project authority/legal review. | `docs/CONTRACT.md` OPS-K-GOV-1; SOFTWARE_DECOMP OI-001, OI-003 |
| Certification wording | Must not claim engineering certification, approval, sealing, legal clearance, or compliance for reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-AGENT-4 |

## Construction

### Contributor Certification Template Fields

| Field | Required value or status | Notes |
|---|---|---|
| contributor_name | Required | Person or organization submitting the data. |
| contribution_description | Required | Short description of submitted data or artifact. |
| source_name | Required for public data records | Matches `docs/IP_AND_DATA_BOUNDARY.md` required provenance fields. |
| source_location | Required or `TBD` | URL, path, document identifier, or `TBD`. |
| source_license | Required or `TBD` | Redistribution basis or unresolved. |
| redistribution_status | Required | `public_permissive`, `private_only`, `unknown`, or `protected_suspected`. |
| contributor_certification | Required | Statement that the contribution is original, permissively redistributable, or otherwise submitted with documented rights; no protected standards/proprietary content is copied unless rights are documented. |
| protected_content_screen | Required | Records whether protected standards text/tables/figures/examples, copied formulas, protected dimensional tables, or proprietary catalog data appear suspected. |
| private_data_screen | Required | Records whether user-private project, owner, rule-pack, material, or component data is present. |
| review_status | Required | `pending`, `accepted`, `rejected`, or `quarantined`. |
| reviewer | Required at disposition | Maintainer/reviewer identity; authority model remains `TBD`. |
| disposition_notes | Required at disposition | Short rationale and links to evidence, issue, or quarantine record. |

### Review Routing States

| State | Meaning |
|---|---|
| Intake pending | Contribution has not yet passed completeness/provenance screening. |
| Provenance review | Source, license, redistribution status, and contributor attestation are being checked. |
| Protected-content review | Contribution is blocked pending protected-content/private-data review. |
| Quarantined | Suspected protected or private content has been isolated outside public examples. |
| Accepted for public repo | Reviewer recorded public redistribution basis and no protected/private-data blocker. |
| Rejected | Contribution cannot be accepted under current evidence. |

## References

- `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5.
- `docs/CONTRACT.md` invariant index.
- `docs/DIRECTIVE.md` sections 5-6.
- `docs/SPEC.md` sections 6, 8, 10-11.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 rows DEL-01-03, SOW-028, SOW-048, OBJ-002, AB-00-01, AB-00-02, AB-00-06, AB-00-08.
- `docs/_Registers/Deliverables.csv`, `ScopeLedger.csv`, `ContextBudgetQA.csv` rows named in `_CONTEXT.md`.
