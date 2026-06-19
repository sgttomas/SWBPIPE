# D-18 — Component Macro-Element Realization

**Status:** AWAITING_RULING — packet drafted; no ruling recorded. Only the human project authority rules.
**Prepared:** 2026-06-18 by decision-preparation subscope, requested for the Phase D lead-up.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-18.
**Plan basis:** `plans/PLAN_2026-06-17_prd_completion.md` §2.3 row D-18 (`:94`); Phase D rows D1–D4 (`:585–588`), D8 (`:592`), phase-exit evidence (`:595`).
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

Citations pinned at repo HEAD `1c27081c5`. Symbol names are the durable anchors if line numbers drift.

---

## 1. Decision statement and scope

**Decide:** how the four R4 piping-component families — **bend** (DEL-03-03), **branch connection** (DEL-03-04), **rigid/semi-rigid** (DEL-03-05), and **expansion joint** (DEL-03-06) — are *realized in the solver*: as **solver macro-elements** (a bend-flexibility element, a rigid/semi-rigid element, an EJ stiffness element with their own assembled stiffness contributions) or as **straight-element geometry plus user-flexibility multipliers / modifiers applied in stress recovery**, or a **hybrid** of the two per family.

The component schema already permits both consumption modes per family: `ComponentMechanicsInterface.solver_consumption` enumerates `mechanics_geometry_only` and `mechanics_geometry_and_user_flexibility` (plus `not_solver_consumed`, `TBD`) (`schemas/component.schema.yaml:497–505`). D-18 selects which mode each family adopts and therefore what solver/stress-recovery code Phase D builds.

**In scope:** the realization mechanism per component family; where the user-entered factors (SIFs, flexibility factors, stiffnesses) enter the computation (assembled stiffness vs. stress recovery); the code-neutral-boundary posture of each mechanism; the implementation sequencing implication for D1–D4.

**Out of scope:** the *values* of any SIF / flexibility factor / stiffness — these are **user-entered** and never computed by the software (the binding boundary, §2.4; not reopened here). The assembled nonlinear solve loop and convergence tolerance (D-16 / D-19, the R4 hinge — D6). Spring-hanger scope (D-15 — D5). Sparse-solver adoption timing (D-17 — D7). The report renderer that surfaces provenance (D8 / DEL-08-01, DEL-08-03 — depends on its own maturity, not on this ruling's mechanism).

**What this blocks.** Phase D items **D1 (bend), D2 (branch), D3 (rigid), D4 (expansion joint)** each carry `D-18` as their decision gate (`plans/PLAN_2026-06-17_prd_completion.md:585–588`; D3's row carries no gate but is the rigid family this decision still characterizes). Through them it blocks the **R4 exit criteria** verbatim — "Nonlinear support validation cases converge" and "Component provenance appears in reports" (`docs/PRD.md:1236–1237`) — and the half **"component provenance appears in reports"** routed through D8 (`plans/PLAN_2026-06-17_prd_completion.md:592, 595`). It closes FR-017..020 (`docs/PRD.md:350–353`).

---

## 2. Current state evidence

### 2.1 The component families exist as schema data, with provenance — not as solver elements

- **FACT:** the four families are first-class `ComponentType` enum members: `bend`, `elbow`, `branch`, `reducer`, `valve`, `flange`, `expansion_joint`, `rigid`, `specialty` (`schemas/component.schema.yaml:567–582`). Each family is described by a `ComponentFamilyContract` carrying `geometry_field_kinds`, `rule_modifier_field_kinds`, a `mechanics_interface`, and a `protected_value_policy` (`:244–303`).
- **FACT:** `ComponentRecord` carries per-component `fields`, `completeness`, `provenance`, `privacy_class`, `redistribution_status`, and `review_status` (`:516–565`); `Provenance` (`source_name`/`source_location`/`source_license`/`contributor`/`contributor_certification`/`redistribution_status`/`review_status`) is required on every record, diagnostic, and quantity value (`:685–725`). This is the substrate D8's "component provenance appears in reports" consumes.
- **FACT:** the schema already names the user-entered factor slots as `ComponentFieldKind`s: `sif_user_value`, `flexibility_factor_user_value`, `linear_stiffness`, `rotational_stiffness`, `effective_area`, `movement_limit`, plus the bend/branch geometry kinds (`bend_radius`, `bend_angle`, `bend_plane_orientation`, `branch_reinforcement_area`, …) (`:347–389`). Each slot's `value_status` and `public_repository_value_policy` enforce that public fixtures carry `schema_shape_only` / `invented_non_engineering_example`, never code-derived numbers (`:391–440, :822–833`).
- **FACT (`ASSUMPTION` on absence):** no solver-side realization of any component family exists today. The solver crate set is `frame_kernel`, `linear_supports`, `nonlinear_supports`, `straight_pipe`, `diagnostics`, `performance_harness`, `sparse_direct` (`core/solver/`). A repo-wide search for `rigid_element` / `macro_element` / `bend_flexibility` / EJ-stiffness-element symbols in `core/` returns nothing (absence established by search; `ASSUMPTION` beyond the searched roots).

### 2.2 The kernel is a single generic straight frame element — no rigid / bend / EJ element exists

- **FACT:** `frame_kernel` is "the conservative first slice for `DEL-04-01`… a code-neutral mechanics kernel for 3D Euler-Bernoulli frame stiffness" providing **two-node 3D frame elements, six DOF per node**, a local 12×12 Euler-Bernoulli stiffness (axial/torsion/bending-y/bending-z), direction-cosine transforms, dense assembly + boundary reduction, and an interim dense solve (`core/solver/frame_kernel/README.md`; `local_stiffness` at `core/solver/frame_kernel/src/lib.rs:614`, `transform_global_stiffness` at `:661`). It exposes no element-type discriminator and no rigid / flexibility-modified / stiffness-element variant.
- **`ASSUMPTION` / correction flag:** the plan's D3 row asserts "rigid elements exist in the kernel" (`plans/PLAN_2026-06-17_prd_completion.md:587`); this packet **could not verify** any rigid-element construct in `frame_kernel` or elsewhere in `core/solver/`. What exists is (a) a generic frame element and (b) a rigid **support/restraint** (`prepare_rigid_support` in `core/solver/linear_supports/src/lib.rs:509`), which is a boundary condition, not a rigid component member. A rigid/semi-rigid component would today be expressed by stiffness scaling of the generic element, not a distinct element type. This is material to Option B's cost for the rigid family and is surfaced for the human's awareness.

### 2.3 Stress recovery is code-neutral and consumes explicit resultants — the natural seat for a multiplier path

- **FACT:** the `stress_recovery` crate (`DEL-05-03`) "recovers code-neutral mechanics stresses from explicit element force resultants, section properties, and optional pressure basis inputs" — axial, bending, torsional shear, thin-wall pressure membrane, and a mechanics-only stress *range* from two states (`core/loads/stress_recovery/README.md`). Its boundary is explicit: it "does not implement design-code stress equations, allowables, stress indices, **SIF/flexibility tables**, … protected standards content" (`README.md` Boundary; `core/loads/stress_recovery/src/lib.rs:5`).
- **FACT:** the PRD's §11.7 "Fundamental Stress Recovery" requires the solver to recover the mechanics quantities (forces/moments/stresses, optional principal/von Mises/Tresca) and states "**Code-specific stress formulas shall be handled in user rule packs**" (`docs/PRD.md:526–544`). PRD §16.3 separately lists "**User-entered SIF multiplier behavior**" and "**User-entered flexibility-factor behavior**" as stress-recovery features (`docs/PRD.md:921–922`). This is direct PRD support for a *multiplier applied in stress recovery* mechanism using user-entered values.

### 2.4 The code-neutral boundary (binding; not reopened here)

- **FACT (PRD §1):** the software requires "the responsible engineer to provide all code-specific, edition-specific, material, allowable-stress, component, **stress-intensification, flexibility-factor**, and proprietary design data" (`docs/PRD.md:14`). Components are "represented by elastic members, rigid members, **local flexibility modifiers, stress-intensification factors**, and other engineering corrections" (`:30`).
- **FACT (PRD §6.1/§6.3):** copyright-respecting architecture — "Code-specific values and protected data are entered by the user or imported from lawful private sources" (`docs/PRD.md:65`); every component / SIF / flexibility factor / allowable "must support source/provenance fields" (`:148`).
- **FACT (PRD §11.3):** §11.3.2 is titled "**Bend Element / Bend Macro-Element**" and requires user-entered in-plane/out-of-plane/torsional SIFs and in-plane/out-of-plane flexibility factors, with "**The software shall not supply protected B31J or code-derived bend values in the public distribution**" (`docs/PRD.md:405–420`). §11.3.3 branch: user-entered header/branch SIFs + local flexibility modifiers (`:422–435`). §11.3.4 rigid component: length/weight/COG/inertia/orientation/**optional semi-rigid stiffness**/provenance (`:437–449`). §11.3.6 expansion joint: **axial/lateral/angular/torsional stiffness, effective pressure area, pressure thrust, movement limits, manufacturer provenance** (`:462–474`).
- **Consequence:** under every option below the *values* are user-entered; the schema's `protected_value_policy` and `public_repository_value_policy` enforce no protected numbers in the public distribution (`schemas/component.schema.yaml:290–299, 419–430, 822–833`). D-18 chooses **where the numbers are consumed**, not whether the software computes them — it never does.

### 2.5 What this decision gates (DEL confirmation)

- **FACT:** the four component DELs exist as working deliverables: `DEL-03-03` "Bend and elbow component model fields", `DEL-03-04` "Branch connection component model fields", `DEL-03-05` "Rigid component models for valves, flanges, reducers, and specialty items", `DEL-03-06` "Expansion joint component model" (`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/`). The solver/kernel side is `DEL-04-01` (`frame_kernel`); stress recovery is `DEL-05-03`; the report side is `DEL-08-01` (calculation report generator) + `DEL-08-03` (warnings/assumptions/provenance section) (`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/`).
- **FACT:** R4 deliverables list bend/branch/rigid/EJ/spring-hanger/gaps objects with exit criteria "Nonlinear support validation cases converge" and "Component provenance appears in reports" (`docs/PRD.md:1223–1237`).

---

## 3. Open questions awaiting ruling

1. **Per-family vs. uniform mechanism:** is one realization mode chosen for all four families, or does each family pick its own `solver_consumption` value (the schema permits per-contract choice, §2.1)? `TBD`.
2. **Where the bend/branch flexibility enters:** as a flexibility *factor applied to the straight element's stiffness* (assembled), or purely as a *stress-recovery multiplier* on recovered resultants (§2.3), or both? The schema slot exists either way (`flexibility_factor_user_value`, §2.1). `TBD`.
3. **EJ realization:** an EJ's user-entered axial/lateral/angular/torsional stiffness set (§2.4, PRD §11.3.6) describes a member whose stiffness is *not* derivable from straight-pipe section properties — does this force a stiffness macro-element regardless of the bend/branch choice? (`ASSUMPTION`: yes; see Option C.)
4. **Rigid/semi-rigid:** realized by stiffness scaling of the existing frame element (no new element type) or by a dedicated rigid element? The kernel has no rigid element today (§2.2). `TBD`.
5. **Pressure thrust / pressure-stiffening:** EJ pressure thrust and bend pressure-stiffening (PRD §11.3.2:417, §11.3.6:471) — load-side terms or element terms? `TBD`; interacts with the load-case path, not just D-18.
6. **Diagnostic seat:** the schema already carries `EXPANSION_JOINT_STIFFNESS_DATA_MISSING`, `BEND_GEOMETRY_INCOMPLETE`, `BRANCH_*`, `RIGID_COMPONENT_STIFFNESS_DATA_MISSING` codes (`schemas/component.schema.yaml:227–240`); confirm each chosen mechanism fires the matching code on missing input (the no-silent-default rule, PRD §6.2).

---

## 4. Options

The options are separable per family; the ruling may mix (e.g., multiplier for bend/branch, macro-element for EJ — Option C).

### Option A — User-flexibility-multiplier path only (`mechanics_geometry_only` + stress-recovery modifiers)

All four families are modeled as **straight frame elements carrying the component's centerline geometry**; user-entered SIFs and flexibility factors are applied as **multipliers in stress recovery** (and, where flexibility affects displacements, as a scalar stiffness modifier on the existing element). No new element type is built. `solver_consumption = mechanics_geometry_only` (geometry to the kernel; factors consumed downstream in recovery). EJ stiffness is approximated by scaling the straight element's stiffness terms.

- **Mechanism:** reuses `frame_kernel` (§2.2) and `stress_recovery` (§2.3) unchanged in shape; component factors flow through the existing `flexibility_factor_user_value` / `sif_user_value` slots (§2.1) into the recovery multiplier path that PRD §16.3 already names (`docs/PRD.md:921–922`).
- **For:** lowest implementation risk and smallest new surface; rides the two crates that already exist; the multiplier-in-recovery path is *explicitly* PRD-sanctioned (§2.3, §11.7); fastest route to the R4 "provenance in reports" half (D8) because D1–D4 become mostly data + mapping.
- **Against:** a scalar stiffness modifier cannot faithfully represent an **EJ's directional stiffness set** (independent axial/lateral/angular/torsional, PRD §11.3.6) — forcing either a crude approximation or an honest "not representable" diagnostic; bend flexibility that should change *global displacements* (not just local stress) is only approximately captured by a single element-stiffness scalar.
- **Risk:** **Low** build risk; **medium** fidelity risk for EJ (and, secondarily, for bends whose flexibility materially changes the flexibility solution, the core problem per PRD §11.2).
- **Code-neutral analysis:** **strongest.** The solver computes only generic frame mechanics; every code-derived quantity is a user value multiplied in at recovery time — the software never assembles anything that *looks like* a code formula. Matches §1/§6.1/§11.7 most directly.

### Option B — Full macro-elements (`mechanics_geometry_and_user_flexibility`)

Each family gets a **dedicated solver macro-element**: a bend-flexibility element (flexibility factors modify the assembled element stiffness), a branch-assembly element, a rigid/semi-rigid element, and an EJ stiffness element (the user's directional stiffness set *is* the element stiffness). `solver_consumption = mechanics_geometry_and_user_flexibility` for all four — the kernel consumes user flexibility/stiffness at assembly time.

- **Mechanism:** new element types added to the kernel layer (new code beside `frame_kernel`); each consumes the user-entered factors into its assembled stiffness contribution.
- **For:** highest fidelity — directional EJ stiffness, bend flexibility's effect on global displacements, and semi-rigid behavior are represented natively; matches the PRD's own "Bend Element / **Bend Macro-Element**" framing (`docs/PRD.md:405`).
- **Against:** largest build (four element types + assembly + verification benchmarks) layered on a kernel that today has **no element-type discriminator and no rigid/macro element** (§2.2); each new element needs its own analytic verification under D-04/D-19 tolerance policy; slows D1–D4 and the R4 hinge.
- **Risk:** **High** build/verification risk; lowest fidelity risk.
- **Code-neutral analysis:** **acceptable but the most care-demanding.** A macro-element that consumes *user* flexibility/stiffness values is still code-neutral — the boundary is about *whose numbers*, not *where they enter* (§2.4). The hazard is presentational: a "bend flexibility element" must not embed any default/derived factor; it must read the user slot or fire `BEND_GEOMETRY_INCOMPLETE`/missing-input diagnostics (§3.6). Reviewable, but the boundary must be asserted per element rather than structurally guaranteed by recovery-time multiplication.

### Option C — Hybrid: multiplier-first; macro-element only where a multiplier cannot express the behavior

Adopt Option A's multiplier path for **bend, branch, and rigid/semi-rigid** (geometry to the kernel; user SIFs/flexibility as recovery multipliers; rigid via stiffness scaling), and build a **dedicated EJ stiffness macro-element** for the expansion joint, because its directional stiffness set is not faithfully expressible as a scalar modifier on a straight element (§2.4 PRD §11.3.6; §3.3). `solver_consumption` is then `mechanics_geometry_only` for bend/branch/rigid and `mechanics_geometry_and_user_flexibility` for EJ — exactly the per-family split the schema enum was built to express (§2.1).

- **Mechanism:** Option A for three families; one new element type (EJ) that takes the user's axial/lateral/angular/torsional stiffness directly as its assembled stiffness.
- **For:** captures the fidelity that matters (EJ) at minimal added build; defers/avoids the three element types whose behavior a multiplier *can* express; uses the schema's per-contract `solver_consumption` as designed; keeps D1–D3 light (mostly data + mapping + diagnostics) so the R4 provenance-in-reports half lands sooner; bend/branch can later be *promoted* to macro-elements if a usability/fidelity trigger appears, without re-architecting.
- **Against:** two mechanisms to maintain and explain; a later promotion of bend/branch to macro-elements would still be owed if multiplier fidelity proves insufficient; the EJ element still needs its own verification.
- **Risk:** **Low–medium** — one new element instead of four; the rest rides existing crates.
- **Code-neutral analysis:** **strong.** Three families stay on the structurally-safest recovery-multiplier path; the single EJ element consumes only user-entered stiffness (a manufacturer/private value with provenance, §2.4) — no code-derived term anywhere, and the one element that does consume factors at assembly takes *stiffnesses*, which are not code-formula-shaped the way an SIF is.

---

## 5. Recommendation — `PROPOSAL` (advisory only)

Adopt **Option C (hybrid: multiplier-first; macro-element only where required)**, matching the plan's advisory note (`plans/PLAN_2026-06-17_prd_completion.md:94`):

1. **Bend (D1), branch (D2), rigid/semi-rigid (D3):** realize as straight frame elements carrying centerline geometry, with user-entered SIFs and flexibility factors applied as **stress-recovery multipliers** (the PRD §11.7/§16.3 path, §2.3) and rigidity expressed by stiffness scaling of the existing element. Set `solver_consumption = mechanics_geometry_only`. Missing user factors fire the schema's `BEND_GEOMETRY_INCOMPLETE` / `BRANCH_*` / `RIGID_COMPONENT_*` diagnostics (§3.6) — no silent default (PRD §6.2).
2. **Expansion joint (D4):** build a dedicated **EJ stiffness macro-element** consuming the user's directional stiffness set (axial/lateral/angular/torsional) as its assembled stiffness; pressure thrust handled on the load side; `solver_consumption = mechanics_geometry_and_user_flexibility`; missing stiffness fires `EXPANSION_JOINT_STIFFNESS_DATA_MISSING` (`schemas/component.schema.yaml:236`).
3. **Boundary assertion:** every chosen mechanism reads user slots or diagnoses their absence; the public distribution ships only `schema_shape_only` / invented-example fixtures (§2.4). The EJ element, as the one assembly-time consumer, carries an explicit per-element boundary note that it embeds no default factor.
4. **Promotion path (deferred, not ruled here):** if multiplier fidelity for bends proves insufficient against the §16.2 / R4 validation set (D9), promoting bend/branch to macro-elements becomes a follow-up tranche — the `solver_consumption` slot already accommodates the change without schema churn.

Rationale: Option C is the only option that keeps three of four families on the structurally code-neutral, lowest-risk recovery-multiplier path (Option A's strengths) while honestly representing the one behavior a scalar multiplier cannot — directional EJ stiffness — instead of approximating it. It uses the schema's per-family `solver_consumption` enum as designed (§2.1) and lands the R4 "provenance in reports" half soonest. This recommendation is **advisory** and a `PROPOSAL` only; it confers no authority and changes no state. The human project authority may rule any option or mix.

---

## 6. Per-component applicability table

| Family (DEL) | Behavior the PRD requires | Multiplier-expressible? | Recommended `solver_consumption` | Realization |
|---|---|---|---|---|
| **Bend / elbow** (DEL-03-03; FR-017; PRD §11.3.2) | radius/angle/orientation geometry; user in-plane/out-of-plane/torsional SIFs; user in/out-of-plane flexibility factors; optional pressure-stiffening | **Mostly** — flexibility factor as element-stiffness scalar + SIF as recovery multiplier; global-displacement effect of bend flexibility is approximate | `mechanics_geometry_only` | Straight element + recovery multipliers; promotable to macro-element if D9 fidelity demands |
| **Branch connection** (DEL-03-04; FR-018; PRD §11.3.3) | geometry + reinforcement; user header/branch SIFs; user local flexibility modifiers | **Yes** — SIFs + local modifiers applied at the branch node in recovery; branch-assembly path is topological, not a new element stiffness | `mechanics_geometry_only` | Branch-assembly mapping + recovery multipliers |
| **Rigid / semi-rigid** (DEL-03-05; FR-019; PRD §11.3.4) | length/weight/COG/inertia/orientation; **optional** semi-rigid stiffness; provenance | **Yes** — rigidity via stiffness scaling of the existing frame element; semi-rigid via user stiffness on that element; mostly data model + mass mapping + provenance (no kernel rigid element exists today, §2.2) | `mechanics_geometry_only` | Stiffness-scaled frame element + data/mass/provenance |
| **Expansion joint** (DEL-03-06; FR-020; PRD §11.3.6) | **independent axial/lateral/angular/torsional stiffness**; effective pressure area; pressure thrust; movement limits; manufacturer provenance | **No** — a scalar modifier cannot represent four independent directional stiffnesses | `mechanics_geometry_and_user_flexibility` | **Dedicated EJ stiffness macro-element** (user stiffness = element stiffness); pressure thrust on load side |
| *(Reducer — PRD §11.3.5)* | concentric/eccentric geometry; section transition; optional user local stress modifiers | **Yes** — section-transition geometry + optional recovery multiplier | `mechanics_geometry_only` | Two-section straight mapping + optional recovery multiplier *(noted for completeness; not a D1–D4 row)* |

---

## 7. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Plan D1 (bend), D2 (branch), D3 (rigid)** | Become geometry-to-kernel + recovery-multiplier + mapping/provenance/diagnostics tranches; `solver_consumption = mechanics_geometry_only` set per family contract (`schemas/component.schema.yaml:497–505`). |
| **Plan D4 (expansion joint)** | Gains a dedicated EJ stiffness element tranche; `solver_consumption = mechanics_geometry_and_user_flexibility`; pressure thrust routed to the load path. |
| **`frame_kernel` (DEL-04-01)** | Option C adds at most one new element (EJ) beside the generic frame element (§2.2); Options A/B differ (zero / four). The plan's "rigid elements exist in the kernel" wording (`:587`) should be corrected to "rigidity expressed via stiffness scaling" — see §2.2. |
| **`stress_recovery` (DEL-05-03)** | Gains the user-SIF / user-flexibility multiplier application path PRD §16.3 names (`docs/PRD.md:921–922`); its code-neutral boundary is preserved (no tables, only user multipliers). |
| **Component schema** | No schema change required — the `solver_consumption` enum, factor slots, and `*_*_MISSING` diagnostic codes already exist (`schemas/component.schema.yaml:227–240, 383–384, 497–505`); the ruling fills the per-family contract value. |
| **D8 / reports (DEL-08-01 + DEL-08-03)** | Unaffected by mechanism choice — provenance fields render from `ComponentRecord.provenance` (`:559`) regardless of realization; "component provenance appears in reports" (R4 exit) decouples cleanly. |
| **D9 R4 validation (DEL-09-03)** | The chosen mechanism per family sets what the §16.2 branch-assembly and convergence benchmarks must witness; EJ element needs its own analytic verification under D-19 tolerance policy. |
| **D-16 / D-19 (assembled nonlinear solve / convergence tolerance)** | Independent — D-18 sizes the element library; D-16/D-19 size the iterative loop wrapping it. |
| **Register row D-18** | `NOT_PREPARED` → `AWAITING_RULING` at fan-in (dispatching persona; not this packet). |

---

## 8. Authority and ruling record

Only the **human project authority** rules on D-18. Agents prepared this packet and may not certify, approve, or adopt it; the recommendation in §5 is **advisory**.

Per existing decision practice, the accepted ruling is recorded as a `DEC` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (the decision log table; latest entry `DEC-040`) — D-01/D-04/D-14 were recorded this way as `DEC-018`/`DEC-024`+`DEC-026`/`DEC-035` — after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-18 from `AWAITING_RULING` to `RULED` with a pointer. Any per-family choice left `TBD` at ruling time remains `TBD` in the component family contract until a later governed record supplies it. This packet does not edit the register and does not resolve the decision.
