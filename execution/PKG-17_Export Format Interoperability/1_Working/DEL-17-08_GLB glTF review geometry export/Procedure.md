# Procedure: DEL-17-08 GLB/glTF review geometry export

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-08-DECL-004`.

## Purpose

This procedure defines how to maintain and review the bounded JSON glTF centerline export without crossing into binary GLB, broader geometry, viewer compatibility, engineering validation, solver-fidelity, release, code-compliance, or professional-acceptance claims.

## Prerequisites

Before implementation or later enrichment of DEL-17-08, confirm:

1. DEL-17-01 remains the admitted PKG-17 source-basis authority for GLB/glTF review-geometry boundaries.
2. DEL-17-02 remains the common export package/profile/stable-ID/loss-report contract.
3. The GLB/glTF profile has a cited GLTF-2.0 version basis.
4. The source model identity, unit policy, coordinate policy, and canonical ID families are available from project-owned schema/contracts.
5. The write scope authorizes the target files being changed.
6. Public fixtures, if any, are invented or rights-cleared and contain no private project data, protected standards content, proprietary catalog values, owner criteria, or commercial examples.

Declared upstream dependencies for this deliverable are `DEL-17-02`, `DEL-02-01`, `DEL-07-01`, `DEL-13-04`, and `DEL-15-01`; those dependencies do not authorize implementation or claims by themselves.

## Steps

### Phase A document population

1. Read the deliverable-local truth set: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `MEMORY.md`.
2. Read AGENTS/TASK/ORCHESTRATOR/four-documents instructions.
3. Read DEL-17-01 and DEL-17-02 four-document kits and DEL-17-01 source-basis records.
4. Read project governance and technical references: `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/SPEC.md`, `docs/TYPES.md`, `schemas/model.schema.yaml`, and `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`.
5. Read the Khronos glTF 2.0 source slices for asset structure, coordinate/units convention, GLB container behavior, mesh primitive modes, nodes, extensions, and `extras` metadata.
6. Populate the four documents at contract level only.
7. Preserve `TBD` entries for unresolved geometry coverage, identity metadata placement, sidecar schema, filename/profile defaults, coordinate transform, fixture behavior, and viewer behavior.
8. Do not update `MEMORY.md` in Phase A.
9. Create or update the run record in `_run_records/`.
10. If the four documents exist and `_STATUS.md` is `OPEN`, perform the safe status transition to `INITIALIZED`.

### Future profile-development procedure

1. Declare the review-geometry export profile ID, version, glTF target version basis, and package member inventory.
2. Declare whether the target artifact is `.glb`, `.gltf`, or both.
3. Declare the source coordinate basis, glTF transform, origin policy, and meter conversion policy.
4. Declare the first review-geometry entity coverage by canonical family.
5. For each family, classify behavior as exported, omitted, approximated, delegated, unsupported, or `TBD`.
6. Select direct identity metadata locations only when the profile documents the glTF target object and consumer-risk policy.
7. Define sidecar ID-map behavior for every canonical ID that cannot be carried directly or reliably in the GLB/glTF artifact.
8. Define manifest and loss-report entries before writing the target artifact.
9. Define diagnostics for missing units, ambiguous coordinate systems, duplicate IDs, invalid geometry, unmapped emitted entities, and private/protected metadata exposure.
10. Use only invented or rights-cleared fixtures for future deterministic checks.

### Future export-package review procedure

1. Confirm every emitted centerline entity has the same canonical ID in node `extras`, primitive `extras`, and exactly one authoritative sidecar row after deterministic write/read round trip; block mismatches.
2. Confirm every target artifact is listed in the manifest.
3. Confirm geometry limitations appear in the loss report.
4. Confirm GLB/glTF target facts are limited to GLTF-2.0 source evidence and selected profile behavior.
5. Confirm current output retains its fixed versioned generator and omits timestamps. Do not declare a normative timestamp/generator policy until the owner selects the exact PDU-031 policy.
6. For E-003, confirm binary GLB and other non-JSON package members have hashes or an explicit, manifest-recorded reason why a hash is unavailable.
7. Confirm no statement implies solver readiness, analysis fidelity, target compatibility, formal validation, code compliance, release readiness, or professional acceptance.

## Verification

Run from repository root for this Phase A task:

```bash
tools/validation/check_four_documents.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export"
tools/validation/check_min_viable_fileset.sh "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export"
git diff --check -- "execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export"
```

Manual review should confirm:

- all non-trivial claims are source-grounded or marked `TBD`;
- GLB/glTF remains visual review geometry only;
- identity is preserved through metadata or sidecars without relying only on display order;
- unresolved behavior remains `TBD`;
- no proprietary examples, copied protected standards data, implementation code, schema edits, compatibility claims, release claims, formal validation claims, code-compliance claims, or professional claims were added.

## Records

Phase A records are:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_STATUS.md`
- `_run_records/TASK_RUN_2026-05-18_1156.md`

Future records remain `TBD` and may include a profile artifact, manifest, ID map, loss report, diagnostics, generated GLB/glTF artifact, and invented fixtures only when a later task explicitly authorizes those writes.

PDU-036 check: verify every emitted line primitive has current authoritative ID-map correlation. Record impossible-bend diagnostic coverage as absent/outside the selected profile; do not add GLB, viewer, or broader bend geometry.
