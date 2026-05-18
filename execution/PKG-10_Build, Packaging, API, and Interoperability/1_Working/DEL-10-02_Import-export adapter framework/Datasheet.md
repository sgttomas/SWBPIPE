# Datasheet: DEL-10-02 Import/export adapter framework

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-10-02 |
| Name | Import/export adapter framework |
| Package ID | PKG-10 |
| Package | Build, Packaging, API, and Interoperability |
| Type | BACKEND_FEATURE_SLICE |
| Scope item | SOW-030 |
| Objective | OBJ-009 |
| Context envelope | L |
| Current production mode | Setup documentation only |

## Attributes

| Attribute | Value |
|---|---|
| Deliverable purpose | Define the framework obligations for future import/export adapters. |
| Anticipated artifacts | Adapter interfaces; sample invented adapter. |
| Setup-run artifact boundary | This session produces setup documents and registers only; adapter source, tests, sample adapters, manifests, and repo-level artifacts are out of write scope. |
| Architecture baseline | Schema-first command/query/job result envelopes; JSON Schema 2020-12 contracts; canonical JSON/JCS-compatible hashing where JSON payloads are hashed. |
| Adapter boundary | Adapters may translate external data but cannot bypass units, provenance, redistribution, diagnostics, private/public data controls, validation, sandboxing, envelopes, or report controls. |
| External format list | TBD; concrete protected or proprietary external formats are not bundled defaults in this deliverable. |
| Public API transport | TBD; public transport protocol remains a later human/product decision. |

## Conditions

| Condition | Source |
|---|---|
| All import/export operations must be unit-aware and deterministically report unit conversions. | docs/PRD.md section 6.6; docs/CONTRACT.md OPS-K-UNIT-1 |
| Imported data must flag missing required fields, missing or inconsistent units, missing provenance, unclear redistribution status, protected-table risk, and user-defined reasonableness concerns. | docs/PRD.md section 13.5 |
| Private rule packs, component libraries, material data, project files, and calculation results must not be transmitted or exported unexpectedly. | docs/PRD.md sections 18.2 and 18.3; docs/CONTRACT.md OPS-K-PRIV-1 |
| Adapter outputs for nontrivial operations use diagnostics/result envelopes and must not make certification or compliance claims. | docs/_Decomposition/SOFTWARE_DECOMP.md AB-00-06; docs/CONTRACT.md OPS-K-AUTH-1 |
| Adapter interfaces remain code-neutral and do not bundle protected standards data or proprietary tool behavior. | INIT.md; docs/DIRECTIVE.md; docs/IP_AND_DATA_BOUNDARY.md |

## Construction

The future framework should be described as a shell around import/export providers, not as concrete bundled adapters. Required conceptual pieces are:

- adapter identity and capability metadata;
- import and export request envelopes;
- unit-validation hooks before data enters domain workflows;
- provenance and redistribution-status validation;
- diagnostics and warning emission using project warning classes;
- private/public data boundary checks before writing exported payloads;
- rule-pack and report hooks that preserve sandboxing and report controls;
- manifest/hash hooks for reproducible imported and exported artifacts.

## References

- INIT.md
- AGENTS.md
- docs/DIRECTIVE.md
- docs/CONTRACT.md
- docs/TYPES.md
- docs/SPEC.md
- docs/IP_AND_DATA_BOUNDARY.md
- docs/PRD.md
- docs/_Decomposition/SOFTWARE_DECOMP.md
- docs/_Registers/Deliverables.csv
- docs/_Registers/ScopeLedger.csv
- docs/_Registers/ContextBudgetQA.csv
- execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-03_Application service command-query-job model/Specification.md
- execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-06_Diagnostics, warning, and result-envelope contract/Specification.md
- execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-07_API boundary and adapter contract map/Specification.md
- execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/Specification.md
