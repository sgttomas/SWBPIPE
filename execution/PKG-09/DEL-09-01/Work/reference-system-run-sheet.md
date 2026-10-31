# Reference system run sheet (exit criterion 7)

**Basis:** `main` at `66ecc9f17`, at `5bb2f9ff0` for the tee (§3.6 item 3), and at `148a1089b`
for the run sheet's id cells and numbering (#41: §2, §3.4, §3.5, §3.7); fixture `fixtures/reference_system/reference_system.json`
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
**Apply** under *Review changes* (`features/operations/OperationApplyPanel.tsx`). Since #54,
**Review all N queued changes as one batch** can move independent user intents to batch
review in queue order. The move applies nothing; **Apply entire batch** validates and
applies all or none, as one Undo (`features/toolkit/BatchReviewPanel.tsx`). **Validate
batch** checks without applying and is required first for preview-gated operations,
including `insert_bend_at_corner`. Dependent before-values are never adjusted; shared
operation ids are named and refused before anything leaves the single queue. §5 retains
the per-intent cycle counts before optional G9 batching; it assigns no separate timing
rate to an Apply cycle, so no numerical time credit for batching is inferred.

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
   (`features/model-tree/runSheet/RunSheet.tsx:546-569`) is set before the first row; the
   Numbering fields lock once any row has a number (`RunSheet.tsx:551`):
   - **Source** = ⟨P⟩ (`RunSheet.tsx:547-548`; it becomes the `provenance` of every node, pipe,
     support and bend record the sheet creates);
   - **Length unit** = m (the project unit; the default, `RunSheet.tsx:549`);
   - **Numbering** (`RunSheet.tsx:552-564`): **Node prefix** `node:RS-`, **Pipe prefix**
     `pipe:RS-P`, **Start** 10 and **Step** 10 (the defaults, `runSheetModel.ts:102`),
     **Pad numbers to N digits** 3, **Pipe number from** = its start node;
   - the column groups **Section extras** and **Ids and sources** shown, each by its toggle
     button (`RunSheet.tsx:566-568`; `runSheetView.ts:29`; both start hidden, `RunSheet.tsx:63`);
   - the column units chosen in the headers: Ins. thk mm, Ins. dens. kg/m^3, Contents dens.
     kg/m^3, Material density kg/m^3, Mill tolerance mm, Corrosion allowance mm. Bend R keeps
     the project unit, m.

**How the sheet's ids meet the fixture's.** A row's number is assigned when the row first
gets a value, in the order rows are typed, and every row takes one, `F` and `S` rows
included (`RunSheet.tsx:192-195`; `runSheetModel.ts:311-333`; DEL-07-02 `Work/run-sheet-entry.md`
§3.2). With pad 3 the number 10 gives `node:RS-010` (`runSheetModel.ts:287-298`), and a pipe
takes the number of the row whose end node is its start (`runSheetModel.ts:738-753`), as the
fixture names RS-P010 (010 → 020). The ids still to type:

- a pipe whose start node has no sheet number (a typed id, such as a corner's) or is a second
  pipe from one node (the tee branch) falls back to its own row's number
  (`runSheetModel.ts:746-752, 877-887`), which is not the fixture's;
- the B1 and B2 corners' derived ids, RS-070 and RS-090, are the ids of their bends' near
  tangents, which the sheet refuses as duplicates (`runSheetModel.ts:903-909`), so those
  corners are typed;
- the `F` row naming RS-130 takes the number 220, so the branch rows take 230, 240 and 250,
  whose derived ids are not the fixture's RS-215, the B3 straight's RS-P220 or RS-230;
- a support's id derives as `support:⟨node token⟩` and a bend's as `node:⟨corner⟩-T1`, `-T2`,
  `pipe:⟨corner⟩-bend` and `component:⟨corner⟩-bend` (`runSheetModel.ts:299-303`). The
  **Ids and sources** group has a cell for each: **Support id** and **Support label** on a
  row with a support code; **Bend start id**, **Bend end id**, **Bend arc id**, **Bend
  component id** and **Bend geometry source** on a `B` row (`runSheetView.ts:63-69, 461-480`;
  `runSheetModel.ts:764-778, 828-835, 852-856`).

**A pipe id is fixed once shown.** A pipe id derived by either rule is recorded for its row the
first time the row shows it, and from then on no other row may take it, derived or typed, even
after the person overwrites it or deletes the row (`RunSheet.tsx:125-130`;
`runSheetView.ts:300-311`; `runSheetModel.ts:503-508, 733-737, 903-909`). So on each row whose
Pipe id §3.4 marks **(t)**, type the Pipe id before any other cell of that row. If it is
typed after the row's first value, the id the row showed first stays reserved to that row. On
row 9, for example, that is RS-P080, which row 10 then cannot have.

The order chosen here takes 9 typed Node id and Pipe id cells, where the earlier route took 40:
the redundant `F node:RS-130` row of the earlier table (its row 14) is dropped, since the next
row continues from RS-130 anyway, and the second support at RS-060, an `S` row, is inserted
(**Insert row below** row 6, `RunSheet.tsx:470`) after every other row is typed, so it takes
the last number (260) rather than shifting the rest. Typed strictly in order, with both rows
taking numbers in the main chain, the authoring lane's round-trip test counts 35 typed ids.
Ten of the eleven supports and all three bends are entered in the sheet with the fixture's
ids; RS-VS1 stays a form (§3.7). Criteria 6 and 17 compare against the fixture's ids, which this route produces.

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
sheet requires (`runSheetModel.ts:410-421`).

### 3.4 Run-sheet rows (11 min)

26 rows in two chains, from RS-010 and from the tee, queued as one batch (**Queue rows**,
`RunSheet.tsx:637-639`), then Validate batch and Apply entire batch: the batch holds bends, so
it applies only after a clean validation (`BatchReviewPanel.tsx:24-32`). Blank increments are
zero. Section quantities are entered once and inherit down the chain (`run-sheet-entry.md` §2):
on row 2, Ins. thk 60, Ins. dens. 140, Contents dens. 820, Material density 7850, Mill
tolerance 1.0, Corrosion allowance 1.5. The `F` row naming RS-130 inherits row 14's values
(§2.3 rule 1).

Ids are written without their `node:` / `pipe:` prefix; the cells take the full id. **(t)**
marks a typed cell, typed before any other cell of its row when it is a Pipe id (§2); the
others are the derived id. **No.** is the number the row takes. Type the rows in table order,
except row 7, which is inserted after row 26 is typed.

| # | No. | dX | dY | dZ | Kind | Node | Section | Orientation | Other entries | Node id | Pipe id |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 10 | | | 1 | P | | | | Support A | RS-010 | |
| 2 | 20 | 0.4 | | | | | section:RS-S270 | 0 0 1 | Material `material:RS-M1`; the six section quantities | RS-020 | RS-P010 |
| 3 | 30 | 0.15 | | | | | | | | RS-030 | RS-P020 |
| 4 | 40 | 0.6 | | | | | | | | RS-040 | RS-P030 |
| 5 | 50 | 0.15 | | | | | | | | RS-050 | RS-P040 |
| 6 | 60 | 2.2 | | | | | | | Support LS1 | RS-060 | RS-P050 |
| 7 | 260 | | | | S | | | | Support V060; inserted last | | |
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
| 23 | 220 | | | | F | node:RS-130 | | | | | |
| 24 | 230 | | | -2.5 | | | section:RS-S170 | 1 0 0 | Ins. thk 50; Mill tolerance 0.8; Support G3 | RS-215 (t) | RS-P210 (t) |
| 25 | 240 | | | -2.5 | B | | | | Bend B3 | RS-240 | RS-P220 (t) |
| 26 | 250 | 2 | | | | | | 0 0 1 | Support C | RS-230 (t) | RS-P230 (t) |

The nine typed ids, and why each is typed: the B1 and B2 corners (rows 8, 10), whose derived
ids RS-070 and RS-090 are their bends' near tangents; the pipes after them (rows 9, 11), whose
start is a typed corner; the branch's first pipe and its end node (row 24), a second pipe from
RS-130 whose derived number is 230; the B3 straight (row 25), whose start RS-215 is typed; and
the last node and pipe (row 26), whose derived ids are RS-250 and RS-P240. The B3 corner keeps
its derived id RS-240, which no fixture record uses and the bend removes.

**Supports** (Support and Restraints cells, `runSheetModel.ts:22, 764-778`; Support id and
Support label in **Ids and sources**). `*` is all six restraints (`runSheetModel.ts:770`):

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

Each creates `{id, label, node, restraints, family, provenance}` (`runSheetBatch.ts:150-153`),
the fixture's record for these ten. RS-FR1V is the fixture's interim linear vertical support
(spec §10), so code V carries it. RS-VS1 has no code: the codes are A, G, L and V only
(`runSheetModel.ts:22`), and its hanger data has no cell (§3.7).

**Bends** (Bend R, k and k source cells, `runSheetView.ts:57-59`; the rest in **Ids and
sources**, `runSheetView.ts:65-69`):

| # | Corner | Bend R | k | k source | Bend start id | Bend end id | Bend arc id | Bend component id | Bend geometry source |
|---|---|---|---|---|---|---|---|---|---|
| 8 | B1 (6, 0, 1) | 0.4 | 4.2 | ⟨S5⟩ | `node:RS-070` | `node:RS-075` | `pipe:RS-P065` | `component:RS-B1` | ⟨S5⟩ |
| 10 | B2 (6, 0, 7) | (inherits 0.4) | 4.2 | ⟨S5⟩ | `node:RS-090` | `node:RS-095` | `pipe:RS-P085` | `component:RS-B2` | ⟨S5⟩ |
| 25 | B3 (6, 15, 2) | 0.25 | 3.6 | ⟨S5⟩ | `node:RS-220` | `node:RS-225` | `pipe:RS-P225` | `component:RS-B3` | ⟨S5⟩ |

Bend R inherits within a chain and k never does (`runSheetModel.ts:46, 613-618`;
`run-sheet-entry.md` §2.1). A `B` row needs the next row to be a run with the same section and
material (`runSheetModel.ts:939-955`), which rows 9, 11 and 26 inherit. The bend takes the
sheet's Source ⟨P⟩ as the provenance of its tangent nodes, arc and component
(`runSheetBatch.ts:171-186`). The engine places the tangent points at R·tan(Φ/2) = R from the
corner (`bend_authoring.rs:547, 642-643`); for these 90° corners they are exactly the
fixture's (5.6, 0, 1), (6, 0, 1.4), (6, 0, 6.6), (6, 0.4, 7), (6, 15, 2.25), (6.25, 15, 2).
The arc takes the incoming pipe's section and the incoming direction as `y_reference`
(`bend_authoring.rs:659-666`), as the fixture records.

**Bends before anything names their corners or straights.** A bend refuses while any
support, load, region or reference names its corner or either straight
(`OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED`,
`core/model_operations/operation_applier/src/bend_authoring.rs:605-639`; RT-E1's finding).
The order still holds with the bends in the sheet. Each bend's `insert_bend_at_corner` follows
the next row's operations in the same batch (`runSheetBatch.ts:243-249`). The sheet refuses a
support code on a `B` row and an `F` or `S` row at its corner (`runSheetModel.ts:850, 956-960`),
and the supports it creates name nodes, none of them a corner, so nothing in the batch names a
corner or a straight when its bend is inserted. Every later step (§3.5–§3.12) runs on the
realized bends.

The Label cell is optional. Blank, it is the node id's token; for the fixture's text, type
`nodes[].label`. Pipe labels are always the pipe id's token (`runSheetModel.ts:757, 885`).

**Row 5 lands one unit in the last place short.** The sheet adds in binary64
(`runSheetModel.ts:696-700`, §6.2 of the design): 1.15 + 0.15 = 1.2999999999999998, not
the fixture's 1.3. Every other node lands exactly. After the batch applies, select RS-050,
**Start task from current selection**, **Property to edit** = X position, value 1.3,
**Add**, Apply (`PropertyInspector.tsx:474-560, 1700-1709`). (0.75 min)

### 3.5 Bend SIFs (0.75 min)

The person's entries (spec §11.3), which the sheet has no cell for: select the bend
component. Inspector, "Bend realization" (`features/bend-authoring/BendAuthoringForms.tsx:58-93`),
k and its source prefilled: **In-plane SIF i_ip** and **Out-of-plane SIF i_op** = 2.1 / 1.8
(RS-B1, RS-B2) and 2.3 / 1.9 (RS-B3); **Queue bend realization update**, Apply. The fixture's
`modifiers.source_reference` is ⟨S5⟩ for k and SIFs alike.

### 3.6 Components (18.75 min)

Inspector, **New component** (`PropertyInspector.tsx:912-1020`; fields `features/component-creation/RigidDraftFields.tsx:11-32`,
`ConnectorDraftFields.tsx:36-73`; the tee's at `PropertyInspector.tsx:948-969`). **Queue ⟨kind⟩**, Apply.

1. **Valve and flanges** (6 min). Body length unit m, COG unit m and Weight unit N, as prefilled. Solver
   consumption mechanics_geometry_only; Stiffness scale 10.0 with source ⟨S5⟩; Geometry
   source ⟨S5⟩; Provenance ⟨P⟩; Stiffness behavior reference `rigid body, user stiffness
   scale 10.0 (DEC-045); ⟨S5⟩`; End A / End B reference `node:RS-0xx (body end on RS-P0xx)`
   from the table.

   | Component ID | Label | Kind | Node | Realized pipe | Body length | Weight | COG X, Y, Z | End A / End B reference nodes |
   |---|---|---|---|---|---|---|---|---|
   | `component:RS-F1` | Flange F1 | flange | RS-030 | RS-P020 | 0.15 | 350 | −0.075, 0, 0 | RS-020 / RS-030 |
   | `component:RS-V1` | Valve V1 | valve | RS-040 | RS-P030 | 0.6 | 2600 | −0.3, 0, 0.3 | RS-030 / RS-040 |
   | `component:RS-F2` | Flange F2 | flange | RS-050 | RS-P040 | 0.15 | 350 | −0.075, 0, 0 | RS-040 / RS-050 |

   Enter **End A size** and **End B size** as 270, choosing mm in each size unit
   selector. Body length and COG keep their own m units. Both creation forms carry
   independent units (`component-creation/componentIntent.ts`; DEL-03-05
   `Work/component-quantity-units.md`), so no later end-size edits are needed (G6).
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
3. **Tee RS-T1** (3 min), after the run sheet, which makes RS-130 join RS-P120, RS-P130
   and RS-P210.
   1. **New component**: ID `component:RS-T1`; Label `Tee T1`; Kind tee; **New component
      node** RS-130; **New tee header pipe** RS-P120; **New tee branch pipe** RS-P210
      (`PropertyInspector.tsx:947-951`); **Connection angle** 90 with **Angle unit** deg
      (prefilled rad); **Size unit** mm (prefilled m); **Run size** 170 and **Header size** 270; **Connection type**
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

### 3.7 Variable spring RS-VS1 (1.75 min)

The other ten supports are run-sheet rows (§3.4). RS-VS1 is a variable spring hanger with
hanger data, which the sheet's codes and cells cannot carry, so it is entered in the
inspector's support form: select RS-080, "New support configuration"
(`PropertyInspector.tsx:607-611`; `features/support-configuration/SupportConfigurationForm.tsx:179-262`):
**Support ID** `support:RS-VS1`; **Support label** `Variable spring VS1 (user stiffness)`;
**Support node** RS-080; **Support family** Variable spring hanger; no **Restrain**
checkbox; **Support provenance** ⟨P⟩. **Include hanger data** (`SupportConfigurationForm.tsx:223-255`):
Hanger type variable_spring_hanger; **Add hanger stiffness**, DOF UZ, 180000 N/m; Hanger
installed load 8000 N; cold load 8000 N; hot load 7400 N; travel range 50 mm; source
reference ⟨S5⟩; mechanics consumption `linear_spring_primitive_user_stiffness`. **Queue
support creation** (`:323`), Apply. The bends are already realized (§3.4), so this support
cannot block them.

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
- **Support states**, 11 per case, one per support, each active_model_device. In RS-SAM,
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
- labels the sheet derives: pipe labels (`runSheetModel.ts:757, 885`), and a bend's
  "Bend start", "Bend end", "Bend arc" and "Bend" (`runSheetBatch.ts:23, 178-186`), which no
  cell changes, against the fixture's `RS-070 Bend B1 near tangent`, `Bend B1` and so on.
  Node labels can be typed in the Label cell (§3.4) or edited afterwards
  (`PropertyInspector.tsx:1700`). Support labels now match: the ten in the sheet are typed in
  Support label (§3.4), and RS-VS1's in its form (§3.7);
- the connector's stiffness provenance, which the form fills with the connector's own
  (`componentIntent.ts:206-210, 231`);
- `terms: []` on each subtraction combination (`operation_applier/src/lib.rs:6179-6187`);
  the fixture omits it, and the applier treats an empty list as no terms (`lib.rs:9045-9048`);
- each self-weight primitive's provenance records the hash of the model it was generated
  from, which differs from the fixture generator's capture model;
- array order: the bend operation appends its nodes, arc and component
  (`bend_authoring.rs:699-701, 710, 729`), and the sheet creates supports in row order with
  RS-VS1 after them, not in the fixture's order, so records are compared by id;
- the tee: the desktop record lacks `modifiers.source_reference` (G11).

## 4. Pending families

When a spec §10 row lands, the generator adds its entities and this sheet gains a step. The
run-sheet rows do not change, except row 13's support when RS-FR1V becomes one_way.

| Row (spec §10) | Lane | Step it adds |
|---|---|---|
| Eccentric reducer RS-R1 | RT-A3 increment 2 | After §3.6: reducer on RS-P150 at RS-160, offset (0, 0, +25) mm, 300 N at the body midpoint, scale 2.0; before §3.9 |
| Constant effort RS-CE1 | RT-A3 increment 3 | §3.7: RS-100, UZ, constant load 9.0 kN with its travel or movement limit and source |
| Lift-off, friction, gap | RT-A3 increments 4–5 | §3.7: RS-LO1 (RS-110, UZ), RS-FR1 (RS-120, UY, μ 0.30, normal from RS-FR1V UZ), RS-G1 (RS-140, UX, 5 mm) |
| RS-FR1V one_way | RT-A3, R-4 S7 | RS-FR1V becomes nonlinear one_way UZ, which code V cannot carry: it leaves row 13 of §3.4 for the support form of §3.7 |
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
form's pre-G6 rate (about 5.5 s per entry: its former 10 minutes less the six end-size edits, over
its 60 form entries) and each SIF edit at the RS-050 correction's 0.75 minutes. The rows #41
changed use the same two rates, rounded to the quarter minute: the run sheet at its own rate
before #41, 8 minutes over 98 entries (about 4.9 s per entry), and the bend SIFs (§3.5) and
RS-VS1 (§3.7) at the form rate.

| Step | Entries | Queue/apply cycles | Estimate (min) |
|---|---|---|---|
| 3.1 Pressure profile | 2 | 1 | 1 |
| 3.2 Material | 63 | 3 | 6 |
| 3.3 Sections | 21 | 3 | 3 |
| 3.4 Run sheet: 26 rows (122 cells: 9 node and pipe ids, 41 for the supports, 26 for the bends, 46 others) and header (13) | 135 | 1 | 11 |
| 3.4 RS-050 correction | 3 | 1 | 0.75 |
| 3.5 Bend SIFs | 9 | 3 | 0.75 |
| 3.6 Valve and flanges, with independent quantity units | 66 | 3 | 6 |
| 3.6 Expansion joint | 53 | 1 | 6 |
| 3.6 Tee RS-T1 (15 form entries) and the person's two SIFs | 21 | 3 | 3 |
| 3.7 Variable spring RS-VS1 | 20 | 1 | 1.75 |
| 3.8 Reference configuration | 14 | 1 | 1.5 |
| 3.9 Self-weight (creates the three cases) | 93 | 3 | 6.25 |
| 3.10 Pressure regions | 86 | 3 | 5 |
| 3.11 Analysis states | 68 | 3 | 5 |
| 3.12 Combinations | 14 | 2 | 1.5 |
| 3.13 Solve and results | 1 | — | 2 |
| **Total** | **669** | **32** | **about 61 (60.5)** |

How the #41 rows are counted:

- **§3.4, 135 entries.** The header's 13: Source, the two prefixes, Pad numbers to N digits,
  Pipe number from, the two group toggles and the six column units (the 10 before #41, plus
  pad, Pipe number from and the Ids and sources toggle). The rows' 122 cells: 46 increments,
  codes and section cells (the earlier 48, less the dropped `F` row's Kind and Node); 41 for
  the supports (four cells each for ten supports, Support, Restraints, Support id and Support
  label, and the `S` row's Kind); 26 for the bends (three `B` codes, two Bend R values, three
  k, three k sources, twelve bend ids and three geometry sources); and the 9 typed node and
  pipe ids. At about 4.9 s per entry, 135 × 8/98 = 11.0 minutes. Still one queue/apply cycle:
  the bends are in the sheet's batch.
- **§3.5, 9 entries.** Per bend, the component selection and two SIFs. At 5.5 s per entry,
  9 × 5.5 s = 49.5 s, rounded to 0.75 minutes. One apply per bend, so three cycles.
- **§3.7, 20 entries.** RS-VS1's 20 of the earlier 98, as reconstructed here: five support fields (id, label, node,
  family, provenance) and fifteen hanger entries (the checkbox, the type, the stiffness DOF,
  value and unit, four quantities of value and unit, the source and the mechanics
  consumption). The other 78 were the ten supports now in the sheet: five fields each, 27
  restraint checkboxes and V060's "Create a new support". At 5.5 s per entry, 20 × 5.5 s =
  110 s, rounded to 1.75 minutes. One cycle.

Before G6, the #41 totals were: entries 757 − 98 + 135 − 39 + 9 − 98 + 20 = 686, plus the tee's Size unit entry
(review r1) = 687; cycles 51 − 6 + 3 − 11 + 1 = 38; minutes 77.25 − 8 + 11 − 5.5 + 0.75 − 13 +
1.75 = 64.25, plus 0.25 for the tee = 64.5.

G6 removes six end-size task edits (24 entries and six cycles), adding six size-unit
selections in the creation forms. The step estimate is 10 - 6 × 0.75 + 6 × 5.5/60 =
6.05 minutes, rounded to 6 under the existing quarter-minute convention. Thus 687 - 24
+ 6 = 669 entries; 38 - 6 = 32 cycles; 64.5 - 10 + 6 = 60.5 minutes. This accounts for
G6 only; it does not yet count G9 batching. The earlier totals above retain their
historical basis.

The estimate is about 61 minutes, about twice the provisional 30. It was about 77 before #41
(the tee, #45, adds 3), and about 130 before #38's fills and case copies (gaps G7, G8),
which brought the per-member load/reference state and pressure regions (§3.8, §3.10, §3.11)
from about 1,450 entries and 66.5 minutes to 168 entries and about 11.5 minutes; those entry
counts are the authoring lane's, measured by its round-trip test of the fixture (#38). #41's
id cells and numbering (gaps G1–G3) moved ten supports and the three bends into the sheet:
§3.4 to §3.7 went from about 27.25 minutes and 238 entries to about 14.25 minutes and 167
entries. The 40 typed ids became 9, and the support and bend cells add 67. The authoring lane's
round-trip test (#41) reports the 9 ids, the ten supports' ids and labels, and the twelve bend
ids with three geometry sources; the other cells are counted here from the code. The
component forms (§3.6),
about 18.75 minutes, are now the largest share; the run sheet itself is about 11.

## 6. Gaps found

Each is a step the desktop on `main` cannot do as the fixture records it, or does only at a
cost §5 shows. None is worked around outside the operation route.

| # | Gap | Evidence | Owner | What the person does |
|---|---|---|---|---|
| G1 | Closed by #41 (authoring lane I3): **Support id** and **Support label** cells in the "Ids and sources" group, so two supports at one node take their own ids; ten of the eleven supports are rows, with the fixture's ids and labels. RS-VS1 stays a form: no support code carries a hanger | `runSheetView.ts:29, 63-64, 461-468`; `runSheetModel.ts:22, 764-778` | DEL-07-02 | Support cells (§3.4); RS-VS1 by form (§3.7) |
| G2 | Closed by #41: **Bend start id**, **Bend end id**, **Bend arc id**, **Bend component id** and **Bend geometry source** cells on `B` rows, used by the bend insertion and the sheet's record projection. The bend labels stay fixed (§3.14) | `runSheetView.ts:65-69, 469-480`; `runSheetModel.ts:828-835, 852-856`; `runSheetBatch.ts:171-197` | DEL-07-02 | `B` rows (§3.4); SIFs by form (§3.5) |
| G3 | Closed by #41: **Pad numbers to N digits** and **Pipe number from** = its start node, with a stated fallback; a pipe id is fixed once shown | `RunSheet.tsx:557-563`; `runSheetModel.ts:287-298, 738-753`; `runSheetView.ts:300-311` | DEL-07-02 | Types 9 ids, each Pipe id first in its row (§2, §3.4) |
| G4 | Increments add in binary64, so RS-050 lands at 1.2999999999999998 m (as designed, §6.2) | `runSheetModel.ts:696-700` | DEL-07-02 | One X-position edit (§3.4) |
| G5 | Closed by #36 (authoring lane I1): the E/nu create form takes the material-level thermal expansion coefficient, and a wasm-engine test creates RS-M1 from the form equal to the fixture's record | `PressureAuthoringPanel.tsx:70` | DEL-07-03 | §3.2 step 1 |
| G6 | Closed by independent units for body length, End A size, End B size and the COG vector in both creation forms | DEL-03-05 `Work/component-quantity-units.md`; `rigidQuantityUnits.test.tsx` | DEL-03-05 | Six size-unit selections at creation; no later end-size edits (§3.6) |
| G7 | Closed by #38: "Fill … from a template" for reference-configuration members, element states and support states, "Add every primitive of this case" for load sources, and "Copy analysis state from another case" | `BulkFillControls.tsx:64-122`; `LoadReferenceStateInputs.tsx:79, 107, 147, 193, 228` | DEL-07-03 | §3.8, §3.11 |
| G8 | Closed by #38: "Copy pressure regions from another case", with ids re-derived | `PressureAuthoringPanel.tsx:174` | DEL-07-03 | §3.10 |
| G9 | Closed by #54: independent queued user intents can move to one reviewed atomic batch, with named refusals for stale dependencies and duplicate operation ids | DEL-07-02 `Work/queued-changes-batch.md`; `queuedChangesBatch.test.tsx` | DEL-07-02 | Optional batching under Review changes; §5 retains 32 per-intent cycles before batching, with no inferred time credit |
| G10 | Closed in DEL-09-01: tee creation requires a run size, header size, connection type and reinforcement reference; spec §5 now gives invented values, which EX does not read | `componentIntent.ts:380-392`; `operation_applier/src/lib.rs:3387-3446` | DEL-09-01 (spec §5) | §3.6 |
| G11 | No modifier-source entry for a tee: the inspector offers header and branch SIFs but not `modifiers.source_reference`, which it offers only on rigid components | `PropertyInspector.tsx:1920-2009, 2097-2105`; applier field rule `lib.rs:679-682` | DEL-03-05 | Leaves the SIF source unentered (§3.6) |

**RT-G2 candidates not named in spec §11.2:**
- a unit per quantity on the rigid-component form (G6, implemented; see DEL-03-05 `Work/component-quantity-units.md`);
- applying several queued intents as one reviewed batch (G9, implemented in #54);
- a modifier-source field for a tee, beside its SIFs (G11).
