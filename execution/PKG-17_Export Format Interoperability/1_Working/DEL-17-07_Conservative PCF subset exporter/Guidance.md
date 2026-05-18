# Guidance: DEL-17-07 Conservative PCF subset exporter

## Purpose

Use this deliverable to make PCF useful without overstating it. The exporter should help users exchange plant-design geometry and component context, while making every unsupported, approximate, delegated, or unknown behavior visible.

PCF is a secondary interoperability path in the PKG-17 strategy. The public CAEPIPE PCF translator documentation shows enough fallback and mapping behavior that this path should not be treated as the first deterministic solver-validation backbone.

## Principles

### Keep the subset narrow

Start from the plan-listed conservative subset: straight pipe, elbows/bends, tees, reducers, flanges, valves, end connections, line numbers, nominal size, OD/wall-thickness attributes where supported, basic spec/material labels, coordinates, component identifiers, and stable ID sidecar mapping.

Do not expand the subset because a downstream translator might infer something. If source evidence does not confirm the behavior, mark it `TBD`, unsupported, or delegated.

### Make defaults explicit

The CAEPIPE PCF translator source documents default or external behaviors for areas such as units, load data, fluid specific gravity, weight, OD/wall thickness, support mapping, material mapping, boundary conditions, and component simulation. Those are target-path facts, not OpenPipeStress exporter defaults.

For this deliverable, a hidden default is a reportable condition:

- if OpenPipeStress has explicit source data and the PCF profile can carry it, export it;
- if the target path may infer it from translator configuration, record delegated behavior;
- if the behavior affects solver-ready interpretation and is not source-confirmed, block or mark `TBD`;
- if a component is represented by a target-side approximation, record the approximation and affected canonical IDs.

### Preserve identity even when PCF cannot

Stable ID sidecars are expected for this slice unless direct PCF identity carriage is later source-confirmed. The sidecar should connect canonical model IDs to target records, omitted entries, diagnostics, and loss-report rows.

### Treat support and restraint semantics cautiously

Support/restraint semantics should be reviewed separately from geometry/component export. Public PCF translator evidence indicates that support transfer can depend on PCF attributes, SKEY values, mapping databases, options, stiffness/gap/friction customization, or default hanger behavior. The first exporter should not imply reliable preservation of support behavior unless the profile explicitly proves it.

Use the following criteria when classifying support/restraint outcomes in the future profile:

| Outcome | Use when |
|---|---|
| Reliably preserved | The selected profile has source-confirmed PCF attributes, mapping rules, units, coordinates, support type, stiffness/gap/friction semantics where applicable, and stable ID traceability for the supported support/restraint class. |
| Delegated | The downstream translator or user-owned mapping database may decide the final support type or properties, and OpenPipeStress can identify that delegation without treating it as local support preservation. |
| `TBD` | Public/project-owned evidence does not yet resolve whether the selected profile can preserve the support/restraint class. |
| Unsupported | The selected profile cannot carry the required semantics or the project intentionally excludes the class from the conservative subset. |
| Blocked | Loss of support/restraint meaning would make the exported package misleading for solver-ready or compatibility-sensitive use. |

This classification belongs in the profile and loss report, not in hidden writer defaults. Sources: CAEPIPE-PCF `Supports`; PLAN-EXPORT-INTEROP section `5. PCF`; DEL-17-02 loss-report requirements.

### Avoid professional and compatibility overclaims

Successful PCF file creation is export evidence only. It is not proof that a downstream tool imported the model correctly, that stress-model semantics are complete, that a design complies with any code, or that a professional has accepted the work.

## Considerations

| Topic | Guidance |
|---|---|
| Units | Require explicit unit policy. Do not use dimensionless or missing unit metadata as fallback. |
| OD and wall thickness | Prefer explicit source attributes where supported by the selected PCF profile. External mapping database fallback is delegated target behavior, not local data. |
| Pressure and temperature | Do not create load values from translator defaults. Missing values remain diagnostics, `TBD`, or loss-report entries. |
| Material/spec labels | Labels may be exported if source-owned, but target material property mapping remains delegated unless source-confirmed. |
| Component mappings | Direct component export and target-side simulation are different conditions. Record approximations. |
| Free ends and equipment connections | Avoid implying boundary-condition preservation when PCF lacks connection detail. |
| CAEPIPE notes | CAEPIPE-PCF behavior may be documented as a secondary route only. It must not become a compatibility claim. |

## Trade-offs

| Choice | Benefit | Cost |
|---|---|---|
| Narrow profile with strong diagnostics | Avoids hidden behavior and supports auditability. | Some models will export with many unsupported/TBD entries. |
| Sidecar stable ID map | Preserves traceability even when PCF cannot carry canonical IDs directly. | Requires package consumers to keep target file and sidecar together. |
| Blocking support/restraint ambiguity | Reduces risk of misleading stress-model handoff. | Limits usefulness for downstream tools that tolerate inferred support behavior. |
| Invented-only fixtures | Preserves IP and standards boundaries. | Does not prove compatibility with real proprietary plant models. |

## Examples

No proprietary, vendor, standards-derived, or client examples are admitted. Future examples for this deliverable should use small invented models with clearly artificial names, dimensions, units, and provenance. Example values shall not be copied from protected standards, catalogs, or owner design bases.

## Conflict Table (for human ruling)

Keep the profile/version conflict open until an admitted public source or explicit human project authority selects the first supported PCF target profile. The plan-listed subset is candidate scope, not final support classification.

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| DEL-17-07-CF-001 | The plan lists an initial conservative PCF subset, but exact PCF target-version/profile behavior is not selected. | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section `5. PCF` | DEL-17-01 `Source_Basis_Register.md` TBD-17-01-005 and `CAEPIPE_Question_Dossier.md` CQ-17-01-006 | Datasheet Attributes; Specification Profile Requirements; Procedure Step 3 | Treat plan-listed subset as candidate scope only; keep profile version and final support classes `TBD` until downstream source/profile decision. | TBD |
