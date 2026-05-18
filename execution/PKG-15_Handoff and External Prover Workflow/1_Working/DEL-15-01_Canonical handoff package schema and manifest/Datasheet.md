# Datasheet: DEL-15-01 Canonical handoff package schema and manifest

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
| Lifecycle input state | OPEN at PREPARATION time | `_STATUS.md` |

## Attributes

| Attribute | Value | Source / status |
|---|---|---|
| Contract surface | Schema-compliant handoff package plus manifest | SOW-074 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#4-structured-scope-of-work-ssow` |
| Schema baseline | JSON Schema 2020-12 contracts | `_CONTEXT.md#Architecture Basis Injection`; `execution/_Decomposition/SOFTWARE_DECOMP.md#82-resolved-architecture-baseline` |
| Hash baseline | Canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; manifest hashes for non-JSON/binary assets | `_CONTEXT.md#Architecture Basis Injection`; `execution/_Decomposition/SOFTWARE_DECOMP.md#81-architecture-basis-register` |
| Required package contents named by scope | model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags | SOW-074 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| Professional boundary | Handoff packages support downstream modeling and professional validation workflows without automatic professional approval states | OBJ-017 in `execution/_Decomposition/SOFTWARE_DECOMP.md#5-objectives`; `docs/CONTRACT.md#1-invariant-index` |
| Target-specific commercial parsers | Deferred / out of this deliverable | SOW-074 notes; `execution/_Decomposition/SOFTWARE_DECOMP.md#11-open-issues` |
| Canonical package container | TBD | OI-015 in `execution/_Decomposition/SOFTWARE_DECOMP.md#11-open-issues` |
| Handoff target list | TBD | OI-015 in `execution/_Decomposition/SOFTWARE_DECOMP.md#11-open-issues` |
| Exact schema property names and `$id` values | TBD | No accessible source defines them for DEL-15-01 |

## Conditions

| Condition | Record |
|---|---|
| Data boundary | The public repository must not include protected standards text, protected tables, proprietary vendor data, private project data, or private rule-pack payloads. Source: `docs/IP_AND_DATA_BOUNDARY.md#3-public-repository-must-not-contain`. |
| Unit boundary | Physical values crossing schema, import/export, report, or rule-evaluation boundaries must carry explicit unit metadata unless explicitly dimensionless. Source: `docs/SPEC.md#4-unit-system-and-dimensional-analysis`. |
| Diagnostics boundary | Warnings and diagnostics must preserve structured source/provenance fields and must not become code-compliance or professional-approval claims. Source: `docs/CONTRACT.md#1-invariant-index`; AB-00-06 in decomposition. |
| API/adapter boundary | Adapters and plugins cannot bypass validation, unit checks, provenance, diagnostics, privacy, protected-content, or professional-boundary controls. Source: AB-00-07 in `execution/_Decomposition/SOFTWARE_DECOMP.md#81-architecture-basis-register`. |
| Dependency context | DAG-002 mirror lists upstream architecture-basis, result export, audit manifest/model hash, immutable model state, analysis run, local FEA handoff, and canonical domain model rows as ACTIVE evidence. Source: local `Dependencies.csv`. |

## Construction

The deliverable is a contract-definition unit, not an implementation artifact in this setup pass. Construction evidence currently supports the following schema/manifest slots:

| Slot | Required treatment | Source |
|---|---|---|
| Package identity | Include stable package/manifest identity and schema version fields. Exact names TBD. | SOW-074; JSON Schema 2020-12 baseline |
| Model hash | Represent hash metadata for the model basis. Exact algorithm field names TBD. | SOW-074; AB-00-04 hash basis |
| Units manifest | Represent explicit units and dimensional intent. Exact unit schema reference TBD. | SOW-074; `docs/SPEC.md#4-unit-system-and-dimensional-analysis` |
| Entity IDs | Preserve stable model/entity identifiers used by downstream mapping. Exact ID vocabulary TBD. | SOW-074; `docs/TYPES.md#2-stable-identifiers` |
| Library/rule references | Reference libraries and rule packs by identity/checksum/provenance without copying protected/private payloads. Exact reference schema TBD. | SOW-074; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/SPEC.md#9-reporting-and-audit` |
| Warnings and unresolved assumptions | Carry structured warnings and assumptions as review evidence. Exact warning schema reference TBD. | SOW-074; `docs/SPEC.md#8-gui-requirements`; `docs/SPEC.md#9-reporting-and-audit` |
| Target mapping metadata | Reserve a manifest surface for mapping to downstream target fields. Detailed target mapping contract is DEL-15-02. | SOW-074; DEL-15-02 row in decomposition |
| Unsupported-target flags | Reserve explicit unsupported/approximate target behavior flags. Detailed target contract is DEL-15-02. | SOW-074; OI-015 |
| Provenance | Preserve source/provenance for reliance-affecting data. Exact provenance object reference TBD. | `docs/DIRECTIVE.md#22-epistemology--what-is-warranted`; `docs/CONTRACT.md#1-invariant-index` |

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
