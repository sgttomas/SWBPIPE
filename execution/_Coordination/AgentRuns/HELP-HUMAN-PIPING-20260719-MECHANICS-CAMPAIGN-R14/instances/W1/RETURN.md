# W1 MANAGER RETURN — PKG-04 Mechanics Wave (R14)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1
**Role:** WORKING_ITEMS (Agent 1, PKG-04 package manager), reporting to HELP_HUMAN
**Branch:** `claude/piping-r14-pkg04-mechanics` (base `6152908b3`, post-#288)
**Date:** 2026-07-19/20 (UTC)
**Status:** WAVE COMPLETE — three tranches LANDED; no push, PR, merge, rebase, or receipt performed (HELP_HUMAN fan-in acts)

## 1. Per-Tranche Dispositions

| Tranche | Scope | Disposition | Commit | Chain |
|---|---|---|---|---|
| T1 | DEL-04-02 sole Remaining + DEL-04-04 Remaining item 1 — governed DEL-08-04 result-envelope producer-path binding (one integration owner) | **LANDED** | `723c95b0f` | brief v3 (v1 verifier BLOCK: stale witness premise; executor BLOCK: DEL-08-04 vocabulary gap; both cured by recorded amendments) → verifier v3 COMMIT-SAFE (18/18) → executor PASS (9/9 predicates) → impl verifier COMMIT-SAFE (18/18, 13/13 checks re-run) |
| T2 | DEL-04-03 sole Remaining — constant-effort spring-hanger assembled-solve consumption (DEC-049 exclusions preserved) | **LANDED** | `faee4faed` | brief v3 (v1 BLOCK: empty-restraints pinned-fixture shape would have been blocked; v2 BLOCK: second inventory-mirror README outside fence; cured) → verifier v3 COMMIT-SAFE (18/18) → executor PASS (10/10) → impl verifier COMMIT-SAFE (18/18, 10/10 checks re-run, pinned/del1005 SHAs reproduced) |
| T3 | DEL-04-01 first Remaining — arc pressure-thrust for curved-bend macro spans | **LANDED** | `6326b2f93` | brief v2 (v1 BLOCK: the pair-only "statically-exact" design was physically sign-inverted — refuted by the verifier's worked integration, adopted as corrected basis; complete self-equilibrated cap+wall design) → verifier v2 COMMIT-SAFE (17/17, physics independently re-derived incl. membrane-state and Bourdon-direction checks) → executor PASS (9/9, derive-first) → impl verifier COMMIT-SAFE (20/20, line-level sign verification, closed forms recomputed) |

No tranche was parked or referred; no D-49/D-52 limit was hit. Every
verifier/executor BLOCK is preserved unsoftened in the tranche instance
directories with its curing amendment recorded in the brief.

## 2. Wave Check Tally (registered checks, run once for the branch)

Executed at wave closeout from `REPO_ROOT` via
`tools/software_workflow/run_registered_checks.py`, each as its own halting
step; normalized JSON persisted beside this return:

| Check | Result | JSON |
|---|---|---|
| `piping-pytest` | PASS | `instances/W1/CHECK_piping-pytest.json` |
| `evidence-sweep` (DEC-025 pre-push gate) | PASS | `instances/W1/CHECK_evidence-sweep.json` |
| `harness-pytest` | PASS | `instances/W1/CHECK_harness-pytest.json` |
| `harness-self-check` | PASS | `instances/W1/CHECK_harness-self-check.json` |

The single evidence sweep produced exactly one new artifact
(`validation/evidence/sweeps/SWEEP_20260720T041149Z_6326b2f936f2-dirty.json`;
before/after snapshot verified 286→287, delta exactly one file; the
`-dirty` suffix reflects the then-untracked closeout files committed with
this return). Per-tranche checks (cargo fmt/test per touched crate,
contract test, claims-language, path-anchors, `git diff --check`, JSON
parse, change-scope containment with persisted JSON, CLI-stability
byte-diffs) were run by each executor and independently re-run by each
implementation verifier — tallies in the per-tranche `EXECUTE_RETURN*.md`
and `VERIFY_IMPL.md` artifacts.

**DEC-025 treatment note for fan-in:** the `ORCHESTRATION_PLAN.md`
execution-rules sentence ("per-tranche commits, each gated by the
tranche's full check set including the DEC-025 evidence sweep") was
refined by the controlling HELP_HUMAN W1 dispatch ("Run the registered
checks once for the branch ... single evidence-sweep ... pre-push DEC-025
gate"). The refinement is recorded in the T1 brief §6 and applied
wave-wide: no commit of this branch is pushed un-swept.

## 3. Run-Record Pointers

- DEL-04-02: `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md`
- DEL-04-04: `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md`
- DEL-04-03: `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T2_CONSTANT_EFFORT.md`
- DEL-04-01: `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T3_ARC_PRESSURE_THRUST.md`
- Briefs: `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md`,
  `..._T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md`, `..._T3_DEL-04-01_ARC_PRESSURE_THRUST.md`
- Chain artifacts: `instances/W1/T{1,2,3}/` (rationales, all verifier
  returns including preserved BLOCKs, executor returns, containment JSONs).

## 4. Parked Owner-Gated Rows (named, never executed)

- DEL-04-04 items 2–6: PDU-035 formal REVIEW disposition and accepted
  dimensional/conversion basis; friction path-history D-XX ruling
  (mechanics plan §4); the three threshold-promotion rows
  (DEC-046/DEC-052/DEC-054 lineage, one stage-gated D-05b/R5).
- DEL-04-01 second Remaining item: G1/G2/G4 + M2/M3 re-disposition
  (owner re-disposition where not closed by evidence).
- Campaign-level parked cluster (unchanged, per the campaign plan): D-45;
  DEC-046 tolerance/threshold promotions; PDU-035 dispositions;
  DEL-04-05 stage-gated/D-05b rows.

## 5. Follow-Ons Reported for HELP_HUMAN Routing (not executed in W1)

1. **DEL-08-04 vocabulary extension** (from T1): the result-export
   `ResultFamily`/`DimensionId` vocabulary cannot truthfully represent
   stiffness (`N/m`, `N*m/rad`), energy/work, or count/state result
   classes; the T1 producer discloses such rows per-row non-blockingly.
   Extension belongs to a separate lawful selection on the PKG-08 surface.
2. **Reproduction-manual case-1 refresh** (from T2): the manual's
   documented case-1 solve expectations are stale for post-T2 sources
   (one new non-consumption warning diagnostic and review-row text);
   needs a dated note through a docs-lane selection per the R12→R13
   case-3 precedent. The T2 impl verifier also flagged the stale fallback
   fixture `fixtures/product_preview/invented_mechanics_result.json`
   disclosure text for the same docs-lane touch, and the executor flagged
   the read-only `validation.rs` `CONSTANT_EFFORT_USER_DATA_REVIEWED`
   info-text for consideration there.
3. **DEC-025 plan-text refinement** (§2 above) — surfaced for fan-in
   visibility.

## 6. Model Attribution

All W1 children (three brief verifiers ×(1–3 rounds), three executors,
three implementation verifiers) and this manager ran on `claude-fable-5`
(Claude Fable 5). Commits carry the
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer.

## 7. Enumerated Wave-Level Claims (for HELP_HUMAN fan-in refutation)

- W1-C1. Commits `723c95b0f`, `faee4faed`, `6326b2f93` exist on
  `claude/piping-r14-pkg04-mechanics` in that order on base `6152908b3`,
  each containing only its tranche's §5-fence paths, and the working tree
  after the closeout commit is clean of all wave-fence paths.
- W1-C2. Every tranche passed a fresh-context adversarial brief
  verification and a fresh-context implementation verification with
  terminal COMMIT-SAFE before its commit; every intermediate BLOCK
  (T1: v1 verifier + first executor; T2: v1 and v2 verifiers; T3: v1
  verifier) is preserved unsoftened with a recorded curing amendment.
- W1-C3. The four registered checks pass at the wave head, and exactly
  one new sweep artifact exists for this branch (the DEC-025 pre-push
  gate), committed with this return.
- W1-C4. The struck Remaining rows are exactly: DEL-04-02 (sole item),
  DEL-04-04 (item 1 of 6), DEL-04-03 (sole item), DEL-04-01 (item 1 of
  2); all other Remaining rows in PKG-04 are byte-unchanged; no new
  Remaining rows were added anywhere.
- W1-C5. No lifecycle state changed (all four deliverables remain
  `IN_PROGRESS`); no threshold, tolerance, acceptance criterion, or
  normative content was created or promoted; PDU-035, DEC-046, D-45,
  D-38, and the friction D-XX holds are untouched.
- W1-C6. The five committed `del1005_payload_binding_*` witnesses are
  byte-identical at the wave head; the pinned tp_runner_015 solve case
  exits 0/COMPLETED with the T2-recorded output SHA unchanged by T3; the
  three historical tp_runner_015 witnesses were not edited.
- W1-C7. No push, pull, fetch, PR, merge, rebase, receipt append, or
  network action was performed by W1 or any child; `loop/LOOP_RECEIPTS.md`
  is untouched by this wave.
- W1-C8. All work stayed inside `projects/chirality-piping/**`;
  `_DomainEngines/**`, app-dev, and PEC are untouched; the offline
  constraint held (no install/provisioning by any child).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
