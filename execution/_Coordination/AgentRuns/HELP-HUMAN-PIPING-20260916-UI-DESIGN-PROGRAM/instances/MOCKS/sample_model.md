# Sample model — Loop 4 header

**Sample.** One small fictional piping system used in every mock frame so that the nine states read as one project. Every number is a placeholder chosen to be plausible to a stress engineer; nothing here is engine output, and the result values are illustrative, not computed. Units: SI (mm, °C, bar, N, kN, kN·m, MPa, N/mm). Vertical axis Y. Engineer in the frames: R. Tufts. Project file: "Loop 4 header".

Pump P-401 discharge header to vessel V-402, with a branch to the header tie-in at N3.

## Node rows (the layout grammar)

One row per node; the row carries the element that arrives at the node (From → Node), its length as DX DY DZ from the From node, and the section, material and load set in force on that element. A value marked ↓ is propagated from the row above by connectivity; the rest were typed (entered). Node 140 is a branch: its From names the tee at 70.

| Node | From | Type | DX [mm] | DY [mm] | DZ [mm] | Section | Material | Load set | X, Y, Z [mm] | Note |
| ---: | ---: | --- | ---: | ---: | ---: | --- | --- | --- | ---: | --- |
| 10 | — | — | 0 | 0 | 0 | — | — | — | 0, 0, 0 |  |
| 20 | 10 | Pipe | 3000 | 0 | 0 | P1 | CS-A | OP1 | 3000, 0, 0 |  |
| 30 | 20 | Bend | 1500 | 0 | 0 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 0, 0 | bend R 330 |
| 40 | 30 | Pipe | 0 | 2500 | 0 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 2500, 0 |  |
| 50 | 40 | Bend | 0 | 1500 | 0 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 4000, 0 | bend R 330 |
| 60 | 50 | Pipe | 0 | 0 | 1200 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 4000, 1200 |  |
| 70 | 60 | Pipe | 0 | 0 | 1300 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 4000, 2500 |  |
| 80 | 70 | Pipe | 0 | 0 | 1400 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 4000, 3900 |  |
| 90 | 80 | Valve | 0 | 0 | 600 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 4000, 4500 | valve, 1200 N |
| 100 | 90 | Reducer | 0 | 0 | 400 | P1 ↓ | CS-A ↓ | OP1 ↓ | 4500, 4000, 4900 | reducer P1→P2 |
| 110 | 100 | Bend | 0 | 0 | 1600 | P2 | CS-A ↓ | OP1 ↓ | 4500, 4000, 6500 | bend R 250 |
| 120 | 110 | Pipe | 0 | −1500 | 0 | P2 ↓ | CS-A ↓ | OP1 ↓ | 4500, 2500, 6500 |  |
| 130 | 120 | Pipe | 0 | −1500 | 0 | P2 ↓ | CS-A ↓ | OP1 ↓ | 4500, 1000, 6500 |  |
| 140 | 70 | Pipe | 2000 | 0 | 0 | P3 | CS-B | OP2 | 6500, 4000, 2500 | branch from the tee at 70 |
| 150 | 140 | Bend | 1000 | 0 | 0 | P3 ↓ | CS-B ↓ | OP2 ↓ | 7500, 4000, 2500 | bend R 170 |
| 160 | 150 | Pipe | 0 | −3000 | 0 | P3 ↓ | CS-B ↓ | OP2 ↓ | 7500, 1000, 2500 |  |

Bends are at nodes 30, 50, 110 and 150 (bend nodes carry their radius); the valve is element 80–90 (1 200 N); the reducer 90–100 goes from P1 to P2; the welding tee at 70 is node data, not a row.

## Sections

| Section | Nominal | OD [mm] | Wall [mm] | Corrosion [mm] | Mill tol. [%] | Insulation [mm] | Bend R [mm] | Origin |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| P1 | DN200 | 219.1 | 8.2 | 1.5 | 12.5 | 50 | 330 | Imported · sections.csv · 2026-09-16 |
| P2 | DN150 | 168.3 | 7.1 | 1.5 | 12.5 | 50 | 250 | Imported · sections.csv · 2026-09-16 |
| P3 | DN100 | 114.3 | 6 | 1.5 | 12.5 | 40 | 170 | Imported · sections.csv · 2026-09-16 |

## Materials

| Material | Description | E 20 °C [MPa] | E 150 °C [MPa] | α [µm/m·K] | Density [kg/m³] | Sc [MPa] | Sh [MPa] | Source |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| CS-A | Carbon steel A (sample) | 203000 | 196000 | 12 | 7850 | 137.9 | 137.9 | user library cs-a.mlb · provenance recorded |
| CS-B | Carbon steel B (sample) | 203000 | 196000 | 11.7 | 7850 | 128 | 128 | not recorded (provenance warning) |

CS-B's provenance is deliberately unrecorded: it produces the one Provenance warning (node 140) that stays open through states 4 to 9.

## Load sets (read through on the node row)

| Set | T1 [°C] | P1 [bar] | T2 [°C] | P2 [bar] | Note |
| --- | ---: | ---: | ---: | ---: | --- |
| OP1 | 150 | 12.00 | 40 | 12.00 | operating / standby |
| OP2 | 80 | 6.00 | 40 | 6.00 | branch, operating / standby |

## Restraints

| Node | Tag | Type | Direction | Gap [mm] | μ | Stiffness | Library | Max variation [%] | Origin | Note |
| ---: | --- | --- | --- | ---: | ---: | --- | --- | ---: | --- | --- |
| 10 | N1 | Anchor | all | — | — | rigid | — | — | entered | pump P-401 discharge nozzle |
| 20 | RS-01 | +Y | Y | 3 | 0.30 | rigid | — | — | entered | resting support, 3 mm as surveyed |
| 60 | H1 | Variable spring | Y | — | — | designed | Vendor-A springs | 25 | accepted from P-09 | to be designed |
| 80 | RS-02 | Rigid (Y) | Y | 0 | — | rigid | — | — | entered | rod from steel |
| 120 | G-01 | Guide | X, Z | 2 | 0.30 | rigid | — | — | entered |  |
| 130 | N2 | Anchor | all | — | — | rigid | — | — | entered | vessel V-402 nozzle |
| 160 | N3 | Anchor | all | — | — | rigid | — | — | entered | header tie-in |

## Loads

| Node | Kind | Direction | Value | Unit | Case | Note |
| ---: | --- | --- | --- | --- | --- | --- |
| 40 | Force | Y | −2000 | N | W | instrument bridle weight |
| 130 | Displacement | Y | 4 | mm | T1 | vessel growth at the nozzle |
| 130 | Displacement | Y | 1.2 | mm | T2 | vessel growth, standby |

Seismic: SE1 = 0.3 g in X (static, user-supplied factor).

## Node data

| Node | Kind | Value | Detail |
| ---: | --- | --- | --- |
| 70 | Branch connection | Welding tee | SIF from rule pack sample-rules 1.2 |
| 90 | Flange | WN | weight 220 N |

## Rule pack (sample)

`sample-rules 1.2` · sha256:9b1c4e02… · user · private · invented values for this sample, no code content. The rule expressions below are rendered display-only in the product; the allowables are invented for this sample.

| Rule | Name | Expression (display only) | Allowable CS-A [MPa] | Allowable CS-B [MPa] |
| --- | --- | --- | ---: | ---: |
| SUS-A1 | Sustained | S_L ≤ S_h | 137.9 | 128 |
| EXP-A1 | Expansion range | S_E ≤ f · (1.25 S_c + 0.25 S_h) | 206.8 | 192 |
| OCC-A1 | Occasional | S_L + S_O ≤ 1.33 S_h | 183.4 | 170.2 |

## Load cases

Generated from the rule pack on 2026-09-17 10:05, one row edited and one authored:

| Case | Expression | Stress type | Rule | Origin |
| --- | --- | --- | --- | --- |
| SUS | W + P1 | Sustained | SUS-A1 | generated · sample-rules 1.2 |
| OPE1 | W + P1 + T1 | Operating | — | generated · sample-rules 1.2 |
| OPE2 | W + P2 + T2 | Operating | — | generated · sample-rules 1.2 |
| EXP1 | OPE1 − SUS | Expansion | EXP-A1 | edited by R. Tufts · was T1 (generated) |
| EXP2 | T2 | Expansion | EXP-A1 | generated · sample-rules 1.2 |
| OCC1 | W + P1 + SE1 | Occasional | OCC-A1 | authored by R. Tufts |

## Runs

| Run | When | Outcome | Settings | Note |
| --- | --- | --- | --- | --- |
| Run 01 | 2026-09-16 17:48 | solved | — | weight only, superseded by model changes |
| Run 02 | 2026-09-17 15:02 | failed | S-02 · iteration limit 50 | nonlinear support at node 20 did not converge after 50 iterations |
| Run 03 | 2026-09-17 15:21 | solved | S-03 · iteration limit 200 · converged in 74 iterations | 6 cases · hanger design pass · rules checked |
| Run 04 | 2026-09-18 09:12 | solved | S-03 | after P-12 row 1 (variable spring at 80) |

Run 03 is the solved run of state 7 (evidence label INTERNALLY_VERIFIED, sha256:4df0f798…, solver 0.2.0); Run 02 is the failed run of state 6; Run 04 is the run the Review page (state 9) reports after proposal P-12 row 1 was accepted.

## Results of Run 03 (state 7) — sample values

Stresses at the To node of each element for case EXP1, sorted by ratio; the ratio is stress over the rule allowable for the element's material, and every ratio is shown with its rule ID and pack version.

| Node | Element | Case | Stress [MPa] | Allowable [MPa] | Ratio | Rule | Pack |
| ---: | --- | --- | ---: | ---: | ---: | --- | --- |
| 70 | 60–70 | EXP1 | 148.9 | 206.8 | 0.72 | EXP-A1 | 1.2 |
| 50 | 40–50 | EXP1 | 131.0 | 206.8 | 0.63 | EXP-A1 | 1.2 |
| 30 | 20–30 | EXP1 | 118.3 | 206.8 | 0.57 | EXP-A1 | 1.2 |
| 80 | 70–80 | EXP1 | 96.2 | 206.8 | 0.47 | EXP-A1 | 1.2 |
| 110 | 100–110 | EXP1 | 89.6 | 206.8 | 0.43 | EXP-A1 | 1.2 |
| 140 | 70–140 | EXP1 | 77.3 | 192.0 | 0.40 | EXP-A1 | 1.2 |
| 40 | 30–40 | EXP1 | 64.1 | 206.8 | 0.31 | EXP-A1 | 1.2 |
| 60 | 50–60 | EXP1 | 58.8 | 206.8 | 0.28 | EXP-A1 | 1.2 |
| 120 | 110–120 | EXP1 | 52.4 | 206.8 | 0.25 | EXP-A1 | 1.2 |
| 130 | 120–130 | EXP1 | 47.9 | 206.8 | 0.23 | EXP-A1 | 1.2 |
| 150 | 140–150 | EXP1 | 42.6 | 192.0 | 0.22 | EXP-A1 | 1.2 |
| 90 | 80–90 | EXP1 | 33.5 | 206.8 | 0.16 | EXP-A1 | 1.2 |
| 100 | 90–100 | EXP1 | 30.1 | 206.8 | 0.15 | EXP-A1 | 1.2 |
| 20 | 10–20 | EXP1 | 26.4 | 206.8 | 0.13 | EXP-A1 | 1.2 |
| 160 | 150–160 | EXP1 | 21.7 | 192.0 | 0.11 | EXP-A1 | 1.2 |

Envelope (governing case per element):

| Node | Element | Governing case | Stress [MPa] | Allowable [MPa] | Ratio | Rule | Pack |
| ---: | --- | --- | ---: | ---: | ---: | --- | --- |
| 70 | 60–70 | EXP1 | 148.9 | 206.8 | 0.72 | EXP-A1 | 1.2 |
| 50 | 40–50 | EXP1 | 131.0 | 206.8 | 0.63 | EXP-A1 | 1.2 |
| 30 | 20–30 | EXP1 | 118.3 | 206.8 | 0.57 | EXP-A1 | 1.2 |
| 80 | 70–80 | OCC1 | 92.5 | 183.4 | 0.50 | OCC-A1 | 1.2 |
| 40 | 30–40 | OCC1 | 81.0 | 183.4 | 0.44 | OCC-A1 | 1.2 |
| 110 | 100–110 | EXP1 | 89.6 | 206.8 | 0.43 | EXP-A1 | 1.2 |
| 140 | 70–140 | EXP1 | 77.3 | 192.0 | 0.40 | EXP-A1 | 1.2 |
| 20 | 10–20 | OCC1 | 70.2 | 183.4 | 0.38 | OCC-A1 | 1.2 |
| 90 | 80–90 | OCC1 | 69.0 | 183.4 | 0.38 | OCC-A1 | 1.2 |
| 60 | 50–60 | OCC1 | 55.4 | 183.4 | 0.30 | OCC-A1 | 1.2 |
| 100 | 90–100 | OCC1 | 47.7 | 183.4 | 0.26 | OCC-A1 | 1.2 |
| 120 | 110–120 | EXP1 | 52.4 | 206.8 | 0.25 | EXP-A1 | 1.2 |
| 130 | 120–130 | EXP1 | 47.9 | 206.8 | 0.23 | EXP-A1 | 1.2 |
| 150 | 140–150 | OCC1 | 39.2 | 170.2 | 0.23 | OCC-A1 | 1.2 |
| 160 | 150–160 | OCC1 | 34.2 | 170.2 | 0.20 | OCC-A1 | 1.2 |

Governing: node 70, element 60–70, EXP1, 148.9 / 206.8 MPa = 0.72 · EXP-A1 · pack 1.2.

Restraint loads used in the story (Run 03): node 80 W +4.18 kN, OPE1 −9.8 kN; node 130 OPE1 7.4 kN, Mz 3.1 kN·m. OPE1 vertical displacements: node 40 +3.9, 50 +6.2, 60 +5.9, 70 +4.1, 80 0.0, 120 +4.3, 130 +4.0 mm.

## Hanger design (state 8)

Library: Vendor-A springs (user import · source recorded in Libraries, 24 sizes, imported 2026-09-15).

| Node | Tag | Type | Design load [N] | Travel [mm] | Library | Size | Rate [N/mm] | Cold load [N] | Hot load [N] | Variation [%] |
| ---: | --- | --- | ---: | ---: | --- | --- | ---: | ---: | ---: | ---: |
| 60 | H1 | Variable spring | 5980 | +5.9 | Vendor-A springs | A-3 | 120 | 6688 | 5980 | 11.8 |

## Proposals

**P-09 — Add a variable spring hanger at node 60** (2026-09-17 11:40, accepted)

| Row | Table | Node | Field | Old | New |
| ---: | --- | ---: | --- | --- | --- |
| 1 | Restraints | 60 | Type | — | Variable spring |
|  |  |  | Library | — | Vendor-A springs |
|  |  |  | Max variation | — | 25 % |
Rationale: The top of the riser at 60 rises about 6 mm at T1 with no vertical support between 20 and 80.

**P-12 — Replace the rigid support at node 80 with a variable spring** (2026-09-17 16:22)
Asked by R. Tufts at 16:20: "Nozzle load at 130 looks high in OPE1. Suggest support changes."

| Row | Table | Node | Field | Old | New |
| ---: | --- | ---: | --- | --- | --- |
| 1 | Restraints | 80 | Type | Rigid (Y) | Variable spring |
|  |  |  | Library | — | Vendor-A springs |
|  |  |  | Max variation | — | 25 % |
|  |  |  | Design load | — | 4 180 N |
|  |  |  | Travel | — | TBD |
| 2 | Restraints | 20 | Gap | 3 mm | 0 mm |
Rationale: The rigid rod at 80 holds the header down against the riser's thermal rise (about 6 mm at T1): OPE1 reaction −9.8 kN against +4.18 kN in W, and 7.4 kN on the vessel nozzle (130). A spring set at the W load carries the weight and lets the header rise; closing the gap at 20 lets the +Y support carry weight in OPE1 too.

Constraints considered: Design load: Run 03 load at 80 in W, 4.18 kN; Travel: TBD until a run with the spring; Library: Vendor-A springs (the only one); The gap at 20 is recorded as surveyed.

TBD: Travel at 80 · needs a run with the spring; Keep the gap at 20 as surveyed? (row 2).

In the frames, P-12 row 1 is accepted at 16:31 (state 8, Table) and row 2 is still pending on the Review page (state 9).

## Run 04 envelope (state 9, the Review page's live table)

| Node | Element | Governing case | Stress [MPa] | Allowable [MPa] | Ratio | Rule | Pack |
| ---: | --- | --- | ---: | ---: | ---: | --- | --- |
| 70 | 60–70 | EXP1 | 119.9 | 206.8 | 0.58 | EXP-A1 | 1.2 |
| 50 | 40–50 | EXP1 | 112.4 | 206.8 | 0.54 | EXP-A1 | 1.2 |
| 30 | 20–30 | EXP1 | 103.8 | 206.8 | 0.50 | EXP-A1 | 1.2 |
| 80 | 70–80 | OCC1 | 84.7 | 183.4 | 0.46 | OCC-A1 | 1.2 |
| 40 | 30–40 | OCC1 | 79.3 | 183.4 | 0.43 | OCC-A1 | 1.2 |
| 110 | 100–110 | EXP1 | 76.1 | 206.8 | 0.37 | EXP-A1 | 1.2 |

Sample. These values illustrate the screens; they are not the output of any solver or rule check.
