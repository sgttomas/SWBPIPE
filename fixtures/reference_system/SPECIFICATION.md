# The reference system

This document specifies one invented piping system on which the release target
[`docs/RELEASE_TARGET_v1.0.md`](../../docs/RELEASE_TARGET_v1.0.md) examines exit
criteria 1, 2, 6, 7 and 17. DEL-09-01 owns it, with the fixture, the run sheet and the
references.

**Basis.** Model document 0.9.0, as the generator
[`generate_reference_system.py`](generate_reference_system.py) writes it beside this
document: the model [`reference_system.json`](reference_system.json), its authoring record
[`authoring.json`](authoring.json) and the named results
[`key_results.json`](key_results.json). The fixture is a first tranche of the system. This
document states the whole system; where the fixture does not yet hold a part of it, §10
says so, and that part keeps its place and identities here. The run sheet that enters the
fixture from a blank project is [`RUN_SHEET.md`](RUN_SHEET.md).

Every value here is invented for this fixture: geometry, sections, material, loads,
supports and spectra. Section sizes are deliberately non-standard. Nothing is taken from a
standard, a supplier catalogue, a commercial example or a project
([`docs/IP_AND_DATA_BOUNDARY.md`](../../docs/IP_AND_DATA_BOUNDARY.md)). The system is a
test object, not a design; no value is offered as suitable for any real service. SWBPIPE
computes mechanics; it does not determine code compliance (F-PIP-2). Meeting a criterion on
this system is evidence for the owner's release decision, not the decision (F-PIP-3).

## 1. Consumers

| Criterion | What it reads here | Where |
|---|---|---|
| 1. One model, every required analysis | The whole system (§2–§8) solved on EX in the desktop with no admitted family refused | §9, §10 |
| 2. Benchmarks | The independent references of the reference checks | §9.2 |
| 6. MBF round trip | The model saved as MBF and imported back, with its identity set | §11.1 |
| 7. Authoring time | The run sheet that enters the system from blank, and its solve | §11.2, the run sheet |
| 17. Embedded agent | The operation families the agent needs to build the system, and the key results it reports | §11.3 |

**Expected values.** A key result gets an expected value only from (a) a closed-form or
hand-calculation reference with an independent generator, (b) an exact identity between
published quantities, or (c) a verified solve, frozen in
[`validation/references/reference_system/`](../../validation/references/reference_system/)
with its basis recorded. No expected value is entered otherwise, and none is changed to
silence a failure. §9 names the results and their basis; it gives no values.

## 2. Conventions

- **Units.** SI. Lengths in m for coordinates and mm for section dimensions; N, N·m, kPa,
  °C, kg/m³. The project's units are m, N, N\*m, rad, Pa, Pa and degC.
- **Axes.** Global Z is vertical, positive up. This is the fixture's stated choice: the
  solver has no fixed vertical axis. Gravity is entered on each weight case as
  `global_z`, value −9.80665 m/s² (standard gravity, a definitional constant). The static
  seismic cases enter gravity as a positive magnitude with a signed factor per axis (§8.1).
  Wind and seismic directions are named global axes.
- **Identities.** Stable ids, so criterion 6 can compare them, for example
  `node:RS-010`, `pipe:RS-P010`, `component:RS-B1`, `support:RS-A`, `load:RS-SUS`,
  `combination:RS-EXP`, `region:RS-SUS-RG-1`, `material:RS-M1` and `section:RS-S270`.
  Node numbers step by 10, leaving room to insert.
- **Route.** `pressure_contract` `{version: "3.0.0", mode: "exact_pressure_v3",
  shear_model: "euler_bernoulli"}`.
- **Provenance.** Every authored record carries the provenance
  `invented reference-system fixture input (DEL-09-01); not standard, supplier, commercial or project data`.
  Every user value that needs a source (bend k and SIFs, tee SIFs, stiffness scales,
  component geometry, the joint's stiffness, rods and pressure area, the spring's hanger
  data) cites `invented user value, DEL-09-01`.

## 3. Geometry

The system runs from equipment nozzle A to equipment nozzle B, with a branch to equipment
nozzle C. It contains three realized bends, a tee, an eccentric reducer, a flanged valve
and a tied expansion joint. It has 27 nodes (corner points are not nodes; each realized bend
has two tangent nodes) and 26 members: 22 straight, three arcs and the span the expansion
joint replaces.

### 3.1 Nodes

Each node's label is its tag followed by its role, for example `RS-010 Anchor A (nozzle A)`.

| Node | X, Y, Z (m) | Role |
|---|---|---|
| RS-010 | 0, 0, 1.000 | Anchor A (nozzle A) |
| RS-020 | 0.400, 0, 1.000 | Flange F1 start |
| RS-030 | 0.550, 0, 1.000 | Valve V1 start |
| RS-040 | 1.150, 0, 1.000 | Flange F2 start |
| RS-050 | 1.300, 0, 1.000 | Flange F2 end |
| RS-060 | 3.500, 0, 1.000 | Line stop LS1; vertical support V060 |
| RS-070 | 5.600, 0, 1.000 | Bend B1 near tangent |
| RS-075 | 6.000, 0, 1.400 | Bend B1 far tangent |
| RS-080 | 6.000, 0, 4.000 | Variable spring VS1 (riser) |
| RS-090 | 6.000, 0, 6.600 | Bend B2 near tangent |
| RS-095 | 6.000, 0.400, 7.000 | Bend B2 far tangent |
| RS-100 | 6.000, 3.000, 7.000 | Constant-effort support CE1 |
| RS-110 | 6.000, 7.000, 7.000 | Lift-off support LO1 |
| RS-120 | 6.000, 11.000, 7.000 | Resting support with friction FR1 |
| RS-130 | 6.000, 15.000, 7.000 | Tee T1 |
| RS-140 | 6.000, 18.000, 7.000 | Gap guide G1; vertical support V140 |
| RS-150 | 6.000, 20.000, 7.000 | Reducer R1, large end |
| RS-160 | 6.000, 20.400, 6.975 | Reducer R1, small end (eccentric, flat on bottom) |
| RS-170 | 6.000, 23.000, 6.975 | Guide G2 |
| RS-180 | 6.000, 25.000, 6.975 | Expansion joint EJ1, face i |
| RS-190 | 6.000, 25.600, 6.975 | Expansion joint EJ1, face j |
| RS-200 | 6.000, 28.000, 6.975 | Vertical support V200 |
| RS-210 | 6.000, 30.000, 6.975 | Anchor B (nozzle B) |
| RS-215 | 6.000, 15.000, 4.500 | Branch guide G3 |
| RS-220 | 6.000, 15.000, 2.250 | Bend B3 near tangent |
| RS-225 | 6.250, 15.000, 2.000 | Bend B3 far tangent |
| RS-230 | 8.000, 15.000, 2.000 | Anchor C (nozzle C; seismic anchor movement) |

Corner points: B1 (6, 0, 1), B2 (6, 0, 7), B3 (6, 15, 2). Bend authoring realizes each
corner into its two tangent nodes and its arc (§11.2). The eccentric drop of 25 mm at
RS-160 is half the outside-diameter difference (270 − 220 mm)/2, so the bottom of pipe stays
at Z = 6.865 m.

### 3.2 Members

| Member | From → to | Section | Carries |
|---|---|---|---|
| RS-P010 | 010 → 020 | S270 | |
| RS-P020 | 020 → 030 | S270 | Flange F1 |
| RS-P030 | 030 → 040 | S270 | Valve V1 |
| RS-P040 | 040 → 050 | S270 | Flange F2 |
| RS-P050 | 050 → 060 | S270 | |
| RS-P060 | 060 → 070 | S270 | |
| RS-P065 | 070 → 075 | S270 | Bend B1 (arc) |
| RS-P070 | 075 → 080 | S270 | |
| RS-P080 | 080 → 090 | S270 | |
| RS-P085 | 090 → 095 | S270 | Bend B2 (arc) |
| RS-P090 | 095 → 100 | S270 | |
| RS-P100 | 100 → 110 | S270 | |
| RS-P110 | 110 → 120 | S270 | |
| RS-P120 | 120 → 130 | S270 | Tee T1 header (in) |
| RS-P130 | 130 → 140 | S270 | Tee T1 header (out) |
| RS-P140 | 140 → 150 | S270 | |
| RS-P150 | 150 → 160 | S270 | Reducer R1 |
| RS-P160 | 160 → 170 | S220 | |
| RS-P170 | 170 → 180 | S220 | EJ1 face pipe i |
| RS-P180 | 180 → 190 | S220 | Replaced by EJ1 (carries no load or result) |
| RS-P190 | 190 → 200 | S220 | EJ1 face pipe j |
| RS-P200 | 200 → 210 | S220 | |
| RS-P210 | 130 → 215 | S170 | Tee T1 branch |
| RS-P220 | 215 → 220 | S170 | |
| RS-P225 | 220 → 225 | S170 | Bend B3 (arc) |
| RS-P230 | 225 → 230 | S170 | |

Each member's label is its tag, followed by what it carries in parentheses when it carries
something. While the reducer is pending (§10), RS-P150's label reads
`RS-P150 (Reducer R1 (plain S270 member in tranche 1))`.

Pipe `y_reference` (orientation): horizontal straights take global +Z and vertical straights
global +X. Each bend arc takes the direction of its incoming straight toward the corner, as
`insert_bend_at_corner` sets it: +X for B1, +Z for B2 and −Z for B3.

## 4. Material and sections

### 4.1 Material RS-M1 (invented, carbon-steel-like)

- `constitutive_basis` `homogeneous_isotropic_E_nu_v1`; elastic modulus 200 GPa;
  `poisson_ratio` 0.30; `thermal_expansion_coefficient` 12.0 × 10⁻⁶ 1/K, constant;
  density 7850 kg/m³ (on each member's section as `material_density`).
- `temperature_points` (elastic modulus, invented): `point:RS-M1-T020` 20 °C 200 GPa;
  `point:RS-M1-T100` 100 °C 195 GPa; `point:RS-M1-T200` 200 °C 189 GPa;
  `point:RS-M1-T300` 300 °C 182 GPa. Each point repeats ν and α.
- One expansion law, `law:RS-M1-SECANT`: `engineering_secant`, datum 20 °C, constant
  coefficient 12.0 × 10⁻⁶ 1/K.
- The operating case selects each member's modulus at the operating temperature through its
  element state's `material_selection` (`temperature_interpolation`, 210 °C,
  `piecewise_linear`, extrapolation `forbidden`). The other cases select the 20 °C point
  (`exact_point`). The case-level `modulus_basis_ref` and `modulus_basis_temperature` are
  not used: on a load/reference-state document they are refused
  (`LOAD_STATE_CASE_MODULUS_BASIS_UNSUPPORTED`). The temperature table stays in the system
  even though the MBF writer refuses it (§11.1).

### 4.2 Sections (invented, non-standard)

| Section | OD × wall (mm) | Mill tolerance (mm) | Corrosion allowance (mm) | Insulation (mm @ kg/m³) | Contents (kg/m³) |
|---|---|---|---|---|---|
| RS-S270 | 270 × 9.0 | 1.0 | 1.5 | 60 @ 140 | 820 |
| RS-S220 | 220 × 8.0 | 0.9 | 1.5 | 60 @ 140 | 820 |
| RS-S170 | 170 × 7.0 | 0.8 | 1.5 | 50 @ 140 | 820 |

Each section record (`section:RS-S270`, named `RS-S270`, and so on) holds the outside
diameter and wall. Each member references its section and carries the full section data in
its own `section`: the outside diameter and wall again, mill tolerance, corrosion
allowance, material density 7850 kg/m³, insulation and contents. Every member declares its
corrosion allowance (EX requires it). Under EX, stiffness and pipe metal mass use the
nominal wall; contents mass and pressure end-cap areas use the reduced bore (nominal wall
less mill tolerance and corrosion allowance). The hydrotest's test contents are
1000 kg/m³ (§8.3).

## 5. Components

All modifier and stiffness values are user inputs, invented here; SWBPIPE supplies no
default k, SIF or stiffness.

| Component | Kind | Maps / node | Data |
|---|---|---|---|
| RS-B1 | `bend`, realized (`curved_bend_macro_element`), at RS-070 | arc RS-P065 | R 0.400 m, 90°; k 4.2; SIF in-plane 2.1, out-of-plane 1.8 |
| RS-B2 | `bend`, realized, at RS-090 | arc RS-P085 | R 0.400 m, 90°; k 4.2; SIF 2.1 / 1.8 |
| RS-B3 | `bend`, realized, at RS-220 | arc RS-P225 | R 0.250 m, 90°; k 3.6; SIF 2.3 / 1.9 |
| RS-T1 | `tee` at RS-130 | header `RS-P120`, branch `RS-P210` | `branch_header_sif_user_value` 2.4; `branch_branch_sif_user_value` 2.0; connection angle 90°; run size 170 mm (the branch outlet) and header size 270 mm; connection type `invented unreinforced tee`; reinforcement reference `invented: no reinforcement`. The sizes, type and reference are descriptive geometry that tee creation requires; EX joins the tee rigidly and reads none of them. No flexibility factor: branch flexibility is assembled on neither route. No `mechanics_interface`, which the solver reads as `mechanics_geometry_only` |
| RS-R1 | `reducer`, eccentric | RS-P150, node RS-160 | end sizes 270 / 220 mm; `eccentric_offset` (0, 0, +25) mm (from the centreline at RS-160 to the centreline at RS-150, the body's other end); weight 300 N, CoG at the body midpoint; stiffness scale 2.0 |
| RS-F1 | `flange` | RS-P020, node RS-030 | weight 350 N; CoG at the body midpoint; stiffness scale 10.0 |
| RS-V1 | `valve` | RS-P030, node RS-040 | weight 2600 N; CoG offset (0, 0, +0.30) m from the body midpoint (actuator above); stiffness scale 10.0 |
| RS-F2 | `flange` | RS-P040, node RS-050 | weight 350 N; CoG at the body midpoint; stiffness scale 10.0 |
| RS-EJ1 | `expansion_joint`, objective connector 1.0.0 | replaces RS-P180, faces RS-180 / RS-190 | §5.1 |

The bend and tee labels are `Bend B1`, `Bend B2`, `Bend B3` and `Tee T1`; the valve and
flanges are `Valve V1`, `Flange F1` and `Flange F2`. The bends' k and SIFs, and the tee's
SIFs, are the person's entries with their source (§11.3).

The member under the eccentric reducer RS-R1, RS-P150, carries section S270, the larger
end's. The reducer's weight and its centre of gravity are those its row gives.

The valve and flanges are rigid-family components (`mechanics_geometry_only`). Each body is
its mapped member: body length the member's length, both end sizes 270 mm, end references
`node:<start> (body end on <member>)` and `node:<end> (body end on <member>)`. The
stiffness scales are user values with a non-blank source reference, and each component's
stiffness behaviour reference reads `rigid body, user stiffness scale 10.0 (DEC-045);
invented user value, DEL-09-01`. The centre of gravity is entered from the component's
node, in global axes; "body midpoint" therefore becomes (−0.075, 0, 0) m for RS-F1 and
RS-F2 and (−0.3, 0, 0.3) m for RS-V1.

### 5.1 Expansion joint RS-EJ1

- Label `Expansion joint EJ1 (tied)`, at node RS-190. Topology `replaces_span` RS-P180
  (0.600 m). Connector axes, a right-handed triad: x along global +Y (the pipe axis), y
  along global −X, z along global +Z. Both end node axes global; attachment offsets zero
  (required for a pressurized joint).
- Work matrix (diagonal, invented): axial 150 kN/m; lateral 400 kN/m in each lateral
  direction; torsion 500 kN·m/rad; bending 1.5 kN·m/rad about each lateral axis.
- `reference_state` `stress_free`; `q_ref` zero; installed reference temperature 20 °C
  with `temperature_applicability` `fixed_installed_parameters_v1`: the connector has no
  temperature law, so the operating temperature does not change its parameters.
- The replaced span RS-P180 carries no load, member result or self-weight. It is excluded
  from weight generation and from every primitive load, and its element state in each case
  has zero eigenstrain (`thermal_state` `unchanged_reference`). A load on the replaced span
  is refused (`JOINT_REPLACED_SPAN_LOAD_UNOWNED`).
- `pressure_model` `constant_effective_area_reference_v1`, effective area 0.0490 m², basis
  `internal_differential_zero_external_v1`.
- `hardware` `explicit_axial_rods_v1`: two bilateral linear rods, `rod:RS-EJ1-RYP` at
  connector y = +0.200 m and `rod:RS-EJ1-RYN` at y = −0.200 m (z = 0, at both ends), each
  1.2 × 10⁸ N/m, reference extension zero.
- Calibration `constant_structural_elasticity_v1`, installed geometry `authored model
  geometry`. The connector's provenance: measurement restraints `invented; no measurement`,
  basis transform reference `none`, validity statement `invented test object; no
  applicability claim`. The stiffness's own provenance: measurement restraints `invented
  diagonal work matrix; no measurement`, basis transform reference `none (authored directly
  in the connector basis)`.
- Regions: EJ1 is a coaxial connector inside region RG-4 (§7), with straight face pipes
  RS-P170 and RS-P190 in that region; neither face is a region terminal; no arc or other
  connector sits at a face.

## 6. Supports

All supports are axis-aligned and carry no authored direction.

| Support | Node | Family | Data | Why here |
|---|---|---|---|---|
| RS-A | RS-010 | `anchor` | all six DOF | Nozzle A |
| RS-B | RS-210 | `anchor` | all six DOF | Nozzle B |
| RS-C | RS-230 | `anchor` | all six DOF | Nozzle C; seismic anchor movement point |
| RS-LS1 | RS-060 | `line_stop` | UX | Axial stop on the first run, so thermal growth of the run is directed into the riser |
| RS-V060 | RS-060 | `vertical_support` | UZ | Carries the valve run |
| RS-VS1 | RS-080 | `variable_spring_hanger` | no restraint; hanger stiffness UZ 180 kN/m; installed and cold load 8.0 kN, hot load 7.4 kN, travel range 50 mm (all invented); `mechanics_consumption` `linear_spring_primitive_user_stiffness` | Riser, where vertical growth rules out a rigid support; sized by hanger sizing (§8.8) |
| RS-CE1 | RS-100 | `constant_effort_support` | UZ; `constant_load` 9.0 kN (invented input) | Near bend B2, where vertical movement is largest; sized by hanger sizing |
| RS-LO1 | RS-110 | `nonlinear` | `lift_off`, UZ, initial state active, contact on positive reaction | A rack support near the riser, where operating movement can lift the pipe; whether it lifts is a solve result, not claimed |
| RS-FR1V | RS-120 | `nonlinear` | `one_way`, UZ (resting); in tranche 1 an interim linear `vertical_support` UZ (§10) | Normal-force source for RS-FR1, so criterion 1 exercises a lift-off friction shoe |
| RS-FR1 | RS-120 | `nonlinear` | `friction`, UY (axial), μ 0.30 (invented), normal reaction from RS-FR1V UZ | Axial sliding along the rack |
| RS-G1 | RS-140 | `nonlinear` | `gap`, UX, initial state inactive, closes on positive displacement, gap 5 mm | A lateral stop with clearance beyond the tee |
| RS-V140 | RS-140 | `vertical_support` | UZ | Carries the header beyond the tee |
| RS-G2 | RS-170 | `guide` | UX, UZ | Guides the reduced run ahead of EJ1 |
| RS-V200 | RS-200 | `vertical_support` | UZ | Carries the run beyond EJ1 |
| RS-G3 | RS-215 | `guide` | UX, UY | Guides the branch drop |
| RS-SN1 | RS-110 | `snubber` | UX, rigid | Dynamic-only lateral restraint on the rack, where no lateral restraint exists |

The support labels are: `Anchor A (nozzle A)`, `Anchor B (nozzle B)`, `Anchor C (nozzle C)`,
`Line stop LS1`, `Vertical support V060`, `Variable spring VS1 (user stiffness)`,
`RS-FR1V interim linear vertical support (one_way target pending R-4 S7)`,
`Vertical support V140`, `Guide G2`, `Vertical support V200` and `Branch guide G3`.

The constant-effort force acts along the positive UZ axis, its single translational
restraint. A `variable_spring_hanger` must carry entered installed, cold and hot loads and a
travel range or movement limit (`SPRING_HANGER_LOAD_MISSING`, `SPRING_HANGER_TRAVEL_MISSING`),
so RS-VS1 carries invented ones. They are review metadata: only the stiffness is consumed,
and each entered load warns `SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`. Like the
constant-effort load, they are inputs; hanger sizing reports its own sized loads and travel
as results (§8.8) and does not overwrite them. Every static solve refuses a model with a
snubber (`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`), so RS-SN1 enters only with the response
spectrum (§8.5).

## 7. Pressure regions

Every terminal is `transfers_to_wall`. A shared terminal that is
`separately_supported_or_compensated` is refused
(`EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED`), and treating each nozzle as a
capped closure keeps one rule throughout. Regions are case-scoped: each pressurized case
(§8) carries the same five regions with its own pressure, with ids `region:<case>-<region>`
(for example `region:RS-SUS-RG-1`) and basis `internal_differential_zero_external_v1`.

| Region | Members | Terminals, in order |
|---|---|---|
| RG-1 | RS-P010 … RS-P120 (14 members: valve and flange members and arcs RS-P065, RS-P085 included) | RS-010, RS-130 |
| RG-2 | RS-P210, RS-P220, RS-P225, RS-P230 | RS-130, RS-230 |
| RG-3 | RS-P130, RS-P140 | RS-130, RS-150 |
| RG-R | RS-P150 (the reducer body) | RS-150, RS-160 |
| RG-4 | RS-P160, RS-P170, RS-P190, RS-P200; connector `component:RS-EJ1` | RS-160, RS-210 |

Regions split at the tee node with every terminal there `transfers_to_wall`, and the reducer
body is its own region joined to its neighbours at its end nodes.

## 8. Load cases, combinations and analyses

Operating pressure 1600 kPa; test pressure 2600 kPa; installation temperature 20 °C;
operating temperature 210 °C on every member except the replaced span RS-P180 (§5.1). All
are invented. The system's operating condition is one condition set applied to every
member (`condition_sets[]`, `pipe_segments[].condition_set_ref`,
`load_cases[].condition_use`); until it is authored (§10), each case states its temperature
in its element states and its pressure in its regions. The condition set carries
temperature, pressure and installation temperature only: the operating contents are the
sections' `contents_density` (§4.2), with no condition contents override, and no case
carries condition `added_weight`. Only RS-HYD uses `condition_use.contents_density` (§8.3).
Component weights are rigid-component `weight` at a centre of gravity, not added weight.

With nonlinear supports in the model, every static case is a nonlinear solve with its own
support status. Occasional effects are therefore cases that include the sustained loads,
and each occasional term is a difference from RS-SUS. With a constant-effort support, SRSS
and absolute-sum operands must count the constant-effort force zero times, so they are
differences, not the plain occasional cases.

All cases share one reference configuration, `reference:RS-INSTALLED` (`Installed
reference (20 degC)`, authored model geometry), whose 26 members each take the
`temperature_reference` basis at 20 °C with fit `none`. Each case's analysis state
(`openpipestress.load_reference_state/1.0.0`, history `independent_equilibrium`) holds one
element state per member, one support state per support (`active_model_device`) and its
self-weight primitives as load sources at factor 1.

### 8.1 Load cases

| Case | Content | Analysis (criterion 1) |
|---|---|---|
| RS-SUS | W + P (operating pressure) | Sustained |
| RS-OPE | W + P + T (operating temperature, modulus at 210 °C) | Operating |
| RS-HYD | W with test contents + test pressure; `test_condition` `{kind: hydrostatic, held_supports: springs_and_constant_effort}` | Hydrotest |
| RS-WIN | W + P + wind: 0.9 kPa, shape factor 0.7, `global_x`, exposed members RS-P070 … RS-P200 except the replaced span RS-P180 | Wind |
| RS-EQX | W + P + seismic, `g_factor_x` 0.30 | Static seismic, X |
| RS-EQY | W + P + seismic, `g_factor_y` 0.30 | Static seismic, Y |
| RS-EQZ | W + P + seismic, `g_factor_z` 0.20 | Static seismic, Z |
| RS-SAM | W + P + anchor RS-C moved UX +15 mm, UY −10 mm | Seismic anchor movement |

The case labels in the fixture are `RS-SUS sustained: W + P (operating pressure)`,
`RS-OPE operating: W + P + T (210 degC, modulus at 210 degC)` and
`RS-SAM seismic anchor movement: W + P + anchor RS-C moved UX +15 mm, UY -10 mm`.

W is generated self-weight of every member except RS-P180 (gravity as §2), with component
weights at their centres of gravity in place of their members' distributed weight: 22 line
loads on the straight and arc members other than RS-P020, RS-P030 and RS-P040, and a force
and a moment for each of RS-F1, RS-V1 and RS-F2, 28 primitives per case. The fixture's W is
the product's self-weight generator's output for each case (`core/product_physics/src/self_weight.rs`, through
`core/loads/self_weight_wasm`), captured in
[`self_weight_capture.json`](self_weight_capture.json); the fixture generator copies it and
computes no weight. Each case's provenance, and its request's, is the record provenance
(§2) followed by `; W of <case>: self-weight of every member except RS-P180, component
weights at their centres of gravity (spec §8.1)`.

Each static seismic case, RS-EQX, RS-EQY and RS-EQZ, enters a gravity acceleration of
9.80665 m/s² as a positive magnitude and one factor on its own axis: `g_factor_x` 0.30,
`g_factor_y` 0.30 and `g_factor_z` 0.20 respectively, each positive. The factor's sign is the
direction's sign.

Each self-weight primitive's id is derived from its case and its target:
`load:self-weight:<n>:<case id>:<m>:<pipe id>` for a member's line load, and
`load:self-weight:<n>:<case id>:component:<m>:<component id>:force:z` or
`…:moment:<axis>` for a component's weight, where `<n>` and `<m>` are the character counts
of the id that follows each. For example `load:self-weight:11:load:RS-SUS:12:pipe:RS-P010`.

Wind is uniform pressure over the marked members (wind on a replaced span is refused). On
EX, the anchor movement is `analysis_state.support_states[].boundary_motion` with meaning
`absolute_reference_displacement`; the support-targeted primitive is refused on EX.

### 8.2 Combinations

| Combination | Basis | Operands | Analysis |
|---|---|---|---|
| RS-EXP | `result_state_subtraction`, with `difference_basis` recorded | RS-OPE − RS-SUS | Expansion with its basis |
| RS-dWIN | `result_state_subtraction` | RS-WIN − RS-SUS | Wind term |
| RS-dEQX, RS-dEQY, RS-dEQZ | `result_state_subtraction` | RS-EQX, RS-EQY, RS-EQZ, each − RS-SUS | Seismic terms |
| RS-dSAM | `result_state_subtraction` | RS-SAM − RS-SUS | Anchor movement term |
| RS-EQ-SRSS | `srss` | RS-dEQX, RS-dEQY, RS-dEQZ | Directional SRSS |
| RS-EQ-ABS | `absolute_sum` | RS-dEQX, RS-dEQY, RS-dEQZ | Directional absolute sum |
| RS-OCC-SRSS | `mechanics` with combination operands | RS-SUS + RS-EQ-SRSS | Sustained plus seismic |
| RS-OCC-ABS | `mechanics` with combination operands | RS-SUS + RS-EQ-ABS | Sustained plus seismic |

The labels of the two in the fixture are `RS-EXP expansion: RS-OPE - RS-SUS` and
`RS-dSAM anchor movement term: RS-SAM - RS-SUS`. Sustained taking the operating support
status (`subtrahend_support_state`) is an option the system does not use.

### 8.3 Hydrotest

RS-HYD holds the spring and the constant-effort support (`held_supports:
springs_and_constant_effort`). The test contents density (1000 kg/m³) is the case's
`condition_use.contents_density`. No temperature applies.

### 8.4 Modal

Modal analysis is a request beside the model, not a model member. The request names RS-OPE
as its required state case: the modal model's stiffness, member moduli, support states and
contents mass come from RS-OPE's resolved state, nonlinear supports are linearized in its
status, and `boundary_motion` degrees of freedom stay restrained. Modes up to 60 Hz are
extracted, with at least the first ten reported.

### 8.5 Response spectrum

One invented spectrum, `spectrum:RS-SP1` in `spectra[]`: abscissa frequency (Hz), ordinate
`g_multiple` with `g_value` 9.80665 m/s², damping ratio 0.05, log–log interpolation.
Points: (0.5, 0.12), (2.0, 0.75), (9.0, 0.75), (40.0, 0.30), (100.0, 0.30).

The spectrum request, beside the model, applies RS-SP1 in X, Y and Z with scale factors 1.0,
1.0 and 0.67, uses the modal request of §8.4, and turns on the missing-mass correction. Its
mode and directional combination methods are DEL-04-10's to define. The snubber RS-SN1 (§6)
is the system's dynamic-only restraint.

### 8.6 Equilibrium checks need no extra case

Each check in §9.2 reads cases that already exist, so the model carries no auxiliary
weight-only or pressure-only case.

### 8.7 Seismic anchor movement and static seismic on one document

On EX, `boundary_motion` and the temperature state both need the load/reference-state
group, and EX refuses generated wind and seismic loads on that group
(`EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED`). While that holds, one document cannot
hold RS-OPE, RS-SAM and RS-WIN, RS-EQX, RS-EQY or RS-EQZ together.

### 8.8 Hanger sizing

Hanger sizing is a request beside the model. It sizes RS-VS1 and RS-CE1 from an invented
private catalogue (invented records only) and produces a hanger table. The procedure, its
load cases and its record selection rule belong to DEL-04-11. Sizing never overwrites
RS-VS1's or RS-CE1's entered inputs.

## 9. Key results

### 9.1 Named results

| Id | Result | Cases | Basis |
|---|---|---|---|
| KR-01 | Reactions at anchors A, B and C (three forces, three moments) | every case and combination | verified solve |
| KR-02 | Largest translation and its node | RS-SUS, RS-OPE, RS-EXP, RS-HYD, RS-OCC-SRSS, RS-OCC-ABS | verified solve |
| KR-03 | Spring RS-VS1 and constant-effort RS-CE1 loads | RS-SUS, RS-OPE | verified solve |
| KR-04 | Nonlinear support status: RS-LO1 in contact or lifted, RS-G1 open or closed, RS-FR1 sticking or sliding, with RS-FR1's force | RS-OPE, RS-SUS | verified solve; status must match exactly |
| KR-05 | Tee intensified stress rows at the header (RS-P120) and branch (RS-P210) ends at RS-130, on both section bases | RS-SUS, RS-OPE, RS-EXP | presence: identity; values: verified solve |
| KR-06 | RS-EJ1 tie-rod forces, joint pressure force pAe and effective axial force | RS-SUS, RS-OPE, RS-HYD | verified solve |
| KR-07 | Hydrotest marked as a test case, with RS-VS1 and RS-CE1 held | RS-HYD | identity (published flags) |
| KR-08 | Frequencies up to 60 Hz, mass participation per direction | modal | verified solve |
| KR-09 | Spectrum reactions at A, B, C and displacements, per direction and combined | spectrum | verified solve |
| KR-10 | Hanger table: per hanger, the sized load, travel and selected invented record | hanger sizing | verified solve |

### 9.2 References that need no invented values

| Id | Check | Basis |
|---|---|---|
| EQ-1 | The sum of vertical reactions over all supports, the spring and the constant-effort support included, equals the total weight, in RS-SUS, RS-OPE and RS-SAM, and in RS-HYD (with the test contents) when it is authored | closed form: the total weight recomputed independently from the inputs (§4, §5) on EX's stated mass bases |
| EQ-2 | In RS-SUS, RS-OPE and RS-SAM, and in RS-HYD when it is authored, the horizontal reaction resultant is zero, and the reaction moment resultant about the origin equals minus the moment of the weight. Every closure transfers to the wall, so each region, the tee junction and the pressurized joint are self-equilibrated under pressure, and temperature adds no load | closed form from the inputs |
| EQ-3 | On every published signed component row (support force and moment components, translation and rotation components, element end-force components), RS-EXP equals RS-OPE − RS-SUS. Magnitude rows (resultant displacement, reaction magnitude) are checked by recomputing them from the combined components, not by subtracting magnitudes. Rows the combination withholds, such as tie-rod forces, are excluded | exact identity |
| EQ-4 | On the same signed component rows, RS-EQ-SRSS equals √(Σ dEQ²) and RS-EQ-ABS equals Σ \|dEQ\| over the operand rows, so SRSS ≤ ABS on each. Magnitude rows follow the combination's own recomputation rule, as for EQ-3; withheld rows are excluded | exact identity |
| EQ-5 | The RS-dSAM reactions are self-equilibrated (resultant force and moment zero) | exact identity: no load differs between RS-SAM and RS-SUS |
| EQ-6 | The resultant of RS-dWIN reactions equals minus the generated wind resultant, and that of each of RS-dEQX, RS-dEQY and RS-dEQZ equals minus the factor times g times the total mass on the generator's stated mass basis | closed form from the inputs |
| EQ-7 | Spectrum directional combination recomputed from the published per-direction results | exact identity |

Each closed-form reference is computed by an independent generator in
[`validation/references/reference_system/`](../../validation/references/reference_system/),
not by the solver under test. EQ-1 and EQ-2 are frozen there for the cases the fixture holds
(the total weight and its moment about the origin, recomputed from §3–§5 without the
captured self-weight); EQ-3 and EQ-5 are identities recorded in its
[README](../../validation/references/reference_system/README.md).
[`tests/test_reference_system_fixture.py`](../../tests/test_reference_system_fixture.py)
checks all four in both solver modes. RS-HYD's weight with the test contents is referenced
when RS-HYD is authored. The same test checks KR-05's presence, an identity: the tee's rows
at the RS-130 ends of RS-P120 and RS-P210 on both section bases in RS-SUS and RS-OPE
(RS-EXP withholds intensified rows); KR-05's values wait for a verified solve.

## 10. Pending-family ledger

The fixture contains every row marked "in tranche 1", and its cases RS-SUS, RS-OPE and
RS-SAM solve on EX with no blocking diagnostic
([`tests/test_reference_system_fixture.py`](../../tests/test_reference_system_fixture.py)).
A pending row keeps its place and identities in this document; when it is authored, the
generator adds its entities without moving a node. The generator holds the same rows, by
family, with one pending flag each, and writes them to `authoring.json` (`pending`); the
first column is the key the two share, and
[`tests/test_reference_system_records.py`](../../tests/test_reference_system_records.py)
checks that the families, the flags and the identities agree. "What the fixture shows" is
what a solve of the fixture shows today; a family the fixture does not contain is absent
from it. Whether the exact route admits a pending family is a matter for the deliverable
that owns it under exit criterion 1.

| Family or analysis | State | Identities | What the fixture shows |
|---|---|---|---|
| Straight members, sections, temperature-dependent material | in tranche 1 | — | solved |
| Realized bends with user k and SIFs | in tranche 1 | — | solved |
| Valve and flanges, weight at CoG, stiffness scale | in tranche 1 | — | solved |
| Expansion joint with tie rods, pressurized | in tranche 1 | — | solved |
| Pressure (five regions, shared terminals `transfers_to_wall`) | in tranche 1 | — | solved |
| Anchors, guides, line stop, vertical supports | in tranche 1 | — | solved |
| Variable spring as a user-stiffness spring | in tranche 1 | — | solved |
| Tee RS-T1 with user SIFs | in tranche 1 | `component:RS-T1` | solved, with KR-05's rows present |
| Eccentric reducer RS-R1 in region RG-R | pending; RS-P150 is in tranche 1 as a plain S270 member in its own region RG-R | `component:RS-R1` | absent; RS-P150 is a plain S270 member |
| Constant-effort support RS-CE1 | pending | `support:RS-CE1` | absent |
| Lift-off, friction and gap (RS-LO1, RS-FR1, RS-G1) | pending | `support:RS-LO1`, `support:RS-FR1`, `support:RS-G1` | absent |
| RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7 | pending; RS-FR1V is in tranche 1 as an interim linear `vertical_support` UZ | `support:RS-FR1V` | RS-FR1V solved as a linear vertical support; no one-way support and no friction |
| RS-SUS, RS-OPE (thermal, modulus by temperature) | in tranche 1 as linear cases (load/reference-state group) | — | solved |
| RS-EXP | in tranche 1 as a subtraction; `difference_basis` pending | — | solved as a subtraction, with no `difference_basis` |
| RS-WIN, RS-EQX, RS-EQY, RS-EQZ on the document that carries temperature | pending | `load:RS-WIN`, `load:RS-EQX`, `load:RS-EQY`, `load:RS-EQZ`, `combination:RS-dWIN`, `combination:RS-dEQX`, `combination:RS-dEQY`, `combination:RS-dEQZ` | absent; on EX they would meet §8.7's refusal |
| RS-SAM (`boundary_motion` on rigid anchor RS-C) | in tranche 1, on the load/reference-state document beside the temperature state | — | solved |
| RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS | pending | `combination:RS-EQ-SRSS`, `combination:RS-EQ-ABS`, `combination:RS-OCC-SRSS`, `combination:RS-OCC-ABS` | absent |
| RS-HYD | pending | `load:RS-HYD` | absent |
| Operating condition set | pending | — | absent; each case states its temperature and pressure (§8) |
| Modal (RS-OPE linearization) | pending | — | absent: no modal request |
| Response spectrum, RS-SP1 | pending | `spectrum:RS-SP1` | absent: no spectrum and no spectrum request |
| Hanger sizing and table | pending | — | absent: no sizing request |
| Snubber RS-SN1 (RS-110, UX, rigid) | pending | `support:RS-SN1` | absent; on any static solve it would be refused (§6) |

With the pending supports and the reducer overlaid on the full system in tests only
([`core/product_physics/tests/rt_a3_s6_reference_system.rs`](../../core/product_physics/tests/rt_a3_s6_reference_system.rs)
and
[`core/product_physics/tests/rt_a3_reducer_2b_reference_system.rs`](../../core/product_physics/tests/rt_a3_reducer_2b_reference_system.rs)),
the exact route admits RS-R1, RS-CE1, RS-LO1, RS-FR1 and RS-G1: the combined test overlay
solves with no blocking finding. Every case of that overlay is published unqualified,
numerical quality `unresolved` with `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`,
because a published case of a solve with a gap support is never qualified or confirmed, and
its report package is refused (LIM-017 and LIM-016 in
[`docs/RELEASE_LIMITATIONS_v1.0.md`](../../docs/RELEASE_LIMITATIONS_v1.0.md)), and its rendered report states its standing. Removing
RS-G1 does not restore eligibility: the same overlay without RS-G1 publishes every case
`NUMERICAL_INTEGRITY_SENSITIVE`, and its report package is refused too (LIM-017), and its rendered report states its standing. Both overlays use
the interim linear RS-FR1V. With RS-FR1's intended one-way normal source, they solve and are
not confirmed: the confirmation admits only a base restraint as a derived normal's source, so
without RS-G1 each case is `NUMERICAL_INTEGRITY_SENSITIVE` with that named reason. The
published support states are reported, not
verified (KR-04). None of these families is in the fixture until this specification adopts
it.

**RS-P180 under MBF export (a coverage note, not a ledger row).** RS-OPE selects RS-P180's
modulus at 210 °C (`material_selection` `temperature_interpolation`) while its
`thermal_state` is `unchanged_reference`. When the MBF writer exports load/reference-state
thermal cases, a member exported at a temperature that is not its reference temperature
needs a rule for RS-P180; whether RS-EJ1's MBF representation removes RS-P180 from the
exported members is DEL-17-04's to establish. The fixture keeps RS-P180 as it is.

## 11. Consumer views

### 11.1 Criterion 6 (MBF)

The identity set is DEL-17-04's: nodes, pipes, components, supports, load cases, primitive
loads, materials, pressure regions and sections. Exit criterion 6 requires that the desktop
save this system as MBF with no family of criterion 1's system omitted or unsupported in
the loss report, and import it back with the same identities; DEL-17-04 AC-003 judges it,
with every Must family marked covered or carrying its named refusal.

What an export of the 0.9.0 fixture can show follows the ledger (§10). The fixture holds the
temperature-dependent modulus table (§4.1), the realized bends, the tee, the valve and
flanges, the expansion joint and its rods, the variable spring, the linear supports, the
cases RS-SUS, RS-OPE and RS-SAM and the combinations RS-EXP and RS-dSAM. The writer refuses
the temperature table today (`MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED`); it stays
in the system, and no family is dropped to make criterion 6 pass. Which of the other families
the fixture holds the loss report lists is DEL-17-04's to state from an export of the
fixture. The eccentric reducer, the constant-effort, lift-off, friction and gap supports, the
snubber, the other cases and combinations, the spectrum and the modal and hanger-sizing
requests are pending (§10): the fixture does not hold them, so no export of it reports them.

Each family moves to "covered" as a coverage tranche lands. Any family with no admitted
public MBF source is listed in the release limitations and declared acceptable by Agent 0,
and the limitations are shown to the owner with the request for the verdict (exit criterion
3). Declaring it acceptable does not demonstrate the refused capability and does not make
criterion 6 met.

### 11.2 Criterion 7 (authoring time)

Criterion 7 says the system is "entered by run sheet and solved in ≤ 30 minutes
*(provisional)*". The run sheet enters nodes, members, realized bends, the valve and
flanges and ten of the eleven supports through the desktop's run sheet, and everything else
through the desktop's forms and load manager. [`RUN_SHEET.md`](RUN_SHEET.md) gives the
steps for the fixture as it is, their entry counts and an estimate of the time, and lists
every difference between the model it yields and the fixture. When a pending row of §10 is
authored, the run sheet gains the steps that enter it.

### 11.3 Criterion 17 (embedded agent)

The agent builds the system through the catalog's operation route. The operation families
it needs, by the applier's vocabulary
([`core/model_operations/operation_applier/src/`](../../core/model_operations/operation_applier/src/)):

| Need | Applier family |
|---|---|
| Project units | `set_project_unit` |
| Material with temperature points and expansion law | `create_material` (`rich_authoring.rs`, `load_state_authoring.rs`) |
| Sections | `create_section`, `assign_section` |
| Nodes and members | `create_node`, `connect_pipe_run` |
| Realized bends | `insert_bend_at_corner`, `bend_realization` (`bend_authoring.rs`) |
| Tee, reducer, valve, flanges | `insert_component_symbol`, `set_field` on component members |
| Expansion joint and rods; pressure contract and regions | `joint_pressure_authoring.rs`, `pressure_authoring.rs` |
| Supports, nonlinear members, hanger data | `create_support`, `update_support` (`rich_authoring.rs`, `boundary_association.rs`) |
| Self-weight and primitive loads | `create_load_case`, `create_primitive_load`, the self-weight generator |
| Wind and seismic | `equivalent_static` on the case |
| Temperature states and anchor movement | `load_state_authoring.rs` (`reference_configurations`, `analysis_state`) |
| Combinations | `create_combination`, `create_combination_term` |
| Condition set | `create_condition_set` (`condition_set_authoring.rs`) |
| Solve and analysis requests | the catalog's solve and read tools; the modal, spectrum and hanger-sizing requests |

The key results the agent reports are §9.1's.

**Bend k is user entry.** The applier refuses a flexibility factor k without a user entry
and its source (`OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED`, `bend_authoring.rs`), and the
host refuses an agent-written k. Catalog admission alone cannot let the agent realize
B1–B3: criterion 17 needs a route in which the person enters k and its source (here, §5's
values) and the operation records the person as that value's author, without relabelling
agent authorship. `authoring.json` marks these inputs, and the bend and tee SIFs with their
sources, as the person's.

## 12. Maintaining this document

- Geometry, ids and invented inputs change here first, then in the generator, so that every
  consumer's target moves together.
- When a pending row is authored, the same change flips its flag in the generator,
  regenerates the fixture, updates §10 and the run sheet, and adds any verified expected
  value with its basis.
- At release, every row still pending here has an entry in
  [`docs/RELEASE_LIMITATIONS_v1.0.md`](../../docs/RELEASE_LIMITATIONS_v1.0.md), the
  criterion 3 list.
