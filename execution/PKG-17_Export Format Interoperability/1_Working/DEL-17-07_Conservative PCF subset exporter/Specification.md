# Specification: DEL-17-07 Conservative PCF subset exporter

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-07-DECL-001`.

## Scope

DEL-17-07 shall define and later implement a conservative PCF subset exporter for broader plant-design interoperability. The deliverable shall produce a PCF subset profile, PCF writer behavior, unsupported behavior report, and invented fixtures.

This deliverable shall not claim PCF completeness, CAEPIPE compatibility, solver validation, release readiness, code compliance, professional acceptance, or formal external-tool approval. It shall not implement schema changes or code in this Phase A four-document pass.

## Requirements

### Source Authority Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-07-REQ-001 | The PCF subset profile shall consume DEL-17-01 as the source-basis authority for PCF target behavior claims. |
| DEL-17-07-REQ-002 | The PCF subset profile shall consume DEL-17-02 for export package, profile, stable ID map, manifest, and loss-report rules. |
| DEL-17-07-REQ-003 | Target behavior not supported by admitted public/project-owned evidence shall remain `TBD`, delegated, unsupported, or loss-reported. |
| DEL-17-07-REQ-004 | The exporter shall not rely on hidden translator defaults for stress-relevant or identity-relevant behavior. |

### Profile Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-07-REQ-010 | The PCF profile shall declare `target_family = PCF` and shall include profile ID, profile version, target version basis, source-basis IDs, and boundary notes. |
| DEL-17-07-REQ-011 | The PCF profile shall declare unit policy, coordinate policy, stable-ID policy, line/component identity policy, and loss-report policy. |
| DEL-17-07-REQ-012 | The target version basis shall remain `TBD` until public evidence or human project authority selects a first PCF target profile. |
| DEL-17-07-REQ-013 | The profile shall classify each candidate entity family as exported, omitted, approximated, delegated, unsupported, or `TBD`. |
| DEL-17-07-REQ-014 | The initial candidate subset shall be limited to the plan-listed PCF entities and attributes unless later source review narrows or expands it. |
| DEL-17-07-REQ-015 | The PCF profile shall carry DEL-17-01, DEL-17-02, CAEPIPE-PCF, and PLAN-EXPORT-INTEROP source-basis references; missing references shall block package acceptance. |

### Writer Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-07-REQ-020 | The PCF writer shall emit only profile-supported content from unit-aware, provenance-bearing OpenPipeStress model data. |
| DEL-17-07-REQ-021 | The writer shall produce diagnostics for missing or ambiguous units, coordinates, nominal size, OD, wall thickness, material/spec labels, component identity, end-connection data, and support/restraint semantics when those fields affect the selected profile. |
| DEL-17-07-REQ-022 | The writer shall not invent pressure, temperature, fluid density, component weight, OD, wall thickness, material properties, support type, equipment connection, or boundary-condition data. |
| DEL-17-07-REQ-023 | The writer shall preserve canonical OpenPipeStress identity through target metadata when explicitly supported or through a sidecar stable ID map when direct PCF carriage is not source-confirmed. |
| DEL-17-07-REQ-024 | The writer shall record target-generated identity separately from canonical OpenPipeStress identity. |

### Unsupported and Loss-Report Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-07-REQ-030 | Every PCF export attempt shall produce or reference a loss report. |
| DEL-17-07-REQ-031 | Loss report entries shall include affected canonical IDs, category, severity, target artifact if any, reason, source-basis reference, and downstream implication, consistent with DEL-17-02. |
| DEL-17-07-REQ-032 | Support/restraint semantics shall be listed separately when the PCF target path cannot preserve them reliably. |
| DEL-17-07-REQ-033 | Free-end, equipment-nozzle, adjacent-pipeline, and boundary-condition semantics shall be blocked, `TBD`, delegated, or loss-reported when the target path would otherwise depend on translator anchoring or incomplete PCF connection data. |
| DEL-17-07-REQ-034 | Component mappings known to become target-side rigid elements, equivalent pipes, concentrated masses, or other approximations shall be recorded as approximated or delegated behavior rather than direct support. |

### Fixture and Data-Boundary Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-07-REQ-040 | Fixtures shall be invented or otherwise redistribution-safe with documented provenance. |
| DEL-17-07-REQ-041 | Fixtures shall not copy proprietary plant models, vendor examples, protected standards text, protected tables, material allowables, SIF/flexibility values, owner criteria, or commercial catalog values. |
| DEL-17-07-REQ-042 | Public fixtures shall avoid code-specific compliance semantics and shall not imply professional acceptance. |

## Standards

| Source | Status in this phase |
|---|---|
| Public CAEPIPE PCF translator documentation (`CAEPIPE-PCF`) | Accessible source for translator behavior, defaults, and mappings; not authority for OpenPipeStress completeness or compatibility claims. |
| DEL-17-01 source basis | Governing project source-basis authority for downstream PCF claims. |
| DEL-17-02 export contract | Governing project contract for profile, manifest, ID map, and loss reporting. |
| PCF format specification | `TBD`; no redistributed proprietary specification text is admitted in this phase. |
| Engineering design codes and standards | Out of scope for this exporter; no code-compliance or professional-approval claim is permitted. |

## Verification

| Requirement group | Verification approach |
|---|---|
| Source authority | Review that each PCF behavior statement cites DEL-17-01, DEL-17-02, `PLAN-EXPORT-INTEROP`, `CAEPIPE-PCF`, or project governance sources. |
| Conservative profile | Check that every selected entity family is classified and that unresolved behavior remains `TBD`. |
| Writer behavior | Future implementation tests shall use invented fixtures and verify that supported records are emitted deterministically and unsupported content produces diagnostics/loss entries. |
| Units and coordinates | Future tests shall include explicit unit-bearing source data and ambiguous/missing-unit negative cases. Acceptance criteria remain profile-bound: after the PCF profile is selected, tests shall verify declared PCF unit representation, coordinate policy, rejection or diagnostic behavior for missing/ambiguous unit metadata, and no dimensionless fallback for unit-bearing physical values. Sources: CAEPIPE-PCF `Reference / Units`; `docs/SPEC.md` section `3.1 Unit contract`; DEL-17-02 export profile requirements. |
| Stable IDs | Future tests shall verify sidecar identity mapping when direct PCF identity carriage is not source-confirmed. |
| Loss report | Future tests shall assert entries for omitted, approximated, delegated, unsupported, and `TBD` behavior. |
| Boundary controls | Review for absence of proprietary examples, protected standards data, release claims, compatibility claims, code-compliance claims, and professional-acceptance claims. |

### Diagnostic and Loss-Report Acceptance Scaffold

The future implementation tranche shall convert the following scaffold into concrete tests only after the PCF profile is sealed. Until then, expected outcome values remain `TBD` rather than implied support claims.

| Coverage class | Minimum acceptance hook | Source basis |
|---|---|---|
| Missing or ambiguous units | Diagnostic or blocking loss entry; no silent default for unit-bearing physical values | CAEPIPE-PCF `Reference / Units`; `docs/SPEC.md` section `3.1 Unit contract`; DEL-17-02-REQ-021/050/051/053 |
| Coordinates and vertical axis | Profile-declared coordinate/vertical-axis policy; diagnostic when source coordinate basis is absent or ambiguous | CAEPIPE-PCF translator dialog vertical-axis option; DEL-17-02-REQ-021 |
| Nominal size, OD, and wall thickness | Export only explicit/source-confirmed attributes where selected; external mapping-database behavior is delegated or loss-reported | CAEPIPE-PCF `Wall Thickness`, `OD and Nominal Size`, and `OD and Wall Thickness via config_pcf.ini` |
| Material/spec labels | Labels may be emitted only with source-owned provenance; target material-property mapping is delegated unless source-confirmed | CAEPIPE-PCF `Material`; IP-DATA protected-content boundary |
| Canonical identity and target identity | Stable ID sidecar or target metadata links canonical IDs to emitted records, omissions, diagnostics, and loss rows; target-generated IDs remain separate | DEL-17-02 stable ID map requirements |
| End connections and boundary conditions | Free ends, equipment nozzles, adjacent pipelines, and unresolved boundary semantics are blocked, delegated, `TBD`, unsupported, or loss-reported | CAEPIPE-PCF `Boundary Conditions`; PLAN-EXPORT-INTEROP section `5. PCF` |
| Supports and restraints | Support/restraint transfer is classified separately and loss-reported when not reliably preserved by the selected profile | CAEPIPE-PCF `Supports`; PLAN-EXPORT-INTEROP section `5. PCF` |
| Component approximations | Target-side rigid elements, equivalent pipes, concentrated masses, and other simulations are recorded as approximated or delegated behavior | CAEPIPE-PCF component mapping table; DEL-17-02 loss report requirements |
| `TBD` target behavior | `TBD` remains visible in profile, diagnostics, manifest, or loss report and is blocking when solver-ready or compatibility-sensitive | DEL-17-01 TBD-17-01-005; DEL-17-02-REQ-023/053/092 |

## Documentation

The deliverable shall document:

- PCF subset profile and unresolved target-version basis;
- supported entity and attribute classes;
- unsupported, approximated, delegated, omitted, and `TBD` behavior classes;
- stable ID and sidecar mapping policy;
- unit and coordinate assumptions;
- invented fixture provenance;
- loss-report examples using invented data only;
- explicit limitation that PCF is conservative interoperability only.
