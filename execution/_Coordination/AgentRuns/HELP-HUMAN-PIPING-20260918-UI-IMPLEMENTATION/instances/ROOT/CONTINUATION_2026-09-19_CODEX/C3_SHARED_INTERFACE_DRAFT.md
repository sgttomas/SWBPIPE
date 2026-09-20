# C3 shared presentation interface — draft for manager acknowledgement

ROOT coordination inside authorized G-30. No implementation activation, ownership
transfer, persisted schema or Runtime contract follows from this draft. Shell
remains sole writer of its state/controller/App/styles/tests; canvas owns the pure
projection and viewport/material files. Managers confirm exact names before child
launch. All new implementers remain Astra/low under Astra/high managers.

The canonical state is explicit Hide plus
isolationSelectionKeys: ReadonlySet<EntityKey> | null. Null means inactive;
Isolate captures current valid drawable selection. Later selection changes do not
change that snapshot. Deleted keys are pruned. If all captured keys are deleted,
the empty snapshot remains active until Show All or another Isolate; new geometry
outside an active snapshot is dimmed. This preserves explicit isolation intent.

Canvas supplies a pure deriveViewportVisibility(index, explicitHiddenKeys,
isolationSelectionKeys) returning hiddenKeys, dimmedKeys, hiddenCount and
isolationActive. Reuse existing attachment-expansion semantics for explicit Hide.
Let V be current valid drawable entity keys and H the expanded Hide set: count
|H intersect V| once per entity. Dimmed keys are V minus H minus the captured
selection when active. Geometry/label culling, occlusion, invalid geometry, row
filters and merely dimmed entities do not inflate hiddenCount. The current
unimplemented section-plane gap contributes nothing until implemented explicitly.

Shell memoizes that one projection and passes actual hiddenKeys to both tree and
viewport, separate dimmedKeys/isolationActive/hiddenCount to the viewport, and
callbacks for an isolation snapshot and Show All. Show All clears explicit Hide
and isolation through the owning session. It remains operable whenever either
state is active, including an active snapshot that currently dims no entities.
No independently stored viewport mask or competing engineering mutation route.

Dimmed geometry remains pickable by existing click, hover and box ordering; table
selection is unchanged. A dimmed near entity can win over bright farther geometry.
Selection/hover halos remain truthful without silently changing isolation. Explicit
Hide wins; selecting a hidden row does not reveal it. Nodes/attachments with distinct
entity keys keep distinct isolation membership; owned submeshes follow their owner.

Recommended integration order: canvas first freezes the small pure helper/type
surface; shell freezes B3B controller work, then applies the C3 bridge as a separate
checkpoint. ROOT combines the disjoint results for review and validation. Intermediate
additive props may keep a branch buildable, but the final candidate has one canonical
projection and operative callbacks, not duplicated live/legacy state. Return any
shared-file request through ROOT; do not concurrently edit shell-owned files.

Preserve viewportSelection.ts and geometric picking tests, first-profile instrument
bytes and all limits. No new performance timing is activated before the owner-held
profile freeze. Behavioral/resource checks remain required and rendering cost stays
explicitly unqualified. C4 labels and C5 final HUD organisation are later work; C5
must retain an operative way to clear isolation even when hiddenCount is zero.

Standard F-PIP-2/DEC-081 fence applies.
