# Datasheet: DEL-15-02 Target mapping and unsupported-behavior contract

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-15-02 |
| Name | Target mapping and unsupported-behavior contract |
| Package ID | PKG-15 |
| Package name | Handoff and External Prover Workflow |
| Type | API_CONTRACT |
| Scope coverage | SOW-074 |
| Objective support | OBJ-017 |
| Anticipated artifacts | `schemas/target_mapping.schema.json`; provider-neutral unsupported behavior taxonomy |
| Source basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `Dependencies.csv` |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Deliverable purpose | Defines metadata for how internal entities map to target fields and how unsupported or approximate target behavior is flagged. | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-15-02 row |
| Package purpose | Implements handoff package structure, manifests, target mapping metadata, unsupported-target flags, and non-authoritative external-prover metadata boundaries. | `_CONTEXT.md` Package Reference; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-15 row |
| Required handoff contents in scope | Schema-compliant handoff packages include model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, and unsupported-target flags. | `_CONTEXT.md` Scope Detail; `docs/_Registers/ScopeLedger.csv` SOW-074 |
| Target-specific commercial parsers | Deferred. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-074 note; `docs/_Registers/ScopeLedger.csv` SOW-074 note |
| External professional approval state | Excluded from this package. | `_CONTEXT.md` Package Exclusions; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-15 exclusions |
| Target surfaces | OI-015 names initial export and target surfaces; concrete mappings, target field coverage, target-specific taxonomy extensions, and target-specific implementation remain gated by DEL-17-01 and DEL-17-02. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |
| Canonical package container | TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |
| Target-specific mapping strategy | TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-015 |

## Conditions

| Condition | Value | Source |
|---|---|---|
| Units | Exports must remain unit-aware and dimensionally checked. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/DIRECTIVE.md` Principles |
| Missing values | Missing solve-required or rule-check-required values are explicit findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` Principles |
| Provenance | Reliance-affecting data carries source/provenance fields and review status. | `docs/CONTRACT.md` OPS-K-IP-2; `docs/CONTRACT.md` OPS-K-DATA-3; `docs/TYPES.md` Provenance |
| Private data boundary | Private project, material, component, rule-pack, owner-standard, and company design-basis data are user-controlled and excluded from public surfaces by default. | `docs/IP_AND_DATA_BOUNDARY.md` Private user data; `docs/CONTRACT.md` OPS-K-PRIV-1 |
| Professional boundary | Software and agents must not certify, seal, approve, authenticate, or declare engineering code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/DIRECTIVE.md` Principles |
| Handoff role | Shell/solid FEA remains a specialized local-analysis handoff path, not the primary global analysis model. | `docs/CONTRACT.md` OPS-K-MECH-1; `docs/DIRECTIVE.md` Principles |

## Construction

| Artifact | Status | Construction notes |
|---|---|---|
| Target mapping schema | MATERIALIZED | `schemas/target_mapping.schema.json` is the JSON Schema 2020-12 provider-neutral contract matching `core/handoff/target_mapping/contract.py`; exact commercial target enumeration remains TBD. |
| Unsupported behavior taxonomy | MATERIALIZED PROVIDER-NEUTRAL | `unsupported_behavior_flags`, `approximate_behavior_flags`, `SUPPORTED_BEHAVIOR_STATUSES`, and diagnostics expose unsupported and approximate behavior without silent loss of assumptions; target-specific taxonomy extensions remain TBD. |
| Mapping record identity | MATERIALIZED | Mapping records are traceable through `source_entity_ref`, `target_field_ref`, `mapping_status`, optional unit metadata, and provenance fields. |
| Unsupported/approximate behavior record | MATERIALIZED | Behavior flags carry `behavior_kind`, `behavior_status`, severity, description, source/target references, diagnostics, and provenance. |
| Hash and manifest binding | MATERIALIZED | `source_context` binds mapping metadata to package/model/unit/entity/library/rule references while preserving target/container TBDs. |

## References

- `_CONTEXT.md` - deliverable identity, scope, package envelope, architecture-basis injection.
- `_REFERENCES.md` - local reference index.
- `Dependencies.csv` - approved DAG-006 mirror/evidence surface for predecessor context.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 current decomposition basis, PKG-15 and DEL-15-02 rows.
- `docs/_Registers/Deliverables.csv` - DEL-15-02 row.
- `docs/_Registers/ScopeLedger.csv` - SOW-074 row.
- `docs/_Registers/ContextBudgetQA.csv` - DEL-15-02 row.
- `docs/CONTRACT.md` - invariant catalog.
- `docs/DIRECTIVE.md` - product principles.
- `docs/SPEC.md` - adapter/export and boundary specification slices.
- `docs/TYPES.md` - canonical object and boundary vocabulary.
- `docs/IP_AND_DATA_BOUNDARY.md` - private-data and public-surface boundary policy.
## D-41 R5 T3 PDU-017 Evidence State

Selected seam: target-mapping builder. Unsafe privacy context produces blocking diagnostics; broader runtime redaction reach remains separately owned.
