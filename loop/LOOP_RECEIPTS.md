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
   place chat-only directions become durable. Exception (owner-ruled
   2026-07-10): a direction whose entire content and outcome are re-derivable
   from git/PR history — e.g. "merge PR #N", executed as the recorded merges —
   needs no receipt entry; rule 1 governs (work history lives in commits and
   PRs). Quote a merge direction only when it carries anything beyond the
   merge itself (scope grants, conditions, follow-on instructions).
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

- **2026-07-10 — Receipt 5** (P1 execution, owner-directed fable-subagent orchestration).
  - Owner direction of record (2026-07-10, in-session, Ryan Tufts): "Proceed
    accordingly. Execute the work using fable subagents.  Orchestrate and
    evaluate their work. Work in parallel where agents would have disjoint
    write scopes." Also: "merge PR #141" (executed; merged with PR #136 at
    `0f4d95944`).
  - Executed: P1 sub-tranches 1–3 on `codex/piping-p1-curved-bend` —
    run records `WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P1-CURVEDBEND-001.md`
    and `..._TP-PMM-P1-CURVEDBEND-002.md` (DEL-04-01); witness
    `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL`; benchmark agreement 5.8e-8;
    k-sweep monotonicity asserted.
  - Checks: crate/product/python suites pass (56/27/369); self-check exit 0;
    DEC-025 sweeps `SWEEP_20260710T134128Z_16ba9db23114.json` (clean) and
    `SWEEP_20260710T151446Z_b77e721b2028-dirty.json` (all five surfaces
    pass; sole dirty path is external concurrent-loop state in
    `domains/piping-design/_Sources/**` — untouched per the external-state
    rule).
  - Gate outcome: executed within the DEC-070 fence; PR awaiting owner
    merge; no lifecycle transition. Named residuals: nonlinear-loop
    explicit-stiffness slot (blocking diagnostic in place), arc interior
    stations, consistent arc distributed loads, GUI emission of
    `bend_pipe_ref`.

- **2026-07-10 — Receipt 6** (P2/P3/P4 execution, owner-steered subagent fan-out).
  - Start: `main` at the PR #142 merge (`4ef14f686`); steer of record (launcher, Ryan
    Tufts): "Fan out and fan in with subagents where warranted.  Use the subagent model
    selection protocols in the LOOP instructions."
  - Owner directions of record (2026-07-10, in-session, Ryan Tufts): "proceed with
    merging both PRs" (PR #143, PR #144 — merged at `73b65246f` / `35abf5596`); "resume"
    (P2 agent continuation after transient API terminations).
  - Executed: P4 → PR #143 (TP-PMM-P4-ABSENTDOMAINS-001; DEC-069 naming mirrored onto
    the v0.2 forward-authority surface, flagged for owner review, merged); P3 → PR #144
    (TP-PMM-P3-MILLTOL/OCCLOADGEN/MODULUSBASIS-001, merged); P2 + P1 residual →
    PR #145 (TP-PMM-P1-CURVEDBEND-003, TP-PMM-P2-COMPLEMENTARITY-001,
    TP-PMM-P2-FRICTION-001) awaiting owner merge.
  - New gate: register row `D-38` (temperature interpolation policy) AWAITING_RULING —
    exact selection shipped as conservative floor.
  - Checks: all suites pass per run records; self-check exit 0; harness pytest pass;
    DEC-025 sweeps `SWEEP_20260710T201841Z_c6430cbd5c9b.json` and
    `SWEEP_20260710T214950Z_37b327cf0687.json` (both clean).
  - Gate outcome: executed within DEC-067/068/069/070 fences; no lifecycle transition;
    parked owner gates D-12, D-07b (AWAITING_RULING), D-06b (NOT_PREPARED), D-38.
    Named residuals: sliding prior-iterate direction / nonconvergence corner (run
    records), arc interior stations, temperature-indexed shear modulus, sub-span wind
    marking, GUI emission of new schema slots.

- **2026-07-10 — Receipt 7** (adopted-ruling execution: DEC-061 E3 + DEC-060 coverage tranche).
  - Start: `main` at the PR #146 merge (`199c6d545`); PR #145 merged since Receipt 6;
    steer: none (autonomous run). Step 0: no rulings newer than Receipt 6; census
    ISSUED=1/CHECKING=8/IN_PROGRESS=92; self-check exit 0; harness pytest pass.
  - Executed (parallel fable subagents, disjoint scopes, adopted-but-unexecuted
    rulings per WORKPLAN §5): DEC-061/E3 → `TP-E3-PDFEMIT-001`, PR #148
    (`core/reporting/pdf_emitter`, run record in DEL-08-01); DEC-060 first coverage
    tranche → `TP-E8-COVTELEM-001`, PR #147 (all three lanes measured, first
    clean-head artifact `COVERAGE_20260710T232606Z_e9cd806811b3.json`, no floors;
    run record in DEL-09-05). Both PRs awaiting owner merge; both touch
    `plans/PLAN_COMPLETION_LOG.md` — second merge needs a trivial rebase.
  - Checks: per run records — crate/python/desktop suites pass; self-check exit 0;
    DEC-025 clean-head sweeps `SWEEP_20260710T234618Z_dc63c4a83eed.json` and
    `SWEEP_20260710T233804Z_1094b0deb266.json` pass.
  - Deltas (live tree wins): D-10b packet's "assembled section model" was private and
    HTML-shaped — made public emission-neutral with byte-identity verified; package
    assembly lives in `pdf_emitter`, not `report_generator` (dependency cycle) — both
    in the TP-E3 run record. Completion-plan E4 row understates the live tree
    (`core/security/redaction` + schema + docs exist, tests pass).
  - Gate outcome: executed within the DEC-060/DEC-061 fences; no lifecycle transition,
    no register change; parked gates D-12, D-07b, D-38 (AWAITING_RULING), D-06b
    (NOT_PREPARED, R5-exit lead-up). E3/E8 remainders kept in their plan rows.

- **2026-07-10 — Receipt 8** (receipt-rule clarification, owner-directed; same session
  as Receipt 7).
  - Context: PRs #147/#148/#149 merged at owner direction (`a331c8a83`/`4b3ac073d`/
    `ea627df63`; the merges themselves are git history, not receipt content — this
    entry exists only for the protocol change below).
  - Owner direction of record (2026-07-10, in-session, Ryan Tufts): 'Make this clear
    in your instructions: "merge direction itself needs no receipt addendum — the
    merges are re-derivable from git history, which per the ledger rules is where
    work history lives."'
  - Artifacts: rule 2 exception added to this ledger's Rules; WORKPLAN protocol
    step 3 aligned (this directory).
  - Gate outcome: governance/docs edit executed at owner direction; no code, no
    lifecycle transition, no register change.

- **2026-07-10 — Receipt 9** (five-tranche fable fan-out: E2/E7 + mechanics residuals).
  - Start: `main` at the PR #151 merge (`3752891a3`); steer of record (launcher, Ryan
    Tufts): "Fan-out and fan-in with subagents where warranted.  Use only `fable`
    models, contrary to your loop instructions." Step 0: no rulings newer than
    Receipt 8; census ISSUED=1/CHECKING=8/IN_PROGRESS=92; self-check exit 0.
  - Executed (five parallel fable subagents, isolated worktrees, disjoint scopes; all
    adopted-but-unexecuted scope per WORKPLAN §5): E7 scanner-extension tooling
    (DEC-058) → `TP-E7-SCANEXT-001`, PR #152; DEC-067 nonconvergence-diagnostic
    corner → `TP-PMM-P2-NONCONVDIAG-001`, PR #153; E2 validation manual, 63 case
    pages (DEC-054-carried §16.2/§16.5) → `TP-E2-VALMANUAL-001`, PR #154; GUI
    emission of DEC-068/070 user-entered slots → `TP-PMM-GUIEMIT-001`, PR #156;
    DEC-070 arc residuals (consistent distributed loads + interior stations) →
    `TP-PMM-P1-CURVEDBEND-004`, PR #157. All PRs awaiting owner merge; run records
    in DEL-08-05 / DEL-04-04 / DEL-09-04 / DEL-07-02 / DEL-04-01; completion-log
    entries landed by this fan-in PR (agents held them to avoid the Receipt-7
    dual-prepend conflict).
  - Checks: per run records — all crate/python/desktop suites pass; self-check
    exit 0 each tranche; DEC-025 clean-head sweeps
    `SWEEP_20260711T012810Z_1d160589cbea.json`,
    `SWEEP_20260711T012926Z_658e15a5bb46.json`,
    `SWEEP_20260711T013618Z_4012b4c57496.json`,
    `SWEEP_20260711T014521Z_8b456e6e1c02.json`,
    `SWEEP_20260711T020757Z_67f06454737f.json` all pass (fresh-worktree
    `npm ci` retry chains recorded where they occurred; no port collisions).
  - Deltas (live tree wins): practitioner harness lives at repo root
    `tools/practitioner_harness/`, not under the project (three agents confirmed);
    `operation_applier` accepts no component-geometry `set_field` paths — GUI slots
    (like all existing geometry fields) enter/emit but await a core-scoped
    field-rule-registry tranche; live runner solve emits 830 result_refs vs 822 in
    the TP-RUNNER-015 witness (refresh is bounded DEL-10-05 work); stale
    `validation/benchmarks/mechanics/README.md` registration note fixed in both
    PR #154 and PR #157 — the one predicted merge conflict; prefer #157's text.
  - Gate outcome: executed within DEC-054/058/067/068/070-class adopted authority;
    no lifecycle transition, no register change; owner scan/sign-off acts untouched
    (`validation/evidence/releases/` left absent). Parked gates: D-38, D-12, D-07b
    (AWAITING_RULING), D-06b (NOT_PREPARED). Named lawful remainder: E3 container
    assembly, E5 packaging tranche + DEC-059 export-pipeline build, E4 remainder,
    E6 issue-template authoring, E8 gate-outcome records, applier field-rule
    registry extension.

- **2026-07-10 — Receipt 10** (wave 2 of the same session: E3/E5 remainders + applier
  seam; four fable subagents).
  - Start: wave-1 PRs #152/#153/#154/#156/#157 merged at owner direction (git history;
    the one predicted README conflict resolved in PR #157's branch keeping its text);
    PR #158 held open at owner direction for this fan-in.
  - Executed (isolated worktrees, disjoint scopes — zero cross-PR file overlaps): E3
    container assembly (DEC-028/057/061) → `TP-E3-CONTAINER-001`, PR #160 (new crate
    `core/reporting/report_package`, deterministic `.opsproj`); DEC-059 sanitized-
    export pipeline build → `TP-E5-EXPORTPIPE-001`, PR #161 (consumes the merged
    PR #152 lint CLI as-is); operation-applier field rules (DEC-020 seam ordinary R5
    work) → `TP-APP-R5-FIELDRULES-001`, PR #162 (component geometry + DEC-068 slots
    now apply; corpus 75 cases, 66–75 pending human review entry); DEC-057-named E5
    packaging tranche → `TP-E5-PACKAGING-001`, PR #163 (bundle.active + real `.icns`
    + deterministic zip/checksum + §8 record mechanics; real build boot-checked).
    All four PRs awaiting owner merge; run records in DEL-08-01 / DEL-10-04 (×2) /
    DEL-16-02; completion-log entries landed by this fan-in commit.
  - Checks: per run records — all suites pass; self-check exit 0 each tranche;
    DEC-025 clean-head sweeps `SWEEP_20260711T023430Z_a4fbed24b6b4.json`,
    `SWEEP_20260711T024350Z_b9ba51af9f3c.json`,
    `SWEEP_20260711T024706Z_9fc322fcad8d.json`,
    `SWEEP_20260711T025412Z_0b9944768560.json` +
    `SWEEP_20260711T025817Z_3115a08cdfee.json` (PR #163 retry chain incl. one
    port-5174 collision-class failure) all pass.
  - Deltas (live tree wins): `geometry.bend_pipe_ref` lives in the session/preview
    contract (`ComponentGeometryInput`), not `schemas/model.schema.yaml` (Component
    geometry is `additionalProperties: Quantity` there); canonical schema names
    (`exposed_element_refs`/`g_factors.{x,y,z}`) differ from the session shape the
    GUI+applier share (`exposed_pipe_refs`/`g_factor_x|y|z`) — parallel surfaces,
    tests pin the correspondence; PR #162's base predated the PR #156 merge, so its
    "GUI emitters don't exist" note is stale — post-merge the GUI-entered slots
    apply end-to-end (same session paths; verify after both merge); root
    `Cargo.lock` at old main was stale w.r.t. the in-repo curved_bend path crate
    (refreshed in PR #163); no runner verb home exists for the report container
    (DEC-065 vocabulary is ruled — binding recorded as residual, no verb added).
  - Gate outcome: executed within DEC-028/054/057/059/061/020-class adopted
    authority; no lifecycle transition, no register change; no release act, no
    publication, no signing (D-06b NOT_PREPARED, untouched), no public repo, no CI
    activation. Parked gates: D-38, D-12, D-07b (AWAITING_RULING), D-06b
    (NOT_PREPARED); corpus cases 66–75 human review entry. Named lawful remainder:
    E4 remainder, E6 issue-template authoring, E8 gate-outcome records, desktop
    menu + caller-side save binding for the report container, runner export-results
    payload binding, `geometry.center_of_gravity` vector-payload ruling
    (new D-XX if sought).

- **2026-07-10 — Receipt 11** (wave 3 of the same session: E4/E6/E8 closure of the
  unblocked Phase E surface; three fable subagents).
  - Start: wave-1+2 PRs and fan-in #158 merged at owner direction (git history);
    Receipt 10 named E4/E6/E8 as the lawful remainder.
  - Executed (isolated worktrees, disjoint scopes): E6 public issue templates
    (DEC-027 posture, project-local `.github/ISSUE_TEMPLATE/`, inert in monorepo) →
    `TP-E6-ISSUETEMPLATES-001`, PR #164 (merged at owner direction); E8 gate-outcome
    records (governed-values-only per-family records; TBD stays TBD; no labels —
    PB-TBD-003; no floor promotion, DEC-060 base 1 artifact/1 commit recorded) →
    `TP-E8-GATERECORDS-001`, PR #166 (merged at owner direction); E4 redaction
    export-controls app binding (TS mirror + export-workflow panel,
    private-by-default, shared 23-case parity corpus over all 16 reason codes) →
    `TP-E4-REDACTION-001`, PR #167 (awaiting owner merge). Run records in
    DEL-01-03 / DEL-09-05 / DEL-12-02; completion-log entries landed by this
    fan-in commit.
  - Checks: per run records — suites pass (E4: python 439, desktop 471; E8: pytest
    421 + harness 263; E6: docs-class + sweep anyway); self-check exit 0 each;
    DEC-025 clean-head sweeps `SWEEP_20260711T031432Z_1581b8c0de72.json`,
    `SWEEP_20260711T032140Z_e2ea37194c8a.json` +
    `SWEEP_20260711T032717Z_af74a1096ff0.json`,
    `SWEEP_20260711T035249Z_9aa5dcdc1a29.json` all pass.
  - Deltas (live tree wins): E4's live remainder confirmed as the app-side binding
    (core contract complete since 2026-06-07; DEL-12-02 records show the binding
    was never picked up); browser/wasm app cannot call the python core — parity
    corpus pattern used instead of a runtime bridge (existing hash-seam precedent);
    first E4 practitioner-pytest failures were the main checkout's live state, not
    the tranche (clean from the worktree); E8's live coverage-artifact census:
    1 of the ≥5/≥2-commit floor-promotion prerequisite.
  - Gate outcome: executed within DEC-054/027/058/060-class adopted authority; no
    lifecycle transition, no register change; no release act, publication, signing,
    scan, or intake mechanism. Parked gates: D-38, D-12, D-07b (AWAITING_RULING),
    D-06b (NOT_PREPARED); PB-TBD-003 (release-label vocabulary); §17.5 legal review;
    corpus cases 66–75 human review entry. With this wave the unblocked Phase E
    agent surface is exhausted — remaining named work is either owner-gated or
    was newly unblocked by today's merges (desktop menu + caller-side save binding
    for the report container, runner export-results payload binding, REXC-REQ-012
    breadth, E2 tolerance-record fill, arc pressure-thrust treatment).

- **2026-07-10 — Receipt 12** (loop-instruction consolidation, owner-adopted;
  deliverables become the work-discovery surface).
  - Start: main at the PR #169 merge (3cdc265ac); PR #165 refreshed against
    Receipts 9–10 and again against wave-3 Receipt 11 (concurrent-loop merges).
  - Owner directions of record (2026-07-10, in-session, Ryan Tufts): "I really want
    the deliverables themselves to be the means of discovering work going forward,
    with the loop mechanics included as the development loop instructions. ... We
    need to simplify these instructions and avoid creating unnecessary registers or
    plans."; on the presented consolidation proposal: "Adopted.  Use fable subagents
    where warranted."
  - Artifacts (commits on PR #165): `WORKPLAN_2026-07-10_piping_loop.md`
    (supersedes the 2026-07-04 plan; adds fence F-PIP-5); `_COORDINATION.md`
    rewritten as a ruled-record stub (stage record, DEC-040 augmentation, H4
    posture preserved verbatim); `## Remaining` sections in 26 deliverable
    `_STATUS.md` files (50 open items rehomed from plans, refreshed against
    Receipts 9–11 / PRs #152–#167: 11 landed rows dropped, 9 sharpened, 6 residuals
    added incl. DEL-10-05, DEL-16-02; rehoming record incl.
    deliberately-not-rehomed F3 / FR-024 / Phase-I-generator items:
    `plans/PLAN_COMPLETION_LOG.md` 2026-07-10 entry); plan retirement banners;
    `NEXT_INSTANCE_PROMPT.md` historical banner; `list_deliverable_status.py`
    `RemainingItems` column + tests; `LOOP_INIT.md` §7 synced to the 2026-07-05
    owner revision; README/docs entry pointers aligned. PR from branch
    `claude/chirality-piping-loop-init-57a151`, awaiting owner merge.
  - Deltas (live tree wins): the concurrent loop's waves 1–3 landed nine of the
    originally rehomed rows while this PR was open (E2/E3/E4/E5/E6/E7/E8 slices,
    mechanics residuals) — each drop/sharpen is cited in the affected `Remaining`
    bullets; wave-3 newly named REXC-REQ-012 breadth and the arc pressure-thrust
    treatment, rehomed here (DEL-12-02, DEL-04-01).
  - Checks: self-check exit 0; harness pytest pass; coordination tests pass;
    deliverable-status summary run (Remaining items: 50 across 26 deliverables);
    DEC-025 clean-head sweep `SWEEP_20260711T040758Z_e648462f1d05.json` pass.
  - Gate outcome: coordination/control consolidation executed at owner adoption;
    no lifecycle transition, no register change; parked gates unchanged: D-12,
    D-07b, D-38 (AWAITING_RULING), D-06b (NOT_PREPARED).

- **2026-07-11 — Receipt 13** (D-41 concordance activation ruled; on-ruling
  tranche).
  - Start: main at the PR #177 merge (551f84ef6); fresh worktree branch
    `claude/piping-concordance-2026-07-11`.
  - Owner ruling of record (2026-07-11, in-session D-41 decision slate, Ryan
    Tufts), answers verbatim: scope — "O-A: Whole-corpus (Recommended)";
    declaration — "Affirm the 2026-07-11 declaration" (reaffirming "I will
    suspend work in Chirality Piping for the time being."); frozen SHA —
    "551f84ef6 — current origin/main (Recommended)".
  - Artifacts: `D-41_RULING_2026-07-11.md`; packet §10 transcription; register
    row D-41 → RULED; `DEC-073` (SOFTWARE_DECOMP.md §12); 100 `_STATUS.md`
    bootstrap items flipped to `(ruled: D-41, 2026-07-11)` with pinned SHA
    `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; ruled profile surface stub
    `docs/CHECKING_ENTRY_PROFILES.md`. DEL-01-01 untouched.
  - Gate outcome: ruled/executed; HARD RULE — this tranche merges to main
    (owner merge) before any concordance dispatch; discovery reads bind to a
    frozen worktree at 551f84ef6; R0 calibration (DEL-04-01, DEL-10-05,
    DEL-12-02) reports to owner before wave scale-out.

- **2026-07-11 — Receipt 14** (D-41 tranche merged; R0 calibration executed;
  same session as Receipt 13).
  - Owner direction of record (2026-07-11, in-session, Ryan Tufts, on the
    PR #179 merge gate): "You have my permission to merge it yourself." —
    self-merge grant for PR #179 only; merged as e3998349b (HARD RULE
    satisfied before dispatch). Ruling SHA backfilled into
    `D-41_RULING_2026-07-11.md` per the D-31 precedent.
  - Run opened: `execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/`
    (RUN_BASIS.md; census 101 = 100 IN_PROGRESS + 1 ISSUED re-verified;
    DAG-007 pointer re-verified live at frozen tree).
  - R0 executed per plan §8: three fable pilot ledgers + notes and an
    independent reviewer pass under `R0_CALIBRATION/`; reviewer verdict
    READY WITH NAMED CONVENTIONS (28/0/2 citation checks; 8 conventions
    proposed for R1+; details in R0_REVIEW.md — never here).
  - Gate outcome: stopped/awaiting owner — R0 findings reported in-session
    per the owner's R0-first direction; wave scale-out (R1+) and the
    convention set await the owner's response. No lifecycle transition; no
    deliverable or product surface edited; run artifacts only.

- **2026-07-11 — Receipt 15** (R0b convention-calibration round; same session).
  - Owner directions of record (2026-07-11, in-session slate answers,
    verbatim): scale-out question — "Another R0 round first"; PR #180 —
    "Merge it yourself" (self-merge grant for #180 only; merged 4a594adfa).
  - R0b executed: convention set under test
    `R0B_CALIBRATION/R0B_CONVENTIONS.md`; pilots DEL-05-03/DEL-07-05/
    DEL-09-01 (fable; frozen tree 551f84ef6; tree verified clean); reviewer
    pass `R0B_CALIBRATION/R0B_REVIEW.md`.
  - Outcome: SCALE-READY — divergences 7 RESOLVED / 4 PARTIALLY / 0 NOT;
    spot-checks 24/1/1 (one count correction recorded); 13 addenda + 2
    repairs proposed; R0/R0b ledgers marked calibration-only
    (predates-conventions note in RUN_BASIS.md).
  - Gate outcome: stopped/awaiting owner — conventions+addenda adoption and
    R1+ scale-out are the owner's ruling; no lifecycle transition; run
    artifacts only.

- **2026-07-11 — Receipt 16** (scale-out adopted; R1 complete; paused before
  R2; same session).
  - Owner directions of record (2026-07-11, in-session, verbatim): scale-out
    slate — "Adopt + scale out (Recommended)"; PR #182 — "Merge it yourself"
    (merged 07781c664); pause — "Pause once R1 is done.  Record your
    progress as appropriate for this task.  And give me a prompt to resume
    the work."
  - Artifacts: `R1_CONVENTIONS.md` (adopted binding set: conventions 1–8 +
    addenda 1–13 + repairs + wave plan); R1 index set in the run folder
    (DELIVERABLE_INVENTORY 101 / IMPLEMENTATION_SURFACES 231 /
    VERIFICATION_INDEX 147 suites / VALIDATION_AND_PROVENANCE_INDEX 396 /
    AUTHORITY map + DECISIONS_INDEX 74); RUN_BASIS phase log incl. pause
    entry.
  - Gate outcome: paused at owner direction after R1. R2 not started (the
    W1 launch aborted mechanically before any agent ran; no wave
    artifacts). Resume: R2 W1 (PKG-00–03) per RUN_BASIS resume point. No
    lifecycle transition; discovery read-only; repair tranches remain
    gate-stopped.

- **2026-07-11 — Receipt 17** (concordance resume: pre-dispatch verification +
  owner steer of record; R2 W1 dispatch follows).
  - Start: dedicated worktree
    `.claude/worktrees/chirality-piping-d41-concordance-9811cb`, branch
    `claude/chirality-piping-d41-concordance-9811cb`, HEAD `052b3c2b2`
    (includes the PR #184 merge `5e2763f0a`), tree clean. Account verified
    ryan@chirality.ai ("ryan@chirality.ai's Organization"); orchestrator model
    `claude-fable-5`; opus subagent availability verified by live ping.
  - Frozen evidence worktree recreated at
    `.claude-worktrees/piping-frozen-551f84ef6`; HEAD verified equal to the
    full pinned SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; porcelain
    clean; strictly read-only.
  - Resume boundary re-verified: R1 complete; R2 never started; no `WAVES/`
    or `PACKAGE_SUMMARIES/` artifacts exist (the aborted W1 launch produced
    no agent output); R0/R0b ledgers remain calibration-only.
  - Owner steer of record (2026-07-11, resume prompt, verbatim): "Steer (this
    run, owner-ruled 2026-07-11, overrides LOOP_INIT §7 defaults per its own
    terms): fable orchestrates; opus for R2 discovery pilots in waves W1-W5,
    EXCEPT fable for ALL discovery pilots in W2 (PKG-04-05, solver mechanics —
    engineering-adjudication risk), PKG-12 in W4 (security/privacy, F-PIP-1
    fence-adjacent, first SECURITY-class encoding), and PKG-01 in W1 (F-PIP-2
    professional-claims fence; DEL-01-01 ISSUED). Fable at high effort for the
    per-wave fan-in verification pass, scoped to: all self-flagged rows, all
    non-ALIGNED rows, plus >=2 ALIGNED/IMPLEMENTED_UNDOCUMENTED rows sampled
    per ledger. Any ledger the verification pass finds defective is re-run by
    a fable pilot. R3 synthesis and R6 backcheck agents are fable."
  - Gate outcome: executing — R2 W1 (PKG-00–03, 25 pilots) dispatched under
    `R1_CONVENTIONS.md`; ≤4 concurrent pilots; wave boundary =
    validate/commit/push/PR/receipt, then STOP for owner merge direction.

- **2026-07-11 — Receipt 18** (R2 W1 complete; wave-boundary stop).
  - Executed per Receipt 17's steer: 25 W1 ledgers (504 rows; opus pilots,
    fable for PKG-01), fable high-effort fan-in (17 SOUND / 8 DEFECTIVE),
    8 fable re-runs + 3 owning-pilot string corrections, Part C SECURITY
    spot-checks PASS. Artifacts: `WAVES/W1/` (25 ledgers + notes + 4
    verification reports) and `PACKAGE_SUMMARIES/PKG-0{0..3}.md` in the run
    folder; phase detail in RUN_BASIS (W1-complete entry) — never here.
  - Checks: full-wave structural validation pass (header/enums/ClaimID/
    addenda sweeps, 0 errors); notes histograms reproduce; frozen worktree
    porcelain empty throughout; no product/deliverable surface touched, so
    no DEC-025 sweep owed (run artifacts only).
  - Notable for W2 calibration (detail in package summaries): rev-0.7
    pointer-drift encoding adjudicated STALE-side with owner-calibration
    caveat; SECURITY marker/routing harmonization; overtaken Gate-C review
    prose encodes STALE on affected DECL rows.
  - Gate outcome: stopped/awaiting owner — wave branch pushed and PR opened
    (branch `claude/chirality-piping-d41-concordance-9811cb`); W1 merge
    direction or explicit per-PR self-merge grant required before W2
    (PKG-04–05, all-fable) dispatches. No lifecycle transition; no register
    change; discovery read-only.

- **2026-07-11 — Receipt 19** (pause after W1; same session as Receipts
  17–18).
  - Owner direction of record (2026-07-11, in-session, verbatim): "Once this
    wave fans-in and you have dealt with everything so it's safe to pause,
    do so.  Do not launch another wave."
  - State at pause: W1 fan-in, re-runs, revalidation, package summaries,
    Receipt 18, and PR #187 were already complete when the direction
    arrived; no agents in flight; no partial artifacts. Pause entry appended
    to RUN_BASIS (resume point: W2 per the Receipt-17 steer + W1 calibration
    notes, after the owner acts on PR #187).
  - Gate outcome: paused at owner direction. W2 is NOT to be launched
    without new owner direction; PR #187 merge remains the owner's act.

- **2026-07-11 — Receipt 20** (resume after W1 pause; R2 W2 dispatched).
  - Owner directions of record (2026-07-11, in-session, verbatim): "merge
    the PR and then wait for permission to proceed further." — executed:
    PR #187 merged to main as `667a679594f271ec45b3daa05f0d98a6575236bd`;
    then "This is your permission to resume."
  - Pre-dispatch state: branch `claude/chirality-piping-d41-concordance-9811cb`
    clean at `f99eb7f78`; frozen evidence worktree verified at
    `551f84ef6be656f1603ce0acfa5e3935aa9683c7`, porcelain empty; no partial
    W2 artifacts; `WAVES/W2/` created empty.
  - Gate outcome: executing — R2 W2 (PKG-04–05, 11 deliverables, ALL-fable
    discovery pilots per the Receipt-17 steer) dispatched under
    `R1_CONVENTIONS.md` plus the W1 calibration notes
    (`PACKAGE_SUMMARIES/PKG-0{0..3}.md`); ≤4 concurrent pilots; fable
    high-effort fan-in after the wave; wave boundary =
    validate/summarize/commit/push/PR/receipt, then STOP for owner merge
    direction (no grant reuse).

- **2026-07-11 — Receipt 21** (paused mid-W2 at owner direction; batch 2
  complete).
  - Owner direction of record (2026-07-11, in-session, verbatim): "Pause
    again when Batch 2 is in and fully accounted for.  We will continue
    with Batch 3 when I give you approval."
  - State at pause: W2 batches 1–2 complete and accounted for — 8 of 11
    ledgers written (DEL-04-01..06, DEL-05-01/02; 181 rows), each batch
    structurally validated (0 errors) and committed (batch 1 `951e3a94b`,
    batch 2 `4d1c96a04`); frozen worktree porcelain empty throughout; no
    agents in flight; no partial artifacts. Fan-in has NOT run — it is a
    whole-wave pass and waits for batch 3.
  - Gate outcome: paused at owner direction. Resume point: dispatch W2
    batch 3 (DEL-05-03/04/05, fable pilots) on owner approval, then the
    wave fan-in and the standard wave-boundary protocol. Pause entry
    appended to RUN_BASIS.

- **2026-07-12 — Receipt 22** (resume: W2 batch 3 dispatched; standing
  self-merge grant through R4 boundary).
  - Owner direction of record (2026-07-12, in-session, verbatim): "Proceed
    now with the immediate next step of dispatch batch 3.  You may continue
    through successive PRs in this matter by approving merger yourself, not
    waiting for human ruling until you get to R4."
  - Interpretation of record: standing authorization, scoped to this
    concordance exercise, for the orchestrator to open and self-merge the
    wave-boundary PRs and continue autonomously through W2–W5, R3, R6, and
    RUN_SUMMARY. It supersedes — by explicit owner direction — the earlier
    per-PR merge-grant pacing ("Never reuse an earlier grant") for merge
    approvals only. Everything else stands: Receipt-17 model steer, ≤4
    concurrent pilots, per-batch structural validation and commits, fan-in
    scope, fences F-PIP-1..5, frozen-tree read-only discipline, and the
    hard STOP before R4/R5 (no decision gates, repairs, lifecycle
    transitions, or register/DAG changes without explicit owner ruling).
  - Pre-dispatch state verified: branch clean at `cffc205c7`; frozen
    worktree at pinned SHA `551f84ef6` with empty porcelain; W2 batches 1–2
    validated and committed; scratchpad W2 brief intact.
  - Action: dispatched W2 batch 3 — DEL-05-03, DEL-05-04, DEL-05-05, three
    concurrent fable discovery pilots under W2_PILOT_BRIEF (R1_CONVENTIONS
    + W1 calibration items). RUN_BASIS resume/grant entry appended.

- **2026-07-12 — Receipt 23** (R2 wave W2 complete; PR self-merged under
  standing grant).
  - Batch 3 (DEL-05-03 19 rows / DEL-05-04 21 / DEL-05-05 21) returned
    clean: 0 validation errors, no STOP-worthy contradictions, frozen
    porcelain empty; commit `c3a1986a4`. Wave totals: 11 ledgers, 242
    rows (PKG-04 133, PKG-05 109).
  - Fan-in (fable, high effort, one verifier per package, owner-ruled
    scope incl. all self-flagged and non-ALIGNED rows): 11/11 SOUND, 100
    PASS / 25 QUALIFIED / 0 FAIL, zero re-runs. Reports
    `WAVES/W2/W2_VERIFICATION_PKG-04.md`, `..._PKG-05.md`.
  - Non-defect corrections routed to owning pilots (never repaired
    centrally): ATTESTED record-not-present markers on DEL-04-03-REQ-004
    and DEL-04-05-REQ-006; declaration-level evidence pointers with
    explicit qualifier on DEL-05-04-REQ-014. Each pilot re-verified the
    underlying facts against the frozen tree before editing; single-cell
    diffs confirmed; all 11 ledgers revalidated (0 errors); commit
    `528c8c7aa`.
  - Package summaries written: `PACKAGE_SUMMARIES/PKG-04.md`, `PKG-05.md`
    (histograms computed from ledgers; cross-ledger risks itemized for W3
    calibration and R3).
  - Wave boundary executed under the 2026-07-12 standing owner grant
    (Receipt 22): PR opened and self-merged by the orchestrator; W3
    (opus pilots per the Receipt-17 steer) proceeds without a per-PR
    stop. Hard STOP before R4/R5 unchanged.

- **2026-07-12 — Receipt 24** (R2 wave W3 complete; PR #198 opened, merge
  left to owner; PAUSED at owner direction before W4; handoff prepared).
  - W3 (PKG-06..08, 19 deliverables, opus pilots per the Receipt-17 steer)
    executed in five ≤4-concurrency batches (commits `28a9c97a9`,
    `3ca576915`, `f898a6fb0`, `816add923`, `9ee1ef92f`; batch 4 recovered
    at session resume). Post-correction wave totals: 399 rows (PKG-06 99,
    PKG-07 168, PKG-08 132).
  - Fan-in (fable, high effort, one verifier per package): **19/19 SOUND**,
    199 PASS / 49 QUALIFIED / 7 FAIL, zero DEFECTIVE, zero re-runs; all
    FAILs string-correctable. Part-C-style SECURITY spot-check of
    DEL-08-05's convention-6 split: PASS. Reports
    `WAVES/W3/W3_VERIFICATION_PKG-0{6,7,8}.md`.
  - Ten non-defect corrections routed to owning pilots (never centrally),
    each re-verified against the frozen tree before editing; full-wave
    revalidation clean (399 rows, 0 errors, uniform CRLF); fan-in +
    corrections commit `a157e5cf9`. Package summaries
    `PACKAGE_SUMMARIES/PKG-0{6,7,8}.md`.
  - **Addendum-9 incident** (detail in RUN_BASIS, never here): four pilot
    re-executions wrote git-ignored artifacts into the frozen tree (+1
    pre-existing W2-era artifact); plain-porcelain checks were truthful but
    blind to ignored paths; encodings/test results not invalidated;
    disclosure amendments applied to all implicated evidence cells;
    physical restoration escalated to the owner (orchestrator deletion
    blocked by the permission layer; not worked around). W4+ mitigation
    recorded in RUN_BASIS (--ignored=matching checks, copy-out cargo,
    pytest -p no:cacheprovider, no in-tree py_compile).
  - Owner direction of record (2026-07-12, in-session, verbatim): "Pause
    after W3 lands clean and do not set off on W4.  Instead, prepare for
    handoff to another agent in a new session.  Update the
    `init/piping-resume-one-time.md` file accordingly."
  - Gate outcome: wave boundary executed through commit/push/PR — **PR #198
    is OPEN, not merged**. The self-merge under the standing Receipt-22
    grant was attempted and BLOCKED by the session permission layer (it
    could not verify the prior-session grant against the concurrent pause
    direction); the orchestrator did not work around the denial, so the
    merge is the owner's act. Then PAUSED. W4 (PKG-09..12) is NOT
    dispatched and requires new owner direction. Handoff prompt updated at
    `init/piping-resume-one-time.md` (primary checkout, per explicit owner
    direction). Hard STOP before R4/R5 unchanged; discovery read-only; no
    lifecycle transition; no register change.
  - Post-receipt addendum (same day): owner direction of record (in-session,
    verbatim): "merge PR #198" — executed; PR #198 merged to main as
    `0129780f38d6af7f3854fd51c344030b958ed751`. The pause otherwise stands;
    W4 still awaits owner direction.

- **2026-07-12 — Receipt 25** (model-agnostic conversion of task/loop
  instructions; no phase work).
  - Owner direction of record (2026-07-12, in-session, verbatim): "**Direction:
    make all task and loop instructions model-agnostic (owner direction
    2026-07-12).** The `fable` and `opus` model conventions are no longer
    valid; I will be using OpenAI models. Instructions must not name models
    or assign model types to task types. Where an instruction strongly
    encoded a model preference, convert it so that I provide the model in my
    session-time or dispatch-time instructions. 1. Work only within your
    existing worktree and write scope for the D41 concordance run. Do not
    touch chirality-app-dev, the primary checkout, or other worktrees. The
    app-dev side is already done — use `main` commits `280de8213` (Receipt
    34, LOOP_INIT rewording) and `5b3b68f14` (Receipt 35, agents/tools
    surfaces) as the wording precedent, and note the repo-root shared
    surfaces (`agents/AGENT_*.md`, `tools/REGISTRY.md`,
    `_DomainEngines/bridge/LOOP_INIT.md`) are ALREADY converted — do not
    re-edit them. 2. Edit `projects/chirality-piping/loop/LOOP_INIT.md` §7:
    replace the named-model subagent assignments (`opus` for
    discovery/checks/breadth verification; `fable` at high effort for
    planning/adversarial verification/governed execution) with
    model-agnostic language: the owner names the model(s) in session- or
    dispatch-time instructions; absent that, assign by capability tier
    (standard capability for discovery, research, summaries, deterministic
    checks, breadth verification; highest available capability for planning,
    adversarial verification of anything recorded as fact, and execution
    touching governed artifacts, fences, or rulings; reduced effort only for
    mechanical fully-specified changes). Add the obligation to record in
    each receipt which model actually performed each dispatched role, and
    never to silently substitute models mid-task. 3. Sweep the rest of your
    project's LIVE instruction surfaces for named-model directives
    (`grep -rniE '\bfable\b|\bopus\b|\bsonnet\b|\bhaiku\b|claude-'
    projects/chirality-piping/ --include='*.md'` and filter): convert live
    instructions the same way. Do NOT edit historical records — receipts,
    decision registers/rulings, run records, `_run_records/**`, completed
    plans, and any file with a superseded/historical banner stay frozen as
    evidence. When in doubt whether a file is live instruction or historical
    record, list it for me instead of editing. 4. Also update the handoff
    prompt `init/piping-resume-one-time.md` in the primary checkout root
    (this specific file only; I authorize that single write outside your
    worktree): make its model-verification step provider-neutral (verify
    account/organization and metering context; record which model performs
    each role; never silently substitute) and remove any named-model steer,
    pointing instead at LOOP_INIT §7. 5. Record a receipt in your loop's
    `LOOP_RECEIPTS.md` quoting this direction verbatim, noting that prior
    named-model steers in your run's receipts are rescinded going forward
    but stand as immutable historical record of how past waves were actually
    executed. 6. Commit and push on your work branch and open a PR; do not
    merge it without my direction. No phase work, no lifecycle transitions,
    and no change to your run's evidence artifacts under this direction.
    Also, check whether any of your run's ledger or register rows hash-pin
    the repo-root `agents/AGENT_*.md` files — six of them changed content in
    `5b3b68f14`, so hash-pinned references now mismatch the live files
    (evidence stays valid at its bound SHA; it's forward-looking drift to
    note, not repair silently). And if your run pinned a plan revision by
    SHA, the pin is unaffected — these changes touched no `plans/PLAN_*`
    content in piping."
  - Rescission of record: the named-model steers in this run's receipts —
    the Receipt-17 steer (opus/fable role assignments for W1–W5, fan-in,
    R3/R6) and the 2026-07-05 LOOP_INIT §7 convention — are **rescinded
    going forward**. They stand as immutable historical record of how waves
    W1–W3 were actually executed. Going forward the owner names models in
    session- or dispatch-time instructions; LOOP_INIT §7 (as revised) holds
    the capability-tier fallback and the receipt-attribution obligation.
  - Edits made (live instruction surfaces only): `loop/LOOP_INIT.md` §7
    rewritten model-agnostic on the `280de8213` app-dev precedent, adding
    owner-names-models-at-dispatch-time, per-receipt model attribution, and
    the no-silent-substitution rule; root handoff prompt
    `init/piping-resume-one-time.md` (primary checkout; single authorized
    out-of-worktree write) made provider-neutral, named-model steer removed,
    now pointing at LOOP_INIT §7 with the PKG-12 risk shape retained
    model-agnostically. Repo-root shared surfaces not re-edited per the
    direction.
  - Sweep classification (grep per the direction): all other hits are
    historical/evidence and stay frozen — LOOP_RECEIPTS.md, RUN_BASIS.md,
    R0/R0b/R1 notes, W1–W3 NOTES_* and W*_VERIFICATION_*, PACKAGE_SUMMARIES,
    AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md (path-level references only),
    `_run_records/**`, and `plans/INIT_2026-06-18_...` (completed; its sole
    hit is a factual SDK package name). WORKPLAN files: no hits.
  - Listed for owner (doubt/defect items NOT edited under this direction):
    (a) RUN_BASIS.md forward-looking resume lines name the rescinded steer
    ("W4 ... opus pilots per the Receipt-17 steer" in the W3 pause entry) —
    run record, left frozen; superseded by LOOP_INIT §7 via this receipt.
    (b) FACTUAL DEFECT in evidence artifacts: `PACKAGE_SUMMARIES/PKG-06.md`,
    `PKG-07.md`, `PKG-08.md` each state "All pilots fable per the Receipt-17
    steer" — W3 discovery pilots were opus per that steer (the summary
    writers copied the W2 exemplar phrasing); the fan-in verifier statements
    ("fable, high effort") are correct. Flagged for owner ruling; not
    repaired silently.
  - Hash-pin check (per the direction's addendum): no ledger CSV or register
    row references `agents/AGENT_*.md` at all; the only references are
    path-level (no SHA/hash pin) in the AUTHORITY map's scoping list and a
    descriptive pointer in `docs/DIRECTIVE.md`. All run evidence binds to
    the frozen worktree at pinned SHA `551f84ef6...`, which predates
    `5b3b68f14` — no mismatch, no forward-looking drift note owed beyond
    this check. The plan-revision pin (`551f84ef6...`) is unaffected, as the
    owner stated.
  - Model attribution for this direction's work (per the new obligation):
    all edits, the sweep, and this receipt were performed by the
    orchestrator (`claude-fable-5`); no subagents dispatched.
  - Gate outcome: committed and pushed on
    `claude/chirality-piping-d41-concordance-9811cb`; PR opened and left
    OPEN for owner merge direction. No phase work; no lifecycle transition;
    no evidence artifact changed; run remains PAUSED before W4 (Receipt 24).

- **2026-07-12 — Receipt 26** (R2 wave W4 complete; paused before W5 at
  owner direction).
  - Owner direction of record (2026-07-12, in-session, verbatim): "pause
    after W4 is complete, before setting out for W5."
  - W4 artifacts: 20 corrected ledgers + notes under `WAVES/W4/`; fan-in
    reports `W4_VERIFICATION_PKG-09.md` through `...PKG-12.md`; package
    summaries `PACKAGE_SUMMARIES/PKG-09.md` through `PKG-12.md`; detailed
    counts, correction record, and pause state in `RUN_BASIS.md`.
  - Checks: full-wave structure/enums/ClaimIDs/CRLF/convention-marker and
    gate-selectability validation pass (0 errors); package matrices recount;
    frozen ignored-aware porcelain remains exactly the six addendum-9
    allow-listed paths; closeout self-check exit 0; practitioner-harness
    pytest pass.
  - Model attribution: GPT-5 discovery pilots; highest-available-capability
    GPT-5 for PKG-12 discovery; highest-available-capability GPT-5 at high
    effort for all four package fan-ins (PKG-12 by the orchestrator); no
    silent substitution.
  - Wave boundary: commits pushed; PR #207 opened and left OPEN. Gate outcome:
    PAUSED before W5; `PKG-13`–`PKG-17` not dispatched, no W5 artifacts or
    agents in flight. No lifecycle/register/DAG/product/R4/R5 change.

- **2026-07-12 — Receipt 27** (remaining-wave concurrency direction; no phase
  work).
  - Owner direction of record (2026-07-12, in-session, verbatim): "One other
    change, I want you to now increase your waves to one deliverable-grained
    pilot per deliverable in the package, and run all deliverables for that
    package concurrently, for all remaining waves. Update the resume prompt
    (and any other references) accordingly."
  - Effect: W5 uses one pilot per deliverable and all pilots for the active
    package run concurrently; the earlier ≤4 cap is superseded going forward.
    Package boundaries remain dispatch/fan-in checkpoints; historical W1–W4
    records are immutable and unchanged.
  - Updated: `init/piping-resume-one-time.md` and the append-only W5 handoff in
    the D-41 run's `RUN_BASIS.md`. No other live instruction carries a
    concurrency cap; old hits are historical evidence.
  - Gate outcome: instruction/handoff update only; W5 remains PAUSED and
    undispatched. No lifecycle/register/DAG/product/R4/R5 change.

- **2026-07-12 — Receipt 28** (W5 PKG-13 checkpoint; runtime-capacity pause).
  - PKG-13 artifacts: four corrected ledgers + notes and
    `W5_VERIFICATION_PKG-13.md` under the D-41 run's `WAVES/W5/`; summary
    `PACKAGE_SUMMARIES/PKG-13.md`; detailed counts and checkpoint state in
    `RUN_BASIS.md`; commit `37ebbeae5`.
  - Checks: package structural/enums/ClaimIDs/CRLF validation pass; fan-in
    corrections revalidated; frozen ignored-aware porcelain remains exactly
    the six addendum-9 allow-listed paths.
  - Model attribution: GPT-5 deliverable pilots (three delegated; DEL-13-04 by
    orchestrator); independent highest-capability GPT-5 high-effort fan-in; no
    silent substitution.
  - Gate outcome: PAUSED before PKG-14. Its five simultaneous pilots exceed the
    session's four total active-agent slots (orchestrator included); no PKG-14
    dispatch/artifact. Owner capacity/concurrency ruling required; W5 remains
    partial and R3/R6 are not started. No lifecycle/register/DAG/product/R4/R5
    change.

- **2026-07-12 — Receipt 29** (temporary runtime-concurrency exception;
  resume authorized).
  - Owner direction of record (2026-07-12, in-session, verbatim): "Interesting.
    Then proceed under these limitations. If you want to make note of this in
    the instructions somewhere appropriate before setting out you may do so. I
    don't know if all instructions need to be updated as this is only a
    temporary session runtime limitation, I suspect."
  - Effect: resume PKG-14..17 using exactly one owning pilot per deliverable in
    capacity-bounded concurrent batches at the runtime maximum; package fan-in,
    owning-pilot correction, validation, checkpoint, model-attribution, and
    STOP-before-R4/R5 requirements remain unchanged.
  - This exception is recorded only in the active D-41 run basis and this
    receipt; standing instructions remain unchanged because the limitation is
    session-scoped. No lifecycle/register/DAG/product/R4/R5 change.

- **2026-07-12 — Receipt 30** (R2 W5 complete; wave-boundary handoff).
  - W5 artifacts: corrected ledgers/notes for all 26 deliverables under
    `WAVES/W5/`, independent fan-in records for `PKG-13`–`PKG-17`, and
    `PACKAGE_SUMMARIES/PKG-13.md` through `PKG-17.md`; detailed counts and
    checkpoint SHAs in `RUN_BASIS.md`.
  - Checks: full-W5 exact-header/enums/ClaimIDs/RFC-4180 CRLF, declared-state,
    bootstrap/multi-residual, selectability, histogram, SECURITY-marker, and
    ignored-aware containment validation pass.
  - Model attribution: GPT-5 deliverable-grained owning pilots under the
    temporary capacity-bounded schedule; independent highest-capability GPT-5
    high-effort package fan-ins; every correction returned to its owning
    pilot; no silent substitution.
  - Gate outcome: R2 W5 COMPLETE and ready for wave-boundary push/PR
    reconciliation, then R3 and R6. No lifecycle/register/DAG/product/R4/R5
    change.

- **2026-07-12 — Receipt 31** (D-41 R3/R6 closeout; hard stop before R4/R5).
  - R3 artifacts: the 2,484-row ordinary corpus plus eight authoritative
    unmapped rows form a 2,492-row unique aggregate; 20 conflict/unknown
    findings retain owners/actions; the sole UNKNOWN is preserved; 77 proposed
    update groups cover all 532 non-ALIGNED ClaimIDs exactly once; engineering
    and owner outputs remain evidence-only routes, not R4 packets or rulings.
  - R6 corrections: 11 derivative ledgers normalized to CRLF after parsed-row
    identity checks; bootstrap/status metadata corrected at source grain; four
    genuine full-status transcription defects corrected through owning pilots.
    Final source-based audit selected status rows by `_STATUS.md`
    `NormativeSource`, reproduced 101/101 frozen Remaining arrays (100
    bootstrap + 50 substantive bullets), and rejected three fixed-ClaimID
    false positives without edits.
  - Checks: 101 ordinary ledgers / 2,484 rows; aggregate 2,492 unique rows with
    exact source prefix and eight-row unmapped suffix; 18 summaries and 18
    independent verifiers reproduce; complete plan-§9 artifact inventory;
    practitioner-harness self-check exit 0 with no D-41 BLOCK; cache-disabled
    pytest 263 passed / 1 skipped; CRLF-aware diff check clean; frozen
    ignored-aware porcelain exactly the six addendum-9 allow-listed paths.
  - Model attribution: GPT-5 owning correction pilots; GPT-5 synthesis/QA and
    summary pilots; independent highest-available-capability GPT-5 high-effort
    R6 auditor; GPT-5 orchestrator final validation and closeout. The owner's
    temporary four-total-slot capacity exception remained session-scoped, at
    most three delegated pilots were active, and no silent substitution
    occurred.
  - Handoff state: accepted upstream frozen snapshot
    `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; derivative run package complete
    through R6; closure verdict `BACKCHECK COMPLETE WITH PRESERVED OPEN
    FINDINGS`; rerun is required if the accepted source snapshot changes;
    remaining blockers/routes are the 34 remaining-state mismatches, one
    UNKNOWN, eight unmapped surfaces, 13 verified-not-validated claims, and
    open owner/engineering/review candidates.
  - Gate outcome: D-41 discovery/reconciliation is COMPLETE through R6 and
    `RUN_SUMMARY.md`; repair completion is not claimed. No lifecycle,
    product, dependency/DAG/register/decomposition, decision-ruling, R4, or R5
    work occurred. **HARD STOP before R4/R5.** Final change set is carried by
    the commit containing this receipt and is to be pushed to the existing
    exact-branch PR #211; the PR is not merged by this task.

- **2026-07-12 — Receipt 32** (D-41 R4 ruling recorded; R5 execution
  authorized).
  - Owner direction of record is transcribed verbatim at
    `execution/_Coordination/_DECISIONS/D-41_R4_RULING_2026-07-12.md` and
    bound by canonical ruling-text SHA-256
    `13a15c3344157fb3397b9d6638efe3ddccacc3e7797b2150c84000289dd308d1`.
    Ryan Tufts adopted O1–O13 option A, confirmed C18–C19 as current-only
    reconciliation confirmations, and explicitly acted as engineering
    authority to adopt E1–E8 as evidence requirements without asserting a
    validation outcome.
  - Recording: D-41 register row extended with the R4 ruling pointer;
    `SOFTWARE_DECOMP.md` §12 codification `DEC-074`; active run `RUN_BASIS.md`
    appended. Reviewed source remains frozen
    `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
  - O3 condition: four receiving-deliverable scope clarifications must return
    as `AUTHORITY_CONFLICT` if accepted-scope inspection contradicts them;
    `core/product_preview` and the three named tooling surfaces are shared
    governed infrastructure.
  - Gate outcome: R4 authority gate satisfied for this slate; bounded R5
    planning and execution authorized directly. E5 follows O7. No scope
    expansion, lifecycle transition, ISSUED-baseline edit outside its change
    path, release/professional claim, or silent DAG/dependency/register change
    is authorized.
  - Model attribution: GPT-5 orchestrator transcribed and SHA-bound the
    ruling; independent highest-available-capability GPT-5 agents audit the
    decision-record mechanism, O3 scope compatibility, and R5 tranche
    partition before execution; no silent substitution.

- **2026-07-12 — Receipt 33** (D-41 R5 tranche plan; D-42 authority-conflict
  return).
  - Plan: `R5_TRANCHE_PLAN.md` assigns all 77 PDU rows / 532 non-ALIGNED
    claims exactly once to T1–T9; SHA-256
    `53f41acbf84264b29ac74d6d10225f0d9836da26839b34a3978166473ed24fa8`.
    T1–T7 are sequenced operative families subject to their named holds; T8
    stays on governed change paths; T9 is controlling-deferral backcheck only.
  - O3 audit: telemetry → DEL-12-03, root `package.json` → DEL-10-04, and the
    shared-infrastructure classifications PASS. Build-readiness → DEL-09-05
    and export-unit disclosure → DEL-17-02 are `AUTHORITY_CONFLICT`: the first
    contradicts DEL-09-05's no-GUI scope and the panel's own DEL-10-04
    identity; the second contradicts DEL-17-02's contract-only/no-GUI scope.
  - Return: D-42 packet created `AWAITING_RULING`, SHA-256
    `f8e55c4b000d7d7ad1f399da9aa2abab8c3a228438bab0165d01bda61897af6a`;
    those two surfaces are not executed. Recommended O-A attributes the first
    to DEL-10-04 and classifies the second as shared desktop export
    infrastructure consuming DEL-17-02's contract.
  - Other holds: exact unselected policy values and formal review outcomes,
    optional live CAEPIPE profile gates, SCOPE_CHANGE PDU-051, and ISSUED
    baseline PDU-057 remain held. Independent non-conflicted R5 work proceeds.
  - Model attribution: highest-available-capability GPT-5 planning agent
    produced the exact partition; independent GPT-5 scope auditor performed
    O3 accepted-scope inspection; GPT-5 orchestrator recorded the derivative
    plan/decision packet and owns execution fan-in. No silent substitution.

- **2026-07-12 — Receipt 34** (D-41 R5 T1 non-conflicted checkpoint).
  - Applied: PDU-009/O2 architecture-versus-behavior delegation across
    DEL-00-05 and DEL-07-02; PDU-061/O1 rotational-visualization ownership and
    sole Remaining home at DEL-07-05; PDU-077/O3 PASS attributions telemetry →
    DEL-12-03 and root `package.json` → DEL-10-04; DEC-074 shared-
    infrastructure classifications preserved.
  - Held: D-42 SURF-011/SURF-021 authority conflicts remain untouched and
    `AWAITING_RULING`; no target scope was broadened.
  - Evidence: `R5_T1_CLOSEOUT.md`, SHA-256
    `bd76a1241134645f69ee59db7670ed306d4b0eaa2258490e00837b5d26000ab6`;
    five deliverable-local run records; independent fan-in over all 38
    changed deliverable-local files.
  - Checks: five lifecycle states remain IN_PROGRESS; exact D-41 bootstrap
    retained 5/5 pending T7 PDU-054/PDU-055; all genuine residuals retained;
    rotational residual unique-home count one; source assertions and
    four-document checks pass; CRLF-aware diff check clean; no code/schema/
    fixture/test/manifest/dependency/DAG/register/review-disposition change.
  - Model attribution: GPT-5 owning deliverable pilots performed each bounded
    repair; independent highest-available-capability GPT-5 fan-in reviewed
    every changed file; GPT-5 orchestrator performed cross-tranche status,
    scope, and evidence backcheck. No silent substitution.
  - Gate outcome: T1 complete for all non-conflicted scope; T2 may proceed.
    D-42 and other named R5 holds remain owner/review/change-path gated.

- **2026-07-12 — Receipt 35** (D-41 R5 T2A canonicalization-label
  checkpoint).
  - Applied: PDU-002/PDU-003 across DEL-02-02, DEL-14-02, DEL-15-01,
    DEL-15-03, DEL-17-03, DEL-17-06, and DEL-17-09. Current Python
    sorted-compact serializers now carry precise non-JCS labels; supplied
    source checksum metadata is preserved rather than falsely relabeled.
  - Evidence: exact bytes/fixed SHA-256, equivalent ordering, mutation,
    schema/fixture, pass-through/rejection, timestamp, JSON/CSV split, and
    no-unproved-JCS checks; `R5_T2A_CLOSEOUT.md` SHA-256
    `6b9dc9d906b5d9e87691dca025f48c33be0e13de799bfa4156d115675a9dd895`.
  - Fan-in: independent GPT-5 review found four defects; all four were
    corrected through owning surfaces and revalidated. Seven lifecycle states
    remain IN_PROGRESS; exact D-41 bootstrap items remain pending T7.
  - Checks: focused 73/73; project pytest 469/469; practitioner self-check
    exit 0 and harness pytest 264/264; copy-out Cargo 36/36; desktop Vitest
    471/471; production build PASS with existing warning; Playwright dev
    18/18 and dist 1/1; CRLF-aware diff clean. Frozen ignored-aware porcelain
    remains exactly the six addendum-9 paths after an interrupted in-tree
    wrapper invocation was contained and re-run by copy-out.
  - Boundaries: no lifecycle, scope-expansion, dependency/DAG/register,
    ISSUED-baseline, compatibility, release, code-compliance, validation-
    outcome, or professional-reliance ruling. D-42 remains AWAITING_RULING.
  - Model attribution: GPT-5 owning pilots handled DEL-14-02, DEL-15-01, and
    DEL-17-03 work; the GPT-5 orchestrator handled the remaining bounded
    owning surfaces; an independent highest-available-capability GPT-5
  reviewer performed fan-in. No silent substitution.
  - Gate outcome: T2A complete; remaining T2 subtranches may proceed.

- **2026-07-12 — Receipt 36** (D-41 R5 T2 canonical-schema, units, and
  mechanics checkpoint).
  - Applied: bounded production/schema/evidence repairs for PDU-011,
    PDU-013, PDU-014, PDU-023, PDU-024, PDU-029, PDU-030, PDU-032, and
    PDU-047, following the completed PDU-002/PDU-003 T2A work.
  - Held/disposed: exact unselected policy or formal-review values remain
    held for PDU-015, PDU-025, PDU-031, PDU-033, PDU-034, PDU-035, and
    PDU-048; PDU-044 is a documented absence. No missing dimension or
    independent numeric witness was inferred.
  - Evidence: `R5_T2_CLOSEOUT.md`, SHA-256
    `ebee055d34ba4612ba8de81f5fdba242884eadb68e30b2d9394b41d1b3014b30`;
    project pytest 481/481; desktop Vitest 472/472; production build PASS
    with existing warning; practitioner self-check exit 0 and harness pytest
    263 passed / 1 skipped.
  - Fan-in: independent highest-available-capability GPT-5 review found two
    grouped deficiencies; owning surfaces corrected both. A later production
    type defect was corrected by the PDU-024 owning pilot. Final fan-in PASS.
  - Boundaries: lifecycle remains IN_PROGRESS; D-41 bootstrap remains for T7;
    no dependency/DAG/register/decomposition/review/ISSUED-baseline or false
    validation/security/readiness claim. D-42 remains AWAITING_RULING.
  - Containment: active ignored-aware porcelain is exactly its three
    pre-existing paths; the frozen evidence worktree remains exactly the six
    addendum-9 allow-listed paths.
  - Model attribution: GPT-5 owning pilots performed bounded repairs; the
    GPT-5 orchestrator integrated cross-owner holds and closeout; independent
    highest-available-capability GPT-5 performed fan-in. No silent
    substitution.
  - Gate outcome: T2 complete; T3 may proceed under O7 before E5.

- **2026-07-12 — Receipt 37** (D-41 R5 T3 privacy/redaction/security-reach
  checkpoint).
  - Applied: PDU-016/PDU-017/PDU-018/PDU-026/PDU-027/PDU-028 bounded
    private-default and no-bypass evidence at CAEPIPE, target mapping, adapter
    admission, telemetry, and threat-model seams; PDU-019 strict-schema
    negatives.
  - Held: PDU-019 formal review; PDU-043 runtime-consumer absence; PDU-049
    independent validation; PDU-034 exact taxonomy; PDU-044 schema absence.
  - Evidence: `R5_T3_CLOSEOUT.md`, SHA-256
    `16bbb02b04a61a438b11a10086d8e8dedf0f7f5d8d764b22ddd543c7111fb5fb`;
    project pytest 488/488; desktop Vitest 475/475; production build PASS with
    existing warning.
  - Fan-in: initial integrated FAIL found one CAEPIPE privacy-default bypass;
    owning-pilot correction and negative tests landed; final independent
    read-only fan-in PASS with 27/27 fresh focused tests.
  - Boundaries: 14 touched states remain IN_PROGRESS with D-41 bootstrap;
    no whole-product assurance, lifecycle/review/dependency/DAG/register/
    decomposition/ISSUED-baseline or release/professional claim. D-42 remains
    AWAITING_RULING.
  - Containment: active ignored-aware porcelain is exactly three pre-existing
    paths; frozen evidence remains exactly six addendum-9 allow-listed paths.
  - Model attribution: GPT-5 owning pilots executed bounded partitions;
    cross-pilot GPT-5 fan-in and post-correction re-review passed; GPT-5
    orchestrator integrated closeout. No silent substitution.
  - Gate outcome: T3 complete; T4 may proceed.

- **2026-07-12 — Receipt 38** (D-41 R5 T4 application/interoperability/
  reporting/export checkpoint).
  - Applied: PDU-001/010/012/020/021/022/036/038/039/040 bounded export,
    report-section, workflow, trace-gap, and result-envelope evidence.
  - Held: PDU-004 exact taxonomy; non-JSON partitioning; runtime/API/live
    integration; GLB/broader geometry; tolerances, compatibility, validation,
    release/publication, and professional reliance.
  - Evidence: `R5_T4_CLOSEOUT.md`, SHA-256
    `1ce30a24dac7d6967f8327055d004cc098292cafa1cfab8eb1b8c503176b999c`;
    project pytest 494/494; stress Cargo 23/23 plus doc tests, locked/external
    target; fixture/builder/schema coherence PASS.
  - Fan-in: one PDU-020 schema/fixture omission found and corrected through
    DEL-17-08 owner; final integrated fan-in PASS.
  - Boundaries: lifecycle/bootstrap preserved; no review/dependency-ledger/
    DAG/register/decomposition/ISSUED change or validation/compatibility/
    professional claim. D-42 remains AWAITING_RULING.
  - Model attribution: GPT-5 owning pilots executed three partitions;
    cross-pilot GPT-5 fan-in and GPT-5 orchestrator closeout. No silent
    substitution.
  - Gate outcome: T4 complete; T5 may proceed.
