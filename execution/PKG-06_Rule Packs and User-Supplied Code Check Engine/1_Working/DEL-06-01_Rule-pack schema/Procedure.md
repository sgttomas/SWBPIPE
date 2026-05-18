# Procedure: DEL-06-01 Rule-pack schema

## Purpose

This procedure describes how to produce and verify the future rule-pack schema artifact from the current setup evidence. It is operational guidance only; this setup run does not create product schema files or modify repo-level product documentation.

## Prerequisites

- Read the sealed deliverable context in `_CONTEXT.md`.
- Confirm scope coverage for SOW-016 and SOW-042 in `docs/_Registers/ScopeLedger.csv`.
- Confirm the deliverable row and context note for DEL-06-01 in `docs/_Registers/Deliverables.csv` and `docs/_Registers/ContextBudgetQA.csv`.
- Apply applicable invariants from `docs/CONTRACT.md`, especially protected-content, provenance, rule-pack, unit, privacy, and professional-boundary invariants.
- Preserve the architecture basis for JSON Schema 2020-12, canonical JSON/JCS-compatible hashing, diagnostics, no-bypass validation, and layered validation gates.
- Stop and escalate if future work requires protected standards text, protected formulas, proprietary allowables, or private rule-pack contents to be committed publicly.

## Steps

1. Confirm that the work is limited to rule-pack schema structure and supporting documentation.
2. Define the schema envelope for rule-pack identity, schema version, rule-pack version, lifecycle/status, source notes, and contributor/review metadata.
3. Define provenance and redistribution record groups for every rule-pack value class, including formulas, allowable slots, required inputs, and public-example candidates.
4. Define checksum metadata fields for algorithm, hash scope, canonicalization basis, hash value, and invalidation behavior.
5. Define required-input declarations with dimensional category, allowed unit metadata, source/provenance requirement, missing-value behavior, and applicability conditions.
6. Define declarative formula slots and variable bindings without specifying protected formulas or arbitrary executable code.
7. Define user-supplied allowable slots with units, provenance, source notes, redistribution status, and completeness flags, without bundling actual protected values.
8. Define check criteria fields for applicability, formula references, limit references, comparison category, pass/fail/incomplete outcome, and diagnostic emission.
9. Define diagnostic categories for missing inputs, missing provenance, missing units, checksum mismatch, redistribution warning, protected-content warning, evaluator error, and rule-check blocking.
10. Add explicit public/private and professional-boundary wording so user-rule checked status is not confused with human professional approval.
11. Build invented non-code fixtures only if a later brief authorizes example work; otherwise keep fixture content `TBD`.
12. Verify that no repo-level product artifacts outside the assigned deliverable folder are edited by setup/document production work.

## Verification

Use these checks for a future implementation pass:

- Schema artifact validates against JSON Schema 2020-12.
- Required metadata, provenance, redistribution, checksum, units, input, formula, allowable, check, and diagnostic fields are present or conditionally required as specified.
- Negative fixtures produce explicit findings for missing values, missing provenance, missing units, checksum mismatch, and redistribution gaps.
- Protected-content review finds no standards text, copied protected formulas, protected tables, proprietary allowables, or real code-derived public examples.
- Checksum fixtures demonstrate canonical JSON/JCS-compatible behavior for JSON rule-pack payloads.
- Report/status wording preserves the mechanics-solved, user-rule-checked, and human-approved distinction.

## Records

Retain these records within the deliverable or later implementation evidence:

- Updated setup documents: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
- Semantic evidence: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`
- Dependency evidence: `Dependencies.csv`, `_DEPENDENCIES.md`
- Run records under `_run_records/`
- Future schema validation logs and protected-content/provenance review records when implementation begins

