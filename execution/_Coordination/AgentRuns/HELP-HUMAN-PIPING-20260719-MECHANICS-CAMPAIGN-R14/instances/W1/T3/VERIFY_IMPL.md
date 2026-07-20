# VERIFY_IMPL — Fresh-Context Implementation Verification of T3 DEL-04-01 Arc Pressure-Thrust (Pre-Commit)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T3 (implementation verification)
**Role:** Fresh-context implementation verifier (governed Agent 2, non-delegating)
**Objects reviewed:**
- Executor return `instances/W1/T3/EXECUTE_RETURN.md` (claims E1–E20)
- Governing sealed brief `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md` (v2 EFFECTIVE; §3 predicates, §5 fence, §6 checks, v2 amendment record)
- Verifier history: `instances/W1/T3/VERIFY_BRIEF.md` (v1 BLOCK, worked integration) and `instances/W1/T3/VERIFY_BRIEF_V2.md` (v2 COMMIT-SAFE, independent re-derivation) — both read in full
- The live UNCOMMITTED working tree on branch `claude/piping-r14-pkg04-mechanics` against base `faee4faed`

Date: 2026-07-19. Offline throughout (`CARGO_NET_OFFLINE=true`, `--offline`);
read-only except this file; ephemeral captures in the session scratchpad only.
Every physics identity below was checked against the v2 verifier's derivation
AND re-derived by me directly against the crate's frame conventions
(`ArcGeometry` doc, `arc_section_resultants` section frame, `unit_load_actions`
sign conventions); closed-form fixture numerics were recomputed independently
in python3. No claim was accepted on the executor's authority.

---

## 1. Independent Physics Reading of the Implementation (tasks (a)–(e))

### (a) New crate methods — sign conventions and closed forms CORRECT

Frame ground truth re-read from source: local x = radial center→node i, z =
bend-plane normal, node i at arc angle 0, node j at `phi`; so
`t(theta) = (-sin theta, cos theta, 0)`, `t_i = (0,1,0)`,
`t_j = (-sin phi, cos phi, 0)` — exactly the v2 verifier's setup.

- `end_tangents()` returns `rotate_to_global(local_axes, (0,1,0))` and
  `rotate_to_global(local_axes, (-sin phi, cos phi, 0))`: the correct unit
  end tangents, i→j oriented, from the validated geometry only (no new
  geometry source). Verified against the frame doc and the pre-existing
  tangent algebra in the crate tests.
- `consistent_radial_pressure_nodal_loads(thrust)`:
  - Wall resultant coded as `thrust*(sin phi, 1-cos phi, 0)` = `pA(t_i - t_j)`
    — the OUTWARD wall load's resultant (the v2 sign, not the refuted v1
    inversion). I re-derived it: `∫ q ds = (pA/R)∫ e_r R dθ = pA(sin phi,
    1-cos phi, 0)`. CORRECT.
  - Node-i wall moment coded as `thrust*R*(cos phi - 1)` about local z. I
    re-derived: `∫ (r(θ)-r_i) × pA e_r dθ = -pA R (1-cos phi) ẑ`. CORRECT.
  - `radial_pressure_load_actions`: axial `pA[1, -cos phi, -sin phi]` and
    in-plane moment `pA·R[-1, cos phi, sin phi]` over the plain
    `{1, cos, sin}` basis. I re-derived both from far-segment equilibrium:
    axial `pA(1-cos(phi-θ))`, moment `pA·R(cos(phi-θ)-1)`; the coefficient
    expansions match exactly. Out-of-plane/torsion identically zero (load in
    bend plane) and carry no code path. CORRECT and exactly the v2 §1(h)
    closed forms.
  - Structure mirrors `consistent_uniform_nodal_loads` line-for-line
    (tip deflection by unit-load theorem with the SAME strain-energy weights
    as `end_flexibility` — in-plane factor on bending, plain `trig_gram` +
    `quad` since the radial actions need only the plain sub-basis — then
    `X = -K_jj δ0`, `p_j = -X`, `p_i = H X + W_i` with the same
    `equilibrium_transfer(chord_local)`; identical block rotation to global).
    I verified the equilibrium algebra: force sum `p_i + p_j = W_i,f =
    pA(t_i - t_j)` and the moment identity through the transfer, i.e. the
    vector is statically equivalent to the wall load by construction.
- `arc_section_resultants_with_radial_pressure` adds exactly three terms to
  the unmodified `arc_section_resultants` result: axial
  `+pA(1-cos(phi-θ))`, slot-1 `-pA sin(phi-θ)`, slot-5
  `+pA·R(cos(phi-θ)-1)`. I checked the slot-1 sign against the crate's
  section frame (y = INWARD radial): far-segment wall force
  `pA(t(θ)-t_j)` dotted with inward gives `-pA sin(phi-θ)` — the code's
  sign is right for the crate's own frame; combined with the true node-j
  membrane force `+pA t_j` the shear cancels identically, which the tests
  confirm empirically at three stations. Zero-thrust reduction is exact by
  construction and unit-tested at five fractions.
- Membrane state produced by the implementation: axial `+pA` TENSION (the
  closed-vessel wall tension, NOT `-pA` compression), zero shear, zero
  internal moment; tip displacement `(pA R/EA)(-(1-cos phi), sin phi, 0)`
  moves the free node radially OUTWARD (`u·n > 0` at node B) — the OPENING
  Bourdon direction, opposite to the refuted v1 pair-only design.

### (b) product_physics macro branch and recovery CORRECT

- `add_pressure_thrust_loads` gains one branch at the top of the per-load
  loop: macro membership (`curved_bends_by_pipe.get(&load.element_index)`)
  routes to `add_curved_bend_pressure_thrust_load` and `continue`s; the
  straight-span arm below is textually identical to the pre-change body
  (diff shows insertions only in that function).
- Cap pair signs in `add_curved_bend_pressure_thrust_load`:
  `force[i] -= axial_load * tangent_i` (i.e. `-pA·t_i` at node i) and
  `force[j] += axial_load * tangent_j` (`+pA·t_j` at node j). CORRECT.
  The consistent wall vector is ADDED to the force vector through
  `element_dof_map` (all 12 slots, forces and moments). Branch keys on
  `element_index` only, so both thrust sources get identical treatment
  (unit-tested with a mixed two-source list; assembled vector equals cap
  pair + wall at the summed thrust).
- Recovery (`recover_curved_bend_local_forces`): the consistent wall vector
  is SUBTRACTED in the GLOBAL frame alongside the uniform equivalent-load
  subtraction, before the chord-frame rotation — so recovered end forces
  are `K(d - u_free) - p_uniform - p_wall`, the true member forces of the
  continuously loaded arc. The cap pair is (correctly) NOT subtracted: cap
  forces are nodal external loads, and the crate contract for true member
  forces is `K d - p` with `p` the consistent (element-distributed) vector
  only. No double-count, no sign flip: assembly ADDS `+p_wall`, recovery
  SUBTRACTS `+p_wall`, exactly the uniform-load pattern.
- The ad-hoc chord-UX pressure correction
  (`local_forces[UX] += pressure_axial_load` / `-=` at j) is REMOVED from
  the macro recovery path only; `corrected_local_forces_for_axial_effects`
  (the straight-span path) has zero diff hunks — byte-identical to base.
- `curved_bend_station_resultants` gains the `pressure_thrust` parameter,
  fed at its single call site by `pressure_thrust_for_pipe(pipe_index, …)`,
  and forwards it to `arc_section_resultants_with_radial_pressure`.
- The one assembled `force` vector flows unchanged to `reduce_system` and to
  `append_nonlinear_support_loop_results(…, &force, …)` (call site
  re-read at head; no diff hunk touches it) — linear, sparse, and nonlinear
  paths consume the identical complete arc loading.
- Review-row basis at head emits only
  `pressure_thrust_treatment=arc_end_cap_tangent_pair_plus_consistent_radial_wall_load`
  (line ~6965); the old `straight_chord_axial_end_forces` string appears
  nowhere in production code — only inside the updated test's negative
  assertion. Remaining `straight_chord` matches are the unchanged
  legacy-mode test name.

### (c) New unit tests re-run and read — they assert what is claimed

All suites re-run by me (results in §4). Test-content reading:

- Sharp check `curved_bend_macro_span_pressure_shows_membrane_station_state`
  (product_physics): end-supported (anchored-free) pressurized macro span
  with no other load; at ALL THREE interior stations (`quarter-1`,
  `midspan`, `quarter-3` — the DEC-026-tier fractions 0.25/0.5/0.75)
  asserts axial `= round6(pA)` within `1.0e-9·pA` and shear-y, shear-z,
  torsion, bending-y, bending-z each `≤ 1.0e-9·pA`; PLUS membrane end
  forces `∓pA·t` with zero end moments, station stress `pA/A_s`,
  closed-form flexibility-independent tip stretch cross-checked against the
  direct macro-element oracle, hoop row present, pressure-longitudinal row
  absent, and no `PRESSURE_LOAD_NOT_APPLIED_TO_FRAME_VECTOR`. This genuinely
  asserts `+pA` local-tangent axial with zero shear and zero internal
  moment at multiple interior stations within the recorded tier.
- Self-equilibrium: `cap_pair_plus_consistent_radial_load_is_self_equilibrated`
  (crate) checks net FORCE (`≤1e-12·pA`) AND net MOMENT about the arbitrary
  off-arc point `[1.9, -0.8, 0.6]` (`≤1e-12·pA·R`), plus the node-by-node
  canonical-lumping cancellation. The product-side test repeats both checks
  on the assembled vector about `[0.7, -1.3, 0.4]` at `1e-12` of scale.
- `consistent_radial_pressure_load_satisfies_rigid_equilibrium` verifies the
  wall resultant `pA(t_i - t_j)` and zero total center moment on an
  axis-aligned AND a shifted/rotated arc.
- `anchored_arc_under_complete_pressure_system_reaches_membrane_state`
  verifies true end forces, zero end moments, closed-form tip displacement,
  and the membrane station state for (k_in,k_out) ∈ {(1,1),(2.5,1.75)}.
- Chord reduction, zero-thrust reduction, and non-finite rejection tests
  present and correct as claimed.

### (d) Independent numeric spot-check of the hand-calc closed forms

Recomputed in python3 from the invented inputs (D_o=0.2191, t_w=0.0081,
E=195e9, p=2.5e6, R=1.4, Phi=pi/2):

| Quantity | My value | Witness value | Δ |
|---|---|---|---|
| A (internal) | 3.233359360399314e-2 | same | 0.0 |
| A_s (steel) | 5.369296004250315e-3 | same | 0.0 |
| F_p = pA | 8.083398400998286e4 | same | 0.0 |
| E·A_s | 1.0470127208288115e9 | 1.047012720828812e9 | 4.8e-7 (last-digit rounding of the printed value; 4.6e-16 relative) |
| u_x = -F_p·R/(E·A_s) | -1.0808615345608499e-4 | -1.080861534560850e-4 | 1.4e-20 |
| u_y | +1.0808615345608499e-4 | +1.080861534560850e-4 | 1.4e-20 |

The tip-displacement closed form itself was re-derived two independent ways
(unit-load theorem on the membrane state; deformed-geometry stretch at
constant turning) and matches the witness §8 and the fixture table.

### (e) Benchmark fixture vs hand calc, and the measured deviation

`CBPT_WITNESS_ROWS` records exactly the hand-calc table values (both k rows
identical, as the membrane closed form requires); every `ExpectedValue` has
`tolerance_policy: None`; `CBPT_RELATIVE_TOLERANCE = 1.0e-9` reuses the
recorded DEC-026 analytic tier; `CBPT_NEAR_ZERO_SCALE_FLOOR = 1.0` follows
the existing fixture-local near-zero-floor convention (exact-zero comparison
floor, not a new acceptance surface). I re-ran the witness-table test with
`--nocapture`: **measured normalized deviation 1.4551915228366852e-11** —
reproducing the executor's "~1.5e-11" and sitting ~68× inside the tier.

## 2. Independent Fence-Containment Derivation

From `git status --porcelain` and `git diff --stat faee4faed` (re-derived, not
copied):

| Changed/untracked path | Fence disposition |
|---|---|
| `core/product_physics/src/lib.rs` (M, 443/21) | §5.2 |
| `core/solver/curved_bend/src/lib.rs` (M, 549/0) | §5.3 (additive-only verified below) |
| `DEL-04-01 …/_STATUS.md` (M) | §5.6 |
| `DEL-04-01 …/MEMORY.md` (M, 49/0) | §5.6 |
| `validation/benchmarks/mechanics/README.md` (M, 1/0) | §5.5 |
| `validation/benchmarks/mechanics/src/lib.rs` (M, 388/1) | §5.5 |
| `validation/hand_calcs/mechanics/README.md` (M, 1/0) | §5.5 |
| `DEL-04-01 …/_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T3_ARC_PRESSURE_THRUST.md` (??) | §5.6 |
| `…/instances/W1/T3/` (?? — EXECUTE_RETURN.md, CHANGE_SCOPE_CONTAINMENT.json, plus prior verifier/rationale artifacts) | §5.7 |
| `validation/hand_calcs/mechanics/curved_bend_pressure_thrust_arc.md` (??) | §5.4 |
| `…/AgentRuns/…/ORCHESTRATION_PLAN.md` (??) | lawful non-executor R14 state (set aside per tasking) |
| `…/instances/W3/` (??) | lawful non-executor R14 state (set aside per tasking) |
| `…/CANDIDATE_BRIEF_2026-07-19_T3_…ARC_PRESSURE_THRUST.md` (??) | §5.1 / lawful R14 state |

Nothing else is modified or untracked. No Cargo.toml/Cargo.lock changed. No
path under `validation/witness/**`, `core/runner/**`, docs, schemas, DAG,
receipts, or any §9 surface is in the diff (verified:
`git diff --name-only faee4faed -- …/validation/witness/` is empty).
**Containment: PASS by my own derivation.** I additionally re-ran
`tools/software_workflow/validate_change_scope.py` myself with the §5
allowed set: `status: PASS, violations: []` — agreeing with the executor's
persisted `CHANGE_SCOPE_CONTAINMENT.json` (which parses as valid JSON).

Strict additivity of the crate diff verified two ways: `git diff --numstat`
= 549 insertions / 0 deletions, and an independent difflib opcode comparison
of base vs head file contents: three pure `insert` blocks (168 + 29 + 352
lines), zero deletes/replaces. Test-count deltas: curved_bend 16→23
(7 new), product_physics 83→86 (3 new; 1 pre-existing basis-string test
updated as the brief directs), benchmarks/mechanics +2.

## 3. Per-Claim Verification Table (E1–E20)

| Claim | Verdict | Evidence |
|---|---|---|
| E1 | **CONFIRMED** | Three new public methods present; difflib opcodes show pure insertions only (§2); all 16 pre-existing crate tests pass unchanged in my re-run. |
| E2 | **CONFIRMED** | Side-by-side read of `consistent_radial_pressure_nodal_loads` vs `consistent_uniform_nodal_loads`: identical X/p_j/p_i/rotation structure; load actions match my independent closed forms (§1(a)); plain `{1,cos,sin}` basis via pre-existing `trig_gram`/`quad`; out-of-plane/torsion identically zero with no code path. |
| E3 | **CONFIRMED** | `load_resultant_at_i` force `pA(sin phi, 1-cos phi, 0)`, moment `pA·R(cos phi - 1)ẑ` — both re-derived by me; crate test checks force-sum and zero center moment on axis-aligned + shifted arcs (test read in full). |
| E4 | **CONFIRMED** | Test read: net force ≤1e-12·pA, net moment about off-arc `[1.9,-0.8,0.6]` ≤1e-12·pA·R, node-by-node lumping cancellation; passes in my re-run. |
| E5 | **CONFIRMED** | Test read: both (k_in,k_out) pairs; end forces `∓pA·t` ≤1e-9; zero end moments; closed-form tip `(pA R/EA)(-(1-cos φ), sin φ, 0)` with zero out-of-plane/rotations; stations {0.25,0.5,0.75} membrane state ≤1e-9. |
| E6 | **CONFIRMED** | Exactly three added terms (my §1(a) sign check incl. the inward-y shear convention); zero-thrust equality test at five fractions; `arc_section_resultants` itself untouched (additive diff). |
| E7 | **CONFIRMED** | Diff: macro branch is a pure insertion at loop top with `continue`; straight-span arm all context lines; single call site at the same assembly seam (after `add_uniform_element_loads`, before thermal/constant-effort/`reduce_system`), one `force` vector to all solve paths. |
| E8 | **CONFIRMED** | Branch keys on `element_index` only; mixed-source product test read and passing (cap pair + wall at summed thrust; fp self-equilibrium; empty-list no-op). |
| E9 | **CONFIRMED** | Deleted-lines listing shows the chord-UX block (`local_forces[UX] += …`) removed; wall vector subtracted in global frame pre-rotation; `corrected_local_forces_for_axial_effects` has zero hunks. |
| E10 | **CONFIRMED** | New `pressure_thrust` parameter fed by `pressure_thrust_for_pipe` at the single call site; forwarded to `arc_section_resultants_with_radial_pressure`. |
| E11 | **CONFIRMED** | Sharp-check test read in full (§1(c)); passes in my re-run of the 86-test suite. |
| E12 | **CONFIRMED** | Test present and passing; `append_nonlinear_support_loop_results(…, &force, …)` call site unchanged and consumes the same assembled vector (re-read at head). |
| E13 | **CONFIRMED** | Grep at head: new string emitted at the single production site; old string only inside the updated test's negative assertion; remaining `straight_chord` match is the unchanged legacy-mode test name. |
| E14 | **CONFIRMED** | Suite diff 388/1 with the single deletion being `assert_eq!(fixtures.len(), 22)`→23; new family variant + CBPT block all additive; `tolerance_policy: None` on all 12 expected values; no existing fixture value or README posture text touched (README diffs are exactly the one inventory line each). |
| E15 | **CONFIRMED** | k=1/k=2 witness rows identical (membrane closed form); flexibility-independence test present; measured deviation 1.4551915228366852e-11 reproduced by my `--nocapture` re-run. |
| E16 | **CONFIRMED** | My own runs: pinned solve exit 0, stdout SHA-256 `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613` (equals the executor's before/after AND T2's recorded wave-head after-SHA — continuity verified); five del1005 outputs `cmp` byte-identical to committed witnesses, exits 0/0/0/1/1; committed-witness SHAs recomputed and equal to the executor's five recorded SHAs; zero diff paths under `validation/witness/**`. |
| E17 | **CONFIRMED** | §2: my independent containment derivation and my own validator re-run (PASS, 0 violations) agree; residual untracked entries are the lawful non-executor R14 state. |
| E18 | **CONFIRMED** | `_STATUS.md` diff: exactly the first Remaining line removed; owner-gated second row is an unchanged context line (byte-identical); `IN_PROGRESS` retained; one new History line; `Last Updated` 2026-07-19; no other deliverable touched; the only added "Remaining"-mentioning line in the whole diff is that History sentence — no new Remaining row anywhere. |
| E19 | **CONFIRMED** | All my re-runs completed offline from cache; diff touches no thermal/D-38, tolerance-policy, SIF, schema, docs, runner, or witness surface; no commit/branch/merge performed (working tree still uncommitted on the wave branch). |
| E20 | **CONFIRMED** (with one unfalsifiable sub-part noted as defect I1) | The witness file exists, covers every §3.2 item (cap/wall decomposition, both worked integrals with the per-cap `pA·R` moments, self-equilibrium, canonical zero-lumping, membrane identity in BOTH segment conventions, small-angle reduction, plain-sub-basis derivation, closed forms), and each load-bearing identity maps to an executable test as enumerated. The write-ordering ("written FIRST") is not falsifiable from uncommitted tree state; immaterial because the physics content is independently verified true. |

**Tally: 20 of 20 CONFIRMED** (E20 with an immaterial unfalsifiable ordering
sub-claim, noted as I1).

## 4. Re-Run Check Results (all executed by me, offline)

| Check | Result |
|---|---|
| `cargo fmt --check` — product_physics / curved_bend / benchmarks/mechanics / runner/headless | PASS ×4 |
| `cargo test --offline` core/solver/curved_bend | PASS — 23 passed, 0 failed |
| `cargo test --offline` core/product_physics | PASS — 86 passed, 0 failed |
| `cargo test --offline` validation/benchmarks/mechanics | PASS — 36 passed, 0 failed (deviation print reproduced: 1.4551915228366852e-11 ≤ 1.0e-9) |
| `cargo test --offline` core/runner/headless | PASS — 23 (lib) + 1 + 15 (bins) passed, 0 failed |
| Two-mirror README check | PASS — `git diff --numstat faee4faed` = `1 0` for each; both lines are the truthful CBPT inventory rows |
| Pinned tp_runner_015 solve | PASS — exit 0; stdout SHA-256 `b3cd85af…8613` = executor's value = T2 wave-head after-SHA |
| del1005 five-case byte-identity | PASS — 5/5 `cmp` identical; exits 0/0/0/1/1; witness SHAs match the executor's record |
| `validate_claims_language.py --repo-root .` | PASS — 262 files scanned, DEC-081 taxonomy satisfied |
| `validate_path_anchors.py . --text` | PASS — 659 surfaces, no literal home-dir paths |
| `git diff --check` vs base | PASS |
| JSON parse of `CHANGE_SCOPE_CONTAINMENT.json` | PASS |
| `validate_change_scope.py` (my own invocation, §5 allowed set) | PASS — 0 violations |
| Independent closed-form spot check (python3) | PASS — §1(d) |

## 5. Defects

| ID | Severity | Defect |
|---|---|---|
| I1 | INFO | E20's "derivation written FIRST" ordering is unfalsifiable from uncommitted working-tree state (no intermediate commits). The checkable substance — witness completeness against §3.2 and test-anchoring of every identity — is verified true, so nothing load-bearing rests on the ordering claim. |
| I2 | INFO | `add_curved_bend_pressure_thrust_load` silently returns (applies no load) if the crate calls error. The comment's infallibility argument is sound — `CurvedBendMacroBuild.macro_element` is build-time validated, so `geometry()` cannot fail on this path — and the pattern matches the surrounding infallible assembly code; noted for the record only. |
| I3 | INFO | Pre-existing unused-import warnings in the benchmarks/mechanics crate (equivalent-static symbols) surface during test builds; no `use` line is touched by this diff — present at base, not a T3 regression. |

No MAJOR, MINOR-blocking, or physics defect found. The v1 failure mode
(sign inversion / pair-only equilibrant) is affirmatively absent: the
implementation applies the outward wall load's consistent vector PLUS the cap
pair, produces `+pA` tension (not compression), zero shear, zero internal
moment at interior stations, the flexibility-independent outward (opening)
membrane tip displacement, and exact self-equilibrium — each verified by my
own re-derivation, by reading the assertions, and by re-running the tests.

## 6. Verdict

Fence containment independently re-derived and validator-confirmed; the
implementation's physics re-checked sign-by-sign against the crate's own
frame conventions and the v2 accepted derivation; every §6 check re-executed
by me with the claimed outcomes; both empirical guards reproduced bit-exactly
(pinned solve SHA continuity with the T2 wave head; del1005 5/5
byte-identity); all 20 executor claims survived refutation attempts; the
witness closed forms recomputed independently to the last digit; the
benchmark deviation reproduced at 1.455e-11 inside the recorded DEC-026 tier.
Deliverable-state update is exactly the bounded §3.8 shape. Defects found are
three informational notes, none load-bearing.

VERDICT: COMMIT-SAFE

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
