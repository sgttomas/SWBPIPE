# Notes — CLAIM_CONCORDANCE_DEL-04-05 (R2 wave W2)

Deliverable: DEL-04-05 "Sparse solver performance harness" (PKG-04).
Discovery basis: FROZEN worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
only. Ledger encoded under `R1_CONVENTIONS.md` (conventions 1–8 + addenda
1–13) plus the W1 calibration items in `PACKAGE_SUMMARIES/PKG-0{0..3}.md`.

Run-level `NormativeSource` path alias (addendum 12): unqualified file names
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REVIEW.md`, `Review_Findings.csv`,
`_run_records/...`) resolve under
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-05_Sparse solver performance harness/`;
all other paths are repo-`projects/chirality-piping`-relative.

## 1. Histograms (recounted from the CSV before writing)

Row count: 18.

Disposition histogram:

| Disposition | count |
|---|---|
| ALIGNED | 12 |
| PARTIALLY_IMPLEMENTED | 1 |
| ACCEPTED_DIVERGENCE | 1 |
| STALE_SETUP_SPECIFICATION | 4 |

ClaimType histogram:

| ClaimType | count |
|---|---|
| REQUIREMENT | 7 |
| ACCEPTANCE | 0 |
| EXCLUSION | 2 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 2 |

Requirement IDs: the kit's native scheme is `DEL-04-05-RQ-001..007` (RQ
form, recorded in-row in `NormativeSource`); ClaimIDs use the addendum-12
form (`DEL-04-05-REQ-001..007`).

Mechanical selectability (convention 6): 0 YES rows. Both non-bootstrap
residuals are gated — REM-001 `stage-gated: R5 release`, REM-002
`gated: D-05b conditions per DEC-059` — so the `_STATUS.md` surface row
(DECL-005) derives NO under convention 3 (YES iff any listed residual is
ungated), and every other row has no recorded item (addendum-12 default NO).
This matches the R1 `DELIVERABLE_INVENTORY.csv` grain (Selectable=NO),
re-derived here mechanically from the frozen `_STATUS.md`. The owner
suspension is a run-level caveat (RUN_BASIS), never per-row. Gate-state cells
reflect the frozen register state (D-41 `AWAITING_RULING`) per the RUN_BASIS
codification.

## 2. Self-flagged rows

- DEL-04-05-REQ-003 — class call: encoded GOVERNANCE (threshold-deferral
  discipline) rather than REPORTING/MECHANICS; the claim's substance is
  "record without inventing release thresholds", which I read as governance
  about thresholds, with metric recording itself covered by REQ-005. The
  bounded DEC-050/DEC-053 generated-grid policies were judged NOT to defeat
  the claim because they are human-ruled governed records explicitly scoped
  below release grade — reviewer eyes welcome on both calls.
- DEL-04-05-REQ-006 — mirrored the DEL-04-04-REQ-006 pattern:
  PARTIALLY_IMPLEMENTED at implementation-evidence grain (unit metadata is
  metadata-only; dimensional checking beyond metadata binding open), MEDIUM
  (addendum-13 cap; the finding PKG04-DEL0405-PKG02-001 is
  TECHNICALLY_ADDRESSED_PENDING_HUMAN with HumanDisposition TBD), and
  `AuthorityNeeded=REVIEW` rather than addendum-13's literal OWNER — kept for
  intra-wave consistency with DEL-04-04; W1 already noted this variance for
  R3 dedupe.
- DEL-04-05-EXC-001 — judgment call the conventions leave open: the
  Specification's setup-pass exclusion sentence ("does not implement a
  benchmark runner, add tests, choose thresholds, edit solver code, select a
  numerical library") is overtaken as a description of the current
  deliverable scope; I encoded ACCEPTED_DIVERGENCE because each departure is
  permitted by a named recorded human authorization or ruling (MEMORY.md
  2026-05-01 bounded-implementation authorization; DEC-023; DEC-050/DEC-053),
  per addendum 11. Convention 1 forbids STALE_SETUP_SPECIFICATION on
  exclusion rows, so the substance disposition had to carry it. A reviewer
  could instead read the exclusion as historically-scoped-and-accurate
  (ALIGNED). Confidence MEDIUM.
- DEL-04-05-EXC-002 census: the proprietary-data clause was split out of the
  same Scope sentence as a separate durable exclusion (ALIGNED); splitting
  grain is a judgment call.
- DEL-04-05-DECL-001/DECL-002/DECL-003 — `AuthorityNeeded=OWNER` per W1
  calibration item 1's trigger: each surface carries an overtaken TBD
  register (Specification Documentation TBD list; Datasheet Construction +
  Open Setup Questions; Guidance Open Enrichment Items), not just pointer
  drift or future-tense prose.
- DEL-04-05-DECL-004 — Procedure encoded STALE_SETUP_SPECIFICATION (unlike
  DEL-04-04's Procedure, which was kept ALIGNED): this Procedure declares a
  "current setup-only boundary" as present-tense current state and carries an
  overtaken "current value is `TBD`" threshold line, and received no
  post-implementation refresh. `AuthorityNeeded=NO` because a single
  overtaken TBD line is not an "overtaken TBD register" (DEL-04-04 DECL-002
  precedent). Both halves of that call are reviewer-checkable.
- DEL-04-05-DECL-007 — census judgment: included
  `core/solver/performance_harness/README.md` as a deliverable-owned in-tree
  README (it self-identifies: "This crate is the bounded implementation slice
  for `DEL-04-05`") even though the crate surface (SURF-136) maps to multiple
  deliverables. W1/W2 precedent exists (DEL-04-04 DECL-007).
- DEL-04-05-REM-002 — `AuthorityNeeded=NO` on a gated residual: the gate is
  the already-ruled DEC-059 conditional-activation prerequisites, so no new
  authority is needed for the record or its unlock; contrast REM-001's
  ENGINEERING routing (numeric threshold promotion, W1 calibration item 8).
  If AuthorityNeeded is meant to name the authority that will eventually
  accept the hosted evidence, this could instead be OWNER.
- ACCEPTANCE census = 0 — the Specification's Verification table is
  "minimum setup expectations" that restate RQ-002/003/004/007 at
  setup-expectation grain; under the addendum-12 grain rule (verification
  tables that merely restate requirements do NOT get mirrored ACCEPTANCE
  rows) I encoded no ACC rows. DEL-04-04 had one ACC row only because its
  verification section carried substance beyond the requirement table
  (DEC-067 transition witnesses); no analogous extra substance exists here.

- 2026-07-12 post-verification string correction (W2_VERIFICATION_PKG-04.md
  §3.1, verifier-recommended; applied by the owning pilot) —
  DEL-04-05-REQ-006 VerificationEvidence: the citation
  `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`
  was transcribed from the kit MEMORY.md 2026-05-16 finding-resolution
  addendum but the directory does not exist anywhere in the frozen tree
  (absence re-verified by this pilot by full-tree find before editing). Marked
  per convention 7: `(ATTESTED: kit MEMORY.md 2026-05-16 DEV-001 PKG-02
  grounded finding-resolution addendum, record not present in tree at frozen
  SHA 551f84ef6)` appended inside the cell. No other cell, row, or disposition
  changed; the substantive Review_Findings.csv citation on the row resolves
  in-tree and the disposition never rested on the matrix. Histogram recount:
  the edit touched only VerificationEvidence text — no disposition,
  ClaimClass, ClaimType, Confidence, or AuthorityNeeded value changed, so the
  section-1 histograms are unaffected and stand as recounted.

## 3. Evidence-execution log

Re-executed side-effect-free at frozen SHA 551f84ef6 (artifacts redirected
outside the tree; `git -C FROZEN status --porcelain` empty before and after
every run):

- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
  tests/test_sparse_default_promotion_observation.py
  tests/test_sparse_suitability_observation.py -q -p no:cacheprovider` —
  PASS, 2 tests.

NOT re-executed (cited as recorded):

- `cargo test` for `core/solver/performance_harness` (20 static tests): I
  attempted `CARGO_TARGET_DIR=<scratch> cargo test --manifest-path
  core/solver/performance_harness/Cargo.toml --locked` in the frozen tree and
  it refused ("the lock file ... needs to be updated but --locked was
  passed"); porcelain remained clean after the refusal. Running unlocked
  would write `Cargo.lock` inside the frozen tree — not side-effect-free, so
  it was not run (addendum 9). Cited instead: DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (cargo_crate_sweep=pass) at commit `e648462f1`, marker `not re-executed at
  frozen SHA 551f84ef6`, with the addendum-10 qualifier backed by diffs I
  actually ran: `git diff e648462f1d05...551f84ef6` is empty over
  `core/solver/performance_harness/`, `core/solver/sparse_direct/`,
  `validation/benchmarks/`, `tests/test_sparse_default_promotion_observation.py`,
  and `tests/test_sparse_suitability_observation.py` (also empty over all of
  `core/`); the only differing path under `validation/` is the sweep-record
  JSON itself (exclusion clause carried in-row).
- `_REVIEW.md` recorded reviews (PKG-02 audit 2026-05-16 WARNING verdict;
  lifecycle-readiness PB-001/TB-001/AC-001, 2026-06-05) and the
  `DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16` resolution matrix:
  recorded, `not re-executed at frozen SHA 551f84ef6`.

DecisionBasis resolution checks (convention 7): D-03/D-17/D-26/D-05b/D-41
records, DEC-023/DEC-050/DEC-053/DEC-059 codifications
(`SOFTWARE_DECOMP.md` §12), both LifecycleCorrection `Decision_Log.md`
folders, `plans/PLAN_2026-06-17_prd_completion.md` §3 D7/D-17 rows, and
`Review_Findings.csv` all resolve in the frozen tree. The MEMORY.md
2026-05-01 authorization used on EXC-001 is an in-tree recorded ruling note
(deliverable MEMORY), not an external attestation. No ATTESTED-level
citations were needed in this ledger.

Residual re-verification (dispatch instruction): both non-bootstrap
residuals were re-verified against the frozen `_STATUS.md` and are current —
REM-001's release/external threshold axes are named observational-only in
the DEC-053 codification and MEMORY Open TBDs; REM-002's hosted-CI gate is
confirmed inactive by the DEC-059 codification ("No CI is activated..."). No
landed-but-still-recorded item, no omitted evidence-backed residual found
(the crate's remaining follow-ons — e.g. future sparse-adapter integration —
are recorded in MEMORY Open TBDs as downstream work, not omitted residuals
of this deliverable's `## Remaining`).

## 4. Convention friction notes

1. **Kit `RQ` vs addendum-12 `REQ` ClaimID TYPE**: the kit's native
   requirement IDs are `DEL-04-05-RQ-00N`; addendum 12 fixes ClaimID TYPE
   tokens to REQ/ACC/EXC/DECL/REM/UNMAP. I used `REQ` in ClaimIDs and
   recorded the native RQ form in `NormativeSource` (same pattern as the
   DEL-04-04 2-digit/3-digit handling).
2. **Setup-era exclusions overtaken by authorized implementation**:
   convention 1 bars staleness dispositions on exclusion rows, but this kit's
   exclusions are themselves setup-pass-scoped prose. ACCEPTED_DIVERGENCE
   (EXC-001) was the best fit; the convention set has no disposition for
   "exclusion text overtaken by permitted scope evolution" as such.
3. **Cargo `--locked` refusal in the frozen tree** (predicted by the
   DEL-04-04 notes): confirmed for this crate too; the workspace-level sweep
   record plus addendum-10 diff is the workable evidence path for PKG-04/05
   crate tests.
4. **Bootstrap item on a multi-residual `_STATUS.md` row**: as in DEL-04-04,
   all three `## Remaining` items were transcribed byte-exact in
   `RecordedRemaining` (the cell is a verbatim copy surface), while
   gate/source cells list entries only for the two non-bootstrap residuals —
   no annotated bootstrap variant (W1 calibration item 5).

**Owner-calibration caveat (stated once per W1 calibration item 1):** the
rev-0.7 decomp authority-pointer drift on DECL-002 follows the W1 fan-in
adjudication (STALE side under addendum 4, drift facts in-row, decomp §13
sanction as immateriality context), with OWNER routing because the kit also
carries overtaken TBD registers. If the owner later calibrates the
corpus-wide pointer-drift pattern to ALIGNED-with-note, the affected rows
flip without touching their drift facts.

## 5. Boundary compliance

- All discovery reads were from the FROZEN worktree at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; `git status --porcelain` on the
  frozen worktree was empty before discovery, after the cargo `--locked`
  refusal, after the pytest re-execution, and after ledger writing.
- Writes were confined to exactly two files:
  `RUN/WAVES/W2/CLAIM_CONCORDANCE_DEL-04-05.csv` and
  `RUN/WAVES/W2/NOTES_DEL-04-05.md` (CSV generated by a scratchpad script
  named `build_csv_DEL-04-05.py` per W1 calibration item 7; script and the
  redirected `CARGO_TARGET_DIR` live outside the repo).
- No lifecycle transition, DAG mutation, register edit, or product-file
  edit. No release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim is made by any row (rows
  quoting recorded "R5 release"/"release thresholds"/compliance-exclusion
  language quote the record only and say so in-row). All dispositions are
  agent judgments, not owner or engineering rulings; authority is routed via
  `AuthorityNeeded`.
- No agent-workflow implications encountered; no `DEFERRED_AGENT_WORKFLOW`
  rows. No `IMPLEMENTED_UNMAPPED` rows: every material surface in this
  deliverable's orbit (SURF-136 performance_harness, SURF-137 evidence
  generator, SURF-138 sparse_direct, shared solver/loads crates) carries
  deliverable attribution in `IMPLEMENTATION_SURFACES.csv`.
