# The reference system (exit criterion 1)

**Basis:** `main` at `4d8cb7b0c`, model document 0.8.0. Model document 0.9.0's member
list, frozen by Agent 0 at `31bde47a2` on `claude/rt-load-cases`
(DEL-02-01 `Work/model-document-0.9.0.md` §1.2, §2.1, §2.2, §2.4), is cited where the
target needs a 0.9.0 member. Release target `docs/RELEASE_TARGET_v1.0.md` §4, criteria 1,
6, 7 and 17.

This document specifies one invented piping system that four exit criteria test
against. The owner placed it in DEL-09-01 through Agent 0 (2026-10-31, P-1), replacing
DEL-17-04 `Work/desktop-mbf-export-import.md` §5 O-1's assignment to W-17. It is a
specification and a ledger. The model document itself is generated
(`fixtures/reference_system/generate_reference_system.py`), and its run sheet will be
`reference-system-run-sheet.md` beside this document.

Every value here is invented for this fixture: geometry, sections, material, loads,
supports and spectra. Section sizes are deliberately non-standard. Nothing is taken from
a standard, a supplier catalogue, a commercial example or a project
(`docs/IP_AND_DATA_BOUNDARY.md`). The system is a test object, not a design; no value is
offered as suitable for any real service. SWBPIPE computes mechanics; it does not
determine code compliance. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081). Meeting a criterion on this system is evidence for the owner's release
decision, not the decision (F-PIP-3).

## 1. Consumers

| Criterion | What it reads here | Where |
|---|---|---|
| 1. One model, every Must analysis | The whole model and its cases (§2–§8) solved on EX in the desktop with no admitted family refused; the key results of §9 | §9, §10 |
| 6. MBF round trip | The model saved as MBF and imported back: the families of §11.1 in the loss report, and the identity set | §11.1 |
| 7. Authoring time | The run sheet and follow-up forms that enter the model from blank, and the time to solve | §11.2, run sheet |
| 17. Embedded agent | The operation families the agent needs to build the model, and the key results it reports | §11.3 |

**Expected values.** A key result gets an expected value only from (a) a closed-form or
hand-calculation reference with an independent generator, (b) an exact identity
between published quantities, or (c) a verified solve after its analysis lands,
frozen in `validation/references/reference_system/` with its basis recorded. No
expected value is entered otherwise, and none is changed to silence a failure. §9
names the results and their basis; it gives no values.

## 2. Conventions

- **Units.** SI. Lengths in m for coordinates and mm for section dimensions; N, N·m,
  kPa, °C, kg/m³. The project's units are SI at creation.
- **Axes.** Global Z is vertical, positive up. This is the fixture's stated choice: the
  solver has no fixed vertical axis. Gravity is entered on the weight case as
  `global_z`, value −9.80665 m/s² (standard gravity, a definitional constant), and the
  seismic case enters `gravity_acceleration` explicitly. Wind and seismic directions
  are named global axes.
- **Identities.** Stable ids, so criterion 6 can compare them: `node:RS-nnn`,
  `pipe:RS-Pnnn`, `component:RS-<tag>`, `support:RS-<tag>`, `load:RS-<tag>`,
  `combination:RS-<tag>`, `material:RS-M1`, `section:RS-S270` and so on. Node numbers
  step by 10, leaving room to insert.
- **Route.** `pressure_contract` `{version: "3.0.0", mode: "exact_pressure_v3",
  shear_model: "euler_bernoulli"}`.

## 3. Geometry

The system runs from equipment nozzle A to equipment nozzle B, with a branch to
equipment nozzle C. It contains three realized bends, a tee, an eccentric reducer, a
flanged valve and a tied expansion joint. It has 27 nodes (corner points are not nodes;
each realized bend has two tangent nodes) and 26 members: 22 straight, three arcs and the
span the expansion joint replaces.

### 3.1 Nodes

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

Corner points: B1 (6, 0, 1), B2 (6, 0, 7), B3 (6, 15, 2). The run sheet enters corners
and the bend authoring realizes them (§11.2). The eccentric drop of 25 mm at RS-160 is
half the outside-diameter difference (270 − 220 mm)/2, so the bottom of pipe stays at
Z = 6.865 m.

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

Pipe `y_reference` (orientation): horizontal straights take global +Z and vertical
straights global +X. Each bend arc takes its incoming direction, as `insert_bend_at_corner`
sets it.

## 4. Material and sections

### 4.1 Material RS-M1 (invented, carbon-steel-like)

- `constitutive_basis` `homogeneous_isotropic_E_nu_v1`; `poisson_ratio` 0.30;
  `thermal_expansion_coefficient` 12.0 × 10⁻⁶ 1/K, constant; density 7850 kg/m³ (on the
  sections as `material_density`).
- `temperature_points` (elastic modulus, invented): 20 °C 200 GPa; 100 °C 195 GPa;
  200 °C 189 GPa; 300 °C 182 GPa. Each point repeats ν and α.
- The operating case selects each member's modulus at the operating temperature
  through its element state's `material_selection` (`kind: temperature_interpolation`,
  210 °C, interpolated between points). The case-level `modulus_basis_ref` and
  `modulus_basis_temperature` (DEC-077/092) are not used: on a load/reference-state
  document they are refused (`LOAD_STATE_CASE_MODULUS_BASIS_UNSUPPORTED`). The
  temperature table is a Must family (RT-G3) and stays in the fixture even though the
  MBF writer refuses it (§11.1).

### 4.2 Sections (invented, non-standard)

| Section | OD × wall (mm) | Mill tolerance (mm) | Corrosion allowance (mm) | Insulation (mm @ kg/m³) | Contents (kg/m³) |
|---|---|---|---|---|---|
| RS-S270 | 270 × 9.0 | 1.0 | 1.5 | 60 @ 140 | 820 |
| RS-S220 | 220 × 8.0 | 0.9 | 1.5 | 60 @ 140 | 820 |
| RS-S170 | 170 × 7.0 | 0.8 | 1.5 | 50 @ 140 | 820 |

Every section declares its corrosion allowance (EX requires it). Material density is
7850 kg/m³ on each. Under EX, stiffness and pipe metal mass use the nominal wall;
contents mass and pressure end-cap areas use the reduced bore (nominal wall less mill
tolerance and corrosion allowance). The hydrotest's test contents are 1000 kg/m³
(§8.3).

## 5. Components

All modifier and stiffness values are user inputs, invented here; SWBPIPE supplies no
default k, SIF or stiffness.

| Component | Kind | Maps / node | Data |
|---|---|---|---|
| RS-B1 | realized bend (`curved_bend_macro_element`) | RS-P065 | R 0.400 m, 90°; k 4.2; SIF in-plane 2.1, out-of-plane 1.8 |
| RS-B2 | realized bend | RS-P085 | R 0.400 m, 90°; k 4.2; SIF 2.1 / 1.8 |
| RS-B3 | realized bend | RS-P225 | R 0.250 m, 90°; k 3.6; SIF 2.3 / 1.9 |
| RS-T1 | `tee` at RS-130 | header `RS-P120`, branch `RS-P210` | `branch_header_sif_user_value` 2.4; `branch_branch_sif_user_value` 2.0; connection angle 90° |
| RS-R1 | `reducer`, eccentric | RS-P150, node RS-160 | end sizes 270 / 220 mm; `eccentric_offset` (0, 0, +25) mm (from the centreline at RS-160 to the centreline at RS-150, the body's other end); weight 300 N, CoG at the body midpoint; stiffness scale 2.0 |
| RS-F1 | `flange` | RS-P020, node RS-030 | weight 350 N; CoG at the body midpoint; stiffness scale 10.0 |
| RS-V1 | `valve` | RS-P030, node RS-040 | weight 2600 N; CoG offset (0, 0, +0.30) m from the body midpoint (actuator above); stiffness scale 10.0 |
| RS-F2 | `flange` | RS-P040, node RS-050 | weight 350 N; CoG at the body midpoint; stiffness scale 10.0 |
| RS-EJ1 | `expansion_joint`, objective connector 1.0.0 | replaces RS-P180, faces RS-180 / RS-190 | §5.1 |

The rigid-family stiffness scales are DEC-045 user values with a non-blank source
reference naming this document. The centre-of-gravity vector is entered from the
component's node, in global axes (R10); the generator converts "body midpoint" into
that vector.

### 5.1 Expansion joint RS-EJ1

- Topology `replaces_span` RS-P180 (0.600 m). Connector axes, a right-handed triad: x
  along global +Y (the pipe axis), y along global −X, z along global +Z. Attachment
  offsets zero (required for a pressurized joint).
- Work matrix (diagonal, invented): axial 150 kN/m; lateral 400 kN/m in each lateral
  direction; bending 1.5 kN·m/rad about each lateral axis; torsion 500 kN·m/rad.
- `reference_state` `stress_free`; `q_ref` zero; installed reference temperature
  20 °C with `temperature_applicability` `fixed_installed_parameters_v1`: the
  connector has no temperature law, so the operating temperature does not change its
  parameters.
- The replaced span RS-P180 carries no load, member result or self-weight. It is
  excluded from weight generation and from every primitive load, and its element
  state in each case has zero eigenstrain (`thermal_state` `unchanged_reference`). A
  load on the replaced span is refused (`JOINT_REPLACED_SPAN_LOAD_UNOWNED`).
- `pressure_model` `constant_effective_area_reference_v1`, effective area 0.0490 m².
- `hardware` `explicit_axial_rods_v1`, two bilateral linear rods at connector
  y = ±0.200 m (z = 0), each 1.2 × 10⁸ N/m, reference extension zero.
- Regions: EJ1 is a coaxial edge inside region RG-4 (§7), with straight face pipes
  RS-P170 and RS-P190 in that region; neither face is a region terminal; no arc or
  other connector sits at a face.

## 6. Supports

All supports are axis-aligned: 0.9.0's `supports[].direction` is not admitted on
lift-off or constant-effort supports (`SUPPORT_DIRECTION_RECORD_UNSUPPORTED`), and no
support here needs a skew.

| Support | Node | Family | Data | Why here |
|---|---|---|---|---|
| RS-A | RS-010 | `anchor` | all six DOF | Nozzle A |
| RS-B | RS-210 | `anchor` | all six DOF | Nozzle B |
| RS-C | RS-230 | `anchor` | all six DOF | Nozzle C; seismic anchor movement point |
| RS-LS1 | RS-060 | `line_stop` | UX | Axial stop on the first run, so thermal growth of the run is directed into the riser |
| RS-V060 | RS-060 | `vertical_support` | UZ | Carries the valve run |
| RS-VS1 | RS-080 | `variable_spring_hanger` | UZ; stiffness 180 kN/m; installed and cold load 8.0 kN, hot load 7.4 kN, travel range 50 mm (all invented) | Riser, where vertical growth rules out a rigid support; sized by hanger sizing (§8.8) |
| RS-CE1 | RS-100 | `constant_effort_support` | UZ; `constant_load` 9.0 kN (invented input) | Near bend B2, where vertical movement is largest; sized by hanger sizing |
| RS-LO1 | RS-110 | `nonlinear` | `lift_off`, UZ, initial state active, contact on positive reaction | A rack support near the riser, where operating movement can lift the pipe; whether it lifts is a solve result, not claimed |
| RS-FR1V | RS-120 | `nonlinear` | `one_way`, UZ (resting); interim: a linear `vertical_support` UZ until R-4 stage S7 lands (§10) | Normal-force source for RS-FR1, so criterion 1 exercises a lift-off friction shoe |
| RS-FR1 | RS-120 | `nonlinear` | `friction`, UY (axial), μ 0.30 (invented), normal reaction from RS-FR1V UZ | Axial sliding along the rack |
| RS-G1 | RS-140 | `nonlinear` | `gap`, UX, initial state inactive, closes on positive displacement, gap 5 mm | A lateral stop with clearance beyond the tee |
| RS-V140 | RS-140 | `vertical_support` | UZ | Carries the header beyond the tee |
| RS-G2 | RS-170 | `guide` | UX, UZ | Guides the reduced run ahead of EJ1 |
| RS-V200 | RS-200 | `vertical_support` | UZ | Carries the run beyond EJ1 |
| RS-G3 | RS-215 | `guide` | UX, UY | Guides the branch drop |
| RS-SN1 | RS-110 | `snubber` (0.9.0) | UX, rigid | Dynamic-only lateral restraint on the rack, where no lateral restraint exists; static solves ignore it once DEL-04-10's slice lands (dynamics lane, 2026-10-31) |

The constant-effort force acts along the positive UZ axis, its single translational
restraint (DEC-049). A `variable_spring_hanger` must carry entered installed, cold and hot
loads and a travel range or movement limit (`SPRING_HANGER_LOAD_MISSING`,
`SPRING_HANGER_TRAVEL_MISSING`), so RS-VS1 carries invented ones. They are review metadata:
only the stiffness is consumed, and each entered load warns
`SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD`. Like the constant-effort load, they are inputs;
hanger sizing reports its own sized loads and travel as results (§8.8) and does not
overwrite them.

## 7. Pressure regions

Every terminal is `transfers_to_wall`. A shared terminal that is
`separately_supported_or_compensated` is refused (`EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED`),
and treating each nozzle as a capped closure keeps one rule throughout. Regions are
case-scoped: each pressurized case (§8) carries the same five regions with its own
pressure.

| Region | Members | Terminals |
|---|---|---|
| RG-1 | RS-P010 … RS-P120 (valve and flange members and bends B1, B2 included) | RS-010, RS-130 |
| RG-2 | RS-P210 … RS-P230 (bend B3 included) | RS-130, RS-230 |
| RG-3 | RS-P130, RS-P140 | RS-130, RS-150 |
| RG-R | RS-P150 (the reducer body) | RS-150, RS-160 |
| RG-4 | RS-P160, RS-P170, RS-P190, RS-P200; connector RS-EJ1 | RS-160, RS-210 |

This follows RT-A3's rules (RT-A3 manager, 2026-10-31): regions split at the tee node
with every terminal there `transfers_to_wall`, and the reducer body is its own region
joined to its neighbours at its end nodes.

## 8. Load cases, combinations and analyses

Operating pressure 1600 kPa; test pressure 2600 kPa; installation temperature 20 °C;
operating temperature 210 °C on every member except the replaced span RS-P180 (§5.1).
All are invented. Under 0.9.0 the
operating condition is one condition set applied to every member
(`condition_sets[]`, `pipe_segments[].condition_set_ref`, `load_cases[].condition_use`);
under 0.8.0 it is authored per case. The condition set carries temperature, pressure and
installation temperature only: the operating contents are the sections'
`contents_density` (§4.2), with no condition contents override, and no case carries
condition `added_weight`. Only RS-HYD uses `condition_use.contents_density` (§8.3).
Component weights are rigid-component `weight` at a centre of gravity, not added weight.

With nonlinear supports in the model, every static case is a nonlinear solve with its
own support status. Occasional effects are therefore cases that include the sustained
loads, and each occasional term is a difference from SUS. With a constant-effort
support, this is also what DEL-05-02 `Work/absolute-sum-srss-combinations.md` §5.1
requires: SRSS and absolute-sum operands must count the constant-effort force zero
times, so they are differences, not the plain occasional cases (load-cases manager,
2026-10-31).

### 8.1 Load cases

| Case | Content | Analysis (criterion 1) |
|---|---|---|
| RS-SUS | W + P (operating pressure) | Sustained |
| RS-OPE | W + P + T (operating temperature, modulus at 210 °C) | Operating (nonlinear) |
| RS-HYD | W with test contents + test pressure; `test_condition` `{kind: hydrostatic, held_supports: springs_and_constant_effort}` | Hydrotest |
| RS-WIN | W + P + wind: 0.9 kPa, shape factor 0.7, `global_x`, exposed members RS-P070 … RS-P200 except the replaced span RS-P180 | Wind |
| RS-EQX | W + P + seismic, `g_factor_x` 0.30 | Static seismic, X |
| RS-EQY | W + P + seismic, `g_factor_y` 0.30 | Static seismic, Y |
| RS-EQZ | W + P + seismic, `g_factor_z` 0.20 | Static seismic, Z |
| RS-SAM | W + P + anchor RS-C moved UX +15 mm, UY −10 mm | Seismic anchor movement |

W is generated self-weight of every member except RS-P180 (gravity as §2), with component weights at
their centres of gravity in place of their members' distributed weight (W-15). The fixture's W
is the product's self-weight generator's output for each case (`self_weight.rs`, through
`core/loads/self_weight_wasm`), captured in `fixtures/reference_system/self_weight_capture.json`;
the fixture generator copies it and computes no weight. Wind is
DEC-068's uniform pressure over the marked members (wind on a replaced span is
refused). On EX, the anchor movement is `analysis_state.support_states[].boundary_motion`
with meaning `absolute_reference_displacement`; the support-targeted primitive is
refused on EX.

### 8.2 Combinations

| Combination | Basis | Operands | Analysis |
|---|---|---|---|
| RS-EXP | `result_state_subtraction`, with `difference_basis` recorded (0.9.0) | RS-OPE − RS-SUS | Expansion with its basis |
| RS-dWIN | `result_state_subtraction` | RS-WIN − RS-SUS | Wind term |
| RS-dEQX, RS-dEQY, RS-dEQZ | `result_state_subtraction` | RS-EQx − RS-SUS | Seismic terms |
| RS-dSAM | `result_state_subtraction` | RS-SAM − RS-SUS | Anchor movement term |
| RS-EQ-SRSS | `srss` (0.9.0) | RS-dEQX, RS-dEQY, RS-dEQZ | Directional SRSS |
| RS-EQ-ABS | `absolute_sum` (0.9.0) | RS-dEQX, RS-dEQY, RS-dEQZ | Directional absolute sum |
| RS-OCC-SRSS | `mechanics` with combination operands (0.9.0) | RS-SUS + RS-EQ-SRSS | Sustained plus seismic |
| RS-OCC-ABS | `mechanics` with combination operands (0.9.0) | RS-SUS + RS-EQ-ABS | Sustained plus seismic |

SUS + SRSS over differences is admitted with nonlinear supports (Agent 0, OQ-1;
DEL-05-02 `absolute-sum-srss-combinations.md` §5.2). Sustained taking the operating
support status (`subtrahend_support_state`) is RT-A5's Should option; the fixture uses
its default until that slice lands.

### 8.3 Hydrotest

RS-HYD holds the spring and the constant-effort support (`held_supports:
springs_and_constant_effort`; the load-cases lane's D-2 test-condition design,
DEL-05-01 `Work/hydrostatic-test.md`). The test contents density (1000 kg/m³) is the
case's `condition_use.contents_density` (0.9.0 §1.6). No temperature applies.

### 8.4 Modal

Modal analysis is a request beside the model, not a model member. The request names
RS-OPE as its linearization state (the dynamics lane's M5: nonlinear supports
linearized in the status of a named solved case). Modes up to 60 Hz are extracted, with
at least the first ten reported.

### 8.5 Response spectrum

One invented spectrum, `spectra[]` RS-SP1 (0.9.0): abscissa frequency (Hz), ordinate
`g_multiple` with `g_value` 9.80665 m/s², damping ratio 0.05, log–log interpolation.
Points: (0.5, 0.12), (2.0, 0.75), (9.0, 0.75), (40.0, 0.30), (100.0, 0.30).

The spectrum request (DEL-04-10's, beside the model) applies RS-SP1 in X, Y and Z with
scale factors 1.0, 1.0 and 0.67, uses the modal request of §8.4, and turns on the
missing-mass correction. The mode and directional combination methods are the dynamics
lane's to define; the request names them when that design lands. One axis snubber,
RS-SN1 (§6), exercises RT-A8's dynamic-only restraint. It stays pending until DEL-04-10's
slice replaces the interim refusal: under 0.9.0, every static solve refuses a model with a
snubber (`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`).

### 8.6 Equilibrium checks need no extra case

Each check in §9.2 reads cases that already exist, so the model carries no auxiliary
weight-only or pressure-only case.

### 8.7 Seismic anchor movement and static seismic on one document

On EX, `boundary_motion` and the temperature state both need the load/reference-state
group, and that group refuses generated wind and seismic loads
(`EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED`). One document cannot hold RS-OPE,
RS-SAM and RS-WIN/RS-EQx together until #1678 lands (§10).

### 8.8 Hanger sizing

Hanger sizing is a request beside the model. It sizes RS-VS1 and RS-CE1 from an invented
private catalogue (`fixtures/reference_system/`, invented records only, OC-4) and
produces a hanger table. The procedure, its load cases and its record selection rule
belong to RT-A11's owner, not yet assigned. The geometry slots and the key results of
§9.1 are fixed here.

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
| EQ-1 | The sum of vertical reactions over all supports, the spring and the constant-effort support included, equals the total weight, in RS-SUS, RS-OPE and RS-HYD (with the test contents) | closed form: the total weight recomputed independently from the inputs (§4, §5) on EX's stated mass bases |
| EQ-2 | In RS-SUS, RS-OPE and RS-HYD the horizontal reaction resultant is zero, and the reaction moment resultant equals minus the moment of the weight. Every closure transfers to the wall, so each region, the tee junction and the pressurized joint are self-equilibrated under pressure, and temperature adds no load | closed form from the inputs |
| EQ-3 | On every published signed component row (support force and moment components, translation and rotation components, element end-force components), RS-EXP equals RS-OPE − RS-SUS. Magnitude rows (resultant displacement, reaction magnitude) are checked by recomputing them from the combined components, not by subtracting magnitudes. Rows the combination withholds, such as tie-rod forces, are excluded | exact identity |
| EQ-4 | On the same signed component rows, RS-EQ-SRSS equals √(Σ dEQ²) and RS-EQ-ABS equals Σ \|dEQ\| over the operand rows, so SRSS ≤ ABS on each. Magnitude rows follow the combination's own recomputation rule, as for EQ-3; withheld rows are excluded | exact identity |
| EQ-5 | The RS-dSAM reactions are self-equilibrated (resultant force and moment zero) | exact identity: no load differs between RS-SAM and RS-SUS |
| EQ-6 | The resultant of RS-dWIN reactions equals minus the generated wind resultant, and that of RS-dEQx equals minus the factor times g times the total mass on the generator's stated mass basis | closed form from the inputs |
| EQ-7 | Spectrum directional combination recomputed from the published per-direction results | exact identity |

Each closed-form reference is computed by an independent generator in
`validation/references/reference_system/`, not by the solver under test.

## 10. Pending-family ledger

State on `main` at `276e581b3`, as verified by the tranche-1 solve on EX
(`tests/test_reference_system_fixture.py`, 2026-10-30). Tranche 1 of the fixture contains
every family marked "in tranche 1", and RS-SUS, RS-OPE and RS-SAM solve on EX with no
blocking diagnostic. A pending family keeps its geometry slot and ids; when its row lands, the
generator adds its entities without moving a node. "Expected until then" is the refusal
or absence a solve of the full model shows today.

| Family or analysis | State | Row / lane | Expected until then |
|---|---|---|---|
| Straight members, sections, temperature-dependent material | in tranche 1 | — | — |
| Realized bends with user k and SIFs | in tranche 1 | — | — |
| Valve and flanges, weight at CoG, stiffness scale | in tranche 1 | — | — |
| Expansion joint with tie rods, pressurized | in tranche 1 | — | — |
| Pressure (five regions, shared terminals `transfers_to_wall`) | in tranche 1 | — | — |
| Anchors, guides, line stop, vertical supports | in tranche 1 | — | — |
| Variable spring as a user-stiffness spring | in tranche 1 | — | — |
| Tee RS-T1 with user SIFs | pending; the junction at RS-130 is in tranche 1 without the component | RT-A3 increment 1 (`claude/rt-a3-tee`) | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED` |
| Eccentric reducer RS-R1 in region RG-R | pending; RS-P150 is in tranche 1 as a plain S270 member in its own region | RT-A3 increment 2 | `EXACT_PRESSURE_REDUCER_IN_REGION` |
| Constant-effort support RS-CE1 | pending | RT-A3 increment 3 | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED` |
| Lift-off, friction and gap (RS-LO1, RS-FR1, RS-G1) | pending | RT-A3 increment 4; with the connector, increment 5 | `EXACT_PRESSURE_FAMILY_NOT_ADMITTED` |
| RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7 | interim: RS-FR1V authored as a linear `vertical_support` UZ (Agent 0, 2026-10-31) | RT-A3, R-4 stage S7, after increment 4 (DEL-04-07 `Work/route-unification-r4.md` §0a) | a nonlinear friction normal source is refused on both routes |
| RS-SUS, RS-OPE (thermal, modulus by temperature) | in tranche 1 as linear cases (load/reference-state group) | — | — |
| RS-EXP | in tranche 1 as a subtraction; `difference_basis` pending | RT-A5 slice 1b / load-cases lane (0.9.0) | basis not recorded |
| RS-WIN, RS-EQX, RS-EQY, RS-EQZ on the document that carries temperature | pending | #1678 / load-cases lane | `EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED` |
| RS-SAM (`boundary_motion` on rigid anchor RS-C) | in tranche 1, on the load/reference-state document beside the temperature state; it shares #1678's conflict with wind and seismic | — (plain EX documents: RT-A3) | — |
| RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS | pending | RT-A4 / load-cases lane (0.9.0 S1, then its slice) | not authorable at 0.8.0 |
| RS-HYD | pending | RT-A6 / load-cases lane (0.9.0, then its slice) | not authorable at 0.8.0 |
| Operating condition set | pending | RT-G5 / load-cases lane (0.9.0) | authored per case at 0.8.0 |
| Modal (RS-OPE linearization) | pending | RT-A7 / dynamics lane (DEL-04-09, M5 in scope) | no modal analysis |
| Response spectrum, RS-SP1 | pending | RT-A8 / dynamics lane (DEL-04-10, proposed) | no spectrum analysis; `spectra[]` not authorable at 0.8.0 |
| Hanger sizing and table | pending | RT-A11 / owner to be assigned by Agent 0 | no sizing procedure |
| Snubber RS-SN1 (RS-110, UX, rigid) | pending | RT-A8 / dynamics lane (DEL-04-10 slice: static solves ignore a snubber) | `SNUBBER_DYNAMIC_ONLY_UNSUPPORTED` on every static solve; not authorable at 0.8.0 |

The generator keeps a pending flag per row of this table, and the fixture test checks
that the two agree.

## 11. Consumer views

### 11.1 Criterion 6 (MBF)

The identity set is DEL-17-04 A-3's: nodes, pipes, components, supports, load cases,
primitive loads, materials, pressure regions and sections. Against the writer and
importer on `main` (DEL-17-04 §1.4), this system's families expected in the loss report
today are:

- the temperature-dependent modulus table (`MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED`):
  a gap for the MBF lane, which Agent 0 will staff. The family stays in the fixture;
  it is not dropped to make criterion 6 pass;
- the tee, the reducer, the valve and flanges, the expansion joint and its rods;
- the gap, lift-off and friction supports, the constant-effort support, and the
  variable spring as a hanger;
- more than one sustained-type and one thermal-type case, wind and seismic, hydrotest,
  every combination, and the modal, spectrum and hanger-sizing requests.

Each moves to "covered" as a coverage tranche lands; any family with no admitted public
MBF source goes to the criterion 3 limitations list, which the owner accepts.

### 11.2 Criterion 7 (authoring time)

**Reading, under the provisional value.** Criterion 7 says the system is "entered by
run sheet and solved in ≤ 30 minutes *(provisional)*". The run sheet (DEL-07-02
`Work/run-sheet-entry.md`) creates nodes, members, sections, materials, realized bends
and four rigid support codes (anchor, guide, line stop, vertical support). This
document reads criterion 7 as run-sheet rows plus the follow-up forms for everything
else, with small counts. That reading is Agent 0's working reading for now (2026-10-31);
the criterion text is the owner's, and Agent 0 takes the question to the owner with the
RT-U2 walkthrough results.

Counts for tranche 1 (`reference-system-run-sheet.md` §5): 26 run-sheet rows (three
chains: from A, from the tee, and the branch) and about 1,950 follow-up form entries,
about 1,480 of them the per-member load/reference states and pressure regions of the
three cases. The run sheet's
estimate is about 130 minutes, against the provisional 30; it is not a measurement. On
`main` the sheet cannot carry the fixture's support or bend ids, so all eleven supports
and the three bends are entered by forms (run sheet §2, §6). The full model adds the
tee, the reducer, the other non-rigid supports, five more cases with their regions, the
other eight combinations and the three analysis requests.

**What a fuller run sheet would absorb** (an RT-G2 candidate, Agent 0, 2026-10-31):
- a component column per row: flange, valve, reducer and tee, with weight and stiffness
  scale, as industry spreadsheet entry does;
- support codes for the spring, constant-effort, lift-off, friction and gap families,
  with their data in the row's detail;
- a pressure column per row that builds the regions, splitting them at tees and at the
  reducer automatically;
- a condition-set column (0.9.0 §4.1 already designs it) that removes per-case
  temperature and pressure entry;
- an expansion-joint row kind for a replaced span with rods.

### 11.3 Criterion 17 (embedded agent)

The agent builds the system through the catalog's operation route. The operation
families it needs, by the applier's existing vocabulary (`core/model_operations/operation_applier/`):

| Need | Applier family |
|---|---|
| Project units | `set_project_unit` |
| Material with temperature points | `create_material` (`rich_authoring.rs`, `load_state_authoring.rs`) |
| Sections | `create_section`, `assign_section` |
| Nodes and members | `create_node`, `connect_pipe_run` |
| Realized bends | `insert_bend_at_corner` (`bend_authoring.rs`) |
| Tee, reducer, valve, flanges | `insert_component_symbol`, `set_field` on component members |
| Expansion joint and rods; pressure contract and regions | `joint_pressure_authoring.rs`, `pressure_authoring.rs` |
| Supports, nonlinear members, hanger data | `create_support`, `update_support` (`rich_authoring.rs`, `boundary_association.rs`) |
| Self-weight and primitive loads | `create_load_case`, `create_primitive_load`, the self-weight generator |
| Wind and seismic | `equivalent_static` on the case |
| Temperature states and anchor movement | `load_state_authoring.rs` (`reference_configurations`, `analysis_state`) |
| Combinations | `create_combination`, `create_combination_term` |
| 0.9.0 members | condition sets, `test_condition`, `spectra[]`, combination operands (S1 adds their operations) |
| Solve and analysis requests | the catalog's solve and read tools; modal, spectrum and hanger-sizing requests when they land |

RT-E1's milestone 2 takes its admitted kinds from this table. The key results the agent
reports are §9.1's.

**Bend k is user entry.** The applier refuses a flexibility factor k without a user
entry and its source (`OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED`, `bend_authoring.rs`),
and the host refuses an agent-written k (DEL-16-06 `Work/embedded-agent-host.md`).
Catalog admission alone cannot let the agent realize B1–B3. RT-E1 needs a route in
which the person enters k and its source (here, §5's values) and the operation records
the person as that value's author, without relabelling agent authorship. This is an
RT-E1 dependency of criterion 17, not a change to the fixture.

## 12. Maintaining this document

- When a row lands, the same PR (or a follow-up in this deliverable) flips its pending
  flag in the generator, regenerates the fixture, updates §10, and adds any verified
  expected value with its basis.
- When 0.9.0 S1 lands, its ripple re-runs the generator with the version token bumped;
  authoring the 0.9.0 members themselves follows here.
- Geometry, ids and invented inputs change only by an edit to this document first, so
  that every consumer's target moves together.
- At release, every row still pending here corresponds to a LIM entry in
  `docs/RELEASE_LIMITATIONS_v1.0.md`, the single criterion 3 list (Agent 0, 2026-10-31).
  Each ledger update checks that correspondence.
