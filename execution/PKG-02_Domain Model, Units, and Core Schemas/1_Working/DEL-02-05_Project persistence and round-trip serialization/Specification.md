# Specification - DEL-02-05 Project Persistence and Round-Trip Serialization

## Scope

This specification defines the bounded requirements for DEL-02-05: create/open/save/versioned project persistence and deterministic round-trip serialization for project models, unit metadata, loads, rule-pack references, and provenance metadata.

In scope:

- project file schema contract;
- round-trip serialization and deterministic canonicalization behavior for JSON payloads;
- persistence service contract for create/open/save/version validation and migration-status reporting;
- validation and test obligations for schema, round-trip, hash stability, provenance, unit metadata, and protected-content boundaries.

Out of scope:

- numerical solving, stress recovery, GUI views, report rendering, and rule-pack expression evaluation;
- physical project package/container selection, migration framework/tooling, binary asset packaging, exact dependency versions, public API transport, and exact schema file layout, all of which remain TBD under SCA-001;
- any bundled protected standards content, code-specific formulas, tables, allowables, or proprietary project examples.

Sources: `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` PKG-02 and DEL-02-05 rows; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| REQ-02-05-001 | The persistence surface shall support project create, open, save, and version handling. | SOW-050; `docs/PRD.md` section 10 FR-001 |
| REQ-02-05-002 | A project file round trip shall preserve model content, unit metadata, loads, rule-pack references, and provenance metadata without loss. | SOW-050; `docs/PRD.md` section 10 FR-001 |
| REQ-02-05-003 | The project file schema shall align with the machine-readable schema scope for project, model, material, component, load, result, and report schemas where this persistence deliverable references those objects. | SOW-041; `docs/SPEC.md` section 3 |
| REQ-02-05-004 | Public schemas/interchange for this deliverable shall use JSON Schema 2020-12 as the baseline. Exact schema file layout and code-generation tooling are TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-041 and section 8.2 |
| REQ-02-05-005 | JSON persistence payloads shall be deterministic and hash-ready using canonical JSON with JCS-compatible canonicalization. Exact canonicalization library/tooling is TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and section 8.2 |
| REQ-02-05-006 | The schema shall include explicit schema/version metadata and migration status sufficient to detect unsupported, stale, or failed migrations. The migration framework is TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
| REQ-02-05-007 | Persisted numerical and engineering-relevant fields shall carry explicit units or references to a declared unit system sufficient for downstream unit checks. Silent unit defaults are not permitted. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
| REQ-02-05-008 | Provenance metadata shall be preserved for materials, components, SIF/flexibility values, allowables, rule-pack values, and other engineering-reliance data when those records are present in a project. Missing provenance shall be representable as a warning/finding, not silently accepted as complete. | `docs/CONTRACT.md` OPS-K-DATA-3 and OPS-K-DATA-2; `docs/DIRECTIVE.md` section 3; `docs/IP_AND_DATA_BOUNDARY.md` section 4 |
| REQ-02-05-009 | Rule-pack data shall be represented as user/private rule-pack references with version/checksum/source-note capability; the public project schema/tests shall not embed protected rule formulas, allowables, or code-specific values. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` OPS-K-RULE-3 and OPS-K-IP-1; `docs/PRD.md` section 12.4 |
| REQ-02-05-010 | Persistence operations shall preserve application-service, domain-core, storage, schema, validation, and adapter boundaries; adapters/plugins shall not bypass unit, provenance, diagnostics, sandboxing, envelope, report, or public/private data-boundary checks. | `docs/SPEC.md` section 1; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 and AB-00-07 |
| REQ-02-05-011 | Open/save/validate/migrate failures shall be expressible through structured diagnostics/result envelopes carrying code, class, severity, source, affected object, message, remediation, and provenance where applicable. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-06; `docs/SPEC.md` section 7 |
| REQ-02-05-012 | Persistence artifacts shall support reproducibility metadata needed by downstream reports and audit manifests, including model hash and input-manifest compatibility for JSON payloads. | `docs/SPEC.md` section 8; `docs/PRD.md` section 15.3; `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-039 and AB-00-04 |
| REQ-02-05-013 | Private project, material, component, and rule-pack data shall remain local-first and shall not be transmitted or committed publicly by default. | `docs/CONTRACT.md` OPS-K-PRIV-1; `docs/PRD.md` section 18 |
| REQ-02-05-014 | Public fixtures and examples used by this deliverable shall be invented, original, public-domain, or permissively licensed and shall include provenance/redistribution status when data records are present. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5; `docs/VALIDATION_STRATEGY.md` section 5 |
| REQ-02-05-015 | The round-trip test suite shall include schema validation, deterministic reserialization, JSON canonicalization/hash stability, migration-status handling, unit/provenance preservation, private-data boundary checks, and protected-content/provenance gates where relevant. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08; `docs/VALIDATION_STRATEGY.md` sections 2 and 4; `docs/PRD.md` section 22.1 |
| REQ-02-05-016 | No persistence status, field, diagnostic, report hook, or example shall claim professional approval, certification, sealing, authentication, or automatic code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4; `docs/DIRECTIVE.md` section 3 |
| REQ-02-05-017 | The project file schema shall enumerate logical envelope slots for document/schema metadata, project identity, unit-system reference, model payload, load payloads, rule-pack references, provenance/redistribution metadata, diagnostics or migration status, reproducibility metadata, and optional review records. | SOW-050; `docs/SPEC.md` section 3; `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
| REQ-02-05-018 | Round-trip acceptance criteria shall compare semantic equality for model content, unit metadata, loads, rule-pack references, provenance metadata, and reproducibility metadata after parse -> validate/normalize -> serialize -> parse. | SOW-050; `docs/PRD.md` section 10 FR-001; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
| REQ-02-05-019 | Unit round-trip criteria shall distinguish explicit units, declared unit-system references, missing units, and incompatible unit metadata; missing or incompatible units shall produce findings instead of silent defaults. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002; `docs/DIRECTIVE.md` sections 2.2 and 3 |
| REQ-02-05-020 | Hash and reproducibility criteria shall identify the compared JSON payload, any input manifest, and any referenced non-JSON or binary manifest; exact payload partitioning remains TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2; `docs/SPEC.md` section 8; `docs/PRD.md` section 15.3 |
| REQ-02-05-021 | Migration handling shall define status semantics and diagnostics for unsupported, stale, and failed migrations; additional status names and the migration framework/tooling remain TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
| REQ-02-05-022 | The persistence service contract shall define create, open, save, validate, version-check, and migrate-operation behavior in schema-first command/query/result-envelope terms; exact language/API signatures remain TBD. | SOW-050; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and section 8.2; `docs/SPEC.md` section 1 |
| REQ-02-05-023 | Diagnostics emitted by persistence operations shall define deterministic class coverage for schema, migration, unit metadata, provenance, rule-pack reference, IP/data-boundary, private-data, and professional-boundary failures or warnings. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/SPEC.md` section 7; `docs/CONTRACT.md` invariant index |
| REQ-02-05-024 | Rule-pack references persisted in projects shall carry enough metadata to identify private/public status, version/checksum, source note, redistribution status, and missing/private-reference diagnostics without exposing protected rule content in public artifacts. | `docs/SPEC.md` section 6; `docs/PRD.md` sections 12.2 and 12.4; `docs/IP_AND_DATA_BOUNDARY.md` sections 3, 6, and 7 |
| REQ-02-05-025 | Create/open/save behavior shall preserve the local-first/private-data boundary by default; exact enforcement split between persistence operations, export/commit safeguards, and PKG-12 controls remains TBD. | `docs/CONTRACT.md` OPS-K-PRIV-1; `docs/PRD.md` sections 18.1-18.3; `docs/IP_AND_DATA_BOUNDARY.md` section 6 |
| REQ-02-05-026 | Optional human-review records, if stored, shall bind to specific model/rule/report hashes and use authority labels that do not survive content changes and do not imply software certification or automatic code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2; `docs/TYPES.md` section 4 |

### Contract Detail Tables

#### Project Envelope Slots

| Slot | Minimum contract content | Verification note |
|---|---|---|
| `document_metadata` | Document kind, schema version, project/document identity, and version-handling metadata. | Fixture exposes version fields and migration status or diagnostic behavior. |
| `project` | Project identity, design-basis metadata, storage policy/private-data indicators, and report settings where applicable. | Fields align to `Project` object scope in `docs/SPEC.md` section 3; exact field names TBD. |
| `units` | Declared unit system and/or explicit unit metadata on numerical values. | Missing units and incompatible dimensions fail with deterministic findings. |
| `model` | Nodes, elements, sections, materials, components, supports, and references used by the persisted model where present. | Round-trip comparison verifies stable identity and references, not text formatting. |
| `loads` | Load cases and load records with units and source/provenance where needed. | Round-trip comparison verifies load IDs, types, magnitudes, units, and source fields where present. |
| `rule_pack_refs` | Rule-pack ID/name, version, checksum, source note, redistribution/private status, and required-input linkages where present. | Public fixtures reference only invented or permissively licensed rule packs. |
| `provenance` | Source/provenance and redistribution/review metadata for public data records; private/source labels where project records are private. | Public data records follow `docs/IP_AND_DATA_BOUNDARY.md` section 4 fields. |
| `diagnostics_and_migration` | Validation findings, migration status, and structured result-envelope diagnostics. | Unsupported, stale, and failed migration cases have deterministic diagnostics. |
| `reproducibility` | Model hash, input-manifest compatibility, solver/report linkage where applicable, and rule-pack checksum references. | Hash inputs are identified and stable for canonical JSON payloads. |
| `review_records` | Optional human review labels and hashes for project-specific acceptance records. | Any review label remains human authority only and cannot be generated as code compliance. |

#### Round-Trip Equality Criteria

| Category | Equal after round trip when | Failure examples |
|---|---|---|
| Model content | Stable object IDs, object types, references, coordinates/geometry fields, component/material/section references, and support/load associations remain semantically equivalent. | Missing object, changed reference, changed engineering-relevant value, or unreported schema loss. |
| Unit metadata | Explicit units or declared unit-system references remain present and dimensionally compatible. | Missing unit, incompatible unit dimension, or silent default insertion. |
| Loads | Load-case IDs, load types, magnitudes, units, and source/provenance fields where present are preserved. | Changed magnitude, missing units, lost load case, or lost source note. |
| Rule-pack references | ID/name, version, checksum, source note, private/public or redistribution status, and required-input linkages remain present. | Expanded protected rules in public artifacts, checksum loss, or private/public status loss. |
| Provenance metadata | Provenance fields required for engineering-reliance data remain present or generate findings when missing. | Lost source, lost redistribution status, or missing provenance accepted as complete. |
| Reproducibility metadata | Canonical JSON payload hash basis and input-manifest compatibility are reproducible for the selected payload/manifest partition. | Hash includes volatile/session fields without approved basis, or payload partition is undocumented. |

#### Migration Status Semantics

| Status / case | Required handling | Open detail |
|---|---|---|
| Supported/current schema | Validate and round-trip without migration diagnostics. | Exact status label TBD. |
| Unsupported schema | Return a structured diagnostic and avoid silent coercion. | Source supports unsupported migration detection; exact code TBD. |
| Stale schema | Report migration-needed status or diagnostic before accepting as current. | Migration tool behavior TBD. |
| Failed migration | Return a structured failure diagnostic with affected object/source where available. | Rollback/transaction behavior TBD. |
| Newer-than-supported schema | TBD; likely a structured unsupported-version diagnostic, but this is not explicitly settled by the current sources. | Human/architecture ruling needed. |

#### Persistence Service Operations

| Operation | Minimum inputs | Minimum outputs / diagnostics | Notes |
|---|---|---|---|
| Create project | Project identity, unit system, storage/private-data policy, optional template/reference selection. | Versioned project envelope or diagnostics/result envelope. | Exact constructor/API shape TBD. |
| Open project | Project artifact reference and caller context. | Parsed project envelope, validation result, migration status, diagnostics. | Must not bypass schema, unit, provenance, or data-boundary checks. |
| Save project | Validated project envelope, target artifact reference, canonicalization/hash options. | Saved artifact reference, version metadata, hash/manifest evidence, diagnostics. | Physical container remains TBD. |
| Validate project | Project envelope or artifact reference. | Schema, unit, provenance, rule-pack-reference, private-data, and protected-content diagnostics. | Diagnostics use structured result-envelope fields. |
| Version check | Project schema/version metadata. | Current/stale/unsupported/failed status or diagnostic. | Exact status labels TBD. |
| Migrate project | Source project, target schema version, migration policy. | Migrated project or failed migration diagnostics. | Migration framework/tooling TBD. |

#### Diagnostic Class Coverage

| Class / namespace | Applies when | Source basis |
|---|---|---|
| `SCHEMA_VALIDATION` (PROPOSAL) | Project envelope or object payload violates schema. | SOW-041; JSON Schema 2020-12 baseline. |
| `MIGRATION` (PROPOSAL) | Schema version is unsupported, stale, or migration fails. | AB-00-04; OI-011. |
| `UNIT_METADATA` (PROPOSAL) | Unit metadata is missing or incompatible. | OPS-K-UNIT-1; PRD FR-002. |
| `PROVENANCE_WARNING` | Source/provenance is missing or weak. | `docs/SPEC.md` section 7; OPS-K-DATA-2/3. |
| `RULE_CHECK_BLOCKING` | Mechanics may exist, but required user/rule-pack data is missing. | `docs/SPEC.md` section 7; PRD section 14.4. |
| `IP_BOUNDARY_WARNING` | Public contribution, fixture, report hook, or export may expose protected/private content. | `docs/SPEC.md` section 7; IP/data-boundary policy. |
| `PRIVATE_DATA` (PROPOSAL) | An operation would transmit, commit, export, or publish private project/rule/library data by default. | OPS-K-PRIV-1; PRD section 18. |
| `PROFESSIONAL_BOUNDARY` (PROPOSAL) | A status or artifact could imply certification, sealing, approval, authentication, or automatic code compliance. | OPS-K-AUTH-1; `docs/TYPES.md` section 4. |

PROPOSAL: Exact diagnostic code prefixes remain TBD; the table above records required deterministic coverage, not final code names.

## Standards

| Standard / basis | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Public schema/interchange baseline for the project file schema. | Required by SCA-001; exact schema file layout TBD. |
| Canonical JSON / JCS-compatible canonicalization | Hash basis for JSON payloads. | Required by SCA-001; exact library/tooling TBD. |
| OpenPipeStress invariant catalog | Binding constraints for IP boundary, unit safety, provenance, professional boundary, private data, and agent outputs. | Required by `docs/CONTRACT.md`. |
| OpenPipeStress data-boundary policy | Controls public/private data handling and provenance fields. | Required by `docs/IP_AND_DATA_BOUNDARY.md`. |

No piping design code or protected standards dataset is a governing input to this deliverable. The persistence schema may carry user-supplied rule-pack references and provenance, but it must not bundle protected code content.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| REQ-02-05-001, REQ-02-05-006, REQ-02-05-010, REQ-02-05-011, REQ-02-05-021, REQ-02-05-022, REQ-02-05-023 | Persistence service contract review: create/open/save/validate/version-check/migrate operations have defined inputs, outputs, diagnostics/result envelopes, boundary checks, and migration-status behavior. |
| REQ-02-05-002, REQ-02-05-005, REQ-02-05-018, REQ-02-05-020 | Round-trip tests: parse a project fixture, validate/normalize, serialize canonically, reparse, and compare category-level semantic equality plus canonical JSON/hash behavior. |
| REQ-02-05-003, REQ-02-05-004, REQ-02-05-017 | Schema validation: fixtures validate against JSON Schema 2020-12 contracts; invalid fixtures fail with deterministic diagnostics; project-envelope slots are covered or marked TBD. |
| REQ-02-05-007, REQ-02-05-008, REQ-02-05-019 | Unit/provenance tests: explicit units and declared unit systems survive round trip; missing/incompatible units and missing/weak provenance produce findings rather than silent defaults. |
| REQ-02-05-009, REQ-02-05-024 | Rule-pack reference tests: ID/name, version, checksum, source note, and private/public or redistribution status survive round trip; public fixtures do not expand protected rule content. |
| REQ-02-05-013, REQ-02-05-014, REQ-02-05-016, REQ-02-05-025, REQ-02-05-026 | Protected-content/private-data/professional-boundary review: fixtures contain invented/permissive data only; private references are not publicly committed or transmitted by default; no compliance/certification status appears; optional review records bind to hashes if present. |
| REQ-02-05-012, REQ-02-05-015, REQ-02-05-020 | Reproducibility tests: model-hash/input-manifest compatibility is stable for deterministic JSON payloads; payload/manifest partition is documented; protected-content/provenance gates run where public examples/templates are present. |

## Documentation

The deliverable should leave or update these records:

- project file schema documentation, including schema version, migration status semantics, canonicalization/hash basis, and unresolved implementation TBDs;
- persistence service contract documentation for create/open/save/version validation and diagnostics/result envelopes;
- round-trip test documentation naming fixtures, expected preserved fields, semantic equality checks, canonical hash expectations, and protected-content/provenance gates;
- diagnostic taxonomy coverage and migration-status case matrix, with exact code names/status names marked TBD where not approved;
- fixture/public-data review evidence covering provenance, redistribution status, protected-content screening, private-data handling, and professional-boundary language;
- open decisions for physical project package/container, migration framework/tooling, binary asset packaging, exact schema file layout, dependency/tool selections, and hash payload partitioning;
- no protected standards excerpts, proprietary data, or code-compliance/certification claims.
