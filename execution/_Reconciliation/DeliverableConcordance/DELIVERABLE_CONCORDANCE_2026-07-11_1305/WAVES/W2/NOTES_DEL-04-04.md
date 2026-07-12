# Notes — CLAIM_CONCORDANCE_DEL-04-04 (R2 wave W2)

Deliverable: DEL-04-04 "Nonlinear support active-set solver" (PKG-04).
Discovery basis: FROZEN worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
only. Ledger encoded under `R1_CONVENTIONS.md` (conventions 1–8 + addenda
1–13) plus the W1 calibration items in `PACKAGE_SUMMARIES/PKG-0{0..3}.md`.

Run-level `NormativeSource` path alias (addendum 12): unqualified file names
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REVIEW.md`, `Review_Findings.csv`,
`_run_records/...`) resolve under
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/`;
all other paths are repo-`projects/chirality-piping`-relative.

## 1. Histograms (recounted from the CSV before writing)

Row count: 25.

Disposition histogram:

| Disposition | count |
|---|---|
| ALIGNED | 21 |
| PARTIALLY_IMPLEMENTED | 2 |
| STALE_SETUP_SPECIFICATION | 2 |

ClaimType histogram:

| ClaimType | count |
|---|---|
| REQUIREMENT | 9 |
| ACCEPTANCE | 1 |
| EXCLUSION | 4 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 4 |

Requirement IDs: the kit's native scheme is 2-digit (`DEL-04-04-REQ-01..09`,
recorded in-row in `NormativeSource`); ClaimIDs use the addendum-12 3-digit
form (`DEL-04-04-REQ-001..009`).

Mechanical selectability (convention 6): 3 YES rows — DECL-005 (the
`_STATUS.md` surface row; two ungated residuals present), REM-002, REM-003.
REM-001 is gated (`gated: new D-XX ruling per mechanics plan §4`), REM-004 is
stage-gated (`stage-gated: R5 release evidence`). The owner suspension is a
run-level caveat (RUN_BASIS), never per-row. Gate-state cells reflect the
frozen register state (D-41 `AWAITING_RULING`) per the RUN_BASIS codification.

## 2. Self-flagged rows

- DEL-04-04-REQ-001 — MECHANICS class with `ValidationEvidence=NOT_APPLICABLE`
  (D5 in-cell reason given: architecture-boundary claim, no numeric
  assertion). Reviewer eyes welcome on the class choice vs GOVERNANCE.
- DEL-04-04-REQ-006 — calibration-item-6 grain call: encoded
  PARTIALLY_IMPLEMENTED at implementation-evidence grain (unit metadata is
  metadata-only; no conversion/dimensional computation; canonical unit basis
  TBD). ALIGNED-at-contract-grain was arguable via the requirement's "where
  applicable". Also the REVIEW routing (pending human disposition of WARNING
  finding PKG04-DEL0404-PKG02-001) vs addendum-13 OWNER routing — W1 noted
  this variance for R3 dedupe.
- DEL-04-04-REQ-008 — calibration-item-6 grain call: PARTIALLY_IMPLEMENTED at
  implementation-evidence grain (solver-version disclosure and final
  result-envelope integration are recorded TBDs); ALIGNED was arguable via
  the kit verification's "present or explicitly TBD" clause.
- DEL-04-04-DECL-002 — Datasheet carries rev-0.7 pointer drift (W1
  adjudicated STALE side) plus ONE overtaken friction-TBD construction cell;
  I kept `AuthorityNeeded=NO` because a single overtaken cell is not an
  "overtaken TBD register" (the W1 OWNER trigger). Reviewer may disagree.
- DEL-04-04-DECL-003 — Guidance encoded STALE_SETUP_SPECIFICATION on widened
  addendum 4 (setup-era future-tense "prepares a future ... slice", "Examples
  intentionally not specified" vs the implemented 28-fixture slice). Guidance
  is advisory, so the staleness call is a judgment the conventions leave
  open; Confidence MEDIUM.
- DEL-04-04-DECL-004 — Procedure kept ALIGNED: it declares a documentation
  setup procedure (historically accurate) and its step 3 was refreshed with
  current DEC-067 mechanics; a stricter reading of addendum 4 could push it
  STALE alongside Guidance.
- DEL-04-04-DECL-007 — census judgment: included
  `core/solver/nonlinear_supports/README.md` as a deliverable-owned in-tree
  README (it self-identifies: "This crate is the bounded implementation slice
  for DEL-04-04") even though the crate surface (SURF-135) is mapped to
  multiple deliverables. W1 precedent exists (2 README DECL rows).
- DEL-04-04-REM-001 — `AuthorityNeeded=OWNER` because the gate is a ruling
  that does not exist yet ("new D-XX"); no named decision ID is available to
  route instead.

## 3. Evidence-execution log

Re-executed side-effect-free at frozen SHA 551f84ef6 (all with artifacts
redirected outside the tree; `git -C FROZEN status --porcelain` empty before
and after every run):

- `CARGO_TARGET_DIR=<scratch> cargo test --manifest-path
  validation/benchmarks/nonlinear/Cargo.toml --locked` — PASS, 19 tests.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest
  tests/test_nonlinear_support_regression.py -q -p no:cacheprovider` — PASS,
  8 tests.

NOT re-executed (cited as recorded):

- `cargo test` for `core/solver/nonlinear_supports` (19 static tests) and
  `core/solver/nonlinear_integration` (16 static tests): `--locked` refuses
  in the frozen tree (Cargo.lock update required), and running unlocked would
  write `Cargo.lock` inside the frozen tree — not side-effect-free, so it was
  not run (addendum 9). Cited instead: DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (cargo_crate_sweep=pass) at commit `e648462f1`, marker `not re-executed at
  frozen SHA 551f84ef6`, with the addendum-10 qualifier backed by a diff I
  actually ran: `git diff e648462f1d05...551f84ef6` over
  `core/solver/nonlinear_supports/`, `core/solver/nonlinear_integration/`,
  and `validation/` is empty except the sweep-record JSON itself (exclusion
  clause carried in-row).
- `_REVIEW.md` lifecycle-readiness checks (PB-001, TB-001) and the PKG-02
  audit verdicts: recorded reviews, `not re-executed at frozen SHA 551f84ef6`.

Attestation-level citations (convention 7): "Receipt 6" (named by `_STATUS.md`
Remaining item 1's source triple) is a loop receipt not present in the frozen
tree — marked `ATTESTED: loop receipt named in _STATUS.md, record not present
in tree` in-row. The other two members of that source triple (mechanics plan
§4, TP-PMM-P2-FRICTION-001 run record) resolve in-tree.

Residual homing (convention 3): both threshold residuals say "see also
DEL-09-03"; DEL-09-03's frozen `_STATUS.md ## Remaining` carries only the
bootstrap item, so the residuals are homed here with the cross-reference
recorded in-row — no REMAINING_STATE_MISMATCH.

## 4. Convention friction notes

1. **Kit 2-digit vs addendum-12 3-digit requirement IDs**: handled by
   recording the kit-native form in `NormativeSource` and using the fixed
   ClaimID form; the conventions never say where the native form belongs
   in-row — I used the NormativeSource cell (matches the dispatch prompt's
   "record the kit's native form in-row").
2. **Bootstrap item on a multi-residual `_STATUS.md` row** (addendum 3 vs W1
   calibration item 5): I transcribed all five `## Remaining` items byte-exact
   in `RecordedRemaining` (bootstrap included, since the cell is a verbatim
   copy surface), but listed gate/source entries only for the four
   non-bootstrap residuals — no annotated bootstrap variant in the gate cell.
   W1's DEL-02-05 ledger annotated the bootstrap gate in-cell; calibration
   item 5 reads against that, but "column defaults (NONE_RECORDED)" is not
   literally satisfiable on a row whose other residuals populate the cells.
   Reviewer should confirm this reading at fan-in.
3. **Cargo `--locked` refusal in the frozen tree**: the pinned crates'
   `Cargo.lock` state makes crate-local re-execution impossible without a
   tree write; addendum 9 handled it, but W2+ pilots for PKG-04/05 will all
   hit this — the workspace-level sweep record plus addendum-10 diff is the
   workable evidence path.
4. **ENGINEERING routing on ALIGNED REMAINING_WORK rows** (calibration item
   8): the residual records are valid as records (ALIGNED), while the work
   they name needs engineering authority; I put ENGINEERING in
   `AuthorityNeeded` on REM-002/003/004. If AuthorityNeeded is meant to name
   authority needed to *resolve the row's finding* rather than to *perform
   the recorded work*, these would be NO — conventions leave it open.

**Owner-calibration caveat (stated once per W1 calibration item 1):** the
rev-0.7 decomp authority-pointer drift on DECL-002 follows the W1 fan-in
adjudication (STALE side under addendum 4, drift facts in-row, decomp §13
sanction as immateriality context). If the owner later calibrates this
corpus-wide pattern to ALIGNED-with-note, the affected row flips without
touching its drift facts.

## 5. Boundary compliance

- All discovery reads were from the FROZEN worktree at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; `git status --porcelain` on the
  frozen worktree was empty before discovery, after each re-execution, and
  after ledger writing.
- Writes were confined to exactly two files:
  `RUN/WAVES/W2/CLAIM_CONCORDANCE_DEL-04-04.csv` and
  `RUN/WAVES/W2/NOTES_DEL-04-04.md` (CSV generated by a scratchpad script
  named `build_csv_DEL-04-04.py` per W1 calibration item 7; script lives
  outside the repo).
- No lifecycle transition, DAG mutation, register edit, or product-file edit.
  No release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim is made by any row (rows
  quoting recorded "release use"/"R5 release evidence" language quote the
  record only and say so in-row). All dispositions are agent judgments, not
  owner or engineering rulings; authority is routed via `AuthorityNeeded`.
- No agent-workflow implications encountered; no `DEFERRED_AGENT_WORKFLOW`
  rows.
