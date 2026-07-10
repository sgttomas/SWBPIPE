# Piping Loop Receipts

> **Epistemic status: derivative handoff ledger — not authority.** Append-only.
> Current state is always re-derived from the live tree, the decision register,
> the approved DAG, deliverable-local `_STATUS.md` files, git history, and the
> deterministic checks; on any disagreement those sources govern (K-AUTH-1). A
> receipt records only what tools cannot re-derive: owner directions given
> outside governed artifacts, gate outcomes and their rationale, deltas found in
> dated maps (which are never edited), and pointers to the authoritative
> artifacts created elsewhere.

## Rules (fixed — part of the loop protocol)

1. **Pointer, not narrative.** Prefer paths, PR numbers, commit SHAs, decision
   IDs, and TP/test-plan IDs. Work history lives in commits, PRs, decision
   records, run records, and deliverable-local files — never here.
2. **Owner directions verbatim.** Any owner direction not otherwise captured in
   a governed artifact is quoted word-for-word with its date. This is the one
   place chat-only directions become durable.
3. **Measurements as check summaries only.** `self-check pass; harness pytest
   pass; deliverable-status summary run; work-type checks pass` is the maximum —
   counts and tables belong in run records and evidence artifacts.
4. **Stale-map deltas as one-line pointers.** Dated assessments and plan files
   are never edited to match reality; a receipt line says what disagreed and
   where to verify.
5. **Gate outcomes with reason.** stopped / executed / awaiting owner — and
   why, especially no-op outcomes ("all remaining work is owner-shaped"), so
   the next loop neither rediscovers the stop nor invents work around it.
6. **Capped.** Roughly 6–12 lines per receipt. If it wants more, the detail
   belongs in a decision packet, PR description, or project-local record.

## Receipts

- **2026-07-04 — Receipt 0** (loop creation).
  - Start: local `main` synced with origin at the PR #54 merge; loop artifacts
    authored on branch `claude/pec-init-prompt` (PR #55).
  - Owner direction of record (2026-07-04, in-session, Ryan Tufts): "I want the
    new init-prompt to be the current leaner convention, not this older one.
    All projects should have the current development loop structure and
    workflow, and therefore same type of init prompt." — with "Dedicated loops
    per project" selected over pointing at the shared bridge loop.
  - Artifacts created: `projects/chirality-piping/loop/{LOOP_INIT.md
    (byte-identical copy of the generic loop init), this WORKPLAN, this
    ledger}`; `projects/chirality-piping/init/init-prompt.md` rewritten to the
    lean launcher pointing here.
  - Deltas (live tree wins): the legacy entry
    `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` remains in place as a
    dated historical map — status claims inside it are not refreshed by this
    loop's creation.
  - Gate outcome: structure-only tranche; no piping deliverable work selected,
    no lifecycle transition, no register change. First working iteration starts
    at the next launch's Step 0.
  - Parked lanes: owner rulings on the piping register's open rows (re-derive
    from `execution/_Coordination/_DECISIONS/_REGISTER.md` at Step 0).

- **2026-07-05 — Receipt 1** (post-DEL-10-05 merge and E2 draft slice).
  - Start: `REPO_ROOT=/Users/ryan/.codex/worktrees/04f4/chirality`; PR #67
    merged at `433a50edcca2daff13734b0d8e5ce62cd11f492f`; work continued on
    `codex/piping-post-del-10-05-next` from `origin/main`.
  - Owner directions of record (2026-07-05, in-session, Ryan Tufts): "I
    approve the PR #62 so merge it and do what is necessary on account of
    that.  Then proceed towards final DEL-10-05 implementation."; "I approve
    merging the PR and proceeding as necessary from there."
  - Selection: Phase E/E1 was satisfied by merged `DEC-065` / TP-RUNNER-015;
    next lawful tranche was E2 `DEL-09-04` draft validation-manual
    reproduction slice, using the E1 runner evidence.
  - Artifacts: commit `8c57cf5f467157d9d2a47654ec68e34cda2c1743`;
    `docs/validation_manual/headless_runner_reproduction.md`;
    `validation/witness/inputs/`; DEL-09-04 run record
    `WORKING_ITEMS_RUN_2026-07-05_TP-VALIDMANUAL-RUNNERREPRO-001.md`.
  - Checks: focused generator/runner/schema checks pass; self-check exit 0;
    harness pytest pass; DEC-025 five-surface sweep pass with summary
    `validation/evidence/sweeps/SWEEP_20260705T050428Z_8c57cf5f4671.json`.
  - Gate outcome: executed within R5/E2 draft-evidence fence; no DEL-09-04
    lifecycle transition; remaining E2 public benchmark thresholds, clean-env
    demonstration, and R5-exit decisions remain parked/human-gated.

- **2026-07-09 — Receipt 2** (orientation + physical-model evaluation, owner-steered).
  - Start: `origin/main` at the PR #135 merge (`18334da97`); piping tree
    unchanged since Receipt 1; work on `codex/piping-physical-model-eval`.
  - Owner steer of record (2026-07-09, launcher `Steer` line, Ryan Tufts):
    "Report back on the current state of development and what, if anything,
    is currently planned.  Then provide a focused evaluation of the physical
    model, both in terms of using the appropriate methods and also
    comprehensive coverage of the domain being modelled.  You can use
    research subagents if warranted."
  - Step 0: no register rulings newer than `DEC-065`; open gates `D-12`,
    `D-07b` AWAITING_RULING, `D-06b` NOT_PREPARED; status summary run
    (ISSUED=1, CHECKING=8, IN_PROGRESS=92); self-check exit 0.
  - Artifact: `plans/ASSESSMENT_2026-07-09_physical_model_evaluation.md`
    (dated map; candidate follow-ups CAND-1..5 in its final section).
  - Gate outcome: reporting tranche only — no lifecycle transition, no
    register change, no deliverable code work; CAND-1..5 parked for owner
    ruling/adoption.

- **2026-07-09 — Receipt 3** (physical-model program adoption, same session as Receipt 2).
  - Owner directions of record (2026-07-09, in-session, Ryan Tufts): "I want
    to introduce flexibility factors and bend flexibility together and now
    properly solve those elements.  I approve CAND-2 and want to include the
    flexibility factors rather than disclosing that there are none.  I agree
    with your other candidates.  More importantly I want to fold this into
    the loop instruction documentation - in whatever the most appropriate
    form is for that.  What's your proposal?" — then, over the proposed
    five-part shape: "this shape is approved.  Proceed accordingly."
  - Artifacts: `plans/PLAN_2026-07-09_physical_model_mechanics.md`; register
    rows `D-34`–`D-37` + packet `D-34_bend_flexibility_stiffness_realization.md`
    + ruling records `D-35/36/37_RULING_2026-07-09.md`; codifications
    `DEC-066`–`DEC-069` in `SOFTWARE_DECOMP.md` §12; WORKPLAN owner-intent
    addendum + tactical pointer-index line (this directory).
  - Deltas: assessment CAND-1 superseded by implementation (dated map not
    edited, per rule 4).
  - Checks: self-check exit 0; harness pytest pass.
  - Gate outcome: governance/docs tranche executed at owner direction;
    `D-34` method fork AWAITING_RULING (O-A recommended) — P1 implementation
    starts on that ruling; no lifecycle transition, no code change.

- **2026-07-10 — Receipt 4** (D-34 ruling, same session).
  - Owner ruling of record (2026-07-10, in-session, Ryan Tufts): the D-34
    packet options were presented in-session with O-A recommended; owner
    selected **"O-B: curved-bend macro-element"** via structured selection.
  - Artifacts: packet §7 disposition filled
    (`D-34_bend_flexibility_stiffness_realization.md`); register row `D-34`
    → RULED; codification `DEC-070` in `SOFTWARE_DECOMP.md` §12.
  - Checks: self-check exit 0; harness pytest pass (this branch).
  - Gate outcome: ruling recorded; P1 (curved-bend macro-element) is now
    lawful bounded-tranche work under the mechanics plan once PR #141
    merges — O-B noted as the larger tranche, splittable formulation →
    assembly → recovery → benchmark; no code change under this receipt.
