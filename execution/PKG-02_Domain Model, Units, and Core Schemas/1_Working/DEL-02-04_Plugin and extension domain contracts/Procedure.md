---
doc_id: DEL-02-04-PROCEDURE
doc_kind: deliverable.procedure
status: draft
created: 2026-04-30
deliverable_id: DEL-02-04
package_id: PKG-02
---

# Procedure: Plugin and Extension Domain Contracts

## Purpose

This procedure describes how to produce and later maintain the DEL-02-04 plugin/extension domain contract artifacts without expanding beyond the sealed PKG-02 domain/API scope or introducing protected/private data into public artifacts.

## Prerequisites

- Sealed deliverable context for DEL-02-04 with explicit write scope. SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-02-04.
- Current `_STATUS.md` state that permits drafting or update. For this Pass 3 run, state is `SEMANTIC_READY`; the prior Pass 1+2 drafting history includes `OPEN` as recorded in `_STATUS.md`.
- Scope, deliverable, and context-budget register rows for DEL-02-04 and SOW-038. SourcePath: `docs/_Registers/Deliverables.csv`; SectionRef: row DEL-02-04. SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038. SourcePath: `docs/_Registers/ContextBudgetQA.csv`; SectionRef: row DEL-02-04.
- Applicable contract invariants from `docs/CONTRACT.md`, especially IP/data boundary, unit safety, rule sandboxing, report controls, privacy, and agent epistemic constraints.
- SCA-001 architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08. SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection.
- No human-declared upstream/downstream dependency list is available in `_DEPENDENCIES.md`; dependencies are coordinated externally by humans. ASSUMPTION: implementation work may later need outputs from DEL-02-01 and DEL-02-02, but this document kit does not add that dependency.

## Steps

1. Confirm identity and boundary.
   - Verify deliverable ID `DEL-02-04`, package `PKG-02`, type `API_CONTRACT`, scope item `SOW-038`, and objective `OBJ-009`.
   - Confirm the work is limited to plugin/adapter domain/API contract artifacts and does not implement runtime loading, external formats, solver behavior, or GUI views.

2. Gather governing source slices.
   - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and the DEL-02-04 rows in the registers.
   - Read `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/DIRECTIVE.md`, `docs/VALIDATION_STRATEGY.md`, and relevant SCA-001 rows in `docs/_Decomposition/SOFTWARE_DECOMP.md`.
   - Use PRD sections only as supporting context where the decomposition or scope ledger cites them.

3. Define the plugin interface specification at contract level.
   - Identify required manifest concepts: plugin identity, version, extension point declarations, schema/envelope version, capability request, provenance/data-boundary declaration, diagnostics compatibility, and review status.
   - Mark exact schema file layout, field names, code-generation tooling, and transport binding as `TBD` unless later source material resolves them.

4. Define extension point families conservatively.
   - Candidate families are import adapter, export adapter, report/output extension, validation hook, and rule-pack integration hook.
   - Mark the registry as ASSUMPTION until the human project authority or later architecture/API deliverables approve concrete extension point names.

5. Apply mandatory no-bypass controls.
   - Require unit-aware validation for dimensional/numerical data.
   - Require source/provenance, redistribution status, and review disposition for imported or public data.
   - Require diagnostics/result-envelope compatibility for plugin/adapter failures and warnings.
   - Require protected-content, public/private data-boundary, and report-control checks before data is committed, exported, or included in public artifacts.
   - Prohibit certification, approval, endorsement, or automatic code-compliance claims.

6. Draft sandbox and permission model notes.
   - Start from deny-by-default.
   - Record capability classes as ASSUMPTION/TBD until implementation design resolves them.
   - Include at least these candidate classes for later review: governed project mutation, read-only query access, private-library access, filesystem path access, network access, report/export generation, background job execution, and rule-pack evaluator integration.
   - Do not resolve exact sandbox technology, process isolation, permissions syntax, or storage paths in this deliverable.

7. Map requirements to verification.
   - For every normative requirement, define a future verification path: schema validation, unit tests, provenance checks, protected-content gates, diagnostics envelope tests, rule-sandbox tests, or architecture review.
   - Use invented, public-safe fixtures only.

8. Perform cross-document consistency review.
   - Confirm Datasheet attributes appear in Specification requirements where appropriate.
   - Confirm Specification requirements have Guidance rationale and Procedure verification hooks.
   - Replace unsupported detail with `TBD` or `ASSUMPTION`.
   - Add or update the Guidance conflict table if sources disagree.

9. Stop and escalate when needed.
   - Stop if protected standards/proprietary data is needed, if permission/sandbox design would require resolving a human security decision, if a requirement would blur user-rule status with professional approval, or if source conflicts affect safety or code-relevant values.

## Verification

Completion checks for this deliverable:

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist and retain the default required sections.
- Scope remains DEL-02-04 and PKG-02; no files outside the allowed write targets are modified.
- Requirements cite accessible source slices or are labeled ASSUMPTION/TBD.
- No protected standards/code text, copied tables, copied code formulas, proprietary commercial data, or private project/rule/library data is introduced.
- No statement claims certification, sealing, approval, endorsement, or automatic engineering code compliance.
- Terms are used consistently: plugin/adapter, extension point, schema-first envelope, unit-aware validation, provenance, diagnostics/result envelope, public/private data boundary, and report controls.
- Remaining implementation details are visible as TBD, especially public API transport, import/export format list, exact permission taxonomy, exact sandbox mechanism, exact dependency versions, CI thresholds, and concrete schema layout.

## Records

- `Datasheet.md` records deliverable identity, attributes, conditions, construction assumptions, and source references.
- `Specification.md` records normative requirements, standards/baselines, verification mapping, and documentation obligations.
- `Guidance.md` records rationale, principles, considerations, trade-offs, invented safe examples, and conflict status.
- `Procedure.md` records the production and maintenance procedure for the contract kit.
- Future manifest/schema review record should identify the approved fields or explicit `TBD` placeholders for plugin identity/version, extension declarations, schema/envelope version, capability declarations, provenance/data-boundary declarations, diagnostics compatibility, and review/status metadata.
- Future canonicalization/hash verification record should identify the manifest or payload schema version, canonicalization basis, sample invented fixtures, expected hashes, and unresolved `TBD` items when JSON hashing is required.
- Future sandbox/privacy verification record should document rule-pack-facing sandbox tests, denied arbitrary-code/filesystem/network paths, capability-grant review evidence, and any approved exception for telemetry-facing or private-data exposure.
- `_run_records/TASK_RUN_*.md` records the TASK execution, sources consulted, outputs, QA checks, and warnings.
- `_STATUS.md` records the safe lifecycle update when the current state permits it.
