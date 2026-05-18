# Specification: DEL-16-01 Structured model operation schema

## Scope

This deliverable covers the data-model contract for structured model operations used by GUI and agent proposal workflows. Its available source scope is limited to defining operation categories and boundary requirements for a future `schemas/model_operation.schema.json` and fixtures.

In scope:

- Structured operation records for add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge edits.
- A schema-first route for GUI and agent edits before validation, diff preview, and controlled application.
- Boundary fields or references needed to preserve unit, provenance, diagnostics, persistence, and professional-boundary controls, where those needs are explicitly supported by source documents.

Out of scope:

- The operation validator, deterministic diff preview, and invalid-operation blocking behavior owned by DEL-16-02.
- User acceptance and audit trail implementation owned by DEL-16-03.
- Agent rationale and professional-boundary controls owned by DEL-16-04, except for schema fields needed to preserve the boundary.
- Product code, concrete dependency versions, and exact schema field layout; these remain TBD unless later accepted by a sealed implementation brief or human ruling.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-16 table, SOW-069, OBJ-015, open issues `OI-012` and `OI-016`; `docs/_Registers/Deliverables.csv` rows `DEL-16-01` through `DEL-16-04`.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-16-01-R001 | The schema shall represent GUI and agent edits as structured model operations before they can alter accepted model state. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `_CONTEXT.md` Context Envelope | Schema review; fixture showing a proposal operation record rather than direct persisted mutation. |
| DEL-16-01-R002 | The schema shall cover the operation categories named in the deliverable context: add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge operations. | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-16-01 row | Schema review; category fixtures. |
| DEL-16-01-R003 | Operation records shall preserve enough structure to support schema validation, constraint validation, diff preview, and controlled application by downstream services. Exact field layout is TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 | Cross-check against DEL-16-02 when available; placeholder acceptance criteria remain TBD. |
| DEL-16-01-R004 | Operation records shall not encode autonomous engineering acceptance, certification, sealing, authentication, or code-compliance claims. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4; `docs/SPEC.md` section 4.3 | Schema lint/review for forbidden acceptance/status terms; fixture review. |
| DEL-16-01-R005 | The public schema and fixtures shall not include protected standards text, protected tables, proprietary catalog values, private project data, or invented engineering defaults. | `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AGENT-1; `docs/IP_AND_DATA_BOUNDARY.md` location TBD | Protected-content/provenance review; fixture inspection. |
| DEL-16-01-R006 | The schema shall align with JSON Schema 2020-12 as the accepted public schema/interchange baseline. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2; `docs/TYPES.md` section 8 | Schema validator configured for JSON Schema 2020-12. |
| DEL-16-01-R007 | Operation records shall preserve references to the canonical model/domain contracts rather than redefining physical model structure. | `docs/SPEC.md` sections 1 and 3; `docs/TYPES.md` `Model`, `Reference`, `TraceabilityLink` rows | Schema review for reference-based target addressing; no duplicate model schema. |
| DEL-16-01-R008 | Operation records shall expose diagnostics/provenance boundary hooks where validation or application can produce findings, without silently defaulting missing engineering inputs. | `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-DATA-3; `docs/TYPES.md` `Diagnostic`, `Provenance`, `Quantity` rows | Fixture review for explicit missing/TBD findings and provenance-bearing values. |
| DEL-16-01-R009 | ASSUMPTION: Operation records should preserve command-intent separation between transient GUI/editor actions and durable model mutation. | `docs/SPEC.md` viewport/editor slice; `docs/TYPES.md` `ViewportEditorSession` and `ViewportCommandIntent` rows | Human review required; exact operation-intent mapping is TBD. |
| DEL-16-01-R010 | ASSUMPTION: Operation records should carry or reference deterministic identity/hash context when they affect persisted project/model payloads. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04; `docs/SPEC.md` section 4.4; `docs/TYPES.md` `Checksum` row | Human review required; physical package/container and exact hash partitioning remain TBD. |

## Standards

| Standard or Control | Applicability |
|---|---|
| JSON Schema 2020-12 | Governing schema/interchange baseline for public schema artifacts, per accepted architecture basis. |
| OPS contract invariants | Binding project controls for source fidelity, IP/data boundary, unit/provenance behavior, professional boundary, and agent non-invention. |
| Protected-content and private-data policy | Applies to fixtures and public schema defaults. The exact policy text was read from referenced governance files; legal conclusions remain outside this deliverable. |
| Engineering design codes | TBD/not directly accessible for this deliverable; no code clause text or engineering values are asserted. |

## Verification

| Verification Item | Method | Status |
|---|---|---|
| Schema parses as JSON Schema 2020-12 | Future schema validation command | TBD - schema not implemented in this setup pass. |
| Operation categories represented | Fixture set covering declared categories | TBD. |
| No direct mutation route in fixtures | Review fixture semantics against proposal/controlled-application route | TBD. |
| No protected or private data in public fixtures | Protected-content/provenance gate | TBD. |
| Professional-boundary terms absent or constrained | Schema and fixture review | TBD. |
| Cross-deliverable validation handoff | Compare field needs with DEL-16-02/03/04 when those deliverables exist | TBD; not performed here to avoid sibling DEL scanning. |

## Documentation

Expected records for a future implementation pass:

- `schemas/model_operation.schema.json`
- operation fixtures for each supported category
- schema validation evidence
- protected-content/provenance review notes for public fixtures
- unresolved TBD list for operation granularity, hash binding, and command-intent mapping

No implementation evidence is created by this setup document.
