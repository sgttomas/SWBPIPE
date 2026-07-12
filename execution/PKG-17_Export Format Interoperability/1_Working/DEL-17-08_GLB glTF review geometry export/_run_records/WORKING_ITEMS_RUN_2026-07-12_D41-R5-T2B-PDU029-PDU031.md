# WORKING_ITEMS Run: D-41 R5 T2B PDU-029/PDU-031

- Date: 2026-07-12
- Deliverable: `DEL-17-08`
- Authority: `DEC-074` O11/E7
- Claims: `DEL-17-08-ACC-011`; `DEL-17-08-REQ-021`

## Selected Scope

The current bounded profile remains JSON `model.gltf` with an embedded buffer,
`LINES` centerline segments, direct node/primitive identity `extras`, and an
authoritative `id_map.json`. This tranche does not select binary GLB, broader
geometry, viewer compatibility, rendered visual QA, or engineering validation.

## PDU-029 Repair

- Added a blocking `RG-STABLE-ID-ROUNDTRIP-MISMATCH` diagnostic when emitted
  centerline node/primitive identity does not correlate one-to-one with the
  authoritative sidecar.
- Added deterministic write/read round-trip evidence proving canonical IDs,
  glTF node refs, primitive refs, and sidecar refs remain correlated.
- Added a negative mismatched-sidecar test proving package acceptance blocks.
- This is package identity-integrity evidence only, not viewer retention or
  compatibility evidence.

## PDU-031 Hold

The current implementation already emits a fixed versioned generator and no
timestamp. Tests now enforce those observable facts and repeated deterministic
output. The R5 tranche plan states that PDU-031's exact policy is unselected,
so no normative `generator_policy` or `timestamp_policy` field was invented.
The hold remains explicit in `_STATUS.md`.

## Validation

`python3 -m pytest -p no:cacheprovider tests/test_review_geometry_export_package.py -q`

Result: `13 passed`.

Four-document, minimum-fileset, JSON-schema parse, and scoped `git diff
--check` validation also passed.

## Fences

No binary GLB, broader geometry family, viewer/rendered-visual claim,
engineering validation, DEL-14-04, dependency, DAG, register, lifecycle,
release, professional, compliance, protected-content, or private-data surface
was changed. The D-41 bootstrap remains.
