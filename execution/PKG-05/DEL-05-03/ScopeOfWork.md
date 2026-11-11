---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-03
package_id: PKG-05
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-015]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-05-03 Fundamental stress recovery module

## Purpose and Objective Traceability

This deliverable turns the forces and moments of a solved model into the stresses an engineer publishes. At the ends of every member and at stations along it, it gives the axial, bending and torsional stresses in the pipe wall and, where the model carries pressure, the membrane, hoop and radial stresses of the pressurized wall, with their extremes around the section and their maximum along the member. It also defines the cross-section those stresses are taken on: the wall as specified, and the wall reduced by mill tolerance and corrosion allowance, with every stress published on both. And it defines how a straight pipe accounts for shear deformation. These are fundamental mechanics quantities. Which of them a design code limits, and to what value, is for the user's rule pack.

It serves SOW-015 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-05, row DEL-05-03: does not encode code stress equations), and PRD §12.7 and §22.3 with FR-SOL-003 and FR-SOL-008. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. Fundamental stresses are among the results a user's engineer issues under their own responsibility, and the stiffness and section definitions stated here are part of every analysis of criterion 1. It carries two requirements: stresses are published on the nominal and the reduced section, and the product chooses no governing one; and the shear model is an explicit selection whose authored default is the Timoshenko straight pipe with the energy-matched annular shear factor.

- **OUT-001** — Fundamental stress recovery: the stress definitions, the section bases, the shear model, the section quantities and member maxima, and their findings.

## Deliverable Definition — Ontology

**Stresses of the wall.**

- *Sign convention.* Actions act on the positive-x cut face of the member's local frame: N tensile, T about +x, M_y and M_z by the right-hand rule, so that σ_x(y, z) = N/A + M_y·z/I − M_z·y/I.
- *Component stresses.* Axial normal σ = N/A; bending normal about each local axis, M_y/Z and M_z/Z; torsional shear τ = T·r_o/J. A, I, J = 2I and Z = I/r_o are those of the section basis, and r_o is half the outside diameter.
- *Wall force and effective force.* On a pressurized straight member the wall carries N_w = N_m + 2ν·p·A_i and the effective axial force is S = N_m + (2ν − 1)·p·A_i, with N_m the recovered mechanical force and A_i the area of the pressure bore. Both are formed once, not per basis.
- *Pressure-derived stresses.* On each basis: the axial membrane stress σ_m = N_w/A_b; and the closed thick-wall annulus solution of that basis's own section under internal pressure p at its inner radius r_i,b, with radial stress −p at the inner surface and 0 at the outer, and hoop stress 2p·π·r_i,b²/A_b at the outer surface and that plus p at the inner. On a realized arc the hoop is the toroidal membrane hoop at the intrados, crown and extrados, with the radial surface values. On the nominal basis the membrane stress carries the end thrust of the pressure bore, so it is not half the hoop stress; results say so.
- *Stations.* A member's two ends and the stations asked for along it, each with an identity and a fraction of the length in [0, 1]. A station keeps its identity and fraction, and a sweep keeps the order it was asked in. On a realized arc the actions at a station are in the arc's tangent frame there.
- *Section extremes.* At one station and basis, the largest and smallest normal stress around the outer surface, σ_N ± hypot(σ_y, σ_z), with the fibre at which each occurs.
- *Transverse-shear maximum.* At one station and basis, τ_v = c_b·hypot(V_y, V_z), with c_b = ((3 + 2ν)·r_o² + (1 + 2ν)·r_i,b²)/(4(1 + ν)·I_b), the Saint-Venant solution for the annulus.
- *Intensified bending.* At a station of a component that carries the user's directional stress-intensification factors i_ip and i_op, hypot(i_ip·M_z, i_op·M_y)/Z, with each factor on the moment of its own plane. It has no axial or torsional term, the factors are the user's, and it is never combined across cases.
- *Elastic normal-stress maximum.* For a member and basis, the maximum over the whole span of |σ_axial| + hypot(M_y, M_z)/Z. It is published as an enclosure: a value and a bound that contain the maximum of the member's stress field between stations, not the largest sampled station.
- *Headline.* For a case and basis, the largest elastic normal-stress maximum among the members whose rows are eligible, with the member it occurs on. Rows that a component marks as reference-section results of a scaled body (DEL-03-05) are published but are not part of any maximum or headline. It is withheld when that basis's stress rows are incomplete.
- *Stress range.* The component-wise absolute difference of two recovered stress states.

**Section bases.** On the exact route a pipe with outside diameter D, wall t, mill tolerance c_m and corrosion allowance c_a (c_m, c_a ≥ 0, c_m + c_a < t) has:

| Name | Wall | Inner radius | Used for |
|---|---|---|---|
| Nominal section | t | D/2 − t | stiffness, the Timoshenko shear area, pipe metal mass, nominal-basis stresses |
| Reduced section | t_r = (t − c_m) − c_a | D/2 − t_r | reduced-basis stresses |
| Pressure bore | — | D/2 − t_r | the pressure end-thrust area A_i, contents mass |

The binary64 order is as written: t_r = (t − c_m) − c_a, then the radius. Corrosion allowance is required on every pipe section of an exact-route document, zero included, and is refused on a pressure-free document. A pressure-free document keeps one section, the wall less mill tolerance, for stiffness, mass and stress alike.

**Shear model.** A document selects `euler_bernoulli` or `timoshenko_annulus_energy_v1` for all its straight members: on the exact route in the pressure contract, where the selection is required; on the pressure-free route in the model settings, where absence means Euler–Bernoulli. Realized arcs are Euler–Bernoulli under either selection and say so in their results. For the Timoshenko selection, from the stiffness section's inner and outer radii and the member's Poisson's ratio ν:

```
m = r_i/r_o;  m2 = m*m;  a = 1 + m2;  b = 1 - m2;  n1 = 1 + nu
c = (8*nu + 14)*nu + 7
D = c*(b*b) + 48*(m2*(n1*n1))
kappa_E = (6*(n1*n1))*(a*a) / D
A_s = kappa_E * A;   phi = 12*E*I / (G*A_s*L*L)
```

in that operation order, without fused multiply-add. κ_E is the energy-matched shear factor of the annulus; it is not 5/6 and not Cowper's. On the exact route ν is the member's selected value. On the pressure-free route, where E and G are authored, ν = E/(2G) − 1, and the factor is formed from the moduli without forming 1 + ν. A published nodal rotation of a Timoshenko member is the section's rotation, not the slope of the centreline.

**Boundary.** This deliverable defines stresses, sections and the shear selection. The following belong elsewhere:

- the frame element's stiffness, which is formed from the shear definition above, and the kernel's integrity check: DEL-04-01. A change to the definition changes that element with it;
- a member's end actions and station resultants, and the Timoshenko behaviour of member loads: DEL-04-02;
- the area, second moment, section modulus and torsion constant of an annulus as formulas, and mass per length: DEL-03-08. The product's computation of them, the pressure loads, the solve and the publication of every row: DEL-04-07;
- the desktop's pressure authoring, whose code DEL-07-03 lists, and its model-settings editor, whose code DEL-02-01 lists, are where a document's shear selection is written. The default written there and the authoring clause of AC-004 remain this deliverable's;
- a verified bound on a published stress: DEL-04-08. Ranges and combinations across load cases as published results: DEL-05-02;
- the stress rows' place in the result and export formats: DEL-08-04. The stress benchmark suite: DEL-09-02. The local-analysis handoff: DEL-10-03;
- stress-intensification and flexibility factors as component data: DEL-03-03, DEL-03-04. Pressure-dependent corrections of those factors are outside the product;
- code stress equations, stress categories, allowables and the choice of a governing basis: the user's rule pack (PKG-06).

**Not decided.** Whether the optional principal, von Mises and Tresca equivalent stresses of PRD §12.7 are recovered. They are not among the quantities above.

## Completion and Reliance Basis — Epistemology

A recovered stress is a statement about the discrete model's forces and the section the user entered. Its correctness is examined against closed forms and references made without product code, and for the quantities a reader must reproduce, bit for bit.

### Requirements

- **REQ-001** — The module recovers fundamental mechanics stresses from force resultants, moments, pressure and section properties: axial, bending and torsional stresses on every member and, on a pressurized member, the pressure-derived membrane, hoop and radial stresses (SOW-015; PRD §12.7; FR-SOL-008).
- **REQ-002** — The module is mechanics only. It encodes no code stress category, allowable, stress index or compliance decision, and no output of it states or implies one (the decomposition's row; `docs/CONTRACT.md` OPS-K-MECH-2, OPS-K-RULE-1, OPS-K-IP-1).
- **REQ-003** — Inputs and outputs are unit-aware and dimensionally checked across force, moment, pressure, section-property and stress quantities (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-004** — A missing, non-finite or non-positive input that recovery requires is a finding that names it. Nothing is defaulted (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-AGENT-1).
- **REQ-005** — A recovered stress keeps its source and provenance, is recovered only from a mechanics-solved result, carries the human-review notice, and never carries certification, compliance or human approval (decomposition AB-00-03, AB-00-06; PRD §21.2).
- **REQ-006** — Recovery is deterministic: the same actions and section give the same stresses bit for bit, so every check of it is deterministic (`docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-007** — Adapters, reports and rule packs take stresses from the published mechanics result. None recomputes a stress on a definition of its own or reaches around the result (decomposition AB-00-02; `docs/CONTRACT.md` OPS-K-MECH-2).
- **REQ-008** — Station stresses keep the station's identity, and a sweep of stations keeps the order it was asked in (SOW-015).
- **REQ-009** — A stress range is the component-wise difference of two recovered states, formed only when both states are complete and unblocked. It is not a fatigue, equivalent-stress or code-range result (SOW-015; PRD §22.3).
- **REQ-010** — *Section bases.* On the exact route every stress is recovered on the nominal section and on the reduced section, each result names its basis, each basis has its own member maxima and its own headline, and the product names no governing basis. Both are published where they coincide. The bases are formed in the stated binary64 order, so that a reader forms the same sections from the published inputs.
- **REQ-011** — *Shear model.* Straight members are Euler–Bernoulli or Timoshenko as the document selects, with the factor, shear area and Poisson's ratio defined above; realized arcs are Euler–Bernoulli and disclose it; and a results record names each member's theory and, for Timoshenko, its factor and shear area. A missing selection on the exact route, a selection where it does not belong, a missing ν or G, or a ν outside −1 < ν < 1/2 is refused by name. No default factor exists. An exact-route document that the product authors without an explicit choice carries `timoshenko_annulus_energy_v1`; a pressure-free document's selection is never written unless the user makes one.
- **REQ-012** — *Section quantities and maxima.* The section extremes, the transverse-shear maximum, the toroidal hoop and radial values and the intensified bending are each a fixed sequence of correctly rounded binary64 operations, so that a reader recomputes each published value bit for bit from the published rows. The elastic normal-stress maximum of a member contains the maximum of the member's stress field over its whole span (PRD §12.7, §12.8; FR-SOL-008).

### Acceptance criteria

- **AC-001** — For known end actions on a straight member, the axial, bending and torsional stresses at the ends and at every station equal N/A, M_y/Z, M_z/Z and T·r_o/J at the analytic tolerance (1e-9 relative, with a stated absolute floor near zero), with the signs of the stated convention, on both routes.
- **AC-002** — On a pressurized straight member the radial and hoop stresses at the inner and outer surfaces equal the closed thick-wall solution of each basis's own annulus, and the membrane stress equals N_w/A_b with N_w formed on the pressure bore. On a realized arc the hoop and radial values agree with their independent references at the intrados, crown and extrados.
- **AC-003** — On an exact-route document every stress row appears once per basis, at the same member, station and component, each naming its basis. The reduced wall and radius have the bits of the stated order, and a reader forms both sections from the published dimensions bit for bit. Each basis has its own maximum and headline, and no result names a governing basis. A row marked as a reference-section result is published and is excluded from its basis's maximum and headline. With zero mill tolerance and corrosion allowance the two bases give the same values. A pressure-free document publishes one section and no pressure stress.
- **AC-004** — κ_E and A_s have the bits of the stated order against values computed in exact rational arithmetic. A Timoshenko member's deflections, rotations and end actions agree with the closed-form Timoshenko beam for tip load, distributed load and prescribed movement, and an Euler–Bernoulli twin differs from it by the closed-form shear term. A realized arc between Timoshenko members is Euler–Bernoulli and says so. Each refusal of REQ-011 is raised with its code and names the document, member or case; a pressure-free document with no selection gives the results of an explicit Euler–Bernoulli selection. Authoring an exact-route document without a choice writes `timoshenko_annulus_energy_v1`, and authoring never writes a pressure-free `model_settings.shear_model` unprompted.
- **AC-005** — Each section quantity has the bits of its stated sequence against values computed in exact rational arithmetic, including a toroidal hoop one unit in the last place from the edge of its domain. A member's published maximum and bound contain the exact maximum of its closed-form stress field for uniform and partially loaded spans, including a maximum that falls between stations.
- **AC-006** — A stress range equals the component-wise absolute difference of its two states, and a range of a blocked or incomplete state is refused with a finding.
- **AC-007** — A missing resultant or section property, a non-finite value, a non-positive area or modulus, a quantity whose dimension does not fit its use, or a station fraction outside 0 to 1 is reported as a finding that names it, and no stress is recovered from it. A sweep returns its stations in the order asked, each with its identity.
- **AC-008** — Recovery refuses a result that is not mechanics solved and adds the human-review notice to what it recovers. The module's sources, tests and fixtures contain no code stress equation, allowable, stress-intensification or flexibility table or pipe table and only invented values, and no message or record states or implies approval or compliance.
- **AC-009** — Repeated recovery from the same result gives bit-identical stresses, section quantities and maxima.

## Production and Verification Method — Praxeology

**Method choices that define the module.**

- Stresses are closed-form functions of the published actions and the section; nothing is fitted or interpolated between stations. A member's maximum is found from the polynomial stress field between stations, not by sampling more stations.
- The exact route forms its three section geometries once per pipe and uses them by name; no later step forms a wall from the dimensions again.
- A quantity a reader must reproduce is defined by its operation sequence, not by a formula and a tolerance.
- The shear selection is per document, not per member.
- The closed forms of the shear factor are the oracle for the Timoshenko member. No published benchmark is formulation-matched to κ_E.

**How each criterion is examined.**

- **VER-001** — Comparison with named references (closed-form stress formulas and the hand calculations under `validation/hand_calcs/stress/`: axial, bending and torsional stress, the station sweep and the stress range): stresses recovered from known actions on both routes, for AC-001 and AC-006.
- **VER-002** — Comparison with named references (the thick-wall annulus closed form; `validation/references/t4_i8` and `validation/references/t4_u6` for the two bases; `validation/references/t4_u4b` for arcs): pressure-derived stresses and the per-basis rows, maxima and headlines, with an executable check that a reader forms both sections from the published dimensions and that a row marked as a reference-section result is published and left out of its basis's maximum and headline.
- **VER-003** — Comparison with named references (the SHEAR_REFERENCE closed forms, private `sgttomas/chirality@f2211ecef6`; `validation/references/t4_u8`, `validation/references/kd5_timoshenko` and `validation/references/pf_timoshenko`): the factor's bits, Timoshenko members against closed forms and their Euler–Bernoulli twins on both routes and in both solver modes, with an executable check of each refusal and disclosure and of the selection that authoring writes.
- **VER-004** — Comparison with a named reference (the frozen bits of `core/loads/stress_recovery/tests/u4b_section_bits/`, computed in exact rational arithmetic by a generator that imports no product code) for the section quantities; and comparison of each member maximum and bound with the exact maximum of the closed-form stress field.
- **VER-005** — Executable check: observes each finding of AC-007 with the input it names, the absence of a stress for it, the order and identity of swept stations, the refusal of a result that is not mechanics solved, and bit-identical output on repeated recovery.
- **VER-006** — Judgment by a named role (an independent reviewer who did not write the code): examines the module's sources, tests, fixtures and messages against PRD §20.1, §21.2 and `docs/claims_registry.md`, with the protected-content scan's result for the same files.

## Governing Values and Decisions — Axiology

- **AX-001** — *Mechanics, not code stress.* Where a quantity could be shaped to a code's equation or category, the plain mechanics quantity is the better one. A user's convenience never justifies a built-in code formula, allowable or governing choice.
- **AX-002** — *Reproducible to the bit.* Better is more published stress quantities that a reader can recompute exactly from the published rows. Speed or a shorter formula never justifies a sequence the reader cannot follow.
- **AX-003** — *A maximum that cannot be missed.* Better is a tighter bound on a member's maximum stress. Tightness is never bought by a value that could lie below the true maximum.
- **AX-004** — *Nothing assumed.* Better is fewer values the module could supply by itself; the target is none. No section property, corrosion allowance, shear factor or Poisson's ratio is supplied to make a stress recoverable.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-015 OBJ-003 | REQ-001 REQ-003 | AC-001 | VER-001 | Outside source: closed-form stress formulas and the hand calculations under `validation/hand_calcs/stress/`, which import no product code |
| OUT-001 | SOW-015 OBJ-003 | REQ-001 | AC-002 | VER-002 | Outside source: the thick-wall annulus closed form and `validation/references/t4_i8`, `t4_u6` and `t4_u4b`, generated without product code |
| OUT-001 | SOW-015 OBJ-003 | REQ-010 | AC-003 | VER-002 | Outside source `validation/references/t4_u6` for the values. The reader forming the sections again compares the product with its own published inputs and only guards against change. The exclusion of a marked row is stated by the criterion |
| OUT-001 | SOW-015 OBJ-003 | REQ-011 | AC-004 | VER-003 | Outside source: the SHEAR_REFERENCE closed forms and `validation/references/t4_u8`, `kd5_timoshenko` and `pf_timoshenko`, generated without product code |
| OUT-001 | SOW-015 OBJ-003 | REQ-012 | AC-005 | VER-004 | Outside source: bits computed in exact rational arithmetic by a generator that imports no product code, and the exact maximum of a closed-form stress field |
| OUT-001 | SOW-015 OBJ-003 | REQ-009 | AC-006 | VER-001 | Outside source: `validation/hand_calcs/stress/stress_range.md`; the criterion states the refusal |
| OUT-001 | SOW-015 OBJ-003 | REQ-004 REQ-008 | AC-007 | VER-005 | The criterion states the expected finding for each defect; PRD §5.5 says that none is defaulted |
| OUT-001 | SOW-015 OBJ-003 | REQ-002 REQ-005 REQ-007 | AC-008 | VER-005 VER-006 | PRD §20.1 and §21.2 list what must not appear, and DEL-05-04 states what a recovered result may carry |
| OUT-001 | SOW-015 OBJ-003 | REQ-006 | AC-009 | VER-005 | The product's own earlier output: it guards against change and shows nothing about correctness |
