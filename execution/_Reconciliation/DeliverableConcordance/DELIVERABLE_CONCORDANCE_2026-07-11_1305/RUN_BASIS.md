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

## Fences (restated)

Discovery is read-only; no lifecycle transition (`LIFECYCLE_REASSESSMENT_REQUIRED`
recorded, never applied); no DAG mutation; no cross-project edit; no
agent-workflow redesign (`DEFERRED_AGENT_WORKFLOW` → observations artifact
only); no release-readiness, issuance, certification, sealing,
professional-approval, or code-compliance claim (F-PIP-1..5); agent
dispositions are never represented as owner or engineering rulings.
