# VERIFY_BRIEF — Fresh-Context Adversarial Verification of T3 DEL-04-01 Arc Pressure-Thrust Candidate Brief

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T3
**Role:** Fresh-context adversarial verifier (governed Agent 2, non-delegating)
**Objects reviewed:**
- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md` (CB-2026-07-19-T3-DEL-04-01-ARC-PRESSURE-THRUST-001)
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T3/CURRENT_CANDIDATE_RATIONALE.md`

**Live tree at verification:** branch `claude/piping-r14-pkg04-mechanics`, HEAD `faee4faed`
(T1 landed at `723c95b0f`, T2 at `faee4faed`; brief authored at basis `6152908b3`
with symbol-level references — landed T1/T2 edits treated as expected context, not drift).
Date: 2026-07-19. Offline; read-only except this file.

---

## 1. Per-Claim Verification Table (C1–C14)

| Claim | Verdict | Evidence |
|---|---|---|
| C1 | **CONFIRMED** | `_STATUS.md` (DEL-04-01 folder): `Current State: IN_PROGRESS`; `## Remaining` contains exactly two items; first is verbatim "Treat arc pressure-thrust beyond the recorded straight-chord treatment for curved-bend macro spans (source: Receipt 11 named remainder / TP-PMM-P1-CURVEDBEND-004 boundaries)"; second is the G1/G2/G4 + M2/M3 re-disposition row carrying `(gated: owner re-disposition where not closed by evidence)`. |
| C2 | **CONFIRMED** | `MEMORY.md` lines 379–411, entry "2026-07-10 - TP-PMM-P1-CURVEDBEND-004 arc-consistent loads and interior stations"; exact residual sentence at lines 407–408: "Pressure thrust keeps the recorded straight-chord axial treatment on macro spans (stations inherit it); D-38 temperature semantics untouched." |
| C3 | **CONFIRMED** | `core/product_physics/src/lib.rs`: `add_pressure_thrust_loads` (lines 5694–5717) applies `force[i] -= axial_load·local_x`, `force[j] += axial_load·local_x` using the straight pipe's chord `frame_element().orientation().local_axes[0]` for every load, with no macro-span branch (contrast `add_thermal_equivalent_loads`, lines 5719–5747, which does branch on `curved_bends_by_pipe`). Inline comment at the load-case assembly, lines 1140–1142: "Pressure thrust keeps the straight-element treatment on macro spans: equal/opposite axial end forces along the chord direction (decision recorded in the curved-bend review-row basis)." Review-row basis string `pressure_thrust_treatment=straight_chord_axial_end_forces` at line 6896. |
| C4 | **CONFIRMED** (one wording imprecision, see D3) | `corrected_local_forces_for_axial_effects` (5784–5803) adds the pressure axial into chord `UX`/`DOF_PER_NODE+UX`. `recover_curved_bend_local_forces` receives `pressure_thrust_loads` (call at 1295–1302; chord-UX correction applied at 5877–5881). `curved_bend_station_resultants` (5928–5983) does **not** take `pressure_thrust_loads` as a parameter — it inherits the correction through `corrected_local_forces` (call at 1341–1347), which matches MEMORY's "stations inherit it" but not the claim's literal parameter-level wording. `pressure_thrust_for_pipe` gates `include_pressure_longitudinal` (1405–1407). |
| C5 | **CONFIRMED** | `core/solver/curved_bend/src/lib.rs`: `geometry()` (149), `radius()` (180), `included_angle()` (184), `arc_length()` (188), `orientation()` (194), `end_flexibility()` (205), `local_stiffness()` (241), `global_stiffness()` (254), `consistent_uniform_nodal_loads()` (276), `arc_section_resultants()` (359) with documented section frame "x tangent toward node j" (348, 437–445). `ArcGeometry` local frame (62–74): x radial at node i, node i at arc angle 0, node j at `included_angle` about local z — unit end tangents follow as t(θ)=(−sin θ, cos θ, 0) in the local frame with no new geometry source (matches crate tests at 1617–1640: `tangent_i = [0,1,0]`, `tangent_j = [−sin φ, cos φ, 0]`). |
| C6 | **REFUTED IN PART — decisive part refuted** | Independent worked integration in §2 below. (a) The resultant identity `∫ q ds = p·A·(t_j − t_i)` holds **only for a radially INWARD line load**; the physical pressure-imbalance wall load on a bend is radially **OUTWARD** and has resultant `p·A·(t_i − t_j)` — the opposite sign. (b) The distributed radial load has zero moment about the center (true), and the end-tangent **pair** has zero **net** moment about the center (true, moments −pAR and +pAR cancel), but the brief §2 parenthetical "each radial line-load element **and each end-tangent force** has zero moment about the center" is mathematically false: each end-tangent force has moment of magnitude `p·A·R` about the center. (c) Chord reduction as φ→0: true. Consequence: the proposed end-tangent pair is the **equilibrant**, not the equivalent, of the physical outward wall load; pair-only loading is not statically exact — see §2.4. |
| C7 | **CONFIRMED** | `Dependencies.csv`: exactly eight `EXECUTION`-class `UPSTREAM` rows — five CONSTRAINT (DAG-002-E0100..E0104, DEL-00-01/02/03/06/08) + three PREREQUISITE (E0429/E0430/E0431, DEL-02-01/02-02/02-03) — all `SATISFIED`. (Three ANCHOR rows are `NOT_APPLICABLE`, consistent with "deliverable-local EXECUTION UPSTREAM".) |
| C8 | **CONFIRMED** | `validation/benchmarks/mechanics/src/lib.rs` `fixture_inventory()` (547–572): 22 fixtures; only two involve curved bends (`expansion_loop_curved_bend_thermal_fixture`, `curved_bend_distributed_fixed_end_fixture`) and neither contains any pressure load (verified by function-body scan); the pressure fixtures (tp_phys_008/009) are straight-pipe. `validation/benchmarks/nonlinear/src/lib.rs`: every model has `curved_bend_elements: Vec::new()`. `validation/benchmarks/stress`: section-level, no spans. No `del1005_payload_binding_*` input contains a bend component. Pinned `tp_runner_015_final_cli_solve_input.json`: bend `component:C-110` has `solver_consumption: "mechanics_geometry_only"` and no `bend_pipe_ref`; macro realization requires `solver_consumption == "curved_bend_macro_element"` plus a resolvable `bend_pipe_ref` (`is_curved_bend_macro_component` at product_physics 3277–3281 with `DEC_070_CURVED_BEND_SOLVER_CONSUMPTION = "curved_bend_macro_element"` at line 85; `build_curved_bend_macro_elements` 3303–3356) — so the pinned fixture's bend is **not** macro-realized on the live tree and §3.7's conditional resolves to no output change. |
| C9 | **CONFIRMED** | `validation/benchmarks/mechanics/README.md` line 70 records the "DEC-026 analytic-class `1.0e-9` relative tier" as the recorded comparison basis in use for the existing curved-bend fixtures; DEL-04-01 MEMORY.md corroborates (lines 404–406). Reuse at the recorded tier creates no tolerance. |
| C10 | **REFUTED** | Fence gap, with in-wave precedent. The suite's `readiness_metadata_matches_documented_boundaries` test (`validation/benchmarks/mechanics/src/lib.rs` ~5804–5838, on `BENCHMARK_README`/`HAND_CALC_README` from `include_str!` at 54–55) asserts every `fixture_id` appears in **both** `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/README.md`, and that the hand-calc README also contains the fixture's hand-calc file name. A new §3.6 fixture with "inventory registration" therefore fails §6's `cargo test … benchmarks/mechanics` unless one additive line is added to each mirror README — but §5.5 says "no README" and §9 forbids README edits. The default §3.6 path is dead-on-arrival inside the fence; only the fallback clause is executable. The T2 brief in this same wave hit exactly this defect and was amended (its records note "the suite README … also mirrors the fixture", "the v2 fence's README allowance was incomplete") to permit additive truthful lines in both READMEs; T3's fence fails to carry that recorded precedent. Remainder of C10 (all other §4 tasks map into §5 items 2–4, 6–7; curved_bend additive-only by predicate) checks out. |
| C11 | **REFUTED** | Independent 10-class re-screen in §3 below hits **class 9** (claim stronger than warrant / erroneous basis): the brief's §2 static-identity bullet contains a false moment statement and a sign-inverted equivalence claim (C6), and the §4.2 derive-first gate cannot catch it because §3.2 pre-specifies the identities the executor must derive — they are internally consistent for the inward radial load, so the mandated derivation completes "exactly" while landing inverted physics. The rationale's class 1/4 and class 8 dispositions are otherwise sound (no owner-gated method fork; elementary statics on invented geometry; no protected content). |
| C12 | **CONFIRMED** | All §6 tools exist at `REPO_ROOT` (`tools/validation/validate_claims_language.py`, `tools/validation/validate_path_anchors.py`, `tools/software_workflow/validate_change_scope.py`); all named Cargo manifests exist; offline flags specified; worktree carries the R12-ENVREPAIR-01 offline provisioning per the R14 plan; build target dirs present. |
| C13 | **CONFIRMED** | Brief §10 block: `AgentClassification: CLASSIFY_ELIGIBLE` and `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL` kept distinct; `OwnerCaseSelection: NONE`; `EffectStatus: HELD`; preserved gates enumerated. |
| C14 | **CONFIRMED** (caveat noted) | `ORCHESTRATION_PLAN.md`: W1/PKG-04 T3 is the named queue item; managers author per-tranche briefs and run the D-52/D-54 chain; per-tranche commits; one PR per wave with HELP_HUMAN merging (no manager push/PR/merge/receipt). The single wave-level DEC-025 sweep refinement is recorded in the T1 brief (§5 item 9 and §6: registered checks and the sweep run once at W1 closeout "per the controlling HELP_HUMAN W1 dispatch"); T1 (verified COMMIT-SAFE at v3) and T2 both landed under it. Caveat: plan-v1 literal text says each tranche commit is gated by a check set "including the DEC-025 evidence sweep"; the refinement rests on the recorded controlling W1 dispatch citation, an accepted in-wave precedent. |

Tally: 11 CONFIRMED (one with a minor wording caveat), 2 REFUTED (C10, C11), 1 REFUTED IN PART with the decisive physics part refuted (C6).

---

## 2. Independent Verification of C6 (worked integration)

Setup (matches the crate's conventions, `core/solver/curved_bend/src/lib.rs` 62–74):
center of the arc at the origin of the bend-plane frame, radius R, node i at arc
angle 0, node j at angle φ (the included angle), measured about local z.

- Position: r(θ) = R·e_r(θ), e_r(θ) = (cos θ, sin θ, 0).
- Unit tangent oriented i→j: t(θ) = dr/ds = (−sin θ, cos θ, 0); so
  t_i = t(0) = (0, 1, 0), t_j = t(φ) = (−sin φ, cos φ, 0). ds = R dθ.
- Note dt/ds = −(1/R)·e_r (points radially **inward**).

### 2.1 Resultant of the distributed radial line load

For load q(θ) = σ·(pA/R)·e_r(θ) with σ = +1 (outward) or σ = −1 (inward):

∫₀^φ q ds = σ·pA ∫₀^φ (cos θ, sin θ, 0) dθ = σ·pA·(sin φ, 1 − cos φ, 0).

Compare pA·(t_j − t_i) = pA·(−sin φ, cos φ − 1, 0) = **−**pA·(sin φ, 1 − cos φ, 0).

Therefore:
- **outward** load (σ=+1): resultant = pA·(sin φ, 1−cos φ, 0) = pA·(t_i − t_j) ≠ pA·(t_j − t_i) for φ > 0;
- **inward** load (σ=−1): resultant = pA·(t_j − t_i) — the brief's identity holds only in this case.

(Equivalently: ∫ pA·(dt/ds) ds = pA·(t_j − t_i) trivially, and dt/ds is the inward radial direction.)

Numerical cross-check (R = 2.7, pA = 13.4, φ = 1.1, midpoint rule, 2×10⁶ panels):
outward resultant (11.9421786, 7.3218120); pA(t_j − t_i) = (−11.9421786, −7.3218120);
pA(t_i − t_j) = (+11.9421786, +7.3218120). Confirms the sign relations exactly.

### 2.2 Moments about the arc center

- Each distributed element: r × q ds ∝ e_r × e_r = 0. Zero moment — confirmed
  (numerical: ~1e-19, integration noise).
- End-tangent forces: moment of a force pA·t at radius R about the center is
  R·e_r × pA·e_θ = pA·R·ẑ. So −pA·t_i at node i contributes **−pA·R·ẑ** and
  +pA·t_j at node j contributes **+pA·R·ẑ** (numerical: ∓36.18 with pA·R = 36.18).
  The **pair's net** moment about the center is zero; **each individual force's
  moment is pA·R ≠ 0**. The brief §2 parenthetical ("each radial line-load element
  and each end-tangent force has zero moment about the center") is false as stated;
  only the pair-cancellation form of the argument is valid.

### 2.3 Chord reduction

As φ → 0, t_i, t_j and the unit chord all converge (numerically verified at
φ = 1e-3, 1e-6); the pair {−pA·t_i, +pA·t_j} reduces to the existing straight-chord
treatment {−pA·c, +pA·c}. Confirmed.

### 2.4 Sign/orientation cross-check and physical consequence

Fluid statics fixes the direction of the physical load. Control volume on the fluid
column between two arc sections: wall-on-fluid force W satisfies
pA·t(θ₁) − pA·t(θ₂) + W = 0, so fluid-on-wall per unit arc length is
−pA·(dt/ds) = +(pA/R)·e_r — radially **OUTWARD** (consistent with the textbook
elbow result: net pressure force on a 90° bend is pA√2 along the bisector away from
the center of curvature, and with pressurized curved tubes tending to straighten).

Hence the exact per-span pressure system consistent with the code's existing
convention (end thrusts ±pA representing cut/cap forces, which telescope across
tangent-matched junctions) is:

  {−pA·t_i at i, +pA·t_j at j} **PLUS** the distributed outward wall load (pA/R)·e_r along the arc,

which is fully self-equilibrated (zero resultant, zero moment) — exactly the
fine-discretization limit of applying the current straight-span pair to a polyline
approximation of the arc. The brief's selected core (§3.1, §4) applies the
**end-tangent pair only**. That pair equals the *equilibrant* of the outward wall
load (their sum is zero), so pair-only loading:

- applies a net unbalanced force pA·(t_j − t_i) (pointing toward the arc center)
  per pressurized macro bend — a per-span pressure load system with nonzero
  resultant, which pressure statics on a closed system forbids;
- in assembly with tangent-matched neighbor spans, cancels the neighbors' end
  thrusts at both bend nodes and thereby deletes the physical outward bend force
  entirely — whereas the current chord treatment, combined with the same neighbor
  thrusts, delivers node loads pA·(t_i − c) and pA·(c − t_j) whose total force is
  pA·(t_i − t_j) and whose total moment about the arc center is zero, i.e. the
  chord treatment already lumps the outward wall load statically exactly at the
  two bend nodes;
- for an isolated anchored bend, produces net support reactions of the **opposite
  sign** to the true pressure statics.

So the proposed treatment is not "statically-exact"; on net-force grounds it is a
regression from the recorded chord treatment. The correct arc-exact discharge of the
Remaining item is the end-tangent pair **plus** the distributed outward radial load
(or any statically/work-consistent nodal reduction of that combined self-equilibrated
system). Because §3.2 pre-specifies the (inward-load-consistent) identities as the
hand-calc content and §3.6 ties the fixture to those same closed forms, the brief's
derive-first and benchmark gates are self-consistent with the inverted physics and
would not catch the error.

---

## 3. Independent 10-Class D-52 §4.1 Fast-Reject Re-Screen

Re-derived item by item against `execution/_Coordination/_DECISIONS/D-52_four_lens_standing_approval_overlay.md` §4.1 (ambiguity is a hit):

1. **Irreducible owner preference / two defensible outcomes:** not hit *as a method
   fork* — I probed the rationale's own attempted failure mode (arc treatment as an
   owner-gated fork analogous to friction D-XX) and agree it fails: once the statics
   are done correctly there is no competing engineering model, and the Remaining row
   carries no `(gated:)` suffix. The multiplicity that does exist (pair-only vs
   pair-plus-distributed) is not owner preference; it is settled by exact statics —
   but settled **against** the brief's selection, which is a class-9 basis defect,
   not a class-1 hit.
2. **Professional/safety/legal accountability:** not hit. Preview mechanics on
   invented fixtures; regression-evidence posture; no reliance claim.
3. **Conflict ruling undetermined by authority chain:** not hit. The chord treatment
   is a recorded interim decision whose own residual directs this work.
4. **Scope/boundary/normative/acceptance-criteria change:** not hit. Recorded
   deliverable scope; DEC-026 tier reused as recorded; no threshold created. The
   optional §3.5 refinement is bounded ("only where exactly derivable", recorded
   fallback) and stays inside the fence.
5. **Lifecycle/stage/release/evidence-posture act:** not hit. `IN_PROGRESS`
   unchanged; suite claim posture unchanged; regression evidence only.
6. **External/procurement/publication:** not hit. Offline; no push/PR/merge in the
   tranche.
7. **Merge authority / destructive action:** not hit. Ordinary wave-branch commits;
   existing fixtures/witnesses untouched; pinned-fixture before/after capture (moot
   on the live tree — the pinned bend is not macro-realized, §1 C8).
8. **Protected/private data exposure:** not hit. The derivation is elementary
   statics on invented geometry (parametrize the arc, integrate); it requires no
   code-book SIF/flexibility values or protected tables, and brief §9 expressly
   excludes them.
9. **Evidence unavailable / stale basis / claim beyond warrant:** **HIT.** The §1
   basis facts are current on the post-T2 tree (chord treatment, inline comment,
   recovery inheritance, curved-bend API, fixture exposure, DEC-026 tier, del1005
   and pinned-fixture posture all verified live — no staleness from T1/T2, whose
   product_physics edits are disjoint from the pressure-thrust path). But the
   brief's central §2 static-identity bullet asserts (i) a per-force zero-moment
   statement that is mathematically false (§2.2 above) and (ii) an equivalence whose
   sign is inverted relative to the physical outward pressure-imbalance load
   (§2.1/§2.4) — a claim stronger than (and contrary to) its warrant, propagated
   into §3.1/§3.2/§3.3/§3.6 as the tranche's core. The derive-first gate does not
   restore the warrant because the erroneous identities are themselves the mandated
   derivation target.
10. **Protected domain-engine paths / prover / higher-order boundaries:** not hit.

**Re-screen result: FAIL (class 9).** The rationale's §1 screen recorded "not hit"
for class 9 on the strength of live-verified code facts; those facts are indeed
live-verified, but the screen did not check the physics claim itself, which is the
outcome-determining premise (D-52 §4.2 gate 1: "every outcome-determining premise
is present in current, accepted project artifacts" — the sign of the physical load
is not, and the brief's derivation of it is wrong).

---

## 4. Additional Defects

| ID | Severity | Defect |
|---|---|---|
| D1 | **MAJOR (blocking)** | Physics basis error in the tranche core (C6/§2 above): the end-tangent pair is the equilibrant, not the equivalent, of the physical outward pressure-imbalance load; per-span pressure loading stops being self-equilibrated; assembled behavior deletes the outward bend thrust the current chord treatment delivers statically exactly; isolated-bend reactions flip sign. The brief's own gates (derive-first §4.2, fixture-tied-to-closed-forms §3.6, unit tests "values against the hand calc" §4.2) are all anchored to the same pre-specified identities and therefore cannot catch it. |
| D2 | **MAJOR (blocking)** | §5 write fence omits the two mirror READMEs (`validation/benchmarks/mechanics/README.md`, `validation/hand_calcs/mechanics/README.md`) that the suite's readiness test enforces per fixture id (and hand-calc file name), while §3.6 requires an additive fixture with inventory registration and §6 requires the suite's tests to pass. The default benchmark-evidence path is unsatisfiable inside the fence; only the fallback survives. The same defect was found and cured by amendment in this wave's T2 brief — a recorded precedent the T3 fence fails to carry. If the fixture is kept, the fence must additionally permit exactly one additive truthful inventory line in each mirror README (per the T2 precedent). |
| D3 | MINOR | §1/C4 wording: `curved_bend_station_resultants` does not receive `pressure_thrust_loads` as a parameter; stations inherit the chord-UX pressure correction through the recovered `corrected_local_forces` (product_physics 1341–1347, 5928–5983). Substantively equivalent to MEMORY's "stations inherit it", but the brief's symbol-level statement is imprecise. |
| D4 | MINOR | §3.3's tangent-consistent recovery predicate inherits D1: the direction/sign of the sectional pressure-axial correction must come from the corrected derivation, not be fixed by this brief's current §2 text. Basis-string truth-maintenance (`pressure_thrust_treatment=…`, line 6896) is otherwise well-targeted. |
| D5 | MINOR | §3.5's optional "work-equivalent (fixed-end) refinement of the radial load" leaves the radial load's sign unspecified ("directed radially", unsigned, in §2); after D1 is cured the refinement clause must name the outward wall load (or the combined self-equilibrated system) explicitly. Boundedness of the clause itself is adequate. |
| D6 | INFO | T1 envelope interaction: T3 proposes no new result kinds; changed recovery basis strings flow through existing result metadata, and §6's read-only `core/runner/headless` test run covers the downstream regression. Acceptable as written. |
| D7 | INFO | Pinned-fixture consequence statement (§3.7) is truthful and appropriately conditional: on the live tree the tp_runner_015 bend (`component:C-110`, `solver_consumption: "mechanics_geometry_only"`, no `bend_pipe_ref`) is not macro-realized, so no output change will occur; the five `del1005_payload_binding_*` inputs contain no bend, so byte-identity is satisfiable. F-PIP-2/DEC-081 fence language present and correctly placed (§9, §10); EffectStatus HELD and OwnerCaseSelection NONE verified (C13). |

---

## 5. Verdict

The brief's governance mechanics, selection facts, fixture-exposure survey,
dependency posture, attribution block, and campaign-authority citations survive
adversarial verification (C1–C5, C7–C9, C12–C14). It fails on the two things that
make this tranche what it is: (1) the "statically-exact" arc treatment at its core
rests on a sign-inverted static identity and a false per-force moment justification
— the proposed end-tangent pair is the equilibrant of the physical outward
pressure-imbalance load, not its equivalent, and every internal gate (derive-first
hand calc, fixture closed forms, unit-test values) is anchored to the same erroneous
pre-specified identities, so execution would land physically inverted bend-pressure
behavior with self-consistent evidence; and (2) the §5 write fence forbids the two
mirror README lines that the benchmark suite's readiness test provably requires for
any new fixture, making the §3.6 default evidence path dead-on-arrival despite this
wave's own T2 amendment precedent. A corrected brief must re-derive §2/§3 around the
self-equilibrated exact system (end-tangent pair plus distributed outward radial
wall load, or a statically/work-consistent nodal reduction of it), restate the
moment argument as pair-cancellation, re-anchor §3.2/§3.6/§4.2 to the corrected
closed forms, and extend the §5 fence with the two additive README inventory lines.

VERDICT: BLOCK

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
