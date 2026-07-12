# W2 Discovery Notes — DEL-05-03 "Fundamental stress recovery module" (PKG-05)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W2. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-03.csv` (19 claim rows + header).

**NormativeSource path alias (declared once per addendum 12):** bare filenames
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`,
`_DEPENDENCIES.md`, `Dependencies.csv`, `Review_Findings.csv`,
`_run_records/...`) resolve to the deliverable folder
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/`.
All other paths are repo-relative under `projects/chirality-piping/`.

Requirement-ID note: the kit's native scheme is `DEL-05-03-RQ-*`
(self-identifying); ClaimIDs use the addendum-12 form `DEL-05-03-REQ-NNN`
with the kit-native ID recorded in each row's `NormativeSource`
(REQ-NNN ↔ RQ-NNN, 1:1 in order).

Calibration-evidence note: DEL-05-03 was an R0b calibration pilot
(`R0B_CALIBRATION/R0B_CLAIM_CONCORDANCE_DEL-05-03.csv`, 17 rows). That ledger
predates the adopted convention set and is calibration evidence only; this
ledger re-encodes the deliverable from the frozen tree under
`R1_CONVENTIONS.md` plus the W1 calibration items. Material re-encoding
deltas from R0b: the bootstrap `REMAINING_WORK` row (R0b C04) is eliminated
per addendum 2 (verbatim transcription now lives only on the `_STATUS.md`
surface row); the declared-state census grows from 3 to 7 rows per
addendum 1 (adding Procedure, `_STATUS.md`, `MEMORY.md`, and the crate
README — the README census produced a NEW staleness finding, DECL-007); the
R0b `UNMAPPED-DEL-05-03-01` row is not re-minted (see the
IMPLEMENTED_UNMAPPED note below); declared-state rows take
`SourceReliability=NOT_APPLICABLE` per addendum 6; mechanics rows resting on
agent-generated witness validation take `UNVERIFIED` per addendum 6 and the
R1 validation index.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (19 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 16 |
| STALE_SETUP_SPECIFICATION | 2 |
| ACCEPTED_DIVERGENCE | 1 |

ClaimType histogram (19 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 9 |
| ACCEPTANCE | 1 |
| EXCLUSION | 2 |
| DECLARED_STATE | 7 |

Acceptance rows: 1. Unlike the sibling DEL-05-01/DEL-05-02 verification
tables (which only restate requirements and got no mirrored ACCEPTANCE rows),
the Specification `## Verification` row "Hand calculations" states a distinct
acceptance-scope claim of its own — release hand-calc, benchmark publication,
and tolerance acceptance scope remain TBD — so it is ledgered once at
addendum-12 grain as ACC-001 (self-flagged below). The other verification
rows restate requirement hooks and were not mirrored.

REMAINING_WORK rows: 0. The deliverable's `## Remaining` carries only the
seeded `(gated: D-41)` bootstrap item, transcribed byte-exact into
DECL-005's `RecordedRemaining` (verified equal to the frozen `_STATUS.md`
bytes programmatically) and excluded from residual/gate/selectability
analysis per addendum 2; its gate/source cells stay at column defaults per
W1 calibration item 5.

IMPLEMENTED_UNMAPPED rows: 0. Every material surface in this deliverable's
orbit (`core/loads/stress_recovery` SURF-095, `core/loads/primitive_loads`
SURF-094, `core/solver/frame_kernel` SURF-132, `core/solver/straight_pipe`
SURF-139, `core/product_physics` SURF-102, `fixtures/product_preview`
SURF-160, `schemas/analysis_status.schema.yaml` SURF-176) carries a
deliverable attribution in the R1 `IMPLEMENTATION_SURFACES.csv`. The R0b
ledger's `UNMAPPED-DEL-05-03-01` row (the product_physics stress result-row
slice) is therefore not re-minted: SURF-102 attributes the surface to 24
deliverables including DEL-05-03, matching the W1/W2 hygiene practice
(addendum 8 at material-surface grain with attribution present ⇒ no row).
The underlying ownership question — the 2026-06-21 D1/D2 multiplier-row
tranches were recorded under this deliverable's run records but implemented
in the shared `core/product_physics` crate — is noted here for R3, not
ledgered as an unmapped surface.

Declared-state census (addendum 1): four kit surfaces + `_STATUS.md` +
`MEMORY.md` (dated log; ledgered because addendum 1 names it, with dated-entry
drift as in-row notes only) + one deliverable-owned in-tree README
(`core/loads/stress_recovery/README.md`, which names itself "the bounded
implementation slice for `DEL-05-03`") = 7 DECL rows.

SelectableUnderCurrentLoop: 0 YES rows — the only recorded `## Remaining`
item is the excluded bootstrap item, so every row defaults NO per
addendum 12 (no recorded item). This reproduces the R1
`DELIVERABLE_INVENTORY.csv` grain for DEL-05-03
(NonBootstrapRemaining=NONE, SelectableUnderCurrentLoop=NO).

**Owner-calibration caveat (stated once per W1 calibration item 1):** the
Datasheet References section cites `SOFTWARE_DECOMP.md` revision 0.7 while
the frozen decomposition header is revision 0.8, `status: current_basis`.
Encoded STALE-side on the affected DECLARED_STATE surface row (DECL-002,
which also carries the API-enumeration drift) with drift facts in-row,
AuthorityNeeded=NO (pure pointer drift; the Datasheet carries no overtaken
TBD register). `_CONTEXT.md` (Accepted Revision 0.7, and an SCA-001
architecture-basis injection citing revision 0.7) and `_REFERENCES.md`
(Accepted revision 0.7) also carry the stale pointer, and the dated MEMORY
2026-06-04 authority-refresh entry cites "revision 0.7 plus DAG-006" — none
of these is a censused declared-state surface (the MEMORY citation is a
dated historical entry), so they are recorded here as notes only.
`_DEPENDENCIES.md` already cites the current DAG-007 authority.

## 2. Self-flagged rows

- **DEL-05-03-DECL-007** — NEW finding vs R0b, judgment call on grain: the
  crate README's Scope/Verification enumerations omit the station-recovery/
  station-sweep surface (landed 2026-05-17) and the 2026-07-10 modulus-basis
  range record; zero occurrences of "station" in the 60-line README against
  three public station functions in the frozen lib.rs. I read the README as
  an enumerated declaration (same shape as the Datasheet API list adjudicated
  STALE-side in R0b/W1) and encoded widened STALE_SETUP_SPECIFICATION rather
  than ALIGNED-at-contract-grain (W1 calibration item 6). Reviewer eyes
  wanted: the README's opening sentence self-scopes to "element force
  resultants", which a contract-grain reading could call merely incomplete
  rather than no-longer-describing.
- **DEL-05-03-ACC-001** — three stacked judgments: (a) minted an ACCEPTANCE
  row where sibling ledgers minted none, because the "Hand calculations"
  verification row states a distinct open acceptance scope, not a restated
  requirement; (b) ACCEPTED_DIVERGENCE rests on the 2026-06-05
  blocker-closure ruling's acceptance of "explicit and bounded" residual
  TBDs — addendum 11 requires a record that *permits* the deferral, and I
  judged that acceptance a permission; (c) AuthorityNeeded=ENGINEERING (the
  brief's solver-mechanics strictness: tolerance/threshold definition is an
  engineering adjudication) chosen over addendum 13's OWNER routing for
  pending-disposition validation evidence — the disposition rests on the
  ruling, not the pending witnesses, and Confidence is capped MEDIUM either
  way.
- **DEL-05-03-REQ-001 / REQ-008 / REQ-009 (SourceReliability=UNVERIFIED)** —
  these MECHANICS rows rest materially on the hand-calc witness/benchmark
  evidence, which the R1 validation index classifies UNVERIFIED
  (agent-generated, agent-audited, no human disposition); the other REQ rows
  rest on crate evidence covered by the 2026-06-05 named human ruling and are
  REVIEWED. R0b encoded these three REVIEWED; the flip to UNVERIFIED follows
  addendum 6 + the R1 index. Dispositions stay ALIGNED because the
  Specification explicitly declares the validation acceptance scope open
  (plan §6's "explicit statement that validation remains open") — no
  unit-test pass is promoted to engineering validation.
- **DEL-05-03-REQ-003** — AuthorityNeeded=ENGINEERING on an ALIGNED row: the
  conversion-catalog TBD is a declared bounded deferral (addendum 5 substance
  disposition), but resolving it is an engineering-content act, so I routed
  ENGINEERING rather than NO/OWNER. The candidate home DEL-02-02 carries a
  bootstrap-only `## Remaining` (homing failed on both sides, disclosed
  in-row).
- **DEL-05-03-DECL-001** — grain note: the Specification predates the
  2026-07-10 additive modulus-basis extension but no Specification sentence
  is contradicted (behavior-class prose, no exhaustive API enumeration).
  Encoded ALIGNED at contract grain per W1 calibration item 6; stating the
  grain here and flagging the row.

## 3. Evidence-execution log

Re-executed (side-effect-free per addendum 9):

- `cargo test --locked` for `core/loads/stress_recovery`, run in place inside
  the frozen worktree with external
  `CARGO_TARGET_DIR=<scratchpad>/del0503_target` (the committed `Cargo.lock`
  satisfied `--locked`; no out-of-tree copy was needed): **26 passed,
  0 failed, 0 doctests**.
- `cargo test --locked` for `validation/benchmarks/stress`, same pattern:
  **22 passed, 0 failed, 0 doctests**.
- Frozen-tree `git status --porcelain` empty BEFORE and AFTER (re-checked
  after each cargo run and after the addendum-10 diff below); HEAD verified
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- The witness rendering pytest for
  `validation/witness/fixtures/tp_phys_015_section_property_stress_witness.json`
  (`tests/test_calculation_witness.py`; deterministic-rendering gate) — cited
  on REQ-001/ACC-001 as recorded; not re-run because I could not rule out
  generated-file writes into the frozen tree.
- Per-tranche recorded passes in deliverable `_run_records/` (TP-PHYS-001
  13 tests, TP-PHYS-003-B 18, TP-PHYS-004-D 20, TP-PHYS-007-B 22,
  force-per-length fan-in 24 locked; 2026-06-21 D1/D2 product-physics/
  Vitest/build/Playwright sets) — historical states, cited on REQ-006 and
  DECL-006.
- DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (cargo_crate_sweep=pass, clean tree) at commit
  `e648462f1d0521e26df15d04a988391343018886`, ancestor of the frozen SHA
  (R1 `VERIFICATION_INDEX.csv` row RUST-05, stress_recovery 26 tests) —
  corroboration only; the primary crate evidence in this ledger is the
  in-place re-execution above.

Addendum-10 qualifier — diff actually run:
`git diff --stat e648462f1d0521e26df15d04a988391343018886..551f84ef6be656f1603ce0acfa5e3935aa9683c7`
over `projects/chirality-piping/{core/loads/, core/product_physics/,
validation/benchmarks/stress/, validation/hand_calcs/stress/,
validation/witness/}` returned **empty**. Explicit exclusion clause carried
wherever the qualifier appears: the full window touches 287 files — repo-root
`docs/`, `projects/chirality-piping/docs/`, and the seeded
`execution/**/_STATUS.md` surfaces DO differ and are excluded from the
identity claim.

Structural reads at frozen SHA: crate `src/lib.rs` (26 `#[test]` functions
counted; public API located at lines 490/542/556/580/696/784;
`StressRangeModulusBasisRecord` line 384), crate README (60 lines; zero
station/sweep/modulus mentions), all kit/coordination surfaces,
`Dependencies.csv` (12 rows: 2 ANCHOR + 10 EXECUTION, incl.
E0454/E0455/E0456/E0458 interface rows), `Review_Findings.csv` (W001/W002
ACCEPT_AS_IS/RESOLVED), `_REVIEW.md` (PKG-02 audit + lifecycle-readiness
review + Gate 5 addendum), frozen `_REGISTER.md` rows D-38/D-41 (both
AWAITING_RULING at frozen SHA; the RUN_BASIS ruling-after-freeze codification
covers D-41), frozen `SOFTWARE_DECOMP.md` header (revision 0.8,
current_basis), K-CONFLICT-1 `Decision_Log.md` and the 2026-06-05 ruling
packet/review snapshot (existence verified),
`core/product_physics/src/lib.rs` line 6462 ("base frame stiffness
unchanged" provenance string still emitted, cited in DECL-006's dated-drift
note).

## 4. Convention friction notes

1. **README staleness vs contract grain.** Addendum 1 pulls deliverable-owned
   READMEs into the declared-state census, and addendum 4 widens STALE to
   "no longer describes the frozen implemented slice" — but neither says how
   *complete* an enumerated README must be before omission counts as
   staleness. DECL-007 is the boundary case (self-flagged); a one-line rule
   ("an enumerated scope/API list missing a landed public surface is stale;
   narrative prose is judged at contract grain") would converge pilots.
2. **ACCEPTANCE-row grain inside verification tables.** Addendum 12 excludes
   mirrored rows for requirement-restating tables but does not positively
   define what earns an ACCEPTANCE row; I used "states a distinct
   acceptance criterion or acceptance-scope of its own" (ACC-001). Worth a
   named rule before W3-W5 hit validation-class packages.
3. **AuthorityNeeded on ALIGNED rows carrying declared TBDs.** The column
   contract allows ENGINEERING/OWNER on any row, but nothing says whether a
   declared bounded deferral routes the authority that would *resolve* it
   (my reading: yes — REQ-003 ENGINEERING, REQ-005 OWNER, ACC-001
   ENGINEERING) or NO because the row itself needs no adjudication. The
   fan-in should harmonize this across the wave.
4. **Addendum 13 vs solver-mechanics ENGINEERING routing.** When
   pending-disposition validation evidence (OWNER per addendum 13) coincides
   with a numeric-tolerance question (ENGINEERING per the W2 brief), one
   cell must win; I chose ENGINEERING on ACC-001 and disclosed the conflict
   in-row and above. A precedence note would remove the judgment call.
5. **In-place vs copy re-execution.** Addendum 9's blessed pattern (external
   CARGO_TARGET_DIR, in-place) worked here because the crates commit their
   `Cargo.lock`; the DEL-05-02 pilot needed the byte-identical-copy pattern.
   Both are now exercised in W2 — the fan-in may want to note both as
   conforming.

## 5. Boundary-compliance statement

Fences held. All discovery reads were from the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; no file in the frozen worktree
was created, modified, or deleted (the two cargo re-executions ran with an
external CARGO_TARGET_DIR; `git status --porcelain` on the frozen worktree
was empty before and after every execution and diff). Writes were confined
to exactly two run files: `WAVES/W2/CLAIM_CONCORDANCE_DEL-05-03.csv` and
`WAVES/W2/NOTES_DEL-05-03.md` (plus the deliverable-named generator script
`build_csv_DEL-05-03.py` in the session scratchpad, outside both trees). No
lifecycle transition, DAG mutation, register edit, or product-file edit. No
release-readiness, issuance, certification, sealing, professional-approval,
or code-compliance claim is made anywhere in these outputs (F-PIP-1..5).
All dispositions are agent judgments routed through AuthorityNeeded, never
owner or engineering rulings. No agent-workflow change is proposed; no
DEFERRED_AGENT_WORKFLOW rows were needed.
