# NOTES — DEL-08-04 Result export format (R2 wave W3)

Deliverable: **DEL-08-04** (PKG-08 Reporting, Audit, and Reproducibility), status
IN_PROGRESS. Frozen source tree pinned SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ledger: `CLAIM_CONCORDANCE_DEL-08-04.csv` (23 rows, 20 columns, RFC-4180 CRLF, header
byte-exact to the DEL-08-01 exemplar).

Run-level `NormativeSource` aliases used in the ledger (addendum 12): `SOFTWARE_DECOMP.md`
= `execution/_Decomposition/SOFTWARE_DECOMP.md`; `ScopeLedger.csv` / `Deliverables.csv` /
`ContextBudgetQA.csv` = `docs/_Registers/*`; kit filenames are relative to the deliverable
folder `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/`;
`<DEL-08-04>` in evidence cells = that same folder.

## 1. Histograms (recount reproduces from the CSV)

**ClaimType (n=23):**

| ClaimType | Count |
|---|---|
| REQUIREMENT | 12 |
| ACCEPTANCE | 1 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |

**Disposition (n=23):**

| Disposition | Count |
|---|---|
| ALIGNED | 20 |
| STALE_SETUP_SPECIFICATION | 3 |

`SelectableUnderCurrentLoop=YES`: 2 rows (DECL-005 `_STATUS`, REM-001) — mechanical
derivation from the one ungated non-bootstrap residual only; the owner suspension stays a
run-level caveat (convention 6 / RUN_BASIS). No `PARTIALLY_IMPLEMENTED`,
`ACCEPTED_DIVERGENCE`, `IMPLEMENTED_UNMAPPED`, `UNKNOWN`, or `AUTHORITY_CONFLICT` rows.

STALE rows are exactly three setup-era declared-state kit surfaces (DECL-001 Specification,
DECL-002 Datasheet, DECL-004 Procedure). All 12 requirement rows take a substance
disposition (ALIGNED) — no requirement/acceptance/exclusion row takes
`STALE_SETUP_SPECIFICATION` (convention 1). Note the deliberate departure from the
DEL-08-01 exemplar's 4-STALE pattern: **DECL-003 (Guidance) is ALIGNED here**, not STALE
(see §2, §4).

## 2. Census reasoning

- **REQUIREMENT: 12.** DEL-08-04-R1..R12 re-verified against the frozen Specification.md
  requirements table (exactly twelve) and the R1 `DELIVERABLE_INVENTORY.csv` row (lists
  R1..R12). ClaimIDs use the fixed form `DEL-08-04-REQ-NNN` (addendum 12); native
  `DEL-08-04-RN` recorded in `NormativeSource`. Requirement substance verified against the
  frozen `core/reporting/result_export` crate (`src/lib.rs`) and `schemas/results.schema.yaml`.
- **ACCEPTANCE: 1.** The Specification requirements-table restatements and the `## Verification`
  V-1..V-8 table are future-implementation **verification** of the requirements and are NOT
  mirrored to acceptance rows (addendum 12 — acceptance rows never merged with requirement
  rows). One ACC row (ACC-001) captures the distinct **setup-session** acceptance basis
  (four-document kit + semantic/dependency artifacts present; `Dependencies.csv` valid v3.1;
  boundary visibility) from the Procedure `## Verification` setup-gate table. Judgment call —
  self-flagged (reviewer may prefer zero ACC rows), consistent with the exemplar's ACC-001.
- **EXCLUSION: 3.** Durable, declared scope deferrals to *other* deliverables/phases, none
  duplicating a requirement: EXC-001 private-data redaction/export controls deferred to
  PKG-12; EXC-002 excludes authenticating/certifying engineering work (`_CONTEXT.md` Package
  Exclusions); EXC-003 downstream consumer implementation (report/GUI/CLI/adapters) + public
  API transport out of scope (consumers are separate deliverables). The additional-format-TBD
  boundary is **not** encoded as an exclusion because it is requirement R2's own substance
  (avoids double-encoding). EXC-003 is a self-flagged judgment call (distinct from REQ-012,
  which requires the contract to *support* downstream consumption).
- **DECLARED_STATE: 6.** Addendum-1 census = four kit docs + `_STATUS.md` + `MEMORY.md`.
  **README census = 0**: no deliverable-owned in-tree README exists — the DEL-08-04 folder
  has none, and the primary material surface `core/reporting/result_export` has **no README**
  (verified by `find … -iname README*` → empty). `_CONTEXT.md`/`_REFERENCES.md`/`_REVIEW.md`/
  `Review_Findings.csv` are not census surfaces and get no DECL rows (consistent with the
  6-DECL W3 exemplar).
- **REMAINING_WORK: 1.** The single ungated non-bootstrap residual from `_STATUS.md
  ## Remaining` (REM-001, target-format unit-conversion witnesses hardening). The seeded
  `(gated: D-41)` bootstrap item is recorded byte-exact ONLY in DECL-005 (`_STATUS`)
  `RecordedRemaining` and gets no row and no gate/source-cell annotation (addendum 2 + W2
  exclusion variant, calibration 11). This matches the inventory row (RemainingItemCount 2,
  BootstrapItemPresent YES, NonBootstrapItems 1).
- **IMPLEMENTED_UNMAPPED: 0.** Every material surface in DEL-08-04's orbit is already
  deliverable-mapped in R1 `IMPLEMENTATION_SURFACES.csv`: SURF-114 (result_export crate,
  NamedInSPEC), SURF-202 (results.schema.yaml, NamedInSPEC), SURF-040 (desktop result-export
  panel), SURF-164 (fixtures/results), plus shared consumers SURF-121/187/226/228. These are
  heavily multi-attributed shared crates/schemas but DEL-08-04 is the export-format owner;
  nothing is materially implemented yet unmapped.

## 3. Self-flagged rows (verifier should re-check)

- **DECL-003 (Guidance) = ALIGNED, not STALE (top self-flag).** Unlike the exemplar's
  Guidance (which declared the generator unimplemented + open questions since settled),
  DEL-08-04's Guidance carries **no** setup-era "not implemented in this setup session"
  declaration and **no** since-settled open questions. Its declarations — schema-first JSON
  envelope baseline, additional formats TBD, and the units/diagnostics/provenance/determinism/
  privacy principles — all remain consistent with the frozen implementation (format-TBD is
  still true per requirement R2). Widened addendum 4 needs a declaration that "no longer
  describes the frozen implemented slice"; none does here. A reviewer strict on setup framing
  might prefer STALE. Confidence MEDIUM.
- **DEL-08-04-REQ-006 (human-approved record state).** The requirement asks exported statuses
  to distinguish "… and human-approved record states where present." The crate `AnalysisStatus`
  enum and the schema `AnalysisStatus` enum both stop at `HUMAN_REVIEW_REQUIRED` and carry an
  explicit schema comment: "Automatic software-emitted analysis statuses allowed in result
  exports. Human project acceptance is external and hash-bound." I read this as a **deliberate
  boundary** (an automatic `HUMAN_APPROVED` software status would collide with R7's no-approval-
  claim), so human-approved records are referenced externally via the hash-bound acceptance /
  reproducibility path rather than an enum value. Chose **ALIGNED at the software-emitted-status
  grain**, MEDIUM. A reviewer may prefer PARTIALLY_IMPLEMENTED on the literal enum omission.
- **DEL-08-04-REQ-003 (unit preservation) grain + residual link.** Chose ALIGNED at envelope
  unit-preservation grain (units/dimension carried or blocked; `conversion_performed=false`),
  MEDIUM, with the residual REM-001 (remaining target-format unit-conversion witnesses) noted
  in `RemainingWork` rather than downgrading the substance disposition.
- **DEL-08-04-ACC-001.** Whether the setup-kit `## Verification` gate warrants an ACCEPTANCE
  row under the narrowed addendum-12 grain is a judgment call; included one, reviewer may
  prefer zero.
- **DEL-08-04-EXC-003.** Distinguishing "IMPLEMENTING downstream consumers is out of scope"
  (EXC-003) from "the contract must SUPPORT downstream consumption" (REQ-012) is a grain call;
  both are encoded, non-overlapping.
- **DEL-08-04-DECL-006 (rev-0.7 / DAG-006 pointer drift home).** As in the exemplar, the
  SOFTWARE_DECOMP rev-0.7 / DAG-006 pointer drift lives only on **non-census** surfaces
  (`_CONTEXT.md` Accepted Revision 0.7 / Architecture Basis Injection; `_REFERENCES.md`) and a
  **dated** MEMORY entry (2026-06-04 `TP-AUTHORITY-REFRESH-0_7-DAG006`); none of the four kit
  DECLARED_STATE surfaces carries a revision number. Recorded as an in-row MEMORY note (dated
  entry → note, never a staleness disposition per addendum 1); owner-calibration caveat raised
  once in §4. Departs from calibration 1's assumed STALE home — self-flagged.

## 4. Convention-friction / calibration notes

- **Setup-era future-tense on Specification / Datasheet / Procedure is overtaken (widened
  addendum 4).** The kit was written under a "this setup run does not implement exporter code
  / schema / tests" posture; the frozen tree carries a substantial implemented slice
  (`schemas/results.schema.yaml`, `core/reporting/result_export` crate with 12 tests,
  `tests/test_results_schema.py`, two invented serialized fixtures, desktop result-export
  preview). DECL-001/002/004 are STALE_SETUP_SPECIFICATION with `AuthorityNeeded=OWNER` (R5
  repair candidates, recorded as disposition/RemainingWork only; no repair performed).
- **Undated MEMORY blocks are accurate, not stale (calibration 1).** MEMORY's undated
  companion blocks (Implementation Summary / Boundary Decisions / Remaining TBDs) still
  describe the frozen slice correctly — the Remaining-TBDs list (additional formats, public
  API transport, FEA handoff, GUI/report/CLI/adapter integration, private redaction PKG-12,
  release thresholds) is all genuinely still deferred. No stale setup-era current-state head
  is present, so DECL-006 is ALIGNED (calibration item 1's STALE branch does not fire).
- **rev-0.7 authority-pointer drift (owner-calibration caveat, raised once):** frozen
  `_CONTEXT.md`/`_REFERENCES.md` cite SOFTWARE_DECOMP revision 0.7 / DAG-006 as current basis
  while the frozen decomp header is revision 0.8 and the live DAG is DAG-007. Pure
  authority-pointer drift; `AuthorityNeeded=NO`. Calibration 1 directs STALE on the affected
  DECLARED_STATE surface row, but the drift never lands on a census DECLARED_STATE surface
  here (non-census `_CONTEXT`/`_REFERENCES` + dated MEMORY entry only) — the same friction the
  exemplar reported. Recorded as an in-row MEMORY note; no invented DECL row for
  `_CONTEXT`/`_REFERENCES`.
- **Class assignment for determinism (addendum 7 / calibration 8).** R10 export determinism
  is REPORTING with `ValidationEvidence=NOT_APPLICABLE` and an explicit "deterministic ordering
  is a verification property, not promoted to engineering validation" reason — verification ≠
  validation held; the ordering test is cited as verification, never lifted to validation.
- **SourceReliability weakest-leg keying (calibration 5 / addendum 6).** All REQ/ACC/EXC/REM
  rows key `UNVERIFIED` to the weakest load-bearing leg (project-original agent-generated crate/
  schema/test/desktop evidence, human disposition pending). DECLARED_STATE prose rows are
  `NOT_APPLICABLE`. No REVIEWED/VETTED claimed anywhere (no human ruling covers the cited
  technical records; the `_REVIEW.md` DEV-001 PKG-02 audit is a PASS compatibility audit, cited
  as verification support on REQ-009, not promoted to REVIEWED).
- **AuthorityNeeded as router, not work queue (calibration 6).** REM-001 and every ALIGNED
  requirement/exclusion row route `AuthorityNeeded=NO`: recorded residuals/boundaries with no
  numeric or authority claim being promoted. The three STALE DECL rows route OWNER (R5 repair
  candidates). No gate-named tokens (D-07b/D-38) — the record names no gate governing any row.
- **F-PIP watch (reporting/governance deliverable).** REQ-007/008 and EXC-002 concern the
  professional/certification boundary and protected/private content; these are the deliverable's
  own declared *negative* obligations (exports must NOT certify/seal/approve/authenticate/claim
  compliance; must NOT embed protected content). Encoded as the deliverable's claims,
  quoted/attributed. I made no release-readiness, issuance, certification, sealing, or
  professional-approval assertion of my own anywhere in the outputs; all dispositions are agent
  judgments routed via `AuthorityNeeded`.

## 5. Evidence-execution log

All re-executions honored addendum 9 (external `CARGO_TARGET_DIR` to my scratch dir,
`PYTHONDONTWRITEBYTECODE=1`); `git -C FROZEN status --porcelain` was **empty before and after
every command**, and frozen HEAD stayed `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.

Re-executed side-effect-free at frozen SHA `551f84ef6` (all PASS; frozen tree clean after):

- `cargo test` (external target dir): `core/reporting/result_export` **12/12** (envelope
  validation; unit/dimension blocking; force/moment/section-property metadata mandatory;
  governed result-set vocabulary; human-review-status mandatory; professional-boundary block;
  rule-pack redaction + protected-suspected block; deterministic ordering; TP-PHYS-015 and
  TP-SECTION-021 fixture serialization). Doc-tests 0/0.
- `python3 tests/test_results_schema.py` **PASS** (structural schema checks + jsonschema
  instance validation of both invented result fixtures; no SKIP emitted → jsonschema path ran).
- `python3 tools/validation/validate_dependencies_schema.py <DEL-08-04>/Dependencies.csv`
  → **VALID** (29 columns, 22 data rows).

Cited as recorded (NOT re-executed at the frozen SHA — marked in-row with `not re-executed at
frozen SHA 551f84ef6`): the desktop Vitest result-export/unit-witness suites (MEMORY
2026-06-17/18: `App.test.tsx` 55/55, full desktop Vitest 399/399), bound to their
`_run_records/**`; and the `_REVIEW.md` DEV-001 PKG-02 downstream compatibility audit
(2026-05-16, DEL-02-04 no-bypass PASS). No addendum-10 "content-identical … (diff empty over
<paths>)" qualifier is used (no ancestor-commit diff was run).

Direct-inspection facts verified at the frozen SHA: `core/reporting/result_export` contains
only `Cargo.toml`, `.gitignore`, `src/lib.rs` — **no README** (README census = 0);
`schemas/results.schema.yaml` `deliverable_id` is `const DEL-08-04` and `AnalysisStatus` enum
stops at `HUMAN_REVIEW_REQUIRED` with the "human acceptance external and hash-bound" comment
(backs REQ-006 self-flag); `Review_Findings.csv` is header-only (no findings); frozen
`_STATUS.md` is IN_PROGRESS with the two `## Remaining` items exactly as transcribed.

## 6. Boundary-compliance statement

- All fences held. Discovery was read-only outside my two output files
  (`WAVES/W3/CLAIM_CONCORDANCE_DEL-08-04.csv` and `WAVES/W3/NOTES_DEL-08-04.md`). No
  `_STATUS.md`, register, DAG, product file, schema, crate, or cross-project file was edited;
  no lifecycle transition was applied (`STALE_SETUP_SPECIFICATION` and R5 repair candidates
  are recorded as dispositions/RemainingWork only; `LIFECYCLE_REASSESSMENT_REQUIRED` never
  applied); no DAG mutation.
- No F-PIP-1..5 claim (release-readiness, issuance, certification, sealing, professional
  approval, code-compliance) appears in either output outside attributed quotes of the
  deliverable's own negative obligations.
- No `DEFERRED_AGENT_WORKFLOW` implications arose for this deliverable.
- Frozen evidence tree: `git status --porcelain` empty **before AND after** every read and
  re-execution; all build/bytecode artifacts were redirected outside the frozen tree (external
  `CARGO_TARGET_DIR`; `PYTHONDONTWRITEBYTECODE=1`). Frozen HEAD verified
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7` throughout.
- **STOP-worthy contradictions: NONE** (the D-41 `AWAITING_RULING` frozen-register state is
  ruling-after-freeze mechanics per RUN_BASIS, not a conflict).
