# Datasheet: DEL-17-08 GLB/glTF review geometry export

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-08 |
| Package ID | PKG-17 |
| Package name | Export Format Interoperability |
| Deliverable name | GLB/glTF review geometry export |
| Type | BACKEND_FEATURE_SLICE |
| Scope items | SOW-030, SOW-074 |
| Objectives | OBJ-009, OBJ-017 |
| Source basis | DEL-17-01 source-basis register; DEL-17-02 export package/profile/stable-ID contract |
| Lifecycle role | Target-specific Phase A contract draft for review-geometry export |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Target family | GLB/glTF review geometry | `_CONTEXT.md`; SOFTWARE_DECOMP DEL-17-08; PLAN-EXPORT-INTEROP Geometry Exchange |
| Primary output intent | Lightweight 3D visual inspection and review context | PLAN-EXPORT-INTEROP Geometry Exchange |
| Non-authority boundary | Geometry-only review aid; not solver geometry, analysis proof, formal validation, code compliance, release readiness, or professional acceptance | DEL-17-01 F-17-01-005; DEL-17-02 Boundary Requirements; CONTRACT OPS-K-AUTH-1 |
| Format basis | Khronos glTF 2.0 public specification | GLTF-2.0, sections 2.4, 3.4, 4.2-4.4, and 5.24-5.25 |
| Binary container | `.glb` as Binary glTF when a single review artifact is preferred | GLTF-2.0 section 4 |
| JSON-plus-assets option | `.gltf` with external or embedded resources remains a possible profile option | GLTF-2.0 sections 2.4 and 4.1 |
| Coordinate and unit basis | glTF is right-handed, +Y-up, and uses meters for linear distances | GLTF-2.0 section 3.4 |
| Stable identity basis | Canonical OpenPipeStress IDs must be preserved directly through target metadata where supported or through a manifest-referenced sidecar | DEL-17-02 stable ID map requirements |
| Loss reporting | Required even for successful review-geometry exports | DEL-17-02 loss report requirements |

## Conditions

| Condition | Phase A disposition |
|---|---|
| Input source model | Canonical model surface from `schemas/model.schema.yaml`; exact export service boundary remains `TBD`. |
| Export profile | A GLB/glTF profile is required before implementation; target profile fields follow DEL-17-02. |
| Geometry coverage | Centerline, pipe segment, component, support marker, equipment interface, annotation, and load-case visual coverage remain `TBD` by entity family. |
| Visual fidelity | Review geometry may be simplified. It shall not be described as solver-fidelity, analysis-fidelity, or CAD/manufacturing fidelity. |
| Identity metadata | glTF `name`, `extras`, or extensions may be candidates for direct identity metadata; final direct-versus-sidecar policy remains `TBD`. |
| Sidecar identity map | Required when direct glTF metadata is insufficient, stripped by consuming tools, or not appropriate for a canonical ID family. |
| Coordinate conversion | Source-to-glTF axis/origin/scale conversion rules remain `TBD` until the profile declares the source coordinate policy and target transform. |
| Units | Linear geometry emitted to glTF must resolve to meters or carry explicit conversion diagnostics. |
| Private/protected data | Exported metadata and sidecars must not contain private project data, protected standards content, proprietary catalog values, owner criteria, or copied commercial examples unless separately rights-cleared. |

## Construction

The DEL-17-08 export package concept should include, at contract level:

| Package member | Role | Phase A status |
|---|---|---|
| `model.glb` or `model.gltf` | Visual review geometry target artifact | `TBD` exact filename and profile default |
| `review_geometry_profile.json` | Declares target family, glTF version basis, coordinate/unit policy, visual entity coverage, identity policy, and boundary notes | `TBD` schema/location; no schema edit in this phase |
| `id_map.json` | Maps canonical IDs to glTF nodes, meshes, primitives, metadata fields, or omitted entries | Required by DEL-17-02; exact fields `TBD` |
| `export_manifest.json` | Lists package members, source model reference/hash basis, profile, diagnostics, and boundary notes | Required by DEL-17-02; exact fields `TBD` |
| `loss_report.json` and/or `loss_report.md` | Records exported, omitted, approximated, delegated, unsupported, and `TBD` review-geometry behavior | Required by DEL-17-02; exact rendering `TBD` |
| Geometry fixtures | Invented or public-permissive fixtures for deterministic checks | `TBD`; no fixtures created in Phase A |

### Future Data Slots

These slots remain descriptive records, not implementation authority. They are kept as `TBD` until a later profile, schema, or fixture task supplies source-grounded values.

| Slot ID | Future record | Source reread evidence | Phase A disposition |
|---|---|---|---|
| B-001 | Exact package filenames, schema or storage locations, and field sets for `review_geometry_profile.json`, `id_map.json`, manifest, and loss report. | DEL-17-02 Datasheet `## Contract Objects`, `## Required Export-Profile Fields`; DEL-17-02 Specification `## Export Package Requirements`, `## Stable ID Map Requirements`, `## Manifest Requirements`, `## Loss Report Requirements`. | Converted to `TBD`; Construction remains the descriptive inventory. |
| F-002 | Future fixture inventory, fixture rights basis, and fixture-to-check coverage. | IP-DATA `## Public Repository Boundary`; DEL-17-01 Datasheet `## Boundary Facts`; DEL-17-02 Guidance `## Boundary Guidance`. | Converted to `TBD`; no fixtures are created in Phase A. |
| X-002 | Canonical ID families and schema source for emitted and intentionally omitted review entities. | DEL-17-02 Datasheet `## Stable ID Families`; PLAN-EXPORT-INTEROP `## Stable ID Strategy`; `schemas/model.schema.yaml` top-level description. | Converted to `TBD`; canonical family list is not enumerated locally until the source schema/profile boundary is selected. |

## References

| Source ID | Source |
|---|---|
| GLTF-2.0 | Khronos glTF 2.0 specification, `https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html` |
| PLAN-EXPORT-INTEROP | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` |
| DEL-17-01 | `DEL-17-01_CAEPIPE and export-format source basis` four-document kit and `Source_Basis_Register.md` |
| DEL-17-02 | `DEL-17-02_Export package, profile, and stable ID map contracts` four-document kit |
| CONTRACT | `docs/CONTRACT.md` |
| IP-DATA | `docs/IP_AND_DATA_BOUNDARY.md` |
| SPEC / TYPES | `docs/SPEC.md`, `docs/TYPES.md` |
| MODEL-SCHEMA | `schemas/model.schema.yaml` |
