# WORKING_ITEMS Run: D-41 R5 T4 PDU-010/PDU-020/PDU-038

- Date: 2026-07-12
- Deliverable: `DEL-17-08`
- Authority: `DEC-074` O11/E7
- Epistemic status: bounded JSON glTF fixture verification, not validation

The review-geometry manifest now records the selected profile's source-basis
references directly. Focused evidence covers source model, export profile,
member inventory, source basis, boundary notes, emitted JSON glTF artifact,
and exported/omitted loss-report content.

Post-fan-in correction made `manifest.source_basis_refs` schema-required,
updated the governed fixture and manifest checksum, and added negative evidence
that omission is rejected.

The profile remains deterministic JSON `.gltf`, embedded buffer, `LINES`,
and centerline-only. No GLB, surface/tube or broader entity geometry, viewer
compatibility, rendered QA, target behavior/layout/tolerance, comparison
semantic, or engineering-validation claim was added.

Validation: focused cache-disabled export tests passed 36/36; full
cache-disabled project tests passed 493/493; `git diff --check` passed.
Lifecycle remains `IN_PROGRESS`; the exact D-41 bootstrap is preserved for T7.
