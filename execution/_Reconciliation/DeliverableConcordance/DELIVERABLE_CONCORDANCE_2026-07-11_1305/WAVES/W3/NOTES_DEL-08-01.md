# NOTES — DEL-08-01 Calculation report generator (R2 wave W3)

Deliverable: **DEL-08-01** (PKG-08 Reporting, Audit, and Reproducibility), status
IN_PROGRESS. Frozen source tree pinned SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ledger: `CLAIM_CONCORDANCE_DEL-08-01.csv` (23 rows, 20 columns, RFC-4180 CRLF, header
byte-exact).

Run-level `NormativeSource` aliases used in the ledger (addendum 12): `SOFTWARE_DECOMP.md`
= `execution/_Decomposition/SOFTWARE_DECOMP.md`; `ScopeLedger.csv` / `Deliverables.csv` =
`docs/_Registers/*`; kit filenames are relative to the deliverable folder
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/`.

## 1. Histograms (recount reproduces from the CSV)

**ClaimType (n=23):**

| ClaimType | Count |
|---|---|
| REQUIREMENT | 10 |
| ACCEPTANCE | 1 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 3 |

**Disposition (n=23):**

| Disposition | Count |
|---|---|
| ALIGNED | 19 |
| STALE_SETUP_SPECIFICATION | 4 |

STALE rows are exactly the four setup-era declared-state surfaces (DECL-001 Specification,
DECL-002 Datasheet, DECL-003 Guidance, DECL-004 Procedure). All 10 requirement rows take a
substance disposition (ALIGNED) — no requirement/acceptance/exclusion row takes
`STALE_SETUP_SPECIFICATION` (convention 1). DECL-005 (_STATUS) and DECL-006 (MEMORY) are
ALIGNED (status accurate; MEMORY overtaken block corrected in-file — see §4).

## Census reasoning

- **REQUIREMENT: 10.** R-08-01-001..010 re-verified against the frozen Specification.md
  requirements table and R1 `DELIVERABLE_INVENTORY.csv` (both list exactly ten). ClaimIDs
  use the fixed form `DEL-08-01-REQ-NNN` (addendum 12); native `R-08-01-00N` recorded in
  `NormativeSource`.
- **ACCEPTANCE: 1.** The Specification requirements-table `Verification` column merely
  restates each requirement and is NOT mirrored to acceptance rows (addendum 12). One ACC
  row (ACC-001) captures the distinct setup-session acceptance basis (four-document kit
  presence, dependency-register validity, source-grounding) from `## Verification` and
  `## Acceptance Criteria For This Setup Session`, which is not a requirement mirror. This
  is a judgment call — self-flagged.
- **EXCLUSION: 3.** Durable product-scope exclusions only: result export/transport format
  is DEL-08-04 (EXC-001); detailed private-data redaction/export controls are deferred to
  PKG-12 (EXC-002); scope excludes authenticating/certifying engineering work (EXC-003).
  The setup-session write-boundary ("this setup session does not implement renderer
  source…") is NOT encoded as an exclusion — it is a setup-era declared state now overtaken
  by implementation, ledgered as STALE on the Specification/Datasheet surface rows.
- **DECLARED_STATE: 6.** Addendum-1 census = four kit docs + `_STATUS.md` + `MEMORY.md`.
  No deliverable-owned in-tree README exists (`find` over the deliverable folder and the
  `core/reporting/*` crates: only `core/reporting/audit_manifest/README.md`, which is the
  DEL-08-02 crate, not DEL-08-01-owned). `_CONTEXT.md`/`_REFERENCES.md` are not census
  surfaces and get no DECL rows (consistent with the 6-DECL W2 ledgers).
- **REMAINING_WORK: 3.** The three real ungated non-bootstrap residuals from `_STATUS.md
  ## Remaining` (REM-001 report-package seam binding; REM-002 cross-layer component-
  provenance test; REM-003 .opsproj versioning policy). The seeded `(gated: D-41)` bootstrap
  item is recorded byte-exact ONLY in the DECL-005 (_STATUS) `RecordedRemaining` and gets no
  row and no gate/source-cell annotation (addendum 2 + W2 exclusion variant, calibration 11).
- **IMPLEMENTED_UNMAPPED: 0.** Every material surface in DEL-08-01's orbit (report_generator,
  report_renderer, report_package, pdf_emitter crates; report_generator.schema.yaml; the
  invented fixture; the desktop report feature/render service) is already deliverable-mapped
  in R1 `IMPLEMENTATION_SURFACES.csv` and documented in MEMORY/_run_records. Nothing is
  materially implemented yet unmapped.

## 2. Self-flagged rows

- **DEL-08-01-REQ-002** — Forward-looking capability breadth (calibration 6). The
  enumerated report fields (software/solver version, model hash, load cases/combinations,
  rule-pack name/version/checksum, notices "where those inputs exist") are exercised only
  against the invented fixture at the frozen SHA. I chose **ALIGNED at report-
  contract/omission-rule grain** (not PARTIALLY_IMPLEMENTED): the schema + renderer support
  the fields under explicit omission rules and the golden fixture asserts them. Confidence
  MEDIUM. Reviewer eyes on the grain choice.
- **DEL-08-01-REQ-007** — Diagnostics warning classes: nonlinear and IP-boundary classes are
  present only via the fixture path. Chose ALIGNED at pass-through-contract grain, MEDIUM.
- **DEL-08-01-REQ-008** — Rule-pack safe-metadata requirement (calibration 6 grain call).
  The report_generator contract validates rule-pack review/completeness/private/protected
  reference metadata (implemented + verified), but full end-to-end rule-pack-reference
  binding is a recorded Phase C downstream residual (MEMORY 2026-06-11). Chose **ALIGNED at
  report-generator-contract grain**, MEDIUM, rather than PARTIALLY_IMPLEMENTED; the deferred
  slice is the downstream binding, not this deliverable's contract obligation.
- **DEL-08-01-ACC-001** — Whether the setup-session acceptance criteria warrant an
  ACCEPTANCE row at all under the narrowed addendum-12 grain is a judgment call (see §1).
  Included one row; reviewer may prefer zero.
- **DEL-08-01-DECL-006 (rev-0.7 authority-pointer drift home)** — The rev-0.7 / DAG-006
  citation drift (calibration 1) lives in the frozen kit only on **non-census** surfaces
  (`_CONTEXT.md` "Accepted Revision 0.7 / current_basis" and Architecture Basis Injection;
  `_REFERENCES.md`) and on the **dated** MEMORY entry 2026-06-04 (`TP-AUTHORITY-REFRESH-
  0_7-DAG006`). None of the four DECLARED_STATE kit surfaces carries the revision number, so
  there is no census surface row on which to encode `STALE_SETUP_SPECIFICATION` for the
  pointer drift. I recorded the drift as an in-row note on the MEMORY surface row (dated
  entry → note, never a staleness disposition per addendum 1) and I raise the owner-
  calibration caveat once here (§4). This departs from calibration 1's assumed home;
  self-flagged for reviewer confirmation.

## 3. Evidence-execution log

All re-executions honored addendum 9 (external `CARGO_TARGET_DIR` to my scratch dir,
`PYTHONDONTWRITEBYTECODE=1`); `git -C FROZEN status --porcelain` was **empty before and after
every command**.

Re-executed side-effect-free at frozen SHA `551f84ef6` (all PASS; frozen tree clean after):

- `cargo test` (external target dir): `core/reporting/report_generator` 10/10;
  `core/reporting/report_renderer` 8/8; `core/reporting/report_package` 12/12 (incl. double
  independent-build byte-identity, manifest/member-hash agreement, JCS idempotence, input-
  change hash propagation); `core/reporting/pdf_emitter` 8/8.
- `python3 tests/test_report_generator_contract.py` PASS;
  `python3 tests/test_report_sections_contract.py` PASS;
  `python3 tests/test_report_protected_content_linter.py` PASS.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-08-01>/Dependencies.csv`
  → VALID (29 columns, 18 rows: 17 ACTIVE / 1 RETIRED).

Cited as recorded (NOT re-executed at the frozen SHA — marked in-row with `not re-executed
at frozen SHA 551f84ef6`): the desktop Vitest/Playwright suites for report unit disclosure
and rendered-report unit basis (MEMORY 2026-06-12/17: report Vitest 53/53, full desktop
Vitest 399/399, Playwright smoke), bound to their `_run_records/**`; and the DEV-001 PKG-02
downstream audit (`_REVIEW.md`, 2026-05-16). No addendum-10 "content-identical … (diff empty
over <paths>)" qualifier is used in this ledger (I did not run any ancestor-commit diff).

Direct-inspection facts verified at the frozen SHA: `tools/validation/` contains only
`validate_dependencies_schema.py` — the Procedure-named `check_four_documents.sh` and
`check_min_viable_fileset.sh` do NOT exist (backs DECL-004 STALE and the ACC-001 note);
`SOFTWARE_DECOMP.md` header is `revision: 0.8`, `status: current_basis`; `_DAG/_LATEST.md`
→ DAG-007 (DAG-006 superseded); DEC-018/021/028/057/061/065 all present in decomp §12.

## 4. Convention-friction notes

- **rev-0.7 authority-pointer drift (owner-calibration caveat, raised once):** the frozen
  `_CONTEXT.md`/`_REFERENCES.md` cite SOFTWARE_DECOMP revision 0.7 / DAG-006 as current
  basis while the frozen decomp header is revision 0.8 and the live DAG is DAG-007. This is
  pure authority-pointer drift; `AuthorityNeeded=NO`. Calibration 1 directs encoding STALE
  on the *affected DECLARED_STATE surface row*, but on this deliverable the drift never lands
  on a census DECLARED_STATE surface (see §2, DECL-006 self-flag) — friction between
  calibration 1's assumed home and the addendum-1 census. Recorded as an in-row MEMORY note
  and here; no invented DECL row for `_CONTEXT`/`_REFERENCES`.
- **Setup-era future-tense on the four kit docs is uniformly overtaken.** The kit was written
  under a "does not implement in this setup session" posture; the frozen tree carries a
  substantial implemented slice (report_generator/renderer/package/pdf_emitter crates,
  schema, fixture, contract tests, desktop render seam). Widened addendum 4 governs — all
  four kit DECL rows are STALE_SETUP_SPECIFICATION with `AuthorityNeeded=OWNER` (R5 repair
  candidates, recorded as disposition/RemainingWork only; no repair performed).
- **Undated/overtaken MEMORY block corrected in-file (calibration 9).** MEMORY's 2026-05-02
  companion `## Remaining TBDs` list names items since implemented (GUI report preview;
  protected-content linter integration). Later dated entries in the same file (2026-06-11
  TP-APP-R2-REPORTRENDER-001 render seam + three-point protected-content lint gating; 2026-
  06-06 hardening) correct the drift, so DECL-006 is **ALIGNED-with-note** and the note names
  those in-file correcting entries — not a staleness disposition.
- **Class assignment for reproducibility (addendum 7).** R-08-01-010 report-determinism is
  encoded REPORTING with `ValidationEvidence=NOT_APPLICABLE` and an explicit "reproducibility
  is a verification property, not promoted to engineering validation" note — verification ≠
  validation held (calibration 8 / PKG-04/05 spirit); the container byte-identity suite is
  cited as verification, never lifted to validation.
- **AuthorityNeeded as router, not work queue (calibration 14).** All three REMAINING_WORK
  rows and REQ-009 route `AuthorityNeeded=NO`: recorded TBDs/residuals with no numeric or
  authority claim being promoted. Scope-boundary token disclosed: REM-001's residual states
  a *new runner verb* is ruled out of scope ("no new runner verb without a ruling",
  DEC-065) — this is a scope statement in the run record, not a gate token blocking the
  residual, so the item stays ungated/selectable with NO authority routing.
- **F-PIP watch (reporting deliverable).** Requirements R-003/R-004/R-008 and EXC-003 concern
  protected content and the professional/certification boundary. These are the deliverable's
  own declared *negative* obligations (reports must NOT certify/seal/authenticate/claim code
  compliance; templates must NOT reproduce protected content); I encoded them as the
  deliverable's claims, quoted/attributed, and made no release-readiness, issuance,
  certification, sealing, or professional-approval assertion of my own anywhere in the
  outputs. Dispositions are agent judgments, routed via AuthorityNeeded, never phrased as
  owner/engineering rulings.

## 5. Boundary-compliance statement

- All fences held. Discovery was read-only outside my two output files
  (`WAVES/W3/CLAIM_CONCORDANCE_DEL-08-01.csv` and `WAVES/W3/NOTES_DEL-08-01.md`). No
  `_STATUS.md`, register, DAG, product file, or cross-project file was edited; no lifecycle
  transition was applied (`STALE_SETUP_SPECIFICATION` and repair candidates are recorded as
  dispositions/RemainingWork only); no DAG mutation.
- No F-PIP-1..5 claim (release-readiness, issuance, certification, sealing, professional
  approval, code-compliance) appears in either output.
- No `DEFERRED_AGENT_WORKFLOW` implications arose for this deliverable.
- Frozen evidence tree: `git status --porcelain` empty before AND after every read and
  re-execution; all build/bytecode artifacts were redirected outside the frozen tree
  (external `CARGO_TARGET_DIR`; `PYTHONDONTWRITEBYTECODE=1`). Frozen HEAD verified
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7` throughout.
- STOP-worthy contradictions: NONE (the D-41 `AWAITING_RULING` frozen-register state is
  ruling-after-freeze mechanics per RUN_BASIS, not a conflict).
