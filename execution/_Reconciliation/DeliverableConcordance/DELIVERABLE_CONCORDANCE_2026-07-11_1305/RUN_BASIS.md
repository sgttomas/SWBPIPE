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

## Fences (restated)

Discovery is read-only; no lifecycle transition (`LIFECYCLE_REASSESSMENT_REQUIRED`
recorded, never applied); no DAG mutation; no cross-project edit; no
agent-workflow redesign (`DEFERRED_AGENT_WORKFLOW` → observations artifact
only); no release-readiness, issuance, certification, sealing,
professional-approval, or code-compliance claim (F-PIP-1..5); agent
dispositions are never represented as owner or engineering rulings.
