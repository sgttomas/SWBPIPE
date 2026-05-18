---
doc_id: DEL-03-02-PROCEDURE
doc_kind: deliverable.procedure
status: draft
created: 2026-04-30
deliverable_id: DEL-03-02
package_id: PKG-03
---

# Procedure: Pipe Section and Component Library Schema

## Purpose

This procedure describes how to produce and later maintain DEL-03-02 setup/schema artifacts without introducing protected tables, proprietary catalog data, invented engineering values, or unsupported compliance claims.

## Prerequisites

- Sealed DEL-03-02 context with write scope limited to this deliverable folder for setup evidence.
- Register rows for DEL-03-02, SOW-018, and ContextBudgetQA row DEL-03-02.
- Applicable contract invariants from `docs/CONTRACT.md`, especially IP, provenance, unit safety, privacy, governance review, and agent epistemic constraints.
- Architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
- Human-owned dependency declarations are not tracked in `_DEPENDENCIES.md`; no explicit upstream/downstream list is available.

## Steps

1. Confirm identity and boundary.
   - Verify deliverable ID `DEL-03-02`, package `PKG-03`, type `DATA_MODEL_CHANGE`, scope item `SOW-018`, and objective `OBJ-004`.
   - Confirm this setup pass does not edit repo-level schema files.

2. Gather governing source slices.
   - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, register rows, decomposition row DEL-03-02, SOW-018, OBJ-004, and applicable contract invariants.
   - Use only accessible local governance/decomposition sources for setup evidence.

3. Define schema intent without values.
   - Identify future record classes: section library record and component library record.
   - Record expected concepts: identity, schema version, user-entered dimensions/weights/COG where applicable, unit-bearing values, provenance, redistribution status, completeness status, and diagnostics hooks.
   - Leave exact field names, enum names, and reusable schema references as `TBD`.

4. Apply data-boundary rules.
   - Reject protected standards text, protected dimensional tables, proprietary catalog data, code-derived values, copied formulas, and private project data as public defaults.
   - Mark suspected protected content for quarantine and human review.

5. Map requirements to verification.
   - For each requirement, identify future schema validation, unit check, provenance gate, protected-content gate, round-trip test, or architecture review.
   - Use invented/public-safe fixtures only.

6. Perform consistency review.
   - Confirm Datasheet, Specification, Guidance, and Procedure use consistent terms: section library record, component library record, provenance, redistribution status, unit-aware value, protected-content gate, and private/public data boundary.
   - Convert unsupported details to `TBD`, `ASSUMPTION`, or a Conflict Table entry.

7. Stop and escalate when needed.
   - Stop if implementation would require protected tables, proprietary data, code-derived defaults, legal conclusions about redistribution rights, or professional/certification claims.

## Verification

- The four setup documents exist and retain required sections.
- No repo-level schema files are edited during this setup pass.
- No protected dimensional tables, standards data, proprietary catalog values, or invented engineering values are introduced.
- Unknown schema fields, enum names, validation severities, and implementation mechanisms remain `TBD`.
- Requirements cite accessible sources or are labeled as assumptions.
- Dependency extraction artifacts validate against Dependencies.csv v3.1.

## Records

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` record setup evidence.
- `_SEMANTIC.md` records the semantic lens generated for this deliverable.
- `_SEMANTIC_LENSING.md` records warranted enrichment items.
- `Dependencies.csv` and `_DEPENDENCIES.md` record extracted dependency evidence.
- `_run_records/TASK_RUN_*.md` records the required sequence, QA checks, missing inputs, and human rulings needed.

