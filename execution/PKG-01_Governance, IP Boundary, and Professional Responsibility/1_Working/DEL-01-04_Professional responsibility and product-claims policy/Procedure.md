# Procedure: DEL-01-04 Professional responsibility and product-claims policy

## Purpose

Define the local procedure for producing and checking professional-boundary policy language and report notice text for OpenPipeStress.

## Prerequisites

- Sealed deliverable context for DEL-01-04.
- Applicable sources: INIT.md, docs/DIRECTIVE.md, docs/CONTRACT.md, docs/TYPES.md, docs/SPEC.md, docs/AGENTIC_DEVELOPMENT_WORKFLOW.md, decomposition/register rows for DEL-01-04 and SOW-034.
- No protected standards/code text or proprietary engineering data in the drafting input.
- Human/legal/professional rulings available for jurisdiction-specific language: `TBD`.

## Steps

1. Confirm the target artifact is policy/report-notice language, not solver, GUI, schema, or report-generator implementation.
2. Extract the required professional boundary from SOW-034 and OPS-K-AUTH-1.
3. Extract related report, governance, and agent boundaries from OPS-K-REPORT-1, OPS-K-REPORT-2, OPS-K-GOV-3, OPS-K-AGENT-1 through OPS-K-AGENT-4, and OPS-K-MECH-2.
4. Draft permitted-claim language that describes only supported behavior: mechanics computation, user-rule evaluation, warnings, provenance, assumptions, limitations, and human-review support.
5. Draft prohibited-claim language that excludes certification, sealing, approval, authentication, official endorsement, and compliance-for-reliance claims by software or agents.
6. Draft report notice language that states code-specific data is user-supplied and professional reliance requires competent human review.
7. Mark legal conclusions, jurisdiction-specific obligations, unresolved acceptance workflow details, and unspecified notice wording as `TBD`.
8. Check the draft against protected-content boundaries before any public repo-level artifact is produced.
9. Route the resulting repo-level policy/report-notice artifact to human review before any `ISSUED` lifecycle state or project-policy claim.
10. If human acceptance records are introduced, bind each record to the relevant model, rule-pack, and report hashes and require re-review after content changes.

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
