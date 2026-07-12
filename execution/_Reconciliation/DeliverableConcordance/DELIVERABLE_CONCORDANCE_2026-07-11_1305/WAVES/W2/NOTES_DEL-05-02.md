# W2 Discovery Notes — DEL-05-02 "Load-case algebra engine" (PKG-05)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W2. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-02.csv` (23 claim rows + header).

**NormativeSource path alias (declared once per addendum 12):** bare filenames
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`,
`_DEPENDENCIES.md`, `Dependencies.csv`, `Review_Findings.csv`,
`_run_records/...`) resolve to the deliverable folder
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/`.
All other paths are repo-relative under `projects/chirality-piping/`.

Requirement-ID note: the kit's native scheme is `REQ-05-02-*`
(self-identifying); ClaimIDs use the addendum-12 form
`DEL-05-02-REQ-NNN` with the kit-native ID recorded in each row's
`NormativeSource`.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (23 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 21 |
| STALE_SETUP_SPECIFICATION | 2 |

ClaimType histogram (23 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 9 |
| EXCLUSION | 5 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 2 |

Acceptance rows: 0. The Specification "Verification" table maps each
requirement to named crate tests but states no distinct acceptance criteria,
expected values, or tolerances of its own — per addendum 12 no mirrored
ACCEPTANCE rows were minted (same shape and treatment as the sibling
DEL-04-01 ledger). IMPLEMENTED_UNMAPPED rows: 0 — every material surface in
this deliverable's orbit (`core/loads/load_case_algebra`,
`core/loads/primitive_loads`, `core/loads/stress_recovery`,
`core/product_physics`, `core/model_operations/operation_applier`,
`schemas/material.schema.yaml`, `schemas/model.schema.yaml`,
`schemas/results.schema.yaml`, desktop load-cases feature, wasm engine
surfaces, `validation/benchmarks/stress`, `fixtures/*`) carries a
deliverable attribution in `IMPLEMENTATION_SURFACES.csv` (SURF-004/005/027/
065/093/094/095/096/099/102/135/136/155/160/176/189/190/202/228).

Declared-state census (addendum 1): four kit surfaces + `_STATUS.md` +
`MEMORY.md` (both carry current-state declarations) + one deliverable-owned
in-tree README (`core/loads/load_case_algebra/README.md`, which names itself
"the bounded implementation slice for `DEL-05-02`") = 7 DECL rows.

SelectableUnderCurrentLoop: 0 YES rows — both non-bootstrap residuals carry
the recorded suffix `gated: D-38`, and the frozen register shows D-38
`AWAITING_RULING` (register row dated 2026-07-10; ruling record column
empty). Mechanical derivation per convention 6/addendum 3 gives NO on the
`_STATUS.md` surface row and both REM rows; all other rows default NO per
addendum 12 (no recorded item). This reproduces the R1
`DELIVERABLE_INVENTORY.csv` grain (SelectableUnderCurrentLoop=NO). The
RUN_BASIS ruling-after-freeze codification covers D-41 only; D-38 has no
post-freeze ruling claimed anywhere in the run records, so its frozen
AWAITING_RULING state was used as-is.

**Owner-calibration caveat (stated once per W1 calibration item 1):** the
Datasheet References section cites `SOFTWARE_DECOMP.md` revision 0.7 while
the frozen decomposition header is revision 0.8, `status: current_basis`.
Encoded STALE-side on the affected DECLARED_STATE surface row (DECL-002)
with drift facts in-row, AuthorityNeeded=NO (pure pointer drift; the
Datasheet carries no overtaken TBD register). `_CONTEXT.md` (Accepted
Revision 0.7) and `_REFERENCES.md` (Accepted revision 0.7) also carry the
stale pointer, and the dated MEMORY 2026-06-04 authority-refresh entry cites
"revision 0.7 plus DAG-006" — none of these is a censused declared-state
surface (and the MEMORY citation is a dated historical entry), so they are
recorded here as notes only. `_DEPENDENCIES.md` already cites the current
DAG-007 authority.

## 2. Self-flagged rows

- **DEL-05-02-DECL-003** — judgment call: W1 calibration item 3 covers
  overtaken *review-disposition* kit prose; here the overtaken Guidance
  sentence is *dependency-maturity* prose ("DEL-05-01 and DEL-05-04 remain
  pending dependency maturity items in the local register") overtaken by the
  same 2026-06-05 blocker-closure human ruling that flipped
  `Dependencies.csv` rows E0451/E0453 to SATISFIED. I extended the item-3
  encoding by analogy: STALE_SETUP_SPECIFICATION on the surface row, the
  ruling packet as DecisionBasis, AuthorityNeeded=REVIEW. Reviewer eyes
  wanted on whether widened convention 1 alone (plain STALE, NO) was the
  intended encoding instead.
- **DEL-05-02-REM-002** — gate-metadata scope question, disclosed in-row:
  `_STATUS.md` gates the shear-modulus residual `(gated: D-38)`, but the
  D-38 packet/register row scopes the *temperature interpolation policy*;
  the source run record calls the shear slot "a recorded residual outside
  the (E, α) grant" without naming a gate. I transcribed the recorded gate
  mechanically and kept Disposition=ALIGNED (the residual itself is real,
  current, and correctly homed); the possible gate-metadata over-breadth is
  in `RemainingWork` and routed AuthorityNeeded=D-38. Alternative reading:
  REMAINING_STATE_MISMATCH on gate metadata — I judged that too strong for
  an owner-consolidated register row, but flagging for the fan-in.
- **DEL-05-02-REQ-005** — class choice: encoded SECURITY (OPS-K-RULE-2
  no-arbitrary-executable-rules surface) but did NOT apply the convention-6
  SECURITY deferral marker, because the claim is a structural-absence claim
  with no recorded owner-gated sufficiency deferral on this slice (evaluator
  sandbox sufficiency is DEL-06-02 scope; the E0616 interface row is RETIRED
  non-gating by human ruling). If the fan-in reads convention 6 as requiring
  the em-dash marker on every SECURITY row lacking validation, this row and
  EXC-004 are the ones to harmonize.
- **DEL-05-02-REQ-003 / REM-001 (SourceReliability=UNVERIFIED)** — these
  rows rest materially on the 2026-07-10 TP-PMM-P3 witness/benchmark
  evidence, which is agent-generated with agent audit and no human
  disposition yet, hence UNVERIFIED per addendum 6 — while the other REQ
  rows rest on the crate evidence covered by the 2026-06-05 named human
  ruling and are REVIEWED. The split within one ledger is deliberate;
  flagging so the fan-in doesn't read it as drift.
- **DEL-05-02-DECL-001** — grain note: the Specification predates the
  2026-06-12 additive crate changes (RangeMode token helpers; 17→18 tests)
  and the 2026-07-10 modulus-basis records riding in result envelopes, but
  no Specification sentence is contradicted (the crate needed no change for
  TP-PMM-P3 per the run record). Encoded ALIGNED at contract grain per W1
  calibration item 6; stating the grain here and flagging the row.

## 3. Evidence-execution log

Re-executed (side-effect-free per addendum 9):

- `cargo test --offline` on a scratch copy of `core/loads/load_case_algebra`
  (with `core/loads/primitive_loads` and its transitive path deps
  `core/solver/frame_kernel`, `core/solver/linear_supports` copied
  alongside), copies verified byte-identical to the frozen tree via
  `diff -r` before the run; external
  `CARGO_TARGET_DIR=<scratchpad>/del0502_target`; result: **18 passed,
  0 failed, 0 doctests**. Frozen-tree `git status --porcelain` empty BEFORE
  and AFTER (also re-checked after the addendum-10 diffs below). Nothing was
  executed inside the frozen tree itself.

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- DEC-025 five-surface sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (cargo_crate_sweep=pass, clean tree) at commit
  `e648462f1d0521e26df15d04a988391343018886`, an ancestor of the frozen SHA
  (R1 `VERIFICATION_INDEX.csv` rows RUST-03 load_case_algebra 18 tests,
  RUST-05 stress_recovery 26, RUST-08 product_physics 71).
- TP-PMM-P3-MODULUSBASIS-001 run-record check set (product_physics 71,
  stress_recovery 26, stress benchmarks 22, pytest 377) — recorded passes in
  `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P3-MODULUSBASIS-001.md`.

Addendum-10 qualifier — diff actually run:
`git diff e648462f1d05...551f84ef6` over
`projects/chirality-piping/{core/loads/, core/product_physics/,
validation/benchmarks/stress/, validation/hand_calcs/stress/, schemas/}`
returned **empty**. Explicit exclusion clause carried in every row using the
qualifier: repo-root `docs/`, `projects/chirality-piping/docs/`
(AGENTIC_DEVELOPMENT_WORKFLOW.md, TYPES.md), and the 100 seeded
`execution/**/_STATUS.md` surfaces DO differ in that window (110 piping
paths total) and are excluded from the identity claim.

Structural reads at frozen SHA: crate `src/lib.rs` (18 `#[test]` functions
enumerated; no parser/eval surface; single path dependency per
`Cargo.toml`), README, all kit/coordination surfaces, `Dependencies.csv`
(19 rows, E0451/E0452/E0453 SATISFIED, E0616 RETIRED), `Review_Findings.csv`
(W001/W002 ACCEPT_AS_IS/RESOLVED), frozen `_REGISTER.md` row D-38
(AWAITING_RULING), frozen `SOFTWARE_DECOMP.md` header (revision 0.8,
current_basis).

## 4. Convention friction notes

1. **Item-3 analogs beyond review prose.** W1 calibration item 3 names
   overtaken review-disposition prose specifically; overtaken
   dependency-maturity prose (DECL-003 here) has the same shape (kit says
   pending, frozen register records a human-ruled disposition) but no named
   encoding. I reused the item-3 form; a one-line generalization ("overtaken
   kit prose contradicted by a human-ruled register state → STALE with the
   ruling as DecisionBasis, REVIEW") would remove the judgment call for
   W3–W5.
2. **Gate suffixes pointing at possibly-narrower decisions.** Nothing in the
   conventions says what to do when a recorded gate suffix names a decision
   whose packet scope may not cover the residual (REM-002). Mechanical
   transcription plus an in-row note is what I did; worth a named rule if it
   recurs.
3. **SECURITY marker trigger.** Convention 6's marker is defined for
   "accepted scope defers sufficiency review"; the harmonized W1 form binds
   the marker + OWNER routing to *owner-gated* deferrals. A row that is
   SECURITY-class but makes a pure absence claim (REQ-005, EXC-004) fits
   neither branch — stated my reading in-row; see self-flags.
4. **REVIEWED vs UNVERIFIED within one deliverable.** Addendum 6 resolves
   cleanly per-row, but produces a mixed ladder inside one ledger when a
   deliverable has both human-ruled (2026-06-05) and post-ruling
   agent-generated (2026-07-10) evidence. Not a defect — just noting the
   histogram consequence for R3.

## 5. Boundary-compliance statement

Fences held. All discovery reads were from the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; no write of any kind was made
under the frozen tree (re-execution used byte-identical scratch copies with
an external CARGO_TARGET_DIR; `git status --porcelain` on the frozen
worktree was empty before and after every execution and diff). Writes were
confined to exactly two run files:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-02.csv` and `WAVES/W2/NOTES_DEL-05-02.md`
(plus the deliverable-named generator script in the session scratchpad,
outside both trees). No lifecycle transition, DAG mutation, register edit,
or product-file edit. No release-readiness, issuance, certification,
sealing, professional-approval, or code-compliance claim is made anywhere in
these outputs (F-PIP-1..5). All dispositions are agent judgments routed
through AuthorityNeeded, not owner or engineering rulings. No agent-workflow
change is proposed; no DEFERRED_AGENT_WORKFLOW rows were needed.
