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
