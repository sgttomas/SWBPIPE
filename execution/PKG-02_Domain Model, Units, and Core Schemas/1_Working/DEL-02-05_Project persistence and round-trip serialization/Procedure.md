# Procedure - DEL-02-05 Project Persistence and Round-Trip Serialization

## Purpose

This procedure describes how to produce and check the DEL-02-05 artifacts: project file schema, round-trip tests, and persistence service contract. It is operational guidance for implementation and review; it does not itself implement the schema or service.

## Prerequisites

- Confirm the active deliverable is DEL-02-05 under PKG-02 and that write scope is limited to the deliverable-local artifacts. Source: `_CONTEXT.md`; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 4.
- Use `_CONTEXT.md` revision 0.4, `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4, and register rows for DEL-02-05, SOW-050, SOW-041, and ContextBudgetQA DEL-02-05.
- Apply SCA-001 basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 only to the extent they constrain this persistence deliverable.
- Treat `_DEPENDENCIES.md` as human-owned dependency context. Current state: no specific upstream/downstream dependency list was declared.
- Keep protected standards/code data, proprietary values, and code-compliance/certification claims out of public artifacts. Source: `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`.

## Steps

1. Confirm scope and lifecycle state.
   - Verify the deliverable folder, status, allowed writes, and anticipated artifacts.
   - Stop or escalate if the task requires protected data or expands outside project persistence.

2. Establish the source set.
   - Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `docs/_Registers/*.csv` rows for this deliverable, `docs/CONTRACT.md`, and the relevant SOFTWARE_DECOMP sections.
   - Record missing or inaccessible sources as warnings; do not invent requirements.

3. Define the persistence boundary.
   - Include create/open/save/version behavior, project schema, round-trip serialization, and service diagnostics.
   - Exclude numerical solving, GUI views, rule-pack evaluator behavior, report rendering, external API transport, and physical package/container decisions unless a later brief authorizes them.

4. Draft or update the project file schema contract.
   - Include schema/version metadata, migration status, project identity, unit-system references, model payload, load payloads, rule-pack references, and provenance metadata.
   - Enumerate the logical project-envelope slots before choosing exact files or generated types: document metadata, project identity/storage policy, unit system, model payload, load payloads, rule-pack references, provenance/redistribution metadata, diagnostics/migration status, reproducibility metadata, and optional review records.
   - For rule-pack references, capture ID/name, version, checksum, source note, redistribution/private status, and missing/private-reference diagnostics without embedding protected rule content in public artifacts.
   - For provenance-bearing public data records, include source, source location, license or redistribution basis, contributor certification, redistribution status, and review status when applicable.
   - Use JSON Schema 2020-12 as the public schema/interchange baseline.
   - Mark exact schema file layout, code-generation tooling, migration framework, and physical project container as TBD unless already resolved by an approved decision.

5. Define the persistence service contract.
   - Specify create/open/save/validate/version-check/migrate entry points and expected result envelopes.
   - For each operation, document minimum inputs, outputs, diagnostics, and boundary checks:
     - create: project identity, declared unit system, storage/private-data policy, and optional template/reference selection;
     - open: project artifact reference, schema/version handling, validation, migration status, and diagnostics;
     - save: validated project envelope, target artifact reference, canonicalization/hash evidence, and diagnostics;
     - validate: schema, unit, provenance, rule-pack-reference, private-data, and protected-content diagnostics;
     - version-check/migrate: unsupported, stale, failed, and TBD newer/current status handling.
   - Require structured diagnostics for validation, migration, protected-content, private-data, unit, provenance, rule-pack-reference, and professional-boundary failures or warnings.
   - Preserve application-service/domain-core/storage boundaries; adapters and plugins cannot bypass validation or data-boundary checks.

6. Define canonicalization and reproducibility handling.
   - Use canonical JSON with JCS-compatible canonicalization as the basis for JSON payload hashes.
   - Record whether each hash applies to a normalized JSON payload, input manifest, referenced private/rule/library artifact, or non-JSON/binary asset manifest. Exact split is TBD.
   - Document which fields are excluded from deterministic comparison because they are volatile, environment-specific, or session-only. If exclusion cannot be justified from source or approved decision, mark the partition TBD.
   - Avoid relying on non-deterministic field ordering, timestamps, machine-local paths, or session-only state for deterministic comparisons.

7. Build the round-trip test plan.
   - Include a fixture inventory with valid invented fixtures and invalid fixtures.
   - Minimum fixture classes: valid minimal invented project; invalid schema shape; missing units; incompatible unit metadata; missing/weak provenance; private rule-pack reference; missing rule-pack reference; unsupported/stale/failed migration version cases; and synthetic protected-content/private-data boundary warnings that do not include actual protected data.
   - Test parse -> validate/normalize -> serialize -> parse cycles for semantic equality.
   - Test semantic equality by category: model identity/references, unit metadata, loads, rule-pack references, provenance metadata, and reproducibility metadata.
   - Test canonical output/hash stability for JSON payloads and record expected canonical JSON/hash outputs for each fixture where the payload partition is known.
   - Test unit metadata preservation, missing provenance findings, migration status handling, rule-pack reference preservation, and private/protected data boundary checks.

8. Review public-data safety.
   - Confirm public fixtures are invented, original, public-domain, or permissively licensed.
   - Confirm no protected standards text, protected tables, proprietary formulas, material allowables, copied SIF/flexibility data, or commercial examples are present.
   - Confirm no persisted status claims automatic code compliance or professional approval.
   - Produce or update a `public_fixture_data_boundary_review` record (PROPOSAL; final path TBD) with fixture name, source/provenance, redistribution status, contributor certification, protected-content review status, private-data review status, and professional-boundary review status.

9. Record open items.
   - Keep physical project package/container, migration framework/tooling, binary asset packaging, exact schema file layout, public API transport, dependency versions, diagnostic code names, status labels, and hash payload partitioning as TBD unless resolved by human/architecture decision.
   - Record the open-decision artifact path as TBD until the repo-level ADR/open-decision convention is approved; AB-00-01 requires decision records, and OI-011 assigns physical container/migration details to PKG-02 / human architecture decision.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Artifacts address DEL-02-05 only and do not implement solver, GUI, reporting, rule evaluator, or external transport behavior. |
| Schema check | Project fixtures validate against JSON Schema 2020-12 contracts; invalid fixtures fail deterministically; each logical project-envelope slot is covered or explicitly marked TBD. |
| Round-trip check | Required model, unit, load, rule-pack reference, provenance, and reproducibility metadata survives round trip without semantic loss. |
| Equality check | The test plan states per-category equality criteria for model content, unit metadata, loads, rule-pack references, provenance metadata, and reproducibility metadata. |
| Canonicalization check | JSON payloads produce stable canonical serialization and stable JCS-compatible hash inputs; the payload/manifest partition is documented or marked TBD. |
| Migration check | Unsupported, stale, and failed migration cases produce explicit migration status or diagnostics; newer/current status labels remain TBD unless approved. |
| Unit/provenance check | Missing/inconsistent unit metadata and missing/weak provenance are surfaced as findings. |
| Rule-pack reference check | Rule-pack reference ID/name, version, checksum, source note, and private/public or redistribution status survive round trip without exposing protected rule content. |
| Boundary check | Public fixtures contain no protected standards/code data, no proprietary values, no private rule-pack expansion, and no compliance/certification claims. |
| Service-contract check | Create/open/save/validate/version-check/migrate failures return structured diagnostics/result envelopes with affected objects and remediation where applicable. |
| Review-record check | Public fixture/data-boundary review evidence names provenance, redistribution status, protected-content review, private-data review, and professional-boundary review disposition. |

## Records

Retain or produce:

- project file schema and schema-version notes;
- persistence service contract;
- project-envelope field inventory and schema-version notes;
- round-trip fixture inventory and expected canonical/hash outputs;
- expected semantic equality criteria by fixture category;
- validation diagnostics examples for invalid units, missing provenance, rule-pack reference failures, unsupported/stale/failed migration status, protected/private data warnings, and professional-boundary warnings;
- open-decision log for physical container, migration tooling, schema layout, dependency versions, diagnostic code names, status labels, and hash payload partitioning, with artifact path TBD until approved;
- `public_fixture_data_boundary_review` record (PROPOSAL; final path TBD) or equivalent review evidence showing that no protected standards/code data, proprietary values, private data leakage, or professional approval claims were introduced.
