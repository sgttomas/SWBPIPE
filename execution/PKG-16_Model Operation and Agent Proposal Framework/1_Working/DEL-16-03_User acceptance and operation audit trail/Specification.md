# Specification: DEL-16-03 User acceptance and operation audit trail

## Scope

DEL-16-03 covers the backend feature slice for recording accepted/rejected model operations, affected entities, actor/source metadata, timestamps, assumptions, operation history, rationale, and audit metadata needed for reproducible model-state review.

The scope is bounded to SOW-069 and SOW-070 and supports OBJ-015. It does not implement hidden model mutation, autonomous engineering acceptance, professional approval, certification, sealing, or code-compliance claims. Exact audit-log schema, persistence details, and autonomy policy beyond the default user-acceptance posture remain TBD until a sealed implementation brief resolves them.

## Requirements

| ID | Requirement | Source | Verification approach |
|---|---|---|---|
| DEL-16-03-REQ-001 | GUI and agent edits that reach this surface shall be represented as structured model operations before controlled application. | SOW-069 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5; `docs/_Registers/ScopeLedger.csv` row SOW-069 | Acceptance workflow tests confirm only structured operation inputs are accepted by the audit workflow. |
| DEL-16-03-REQ-002 | The acceptance workflow shall account for schema validation, constraint validation, and diff preview before controlled application through the model engine. | SOW-069 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5 | Tests cover accepted and rejected outcomes for validation/diff statuses. Exact validation result schema is TBD. |
| DEL-16-03-REQ-003 | The audit trail shall record accepted and rejected operation outcomes. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 7 PKG-16 | Operation audit log tests assert both accepted and rejected dispositions are recorded. |
| DEL-16-03-REQ-004 | The audit trail shall record affected entities, actor/source metadata, timestamps, and assumptions. | `_CONTEXT.md` | Operation audit log tests assert presence of these fields or explicit TBD placeholders where schema names are unresolved. |
| DEL-16-03-REQ-005 | Accepted model operations shall preserve operation history, rationale, assumptions, affected entities, and audit metadata needed for reproducible model-state review. | SOW-070 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 5; `docs/_Registers/ScopeLedger.csv` row SOW-070 | Tests confirm accepted operations retain review metadata and can be retrieved for model-state review. |
| DEL-16-03-REQ-006 | The default workflow shall require user acceptance unless later explicitly changed by human-approved scope or architecture decision. | `_CONTEXT.md`; OI-016 in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 12 | Tests cover that proposed operations do not become accepted audit records without an acceptance signal. |
| DEL-16-03-REQ-007 | The audit trail shall not represent operation acceptance as professional approval, certification, sealing, or code compliance. | `docs/DIRECTIVE.md` section 3; `INIT.md`; `docs/TYPES.md` section 9; SOW-070 note | Tests or review checks assert terminology remains development/audit oriented and does not create professional reliance claims. |
| DEL-16-03-REQ-008 | Public examples and records shall not introduce protected standards text, code-specific values, proprietary project data, or private engineering data by default. | `docs/DIRECTIVE.md` section 3; `docs/IP_AND_DATA_BOUNDARY.md` sections 3-5; `docs/CONTRACT.md` OPS-K-PRIV and OPS-K-AGENT invariants | Protected-content/provenance review gate for fixtures and sample audit records. |
| DEL-16-03-REQ-009 | Missing data and assumptions shall be visible rather than silently defaulted. | `docs/DIRECTIVE.md` section 3; `docs/CONTRACT.md` OPS-K-AGENT-1; `docs/SPEC.md` section 12 | Tests verify unresolved assumptions and missing required inputs are recorded or surfaced as TBD. |

## Standards

No external piping code, protected standard, or engineering acceptance standard text is locally accessible for this deliverable. Governing standards for this setup pass are the project governance documents and accepted software decomposition listed in `_REFERENCES.md`.

Architecture basis in `_CONTEXT.md` records JSON Schema 2020-12 contracts and canonical JSON/JCS-compatible hash basis where JSON payloads are hashed, but the exact DEL-16-03 audit-log schema and persistence mechanism are TBD.

## Verification

| Verification target | Required evidence |
|---|---|
| Scope alignment | DEL-16-03 ID, PKG-16 package, SOW-069/SOW-070 scope, and OBJ-015 objective match `_CONTEXT.md`, registers, and decomposition. |
| Audit-log content | Operation audit log records disposition, affected entities, actor/source metadata, timestamp, assumptions, rationale/history where accepted, and audit metadata. |
| Acceptance workflow | Tests cover accepted and rejected model-operation paths and demonstrate that default user acceptance gates proposed operations. |
| Boundary language | Review confirms no professional approval, certification, sealing, or code-compliance claim is introduced. |
| Data boundary | Review confirms no protected standards text, proprietary project data, or private engineering data appears in public fixtures or examples. |

## Documentation

Expected artifacts from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`:

- operation audit log
- acceptance workflow tests

Additional implementation documentation remains TBD until the future sealed Type 2 brief defines exact module/file targets.
