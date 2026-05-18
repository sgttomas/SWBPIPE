---
doc_id: DEL-02-04-SPECIFICATION
doc_kind: deliverable.specification
status: draft
created: 2026-04-30
deliverable_id: DEL-02-04
package_id: PKG-02
---

# Specification: Plugin and Extension Domain Contracts

## Scope

DEL-02-04 defines the domain/API contract surface for plugin and adapter extension points in OpenPipeStress. It covers plugin/adapter interface expectations, mandatory governance checks, sandbox/permission-model notes, diagnostics obligations, and data-boundary constraints. SourcePath: `_CONTEXT.md`; SectionRef: Description and Anticipated Artifacts.

This deliverable is bounded to PKG-02 domain/API definitions. It does not implement a plugin loader, public API transport, external import/export formats, GUI views, numerical solving behavior, rule-pack evaluator internals, storage container mechanics, or concrete dependency versions. SourcePath: `_CONTEXT.md`; SectionRef: Package Reference; SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline and DEC-012.

## Requirements

| ID | Requirement | Evidence |
|---|---|---|
| DEL-02-04-REQ-01 | The contract shall define extension points for plugins/adapters without allowing those extensions to bypass governance, validation, diagnostics, unit safety, report controls, or public/private data-boundary checks. | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: SOW-038. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02 and AB-00-07. |
| DEL-02-04-REQ-02 | Plugin/adapter data ingress and egress shall be unit-aware where dimensional or numerical values are present, and incompatible or missing unit metadata shall be reported rather than silently defaulted. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1 and OPS-K-DATA-2. SourcePath: `docs/PRD.md`; SectionRef: 6.6 Unit Safety. |
| DEL-02-04-REQ-03 | Plugin/adapter contracts shall preserve source/provenance metadata for material, component, rule, load, allowable, SIF/flexibility, and report-affecting values. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-3. SourcePath: `docs/PRD.md`; SectionRef: 6.3 Data Provenance by Design. |
| DEL-02-04-REQ-04 | Any public data contribution path exposed through plugins/adapters shall require source, source location, redistribution/license status, contributor certification, and review disposition; missing values shall be `TBD` or rejected by policy. | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2 and OPS-K-AGENT-1. |
| DEL-02-04-REQ-05 | Plugin/adapter contracts shall prohibit public redistribution of protected standards text, protected tables, protected examples, copied code formulas, protected dimensional data, proprietary vendor data without rights, and private rule/project/library data. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 3. Public repository must not contain. |
| DEL-02-04-REQ-06 | Suspected protected or proprietary content encountered by a plugin/adapter shall be classified, quarantined, and escalated for human/legal review rather than imported into public artifacts. | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 5. Quarantine rule. SourcePath: `docs/DIRECTIVE.md`; SectionRef: 5. Authority and stop rules. |
| DEL-02-04-REQ-07 | Plugin/adapter diagnostics shall use the project diagnostic/result-envelope basis, including code, class, severity, source, affected object, message, remediation, and provenance where applicable. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06. |
| DEL-02-04-REQ-08 | Plugin/adapter outputs and reports shall not claim certification, sealing, approval, authentication, endorsement, or engineering code compliance for reliance. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1. SourcePath: `docs/DIRECTIVE.md`; SectionRef: 4.2 Out of scope. |
| DEL-02-04-REQ-09 | Mutating plugin/adapter operations shall route through schema-first application-service commands or equivalent governed service boundaries, not direct domain-core, solver, storage, or report-control bypasses. For this contract, an equivalent governed boundary preserves schema validation, unit checks where dimensional data is present, diagnostics/result envelopes, provenance and public/private data-boundary checks, report controls, and audit/reproducibility metadata. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-02, AB-00-03, and AB-00-07. SourcePath: `docs/SPEC.md`; SectionRef: 1. Architectural overview. |
| DEL-02-04-REQ-10 | Read-only and long-running plugin/adapter operations shall preserve the command/query/job distinction where exposed through application services, including cancellation/progress and reproducibility metadata when applicable. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-03. SourcePath: `docs/PRD.md`; SectionRef: 20. Performance Requirements. |
| DEL-02-04-REQ-11 | Public plugin manifests and interchange contracts shall align with the accepted JSON Schema 2020-12 and schema-first envelope baseline. Exact schema file layout and code-generation tooling remain TBD. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline and DEC-010. |
| DEL-02-04-REQ-12 | JSON payload hashes used by plugin/adapter manifests, provenance records, or reproducibility artifacts shall use the accepted canonical JSON/JCS-compatible hash basis when hashing is required. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 and 8.2 Resolved architecture baseline. |
| DEL-02-04-REQ-13 | Rule-pack-facing extension hooks shall remain sandboxed, deterministic, unit-aware, and incapable of arbitrary filesystem or network access. Exact expression grammar/library remains TBD and is not resolved by this deliverable. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-RULE-2. SourcePath: `docs/PRD.md`; SectionRef: 12.3 Rule-Pack Evaluator. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-006. |
| DEL-02-04-REQ-14 | Plugin/adapter contract verification shall include layered checks for schema conformance, unit safety, provenance, diagnostics, protected-content gates, and relevant adapter/plugin regression behavior. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-08. SourcePath: `docs/VALIDATION_STRATEGY.md`; SectionRef: 2. Benchmark families and 4. Release gate. |
| DEL-02-04-REQ-15 | The contract shall explicitly record remaining implementation-level TBDs: public API transport, concrete import/export formats, exact sandbox mechanism, detailed permission names, exact dependency versions, and CI thresholds. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline, OI-004, and DEC-012. |
| DEL-02-04-REQ-16 | Public plugin manifest/interface documentation shall include a placeholder concept inventory before contract issuance. Exact JSON Schema file layout, field names, transport binding, and code-generation tooling remain TBD until a human/architecture review resolves them. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline and DEC-010. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: DEC-012. |
| DEL-02-04-REQ-17 | Exact extension-point registry entries, permission taxonomy names, sandbox mechanism, and approval path shall remain `TBD` until a documented human/project authority or security/architecture review records a decision; the interim contract shall not grant plugin capabilities by default. | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and DEC-012. |
| DEL-02-04-REQ-18 | Plugins/adapters shall not receive telemetry-facing data or transmit private project, rule-pack, component, material, or calculation-result data by default. Any later plugin exposure to telemetry-facing data remains TBD and requires explicit privacy/security ruling and verification coverage. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-PRIV-1 and OPS-K-PRIV-2. SourcePath: `docs/PRD.md`; SectionRef: 18.2 Telemetry. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: OI-008. |

## Standards

No protected engineering code, standards clauses, tables, or examples are incorporated into this deliverable.

Applicable internal standards and baselines:

- `docs/CONTRACT.md` invariant catalog, especially OPS-K-IP-*, OPS-K-DATA-*, OPS-K-AUTH-*, OPS-K-UNIT-1, OPS-K-RULE-2, OPS-K-REPORT-*, OPS-K-PRIV-*, and OPS-K-AGENT-*.
- `docs/TYPES.md` for deliverable types, analysis-status vocabulary, epistemic labels, and data provenance labels.
- `docs/SPEC.md` for layer responsibilities, rule-pack evaluator constraints, diagnostics classes, reporting/audit, and Type 2 acceptance semantics.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data and provenance policy.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for SOW-038, OBJ-009, SCA-001 basis IDs, JSON Schema 2020-12, schema-first envelopes, canonical JSON/JCS-compatible hashing, and remaining-TBD boundaries.

External implementation standards:

- JSON Schema 2020-12 is the accepted public schema/interchange baseline by SCA-001. This document does not reproduce the JSON Schema specification text. SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| REQ-01, REQ-09, REQ-10 | Architecture review confirms plugin/adapter calls enter through governed service/domain contracts and cannot bypass validation, diagnostics, envelopes, or report controls. |
| REQ-02 | Unit schema tests and dimensional validation tests reject incompatible units and surface missing-unit diagnostics. |
| REQ-03, REQ-04 | Provenance-field schema validation and contribution-review checklist confirm required source, license, contributor, redistribution, and review metadata. |
| REQ-05, REQ-06 | Protected-content/provenance gate and quarantine procedure exercise public and private data paths with invented non-code fixtures only. |
| REQ-07 | Diagnostic envelope tests confirm required fields and warning class propagation for plugin/adapter failures. |
| REQ-08 | Report/template and message review confirms no certification, approval, endorsement, or code-compliance claims. |
| REQ-11, REQ-12 | Schema validation and canonicalization/hash tests verify manifest and envelope stability once concrete schemas exist. |
| REQ-13 | Security tests for any rule-pack-facing hook confirm no arbitrary code, filesystem, or network access unless a later human-approved sandbox design allows a bounded capability. |
| REQ-14 | Layered test plan maps plugin/adapter behavior to schema, unit, provenance, diagnostics, protected-content, and regression gates. |
| REQ-15, REQ-17 | Review confirms implementation-level TBDs remain visible and are not silently resolved or treated as approved capabilities. |
| REQ-16 | Manifest/interface documentation review confirms each concept slot is present as an approved field or explicit `TBD` placeholder. |
| REQ-18 | Privacy/security review confirms plugins/adapters do not receive telemetry-facing or private engineering data by default; any exception has an explicit human ruling and test evidence. |

### Verification acceptance criteria and interim gates

| Gate | Applies to | Pass/hold criteria |
|---|---|---|
| Schema/manifest review gate | REQ-11, REQ-12, REQ-16 | Hold until concrete schema layout, field names, transport binding, and code-generation tooling are either approved or carried as explicit `TBD`; pass only with schema validation and canonicalization/hash evidence where hashing is required. |
| Layered plugin/adapter gate | REQ-01 through REQ-14 | Pass only when schema, unit, provenance, diagnostics, protected-content/provenance, and regression evidence is present or the missing evidence is explicitly recorded as `TBD`/open risk for human review. |
| Rule-pack sandbox gate | REQ-13 | Pass only when tests demonstrate deterministic, unit-aware behavior and no arbitrary code execution or unauthorized filesystem/network access. Exact sandbox technology remains TBD until approved. |
| Human-ruling gate | REQ-15, REQ-17, REQ-18 | Hold until open decisions for transport, registry, permission taxonomy, sandbox mechanism, import/export formats, telemetry/private-data exposure, and approval owner are recorded or explicitly deferred. |
| Diagnostic class check | REQ-02, REQ-03, REQ-05, REQ-06, REQ-07, REQ-13 | Pass only when missing units, weak provenance, protected-content risk, assumptions, and rule-blocking outcomes surface through the existing warning/diagnostic classes where applicable; exact severity taxonomy beyond the source-defined classes remains TBD. |

## Documentation

Required documentation artifacts for DEL-02-04:

- Plugin interface specification covering identity, manifest metadata, extension point registration, capability declarations, validation lifecycle, diagnostic envelope behavior, and provenance/data-boundary obligations.
- Sandbox/permission model notes covering deny-by-default posture, capability grant concepts, private data restrictions, network/filesystem/process boundaries, report/export controls, and explicit TBDs.
- Requirement-to-verification matrix, as above, suitable for later PKG-10 implementation and PKG-12 security/privacy review.
- Human-ruling log or future open-issue references for public API transport, exact extension point registry, exact permission taxonomy, import/export format list, and concrete sandbox technology.

Manifest/interface placeholder inventory for later schema work:

| Concept slot | Current status | Evidence |
|---|---|---|
| Plugin identity and version | TBD field names/layout | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: 8.2 Resolved architecture baseline. |
| Extension-point declaration | TBD approved registry names | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-038. |
| Schema/envelope version | TBD field names/layout | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: DEC-010. |
| Capability request/declaration | TBD permission names and approval path | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 and DEC-012. |
| Provenance/data-boundary declaration | Required concept; exact manifest fields TBD | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Required provenance fields. |
| Diagnostics compatibility | Required concept; exact manifest fields TBD | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06. |
| Review/status metadata | Required concept; exact manifest fields TBD | SourcePath: `docs/TYPES.md`; SectionRef: 5. Epistemic labels. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AGENT-4. |
