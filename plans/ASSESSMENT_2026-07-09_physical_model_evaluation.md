# Physical Model Evaluation — methods appropriateness and domain coverage

> **Epistemic status: agent-authored dated assessment — a map, not authority.**
> Produced 2026-07-09 at owner steer inside the piping work loop
> (`loop/WORKPLAN_2026-07-04_piping_loop.md`). Per-claim citations are
> repo-relative; the live tree governs on any disagreement. This document
> makes no lifecycle, release, readiness, or professional claim. Basis:
> direct reads of `docs/PRD.md`, `schemas/`, `core/`, `validation/`, and
> `execution/_Coordination/_DECISIONS/` at the PR #135 merge state of `main`.

## Scope of the evaluation

Two questions, judged against standard practice for beam-element pipe stress
tools (CAESAR II / CAEPIPE / AutoPIPE class) while respecting the project's
declared code-neutral boundary (missing *code content* is by design; missing
*physics* is a gap):

1. Are the numerical/mechanical **methods** appropriate?
2. Is the **domain coverage** comprehensive for a practicing stress engineer?

Status legend used below: (a) implemented with validation evidence ·
(b) implemented, thin/untested evidence · (c) schema slot only ·
(d) explicitly deferred with a decision record · (e) absent and
unacknowledged.

---

## Part 1 — Methods appropriateness

### 1.1 Element formulation — sound and standard

`core/solver/frame_kernel/src/lib.rs` implements a standard 2-node, 12-DOF 3D
**Euler–Bernoulli** frame element (`local_stiffness`, ~line 686): axial
`EA/L`, torsion `GJ/L`, textbook Bernoulli bending terms about both transverse
axes. Local→global uses a block-diagonal rotation built from orthonormal local
axes with a right-handedness check (`FrameOrientation`, ~lines 480–535);
`K_global = TᵀK_localT`, applied consistently in displacement pull-back and
force push (`core/solver/straight_pipe/src/lib.rs` ~1310–1350).

- **Distributed/point loads:** consistent (work-equivalent) Hermitian-cubic
  load vectors including partial spans
  (`straight_pipe/src/lib.rs::add_spanned_uniform_equivalent_load`, ~1165);
  reduces to the correct `qL/2, qL²/12` fixed-end set at full span. Correct.
- **Thermal:** standard initial-strain treatment — axial pre-force
  `E·A·αΔT` applied as a self-equilibrated end-force pair and removed in
  recovery (`core/product_physics/src/lib.rs::build_thermal_element_loads`
  ~3724, `corrected_local_forces_for_axial_effects` ~3921). Correct for
  straight members.
- **Pressure thrust:** `F = p·A_internal` (or user effective area for EJs) as
  axial end-force pair (`product_physics/src/lib.rs` ~3767, ~3871).
- **No shear deformation (Timoshenko) term** — a minor deviation from
  CAESAR II-class defaults; negligible for ordinary L/D, over-stiffens short
  stiff stubs.

### 1.2 Bend/elbow treatment — the central methods concern

There is **no curved-pipe element and no automatic arc meshing**; the
analytical mesh is exactly the user's nodes and segments as straight prismatic
elements (`product_physics/src/lib.rs::build_model` ~2534–2643). Under the
DEC-045 `mechanics_geometry_only` realization, the user flexibility factor and
SIF are consumed **only as a post-solve multiplier on the recovered stress
summary** (`append_component_stress_multiplier_result` ~4630; the emitted
`sign_convention` string states "base frame stiffness unchanged" ~4654).
Grep confirms `flexibility_factor_user_value` never modifies any stiffness
term.

Consequence: in this product class the bend flexibility factor `k` (routinely
5–20) divides the bend's bending stiffness and is the dominant physics of the
whole flexibility analysis. With bends as plain straight chords the global
solution is computed on an **over-stiff structure**: thermal displacements are
under-predicted and anchor/nozzle reactions and moments **over-predicted**
(often the governing results), and the stress multiplier cannot compensate for
the wrong force distribution feeding it. Ovalization (von-Kármán) flexibility
and pressure stiffening are likewise absent. PRD §11.3.2 requires a
"user-entered in-plane/out-of-plane flexibility factor" on a bend
element/macro-element, so as written this is an unmet PRD requirement —
partially acknowledged via D-18 and the R4 packet's "invented preview path"
wording, but **no document states plainly that flexibility factors are
solve-inert**.

The **expansion joint** is handled correctly in principle: a dedicated
user-stiffness macro-element assembled into the global matrix
(`frame_kernel::UserStiffnessElement` ~601;
`product_physics::build_expansion_joint_user_stiffness_elements` ~2732) with
pressure thrust from user effective area.

### 1.3 Nonlinear supports — right method class, two real defects

The active-set fixed-point loop (DEC-044) — assemble once, per iteration build
the active boundary, solve, classify, repeat until no state changes
(`core/solver/nonlinear_integration/src/lib.rs::solve_active_set_frame_with_mode`
~283–363) — is the standard method for one-way/gap/lift-off/friction
supports, with good practice around it (iteration cap, non-convergence
diagnostics in `core/solver/diagnostics/src/lib.rs` ~494, free-DOF
force/moment/work equilibrium residual reporting ~673–678).

- **Defect A — complementarity quantity not switched by state.**
  Classification of one-way/lift-off supports keys only on reaction sign
  (`core/solver/nonlinear_supports/src/lib.rs::classify_support_state`
  ~543–563) while gaps key only on displacement (~553); but the boundary
  mapping releases `Inactive` DOFs and pins `Active` ones
  (`nonlinear_integration::prescribed_displacement_for_state` ~601). A
  released one-way support recovers ≈0 reaction, so it re-classifies
  `Inactive` even when the pipe displaces into it; a closed gap pinned at the
  clearance keeps `displacement == gap`, so it stays `Active` even in
  tension. Re-engagement of a lifted one-way support and lift-off of a closed
  gap can therefore be missed, and results depend on the seeded
  `initial_states`. A correct complementarity solver flips the governing test
  with the active state.
- **Defect B — friction is stick/free, not Coulomb.** The classifier's
  stick/slip test `|F_t| ≤ μ|N|` is correct (~566–596), but in the assembled
  loop `Sliding` simply **releases** the DOF (zero tangential resistance) and
  `Sticking` fully fixes it; the bounded sliding force `±μN` is never applied
  as a load. The friction normal is caller-supplied or derived from a separate
  named DOF (`friction_normal_for_support` ~1034), i.e. normal–tangential
  coupling is not solved self-consistently; there is no load-step path
  history. Sliding supports are under-restrained relative to real Coulomb
  friction (closer to simplified-friction tools than to CAESAR II).
- **Safeguards are thin:** no relaxation, line search, or cycle detection
  beyond a deterministic sliding-persistence anti-chatter rule (~522);
  active-set oscillation surfaces only at the iteration cap.

### 1.4 Solvers — well built

- **Dense** (`frame_kernel::solve_dense` ~895): Gaussian elimination with
  partial pivoting, zero-pivot guard, singular-system error. Appropriate as
  the default path and parity oracle.
- **Sparse** (`core/solver/sparse_direct/src/lib.rs`, DEC-023): a genuinely
  well-built hand-rolled skyline/profile **LDLᵀ** with deterministic reverse
  Cuthill–McKee ordering (~496), fixed operation order, and a factorization
  report exposing pivot extremes, a condition-ratio proxy, and located
  nonpositive pivots (~377). No numeric pivoting — standard and acceptable
  for SPD stiffness matrices, with failures surfaced rather than absorbed.
  Caveat: rigid-body-mode detection is only post-hoc via pivot signals,
  partially meeting PRD §11.8.
- The **dense-vs-sparse parity oracle** runs per nonlinear solve
  (`nonlinear_integration` ~761–807) — a strong robustness practice.

### 1.5 Stress recovery — correct mechanics, two notes

`core/loads/stress_recovery/src/lib.rs` recovers `N/A`, `M/Z` per axis,
`T·r/J`, thin-wall hoop `pr/t` and longitudinal `hoop/2` (~452–906);
element-end resultants from `K·d` with fixed-end/axial effects subtracted and
interior stations integrated by statics. Notes: (a) user SIFs never enter
recovery — by design they live in the review multiplier and rule packs;
(b) the summary combines bending as **absolute sum** `|σby|+|σbz|`
(`summarize_components` ~987), not the SRSS resultant PRD §11.7 implies —
conservative, but a flag-worthy choice.

### 1.6 Verification approach — strong for the linear core, missing where it matters most

Closed-form analytic seeds (cantilever `PL³/3EI`, portal frame, branch
assembly, straight-pipe weight; `validation/benchmarks/mechanics/`,
`validation/hand_calcs/mechanics/`), exact stiffness-term unit tests at 1e-9,
a thirteen-fixture nonlinear multi-support hand-calc suite
(`validation/hand_calcs/nonlinear/`), and live parity/residual evidence.
Gaps: release tolerance/CI thresholds remain `TBD`
(`docs/VALIDATION_STRATEGY.md`), and there is **no benchmark exercising bend
flexibility against a curved-element or known-flexibility reference** — and no
expansion-loop / L-bend thermal-bending benchmark at all (thermal evidence is
axial-only), which is the single most representative validation case for this
product class.

### Ranked methods concerns

1. Bend/branch flexibility absent from global stiffness (stress-multiplier
   only) — the core flexibility-analysis physics is not represented.
2. Active-set classification does not switch the governing quantity by state
   (missed re-engagement / lift-off transitions; seed-state dependence).
3. Friction is stick/free, no bounded `μN` slip force, decoupled normal.
4. Thin nonlinear convergence safeguards (cap-only; no relaxation/cycle
   detection).
5. Euler–Bernoulli only (no shear deformation).
6. Absolute-sum bending summary instead of SRSS resultant; von
   Mises/Tresca/principal absent.
7. Rigid-body-mode detection only post-hoc via pivots.

Balance: the 12-DOF element, load vectors, thermal/pressure treatment, EJ
macro-element, deterministic sparse solver with surfaced diagnostics, parity
oracle, and analytic-seed discipline are all correct and appropriately built.
**The linear straight-member engine is sound; the risk is concentrated in
component (bend) realization and nonlinear-support state logic.**

---

## Part 2 — Domain coverage

### 2.1 Load classes

- Deadweight (metal/contents/insulation, distributed + lumped): **(a)**
  (`core/section_properties/calculator.py`; `core/loads/primitive_loads`;
  `validation/hand_calcs/mechanics/straight_pipe_weight_recovery.md`).
- Pressure thrust incl. EJ effective area: **(a/b)** (evidence
  `tp_phys_008`, TP-R4-D4-EJTHRUST-001).
- Bourdon effect / pressure stiffening: **(e)** — no term anywhere; PRD
  §11.3.2 has a settings slot but no stiffness path exists for it to act on.
- Thermal expansion: **(b)** — axial `E·A·αΔT` validated
  (`fixed_fixed_thermal_axial.md`); **no routed-system thermal-bending
  benchmark**.
- Multiple temperature cases: **(c)** — `load_type: temperature` labels
  without per-case material property semantics.
- Imposed displacement / anchor movement / settlement: **(a)**
  (`frame_kernel::reduce_system_with_prescribed_displacements`; settlement
  fixtures in `core/solver/linear_supports`).
- Concentrated / uniform / partial-span external loads: **(a)**.
- Wind: **(c)** — enum label; user-supplied force-per-length only, no
  wind-pressure load generation.
- Snow/ice: **(e)**.
- Seismic static-equivalent: **(c)** — no mass×g generation from the model's
  own weight; user hand-computes what is one input in every peer tool.
  PRD §11.5 lists it as if provided — effectively unacknowledged.
- Water hammer / relief thrust (static equivalent): **(c)** — generic
  occasional nodal forces only.
- Hydrotest: **(b/c)** — pressure category exists; no water-fill case
  construction.
- Dynamics (modal/spectrum/harmonic/time-history), FR-024: **(d)** —
  greenfield per `D-12_fr024_fr025_disposition.md` §2.1; **D-12 is
  AWAITING_RULING**; v0.2 PRD de-scopes dynamics.

### 2.2 Load cases and combinations

Three-tier case/combination/rule-check structure per PRD §11.6 exists and is
real: factored linear terms, `result_state_subtraction`, `range_envelope`
(`schemas/model.schema.yaml`; `core/loads/load_case_algebra`), with
dimension-consistency checks — **(a/b)**. Expansion stress-range recovery
implemented and hand-calced (`stress_recovery::recover_stress_range` ~658;
`validation/hand_calcs/stress/stress_range.md`) — **(a)**.
**Hot/cold modulus handling: (e)** — temperature-dependent slots exist in
`schemas/material.schema.yaml` but no mechanism solves at `E_hot` and
evaluates range at `E_cold`; combinations operate on solved quantities only.
Physics/workflow, not code content; unacknowledged.

### 2.3 Components

- Straight pipe: **(a)**.
- Bends/elbows: **(c for stiffness; b for stress multiplier)** — see §1.2.
- Mitered bends: **(e)**.
- Branch/tees: **(b/c)** — user SIF slots + one mechanics benchmark
  (`branch_assembly.md`); flexibility modifiers are review data, not
  stiffness.
- Reducers: **(c)** — schema slot; no stepped-section element.
- Valves/flanges as rigid: **(b)** — user semi-rigid stiffness; no rigid-link
  constraint element (no silent defaults is deliberate).
- Expansion joints: **(b)** — sound user-stiffness macro-element + thrust;
  tied/hinged/gimbal hardware is metadata only, no tie-rod kinematics.
- Spring hangers: **(b, deliberately narrow per D-15/DEC-049)** — user-entered
  variable/constant records, diagnostics, provenance; **no sizing/selection
  algorithm** (the cold-load/travel arithmetic is code-neutral and every peer
  tool performs it — the D-15 deferral conflates catalog data with that
  arithmetic).
- Flanged-joint leakage evaluation: **(e as physics / borderline
  code-content)** — no gasket/bolt data slots or hook.

### 2.4 Supports / restraints

Anchors, guides, line stops, translational/rotational springs: **(a)**
(`core/solver/linear_supports`; `schemas/model.schema.yaml` support enum).
One-way/gap/lift-off/friction: **(a within a bounded envelope)** — real
active-set loop plus the thirteen-fixture hand-calc suite, with the project's
own R4 packet bounding the evidence to seed/product/fixture surfaces
(`plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md` ~91, 127–129) and
the §1.3 method defects noted above. Snubbers: **(e)** (meaningless without
dynamics, but nowhere acknowledged).

### 2.5 Materials and sections

Temperature-dependent property slots: **(c→b)** — schema slots with an open
`temperature_interpolation_policy` decision inside
`schemas/material.schema.yaml`; solver consumes single-valued E/α.
Hot/cold allowables: **(c by design)** — user/rule-pack supplied, correct per
boundary. Insulation/lining/contents/corrosion allowance: **(a/b)**
(`schemas/section.schema.yaml` ~533–545; `core/section_properties`).
**Mill tolerance: (e)** — no field anywhere; routine practice, unacknowledged.

### 2.6 Environment / special domains

Buried pipe/soil springs, FRP/orthotropic materials, jacketed pipe,
slug/two-phase forces, fatigue cycle counting, snubbers: **all (e)** — none
appear in PRD §5 non-goals, the open-questions section, or any D-record.
Nozzle flexibility: **(c-ish)** — the generic `UserStiffnessElement` could
carry user-entered nozzle stiffnesses but there is no first-class object.
Equipment load checks (API 610 class): **(d-by-boundary)** — correctly
excluded as code content; reaction reporting (the physics input) exists.
Local FEA export (FR-025): **(d)** — contract-only
(`schemas/local_fea_handoff.schema.yaml`; D-12 pending).

### 2.7 Outputs

Displacements, local/global forces/moments, reactions: **(a)**
(`schemas/results.schema.yaml`). Stress categories: **(b, incomplete vs PRD)**
— axial/bending/torsion/hoop/longitudinal + ranges; **no principal / von
Mises / Tresca** (PRD §11.7 optional items, silently unmet; mechanics, not
code content); thin-wall pressure basis only. Spring selection reports:
**(d via D-15)**. Code-check hook points (rule packs, grammar, blocking on
missing inputs): **(a)** — this part of the code-neutral architecture is
genuinely well built out.

### Ranked coverage gaps (most consequential for a practicing engineer)

1. **Bend flexibility solve-inert** (unacknowledged as a physics defect;
   fails PRD §11.3.2 as written).
2. **No hot/cold modulus mechanism; temperature cases lack property
   semantics** (unacknowledged).
3. **Dynamics wholly absent** (acknowledged; D-12 awaiting ruling).
4. **Seismic/wind are labels, not load generators** (effectively
   unacknowledged; code-neutral mechanics, cheap relative to value).
5. **No spring-hanger sizing/selection** (acknowledged by D-15, but the
   deferral sweeps in code-neutral sizing arithmetic).
6. **No von Mises/Tresca/principal; thin-wall pressure only** (silently
   unmet PRD optional items).
7. **Thermal validation evidence is axial-only** — no expansion-loop
   benchmark (unacknowledged evidence gap).
8. **Mill tolerance absent** (unacknowledged; small fix).
9. **Friction fidelity + bounded nonlinear evidence envelope**
   (acknowledged residual).
10. **Special domains absent and unnamed** (buried pipe, jacketed, FRP,
    slug, snubbers, fatigue counting, first-class nozzle flexibility) —
    individually deferrable, but absence should read as decision, not
    omission.
11. **EJ hardware kinematics metadata-only; mitered bends/stepped reducers
    have no formulation** (partially acknowledged via D-18/R4 residuals).
12. **FR-025 contract-only** (acknowledged; right architectural shape).

---

## Overall verdict

The foundation is unusually honest and well-evidenced for what it covers: a
validated linear 3D frame solver (dense + deterministic sparse LDLᵀ with a
live parity oracle), consistent load assembly, real imposed displacements, a
genuine nonlinear active-set support loop with a serious hand-calc suite,
clean load-case algebra with range envelopes, and an exemplary
rule-pack/provenance boundary. But measured against ordinary stress
engineering practice, the current model is **a structural frame analyzer with
piping metadata**: until flexibility factors physically soften bends (gap 1),
modulus/temperature-case handling exists (gap 2), and occasional-load
generation is more than a label (gap 4), a practicing engineer cannot
reproduce even a basic operating/sustained/expansion evaluation of a real
routed line at parity with established tools — and gaps 1, 2, and 4, unlike
dynamics, are acknowledged nowhere in the project's own decision records.

## Candidate follow-ups surfaced (owner decision required; none executed)

- **CAND-1 (disclosure, small):** state plainly in `docs/PRD.md` /
  validation-strategy surfaces that user flexibility factors are currently
  solve-inert ("base frame stiffness unchanged"), so the limitation reads as
  a decision rather than an omission.
- **CAND-2 (mechanics, largest value):** bend flexibility entering the global
  stiffness — either a curved-bend macro-element or user-k-scaled bending
  stiffness on bend spans — plus an expansion-loop thermal benchmark against
  a known-flexibility reference. Touches the DEC-045 realization ruling.
- **CAND-3 (mechanics):** fix the active-set complementarity test (switch
  governing quantity by state) and apply bounded `±μN` sliding friction
  forces; extend the fixture suite to cover re-engagement/lift-off
  transitions.
- **CAND-4 (workflow physics):** hot/cold modulus basis control per load
  case; seismic/wind static-equivalent load generation from model mass and
  user parameters; mill-tolerance slot.
- **CAND-5 (disclosure, small):** name the absent special domains (buried
  pipe, jacketed, FRP, slug, snubbers, fatigue counting, nozzle flexibility)
  in PRD non-goals/open questions.
