# Specification: DEL-15-01 Canonical handoff package schema and manifest

## Scope

This deliverable defines the contract surface for a canonical handoff package schema and handoff manifest for downstream modeling and professional validation workflows. The scope is bounded to DEL-15-01, PKG-15, SOW-074, and OBJ-017.

Included:

- schema/manifest requirements for model hash, units manifest, entity IDs, library/rule references, unresolved assumptions, warnings, target mapping metadata, unsupported-target flags, and provenance;
- JSON Schema 2020-12 alignment for the anticipated `schemas/handoff_package.schema.json` artifact;
- professional-boundary, unit, provenance, diagnostics, and data-boundary constraints that affect handoff package shape.

Excluded:

- implementation of the downstream export workflow, which belongs to DEL-15-03;
- detailed target mapping semantics and unsupported-behavior taxonomy beyond reserving the required surface, which belongs to DEL-15-02;
- external-prover lifecycle or approval metadata, which belongs to DEL-15-04;
- target-specific commercial parser implementation, commercial output ingestion, or automatic professional approval records.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#7-deliverables`; SOW-074; OBJ-017; OI-015.

## Requirements

| Req ID | Requirement | Source | Verification |
|---|---|---|---|
| DEL-15-01-R01 | The handoff package contract shall be represented as a schema-first API/data contract. | `_CONTEXT.md` Type `API_CONTRACT`; `docs/TYPES.md#3-software-deliverable-types` | Confirm artifact naming and schema document role in this deliverable. |
| DEL-15-01-R02 | The schema baseline shall use JSON Schema 2020-12 unless a later approved architecture change supersedes it. | `_CONTEXT.md#Architecture Basis Injection`; `execution/_Decomposition/SOFTWARE_DECOMP.md#82-resolved-architecture-baseline` | Validate `schemas/handoff_package.schema.json` with Draft 2020-12 tooling. |
| DEL-15-01-R03 | The package shall include or reference a model hash through the required `model_hash` checksum object, including algorithm, value, canonicalization, and provenance. | SOW-074; AB-00-04 | Review schema properties and fixture validation for model-hash presence. |
| DEL-15-01-R04 | The package shall include a `units_manifest` with unit system, dimensional basis, entries, diagnostics, and provenance sufficient to avoid silent missing-unit behavior. | SOW-074; `docs/SPEC.md#4-unit-system-and-dimensional-analysis`; OPS-K-UNIT-1 | Schema review plus fixture validation for unit-related fields. |
| DEL-15-01-R05 | The package shall preserve stable entity IDs used by downstream mapping and review through `entity_ids` records. | SOW-074; `docs/TYPES.md#2-stable-identifiers`; `docs/TYPES.md#8-canonical-domain-object-registry` | Schema review for required ID/reference fields and no positional-only coupling. |
| DEL-15-01-R06 | The package shall carry library and rule-pack references by identity, version/checksum/source note, or equivalent reference metadata without copying protected/private payloads through `library_refs` and `rule_pack_refs`. | SOW-074; `docs/IP_AND_DATA_BOUNDARY.md#6-private-user-data`; `docs/SPEC.md#9-reporting-and-audit` | Protected-content/privacy review and schema review for reference-only design. |
| DEL-15-01-R07 | The package shall carry unresolved assumptions, warnings, and diagnostics as explicit review evidence, not silent defaults. | SOW-074; OPS-K-DATA-2; AB-00-06; `docs/SPEC.md#8-gui-requirements` | Schema review for `unresolved_assumptions`, `warnings`, `diagnostics`, and provenance fields. |
| DEL-15-01-R08 | The package shall reserve target mapping metadata required by SOW-074 while leaving detailed target mapping rules to DEL-15-02. | SOW-074; DEL-15-02 row in decomposition | Cross-deliverable interface review against `schemas/target_mapping.schema.json`. |
| DEL-15-01-R09 | The package shall reserve unsupported-target flags required by SOW-074 while leaving detailed unsupported behavior semantics to DEL-15-02. | SOW-074; OI-015 | Cross-deliverable interface review against `schemas/target_mapping.schema.json`. |
| DEL-15-01-R10 | The package and manifest shall preserve provenance for reliance-affecting data and references. | OPS-K-DATA-3; `docs/DIRECTIVE.md#25-axiology--what-values-govern`; `docs/IP_AND_DATA_BOUNDARY.md#4-required-provenance-fields` | Schema review for provenance-bearing records and protected-content gate. |
| DEL-15-01-R11 | The contract shall not declare certification, sealing, approval, authentication, code compliance, or professional acceptance as automatic software output. | OPS-K-AUTH-1; OBJ-017; `docs/TYPES.md#4-analysis-status-vocabulary` | Text/schema enum review for prohibited automatic statuses. |
| DEL-15-01-R12 | The package container, supported handoff targets, and target-specific mapping strategy remain TBD and shall not be invented in this deliverable setup pass. | OI-015; `_CONTEXT.md#Architecture Basis Injection` | Confirm TBDs are explicit in drafts and schema planning notes. |

## Standards

| Standard / governing source | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Public schema/interchange baseline for the handoff schema. | Applicable by architecture basis; `$id` is `https://openpipestress.org/schemas/handoff_package.schema.json`. |
| Project invariant catalog (`docs/CONTRACT.md`) | Governs IP, units, data, professional-boundary, report, governance, and agent behavior. | Applicable. |
| Project type vocabulary (`docs/TYPES.md`) | Governs deliverable/API contract meaning, stable identifiers, status vocabulary, and epistemic labels. | Applicable. |
| IP and Data Boundary Policy (`docs/IP_AND_DATA_BOUNDARY.md`) | Governs protected/private data exclusion and provenance fields. | Applicable. |
| Product/decomposition basis (`execution/_Decomposition/SOFTWARE_DECOMP.md`) | Governs DEL-15-01 scope, SOW-074, OBJ-017, PKG-15 exclusions, architecture basis, and OI-015 TBDs. | Applicable. |

## Verification

| Check ID | Check | Expected result |
|---|---|---|
| V-01 | Confirm required four documents exist and keep the default schema sections. | PASS for this setup output. |
| V-02 | Confirm all non-trivial schema requirements trace to `_CONTEXT.md`, decomposition, registers, or governing references. | PASS if every requirement has a source. |
| V-03 | Confirm unsupported specifics are marked `TBD` or `ASSUMPTION`. | PASS if exact target list, package container, and target-specific mappings are not invented. |
| V-04 | Confirm professional-boundary prohibited terms are not used as automatic software statuses. | PASS if no schema requirement creates automatic approval/compliance/certification states. |
| V-05 | Confirm local `Dependencies.csv` remains a DAG-002 mirror with approved rows preserved as ACTIVE. | PASS if dependency mirror is not retired/reclassified by setup. |
| V-06 | Run `python3 tests/test_handoff_package_schema.py` and `python3 tools/validation/validate_dependencies_schema.py <DEL folder>/Dependencies.csv` when `Dependencies.csv` exists. | PASS/FAIL recorded in final run report. |

## Documentation

Required deliverable records:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- local dependency validation result in the final run report

Implemented evidence artifacts from `_CONTEXT.md`:

- `schemas/handoff_package.schema.json`
- `tests/test_handoff_package_schema.py`
- `fixtures/invented_handoff_package.json`
