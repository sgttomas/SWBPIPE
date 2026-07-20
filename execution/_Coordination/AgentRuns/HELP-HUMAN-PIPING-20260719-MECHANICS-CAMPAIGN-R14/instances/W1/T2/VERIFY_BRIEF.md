# VERIFY_BRIEF — T2 DEL-04-03 Constant-Effort Solve Candidate Brief

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T2
**Role:** Fresh-context adversarial verifier (governed Agent 2, non-delegating)
**Date:** 2026-07-19
**Objects reviewed:**
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md`
(doc_id `CB-2026-07-19-T2-DEL-04-03-CONSTANT-EFFORT-001`) and
`instances/W1/T2/CURRENT_CANDIDATE_RATIONALE.md` (claims C1–C15).
**Verified against:** live tree at `723c95b0f` on
`claude/piping-r14-pkg04-mechanics` (post-T1; T1's additive landed edits
treated as expected context, not drift). All checks offline and read-only;
this file is the single durable write of this run.

## 1. Per-Claim Verdicts (C1–C15)

| Claim | Verdict | Evidence (live tree) |
|---|---|---|
| C1 | CONFIRMED | DEL-04-03 `_STATUS.md`: `Current State: IN_PROGRESS`; `## Remaining` holds exactly one item whose text matches the brief's selected item verbatim. |
| C2 | CONFIRMED | `execution/_Decomposition/SOFTWARE_DECOMP.md` line 631 (DEC-049): Option B adoption, the quoted code-neutral exclusions, and the "unless a later D6/D9 tranche proves deeper assembled-solve consumption" clause are all present; the Remaining row cites "PRD plan §3 D5 row / DEC-049" and directs the deeper consumption. Observation (not a refutation): the same DEC-049 acceptance note states "D6 remains the owner for any future nonlinear or deeper assembled-solve consumption"; the brief and rationale do not surface this ownership annotation. The owner-adopted rehoming of open scope into deliverable `## Remaining` rows (loop Receipt 12, 2026-07-10 History entry) plausibly supersedes it in place, but the tension went unrecorded. See DEF-5. |
| C3 | CONFIRMED | `core/product_physics/src/lib.rs`: `build_model` support build returns `None` for `is_constant_effort_support(support)` (line 3064), after the `support.nonlinear.is_some()` branch (line 3060). Every other occurrence of `hanger.constant_load` is the struct field (line 347), unit normalization (line 3859), review-row emission (line 7006), input validation (`src/validation.rs` lines 287, 851), and tests. No force, stiffness, or reaction consumption exists on any solve path. |
| C4 | CONFIRMED | Normalization to `Dimension::Force` for `hanger.constant_load` at lib.rs lines 3855–3874; `constant_effort_user_input_review` rows carry the disclosure "no global constant-effort load or nonlinear behavior is claimed by this preview row" (line 7033). The disclosure text is asserted only inside product_physics's own tests (line 10294); no other crate or GUI surface hardcodes it. |
| C5 | CONFIRMED | Per-load-case assembly in `solve_load_case`: `load_application.global_load_vector` + `add_uniform_element_loads` + `add_pressure_thrust_loads` + `add_thermal_equivalent_loads` (lines 1129–1148) precede `reduce_system` (line 1150); the identical `&force` vector is passed to `append_nonlinear_support_loop_results` (lines 1258–1266) and enters `NonlinearFrameSolveInput.force` (`force.to_vec()`). The sparse path consumes the same reduced force through `solve_preview_reduced_system`. The single-seam design is implementable as described. |
| C6 | REFUTED (consequence clause) | The containment sub-claims are true: `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json` contains `support:CE-120` with `family=constant_effort_support`; no `validation/benchmarks/**` fixture and no `del1005_payload_binding_*` input or generated witness contains a constant-effort support (the only other witness surfaces carrying the string are the historical generated `tp_runner_014`/`tp_runner_015` outputs), so the del1005 five-case byte-identity predicate is satisfiable. The consequence characterization is false: `support:CE-120` declares `"restraints": []` — zero translational restraint DOFs — so brief predicate §3.2 mandates a blocking diagnostic for it. In this architecture a blocking diagnostic from the build path yields `blocked_envelope` (lib.rs lines 805–816, 7811): `status.mechanics = MODEL_INCOMPLETE`, `results = []`. The committed witness shows the case currently completes as `COMPLETED` / `MECHANICS_SOLVED` with 822 result refs. Post-T2 the case does not "lawfully change output"; it flips to a fully blocked run with zero results, and under the recorded DEC-065 exit-code policy ("exit 0 only when no blocking diagnostic is present") the documented expected exit flips 0 → 1. See DEF-1. |
| C7 | CONFIRMED | No code, test, or tool in `core/`, `tests/`, or `tools/` references the tp_runner_015 witnesses; `docs/validation_manual/headless_runner_reproduction.md` records that committed witnesses and prior reproduction bundles "remain truthful for their pinned pre-#287 commits and are not edited." The brief edits no witness. (The distinct live-procedure consequence is DEF-1, charged to C6/C8/C11, not to this claim.) |
| C8 | REFUTED | Mechanically the fence covers the §4 tasks: implementation and unit tests live in `core/product_physics/src/lib.rs` (fence 2); the hand-calc is fence 3; benchmark fixture functions and `fixture_inventory()` live in the single `validation/benchmarks/mechanics/src/lib.rs` (fence 4); deliverable state is fence 5; instance dir is fence 6; running `core/runner/headless` tests read-only under §6 is consistent with its non-writability. But "the excluded surfaces are genuinely not required" fails twice. (a) DEF-1 makes a DEL-09-04 manual disposition (dated historical note on Part 1 case 1, per the exact precedent of the case-3 note landed 2026-07-19) or at minimum a recorded referral obligation a required consequence of the §3 predicates; the fence forbids `docs/**` and other-deliverable writes and the brief contains no referral provision. (b) The suite README's Readiness Boundary states "Fixture inventory: explicit in `fixture_inventory()` and mirrored above" over an explicit fixture-ID table (README lines 33–46); adding the §3.7 fixture with "no README" makes that recorded claim false, so the default §3.7 path predictably self-triggers its own fallback. See DEF-2, DEF-4. |
| C9 | CONFIRMED | `Dependencies.csv`: five EXECUTION/CONSTRAINT rows `DAG-002-E0110..E0114` all `SATISFIED`; three EXECUTION/PREREQUISITE rows `DAG-002-E0435/E0436/E0437` (DEL-04-01, DEL-02-01, DEL-02-02) `TBD` (the two additional rows are ANCHOR-class). Targets hold committed implementation evidence consumed read-only (`core/solver/frame_kernel`, the canonical model/schema crates, `core/units`). The DEC-049 user-data slice landed 2026-06-21 under the same TBD rows (last refreshed 2026-06-16), and the 2026-06-05 package review-readiness fan-in is in `_STATUS.md` History. Selection authority for the R14 queue is recorded as HUMAN in `ORCHESTRATION_PLAN.md`. The brief resolves no row. |
| C10 | CONFIRMED | §3.5 compares computed displacement only against user-entered `movement_limit`/`travel_range` values; warning is non-blocking; no software-defined constant, threshold, or acceptance criterion is introduced. |
| C11 | REFUTED | The rationale's §1 screen missed a class-9 hit (and a class-4 ambiguity). Independent re-screen in §2 below. Rationale §5's second attempted refutation ("output change with witnesses preserved") analyzed the wrong failure mode: the hazard is not evidence destruction but a basis mischaracterization of what the change is. |
| C12 | CONFIRMED | All four named tools exist at `REPO_ROOT` (`tools/validation/validate_claims_language.py`, `tools/validation/validate_path_anchors.py`, `tools/software_workflow/validate_change_scope.py`, `tools/software_workflow/run_registered_checks.py`); `software-workflow.json` present; T1's `EXECUTE_RETURN_V2.md` records the same offline cargo suite (headless / product_physics / nonlinear_integration / result_export, `CARGO_NET_OFFLINE=true --offline`) passing in this worktree under the R12-ENVREPAIR-01 provisioning. The §6 plan is executable offline here. |
| C13 | CONFIRMED | Brief §10 keeps `AgentClassification: CLASSIFY_ELIGIBLE` and `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL` distinct, `OwnerCaseSelection: NONE`, `EffectStatus: HELD`, `RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL`; the cited governance commit `f14fa77518a06f112ae72a8fcce4de0fab958d47` resolves to a commit object in this repository. Attribution matches the D-52 §4.5 / D-54 §3.3 shapes (`AgentJudgment: SELECT_AND_ADVANCE`). |
| C14 | CONFIRMED | `ORCHESTRATION_PLAN.md`: W1/PKG-04 queue names "T2 DEL-04-03 constant-effort spring-hanger solve behavior (DEC-049 exclusions hold)"; execution rules give per-tranche commits, one branch per wave, one PR per wave, HELP_HUMAN merging under the session grant (no manager push/PR/merge/receipt). The T1 brief §5 item 9 and §6 record the controlling HELP_HUMAN dispatch refinement: the single DEC-025 evidence sweep runs once at W1 closeout as the branch pre-push gate. |
| C15 | CONFIRMED | `validation/benchmarks/mechanics/README.md` line 69 and `src/lib.rs` (e.g., lines 2386–2390, 5561–5582) record the DEC-026 analytic-class `1.0e-9` relative tier as the suite's comparison basis; §3.7 reuses it and creates no tolerance. |

Tally: 12 CONFIRMED, 3 REFUTED (C6 consequence clause, C8, C11).

## 2. Independent 10-Class D-52 §4.1 Fast-Reject Re-Screen

Re-derived item by item against the live tree (ambiguity counted as a hit):

1. Irreducible owner preference / two defensible outcomes: no hit. The work
   item is recorded deliverable scope; residual design multiplicity is the
   D-54 lane's subject and is recorded with rejected alternatives.
2. Professional/safety/legal/fiduciary/hazard accountability: no hit.
   Invented fixtures, preview mechanics, review posture preserved.
3. Conflict ruling not determined by the authority chain: no hit, with one
   recorded observation — DEC-049's acceptance annotation assigns future
   deeper assembled-solve consumption to "D6" while the accepted DEL-04-03
   Remaining row directs it here; the owner-adopted rehoming of open scope
   into deliverable Remaining rows (Receipt 12) determines the result in
   favor of the Remaining row, so the chain does resolve it, but the brief
   should have surfaced the annotation (DEF-5).
4. Scope/boundary change, new normative content, new acceptance criteria:
   AMBIGUOUS — HIT under the ambiguity rule. The declared-DOF positive-axis
   sign convention itself is disclosed interface semantics, not normative
   engineering content (concur with rationale §5), and the §3.5 user-limit
   warning introduces no software constant (C10). But §3.2's fail-closed rule
   retroactively invalidates an input shape the landed DEC-049 slice accepts
   as valid today: a constant-effort support with no declared restraints
   passes `validate_constant_effort_support` (which requires positivity,
   travel metadata, and source — never a restraint) and currently solves to
   completion; the sole committed exemplar (`support:CE-120`) has exactly
   that shape. Turning previously valid, solvable user models into blocked
   models is a product validity-boundary change the brief does not disclose
   as such (DEF-3).
5. Lifecycle/stage/issuance/release/acceptance/evidence-posture act: no hit.
6. Third-party/procurement/publication/external action: no hit; offline,
   no push/PR/merge at tranche level.
7. Merge/integration authority, destructive action: no hit in the strict
   sense probed — no committed witness, bundle, or evidence file is edited,
   and the historical records remain truthful for their pinned commits. The
   real damage routes through class 9, not evidence destruction.
8. Protected/private data exposure: no hit. DEC-049 exclusions are fence
   exclusions and predicates; all fixtures invented.
9. Evidence unavailable / stale basis / claim beyond warrant: HIT. The
   brief's verified-at-preparation basis asserts the tranche "lawfully
   changes that fixture's solve output" (§1) and that the "output CHANGES by
   design" with before/after capture (§3.8). The live fixture fact refutes
   the characterization: `support:CE-120` declares zero restraints, so the
   brief's own §3.2 turns the run into a blocked `MODEL_INCOMPLETE` envelope
   with zero results and a 0→1 exit flip — not a changed solve output. The
   three TBD prerequisite rows, by contrast, are NOT a stale-basis hit: the
   brief's recorded posture (committed target evidence consumed read-only,
   identical posture at the DEC-049 slice landing, HUMAN campaign selection,
   no row resolved) is truthful and sufficient (C9).
10. Protected domain-engine paths / prover activation / higher-order
    boundaries: no hit.

**Re-screen result: FAIL (class 9 hit; class 4 ambiguity-hit).** The
rationale's C11 "no class hit" is refuted.

## 3. Additional Defects

| ID | Severity | Defect |
|---|---|---|
| DEF-1 | Critical | Pinned-fixture impact mischaracterized. `tp_runner_015_final_cli_solve_input.json` carries `support:CE-120` with `"restraints": []` and positive `constant_load` (375 N). Brief §3.2 mandates a blocking diagnostic for zero declared translational restraint DOFs; blocking diagnostics route to `blocked_envelope` (results emptied, `MODEL_INCOMPLETE`). The documented live reproduction procedure (`docs/validation_manual/headless_runner_reproduction.md`, Part 1 case 1: expected exit 0, `COMPLETED`, empty validation diagnostics, non-empty `result_refs`; case 1 carries no historical note, so it claims current sources) would fail on the implementation head, including an expected-exit flip 0 → 1 under the recorded DEC-065 exit-code policy. |
| DEF-2 | High | No predicate or referral guards DEF-1. §3.8 requires only `CliOutput` shape stability, witness non-edit, and before/after capture; §7's stop conditions would not fire on a blocked-but-shape-valid run. The executor could therefore close the tranche "successfully" while silently falsifying a live DEL-09-04 manual claim, with `docs/**` and other-deliverable writes excluded by §5 and no provision to report the consequence. The R13 precedent (case-3 dated historical note, landed via the DEL-09-04 VALMANUAL refresh) shows the governed remedy lives outside this fence. |
| DEF-3 | Medium | Undisclosed compatibility break: §3.2 rejects (as blocking) restraint-less constant-effort supports that the landed DEC-049 validation slice accepts and that currently solve to completion; the brief presents §3.2 only as "no hidden defaults, fail-closed" and never as a retroactive validity restriction on accepted input shapes. The fail-closed direction is defensible (the alternative is inferring a direction the user never entered), but the consequence must be surfaced and dispositioned, not implied. |
| DEF-4 | Medium | §3.7's default path is predictably self-defeating: the suite README mirrors the fixture-ID inventory in an explicit table and records "Fixture inventory: explicit in `fixture_inventory()` and mirrored above"; adding a fixture with "no README" edit falsifies that recorded claim, so the documented fallback (drop the fixture, record the obstacle) is the near-certain outcome. Not unsafe — the fallback is recorded and evidence predicate §3.6 stands — but the brief presents the fixture as the default lawful path when the obstacle is already visible in the live tree. |
| DEF-5 | Low | DEC-049's acceptance annotation "D6 remains the owner for any future nonlinear or deeper assembled-solve consumption" is neither quoted nor dispositioned by the brief or rationale, though the brief quotes the adjacent decision text. The owner-adopted rehoming of open scope into `## Remaining` rows plausibly resolves it in favor of DEL-04-03, but an unsurfaced ownership annotation in the governing ruling should be recorded. |
| DEF-6 | Low | The historical generated witness `tp_runner_014_headless_entrypoint_preview_run.json` also carries constant-effort review rows. No live test or tool references it, so no impact was found; noted for completeness since the brief's witness-exposure inventory names only tp_runner_015 and the del1005 set. |

Interaction with T1 (probed, no defect): the landed
`core/runner/headless/src/result_envelope_binding.rs` treats
out-of-vocabulary result rows with per-row NON-BLOCKING disclosure
("Out-of-vocabulary classes are disclosed, never coerced"), so a new
`constant_effort_support_applied_load` kind would be disclosed, not fail;
`constant_effort_user_input_review` remains in-vocabulary and its
sign-convention text is asserted nowhere outside product_physics. §6's
read-only headless test run is consistent with the fence. The ideal-element
physics (constant force, zero stiffness, per-load-case entry at the shared
assembly seam, superposition identity for linear models) is sound as
specified and the single-seam claim is implementable (C5).

## 4. Verdict

The brief is well-grounded on twelve of fifteen claims, and its physics,
fence mechanics, attribution, and campaign authorization all verify. It
fails on the one fact a fresh context exists to catch: the sole committed
constant-effort exemplar does not satisfy the brief's own fail-closed
predicate, so the promised "lawful output change" is actually a conversion
of a documented passing reproduction case into a blocked, empty-result,
exit-1 run — with no predicate to stop closeout and no lawful write or
referral surface to record the consequence. A false COMMIT-SAFE here would
convert human authority over exactly the kind of undisclosed consequential
impact D-52 reserves to the owner. The cure is bounded and available to the
W1 manager: re-author with (a) a truthful §1/§3.8 statement of the
tp_runner_015 case-1 outcome under §3.2, (b) either an explicit predicate
gate or a recorded HELP_HUMAN referral for the DEL-09-04 manual consequence
(case-1 dated note, per the case-3 precedent), (c) disclosure and
disposition of the §3.2 retroactive validity restriction (DEF-3), and
ideally (d) an honest default/fallback framing for §3.7 (DEF-4) and a
DEC-049 D6-annotation note (DEF-5).

VERDICT: BLOCK — C6 (consequence clause), C8, and C11 are refuted; D-52
§4.1 class 9 is hit (basis mischaracterization: the pinned fixture's
`restraints: []` makes §3.2 block the documented reproduction case rather
than change its output), with a class 4 ambiguity (retroactive invalidation
of an accepted input shape); the fence and predicates provide no lawful
disposition for the resulting falsification of the live DEL-09-04
validation-manual claim.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
