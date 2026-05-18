# Datasheet: DEL-15-04 External prover boundary metadata

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-15-04 | `_CONTEXT.md` |
| Name | External prover boundary metadata | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-15-04 |
| Package ID | PKG-15 | `_CONTEXT.md` |
| Package Name | Handoff and External Prover Workflow | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` `PKG-15` |
| Type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-15-04 |
| Scope Coverage | SOW-075 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-075 |
| Objective Support | OBJ-017; OBJ-018 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective mapping |
| Context Envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row DEL-15-04 |
| Accepted Decomposition Basis | revision 0.5 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |

## Attributes

| Attribute | Value | Evidence / Status |
|---|---|---|
| Metadata purpose | Support external-prover workflow metadata | `docs/_Registers/ScopeLedger.csv` row SOW-075 |
| Required metadata posture | Flexible names, tags, notes, external references, attachments, and comparison-report linkage | `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-04 row and SOW-075 note |
| Prohibited automatic statuses | No hard-coded approval, certification, code-compliance, formal prover lifecycle, or automatic professional acceptance status | `docs/_Registers/Deliverables.csv` row DEL-15-04; `docs/_Registers/ScopeLedger.csv` row SOW-075; `docs/TYPES.md` section 4 |
| Commercial tool result ingestion | Comprehensive commercial-tool result ingestion is out of MVP scope | `docs/_Registers/ScopeLedger.csv` row SOW-075; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-016 |
| Professional reliance boundary | Non-authoritative; software output remains decision support until competent human review | `INIT.md` Agent rule; `docs/DIRECTIVE.md` sections 1-3; `docs/CONTRACT.md` OPS-K-AUTH-1 |
| Public/private data boundary | External artifacts and examples must not introduce protected standards text, proprietary values, private project data, or commercial software examples without permission | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6; `docs/SPEC.md` report/result export boundary sections |
| Schema basis | JSON Schema 2020-12 contracts and schema-first envelopes are the accepted architecture basis | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-010 |
| Concrete schema file path | TBD | Anticipated artifact is `external reference fields`, but no file path is defined in local sources |
| Exact field names and cardinality | TBD | Sources define allowed metadata categories but not a concrete data model |

## Conditions

| Condition | Value | Source |
|---|---|---|
| Upstream architecture basis | AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 | `_CONTEXT.md` Architecture Basis Injection; `Dependencies.csv` rows DAG-002-E0723 through DAG-002-E0729 |
| Upstream professional-boundary dependency | DEL-01-04 | `Dependencies.csv` row DAG-002-E0818 |
| Upstream handoff/state dependencies | DEL-15-01, DEL-15-02, DEL-15-03, DEL-14-01 | `Dependencies.csv` rows DAG-002-E0819 through DAG-002-E0822 |
| Approved DAG mirror status | 12 ACTIVE rows, synchronized from approved DAG-002 | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Boundary validation tests | Required anticipated artifact, exact tests TBD | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-15-04 |

## Construction

| Construct | Required Shape | Source / Status |
|---|---|---|
| External reference record | Must allow flexible external references and related descriptive metadata without implying approval | `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-04 row; SOW-075 |
| Descriptive fields | Names, tags, notes, references, and attachments are in scope as metadata categories | `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-04 row |
| Comparison linkage | Comparison reports may support external-prover workflows but are diagnostic/handoff support only | `docs/_Registers/ScopeLedger.csv` rows SOW-073 and SOW-075; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-015 |
| Status fields | Must not encode automatic professional/code-compliance authority; any human acceptance reference, if present, is external and hash-bound | `docs/TYPES.md` section 4; `docs/SPEC.md` sections 4.4 and analysis-status boundary |
| Protected/private data handling | Source/provenance and redistribution status are required for public data records; protected or uncertain content is quarantined/escalated | `docs/IP_AND_DATA_BOUNDARY.md` sections 4-6 |

## References

- `_CONTEXT.md` - deliverable identity, scope, objectives, architecture basis, and anticipated artifacts.
- `_REFERENCES.md` - governing source list for this folder.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-002 local mirror/evidence surface.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.5 package, scope, objective, decision, and issue context.
- `docs/_Registers/Deliverables.csv` - row DEL-15-04.
- `docs/_Registers/ScopeLedger.csv` - row SOW-075.
- `docs/_Registers/ContextBudgetQA.csv` - row DEL-15-04.
- `INIT.md`, `docs/DIRECTIVE.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/IP_AND_DATA_BOUNDARY.md` - governance, authority, status, schema, and data-boundary constraints.
