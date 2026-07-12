# W4 Fan-in Verification — PKG-09 (Verification, Validation, and Quality Oracles)

Verifier: **GPT-5, highest available capability tier, high effort**. No model
substitution was used. Scope: `CLAIM_CONCORDANCE_DEL-09-01..05.csv` and
`NOTES_DEL-09-01..05.md` under `WAVES/W4/`, verified against the frozen evidence
worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. The verification covered
every pilot-self-flagged row, every non-`ALIGNED` row, and at least two
`ALIGNED` rows in each ledger. It used `R1_CONVENTIONS.md` Parts A–D, R0/R0b,
the binding W1–W3 calibration in `PACKAGE_SUMMARIES/PKG-00..08.md`, and the
addendum-9 containment rules. All judgments are agent judgments, not human
rulings. No ledger, notes file, lifecycle state, DAG, or product artifact was
edited.

**Verdicts: DEL-09-01 DEFECTIVE · DEL-09-02 DEFECTIVE · DEL-09-03 SOUND ·
DEL-09-04 SOUND · DEL-09-05 SOUND.** Checked-row totals: **51 PASS / 1
QUALIFIED / 12 FAIL** across 64 rows. The twelve FAILs are string/cell defects
only: two in DEL-09-01 and ten instances of a common encoding defect in
DEL-09-02. The owning pilots can correct them without rediscovery or test
re-execution; exact routing is in §3.

## 1. Mechanical and containment checks

- All five CSVs parse as RFC-4180 at exactly 20 columns with the byte-exact run
  header and CRLF line endings. ClaimIDs are unique, contiguous within type,
  type-consistent, and use the fixed addendum-12 form. Row totals are
  25/18/21/26/30.
- Recounted disposition histograms are: DEL-09-01 16 `ALIGNED`, 3
  `PARTIALLY_IMPLEMENTED`, 4 `STALE_SETUP_SPECIFICATION`, 1
  `ACCEPTED_DIVERGENCE`, 1 `REMAINING_STATE_MISMATCH`; DEL-09-02 10 `ALIGNED`,
  1 `IMPLEMENTED_DIFFERENTLY`, 1 `PARTIALLY_IMPLEMENTED`, 5
  `STALE_SETUP_SPECIFICATION`, 1 `REMAINING_STATE_MISMATCH`; DEL-09-03 15
  `ALIGNED`, 4 `STALE_SETUP_SPECIFICATION`, 2 `REMAINING_STATE_MISMATCH`;
  DEL-09-04 21 `ALIGNED`, 3 `STALE_SETUP_SPECIFICATION`, 2
  `ACCEPTED_DIVERGENCE`; DEL-09-05 25 `ALIGNED`, 5
  `STALE_SETUP_SPECIFICATION`. Notes histograms reproduce.
- Frozen evidence facts independently checked include the DEL-09-01 mechanics
  fixture/witness census (21 fixtures and 21 witness notes), the DEL-09-04
  validation-manual census (63 case files), DEC-026's analytic-seed/TBD split,
  DEC-046's surviving unmeasured convergence entries, DEC-054's explicit
  carry-forward of the §16.2/§16.5 evidence systems, DEC-058's owner-signatory
  boundary, DEC-060's floor-promotion mechanism without a numeric floor, and
  DEC-062's owner-paced issuance waves.
- Before and after the reads, frozen-tree `git status --porcelain
  --ignored=matching` contained only the six addendum-9 allow-listed incident
  paths: `.pytest_cache/`, the two reporting `Cargo.lock` files, two
  `__pycache__/` directories, and `validation/benchmarks/nonlinear/target/`.
  This verifier performed read-only inspection and no build, test,
  `py_compile`, or in-tree mutation.

## 2. Per-ledger verification

### 2.1 DEL-09-01 — DEFECTIVE

| ClaimID(s) | Result | Independent finding |
|---|---|---|
| REQ-001 | **FAIL** | The structural technical claim rests on crate source, tests, and reviews, so addendum 6 requires `SourceReliability=UNVERIFIED` at its weakest technical leg; `NOT_APPLICABLE` is reserved for prose-only declaration rows. |
| REQ-002 | PASS | The implemented fixture/schema mechanism and disclosed contract grain resolve in frozen evidence. |
| REQ-004, ACC-004 | PASS | The partially implemented mechanics-suitability/coverage boundary is supported by the fixture and witness-note evidence and does not promote a numeric adequacy ruling. |
| ACC-001..003, ACC-005..006 | PASS | Acceptance census and artifact-presence claims resolve; the evidence remains at the disclosed unreviewed agent-evidence grain. |
| ACC-007 | **FAIL** | Its `RemainingWork` incorrectly calls `DECL-007/DECL-006` the runner/tolerance homes. Those are census rows; the row's own implementation evidence and notes correctly identify DEL-10-05/DEL-09-04. |
| DECL-001..004 | PASS | The four setup-era declarations are overtaken by the implemented mechanics benchmark slice and are correctly `STALE_SETUP_SPECIFICATION`. |
| REM-001 | PASS | `ACCEPTED_DIVERGENCE` is expressly grounded in DEC-054's carry-forward of the complete §16.2 evidence system. |
| REM-002 | PASS | The frozen `_STATUS` residual exists and its mismatch/homing treatment is supported by current runner/tolerance records. |

Tally: **14 PASS / 0 QUALIFIED / 2 FAIL**.

### 2.2 DEL-09-02 — DEFECTIVE

The semantic dispositions below are otherwise supportable, but ten rows and
the notes contain double-decoded UTF-8 sequences such as `â`, `â`, `Â§Â§`,
and `â`. A governed CSV/notes package cannot pass with corrupted authoritative
strings, so every affected row is scored FAIL.

| ClaimID(s) | Result | Independent finding |
|---|---|---|
| REQ-002, REQ-003 | **FAIL** | SECURITY classifications, OWNER routing, and no-downgrade handling are semantically sound; their strings are encoding-corrupted. |
| REQ-004, REQ-006 | **FAIL** | Frozen evidence supports the encoded mechanisms, but the row strings are encoding-corrupted. |
| REQ-005 | PASS | The unit-related claim is correctly routed OWNER/MEDIUM while human disposition remains pending; no human ruling is inferred. |
| REQ-007 | **FAIL** | `IMPLEMENTED_DIFFERENTLY` is supported by DEC-026's analytic `1e-9` seed and retained unmeasured-pair TBD, but the row string is encoding-corrupted. |
| REQ-008 | **FAIL** | `PARTIALLY_IMPLEMENTED` is supported by the still-open result-envelope integration, but the row string is encoding-corrupted. |
| EXC-001, EXC-002 | **FAIL** | The exclusion boundaries are semantically supportable; their strings are encoding-corrupted. |
| DECL-001..004 | PASS | Setup-era future-state declarations are correctly stale against the implemented slice. |
| DECL-005 | **FAIL** | Its semantic census treatment is supportable, but the row string is encoding-corrupted. |
| DECL-007 | PASS | README census grain follows the binding W1–W3 calibration. |
| REM-001 | **FAIL** | The remaining-state mismatch is defensible from the frozen status/readiness evidence, but the row string is encoding-corrupted. |

Tally: **6 PASS / 0 QUALIFIED / 10 FAIL**.

### 2.3 DEL-09-03 — SOUND

| ClaimID(s) | Result | Independent finding |
|---|---|---|
| REQ-002, REQ-006 | PASS | Both pending-human findings are encoded under addendum 13 without treating the pending disposition as approval or rejection. |
| REQ-003 | PASS | The SECURITY marker is tied to the protected-content/review-sufficiency judgment, uses OWNER routing, and does not force a lower disposition. |
| ACC-001 | PASS | The acceptance claim is correctly bounded to the recorded artifact/evidence grain. |
| DECL-001..004 | PASS | All four setup-era declarations are demonstrably overtaken and correctly stale. |
| DECL-007 | PASS | The README is correctly included at the self-identifying deliverable census grain required by calibration. |
| REM-001, REM-002 | PASS | Both frozen remaining items are absent or differently represented in current status and are correctly encoded as mismatches; neither creates a hidden threshold ruling. |

Tally: **11 PASS / 0 QUALIFIED / 0 FAIL**.

### 2.4 DEL-09-04 — SOUND

| ClaimID(s) | Result | Independent finding |
|---|---|---|
| REQ-010 | PASS | No SECURITY marker is required for the deterministic public/private contract itself; the row does not make a sufficiency or protected-content-release judgment. |
| ACC-001..005 | PASS | The complete acceptance census is supported at the disclosed validation-manual/evidence-record grain; the frozen manual contains 63 case files. |
| DECL-001, DECL-002, DECL-004 | PASS | The three future/setup declarations are overtaken and correctly stale. |
| REM-001 | PASS | DEC-054 expressly carries the complete §16.5 evidence system forward, supporting `ACCEPTED_DIVERGENCE`. |
| REM-002 | PASS | DEC-046 expressly leaves unmeasured convergence entries TBD, supporting `ACCEPTED_DIVERGENCE` without inventing a threshold. |

Tally: **11 PASS / 0 QUALIFIED / 0 FAIL**.

### 2.5 DEL-09-05 — SOUND

| ClaimID(s) | Result | Independent finding |
|---|---|---|
| REQ-002 | **QUALIFIED** | The aggregate clean-sweep gate record supports the encoded process boundary but is not candidate-specific validation; the row discloses that limitation and does not over-promote reliability. |
| REQ-003, REQ-008 | PASS | SECURITY markers are properly limited to owner-gated sufficiency/release judgments, route to OWNER, and preserve the no-downgrade rule. |
| REQ-012 | PASS | The human-record contract is present; the row does not manufacture a human disposition. |
| DECL-001..004, DECL-006 | PASS | Each setup-era declaration is demonstrably overtaken and correctly stale; MEMORY census treatment follows the undated/current-state calibration. |
| REM-003 | PASS | Owner-paced issuance is supported by DEC-062 and correctly remains a routed project action, not an agent ruling. |

Tally: **9 PASS / 1 QUALIFIED / 0 FAIL**.

## 3. Exact correction routing

### 3.1 DEL-09-01 owning pilot — two cell corrections

1. `DEL-09-01-REQ-001`: change `SourceReliability` from `NOT_APPLICABLE` to
   `UNVERIFIED`. Preserve the remaining 19 cells.
2. `DEL-09-01-ACC-007`: in `RemainingWork`, replace the false
   `DECL-007/DECL-006` home references with the actual homes
   `DEL-10-05/DEL-09-04`, conforming the cell to its own
   `ImplementationEvidence` and the notes. Preserve the disposition and all
   other cells.
3. Update the notes only as needed to record the two corrections and corrected
   fan-in outcome; recount histograms from the CSV. No evidence rerun or
   rediscovery is required.

### 3.2 DEL-09-02 owning pilot — artifact-wide encoding normalization

Normalize the CSV and notes from double-decoded/mojibake text to the intended
UTF-8 characters while preserving semantic content, cell boundaries, the
20-column header, and RFC-4180 CRLF. The affected CSV rows independently found
are `REQ-002`, `REQ-003`, `REQ-004`, `REQ-006`, `REQ-007`, `REQ-008`,
`EXC-001`, `EXC-002`, `DECL-005`, and `REM-001`; normalization must be
artifact-wide rather than limited to this list because the notes are affected
too. Confirm that no sequences `â`, ``, ``, ``, or `Â` remain. Preserve
dispositions and substantive wording. No evidence rerun or rediscovery is
required.

### 3.3 Return gate

After both owning pilots correct their artifacts, the orchestrator should run
the full PKG-09 structural validator again, recount all CSV-derived histograms,
confirm DEL-09-01's two exact cell changes, scan DEL-09-02 CSV/notes for
encoding remnants, and repeat the addendum-9 frozen porcelain check with
`--ignored=matching` and the six incident paths allow-listed. If those checks
pass, the correction-only defects are closed; no fresh discovery dispatch is
indicated.

## 4. Package result and forward carry

PKG-09 is **DEFECTIVE pending owning-pilot correction** because DEL-09-01 and
DEL-09-02 contain correctable governed-string defects. DEL-09-03..05 are
SOUND. Carry to R3: DEL-09-05 REQ-002 remains a disclosed evidence-grain
qualification; it is not a repair request and does not authorize a lifecycle
transition. No R4/R5 work is authorized by this verification.
