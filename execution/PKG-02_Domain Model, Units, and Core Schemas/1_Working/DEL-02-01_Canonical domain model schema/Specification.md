# Specification: DEL-02-01 Canonical domain model schema

## Scope

This specification governs the DEL-02-01 document-level contract for a canonical domain model schema in PKG-02. The deliverable defines the intended public schema boundary for project, model, node, element, material, component, load, result, and report entities. It also records constraints that a future implementation of `schemas/model.schema.yaml` must satisfy.

In scope:

- canonical object families and common schema definitions required by SOW-041;
- JSON Schema 2020-12 as the public schema/interchange baseline;
- unit, provenance, diagnostics, status, reproducibility, and data-boundary requirements that apply to model-schema records;
- verification expectations for schema validation, examples, and round-trip behavior.

Out of scope:

- numerical solver implementation;
- GUI model tree/property editor behavior;
- detailed material/component library population;
- rule-pack evaluator implementation or code-specific rule content;
- report generator implementation;
- physical project-file package/container selection;
- any code-compliance, certification, approval, or professional-authentication claim.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-02-01-01 | The public model schema shall use JSON Schema 2020-12 as its baseline unless a later human-approved architecture change supersedes SCA-001. | `docs/_Registers/ScopeLedger.csv` SOW-041; `docs/_Decomposition/SOFTWARE_DECOMP.md` Section 8.2 | Schema declares JSON Schema 2020-12 dialect; review verifies no conflicting schema baseline. |
| REQ-02-01-02 | The schema shall define or reference canonical records for Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, and Report. | `_CONTEXT.md` Description; `docs/SPEC.md` Section 3; `docs/DIRECTIVE.md` Section 2.1 | Schema coverage review maps each required family to a definition or justified reference. |
| REQ-02-01-03 | Addressable schema records shall carry stable identifiers and use explicit references rather than implicit positional coupling where downstream editing, solving, reporting, or audit depends on identity. | `docs/TYPES.md` Section 2; `docs/DIRECTIVE.md` Section 2.1 | Schema review verifies ID/reference fields for addressable object families. |
| REQ-02-01-04 | Dimensional values shall be unit-aware; missing or incompatible units shall be representable as validation findings rather than silently accepted defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2; `docs/PRD.md` FR-002; `docs/SPEC.md` Sections 7 and 9 | Schema tests reject incompatible/missing units where required; findings are explicit. |
| REQ-02-01-05 | Engineering-reliance data such as materials, components, allowables, SIF/flexibility inputs, loads, rule references, reports, and imported records shall carry source/provenance and redistribution/private-status metadata where applicable. | `docs/CONTRACT.md` OPS-K-DATA-3 and OPS-K-IP-2; `docs/IP_AND_DATA_BOUNDARY.md` Section 4; `docs/PRD.md` Sections 13.4 and 13.5 | Schema includes provenance/status fields or reusable definitions; protected-content review verifies no protected data is bundled. |
| REQ-02-01-06 | The schema shall not include protected standards text, tables, figures, copied code formulas, material allowable tables, SIF/flexibility tables, protected dimensional tables, proprietary vendor catalogs, or private user data as public defaults. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` Sections 2 and 3; `docs/PRD.md` Section 17.1 | Protected-content/provenance gate scans schemas, fixtures, and examples. |
| REQ-02-01-07 | The schema shall preserve the distinction among mechanics-solved data, user-rule-check data, and human professional approval records; it shall not define automatic `CODE_COMPLIANT` status. | `docs/TYPES.md` Section 4; `docs/DIRECTIVE.md` Section 2.2; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 | Status enum review confirms allowed statuses and excludes automatic compliance statuses. |
| REQ-02-01-08 | Result and report-facing records shall be compatible with diagnostic/result-envelope data carrying code, class, severity, source, affected object, message, remediation, and provenance. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` Sections 7 and 8 | Schema review verifies diagnostic envelope fields or references; tests cover each warning class where applicable. |
| REQ-02-01-09 | Persisted JSON payloads governed by the schema shall be compatible with deterministic, versioned, migration-aware, round-trip-testable persistence and JCS-compatible hashing where JSON payload hashes are used. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and Section 8.2; `docs/PRD.md` FR-001 and Section 15.3 | Round-trip fixture test preserves model, units, loads, rule references, and provenance metadata; hash basis documented where used. |
| REQ-02-01-10 | Adapters, plugins, and import/export consumers shall not be able to bypass unit, provenance, diagnostics, validation, or public/private data-boundary constraints expressed by the schema. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 and AB-00-07; `docs/SPEC.md` Section 1 | API/import/export contract review confirms schema validation remains mandatory at boundaries. |
| REQ-02-01-11 | Public examples or fixtures associated with the schema shall use invented, public-domain, original, or permissively licensed data with provenance; code-specific examples remain private/user-supplied. | `docs/CONTRACT.md` OPS-K-RULE-1; `docs/IP_AND_DATA_BOUNDARY.md` Sections 2 and 6; `docs/PRD.md` Sections 7.5 and 17.1 | Example-data review verifies provenance labels and absence of protected/code-specific content. |
| REQ-02-01-12 | Schema acceptance shall include layered checks appropriate to schemas, units, provenance, protected-content boundaries, deterministic round trips, and downstream result/report compatibility. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08; `docs/SPEC.md` Sections 9 and 11 | Test evidence or review record links each check to a schema artifact or documented deferral. |

## Object-Family Coverage Crosswalk

This crosswalk is acceptance-supporting planning content for the future `schemas/model.schema.yaml`; it is not evidence that the schema file has already been implemented. Any new canonical vocabulary still requires the anticipated `docs/TYPES.md` update or a human ruling before it is treated as frozen.

| Family or record group | Expected schema treatment | Source basis | Pass 3 disposition |
|---|---|---|---|
| Project | Define a root/project record with stable identity, units, storage/privacy posture, rule-pack references, report settings, and persistence hooks where applicable. | `_CONTEXT.md` Description; `docs/SPEC.md` Section 3; `docs/PRD.md` FR-001 and Appendix A | Required coverage. |
| Model | Define the model container and collections for nodes, elements, components, supports, loads, libraries, results, and report-facing references. | `docs/SPEC.md` Section 3; `docs/DIRECTIVE.md` Section 2.1 | Required coverage. |
| Node and Element | Define addressable records for coordinates/DOF context and analytical member connectivity, with explicit references rather than positional coupling. | `docs/SPEC.md` Sections 3 and 4.1; `docs/TYPES.md` Section 2 | Required coverage. |
| Material and Component | Define reusable records or references with source/provenance, redistribution/private status, and unit-bearing properties where engineering reliance may be affected. | `docs/SPEC.md` Section 3; `docs/PRD.md` Sections 13.1, 13.3, and 13.4; `docs/CONTRACT.md` OPS-K-DATA-3 | Required core records; detailed library population remains outside DEL-02-01. |
| Section and Support/Restraint | Define shared references or common records needed by the canonical model; keep detailed calculation behavior, library content, and nonlinear solver behavior in their owning deliverables. | `docs/SPEC.md` Sections 3 and 4.4; `docs/PRD.md` Sections 11.4 and 13.2 | Adjacent coverage required at the reference/common-record level; detailed ownership remains `TBD` where not explicit. |
| LoadCase, Combination, and Load umbrella | Treat `LoadCase` as primitive loading and `Combination` as algebraic combination records; treat `Load` in register prose as the umbrella for load-related schema coverage until vocabulary is frozen. | `_CONTEXT.md` Description; `docs/SPEC.md` Sections 3 and 5; `docs/PRD.md` Sections 10 and 11.6 | Required coverage with no public code-specific combination defaults. |
| RulePack reference | Define references, redistribution/private status, checksum/source notes, and required-input links; do not define protected rule formulas or evaluator internals here. | `docs/SPEC.md` Section 6; `docs/CONTRACT.md` OPS-K-RULE-3; `docs/PRD.md` Sections 12.1 through 12.4 | Reference-only coverage for DEL-02-01. |
| Result | Define result-facing records compatible with mechanical outputs, warning classes, diagnostics, affected-object references, provenance, and authority/status boundaries. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-06; `docs/SPEC.md` Sections 7 and 8 | Required coverage. |
| Report | Define report-facing records for input manifest, model hash/checksum references, warnings, assumptions, provenance summary, limitations, and professional-boundary notices. | `docs/SPEC.md` Section 8; `docs/PRD.md` Section 15 | Required coverage; report rendering remains outside DEL-02-01. |

## Acceptance Crosswalks

### Unit Applicability

| Field class | Minimum acceptance evidence | Source basis |
|---|---|---|
| Coordinates, geometry, dimensions, section properties, stiffnesses, weights, pressures, temperatures, and imposed displacements/rotations | Schema fields carry units or explicit unit references; missing/incompatible units can be reported as validation findings. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` FR-002; `docs/SPEC.md` Sections 3, 5, and 9 |
| Loads, load cases, combinations, results, reactions, forces, moments, and stresses | Unit context is preserved through input, result, and report-facing records. | `docs/PRD.md` FR-002 and FR-016; `docs/SPEC.md` Sections 5 and 8 |
| Dimensionless statuses, labels, identifiers, provenance records, and notices | Schema distinguishes non-dimensional metadata from dimensional quantities rather than forcing meaningless units. | `docs/TYPES.md` Sections 2, 4, 5, and 7 |

### Provenance And Status Applicability

| Record family | Minimum acceptance evidence | Source basis |
|---|---|---|
| Public data records, examples, and fixtures | Reusable provenance/status fields cover source name, source location, source license, contributor, contributor certification, redistribution status, and review status, or the review record documents an explicit deferral. | `docs/IP_AND_DATA_BOUNDARY.md` Section 4; `docs/CONTRACT.md` OPS-K-IP-2 |
| Materials, components, sections, SIF/flexibility inputs, allowables, loads, rule references, reports, and imported records | Source/provenance and redistribution/private-status metadata are available where engineering reliance or public redistribution may be affected. | `docs/CONTRACT.md` OPS-K-DATA-3; `docs/PRD.md` Sections 13.1 through 13.5 |
| Analysis and authority states | Status values preserve mechanics-solved, user-rule-checked, and human-approved-for-project distinctions, and exclude automatic code-compliance statuses. | `docs/TYPES.md` Section 4; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03; `docs/CONTRACT.md` OPS-K-AUTH-1 |

### Diagnostics, Results, And Reports

| Envelope field or concept | Result schema expectation | Report schema expectation | Source basis |
|---|---|---|---|
| code, class, severity | Diagnostic/result record can carry machine-readable code/class/severity and supported warning classes. | Report record can disclose warning classes and blocking/nonblocking status without converting them into compliance claims. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` Section 7 |
| source and affected object | Diagnostic/result record references the emitting subsystem/source and the affected model object where applicable. | Report record can trace warnings, assumptions, and missing data back to model objects or source notes. | `docs/SPEC.md` Sections 7 and 8; `docs/PRD.md` Section 15.1 |
| message and remediation | Diagnostic/result record can carry human-readable message and remediation/next-action text. | Report record can include limitations, assumptions, and warnings without embedding protected standards content. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/PRD.md` Sections 15.1 and 15.2 |
| provenance, input manifest, and hash/checksum references | Result record preserves provenance and reproducibility links where available. | Report record can include input manifest, model hash/checksum references, rule-pack checksum, provenance summary, and professional-boundary notice. | `docs/SPEC.md` Section 8; `docs/PRD.md` Sections 15.1 and 15.3 |

### Hash And Persistence Boundary

- When JSON payload hashes are used, the review evidence must identify the payload boundary being hashed and confirm the canonical JSON/JCS-compatible basis from AB-00-04. If no hash is generated for a schema path or fixture, record the item as a documented deferral rather than implying coverage.
- DEL-02-01 may define schema compatibility hooks for version, migration status, canonicalization/hash metadata, units, loads, rule-pack references, diagnostics, and provenance. Physical project package/container choice and migration implementation remain tied to DEL-02-05 or a later human-approved architecture decision.

## Standards

| Standard or governing basis | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Public schema/interchange baseline for `schemas/model.schema.yaml` | Required by SOW-041 and SCA-001 |
| Canonical JSON / JCS-compatible hashing | Applies where JSON payload hashes are generated for reproducibility/audit | Required by AB-00-04 for hashed JSON payloads; exact implementation details TBD |
| OpenPipeStress invariants in `docs/CONTRACT.md` | Binding constraints for IP boundary, data provenance, units, authority, reports, privacy, and agents | Required |
| OpenPipeStress vocabulary in `docs/TYPES.md` | Stable IDs, analysis statuses, epistemic labels, and provenance labels | Required unless superseded by human-approved change |
| Protected engineering standards/codes | May be referenced as user/private source context only; must not be embedded as public schema content or examples | Public redistribution prohibited without explicit rights |

## Verification

Minimum verification expectations for the eventual schema artifact:

- JSON Schema dialect check confirms JSON Schema 2020-12.
- Coverage check maps Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, and Report to schema definitions or justified references.
- Object-family coverage check uses the crosswalk above to classify required coverage, adjacent reference/common-record coverage, and documented deferrals.
- Unit-field checks confirm dimensional values carry units or explicit unit references.
- Provenance checks confirm engineering-reliance and imported/private data can carry source, license/redistribution, contributor/review, and privacy status.
- Protected-content review confirms no standards text, protected tables, code formulas, proprietary values, or private user data are bundled.
- Status/authority review confirms no automatic `CODE_COMPLIANT` or certification-like status exists.
- Round-trip fixture check confirms model data, units, loads, rule-pack references, diagnostics, hashes/checksums, and provenance metadata can survive serialization without loss.
- Hash-scope review confirms each JSON payload hash has a documented payload boundary and canonicalization basis, or a visible deferral.
- Diagnostics check confirms warning/result envelope compatibility.
- Fixture review confirms each public example or validation fixture has a provenance manifest and protected-content disposition.
- Documentation review confirms TBDs and assumptions are visible.

## Documentation

Required or expected records:

- `schemas/model.schema.yaml` as the future primary schema artifact. This path is outside the current four-document write scope.
- `docs/TYPES.md` update if new canonical type names, statuses, provenance labels, or schema vocabulary are introduced. This path is outside the current four-document write scope.
- Schema validation/test evidence, including object-family coverage, unit applicability, provenance/status applicability, diagnostic/result/report mapping, hash-scope decisions or deferrals, fixture provenance manifests, and protected-content gates when implementation proceeds.
- Human review notes for schema file layout, code-generation tooling, `$id` URI, migration metadata, and any objective-mapping discrepancy recorded in `Guidance.md`.
- `_run_records/TASK_RUN_*.md` for document drafting/enrichment runs.
