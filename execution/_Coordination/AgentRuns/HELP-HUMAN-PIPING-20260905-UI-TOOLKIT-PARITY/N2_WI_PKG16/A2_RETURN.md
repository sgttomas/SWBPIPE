# A2 detail design return

RUN_STATUS: SUCCESS (read-only reconnaissance completed; design proposals, not accepted authority)
ControlSurface: FILE — A2_BRIEF.md
Role: ephemeral Agent 2; no delegation; native harness role instruction-asserted.
Package: PKG-16 / DEL-16-02
WriteAuthorization: EXPLICIT_BRIEF_TEXT — this A2_RETURN.md only. No source modifications.
Method: software-repository-reconnaissance v1; companion BRIEF_SCHEMA, TOOL_POLICY, QA_CHECKS read. Targeted source reads used. Procedural warning: deterministic discovery was not run before manual exploration; no build/test/network commands executed. Root and project AGENTS and AGENT_TASK read.
ToolsUsed: bash cat, sed, rg, git rev-parse; python3 for this output.

## Recommended interfaces (proposals for manager acceptance)

Use existing OperationIntent envelope and JSON-encoded change.after, object identity in target_ref. Add bounded whole-value operations rather than general JSON patch: `set_support_configuration` (Support; field_path `configuration`), `set_material_temperature_points` (Material; field_path `temperature_points`), `set_wind_exposed_spans` (Load; field_path `equivalent_static.wind.exposed_spans`). Each uses canonical JSON change.before for stale detection, unit `none`, dimension `dimensionless`; nested Quantity values retain explicit entered units and are dimension-validated individually. Existing create_support can reuse the support validator and accept identical rich fields while retaining legacy bare/properties.linear_stiffness behavior. Whole-list replacement allows add/edit/delete atomically, avoiding index-address instability. Unknown payload fields should block, not be silently dropped.

### Rich support payload

`{family, restraints, stiffness?, hanger?, nonlinear?, provenance}` modifies only the named configuration fields; retain id, label, node and unrelated metadata. Optional absent rich members in this explicit replacement clear old values (document this; before includes all replaced values). Alternatively target full Support record with immutable id/node invariants if parent prefers the existing record operation mechanism.

`stiffness={dof,value:{value,unit}}`. Canonical DOF vocabulary UX/UY/UZ/RX/RY/RZ. Validate positive finite stiffness; translational DOF requires linear_stiffness unit, rotational requires rotational_stiffness. Existing top-level stiffness takes precedence over hanger.stiffness: reject conflicting duplicates in new authoring rather than silently choosing.

`hanger={hanger_type, stiffness?, installed_load?, cold_load?, hot_load?, constant_load?, travel_range?, movement_limit?, manufacturer_reference?, source_reference, load_side_review_reference?, mechanics_consumption?}`.
Canonical hanger types `variable_spring_hanger`, `constant_effort_support` (consumer also recognizes legacy `spring_hanger`). Variable requires stiffness, strictly positive installed/cold/hot Force quantities, at least one strictly positive Length travel_range/movement_limit, nonempty source_reference. Constant requires strictly positive Force constant_load, positive travel or movement limit, source_reference. These requirements are already implemented in product_physics/src/validation.rs:702–934. Do not infer catalog data, preload, hot/cold loads or spring rate. Metadata mechanics_consumption is an optional string, not a validated enum or actual solve switch; do not expose as an invented editable enum.

Observed linear family distinctions are `line_stop`, `vertical_support`, `spring`; six restraints map to Anchor and other bare restraints to Guide (product_physics/src/lib.rs:3107–3204). Variable/constant selection additionally uses hanger_type with fallback to family (7526–7558). Preserve previously supported family values; do not invent a new family enum from display labels.

Constant effort assembled consumption additionally requires exactly one declared translational restraint DOF and a known node; otherwise existing consumer returns non-consumption warning, not automatically a rigid support. Keep that distinction visible (lib.rs:7580–7680). Nonlinear presence takes precedence over constant/linear support classification. New configuration should reject simultaneous hanger and nonlinear as ambiguous authoring (proposed stricter authoring rule, not claimed existing physics invariant).

`nonlinear={behavior,dof,initial_state,active_when?,contact_when?,closes_when?,gap?,friction_coefficient?,normal_reaction?,normal_reaction_source?}`.
Canonical behavior one_way/gap/lift_off/friction; initial state active/inactive/sticking/sliding. one_way requires active_when, lift_off requires contact_when: positive_reaction/negative_reaction. gap requires closes_when positive_displacement/negative_displacement and finite nonnegative Length gap. Friction requires finite nonnegative dimensionless coefficient and exactly one of finite nonnegative Force normal_reaction or `{support_ref,dof}` normal_reaction_source. Derived source must resolve to an existing linear support with that translational DOF restrained and known node. Source nonlinear support is rejected, hence self-reference is rejected as well. Consumer permits canonical aliases and all six DOFs; do not invent unsupported state/behavior restrictions. Sources: product_physics/src/lib.rs:2617–2698,2714–2960,8662–8714; solver/nonlinear_supports/src/lib.rs:96–155. Gap dimension currently normalizes Length even for rotational DOF; retain consumer behavior or surface limitation rather than invent rotational-gap mechanics.

### Material temperature table

After is `[{id,temperature?,elastic_modulus?,shear_modulus?,thermal_expansion_coefficient?,provenance?}, ...]` using exact MaterialTemperaturePointInput shape (product_physics/src/lib.rs:548–564). Require unique nonempty point ids; same basis id across different materials is intentional and must remain allowed. Quantities validate Temperature / Stress / Stress / ThermalExpansionCoefficient respectively; E/G finite positive if present; no invented positive-only alpha restriction. Require finite quantities and dimensional units; compare duplicate temperatures after normalization, not raw unit values.

Existing model permits incomplete points, so authoring may retain incomplete unselected rows with explicit readiness diagnostics. At acceptance of a changed table, guard used exact basis references against deletion, and revalidate affected selected bases. Exact selection requires named point and E/G on every used material; alpha absence stays absence. Interpolation requires a strict adjacent bracket, E/G/alpha on both endpoints, no duplicate normalized temperatures, and no extrapolation or base-value fallback. Load case exact ref and solve temperature are mutually exclusive. Sources: product_physics/src/lib.rs:5230–5500; normalization 3747–3800. Existing field path temperature_points.<id>.shear_modulus.value should remain backward compatible (operation_applier/src/lib.rs:7095). New list replacement must not break the globally shared basis-id intent or silently remove load-case refs.

### Wind spans

After is `[{pipe_ref,start_fraction:{value,unit},end_fraction:{value,unit}}, ...]` using exact WindExposedSpanInput (product_physics/src/lib.rs:467–478). Fractions dimensionless, finite, 0 <= start < end <= 1. Pipe refs must exist. Same-pipe spans may be adjacent/disjoint but must not overlap; reject if same pipe is also in exposed_pipe_refs. Distinct span list order should remain authored order; validate overlaps using a sorted temporary copy. For switching whole/partial selection atomically, preferably use a single `set_wind_exposure` operation with `{exposed_pipe_refs,exposed_spans}` replacing both; otherwise users must remove old refs before adding spans. This combined variant is the recommended final choice.

Preserve pressure, shape_factor and direction unchanged. Empty exposure can be authored as incomplete configuration only with existing readiness diagnostic; it is not solvable wind generation. Consumer wind direction enum global_x/global_y/global_z; pressure and shape_factor remain required explicit inputs. Sources: product_physics/src/lib.rs:4990–5190; no new wind physics needed.

## Implementation split and acceptance checks

Parent owns operation-kind dispatch/apply integration in operation_applier/src/lib.rs. A bounded implementation child can own a new rich_authoring.rs module returning validated replacement values/errors, plus module tests, with parent adding private module access to existing Checker, quantity conversion and entity helpers. Avoid concurrent writes to lib.rs. UI consumes manager-approved wire contract only after integration frozen.

Tests should cover stale before and input immutability; wrong nested unit dimensions; missing required hanger inputs; nonlinear invalid enums/source refs; exact/interpolated table dependency breakage and mixed-unit duplicate temperatures; wind adjacent allowed/overlap and whole-vs-partial collision blocked; native/Wasm same hash. No physics constants or solver changes are required for this scope.

MISSING: complete operation wire naming is a proposal requiring parent cross-manager contract acceptance; this return does not substitute for frozen contract. No isolated builds/tests executed.
NEEDS_HUMAN_RULING: none for these bounded existing-consumer authoring interfaces. D58 live-agent binding remains held.
DEPENDENCY_NOTES: operation interface freeze precedes UI integration; fresh independent code review required for source diff. No source implementation closure claimed.
Handoff: derivative reconnaissance of current code and sealed brief; ready for parent review; remaining implementation/verification and operation-wire freeze belong to parent.
