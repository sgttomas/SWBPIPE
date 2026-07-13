# Guidance: DEL-17-08 GLB/glTF review geometry export

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-08-DECL-003`.

## Purpose

Use DEL-17-08 as a narrow JSON glTF visual-review export surface. The selected profile lets a user inspect centerline shape and correlate emitted objects back to OpenPipeStress canonical IDs through direct `extras` plus an authoritative sidecar. It is not binary GLB, a solver model, an analysis result, a compatibility claim, engineering validation, or professional acceptance evidence.

## Principles

### Keep visual review separate from solver meaning

GLB/glTF is useful because it is lightweight and broadly viewable. That is also why the boundary needs to be explicit: a visible pipe-like shape does not prove that wall properties, support behavior, branch flexibility, load cases, stress recovery, rule-pack inputs, or target-solver interpretation are complete.

When the exporter emits simplified geometry, describe it as review geometry. If the visual representation drops support/restraint details, component semantics, private metadata, or user rule context, record the loss rather than implying the target file carries those meanings.

### Preserve identity before appearance polish

The review artifact is only useful to OpenPipeStress workflows if visible objects can be traced back to the canonical model. Prefer stable canonical IDs over display-order assumptions. Use direct metadata only where the profile says it is reliable enough; otherwise pair the geometry artifact with a sidecar ID map and reference it from the manifest.

For the selected centerline profile, direct node/primitive `extras` are paired with an always-authoritative sidecar; package acceptance blocks when those representations do not correlate one-to-one after deterministic write/read round trip. This establishes package identity integrity only, not receiving-viewer retention or compatibility.

Do not rely on glTF indices or generated names as the only identity layer unless the loss report records the limitation and the profile accepts it for the selected use.

### Make target behavior explicit

The profile should say what is emitted and what is not. This is especially important for supports, restraints, components, equipment interfaces, and annotations, where a visual marker may communicate less than the source model knows.

Unresolved behavior remains `TBD`; it should not be softened into phrases such as "supported by default" or "compatible with viewers" unless a later source-grounded profile and test set support that wording.

### Keep data boundaries visible

GLB/glTF metadata and sidecars can accidentally expose private project names, owner criteria, component tags, rule-pack notes, or proprietary catalog data. The review-geometry profile should distinguish user-private metadata from public fixture metadata and should default public examples to invented or rights-cleared data only.

## Considerations

| Topic | Guidance |
|---|---|
| Coordinate policy | glTF uses a right-handed +Y-up coordinate basis and meters for linear distances. Source model axes, project vertical axis, origin, scale, and transform recording remain profile decisions. |
| Geometry level of detail | First coverage may be centerline-only, simplified tube/symbol geometry, or another declared subset. The choice remains `TBD` and should be loss-report driven. |
| Metadata placement | glTF `extras` and object `name` fields are candidate places for direct review metadata. A sidecar remains necessary when metadata needs stronger auditability or may be stripped by consuming tools. |
| Extensions | Avoid required custom extensions unless the profile records viewer impact and fallback behavior. Extensions used by an asset must be declared according to GLTF-2.0. |
| Hashing | GLB is binary. The manifest should hash the emitted binary when deterministic or record why a hash cannot be stable. |
| Fixtures | Public fixtures must be invented or rights-cleared and should avoid real project/client/tag data. |
| Viewer behavior | Viewer-specific rendering, selection, metadata display, and stripping behavior remain `TBD`; do not claim viewer compatibility in Phase A. |
| Timestamp/generator | Current JSON output uses a fixed versioned generator and omits timestamp fields. Treat this as tested implementation behavior, not the normative policy PDU-031 leaves owner-unselected. |

For E-001, viewer-specific behavior should be treated as evidence-dependent wording. A later profile may name viewer behavior only after source-grounded or test-recorded evidence exists; until then, guidance stays limited to review intent and metadata-risk caution.

## Trade-offs

| Option | Benefit | Cost or risk | Phase A position |
|---|---|---|---|
| Centerline-only line primitives | Simple, compact, easy to correlate with line/element IDs | May be mistaken for incomplete physical geometry or omit component volume | Selected bounded JSON glTF profile with explicit loss reporting |
| Simplified tube/surface primitives | More inspectable visually | Requires more geometry generation and may imply false dimensional fidelity | `TBD`; must be labeled review geometry |
| Direct `extras` metadata | Keeps identity near the visible object | Some tools may ignore or strip metadata; schema remains target-specific | Selected only with authoritative sidecar; no viewer-retention claim |
| Sidecar ID map | Stronger audit trail and stable identity contract | Requires consumers to keep files together | Required and blocking-correlated for emitted centerlines |
| Required custom extension | Can structure metadata explicitly | Reduces viewer portability and requires extension governance | Defer unless later profile justifies it |

### Geometry Detail Rationale

For D-002, a later profile should choose the first review-geometry level of detail by asking whether the representation supports visual orientation, canonical-ID correlation, and limitation disclosure without implying solver, CAD, fabrication, or analysis fidelity. Centerline, simplified tube/surface, symbol, annotation, and overlay choices should be justified by coverage, identity, viewer-risk, package size, deterministic generation, and loss-report clarity; unchosen or unresolved families remain `TBD`.

## Examples

No example model, GLB file, glTF JSON snippet, proprietary fixture, commercial target file, or standards-derived engineering example is included in Phase A.

Acceptable future examples shall be invented or rights-cleared and shall demonstrate only:

- a visible review entity correlated to a canonical ID;
- an omitted entity recorded in the ID map or loss report;
- a coordinate/unit diagnostic;
- a manifest entry for a GLB/glTF artifact.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in Phase A. Remaining open items are `TBD`, not conflicts. | N/A | N/A | N/A | N/A | N/A |

For PDU-036, do not manufacture impossible-bend geometry merely to satisfy a diagnostic species. Preserve the O11 profile and record the absent bend-specific evidence honestly.
