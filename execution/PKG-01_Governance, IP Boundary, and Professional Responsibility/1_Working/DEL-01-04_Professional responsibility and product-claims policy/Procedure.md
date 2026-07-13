# Procedure: DEL-01-04 Professional responsibility and product-claims policy

## Purpose

Define the local procedure for producing and checking professional-boundary policy language and report notice text for OpenPipeStress.

## Prerequisites

- Sealed deliverable context for DEL-01-04.
- Applicable sources: docs/DIRECTIVE.md, docs/CONTRACT.md, docs/TYPES.md, docs/SPEC.md, docs/IP_AND_DATA_BOUNDARY.md, execution/_Decomposition/SOFTWARE_DECOMP.md revision 0.7, approved DAG-006 context, and register rows for DEL-01-04, SOW-034, and SOW-064.
- No protected standards/code text or proprietary engineering data in the drafting input.
- Human/legal/professional rulings available for jurisdiction-specific language: `TBD`.

## Steps

1. Confirm the target artifact is policy/report-notice language, not solver, GUI, schema, or report-generator implementation.
2. Extract the required professional boundary from SOW-034 and OPS-K-AUTH-1.
3. Extract related report, governance, and agent boundaries from OPS-K-REPORT-1, OPS-K-REPORT-2, OPS-K-GOV-3, OPS-K-AGENT-1 through OPS-K-AGENT-4, and OPS-K-MECH-2.
4. Extract the SOW-064 design-engine/product-scope boundary and keep it subordinate to the non-authoritative professional reliance boundary.
5. Draft permitted-claim language that describes only supported behavior: mechanics computation, user-rule evaluation, warnings, provenance, assumptions, limitations, design-authoring support, and human-review support.
6. Draft prohibited-claim language that excludes certification, sealing, approval, authentication, official endorsement, and compliance-for-reliance claims by software or agents.
7. Review `docs/report_notice_template.md` for notice language that states code-specific data is user-supplied and professional reliance requires competent human review.
8. Mark legal conclusions, jurisdiction-specific obligations, unresolved acceptance workflow details, and final acceptance of notice wording as `TBD`.
9. Check drafts against protected-content boundaries before publication or release use.
10. Route any repo-level policy/report-notice revision or issuance decision to human review before any `ISSUED` lifecycle state or project-policy acceptance claim.
11. If human acceptance records are introduced, bind each record to the relevant model, rule-pack, and report hashes and require re-review after content changes.

## Verification

| Check | Action |
|---|---|
| Scope check | Confirm edits are limited to the authorized deliverable or later authorized repo-level target. |
| Prohibited-claim check | Confirm no affirmative certification, sealing, approval, authentication, endorsement, or compliance-for-reliance claim remains. |
| Status check | Confirm mechanics, user-rule, missing-data, and human-approval states remain distinct. |
| Provenance check | Confirm report notice language requires source/provenance disclosure where engineering reliance may be affected. |
| Publication-readiness screen | Confirm exact notice wording, protected-content checks, and human acceptance status before repo-level publication. |
| Human-gate check | Confirm draft/proposal status remains visible until human acceptance. |

## Records

- Deliverable-local four-document kit.
- `_SEMANTIC.md` semantic matrix output.
- `_SEMANTIC_LENSING.md` enrichment register.
- `Dependencies.csv` and `_DEPENDENCIES.md` dependency extraction outputs.
- `_run_records/TASK_RUN_*.md` evidence record.
- `_run_records/WORKING_ITEMS_RUN_*.md` current-basis refresh record.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The professional-responsibility and product-claims policy is current under SOFTWARE_DECOMP revision 0.8 and DAG-007 coordination. It preserves the distinction between computational evidence, professional judgment, validation, and approval; no engineering ruling is created here.
