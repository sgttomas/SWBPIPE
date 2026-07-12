# NOTES — DEL-04-03 Linear support and restraint models (R2 wave W2)

Pilot: fable discovery pilot (W2, PKG-04, per Receipt-17 all-fable steer).
Basis: `R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13) plus the W1
cross-ledger calibration items (`PACKAGE_SUMMARIES/PKG-00..03.md`). All
discovery reads from the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

**NormativeSource path alias (addendum 12, declared once):** unqualified
paths in the ledger resolve relative to
`FROZEN/projects/chirality-piping/`; bare kit filenames (`Specification.md`,
`Datasheet.md`, `Guidance.md`, `Procedure.md`, `_STATUS.md`, `MEMORY.md`,
`_CONTEXT.md`, `Review_Findings.csv`, `_run_records/…`) resolve inside
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/`.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (23 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 19 |
| STALE_SETUP_SPECIFICATION | 4 |

ClaimType histogram:

| ClaimType | Count |
|---|---|
| REQUIREMENT | 12 |
| EXCLUSION | 3 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 1 |

ACCEPTANCE = 0: the Specification has no separate acceptance-criteria
section; its Verification table restates the R01–R12 requirements
row-for-row, so no mirrored ACCEPTANCE rows are ledgered (addendum-12
grain, consistent with the W1 PKG-02/PKG-03 encoding).
IMPLEMENTED_UNMAPPED = 0: the deliverable's material surface
(`core/solver/linear_supports`, SURF-133) is deliverable-attributed in the
R1 index and its kit; the DEC-049 hanger user-data slice lives inside
already-mapped shared surfaces (`core/product_physics` SURF-102, desktop
SURF-005) whose slice grain is ledgered by their attributed deliverables.

Selectability: 2 YES rows (DECL-005, REM-001 — the mechanically ungated
DEC-049 constant-effort residual), matching the R1 inventory's
inventory-grain YES; all other rows NO per the addendum-12 default. The
owner suspension is a run-level caveat (RUN_BASIS), never per-row.

## 2. Self-flagged rows

- **DEL-04-03-DECL-001 / DECL-002 / DECL-003 / DECL-004** — judgment call
  on staleness grain. All four kit surfaces declare the implemented slice
  as the `core/solver/linear_supports` crate only, while the frozen
  implemented slice also includes the DEC-049 D5 spring-hanger user-data
  extension landed 2026-06-21 ("bounded DEL-04-03 user-data extension" per
  PRD plan §5 risk row; `_STATUS.md` Remaining item 1; MEMORY 2026-06-21
  entry). I encoded post-alignment drift → STALE_SETUP_SPECIFICATION on all
  four under widened addendum 4, drift facts in-row, AuthorityNeeded=NO
  (repair candidates; no overtaken TBD register found — Datasheet TBDs
  remain genuinely open). A reviewer could instead read Guidance/Procedure
  as bounded-purpose documents that stay ALIGNED for their declared slice;
  I chose the consistent kit-wide encoding because the W1 fan-in adjudicated
  the under-staling side defective on textually identical drift. Grain
  used: whole-deliverable implemented slice, not crate-only slice.
- **DEL-04-03-REQ-004** — addendum-13 application: the only open review
  finding (PKG04-DEL0403-PKG02-001, WARNING) rests on this claim and is
  TECHNICALLY_ADDRESSED_PENDING_HUMAN with HumanDisposition TBD. Disposition
  ALIGNED rests on independent grounds (direct code inspection + re-executed
  tests), Confidence capped MEDIUM, AuthorityNeeded=OWNER. Reviewer eyes
  welcome on whether the cap applies when the pending record is a
  *review-finding resolution* rather than validation evidence proper.
- **DEL-04-03-REQ-004 — post-verification string correction (2026-07-12).**
  Per the W2 fan-in verification (`W2_VERIFICATION_PKG-04.md` §3.1,
  QUALIFIED, string correction recommended), the ValidationEvidence cell's
  transcribed pointer to
  `execution/_Reconciliation/Reviews/DEV001_FINDING_RESOLUTION_PKG02_GROUNDED_2026-05-16/RESOLUTION_MATRIX.csv`
  does not resolve anywhere in the frozen tree (absence re-verified by this
  owning pilot with a full-tree `find` before editing). The path was
  transcribed from the DEL-04-03 kit MEMORY.md ("Resolution evidence is
  indexed in ..."), so the convention-7 attestation marker
  `ATTESTED: DEL-04-03 kit MEMORY.md, record not present in tree at frozen
  SHA 551f84ef6` was appended inside that cell's parenthetical, directly
  after the path. No other cell or row was touched; the row's disposition
  (ALIGNED), Confidence (MEDIUM), and AuthorityNeeded (OWNER) are unchanged,
  as the disposition rests on the in-tree Review_Findings.csv record and
  independent code inspection + re-executed tests, not on the matrix.
  Histogram recount: the edit touched only citation prose in
  ValidationEvidence — no ClaimType, LifecycleState, disposition, gate, or
  selectability field changed, so the §1 histograms are unaffected.
  Frozen-tree porcelain re-checked empty after the correction.
- **DEL-04-03-REQ-009** — process-discipline claim ("before release use"):
  no release exists, so the conditional is unexercised; encoded ALIGNED /
  MEDIUM mirroring the R0b DEL-05-03 RQ-006 pattern.
- **DEL-04-03-REQ-010** — DEC-050 has since ruled a sparse evidence lane
  (dense default) at package level; I judged the linear crate's own sparse
  integration point still open, so the requirement's remain-TBD posture
  holds (ALIGNED, not STALE/overtaken). Grain stated in-row.
- **DEL-04-03-REQ-012** — brief calibration item 6 (capability-gap grain):
  encoded ALIGNED at contract grain — the requirement claims the two
  surfaces are "the current implemented surfaces," which is true; limited
  validation breadth is a kit-declared open limitation, not a gap in this
  claim. Grain used: contract grain.

## 3. Evidence-execution log

Re-executed (side-effect-free, addendum 9):

- `cargo test` for `open_pipe_stress_linear_supports`: **14/14 PASS**.
  Mechanics of the run: an in-place frozen-tree run with `--locked` was
  attempted first and **declined by cargo** (it demanded a `Cargo.lock`
  update, which would have written into the frozen tree), so the crate and
  its sole path dependency were byte-copied out (`cp -R` of
  `core/solver/linear_supports/` and `core/solver/frame_kernel/` to pilot
  scratch) and the suite ran there with an external `CARGO_TARGET_DIR` and
  `--offline`. Frozen-tree `git status --porcelain` was **empty before and
  after** both the declined in-place attempt and the copy run.
- `git diff e648462f1d05…551f84ef6` over `core/solver/linear_supports/` and
  `core/solver/frame_kernel/`: **empty** (read-only; ancestry of
  `e648462f1d05` verified with `git merge-base --is-ancestor`). This is the
  basis for the addendum-10 qualifier on the RUST-27 sweep citation —
  scoped to exactly those two paths, with the sweep itself marked
  `not re-executed at frozen SHA 551f84ef6`.

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- RUST-27 / RUST-34 sweep `SWEEP_20260711T040758Z_e648462f1d05.json`
  (cargo crate sweep pass at ancestor commit `e648462f1`).
- MEMORY/run-record verification history: 2026-06-05 hardening + fan-in
  (linear_supports 14, frame_kernel 33/34), TP-R4-D5-HANGERDATA-001
  validations (product_physics 43/43, desktop 407/407, Playwright 18/18,
  DEC-025 sweep `SWEEP_20260621T202442Z_4829dea6c2e0-dirty.json`).
- Protected-content / prohibited-claim scans recorded in MEMORY
  (2026-05-01; 2026-06-05 fan-in).
- Hand-calc witness HC-MECH-007
  (`validation/hand_calcs/mechanics/tp_phys_002_linear_static_integration.md`)
  read directly at the frozen SHA; its consuming benchmark suite (RUST-34)
  cited as recorded.

Frozen-tree porcelain: empty at pilot start, after the declined `--locked`
attempt, after the copy re-execution, and at pilot end.

## 4. Convention friction notes

1. **Bootstrap gate/source cells vs multi-residual joining (addendum 2/3 vs
   W1 calibration item 5):** with one real residual plus the bootstrap item
   in `## Remaining`, I transcribed both items byte-exact into the
   `_STATUS.md` surface row's `RecordedRemaining` (semicolon-joined in
   `## Remaining` order, matching the W1 DEL-02-05 encoding) but kept
   `GateOrStageConstraint`/`RemainingSource` scoped to the real residual
   only (`UNGATED` / PRD plan §3 D5 row / DEC-049), per the W2 brief's
   byte-exact-bootstrap calibration ("gate/source cells stay at column
   defaults"). The W1 DEL-02-05 ledger instead annotated the bootstrap's
   gate value in that cell — the two waves now differ on this cell's shape;
   R3 aggregation should pick one.
2. **"Deliverable-owned in-tree README" census** includes the crate README
   (`core/solver/linear_supports/README.md`), following the W1 DEL-02-02
   DECL-007 precedent; the census phrase itself doesn't say whether crate
   READMEs on *shared* surfaces (e.g. `core/product_physics/README.md`, if
   any) would join the census of every attributed deliverable — not
   exercised here.
3. **Addendum-9 gap:** the side-effect-free definition covers redirecting
   build artifacts, but not the case where cargo itself refuses `--locked`
   and any in-place run would rewrite `Cargo.lock`. I treated a byte-copy
   re-execution outside the tree (content equality guaranteed by `cp -R`
   from the frozen tree) as satisfying the re-execution bar and said so
   in-row; a convention blessing or banning this pattern would help W3–W5.
4. **Hanger-slice homing:** DEC-049 work is recorded under DEL-04-03 but
   implemented in shared surfaces owned at slice grain by several
   deliverables (product_physics, desktop). Convention 8's named-slice
   grain handled it, but the kit-staleness consequence (friction note in
   §2 self-flags) is the kind of cross-surface drift the conventions leave
   to pilot judgment.

Owner-calibration caveat (noted once per the W1 adjudication): the
Datasheet rev-0.7 → 0.8 authority-pointer drift on DECL-002 follows the
corpus-wide W1 encoding (STALE-side under addendum 4, drift facts in-row,
SOFTWARE_DECOMP §13 "stale … until refreshed" carried as immateriality
context). If the owner later calibrates the corpus-wide pattern to
ALIGNED-with-note, DECL-002's rev-drift component flips mechanically; its
hanger-slice drift component (shared with DECL-001/003/004) is independent
of that calibration.

## 5. Boundary-compliance statement

All fences held. Discovery reads were confined to the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; the frozen tree porcelain was
verified empty before and after every evidence action, including the
declined in-place cargo attempt (which wrote nothing). Writes were confined
to exactly two files:
`RUN/WAVES/W2/CLAIM_CONCORDANCE_DEL-04-03.csv` and this notes file (plus
pilot-scratch artifacts outside both trees). No lifecycle transition, DAG
mutation, register edit, product-file edit, or cross-project edit was made
or proposed as operative. No release-readiness, issuance, certification,
sealing, professional-approval, or code-compliance claim appears in these
outputs (F-PIP-1..5); quoted negative/boundary statements are attributed to
their sources. All dispositions are this agent's judgments, never owner or
engineering rulings; authority is routed only through `AuthorityNeeded`
(one OWNER routing: REQ-004).
