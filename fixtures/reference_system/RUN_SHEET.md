# Reference system run sheet

This is the script a person follows to enter the reference system in the desktop from a
blank project and solve it (release target exit criterion 7). It enters the fixture as
committed, [`reference_system.json`](reference_system.json) (model document 0.9.0), whose
system is stated in [`SPECIFICATION.md`](SPECIFICATION.md). Where the fixture and the
specification differ, the fixture is what this sheet enters; §5 lists the families the
specification holds and the fixture does not yet.

Every value is the fixture's own invented data. Every duration is an estimate, not a
measurement; the measured time comes from a person's timed entry (DEL-07-02, DEL-07-06). No
person is timed until the scripted journey follows this sheet's route step for step
(DEL-07-02 VER-007).
Following this script tests authoring effort; it states nothing about the system or the
product (F-PIP-2, F-PIP-3).

The route up to the solve is entered by script in
[`referenceEntryJourney.session.test.tsx`](../../apps/desktop/src/features/workspace/referenceEntryJourney.session.test.tsx),
which compares the entered model with the fixture record by record and allows only the
differences of §4. That test runs no solve.

## 1. Conventions

**Pasted texts.** Two texts recur. Paste them; do not retype them. A form field takes a paste
directly. A run-sheet grid cell does not: press **F2** to open the cell's editor, paste, and
press **Enter** to commit.

- **⟨P⟩** = `invented reference-system fixture input (DEL-09-01); not standard, supplier, commercial or project data`
- **⟨S5⟩** = `invented user value, DEL-09-01`

⟨P⟩ is the provenance of every record; ⟨S5⟩ is the source of every user value that needs
one. Both are exactly the fixture's.

**Where things are.** The rail has four stages, **Model**, **Loads**, **Results** and
**Review**, and opens pages, among them **Project** and **Analyze**, over the stage. The
Model stage has two tabs: **Model**, the model tree with the property inspector beside it,
and **Review changes**. The Loads stage has one tab, **Load Cases**; the Results stage has
**Results** and **Evidence**. A selection in the model tree needs the Model stage's Model
tab: a step that selects there after working on the Loads stage says so. The model tree
switches between **Tree** and **Grid**; in Grid, the **Run sheet** button shows the run
sheet. The inspector has the tabs **Properties** and **Task**. Properties shows **New
section**, **New component** and, with a node selected, **New support configuration**.

**Task edits.** To change one stored value: select the entity; inspector **Task** tab;
**Start task from current selection**; **Property to edit**; the value; **Add**; then Apply
under Review changes. Afterwards press **Cancel** in the task footer and then the
**Properties** tab. The inspector stays in its Task view otherwise, and selecting another
entity does not leave it, so the next step's creation form would stay hidden.

**Queue and apply.** Each form queues a change. A single queued change is applied with
**Apply** under Review changes. A batch (the run sheet's rows, a self-weight plan) is
applied under **Batch review** with **Validate batch** and then **Apply entire batch**, all
or nothing, as one Undo step; a batch that realizes a bend applies only after a clean
validation. Optionally, **Review all N queued changes as one batch** moves independent
queued changes into one batch; §6 counts the cycles without that.

**Steps.** Each step has a name and is self-contained: where it is done, what is entered,
and how it is applied. The order in §2 is the order to follow; a step names any step it
must follow. A family added to the fixture adds a step, or rows to a step's table, without
renumbering the others. Fields not mentioned are left as the form shows them.

## 2. Order of steps

| Step | Surface | Must follow |
|---|---|---|
| [units](#step-units) | Project page, units dialog | — |
| [profile](#step-profile) | Loads › Load Cases, Pressure mechanics profile | units |
| [material](#step-material) | Loads › Load Cases, Create explicit E/nu material; Model › Model, inspector | profile |
| [sections](#step-sections) | Model › Model, inspector, New section | units |
| [rows](#step-rows) | Model › Model, Grid, Run sheet | material, sections |
| [rs050](#step-rs050) | Model › Model, inspector, Task | rows |
| [joint](#step-joint) | Model › Model, inspector, New component | rows |
| [tee](#step-tee) | Model › Model, inspector, New component; Task | rows |
| [spring](#step-spring) | Model › Model, inspector, New support configuration | rows |
| [reference](#step-reference) | Loads › Load Cases, Reference configurations | rows |
| [weight](#step-weight) | Model › Review changes, Self weight | rs050, joint, tee, spring |
| [regions](#step-regions) | Loads › Load Cases, Pressure regions | weight |
| [states](#step-states) | Loads › Load Cases, Analysis state | reference, weight |
| [combinations](#step-combinations) | Loads › Load Cases, Create combination | weight |
| [solve](#step-solve) | Analyze page; Results › Results | every step above |

The bends are realized in step rows, before any support, load, region or reference names a
corner or a straight (`OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED`); the supports the sheet
creates name nodes, none of them a corner. The self-weight generator creates each case and
refuses an existing case id, and it reads the finished geometry, sections and components, so
step weight follows them.

## 3. Steps

### Step units

*Project* page, **Blank load/reference model**. In the units dialog keep **Preset** = SI and
press **Create**. SI gives the fixture's units: m, N, N\*m, rad, Pa, Pa, degC. The blank
already carries `pressure_contract` 3.0.0/exact_pressure_v3, with the Timoshenko shear
model, and an empty `reference_configurations`; step profile sets the shear model.

### Step profile

*Loads* stage, *Load Cases* tab, panel **Pressure mechanics profile**: **Pressure profile** =
exact_pressure_v3; **Shear model**: select Euler–Bernoulli (the blank shows Timoshenko).
**Queue pressure profile**, Apply. Done
before any member exists, the panel's corrosion-allowance condition holds.

### Step material

1. Same panel, **Create explicit E/nu material**: **Exact material ID** `material:RS-M1`;
   **Exact material label** `RS-M1 invented carbon-steel-like material`; **Constitutive
   basis** homogeneous_isotropic_E_nu_v1; **Elastic modulus E** 200 GPa; **Poisson ratio
   nu** 0.3, unit 1; **Thermal expansion coefficient** 1.2e-5 1/K; **Constitutive
   provenance** ⟨P⟩. **Queue exact material**, Apply.
2. Return to the Model stage's Model tab and select `material:RS-M1` in the model tree.
   Inspector, **Temperature-dependent properties**: **Add temperature point** four times;
   for each, ID, Temperature, Elastic modulus, Poisson ratio (unit 1), Thermal expansion
   coefficient and **Point provenance** ⟨P⟩; leave the shear modulus blank. **Queue temperature points**, Apply.

   | Point ID | Temperature | Elastic modulus | Poisson ratio | Thermal expansion coefficient |
   |---|---|---|---|---|
   | `point:RS-M1-T020` | 20 degC | 200 GPa | 0.3 | 1.2e-5 1/K |
   | `point:RS-M1-T100` | 100 degC | 195 GPa | 0.3 | 1.2e-5 1/K |
   | `point:RS-M1-T200` | 200 degC | 189 GPa | 0.3 | 1.2e-5 1/K |
   | `point:RS-M1-T300` | 300 degC | 182 GPa | 0.3 | 1.2e-5 1/K |

3. Same selection, **Expansion laws (current exact model)**: **Add expansion law**;
   definition engineering_secant; ID `law:RS-M1-SECANT`; datum temperature 20 degC; data
   constant; constant coefficient 1.2e-5 1/K; **Expansion law provenance** ⟨P⟩. **Queue
   expansion laws**, Apply.

### Step sections

On the Model stage's Model tab, select the project in the model tree. Inspector, **New
section**, three times: Section ID, Name, Length unit mm, Outside diameter, Wall thickness,
**Provenance** ⟨P⟩; queue, Apply. Type is already pipe.

| Section ID | Name | Outside diameter | Wall thickness |
|---|---|---|---|
| `section:RS-S270` | RS-S270 | 270 | 9.0 |
| `section:RS-S220` | RS-S220 | 220 | 8.0 |
| `section:RS-S170` | RS-S170 | 170 | 7.0 |

These hold the outside diameter and wall only; the other section quantities are entered per
member in step rows.

### Step rows

*Model* tab, model tree **Grid**, **Run sheet**.

**Header**, before the first row (the Numbering fields lock once a row has a number):

- **Source** ⟨P⟩. It becomes the provenance of every node, pipe, support and bend record the
  sheet creates.
- **Length unit** m (the default).
- **Numbering**: **Node prefix** `node:RS-`; **Pipe prefix** `pipe:RS-P`; **Start** 10 and
  **Step** 10 (the defaults); **Pad numbers to N digits** 3; **Pipe number from** its start
  node.
- Show the column groups **Section extras**, **Ids and sources** and **Component details**,
  each by its header button.
- Column units in the headers: Ins. thk mm; Ins. dens. kg/m^3; Contents dens. kg/m^3;
  Material density kg/m^3; Mill tolerance mm; Corrosion allowance mm. Bend R keeps the
  project unit, m.

**Rows.** The sheet starts with one row. Type the 26 rows in table order. After completing
each of rows 1 to 25, press **Enter** in its **Support** cell, empty or not: the row is then the
last, so this adds the next row and moves to its dX cell. Do not press it after row 26. Then **Queue rows**, **Validate batch** and **Apply entire
batch**. Blank increments are zero. Section, material and the section
quantities are entered once and inherit down the chain; the `F` row continues from RS-130
with row 14's values. The Condition set column stays blank: the fixture has no condition
set. **No.** is the sheet's id number, not the grid row; `F` and `S` rows take none. Ids are
written here without their `node:` or `pipe:` prefix; the cells take the full id. **(t)**
marks an id typed in the Node id or Pipe id cell; the others are derived from the
numbering.

| # | No. | dX | dY | dZ | Kind | Node | Section | Orientation | Other cells | Node id | Pipe id |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 10 | | | 1 | P | | | | Support A | RS-010 | |
| 2 | 20 | 0.4 | | | | | section:RS-S270 | 0 0 1 | Material `material:RS-M1`; Ins. thk 60, Ins. dens. 140, Contents dens. 820, Material density 7850, Mill tolerance 1.0, Corrosion allowance 1.5 | RS-020 | RS-P010 |
| 3 | 30 | 0.15 | | | L | | | | Flange F1, Component details | RS-030 | RS-P020 |
| 4 | 40 | 0.6 | | | V | | | | Valve V1, Component details | RS-040 | RS-P030 |
| 5 | 50 | 0.15 | | | L | | | | Flange F2, Component details | RS-050 | RS-P040 |
| 6 | 60 | 2.2 | | | | | | | Support LS1 | RS-060 | RS-P050 |
| 7 | — | | | | S | | | | Support V060 | | |
| 8 | 70 | 2.5 | | | B | | | | Bend B1 | RS-B1-CORNER (t) | RS-P060 |
| 9 | 80 | | | 3 | | | | 1 0 0 | | RS-080 | RS-P070 (t) |
| 10 | 90 | | | 3 | B | | | | Bend B2 | RS-B2-CORNER (t) | RS-P080 |
| 11 | 100 | | 3 | | | | | 0 0 1 | | RS-100 | RS-P090 (t) |
| 12 | 110 | | 4 | | | | | | | RS-110 | RS-P100 |
| 13 | 120 | | 4 | | | | | | Support FR1V | RS-120 | RS-P110 |
| 14 | 130 | | 4 | | | | | | | RS-130 | RS-P120 |
| 15 | 140 | | 3 | | | | | | Support V140 | RS-140 | RS-P130 |
| 16 | 150 | | 2 | | | | | | | RS-150 | RS-P140 |
| 17 | 160 | | 0.4 | -0.025 | | | | | | RS-160 | RS-P150 |
| 18 | 170 | | 2.6 | | | | section:RS-S220 | | Mill tolerance 0.9; Support G2 | RS-170 | RS-P160 |
| 19 | 180 | | 2 | | | | | | | RS-180 | RS-P170 |
| 20 | 190 | | 0.6 | | | | | | | RS-190 | RS-P180 |
| 21 | 200 | | 2.4 | | | | | | Support V200 | RS-200 | RS-P190 |
| 22 | 210 | | 2 | | | | | | Support B | RS-210 | RS-P200 |
| 23 | — | | | | F | node:RS-130 | | | | | |
| 24 | 220 | | | -2.5 | | | section:RS-S170 | 1 0 0 | Ins. thk 50; Mill tolerance 0.8; Support G3 | RS-215 (t) | RS-P210 (t) |
| 25 | 230 | | | -2.5 | B | | | | Bend B3 | RS-B3-CORNER (t) | RS-P220 (t) |
| 26 | 240 | 2 | | | | | | 0 0 1 | Support C | RS-230 (t) | RS-P230 (t) |

Why the ten ids are typed: the three bend corners are temporary nodes, named so that their
default ids do not coincide with a tangent node's or, for B3, with RS-230; the straights
after B1 and B2 start at unnumbered tangent nodes; RS-215 and RS-P210 leave the tee as a
second pipe from RS-130; RS-P220 follows; and the last row's own number is 240. Bend
realization removes the corners.

**Labels.** Type each node's label in the row's Label cell; the bend tangent nodes take
derived labels (§4).

| # | Label |
|---|---|
| 1 | `RS-010 Anchor A (nozzle A)` |
| 2 | `RS-020 Flange F1 start` |
| 3 | `RS-030 Valve V1 start` |
| 4 | `RS-040 Flange F2 start` |
| 5 | `RS-050 Flange F2 end` |
| 6 | `RS-060 Line stop LS1; vertical support V060` |
| 9 | `RS-080 Variable spring VS1 (riser)` |
| 11 | `RS-100 Constant-effort support CE1` |
| 12 | `RS-110 Lift-off support LO1` |
| 13 | `RS-120 Resting support with friction FR1` |
| 14 | `RS-130 Tee T1` |
| 15 | `RS-140 Gap guide G1; vertical support V140` |
| 16 | `RS-150 Reducer R1, large end` |
| 17 | `RS-160 Reducer R1, small end (eccentric, flat on bottom)` |
| 18 | `RS-170 Guide G2` |
| 19 | `RS-180 Expansion joint EJ1, face i` |
| 20 | `RS-190 Expansion joint EJ1, face j` |
| 21 | `RS-200 Vertical support V200` |
| 22 | `RS-210 Anchor B (nozzle B)` |
| 24 | `RS-215 Branch guide G3` |
| 26 | `RS-230 Anchor C (nozzle C; seismic anchor movement)` |

**Valve and flanges** (rows 3–5, Component details). Kind L is a flange and V a valve, on the
row's pipe at its end node. Enter for each: Component id and Component label; Body length;
End A size 270 and End B size 270, each with its size unit mm; Weight; COG X, Y, Z; End A
reference and End B reference; **Stiffness behavior reference**
`rigid body, user stiffness scale 10.0 (DEC-045); invented user value, DEL-09-01`; Stiffness
scale 10.0; **Stiffness scale source** ⟨S5⟩; Solver consumption mechanics_geometry_only;
**Component geometry source** ⟨S5⟩; **Component provenance** ⟨P⟩. Leave Body length unit, Weight unit and COG unit blank:
they show the project's m and N.

| # | Component id | Component label | Body length | Weight | COG X, Y, Z | End A reference | End B reference |
|---|---|---|---|---|---|---|---|
| 3 | `component:RS-F1` | Flange F1 | 0.15 | 350 | −0.075, 0, 0 | `node:RS-020 (body end on RS-P020)` | `node:RS-030 (body end on RS-P020)` |
| 4 | `component:RS-V1` | Valve V1 | 0.6 | 2600 | −0.3, 0, 0.3 | `node:RS-030 (body end on RS-P030)` | `node:RS-040 (body end on RS-P030)` |
| 5 | `component:RS-F2` | Flange F2 | 0.15 | 350 | −0.075, 0, 0 | `node:RS-040 (body end on RS-P040)` | `node:RS-050 (body end on RS-P040)` |

**Supports** (Support and Restraints cells; Support id and Support label under Ids and
sources). Restraints `*` is all six.

| # | Support | Restraints | Support id | Support label |
|---|---|---|---|---|
| 1 | A | * | `support:RS-A` | Anchor A (nozzle A) |
| 6 | L | UX | `support:RS-LS1` | Line stop LS1 |
| 7 | V | UZ | `support:RS-V060` | Vertical support V060 |
| 13 | V | UZ | `support:RS-FR1V` | RS-FR1V interim linear vertical support (one_way target pending R-4 S7) |
| 15 | V | UZ | `support:RS-V140` | Vertical support V140 |
| 18 | G | UX UZ | `support:RS-G2` | Guide G2 |
| 21 | V | UZ | `support:RS-V200` | Vertical support V200 |
| 22 | A | * | `support:RS-B` | Anchor B (nozzle B) |
| 24 | G | UX UY | `support:RS-G3` | Branch guide G3 |
| 26 | A | * | `support:RS-C` | Anchor C (nozzle C) |

The eleventh support, the variable spring RS-VS1, has hanger data that no support code
carries; step spring enters it.

**Bends** (Bend R, k, k source and the two SIF cells; the ids and Bend geometry source under
Ids and sources). Bend R inherits within a chain; k and the SIFs never inherit. The SIFs'
source is the row's k source, the one source the bend keeps for k and both SIFs.

| # | Corner | Bend R | k | k source | In-plane SIF i_ip | Out-of-plane SIF i_op | Bend start id | Bend end id | Bend arc id | Bend component id | Bend geometry source |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 8 | B1 (6, 0, 1) | 0.4 | 4.2 | ⟨S5⟩ | 2.1 | 1.8 | `node:RS-070` | `node:RS-075` | `pipe:RS-P065` | `component:RS-B1` | ⟨S5⟩ |
| 10 | B2 (6, 0, 7) | (inherits 0.4) | 4.2 | ⟨S5⟩ | 2.1 | 1.8 | `node:RS-090` | `node:RS-095` | `pipe:RS-P085` | `component:RS-B2` | ⟨S5⟩ |
| 25 | B3 (6, 15, 2) | 0.25 | 3.6 | ⟨S5⟩ | 2.3 | 1.9 | `node:RS-220` | `node:RS-225` | `pipe:RS-P225` | `component:RS-B3` | ⟨S5⟩ |

A `B` row needs the next row to be a run with the same section and material, which rows 9,
11 and 26 inherit. For these 90° corners the engine places the tangent points at R from the
corner, exactly the fixture's nodes, and gives each arc the incoming pipe's section and
direction.

### Step rs050

The sheet adds increments in binary64, so RS-050 lands at 1.2999999999999998 m, one unit in
the last place short of the fixture's 1.3; every other node lands exactly. A Task edit
(§1): select RS-050 in the model tree; **Task**; **Start task from current selection**;
**Property to edit** X position; value 1.3; **Add**; Apply. Then **Cancel** and
**Properties**, so that step joint's **New component** shows.

### Step joint

Select the project. Inspector, **New component**: Kind expansion_joint; **New component node** RS-190;
**New component realized pipe** (the replaced span) RS-P180; ID `component:RS-EJ1`; label `Expansion joint EJ1 (tied)`.
Then:

- Connector axes (global) `0 -1 0; 1 0 0; 0 0 1`; End i node axes (global) and End j node
  axes (global) `1 0 0; 0 1 0; 0 0 1`; End i offset (local) and End j offset (local)
  `0 0 0`, Offset unit m.
- Connector reference state stress_free; Reference translation q_ref `0 0 0` m; Reference
  rotation q_ref `0 0 0` rad; Installed reference temperature 20 degC.
- Translation scale 1 m; Work matrix upper triangle (N\*m)
  `150000 0 0 0 0 0 400000 0 0 0 0 400000 0 0 0 500000 0 0 1500 0 1500`; Installed geometry
  (calibration) `authored model geometry`.
- Connector hardware explicit_axial_rods_v1; **Add tie rod** twice: Rod id
  `rod:RS-EJ1-RYP` with both offsets `0 0.2 0`, and `rod:RS-EJ1-RYN` with both offsets
  `0 -0.2 0`; for each, Offset unit m, Rod stiffness 1.2e8 N/m, Reference extension 0 m,
  **Rod source reference** ⟨S5⟩.
- Connector pressure model constant_effective_area_reference_v1; Joint effective area
  0.049 m^2; Connector joint pressure basis internal_differential_zero_external_v1; **Joint
  pressure area source reference** ⟨S5⟩.
- **Stiffness source reference** ⟨S5⟩; **Measurement restraints** `invented; no measurement`;
  **Basis transform reference** `none`; **Validity statement** `invented test object; no
  applicability claim`; **New component provenance** ⟨P⟩.

Queue, Apply.

### Step tee

1. Inspector, **New component**: Kind tee; **New component node** RS-130; **New tee header pipe** RS-P120;
   **New tee branch pipe** RS-P210; ID `component:RS-T1`; label `Tee T1`; Connection angle 90 with its
   angle unit deg; length unit mm; Run size 170; Header size 270; Connection type
   `invented unreinforced tee`; Reinforcement reference `invented: no reinforcement`;
   **New component geometry source** ⟨S5⟩; **New component provenance** ⟨P⟩. Queue, Apply.
   The form needs the four descriptive members before it queues; EX reads none of them.
2. Three Task edits (§1) of `component:RS-T1`, each its own Apply and each followed by
   **Cancel** and **Properties**: **Property to edit** Header SIF user value, 2.4; then
   Branch SIF user value, 2.0; then **Modifier source** ⟨S5⟩. The last **Cancel** and
   **Properties** let step spring's form show.

### Step spring

Select node RS-080. Inspector, **New support configuration** (the **Support
configuration** form, **Create support**): **Support ID** `support:RS-VS1`; **Support
label** `Variable spring VS1 (user stiffness)`; **Support node** shows RS-080; **Support
family** variable_spring_hanger; tick no restraint; **Support provenance** ⟨P⟩. Tick
**Include hanger data**: **Hanger type** variable_spring_hanger; **Add hanger stiffness**,
**Hanger stiffness DOF** UZ, quantity 180000 N/m; Hanger installed load 8000 N, cold load
8000 N, hot load 7400 N, travel range 50 mm; **Hanger source reference** ⟨S5⟩; **Hanger
mechanics consumption** linear_spring_primitive_user_stiffness. **Queue support creation**,
Apply.

### Step reference

*Loads* stage, *Load Cases* tab, **Reference configurations**: **Add reference configuration**; ID
`reference:RS-INSTALLED`; label `Installed reference (20 degC)`; geometry
authored_model_geometry. **Add member to reference configuration 1**: pipe RS-P010, basis
temperature_reference, installation temperature 20 degC, fit none, **Member provenance** ⟨P⟩. Under
**Fill reference configuration 1 members from a template**, template the RS-P010 row, targets
every pipe without one, **Fill reference configuration 1 members**: the other 25 members, RS-P180 included.
**Configuration provenance** ⟨P⟩. **Queue reference configurations**, Apply.

### Step weight

*Model* stage, *Review changes* tab, tool tab **Self weight**, **Plan self-weight loads**, once per case,
with **Self-weight action** Create a self-weight case, as it starts: **Self-weight case ID**;
**Self-weight case label**; **Self-weight provenance**. For the first case also enter
**Gravity value** -9.80665, **Gravity unit** m/s^2 and **Gravity direction** GLOBAL Z, and
choose the 25 **Self-weight pipes**, every pipe except RS-P180; the panel keeps the gravity
and the pipes for the next two cases. **Generate
self-weight plan**, **Queue complete self-weight plan**, then Validate batch and Apply
entire batch.

| Self-weight case ID | Self-weight case label | Self-weight provenance |
|---|---|---|
| `load:RS-SUS` | `RS-SUS sustained: W + P (operating pressure)` | ⟨P⟩`; W of RS-SUS: self-weight of every member except RS-P180, component weights at their centres of gravity (spec §8.1)` |
| `load:RS-OPE` | `RS-OPE operating: W + P + T (210 degC, modulus at 210 degC)` | ⟨P⟩`; W of RS-OPE: self-weight of every member except RS-P180, component weights at their centres of gravity (spec §8.1)` |
| `load:RS-SAM` | `RS-SAM seismic anchor movement: W + P + anchor RS-C moved UX +15 mm, UY -10 mm` | ⟨P⟩`; W of RS-SAM: self-weight of every member except RS-P180, component weights at their centres of gravity (spec §8.1)` |

Each case gets the fixture's 28 primitives: 22 line loads, and a force and a moment for each
of RS-F1, RS-V1 and RS-F2.

### Step regions

*Loads* stage, *Load Cases* tab: select RS-SUS in **Load cases**; panel **Pressure regions — RS-SUS …**. For
each region: **Add pressure region**; Region ID; member pipes (and, for RG-4, member
connectors); pressure basis internal_differential_zero_external_v1; pressure 1600 kPa; two
ordered terminals, each with node, closure transfer transfers_to_wall and **Terminal
provenance** ⟨P⟩; **Region provenance** ⟨P⟩. **Queue pressure regions**, Apply.

| Region ID | Member pipes | Member connectors | Terminal 1, terminal 2 |
|---|---|---|---|
| `region:RS-SUS-RG-1` | RS-P010, RS-P020, RS-P030, RS-P040, RS-P050, RS-P060, RS-P065, RS-P070, RS-P080, RS-P085, RS-P090, RS-P100, RS-P110, RS-P120 | | RS-010, RS-130 |
| `region:RS-SUS-RG-2` | RS-P210, RS-P220, RS-P225, RS-P230 | | RS-130, RS-230 |
| `region:RS-SUS-RG-3` | RS-P130, RS-P140 | | RS-130, RS-150 |
| `region:RS-SUS-RG-R` | RS-P150 | | RS-150, RS-160 |
| `region:RS-SUS-RG-4` | RS-P160, RS-P170, RS-P190, RS-P200 | `component:RS-EJ1` | RS-160, RS-210 |

Then select RS-OPE: **Copy pressure regions from case** RS-SUS, **Replace this draft's
pressure regions with the copy**, which re-derives the ids (`region:RS-OPE-RG-1`,
`region:RS-OPE-RG-2`, `region:RS-OPE-RG-3`, `region:RS-OPE-RG-R`, `region:RS-OPE-RG-4`);
**Queue pressure regions**, Apply. The same for RS-SAM (`region:RS-SAM-RG-1`,
`region:RS-SAM-RG-2`, `region:RS-SAM-RG-3`, `region:RS-SAM-RG-R`, `region:RS-SAM-RG-4`).

### Step states

*Loads* stage, *Load Cases* tab, select the case; panel **Analysis state — ⟨case⟩**.
Contract `openpipestress.load_reference_state/1.0.0`; reference configuration
`reference:RS-INSTALLED`; history independent_equilibrium; **Analysis state provenance** ⟨P⟩.
Each list is entered as one typed row and a fill: **Add element state** creates element
state 1 and **Add support state** creates support state 1; after typing the row, **Fill
element states from a template** or **Fill support states from a template**, with the typed
row as template and targets every pipe or support without one. Then **Factor for every
primitive (dimensionless)** 1 and **Add every primitive of this case**. **Queue analysis
state**, Apply, per case.

- **RS-SUS.** **Add element state**. Element state 1: pipe RS-P010; material selection exact_point; material
  `material:RS-M1`; material point `point:RS-M1-T020`; thermal state unchanged_reference;
  **Thermal provenance** ⟨P⟩`; no temperature in this case`. Fill. Then edit RS-P180's
  **Thermal provenance** to ⟨P⟩`; replaced span RS-P180: zero eigenstrain (spec §5.1)`.
  **Add support state**. Support state 1: support RS-A, participation active_model_device;
  fill. Add every primitive.
- **RS-OPE.** The scripted journey
  ([`referenceEntryJourney.session.test.tsx`](../../apps/desktop/src/features/workspace/referenceEntryJourney.session.test.tsx))
  does not yet take this route, and it still re-enters values the forms keep.
  **Copy analysis state from case** RS-SUS, **Replace this draft's analysis state
  with the copy**: the copy keeps the contract, reference configuration, support states,
  history and provenance, which are RS-OPE's too, and leaves out RS-SUS's load sources.
  **Mark element_states absent**, then **Add element state**. Element state 1: pipe RS-P010; operating temperature 210 degC; material
  selection temperature_interpolation; material `material:RS-M1`; selection temperature
  210 degC; interpolation piecewise_linear; extrapolation forbidden; thermal state
  free_length_state; expansion law `law:RS-M1-SECANT`. Fill. Then on RS-P180's row clear the
  operating temperature (value and unit), set thermal state unchanged_reference and **Thermal
  provenance** ⟨P⟩`; replaced span RS-P180: zero eigenstrain (spec §5.1)`; its material
  selection stays as filled. Factor 1, **Add every primitive of this case**.
- **RS-SAM.** **Copy analysis state from case** RS-SUS, **Replace this draft's analysis state
  with the copy**: the copied RS-SUS load sources name primitives RS-SAM does not hold, and
  the panel says so. Factor 1, **Add every primitive of this case**. On RS-C's support state,
  **Add … boundary motion** twice: UX 15 mm and UY -10 mm, each with meaning
  absolute_reference_displacement.

### Step combinations

*Loads* stage, *Load Cases* tab, **Create combination**, twice. For the first: ID; Label; Basis result_state_subtraction;
leave the **Operating minus sustained difference** template unticked (its
`difference_basis` is refused on a document with a pressure contract,
`LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`); **Subtraction minuend** and
**Subtraction subtrahend**; **Provenance** ⟨P⟩. Leave Rationale as the form shows it; the
record does not keep it. **Queue combo**, Apply. For the second, the form keeps the basis, the
subtrahend, the provenance and the rationale: enter ID, Label and **Subtraction minuend**.
**Queue combo**, Apply.

| ID | Label | Minuend | Subtrahend |
|---|---|---|---|
| `combination:RS-EXP` | `RS-EXP expansion: RS-OPE - RS-SUS` | `load:RS-OPE` | `load:RS-SUS` |
| `combination:RS-dSAM` | `RS-dSAM anchor movement term: RS-SAM - RS-SUS` | `load:RS-SAM` | `load:RS-SUS` |

### Step solve

*Analyze* page, **Run mechanics preview**. The timed entry's clock stops when this first
solve of every static load case and combination the sheet enters completes with no blocking
diagnostic (DEL-07-02 REQ-019). On the *Results* stage, *Results* tab: reactions at RS-A, RS-B and RS-C
and the spring load (KR-01, KR-03) in **Support Load Summary**; the largest translation
(KR-02) in **Displacement Extremes**; and the **Key results** panel's **Anchors and equipment
connections**. The fixture's three cases solve on EX in the headless runner with nothing
refused ([`tests/test_reference_system_fixture.py`](../../tests/test_reference_system_fixture.py));
no maintained test solves the model entered through the desktop.

## 4. Differences from the fixture

The steps give the fixture's identities, numbers, units, provenance and sources. These
differences remain; none is in criterion 6's identity set or a numerical value:

- **Project.** The project's id, name and description are the blank's
  (`project:blank-local-…`, `Blank Local Load/reference Model …`), not `project:RS` and the
  fixture's name and description.
- **Blank document members.** The entered document keeps the blank's `analysis_status`
  (`MODEL_INCOMPLETE`, `RULE_INPUTS_INCOMPLETE`, `NOT_PROVIDED`; the fixture's is
  `ready_for_preview_diagnostics`, `not_performed_user_rule_inputs_missing`,
  `not_provided`), its `data_boundary` (absent from the fixture) and its `diagnostics`, the
  one `BLANK_PROJECT_AUTHORING_TARGET` entry naming the project (absent from the fixture).
- **Derived pipe labels.** The sheet labels each straight pipe with its id's token
  (`RS-P020`); the fixture adds what the member carries, on RS-P020, RS-P030, RS-P040,
  RS-P120, RS-P130, RS-P150, RS-P170, RS-P180, RS-P190 and RS-P210 (for example
  `RS-P020 (Flange F1)`).
- **Derived bend labels.** Bend realization labels the tangent nodes `Bend start` and
  `Bend end`, each arc `Bend arc` and each bend component `Bend`; the fixture's are, for
  example, `RS-070 Bend B1 near tangent`, `RS-P065 (Bend B1 (arc))` and `Bend B1`. No cell
  sets them.
- **Joint stiffness measurement restraints.** The form writes RS-EJ1's stiffness provenance
  from the connector's own references, so its measurement restraints read
  `invented; no measurement`; the fixture's read `invented diagonal work matrix; no measurement`.
- **Joint stiffness basis transform reference.** For the same reason its basis transform
  reference reads `none`; the fixture's reads `none (authored directly in the connector basis)`.
- **Combination terms.** Each subtraction combination carries `terms: []`, which the fixture
  omits; an empty list means no terms.
- **Self-weight hash.** Each self-weight primitive's provenance records the hash of the model
  it was generated from, which is not the fixture generator's capture model.
- **Order.** Records are created in step order, so collections and per-member lists are in a
  different order from the fixture's; records are compared by id.

## 5. Pending families

When a family of the specification's §10 ledger enters the fixture, this sheet gains the
step or rows that enter it, at the place below, and §6 is recounted. The run-sheet rows of
step rows do not change, except row 13's support when RS-FR1V becomes one-way.

| Ledger family | Where it enters |
|---|---|
| Eccentric reducer RS-R1 | A step after step rows and before step weight: reducer on RS-P150 at RS-160, offset (0, 0, +25) mm, 300 N at the body midpoint, stiffness scale 2.0 |
| Constant-effort support RS-CE1 | A step beside step spring: RS-100, UZ, constant load 9.0 kN |
| Lift-off, friction and gap | A step beside step spring: RS-LO1 (RS-110, UZ), RS-FR1 (RS-120, UY, μ 0.30, normal from RS-FR1V UZ), RS-G1 (RS-140, UX, 5 mm) |
| RS-FR1V one-way | RS-FR1V leaves row 13 for the support form, as nonlinear one_way UZ |
| `difference_basis` on RS-EXP | Step combinations: the basis |
| Wind and static seismic | Cases RS-WIN, RS-EQX, RS-EQY, RS-EQZ and their difference combinations |
| SRSS and absolute sum | Step combinations: RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS |
| Hydrotest RS-HYD | A case with test contents, test pressure and held supports |
| Operating condition set | A condition set, assigned by the Condition set column of step rows; it replaces the per-member temperatures of step states |
| Modal | A modal request on RS-OPE to 60 Hz, after step solve |
| Response spectrum RS-SP1 | The spectrum and its request, after step solve |
| Hanger sizing | A sizing request for RS-VS1 and RS-CE1, after step solve |
| Snubber RS-SN1 | A step beside step spring: RS-110, UX, rigid |

## 6. Counts and estimate

An **entry** is one typed or pasted value, one choice from a list or picker, or one checkbox
or toggle; a unit is an entry where the person sets it. A **press** is one press of a button
or tab the steps name other than queue, validate and apply, and each key the steps name for
a purpose other than committing an entry: Add …, Fill …, Replace …, Mark …, Generate …, Enter
in a Support cell to add a row, F2 to open a grid cell's editor, Start task, Cancel, the inspector's Task and Properties tabs, Blank
load/reference model, Create and Run mechanics preview. A **cycle** is one Apply or one Apply
entire batch. Moving between stages and pages is not counted.

The estimate rates every grid cell at about 4.9 s (8 minutes per 98 cells) and every form
entry and every press at about 5.5 s, the rates of the earlier estimate of this sheet (a
practised person pasting ⟨P⟩ and ⟨S5⟩). Each step's time is rounded to the nearest quarter
minute, halves up. Step solve's press is entry; the solve itself is given a waiting allowance
of 2 minutes, for the solve to run and its key results to show, which is waiting, not entry.
The estimate does not count moving between stages and pages, the 27 cycles or waiting for a
batch's validation. It is not a measurement. A timed entry records the entry time and the
solve waiting time separately; their sum is compared with the threshold.

| Step | Entries | Presses | Cycles | Arithmetic | Estimate (min) |
|---|---|---|---|---|---|
| units | 0 | 2 | 0 | 2 × 5.5 s = 11 s | 0.25 |
| profile | 2 | 0 | 1 | 2 × 5.5 s = 11 s | 0.25 |
| material | 59 | 5 | 3 | 64 × 5.5 s = 352 s | 5.75 |
| sections | 19 | 0 | 3 | 19 × 5.5 s = 104.5 s | 1.75 |
| rows | 224 | 40 | 1 | 224 × 8/98 min = 18.29 min; 40 × 5.5 s = 220 s | 22 |
| rs050 | 3 | 4 | 1 | 7 × 5.5 s = 38.5 s | 0.75 |
| joint | 52 | 2 | 1 | 54 × 5.5 s = 297 s | 5 |
| tee | 24 | 12 | 4 | 36 × 5.5 s = 198 s | 3.25 |
| spring | 20 | 1 | 1 | 21 × 5.5 s = 115.5 s | 2 |
| reference | 12 | 3 | 1 | 15 × 5.5 s = 82.5 s | 1.5 |
| weight | 37 | 3 | 3 | 40 × 5.5 s = 220 s | 3.75 |
| regions | 86 | 7 | 3 | 93 × 5.5 s = 511.5 s | 8.5 |
| states | 50 | 14 | 3 | 64 × 5.5 s = 352 s | 5.75 |
| combinations | 9 | 0 | 2 | 9 × 5.5 s = 49.5 s | 0.75 |
| solve | 0 | 1 | 0 | 1 × 5.5 s = 5.5 s entry; 2 min solve waiting | 0 entry + 2 waiting |
| **Total** | **597** | **94** | **27** | | **about 61.25 entry + 2 waiting = about 63.25** |

Two subtotals, both estimates: **entry** about 61.25 minutes (the grid cells, form entries and
presses, each step rounded), and **solve waiting** 2 minutes (an allowance, not entry). Their
sum is about 63.25 minutes.

Unrounded, by direct arithmetic, the entry is 224 × 8/98 min + (373 form entries + 94
presses) × 5.5/60 min = 18.286 + 42.808 = 61.09 minutes, and with the 2 minutes of solve
waiting the sum is 63.09 minutes.

How the larger counts arise:

- **material, 59 entries and 5 presses.** The E/nu form's 10 (ID, label, basis, three
  quantities with their units, provenance); the material's selection; four temperature points
  of 10 each (ID, four quantities with their units, provenance); the expansion law's 8. The
  presses are Add temperature point four times and Add expansion law.
- **sections, 19.** The project's selection and 6 for each of three sections (Type is
  preset to pipe).
- **rows, 224 entries and 40 presses.** The header's 14: Source, the two prefixes, the padding, Pipe number from,
  three group toggles and six column units. The rows' 210 cells: 57 Component details cells
  of the valve and flanges (19 each) with their 3 increments and 3 Kind codes; 39 bend
  cells (13, 12 and 14 on rows 8, 10 and 25, including their increments, codes and corner
  ids); 40 support cells (four for each of ten supports) and the `S` row's Kind; 21 node
  labels; and 46 others (increments, section and material cells, section quantities,
  orientations, the `F` row's two cells and the other typed ids). The 40 presses are Enter
  in the Support cell of rows 1 to 25, which adds rows 2 to 26, and F2 before each of the 15
  pasted grid cells (k source and Bend geometry source on three bend rows; Stiffness scale
  source, Component geometry source and Component provenance on three component rows).
- **rs050 and tee.** Each Task edit is 3 entries (selection, property, value) and 4 presses
  (Task, Start task, Cancel, Properties): one edit for rs050 and three for the tee, whose
  creation form adds 15 entries.
- **joint, 52 entries and 2 presses.** The project's selection, kind, node and pipe, ID and
  label; 18 connector, reference and stiffness fields; 9 for each of two tie rods; 10
  pressure and provenance fields. The presses are Add tie rod twice.
- **weight, 37 entries and 3 presses.** Case ID, label and provenance per case (9); the
  gravity value, unit and direction once (3); the 25 pipes, chosen once. Generate self-weight
  plan per case.
- **regions, 86 entries and 7 presses.** RS-SUS's selection; 11 per region (ID, basis,
  pressure value and unit, six terminal fields, provenance) for five regions; 26 picker
  choices (25 member pipes and the connector); two entries for each copy. The presses are Add
  pressure region five times and Replace once per copy.
- **states, 50 entries and 14 presses.** RS-SUS 19: its selection, a typed element row of 6
  fields, the element fill's two choices, one RS-P180 edit and 9 shared entries (contract,
  reference, the typed support state's two fields, the support fill's two choices, factor,
  history, provenance). RS-OPE 20: its selection, the copy's source, 11 element fields, two
  fill choices, four RS-P180 edits and the factor. RS-SAM 11: its selection, the copy's
  source, the factor and two motions of four fields. The presses: for RS-SUS, Add element
  state, Fill element states, Add support state, Fill support states and Add every primitive;
  for RS-OPE, Replace, Mark element_states absent, Add element state, Fill element states and
  Add every primitive; for RS-SAM, Replace, Add every primitive and Add boundary motion twice.
- **combinations, 9.** The first: ID, label, basis, minuend, subtrahend, provenance; the
  second: ID, label, minuend.

The estimate of about 63.25 minutes (61.25 entry and 2 solve waiting) is about twice the
provisional 30. The four largest steps are rows (22), regions (8.5), material (5.75) and
states (5.75).
