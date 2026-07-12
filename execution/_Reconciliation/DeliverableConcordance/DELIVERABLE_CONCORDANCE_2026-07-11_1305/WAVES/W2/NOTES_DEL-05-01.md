# W2 Discovery Notes — DEL-05-01 "Primitive load case engine" (PKG-05)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W2. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-01.csv` (25 claim rows + header).

**NormativeSource path alias (declared once per addendum 12):** bare filenames
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REVIEW.md`, `Review_Findings.csv`)
resolve to the deliverable folder
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/`.
All other paths are repo-relative under `projects/chirality-piping/`.

**Requirement-ID form:** the kit's native scheme is `REQ-05-01-001..012`
(self-identifying); ClaimIDs use the addendum-12 fixed form
`DEL-05-01-REQ-NNN` with the kit-native ID recorded in each row's
`NormativeSource`.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (25 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 19 |
| STALE_SETUP_SPECIFICATION | 6 |

ClaimType histogram (25 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 12 |
| EXCLUSION | 5 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 1 |

Acceptance rows: 0. The Specification "Verification" table restates the twelve
requirements at test-coverage grain (no distinct acceptance criteria, expected
values, or tolerances of its own), and `_REVIEW.md` AC-001..012 mirror the
same requirements; per addendum 12 no mirrored ACCEPTANCE rows were minted.
IMPLEMENTED_UNMAPPED rows: 0 — every material surface in this deliverable's
orbit (primitive_loads, user_loads, solver diagnostics, product_physics,
model schema, load-cases desktop panel, mechanics benchmarks,
section_properties) carries a deliverable attribution in
`IMPLEMENTATION_SURFACES.csv` (SURF-094/096/102/124/131/027/190/226 et al.).

Declared-state census (addendum 1): four kit surfaces + `_STATUS.md` +
`MEMORY.md` (both carry current-state declarations) + one deliverable-owned
in-tree README (`core/loads/primitive_loads/README.md`) = 7 DECL rows.

**Owner-calibration caveat (stated once per W1 calibration item 1):** the kit
cites SOFTWARE_DECOMP revision 0.7 (`Datasheet.md` References;
`_CONTEXT.md` Decomposition Reference / Architecture Basis Injection;
`_REFERENCES.md`) while the frozen decomposition header is revision 0.8,
`status: current_basis`. Encoded STALE-side on the affected DECLARED_STATE
surface row (DECL-002) per the W1 fan-in adjudication; `_CONTEXT.md` and
`_REFERENCES.md` are not censused declared-state surfaces, so their rev-0.7
citations are recorded here as a note only.

**Dominant drift fact for this ledger:** the kit was last aligned in the
2026-06-05 review-readiness tranche (40-test crate surface), while the frozen
slice includes the 2026-07-10 `TP-PMM-P3-OCCLOADGEN-001` tranche
(DEC-068 item 2): static-equivalent seismic/wind load **generators** from
user-entered parameters (`generate_seismic_equivalent_static_loads` /
`generate_wind_equivalent_static_loads`), two new finding codes, schema slot
`LoadCase.equivalent_static_generation`, and a 45-test crate suite. Five of
the six STALE dispositions trace to that single overtaking tranche; MEMORY
additionally lacks the closeout entry Procedure step 10 expects.

## 2. Self-flagged rows

- **DEL-05-01-REQ-008** — the requirement's "shall accept only explicit
  equivalent mechanics loads … unless a later sealed scope authorizes dynamic
  procedure generation" predates the DEC-068 generators. Encoded ALIGNED at
  MEDIUM confidence on the reading that (a) the acceptance boundary
  (`prepare_equivalent_static_loads`) is unchanged, (b) the generators are
  static-equivalent, user-parameter mechanics under a named human ruling, and
  (c) dynamic procedure generation remains absent (D-12). A stricter wording
  reading could argue IMPLEMENTED_DIFFERENTLY; the prose gap itself is
  ledgered on DECL-001/002.
- **DEL-05-01-REQ-009** — ClaimClass VALIDATION per convention 7
  (coverage/harness claim); the in-kit "focused 40-test" figure vs the frozen
  45-test suite is treated as declaration staleness (DECL-001/002), not a
  substance gap, because the coverage areas the requirement names are all
  present and pass.
- **DEL-05-01-DECL-001** — OWNER routing rests on reading the Specification
  "Remaining TBDs" list as a TBD register with one partially overtaken row
  ("Wind/seismic dynamic treatment, occasional-event mapping, and any future
  lawful procedure generators" — static-equivalent generation landed under
  DEC-068 while dynamics stays open). Same "TBD register grain" ambiguity the
  DEL-04-01 W2 ledger flagged.
- **DEL-05-01-DECL-003 / DECL-004** — Guidance and Procedure staleness is an
  omission call (landed generation surface absent from otherwise-accurate
  boundary prose), not a false-statement call like the 40-test figures;
  encoded STALE at MEDIUM under the widened addendum-4 definition. A
  reviewer could defensibly hold either at ALIGNED-with-note.
- **DEL-05-01-DECL-006** — MEMORY: the undated header blocks declare
  "represented only as user-supplied equivalent mechanics loads … generation
  remains TBD" and, unlike the DEL-04-01 DECL-006 precedent (kept ALIGNED
  because later dated entries corrected the block in-file), nothing in this
  file corrects it — the 2026-07-10 tranche has a `_run_records` entry but no
  MEMORY entry at all. Encoded STALE; the addendum-1 dated/undated boundary
  leaves this open.
- **DEL-05-01-DECL-007** — crate README staleness: the Scope list omits the
  generators and generation finding codes added in the same crate. One could
  argue the Boundary section's "does not provide … wind/seismic procedures"
  still reads true for code-content procedures; the scope-list omission of a
  material public API drove the STALE call.

## 3. Evidence-execution log

Re-executed (side-effect-free, addendum 9):

- `git -C <FROZEN> status --porcelain` — **empty before and after** all
  evidence operations (checked at discovery start, after the copy/diff
  operations, and after test re-execution); frozen HEAD verified
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
- `cargo test --offline` on the primitive_loads crate: the crate has no
  committed `Cargo.lock`, so running cargo against the frozen manifest would
  have written a lock file into the frozen tree. Following the W2 DEL-04-01
  scratch-copy pattern, `core/loads/primitive_loads` plus its two path
  dependencies (`core/solver/frame_kernel`, `core/solver/linear_supports`)
  were copied to the session scratchpad preserving relative layout, verified
  byte-identical to the frozen tree (`diff -r` clean on all three), and
  tested there with an external `CARGO_TARGET_DIR`:
  **45 unit tests passed, 0 failed** (0 doctests). Frozen tree untouched.
- Addendum-10 qualifier diff, actually run:
  `git diff --stat e648462f1d0521e26df15d04a988391343018886..551f84ef6be656f1603ce0acfa5e3935aa9683c7 -- projects/chirality-piping/core/loads/ projects/chirality-piping/core/solver/ projects/chirality-piping/core/product_physics/ projects/chirality-piping/validation/benchmarks/ projects/chirality-piping/validation/hand_calcs/ projects/chirality-piping/schemas/`
  → **empty** (content-identical over exactly those six path roots; no
  exclusion clause needed; no other paths are claimed under the qualifier).
- Byte-exact `RecordedRemaining` transcription: the CSV builder script reads
  the two `## Remaining` items directly from the frozen `_STATUS.md` and
  asserts the bootstrap item's presence byte-for-byte (no transliteration;
  W1 calibration item 5).

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- DEC-025 sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (clean tree, commit `e648462f1d05…`, ancestor of the frozen SHA) for the
  crate suite (`VERIFICATION_INDEX.csv` RUST-04, 45 static tests), carried
  with the addendum-10 content-identical qualifier above.
- Historical recorded passes in MEMORY.md/_REVIEW.md (14→20→25→28→32→35→40
  test progression; fmt checks; 2026-06-05 Gate 5 review validation) and the
  TP-PMM-P3-OCCLOADGEN-001 run record's recorded suite results
  (product_physics 66, solver diagnostics 24, mechanics benchmarks 30,
  pytest 376) — recorded evidence only; the primitive_loads suite is
  superseded for currency by the re-execution above.
- Hand-calc witnesses HC-MECH-006/HC-MECH-021 and benchmark fixtures
  MECH-PRIMITIVE-LOAD-PREP / MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC at
  DEC-024/DEC-026 tiers — cited from `VALIDATION_AND_PROVENANCE_INDEX.csv`
  and verified present in the frozen tree; not re-derived.

DecisionBasis resolvability: all cited ruling/record paths verified present
in the frozen tree — `D-36_RULING_2026-07-09.md`, SOFTWARE_DECOMP §12
DEC-068 codification, `LIFECYCLE_CORRECTION_2026-07-02_2050/Decision_Log.md`,
`REV_DEL-05-01_2026-06-05_2021/`. No ATTESTED-level basis was needed.
`Review_Findings.csv` is header-only (zero findings), so the W1
overtaken-review-prose pattern (calibration item 3) does not arise here.

## 4. Convention friction notes

- **Authorized scope extension vs requirement wording (REQ-008):** §7 has no
  disposition for "implementation lawfully extended beyond the requirement
  sentence under a named ruling while the requirement's own constraint still
  holds". ALIGNED + in-row note + DECL staleness is the encoding used here;
  ACCEPTED_DIVERGENCE does not fit (nothing is deferred) and
  IMPLEMENTED_DIFFERENTLY overstates (the required behavior is intact). A
  convention line for ruled scope-extensions would remove the judgment.
- **Missing MEMORY closeout as a staleness driver (DECL-006):** addendum 1
  handles drift *inside* MEMORY entries, but not the case where the latest
  tranche never wrote its expected MEMORY entry, leaving the file's header
  declarations uncorrected. Worth a W3+ harmonization line (encoded STALE
  here; the DEL-04-01 precedent kept a corrected-in-file block ALIGNED).
- **"TBD register" grain (repeat of the DEL-04-01 note):** OWNER routing on
  DECL-001 again required deciding whether a Specification "Remaining TBDs"
  bullet list is a register. A one-line definition would remove the judgment.
- **Missing Cargo.lock vs addendum 9 (repeat):** the byte-identical
  scratch-copy pattern (copy crate + path deps, `diff -r` verify, external
  target dir, porcelain check) was reused from the W2 DEL-04-01 ledger and
  worked cleanly; recommend adopting it as the standard W2+ workaround.
- **Cross-deliverable residual visibility:** DEL-04-01's `_STATUS.md` carries
  a §5 completion-verification residual naming DEL-05-01; this deliverable's
  own `_STATUS.md` does not mirror it. Per addendum 5's bounded homing that
  is not a finding here (the item is homed at DEL-04-01 and gated on owner
  re-disposition); recorded for the W2 fan-in's cross-ledger view.

## 5. Boundary-compliance statement

All fences held. Discovery reads were confined to the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` plus the run's own R1/W1/W2
artifacts; the frozen tree porcelain was empty before and after all evidence
operations (the cargo re-execution ran on a scratchpad copy with an external
target directory). No lifecycle transition, DAG mutation,
register/_STATUS/product-file edit, or cross-project edit was made. No
release-readiness, issuance, certification, sealing, professional-approval,
or code-compliance claim appears in these outputs (F-PIP-1..5); all
compliance-adjacent text is quoted exclusion/disclaimer language. All
dispositions are agent judgments routed through `AuthorityNeeded`, never
owner or engineering rulings. Writes were confined to exactly two files:
`WAVES/W2/CLAIM_CONCORDANCE_DEL-05-01.csv` and `WAVES/W2/NOTES_DEL-05-01.md`
(the CSV generator script `build_csv_DEL-05-01.py` lives in the session
scratchpad, outside the repo, uniquely named per W1 calibration item 7).
`SelectableUnderCurrentLoop` cells are mechanical DAG/lifecycle/gate
derivations only (IN_PROGRESS lifecycle, DAG-007 upstream dependencies
SATISFIED, residual UNGATED → YES on DECL-005/REM-001; NO elsewhere per
addendum 12); the owner suspension remains the run-level caveat.
