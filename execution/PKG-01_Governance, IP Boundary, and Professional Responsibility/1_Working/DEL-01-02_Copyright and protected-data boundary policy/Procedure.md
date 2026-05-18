# Procedure: DEL-01-02 Copyright and protected-data boundary policy

## Purpose

This procedure describes how to produce and review the future repo-level protected-data boundary policy and contribution checklist without introducing protected content or legal conclusions.

## Prerequisites

- Sealed deliverable context for DEL-01-02.
- Local sources listed in `_REFERENCES.md`.
- Applicable invariants from `docs/CONTRACT.md`.
- Human/legal availability for unresolved legal wording, license, contributor certification, and quarantine-path decisions.
- Human project authority assignment of final checklist path and reviewer role: `TBD`.
- No protected standards/code/vendor data in working examples or drafts.

## Steps

1. Confirm scope is limited to DEL-01-02, SOW-003, SOW-028, and OBJ-002.
2. Read governing local sources: INIT.md, AGENTS.md, docs/DIRECTIVE.md, docs/CONTRACT.md, docs/TYPES.md, docs/SPEC.md, docs/IP_AND_DATA_BOUNDARY.md, docs/AGENTIC_DEVELOPMENT_WORKFLOW.md, decomposition, and registers.
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
8. Once the human project authority assigns the final checklist path and reviewer role, perform a field-by-field checklist acceptance review against Specification R2 and R11.
9. Route the repo-level policy/checklist for REVIEW and then human acceptance before treating it as project policy.

## Verification

- The four-document kit stays deliverable-local.
- No repo-level artifacts are edited by this setup run.
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
