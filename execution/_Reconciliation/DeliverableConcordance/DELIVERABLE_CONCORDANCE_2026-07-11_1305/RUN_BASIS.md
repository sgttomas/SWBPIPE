# Run Basis — DELIVERABLE_CONCORDANCE_2026-07-11_1305

> Immutable run record (append-only). Evidence artifact, never a queue or
> selection surface (plan §9).

## Authorization

- **Activation:** D-41, RULED 2026-07-11 (owner in-session decision slate);
  ruling record `execution/_Coordination/_DECISIONS/D-41_RULING_2026-07-11.md`;
  codification `DEC-073` (`execution/_Decomposition/SOFTWARE_DECOMP.md` §12);
  packet `execution/_Coordination/_DECISIONS/D-41_concordance_activation.md`.
- **HARD RULE satisfied:** ruling tranche merged to `main` before this
  dispatch — PR #179, merge commit `e3998349ba3ac20622c93b8091e8a1889d9dfcec`
  (2026-07-11).
- **Activated scope:** whole-corpus (O-A) — 101 deliverables, `PKG-00`–`PKG-17`.
- **Start gate:** owner suspension declaration of record (2026-07-11,
  reaffirmed in-session 2026-07-11): "I will suspend work in Chirality Piping
  for the time being." — bound to frozen-tree SHA below.

## Source-state binding

- **Frozen source tree (all discovery reads):** `main` @
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7` (merge of PR #177, 2026-07-11),
  materialized as a detached read-only git worktree
  (`.claude-worktrees/piping-frozen-551f84ef6`; machine-local path lawful in
  run records per SPEC §0.2.4).
- **Pinned method revision:** `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md`
  §§6–8 at the same SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Shared
  method context: repo-root `docs/DELIVERABLE_CONCORDANCE_METHOD.md`
  (ratified 2026-07-11).
- **Frozen-tree note:** the frozen tree predates the on-ruling flip commit;
  its 100 `_STATUS.md` `## Remaining` sections carry the seeded
  `(gated: D-41)` bootstrap item. That item is program mechanics, not a
  deliverable residual claim: ledger rows record it verbatim where
  `RecordedRemaining` is copied but exclude it from residual-work analysis.
- Any material source change affecting a wave marks its rows `STALE_INPUT`
  and forces a rerun (plan §4).

## Pre-dispatch source-state control (plan §4 items 3–7, executed 2026-07-11)

- `REPO_ROOT` derived from checkout; `WORKING_ROOT` =
  `projects/chirality-piping`.
- **Census (re-enumerated from the frozen filesystem via
  `tools/coordination/list_deliverable_status.py --dag DAG-007`):** 101
  deliverables across `PKG-00`–`PKG-17`; status counts `IN_PROGRESS=100`,
  `ISSUED=1` (`DEL-01-01`). Matches the packet §1 census.
- **Live DAG pointer re-verified at the frozen tree:**
  `execution/_DAG/_LATEST.md` → `DAG-007`
  (`approved_active_graph_authority`, created 2026-06-16; approval record
  `execution/_DAG/DAG-007/APPROVAL_RECORD.md`). `DAG-007` and lock-review
  artifacts remain provenance baselines, re-verified not trusted.
- **Overlapping work:** piping work is suspended per the declaration of
  record; working-tree git state clean at dispatch. Concurrent loops in the
  monorepo (app-dev, pec, `_DomainEngines`) are outside this project's scope
  and write fences.
- **Register/decomposition surfaces verified as intended inputs:**
  `execution/_Coordination/_DECISIONS/_REGISTER.md` (D-41 RULED),
  `execution/_Decomposition/SOFTWARE_DECOMP.md` v0.8 (`DEC-073` present on
  `main`; frozen tree carries through `DEC-072` — expected, per the
  frozen-tree note above).

## Phase log (append-only)

- **2026-07-11 — R0 dispatched.** Calibration pilots per plan §8 R0:
  `DEL-04-01` (mature solver kernel vs setup-era documents),
  `DEL-10-05` (bounded implemented CLI slice vs deferred scope),
  `DEL-12-02` (current `## Remaining` vs stale prose). Claim-level ledgers
  per the §6 contract, dispositions per §7; independent reviewer pass over
  authority precedence, technical-evidence boundaries, disposition
  consistency, and false positives. R0 changes only run artifacts. Findings
  report to the owner before any wave scale-out. Artifacts under
  `R0_CALIBRATION/`.

- **2026-07-11 — R0 complete.** Three pilot ledgers (85 claim rows: DEL-04-01
  35, DEL-10-05 18, DEL-12-02 32; uniform 20-column headers, RFC-4180 clean)
  plus notes, and the independent reviewer pass (`R0_CALIBRATION/R0_REVIEW.md`).
  Reviewer verdict: **READY WITH NAMED CONVENTIONS** — citation spot-checks
  28 passed / 0 failed / 2 qualified (non-resolvable DecisionBasis values,
  transcribed not fabricated); all fences held; no frozen-tree or pilot-file
  modification; read-only re-executions reproduced (DEL-12-02 pytest 11 pass;
  witness count 822 vs Receipt 9's recorded 830 confirmed as
  `STALE_REVIEW_OR_EVIDENCE`). Eight named method conventions proposed for
  R1+ (R0_REVIEW.md "Method conventions"); three-way stale-prose divergence,
  one `SelectableUnderCurrentLoop` contradiction, and column-encoding drift
  identified as the aggregation risks at 101-deliverable scale. Reviewer
  agent note: its first attempt died on a connection error after completing
  verification reads; it was resumed from its own transcript and wrote the
  review itself — no orchestrator authorship. Findings reported to the owner;
  wave scale-out awaits the owner's response per the R0-first direction.

- **2026-07-11 — R0b dispatched (owner-directed second calibration round).**
  Owner slate answer of record (2026-07-11, in-session, verbatim): "Another
  R0 round first" — test the reviewer's proposed conventions on a second
  small sample before committing the corpus. Convention set under test:
  `R0B_CALIBRATION/R0B_CONVENTIONS.md` (provisional, calibration-only, not
  adopted for R1+). Sample (contrasting with R0; none repeated):
  `DEL-05-03` (fundamental stress recovery — mechanics class,
  physical-model-program overlap), `DEL-07-05` (results viewer — GUI class,
  H4 desktop evidence posture), `DEL-09-01` (mechanics benchmark suite —
  validation-class deliverable). Same source-state binding (frozen tree
  `551f84ef6`); artifacts under `R0B_CALIBRATION/`. Run-level caveat per
  convention 6: the owner's suspension declaration (see Authorization) means
  no discovery-independent work is dispatched from selectability findings;
  `SelectableUnderCurrentLoop` cells are mechanical DAG/lifecycle/gate
  derivations only. Run-level codification per convention 8: gate-state
  cells reflect the frozen register state (D-41 `AWAITING_RULING` at
  `551f84ef6`); the run's authority is the live register's RULED row
  (`DEC-073`, PR #179 merge `e3998349b`) — ruling-after-freeze mechanics,
  not a conflict.

- **2026-07-11 — R0b complete.** Three convention-bound pilot ledgers
  (65 rows: DEL-05-03 17, DEL-07-05 23, DEL-09-01 25; uniform headers,
  RFC-4180 clean; frozen tree verified untouched including after sandboxed
  test re-executions) plus per-convention friction notes, and the reviewer
  pass `R0B_CALIBRATION/R0B_REVIEW.md`. Verdict: **SCALE-READY** — R0
  divergences 7 RESOLVED / 4 PARTIALLY / 0 NOT RESOLVED; every R0
  aggregation-killer closed; citation spot-checks 24 pass / 1 qualified /
  1 FAIL (DEL-09-01 fixture count is 21, not 22 — correction recorded in the
  review; no disposition affected); 13 convention addenda proposed
  (R0B_REVIEW.md §6) plus two named repairs (21-count correction at R1;
  SECURITY-encoding spot-check on the first SECURITY-class R1 deliverable).
  **Run-level note (per the review's recommendation): the R0 and R0b
  ledgers PREDATE the final convention set and are calibration evidence
  only — they must not enter any aggregation surface; the six calibration
  deliverables are re-encoded as ordinary members of their R1 waves under
  the adopted conventions+addenda.** Wave scale-out (R1+) and
  convention+addenda adoption await the owner's ruling.

- **2026-07-11 — Conventions adopted; R1 dispatched.** Owner ruling of
  record (in-session scale-out slate, verbatim): "Adopt + scale out
  (Recommended)". Binding encoding rules consolidated in
  `R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13 + named repairs +
  wave plan). R1 whole-corpus read-only inventory dispatched per plan §5
  (authority map) and §8 R1 (index set): `DELIVERABLE_INVENTORY.csv`,
  `IMPLEMENTATION_SURFACES.csv`, `VERIFICATION_INDEX.csv`,
  `VALIDATION_AND_PROVENANCE_INDEX.csv`,
  `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md` — all from the frozen tree
  `551f84ef6`. R2 waves W1–W5 follow with wave-boundary reviewer
  checkpoints.

- **2026-07-11 — R1 complete.** Five parallel index builders (fable; frozen
  tree verified clean by each). Artifacts: `DELIVERABLE_INVENTORY.csv`
  (101 rows, reconciled ID-for-ID with the coordination tool; 100
  `IN_PROGRESS` + 1 `ISSUED`; 26 deliverables carry 50 non-bootstrap
  remaining items; 16 mechanically selectable per convention 6);
  `IMPLEMENTATION_SURFACES.csv` (231 surfaces; 8 with `NONE_FOUND`
  deliverable attribution — the wave unmapped shortlist);
  `VERIFICATION_INDEX.csv` (147 suites, 1,592 static tests; 13 suites with
  no recorded execution evidence, incl. 9 script-style pytest no-ops and
  the src-tauri crate outside sweep roots);
  `VALIDATION_AND_PROVENANCE_INDEX.csv` (396 evidence rows; fixture counts
  crate-asserted — DEL-09-01 = 21, confirming the Part C repair; 2
  equation-source boundary CANDIDATE flags, both sweep dirty-path captures
  only, no validation content citing `domains/piping-design`);
  `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md` + `DECISIONS_INDEX.csv`
  (12 authority families; no open `AUTHORITY_CONFLICT`; 74 decisions
  indexed; 38 reviews indexed with the pre-DEC-025 commit-binding caveat).
  Notable corpus anomalies for waves: DEL-03-03/DEL-03-05 requirement-ID
  collision (identical R01–R11 token sets); 19 deliverables with
  non-self-identifying requirement schemes; 5 desktop panels attributable
  only via `data-testid` prefixes. R2 waves W1–W5 dispatch next
  (deliverable-grained pilots under `R1_CONVENTIONS.md`; wave-boundary
  reviewer checkpoints; ledgers under `WAVES/W<N>/`).

- **2026-07-11 — PAUSED after R1 at owner direction.** Owner direction of
  record (in-session, verbatim): "Pause once R1 is done.  Record your
  progress as appropriate for this task.  And give me a prompt to resume
  the work." R1 is complete and committed; R2 has NOT started — the W1
  orchestration launch aborted on a mechanical scripting error before any
  pilot agent ran (zero agents dispatched, no wave artifacts written), so
  there is no partial-wave state. Resume point: dispatch R2 wave W1
  (`PKG-00`–`PKG-03`, 25 deliverable-grained pilots) under
  `R1_CONVENTIONS.md`, frozen tree `551f84ef6`, ledgers to `WAVES/W1/`,
  wave-boundary reviewer checkpoint, then `PACKAGE_SUMMARIES/PKG-0X.md`;
  waves W2–W5 follow per Part D. The frozen evidence worktree is
  reproducible from the SHA (`git worktree add --detach <path>
  551f84ef6be656f1603ce0acfa5e3935aa9683c7`).

- **2026-07-11 — Resumed; R2 W1 dispatched.** Resume verified against the
  durable pause record: R1 complete, R2 never started, no partial W1
  artifacts. Frozen evidence worktree recreated at the pinned SHA
  (`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; HEAD verified, porcelain
  clean). Owner steer of record for this resume (verbatim in loop
  Receipt 17): fable orchestrates; opus discovery pilots for W1/W3/W5 and
  W4-except-PKG-12; fable discovery pilots for all of W2, PKG-12 in W4, and
  PKG-01 in W1; fable-at-high-effort per-wave fan-in verification scoped to
  all self-flagged rows, all non-ALIGNED rows, plus >=2
  ALIGNED/IMPLEMENTED_UNDOCUMENTED rows per ledger; defective ledgers re-run
  by fable pilots; R3/R6 agents fable. W1 = `PKG-00`–`PKG-03`, 25
  deliverable-grained pilots under `R1_CONVENTIONS.md`, ledgers to
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-XX-XX.csv` + `NOTES_DEL-XX-XX.md`; at most
  four concurrent pilots, each sub-batch persisted and structurally
  validated before the next launches. DEL-01-01 remains ISSUED/read-only
  with `SCOPE_CHANGE` routing and no bootstrap row.

- **2026-07-11 — R2 W1 complete (PKG-00–PKG-03).** 25 deliverable-grained
  ledgers (504 claim rows; 21 opus pilots + 4 fable PKG-01 pilots per the
  Receipt-17 steer; ≤4 concurrent; every sub-batch structurally validated
  before the next launched; one pilot resumed from transcript after a
  connection drop with no artifact loss). Fan-in verification (4 fable
  verifiers at high effort, one per package; scope per the steer plus the
  Part C SECURITY spot-check): 17 SOUND / 8 DEFECTIVE; reports
  `WAVES/W1/W1_VERIFICATION_PKG-0{0..3}.md`. All 8 defective ledgers re-run
  by fable pilots (each independently re-verified the named defects; zero
  disagreements): DEL-00-02/04/05/06/07 (rev-0.7 authority-pointer drift
  adjudicated STALE-side under addendum 4, owner-calibration caveat
  recorded), DEL-02-04 (2 DECL flips + REQ-014), DEL-03-04 (overtaken Gate-C
  review prose), DEL-03-05 (convention-5 gate-column mechanical repair).
  Three SOUND ledgers took owning-pilot string corrections (DEL-02-02
  overtaken claim; DEL-03-02/DEL-03-08 addendum-10 qualifier truncation).
  Part C SECURITY spot-checks: PASS (DEL-02-04 REQ-013, DEL-02-05
  REQ-013/025, DEL-03-07 REQ-003/004) with two W2+ harmonizations named
  (em-dash marker form; OWNER routing). Full-wave structural revalidation
  clean; frozen tree porcelain empty throughout; zero
  AUTHORITY_CONFLICT/UNKNOWN/IMPLEMENTED_UNMAPPED rows; W1 mechanical
  selectability: 11 YES rows (2 DEL-01-03, 9 DEL-02-05). Package summaries:
  `PACKAGE_SUMMARIES/PKG-0{0..3}.md` (ClaimType×Disposition histograms
  computed from ledgers; cross-ledger risks for W2–W5 and R3 recorded
  there). Wave boundary: committed, pushed, PR opened; STOPPED for owner
  merge direction per the Receipt-17 protocol. W2 (PKG-04–05, all-fable
  pilots) dispatches only after the owner acts.

- **2026-07-11 — PAUSED after W1 at owner direction.** Owner direction of
  record (in-session, verbatim): "Once this wave fans-in and you have dealt
  with everything so it's safe to pause, do so.  Do not launch another
  wave." W1 was already complete at the direction's arrival (fan-in done,
  all 8 defective ledgers re-run, revalidated, package summaries written,
  PR #187 open); no agents in flight; nothing partial. Resume point:
  dispatch R2 wave W2 (`PKG-04`–`PKG-05`, 11 deliverables, ALL-fable pilots
  per the Receipt-17 steer) under `R1_CONVENTIONS.md` plus the W1
  cross-ledger calibration notes in `PACKAGE_SUMMARIES/PKG-0{0..3}.md`
  (rev-drift STALE-side encoding, SECURITY marker/routing harmonization,
  overtaken-review-prose encoding, addendum-10 qualifier exclusion clause),
  ledgers to `WAVES/W2/`, after the owner acts on PR #187. The frozen
  evidence worktree remains reproducible from the pinned SHA.

- **2026-07-11 — Resumed; R2 W2 dispatched (PKG-04–PKG-05).** Owner
  permission of record (in-session, verbatim): "This is your permission to
  resume." (following "merge the PR and then wait for permission to proceed
  further."; PR #187 merged to main as `667a679594f`). W2 = 11 deliverables
  (DEL-04-01..06, DEL-05-01..05), ALL-fable discovery pilots per the
  Receipt-17 steer (solver-mechanics engineering-adjudication risk), ≤4
  concurrent, ledgers to `WAVES/W2/`, under `R1_CONVENTIONS.md` plus the W1
  calibration items (rev-drift STALE-side encoding with owner-calibration
  caveat; SECURITY em-dash marker + OWNER routing; overtaken-review-prose
  STALE encoding with Gate record as DecisionBasis; addendum-10
  docs-exclusion discipline; byte-exact bootstrap transcription). Fable
  high-effort fan-in after wave completion; defective ledgers re-run by
  fable pilots; wave boundary = validate/summarize/commit/push/PR/receipt,
  then STOP for owner direction. Receipt 20 records the resume.

- **2026-07-11 — PAUSED mid-W2 at owner direction (batches 1–2 complete).**
  Owner direction of record (in-session, verbatim): "Pause again when Batch
  2 is in and fully accounted for.  We will continue with Batch 3 when I
  give you approval." State at pause: 8 of 11 W2 ledgers written and
  structurally validated clean (batch 1 DEL-04-01..04, 92 rows, commit
  `951e3a94b`; batch 2 DEL-04-05/06 + DEL-05-01/02, 89 rows, commit
  `4d1c96a04`); frozen worktree porcelain empty throughout; no agents in
  flight; nothing partial. The wave fan-in has not run (whole-wave pass;
  runs after batch 3). Resume point: dispatch W2 batch 3 (DEL-05-03,
  DEL-05-04, DEL-05-05; fable pilots per the Receipt-17 steer) on owner
  approval, then the fable high-effort fan-in over all 11 ledgers and the
  wave-boundary protocol. Receipt 21 records the pause.

- **2026-07-12 — Resumed; W2 batch 3 dispatched; standing self-merge grant
  recorded.** Owner direction of record (in-session, verbatim): "Proceed now
  with the immediate next step of dispatch batch 3.  You may continue
  through successive PRs in this matter by approving merger yourself, not
  waiting for human ruling until you get to R4." Effect: (a) W2 batch 3
  (DEL-05-03, DEL-05-04, DEL-05-05; fable pilots per the Receipt-17 steer)
  dispatched; (b) the owner has granted a standing authorization — scoped to
  this concordance exercise — for the orchestrator to open AND self-merge
  the wave-boundary PRs and continue through waves W2–W5, R3, and R6
  without stopping for per-PR merge direction. This supersedes, by explicit
  owner direction, the earlier per-PR "never reuse an earlier grant" pacing
  for merge approvals only; all other fences, batch validation gates, and
  the Receipt-17 steer remain unchanged. The run still STOPs before R4/R5:
  no decision gates, repairs, or lifecycle transitions without explicit
  owner ruling. Receipt 22 records the resume and grant.

## Fences (restated)

Discovery is read-only; no lifecycle transition (`LIFECYCLE_REASSESSMENT_REQUIRED`
recorded, never applied); no DAG mutation; no cross-project edit; no
agent-workflow redesign (`DEFERRED_AGENT_WORKFLOW` → observations artifact
only); no release-readiness, issuance, certification, sealing,
professional-approval, or code-compliance claim (F-PIP-1..5); agent
dispositions are never represented as owner or engineering rulings.
