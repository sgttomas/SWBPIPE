# Procedure: DEL-15-01 Canonical handoff package schema and manifest

## Purpose

Define the bounded procedure for producing and reviewing the canonical handoff package schema and manifest contract for DEL-15-01. This procedure is operational guidance for the deliverable artifact; it does not implement runtime export behavior.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/CONTRACT.md`; `docs/SPEC.md`.

## Prerequisites

| Prerequisite | Evidence / status |
|---|---|
| DEL-15-01 context and references are available. | `_CONTEXT.md`; `_REFERENCES.md` |
| SOW-074 and OBJ-017 are the active scope/objective basis. | `_CONTEXT.md`; decomposition; registers |
| JSON Schema 2020-12 is the schema baseline. | `_CONTEXT.md#Architecture Basis Injection`; AB-00-04/AB-00-07 |
| Unit, provenance, diagnostics, privacy, protected-content, and professional-boundary invariants are available. | `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
| Existing DAG-002 mirror rows remain ACTIVE evidence and are not reclassified by this setup run. | local `Dependencies.csv`; `_DEPENDENCIES.md` |
| Exact package container, target list, and target-specific mapping strategy are unresolved. | OI-015 |

## Steps

1. Confirm deliverable identity from `_CONTEXT.md`, including DEL-15-01, PKG-15, type `API_CONTRACT`, SOW-074, OBJ-017, and anticipated artifacts.
2. Read the DEL-15-01 decomposition entry and associated SOW-074 / OBJ-017 rows. Record scope as schema/manifest contract work, not runtime export implementation.
3. List required handoff-package slots from SOW-074: model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags, and provenance.
4. Apply architecture-basis constraints that are explicitly injected into `_CONTEXT.md`: JSON Schema 2020-12 contracts, canonical JSON/JCS-compatible hash basis for JSON payloads, schema-first envelopes, and no-bypass adapter controls.
5. Mark unsupported implementation details as `TBD`, including exact schema property names, `$id` values, package container, supported target list, target-specific mapping strategy, and executable validation fixtures.
6. Draft or review the future schema artifact so it preserves references and metadata without copying protected standards text, private project data, private rule-pack payloads, proprietary commercial data, or real secrets.
7. Ensure the manifest design includes structured warnings, assumptions, provenance, unit metadata, hashes, and professional-boundary posture.
8. Verify that target mapping metadata and unsupported-target flags are present as contract surfaces while detailed semantics remain delegated to DEL-15-02.
9. Check that no field, enum, status, or explanatory text creates automatic professional approval, certification, sealing, authentication, endorsement, or code-compliance claims.
10. Validate dependency artifacts locally if `Dependencies.csv` exists using `python3 tools/validation/validate_dependencies_schema.py`.

## Verification

| Verification item | Method |
|---|---|
| Source grounding | Check each non-trivial requirement against `_CONTEXT.md`, decomposition, registers, local DAG-002 mirror, or governing references. |
| Schema baseline | Confirm JSON Schema 2020-12 is recorded as the schema baseline; exact schema file validation is TBD until the schema artifact exists. |
| Required slots | Confirm the SOW-074 slots are present as requirements or explicit TBD surfaces. |
| TBD discipline | Confirm package container, target list, exact field names, and target-specific mappings are not invented. |
| Data boundary | Check that schema guidance does not embed protected/private/proprietary payloads or examples. |
| Professional boundary | Check that the contract does not emit automatic approval/compliance/certification statuses. |
| Dependency mirror preservation | Confirm approved DAG-002 rows remain ACTIVE and are not retired, deleted, or reclassified. |

## Records

Maintain these records in the DEL-15-01 folder for setup:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- existing `_DEPENDENCIES.md`
- existing `Dependencies.csv`
- final run report with dependency-schema validation result and any dependency-extract conflicts
