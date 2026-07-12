# NOTES — DEL-08-05 Report protected-content linter (R2 wave W3)

Deliverable: **DEL-08-05** (PKG-08 Reporting, Audit, and Reproducibility), status
IN_PROGRESS. Frozen source tree pinned SHA `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ledger: `CLAIM_CONCORDANCE_DEL-08-05.csv` (22 rows, 20 columns, RFC-4180 CRLF, header
byte-exact to the DEL-08-01 exemplar).

Run-level `NormativeSource` aliases used in the ledger (addendum 12):
`SOFTWARE_DECOMP.md` = `execution/_Decomposition/SOFTWARE_DECOMP.md`; `ScopeLedger.csv` /
`Deliverables.csv` / `ContextBudgetQA.csv` = `docs/_Registers/*`; kit filenames
(`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`, `_STATUS.md`,
`MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`, `Dependencies.csv`) are relative to the
deliverable folder `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/`;
crate/schema/fixture/tool paths (`core/reporting/protected_content_linter/`,
`schemas/report_protected_content_linter.schema.yaml`, `fixtures/report_lint/invented/`,
`tests/test_report_protected_content_linter.py`, `tests/test_release_candidate_scan.py`,
`tools/release/run_release_candidate_scan.py`) are repo-root relative.

## 1. Histograms (recount reproduces from the CSV)

**ClaimType (n=22):**

| ClaimType | Count |
|---|---|
| REQUIREMENT | 12 |
| ACCEPTANCE | 1 |
| EXCLUSION | 2 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |

**Disposition (n=22):**

| Disposition | Count |
|---|---|
| ALIGNED | 17 |
| PARTIALLY_IMPLEMENTED | 1 |
| STALE_SETUP_SPECIFICATION | 4 |

**AuthorityNeeded (n=22):** NO 14, OWNER 8.
**SelectableUnderCurrentLoop (n=22):** NO 22 (no ungated residual anywhere; the sole
non-bootstrap residual is owner/stage-gated).

The 4 STALE rows are exactly the four setup-era kit declared-state surfaces (DECL-001
Specification, DECL-002 Datasheet, DECL-003 Guidance, DECL-004 Procedure). The single
PARTIALLY_IMPLEMENTED row is REQ-010 (CI-guard severity/review-blocking: tooling landed,
CI wiring + owner-gated scan act pending). All 12 requirement rows carry a substance
disposition (11 ALIGNED + 1 PARTIALLY_IMPLEMENTED); none takes
`STALE_SETUP_SPECIFICATION`, even though every requirement sentence is written in the
setup-era future tense ("The future linter shall…") — declaration staleness is ledgered
only on the per-surface DECL rows (convention 1). DECL-005 (_STATUS) and DECL-006
(MEMORY) are ALIGNED.

## 2. Census reasoning

This deliverable is a setup-era kit whose implementation has substantially LANDED
(unlike a pure setup deliverable). The `core/reporting/protected_content_linter` crate,
`protected_content_lint_cli` binary, `report_protected_content_linter.schema.yaml`,
`fixtures/report_lint/invented/*`, `tests/test_report_protected_content_linter.py`, and
the DEC-058 `tools/release/run_release_candidate_scan.py` runner (+
`tests/test_release_candidate_scan.py`) are all present and passing at the frozen SHA.
The kit docs, however, still declare "future implementation" / "setup-artifacts only" —
hence the four STALE kit DECL rows.

- **REQUIREMENT: 12.** DEL-08-05-REQ-001..012, re-verified one-by-one against the frozen
  `Specification.md` requirements table (which lists exactly twelve, IDs native as
  `DEL-08-05-REQ-0NN`, so the fixed addendum-12 form coincides with the native form) and
  against the implemented slice. R1 `DELIVERABLE_INVENTORY.csv` records the same twelve
  PRESENT.
- **ACCEPTANCE: 1.** ACC-001 captures the distinct setup-session acceptance basis
  (four-document kit + semantic artifacts present; `Dependencies.csv` valid v3.1 with
  ACTIVE-row evidence; source-grounding; no protected/certification content) from
  `## Verification` + `## Acceptance Criteria For This Setup Session`, which is not a
  requirement-table restatement. Mirrors DEL-08-01 ACC-001. Judgment call — self-flagged.
- **EXCLUSION: 2.** Durable scope boundaries only: EXC-001 = scope excludes
  authenticating/certifying engineering work (package exclusion; heuristic guard, not a
  sole legal/IP control); EXC-002 = no dependency on DEL-11-04 educational example models
  (invented/synthetic fixtures only; DAG-001-E0621 candidate edge retained non-gating).
  The Specification "Out of scope for this setup session" list (writing linter source,
  adding tests/fixtures, editing report templates, moving to ISSUED) is NOT encoded as
  exclusions — those are setup-era write-boundary declarations now overtaken by
  implementation, ledgered as STALE on the kit DECL rows. The EXC count is a judgment
  call — self-flagged (reviewer may prefer including/excluding the DEL-11-04 independence
  boundary, which partly overlaps REQ-008).
- **DECLARED_STATE: 6.** Addendum-1 census = four kit docs + `_STATUS.md` + `MEMORY.md`.
  **README census = 0**: `find` over the deliverable folder and the
  `core/reporting/protected_content_linter/` crate finds no README (crate files are
  `.gitignore`, `Cargo.lock`, `Cargo.toml`, `src/lib.rs`, `src/bin/protected_content_lint_cli.rs`);
  no deliverable-owned in-tree README exists, so no self-identifying-crate-README DECL row
  is added (W3 calibration item 2). `_CONTEXT.md`/`_REFERENCES.md`/`_REVIEW.md`/
  `Review_Findings.csv` are not census surfaces and get no DECL rows (consistent with the
  6-DECL DEL-08-01 exemplar).
- **REMAINING_WORK: 1.** REM-001 = the single non-bootstrap residual from `_STATUS.md
  ## Remaining` (execute the AC-1–AC-6 release-candidate scan act with owner sign-off).
  The seeded `(gated: D-41)` bootstrap item is recorded byte-exact ONLY in the DECL-005
  (_STATUS) `RecordedRemaining` and gets no row and no gate/source-cell annotation
  (addendum 2 + W2 exclusion variant). R1 inventory records RemainingItemCount 2,
  BootstrapItemPresent YES, one NonBootstrapItem — reconciled: 1 REM row.
- **IMPLEMENTED_UNMAPPED: 0.** Every material surface in DEL-08-05's orbit is already
  deliverable-mapped in R1 `IMPLEMENTATION_SURFACES.csv`: SURF-108 (crate), SURF-109
  (CLI bin), SURF-162 (`fixtures/report_lint`), SURF-200 (schema), SURF-223
  (`run_release_candidate_scan.py`), plus the SURF-039 desktop report-lint panel and the
  report-lint unit-policy inventory surfaces. Nothing materially implemented is unmapped.

## SECURITY-class encoding (convention 6 — first exercise in the waves)

Per R1 Part C, the first SECURITY-class deliverable ledgered in the waves gets a reviewer
spot-check of the convention-6 SECURITY encoding (unexercised in calibration). DEL-08-05
is a protected-content / IP-boundary guard and exercises it. The convention-6 marker
`ValidationEvidence=NONE_FOUND — sufficiency review deferred, owner-gated` (with no
`VERIFIED_NOT_VALIDATED` downgrade and OWNER routing per the W1 SECURITY marker/routing
harmonization) is applied to the rows whose protected-content/IP-boundary DETECTION,
ROUTING, or GATING **sufficiency** is deferred to the owner-gated release-candidate scan
act (DEC-058, owner sole signatory of the legal/protected-data release gate):

- **REQ-002** (flag suspected protected content) — the core heuristic detection.
- **REQ-004** (route suspected content to quarantine/review signals).
- **REQ-010** (CI-guard review-blocking severity policy) — the gated act itself.
- **REM-001** (execute the AC-1–AC-6 scan act) — the owner-gated sufficiency review.

The other SECURITY-class rows (REQ-001 public-surface scoping, REQ-005 safe-metadata
allowance, REQ-008 invented-fixture hygiene, REQ-011 no-telemetry-by-default) are
verification-complete deterministic/design properties, so they carry
`ValidationEvidence=NOT_APPLICABLE` with an explicit in-cell reason and `AuthorityNeeded=NO`
(no sufficiency judgment is deferred for them). GOVERNANCE professional-boundary rows
(REQ-003, REQ-006, REQ-012) and REPORTING rows (REQ-007, REQ-009) carry
`ValidationEvidence=NOT_APPLICABLE` per the DEL-08-01 exemplar. This split — SECURITY rows
both with and without the convention-6 marker — is the main judgment call for the reviewer
spot-check (self-flagged below).

The gate constraint is copied **verbatim** from `_STATUS.md` onto the two residual-bearing
surface rows (DECL-005 `GateOrStageConstraint`, REM-001 `GateOrStageConstraint`):
`gated: owner sole signatory per DEC-058; stage-gated: first release candidate`. Direct
inspection confirms `validation/evidence/releases/` is **absent** at the frozen SHA,
corroborating "the recorded scan … remain owner-only" — the owner has not yet scanned.

## 3. Self-flagged rows (verifier should re-check)

- **REQ-002 / REQ-004 / REQ-010 / REM-001** — the convention-6 SECURITY marker + OWNER
  routing application (which rows genuinely "defer sufficiency review to an owner-gated
  act"). REQ-002 chosen ALIGNED at heuristic-marker/standards-table-signature grain (not
  PARTIALLY_IMPLEMENTED): the detection behavior is fully present; only real-world
  sufficiency is deferred (convention 6 handles that via the ValidationEvidence marker
  without downgrade). REQ-010 chosen PARTIALLY_IMPLEMENTED: severity model + review-blocking
  landed in the DEC-058 runner, but the CI-provider guard wiring is TBD (homed to DEL-10-04
  per Procedure) and the executed scan is owner-gated.
- **SECURITY-class split** — REQ-001/005/008/011 encoded SECURITY class but
  `ValidationEvidence=NOT_APPLICABLE` (verification-complete), while REQ-002/004/010 carry
  the convention-6 deferral marker. A reviewer may prefer to apply the marker more broadly
  (all SECURITY rows) or more narrowly. See the SECURITY-class encoding section.
- **REQ-006 / REQ-012** — professional-authority-boundary detection and
  architecture-basis conformance encoded GOVERNANCE with NOT_APPLICABLE validation and
  `AuthorityNeeded=NO`, matching DEL-08-01 REQ-004/REQ-009; a reviewer could argue REQ-006
  (prohibited-claim detection) is SECURITY-adjacent, or route REQ-012 to REVIEW given the
  named (unrecorded) architecture-review acceptance gate.
- **ACC-001** — whether the setup-session acceptance basis warrants an ACCEPTANCE row at
  all under the narrowed addendum-12 grain (reviewer may prefer zero; DEL-08-01 self-flag
  carried forward).
- **EXC census (2)** — the DEL-11-04-independence exclusion (EXC-002) partly overlaps
  REQ-008's fixture policy; the count is a judgment call.
- **DECL-006 (MEMORY)** — rev-0.7 / DAG-006 authority-pointer drift home (see §4); the
  drift lives on the dated MEMORY entry 2026-06-04 and on non-census `_CONTEXT.md`, not on
  a census surface, so it is recorded as an in-row note, never a staleness disposition
  (addendum 1). Departs from calibration 1's assumed home; confirm.

## 4. Evidence-execution log

Re-executions redirected build/bytecode/temp artifacts outside the frozen tree (external
`CARGO_TARGET_DIR` to my scratch dir; `PYTHONDONTWRITEBYTECODE=1`; `TMPDIR` redirected to
scratch for pytest tmp dirs); plain `git -C FROZEN status --porcelain` (tracked-only) was
**empty before and after every command**, and frozen HEAD stayed
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` throughout. **Disclosed exception (W3 fan-in,
addendum-9 breach):** the `python3 -m pytest tests/test_release_candidate_scan.py` step
below wrote a `.pytest_cache/` directory into the frozen project root — the scratch
`TMPDIR` redirection covers only pytest's tmp-path fixtures, not the rootdir cache, and no
`-p no:cacheprovider` was used, so the cache was written; because the artifact is
git-ignored the plain porcelain check truthfully passed while the write occurred. A `git
status --porcelain --ignored=matching` check would have surfaced it. See the disclosure
note on the pytest line below and W3 verification §3.1. The `cargo test` run (committed
crate `Cargo.lock` + external target dir) and the `python3 tests/...` script runner
(`PYTHONDONTWRITEBYTECODE=1`, no pytest cache) genuinely wrote nothing.

Re-executed at frozen SHA `551f84ef6` (all PASS; tracked tree clean after — with the
disclosed `.pytest_cache/` exception below):

- `cargo test --manifest-path core/reporting/protected_content_linter/Cargo.toml`
  (external target dir) → **15/15** (synthetic-marker ordering/blocking, private-surface
  skip, safe-metadata, standards-table signature designator/clause/JSON/markup/window/
  substring/sparse/two-grid/accepted-provenance, professional-claim blocking).
- `python3 tests/test_report_protected_content_linter.py` → **PASS** (schema/fixture
  contract: `$defs`, const fields, FindingCode/Class enums, forbidden-fixture-term
  absence, DEL-11-04 example-ref absence).
- `python3 -m pytest tests/test_release_candidate_scan.py` (scratch `TMPDIR`) → **14/14**
  (record fields, all-six-classes-recorded, checksums, determinism, unsigned/pending
  sign-off, skipped-checks-fail-toward-review, provenance/quarantine blocking findings,
  engine-integration flags invented lookalike, clean-content negative). **Addendum-9
  disclosure (W3 fan-in):** this pytest invocation wrote a `.pytest_cache/` directory into
  the frozen project root (`projects/chirality-piping/.pytest_cache/`; cached nodeids are
  exactly this module's 14 tests) — an untracked git-ignored artifact invisible to plain
  `git status --porcelain` (hence the truthful-but-incomplete "clean" checks above). The
  scratch `TMPDIR` redirection covers pytest's tmp fixtures but not the rootdir cache; `-p
  no:cacheprovider` would have suppressed it. This is an addendum-9 breach (writes into the
  frozen tree are forbidden even on git-ignored paths). The **test results themselves are
  not invalidated** — a pytest cache affects no pass/fail outcome or encoded fact, and no
  tracked content changed (frozen HEAD unchanged). Containment (worktree restore) is
  escalated to the orchestrator per W3 verification §3.1; the affected per-row
  `VerificationEvidence` cells (REQ-002/003/004/009/010 + REM-001) carry the same
  disclosure.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-08-05>/Dependencies.csv`
  → **VALID** (29 columns, 13 rows: 13 ACTIVE / 0 RETIRED).

Cited as recorded, NOT re-executed at the frozen SHA (marked in-row with `not re-executed
at frozen SHA 551f84ef6`): the DEV-001 downstream PKG-02 compatibility audit `_REVIEW.md`
(2026-05-16, DEL-02-04 no-bypass PASS) backing REQ-012 — an agent package audit, not a
human ruling. No addendum-10 "content-identical … (diff empty over <paths>)" qualifier is
used in this ledger (no ancestor-commit diff was run).

Direct-inspection facts verified at the frozen SHA: `validation/evidence/releases/` does
**not exist** (backs REM-001 / DECL-005 "still absent"); `SOFTWARE_DECOMP.md` header is
`revision: 0.8`, `status: current_basis`; `execution/_DAG/_LATEST.md` → DAG-007 (DAG-006
superseded); the DEC-058 ruling row and the `D-20_protected_content_scan.md` packet are
present in the frozen tree (so the DEC-058 DecisionBasis resolves — convention 7); the two
LIFECYCLE_CORRECTION Decision_Log records and the
`Reconciliation_Run_Summary_2026-05-02_DEL0805_CANDIDATE_E0621.md` (EXC-002 basis) exist;
the crate `Cargo.toml` is dependency-free (backs REQ-011).

## Convention-friction notes

- **rev-0.7 authority-pointer drift (owner-calibration caveat, raised once):** the frozen
  `_CONTEXT.md` Architecture Basis Injection and the dated MEMORY entry 2026-06-04
  (TP-AUTHORITY-REFRESH-0_7-DAG006) cite `SOFTWARE_DECOMP` revision 0.7 / DAG-006 as
  current basis, while the frozen decomp header is revision 0.8 and the live DAG is
  DAG-007. This is pure authority-pointer drift; `AuthorityNeeded=NO`. As on DEL-08-01,
  the drift never lands on a census DECLARED_STATE surface (it is on non-census
  `_CONTEXT.md` and on a **dated** MEMORY entry → note, not staleness, per addendum 1), so
  it is recorded as an in-row note on DECL-006 and here; no invented DECL row for
  `_CONTEXT`. REQ-012's DecisionBasis cites SCA-001 with the revision-0.8 correction noted.
- **Setup-era future tense uniformly overtaken.** The kit was written under a "does not
  implement in this setup session / future implementation" posture; the frozen tree carries
  a substantial implemented slice (linter crate + CLI + schema + invented fixtures +
  deterministic tests + the DEC-058 standards-table extension and release-candidate scan
  runner). Widened addendum 4 governs — all four kit DECL rows are
  `STALE_SETUP_SPECIFICATION` with `AuthorityNeeded=OWNER` (R5 repair candidates, recorded
  as disposition/RemainingWork only; no repair performed).
- **MEMORY current-state block describes the frozen state (calibration 1).** MEMORY has no
  uncorrected setup-era head; its undated current-state blocks (Current Implementation,
  Guardrails, Verification, Open Items) accurately describe the implemented slice and the
  genuinely-surviving TBDs (CI provider, redaction/export controls, quarantine file
  movement, final legal-review workflow) → DECL-006 ALIGNED (calibration 1: an undated
  current-state block describing the frozen state is ALIGNED). Dated log entries are
  historical records.
- **AuthorityNeeded as router, not work queue (calibration 6).** OWNER is used only where
  the owner must ADJUDICATE: the convention-6 SECURITY sufficiency-deferral rows
  (REQ-002/004/010, REM-001) and the four STALE kit DECL rows (R5 repair adjudication).
  REM-001's OWNER routing reflects DEC-058's owner-sole-signatory adjudication of the
  release gate, not merely that the owner performs the scan. GOVERNANCE/REPORTING
  requirement rows and the verification-complete SECURITY rows route `NO`.
- **F-PIP watch (protected-content / release-gate deliverable).** Requirements and
  exclusions here concern protected content, the professional/certification boundary, and a
  release-candidate scan gate. I encoded them as the deliverable's own declared
  obligations/boundaries, quoted/attributed, and made no release-readiness, issuance,
  certification, sealing, professional-approval, or code-compliance assertion of my own
  anywhere in the outputs. All dispositions are agent judgments routed via AuthorityNeeded,
  never phrased as owner or engineering rulings.

## 5. Fence attestation / boundary-compliance statement

- All fences held **except the disclosed addendum-9 exception** (§4): the `python3 -m
  pytest tests/test_release_candidate_scan.py` step wrote a `.pytest_cache/` directory into
  the frozen project root. Otherwise discovery was read-only outside my two output files
  (`WAVES/W3/CLAIM_CONCORDANCE_DEL-08-05.csv` and `WAVES/W3/NOTES_DEL-08-05.md`). No
  `_STATUS.md`, register, DAG, schema, crate, tool, product file, or cross-project file was
  edited; no lifecycle transition was applied (`LIFECYCLE_REASSESSMENT_REQUIRED` never
  applied; `STALE_SETUP_SPECIFICATION` and R5 repair candidates are recorded as
  dispositions/RemainingWork only); no DAG/scope mutation.
- No F-PIP-1..5 claim language (release-readiness, issuance, certification, sealing,
  professional approval, code-compliance) appears in either output outside attributed
  quotes of the deliverable's own declared boundaries.
- No `DEFERRED_AGENT_WORKFLOW` implications arose for this deliverable.
- Frozen evidence tree: plain `git status --porcelain` (tracked-only) **empty before AND
  after** every read and re-execution; cargo build artifacts and Python bytecode were
  redirected outside the frozen tree (external `CARGO_TARGET_DIR`; `PYTHONDONTWRITEBYTECODE=1`;
  scratch `TMPDIR`), and the linter crate's committed `Cargo.lock` meant the `cargo test`
  run wrote nothing. Frozen HEAD verified `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
  throughout. **Disclosed exception (W3 fan-in):** the `python3 -m pytest
  tests/test_release_candidate_scan.py` step wrote a `.pytest_cache/` directory into the
  frozen project root (`projects/chirality-piping/.pytest_cache/`) — an untracked git-ignored
  artifact invisible to plain porcelain; an addendum-9 breach disclosed in §4 and W3
  verification §3.1. Test results are not invalidated and no tracked content changed;
  containment (worktree restore) is escalated to the orchestrator.
- **STOP-worthy contradictions: NONE.** (The D-41 `AWAITING_RULING` frozen-register state
  is ruling-after-freeze mechanics per RUN_BASIS, not a conflict. The owner-gated,
  as-yet-unexecuted release-candidate scan is a stage-gated residual with tooling complete,
  not a contradiction — `validation/evidence/releases/` absent is consistent with every
  record's declared state.)
