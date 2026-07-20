---
doc_id: CB-2026-07-19-T2-DEL-04-03-CONSTANT-EFFORT-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-19
package_id: PKG-04
deliverable_id: DEL-04-03
decision_basis: DEC-049, DEC-026 (reused comparison tier), DEC-046 (preserved gate), DEC-065, DEC-025, DEC-081, DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — T2 DEL-04-03 Constant-Effort Spring-Hanger Solve Behavior

**Status:** `EFFECTIVE (v3) — EXECUTION RELEASED BY W1 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V3 COMMIT-SAFE)`

**Amendment record (v2, 2026-07-19):** the first fresh-context verifier
(`instances/W1/T2/VERIFY_BRIEF.md`) returned `BLOCK`, refuting three v1
claims: the pinned solve fixture's constant-effort support declares
`"restraints": []`, so the v1 fail-closed rule would have converted the
documented reproduction case (exit 0, COMPLETED) into a blocked exit-1 run
— retroactively invalidating an accepted input shape; the v1 fence made
the default benchmark-fixture path self-defeating (the suite README
mirrors the fixture inventory but was non-writable); and the DEC-049
"D6 remains the owner" annotation was unsurfaced. §1, §2, §3.1–§3.2,
§3.7–§3.8, §5, and §7 are amended: solve consumption is now data-driven
opt-in (a support lacking the consumption conditions stays review-only
with a non-blocking warning; accepted input shapes remain valid and the
pinned case keeps exit 0); the fence permits an additive suite-README
inventory line only; the reproduction-manual case-1 consequence is
recorded as a HELP_HUMAN follow-on per the R12→R13 precedent; and the
DEC-049 annotation is surfaced with its rehomed-row reading. No other
section changed in meaning.

**Amendment record (v3, 2026-07-19):** the v2 verifier
(`instances/W1/T2/VERIFY_BRIEF_V2.md`) returned `BLOCK` on one residual
live-tree fact plus two §10 wording defects: the hand-calc home
`validation/hand_calcs/mechanics/README.md` also mirrors the fixture
inventory (a complete per-fixture-ID table with an explicit mirroring
claim), so the v2 fence's README allowance was incomplete — the default
benchmark path would silently falsify that recorded mirror or
self-trigger its fallback. §3.7, §5 item 4, §6, and §10 are amended: the
fence additionally permits ONE additive per-fixture inventory line in
`validation/hand_calcs/mechanics/README.md` (truthful listing only; no
claim-posture or note-text change elsewhere in that file), §6 verifies
both mirrors carry the new fixture when the suite path is taken, and the
§10 `SelectedOutcome` wording is corrected to the v2 opt-in rule. All
other v2 cures were verified sound and are unchanged.

**Prepared by:** WORKING_ITEMS (W1, PKG-04 package manager) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W1, tranche T2

**Selected work item:** the DEL-04-03 `_STATUS.md ## Remaining` sole item:
"Implement full constant-effort spring-hanger solve behavior beyond the
landed user-data slice (catalog sizing and protected/default values stay
excluded per DEC-049) (source: PRD plan §3 D5 row / DEC-049)".

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on
the D-52/`DEC-085` standing-approval overlay. Adoption remains the human
owner's conditional act under the standing rule; this document classifies
and proposes only. The adoption effect is `HELD` until independent
refutation returns `COMMIT-SAFE` and the W1 manager progresses the chain
under the R14 campaign-plan execution rules. No execution writes are
authorized by this document in its current state.

## 1. Purpose and Accepted Basis

Purpose: give constant-effort spring-hanger supports actual assembled-solve
consumption — the ideal constant-effort behavior (a user-entered constant
support force with zero stiffness contribution) — beyond the landed
DEC-049 user-data/review slice, with catalog sizing, protected/default
values, hidden defaults, and every threshold/lifecycle act excluded.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(HEAD `6152908b3246df61150dc91e3558788b05dfb643`, branch
`claude/piping-r14-pkg04-mechanics`; symbol references are used instead of
line numbers because the serialized T1 tranche may land additive
`core/product_physics` edits first):

- root `AGENTS.md` and project `AGENTS.md`; active committed-HEAD workplan
  `loop/WORKPLAN_2026-07-18b_piping_loop.md`; structurally valid
  `loop/LOOP_RECEIPTS.md` through cursor `Receipt-60`;
- campaign plan
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/ORCHESTRATION_PLAN.md`
  (W1 = PKG-04 manager; serialized tranche chains; HUMAN selection
  authority for the mechanics-lane queue);
- `execution/_DAG/_LATEST.md` resolving to approved `DAG-007`;
- DEL-04-03 deliverable folder (under
  `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/`):
  `_STATUS.md` (`IN_PROGRESS`; the single Remaining item above), `MEMORY.md`,
  `Dependencies.csv`, `ScopeOfWork.md`;
- `DEC-049` (D-15 Option B ruling): minimal dedicated spring-hanger model
  with explicit user-entered variable-spring and constant-effort data and
  provenance; code-neutral; "no catalog sizing, no protected standards
  values, no hidden defaults, and no professional/code-compliance claim";
  constant-effort support "starts as user-entered constant support load /
  travel / provenance review evidence unless a later D6/D9 tranche proves
  deeper assembled-solve consumption" — this tranche is that deeper
  assembled-solve consumption, directed by the deliverable's Remaining row.
  The same DEC-049 entry notes "D6 remains the owner for any future
  nonlinear or deeper assembled-solve consumption": that annotation
  assigns the anticipated later tranche its scope home in the
  then-current plan rows, whose open scope was rehomed into
  deliverable-local `_STATUS.md ## Remaining` on 2026-07-10 (F-PIP-5;
  Receipt 12). The rehomed DEL-04-03 Remaining row selected here is that
  successor surface, carries no `(gated:)` suffix, and was queued under
  HUMAN campaign selection authority; this tranche implements only the
  LINEAR constant-force consumption and no nonlinear behavior, so no
  owner gate is crossed;
- solve-path ground truth in `core/product_physics/src/lib.rs` (by symbol):
  `PreviewSupport`/`SpringHangerInput` (optional `constant_load`,
  `travel_range`, `movement_limit`, provenance/review fields);
  `normalize_quantity` already normalizes `hanger.constant_load` to
  `Dimension::Force`; `build_model` currently EXCLUDES constant-effort
  supports from the solve entirely (`if is_constant_effort_support(support)
  { return None; }` in the support build), after the `nonlinear` branch;
  `append_spring_hanger_user_input_results` emits the DEC-049 review rows
  (`constant_effort_user_input_review`) whose sign-convention note
  currently states "no global constant-effort load or nonlinear behavior is
  claimed by this preview row"; per-load-case force assembly happens in the
  load-case solve (`load_application.global_load_vector` +
  `add_uniform_element_loads` + `add_pressure_thrust_loads` +
  `add_thermal_equivalent_loads`) before `reduce_system`, and the nonlinear
  active-set loop consumes the same assembled force vector;
- witness/fixture exposure, verified at preparation and corrected at v2:
  the pinned runner solve fixture
  `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json`
  CONTAINS a `constant_effort_support` whose `restraints` list is EMPTY —
  under this brief's data-driven consumption rule that support stays
  review-only (it does not meet the consumption conditions), the case
  keeps exit 0/COMPLETED, and the fixture's output changes only by the
  new non-consumption warning diagnostic; the committed witnesses are
  already historical records for pinned pre-#287 commits, and the
  reproduction-manual case-1 consequence is handled per §3.8/§7 as a
  recorded HELP_HUMAN follow-on; no `validation/benchmarks/**` fixture
  and no `del1005_payload_binding_*` surface contains a constant-effort
  support (so the five del1005 committed witnesses remain
  byte-preservable);
- mechanics benchmark suite `validation/benchmarks/mechanics/` (crate with
  `fixture_inventory()`, README claim posture: recorded comparison values
  are regression evidence for current solver behavior; release thresholds,
  final tolerance policy, CI gate policy, professional reliance remain
  `TBD` owner-gated) and its recorded reuse of the DEC-026 analytic-class
  `1.0e-9` relative comparison tier; hand-calc home
  `validation/hand_calcs/mechanics/`;
- governance: `DEC-065`, `DEC-046` (untouched preserved gate), `DEC-025`
  (single wave-level sweep per the controlling W1 dispatch, as recorded in
  the T1 brief §6), `DEC-081`, D-52/`DEC-085`, D-54/`DEC-087`;
- `software-workflow.json` and the root tools
  `tools/software_workflow/run_registered_checks.py` /
  `tools/software_workflow/validate_change_scope.py`.

## 2. Live Selection Facts

- DEL-04-03 is `IN_PROGRESS` with exactly one Remaining item (selected).
- Deliverable-local DAG posture (`Dependencies.csv`): five root constraints
  `SATISFIED`; three prerequisite rows `TBD`
  (`DAG-002-E0435` DEL-04-01, `DAG-002-E0436` DEL-02-01, `DAG-002-E0437`
  DEL-02-02). The `TBD` values are satisfaction bookkeeping, not a live
  blocker: each target deliverable has committed implementation evidence in
  the live tree that this tranche consumes read-only, the DEC-049 user-data
  slice itself landed under the identical posture, and DEL-04-03 passed its
  2026-06-05 package review-readiness fan-in consuming the same upstreams.
  Selection authority for the R14 queue is HUMAN (campaign plan). This
  tranche does not resolve, promote, or edit any dependency row.
- The solve gap is confirmed live: constant-effort supports produce review
  rows only and are filtered out of `BuiltModel.supports`; no force,
  stiffness, or reaction consumption exists on any solve path.
- The constant-effort force must enter at per-load-case force assembly so
  that linear, sparse, and nonlinear active-set paths all consume it
  identically (they share the assembled force vector).
- T1 (serialized before this tranche) touches `core/product_physics` only
  additively (new public context surface); no conflict with this tranche's
  surfaces is expected. The executor freeze-check (§4.1) re-verifies.

## 3. Objective and Acceptance Predicates

Implement ideal constant-effort spring-hanger solve behavior so that all of
the following hold on the implementation head:

1. **Constant force consumption (data-driven opt-in, v2).** A support
   with `family = "constant_effort_support"` meets the CONSUMPTION
   CONDITIONS when it declares exactly one translational restraint DOF
   and carries a user-entered positive `hanger.constant_load`
   (unit-normalized to force). A consuming support contributes a constant
   applied nodal force of that magnitude at its node, along the positive
   axis of the declared translational DOF, in EVERY solved load case (the
   ideal constant-effort element: constant force, zero stiffness
   contribution, no restraint row added to the solve). The force enters
   the per-load-case assembled force vector so linear, sparse-mode, and
   nonlinear active-set solves consume it identically.
2. **No hidden defaults; accepted inputs stay valid (v2).** A
   constant-effort support that does NOT meet the consumption conditions
   (missing or non-positive `constant_load`; zero declared translational
   restraint DOFs — the accepted pinned-fixture shape; or more than one
   declared translational restraint DOF) is NOT consumed by the solve: it
   remains user-data review evidence exactly as in the landed DEC-049
   slice, and the solve emits one NON-BLOCKING warning diagnostic per
   such support (new code following the existing `SUPPORT_*` /
   spring-hanger naming patterns) naming the unmet condition — no
   defaulting, no catalog value, no protected data, no invented
   direction, and NO conversion of any previously-valid input into a
   blocking failure. Existing blocking behavior for genuinely malformed
   data (e.g. the existing `SUPPORT_DOF_INVALID` path for an unparseable
   declared DOF) is unchanged. A constant-effort support whose
   `nonlinear` field is present keeps the existing nonlinear-path
   handling unchanged (that branch takes precedence today and continues
   to).
3. **Sign convention explicit.** The applied-force direction convention
   (positive axis of the declared translational DOF) is recorded verbatim
   in the new result row basis/sign-convention text and in the hand-calc
   witness; no gravity-vector coupling and no inferred direction.
4. **Result rows.** Each solved load case emits, for each consuming
   constant-effort support, a result row
   (`constant_effort_support_applied_load` kind or equivalent new kind)
   recording the applied force with basis citing `dec_ref=DEC-049`, the
   sign convention, and `mechanics_consumption=assembled_solve`; the
   existing `constant_effort_user_input_review` rows are preserved, with
   their sign-convention disclosure text updated truthfully (the "no global
   constant-effort load ... is claimed" clause becomes a statement of the
   landed assembled-solve consumption). Review rows for `travel_range` and
   `movement_limit` remain user-data review evidence.
5. **User-limit comparison as warning evidence.** When the user entered
   `movement_limit` and/or `travel_range`, the solve compares the computed
   displacement magnitude at the support node along the acting DOF against
   the user-entered value and emits a NON-BLOCKING warning diagnostic when
   the user's own limit is exceeded (user-data-derived comparison; NOT a
   new threshold, tolerance, or acceptance criterion — no constant is
   introduced by the software).
6. **Superposition correctness.** For a linear model, the solve with a
   constant-effort support equals the solve without it plus the classical
   solution for the equivalent point force (superposition identity),
   demonstrated by unit test and by a new hand-calc witness
   `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`
   with closed-form values.
7. **Benchmark evidence (lawful reuse only, v2).** One new invented
   constant-effort fixture is added to `validation/benchmarks/mechanics`
   with recorded comparison values in the existing regression-evidence
   claim posture, compared at the already-recorded DEC-026 analytic-class
   relative tier (reuse; no new tolerance constant, no threshold, no
   claim-posture change, no policy JSON). Because BOTH the suite README
   (`validation/benchmarks/mechanics/README.md`) and the hand-calc home
   README (`validation/hand_calcs/mechanics/README.md`) mirror the
   fixture inventory, the fence permits ONE additive inventory-line entry
   for the new fixture in EACH of those two files — truthful listings
   only, with no edit to either file's claim-posture, tolerance, `TBD`,
   or note text, and §6 verifies both mirrors carry the new fixture when
   the suite path is taken (v3). If a lawfulness obstacle beyond that
   emerges at execution (any change that would require a recorded-value
   or claim-posture edit), the fixture is dropped, the obstacle recorded,
   and predicate 6's hand-calc + unit-test evidence stands as the tranche
   evidence — a recorded outcome, not a failure.
8. **Existing behavior preserved (v2).** Models with no constant-effort
   supports solve byte-identically (unit-test guard on an existing
   fixture); variable-spring-hanger behavior, DEC-049 review-row coverage,
   and the five committed `del1005_payload_binding_*` witnesses are
   unchanged (byte-identity check for the five del1005 cases). The pinned
   tp_runner_015 solve fixture KEEPS exit 0/COMPLETED (its constant-effort
   support does not meet the consumption conditions); its output changes
   only by the new non-consumption warning diagnostic, captured as
   before/after evidence in the run record; `CliOutput` shape is
   unchanged; the historical committed witnesses are not edited.
9. **Bounded state update (v2).** On success only: DEL-04-03 `_STATUS.md`
   strikes exactly the Remaining item (recording that catalog sizing and
   protected/default values remain excluded per DEC-049); one new History
   entry; updated `Last Updated`; one new `MEMORY.md` entry; one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T2_CONSTANT_EFFORT.md`.
   The run record and MEMORY entry must also record the
   reproduction-manual consequence as a HELP_HUMAN follow-on (the manual's
   documented case-1 solve expectations — result counts/diagnostics — are
   stale for post-tranche sources and need a dated note through a later
   docs-lane selection, per the R12→R13 case-3 precedent); the W1 manager
   return surfaces the same follow-on. No docs write occurs in this
   tranche.
10. **Checks.** The full §6 validation plan passes.

A successful run closes only the named Remaining item. It does not add
catalog behavior, does not change DEC-046/threshold state, and does not
close any other deliverable's scope.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several shapes were defensible, the selection below was made under
D-54/`DEC-087`; the four-lens analysis and materially rejected alternatives
are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T2/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape:

- **Ideal constant-effort element**: constant applied nodal force, zero
  stiffness, no restraint row — the classical analysis treatment of a
  constant-support hanger, entered at force-vector assembly so every solve
  mode shares it.
- **Direction = positive axis of the single declared translational
  restraint DOF**, disclosed verbatim; consumption is data-driven opt-in —
  a support with 0 or >1 declared translational DOFs, or without a
  positive constant load, stays review-only with a non-blocking warning
  (v2; accepted input shapes remain valid). No gravity coupling, no
  schema change, no signed input reinterpretation.
- **User-limit comparisons are warnings derived from user data only.**
- **Evidence = superposition hand-calc + unit tests + one additive
  benchmark fixture** under the existing suite claim posture.

Bounded tasks for the executor child:

### 4.1 Freeze the execution basis

- Begin on the wave branch in the integration checkout; record the base
  commit (the post-T1 wave-branch head, or `6152908b3` if T1 parked)
  before any durable write; verify the tree is clean apart from lawful R14
  state (AgentRuns dir, the R14 candidate briefs, and T1's landed commit).
- Re-verify the §1 symbol-level facts (constant-effort exclusion in
  `build_model`; review-row emission; force-assembly seam). Stop if any
  material basis fact, the DAG pointer, the Remaining text, or
  `software-workflow.json` changed since this brief beyond T1's recorded
  additive edits. Do not silently reinterpret scope.

### 4.2 Implement

- Implement §3.1–§3.5 in `core/product_physics/src/lib.rs` (new bounded
  functions beside the existing support build/force assembly; no signature
  breaks to existing public functions).
- Add unit tests: superposition identity; every non-consumption case in
  §3.2 (warning emitted, no force, exit posture preserved — including the
  empty-restraints shape used by the pinned fixture); direction
  convention; per-load-case application (including a two-load-case
  model); nonlinear-model coexistence (constant force present in the
  assembled vector consumed by the active-set loop); no-op guard for
  models without constant-effort supports; review-row text/count updates.

### 4.3 Evidence

- Write the hand-calc witness (derivation, closed-form values, model
  description, provenance: invented, project-original).
- Add the additive mechanics-suite fixture per §3.7 (new fixture function +
  inventory registration + recorded values; suite tests pass), or record
  the lawfulness obstacle and fall back per §3.7.
- Capture before/after solve stdout for the pinned tp_runner_015 solve
  fixture (ephemeral captures; recorded digests in the run record) and
  verify the five del1005 cases byte-identical against committed witnesses.

### 4.4 Update deliverable state and close out

On success only: apply §3.9, then run §6 in order. On failure or block:
leave deliverable state per the T1-brief §7 pattern, write truthful
evidence and `EXECUTE_RETURN.md` under `instances/W1/T2/`, and return to
the W1 manager. The executor does not commit.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to
(paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed status record or a later
   superseding hold/rejection record;
2. `core/product_physics/src/lib.rs` (and `core/product_physics/Cargo.toml`
   / `Cargo.lock` only if strictly required);
3. NEW file `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`;
4. `validation/benchmarks/mechanics/src/lib.rs` (additive fixture +
   inventory registration only), `validation/benchmarks/mechanics/README.md`
   and `validation/hand_calcs/mechanics/README.md` (ONE additive
   inventory-line entry for the new fixture in each; no claim-posture,
   tolerance, `TBD`, or note-text change), and
   `validation/benchmarks/mechanics/Cargo.lock` if required; no
   recorded-value edit to existing fixtures, no policy JSON;
5. DEL-04-03 deliverable folder
   (`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/`):
   `_STATUS.md`, `MEMORY.md`, one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T2_CONSTANT_EFFORT.md`;
6. the tranche instance directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T2/**`;
7. no evidence-sweep artifact in this tranche (single wave-level DEC-025
   sweep at W1 closeout per the controlling W1 dispatch, recorded in the T1
   brief §6).

Ephemeral writes: task-local Cargo target dirs and scratch captures outside
durable project paths.

No other project file is writable. In particular: no
`core/runner/headless/**`, `core/solver/**`, `core/reporting/**`, schema,
test-harness (`tests/**`), witness (`validation/witness/**`), reproduction
bundle, docs, other-deliverable, register, DAG, decomposition, PRD/PLAN,
workplan, or receipt write; no root-governance, `_DomainEngines/**`,
app-dev, PEC, or external path. No push, pull, fetch, PR, or merge.

## 6. Evidence and Validation Plan

In sequence from `WORKING_ROOT` unless noted; every failure stops
subsequent state-changing closeout; all cargo offline
(`CARGO_NET_OFFLINE=true`, `--offline`); no provisioning:

- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check`;
- `cargo test --offline --manifest-path core/product_physics/Cargo.toml`;
- `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml --check`
  and `cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml`
  (skip both if the suite fell back per §3.7);
- when the suite path is taken: verify both inventory mirrors
  (`validation/benchmarks/mechanics/README.md` and
  `validation/hand_calcs/mechanics/README.md`) list the new fixture, and
  that the diffs of both files are exactly one additive line each (v3);
- `cargo test --offline --manifest-path core/runner/headless/Cargo.toml`
  (read-only downstream regression of the producer path over the changed
  solver behavior);
- the §4.3 del1005 five-case byte-identity check and the pinned-fixture
  before/after capture;
- `python3 tools/validation/validate_claims_language.py --repo-root .` from
  `REPO_ROOT`;
- `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check` from `REPO_ROOT`;
- JSON parsing for every new/changed `.json` file;
- changed-path containment:
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT`,
  persisting JSON stdout to
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T2/CHANGE_SCOPE_CONTAINMENT.json`.

Branch-level registered checks run once at W1 wave closeout per the
controlling W1 dispatch (see T1 brief §6 for the recorded refinement).

## 7. Defect and Failure Disposition

- Fail closed. Any §3 predicate failure, §6 check failure, del1005 byte
  drift, unexpected changed path, or discovery that the physics basis is
  insufficient stops closeout; record truthful evidence under
  `instances/W1/T2/` and return to the W1 manager. In particular, if the
  pinned solve fixture does NOT keep exit 0/COMPLETED, that is a §3.8
  predicate failure that stops closeout — not a recordable consequence.
- No scope drift: no catalog sizing, no protected/default values, no
  gravity-coupled direction inference, no schema/input-shape change, no
  variable-spring behavior change, no threshold/tolerance creation or
  promotion, no lifecycle/stage/release/acceptance act.
- A repair need outside the §5 fence — including the recorded
  reproduction-manual case-1 staleness (§3.9), which is a HELP_HUMAN
  docs-lane follow-on — is reported and returned, not fixed here.
- Dirty checkout beyond lawful R14 state: stop and return the condition.

## 8. Rerun Triggers

A rerun is required when any of these changes after the implementation base
commit: the product_physics support/solve surfaces; DEC-049's ruling state;
the DEL-04-03 Remaining scope or lifecycle; applicable DAG-007 rows or the
DAG pointer; `software-workflow.json`; or a prior failed/blocked result
after its condition resolves. A material governance change returns the
brief itself to HELP_HUMAN before any rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- catalog sizing, manufacturer tables, protected standards values, hidden
  defaults, or any protected-content ingestion (DEC-049 exclusions hold);
- promotion of release thresholds, final tolerance policy, CI gate policy,
  or any DEC-046 record; new tolerance constants, acceptance criteria, or
  normative content of any kind (the DEC-026 analytic tier is reused as
  already recorded, never redefined);
- nonlinear constant-effort behavior models, friction interaction, or any
  DEL-04-04 owner-gated row;
- edits to existing benchmark fixture recorded values, READMEs, policy
  JSON, witnesses, or reproduction bundles;
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
ClassifiedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W1 / T2
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: ideal constant-effort element (constant force, zero stiffness, declared-DOF positive-axis convention, data-driven opt-in consumption with non-blocking non-consumption warnings) per §3–§4 within the §5 fence
JudgedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W1 / T2
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (gravity-opposing direction; signed-input reinterpretation; stiffness-element emulation; nonlinear-path implementation; suite-fixture omission)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T2/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/W1/T2/VERIFY_BRIEF_V3.md` (18/18 claims confirmed; mirror-fence cure corroborated at code level; narrow re-screen pass); history: v1 BLOCK at `instances/W1/T2/VERIFY_BRIEF.md` (cured by v2 amendment), v2 BLOCK at `instances/W1/T2/VERIFY_BRIEF_V2.md` (cured by v3 amendment), both preserved unsoftened
EffectStatus: EFFECTIVE (v3) — EXECUTION RELEASED BY W1 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V3 COMMIT-SAFE)
PreservedGates: DEC-049 exclusions (catalog/protected/defaults); DEC-046 tolerance promotion; DEL-04-04 owner-gated rows; lifecycle/stage/issuance/release/acceptance; prover activation/correlation; publication/external action; merge authority; D-45; D-05b; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the agent
classifies, selects among defensible shapes under D-54, and proposes. The
W1 manager progresses `EffectStatus` only after the independent refutation
returns `COMMIT-SAFE`, under the R14 campaign plan's execution rules. No
execution is released by this document in its current state.
