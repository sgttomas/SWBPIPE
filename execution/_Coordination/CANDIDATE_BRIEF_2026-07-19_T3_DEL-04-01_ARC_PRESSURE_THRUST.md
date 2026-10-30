---
doc_id: CB-2026-07-19-T3-DEL-04-01-ARC-PRESSURE-THRUST-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-19
package_id: PKG-04
deliverable_id: DEL-04-01
decision_basis: DEC-070 (curved-bend realization), DEC-026 (reused comparison tier), DEC-046 (preserved gate), D-38 (untouched), DEC-065, DEC-025, DEC-081, DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — T3 DEL-04-01 Arc Pressure-Thrust for Curved-Bend Macro Spans

**Status:** `EFFECTIVE (v2) — EXECUTION RELEASED BY W1 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V2 COMMIT-SAFE)`

**Amendment record (v2, 2026-07-19):** the first fresh-context verifier
(`instances/W1/T3/VERIFY_BRIEF.md`) returned `BLOCK`, refuting the v1
physics by direct integration: the outward radial pressure-imbalance wall
load on the arc has resultant `p·A·(t_i − t_j)` — so the v1 end-tangent
pair `{−pA·t_i at i, +pA·t_j at j}` is that load's EQUILIBRANT, not its
equivalent — and the v1 "each end force has zero moment about the center"
claim was false (each has magnitude `pA·R`; only the pair-sum vanishes).
The complete, exactly self-equilibrated pressure system on the arc span is
the end-cap force pair PLUS the distributed radial wall load. §1, §2,
§3.1–§3.5, §4, and §5 are amended to the corrected complete-load design
(cap pair at the nodes + exact work-equivalent consistent nodal vector for
the radial wall intensity, with equilibrium-consistent recovery under
which the closed-end wall tension `pA` emerges along the local tangent at
every station), the fence gains the two mirror-README inventory lines the
suite's readiness test enforces (T2 precedent), and the pinned-fixture
consequence statement is corrected (its bend is not macro-realized, so no
witness-facing solve change is expected). The verifier's worked
integration is adopted into the accepted basis. No other section changed
in meaning.

**Prepared by:** WORKING_ITEMS (W1, PKG-04 package manager) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W1, tranche T3

**Selected work item:** the DEL-04-01 `_STATUS.md ## Remaining` first item:
"Treat arc pressure-thrust beyond the recorded straight-chord treatment for
curved-bend macro spans (source: Receipt 11 named remainder /
TP-PMM-P1-CURVEDBEND-004 boundaries)". The second Remaining item (G1/G2/G4 +
M2/M3 re-disposition) is owner-gated and excluded (§9).

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on
the D-52/`DEC-085` standing-approval overlay. Adoption remains the human
owner's conditional act under the standing rule; this document classifies
and proposes only. The adoption effect is `HELD` until independent
refutation returns `COMMIT-SAFE` and the W1 manager progresses the chain
under the R14 campaign-plan execution rules. No execution writes are
authorized by this document in its current state.

## 1. Purpose and Accepted Basis

Purpose: replace the recorded straight-chord axial pressure-thrust
treatment on pipe spans realized as curved-bend macro elements (DEC-070)
with an arc-consistent treatment derived from the existing curved-bend
machinery, witnessed by a closed-form hand calc and an invented benchmark
fixture, with recorded comparison values as regression evidence only — no
tolerance/threshold creation, no owner-gated tolerance policy touch, and no
change to straight-span behavior.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(HEAD `6152908b3246df61150dc91e3558788b05dfb643`, branch
`claude/piping-r14-pkg04-mechanics`; symbol references are used because the
serialized T1/T2 tranches may land `core/product_physics` edits first):

- root and project `AGENTS.md`; active workplan
  `loop/WORKPLAN_2026-07-18b_piping_loop.md`; receipts valid through
  `Receipt-60`; R14 campaign plan (W1 manager; serialized tranche chains;
  HUMAN selection authority);
- `execution/_DAG/_LATEST.md` → approved `DAG-007`;
- DEL-04-01 deliverable folder (under
  `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/`):
  `_STATUS.md` (`IN_PROGRESS`; two Remaining items, first selected),
  `MEMORY.md` — the 2026-07-10 TP-PMM-P1-CURVEDBEND-004 entry records the
  arc-consistent distributed-load and interior-station machinery as landed
  and states: "Pressure thrust keeps the recorded straight-chord axial
  treatment on macro spans (stations inherit it); D-38 temperature
  semantics untouched" — exactly the residual this tranche discharges;
  `Dependencies.csv` (all eight deliverable-local `EXECUTION UPSTREAM`
  rows `SATISFIED`: five root constraints + DEL-02-01/02-02/02-03
  prerequisites);
- curved-bend ground truth: `core/solver/curved_bend/src/lib.rs`
  (`CurvedBendMacroElement`: `geometry()`, `radius()`, `included_angle()`,
  `arc_length()`, `orientation()`, `end_flexibility()`,
  `local_stiffness()`, `global_stiffness()`,
  `consistent_uniform_nodal_loads()`, `arc_section_resultants()` with the
  arc section frame — x tangent toward node j — and existing tangent
  algebra in source and tests);
- solve-path ground truth in `core/product_physics/src/lib.rs` (by
  symbol): `build_pressure_thrust_loads` (per-span axial load `p ×
  internal_area`, or expansion-joint effective-area inputs);
  `add_pressure_thrust_loads` applies equal/opposite axial end forces
  along the span's CHORD `local_x` for every span, including
  macro-realized bend spans (the inline comment records the straight-chord
  decision); `corrected_local_forces_for_axial_effects` adds the pressure
  axial in chord `UX`; the bend-specific recovery/station paths
  (`recover_curved_bend_local_forces`, `curved_bend_station_resultants`)
  receive `pressure_thrust_loads`; `pressure_thrust_for_pipe` gates
  `include_pressure_longitudinal` in endpoint/station stress recovery;
  `CurvedBendMacroBuild` carries `chord`, `arc_length`, `included_angle`,
  `bend_radius`, and the validated `macro_element`;
- mechanics evidence homes: `validation/hand_calcs/mechanics/` (existing
  curved-bend witnesses `curved_bend_distributed_load_fixed_end.md`,
  `expansion_loop_curved_bend_thermal.md`) and
  `validation/benchmarks/mechanics/` (suite claim posture: recorded
  comparison values are regression evidence; thresholds/tolerance/CI
  policy/professional reliance remain `TBD` owner-gated; DEC-026
  analytic-class relative tier already recorded in use);
- fixture exposure verified at preparation: no existing
  `validation/benchmarks/**` fixture combines pressure loading with a
  realized curved-bend macro span; the pinned
  `tp_runner_015_final_cli_solve_input.json` contains a bend component and
  pressure loads — whether that bend is macro-realized is measured at
  execution, and any resulting output change is captured before/after
  (the committed tp_runner_015 witnesses are historical records for
  pre-#287 pinned commits); no `del1005_payload_binding_*` surface solves
  a bend-plus-pressure model outside the suites;
- governance: `DEC-070` (curved-bend macro realization),
  `DEC-046`/tolerance holds (untouched), D-38 (temperature semantics,
  untouched), `DEC-065`, `DEC-025` (single wave-level sweep per the
  controlling W1 dispatch, recorded in the T1 brief §6), `DEC-081`,
  D-52/`DEC-085`, D-54/`DEC-087`;
- `software-workflow.json` and the root check/containment tools.

## 2. Live Selection Facts

- DEL-04-01 is `IN_PROGRESS`; the selected Remaining item is the recorded
  TP-PMM-P1-CURVEDBEND-004 residual; the deliverable is fully
  DAG-unblocked (all execution-upstream rows `SATISFIED`).
- The straight-chord treatment is confirmed live: pressure thrust on a
  macro-realized span is applied as ±(p·A) along the chord direction at
  the two end nodes, and station/endpoint recovery inherits the chord
  axial correction. For a curved span the physical thrust follows the
  local tangent, which differs from the chord by up to the half included
  angle at each end.
- The exact statics (v2; established by the v1 verifier's worked
  integration, adopted here as accepted basis): parametrize the circular
  arc of radius R about its center, with unit tangents t(θ) oriented i→j
  and outward radial unit n(θ). The complete pressure loading of the
  closed-section span decomposes into (a) the end-cap force pair
  `−pA·t_i` at node i and `+pA·t_j` at node j (the pressure-tension
  transmission the straight treatment already models along its axis), and
  (b) the distributed radial wall load `q(s) = (pA/R)·n(s)` per unit arc
  length, whose resultant is `pA·(t_i − t_j)` and whose total moment
  about the arc center is zero (each contribution `r × q ds` vanishes
  because q is radial; each END force separately carries moment `pA·R`
  about the center and only the pair-sum vanishes). The cap pair and the
  wall load are together EXACTLY self-equilibrated: zero net force and
  zero net moment. Neither part alone is a statically equivalent
  replacement for the other — the v1 pair-only design was that error.
  Consequences adopted as predicates: a statically-lumped-to-nodes
  version of the complete system cancels to zero (so the wall load must
  enter through its exact work-equivalent consistent nodal vector to
  produce the real deformation field), and segment equilibrium of the
  completely loaded arc yields wall tension `+pA` along the LOCAL TANGENT
  at every interior station — the closed-end tension emerging naturally
  with no ad-hoc chord correction. As the included angle goes to zero the
  cap pair reduces to the current straight treatment and the wall load
  vanishes.
- The owner-gated tolerance surface is not needed: the mechanics suite
  already records the DEC-026 analytic-class relative tier as its
  comparison basis, so an invented fixture with recorded comparison values
  is lawful reuse.
- Serialized T1/T2 may land first on the wave branch; both touch
  `core/product_physics` in surfaces disjoint from the pressure-thrust
  path (T1: additive context/envelope surfaces; T2: constant-effort
  supports). The executor freeze-check (§4.1) re-verifies the §1 facts.

## 3. Objective and Acceptance Predicates

Implement the arc pressure-thrust treatment so that all of the following
hold on the implementation head:

1. **Complete arc pressure loading (v2).** For every pipe span realized
   as a curved-bend macro element, the pressure-thrust contribution to
   the assembled force vector becomes the complete self-equilibrated arc
   system: (a) end-cap forces `−(p·A)·t_i` at node i and `+(p·A)·t_j` at
   node j, with unit end tangents from the span's validated arc geometry
   (the existing `CurvedBendMacroBuild`/`CurvedBendMacroElement`
   machinery — no re-derived geometry, no new geometry source), PLUS
   (b) the exact work-equivalent consistent nodal load vector (forces and
   moments at both nodes) of the distributed radial wall intensity
   `q(θ) = (pA/R)·n(θ)`, derived in `core/solver/curved_bend` with the
   same closed-form force-method machinery already used by
   `consistent_uniform_nodal_loads` (the radial components are inside the
   crate's trigonometric integration basis). A unit test verifies the
   assembled (a)+(b) system is self-equilibrated to floating-point
   precision (net force and net moment about any point ≈ 0). Both the
   pipe-internal-area and expansion-joint effective-area thrust sources
   receive the same treatment on macro spans. Non-realized (straight)
   spans keep the existing chord treatment bit-for-bit. If the exact
   consistent vector for the radial intensity CANNOT be derived in closed
   form with the existing machinery, the tranche FAILS CLOSED and is
   returned (no pair-only, lumped, or approximate substitute is landed).
2. **Derivation witnessed (v2).** A new hand-calc witness
   `validation/hand_calcs/mechanics/curved_bend_pressure_thrust_arc.md`
   records: the cap/wall decomposition; the worked integrals
   `∫ q ds = pA·(t_i − t_j)` and zero total moment about the arc center
   (with the explicit per-force `pA·R` non-vanishing moments and their
   pair cancellation); the self-equilibrium of the complete system; the
   zero-net static lumping consequence (why a consistent vector is
   required); the station-tension identity (segment equilibrium of the
   completely loaded arc gives `+pA` along the local tangent at every
   interior station); the small-angle reduction to the chord treatment;
   and the closed-form values used by the benchmark fixture. Elementary
   calculus and invented geometry only; consistent with the v1 verifier's
   worked integration in `instances/W1/T3/VERIFY_BRIEF.md`.
3. **Recovery and stations follow the arc (v2).** On macro spans the
   radial wall load is treated like the crate's other distributed loads:
   the recovery path subtracts the equivalent load so end forces are the
   true member forces of the continuously loaded arc, and interior
   stations come from segment equilibrium including the cap and wall
   contributions; the ad-hoc chord `UX` pressure correction is retired
   for macro spans. THE SHARP CHECK: for an invented end-supported
   pressurized arc with no other load, the recovered axial force at every
   tested interior station equals `+pA` along the local tangent within
   the recorded DEC-026 analytic tier. Recovery basis strings are updated
   truthfully (the recorded `pressure_thrust` basis names the complete
   arc treatment instead of the straight-chord decision). Straight-span
   recovery is unchanged; the `include_pressure_longitudinal` gating
   semantics are preserved.
4. **Nonlinear parity.** The same assembled force vector reaches the
   nonlinear active-set loop, so linear and nonlinear solves of the same
   macro-span model see the identical complete arc loading (unit-test
   guard).
5. **Crate change bounded (v2).** The `core/solver/curved_bend` additions
   (consistent radial-load vector, any tangent accessor) are additive
   public methods with their own closed-form tests; no change to existing
   stiffness/flexibility/load/station behavior or recorded values.
6. **Benchmark evidence (v2).** One new invented fixture in
   `validation/benchmarks/mechanics` exercises a pressurized
   macro-realized bend (invented geometry/material/pressure), with
   recorded comparison values (reactions/deflections and the
   station-tension identity of predicate 3) tied to the hand-calc closed
   forms, compared at the already-recorded DEC-026 analytic-class
   relative tier — regression evidence only; no claim-posture change, no
   policy JSON, no new tolerance constant. The fence permits ONE additive
   inventory-line entry for the new fixture in EACH of the two inventory
   mirrors (`validation/benchmarks/mechanics/README.md` and
   `validation/hand_calcs/mechanics/README.md` — truthful listings only,
   enforced by the suite's readiness test; T2 precedent), and §6 verifies
   both mirrors carry exactly one additive line. If a lawfulness obstacle
   beyond that emerges (any edit that would require a recorded-value or
   claim-posture change), the fixture is dropped, the obstacle recorded,
   and predicate-2 hand-calc + unit-test evidence stands (recorded
   fallback, not failure).
7. **Existing behavior preserved (v2).** Models with no realized bend
   spans, or with bends but no pressure loads, solve byte-identically
   (unit-test guards); existing curved-bend benchmark fixtures and their
   recorded values are untouched and still pass; the five committed
   `del1005_payload_binding_*` witnesses remain byte-identical. The
   pinned tp_runner_015 solve fixture's bend is NOT macro-realized on the
   live tree (`mechanics_geometry_only`, no bend-realization data — v1
   verifier finding), so its solve output is expected UNCHANGED; the
   executor still captures before/after as a guard and treats any
   unexpected change as a failure, not a recordable consequence.
8. **Bounded state update.** On success only: DEL-04-01 `_STATUS.md`
   strikes exactly the first Remaining item, leaving the owner-gated
   second item byte-identical; one new History entry; updated
   `Last Updated`; one new `MEMORY.md` entry; one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T3_ARC_PRESSURE_THRUST.md`.
9. **Checks.** The full §6 validation plan passes.

A successful run closes only the named Remaining item. It does not touch
D-38 temperature semantics, DEC-046/tolerance state, the G1/G2/G4+M2/M3
row, or any other deliverable's scope.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several shapes were defensible, the selection below was made under
D-54/`DEC-087`; the four-lens analysis and materially rejected alternatives
are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T3/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape (v2):

- **Complete self-equilibrated pressure system as the required core** —
  end-cap pair PLUS exact work-equivalent consistent radial wall-load
  vector — not a pair-only treatment (the v1 error: the pair alone is the
  wall load's equilibrant), not a multiplier, not a lumped or approximate
  substitute. Fail closed if the exact consistent vector is not derivable
  with the existing force-method machinery.
- **Geometry from the validated build-time macro element only** (single
  source of truth, per the recorded DEC-070 pattern).
- **Equilibrium-consistent recovery** on macro spans (equivalent-load
  subtraction; stations from segment equilibrium; the `+pA`
  local-tangent station tension as the sharp check) with truthful
  basis-string updates; straight spans untouched.
- **Evidence = hand-calc witness + additive benchmark fixture (+ two
  mirror inventory lines) + unit-test guards**, all invented content
  under the existing claim posture.

Bounded tasks for the executor child:

### 4.1 Freeze the execution basis

- Begin on the wave branch; record the base commit (post-T2 wave-branch
  head, or earlier if prior tranches parked) before any durable write;
  verify the tree is clean apart from lawful R14 state.
- Re-verify the §1 symbol-level facts (chord treatment in
  `add_pressure_thrust_loads`; recovery inheritance; curved-bend API).
  Stop if any material basis fact, the DAG pointer, the Remaining text, or
  `software-workflow.json` changed beyond the recorded T1/T2 landings. Do
  not silently reinterpret scope.

### 4.2 Derive, then implement

- Write the §3.2 hand-calc derivation FIRST (cap/wall decomposition,
  worked integrals, self-equilibrium, zero-lumping consequence,
  station-tension identity, closed forms for the fixture), cross-checked
  against the v1 verifier's recorded integration; if any part cannot be
  completed exactly from elementary statics and the existing curved-bend
  machinery, STOP and return the tranche as blocked (physics basis
  insufficient) — do not implement an unwitnessed or approximate
  treatment.
- Implement §3.1/§3.3/§3.4 in `core/product_physics/src/lib.rs` with the
  consistent radial-load vector (and any tangent accessor) added
  additively in `core/solver/curved_bend/src/lib.rs` per §3.5.
- Unit tests: self-equilibrium of the assembled (cap + wall) system on a
  realized span; consistent-vector values against the hand calc; the
  `+pA` local-tangent station-tension check on an end-supported
  pressurized arc; chord reduction for small angles; straight-span
  invariance; no-pressure invariance; nonlinear parity; recovery
  basis-string updates.

### 4.3 Evidence

- Add the §3.6 benchmark fixture (or record the fallback); run the suite.
- Verify del1005 five-case byte-identity; capture pinned-fixture
  before/after if its output changes.

### 4.4 Update deliverable state and close out

On success only: apply §3.8, then run §6 in order. On failure or block:
leave deliverable state unchanged (or record the truthful partial state),
write truthful evidence and `EXECUTE_RETURN.md` under `instances/W1/T3/`,
and return to the W1 manager. The executor does not commit.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to
(paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed status record or a later
   superseding hold/rejection record;
2. `core/product_physics/src/lib.rs` (and Cargo.toml/Cargo.lock only if
   strictly required);
3. `core/solver/curved_bend/src/lib.rs`, additive methods/tests only (no
   change to existing stiffness/flexibility/load/station behavior or
   recorded values), plus its Cargo.lock if required;
4. NEW file `validation/hand_calcs/mechanics/curved_bend_pressure_thrust_arc.md`;
5. `validation/benchmarks/mechanics/src/lib.rs` (additive fixture +
   inventory registration only, including the conventional fixture-count
   assertion bump), `validation/benchmarks/mechanics/README.md` and
   `validation/hand_calcs/mechanics/README.md` (ONE additive
   inventory-line entry for the new fixture in each; no claim-posture,
   tolerance, `TBD`, or note-text change), and the suite `Cargo.lock` if
   required; no existing recorded-value edit, no policy JSON;
6. DEL-04-01 deliverable folder
   (`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/`):
   `_STATUS.md`, `MEMORY.md`, one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T3_ARC_PRESSURE_THRUST.md`;
7. the tranche instance directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T3/**`;
8. no evidence-sweep artifact in this tranche (single wave-level DEC-025
   sweep at W1 closeout per the controlling W1 dispatch, recorded in the
   T1 brief §6).

Ephemeral writes: task-local Cargo target dirs and scratch captures.

No other project file is writable. In particular: no
`core/runner/headless/**`, `core/solver/**` other than `curved_bend`,
`core/reporting/**`, schema, `tests/**`, `validation/witness/**`,
reproduction bundle, docs, other-deliverable, register, DAG,
decomposition, PRD/PLAN, workplan, or receipt write; no root governance,
`_DomainEngines/**`, app-dev, PEC, or external path. No push, pull, fetch,
PR, or merge.

## 6. Evidence and Validation Plan

In sequence from `WORKING_ROOT` unless noted; every failure stops
subsequent state-changing closeout; all cargo offline
(`CARGO_NET_OFFLINE=true`, `--offline`); no provisioning:

- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` and
  `cargo test --offline --manifest-path core/product_physics/Cargo.toml`;
- `cargo fmt --manifest-path core/solver/curved_bend/Cargo.toml --check`
  and `cargo test --offline --manifest-path core/solver/curved_bend/Cargo.toml`
  (skip both if untouched);
- `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml --check`
  and `cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml`
  (skip both if the suite fell back per §3.6);
- when the suite path is taken: verify both inventory mirrors list the
  new fixture and each README diff is exactly one additive line (v2);
- `cargo test --offline --manifest-path core/runner/headless/Cargo.toml`
  (read-only downstream regression over the changed solver behavior);
- del1005 five-case byte-identity; pinned-fixture before/after capture if
  applicable;
- `python3 tools/validation/validate_claims_language.py --repo-root .` and
  `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check` from `REPO_ROOT`; JSON parsing for new/changed
  `.json` files;
- changed-path containment:
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT`,
  persisting JSON stdout to
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T3/CHANGE_SCOPE_CONTAINMENT.json`.

Branch-level registered checks run once at W1 wave closeout per the
controlling W1 dispatch (T1 brief §6 records the refinement).

## 7. Defect and Failure Disposition

- Fail closed. Any §3 predicate failure, §6 check failure, del1005 byte
  drift, unexpected changed path, or an incomplete/inexact derivation
  stops closeout; record truthful evidence under `instances/W1/T3/` and
  return to the W1 manager. An unwitnessed physics treatment is never
  landed.
- No scope drift: no SIF/flexibility-factor change, no D-38 temperature
  edit, no existing-fixture recorded-value change, no threshold/tolerance
  creation or promotion, no owner-gated row work, no
  lifecycle/stage/release/acceptance act.
- A repair need outside the §5 fence is reported and returned, not fixed
  here.
- Dirty checkout beyond lawful R14 state: stop and return the condition.

## 8. Rerun Triggers

A rerun is required when any of these changes after the implementation
base commit: the curved-bend or pressure-thrust solve surfaces; the
DEL-04-01 Remaining scope or lifecycle; applicable DAG-007 rows or the DAG
pointer; `software-workflow.json`; or a prior failed/blocked result after
its condition resolves. A material governance change returns the brief
itself to HELP_HUMAN before any rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- the DEL-04-01 second Remaining item (G1/G2/G4 + M2/M3 re-disposition —
  owner-gated) or any DEL-04-04/DEL-05-x owner-gated row;
- promotion of release thresholds, final tolerance policy, CI gate policy,
  or any DEC-046 record; new tolerance constants, acceptance criteria, or
  normative content (the DEC-026 analytic tier is reused as recorded,
  never redefined);
- protected standards content: no code-book SIF/flexibility values, no
  protected tables — invented geometry and elementary statics only;
- D-38 temperature-semantics changes, D-45 shear-modulus work, or any
  frozen witness / reproduction-bundle / README / policy-JSON edit;
- lifecycle transition, stage/milestone advancement, issuance, release,
  packaging, publication, push, PR creation/merge, hosted CI, network use,
  or any external commitment;
- professional approval, certification, sealing, authentication, or
  code-compliance claims.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (W1 manager, R14 campaign)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W1 / T3
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: complete self-equilibrated arc pressure system (end-cap pair + exact consistent radial wall-load vector) with equilibrium-consistent recovery, hand-calc witness, and additive benchmark fixture per §3–§4 (v2) within the §5 fence
JudgedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W1 / T3
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (chord retention; bisector lumped force; pair-only end-tangent treatment — the refuted v1 design; multiplier-style correction; static lumping)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T3/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/W1/T3/VERIFY_BRIEF_V2.md` (17/17 claims confirmed; physics independently re-derived symbolically plus randomized numerical integration, including the membrane-state strengthening of the station identity — pure +pA local-tangent tension with zero shear and zero internal moment — and the Bourdon opening-direction check; 10-class re-screen pass); history: v1 BLOCK at `instances/W1/T3/VERIFY_BRIEF.md` (physics sign-inversion refuted by worked integration; adopted into the v2 basis and cured by the complete-load design), preserved unsoftened
EffectStatus: EFFECTIVE (v2) — EXECUTION RELEASED BY W1 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V2 COMMIT-SAFE)
PreservedGates: G1/G2/G4+M2/M3 re-disposition (owner-gated); DEC-046 tolerance promotion; D-38; D-45; lifecycle/stage/issuance/release/acceptance; prover activation/correlation; publication/external action; merge authority; D-05b; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the agent
classifies, selects among defensible shapes under D-54, and proposes. The
W1 manager progresses `EffectStatus` only after the independent refutation
returns `COMMIT-SAFE`, under the R14 campaign plan's execution rules. No
execution is released by this document in its current state.
