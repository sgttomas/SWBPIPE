# Explicit section inputs on connect_pipe_run

Pre-repair diagnosis snapshot. The completed repair and actual red/green results
are recorded in RETURN.md and verification.json.

TASK child `/root/physics_resume/create_section_inputs`, launched by
`/root/physics_resume` through delegated-harness-native collaboration.
Full TASK instructions are supplied in BRIEF.md. Root/Piping AGENTS,
LOOP_INIT, and the selected project `software-defect-diagnosis` skill were
read; actual source paths and pre-repair SHA-256 values are recorded in
basis_and_red_control.json. No other role or workflow body was loaded.

## Observed symptom and source trace

The parent-provided native M35 witness explicitly authored material density
1000 kg/m^3 in the section of its invented two-metre pipe. The later self-weight
planner reported `SELF_WEIGHT_INPUT_INVALID: explicit material density required`.
The original synthetic connect operation is retained in
original_native_connect_operation.json; the source batch hash is in the basis
manifest. Native UI witness remains at the parent-supplied private path
`/private/tmp/piping-native-m35-records-20260925/ui/09-weight-plan-full.txt`.
This TASK did not drive the native application.

The earliest source divergence is
`core/model_operations/operation_applier/src/lib.rs::resolve_connect_pipe_run`:
it reconstructs a fresh `section` JSON object from only outside_diameter and
wall_thickness. It does not inspect or preserve explicit material_density,
contents_density, insulation_thickness, insulation_density, or mill_tolerance.
Those fields are already recognized by PipeSectionInput and by the Element
set_field rules. The self-weight planner correctly requires explicit density;
changing that requirement or supplying a hidden density would mask the defect.

## Bounded repair and control

The public-API regression in `tests/create_pipe_section_inputs.rs` carries the
original native connect payload verbatim as JSON values. It checks validation,
applied model and diff-preview section retention, followed by self-weight
planning through the existing M35 API. Additional controls compare creation
with set_field, exercise entered units and explicit zero, preserve null/absence,
report partial insulation, reject unsupported or invalid quantity input, and
verify atomic rollback after a late invalid creation. `red_control.rs` preserves
the pre-repair regression bytes.

Repair is authorized by the brief. A parent message expanded the original
write scope to `resolve_connect_pipe_run` in operation_applier/src/lib.rs and
its directly needed helper wiring after the actual function location was
confirmed. The parent retains product_physics facade and all integration/Git/
native work. No Cargo command has yet run; the parent holds the test lane until
its shared facade is compilable. Production source remains unchanged pending
the failing public control.

The intended repair validates supported inline section quantities without
substituting their entered values or units, then carries the section JSON
through creation. Geometry remains explicit, density is positive, the four
other optional inputs are nonnegative, and absolute mill tolerance must leave
positive wall. No equations, external engineering corpus, or real library data
are introduced. A requested shared section reference should be explicitly
rejected on connect with the existing assignment operation as the route.
