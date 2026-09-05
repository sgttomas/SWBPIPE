---
run-id: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/B1_ENTITIES
run-status: SUCCESS
role: Agent2 ephemeral generalist
role-enforcement: instruction-asserted
parent: N2_WI_PKG16
---
# B1 return — source frozen for fresh review

Implemented Section set_field support for name, section_type, provenance, properties.outside_diameter.value, properties.wall_thickness.value. Type remains pipe. Every Section edit validates the complete candidate in both validate and apply paths: accepted length units, positive dimensions, wall strictly below half OD after conversion to meters. Quantity edits use existing value/unit JSON payload convention; scalar numeric values can only retain the current unit. Inputs remain immutable.

Deletion interface exactly matches sealed B1: operation_kind delete; change_kind delete_material/delete_section/delete_component; target Material/Section/Component; field_path materials/sections/components; before exact engine canonical_json(record); after deleted; unit none; dimension dimensionless. Requires exactly one matching record, canonical current record match, optional current model hash guard, and absence of semantic references. Recursive scan includes canonical typed Reference {object_type,id}, ref, *_ref, *_refs, arrays and preview material/section/component slots. Material-owned temperature-point IDs also prevent deletion when referenced, including modulus_basis_ref. Labels/provenance prose are not reference slots. Never cascades.

## Source files
- core/model_operations/operation_applier/src/lib.rs
- schemas/model_operation.schema.json
- tests/test_model_operation_schema.py

Schema adds the three accepted executable deletion change-kind enum entries; the broader proposal envelope and executable intent remain distinct.

## Evidence
B1_CHECKS.json freezes hashes and commands. Offline crate check PASS: 88 unit + 3 integration tests. Tests cover all five Section grid paths; invalid type/geometry/dimensions/mixed units/staleness; all three delete variants; missing/duplicate IDs, stale record/hash, shape/unit mismatch, canonical and preview references, child-point references, and prose false positives. Schema check attempted but system Python lacks jsonschema; parent dependency-provisioned interpreter must rerun. First compile type mismatch corrected before final passing run. Rustfmt applied only to owned lib.rs.

## Scope and handoff
Declared source fence observed. No desktop, product_physics, Cargo.lock, receipt, lifecycle, commit or push edits. Parent handles fresh 100% diff review and registered checks. No lifecycle or professional acceptance claimed. B3 live section-reference propagation is now an accepted upcoming serial extension; B1 intentionally does not implement assignment or propagation. Accepted basis remains BASIS_HASHES.json + AMENDMENT_B1.md; this return is derivative execution evidence, not authoritative decomposition truth. Closure: implementation complete, review/schema-environment verification pending. No delegation performed.
