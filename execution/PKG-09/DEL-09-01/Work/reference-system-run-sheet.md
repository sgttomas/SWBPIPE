# Reference system run sheet (exit criterion 7)

**Basis:** `main` at `66ecc9f17`, and at `5bb2f9ff0` for the tee (§3.6 item 3); fixture `fixtures/reference_system/reference_system.json`
(tranche 1, model document 0.8.0) and its specification `reference-system.md` beside this
document. Release target `docs/RELEASE_TARGET_v1.0.md` §4, criterion 7. Desktop paths are
relative to `apps/desktop/src/`.

## 1. Purpose, reading and boundaries

This is the script a person follows to enter the reference system from blank and solve it.
Criterion 7 says the system is "entered by run sheet and solved in ≤ 30 minutes
*(provisional)*". The reading used here is the specification's §11.2: run-sheet rows plus
follow-up forms for everything else, with small counts. The criterion text is the owner's.

Every value is the fixture's own invented data; where the fixture and the specification
differ, the fixture wins. Every duration here is an estimate per step, not a measurement.
The measured time comes from the RT-U2 walkthrough (DEL-07-06), which Agent 0 takes to the
owner with the §11.2 reading. Following this script is a test of authoring effort, not a
statement about the system or the product (F-PIP-2, F-PIP-3).

**Text used throughout.** Two strings recur and are pasted, not retyped:

- **⟨P⟩** = `invented reference-system fixture input (DEL-09-01 Work/reference-system.md); not standard, supplier, commercial or project data`
- **⟨S5⟩** = `invented user value, DEL-09-01 Work/reference-system.md §5`

**Queue and apply.** Every form below queues an operation. A single intent is applied with
**Apply** under *Review changes* (`features/operations/OperationApplyPanel.tsx:105-114`); a
batch with **Validate batch**, then **Apply entire batch** (`features/toolkit/BatchReviewPanel.tsx:122-123`).
A batch holding `insert_bend_at_corner` applies only after a clean validation of that
batch (`BatchReviewPanel.tsx:24-32`).

## 2. Preconditions

1. *Project* page: **Blank load/reference model** (`App.tsx:1043-1046`). In the dialog
   "New blank load/reference model: units", keep **Preset** = SI and press **Create**
   (`features/project-units/ProjectUnitsDialog.tsx:137-151, 205-212`). SI gives the
   fixture's units exactly: m, N, N\*m, Pa, Pa, degC, rad (`services/projectService.ts:427-429`).
   This blank already carries `pressure_contract` 3.0.0/exact_pressure_v3 and an empty
   `reference_configurations`, but with the Timoshenko shear model
   (`projectService.ts:494-509`); §3.1 changes it.
2. The run sheet is the Model tree's **Grid** mode (`features/model-tree/ModelTree.tsx:249-267`),
   **Grid family** = Run sheet (`ModelTree.tsx:1102-1108`). Its header
   (`features/model-tree/runSheet/RunSheet.tsx:514-526`) is set before the first row:
   - **Source** = ⟨P⟩ (it becomes the `provenance` of every node and pipe the sheet creates);
   - **Length unit** = m (the project unit; the default);
   - **Numbering**: **Node prefix** `node:RS-0`, **Pipe prefix** `pipe:RS-P0`, **Start** 10,
     **Step** 10;
   - **Section extras** shown, and the column units chosen in the headers: Ins. thk mm,
     Ins. dens. kg/m^3, Contents dens. kg/m^3, Material density kg/m^3, Mill tolerance mm,
     Corrosion allowance mm.

**How the sheet's ids differ from the fixture's.** The sheet's ids are prefix plus a
number (`runSheetModel.ts:244-252`, DEL-07-02 `Work/run-sheet-entry.md` §3.2), so it cannot
make the fixture's ids by numbering alone:

- numbers are not zero-padded, so `node:RS-0` gives RS-010 … RS-090 and then RS-0100;
- a pipe takes its row's number, its **end** node's, while the fixture names a pipe by its
  **start** node (RS-P010 runs 010 → 020);
- a support takes `support:⟨node token⟩` (`runSheetModel.ts:248, 684`), and the grid has no
  support-id cell: only **Node id** and **Pipe id** are editable (`runSheetView.ts:56-57,
  455-459`). Two supports at RS-060 would both be `support:RS-060`, which the sheet refuses
  (`runSheetModel.ts:755-768`);
- a bend row's tangent nodes, arc and component take `node:⟨corner⟩-T1`, `-T2`,
  `pipe:⟨corner⟩-bend` and `component:⟨corner⟩-bend`, with no cell to change them
  (`runSheetModel.ts:249-252, 738-740`), and the sheet's Source as the bend's geometry source
  (`runSheetBatch.ts:177-186`).

What the person does: types the 40 Node id and Pipe id cells §3.4 marks, uses no support code
and no `B` row, and enters the supports and bends by their forms, which take any id (§3.5,
§3.7). Criteria 6 and 17 compare against the fixture's ids, which this route produces.

## 3. Steps

Durations are estimates (§5). Each step names its desktop surface; unlisted fields are left
as the form shows them.

### 3.1 Pressure profile (1 min)

*Load Cases* section, panel "Pressure mechanics profile" (`features/load-cases/LoadCaseManagerPanel.tsx:536`;
`features/pressure-authoring/PressureAuthoringPanel.tsx:116-131`): **Pressure profile** =
exact_pressure_v3, **Shear model** = Euler–Bernoulli, **Queue pressure profile**, Apply. The
fixture's `pressure_contract` is `{3.0.0, exact_pressure_v3, euler_bernoulli}`. Done first,
the button's corrosion-allowance condition (`PressureAuthoringPanel.tsx:110-114`) holds
because there are no members yet.

### 3.2 Material RS-M1 (6 min)

1. Same panel, **Create explicit E/nu material** (`PressureAuthoringPanel.tsx:46-65`):
   **Exact material ID** `material:RS-M1`; **Exact material label** `RS-M1 invented
   carbon-steel-like material`; **Constitutive basis** homogeneous_isotropic_E_nu_v1;
   **Elastic modulus E** 200 GPa; **Poisson ratio nu** 0.3, unit 1; **Thermal expansion coefficient** 1.2e-5 1/K (`PressureAuthoringPanel.tsx:70`, since #36);
   **Constitutive provenance** ⟨P⟩. **Queue exact material**, Apply.
2. Select the material in the Model tree. Inspector, "Temperature-dependent properties"
   (`features/material-temperature/MaterialTemperatureForm.tsx:52-89`): **Add temperature
   point** four times, each with ID, Temperature, Elastic modulus, Poisson ratio (unit 1),
   Thermal expansion coefficient and provenance ⟨P⟩; shear modulus left blank.

   | Point ID | Temperature | Elastic modulus | Poisson ratio | Thermal expansion coefficient |
   |---|---|---|---|---|
   | `point:RS-M1-T020` | 20 degC | 200 GPa | 0.3 | 1.2e-5 1/K |
   | `point:RS-M1-T100` | 100 degC | 195 GPa | 0.3 | 1.2e-5 1/K |
   | `point:RS-M1-T200` | 200 degC | 189 GPa | 0.3 | 1.2e-5 1/K |
   | `point:RS-M1-T300` | 300 degC | 182 GPa | 0.3 | 1.2e-5 1/K |

   **Queue temperature points**, Apply.
3. Same selection, "Expansion laws (current exact model)" (`material-temperature/ExpansionLawsEditor.tsx:39-79`):
   **Add expansion law**; definition engineering_secant; ID `law:RS-M1-SECANT`; datum
   temperature 20 degC; data constant; constant coefficient 1.2e-5 1/K; provenance ⟨P⟩.
   **Queue expansion laws**, Apply.

### 3.3 Sections (3 min)

Inspector, **New section** (`features/model-tree/PropertyInspector.tsx:636-717`): Section ID,
Name, Type pipe, Length unit mm, Outside diameter, Wall thickness, Provenance ⟨P⟩; **Queue
section**, Apply. `section:RS-S270` (name RS-S270) 270 × 9.0; `section:RS-S220` 220 × 8.0;
`section:RS-S170` 170 × 7.0. These are shared pipe sections (OD and wall only), as the run
sheet requires (`runSheetModel.ts:355-366`).

### 3.4 Run-sheet rows (8 min)

26 rows in three chains, queued as one batch (**Queue rows**, `RunSheet.tsx:595-597`), then
Validate batch and Apply entire batch. Blank increments are zero. Support, Restraints, Bend R,
k and k source are blank on every row (§2). Section quantities are entered once and inherit
down the chain (`run-sheet-entry.md` §2): on row 2, Ins. thk 60, Ins. dens. 140, Contents
dens. 820, Material density 7850, Mill tolerance 1.0, Corrosion allowance 1.5. An `F` row
naming RS-130 inherits row 13's values (§2.3 rule 1).

Ids are written without their `node:` / `pipe:` prefix; the cells take the full id. **(t)**
marks a typed cell; the others are the derived id.

| # | dX | dY | dZ | Kind | Node | Section | Orientation | Other entries | Node id | Pipe id |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | | | 1 | P | | | | | RS-010 | |
| 2 | 0.4 | | | | | section:RS-S270 | 0 0 1 | Material `material:RS-M1`; the six section quantities | RS-020 | RS-P010 (t) |
| 3 | 0.15 | | | | | | | | RS-030 | RS-P020 (t) |
| 4 | 0.6 | | | | | | | | RS-040 | RS-P030 (t) |
| 5 | 0.15 | | | | | | | | RS-050 | RS-P040 (t) |
| 6 | 2.2 | | | | | | | | RS-060 | RS-P050 (t) |
| 7 | 2.5 | | | | | | | | RS-B1-CORNER (t) | RS-P060 (t) |
| 8 | | | 3 | | | | 1 0 0 | | RS-080 | RS-P070 (t) |
| 9 | | | 3 | | | | | | RS-B2-CORNER (t) | RS-P080 (t) |
| 10 | | 3 | | | | | 0 0 1 | | RS-100 (t) | RS-P090 (t) |
| 11 | | 4 | | | | | | | RS-110 (t) | RS-P100 (t) |
| 12 | | 4 | | | | | | | RS-120 (t) | RS-P110 (t) |
| 13 | | 4 | | | | | | | RS-130 (t) | RS-P120 (t) |
| 14 | | | | F | node:RS-130 | | | | | |
| 15 | | 3 | | | | | | | RS-140 (t) | RS-P130 (t) |
| 16 | | 2 | | | | | | | RS-150 (t) | RS-P140 (t) |
| 17 | | 0.4 | -0.025 | | | | | | RS-160 (t) | RS-P150 (t) |
| 18 | | 2.6 | | | | section:RS-S220 | | Mill tolerance 0.9 | RS-170 (t) | RS-P160 (t) |
| 19 | | 2 | | | | | | | RS-180 (t) | RS-P170 (t) |
| 20 | | 0.6 | | | | | | | RS-190 (t) | RS-P180 (t) |
| 21 | | 2.4 | | | | | | | RS-200 (t) | RS-P190 (t) |
| 22 | | 2 | | | | | | | RS-210 (t) | RS-P200 (t) |
| 23 | | | | F | node:RS-130 | | | | | |
| 24 | | | -2.5 | | | section:RS-S170 | 1 0 0 | Ins. thk 50; Mill tolerance 0.8 | RS-215 (t) | RS-P210 (t) |
| 25 | | | -2.5 | | | | | | RS-B3-CORNER (t) | RS-P220 (t) |
| 26 | 2 | | | | | | 0 0 1 | | RS-230 (t) | RS-P230 (t) |

Rows 7, 9 and 25 end at the corners B1 (6, 0, 1), B2 (6, 0, 7) and B3 (6, 15, 2), drafted as
`node:RS-B1-CORNER` and so on (`fixtures/reference_system/authoring.json` `bends[].corner`).
They are plain runs; §3.5 realizes them. Each corner joins exactly two pipes, the incoming
one first in model order, as `insert_bend_at_corner` needs.

The Label cell is optional. Blank, it is the node id's token; for the fixture's text, type
`nodes[].label`. Pipe labels are always the pipe id's token (`runSheetModel.ts:663-665`).

**Row 5 lands one unit in the last place short.** The sheet adds in binary64
(`runSheetModel.ts:629-633`, §6.2 of the design): 1.15 + 0.15 = 1.2999999999999998, not
the fixture's 1.3. Every other node lands exactly. After the batch applies, select RS-050,
**Start task from current selection**, **Property to edit** = X position, value 1.3,
**Add**, Apply (`PropertyInspector.tsx:474-560, 1700-1709`). (0.75 min)

### 3.5 Bends B1, B2, B3 (5.5 min)

A bend refuses while any support, load, region or reference names its corner or either
straight (`OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED`, `core/model_operations/operation_applier/src/bend_authoring.rs:605-639`;
RT-E1's finding). So bends follow the run sheet and precede §3.7–§3.12.

1. Select the corner node. Inspector, "Insert bend at corner"
   (`features/bend-authoring/BendAuthoringForms.tsx:95-145`, mounted for a node joining two
   pipes at `:148-158`). Overwrite the four prefilled ids:

   | Corner | Bend radius R | k | Bend start node ID | Bend end node ID | Bend arc pipe ID | Bend component ID |
   |---|---|---|---|---|---|---|
   | `node:RS-B1-CORNER` | 0.4 m | 4.2 | `node:RS-070` | `node:RS-075` | `pipe:RS-P065` | `component:RS-B1` |
   | `node:RS-B2-CORNER` | 0.4 m | 4.2 | `node:RS-090` | `node:RS-095` | `pipe:RS-P085` | `component:RS-B2` |
   | `node:RS-B3-CORNER` | 0.25 m | 3.6 | `node:RS-220` | `node:RS-225` | `pipe:RS-P225` | `component:RS-B3` |

   **Corner bend source of k** ⟨S5⟩; **Geometry source reference** ⟨S5⟩; **Corner bend
   provenance** ⟨P⟩. **Queue insert bend at corner**, then **Apply**, which creates the batch
   (`features/workspace/workspaceSession.ts:1300-1310, 1826-1836`), then **Validate batch** and
   **Apply entire batch**. The engine places the tangent points at R·tan(Φ/2) = R from the corner
   (`bend_authoring.rs:547, 642-643`); for these 90° corners they are exactly the fixture's
   (5.6, 0, 1), (6, 0, 1.4), (6, 0, 6.6), (6, 0.4, 7), (6, 15, 2.25), (6.25, 15, 2). The
   arc takes the incoming pipe's section and the incoming direction as `y_reference`
   (`bend_authoring.rs:659-666`), as the fixture records.
2. Bend SIFs, the person's entries (spec §11.3): select the bend component. Inspector,
   "Bend realization" (`BendAuthoringForms.tsx:58-93`), k and its source prefilled:
   **In-plane SIF i_ip** and **Out-of-plane SIF i_op** = 2.1 / 1.8 (RS-B1, RS-B2) and
   2.3 / 1.9 (RS-B3); **Queue bend realization update**, Apply. The fixture's
   `modifiers.source_reference` is ⟨S5⟩ for k and SIFs alike.

The bends shorten pipes the sheet created, so those rows read "Changed outside the sheet"
(`run-sheet-entry.md` §3.6). That is expected.

### 3.6 Components (18.75 min)

Inspector, **New component** (`PropertyInspector.tsx:912-1020`; fields `features/component-creation/RigidDraftFields.tsx:11-32`,
`ConnectorDraftFields.tsx:36-73`; the tee's at `PropertyInspector.tsx:948-969`). **Queue ⟨kind⟩**, Apply.

1. **Valve and flanges** (10 min). Length unit m and Weight unit N, as prefilled. Solver
   consumption mechanics_geometry_only; Stiffness scale 10.0 with source ⟨S5⟩; Geometry
   source ⟨S5⟩; Provenance ⟨P⟩; Stiffness behavior reference `rigid body, user stiffness
   scale 10.0 (DEC-045); ⟨S5⟩`; End A / End B reference `node:RS-0xx (body end on RS-P0xx)`
   from the table.

   | Component ID | Label | Kind | Node | Realized pipe | Body length | Weight | COG X, Y, Z | End A / End B reference nodes |
   |---|---|---|---|---|---|---|---|---|
   | `component:RS-F1` | Flange F1 | flange | RS-030 | RS-P020 | 0.15 | 350 | −0.075, 0, 0 | RS-020 / RS-030 |
   | `component:RS-V1` | Valve V1 | valve | RS-040 | RS-P030 | 0.6 | 2600 | −0.3, 0, 0.3 | RS-030 / RS-040 |
   | `component:RS-F2` | Flange F2 | flange | RS-050 | RS-P040 | 0.15 | 350 | −0.075, 0, 0 | RS-040 / RS-050 |

   End A and End B size are 270 mm in the fixture, but the form has one length unit for body
   length, end sizes and centre of gravity (`component-creation/componentIntent.ts:534-551`).
   Enter 0.27 here, then set **End A size** and **End B size** to 270 with unit mm on each
   component through the inspector task (`PropertyInspector.tsx:2033-2052`): six edits
   (gap G6).
2. **Expansion joint RS-EJ1** (6 min). ID `component:RS-EJ1`; Label `Expansion joint EJ1
   (tied)`; Kind expansion_joint; node RS-190; realized pipe (the replaced span) RS-P180;
   Connector axes (global) `0 -1 0; 1 0 0; 0 0 1`; End i and End j node axes `1 0 0; 0 1 0;
   0 0 1`; End i and End j offset `0 0 0`, unit m; Reference state stress_free; q_ref
   translation `0 0 0` m and rotation `0 0 0` rad; Installed reference temperature 20 degC;
   Translation scale 1 m; Work matrix upper triangle
   `150000 0 0 0 0 0 400000 0 0 0 0 400000 0 0 0 500000 0 0 1500 0 1500`; Installed
   geometry `authored model geometry`; Hardware explicit_axial_rods_v1 with two tie rods,
   `rod:RS-EJ1-RYP` at offsets `0 0.2 0` and `rod:RS-EJ1-RYN` at `0 -0.2 0` (both ends,
   unit m), stiffness 1.2e8 N/m, reference extension 0 m, source ⟨S5⟩; Pressure model
   constant_effective_area_reference_v1, effective area 0.049 m^2, basis
   internal_differential_zero_external_v1, source ⟨S5⟩; Stiffness source reference ⟨S5⟩;
   Measurement restraints `invented; no measurement`; Basis transform reference `none`;
   Validity statement `invented test object; no applicability claim`; Provenance ⟨P⟩.
3. **Tee RS-T1** (2.75 min), after the run sheet, which makes RS-130 join RS-P120, RS-P130
   and RS-P210.
   1. **New component**: ID `component:RS-T1`; Label `Tee T1`; Kind tee; **New component
      node** RS-130; **New tee header pipe** RS-P120; **New tee branch pipe** RS-P210
      (`PropertyInspector.tsx:947-951`); **Connection angle** 90 with **Angle unit** deg
      (prefilled rad); **Run size** 170 and **Header size** 270, in mm; **Connection type**
      `invented unreinforced tee`; **Reinforcement reference** `invented: no reinforcement`;
      Geometry source ⟨S5⟩; Provenance ⟨P⟩. **Queue tee**, Apply. The form requires the four
      descriptive members before it queues (`component-creation/componentIntent.ts:380-392`),
      and the applier refuses a tee without them (`OP-CREATE-TEE-GEOMETRY-INVALID`,
      `core/model_operations/operation_applier/src/lib.rs:3387-3446`); spec §5 supplies them.
      The created tee has no `modifiers` and no `mechanics_interface` (`lib.rs:2987-3006`),
      which the solver reads as mechanics_geometry_only
      (`core/product_physics/src/preview_physics.rs:307-313`), as for the fixture's record.
   2. The person's SIFs (spec §11.3): select `component:RS-T1`, **Start task from current
      selection**, **Property to edit** = Header SIF user value, value 2.4, **Add**, Apply;
      again with Branch SIF user value, 2.0 (`PropertyInspector.tsx:474-560, 1981-1998`).
      The fixture's `modifiers.source_reference` ⟨S5⟩ cannot be entered: the inspector offers
      no modifier-source property on a tee, only on a rigid component as "Stiffness scale
      source" (`PropertyInspector.tsx:1920-2009, 2097-2105`), although the applier accepts the
      field (`lib.rs:679-682`) (gap G11).

### 3.7 Supports (13 min)

Every support is entered in the inspector's support form, which takes an id: select a node,
"New support configuration" (`PropertyInspector.tsx:607-611`;
`features/support-configuration/SupportConfigurationForm.tsx:179-262`): **Support ID**,
**Support label**, **Support node**, **Support family**, the **Restrain** checkboxes,
**Support provenance** ⟨P⟩; **Queue support creation** (`:323`), Apply. Enter them in the
fixture's order:

| Support ID | Label | Node | Family | Restrained |
|---|---|---|---|---|
| `support:RS-A` | Anchor A (nozzle A) | RS-010 | Anchor | all six |
| `support:RS-B` | Anchor B (nozzle B) | RS-210 | Anchor | all six |
| `support:RS-C` | Anchor C (nozzle C) | RS-230 | Anchor | all six |
| `support:RS-LS1` | Line stop LS1 | RS-060 | Line stop | UX |
| `support:RS-V060` | Vertical support V060 | RS-060 | Vertical support | UZ |
| `support:RS-VS1` | Variable spring VS1 (user stiffness) | RS-080 | Variable spring hanger | none |
| `support:RS-FR1V` | RS-FR1V interim linear vertical support (one_way target pending R-4 S7) | RS-120 | Vertical support | UZ |
| `support:RS-V140` | Vertical support V140 | RS-140 | Vertical support | UZ |
| `support:RS-G2` | Guide G2 | RS-170 | Guide | UX, UZ |
| `support:RS-V200` | Vertical support V200 | RS-200 | Vertical support | UZ |
| `support:RS-G3` | Branch guide G3 | RS-215 | Guide | UX, UY |

RS-VS1 also takes **Include hanger data** (`SupportConfigurationForm.tsx:223-255`): Hanger
type variable_spring_hanger; **Add hanger stiffness**, DOF UZ, 180000 N/m; Hanger installed
load 8000 N; cold load 8000 N; hot load 7400 N; travel range 50 mm; source reference ⟨S5⟩;
mechanics consumption `linear_spring_primitive_user_stiffness`. RS-FR1V is the fixture's
interim linear vertical support (spec §10); it has a run-sheet code (V) but not its id.

### 3.8 Reference configuration (1.5 min)

*Load Cases* section, "Reference configurations" (`features/load-cases/LoadReferenceStateInputs.tsx:38-85`):
**Add reference configuration**; ID `reference:RS-INSTALLED`; label `Installed reference
(20 degC)`; geometry authored_model_geometry; provenance ⟨P⟩. One member typed for RS-P010:
**Add member**, pipe, basis temperature_reference, installation temperature 20 degC, fit
none, provenance ⟨P⟩. Then "Fill reference configuration 1 members from a template"
(`LoadReferenceStateInputs.tsx:79`; `BulkFillControls.tsx:64-82`, since #38): template the
RS-P010 row, targets every pipe without one, **Fill …**: the other 25 members, RS-P180
included. **Queue reference configurations**, Apply.

### 3.9 Self-weight, which creates the cases (6 min)

The self-weight generator creates its case and refuses an existing case id
(`core/product_physics/src/self_weight.rs:205-222, 446`), so RS-SUS, RS-OPE and RS-SAM are
created here. It reads the finished geometry, sections and components, so it follows
§3.3–§3.7; the tee carries no weight. *Review changes* › **Self weight** (`App.tsx:548, 591-593`;
`features/self-weight-authoring/SelfWeightPlanPanel.tsx:87-141`): **Self-weight action**
Create a self-weight case; **Gravity value** -9.80665; **Gravity unit** m/s^2; **Gravity
direction** Global Z; **Self-weight pipes**: every pipe except RS-P180 (25). Per case:

| Self-weight case ID | Self-weight case label | Self-weight provenance |
|---|---|---|
| `load:RS-SUS` | RS-SUS sustained: W + P (operating pressure) | ⟨P⟩`; W of RS-SUS: self-weight of every member except RS-P180, component weights at their centres of gravity (spec §8.1)` |
| `load:RS-OPE` | RS-OPE operating: W + P + T (210 degC, modulus at 210 degC) | as above, with RS-OPE |
| `load:RS-SAM` | RS-SAM seismic anchor movement: W + P + anchor RS-C moved UX +15 mm, UY -10 mm | as above, with RS-SAM |

**Generate self-weight plan**, **Queue complete self-weight plan**, then Validate batch and
Apply entire batch. Each case gets the fixture's 28 primitives (22 pipe line loads, a force
and a moment for each of RS-F1, RS-V1, RS-F2).

### 3.10 Pressure regions (5 min)

*Load Cases*, select the case in **Load cases** (`LoadCaseManagerPanel.tsx:841-866`), then
"Pressure regions — ⟨case⟩" (`PressureAuthoringPanel.tsx:135-184`). Five regions per case,
ids `region:⟨case⟩-RG-1` and so on (for example `region:RS-SUS-RG-1`). Each: **Add pressure
region**; member pipes; pressure basis internal_differential_zero_external_v1; pressure
1600 kPa; two ordered terminals, each `transfers_to_wall` with provenance ⟨P⟩; region
provenance ⟨P⟩. **Queue pressure regions**, Apply. The five regions are typed for RS-SUS
only. For RS-OPE and RS-SAM, "Copy pressure regions from another case"
(`PressureAuthoringPanel.tsx:174`; `BulkFillControls.tsx:111-122`, since #38) copies RS-SUS's
applied regions and re-derives their ids (`region:RS-SUS-RG-1` → `region:RS-OPE-RG-1`);
Queue and Apply per case.

| Region | Member pipes | Member connectors | Terminals |
|---|---|---|---|
| RG-1 | RS-P010 … RS-P120 (14, arcs RS-P065, RS-P085 included) | | RS-010, RS-130 |
| RG-2 | RS-P210, RS-P220, RS-P225, RS-P230 | | RS-130, RS-230 |
| RG-3 | RS-P130, RS-P140 | | RS-130, RS-150 |
| RG-R | RS-P150 | | RS-150, RS-160 |
| RG-4 | RS-P160, RS-P170, RS-P190, RS-P200 | `component:RS-EJ1` | RS-160, RS-210 |

### 3.11 Analysis states (5 min)

Same section and case, "Analysis state — ⟨case⟩" (`LoadReferenceStateInputs.tsx:87-114`):
contract `openpipestress.load_reference_state/1.0.0`; reference configuration
`reference:RS-INSTALLED`; history independent_equilibrium; provenance ⟨P⟩. Then three lists
(`:125-228`), entered with the fills of #38: one row typed and the rest filled
("Fill element states from a template", "Fill support states from a template", targets
every pipe or support without one; `:147, :193`), and "Add every primitive of this case" at
factor 1 for the load sources (`:228`; `BulkFillControls.tsx:102`). RS-SAM starts from
"Copy analysis state from another case" (RS-SUS, `:107`): its RS-SUS load sources are left
out and reported, then it adds its own primitives and RS-C's two motions. RS-P180's row is
edited after each fill (its provenance; in RS-OPE also its temperature and thermal state). A
fill or copy carries only the members a row shows. The rows are:

- **Element states**, 26 per case, one per pipe in model order:
  - RS-SUS and RS-SAM: material selection exact_point, material `material:RS-M1`, point
    `point:RS-M1-T020`; thermal state unchanged_reference, provenance ⟨P⟩`; no temperature
    in this case`, except RS-P180, whose provenance is ⟨P⟩`; replaced span RS-P180: zero
    eigenstrain (spec §5.1)` in every case.
  - RS-OPE: operating temperature 210 degC; material selection temperature_interpolation,
    `material:RS-M1`, selection temperature 210 degC, piecewise_linear, forbidden; thermal
    state free_length_state, expansion law `law:RS-M1-SECANT`. RS-P180 has no operating
    temperature and keeps unchanged_reference with provenance ⟨P⟩`; replaced span RS-P180:
    zero eigenstrain (spec §5.1)`; its material selection is as the others.
- **Support states**, 11 per case, in §3.7's order, each active_model_device. In RS-SAM,
  `support:RS-C` takes two boundary motions with meaning absolute_reference_displacement:
  UX 15 mm and UY -10 mm.
- **Load sources**, 28 per case: each of the case's self-weight primitives, factor 1.

**Queue analysis state**, Apply, per case.

### 3.12 Combinations (1.5 min)

*Load Cases*, "Create combination" (`LoadCaseManagerPanel.tsx:724-830`): **Basis**
result_state_subtraction; provenance ⟨P⟩; any rationale; **Queue combo**, Apply.

| ID | Label | Minuend | Subtrahend |
|---|---|---|---|
| `combination:RS-EXP` | RS-EXP expansion: RS-OPE - RS-SUS | `load:RS-OPE` | `load:RS-SUS` |
| `combination:RS-dSAM` | RS-dSAM anchor movement term: RS-SAM - RS-SUS | `load:RS-SAM` | `load:RS-SUS` |

### 3.13 Solve and key results (2 min)

*Analyze* section, **Run mechanics preview** (`features/solve/SolvePanel.tsx:130-133`); the
resolved load/reference states list beside it (`solve/LoadReferenceStatesBlock.tsx:45`).
Under *Results*: reactions at RS-A, RS-B and RS-C and the spring load (KR-01, KR-03) in
**Support Load Summary** (`results/SupportLoadSummaryPanel.tsx:133-134`); largest translation
(KR-02) in **Displacement Extremes** (`results/DisplacementExtremesPanel.tsx:155-156`); the
**Key results** panel's "Anchors and equipment connections" group (`results/KeyResultsPanel.tsx:171, 223-224`).
The solve time on the desktop is not measured here.

### 3.14 What still differs from the fixture

The route above gives the fixture's ids, numbers and units, except the tee's modifier
source (G11). Text and form differences remain, none in criterion 6's identity set or a
numerical value:

- the project's id, name and description (`projectService.ts:440-455`);
- labels the sheet or forms derive: pipe labels, and the bend form's "Bend start", "Bend
  end", "Bend arc" and "Bend" (`BendAuthoringForms.tsx:113-119`); node labels can be
  edited afterwards (`PropertyInspector.tsx:1700`);
- the connector's stiffness provenance, which the form fills with the connector's own
  (`componentIntent.ts:206-210, 231`);
- `terms: []` on each subtraction combination (`operation_applier/src/lib.rs:6179-6187`);
  the fixture omits it, and the applier treats an empty list as no terms (`lib.rs:9045-9048`);
- each self-weight primitive's provenance records the hash of the model it was generated
  from, which differs from the fixture generator's capture model;
- array order: the bend operation appends its nodes, arc and component
  (`bend_authoring.rs:699-701, 710, 729`), so records are compared by id;
- the tee: the desktop record lacks `modifiers.source_reference` (G11).

## 4. Pending families

When a spec §10 row lands, the generator adds its entities and this sheet gains a step. The
run-sheet rows do not change.

| Row (spec §10) | Lane | Step it adds |
|---|---|---|
| Eccentric reducer RS-R1 | RT-A3 increment 2 | After §3.6: reducer on RS-P150 at RS-160, offset (0, 0, +25) mm, 300 N at the body midpoint, scale 2.0; before §3.9 |
| Constant effort RS-CE1 | RT-A3 increment 3 | §3.7: RS-100, UZ, constant load 9.0 kN with its travel or movement limit and source |
| Lift-off, friction, gap | RT-A3 increments 4–5 | §3.7: RS-LO1 (RS-110, UZ), RS-FR1 (RS-120, UY, μ 0.30, normal from RS-FR1V UZ), RS-G1 (RS-140, UX, 5 mm) |
| RS-FR1V one_way | RT-A3, R-4 S7 | §3.7: RS-FR1V becomes nonlinear one_way UZ |
| `difference_basis` on RS-EXP | RT-A5 slice 1b | §3.12: the basis field |
| Wind, static seismic | #1678 | Cases RS-WIN, RS-EQX/Y/Z and their difference combinations |
| SRSS, absolute sum | RT-A4 | §3.12: RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS |
| Hydrotest RS-HYD | RT-A6 | A case with test contents, test pressure and held supports |
| Condition set | RT-G5 | One condition set; removes per-case temperature entries from §3.11 |
| Modal | RT-A7 | A modal request on RS-OPE to 60 Hz |
| Response spectrum RS-SP1 | RT-A8 | The spectrum and its request |
| Hanger sizing | RT-A11 | A sizing request for RS-VS1 and RS-CE1 |
| Snubber RS-SN1 | RT-A8 | §3.7: RS-110, UX, rigid |

## 5. Counts and estimate

An entry is one typed value, selection or checkbox; buttons that add a row (**Add …**) and
queue or apply buttons are not entries; a quantity whose unit is not prefilled is
two. Times are estimates of a practised person pasting ⟨P⟩ and ⟨S5⟩, not measurements.
The tee's row uses the rates of its neighbours: its form entries at the rigid-component
form's rate (about 5.5 s per entry: §3.6's 10 minutes less the six end-size edits, over
its 60 form entries) and each SIF edit at the RS-050 correction's 0.75 minutes.

| Step | Entries | Queue/apply cycles | Estimate (min) |
|---|---|---|---|
| 3.1 Pressure profile | 2 | 1 | 1 |
| 3.2 Material | 63 | 3 | 6 |
| 3.3 Sections | 21 | 3 | 3 |
| 3.4 Run sheet: 26 rows (88 cells, 40 of them ids) and header (10) | 98 | 1 | 8 |
| 3.4 RS-050 correction | 3 | 1 | 0.75 |
| 3.5 Bends and SIFs | 39 | 6 | 5.5 |
| 3.6 Valve and flanges, with end-size edits | 84 | 9 | 10 |
| 3.6 Expansion joint | 53 | 1 | 6 |
| 3.6 Tee RS-T1 (14 form entries) and the person's two SIFs | 20 | 3 | 2.75 |
| 3.7 Supports | 98 | 11 | 13 |
| 3.8 Reference configuration | 14 | 1 | 1.5 |
| 3.9 Self-weight (creates the three cases) | 93 | 3 | 6.25 |
| 3.10 Pressure regions | 86 | 3 | 5 |
| 3.11 Analysis states | 68 | 3 | 5 |
| 3.12 Combinations | 14 | 2 | 1.5 |
| 3.13 Solve and results | 1 | — | 2 |
| **Total** | **757** | **51** | **about 77 (77.25)** |

The estimate is about 77 minutes, about two and a half times the provisional 30; the tee
(#45) added 2.75. It was about 130
before #38's fills and case copies (gaps G7, G8), which brought the per-member load/reference
state and pressure regions (§3.8, §3.10, §3.11) from about 1,450 entries and 66.5 minutes to
168 entries and about 11.5 minutes; those entry counts are the authoring lane's, measured by
its round-trip test of the fixture (#38). The component and support forms (§3.6, §3.7) are
now the largest share, about 32 minutes; the run-sheet support and bend cells under review
(#41, G1–G3) would move most of §3.5 and §3.7 into §3.4. The run sheet itself is about 8.

## 6. Gaps found

Each is a step the desktop on `main` cannot do as the fixture records it, or does only at a
cost §5 shows. None is worked around outside the operation route.

| # | Gap | Evidence | Owner | What the person does |
|---|---|---|---|---|
| G1 | No support-id cell in the run sheet; ids are `support:⟨node token⟩`, and a second support at one node is a duplicate. Design §1.2 puts support and bend ids in the row's detail. | `runSheetModel.ts:248, 684, 755-768`; `runSheetView.ts:56-57, 455-459` | DEL-07-02 | Support form (§3.7) |
| G2 | No bend-id cells: a `B` row's tangent nodes, arc and component ids derive from the corner; its geometry source is the sheet's Source and its labels are fixed | `runSheetModel.ts:249-252, 738-740`; `runSheetBatch.ts:177-186` | DEL-07-02 | Plain runs to the corner, then "Insert bend at corner" (§3.5) |
| G3 | Numbering cannot pad numbers or name a pipe by its start node | `runSheetModel.ts:244-247`; design §3.2 | DEL-07-02 | Types 40 ids (§3.4) |
| G4 | Increments add in binary64, so RS-050 lands at 1.2999999999999998 m (as designed, §6.2) | `runSheetModel.ts:629-633` | DEL-07-02 | One X-position edit (§3.4) |
| G5 | Closed by #36 (authoring lane I1): the E/nu create form takes the material-level thermal expansion coefficient, and a wasm-engine test creates RS-M1 from the form equal to the fixture's record | `PressureAuthoringPanel.tsx:70` | DEL-07-03 | §3.2 step 1 |
| G6 | **New component** has one length unit for body length, end sizes and centre of gravity | `componentIntent.ts:534-551` | DEL-03-05 | Six end-size edits (§3.6) |
| G7 | Closed by #38: "Fill … from a template" for reference-configuration members, element states and support states, "Add every primitive of this case" for load sources, and "Copy analysis state from another case" | `BulkFillControls.tsx:64-122`; `LoadReferenceStateInputs.tsx:79, 107, 147, 193, 228` | DEL-07-03 | §3.8, §3.11 |
| G8 | Closed by #38: "Copy pressure regions from another case", with ids re-derived | `PressureAuthoringPanel.tsx:174` | DEL-07-03 | §3.10 |
| G9 | Each queued single intent is applied on its own | `OperationApplyPanel.tsx:105-114` | no deliverable lists the file | 51 apply cycles (§5) |
| G10 | Closed in DEL-09-01: tee creation requires a run size, header size, connection type and reinforcement reference; spec §5 now gives invented values, which EX does not read | `componentIntent.ts:380-392`; `operation_applier/src/lib.rs:3387-3446` | DEL-09-01 (spec §5) | §3.6 |
| G11 | No modifier-source entry for a tee: the inspector offers header and branch SIFs but not `modifiers.source_reference`, which it offers only on rigid components | `PropertyInspector.tsx:1920-2009, 2097-2105`; applier field rule `lib.rs:679-682` | DEL-03-05 | Leaves the SIF source unentered (§3.6) |

**RT-G2 candidates not named in spec §11.2:**
- support-id and bend-id cells in the run sheet, as the design's row detail describes (G1,
  G2);
- a numbering pattern with padding, and an option to name a pipe by its start node (G3);
- a unit per quantity on the rigid-component form (G6);
- applying several queued intents as one reviewed batch (G9);
- a modifier-source field for a tee, beside its SIFs (G11).
