# VERIFY_BRIEF_V2 — T2 DEL-04-03 Constant-Effort Solve Candidate Brief (v2 amendment)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T2
**Role:** Fresh-context adversarial verifier, second round (governed Agent 2, non-delegating)
**Date:** 2026-07-19
**Objects reviewed:**
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md`
(doc_id `CB-2026-07-19-T2-DEL-04-03-CONSTANT-EFFORT-001`, v2-amended) and
`instances/W1/T2/CURRENT_CANDIDATE_RATIONALE.md` (v2; claims C1–C18).
**Preserved v1 return read, not edited:** `instances/W1/T2/VERIFY_BRIEF.md`
(BLOCK; intact and unsoftened in the live tree).
**Verified against:** live tree at `723c95b0f` on
`claude/piping-r14-pkg04-mechanics` (post-T1; identical commit to the v1
verification, so v1's live-tree confirmations remain commit-valid and were
spot-checked rather than blindly trusted). All checks offline and read-only;
this file is the single durable write of this run.

## 1. Amendment-Cure Assessment (v1 BLOCK reasons (a)–(d))

The amendment records at the top of both objects state the v1 BLOCK and its
reasons accurately and without softening; the v1 artifact is preserved
verbatim. Cure status, re-derived independently:

| v1 reason | Status | Evidence |
|---|---|---|
| (a) Pinned-fixture conversion (empty-restraints support must stay review-only, non-blocking warning, exit 0/COMPLETED kept, no accepted shape invalidated, closeout-stopping predicate) | **CURED** | Live fixture `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json` `support:CE-120`: `"restraints": []`, positive `constant_load` 375 N, no `nonlinear` field. v2 §3.1 consumption conditions (exactly one translational restraint DOF + positive normalized load) exclude it; §3.2 mandates one NON-BLOCKING warning and expressly forbids converting previously-valid input into blocking failure. Architecturally sound: `core/runner/headless/src/lib.rs` `DiagnosticSeverity` is `Info/Warning/Blocking` and `has_blocking_diagnostics()` counts only `Blocking`, so a Warning-severity diagnostic keeps exit 0 under the recorded DEC-065 policy ("exit 0 only when no blocking diagnostic is present", `docs/validation_manual/headless_runner_reproduction.md`). §7 now carries the explicit closeout-stopping predicate: "if the pinned solve fixture does NOT keep exit 0/COMPLETED, that is a §3.8 predicate failure that stops closeout." |
| (b) Benchmark default path no longer self-defeating (fence permits exactly one additive suite-README inventory line, no claim-posture/tolerance/TBD change) | **NOT FULLY CURED** | The suite-README instance is cured: §5 item 4 permits one additive inventory line in `validation/benchmarks/mechanics/README.md`, constrained against claim-posture/tolerance/`TBD` edits, and that satisfies the suite README's "mirrored above" claim (README §Fixture Families, §Readiness Boundary). But the identical self-defeat structure recurs one mirror down: `validation/hand_calcs/mechanics/README.md` §Fixture Inventory records "The mechanics benchmark crate source inventory is mirrored here so each fixture has an explicit public-original hand-calculation note" over an explicit per-fixture-ID table covering all 21 current fixtures. Adding the §3.7 fixture to `fixture_inventory()` without a row there falsifies that recorded mirror claim; the v2 fence (§5 item 3 permits only the NEW witness file) excludes that README. See V2-DEF-1. |
| (c) DEC-049 "D6 remains the owner" annotation surfaced with a truthful reading | **CURED** | The annotation is quoted in brief §1 with its context. Reading verified against the chain: the annotation sits in the DEC-049 acceptance note (`execution/_Decomposition/SOFTWARE_DECOMP.md` line 631); the 2026-07-10 owner-adopted rehoming (loop Receipt 12; DEL-04-03 `_STATUS.md` History 2026-07-10) moved plan-row open scope into deliverable `## Remaining` rows; the constant-effort solve row landed (ungated) in DEL-04-03 sourced from the PRD D5 row; DEL-04-04's live Remaining rows carry NO constant-effort scope (threshold promotions plus a `(gated:)` friction row), so no competing surface claims it; and the human owner's R14 queue names "T2 DEL-04-03 constant-effort spring-hanger solve behavior" directly (`ORCHESTRATION_PLAN.md`, Selection authority: HUMAN). The conclusion "no owner gate is crossed" is determined by the accepted record, not a contested interpretation. |
| (d) Reproduction-manual case-1 staleness recorded as HELP_HUMAN follow-on, no docs write | **CURED** | §3.9 requires the run record, MEMORY entry, and manager return to record the case-1 consequence as a HELP_HUMAN docs-lane follow-on per the R12→R13 case-3 precedent (the dated 2026-07-19 case-3 note is live in `docs/validation_manual/headless_runner_reproduction.md`); §7 names it as report-and-return, not fix; §5 forbids all docs writes. |

## 2. Per-Claim Verdicts (C1–C18)

Amendment-touched claims (C6, C8, C11, C16, C17, C18) fully re-verified;
the rest independently spot-checked against the live tree at the same commit
the v1 verifier confirmed them on.

| Claim | Verdict | Evidence (live tree) |
|---|---|---|
| C1 | CONFIRMED | DEL-04-03 `_STATUS.md`: `IN_PROGRESS`; `## Remaining` holds exactly the one item, text matching the brief verbatim. |
| C2 | CONFIRMED | `SOFTWARE_DECOMP.md` line 631 (DEC-049): Option B, the quoted code-neutral exclusions, and the "unless a later D6/D9 tranche proves deeper assembled-solve consumption" clause all present; Remaining row cites "PRD plan §3 D5 row / DEC-049". |
| C3 | CONFIRMED | `core/product_physics/src/lib.rs`: `is_constant_effort_support` exclusion in the support build (line 3064, after the nonlinear branch); `hanger.constant_load` occurrences are struct/normalization/review-row/validation/tests only; no solve-path consumption. |
| C4 | CONFIRMED | Normalization to `Dimension::Force` (~line 3859); `constant_effort_user_input_review` disclosure "no global constant-effort load or nonlinear behavior is claimed by this preview row" (line 7033); asserted only in product_physics's own tests. |
| C5 | CONFIRMED | `solve_load_case`: `global_load_vector` + `add_uniform_element_loads` + `add_pressure_thrust_loads` + `add_thermal_equivalent_loads` (lines 1129–1149) precede `reduce_system` (line 1150); nonlinear active-set loop consumes the same assembled vector (v1 evidence, same commit). Single-seam design implementable. |
| C6 (v2) | CONFIRMED | `support:CE-120` declares `"restraints": []` with positive 375 N `constant_load`; fails the v2 consumption conditions → review-only + one Warning-severity diagnostic; Warning is non-blocking in the headless severity model, so exit 0/COMPLETED holds. No `validation/benchmarks/**` or `del1005_payload_binding_*` surface contains a constant-effort support (grep clean). Byte-identity additionally probed against inventory-size leakage: the del1005 mechanics case is a NAMED-case run whose witness exposes `requested_case_count` only (no suite-inventory field), and the whole-suite-default case runs the NONLINEAR crate — so adding a mechanics fixture cannot perturb the five witnesses. "Output changes only by the new warning" holds under the reading §3.2 itself forces (non-consuming supports remain "exactly as in the landed DEC-049 slice", i.e., review-row text byte-preserved); see V2-OBS-1. |
| C7 | CONFIRMED | Manual records committed witnesses as historical for pinned pre-#287 commits; no code/test/tool references them; brief edits no witness. |
| C8 (v2) | **REFUTED** | The fence covers the §4 tasks mechanically, and the suite-README line cures the v1 instance. But "nothing materially more" and "the excluded surfaces are genuinely not required" fail against `validation/hand_calcs/mechanics/README.md` §Fixture Inventory: "The mechanics benchmark crate source inventory is mirrored here so each fixture has an explicit public-original hand-calculation note", over a complete per-fixture-ID table. The default §3.7 path (fixture registered in `fixture_inventory()`) requires one additive row there — the exact parallel of the suite-README line the amendment added — and the fence excludes that file. The landing convention confirms the requirement: the suite README itself records prior fixtures as "registered in `fixture_inventory()` and in the shared hand-calculation README inventory under `validation/hand_calcs/mechanics/`". See V2-DEF-1. |
| C9 | CONFIRMED | `Dependencies.csv`: exactly five EXECUTION/CONSTRAINT rows `DAG-002-E0110..E0114` `SATISFIED`; three EXECUTION/PREREQUISITE rows `E0435/E0436/E0437` (DEL-04-01/DEL-02-01/DEL-02-02) `TBD`; two ANCHOR rows. Targets hold committed implementation evidence consumed read-only; the brief resolves no row; posture recorded truthfully. |
| C10 | CONFIRMED | §3.5 compares computed displacement against user-entered `movement_limit`/`travel_range` only; warning non-blocking; no software constant, threshold, or criterion. |
| C11 (v2) | **REFUTED** (narrowly) | The v1 class-9 hit (fixture-impact mischaracterization) and class-4 ambiguity (retroactive invalidation) are genuinely cured by the v2 text. But "no class hit" fails on the independent re-screen: C8's fence-sufficiency claim is a class-9 claim-beyond-warrant against the live hand-calc README mirror (§3 below). |
| C12 | CONFIRMED | All four tools exist at `REPO_ROOT`; `software-workflow.json` present at `WORKING_ROOT`; T1 executed the same offline cargo suite in this worktree under the R12-ENVREPAIR-01 provisioning. |
| C13 | CONFIRMED | §10 keeps `AgentClassification: CLASSIFY_ELIGIBLE` and `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL` distinct; `OwnerCaseSelection: NONE`; `EffectStatus: HELD`; governance commit `f14fa775` resolves (v1, same commit). The claim as stated is true; adjacent §10 defects are charged separately (V2-DEF-2, V2-DEF-3), not to this claim. |
| C14 | CONFIRMED | `ORCHESTRATION_PLAN.md`: W1 queue names T2 DEL-04-03 constant-effort; Selection authority HUMAN; per-tranche commits, one branch/PR per wave, HELP_HUMAN merges under the session grant. The landed T1 brief §5 item 9 / §6 records the controlling HELP_HUMAN dispatch refinement (single wave-level DEC-025 sweep at W1 closeout), superseding the plan's per-tranche sweep phrasing; T1 landed under exactly that treatment. |
| C15 | CONFIRMED | Suite README line 69 and suite source record the DEC-026 analytic-class `1.0e-9` relative tier; §3.7 reuses it; no tolerance creation. |
| C16 (v2) | CONFIRMED | Live fixture declares `"restraints": []`; v2 §3.1–§3.2 keep that case exit 0/COMPLETED; every consumption path requires exactly one declared translational DOF plus a positive user-entered normalized load — no inference, no default. |
| C17 (v2) | CONFIRMED | Annotation text verified at `SOFTWARE_DECOMP.md` line 631; Receipt 12 (2026-07-10, owner-adopted) rehoming verified in `loop/LOOP_RECEIPTS.md` and DEL-04-03 History; the DEL-04-03 Remaining row is ungated; DEL-04-04's Remaining rows contain no constant-effort scope, so no competing successor surface exists; HUMAN campaign queue names this tranche. Recorded nuance, not a refutation: read strictly, the D6 plan-row's own rehomed successor rows live under DEL-04-04 and dropped the constant-effort assignment, while the D5-row-sourced DEL-04-03 row picked it up — the owner's queue selection of this exact tranche resolves any residue, so the chain determines the result (no class-1/3 hit). |
| C18 (v2) | CONFIRMED | §3.9/§7 record the case-1 consequence as a HELP_HUMAN docs-lane follow-on; no docs write inside the fence; the case-3 precedent (dated 2026-07-19 note) is live in the manual. See V2-OBS-2: the staleness may not even materialize, making the follow-on conservative rather than load-bearing. |

Tally: 16 CONFIRMED, 2 REFUTED (C8, C11 — both on the single hand-calc
README mirror fact).

## 3. Independent 10-Class D-52 §4.1 Fast-Reject Re-Screen (v2 brief)

Re-derived item by item against the live tree and the D-52 §4.1 text;
ambiguity counted as a hit.

1. Irreducible owner preference / two defensible outcomes: no hit. Recorded
   deliverable scope, owner-queued; residual design multiplicity is recorded
   under D-54 with rejected alternatives (including the two v2 additions).
2. Professional/safety/legal/fiduciary/hazard accountability: no hit.
   Invented fixtures, preview mechanics, review posture preserved.
3. Conflict ruling not determined by the authority chain: no hit — probed
   hard on the DEC-049 D6-annotation reading. The annotation is not a gate
   (no `(gated:)` marker, no D-XX requirement); the owner-adopted Receipt-12
   rehoming plus the ungated DEL-04-03 Remaining row plus the HUMAN R14
   queue selection of this exact tranche determine the result, and no
   contrary live surface (DEL-04-04 Remaining included) claims the scope.
   The brief's §1 disposition is a truthful reading of that chain, not a
   contested interpretation requiring referral.
4. Scope/boundary change, new normative content, new acceptance criteria:
   no hit — the v1 ambiguity is cured. The opt-in rule preserves every
   accepted input shape (the pinned empty-restraints exemplar keeps exit
   0/COMPLETED); the permitted README line is constrained to a truthful
   additive listing with no claim-posture/tolerance/`TBD` change; the sign
   convention remains disclosed interface semantics; §3.5 warnings derive
   from user data only.
5. Lifecycle/stage/issuance/release/acceptance/evidence-posture act: no hit.
6. Third-party/procurement/publication/external action: no hit; offline,
   no push/PR/merge at tranche level.
7. Merge/integration authority, destructive action: no hit; no witness,
   bundle, or recorded-value edit; historical records preserved.
8. Protected/private data exposure: no hit; DEC-049 exclusions are
   predicates and fence exclusions; all fixtures invented.
9. Evidence unavailable / stale basis / claim beyond warrant: **HIT
   (narrow)**. The v1 class-9 hit (pinned-fixture mischaracterization) is
   cured — the v2 §1/§3.8 statements now match the live fixture fact. But
   C8's fence-sufficiency claim ("nothing materially more"; "the excluded
   surfaces are genuinely not required") is stronger than its warrant: the
   live `validation/hand_calcs/mechanics/README.md` mirror claim makes one
   excluded surface required by the default §3.7 path. The claim was
   re-asserted at v2 without re-deriving fence sufficiency against the full
   set of inventory-mirror surfaces.
10. Protected domain-engine paths / prover activation / higher-order
    boundaries: no hit.

**Re-screen result: FAIL (single narrow class-9 hit).** Classes 1–8 and 10
pass cleanly; the v1 hits are cured; one new fence-sufficiency
claim-beyond-warrant remains.

## 4. Remaining Defects

| ID | Severity | Defect |
|---|---|---|
| V2-DEF-1 | High | Hand-calc README inventory mirror outside the fence. `validation/hand_calcs/mechanics/README.md` §Fixture Inventory records "The mechanics benchmark crate source inventory is mirrored here so each fixture has an explicit public-original hand-calculation note" over an explicit fixture-ID → note table covering every current fixture. The default §3.7 path registers a new fixture in `fixture_inventory()` and therefore requires one additive row in that table; the §5 fence permits only the NEW witness file in that directory. Consequences: (i) a literal executor who lands the fixture plus the permitted suite-README line silently falsifies the hand-calc README's recorded mirror claim — no §3 predicate, §6 check, or §7 stop condition detects it; (ii) a careful executor hits the §3.7 fallback, whose trigger parenthetical ("any change that would require a recorded-value or claim-posture edit") does not clearly cover an additive inventory-line obstacle, leaving the drop-the-fixture disposition on an ambiguous authorization; either way the "default benchmark path" the amendment claims to have repaired remains predictably self-defeating one mirror down. This refutes C8 and the v1 cure (b). Cure is one line: extend §5 item 4 (or item 3) to permit one additive inventory row in `validation/hand_calcs/mechanics/README.md` under the same no-claim-posture constraint, or honestly re-frame §3.7 with the fallback as the expected outcome. |
| V2-DEF-2 | Medium | §10 `SelectedOutcome` still records "…declared-DOF positive-axis convention, **fail-closed**…" — the refuted v1 language. Under v2 the selected outcome is data-driven opt-in with a NON-BLOCKING warning; "fail-closed" was the v1 term for the blocking rule the v1 verifier refuted. The rationale §2 praxeology lens likewise retains "fail-closed on ambiguous input". The amendment record's "No other section changed in meaning" is therefore inaccurate for §10: its meaning needed to change and did not. Because §10 is the owner-attribution record the standing approval binds to, it must not describe the adopted outcome in refuted v1 terms. One-token cure at re-authoring (e.g., "non-consuming-by-default absent explicit user data, non-blocking warning"). |
| V2-DEF-3 | Low | §10 `IndependentVerifier: PENDING — instances/W1/T2/VERIFY_BRIEF.md` names the v1 artifact, which now holds a completed BLOCK; the actually-pending verification artifact is this file (`VERIFY_BRIEF_V2.md`). Not deceptive — the amendment record at the top of the brief states the v1 BLOCK plainly — but the pointer is stale-in-place rather than merely awaiting progression, and the manager must progress it to name `VERIFY_BRIEF_V2.md` with this verdict in the same act that disposes of this return. |
| V2-OBS-1 | Observation | §3.4's review-row disclosure-text update ("the 'no global constant-effort load … is claimed' clause becomes a statement of the landed assembled-solve consumption") must be read as scoped to CONSUMING supports; §3.2 ("remains … exactly as in the landed DEC-049 slice") and §3.8/§1/C6 ("output changes only by the new warning") jointly force that reading, and the landed disclosure remains truthful for non-consuming rows. An unconditional single-site text change would fail §3.8's before/after check and stop closeout safely. Recommend explicit scoping at re-authoring; not a blocking defect. |
| V2-OBS-2 | Observation | The manual case-1 staleness (§3.9) may not materialize: case 1's documented expectations constrain `request/result validation diagnostics` and `result_refs`, while the new non-consumption warning likely rides the solve envelope diagnostics, and result counts are unchanged for a non-consuming support. Recording it as a HELP_HUMAN follow-on anyway is the conservative disposition; HELP_HUMAN may close it as no-change-needed. No action required. |

Also re-probed with no defect found: the T1-landed
`core/runner/headless/src/result_envelope_binding.rs` discloses
out-of-vocabulary result kinds per-row, NON-BLOCKING, never coerced
("Out-of-vocabulary classes are disclosed, never coerced"), so the new
`constant_effort_support_applied_load` kind and the new warning diagnostic
pass through the T1 envelope path without any `core/runner/headless` write;
del1005 byte-identity is robust even to the fixture-inventory growth (named-
case mechanics run; whole-suite default on the nonlinear crate); F-PIP-2 /
DEC-081 fence language is present and correct in both objects; the physics
(ideal constant-force element at the shared per-load-case assembly seam,
superposition identity) remains sound and implementable as specified.

## 5. Verdict

The v2 amendment is a substantially honest repair: cures (a), (c), and (d)
verify cleanly against the live tree, the v1 BLOCK is preserved unsoftened,
sixteen of eighteen claims stand, and the two v1 critical defects (fixture
conversion; missing manual disposition) are genuinely closed. But cure (b)
fails its own test: the amendment repaired the suite-README inventory mirror
while re-asserting fence sufficiency ("nothing materially more") without
checking the second, identically-structured inventory mirror in
`validation/hand_calcs/mechanics/README.md` — so the default benchmark path
is still self-defeating or silently claim-falsifying, C8 and C11 are again
refuted, and D-52 §4.1 class 9 is again hit. A false COMMIT-SAFE would place
a refuted fence-sufficiency claim and a §10 attribution record still carrying
the refuted v1 "fail-closed" outcome description into the owner's
standing-approval basis. The cure is small and bounded (one fence line for
the hand-calc README row under the existing no-claim-posture constraint, or
an honest fallback-default framing for §3.7; one-token §10 SelectedOutcome
correction; §10 verifier-pointer progression), and everything else in the
brief re-verifies.

VERDICT: BLOCK — C8 and C11 (v2) are refuted: the default §3.7 path requires
one additive inventory row in `validation/hand_calcs/mechanics/README.md`
(whose recorded claim "The mechanics benchmark crate source inventory is
mirrored here" the default path otherwise falsifies, with no predicate or
check to detect it), and that surface is outside the §5 fence — so v1 cure
(b) is incomplete and D-52 §4.1 class 9 (fence-sufficiency claim beyond
warrant) is hit; additionally §10's `SelectedOutcome` still records the
refuted v1 "fail-closed" outcome and its `IndependentVerifier` pointer
misnames the pending artifact, so the owner-attribution record is not yet a
truthful description of what would be adopted.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
