# PKG16 phase A interface proposal

Derivative design from hashed current SOW_V1 targets, SCA009 accepted vocabulary (acceptance consumed through parent), current operation and physical-model consumer source. Not decomposition truth. DEL1601 AC001 / R008, DEL1602 REQ001/004/005/007 and DEL1603 AC001 govern implementation. D58 live integration remains HELD.

## Immediate disjoint stage
One Agent2 integration owner writes operation_applier/src/lib.rs and toolkit_entities.rs. Shared crate lib changes serialize all later backend stages.

- Section `set_field`: `name`, `section_type` (closed `pipe` only), `provenance`, `properties.outside_diameter.value`, `properties.wall_thickness.value`. Preserve entered units. Validate dimensional quantities AND final wall < OD/2 with unit conversion. Preview and apply use identical prospective validation.
- `delete_material`, `delete_section`, `delete_component`: operation_kind delete, target exact object type/id; field_path `materials`/`sections`/`components`, unit none, dimension dimensionless, before canonical JSON of current full entity, after `deleted`. Referential integrity requires scan of known model reference fields and no silent cascade. Missing/stale/unsupported refs block. New delete kinds mirrored in model_operation.schema.json while preserving broader envelope distinctions.
- No delete silently removes connected nodes/pipes/supports. Component presence creates a node reference; component deletion itself only removes the component if no further component references exist.

## Semantic predecessor
By-ref section assignment has preview-model discrepancy: PreviewPipe has inline section, while model.schema has section_ref. Parent to route PKG02 integration decision; setting an ignored ref is prohibited. Proposed accepted behavior: typed ref and source-resolved inline snapshot only if reference-refresh ownership is explicit, otherwise consumer resolves canonical section ref.

## Later serial stage
Support/table/wind exact interfaces pending bounded A2 consumer contract read. Prefer structured existing set_field/update_support slots, not invented operation categories. JSON record replacement is permissible only with explicit nested schema validation, whole-record before-state, no arbitrary patch path.

## Fan-in
Run focused native operation tests with CARGO_TARGET_DIR=/tmp/piping-ui-parity-target; parent coordinates registered piping-pytest/evidence-sweep and fresh full-diff code review before closeout. Source unchanged pending Parent acceptance.
