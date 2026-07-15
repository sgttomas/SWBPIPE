---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-07
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-030, SOW-074]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-07

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-07` in service of project scope [SOW-030, SOW-074] and package objectives [OBJ-009, OBJ-017, OBJ-018].

- **OUT-001** — A conservative PCF subset export-profile and deterministic writer contract with explicit mapping, loss, warning, and target-version boundaries is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-07 Conservative PCF subset exporter

> #### Datasheet: DEL-17-07 Conservative PCF subset exporter
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-07-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-07 |
> | Package ID | PKG-17 |
> | Package | Export Format Interoperability |
> | Type | BACKEND_FEATURE_SLICE |
> | Source context | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-17 table |
> | Scope items | SOW-030, SOW-074 |
> | Objectives | OBJ-009, OBJ-017, OBJ-018 |
>

### CLM-004 — Attributes

> ##### Attributes
>
> DEL-17-07 defines the first conservative PCF export slice for broader plant-design interoperability. It is not the first validation backbone, not a complete PCF implementation, not a CAEPIPE compatibility claim, and not professional validation evidence.
>
> | Attribute | Source-grounded value |
> |---|---|
> | Target family | PCF subset export, bounded by `PLAN-EXPORT-INTEROP` section `5. PCF` and CAEPIPE public PCF translator documentation. |
> | Primary output concept | PCF target file plus export package records required by DEL-17-02: manifest, stable ID map, diagnostics, and loss report. |
> | Initial subset candidates | Straight pipe, elbows/bends, tees, reducers, flanges, valves, end connections, line numbers, nominal size, OD/wall-thickness attributes where supported, basic spec/material labels, coordinates, component identifiers, and stable ID sidecar mapping. Source: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section `5. PCF`. |
> | Mandatory reporting | Unsupported, approximated, delegated, omitted, and `TBD` behavior shall be visible in the loss report. Source: DEL-17-02 `Specification.md` loss-report requirements. |
> | Translator-default posture | Hidden translator defaults shall not be silently relied on; they shall be rejected, warned, delegated, or recorded as `TBD`/loss-report entries. Source: `CAEPIPE-PCF` and DEL-17-01 source basis. |
> | Version/profile basis | `TBD`; no first supported PCF target profile or CAEPIPE translator version is selected in this phase. Closure requires admitted public evidence or explicit human project authority. Sources: DEL-17-01 `Source_Basis_Register.md` TBD-17-01-005; `CAEPIPE_Question_Dossier.md` CQ-17-01-006; `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-004/OI-015/OI-017. |
>

### CLM-005 — Profile Classification Record Slot

> ###### Profile Classification Record Slot
>
> The future PCF subset profile shall include a classification record before writer implementation. This slot defines the required record shape only; it does not finalize support status while the target version/profile basis remains `TBD`.
>
> | Candidate family or attribute class | Classification | Evidence basis | Loss-report handling |
> |---|---|---|---|
> | Straight pipe | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table; DEL-17-01 TBD-17-01-005 | Required before export acceptance |
> | Elbows and bends | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table | Required before export acceptance |
> | Tees and branches | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table; branch/SIF semantics remain bounded by project data-boundary controls | Required before export acceptance |
> | Reducers | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table | Required before export acceptance |
> | Flanges | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF translator flange handling option | Required before export acceptance |
> | Valves | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF translator valve handling option and component mapping table | Required before export acceptance |
> | End connections, free ends, equipment nozzles, adjacent pipelines | `TBD` / delegated / unsupported until profile decision | CAEPIPE-PCF `Boundary Conditions`; PLAN-EXPORT-INTEROP section `5. PCF` | Solver-ready boundary semantics must be blocked or loss-reported when not preserved |
> | Line numbers, nominal size, coordinates, component identifiers | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; DEL-17-02 profile and stable-ID requirements | Required for traceability when exported |
> | OD and wall thickness attributes | `TBD` / delegated when external mapping is needed | CAEPIPE-PCF `Wall Thickness`, `OD and Nominal Size`, and `OD and Wall Thickness via config_pcf.ini` | Missing or externally mapped values require diagnostics/loss entries |
> | Basic spec/material labels | `TBD` / delegated for target property mapping | CAEPIPE-PCF `Material`; IP-DATA protected-content boundary | Material property mapping must not be silently inferred |
> | Supports and restraints | `TBD` / delegated / unsupported unless profile proves preservation | CAEPIPE-PCF `Supports`; PLAN-EXPORT-INTEROP section `5. PCF` | Listed separately in export report and loss report |
> | Stable ID sidecar mapping | Candidate exported sidecar | DEL-17-02 stable ID map requirements; PLAN-EXPORT-INTEROP stable identity rules | Required where direct PCF carriage is not source-confirmed |
>

### CLM-006 — Conditions

> ##### Conditions
>
> The CAEPIPE PCF translator source identifies behaviors that constrain this exporter:
>
> | Condition | PCF exporter implication | Source |
> |---|---|---|
> | PCF-to-CAEPIPE conversion can depend on translator dialog fields such as piping code, specific gravity, node numbering, size filtering, vertical axis, flange handling, and valve handling. | DEL-17-07 shall not encode those settings as hidden OpenPipeStress defaults. Any downstream CAEPIPE translation expectation remains `TBD` or delegated target-tool behavior. | `CAEPIPE-PCF`, pages 1-2 |
> | Unit interpretation depends on PCF `UNITS-BORE`. | The profile shall declare PCF unit policy and shall produce diagnostics when source units cannot be represented explicitly. | `CAEPIPE-PCF`, Reference/Units |
> | Temperature, pressure, fluid density, weights, OD, wall thickness, support type, and material mapping have translator-specific fallback behavior. | Missing values and fallback sources shall be reported rather than silently supplied by the exporter. | `CAEPIPE-PCF`, Reference sections |
> | Free ends may be anchored by CAEPIPE translator behavior because PCF lacks certain connection details. | End-connection and boundary-condition semantics shall be marked explicit, unsupported, delegated, or `TBD`; no solver-ready restraint claim is allowed from PCF alone. | `CAEPIPE-PCF`, Boundary Conditions |
> | Some PCF component identifiers are simulated in CAEPIPE as rigid elements, equivalent pipes, concentrated masses, or other target-side approximations. | Component coverage shall distinguish direct export from downstream approximation and shall record affected canonical IDs. | `CAEPIPE-PCF`, component mapping table |
>

### CLM-007 — Construction

> ##### Construction
>
> The deliverable artifacts are expected to include:
>
> | Artifact | Construction rule |
> |---|---|
> | PCF subset profile | Declare profile ID, target family, target version basis, units, coordinates, supported entity families, delegated behavior, unsupported behavior, and `TBD` behavior. The profile shall consume DEL-17-01 and DEL-17-02. |
> | PCF writer | Emit only the conservative subset selected by the profile. Unsupported or unresolved entities shall not be silently omitted. |
> | Unsupported behavior report | Include loss-report categories required by DEL-17-02, with affected canonical IDs, target artifact reference if any, reason, source-basis reference, and downstream implication. |
> | Invented fixtures | Use original/invented fixtures only. Fixtures shall not reproduce proprietary models, protected standards data, vendor examples, owner criteria, protected pipe tables, material allowables, SIF/flexibility tables, or commercial catalog values. |
>

### CLM-008 — References

> ##### References
>
> The references below distinguish source authorities from declared upstream dependencies. DEL-17-01, DEL-17-02, the export plan, CAEPIPE-PCF, and governance documents are source authorities for this documentation pass. DEL-03-02, DEL-13-04, and DEL-15-02 are declared upstream dependencies to review before implementation; they are not additional PCF target-behavior authorities unless their own source-grounded contracts are consumed in a sealed implementation task.
>
> | Source ID | Location | Use |
> |---|---|---|
> | DEL-17-01 | `execution/.../DEL-17-01_CAEPIPE and export-format source basis/` | Source authority, admitted source IDs, PCF caveats, and unresolved questions. |
> | DEL-17-02 | `execution/.../DEL-17-02_Export package, profile, and stable ID map contracts/` | Common export package, profile, stable ID map, manifest, and loss-report contract. |
> | DEL-03-02 | `execution/.../DEL-03-02_Pipe section and component library schema/` | Declared upstream dependency for component/section source data contracts; implementation use remains `TBD` until consumed. |
> | DEL-13-04 | `execution/.../DEL-13-04_Physical-to-analytical transformation contract/` | Declared upstream dependency for transformation/loss semantics; implementation use remains `TBD` until consumed. |
> | DEL-15-02 | `execution/.../DEL-15-02_External-prover handoff package contract/` | Declared upstream dependency for handoff-package alignment; implementation use remains `TBD` until consumed. |
> | PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | PCF target priority, conservative subset candidates, and risk framing. |
> | CAEPIPE-PCF | `https://www.sstusa.com/pdfs/PCF.pdf` | Public evidence for PCF translator behavior, defaults, mappings, and caveats. |
> | CONTRACT / IP-DATA / SPEC / TYPES | `docs/` | Project invariants, protected-content boundary, unit/provenance/no-bypass constraints, and professional-boundary vocabulary. |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-17-07 Conservative PCF subset exporter

> #### Specification: DEL-17-07 Conservative PCF subset exporter
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-07-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> DEL-17-07 shall define and later implement a conservative PCF subset exporter for broader plant-design interoperability. The deliverable shall produce a PCF subset profile, PCF writer behavior, unsupported behavior report, and invented fixtures.
>
> This deliverable shall not claim PCF completeness, CAEPIPE compatibility, solver validation, release readiness, code compliance, professional acceptance, or formal external-tool approval. It shall not implement schema changes or code in this Phase A four-document pass.
>

### CLM-012 — Requirements

> ##### Requirements
>

### CLM-013 — Source Authority Requirements

> ###### Source Authority Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-07-REQ-001 | The PCF subset profile shall consume DEL-17-01 as the source-basis authority for PCF target behavior claims. |
> | DEL-17-07-REQ-002 | The PCF subset profile shall consume DEL-17-02 for export package, profile, stable ID map, manifest, and loss-report rules. |
> | DEL-17-07-REQ-003 | Target behavior not supported by admitted public/project-owned evidence shall remain `TBD`, delegated, unsupported, or loss-reported. |
> | DEL-17-07-REQ-004 | The exporter shall not rely on hidden translator defaults for stress-relevant or identity-relevant behavior. |
>

### CLM-014 — Profile Requirements

> ###### Profile Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-07-REQ-010 | The PCF profile shall declare `target_family = PCF` and shall include profile ID, profile version, target version basis, source-basis IDs, and boundary notes. |
> | DEL-17-07-REQ-011 | The PCF profile shall declare unit policy, coordinate policy, stable-ID policy, line/component identity policy, and loss-report policy. |
> | DEL-17-07-REQ-012 | The target version basis shall remain `TBD` until public evidence or human project authority selects a first PCF target profile. |
> | DEL-17-07-REQ-013 | The profile shall classify each candidate entity family as exported, omitted, approximated, delegated, unsupported, or `TBD`. |
> | DEL-17-07-REQ-014 | The initial candidate subset shall be limited to the plan-listed PCF entities and attributes unless later source review narrows or expands it. |
> | DEL-17-07-REQ-015 | The PCF profile shall carry DEL-17-01, DEL-17-02, CAEPIPE-PCF, and PLAN-EXPORT-INTEROP source-basis references; missing references shall block package acceptance. |
>

### CLM-015 — Writer Requirements

> ###### Writer Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-07-REQ-020 | The PCF writer shall emit only profile-supported content from unit-aware, provenance-bearing OpenPipeStress model data. |
> | DEL-17-07-REQ-021 | The writer shall produce diagnostics for missing or ambiguous units, coordinates, nominal size, OD, wall thickness, material/spec labels, component identity, end-connection data, and support/restraint semantics when those fields affect the selected profile. |
> | DEL-17-07-REQ-022 | The writer shall not invent pressure, temperature, fluid density, component weight, OD, wall thickness, material properties, support type, equipment connection, or boundary-condition data. |
> | DEL-17-07-REQ-023 | The writer shall preserve canonical OpenPipeStress identity through target metadata when explicitly supported or through a sidecar stable ID map when direct PCF carriage is not source-confirmed. |
> | DEL-17-07-REQ-024 | The writer shall record target-generated identity separately from canonical OpenPipeStress identity. |
>

### CLM-016 — Unsupported and Loss-Report Requirements

> ###### Unsupported and Loss-Report Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-07-REQ-030 | Every PCF export attempt shall produce or reference a loss report. |
> | DEL-17-07-REQ-031 | Loss report entries shall include affected canonical IDs, category, severity, target artifact if any, reason, source-basis reference, and downstream implication, consistent with DEL-17-02. |
> | DEL-17-07-REQ-032 | Support/restraint semantics shall be listed separately when the PCF target path cannot preserve them reliably. |
> | DEL-17-07-REQ-033 | Free-end, equipment-nozzle, adjacent-pipeline, and boundary-condition semantics shall be blocked, `TBD`, delegated, or loss-reported when the target path would otherwise depend on translator anchoring or incomplete PCF connection data. |
> | DEL-17-07-REQ-034 | Component mappings known to become target-side rigid elements, equivalent pipes, concentrated masses, or other approximations shall be recorded as approximated or delegated behavior rather than direct support. |
>

### CLM-017 — Fixture and Data-Boundary Requirements

> ###### Fixture and Data-Boundary Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-07-REQ-040 | Fixtures shall be invented or otherwise redistribution-safe with documented provenance. |
> | DEL-17-07-REQ-041 | Fixtures shall not copy proprietary plant models, vendor examples, protected standards text, protected tables, material allowables, SIF/flexibility values, owner criteria, or commercial catalog values. |
> | DEL-17-07-REQ-042 | Public fixtures shall avoid code-specific compliance semantics and shall not imply professional acceptance. |
>

### CLM-018 — Standards

> ##### Standards
>
> | Source | Status in this phase |
> |---|---|
> | Public CAEPIPE PCF translator documentation (`CAEPIPE-PCF`) | Accessible source for translator behavior, defaults, and mappings; not authority for OpenPipeStress completeness or compatibility claims. |
> | DEL-17-01 source basis | Governing project source-basis authority for downstream PCF claims. |
> | DEL-17-02 export contract | Governing project contract for profile, manifest, ID map, and loss reporting. |
> | PCF format specification | `TBD`; no redistributed proprietary specification text is admitted in this phase. |
> | Engineering design codes and standards | Out of scope for this exporter; no code-compliance or professional-approval claim is permitted. |
>

### CLM-019 — Verification

> ##### Verification
>
> | Requirement group | Verification approach |
> |---|---|
> | Source authority | Review that each PCF behavior statement cites DEL-17-01, DEL-17-02, `PLAN-EXPORT-INTEROP`, `CAEPIPE-PCF`, or project governance sources. |
> | Conservative profile | Check that every selected entity family is classified and that unresolved behavior remains `TBD`. |
> | Writer behavior | Future implementation tests shall use invented fixtures and verify that supported records are emitted deterministically and unsupported content produces diagnostics/loss entries. |
> | Units and coordinates | Future tests shall include explicit unit-bearing source data and ambiguous/missing-unit negative cases. Acceptance criteria remain profile-bound: after the PCF profile is selected, tests shall verify declared PCF unit representation, coordinate policy, rejection or diagnostic behavior for missing/ambiguous unit metadata, and no dimensionless fallback for unit-bearing physical values. Sources: CAEPIPE-PCF `Reference / Units`; `docs/SPEC.md` section `3.1 Unit contract`; DEL-17-02 export profile requirements. |
> | Stable IDs | Future tests shall verify sidecar identity mapping when direct PCF identity carriage is not source-confirmed. |
> | Loss report | Future tests shall assert entries for omitted, approximated, delegated, unsupported, and `TBD` behavior. |
> | Boundary controls | Review for absence of proprietary examples, protected standards data, release claims, compatibility claims, code-compliance claims, and professional-acceptance claims. |
>

### CLM-020 — Diagnostic and Loss-Report Acceptance Scaffold

> ###### Diagnostic and Loss-Report Acceptance Scaffold
>
> The future implementation tranche shall convert the following scaffold into concrete tests only after the PCF profile is sealed. Until then, expected outcome values remain `TBD` rather than implied support claims.
>
> | Coverage class | Minimum acceptance hook | Source basis |
> |---|---|---|
> | Missing or ambiguous units | Diagnostic or blocking loss entry; no silent default for unit-bearing physical values | CAEPIPE-PCF `Reference / Units`; `docs/SPEC.md` section `3.1 Unit contract`; DEL-17-02-REQ-021/050/051/053 |
> | Coordinates and vertical axis | Profile-declared coordinate/vertical-axis policy; diagnostic when source coordinate basis is absent or ambiguous | CAEPIPE-PCF translator dialog vertical-axis option; DEL-17-02-REQ-021 |
> | Nominal size, OD, and wall thickness | Export only explicit/source-confirmed attributes where selected; external mapping-database behavior is delegated or loss-reported | CAEPIPE-PCF `Wall Thickness`, `OD and Nominal Size`, and `OD and Wall Thickness via config_pcf.ini` |
> | Material/spec labels | Labels may be emitted only with source-owned provenance; target material-property mapping is delegated unless source-confirmed | CAEPIPE-PCF `Material`; IP-DATA protected-content boundary |
> | Canonical identity and target identity | Stable ID sidecar or target metadata links canonical IDs to emitted records, omissions, diagnostics, and loss rows; target-generated IDs remain separate | DEL-17-02 stable ID map requirements |
> | End connections and boundary conditions | Free ends, equipment nozzles, adjacent pipelines, and unresolved boundary semantics are blocked, delegated, `TBD`, unsupported, or loss-reported | CAEPIPE-PCF `Boundary Conditions`; PLAN-EXPORT-INTEROP section `5. PCF` |
> | Supports and restraints | Support/restraint transfer is classified separately and loss-reported when not reliably preserved by the selected profile | CAEPIPE-PCF `Supports`; PLAN-EXPORT-INTEROP section `5. PCF` |
> | Component approximations | Target-side rigid elements, equivalent pipes, concentrated masses, and other simulations are recorded as approximated or delegated behavior | CAEPIPE-PCF component mapping table; DEL-17-02 loss report requirements |
> | `TBD` target behavior | `TBD` remains visible in profile, diagnostics, manifest, or loss report and is blocking when solver-ready or compatibility-sensitive | DEL-17-01 TBD-17-01-005; DEL-17-02-REQ-023/053/092 |
>

### CLM-021 — Documentation

> ##### Documentation
>
> The deliverable shall document:
>
> - PCF subset profile and unresolved target-version basis;
> - supported entity and attribute classes;
> - unsupported, approximated, delegated, omitted, and `TBD` behavior classes;
> - stable ID and sidecar mapping policy;
> - unit and coordinate assumptions;
> - invented fixture provenance;
> - loss-report examples using invented data only;
> - explicit limitation that PCF is conservative interoperability only.

- **AC-001** — The contract preserves admitted PCF source authority, selected profile/version gates, canonical identity, units and coordinates, conservative component and attribute mapping, explicit exported/omitted/approximated/delegated/unsupported/TBD behavior, diagnostics, loss reporting, invented or rights-cleared fixtures, and professional non-authority without implying broad PCF compatibility.

## Production and Verification Method — Praxeology

### CLM-022 — Procedure: DEL-17-07 Conservative PCF subset exporter

> #### Procedure: DEL-17-07 Conservative PCF subset exporter
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-023 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-07-DECL-004`.
>

### CLM-024 — Purpose

> ##### Purpose
>
> This procedure describes how to produce and use the conservative PCF subset exporter deliverable without relying on hidden translator defaults or making unsupported interoperability claims.
>

### CLM-025 — Prerequisites

> ##### Prerequisites
>
> - DEL-17-01 source basis has been consumed for PCF source evidence and unresolved questions.
> - DEL-17-02 export package/profile/stable-ID/loss-report contract has been consumed.
> - Declared upstream dependencies are reviewed: `DEL-17-02`, `DEL-03-02`, `DEL-13-04`, and `DEL-15-02`.
> - PCF source behavior is limited to accessible public/project-owned evidence.
> - Fixture data is invented or otherwise redistribution-safe.
> - No protected standards text, proprietary examples, private project data, commercial solver files, or license-bypass instructions are used.
>

### CLM-026 — Steps

> ##### Steps
>

### CLM-027 — 1. Establish the source basis

> ###### 1. Establish the source basis
>
> 1. Read DEL-17-01 `Source_Basis_Register.md` and `CAEPIPE_Question_Dossier.md`.
> 2. Read DEL-17-02 `Specification.md` for profile, ID-map, manifest, and loss-report requirements.
> 3. Read `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section `5. PCF`.
> 4. Read the admitted public PCF translator source slices before making any PCF behavior claim.
> 5. Mark unsupported, source-absent, or version-sensitive behavior as `TBD`.
>

### CLM-028 — 2. Define the PCF profile

> ###### 2. Define the PCF profile
>
> 1. Create a profile record for the PCF target family.
> 2. Declare profile ID, profile version, target version basis, source-basis IDs, and boundary notes.
> 3. Declare unit policy, coordinate policy, identity policy, supported entity families, and loss-report policy.
> 4. Classify each candidate entity family as exported, omitted, approximated, delegated, unsupported, or `TBD`.
> 5. Keep target version/profile basis as `TBD` until public evidence or human project authority resolves it.
>

### CLM-029 — 3. Select the conservative subset

> ###### 3. Select the conservative subset
>
> 1. Start from the plan-listed subset only.
> 2. Keep straight-pipe, bend, tee, reducer, flange, valve, end-connection, line/component identity, nominal-size, OD/wall-thickness, material/spec label, coordinate, and stable-ID behavior separate in the profile.
> 3. Require explicit unit-bearing and provenance-bearing source data for exported physical values.
> 4. Move support/restraint semantics to separate handling unless the selected profile proves reliable preservation.
> 5. Do not add hidden defaults for missing pressure, temperature, fluid density, component weight, material properties, support type, or boundary conditions.
>

### CLM-030 — 4. Implement writer behavior in a later implementation tranche

> ###### 4. Implement writer behavior in a later implementation tranche
>
> 1. Emit only profile-supported PCF records.
> 2. Emit deterministic ordering where the profile and package contract require it.
> 3. Preserve canonical IDs directly only when source-confirmed; otherwise write a stable ID sidecar.
> 4. Produce diagnostics when required source data is missing, ambiguous, unsupported, or delegated.
> 5. Produce a loss report for every export attempt.
> 6. Keep PCF writing inside the approved deliverable write scope for the implementation tranche.
>

### CLM-031 — 5. Build invented fixtures

> ###### 5. Build invented fixtures
>
> 1. Use invented model names, line IDs, component IDs, coordinates, and unit-bearing values.
> 2. Record fixture provenance and redistribution status.
> 3. Include positive cases for supported subset records.
> 4. Include negative cases for unsupported supports/restraints, ambiguous units, missing OD/wall thickness, unsupported components, and unresolved boundary conditions.
> 5. Do not use proprietary plant models, vendor examples, protected standards data, owner criteria, commercial catalog values, material allowables, or SIF/flexibility data.
>
> Fixture provenance records should use this minimum template before any fixture is admitted:
>
> | Field | Required content |
> |---|---|
> | Fixture ID | Stable invented fixture identifier |
> | Fixture purpose | Positive or negative case covered by the PCF profile |
> | Invented model name | Artificial name with no client/vendor/project source |
> | Invented line IDs and component IDs | Artificial IDs traceable to canonical OpenPipeStress IDs |
> | Coordinates and unit-bearing values | Explicit units, dimensions, and provenance note that values are invented |
> | Entity classes covered | Candidate PCF component/support/attribute classes exercised |
> | Redistribution status | `public_permissive`, `private_only`, `unknown`, or `protected_suspected` as applicable |
> | Contributor certification | Statement that the fixture is not copied from protected standards, vendor examples, proprietary plant models, owner criteria, catalog values, material allowables, or SIF/flexibility data |
> | Review disposition | Accepted, rejected, quarantined, or human/legal review required |
>
> Sources: `docs/IP_AND_DATA_BOUNDARY.md` public-data provenance and protected-content rules; DEL-17-02 fixture/data-boundary requirements; Guidance `Examples`.
>

### CLM-032 — 6. Verify outputs

> ###### 6. Verify outputs
>
> 1. Confirm the PCF target file is emitted only for supported profile content.
> 2. Confirm the manifest references the profile, source basis, target file, stable ID sidecar, diagnostics, and loss report.
> 3. Confirm the stable ID sidecar links canonical IDs to emitted target records or loss-report entries.
> 4. Confirm every unsupported, approximated, delegated, omitted, or `TBD` behavior appears in the loss report.
> 5. Confirm no release, compatibility, code-compliance, formal-validation, or professional-acceptance claim is present.
> 6. Confirm generated PCF export evidence is not described as downstream import compatibility, solver validation, release readiness, code compliance, professional acceptance, or external-tool approval.
>

### CLM-033 — Verification

> ##### Verification
>
> For this Phase A documentation pass, verification consists of:
>
> - confirming the four-document kit exists;
> - checking the default section headings required by the four-documents skill;
> - checking that source-grounding gaps remain `TBD`, assumptions, or conflict-table entries;
> - checking that no implementation code, schema files, protected standards data, proprietary examples, or professional/release/compatibility claims were introduced.
>
> Future implementation verification remains `TBD` and should be defined only after the PCF profile and writer tranche are sealed.
>

### CLM-034 — Records

> ##### Records
>
> The deliverable should retain or produce:
>
> - PCF subset profile;
> - PCF target file;
> - export manifest;
> - stable ID sidecar map;
> - diagnostics;
> - loss report;
> - invented fixture provenance records;
> - run records for TASK execution and validation.

- **VER-001** — Validate the contract and review source parity, PCF source/profile gates, deterministic output, identity and unit treatment, every mapping and loss class, warning policy, retained version and mapping TBDs, fixture provenance, protected-content exclusions, and absence of compatibility, release, compliance, or professional-acceptance claims.

## Governing Values and Decisions — Axiology

### CLM-035 — Guidance: DEL-17-07 Conservative PCF subset exporter

> #### Guidance: DEL-17-07 Conservative PCF subset exporter
>

### CLM-036 — Purpose

> ##### Purpose
>
> Use this deliverable to make PCF useful without overstating it. The exporter should help users exchange plant-design geometry and component context, while making every unsupported, approximate, delegated, or unknown behavior visible.
>
> PCF is a secondary interoperability path in the PKG-17 strategy. The public CAEPIPE PCF translator documentation shows enough fallback and mapping behavior that this path should not be treated as the first deterministic solver-validation backbone.
>

### CLM-037 — Principles

> ##### Principles
>

### CLM-038 — Keep the subset narrow

> ###### Keep the subset narrow
>
> Start from the plan-listed conservative subset: straight pipe, elbows/bends, tees, reducers, flanges, valves, end connections, line numbers, nominal size, OD/wall-thickness attributes where supported, basic spec/material labels, coordinates, component identifiers, and stable ID sidecar mapping.
>
> Do not expand the subset because a downstream translator might infer something. If source evidence does not confirm the behavior, mark it `TBD`, unsupported, or delegated.
>

### CLM-039 — Make defaults explicit

> ###### Make defaults explicit
>
> The CAEPIPE PCF translator source documents default or external behaviors for areas such as units, load data, fluid specific gravity, weight, OD/wall thickness, support mapping, material mapping, boundary conditions, and component simulation. Those are target-path facts, not OpenPipeStress exporter defaults.
>
> For this deliverable, a hidden default is a reportable condition:
>
> - if OpenPipeStress has explicit source data and the PCF profile can carry it, export it;
> - if the target path may infer it from translator configuration, record delegated behavior;
> - if the behavior affects solver-ready interpretation and is not source-confirmed, block or mark `TBD`;
> - if a component is represented by a target-side approximation, record the approximation and affected canonical IDs.
>

### CLM-040 — Preserve identity even when PCF cannot

> ###### Preserve identity even when PCF cannot
>
> Stable ID sidecars are expected for this slice unless direct PCF identity carriage is later source-confirmed. The sidecar should connect canonical model IDs to target records, omitted entries, diagnostics, and loss-report rows.
>

### CLM-041 — Treat support and restraint semantics cautiously

> ###### Treat support and restraint semantics cautiously
>
> Support/restraint semantics should be reviewed separately from geometry/component export. Public PCF translator evidence indicates that support transfer can depend on PCF attributes, SKEY values, mapping databases, options, stiffness/gap/friction customization, or default hanger behavior. The first exporter should not imply reliable preservation of support behavior unless the profile explicitly proves it.
>
> Use the following criteria when classifying support/restraint outcomes in the future profile:
>
> | Outcome | Use when |
> |---|---|
> | Reliably preserved | The selected profile has source-confirmed PCF attributes, mapping rules, units, coordinates, support type, stiffness/gap/friction semantics where applicable, and stable ID traceability for the supported support/restraint class. |
> | Delegated | The downstream translator or user-owned mapping database may decide the final support type or properties, and OpenPipeStress can identify that delegation without treating it as local support preservation. |
> | `TBD` | Public/project-owned evidence does not yet resolve whether the selected profile can preserve the support/restraint class. |
> | Unsupported | The selected profile cannot carry the required semantics or the project intentionally excludes the class from the conservative subset. |
> | Blocked | Loss of support/restraint meaning would make the exported package misleading for solver-ready or compatibility-sensitive use. |
>
> This classification belongs in the profile and loss report, not in hidden writer defaults. Sources: CAEPIPE-PCF `Supports`; PLAN-EXPORT-INTEROP section `5. PCF`; DEL-17-02 loss-report requirements.
>

### CLM-042 — Avoid professional and compatibility overclaims

> ###### Avoid professional and compatibility overclaims
>
> Successful PCF file creation is export evidence only. It is not proof that a downstream tool imported the model correctly, that stress-model semantics are complete, that a design complies with any code, or that a professional has accepted the work.
>

### CLM-043 — Considerations

> ##### Considerations
>
> | Topic | Guidance |
> |---|---|
> | Units | Require explicit unit policy. Do not use dimensionless or missing unit metadata as fallback. |
> | OD and wall thickness | Prefer explicit source attributes where supported by the selected PCF profile. External mapping database fallback is delegated target behavior, not local data. |
> | Pressure and temperature | Do not create load values from translator defaults. Missing values remain diagnostics, `TBD`, or loss-report entries. |
> | Material/spec labels | Labels may be exported if source-owned, but target material property mapping remains delegated unless source-confirmed. |
> | Component mappings | Direct component export and target-side simulation are different conditions. Record approximations. |
> | Free ends and equipment connections | Avoid implying boundary-condition preservation when PCF lacks connection detail. |
> | CAEPIPE notes | CAEPIPE-PCF behavior may be documented as a secondary route only. It must not become a compatibility claim. |
>

### CLM-044 — Trade-offs

> ##### Trade-offs
>
> | Choice | Benefit | Cost |
> |---|---|---|
> | Narrow profile with strong diagnostics | Avoids hidden behavior and supports auditability. | Some models will export with many unsupported/TBD entries. |
> | Sidecar stable ID map | Preserves traceability even when PCF cannot carry canonical IDs directly. | Requires package consumers to keep target file and sidecar together. |
> | Blocking support/restraint ambiguity | Reduces risk of misleading stress-model handoff. | Limits usefulness for downstream tools that tolerate inferred support behavior. |
> | Invented-only fixtures | Preserves IP and standards boundaries. | Does not prove compatibility with real proprietary plant models. |
>

### CLM-045 — Examples

> ##### Examples
>
> No proprietary, vendor, standards-derived, or client examples are admitted. Future examples for this deliverable should use small invented models with clearly artificial names, dimensions, units, and provenance. Example values shall not be copied from protected standards, catalogs, or owner design bases.
>

### CLM-046 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> Keep the profile/version conflict open until an admitted public source or explicit human project authority selects the first supported PCF target profile. The plan-listed subset is candidate scope, not final support classification.
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | DEL-17-07-CF-001 | The plan lists an initial conservative PCF subset, but exact PCF target-version/profile behavior is not selected. | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section `5. PCF` | DEL-17-01 `Source_Basis_Register.md` TBD-17-01-005 and `CAEPIPE_Question_Dossier.md` CQ-17-01-006 | Datasheet Attributes; Specification Profile Requirements; Procedure Step 3 | Treat plan-listed subset as candidate scope only; keep profile version and final support classes `TBD` until downstream source/profile decision. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 OBJ-018 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
