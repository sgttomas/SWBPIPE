# Specification: DEL-16-02 Operation validation and diff preview

## Scope

This deliverable covers the backend feature slice that validates proposed structured model operations and creates deterministic diff previews before those operations are applied. It is grounded in SOW-069 and OBJ-015. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`; `docs/_Registers/ScopeLedger.csv`.

This deliverable excludes hidden model mutations, autonomous engineering acceptance, product claims of professional approval or code compliance, and ownership of the structured operation schema itself. The operation schema is upstream DEL-16-01. Source: `_CONTEXT.md#Package Reference`; `Dependencies.csv` row `DAG-002-E0827`; `docs/CONTRACT.md#Invariant index`.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-16-02-001 | Proposed GUI and agent edits must enter this slice as structured model operations, not as direct hidden mutations of accepted model state. | SOW-069 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16` | Tests reject non-operation mutation attempts or mark exact behavior TBD if the operation schema is unavailable. |
| REQ-16-02-002 | The slice must perform schema validation before controlled application of an operation. | SOW-069; `_CONTEXT.md#Description` | Schema-validation tests using valid and invalid operation fixtures. Fixture source and schema path TBD pending DEL-16-01. |
| REQ-16-02-003 | The slice must perform constraint validation before controlled application of an operation. | SOW-069; `Dependencies.csv` row `DAG-002-E0828` | Tests verify constraint findings block or report invalid operations. Exact constraint-engine API TBD pending DEL-13-03. |
| REQ-16-02-004 | The slice must create deterministic diff previews before controlled application. | SOW-069; `_CONTEXT.md#Description`; `Dependencies.csv` rows `DAG-002-E0829`, `DAG-002-E0830` | Determinism tests compare repeated preview output for the same operation and model basis. Exact preview payload fields TBD. |
| REQ-16-02-005 | Invalid operations must be blocked before application. | `_CONTEXT.md#Context Envelope`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16` | Negative tests assert no application handoff occurs after schema or constraint failure. Exact application API TBD. |
| REQ-16-02-006 | Validation and preview outcomes must use structured diagnostics/result-envelope conventions and must not claim certification, sealing, approval, authentication, professional approval, or code compliance. | `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.3`; `docs/CONTRACT.md#Invariant index` | Diagnostic-envelope tests and protected product-claim checks. Exact envelope schema path TBD. |
| REQ-16-02-007 | The implementation must preserve layer/module boundaries: GUI, application services, domain core, schemas, validation, diagnostics, persistence, and tests must not be bypassed by adapters, plugins, or agents. | `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.4` | Architecture and service-boundary review; targeted tests where module paths are defined. |
| REQ-16-02-008 | The slice must be covered by validation tests aligned with the approved layered testing baseline. | `_CONTEXT.md#Anticipated Artifacts`; `execution/_Decomposition/SOFTWARE_DECOMP.md#8` | Test suite includes schema failure, constraint failure, preview determinism, no-apply-on-invalid, and diagnostics/professional-boundary cases. |

## Standards

| Standard or governing source | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Approved schema basis for public schemas/interchange. | Source-supported from `_CONTEXT.md#Architecture Basis Injection`; exact schema file TBD. |
| JCS-compatible canonical JSON hashing | Applies where JSON payloads are hashed for reproducibility or acceptance binding. | Source-supported from `_CONTEXT.md#Architecture Basis Injection`; exact hash payload scope TBD. |
| CONTRACT invariants | Applies to no-invention, professional boundary, no hidden defaults, unit/schema checks, and agent output status. | Source-supported from `docs/CONTRACT.md#Invariant index`. |
| DAG-002 active dependency mirror | Provides approved predecessor/evidence surface for this folder. | Source-supported from `Dependencies.csv` and `_DEPENDENCIES.md`; enum normalization is not performed in this workflow by project instruction. |
| External engineering standards | TBD. No accessible source text in this deliverable defines code clauses or engineering values for operation validation. |

## Verification

| Verification target | Required check |
|---|---|
| Schema-validation path | Valid operation fixtures pass; malformed or schema-incomplete operations fail with structured diagnostics. |
| Constraint-validation path | Constraint-engine findings are preserved and invalid operations do not proceed to application. Exact API TBD. |
| Diff preview determinism | Same operation and same model basis produce stable preview output. Hash basis applies only where JSON payload hashing is used. |
| No hidden mutation | Failed validation and preview-only flows do not mutate accepted model state. Exact state/apply boundary TBD. |
| Professional boundary | Outputs are diagnostics, validation outcomes, and previews only; no automatic human approval or code-compliance status is emitted. |
| Dependency fidelity | Approved DAG-002 mirror rows remain ACTIVE and are not retired, deleted, or reclassified by this setup workflow. |

## Documentation

- Operation validator design notes: TBD pending implementation.
- Diff preview service contract: TBD pending DEL-14-03 and DEL-14-05 interfaces.
- Validation test plan and fixtures: required artifact; exact fixture source TBD pending DEL-16-01.
- Diagnostic/result-envelope mapping: required for implementation; exact schema and module path TBD.
- Dependency preservation note: existing `Dependencies.csv` remains the approved local DAG-002 mirror/evidence surface.
