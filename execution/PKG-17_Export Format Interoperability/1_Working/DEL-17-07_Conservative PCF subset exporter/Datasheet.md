# Datasheet: DEL-17-07 Conservative PCF subset exporter

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-07 |
| Package ID | PKG-17 |
| Package | Export Format Interoperability |
| Type | BACKEND_FEATURE_SLICE |
| Source context | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-17 table |
| Scope items | SOW-030, SOW-074 |
| Objectives | OBJ-009, OBJ-017, OBJ-018 |

## Attributes

DEL-17-07 defines the first conservative PCF export slice for broader plant-design interoperability. It is not the first validation backbone, not a complete PCF implementation, not a CAEPIPE compatibility claim, and not professional validation evidence.

| Attribute | Source-grounded value |
|---|---|
| Target family | PCF subset export, bounded by `PLAN-EXPORT-INTEROP` section `5. PCF` and CAEPIPE public PCF translator documentation. |
| Primary output concept | PCF target file plus export package records required by DEL-17-02: manifest, stable ID map, diagnostics, and loss report. |
| Initial subset candidates | Straight pipe, elbows/bends, tees, reducers, flanges, valves, end connections, line numbers, nominal size, OD/wall-thickness attributes where supported, basic spec/material labels, coordinates, component identifiers, and stable ID sidecar mapping. Source: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` section `5. PCF`. |
| Mandatory reporting | Unsupported, approximated, delegated, omitted, and `TBD` behavior shall be visible in the loss report. Source: DEL-17-02 `Specification.md` loss-report requirements. |
| Translator-default posture | Hidden translator defaults shall not be silently relied on; they shall be rejected, warned, delegated, or recorded as `TBD`/loss-report entries. Source: `CAEPIPE-PCF` and DEL-17-01 source basis. |
| Version/profile basis | `TBD`; no first supported PCF target profile or CAEPIPE translator version is selected in this phase. Closure requires admitted public evidence or explicit human project authority. Sources: DEL-17-01 `Source_Basis_Register.md` TBD-17-01-005; `CAEPIPE_Question_Dossier.md` CQ-17-01-006; `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-004/OI-015/OI-017. |

### Profile Classification Record Slot

The future PCF subset profile shall include a classification record before writer implementation. This slot defines the required record shape only; it does not finalize support status while the target version/profile basis remains `TBD`.

| Candidate family or attribute class | Classification | Evidence basis | Loss-report handling |
|---|---|---|---|
| Straight pipe | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table; DEL-17-01 TBD-17-01-005 | Required before export acceptance |
| Elbows and bends | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table | Required before export acceptance |
| Tees and branches | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table; branch/SIF semantics remain bounded by project data-boundary controls | Required before export acceptance |
| Reducers | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF component mapping table | Required before export acceptance |
| Flanges | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF translator flange handling option | Required before export acceptance |
| Valves | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; CAEPIPE-PCF translator valve handling option and component mapping table | Required before export acceptance |
| End connections, free ends, equipment nozzles, adjacent pipelines | `TBD` / delegated / unsupported until profile decision | CAEPIPE-PCF `Boundary Conditions`; PLAN-EXPORT-INTEROP section `5. PCF` | Solver-ready boundary semantics must be blocked or loss-reported when not preserved |
| Line numbers, nominal size, coordinates, component identifiers | `TBD` | PLAN-EXPORT-INTEROP section `5. PCF`; DEL-17-02 profile and stable-ID requirements | Required for traceability when exported |
| OD and wall thickness attributes | `TBD` / delegated when external mapping is needed | CAEPIPE-PCF `Wall Thickness`, `OD and Nominal Size`, and `OD and Wall Thickness via config_pcf.ini` | Missing or externally mapped values require diagnostics/loss entries |
| Basic spec/material labels | `TBD` / delegated for target property mapping | CAEPIPE-PCF `Material`; IP-DATA protected-content boundary | Material property mapping must not be silently inferred |
| Supports and restraints | `TBD` / delegated / unsupported unless profile proves preservation | CAEPIPE-PCF `Supports`; PLAN-EXPORT-INTEROP section `5. PCF` | Listed separately in export report and loss report |
| Stable ID sidecar mapping | Candidate exported sidecar | DEL-17-02 stable ID map requirements; PLAN-EXPORT-INTEROP stable identity rules | Required where direct PCF carriage is not source-confirmed |

## Conditions

The CAEPIPE PCF translator source identifies behaviors that constrain this exporter:

| Condition | PCF exporter implication | Source |
|---|---|---|
| PCF-to-CAEPIPE conversion can depend on translator dialog fields such as piping code, specific gravity, node numbering, size filtering, vertical axis, flange handling, and valve handling. | DEL-17-07 shall not encode those settings as hidden OpenPipeStress defaults. Any downstream CAEPIPE translation expectation remains `TBD` or delegated target-tool behavior. | `CAEPIPE-PCF`, pages 1-2 |
| Unit interpretation depends on PCF `UNITS-BORE`. | The profile shall declare PCF unit policy and shall produce diagnostics when source units cannot be represented explicitly. | `CAEPIPE-PCF`, Reference/Units |
| Temperature, pressure, fluid density, weights, OD, wall thickness, support type, and material mapping have translator-specific fallback behavior. | Missing values and fallback sources shall be reported rather than silently supplied by the exporter. | `CAEPIPE-PCF`, Reference sections |
| Free ends may be anchored by CAEPIPE translator behavior because PCF lacks certain connection details. | End-connection and boundary-condition semantics shall be marked explicit, unsupported, delegated, or `TBD`; no solver-ready restraint claim is allowed from PCF alone. | `CAEPIPE-PCF`, Boundary Conditions |
| Some PCF component identifiers are simulated in CAEPIPE as rigid elements, equivalent pipes, concentrated masses, or other target-side approximations. | Component coverage shall distinguish direct export from downstream approximation and shall record affected canonical IDs. | `CAEPIPE-PCF`, component mapping table |

## Construction

The deliverable artifacts are expected to include:

| Artifact | Construction rule |
|---|---|
| PCF subset profile | Declare profile ID, target family, target version basis, units, coordinates, supported entity families, delegated behavior, unsupported behavior, and `TBD` behavior. The profile shall consume DEL-17-01 and DEL-17-02. |
| PCF writer | Emit only the conservative subset selected by the profile. Unsupported or unresolved entities shall not be silently omitted. |
| Unsupported behavior report | Include loss-report categories required by DEL-17-02, with affected canonical IDs, target artifact reference if any, reason, source-basis reference, and downstream implication. |
| Invented fixtures | Use original/invented fixtures only. Fixtures shall not reproduce proprietary models, protected standards data, vendor examples, owner criteria, protected pipe tables, material allowables, SIF/flexibility tables, or commercial catalog values. |

## References

The references below distinguish source authorities from declared upstream dependencies. DEL-17-01, DEL-17-02, the export plan, CAEPIPE-PCF, and governance documents are source authorities for this documentation pass. DEL-03-02, DEL-13-04, and DEL-15-02 are declared upstream dependencies to review before implementation; they are not additional PCF target-behavior authorities unless their own source-grounded contracts are consumed in a sealed implementation task.

| Source ID | Location | Use |
|---|---|---|
| DEL-17-01 | `execution/.../DEL-17-01_CAEPIPE and export-format source basis/` | Source authority, admitted source IDs, PCF caveats, and unresolved questions. |
| DEL-17-02 | `execution/.../DEL-17-02_Export package, profile, and stable ID map contracts/` | Common export package, profile, stable ID map, manifest, and loss-report contract. |
| DEL-03-02 | `execution/.../DEL-03-02_Pipe section and component library schema/` | Declared upstream dependency for component/section source data contracts; implementation use remains `TBD` until consumed. |
| DEL-13-04 | `execution/.../DEL-13-04_Physical-to-analytical transformation contract/` | Declared upstream dependency for transformation/loss semantics; implementation use remains `TBD` until consumed. |
| DEL-15-02 | `execution/.../DEL-15-02_External-prover handoff package contract/` | Declared upstream dependency for handoff-package alignment; implementation use remains `TBD` until consumed. |
| PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | PCF target priority, conservative subset candidates, and risk framing. |
| CAEPIPE-PCF | `https://www.sstusa.com/pdfs/PCF.pdf` | Public evidence for PCF translator behavior, defaults, mappings, and caveats. |
| CONTRACT / IP-DATA / SPEC / TYPES | `docs/` | Project invariants, protected-content boundary, unit/provenance/no-bypass constraints, and professional-boundary vocabulary. |
