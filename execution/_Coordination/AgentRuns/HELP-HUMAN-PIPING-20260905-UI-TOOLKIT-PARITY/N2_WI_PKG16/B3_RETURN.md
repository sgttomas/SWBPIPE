---
run-id: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/B3
parent: N2_WI_PKG16
role: ephemeral Agent2
role-enforcement: instruction-asserted
run-status: SUCCESS
---
# B3 implementation return — source frozen

AcceptedBasis: B3_BRIEF.md; R2B_RETURN PASS; all B2_REPAIR_FROZEN_HASHES verified before edits; N3 SECTION_ACCEPTED_SNAPSHOT_V1 and ROW17_CANDIDATE_V2 read. Derivative execution evidence only, not decomposition truth or lifecycle acceptance. No delegation performed.

## Implemented interface
- modify / assign_section / Element / section_ref; before canonical JSON full pipe; after JSON {section_ref:id}; none/dimensionless.
- modify / detach_section / Element / section_ref; before canonical JSON full pipe; after JSON {source_section_ref:currentId}; none/dimensionless. Source Section remains explicit in operation/diff; no fabricated provenance metadata.
- Payload unknown keys, missing/duplicate pipe or Section, invalid type/property/unit/geometry, stale full pipe/hash and detach source mismatch fail closed. Existing bound cache must agree with its old source before reassignment/detach.
- Shared Section scalar edits atomically rematerialize exact OD/wall quantities into every bound pipe after validating old caches and candidate geometry including local mill tolerance. Per-span supplemental fields persist. Unbound pipes unchanged. Additional deterministic diff rows show every changed pipe section, ordered by ID; assignment also previews materialization when it changes geometry. Validation and application share the candidate.
- Direct bound Element OD/wall scalar edits block with shared-edit/detach guidance. Other bound local edits verify cache and effective wall before acceptance.
- Optional Element set_field paths section.material_density.value (density positive), contents_density.value (density nonnegative), insulation_thickness.value (length nonnegative), insulation_density.value (density nonnegative). Existing optional quantity wire applies: absent before TBD, explicit after JSON {value,unit}; no default values or engineering equations. One-sided insulation authoring warns OP-MASS-NOT-SOLVE-READY until the pair is complete.

## Outputs
- core/model_operations/operation_applier/src/section_bindings.rs (new helper)
- core/model_operations/operation_applier/src/lib.rs (integration and focused tests)
- schemas/model_operation.schema.json (assign/detach taxonomy mirror)
- tests/test_model_operation_schema.py (mirror assertion)
- B3_CHECKS.json (frozen hashes and portable commands)

## Validation
Full offline operation crate PASS: 107 unit + 3 integration tests. Provisioned Python schema check PASS. Scoped git diff whitespace check PASS. New tests exercise two bound pipes plus one unbound, propagation/diffs/local field preservation/detach independence; malformed source and cache failures; stale pipe/hash/duplicate targets; local mill tolerance atomic rejection; density and insulation dimensions/sign/readiness. Required shared source name:string and provenance key presence match N3 PreviewSection serde shape and are covered by missing-field negatives. Additional bound-pipe diff rows contain actual full inline section before/after; their shared Section identity is the primary operation target, without invented provenance. Scalar no-op does not create additional pipe diffs. Consumer constraints verified against product_physics source; no physics code edited.

## Handoff
Implementation complete; lib.rs ownership released. Fresh 100% source review and wider native/Wasm/UI/registered tests remain parent-owned before publishing. No desktop, product_physics, Cargo.lock, receipt, lifecycle, commit or push edits. Scope limited to frozen B3 fence. Existing B2 helper files remained untouched. No remaining implementation blocker under this brief; review may identify repairs.
