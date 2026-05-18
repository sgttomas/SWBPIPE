# Specification: DEL-14-01 Immutable model state records

## Scope

This deliverable defines the data-model contract for immutable model state records and the persistence tests needed to verify that saved states remain reproducible and hash-addressable.

In scope:

- A JSON-schema-backed model state record surface for `schemas/model_state.schema.json`.
- Record support for names, tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes.
- Read-only snapshot semantics for saved model states.
- Persistence tests that demonstrate state save/load behavior and deterministic hash handling within the accepted architecture basis.

Out of scope:

- Formal prover approval statuses or automatic professional acceptance states.
- Comprehensive commercial prover output ingestion.
- Public bundling of protected standards text, code tables, proprietary values, or private owner/project data.
- Exact dependency versions, physical project package/container format, hash library selection, and package-specific implementation choices unless approved in a later sealed implementation task.

Sources: `_CONTEXT.md`; SOW-071, OBJ-016, and PKG-14 in `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/SPEC.md` sections 4.4 and 9.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-14-01-R1 | The product shall save named immutable model states. | SOW-071 in `_CONTEXT.md`, `docs/_Registers/ScopeLedger.csv`, and decomposition section 9 |
| DEL-14-01-R2 | A model state record shall carry tags, notes, external references, unresolved assumptions, warnings, and deterministic hashes. | SOW-071 in `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-14-01 |
| DEL-14-01-R3 | Saved model states shall use read-only snapshot semantics. Exact enforcement mechanism is TBD. | Decomposition section 7 DEL-14-01 sizing notes |
| DEL-14-01-R4 | JSON payload hashes shall follow the accepted JCS-compatible canonical JSON basis where the payload is JSON. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` section 4.4 |
| DEL-14-01-R5 | Hash records shall identify payload scope; exact state hash partitioning is TBD. | `docs/SPEC.md` section 4.4 |
| DEL-14-01-R6 | Round-trip persistence must not insert silent engineering defaults for units, provenance, rule-pack values, material data, component data, SIF/flexibility inputs, allowables, or load-basis values. | `docs/SPEC.md` section 4.4 |
| DEL-14-01-R7 | Model state records must preserve unresolved assumptions and warnings as visible review evidence, not hidden defaults. | `docs/DIRECTIVE.md` sections 2.2 and 3; `docs/TYPES.md` entries `Assumption` and `Diagnostic` |
| DEL-14-01-R8 | The record surface must not create software-generated professional approval, certification, sealing, authentication, or code-compliance statuses. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 9 |
| DEL-14-01-R9 | External references must remain explicit and must not bypass schema validation, privacy controls, protected-content screening, or professional-boundary checks. | `docs/TYPES.md` entry `Reference`; `docs/SPEC.md` sections 4.4 and 4.5 |
| DEL-14-01-R10 | The public repository must not embed protected standards content, proprietary values, or private project/rule data in model state examples or fixtures. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-PRIV-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |

## Standards

| Standard / basis | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Public schema/interchange baseline for this data-model change. | Source-supported by `_CONTEXT.md`; exact schema file content TBD |
| Canonical JSON / JCS-compatible basis | Applies to JSON payload hashing where the state payload is JSON. | Source-supported by `_CONTEXT.md` and `docs/SPEC.md` section 4.4; exact library/algorithm binding TBD |
| OpenPipeStress governance invariants | Data-boundary, provenance, no-silent-default, and no-professional-approval constraints. | Source-supported by `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `docs/IP_AND_DATA_BOUNDARY.md` |

No external code clause text or protected standards text is available or required for this deliverable setup.

## Verification

| Requirement(s) | Verification approach |
|---|---|
| R1-R3 | Schema validation and persistence tests should prove that saved records retain identity, metadata, and read-only snapshot semantics. Exact test names TBD. |
| R4-R5 | Hash determinism tests should compare canonicalized JSON payload bytes and verify payload-scope metadata. Exact canonicalization implementation TBD. |
| R6-R7 | Round-trip tests should confirm that assumptions, warnings, provenance-bearing references, and other review evidence survive save/load without silent defaults. |
| R8 | Schema and service tests should reject or omit automatic professional approval/code-compliance status fields. |
| R9-R10 | Protected-content/private-data review should verify examples and fixtures contain only invented or cleared public data and explicit external-reference metadata. |

## Documentation

Required deliverable records:

- `schemas/model_state.schema.json`
- model state persistence tests
- test evidence or validation output from the eventual implementation task
- any unresolved `TBD` decisions for hash scope, canonicalization implementation, schema property names, and persistence service entry points
