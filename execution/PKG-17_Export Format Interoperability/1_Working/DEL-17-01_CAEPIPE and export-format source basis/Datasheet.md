# Datasheet: DEL-17-01 CAEPIPE and export-format source basis

## Identification

| Field | Value |
|---|---|
| DeliverableID | DEL-17-01 |
| PackageID | PKG-17 |
| Name | CAEPIPE and export-format source basis |
| Type | DOC_UPDATE |
| Scope Items | SOW-030, SOW-074, SOW-075 |
| Objectives | OBJ-009, OBJ-017, OBJ-018 |
| Lifecycle Role | First source-basis deliverable for the SCA-004 export interoperability workflow |

## Purpose

This deliverable records the admitted source basis for later export-format work. It is not an exporter, schema, implementation, compatibility claim, release claim, or professional validation artifact.

Later `DEL-17-*` deliverables must consume this source basis before making target-format assumptions about CAEPIPE MBF, CAEPIPE batch execution, CAEPIPE CSV results, PCF translation, GLB/glTF review geometry, or adapter SDK behavior.

## Admitted Source Set

| Source ID | Source | Use in PKG-17 |
|---|---|---|
| PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Human strategic basis for export priorities, boundaries, and target ordering. |
| CAEPIPE-IMPORT-MBF | Official CAEPIPE MBF import/manual page | Public evidence for MBF as a text model input path and command-line behavior. |
| CAEPIPE-EXPORT-DATA | Official CAEPIPE model/result export page | Public evidence for model/result export surfaces including CSV/text-style outputs. |
| CAEPIPE-EXPORT-MBF | Official CAEPIPE MBF export page | Public evidence that CAEPIPE can export model data to MBF. |
| CAEPIPE-BATCH | Official CAEPIPE batch-mode page | Public evidence for external executable invocation constraints. |
| CAEPIPE-PCF | Official CAEPIPE PCF translator document | Public evidence for PCF translation limits, mappings, defaults, and translator-dependent behavior. |
| GLTF-2.0 | Khronos glTF 2.0 specification | Public evidence for review-geometry export format semantics. |
| CONTRACT / IP-DATA / SPEC | Project governance and technical docs | Binding constraints for no protected data, no professional claim, no hidden defaults, and no-bypass adapter boundaries. |

## Source-Basis Findings

| Finding ID | Finding | Source Basis | Status |
|---|---|---|---|
| F-17-01-001 | CAEPIPE MBF is the preferred first solver-specific target because it is a documented text model input/output path and is closer to CAEPIPE model data than PCF translation. | PLAN-EXPORT-INTEROP; CAEPIPE-IMPORT-MBF; CAEPIPE-EXPORT-MBF | ADMITTED |
| F-17-01-002 | CAEPIPE external execution may be supported only as an optional user-owned harness using a user-provided executable and license. | PLAN-EXPORT-INTEROP; CAEPIPE-IMPORT-MBF; CAEPIPE-BATCH; IP-DATA | ADMITTED_WITH_BOUNDARY |
| F-17-01-003 | CAEPIPE CSV or text results may be parsed for regression/handoff evidence, but parsed outputs remain non-authoritative and do not become professional acceptance. | PLAN-EXPORT-INTEROP; CAEPIPE-EXPORT-DATA; CONTRACT | ADMITTED_WITH_BOUNDARY |
| F-17-01-004 | PCF is useful for broader interoperability, but CAEPIPE PCF translation has mapping/default behavior that makes it unsuitable as the first deterministic validation backbone. | PLAN-EXPORT-INTEROP; CAEPIPE-PCF | ADMITTED_WITH_LIMIT |
| F-17-01-005 | GLB/glTF is a review-geometry target, not solver geometry or validation evidence. | GLTF-2.0; PLAN-EXPORT-INTEROP | ADMITTED_WITH_BOUNDARY |
| F-17-01-006 | Later export deliverables must report exported, omitted, approximated, delegated, and unsupported behavior rather than silently accepting target losses. | PLAN-EXPORT-INTEROP; CONTRACT; SCA-004 | ADMITTED |

## Boundary Facts

- No CAEPIPE binary, commercial example, proprietary model file, copied report, or vendor fixture is admitted.
- No protected standards text, standards-derived tables, code allowables, SIF/flexibility tables, or owner design criteria are admitted.
- Pass-through target fields may name solver options only as downstream target configuration, not as local code-checking logic.
- A successful export or external run is regression/handoff evidence only; it is not professional engineering acceptance.

## Downstream Consumers

| Consumer | Dependency on DEL-17-01 |
|---|---|
| DEL-17-02 | Uses this source basis to define common export package/profile/ID-map/loss-report contracts. |
| DEL-17-04 | Uses this source basis before defining CAEPIPE MBF writer assumptions. |
| DEL-17-05 | Uses this source basis before defining CAEPIPE external harness and CSV parser behavior. |
| DEL-17-06 | Uses this source basis before defining automated CAEPIPE CSV/text result parser coverage. |
| DEL-17-07 | Uses this source basis before defining conservative PCF subset behavior. |
| DEL-17-08 | Uses this source basis before defining review geometry export assumptions. |

## Open TBD Register

| TBD ID | Question | Blocks |
|---|---|---|
| TBD-17-01-001 | Which CAEPIPE version/profile is the first supported target? | DEL-17-04, DEL-17-05 |
| TBD-17-01-002 | Which MBF record families are required for the first deterministic writer subset? | DEL-17-04 |
| TBD-17-01-003 | Which MBF fields can safely carry stable canonical IDs, and which require sidecar maps? | DEL-17-02, DEL-17-04 |
| TBD-17-01-004 | What CSV result sections are stable enough for automated parsing in a first harness? | DEL-17-05 |
| TBD-17-01-005 | What PCF subset is conservative enough to avoid hidden translator defaults? | DEL-17-07 |
| TBD-17-01-006 | What geometry fields are sufficient for glTF review without implying solver fidelity? | DEL-17-08 |
