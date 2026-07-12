# W2 Discovery Notes — DEL-04-01 "3D frame stiffness kernel" (PKG-04)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W2. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-04-01.csv` (28 claim rows + header).

**NormativeSource path alias (declared once per addendum 12):** bare filenames
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REVIEW.md`, `Review_Findings.csv`)
resolve to the deliverable folder
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/`.
All other paths are repo-relative under `projects/chirality-piping/`.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (28 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 23 |
| STALE_SETUP_SPECIFICATION | 4 |
| ACCEPTED_DIVERGENCE | 1 |

ClaimType histogram (28 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 12 |
| EXCLUSION | 7 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 2 |

Acceptance rows: 0. The Specification "Verification" table restates the
requirements at "should/TBD" grain (no distinct acceptance criteria, expected
values, or tolerances of its own), so per addendum 12 no mirrored ACCEPTANCE
rows were minted. IMPLEMENTED_UNMAPPED rows: 0 — every material surface in
this deliverable's orbit (frame_kernel, sparse_direct, curved_bend,
nonlinear_integration, diagnostics, product_physics, benchmark crates) carries
a deliverable attribution in `IMPLEMENTATION_SURFACES.csv`.

Declared-state census (addendum 1): four kit surfaces + `_STATUS.md` +
`MEMORY.md` (both carry current-state declarations) + one deliverable-owned
in-tree README (`core/solver/frame_kernel/README.md`) = 7 DECL rows.

**Owner-calibration caveat (stated once per W1 calibration item 1):** the kit
cites SOFTWARE_DECOMP revision 0.7 (`Datasheet.md` Identification;
`_CONTEXT.md` Decomposition Reference) while the frozen decomposition header
is revision 0.8, `status: current_basis`. Encoded STALE-side on the affected
DECLARED_STATE surface row (DECL-002) per the W1 fan-in adjudication;
`_CONTEXT.md` is not a censused declared-state surface, so its rev-0.7
citation is recorded here as a note only.

## 2. Self-flagged rows

- **DEL-04-01-REQ-006** — W1 calibration item 6 grain choice: encoded
  `ACCEPTED_DIVERGENCE` rather than `PARTIALLY_IMPLEMENTED` because DEC-050
  (D-17 ruling) names and permits the exact deferred state at the frozen SHA
  (dense default + sparse evidence lane; default promotion follow-on), which
  meets the addendum-11 named-ruling bar. Grain used: the deliverable's
  sparse-solve-interface contract as a whole, not the frame_kernel crate's
  internal solve function.
- **DEL-04-01-REQ-007** — "unit-aware and dimensionally checked" is satisfied
  via boundary metadata plus an upstream-responsibility contract, accepted by
  the recorded human disposition (ACCEPT_AS_IS on PKG04-DEL0401-PKG02-001);
  the kernel's mechanics fields remain bare numerics. Encoded ALIGNED at
  MEDIUM confidence; a stricter reading could argue PARTIALLY_IMPLEMENTED.
- **DEL-04-01-REQ-011** — residual-homing judgment: the 2026-06-05 review
  named "final result-envelope integration" a downstream TBD; no current
  `_STATUS.md` residual names it and later product_physics envelope work
  covers the recorded boundary, so no REMAINING_STATE_MISMATCH was encoded.
  Reviewer eyes welcome on that homing.
- **DEL-04-01-EXC-001** — the "solver implementation in this setup run"
  exclusion vs the implemented frozen state: encoded ALIGNED because the
  exclusion is temporally scoped to the setup run (which honored it) and
  implementation flows from the separately authorized DEV-001 dispatch;
  DecisionBasis is ATTESTED (dispatch record not present in tree).
  ACCEPTED_DIVERGENCE was the considered alternative.
- **DEL-04-01-DECL-003** — treated Guidance's trade-off-table TBD rows
  (sparse library, coordinate convention) as overtaken TBD registers,
  routing `AuthorityNeeded=OWNER` per W1 calibration item 1. Conventions do
  not settle whether a guidance trade-off table is a "TBD register".
- **DEL-04-01-DECL-004** — Procedure's step-2 "identify decisions still TBD"
  list includes items since sealed (DOF ordering, sparse storage, solver
  library). Treated as stale procedural prose, not a TBD register, so
  `AuthorityNeeded=NO` (contrast DECL-002/003).
- **DEL-04-01-DECL-006** — MEMORY's undated header "Open TBDs" block still
  says the sparse library remains TBD while the dated 2026-06-11 entry
  records DEC-023 resolution. Kept ALIGNED with an in-row drift note per
  addendum 1 (the block sits with the original log entry material and is
  corrected in-file); a stricter reading could call the undated block a
  stale current-state declaration.
- **DEL-04-01-REM-001** — `AuthorityNeeded=NO`: the remainder is a recorded
  formulation-extension work item, not a numeric-threshold question, so the
  W1 calibration item 8 ENGINEERING routing was considered and not applied.
  Any future pressure-thrust formulation choice would itself need lawful
  source grounding per the kit rules.
- **DEL-04-01-REM-002** — the §5 completion check spans DEL-04-04/DEL-05-01/
  DEL-05-02; this ledger records the item as accurately homed and gated
  (owner re-disposition) without auditing the sibling deliverables' gap-row
  state. Cross-ledger reconciliation belongs to the W2 fan-in.

## 3. Evidence-execution log

Re-executed (side-effect-free, addendum 9):

- `git -C <FROZEN> status --porcelain` — **empty before and after** all
  evidence operations (checked at discovery start, before re-execution, and
  after re-execution).
- Addendum-10 qualifier diff, actually run:
  `git diff --stat e648462f1d0521e26df15d04a988391343018886..551f84ef6be656f1603ce0acfa5e3935aa9683c7 -- projects/chirality-piping/core/solver/ projects/chirality-piping/core/product_physics/ projects/chirality-piping/validation/benchmarks/ projects/chirality-piping/validation/hand_calcs/`
  → **empty** (content-identical over exactly those four path roots; no
  exclusion clause needed; no other paths are claimed under the qualifier).
- `cargo test --offline` on the frame_kernel crate: the crate has no
  `Cargo.lock` in-tree, so running cargo against the frozen manifest would
  have written a lock file into the frozen tree. To stay side-effect-free the
  crate was copied to the run scratchpad, verified byte-identical to the
  frozen `core/solver/frame_kernel` (`diff -r` clean), and tested there with
  an external `CARGO_TARGET_DIR`: **36 unit tests passed, 0 failed**
  (0 doctests). Frozen tree untouched (porcelain empty after).

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- DEC-025 sweep `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (clean tree, commit `e648462f1d05…`, ancestor of the frozen SHA) for the
  crate suites: frame_kernel 36, sparse_direct 20, nonlinear_integration 16,
  curved_bend 16 static tests (`VERIFICATION_INDEX.csv` RUST-26/31/28/24),
  carried with the addendum-10 content-identical qualifier above.
- Historical recorded passes in MEMORY.md/_REVIEW.md (11→19→23→33→34-test
  progression; fmt checks; 2026-06-05 review validation) — recorded evidence
  only, superseded for currency by the re-execution above.
- Hand-calc witnesses HC-MECH-019/020 and benchmark comparisons at DEC-026
  tiers — cited from `VALIDATION_AND_PROVENANCE_INDEX.csv` and MEMORY.md;
  not re-derived.

DecisionBasis resolvability: all cited ruling/record paths verified present
in the frozen tree except the DEV-001 dispatch
(`execution/_Coordination/DEV-001_DISPATCH_DEL-04-01.md`), which is marked
`ATTESTED: … record not present in tree` on EXC-001 per convention 7 (the
finding-resolution matrix directory
`execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/`
is likewise absent at the frozen SHA and was therefore not cited as a basis).

## 4. Convention friction notes

- **"TBD register" boundary (addendum 4 / W1 calibration item 1):** the
  OWNER-routing rule keys on kits "carrying overtaken TBD registers", but the
  conventions never define the register grain. This kit has three different
  TBD carriers — a Datasheet attributes table (clear register), a Guidance
  trade-off table (arguable), and a Procedure instruction list (arguably not
  a register). I split them OWNER/OWNER/NO and self-flagged; a one-line
  definition would remove the judgment.
- **Undated header sections in MEMORY files:** addendum 1 distinguishes dated
  log entries (historical) from current-state declarations, but says nothing
  about undated summary blocks written at first-slice time and corrected by
  later dated entries in the same file (DECL-006). Encoded as note-not-
  staleness; worth a W2+ harmonization line.
- **REMAINING_WORK rows whose residual is a verification task, not product
  work (REM-002):** §7 dispositions describe substance states of claims;
  a "verify X closed" item fits awkwardly — ALIGNED here means "the recorded
  item accurately reflects the open state", not "the verification passed".
  Stating that reading explicitly in the conventions would help aggregation.
- **Missing Cargo.lock vs addendum 9:** `--locked` re-execution is impossible
  for crates with no committed lock file without writing into the frozen
  tree. The byte-identical scratch-copy pattern used here (copy, `diff -r`
  verify, external target dir, porcelain check) may be worth adopting as the
  standard workaround for W2+ solver crates.
- **Multi-residual `RecordedRemaining` joining:** followed the W1 DEL-01-03
  precedent (`" ; "` separator, items byte-exact, bootstrap last and
  excluded from the gate/source cells). The conventions themselves never fix
  the separator.

## 5. Boundary-compliance statement

All fences held. Discovery reads were confined to the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` plus the run's own R1 artifacts;
the frozen tree porcelain was empty before and after all evidence operations
(including the cargo re-execution, which ran on a scratchpad copy with an
external target directory). No lifecycle transition, DAG mutation,
register/_STATUS/product-file edit, or cross-project edit was made. No
release-readiness, issuance, certification, sealing, professional-approval,
or code-compliance claim appears in these outputs (F-PIP-1..5). All
dispositions are agent judgments routed through `AuthorityNeeded`, never
owner or engineering rulings. Writes were confined to exactly two files:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-04-01.csv` and `WAVES/W2/NOTES_DEL-04-01.md`
(the CSV generator script lives in the session scratchpad, outside the repo).
`SelectableUnderCurrentLoop` cells are mechanical DAG/lifecycle/gate
derivations only; the owner suspension remains the run-level caveat.
