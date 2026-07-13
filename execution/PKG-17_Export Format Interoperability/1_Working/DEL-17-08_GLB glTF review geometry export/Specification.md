# Specification: DEL-17-08 GLB/glTF review geometry export

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-08-ACC-005`, `DEL-17-08-DECL-001`, `DEL-17-08-EXC-001`, `DEL-17-08-EXC-002`.

## Scope

DEL-17-08 defines and implements a bounded JSON `.gltf` review-geometry profile for lightweight visual inspection, review context, and stable identity correlation back to canonical model entities. Under `DEC-074` O11/E7, the selected profile remains JSON glTF with embedded buffer data, line-mode centerline segments, direct `extras`, and an authoritative sidecar ID map.

This bounded profile does not select or imply binary GLB, broad geometry coverage, viewer compatibility, engineering validation, an API endpoint, release path, external-tool certification, solver geometry, stress analysis handoff, code-compliance process, or professional acceptance workflow.

## Requirements

### Source and Boundary Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-08-REQ-001 | The review-geometry export shall consume DEL-17-01 as the source-basis authority for GLB/glTF boundary claims. |
| DEL-17-08-REQ-002 | The review-geometry export shall consume DEL-17-02 for export package, profile, stable-ID map, manifest, and loss-report behavior. |
| DEL-17-08-REQ-003 | Target behavior not confirmed by GLTF-2.0, DEL-17-01, DEL-17-02, or project governance sources shall remain `TBD`. |
| DEL-17-08-REQ-004 | The export shall be described only as visual review geometry and shall not be represented as solver-fidelity geometry, analysis-fidelity geometry, formal validation evidence, code-compliance evidence, release readiness, target compatibility, or professional acceptance. |
| DEL-17-08-REQ-005 | The export shall not include protected standards text, protected tables, private project data, proprietary target examples, owner criteria, copied commercial files, or undocumented proprietary behavior. |

### glTF/GLB Format Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-08-REQ-010 | A GLB/glTF review-geometry profile shall declare the glTF version basis and whether the emitted target is `.glb`, `.gltf`, or both. |
| DEL-17-08-REQ-011 | A glTF asset shall include the required `asset.version` field required by GLTF-2.0. |
| DEL-17-08-REQ-012 | Linear geometry emitted into glTF shall use meters or shall produce a blocking diagnostic before target writing. |
| DEL-17-08-REQ-013 | The profile shall declare the source coordinate basis, target glTF basis, and transform policy, including the glTF +Y-up/right-handed convention. |
| DEL-17-08-REQ-014 | Mesh primitive topology choices shall be profile-declared. Candidate modes include line-style centerline review geometry and triangle-style surface preview geometry; the selected subset remains `TBD`. |
| DEL-17-08-REQ-015 | Use of glTF extensions, if any, shall follow the GLTF-2.0 extension declaration mechanism and shall remain optional unless a later profile explicitly records the viewer impact. |

### Identity and Metadata Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-08-REQ-020 | Within the selected centerline JSON glTF profile, every emitted entity shall round trip one-to-one between canonical identity in node/primitive `extras` and the authoritative sidecar ID map; intentionally omitted entities remain represented through the loss report. |
| DEL-17-08-REQ-021 | Direct glTF metadata may use `name`, `extras`, or a declared extension only when the profile records the target location and stripping/consumer-risk policy. |
| DEL-17-08-REQ-022 | A sidecar ID map shall be emitted when direct glTF metadata is unavailable, insufficient, not profile-approved, or not reliable for a receiving workflow. |
| DEL-17-08-REQ-023 | The ID map shall distinguish canonical OpenPipeStress IDs from glTF node indices, mesh indices, primitive indices, generated display names, and sidecar row IDs. |
| DEL-17-08-REQ-024 | Omitted canonical entities shall appear in the ID map or loss report with omission reason, affected canonical ID, severity, and downstream implication. |

### Export Package Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-08-REQ-030 | A review-geometry export package shall include or reference a manifest, ID map, loss report, diagnostics, and the emitted GLB/glTF artifact. |
| DEL-17-08-REQ-031 | The manifest shall record source model identity, export profile identity, package member inventory, source-basis references, and boundary notes. |
| DEL-17-08-REQ-032 | The loss report shall classify each material review-geometry limitation as exported, omitted, approximated, delegated, unsupported, or `TBD`. |
| DEL-17-08-REQ-033 | Binary GLB or non-JSON package members shall record a hash or explain why a hash is unavailable. |
| DEL-17-08-REQ-034 | The current bounded implementation uses a fixed versioned generator string and emits no timestamp. The exact normative timestamp/generator policy remains owner-unselected under PDU-031 and shall not be invented by implementation evidence. |
| DEL-17-08-REQ-035 | The review-geometry profile shall carry DEL-17-01, DEL-17-02, and GLTF-2.0 source-basis references; missing references shall block package acceptance. |

### Review Geometry Coverage Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-08-REQ-040 | The profile shall declare which canonical entity families are visually emitted, omitted, approximated, unsupported, or `TBD`. |
| DEL-17-08-REQ-041 | Centerline segments, bends, branches, reducers, components, supports/restraints, equipment interfaces, line labels, material/section labels, load-case indicators, and diagnostics overlays remain `TBD` until the profile selects first coverage. |
| DEL-17-08-REQ-042 | Simplified review shapes shall be recorded as approximations when they replace richer canonical model geometry. |
| DEL-17-08-REQ-043 | Missing units, ambiguous coordinate systems, duplicate stable IDs, zero-length geometry, impossible bend display geometry, and unmapped emitted entities shall produce blocking diagnostics before package acceptance. |

## Standards

| Standard or source | Use | Boundary |
|---|---|---|
| Khronos glTF 2.0 specification | Public target-format reference for asset structure, coordinate/unit convention, GLB container behavior, nodes, meshes, primitive modes, extensions, and `extras` metadata slots | Not a piping stress, solver, code-compliance, or professional-review standard |
| OpenPipeStress CONTRACT | Binding project invariants for IP/data boundary, no hidden defaults, no professional authority claims, and sealed TASK execution | Governance source, not legal or professional advice |
| DEL-17-02 common export contract | Required package/profile/stable-ID/loss-report basis for PKG-17 target exporters | Contract-level only in this Phase A task |

## Verification

Current bounded verification covers the selected JSON glTF profile only:

| Check | Expected result |
|---|---|
| Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
| Default section check | Each document retains its required default schema sections. |
| Source grounding | Claims cite GLTF-2.0, DEL-17-01, DEL-17-02, PLAN-EXPORT-INTEROP, or governance/schema sources. |
| Stable-ID round trip | Written `model.gltf` node/primitive identity metadata and `id_map.json` round trip to the same canonical centerline IDs; mismatches block package acceptance. |
| Deterministic metadata observation | Repeated packages are byte-deterministic, `asset.generator` is a fixed versioned string, and timestamp keys are absent; no normative policy is inferred. |
| TBD preservation | Binary GLB, broader geometry, viewer compatibility, engineering validation, and exact timestamp/generator policy remain unselected. |
| Boundary check | No proprietary examples, protected standards data, compatibility claims, release claims, formal validation claims, code-compliance claims, or professional claims. |

Existing implementation verification covers JSON glTF structure, deterministic package-member hashing, geometry counts, stable-ID round-trip correlation, and invented fixtures. GLB container checks and rendered visual/viewer validation remain outside the selected profile and shall not be inferred.

### Future Requirement-to-Check Map

This map records future acceptance slots only. It does not close the underlying `TBD` profile, implementation, fixture, or viewer-behavior decisions.

| Item ID | Requirement area | Future check slot | Source reread evidence | Phase A disposition |
|---|---|---|---|---|
| A-001 | DEL-17-08 `shall` requirements across profile, identity, package, and coverage requirements. | Build an explicit req-to-check matrix before implementation acceptance. | Current `Specification.md` `## Requirements` and `## Verification`; DEL-17-02 Specification `## Export Package Requirements` through `## Loss Report Requirements`. | Incorporated as future acceptance mapping; pass criteria remain `TBD`. |
| C-001 | Direct identity metadata versus sidecar identity policy. | Decide when glTF `name`, `extras`, or extensions are sufficient and when manifest-referenced sidecar ID mapping is mandatory. | GLTF-2.0 sections 3.3, 3.12, 5.16, 5.23-5.25; DEL-17-02 Datasheet `## Stable ID Families`; DEL-17-02 Guidance `### Stable ID map`. | Converted to `TBD`; profile must define consumer-risk threshold before direct metadata is accepted. |
| C-002 | Export service boundary and trigger/interface. | Identify the application-service/API boundary that consumes the canonical model and invokes the writer. | `_CONTEXT.md` Architecture Basis Injection; SOFTWARE_DECOMP SOW-030 and SOW-062; current `Specification.md` `## Scope`. | Already excluded from Phase A; retained as `TBD` for a later interface contract. |
| F-001 | Source-evidence gate for target profile facts. | Verify each GLB/glTF target fact traces to GLTF-2.0, DEL-17-01, DEL-17-02, or governance evidence. | DEL-17-01 Specification `## Source Authority Requirements`; DEL-17-02 Specification `## Source Authority Requirements`; GLTF-2.0 sections 2.4, 3.4, 4, 5.9, 5.16, 5.23-5.25. | Incorporated as future source-evidence check. |
| X-001 | First review-geometry coverage declaration by canonical entity family. | Check each family is classified as emitted, omitted, approximated, delegated, unsupported, or `TBD`. | DEL-17-02 Datasheet `## Required Export-Profile Fields`, `## Loss Categories`; DEL-17-02 Specification `## Export Profile Requirements`; current `Specification.md` `### Review Geometry Coverage Requirements`. | Already required by REQ-040/REQ-041; disposition recorded here with `TBD` coverage. |
| X-003 | Implementation verification for generated artifacts. | Verify the selected JSON glTF structure, deterministic hashing, geometry counts, and stable-ID correlation; add GLB or rendered visual checks only after separately selected scope. | GLTF-2.0 sections 3.2, 3.4, 5.9, 5.23-5.25; DEL-17-02 Specification `## Export Package Requirements`; `DEC-074` O11/E7. | JSON glTF checks are implemented; GLB, viewer, broader geometry, and engineering validation remain outside the selected scope. |

## Documentation

Current bounded implementation records include:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_STATUS.md` safe lifecycle update when permitted
- `_run_records/TASK_RUN_2026-05-18_1156.md`
- `core/handoff/review_geometry/`, `schemas/review_geometry_export.schema.json`, invented fixtures, and focused tests;
- `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T2B-PDU029-PDU031.md`.

No binary GLB, broad geometry, viewer-compatibility, engineering-validation, public-API, GUI, or release work is authorized by this repair.

## D-41 R5 T4 PDU-036 O11 boundary

REQ-043 evidence is bounded to the selected JSON glTF line/centerline profile. Current emitted line entities require authoritative ID-map correlation. Impossible-bend display geometry is not present in this profile, so its specific diagnostic remains an explicit out-of-scope gap rather than an invented bend representation or validation claim.
