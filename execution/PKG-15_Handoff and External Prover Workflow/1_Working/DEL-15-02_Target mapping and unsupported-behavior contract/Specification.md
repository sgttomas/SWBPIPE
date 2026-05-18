# Specification: DEL-15-02 Target mapping and unsupported-behavior contract

## Scope

This deliverable defines an API contract for target mapping metadata and unsupported or approximate behavior flags used by handoff exports. It covers the target mapping schema and unsupported behavior taxonomy identified in `_CONTEXT.md`.

This deliverable excludes target-specific commercial parser implementation, automatic external-prover approval states, professional acceptance records, comprehensive commercial-tool result ingestion, and any claim that a downstream target has validated, certified, sealed, or approved the engineering work. These exclusions are grounded in the PKG-15 package exclusion and the professional-boundary constraints in `docs/CONTRACT.md`.

## Requirements

| Req ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-15-02-R001 | The contract shall support target mapping metadata for handoff exports. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` DEL-15-02 | Schema review confirms a target mapping metadata surface exists. |
| DEL-15-02-R002 | The contract shall support unsupported-target flags. | `_CONTEXT.md` Scope Detail; `docs/_Registers/ScopeLedger.csv` SOW-074 | Schema review confirms unsupported-target flags are representable. |
| DEL-15-02-R003 | The contract shall support approximate behavior disclosure when target behavior cannot be represented exactly. | `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-02 row | Schema review confirms approximate behavior can be explicitly represented; exact taxonomy values are TBD. |
| DEL-15-02-R004 | The contract shall preserve unresolved assumptions and warnings in the handoff context. | `_CONTEXT.md` Scope Detail; `docs/_Registers/ScopeLedger.csv` SOW-074 | Contract review traces mapping or unsupported records to warnings and unresolved assumptions where applicable. |
| DEL-15-02-R005 | The contract shall preserve unit awareness and dimensional-check boundaries for exported values. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Principles | Schema or validation plan includes unit/dimension metadata or diagnostics for unit-bearing mapped values. |
| DEL-15-02-R006 | The contract shall preserve source, provenance, redistribution/private-public status, and review status for reliance-affecting references where present. | `docs/CONTRACT.md` OPS-K-IP-2; `docs/CONTRACT.md` OPS-K-DATA-3; `docs/TYPES.md` Provenance | Contract review confirms provenance fields or references are available; private payload copying remains prohibited. |
| DEL-15-02-R007 | The contract shall not silently replace missing or unsupported values with defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Principles | Validation plan includes blocking diagnostics or explicit TBD/unsupported findings for missing required data. |
| DEL-15-02-R008 | The contract shall not embed protected standards text, protected tables, proprietary values, private formulas, or private rule-pack payloads into public artifacts. | `docs/IP_AND_DATA_BOUNDARY.md` Private user data; `docs/SPEC.md` result export boundary | Protected-content review confirms only references, checksums, source notes, and allowed metadata are exposed. |
| DEL-15-02-R009 | The contract shall not create software-generated professional approval, certification, sealing, authentication, or code-compliance statuses. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` adapter framework boundary | Vocabulary review confirms status fields are diagnostic/handoff support only and do not imply professional reliance approval. |
| DEL-15-02-R010 | The contract shall remain compatible with the canonical handoff package schema and manifest predecessor. | `Dependencies.csv` DAG-002-E0805; `_DEPENDENCIES.md` authority boundary | Dependency trace confirms DEL-15-01 predecessor relationship remains visible and ACTIVE in the approved mirror. |
| DEL-15-02-R011 | The contract shall preserve redaction/export-control boundaries. | `Dependencies.csv` DAG-002-E0808; `docs/IP_AND_DATA_BOUNDARY.md` Private user data | Schema review confirms private data can be excluded or referenced without public disclosure; exact redaction workflow is TBD. |
| DEL-15-02-R012 | The contract shall keep exact target list, canonical package container, and target-specific mapping strategy as TBD until accepted by the governing workflow. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 | Review confirms no unapproved commercial target list, package container, or strategy has been asserted. |

## Standards

| Standard or governing basis | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | The architecture basis identifies JSON Schema 2020-12 contracts for schema-first boundaries. | Applicable; exact schema file path TBD. |
| Canonical JSON / JCS-compatible hash basis | The architecture basis identifies canonical JSON/JCS-compatible hashing where JSON payloads are hashed. | Applicable to hashed JSON payloads; exact binding fields TBD. |
| OpenPipeStress invariant catalog | Governs unit, provenance, data, professional-boundary, report, and privacy constraints. | Applicable via `docs/CONTRACT.md`. |
| Protected data boundary policy | Governs public/private data handling, protected content, and public export defaults. | Applicable via `docs/IP_AND_DATA_BOUNDARY.md`. |
| External commercial tool standards | Not established in accessible source material. | TBD; do not infer target-specific clauses or behavior. |

## Verification

| Verification ID | Approach | Covers |
|---|---|---|
| V-001 | Schema inspection against `DEL-15-02-R001` through `DEL-15-02-R004`. | Target mapping metadata, unsupported-target flags, approximate behavior, warnings, assumptions. |
| V-002 | Boundary vocabulary review. | No professional approval/status overclaim; no code-compliance claim. |
| V-003 | Protected-content and privacy review. | No private or protected data copied into public artifacts. |
| V-004 | Unit/provenance validation plan review. | Unit awareness, dimensional metadata, source/provenance. |
| V-005 | Dependency mirror check. | Approved DAG-002 predecessor evidence remains ACTIVE and unmodified. |
| V-006 | TBD review gate. | Exact target list, field names, file paths, and taxonomy values remain TBD unless supported by accepted source material. |

## Documentation

Required deliverable documentation artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- target mapping schema (TBD path)
- unsupported behavior taxonomy (TBD path)
- validation evidence for schema, protected-content boundary, and professional-boundary wording (TBD)
