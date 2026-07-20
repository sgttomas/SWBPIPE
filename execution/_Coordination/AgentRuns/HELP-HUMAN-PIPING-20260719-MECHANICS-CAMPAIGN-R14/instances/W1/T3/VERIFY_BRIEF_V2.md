# VERIFY_BRIEF_V2 — Second Fresh-Context Adversarial Verification of T3 DEL-04-01 Arc Pressure-Thrust Candidate Brief (v2 Amendment)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T3 (verification round v2)
**Role:** Fresh-context adversarial verifier (governed Agent 2, non-delegating)
**Objects reviewed:**
- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md` (CB-2026-07-19-T3-DEL-04-01-ARC-PRESSURE-THRUST-001, v2 amendment)
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T3/CURRENT_CANDIDATE_RATIONALE.md` (v2, claims C1–C17)
- Preserved history read, not edited: `instances/W1/T3/VERIFY_BRIEF.md` (v1 BLOCK and worked integration)

**Live tree at verification:** branch `claude/piping-r14-pkg04-mechanics`, HEAD
`faee4faed` — the SAME head the v1 verifier used, so no drift between rounds; I
re-derived the amendment-touched claims in full and spot-checked the carried
v1 confirmations directly against source. Date: 2026-07-19. Offline;
read-only except this file. All physics below was re-derived independently
(symbolic derivation plus a 5-trial randomized numerical integration harness,
2×10⁵–4×10⁵ midpoint panels per integral, run in the session scratchpad);
nothing was accepted on the v1 verifier's or the manager's authority.

---

## 1. Independently Worked Physics (a)–(h)

Setup matching the crate convention (`core/solver/curved_bend/src/lib.rs`
62–74): arc center at the origin of the bend-plane frame, radius R, node i at
arc angle 0, node j at angle φ ∈ (0, π), position r(θ) = R·e_r(θ) with
e_r(θ) = (cos θ, sin θ, 0), unit tangent oriented i→j
t(θ) = (−sin θ, cos θ, 0), outward radial n(θ) = e_r(θ), ds = R dθ.
t_i = (0, 1, 0), t_j = (−sin φ, cos φ, 0).

### (a) Wall-load resultant — CONFIRMED

∫₀^φ q ds = (pA/R)·∫₀^φ e_r(θ) R dθ = pA·(sin φ, 1 − cos φ, 0)
= pA·(t_i − t_j). This is the OUTWARD radial pressure-imbalance load and its
resultant is `pA·(t_i − t_j)`, exactly as the v2 brief §2 states (and with
the sign the v1 verifier proved; the v1 brief had claimed pA·(t_j − t_i)).
Numerics: max abs error ≤ 9.5e-11 across 5 random (R, pA, φ) trials.

### (b) Moments about the arc center — CONFIRMED

- Wall load: r × q ds ∝ e_r × e_r = 0 pointwise, so the total is exactly
  zero (numerics ~1e-17, integration noise).
- End-cap forces: −pA·t_i at r_i contributes moment −pA·R·ẑ; +pA·t_j at r_j
  contributes +pA·R·ẑ (r × pA·e_θ = pA·R·ẑ). Each cap force separately has
  center-moment magnitude pA·R ≠ 0; only the pair-sum vanishes. The v2 text
  states exactly this (the false v1 per-force zero-moment claim is gone).

### (c) Complete-system self-equilibrium — CONFIRMED

Net force: (−pA·t_i) + (+pA·t_j) + pA·(t_i − t_j) = 0 identically. Net
moment about the center: (−pA·R) + (+pA·R) + 0 = 0. With zero net force,
M_P = M_O + (r_O − r_P) × F = M_O for any point P, so the net moment
vanishes about EVERY point, not just the center. Numerically confirmed about
a random off-arc point in each trial (error ≤ 4.6e-10 at pA·R scale). The
§3.1 unit-test predicate ("net force and net moment about any point ≈ 0")
is a true and checkable invariant of the (a)+(b) design.

### (d) Station-tension identity — CONFIRMED (and strengthened)

Segment [0, θ] equilibrium (cap force −pA·t_i at node i plus wall load over
[0, θ], whose resultant is pA·(t_i − t(θ))): external force on the near
segment = −pA·t_i + pA·(t_i − t(θ)) = −pA·t(θ), so the far side acts on the
cut with internal force +pA·t(θ) — tension of magnitude pA along the LOCAL
TANGENT at every interior station. I additionally verified the transverse
(in-plane normal) component is exactly zero AND the internal moment about
the section point is exactly zero: cap-force moment −pA·R·(1 − cos θ) and
near-segment wall moment +pA·R·(1 − cos θ) cancel identically. The complete
system therefore puts the arc in the pure membrane state (uniform tension
pA, zero shear, zero bending) — the closed-vessel wall-tension result
emerging with no ad-hoc chord correction, exactly the §3.3 sharp check.
Numerics: all three identities to ≤ 3e-11 at random interior stations.

Crate-convention cross-check: with the cap forces applied as nodal loads and
the wall load applied as the consistent vector p, the true node-j member
force is (K·d − p)_j = +pA·t_j; feeding that into the
`arc_section_resultants` far-segment pattern gives section force
pA·t_j + pA·(t(θ) − t_j) = pA·t(θ) → axial +pA, and section moment
pA·R·(1 − cos(φ−θ)) + pA·R·(cos(φ−θ) − 1) = 0. The recovery/station design
in §3.3 reproduces the membrane state exactly in the crate's own frames.

### (e) Zero static lumping — CONFIRMED

The natural segment-equilibrium lumping of the wall load to the end nodes is
+pA·t_i at i and −pA·t_j at j (it reproduces the wall load's resultant
pA·(t_i − t_j) and its zero center moment: +pA·R − pA·R = 0). Added to the
cap pair {−pA·t_i, +pA·t_j} this cancels to exactly zero at EACH node. So a
statically-lumped-to-nodes version of the complete system is the zero
vector, and a purely statics-based nodal reduction cannot represent the
load's deformation effect — the exact work-equivalent consistent nodal
vector (with its fixed-end moments) is genuinely required, as §2/§3.1
argue. (Note: static lumping to two nodes is not unique in general; the
brief's indefinite "a statically-lumped-to-nodes version" is satisfied by
the canonical lumping above, and the deformation consequence holds for any
statics-only reduction. Informational, not a defect.)

### (f) Small-angle reduction — CONFIRMED

As φ → 0, t_i, t_j, and the unit chord converge (|t − c| = φ/2 + O(φ²);
verified 5.0e-4 at φ = 1e-3, 5.0e-7 at φ = 1e-6), so the cap pair reduces
to the existing straight-chord treatment {−pA·c, +pA·c}, and the wall load
and its consistent vector vanish as O(φ) (|resultant| = 2pA·sin(φ/2)).
Predicate §3.1's straight-limit claim and the §4.2 chord-reduction unit
test are anchored to a true identity.

### (g) Bourdon-direction sanity check — PASSES, no flag

The complete system produces pure tension pA with zero bending moment at
every station ((d) above). Under rod kinematics (M = 0 ⇒ turning per unit
reference arc length unchanged), the arc lengthens by ε = pA/EA at constant
total turning φ, so the deformed radius is R(1+ε): the pressurized bend
expands radially outward and its curvature decreases — it tends to
open/straighten, never to close. (Full Bourdon straightening additionally
involves cross-section ovalization, a shell effect outside this beam-level
model; what matters here is that the sign is right.) By contrast the
refuted v1 pair-only design applied the net INWARD equilibrant
pA·(t_j − t_i) and would have pulled the bend toward its center — the
inverted behavior the v1 BLOCK caught. The v2 system predicts the correct
direction.

### (h) C15 derivability with existing machinery — CONFIRMED

I derived the radial wall load's internal actions in the crate's own
far-segment convention (load on [θ, φ]) and verified them numerically:

- axial: f·t(θ) = pA·(1 − cos(φ−θ)) — extended-basis coefficients
  pA·[1, −cos φ, −sin φ, 0, 0, 0];
- in-plane moment (z): pA·R·(cos(φ−θ) − 1) — coefficients
  pA·R·[−1, cos φ, sin φ, 0, 0, 0];
- out-of-plane moment and torsion: identically zero (the load lies in the
  bend plane, w_z = 0).

Every action of the rotating radial intensity lies in the PLAIN
{1, cos θ, sin θ} sub-basis — strictly simpler than the uniform-load case,
whose actions need the full extended {1, cos, sin, θ, θ·cos, θ·sin} family
(`distributed_load_actions`, lib.rs 575–624). The existing exact Gram
machinery (`trig_gram` 704–721, `trig_extended_gram` 629–671, `quad`,
`cross_quad`) already integrates all required products; the tip-deflection,
clamped-tip-redundant, and rigid node-i-share steps of
`consistent_uniform_nodal_loads` (276–343) transfer unchanged, with the
load resultant (pA·(sin φ, 1 − cos φ, 0)) and its node-i moment
(pA·R·(cos φ − 1)·ẑ) also in closed form. The same closed forms serve the
station path (the `arc_section_resultants` segment-equilibrium pattern,
359–449). So the exact work-equivalent consistent nodal vector required by
§3.1(b) is derivable as ADDITIVE closed-form code with no new integration
technology, and the family citation to the TP-PMM-P1-CURVEDBEND-004 landing
is accurate (DEL-04-01 `MEMORY.md` lines 381–385). The fail-closed gate is a
real gate that is not expected to fire — which is the correct posture.

---

## 2. Per-Claim Verification Table (C1–C17)

| Claim | Verdict | Evidence (paths relative to `WORKING_ROOT`; live tree at `faee4faed`) |
|---|---|---|
| C1 | **CONFIRMED** | DEL-04-01 `_STATUS.md`: `Current State: IN_PROGRESS` (line 3); `## Remaining` has exactly two items (lines 6–8); first verbatim "Treat arc pressure-thrust beyond the recorded straight-chord treatment for curved-bend macro spans (source: Receipt 11 named remainder / TP-PMM-P1-CURVEDBEND-004 boundaries)"; second is the G1/G2/G4 + M2/M3 row with `(gated: owner re-disposition where not closed by evidence)`. |
| C2 | **CONFIRMED** | `MEMORY.md` lines 379–411, TP-PMM-P1-CURVEDBEND-004 entry; exact residual sentence at 407–408: "Pressure thrust keeps the recorded straight-chord axial treatment on macro spans (stations inherit it); D-38 temperature semantics untouched." |
| C3 | **CONFIRMED** | `core/product_physics/src/lib.rs`: `add_pressure_thrust_loads` (5694–5717) applies ∓/±`axial_load·local_x` with the straight pipe's chord `local_axes[0]` for every load, no macro-span branch (contrast `add_thermal_equivalent_loads` 5719–5747, which branches on `curved_bends_by_pipe`); inline comment 1140–1142 records the straight-chord decision at the assembly call (1143). |
| C4 | **CONFIRMED** (persisting minor wording imprecision, see defect M1) | `corrected_local_forces_for_axial_effects` (5784–5803) adds pressure axial in chord `UX`; `recover_curved_bend_local_forces` receives `pressure_thrust_loads` (signature 5814–5821; chord-UX correction ~5877–5881; call 1295–1302); `curved_bend_station_resultants` (5928–…) receives `corrected_local_forces`, NOT `pressure_thrust_loads` — stations inherit through recovery (call 1341–1347), matching MEMORY's "stations inherit it" but not the claim's literal parameter wording. `pressure_thrust_for_pipe` gates `include_pressure_longitudinal` (1405–1407). |
| C5 | **CONFIRMED** | `core/solver/curved_bend/src/lib.rs`: `geometry()` 149, `radius()` 180, `included_angle()` 184, `arc_length()` 188, `orientation()` 194, `consistent_uniform_nodal_loads()` 276, `arc_section_resultants()` 359 with section frame "x tangent toward node j" (348, 437–448); `ArcGeometry` frame doc 62–74; tangent algebra in tests 1617–1640 (`tangent_j = [−sin φ, cos φ, 0]`, `tangent_i = [0, 1, 0]`). Unit end tangents derivable with no new geometry source. |
| C6 (v2) | **CONFIRMED** | Every part independently re-derived and numerically verified in §1(a)–(f) above: outward-load resultant pA·(t_i − t_j); zero wall moment about the center; per-cap moment pA·R with pair-sum zero; exact self-equilibrium (zero force, zero moment about any point); zero static lumping; +pA local-tangent station tension (plus zero shear and zero moment — membrane state); chord reduction as φ → 0. Consistent with, and independently reproducing, the v1 worked integration. |
| C7 | **CONFIRMED** | `Dependencies.csv`: exactly 8 `EXECUTION` `UPSTREAM` rows (DAG-002-E0100..E0104 CONSTRAINT on DEL-00-01/02/03/06/08 + E0429/E0430/E0431 PREREQUISITE on DEL-02-01/02-02/02-03), all `SATISFIED`. |
| C8 | **CONFIRMED** | Mechanics inventory (`validation/benchmarks/mechanics/src/lib.rs` 547–571, 22 fixtures): full-body scans of `expansion_loop_curved_bend_thermal_fixture` and `curved_bend_distributed_fixed_end_fixture` show zero pressure content; nonlinear suite models all carry `curved_bend_elements: Vec::new()`; stress suite is section-level. No `del1005_payload_binding_*` input contains any bend (grep over all five inputs: no match). Pinned `tp_runner_015_final_cli_solve_input.json` bend `C-110`: `solver_consumption: "mechanics_geometry_only"`, no `bend_pipe_ref`; macro realization requires `curved_bend_macro_element` plus a resolvable `bend_pipe_ref` (`is_curved_bend_macro_component` 3277–3281, `DEC_070_CURVED_BEND_SOLVER_CONSUMPTION` line 85, `curved_bend_realized_pipe_ids` 3288–3300). |
| C9 | **CONFIRMED** | `validation/benchmarks/mechanics/README.md` line 70 records the "DEC-026 analytic-class `1.0e-9` relative tier" as the comparison basis in use; corroborated by MEMORY 404–406. New-fixture comparison at the recorded tier is reuse, not tolerance creation. |
| C10 (v2) | **CONFIRMED** | The v1 fence gap is cured: §5 item 5 now names BOTH mirror READMEs (`validation/benchmarks/mechanics/README.md`, `validation/hand_calcs/mechanics/README.md`), one additive inventory line each, plus the fixture-count assertion bump — exactly what `readiness_metadata_matches_documented_boundaries` (suite lib.rs 5805–5838, `include_str!` 54–55; asserts fixture_id in both READMEs and the hand-calc file name in the hand-calc README) and `assert_eq!(fixtures.len(), 22)` (5643) require. Task-to-fence map: §4.1 → item 7 only; §4.2 → items 2, 3, 4; §4.3 → item 5 + ephemeral captures + item 7; §4.4 → items 6, 7; §6 containment JSON → item 7. Nothing materially more is opened; curved_bend writes additive-only by predicate §3.5. Matches the landed T2 precedent (commit `faee4faed`: inventory 21→22, exactly `+1` line in each mirror README). |
| C11 (v2) | **CONFIRMED** | Independent 10-class re-screen in §3 below: no class hit. The v1 class-9 hit (erroneous physics identities as the outcome-determining premise) is cured — every §2 identity now anchoring §3.1/§3.2/§3.3/§3.6/§4.2 was independently re-derived true in §1. No owner-gated method fork (see §3 class 1/4 probe). |
| C12 | **CONFIRMED** | `tools/validation/validate_claims_language.py`, `tools/validation/validate_path_anchors.py`, `tools/software_workflow/validate_change_scope.py` all present at `REPO_ROOT`; all four named Cargo manifests exist; per-crate `target/` dirs present (product_physics, curved_bend, benchmarks/mechanics, runner/headless) and the cargo registry cache is provisioned — §6 is executable offline. |
| C13 | **CONFIRMED** | Brief §10: `AgentClassification: CLASSIFY_ELIGIBLE` distinct from `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL` (SHA-bound); `OwnerCaseSelection: NONE`; `EffectStatus: HELD`; preserved gates enumerated; `IndependentVerifier: PENDING (v2)` with the v1 BLOCK named truthfully and unsoftened ("physics sign-inversion refuted by worked integration; adopted into the v2 basis"). |
| C14 | **CONFIRMED** (same caveat as v1) | `ORCHESTRATION_PLAN.md`: T3 is the named W1/PKG-04 queue item (line 20); managers author per-tranche briefs, per-tranche commits, one PR per wave, HELP_HUMAN merges (lines 37–48) — no manager push/PR/merge/receipt. Caveat carried from v1: plan literal text gates each tranche commit on a check set "including the DEC-025 evidence sweep" (45–46), while the wave-level single-sweep refinement rests on the controlling W1 dispatch as recorded in the T1 brief (§5 item 9, §6 — "run once at W1 closeout"); T1 (COMMIT-SAFE at v3) and T2 both landed under that recorded refinement, so it stands as accepted in-wave precedent. |
| C15 (v2) | **CONFIRMED** | §1(h) above: all internal actions of q(θ) = (pA/R)·n(θ) lie in the plain {1, cos, sin} sub-basis of the crate's extended family; existing Gram/quad machinery integrates every needed product; resultant and node-i moment closed forms exist; the consistent vector is derivable as additive code with no new integration technology. MEMORY.md 381–385 records the extended-basis family exactly as cited. |
| C16 (v2) | **CONFIRMED** | Pinned fixture inspected directly: bend `C-110` has `solver_consumption: "mechanics_geometry_only"` and the component JSON contains no `bend_pipe_ref`; realization requires `curved_bend_macro_element` + resolvable `bend_pipe_ref`. So §3.7's UNCHANGED expectation is the truthful consequence statement and "any change = failure" is the right guard posture. Additionally, no committed witness pins the `pressure_thrust_treatment=straight_chord_axial_end_forces` basis string (repo grep: only `core/product_physics/src/lib.rs` line ~6896 plus historical run/concordance records), so the truthful basis-string update stays fence-contained. |
| C17 (v2) | **CONFIRMED** | Both READMEs contain exactly 22 distinct `MECH-*` fixture ids each (post-T2), matching the 22-entry `fixture_inventory()`; a repo-wide grep for a sample fixture id (`MECH-CANTILEVER-TIP-FORCE`) hits ONLY these two READMEs — they are the only inventory mirrors; the readiness test enforces exactly these two; the v2 fence permits exactly one additive line in each and §6 verifies the one-line diffs. |

Tally: **17 of 17 CONFIRMED** (C4 and C14 carry the same minor caveats the v1
round recorded; neither is material).

---

## 3. Independent 10-Class D-52 §4.1 Fast-Reject Re-Screen (v2 brief)

Re-derived item by item against
`execution/_Coordination/_DECISIONS/D-52_four_lens_standing_approval_overlay.md`
§4.1, read with the D-54/`DEC-087` reasoned-selection refinement
(`D-54_reasoned_discretion_standing_approval_refinement.md` §3.1–§3.2).
Ambiguity is a hit.

1. **Irreducible owner preference / owner-gated method fork:** not hit —
   probed hard, as tasked. Is "cap pair + exact consistent wall vector"
   uniquely determined by the accepted basis, or is there a competing
   defensible pressure-load model (e.g. an effective-tension-only
   formulation that applies no structural pressure loads and carries
   pressure solely into stress recovery)? Finding: the project's method —
   pressure thrust applied as external nodal loads on every span — is
   already recorded, accepted, live code (`build_pressure_thrust_loads` /
   `add_pressure_thrust_loads`; the inline-comment decision record; the
   review-row basis string), and the selected Remaining item directs
   improving THAT method's macro-span treatment "beyond the recorded
   straight-chord treatment". Within that recorded method, the complete
   self-equilibrated system is uniquely determined: it is the exact
   fine-discretization limit of the current straight-span pair applied to a
   polyline arc (v1 §2.4, adopted basis), and my §1 derivation confirms it
   is the unique arc treatment that is exactly self-equilibrated, reproduces
   the closed-vessel membrane state, and reduces to the chord treatment at
   φ → 0. An effective-tension-only formulation would CHANGE the recorded
   accepted method — a different matter, outside this tranche's scope, and
   not a surviving defensible alternative under the accepted basis. What
   multiplicity remains (pair-only, lumping, multiplier, chord retention,
   parking) is settled by exact statics or recorded direction, and the
   rationale §3 rejects each with sound, truthful reasoning (including the
   new v2 entries 7–9). D-54 §3.2 reasoned selection with recorded
   rationale is the correct lane; no owner ruling is required.
2. **Professional/safety/legal accountability:** not hit. Preview mechanics
   on invented fixtures; regression-evidence posture preserved; no reliance
   claim; hand-calc is elementary statics.
3. **Conflict ruling undetermined by authority chain:** not hit. The chord
   treatment is a recorded interim decision whose own recorded residual
   directs this replacement; the v1 BLOCK is adopted, not contradicted.
4. **Scope/boundary/normative/acceptance-criteria change:** not hit.
   Recorded deliverable scope; DEC-026 tier reused as recorded (README line
   70); no threshold or criterion created; basis-string edits are truth
   maintenance; the two README lines are truthful inventory listings the
   suite's own readiness test requires, not claim-posture text.
5. **Lifecycle/stage/release/evidence-posture act:** not hit. `IN_PROGRESS`
   unchanged; suite claim posture unchanged; recorded values are regression
   evidence only; fallback clause is recorded-fallback, not promotion.
6. **External/procurement/publication action:** not hit. Offline; §5/§9
   forbid push/PR/merge/network.
7. **Merge authority / destructive action:** not hit. Ordinary wave-branch
   commits under the campaign plan; existing fixtures/witnesses byte-
   preserved by predicate §3.7; pinned-case any-change-is-failure guard.
8. **Protected/private data exposure:** not hit. Invented geometry,
   elementary statics; §9 expressly excludes code-book SIF/flexibility
   values and protected tables; the derivation in §1 needed none.
9. **Evidence unavailable / stale basis / claim beyond warrant:** not hit —
   this was the v1 hit, and it is cured. Every outcome-determining §2
   identity is now true (independently re-derived, §1); the §1 basis facts
   are current on the live post-T2 head (same head as the v1 round;
   re-spot-checked: chord treatment, inline comment, recovery inheritance,
   curved-bend API, fixture exposure, DEC-026 tier, del1005/pinned posture,
   README mirrors at 22); the derive-first gate now targets correct
   identities and is cross-checked against the preserved v1 integration,
   so it can actually catch executor error; C15 grounds §3.1(b)'s
   derivability claim in the live crate machinery. No claim exceeds its
   warrant.
10. **Protected domain-engine paths / prover / higher-order boundaries:**
    not hit.

**Re-screen result: PASS (no class hit).** The v1 class-9 defect is cured at
its root: the gates (§3.1 self-equilibrium test, §3.2 witness integrals,
§3.3 sharp check, §4.2 derive-first with v1-integration cross-check, §3.6
fixture closed forms) are all anchored to identities this round re-proved
independently, and they are no longer self-consistent-with-error: a
pair-only or sign-inverted implementation would now fail the self-equilibrium
unit test, the +pA station check, and the witness cross-check.

---

## 4. Remaining Defects

| ID | Severity | Defect |
|---|---|---|
| M1 | MINOR (carried from v1 D3; not blocking) | Brief §1 / rationale C4 still say the station path (`curved_bend_station_resultants`) "receive[s] `pressure_thrust_loads`"; literally it receives `corrected_local_forces` and inherits the pressure correction through recovery (calls 1295–1302, 1341–1347). Substantively equivalent to MEMORY's "stations inherit it"; the §4.1 freeze-check phrasing ("recovery inheritance") is compatible. No gate anchors to the imprecise wording. |
| M2 | INFO | §2's "statically-lumped-to-nodes version of the complete system cancels to zero": static lumping to two nodes is not unique in general; the canonical segment-equilibrium lumping does cancel node-by-node exactly (§1(e)), and the stated consequence (consistent vector required for deformation) holds for any statics-only reduction. The indefinite phrasing is defensible; suggest the executor's hand-calc witness state the canonical lumping explicitly, which §3.2 already requires ("the zero-net static lumping consequence"). |
| M3 | INFO (carried from v1 C14 caveat) | Campaign-plan literal text ("each gated by the tranche's full check set including the DEC-025 evidence sweep") vs the wave-level single-sweep refinement recorded in the T1 brief §5.9/§6 under the controlling W1 dispatch. T1 (COMMIT-SAFE v3) and T2 landed under the refinement; it stands as accepted in-wave precedent. No action for T3, which cites it correctly. |
| M4 | INFO | T1 interaction: T3 proposes no new result kinds; the truthful basis-string change flows through existing result metadata, no committed witness or downstream test pins the `pressure_thrust_treatment` string (repo grep), and §6's read-only `core/runner/headless` test run guards the downstream surface. F-PIP-2/DEC-081 fence language present and correctly placed (brief §9 end, §10; rationale end). T2 interaction: surfaces disjoint (constant-effort supports; fixture 22 of 22), no overlap with the pressure path. |

No MAJOR or blocking defect survives. Both v1 blocking defects (D1 physics,
D2 fence) are verifiably cured; v1 minors D3–D5 are resolved or reduced to
M1/M2 above (D4's recovery-direction concern is mooted by the corrected §2
basis; D5's unsigned-radial-load concern is mooted by §2 naming the outward
wall intensity explicitly and §3.1 requiring it).

---

## 5. Verdict

The v2 amendment does what it claims. The physics core — end-cap pair
{−pA·t_i, +pA·t_j} PLUS the exact work-equivalent consistent nodal vector of
the outward radial wall intensity q(θ) = (pA/R)·n(θ) — was re-derived here
from scratch and is exactly right: self-equilibrated (zero force, zero
moment about any point), zero under static lumping, membrane-exact (+pA
local-tangent tension, zero shear, zero moment at every station), correctly
reducing to the chord treatment at small angle, and predicting the correct
opening/straightening tendency for a pressurized bend. The consistent vector
is derivable with the crate's existing closed-form trigonometric machinery
(the radial load's actions lie in the plain {1, cos, sin} sub-basis), so the
fail-closed gate is sound and §3.1(b) is implementable additively. The fence
now carries the two mirror-README inventory lines the suite's readiness test
provably enforces (landed T2 precedent), the pinned-fixture statement matches
the live tree (not macro-realized; any change = failure), and the gates are
no longer self-consistent-with-error: an inverted or pair-only implementation
would fail the self-equilibrium, station-tension, and witness checks. All 17
enumerated claims are confirmed; the independent 10-class re-screen passes
with the v1 class-9 hit cured at its root; attribution (`EffectStatus: HELD`,
`OwnerCaseSelection: NONE`, truthful unsoftened v1-BLOCK history) is in
order. Remaining findings are two carried minor wording caveats and two
informational notes, none load-bearing.

VERDICT: COMMIT-SAFE

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
