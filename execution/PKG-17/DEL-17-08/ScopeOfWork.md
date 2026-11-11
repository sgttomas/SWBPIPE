---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-08
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-074]
package_objective_refs: [OBJ-009, OBJ-017]
---

# Scope of Work — DEL-17-08 GLB/glTF review geometry export

## Purpose and Objective Traceability

This deliverable is the review geometry export: a glTF file of the model's shape that opens in any glTF viewer, packaged with an identity map, a loss report and diagnostics. The result it contributes is that someone without SWBPIPE can look at the routing in three dimensions and trace every visible object back to the model entity it stands for. It is a picture for review. A visible pipe-like shape says nothing about wall properties, support behaviour, loads or stresses, and the export never presents itself as solver geometry, a fabrication model or evidence of validation.

It serves SOW-030 and SOW-074 under OBJ-009 and OBJ-017 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-08: visual handoff support only, not solver geometry or professional validation), with `docs/PRD.md` §5.8, §16.1 and FR-HAND-005 (Could). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The review geometry export: the profile, the writer that produces the glTF and its package from a model, the schema `schemas/review_geometry_export.schema.json`, the export a user reaches in the desktop, and the invented fixtures.

## Deliverable Definition — Ontology

**Terms the export implements.**

- *Review geometry.* A simplified drawing of the model for visual inspection, in which each drawn object stands for one model entity.
- *Selected profile.* The one profile selected for this export (owner decision DEC-074, items O11 and E7, `execution/_Decomposition/SOFTWARE_DECOMP.md`):
  - the artifact is JSON glTF 2.0, `model.gltf`, with its buffer embedded;
  - geometry is the centerline only, one line segment per element, in line mode (`LINES`);
  - canonical identity is written in the `extras` of each node and primitive;
  - the sidecar `id_map.json` is the authority for identity, whatever the `extras` carry.
- *Package.* The glTF artifact with its profile, `id_map.json`, manifest, loss report and diagnostics, under the common contract (DEL-17-02).
- *Coverage declaration.* The profile's statement, for each family of model entity, that it is exported, omitted, approximated, delegated, unsupported or undecided. The families are: centerline nodes and elements, bends, branches, reducers, components, supports and restraints, equipment interfaces, line labels, material and section labels, load-case indicators and diagnostic overlays.
- *Format facts.* What the Khronos glTF 2.0 specification (source `GLTF-2.0`, `https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html`) fixes for any asset: `asset.version` is required; the coordinate system is right-handed with +Y up; linear distances are in metres; application data goes in `extras`; an extension an asset uses is declared in the asset.

**Boundary.** This deliverable owns the review geometry profile, writer, schema and fixtures.

- The common package, profile, stable ID map, manifest and loss report are DEL-17-02's. The admitted source for glTF facts is DEL-17-01's register.
- The model document, its identifiers and its global frame are DEL-02-01's. The viewport that draws the model inside the desktop is DEL-07-01's and shares nothing with this export.
- The governed export route and the screening of private and protected values are DEL-12-02's. The desktop panel that presents this export is DEL-07-12's.
- How a particular viewer renders the file, and whether it keeps or strips `extras`, is outside this deliverable. The package's identity is whole without the viewer because the sidecar carries it.

**Not decided.**

- Whether a binary `.glb` artifact is produced. The deliverable's name and SOW-030 say GLB and glTF; the selected profile is JSON glTF only.
- Whether any family beyond the centerline is drawn (bend arcs, tube or surface shapes, component and support symbols, labels, overlays), and which fields suffice for review without implying solver fidelity (DEL-17-01, TBD-17-01-006). Until a profile selects them, the coverage declaration records each as not drawn.
- Which axis of the model's global frame maps to glTF's +Y. The model document declares one right-handed global frame and names no vertical axis; the profile declares the transform it uses.
- The policy for the asset's generator text and for timestamps: which of deterministic, normalized, omitted or runtime-dependent is the rule for each.
- The diagnostic for bend display geometry that cannot be drawn (REQ-043). The selected profile draws no bends, so it has no subject until a profile does.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The export rests on DEL-17-01 for every statement about glTF and GLB.
- **REQ-002** — The export follows DEL-17-02 for the package, profile, stable ID map, manifest and loss report.
- **REQ-003** — Target behaviour that `GLTF-2.0`, DEL-17-01, DEL-17-02 or the project's governing documents do not confirm is undecided.
- **REQ-004** — The export is described only as visual review geometry. It is not presented as solver geometry, analysis geometry, validation evidence, code-compliance evidence, compatibility with a target or professional acceptance.
- **REQ-005** — The export holds no protected standards text or table, private project data, proprietary target example, owner criterion or copied commercial file.
- **REQ-010** — The profile declares the glTF version it targets and whether the artifact is `.gltf`, `.glb` or both.
- **REQ-011** — The asset carries the `asset.version` that `GLTF-2.0` requires.
- **REQ-012** — Linear geometry is written in metres, or the export blocks with a diagnostic before anything is written.
- **REQ-013** — The profile declares the source coordinate basis, the glTF basis and the transform between them, including glTF's right-handed, +Y-up convention.
- **REQ-014** — The primitive topology is declared by the profile. The selected profile is line-mode centerline segments.
- **REQ-015** — An extension, if one is used, is declared as `GLTF-2.0` requires and is optional unless the profile records its effect on viewers.
- **REQ-020** — Every drawn entity carries the same canonical identifier in its node `extras`, in its primitive `extras` and in exactly one row of the sidecar, and an entity not drawn is accounted for in the loss report.
- **REQ-021** — Identity is written into `name`, `extras` or a declared extension only where the profile records the location and the risk that a consumer strips it.
- **REQ-022** — The sidecar is produced whenever direct metadata is absent, insufficient, not approved by the profile or not reliable for the receiving workflow. In the selected profile it is always produced.
- **REQ-023** — The ID map keeps canonical SWBPIPE identifiers apart from glTF node, mesh and primitive indices, generated display names and sidecar row identifiers.
- **REQ-024** — An omitted entity appears in the ID map or the loss report with its canonical identifier, the reason, the severity and the downstream implication.
- **REQ-030** — A package holds, or refers to, the manifest, the ID map, the loss report, the diagnostics and the glTF artifact.
- **REQ-031** — The manifest records the source model identity, the profile identity, the member inventory, the source-basis references and the boundary notes.
- **REQ-032** — The loss report classifies each material limitation of the review geometry in one of the six categories of the common contract.
- **REQ-033** — A binary or other non-JSON member records its hash, or the reason no hash is available.
- **REQ-034** — Each generator and timestamp field of the asset and the package carries the declaration DEL-17-02/REQ-014 requires, and no runtime-dependent value enters a hash.
- **REQ-035** — The profile carries source-basis references to DEL-17-01, DEL-17-02 and `GLTF-2.0`, and a package without them is not accepted.
- **REQ-040** — The profile holds the coverage declaration.
- **REQ-041** — The coverage declaration gives every family a classification. Drawing the centerline does not settle the others: each family not drawn is declared as such and reported.
- **REQ-042** — A simplified shape that stands in for richer model geometry is recorded as approximated.
- **REQ-043** — A missing unit, an ambiguous coordinate basis, a duplicate stable identifier, zero-length geometry, bend display geometry that cannot be drawn, and a drawn entity with no mapping each block the package with a diagnostic.

### Acceptance criteria

- **AC-001** — The `model.gltf` written for an invented model is a valid glTF 2.0 asset: it carries `asset.version` 2.0, its accessors, buffer views and embedded buffer agree, its primitives are in line mode, and it uses no undeclared extension.
- **AC-002** — Every centerline element of the invented model is drawn as one segment, and no other geometry is drawn. Each endpoint equals the model's node coordinate converted to metres and transformed as the profile declares, rounded to the nearest single-precision value that a glTF FLOAT position stores. A model whose length unit is missing, or whose coordinate basis the profile does not cover, blocks with a diagnostic and nothing is written.
- **AC-003** — Reading the written `model.gltf` and `id_map.json` back, every node and primitive carries a canonical identifier in its `extras`, the two agree, and the sidecar has exactly one row for it in which the canonical identifier, the glTF indices and any display name are separate fields. An artifact and a sidecar that do not agree are refused. Every entity of the model that is not drawn appears in the ID map or the loss report with the members REQ-024 names.
- **AC-004** — The profile gives each family of the coverage declaration one classification. A loss report accompanies every export, including one that lost nothing, and it holds an entry for each family that is not drawn and for each shape that is approximated. A profile that leaves a family unclassified is refused.
- **AC-005** — Each condition REQ-043 names that the profile can meet blocks the package with a diagnostic that names the entity: a missing unit, an ambiguous coordinate basis, a duplicate stable identifier, a zero-length element and a drawn entity with no mapping.
- **AC-006** — The package for the invented model validates against the schema. Its manifest names the source model, the profile, every member and the three source-basis references, and a package missing a reference is refused. Each JSON member's hash is on the basis DEL-17-02/REQ-007 requires and carries a label that names that basis; a non-JSON member has its hash or the stated reason. Two exports of the same model under the same profile are byte-identical.
- **AC-007** — The package a user saves from the desktop for the session's model is the package AC-001 to AC-006 describe: it validates against the schema, and its glTF opens with the session model's centerline. With AC-006 this carries commitment DEL-17-08:1.
- **AC-008** — Every package states in its boundary notes that the geometry is for visual review only, and no member of a package and no fixture presents it as anything REQ-004 excludes. The `extras` and the sidecar hold identifiers and nothing classed private. Every fixture is invented and carries its provenance.

## Production and Verification Method — Praxeology

**Method choices that define the export.**

- Identity before appearance: a drawn object that cannot be traced to a model entity is a defect, however good it looks.
- The sidecar is always the authority. Direct `extras` are a convenience that a consumer may strip.
- glTF indices and generated names are never the only identity.
- What is not drawn is said: the coverage declaration and the loss report state it, and the picture is never left to imply it.
- No required custom extension: the file stays viewable by any glTF 2.0 reader.

**How each criterion is examined.**

- **VER-001** — Comparison with a named reference (a glTF 2.0 validator or reader that shares no code with the exporter, applying the Khronos specification): examines the written `model.gltf`.
- **VER-002** — Executable check (observes the positions decoded from the written buffer against the invented model's node coordinates under the declared conversion and transform, the segment count against the element count, and exports of a model without a length unit and of a model in an uncovered basis).
- **VER-003** — Executable check (observes the written artifact and sidecar read back from disk: `extras`, sidecar rows and their fields; an artifact paired with an altered sidecar; and the invented model's undrawn entities against the ID map and loss report).
- **VER-004** — Executable check (observes the profile's coverage declaration against the list of families, the loss report of a complete export, and a profile with one family removed).
- **VER-005** — Executable check (observes invented models seeded with each blocking condition of AC-005).
- **VER-006** — Comparison with a named reference (an RFC 8785 canonicalizer that imports no product code) and executable check (observes the package against the schema, the manifest against the members, a package with each source-basis reference removed, and two exports for byte equality).
- **VER-007** — Executable check (observes the package the desktop saves for an invented session model: its validity, its marking, and its segments against the model's elements).
- **VER-008** — Executable check (the project's protected-content scan over the schema and fixtures, and an export seeded with a private value) and judgment by a reviewer who did not write the fixtures: examines provenance and every member for the presentations REQ-004 excludes.

## Governing Values and Decisions — Axiology

- **AX-001** — *Traceable picture.* Better is more of what is drawn traceable to its model entity after the files have been written and read back. Appearance is never improved at the cost of a drawn object without identity.
- **AX-002** — *Honest simplification.* Better is more of the model's families with a stated classification. A richer-looking shape never stands in for geometry the model does not hold without being recorded as approximated.
- **AX-003** — *Viewable anywhere.* Better is fewer features that a plain glTF 2.0 reader would not understand. Structure for SWBPIPE's convenience never becomes a required extension.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 OBJ-009 | REQ-001 REQ-003 REQ-010 REQ-011 REQ-014 REQ-015 | AC-001 | VER-001 | Outside source: the Khronos glTF 2.0 specification, applied by a validator or reader that shares no code with the exporter |
| OUT-001 | SOW-030 OBJ-009 | REQ-012 REQ-013 | AC-002 | VER-002 | The invented model, which the exporter did not write; a closed form: the public definition of the metre conversion, the declared transform and rounding to the nearest IEEE 754 single-precision value, the component type `GLTF-2.0` requires of a position; and the specification's coordinate convention |
| OUT-001 | SOW-074 OBJ-017 | REQ-020 REQ-021 REQ-022 REQ-023 REQ-024 | AC-003 | VER-003 | Owner ruling: decision DEC-074 (O11, E7) and the common contract's stable ID map; the identifiers compared are the invented model's own |
| OUT-001 | SOW-074 OBJ-017 | REQ-032 REQ-040 REQ-041 REQ-042 | AC-004 | VER-004 | Owner ruling: the common contract's loss categories and the family list of this scope |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-043 | AC-005 | VER-005 | Owner ruling: REQ-043 carries the accepted statement of which conditions block; the seeded models, which the exporter did not write, supply the cases |
| OUT-001 | SOW-074 OBJ-017 | REQ-002 REQ-030 REQ-031 REQ-033 REQ-034 REQ-035 | AC-006 | VER-006 | Outside source: RFC 8785 and SHA-256, through a canonicalizer that imports no product code, and a JSON Schema 2020-12 validator; byte equality of two exports guards against change only |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-002 REQ-030 | AC-007 | VER-007 | The invented session model, which the exporter did not write, and the schema |
| OUT-001 | SOW-074 OBJ-017 | REQ-004 REQ-005 | AC-008 | VER-008 | Owner ruling: the decomposition row (visual handoff support only), `docs/IP_AND_DATA_BOUNDARY.md` and `docs/PRD.md` §21.2; the reviewer did not write the fixtures |
