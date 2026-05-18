---
doc_id: DEL-10-01-DATASHEET
doc_kind: deliverable.datasheet
status: draft
created: 2026-04-30
deliverable_id: DEL-10-01
package_id: PKG-10
---

# Datasheet: Public API and Plugin Boundary

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-10-01 |
| Package ID | PKG-10 |
| Package | Build, Packaging, API, and Interoperability |
| Type | API_CONTRACT |
| Scope item | SOW-030 |
| Objective | OBJ-009 |
| Anticipated artifacts | `api/openapi.yaml` or equivalent; plugin boundary doc |
| Current artifact form | Deliverable-local contract kit; repository-level `api/openapi.yaml` is outside this write scope |
| Lifecycle target for setup | `SEMANTIC_READY` after the setup sequence passes |
| Transport decision | TBD; no final external transport selected here |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Boundary purpose | Define public API/plugin boundaries for model import/export, solver invocation, results, and rule-pack hooks. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row DEL-10-01 |
| Public API capability families | Model creation/import/export, load-case definition, solve execution, result extraction, rule-pack evaluation, report generation, and validation-test execution. | `docs/PRD.md` section 19.3 |
| Contract baseline | Schema-first command/query/job/result envelopes; JSON Schema 2020-12 public schema/interchange basis. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| Adapter/plugin baseline | Deny-bypass boundary: adapters/plugins may translate or extend, but cannot bypass units, provenance, diagnostics, rule sandboxing, public/private data controls, or report controls. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-07; `docs/SPEC.md` section 1 |
| Professional boundary | API results can expose mechanics status and user-rule-check status, but not software-declared code compliance or professional approval. | `docs/TYPES.md` sections 4 and 6; `docs/CONTRACT.md` OPS-K-AUTH-1 |
| Protected-data boundary | Public API/plugin contracts must not bundle protected standards text, tables, code-derived formulas, allowables, SIF/flexibility tables, proprietary catalogs, or private project/rule data. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-3 |

## Conditions

| Condition | Required handling |
|---|---|
| Missing units on dimensional data | Reject, block, or emit a diagnostic; do not silently default. |
| Missing provenance or redistribution status | Emit provenance/data-boundary diagnostics and block public contribution paths when needed. |
| Protected or proprietary data suspected | Stop ingestion, quarantine/escalate, and do not import into public artifacts. |
| Rule-pack-facing hook | Preserve sandboxed, deterministic, unit-aware evaluation; do not expose arbitrary code execution. |
| Result publication/export | Use result/diagnostic envelopes and preserve report controls and professional-responsibility notices. |
| Private data exposure | Local-first by default; no telemetry or external transmission of private project/rule/component/material data by default. |

## Construction

This deliverable records a contract boundary, not an implementation. The boundary is organized around these contract surfaces:

| Surface | Contract role | Setup status |
|---|---|---|
| Public API envelope | Common schema-first wrapper for commands, queries, jobs, diagnostics, provenance, and result metadata. | Baseline defined; concrete schema file layout TBD. |
| Model import/export boundary | Allows external data exchange only through schema, unit, provenance, redistribution, and private/public data checks. | Boundary defined; external formats TBD. |
| Solver invocation boundary | Routes solve requests through application-service command/job semantics with diagnostics, progress/cancellation, and reproducibility metadata. | Boundary defined; concrete transport TBD. |
| Results boundary | Exposes mechanical results, diagnostics, warnings, hashes, and rule-pack status without compliance or approval claims. | Boundary defined; result export details shared with DEL-08-04/DEL-10-05. |
| Rule-pack hook boundary | Allows governed rule-pack evaluation hooks while preserving sandboxing, unit safety, checksums, source notes, and private/public markings. | Boundary defined; expression grammar/library TBD. |
| Plugin manifest boundary | Candidate manifest concept slots include identity, version, declared extension point, schema/envelope compatibility, capability request, data-boundary declaration, diagnostics compatibility, and review status. | Concept inventory only; exact field names and permission taxonomy TBD. |

## References

- `_CONTEXT.md` for sealed deliverable identity, scope, objective, write scope, and SCA-001 architecture-basis injection.
- `docs/CONTRACT.md` for applicable invariants: OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-RULE-1/2/3, OPS-K-PRIV-1/2, OPS-K-AUTH-1, and OPS-K-AGENT-1..4.
- `docs/SPEC.md` sections 1, 6, 7, 8, 10, and 11 for layer responsibilities, adapter no-bypass language, rule-pack sandboxing, diagnostics/reporting, and deliverable acceptance semantics.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-030, OBJ-009, AB-00-02/03/04/06/07/08, and OI-004.
- `docs/PRD.md` sections 13.4, 13.5, 18.2, 18.3, and 19.3 for public/private data, import warnings, telemetry/private-data protection, and public API capability families.
