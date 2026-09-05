# Row21 canonical member wire contract v1 — frozen request for N2 and N1

Status: FROZEN_CROSS_MANAGER_REQUEST; not source acceptance or an already-implemented API. Parent-approved bounded global mechanics plus reviewed N6 v3 is basis. N2 rich support and future atomic batch implementation remain required; parent routes final acceptance of this exact contract before N1 source work. No solver equation or export extension.

## Native support record association (N2 request)

Canonical location: optional top-level `boundary_association` on the native model Support record and create_support JSON after payload. Exact allowed object keys:
- boundary_id: nonempty stable string; new group identity entered by user.
- kind: `equipment` or `equipment_nozzle`.
- equipment_reference: nonempty user-supplied descriptive/source reference (not a model EntityRef).
- nozzle_reference: nonempty required iff kind=equipment_nozzle; forbidden for kind=equipment.
- coordinate_system: exact `global`.
No other keys allowed. Trim identifiers/references once at form boundary and validate trim-nonempty server-side without silently changing payload strings. Persist exact accepted strings. Reject unsupported coordinate values; never normalize local to global. Schema if any enumerates optional native Support fields must include this object. Native save/load must roundtrip it; solver PreviewSupport need not consume metadata or change physics. Do not add an equipment collection solely for this association.

Create preservation: create_support accepts and copies validated optional association alongside existing id/label/node/provenance/family/restraints/stiffness. No projection may silently drop it. Legacy supports without it remain valid. A composite creation requires identical association on every member, same existing node, no pre-existing boundary_id in supports, and unique generated support IDs. Check group-newness against the original batch base, not later in-batch members, so member two does not reject member one. All checks operate in N2 batch validation, not only the GUI.

Update preservation: existing `update_support`, field_path `configuration`, before/after projection remains exactly N2 B2_UI_INTERFACE family/restraints/stiffness/hanger/nonlinear/provenance. The association is outside the replaced configuration; omission must preserve it, never clear it. Scalar support edits and member deletion retain/remove association only with their own member. Full model revision/hash precondition guards changed association despite configuration before excluding it. Do not offer member-level association mutation through `configuration`; reject an attempted association key there as unknown. Group identity/ref edits are not part of this first authoring slice; show association read-only and retain ordinary member mechanics editing. Future explicit group-edit API is separate. Support deletion can leave other members; UI shows actual remaining member inventory, not original template completeness.

## Form to exact ordered member payloads (N1 request)

Required form: boundary_id, label, kind, equipment_reference, nozzle_reference conditionally, existing node, provenance, coordinate_system=global, and explicit mode for each UX/UY/UZ/RX/RY/RZ (Free/Rigid/Linear spring). Each spring has a finite positive quantity {value,unit}, linear_stiffness-compatible for U* and rotational_stiffness-compatible for R*. Do not include dimension inside Quantity, because N2 current consumer shape is {value,unit}. No preset stiffness or inferred rigid mode. Free entries emit nothing. All-free emits no batch.

Deterministic order: rigid member first if nonempty; spring members follow UX, UY, UZ, RX, RY, RZ order. Stable member IDs: `${boundary_id}:rigid` and `${boundary_id}:spring:${DOF}`. Do not overwrite collisions or silently append to existing groups. Explicit anchor subset is not labeled fully fixed unless all six rigid.

Every member is an ordinary EditorOperationIntent:
- operation_kind=create; operation_status=proposed; author_type=user for form or agent for accepted agent entry; target.object_type=Support; target.ref=member ID.
- change.change_kind=create_support; field_path=supports; before=not_present; unit=none; dimension=dimensionless; after=JSON serialization of exact member record below.
- operation_id/change_id use existing parent queue ID allocator; source uses existing author provenance contract. Rationale names equipment/nozzle reference, global basis and member DOF(s).

Rigid record: `{id,label,node,provenance,family:"anchor",restraints:[selected rigid DOFs in canonical order],boundary_association:association}`. Omit stiffness/hanger/nonlinear/properties.
Spring record: `{id,label,node,provenance,family:"spring",restraints:[DOF],stiffness:{dof:DOF,value:{value,unit}},boundary_association:association}`. Omit hanger/nonlinear/properties. Example labels append ` / rigid UX,RZ` or ` / spring UY`; labels are display only, association is authoritative grouping metadata.

The ordered member intent array is input to N2's future atomic batch operation, using one accepted base model revision/hash and one user preview/apply acceptance. Outer batch envelope naming remains N2-owned and pending; this document freezes member payloads and composition, not an invented executable batch API. Never loop individual apply calls to simulate atomicity. N1 starts batch wiring only after N2 freezes the actual outer type/service and proves all-or-none validation/apply/undo and canonical hashes.

## Gate tests

N2: unknown association keys/types/kinds/local coordinate/blank refs reject; legacy absence passes; configuration/scalar edit preserve association; create projection/native save-load roundtrip; group base-collision and member ID collisions reject entire batch; duplicate group metadata/node mismatch reject; member two allowed in same new group; agent/user validation parity; preview/apply/hash/undo.
N1: explicit global six modes, mixed form emits exact record order/values and one batch; all-free no mutation; no anchor default; meaningful references/association visible and read-only; member deletion inventory truthful; validation errors surfaced.
PKG04 adapter: exact rigid subset mapping with focused tests, separately serialized after source release. No PCF nozzle support, local/coupled stiffness or allowable-load claims.
