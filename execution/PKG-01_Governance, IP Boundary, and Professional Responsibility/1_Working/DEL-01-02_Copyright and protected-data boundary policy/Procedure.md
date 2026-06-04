# Procedure: DEL-01-02 Copyright and protected-data boundary policy

## Purpose

This procedure describes how to maintain and review the repo-level protected-data boundary policy and contribution checklist without introducing protected content or legal conclusions.

## Prerequisites

- Sealed deliverable context for DEL-01-02.
- Local sources listed in `_REFERENCES.md`.
- Applicable invariants from `docs/CONTRACT.md`.
- Human/legal availability for unresolved legal wording, license, contributor certification, and quarantine-path decisions.
- Checklist path: `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`.
- Human project authority assignment of reviewer role and final governance/legal mechanism: `TBD`.
- No protected standards/code/vendor data in working examples or drafts.

## Steps

1. Confirm scope is limited to DEL-01-02, SOW-003, SOW-028, and OBJ-002.
2. Read governing local sources: AGENTS.md, docs/DIRECTIVE.md, docs/CONTRACT.md, docs/TYPES.md, docs/SPEC.md, docs/IP_AND_DATA_BOUNDARY.md, governance/CONTRIBUTION_REVIEW_CHECKLIST.md, docs/AGENTIC_DEVELOPMENT_WORKFLOW.md, decomposition, and registers.
3. Draft or review the policy sections for allowed public content, prohibited public content, private/user-controlled content, required provenance, quarantine, reports, and contributor review.
4. Draft or review the contribution checklist fields:
   - contribution description;
   - source name and source location;
   - source license or redistribution basis;
   - contributor identity and certification;
   - redistribution status;
   - protected-content risk;
   - private-data risk;
   - reviewer disposition;
   - quarantine/escalation record, if applicable.
5. If suspected protected content appears, stop ingestion, avoid reproducing it, mark the item `PROTECTED_CONTENT_SUSPECTED`, quarantine outside public examples, record the issue, and request human/legal review.
6. Replace unknowns with `TBD` rather than inventing license status, legal conclusions, provenance, examples, formulas, or engineering values.
7. Verify the policy avoids claims of legal advice, code compliance, professional engineering approval, certification, sealing, endorsement, or release fitness.
8. Perform a field-by-field checklist acceptance review against Specification R2 and R11, treating reviewer role and final governance/legal mechanism as `TBD` until assigned by the human project authority.
9. Route the repo-level policy/checklist for REVIEW and then human acceptance before treating it as project policy.

## Verification

- The four-document kit stays deliverable-local.
- Repo-level artifacts remain draft governance surfaces unless accepted by a human gate.
- Prohibited-content categories align with OPS-K-IP-1 and SOW-003.
- Provenance and contributor-review fields align with OPS-K-IP-2 and OPS-K-GOV-4.
- Quarantine language aligns with OPS-K-IP-3 and remains non-legal-conclusive.
- Unknown values and unresolved policy decisions remain `TBD`.
- The dependency register is schema-valid after extraction.

## Records

- `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
