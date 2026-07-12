# NOTES — DEL-09-01 Mechanics benchmark suite (R2 wave W4)

Deliverable: **DEL-09-01** (PKG-09 Verification, Validation, and Quality Oracles),
status IN_PROGRESS. Frozen source tree pinned SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Discovery pilot role/model:
**GPT-5 discovery pilot**; no model substitution occurred. Ledger:
`CLAIM_CONCORDANCE_DEL-09-01.csv` (25 rows, 20 columns, RFC-4180 CRLF).

Run-level `NormativeSource` aliases (R1 addendum 12): kit filenames are relative
to `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/`;
`SOFTWARE_DECOMP.md` means `execution/_Decomposition/SOFTWARE_DECOMP.md`; benchmark,
hand-calculation, core, schema, fixture, and evidence paths are repository-root
relative. All discovery facts were checked against the frozen worktree. R0/R0b
artifacts were used only for calibration; this is an ordinary W4 re-encoding.

## 1. Histograms (recounted from the CSV)

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 7 |
| ACCEPTANCE | 7 |
| EXCLUSION | 1 |
| DECLARED_STATE | 8 |
| REMAINING_WORK | 2 |

| Disposition | Count |
|---|---:|
| ALIGNED | 16 |
| PARTIALLY_IMPLEMENTED | 3 |
| STALE_SETUP_SPECIFICATION | 4 |
| ACCEPTED_DIVERGENCE | 1 |
| REMAINING_STATE_MISMATCH | 1 |

AuthorityNeeded: **NO 18 / OWNER 5 / ENGINEERING 2**. Selectability:
**YES 3 / NO 22**. The YES rows are DECL-005 and REM-001 for the ungated
non-bootstrap PRD §16.2 residual, plus ACC-007 because it touches that same
recorded residual. The seeded `(gated: D-41)` bootstrap item is quoted only in
DECL-005 `RecordedRemaining` and excluded from residual/gate/selectability
analysis.

## 2. Census and final-convention re-encoding

- **REQUIREMENT 7:** one row for each native Specification requirement
  DEL-09-01-RQ-001..007, re-keyed to addendum-12 IDs REQ-001..007. Every row
  carries a substance disposition; setup-era prose staleness is not encoded on
  requirement rows.
- **ACCEPTANCE 7:** the seven distinct Verification/Documentation expectations
  are retained at their own acceptance grain rather than merged into
  requirements. ACC-001..006 mirror the six Verification areas; ACC-007 covers
  the separately stated expected artifact set.
- **EXCLUSION 1:** EXC-001 preserves the durable separation from production
  solver modification and protected/proprietary benchmark sources. Setup-session
  write prohibitions overtaken by later authorized tranches are declaration
  drift, not durable exclusions.
- **DECLARED_STATE 8:** exactly four kit documents (DECL-001..004), `_STATUS.md`
  (DECL-005), `MEMORY.md` (DECL-006), and the two deliverable-owned in-tree
  READMEs (DECL-007 benchmark crate, DECL-008 hand calculations). This replaces
  the calibration ledger's combined-README row and duplicate Specification
  declaration. The Specification's fixture-schema TBD is consolidated into
  DECL-001, so no second declaration row is minted for the same surface.
- **REMAINING_WORK 2:** REM-001 is the recorded §16.2 evidence-system residual.
  REM-002 is finding PKG09-0901-PKG02-001, technically addressed but awaiting
  human disposition and absent from every checked candidate `_STATUS.md` home.
- **IMPLEMENTED_UNMAPPED 0:** the benchmark crate, hand-calculation family, and
  related production/verification surfaces are mapped in the R1 implementation
  index; no material unmapped surface was found for this deliverable.

The four kit documents still describe future/setup-only work while the frozen
tree contains the implemented mechanics benchmark slice, so DECL-001..004 are
`STALE_SETUP_SPECIFICATION`. DECL-005..008 are current-state declarations and
are ALIGNED. Dated MEMORY entries are historical records: their older counts or
rev-0.7/DAG-006 pointers are disclosed as notes, not stale declarations.

## 3. Required 21-fixture repair

The W4 ledger uses **21 fixtures and 21 hand-calculation witness notes**
throughout. Direct inspection confirms:

- `validation/benchmarks/mechanics/src/lib.rs` `fixture_inventory()` has 21
  entries and its own test asserts length 21;
- `validation/hand_calcs/mechanics/README.md` has 21 fixture-to-note rows; and
- the directory has 21 witness-note Markdown files plus its README (22 Markdown
  files total, the calibration miscount source).

This applies R1 Part C's named repair without modifying the calibration ledger.
No disposition changes from the correction.

## 4. Evidence posture and calibration carry-forward

No tests, cargo commands, pytest, Python compilation, or other executable checks
were run by this W4 pilot. Recorded test evidence is clearly attributed: the
ledger cites R0b review §3 checks 18–19, whose independent reviewer ran the
mechanics crate at the same pinned SHA with external `CARGO_TARGET_DIR` (33
passed / 0 failed / 0 ignored) and verified the cited content-identity diff.
Cells say `not re-executed by the W4 pilot`; no recorded pass is presented as a
new W4 execution.

Binding W1–W3 calibration applied:

- weakest-load-bearing-leg SourceReliability is UNVERIFIED for technical rows;
  all DECLARED_STATE prose rows are NOT_APPLICABLE;
- setup-era false factual declarations are STALE only on census declaration
  surfaces; requirement substance remains independently adjudicated;
- the rev-0.7/DAG-006 pointer drift is recorded on the affected historical/
  declaration context rather than propagated as requirement staleness;
- README census is one row per self-identifying deliverable-owned README;
- all-dated MEMORY remains in the census with a historical-record note;
- ACCEPTED_DIVERGENCE is used only for REM-001 because DEC-054 explicitly
  carries the bounded §16.2 residual forward;
- REM-002 pending-disposition evidence is MEDIUM confidence and OWNER-routed
  under addendum 13; and
- AuthorityNeeded routes adjudication, not the performance of ordinary work.

The R0b calibration's ATTESTED marker for the missing DEV-001 dispatch record is
retained wherever it is load-bearing. The exact fixture-schema home remains a
small R3 check: determine whether REM-001 subsumes it or a schema-owner residual
is omitted. No authority conflict was found; the frozen D-41 AWAITING_RULING
state and later live activation are the recorded ruling-after-freeze sequence,
not contradictory authority.

## 5. Self-flagged rows for package fan-in

- **REQ-004 / ACC-004:** fixture-local units are implemented and checked, while
  project-catalog-grain unit authority remains unresolved; both are
  PARTIALLY_IMPLEMENTED, MEDIUM, ENGINEERING.
- **ACC-007:** expected artifact set is partly landed and partly represented by
  REM-001/homed runner and threshold residuals; selectability YES follows the
  touched ungated residual.
- **DECL-001:** combines the Specification's stale setup framing and its still-
  unresolved fixture-schema declaration under the mandatory one-row-per-surface
  census. STALE is the surface disposition; the schema-home question remains in
  RemainingWork rather than a duplicate DECL row.
- **REM-001:** ACCEPTED_DIVERGENCE rests on DEC-054's explicit carry-forward,
  not merely on the presence of a recorded residual.
- **REM-002:** REMAINING_STATE_MISMATCH is used because the pending human
  disposition is evidence-backed but absent from the sole `_STATUS.md` work
  surface after the bounded candidate-home search.
- **ACC census (7):** verification-table expectations are distinct acceptance
  statements, but their near-mirroring of requirements is a known package-
  histogram grain sensitivity for the verifier/R3.

## 6. Addendum-9 containment and fence attestation

Before discovery and after artifact generation, `git status --short
--ignored=matching` in the frozen worktree showed exactly the six allow-listed
pre-existing ignored paths and no others:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The frozen worktree was not modified or cleaned. Because nothing was
re-executed, the lockless-crate copy-out, `pytest -p no:cacheprovider`, and
no-in-tree-`py_compile` rules were not invoked and no new artifact was created.
Writes were limited to this ledger and notes file in W4. No deliverable,
product, lifecycle, DAG, register, decision, or cross-project surface was
edited. Dispositions are discovery-agent judgments only, never owner or
engineering rulings; no lifecycle transition or R4/R5 repair is proposed as
operative.

