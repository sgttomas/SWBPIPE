---
doc_id: DEL-10-03-SPECIFICATION
doc_kind: deliverable.specification
status: draft
created: 2026-04-30
deliverable_id: DEL-10-03
package_id: PKG-10
---

# Specification: Local FEA Handoff Data Contract

## Scope

DEL-10-03 defines a local FEA handoff data contract for selected local-detail problems that may require external shell/solid analysis. It covers the conceptual export package, advisory handoff criteria labels, diagnostics, units, provenance, privacy, protected-content controls, reproducibility metadata, and professional-boundary language. SourcePath: `_CONTEXT.md`; SectionRef: Description, Scope Coverage, Objective Support, Architecture Basis Injection.

This deliverable does not implement shell/solid FEA, generate meshes, choose an external solver, choose final external exchange formats, encode proprietary tool behavior, provide code-specific local-stress acceptance rules, declare global beam analysis sufficient for a project, or certify local FEA results. SourcePath: `_CONTEXT.md`; SectionRef: Context Envelope and Acceptance/risk notes. SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: rows SOW-031 and SOW-049.

## Requirements

| ID | Requirement | Evidence |
|---|---|---|
| DEL-10-03-REQ-01 | The local FEA handoff contract shall treat global centerline/frame analysis as the normal global method and local shell/solid FEA as an optional specialized handoff path. | SourcePath: `docs/DIRECTIVE.md`; SectionRef: section 3 principle 3 and section 4.2. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-MECH-1. |
| DEL-10-03-REQ-02 | The contract shall define schema-ready concept slots for handoff package identity, selected local region, global model context, boundary condition transfer, local-detail notes, diagnostics, limitations, provenance, privacy, and reproducibility metadata. | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03, AB-00-04, AB-00-06, AB-00-07. |
| DEL-10-03-REQ-03 | Handoff payload concepts shall remain unit-aware and dimensionally explicit for coordinates, forces, moments, displacements, rotations, stresses, load/result references, and any user-supplied local-detail assumptions. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1. SourcePath: `docs/SPEC.md`; SectionRef: sections 1, 3, 4, and 8. |
| DEL-10-03-REQ-04 | Handoff payload concepts shall carry source/provenance, redistribution status, privacy classification, and review status where engineering reliance or public contribution risk may be affected. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2, OPS-K-DATA-3, OPS-K-PRIV. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: sections 4-6. |
| DEL-10-03-REQ-05 | The contract shall surface missing solve-required values, missing rule-check values, missing provenance, unresolved local-detail assumptions, and unresolved format/tool decisions as explicit diagnostics or `TBD`s, never as silent defaults. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-2 and OPS-K-AGENT-1. SourcePath: `docs/SPEC.md`; SectionRef: section 7 warning classes. |
| DEL-10-03-REQ-06 | Handoff guidance labels shall be advisory only and shall not assert code compliance, professional approval, certification, sealing, endorsement, or project-specific acceptability. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1. SourcePath: `docs/TYPES.md`; SectionRef: sections 4 and 6. |
| DEL-10-03-REQ-07 | The contract shall prohibit public artifacts from embedding protected standards text, protected tables, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data without rights, private project data, or private rule-pack values. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1 and OPS-K-IP-3. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: sections 2-3. |
| DEL-10-03-REQ-08 | The handoff contract shall align with the schema-first command/query/job/result-envelope boundary and shall not allow an export adapter or plugin to bypass domain validation, unit checks, diagnostics, provenance checks, privacy controls, protected-content screening, or report controls. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02, AB-00-03, AB-00-06, AB-00-07. SourcePath: `docs/architecture/plugin_boundary.md`; SectionRef: Boundary Rules and No-Bypass Constraints. |
| DEL-10-03-REQ-09 | Final public API transport, concrete external FEA format list, concrete adapter implementation, external solver invocation semantics, and schema file placement shall remain `TBD` unless later approved in a separate implementation deliverable. | SourcePath: `_CONTEXT.md`; SectionRef: Still TBD. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-004 and section 8.2. |
| DEL-10-03-REQ-10 | The contract shall include advisory criteria labels that distinguish expected global-centerline sufficiency from recommended local-detail review or local FEA handoff, while keeping the final engineering decision with a competent human. | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: rows SOW-031 and SOW-049. SourcePath: `docs/DIRECTIVE.md`; SectionRef: sections 2.2 and 3. |
| DEL-10-03-REQ-11 | The contract shall require diagnostics/result envelopes to carry code, class, severity, source, affected object, message, remediation, and provenance for handoff-blocking, assumption, provenance, privacy, and IP-boundary findings. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06. SourcePath: `docs/SPEC.md`; SectionRef: section 7. |
| DEL-10-03-REQ-12 | Verification for this setup deliverable shall include document review, semantic/lensing completeness checks, dependency register validation, protected-content/professional-boundary scans, and preservation of unresolved implementation decisions as `TBD`. | SourcePath: `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`; SectionRef: sections 4-5. SourcePath: `docs/VALIDATION_STRATEGY.md`; SectionRef: sections 1, 2, and 4. |

## Standards

No protected engineering code, standards clauses, tables, formulas, examples, material allowables, SIF/flexibility content, protected dimensional data, proprietary external-tool behavior, or proprietary vendor data are incorporated into this deliverable.

Applicable internal baselines:

- `docs/CONTRACT.md` invariant catalog, especially OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-UNIT-1, OPS-K-PRIV, OPS-K-AUTH-1, OPS-K-AGENT-1..4, and OPS-K-MECH-1/2.
- `docs/DIRECTIVE.md` for centerline global model first, local FEA as specialized handoff, stop rules, and professional-responsibility boundaries.
- `docs/TYPES.md` for API contract type, analysis-status vocabulary, epistemic labels, provenance labels, centerline model, and local FEA handoff vocabulary.
- `docs/SPEC.md` for layer responsibilities, domain objects, solver/result/report boundaries, diagnostics classes, verification strategy, and Type 2 acceptance semantics.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data, provenance, quarantine, private user data, and report boundary policy.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-031, SOW-049, OBJ-009, PKG-10, AB-00-02/03/04/06/07/08, and OI-004.

External implementation baseline:

- JSON Schema 2020-12 is the accepted public schema/interchange baseline by SCA-001. This setup deliverable does not create a repository-level schema file and does not choose an external FEA exchange format.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| REQ-01, REQ-06, REQ-10 | Boundary review confirms local FEA is framed as optional guidance and that human review remains required. |
| REQ-02, REQ-03, REQ-04, REQ-11 | Contract review confirms package identity, selected local region, global context, boundary-condition concepts, units, provenance, diagnostics, privacy, and reproducibility slots are represented at concept level. |
| REQ-05, REQ-09 | TBD review confirms missing values, external format/tool decisions, schema placement, and implementation details are visible rather than silently resolved. |
| REQ-07 | Protected-content review confirms no protected standards data, proprietary external-tool behavior, private project data, or private rule-pack values are introduced. |
| REQ-08 | Adapter/API review confirms no-bypass constraints are carried into the handoff boundary. |
| REQ-12 | Setup evidence review confirms four docs, semantic/lensing artifacts, dependency artifacts, run records, and status updates exist and pass local gates. |
| Future implementation gate | Later source-code or schema work must add tests for handoff export schema validation, unit/provenance/privacy checks, protected-content screening, diagnostics, reproducibility metadata, and no-certification language before release use. |

### Interim Setup Acceptance Criteria

| Gate | Pass condition |
|---|---|
| Scope gate | Only DEL-10-03 deliverable-local files are edited. |
| Document gate | Datasheet, Specification, Guidance, and Procedure exist and include default sections. |
| Semantic gate | `_SEMANTIC.md` exists, contains matrices A, B, C, F, D, K, G, X, T, E, and passes the local result-cell audit. |
| Lensing gate | `_SEMANTIC_LENSING.md` exists, covers every cell in matrices A, B, C, F, D, X, E, and preserves lens-not-authority separation. |
| Dependency gate | `Dependencies.csv` exists, validates against v3.1 required columns, uses canonical write-form enums, and `_DEPENDENCIES.md` summarizes active rows. |
| Boundary gate | No external FEA implementation, mesh generation, external tool behavior, final format choice, source code, package manifest, protected data, private engineering data, or certification/compliance claim is introduced. |

## Documentation

Required setup artifacts for DEL-10-03:

- Deliverable-local local FEA handoff contract kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
- Semantic setup artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `_run_records/*`.
- Dependency artifacts: `Dependencies.csv` and `_DEPENDENCIES.md`.

Concept inventory for later schema work:

| Concept slot | Current status | Evidence |
|---|---|---|
| Handoff package schema version | Required concept; exact field name/layout TBD. | `_CONTEXT.md` Anticipated Artifacts; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04. |
| Source model/result references | Required concept; exact object reference layout TBD. | `docs/SPEC.md` sections 3-5 and 8. |
| Selected local region | Required concept; selection UX and persistence details TBD. | `docs/_Registers/ScopeLedger.csv` row SOW-031. |
| Boundary-condition transfer | Required concept; no external solver mapping chosen. | `docs/_Registers/ScopeLedger.csv` row SOW-031. |
| Advisory criteria label | Required concept; guidance only and human-reviewed. | `docs/_Registers/ScopeLedger.csv` row SOW-049. |
| Units/provenance/privacy/diagnostics | Required concepts; exact schema fields TBD. | `docs/CONTRACT.md` OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06. |
| Reproducibility/hash basis | Required where payload hashes are used; canonicalization details TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and section 8.2. |
