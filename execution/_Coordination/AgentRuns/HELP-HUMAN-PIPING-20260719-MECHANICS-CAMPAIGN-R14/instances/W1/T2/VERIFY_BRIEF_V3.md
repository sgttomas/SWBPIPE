# VERIFY_BRIEF_V3 — T2 DEL-04-03 Constant-Effort Solve Candidate Brief (v3 amendment)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T2
**Role:** Fresh-context adversarial verifier, third round (governed Agent 2, non-delegating)
**Date:** 2026-07-19
**Objects reviewed:**
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md`
(doc_id `CB-2026-07-19-T2-DEL-04-03-CONSTANT-EFFORT-001`, v3-amended) and
`instances/W1/T2/CURRENT_CANDIDATE_RATIONALE.md` (v3; claims C1–C18).
**Preserved prior returns read, not edited:** `instances/W1/T2/VERIFY_BRIEF.md`
(v1 BLOCK) and `instances/W1/T2/VERIFY_BRIEF_V2.md` (v2 BLOCK) — both intact
and unsoftened in the live tree.
**Verified against:** live tree at `723c95b0f` on
`claude/piping-r14-pkg04-mechanics` — the identical commit the v1 and v2
verifications ran on, so their live-tree confirmations remain commit-valid
and were spot-checked, not blindly trusted. Working tree clean apart from
lawful untracked R14 state (the AgentRuns directory including sibling
instance dirs, and the two R14 candidate briefs), matching the brief §4.1
lawful-state enumeration. All checks offline and read-only; this file is the
single durable write of this run.

## 1. v3 Cure Assessment (v2 BLOCK reasons, re-derived independently)

| v2 reason | Status | Evidence |
|---|---|---|
| (i) Hand-calc README inventory mirror outside the fence (V2-DEF-1) | **CURED** | §3.7 now names BOTH mirrors and permits "ONE additive inventory-line entry for the new fixture in EACH of those two files — truthful listings only, with no edit to either file's claim-posture, tolerance, `TBD`, or note text"; §5 item 4 lists both `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/README.md` under the same constraint; §6 adds, when the suite path is taken, verification that both mirrors list the new fixture "and that the diffs of both files are exactly one additive line each (v3)"; the §3.7 fallback (drop the fixture, record the obstacle, predicate-6 evidence stands) is intact and unchanged. Live-tree sufficiency of the two-mirror set independently re-derived in §3 below: the suite crate itself `include_str!`s exactly these two READMEs (`validation/benchmarks/mechanics/src/lib.rs` lines 54–55) and its `readiness_metadata_matches_documented_boundaries` test asserts every inventory fixture appears in BOTH (lines 5680, 5692–5697) — the code enforces exactly the two mirrors the v3 fence now permits, and no third mirror exists (V3 third-mirror sweep, §3). One line per mirror is sufficient: the test requires only fixture-ID presence (suite README) and fixture-ID + hand-calc-note filename presence (hand-calc README), both satisfiable by a single additive table row each. |
| (ii) §10 `SelectedOutcome` carried the refuted v1 "fail-closed" wording (V2-DEF-2) | **CURED** | §10 now reads "…declared-DOF positive-axis convention, data-driven opt-in consumption with non-blocking non-consumption warnings) per §3–§4 within the §5 fence" — matching the v2 §3.1–§3.2 predicates exactly (opt-in consumption conditions; non-consuming supports stay review-only with one non-blocking warning; no previously-valid input converted to blocking failure). The word "fail-closed" appears in the brief only inside the v2 amendment record describing the refuted v1 rule (line 20, correct historical usage) and in §7's "Fail closed." executor-disposition rule (correct usage about closeout, not the outcome). Residue remains in the RATIONALE only (V3-DEF-2, Low, non-blocking — see §4). |
| (iii) §10 `IndependentVerifier` pointer misnamed the pending artifact (V2-DEF-3) | **CURED** | The pointer now reads `PENDING (v3) — instances/W1/T2/VERIFY_BRIEF_V3.md; history: v1 BLOCK at instances/W1/T2/VERIFY_BRIEF.md (cured by v2 amendment), v2 BLOCK at instances/W1/T2/VERIFY_BRIEF_V2.md (residual mirror-fence and §10 wording defects, cured by v3 amendment)`. The history is truthful and unsoftened: both BLOCKs are named as BLOCKs with their artifacts; the v2 entry names its own residual defects. The "cured by v3 amendment" annotations are the amendment's claim pending exactly this verification, and the `PENDING (v3)` status states that dependency plainly (V3-OBS-4). Both prior return artifacts are preserved verbatim in the instance directory. |

The v3 amendment records at the top of both objects state the v2 BLOCK and
its reasons accurately ("one residual live-tree fact plus two §10 wording
defects"); "All other v2 cures were verified sound and are unchanged"
matches the v2 return (cures (a), (c), (d) verified CURED there). No
amendment softens either prior verdict.

## 2. Per-Claim Verdicts (C1–C18)

C8 and C11 (the v3-restated claims) fully re-derived; C1, C6, C16, C17, C18
re-checked live per dispatch; the remainder spot-checked against the same
commit (`723c95b0f`) the v2 return confirmed them on, citing that return.

| Claim | Verdict | Evidence |
|---|---|---|
| C1 | CONFIRMED | Re-checked live: DEL-04-03 `_STATUS.md` `Current State: IN_PROGRESS`; `## Remaining` holds exactly one item, text matching the brief's selected item verbatim. |
| C2 | CONFIRMED | Spot-check: `execution/_Decomposition/SOFTWARE_DECOMP.md` line 631 carries the DEC-049 Option B ruling with the quoted code-neutral exclusions (v2 return §2, same commit). |
| C3 | CONFIRMED | Spot-check via v2 return (same commit): `build_model` excludes constant-effort supports after the nonlinear branch; no solve-path consumption of `hanger.constant_load`. |
| C4 | CONFIRMED | Spot-check via v2 return (same commit): normalization to `Dimension::Force`; review-row disclosure text present and asserted only inside product_physics tests. |
| C5 | CONFIRMED | Spot-check via v2 return (same commit): per-load-case assembly precedes `reduce_system`; nonlinear active-set loop consumes the same assembled vector; single-seam design implementable. |
| C6 (v2) | CONFIRMED | Re-checked live: `support:CE-120` in `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json` declares `"restraints": []`, positive `constant_load` 375 N, no `nonlinear` field; `core/runner/headless/src/lib.rs` `has_blocking_diagnostics()` (lines 387–391) counts only `Blocking` severity, so the mandated Warning keeps exit 0/COMPLETED under the recorded DEC-065 policy. Del1005/benchmark containment per v2 return (same commit). Note the fixture's own hanger metadata records `"mechanics_consumption": "load_side_review_only_no_global_solve_consumption"` — the opt-in rule keeps even that recorded string truthful post-tranche. |
| C7 | CONFIRMED | Spot-check via v2 return (same commit): committed witnesses are historical for pinned pre-#287 commits; brief edits no witness. |
| C8 (v3) | **CONFIRMED** (full re-derivation) | (1) Fence-vs-task coverage: §4.1 reads plus instance-dir writes (fence 6); §4.2 implementation and unit tests in `core/product_physics/src/lib.rs` (fence 2); §4.3 hand-calc witness (fence 3), suite fixture + inventory registration + both README lines + `Cargo.lock` (fence 4), ephemeral captures with digests into the run record (fence 5); §4.4 `_STATUS.md`/`MEMORY.md`/run record (fence 5), `EXECUTE_RETURN.md` and containment JSON (fence 6). (2) Two-mirror necessity and sufficiency: the suite crate `include_str!`s exactly `../README.md` and `../../../hand_calcs/mechanics/README.md` (src/lib.rs lines 54–55) and test-enforces every inventory fixture's presence in both (lines 5680, 5692–5697); one additive row per README satisfies both assertions. (3) No third mirror: repo-wide grep on distinctive fixture IDs (`MECH-TP-PHYS-004-LOAD-TO-RESULTANT`, `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END`) and inventory phrases finds, outside the two READMEs and the crate source: dated historical run records/MEMORY/plan-log/concordance snapshots (derivative, dated — not live mirror claims); the headless binding (named-case wiring plus a `NotReusable` default arm already taken by several existing fixtures — not a mirror); and the validation-manual §3.1 case index, which claims pages "for one existing validated case" each, makes no completeness claim, and already lacks the newest fixture `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` — so an additive fixture falsifies no manual claim (V3-OBS-5). (4) Nothing materially more: the permitted README lines are constrained to truthful ID/filename listings (no claim-posture/tolerance/`TBD`/note-text change; §6 enforces exactly-one-additive-line diffs), and no fence item is unused. One precision residue: the required one-token bump of the suite's `assert_eq!(fixtures.len(), 21)` registration test (src/lib.rs line 5507) is not expressly named by fence item 4's "additive fixture + inventory registration only" parenthetical — V3-DEF-1 (Low, non-blocking; see §4): the file is writable, all four prior fixture additions bump this assertion in the registering commit (50f230b09, 2f2bdf175, b77e721b2, 4d3bae24d), §4.3's "suite tests pass" makes the requirement explicit, and every contrary reading fails loudly at §6's suite cargo test with no silent-falsification path. |
| C9 | CONFIRMED | Spot-check via v2 return (same commit): five SATISFIED constraints, three TBD prerequisites, targets hold committed evidence, no row resolved. |
| C10 | CONFIRMED | §3.5 text unchanged at v3: user-entered values only, non-blocking, no software constant. |
| C11 (v3) | **CONFIRMED** (full re-derivation) | The v2 class-9 hit was C8's fence-sufficiency claim beyond warrant against the hand-calc README mirror; that mirror is now inside the fence, the two-mirror set is code-corroborated as exactly sufficient, and no third mirror exists. Independent narrow re-screen (§3 below): classes 4, 8, 9 pass; classes 1–3, 5–8, 10 are untouched by the v3 amendment and their v2 no-hit derivations stand at the same commit. No class hit remains. The Low-severity residues (V3-DEF-1 fence-language precision; V3-DEF-2 rationale phrasing) are language defects with loud-failure or corrected-record dispositions, not stale-basis or beyond-warrant claims. |
| C12 | CONFIRMED | Spot-check via v2 return (same commit): all four tools present; offline cargo suite proven in this worktree by T1. |
| C13 | CONFIRMED | §10 re-read at v3: `AgentClassification: CLASSIFY_ELIGIBLE` and `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL` distinct; `OwnerCaseSelection: NONE`; `EffectStatus: HELD`; governance commit pin unchanged. |
| C14 | CONFIRMED | Spot-check via v2 return (same commit): HUMAN-selected W1 queue names this tranche; per-tranche commits; single wave-level DEC-025 sweep per the recorded T1 refinement. |
| C15 | CONFIRMED | Re-checked live: suite README lines 68–70 record the DEC-026 analytic-class `1.0e-9` relative tier as the recorded comparison basis; §3.7 reuses it, creates nothing. |
| C16 (v2) | CONFIRMED | Re-checked live: `"restraints": []` on `support:CE-120`; v3 §3.1–§3.2 keep the case exit 0/COMPLETED; every consumption path requires exactly one declared translational DOF plus a positive user-entered normalized load — no inference, no default. |
| C17 (v2) | CONFIRMED | Re-checked live: the "D6 remains the owner…" annotation appears exactly once (SOFTWARE_DECOMP.md line 631); Receipt 12 (2026-07-10, owner-adopted) present in `loop/LOOP_RECEIPTS.md`; brief §1 surfaces the annotation with the rehomed-row reading; queue selection HUMAN. |
| C18 (v2) | CONFIRMED | Re-checked live: the case-3 dated note (2026-07-19) is live in `docs/validation_manual/headless_runner_reproduction.md` (lines 26–27, 53); brief §3.9/§7 record the case-1 consequence as a HELP_HUMAN docs-lane follow-on with no docs write in the fence. |

Tally: **18 CONFIRMED, 0 REFUTED.**

## 3. Narrow D-52 §4.1 Re-Screen (v3 amendment surface only)

The v3 amendment touches §3.7, §5 item 4, §6, and §10 (plus rationale C8
restatement). Only classes the amendment could move were re-derived; the
untouched classes' v2 no-hit derivations (VERIFY_BRIEF_V2.md §3, same
commit) were spot-checked and stand.

- **Class 4 (scope/boundary, normative content, acceptance criteria): no
  hit.** The new allowance is one additive inventory line per mirror,
  constrained to a truthful fixture-ID/note-filename listing with no
  claim-posture, tolerance, `TBD`, or note-text change, and mechanically
  bounded by §6's exactly-one-additive-line diff check. An ID→filename
  table row can carry no threshold, criterion, or normative claim. The
  READMEs' `TBD` owner-gated boundaries are expressly untouchable.
- **Class 8 (protected/private data): no hit.** The listed fixture is
  invented and project-original; the listed note is the new fence-3
  hand-calc witness; DEC-049 exclusions remain predicates and fence
  exclusions. Nothing in an additive inventory line can ingest protected
  content, and the no-note-text-change constraint prevents smuggling.
- **Class 9 (stale basis / claim beyond warrant): no hit.** The v2 hit
  (fence sufficiency asserted without checking the second mirror) is cured
  by inclusion of both mirrors, and this round independently re-derived
  sufficiency from the code's own mirror enforcement plus a repo-wide
  third-mirror sweep (C8 evidence). No premise of the v3 text was found
  stale at `723c95b0f`: the pinned-fixture shape, DEC-049 annotation,
  Receipt-12 rehoming, case-3 note, severity model, and suite claim
  posture all verify live; the §4.1 executor freeze-check re-verifies at
  execution.

**Re-screen result: PASS.** No class hit remains.

## 4. Defects and Observations

| ID | Severity | Finding |
|---|---|---|
| V3-DEF-1 | Low | Fence item 4's purpose parenthetical "(additive fixture + inventory registration only)" does not expressly name the one-token update to the suite's inventory-coverage assertion `assert_eq!(fixtures.len(), 21)` (`validation/benchmarks/mechanics/src/lib.rs` line 5507) that registering a 22nd fixture entails. Non-blocking, three grounds: the file is already inside the fence; "inventory registration" matches the uniform landing convention (all four prior fixture additions — commits 50f230b09, 2f2bdf175, b77e721b2, 4d3bae24d — bump this assertion in the registering commit) and §4.3 explicitly requires "suite tests pass"; and both misreading paths fail loudly and safely (bump omitted → §6 suite cargo test fails and stops closeout; bump judged out-of-fence → §7 fail-closed return), so no silent falsification and no authority conversion is reachable. Recommend naming the registration-test update explicitly at the next governed touch of the brief. |
| V3-DEF-2 | Low | Rationale residue of V2-DEF-2: §2 praxeology ("fail-closed on ambiguous input") and §3 item 1 ("explicit, disclosed, and fail-closed") retain v1-era phrasing. Non-blocking: the binding owner-attribution record (§10 `SelectedOutcome`) is now correct; rationale items 8–9 state the opt-in/non-blocking rule unambiguously; read against them, "fail-closed" describes the consumption decision (no consumption absent explicit complete user data), not run blocking. Tidy at next touch. |
| V3-OBS-1 | Observation | §9's exclusion "edits to existing benchmark fixture recorded values, READMEs, policy JSON, witnesses, or reproduction bundles" harmonizes with §5 item 4 as edits-to-existing-content vs additive entries — §6's exactly-one-additive-line diff check operationalizes exactly that distinction, and the specific §5/§3.7 authorization governs. Pre-existing wording (present since v2, where it coexisted with the suite-README allowance); could be tightened to "no edit to existing README content beyond the §5 item 4 additive lines". |
| V3-OBS-2 | Observation | The suite crate mechanically enforces both mirrors: `readiness_metadata_matches_documented_boundaries` asserts every inventory fixture's ID appears in the suite README (line 5680) and its ID plus hand-calc-note filename appear in the hand-calc README (lines 5692–5697). This corroborates the v3 two-mirror fence as exactly necessary and sufficient, and means any mirror omission fails loudly at §6's suite cargo test. (It also shows V2-DEF-1's consequence (i) — "silently falsifies… no §6 check detects it" — was overstated: the omission would have failed loudly; the v2 BLOCK's core, a required surface excluded from the fence, was nonetheless correct.) |
| V3-OBS-3 | Observation | §6's exactly-one-additive-line suite-README check implies the new fixture takes a NEW family row in the Fixture Families table (appending IDs to an existing family's row would modify an existing line and fail the check loudly). Implementable and safe; the executor controls the fixture's family. |
| V3-OBS-4 | Observation | §10's history annotations "cured by v2 amendment"/"cured by v3 amendment" are the amendment's own claims pending verification; the `PENDING (v3)` status and the plainly-stated BLOCK history keep the record non-deceptive. The manager must progress the pointer with this verdict in the same act that disposes of this return. |
| V3-OBS-5 | Observation | Third-mirror sweep detail: the validation-manual §3.1 mechanics case index claims one page per "existing validated case", makes no inventory-completeness claim, and already lacks `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` — an additive fixture without a case page matches existing precedent and falsifies no manual claim. Headless whole-suite hardcoded-count tests target the stress suite (15) and nonlinear regression (5) only; mechanics headless tests are named-case, and unbound fixture IDs take the established `NotReusable` default arm already exercised by several existing fixtures — §6's read-only headless test run is safe against inventory growth. |

## 5. Verdict

The v3 amendment cures exactly what the v2 BLOCK required, and each cure
survives adversarial re-derivation at the source-code level: the fence now
permits precisely the two additive mirror lines that the suite crate's own
tests enforce (no more — the lines are constrained and diff-checked; no
less — no third mirror exists anywhere in the live tree), the §10
attribution record now describes the adopted outcome in the verified v2
opt-in terms, and the verifier pointer truthfully carries the full
BLOCK/BLOCK/PENDING history with both prior returns preserved unsoftened.
All eighteen claims stand; the narrow re-screen of the amendment-reachable
classes (4, 8, 9) passes; the untouched v2 confirmations spot-check clean
at the same commit. The residual findings (a fence-purpose parenthetical
that does not name the conventional registration-test count bump; leftover
"fail-closed" phrasing in the rationale's design prose) are Low-severity
language defects whose every misreading path fails loudly and safely —
none can silently falsify a recorded claim or convert owner authority, so
none meets the BLOCK bar this campaign's v1 and v2 rounds applied.

VERDICT: COMMIT-SAFE — all C1–C18 confirmed; v2 BLOCK reasons (hand-calc
mirror fence exclusion; §10 `SelectedOutcome` wording; §10 verifier
pointer) verified cured against the live tree at `723c95b0f`; D-52 §4.1
re-screen passes with no class hit; residual defects V3-DEF-1 and V3-DEF-2
are Low severity, non-blocking, recommended for tidy-up at the next
governed touch. The W1 manager may progress the adoption chain per the R14
campaign-plan execution rules, progressing the §10 `IndependentVerifier`
pointer to this artifact and verdict in the same act.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
