# Datasheet - DEL-02-05 Project Persistence and Round-Trip Serialization

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-02-05 |
| Package ID | PKG-02 |
| Package | Domain Model, Units, and Core Schemas |
| Deliverable name | Project persistence and round-trip serialization |
| Type | DATA_MODEL_CHANGE |
| Scope items | SOW-050, SOW-041 |
| Objectives | OBJ-001, OBJ-012 |
| Context envelope | M |
| Anticipated artifacts | project file schema; round-trip tests; persistence service contract |
| Source basis | `_CONTEXT.md` revision 0.4; `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4; `docs/_Registers/*.csv`; `docs/CONTRACT.md`; SCA-001 basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08 |

## Attributes

| Attribute | Required / current value | Source |
|---|---|---|
| Persistence purpose | Create, open, save, and version project files. | `docs/_Registers/Deliverables.csv` row DEL-02-05; `docs/PRD.md` section 10 FR-001 |
| Round-trip preservation target | Model, units, loads, rule-pack references, and provenance metadata must round-trip without loss. | `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-050; `docs/PRD.md` section 10 FR-001 |
| Schema baseline | JSON Schema 2020-12 is the public schema/interchange baseline. | `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-041 and section 8.2 |
| Persistence encoding baseline | Versioned, JSON-schema-governed canonical JSON for JSON payloads. | `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 |
| Hash basis | JSON payload hashes use canonical JSON with JCS-compatible canonicalization. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and section 8.2 |
| Project package/container | TBD; SCA-001 leaves the physical project package/container unresolved. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
| Migration mechanism | Migration-aware persistence is required; migration framework/tooling is TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04, section 8.2, OI-011 |
| Core data objects affected | Project, Model, LoadCase, RulePack references, Report/audit metadata, and provenance-bearing domain records. | `docs/SPEC.md` section 3 |
| Application boundary | Storage/persistence must preserve domain invariant enforcement; adapters cannot bypass unit, provenance, diagnostics, or public/private data-boundary checks. | `docs/SPEC.md` section 1; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02 and AB-00-07 |
| Diagnostic behavior | Persistence validation, migration, and round-trip failures should be returned through structured diagnostics/result envelopes. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 and AB-00-06; `docs/SPEC.md` section 7 |
| Test evidence | Round-trip, schema, unit/provenance, hash/canonicalization, migration-status, and protected-content/provenance gates are expected. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-08; `docs/VALIDATION_STRATEGY.md` sections 2 and 4 |

## Conditions

- The deliverable is bounded to the PKG-02 persistence and serialization surface. It must not implement numerical solving, GUI views, rule-pack evaluation logic, reporting, external API transport, or local/private storage policy beyond the persistence contract needed by this deliverable. Source: `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` PKG-02 row.
- The public schema and tests must not introduce protected standards text, protected tables, proprietary values, copied formulas, or commercial examples. Source: `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-5.
- Project files may contain private project/rule data in user-controlled contexts, but public fixtures must use invented, original, or permissively licensed data with documented provenance. Source: `docs/IP_AND_DATA_BOUNDARY.md` sections 2, 4, and 6; `docs/CONTRACT.md` OPS-K-PRIV-1.
- All persisted numerical and engineering-relevant values must be unit-aware or carry enough unit metadata for downstream dimensional checks. Source: `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002.
- Software output and persistence metadata must not claim code compliance, certification, sealing, approval, or professional reliance. Source: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4.

## Construction

The deliverable construction target is a three-artifact kit:

| Artifact | Minimum construction expectation | Source |
|---|---|---|
| Project file schema | Define a versioned project document contract covering project identity, schema version, unit system, model payload, load payloads, rule-pack references, provenance metadata, and validation/migration status. Exact schema file layout is TBD. | `docs/_Registers/Deliverables.csv` row DEL-02-05; `docs/SPEC.md` section 3; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 |
| Round-trip tests | Verify parse -> normalize/validate -> serialize -> parse cycles preserve required project content and produce deterministic canonical JSON/hash behavior for JSON payloads. Exact test harness details are TBD. | `docs/PRD.md` section 10 FR-001; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and AB-00-08 |
| Persistence service contract | Define application-service operations for create/open/save/version validation, migration-status reporting, diagnostics/result envelopes, and data-boundary checks. Exact interface language/API shape is TBD. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-02, AB-00-03, AB-00-06, AB-00-07 |

ASSUMPTION: The persisted project document will be treated as durable project state, while session-only UI state and job-progress state remain outside this deliverable unless later injected by an applicable GUI-state brief.

### Persisted Project Envelope Inventory

This inventory is a logical schema contract checklist, not the final schema file layout. Exact schema file layout, code-generation tooling, physical container, and migration implementation remain TBD under SCA-001.

| Logical slot | Minimum persisted meaning | Source | Open detail |
|---|---|---|---|
| Document identity and schema metadata | Document kind, schema version, project identifier/name, and version-handling metadata. | SOW-050; `docs/PRD.md` section 10 FR-001; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 | Exact field names and schema file layout TBD. |
| Project basis and storage policy | Project/design-basis metadata, storage/private-data policy flags, and report settings where present. | `docs/SPEC.md` section 3 `Project`; `docs/PRD.md` Appendix A; `docs/CONTRACT.md` OPS-K-PRIV-1 | Exact local-first enforcement fields TBD. |
| Unit-system reference | Declared unit system or explicit unit metadata sufficient for downstream unit checks. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/PRD.md` section 10 FR-002; SOW-025 | Missing or incompatible unit metadata must become diagnostics, not defaults. |
| Model payload | Model objects such as nodes, elements, sections, materials, components, supports, and related references where present. | `docs/SPEC.md` section 3; `docs/PRD.md` Appendix A | Field-level object schemas remain coordinated with DEL-02-01. |
| Load payloads | Load cases and load records with units and source/provenance where engineering reliance is affected. | SOW-050; `docs/SPEC.md` section 3 `LoadCase`; `docs/DIRECTIVE.md` section 2.1 | Exact load object schema remains coordinated with load deliverables. |
| Rule-pack references | Rule-pack ID/name, version, checksum, source note, redistribution/private status, and required-input linkage where present. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` OPS-K-RULE-3; `docs/PRD.md` section 12.2 | Public fixtures must not embed protected rule formulas or allowables. |
| Provenance and redistribution metadata | Source/provenance, license or redistribution status, contributor/review fields where public data records are present. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/CONTRACT.md` OPS-K-DATA-3 | Private records may use private/source labels; public records need complete review metadata. |
| Validation, diagnostics, and migration status | Validation findings, migration status, and structured diagnostics/result-envelope fields. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03, AB-00-04, AB-00-06; `docs/SPEC.md` section 7 | Status values beyond unsupported, stale, and failed migration are TBD. |
| Reproducibility metadata | Model hash, input-manifest compatibility, rule-pack checksum references, and canonical JSON/JCS-compatible hash basis for JSON payloads. | `docs/SPEC.md` section 8; `docs/PRD.md` section 15.3; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2 | Exact payload/manifest partition remains TBD. |
| Review records, if present | Human review labels and hashes tied to specific model/rule/report content without automatic compliance claims. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AUTH-2; `docs/TYPES.md` section 4 | Review-record schema is optional and final authority labels remain TBD. |

## References

- `_CONTEXT.md` revision 0.4 for deliverable identity, SOW/objective mapping, accepted architecture basis IDs, and write context.
- `_DEPENDENCIES.md` for declared dependency status; no human-owned upstream/downstream dependency list was provided.
- `docs/_Registers/Deliverables.csv` row DEL-02-05.
- `docs/_Registers/ScopeLedger.csv` rows SOW-050 and SOW-041.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-02-05.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4, especially SOW-041, SOW-050, PKG-02, DEL-02-05, section 8.1, section 8.2, and OI-011.
- `docs/CONTRACT.md` invariant catalog.
- `docs/TYPES.md` sections 3-8.
- `docs/SPEC.md` sections 1, 3, 7, 8, 9, and 11.
- `docs/DIRECTIVE.md` sections 2-5.
- `docs/IP_AND_DATA_BOUNDARY.md` sections 2-7.
- `docs/PRD.md` section 10 FR-001/FR-002, section 11.8, section 12.4, section 15.3, section 18, section 19, and section 22.1.
- `docs/VALIDATION_STRATEGY.md` sections 2, 4, and 5.
