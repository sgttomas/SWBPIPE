# Datasheet: DEL-15-01 Canonical handoff package schema and manifest

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-15-01-DECL-002`.

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-15-01 | `_CONTEXT.md` |
| Name | Canonical handoff package schema and manifest | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Package | PKG-15 Handoff and External Prover Workflow | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Type | API_CONTRACT | `_CONTEXT.md`; `docs/TYPES.md#3-software-deliverable-types` |
| Scope item | SOW-074 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| Objective | OBJ-017 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#5-objectives` |
| Anticipated artifacts | `schemas/handoff_package.schema.json`; handoff manifest schema | `_CONTEXT.md` |
| Lifecycle input state | `IN_PROGRESS` at review-readiness intake | `_STATUS.md` |

## Attributes

| Attribute | Value | Source / status |
|---|---|---|
| Contract surface | Schema-compliant handoff package plus manifest | SOW-074 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#4-structured-scope-of-work-ssow` |
| Schema baseline | JSON Schema 2020-12 contracts | `_CONTEXT.md#Architecture Basis Injection`; `execution/_Decomposition/SOFTWARE_DECOMP.md#82-resolved-architecture-baseline` |
| Hash baseline | `deterministic_sorted_compact_json_payload_hash` for the existing Python sorted-key compact-JSON hash basis (not RFC 8785 JCS); `JCS_compatible_json_payload_hash` retained for backward compatibility with unrepaired producers; manifest hashes for non-JSON/binary assets | D-41 `DEC-074` E1; PDU-002 prerequisite; `_CONTEXT.md#D-41 E1 canonicalization vocabulary boundary` |
| Required package contents named by scope | model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags | SOW-074 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| Professional boundary | Handoff packages support downstream modeling and professional validation workflows without automatic professional approval states | OBJ-017 in `execution/_Decomposition/SOFTWARE_DECOMP.md#5-objectives`; `docs/CONTRACT.md#1-invariant-index` |
| Target-specific commercial parsers | Deferred / out of this deliverable | SOW-074 notes; `execution/_Decomposition/SOFTWARE_DECOMP.md#11-open-issues` |
| Canonical package container | TBD | OI-015 in `execution/_Decomposition/SOFTWARE_DECOMP.md#11-open-issues` |
| Handoff target surfaces | Initial export and target surfaces are named by OI-015; concrete mappings, unsupported-behavior taxonomy extensions, target field coverage, and target-specific implementation remain gated by DEL-17-01 and DEL-17-02 | OI-015 in `execution/_Decomposition/SOFTWARE_DECOMP.md#11-open-issues` |
| Schema property names and `$id` values | `$id`: `https://openpipestress.org/schemas/handoff_package.schema.json`; root and nested properties are materialized in `schemas/handoff_package.schema.json` | DEL-15-01 implementation evidence |

## Conditions

| Condition | Record |
|---|---|
| Data boundary | The public repository must not include protected standards text, protected tables, proprietary vendor data, private project data, or private rule-pack payloads. Source: `docs/IP_AND_DATA_BOUNDARY.md#3-public-repository-must-not-contain`. |
| Unit boundary | Physical values crossing schema, import/export, report, or rule-evaluation boundaries must carry explicit unit metadata unless explicitly dimensionless. Source: `docs/SPEC.md#4-unit-system-and-dimensional-analysis`. |
| Diagnostics boundary | Warnings and diagnostics must preserve structured source/provenance fields and must not become code-compliance or professional-approval claims. Source: `docs/CONTRACT.md#1-invariant-index`; AB-00-06 in decomposition. |
| API/adapter boundary | Adapters and plugins cannot bypass validation, unit checks, provenance, diagnostics, privacy, protected-content, or professional-boundary controls. Source: AB-00-07 in `execution/_Decomposition/SOFTWARE_DECOMP.md#81-architecture-basis-register`. |
| Dependency context | DAG-002 mirror lists upstream architecture-basis, result export, audit manifest/model hash, immutable model state, analysis run, local FEA handoff, and canonical domain model rows as ACTIVE evidence. Source: local `Dependencies.csv`. |

## Construction

The deliverable is a contract-definition unit with a materialized JSON Schema and invented validation fixture. Construction evidence currently supports the following schema/manifest slots:

| Slot | Required treatment | Source |
|---|---|---|
| Package identity | Include stable package identity, schema version, deliverable/package/scope/objective identifiers, and review state fields. | SOW-074; `schemas/handoff_package.schema.json` |
| Model hash | Represent model basis through the required `model_hash` checksum object, including algorithm, value, canonicalization, and provenance. | SOW-074; AB-00-04 hash basis; `schemas/handoff_package.schema.json` |
| Canonicalization label | Use `deterministic_sorted_compact_json_payload_hash` only for the existing sorted-key compact-JSON byte basis; do not interpret it as RFC 8785. Legacy JCS-compatible labels remain accepted until their producers are separately repaired. | `DEC-074` E1; `schemas/handoff_package.schema.json` |
| Units manifest | Represent explicit units through the required `units_manifest` object, including unit system, dimensional basis, entries, diagnostics, and provenance. | SOW-074; `docs/SPEC.md#4-unit-system-and-dimensional-analysis`; `schemas/handoff_package.schema.json` |
| Entity IDs | Preserve stable model/entity identifiers through required `entity_ids` records with `entity_id`, `entity_kind`, `source_ref`, and optional mapping keys. | SOW-074; `docs/TYPES.md#2-stable-identifiers`; `schemas/handoff_package.schema.json` |
| Library/rule references | Reference libraries and rule packs through `library_refs` and `rule_pack_refs` identity/checksum/provenance records without copying protected/private payloads. | SOW-074; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/SPEC.md#9-reporting-and-audit`; `schemas/handoff_package.schema.json` |
| Warnings and unresolved assumptions | Carry structured `warnings`, `unresolved_assumptions`, and `diagnostics` as review evidence with source/provenance fields. | SOW-074; `docs/SPEC.md#8-gui-requirements`; `docs/SPEC.md#9-reporting-and-audit`; `schemas/handoff_package.schema.json` |
| Target mapping metadata | Reserve a manifest surface for mapping to downstream target fields. Detailed target mapping contract is DEL-15-02. | SOW-074; DEL-15-02 row in decomposition |
| Unsupported-target flags | Reserve explicit unsupported/approximate target behavior flags. Detailed target contract is DEL-15-02. | SOW-074; OI-015 |
| Provenance | Preserve source/provenance for reliance-affecting data through the schema's required `provenance` object and nested reference provenance records. | `docs/DIRECTIVE.md#22-epistemology--what-is-warranted`; `docs/CONTRACT.md#1-invariant-index`; `schemas/handoff_package.schema.json` |

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `_STATUS.md`
- local `Dependencies.csv`
- `execution/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/ContextBudgetQA.csv`
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/DIRECTIVE.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
