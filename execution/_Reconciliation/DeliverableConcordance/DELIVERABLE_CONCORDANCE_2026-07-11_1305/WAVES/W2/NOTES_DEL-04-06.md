# Notes — DEL-04-06 Solver diagnostics and singularity detection (R2 W2 discovery ledger)

Ledger: `WAVES/W2/CLAIM_CONCORDANCE_DEL-04-06.csv` — 23 rows, encoded under
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13) plus the W1 calibration
items. All discovery reads from the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

Run-level `NormativeSource` path alias (addendum 12): bare artifact names
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `Review_Findings.csv`,
`_run_records/...`) resolve under
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/`;
code paths resolve from the repo working root `projects/chirality-piping/`.

Requirement-ID scheme: kit-native `REQ-04-06-001..011` (self-identifying);
ClaimIDs use the addendum-12 form `DEL-04-06-REQ-001..011` with the kit-native
form recorded in each row's `NormativeSource`.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (23 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 18 |
| STALE_SETUP_SPECIFICATION | 5 |

ClaimType histogram (23 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 11 |
| EXCLUSION | 5 |
| DECLARED_STATE | 7 |

No ACCEPTANCE rows: the Specification's Verification table restates the
requirements at requirement grain (addendum-12 grain rule — no mirrored
ACCEPTANCE rows). No REMAINING_WORK rows: the frozen `_STATUS.md`
`## Remaining` carries only the seeded `(gated: D-41)` bootstrap item,
recorded verbatim in DECL-005's `RecordedRemaining` and excluded from
residual/gate/selectability analysis per addendum 2. No IMPLEMENTED_UNMAPPED
rows: every material surface in this deliverable's orbit (SURF-131
`core/solver/diagnostics`, SURF-132 `frame_kernel`, SURF-133
`linear_supports`, SURF-094 `primitive_loads`, SURF-138 `sparse_direct`,
SURF-102 `product_physics`) carries deliverable attribution in
`IMPLEMENTATION_SURFACES.csv`, re-verified against the frozen tree.

`SelectableUnderCurrentLoop` = NO on all rows (no non-bootstrap recorded
item; addendum-12 default). Mechanical derivation only; the owner suspension
is a run-level caveat, not encoded per-row.

## 2. Self-flagged rows

- **DEL-04-06-REQ-002 / REQ-003 / REQ-004** — routing judgment: the two
  `Review_Findings.csv` WARNING findings (PKG04-DEL0406-PKG02-001/002) remain
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition` TBD. I kept
  Disposition ALIGNED at HIGH confidence because the disposition rests on
  independent direct source inspection at the frozen SHA (addendum 13
  independent-grounds clause; the addendum-13 MEDIUM cap applies to
  *validation* evidence in that state, which these rows do not rely on), and
  routed the open human gate via `AuthorityNeeded=REVIEW`. Reviewer eyes
  wanted on whether the cap should nonetheless bind.
- **DEL-04-06-REQ-008 / REQ-009** — grain statement per W1 calibration item 6:
  encoded ALIGNED at contract grain (classification behavior with
  caller-supplied thresholds), with governed threshold *values* explicitly
  recorded open in `ValidationEvidence`/`RemainingWork` rather than
  VERIFIED_NOT_VALIDATED, because the requirements themselves demand
  caller-supplied-threshold behavior and the kit records the value TBDs.
  MECHANICS class chosen for numeric-classification correctness.
- **DEL-04-06-REQ-010** — grain statement per W1 calibration item 6: ALIGNED
  at contract grain. The requirement's own "until accepted by a later governed
  decision" clause is partially discharged by DEC-023 (sparse selection) while
  DEC-050/DEC-053 keep promotion TBD and DEC-026 keeps tolerance values TBD;
  the frozen crate still emits both warning-level status diagnostics. The
  alternative reading (IMPLEMENTED_DIFFERENTLY, since the SparseSolverTbd
  message is now an adoption-status record rather than a pure selection TBD)
  is defensible; wording drift is ledgered on DECL-001. MEDIUM confidence.
- **DEL-04-06-DECL-003** — OWNER routing judgment: Guidance carries overtaken
  sparse-TBD framing in its principles/trade-off tables (register-like TBD
  listing), so I applied the W1 calibration item-1 OWNER branch rather than NO.
- **DEL-04-06-DECL-004** — staleness-degree judgment: Procedure drift is
  evidence-count (19 vs 24 tests) and checklist-inventory only; encoded STALE
  at MEDIUM with `AuthorityNeeded=NO` (pure pointer/count drift branch).
- **DEL-04-06-DECL-006** — census/staleness judgment the conventions leave
  open: MEMORY.md's *undated* head sections (Implementation Summary /
  Evidence: 10 tests / Open TBDs incl. "Accepted sparse numerical library
  remains TBD") read as current-state declarations and are overtaken (frozen
  crate: 24 tests, 10-field envelope, DEC-023-resolved selection). Addendum 1
  protects *dated* log entries only, so I attached STALE_SETUP_SPECIFICATION
  to the head block at MEDIUM; treating the whole file as log-structured
  (ALIGNED + drift note) was the alternative.
- **ClaimClass assignments generally** — convention 7 does not name a class
  for error-to-diagnostic mapping claims; I used REPORTING (REQ-001/003/005),
  SCHEMA (REQ-002), WORKFLOW for OPS-K-DATA-2 explicit-findings claims
  (REQ-006/007), MECHANICS for numeric classification (REQ-008/009), and
  GOVERNANCE for authority/boundary claims (REQ-004/010/011, all EXC rows).

## 3. Evidence-execution log

Re-executed (side-effect-free, addendum 9):

- `git -C FROZEN status --porcelain` before discovery: empty.
- Attempted `CARGO_TARGET_DIR=<scratch> cargo test --manifest-path
  core/solver/diagnostics/Cargo.toml --locked` at the frozen SHA: **refused**
  ("the lock file ... needs to be updated but --locked was passed"). Because
  running without `--locked` would rewrite `Cargo.lock` inside the frozen
  tree, crate-local re-execution is not side-effect-free and was **not run**
  (same refusal the DEL-04-04 pilot recorded). The `--locked` refusal itself
  writes nothing; porcelain re-verified empty afterwards.
- `git -C FROZEN diff e648462f1d05..551f84ef6 --stat` over
  `core/solver/diagnostics/`, `core/solver/frame_kernel/`,
  `core/solver/sparse_direct/`, `core/solver/linear_supports/`,
  `core/loads/primitive_loads/`: **empty**; a second diff over all of `core/`
  is also empty. This earns the addendum-10 qualifier used in the ledger;
  `validation/` and `docs/` were not diffed and are not claimed.
- `git -C FROZEN status --porcelain` after all evidence work: empty.

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- DEC-025 sweep `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  — commit `e648462f1d0521e26df15d04a988391343018886` (ancestor of the frozen
  SHA), clean tree, `cargo_crate_sweep=pass` (all discovered crate manifests
  via `tools/release/check_release_readiness.py --profile cargo --execute`),
  overall pass. Sweep JSON inspected directly; crate coverage is
  sweep-granular (it does not name crates individually — the per-crate
  24-static-test figure comes from `VERIFICATION_INDEX.csv` RUST-25 and was
  re-verified by counting `#[test]` items in the frozen `src/lib.rs`: 24).
- `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md`
  — `cargo test ... --locked` 19 tests passing (2026-06-05, pre-sparse-mapping
  slice).
- 2026-06-05 review-readiness fan-in scans (stale wording,
  protected/prohibited claims) — MEMORY.md fan-in entry and package fan-in
  record `TASK_RUN_2026-06-05_2226...` (REQ-011, EXC-005).

Direct frozen-tree inspections: `src/lib.rs` symbol/field verification
(SolverDiagnostic ten fields; `analysis_boundary_mapping`;
`classify_condition_ratio`; `convergence_diagnostic`;
`sparse_solver_tbd_diagnostic` message text recording DEC-023/DEC-050/DEC-053;
`tolerance_policy_tbd_diagnostic`; `diagnostic_from_sparse_error` variants;
NonPositivePivot mapping), crate README, `SOFTWARE_DECOMP.md` header
(revision 0.8, `status: current_basis`), `_STATUS.md`, `Review_Findings.csv`,
decision records via `DECISIONS_INDEX.csv` rows DEC-023/024/026/046/050/053.

## 4. Convention friction notes

- **Owner-calibration caveat (recorded once, per W1 fan-in item 1):** the kit
  cites SOFTWARE_DECOMP revision 0.7 while the frozen decomp header is
  revision 0.8 / `status: current_basis`; encoded STALE-side on DECL-002 with
  drift facts in-row. `_CONTEXT.md` and `_REFERENCES.md` also carry the 0.7
  pointer but are outside the addendum-1 declared-state census, so that drift
  is noted here only.
- Addendum 13's confidence cap is worded for validation evidence; it is silent
  on rows whose *DecisionBasis* cites a pending-human review finding while the
  disposition rests on independent inspection. I routed REVIEW and kept HIGH
  (see self-flags for REQ-002/003/004); a clarifying addendum would remove the
  judgment call.
- Convention 7 lacks a class for error-mapping/diagnostic-content claims
  (neither "numeric/unit correctness" nor GUI); class picks are self-flagged
  above.
- The sweep evidence record is sweep-granular (no per-crate listing), so
  binding "24 static tests" to the recorded pass requires pairing the sweep
  with the R1 verification index plus a frozen-tree static count; a per-crate
  manifest in future sweep records would tighten this.
- MEMORY.md files with undated head sections sit awkwardly under addendum 1's
  dated-entry rule (see DECL-006 self-flag).

## 5. Boundary-compliance statement

All fences held. Discovery reads were confined to the frozen worktree at
`551f84ef6...` and the run's own R1 artifacts; no write of any kind was made
inside the frozen tree (build artifacts redirected to the session scratchpad;
the only cargo invocation used `--locked` and was refused before building;
porcelain verified empty before and after). No lifecycle transition, DAG
mutation, register edit, `_STATUS.md` edit, or cross-project edit. No
release-readiness, issuance, certification, sealing, professional-approval,
or code-compliance claim is made anywhere in this ledger or these notes
(F-PIP-1..5); rows quoting such recorded boundary language do so as
transcription only. All dispositions are agent judgments routed through
`AuthorityNeeded`, never owner or engineering rulings. Writes confined to
exactly two files: `WAVES/W2/CLAIM_CONCORDANCE_DEL-04-06.csv` and this
`WAVES/W2/NOTES_DEL-04-06.md`. CSV generated by a deliverable-named scratch
script (`build_csv_DEL-04-06.py`, session scratchpad), per W1 calibration
item 7.
