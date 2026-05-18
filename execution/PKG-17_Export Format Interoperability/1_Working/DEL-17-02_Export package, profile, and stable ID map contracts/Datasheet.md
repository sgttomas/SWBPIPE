# Datasheet: DEL-17-02 Export package, profile, and stable ID map contracts

## Identity

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-02 |
| Package | PKG-17 Export Format Interoperability |
| Type | API_CONTRACT |
| Lifecycle state after this tranche | SEMANTIC_READY |
| Source basis | DEL-17-01 source-basis dossier and admitted references |
| Downstream consumers | DEL-17-03 through DEL-17-09 |

## Purpose

DEL-17-02 defines the common export contract that later PKG-17 target exporters shall consume before writing target-specific formats. It covers export package structure, export profile metadata, stable ID maps, export manifests, loss reports, and unsupported/approximated/delegated behavior records.

This deliverable is a contract-level planning artifact. It does not implement JSON schemas, Rust modules, exporters, parsers, harnesses, GUI behavior, persistence runtime, public API endpoints, or target-specific file writers.

## Contract Objects

| Object | Role | Required downstream behavior |
|---|---|---|
| Export package | Binds payloads, target files, manifests, ID maps, diagnostics, and loss reports into one reproducible package concept. | Every exporter shall identify which package members it emits and which are outside its tranche. |
| Export profile | Declares target name, profile version, target version basis, unit policy, coordinate policy, entity coverage, and target-specific TBDs. | Target exporters shall consume a profile rather than encode hidden target assumptions. |
| Stable ID map | Maps OpenPipeStress canonical IDs to target record IDs, file names, sidecars, metadata slots, or omitted-target entries. | Every target artifact shall preserve identity directly or through a sidecar map. |
| Export manifest | Records source model identity, profile identity, package members, hashes where applicable, diagnostics, and boundary notes. | Export packages shall be auditable without relying on unstated runtime context. |
| Loss report | Records exported, omitted, approximated, delegated, unsupported, and TBD behavior. | Exporters shall not silently approximate unsupported target concepts. |

## Source Consumption

| Source | Consumption rule |
|---|---|
| DEL-17-01 `Source_Basis_Register.md` | Supplies admitted public/project source IDs and PKG-17 source boundaries. |
| DEL-17-01 `CAEPIPE_Question_Dossier.md` | Supplies unresolved CAEPIPE questions that remain `TBD` for target-specific claims. |
| SCA-004 and DAG-005 | Establish PKG-17 scope and sequencing authority. |
| `docs/CONTRACT.md` and `docs/IP_AND_DATA_BOUNDARY.md` | Govern protected-content, professional-boundary, and no-bypass constraints. |
| `schemas/model.schema.yaml` | Supplies canonical model identity vocabulary for mapping, without changing the schema in this tranche. |

## Local Contract Artifact Inventory

| Artifact | Contract role |
|---|---|
| `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` | Human-readable contract kit for the export package/profile/stable-ID/loss-report surface. |
| `_SEMANTIC.md` | Semantic matrix lens for coverage review; not an engineering authority. |
| `_SEMANTIC_LENSING.md` | Warranted enrichment register applied by Pass 3; not an authority source by itself. |
| `Dependencies.csv` | v3.1 dependency register tying DEL-17-02 to upstream source-basis evidence and downstream PKG-17 consumers. |
| `_run_records/` | Auditable TASK execution records for sealed population and validation passes. |

## Required Export-Profile Fields

| Field | Meaning |
|---|---|
| `profile_id` | Stable identifier for the export profile. |
| `profile_version` | Version of the OpenPipeStress profile contract. |
| `target_family` | Target family such as native JSON, CAEPIPE MBF, stress-neutral CSV/JSON, PCF, GLB/glTF, or adapter SDK target. |
| `target_version_basis` | Cited target version or `TBD` when not yet source-confirmed. |
| `source_basis_ids` | DEL-17-01 source IDs and project references used by the profile. |
| `unit_policy` | Declared output units and conversion responsibility. |
| `coordinate_policy` | Declared coordinate frame, axis convention, and origin policy. |
| `entity_coverage` | Supported, omitted, approximated, delegated, unsupported, and TBD entity families. |
| `stable_id_policy` | Direct ID, sidecar ID, metadata ID, or omitted ID behavior. |
| `loss_report_policy` | Required loss-report categories and blocking diagnostics. |
| `external_execution_policy` | Optional user-owned harness metadata only, when applicable. |
| `boundary_notes` | IP, license, private-data, and professional-boundary constraints. |

## Stable ID Families

The common ID map shall account for at least:

- project and model IDs;
- line, node, element, component, support, material, section, load case, result case, analysis run, and export IDs;
- target file IDs, target record IDs, target metadata keys, target sidecar IDs, and omitted-target placeholders.

If a target cannot carry a canonical ID directly, the exporter shall record the mapping in a sidecar and reference that sidecar from the manifest.

## Loss Categories

| Category | Meaning |
|---|---|
| `exported` | Represented directly in the target package. |
| `omitted` | Not exported, with explicit reason and affected canonical IDs. |
| `approximated` | Represented by a declared approximation with source and target IDs. |
| `delegated` | Passed through to a target tool or external workflow without local interpretation. |
| `unsupported` | Known unsupported behavior that blocks or limits target use. |
| `tbd` | Behavior not source-confirmed; shall not be represented as supported. |

## Boundary Summary

- No bundled commercial solver or target executable.
- No proprietary examples, private target data, protected standards tables, material allowables, SIF/flexibility values, or owner criteria.
- No reverse engineering, license bypass, compatibility overclaim, release claim, code-compliance claim, professional-acceptance claim, or validation claim.
- No target-specific field semantics beyond DEL-17-01 evidence; unresolved behavior remains `TBD`.
