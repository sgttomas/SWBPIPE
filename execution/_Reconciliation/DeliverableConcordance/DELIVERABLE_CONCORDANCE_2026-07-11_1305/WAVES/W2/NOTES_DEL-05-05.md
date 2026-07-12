# W2 Discovery Notes — DEL-05-05 "Concentrated and distributed user load application" (PKG-05)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W2. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-05.csv` (21 claim rows + header).

**NormativeSource path alias (declared once per addendum 12):** bare filenames
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`,
`_DEPENDENCIES.md`, `Dependencies.csv`, `Review_Findings.csv`, `_REVIEW.md`,
`_run_records/...`) resolve to the deliverable folder
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/`.
All other paths are repo-relative under `projects/chirality-piping/`.

Requirement-ID note: the kit's native scheme is `DEL-05-05-R1`..`R10`
(self-identifying, matching the R1 inventory row). ClaimIDs use the
addendum-12 fixed form `DEL-05-05-REQ-NNN` with the kit-native ID recorded in
each row's `NormativeSource`.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (21 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 17 |
| STALE_SETUP_SPECIFICATION | 4 |

ClaimType histogram (21 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 10 |
| EXCLUSION | 4 |
| DECLARED_STATE | 7 |

Acceptance rows: 0. The Specification "Verification" table maps each
requirement to named existing tests but states no distinct acceptance
criteria, expected values, or tolerances of its own — per addendum 12 no
mirrored ACCEPTANCE rows were minted (same treatment as the sibling
DEL-04-01/DEL-05-02 ledgers). REMAINING_WORK rows: 0 — the frozen
`_STATUS.md ## Remaining` carries only the seeded `(gated: D-41)` bootstrap
item, transcribed verbatim into the `_STATUS.md` surface row's
`RecordedRemaining` per addendum 2 and excluded from residual/gate/
selectability analysis; the kit's declared TBDs (result-envelope/API/
persistence/GUI/CLI/report integration, production tolerance policy, release
thresholds, axial-effect provenance beyond `load_id`, professional reliance)
are declared bounded TBDs recorded in-row on the affected REQ/EXC rows, not
`## Remaining` residuals. IMPLEMENTED_UNMAPPED rows: 0 — every material
surface in this deliverable's orbit carries a deliverable attribution in
`IMPLEMENTATION_SURFACES.csv` (`core/loads/user_loads` SURF-096;
`core/loads/load_case_algebra` SURF-093; `core/solver/straight_pipe`
SURF-139; desktop load-cases feature SURF-027; operationService SURF-056;
unitCatalogService SURF-062; src-tauri SURF-005; `schemas/model.schema.yaml`
SURF-190; `schemas/results.schema.yaml` SURF-202).

Declared-state census (addendum 1): four kit surfaces + `_STATUS.md` +
`MEMORY.md` (both carry current-state declarations) + one deliverable-owned
in-tree README (`core/loads/user_loads/README.md`, which names itself "the
bounded implementation slice for `DEL-05-05`") = 7 DECL rows.

SelectableUnderCurrentLoop: 0 YES rows — no non-bootstrap recorded item
exists; all rows default NO per addendum 12. This reproduces the R1
`DELIVERABLE_INVENTORY.csv` grain (NonBootstrapItems=NONE,
SelectableUnderCurrentLoop=NO).

**Owner-calibration caveat (stated once per W1 calibration item 1):** the
Specification (Standards), Datasheet (References), and Procedure
(Prerequisites) all cite `SOFTWARE_DECOMP.md` revision 0.7 while the frozen
decomposition header is revision 0.8, `status: current_basis`. Encoded
STALE-side on the three affected DECLARED_STATE surface rows
(DECL-001/002/004) with drift facts in-row, AuthorityNeeded=NO (pure pointer
drift; none of the three surfaces carries an overtaken TBD register).
`_CONTEXT.md` (Accepted Revision 0.7, Architecture Basis Injection revision
0.7) and the dated MEMORY 2026-06-04 authority-refresh entry ("revision 0.7
plus DAG-006") also carry the stale pointer — neither is a censused
declared-state surface (and the MEMORY citation is a dated historical
entry), so they are recorded here as notes only.

## 2. Self-flagged rows

- **DEL-05-05-DECL-007** — judgment call: the crate README omits the entire
  implemented straight-pipe equivalent recovery family (both
  `apply_straight_pipe_equivalent_user_loads*` public APIs, element-station
  point forces, oriented-direction handling, the axial-effect bridge —
  Specification R7–R10 scope, 13 of 28 tests) from its Scope and
  Verification sections, while its boundary/unit paragraphs are current
  (last refreshed TP-PHYS-014-C 2026-05-17). I read this as material
  post-alignment drift under widened convention 1 and encoded
  STALE_SETUP_SPECIFICATION (MEDIUM). Alternative reading: ALIGNED at
  contract grain, since no README sentence is contradicted (the omissions
  are silence, not false statements) and one Scope bullet ("non-positive
  element lengths where a length is supplied") already presupposes the
  recovery path. Reviewer eyes wanted on which side of the W1
  calibration-item-6 line a material-omission (not future-tense) README
  falls.
- **DEL-05-05-REQ-007 / REQ-008 / REQ-009 / REQ-010
  (SourceReliability=UNVERIFIED)** — ladder choice: these mechanics rows
  carry real validation evidence (DEL-09-01 benchmark fixtures calling this
  crate's APIs directly against hand-calc witnesses), but the witnesses are
  project-original agent-generated with agent audit and no recorded human
  disposition on the witness set, and for mechanics-class claims the
  validation leg is load-bearing — so addendum 6 gives UNVERIFIED even
  though the crate implementation/verification evidence is covered by the
  2026-06-05 named human ruling (which is why REQ-001..006 sit at
  REVIEWED). Same deliberate split as the sibling DEL-05-02 ledger
  (REQ-003/REM-001 there); flagging so the fan-in doesn't read the mixed
  ladder as drift. Confidence stays HIGH on independent grounds (the
  alignment itself is well-evidenced).
- **DEL-05-05-REQ-001** — grain statement: the requirement is an
  input-vocabulary claim ("support ... as explicit user load inputs"); the
  distributed/station categories are benchmark-validated through the
  recovery API, while the generic nodal concentrated force/moment path has
  deterministic unit-test verification only (direct DOF mapping — I
  treated a separate numeric validation basis as not applicable at this
  grain, keeping REVIEWED/ALIGNED rather than VERIFIED_NOT_VALIDATED).
  Stated the grain in-row per W1 calibration item 6; flagging the call.
- **DEL-05-05-EXC-004** — reading call: the desktop Load Cases GUI tranches
  (MEMORY.md 2026-06-11/12) consumed DEL-05-05 design authority, which
  could be read against the "excludes ... GUI ... integration" scope
  sentence. I read the exclusion's "unless separately dispatched" branch as
  covering them (they were separately dispatched WORKING_ITEMS tranches
  with no `user_loads` source change) and kept ALIGNED with the facts
  in-row. Flagging in case the fan-in prefers an explicit
  ACCEPTED_DIVERGENCE-style treatment.
- **DEL-05-05-REQ-005** — routing call: the recorded release-threshold /
  production-tolerance-policy TBDs are declared bounded kit TBDs with no
  numeric values proposed anywhere in the frozen records, so I kept
  AuthorityNeeded=NO (nothing to promote; per the W2 solver-mechanics
  strictness, any future threshold adoption would route ENGINEERING, but no
  such claim exists to route). Flagging the NO-vs-ENGINEERING choice.

## 3. Evidence-execution log

Re-executed (side-effect-free per addendum 9):

- `cargo test --offline` on a scratch copy of `core/loads/user_loads` (with
  its path deps `core/loads/primitive_loads`, `core/solver/frame_kernel`,
  `core/solver/straight_pipe`, `core/solver/linear_supports` copied
  alongside in the same relative layout), all five copies verified
  byte-identical to the frozen tree via `diff -r` before the run; external
  `CARGO_TARGET_DIR=<scratchpad>/del0505_target`; result: **28 passed,
  0 failed, 0 doctests** — matching the 28-test count recorded by the
  2026-06-05 blocker-closure ruling and R1 `VERIFICATION_INDEX.csv` row
  RUST-06. Frozen-tree `git status --porcelain` empty BEFORE and AFTER
  (re-checked after the addendum-10 diffs below). Nothing was executed
  inside the frozen tree itself. Byte-identical out-of-tree copy pattern
  self-flagged as a convention note per the dispatch brief (addendum 9
  in-place run avoided entirely).

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- DEC-025 five-surface sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (cargo_crate_sweep=pass, clean tree) at commit
  `e648462f1d0521e26df15d04a988391343018886`, an ancestor of the frozen SHA
  (R1 `VERIFICATION_INDEX.csv` row RUST-06, 28 static tests).
- 2026-06-05 lifecycle-readiness ruling evidence: locked user-loads crate
  tests 28 passed, 0 doctests (`_REVIEW.md` VAL-001;
  `PKG05_BLOCKER_CLOSURE_RULING_PACKET_2026-06-05_2120.md`; snapshot
  `REV_DEL-05-05_2026-06-05_2120/`).
- DEL-09-01 mechanics benchmark suite (BENCH-001, 21 fixtures): fixtures
  MECH-TP-PHYS-004/005/006/007/009 call
  `apply_straight_pipe_equivalent_user_loads` /
  `..._with_axial_effects` directly (confirmed by structural read of
  `validation/benchmarks/mechanics/src/lib.rs` use-declarations and call
  sites, and the crate dependency on `open_pipe_stress_user_loads` in its
  `Cargo.toml`) against hand-calc witnesses HC-MECH-008/009/010/014 — cited
  as recorded validation provenance; the benchmark suite itself was NOT
  re-executed by this pilot.

Addendum-10 qualifier — diff actually run:
`git diff e648462f1d05..551f84ef6 --name-only` over
`projects/chirality-piping/{core/, validation/benchmarks/mechanics/,
validation/hand_calcs/mechanics/}` returned **empty** (0 paths). Explicit
exclusion clause carried in every row using the qualifier: repo-root `docs/`
and `tools/practitioner_harness/`, `projects/chirality-piping/docs/`
(AGENTIC_DEVELOPMENT_WORKFLOW.md, TYPES.md), `plans/`, `loop/`,
`validation/evidence/sweeps/` (the sweep record itself), and the ~100 seeded
`execution/**/_STATUS.md` surfaces DO differ in that window (110 piping
paths total, plus chirality-app-dev paths outside this project) and are
excluded from the identity claim.

Structural reads at frozen SHA: crate `src/lib.rs` (28 `#[test]` functions
enumerated by name; three path deps per `Cargo.toml`; no code-combination,
default-factor, rule-pack, envelope, persistence, or compliance surface),
crate README, all kit/coordination surfaces, `Dependencies.csv` (rows
DAG-002-E0459/E0460 SATISFIED, DEL-05-05-E001 downstream interface),
`Review_Findings.csv` (W001/W002 ACCEPT_AS_IS/RESOLVED), `_REVIEW.md`,
frozen `SOFTWARE_DECOMP.md` header (revision 0.8, current_basis),
`IMPLEMENTATION_SURFACES.csv` / `VERIFICATION_INDEX.csv` /
`VALIDATION_AND_PROVENANCE_INDEX.csv` rows for this orbit (re-verified
against the frozen tree where load-bearing).

## 4. Convention friction notes

1. **README material omission vs widened STALE.** Widened convention 1
   covers "post-alignment drift", but a README that is silent about half
   the implemented API (rather than asserting anything false or
   future-tense) sits between STALE_SETUP_SPECIFICATION and
   ALIGNED-at-contract-grain. I went STALE (DECL-007) because the surface's
   job is to describe the crate's scope and it no longer does; a named rule
   for material-omission drift on DECL surfaces would remove the judgment
   call for W3–W5.
2. **Validation homed at another deliverable.** The mechanics validation
   for R7–R10 lives in DEL-09-01's benchmark suite (which imports and calls
   this crate). The conventions say nothing about cross-deliverable
   validation homing; I cited it as validation evidence for this
   deliverable's mechanics rows (the fixtures exercise this crate's code
   directly) with the homing stated in-cell. Worth confirming at fan-in
   that cross-homed validation counts toward the mechanics-class bar.
3. **Addendum-6 ladder with split evidence legs.** When verification
   evidence is human-ruled (REVIEWED-grade) but the load-bearing validation
   leg is agent-generated pending human disposition, the row gets one
   SourceReliability cell. I keyed it to the load-bearing leg (UNVERIFIED
   on R7–R10); see self-flags. A one-line rule ("SourceReliability follows
   the weakest load-bearing evidence leg") would standardize this.
4. **Declared bounded TBDs with no `## Remaining` presence.** This
   deliverable's kit TBDs never became `_STATUS.md` residuals, so the
   ledger has zero REMAINING_WORK rows despite a real recorded deferral
   set. Addendum 5 handles the disposition side, but nothing states where
   the TBD inventory should live in a zero-REM ledger — I put each TBD in
   the `RemainingWork` cell of its governing REQ/EXC row. Consistent with
   sibling ledgers, but worth a named convention.

## 5. Boundary-compliance statement

Fences held. All discovery reads were from the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; no write of any kind was made
under the frozen tree (re-execution used byte-identical scratch copies with
an external `CARGO_TARGET_DIR`; `PYTHONDONTWRITEBYTECODE=1` set for the
generator; `git status --porcelain` on the frozen worktree was empty before
and after all evidence operations). Writes were confined to exactly two run
files: `WAVES/W2/CLAIM_CONCORDANCE_DEL-05-05.csv` and
`WAVES/W2/NOTES_DEL-05-05.md` (generator script kept in the session
scratchpad as `build_csv_DEL-05-05.py`). No lifecycle transition, DAG
mutation, register edit, `_STATUS.md` edit, or cross-project edit. No
release-readiness, issuance, certification, sealing, professional-approval,
or code-compliance claim appears in the ledger or these notes (F-PIP-1..5).
All dispositions are agent judgments routed through `AuthorityNeeded`; none
is phrased as an owner or engineering ruling. The owner suspension remains a
run-level caveat only; `SelectableUnderCurrentLoop` cells are mechanical
derivations per convention 6.
