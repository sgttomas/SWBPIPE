# W4 Fan-in Verification — PKG-12 (Security, Privacy, and Private Data Handling)

Verifier: highest-available-capability GPT-5 orchestrator, high-effort
adversarial pass. Scope: all self-flagged rows, every non-`ALIGNED` row, and
at least two `ALIGNED` rows in each of `DEL-12-01..05`, checked against the
frozen worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7` under
`R1_CONVENTIONS.md`, the W1–W3 package calibration, and the addendum-9
containment controls. Dispositions remain agent judgments, never owner,
engineering, legal, or security rulings. No ledger was edited by this
verifier.

**Verdicts: DEL-12-01 SOUND · DEL-12-02 DEFECTIVE · DEL-12-03 DEFECTIVE ·
DEL-12-04 SOUND · DEL-12-05 SOUND.** Checked-row tally: **26 PASS / 2
QUALIFIED / 28 FAIL** across 56 scoped rows. The failures are deterministic
owning-pilot corrections; no discovery rerun is required.

## 1. Mechanical and containment checks

- All five CSVs parse at 20 columns with the byte-exact run header and RFC-4180
  CRLF. After the owning-pilot correction in commit `13d85560b`, every enum is
  controlled and every ClaimID is type-matched and contiguous. Total: 131
  rows (26/33/20/28/24).
- Recounted ClaimType and Disposition histograms reproduce the five notes:
  12-01 types 12/1/5/6/2 and dispositions 19/4/1/2; 12-02 types
  15/8/3/6/1 and dispositions 17/12/3/1; 12-03 types 10/0/2/6/2 and
  dispositions 10/5/2/1/2; 12-04 types 10/7/3/6/2 and dispositions
  19/7/2; 12-05 types 12/0/6/6/0 and dispositions 20/1/3.
- The declaration census is four kit documents plus `_STATUS.md` and
  `MEMORY.md` in every ledger. Bootstrap text occurs only on the status row
  and is excluded from substantive residual analysis.
- Independent verifier re-execution used
  `PYTHONDONTWRITEBYTECODE=1 pytest -p no:cacheprovider` over all five PKG-12
  security suites: **52 passed**. Ignored-aware porcelain before and after
  showed exactly the six incident allow-list paths and no seventh path.

## 2. Per-ledger verification

### DEL-12-01 — SOUND (12 PASS / 1 QUALIFIED / 0 FAIL)

- All seven non-ALIGNED rows were checked against the kit, `Review_Findings`,
  metadata-only storage guard, policy, and tests. REQ-007/009/010/011 are
  bounded implementation or validation breadth gaps; DECL-005 and REM-001/002
  accurately preserve the stale status wording and open, unhomed findings.
- The convention-6 marker is confined to REQ-001, REQ-005, and REM-002. The
  two requirement rows are `ALIGNED` only at policy/metadata-guard grain and
  explicitly disclaim runtime-storage sufficiency; this is consistent with
  the no-downgrade clause. **QUALIFIED** because this is an interpretive grain
  choice to retain for R3, not a factual defect.
- Extra ALIGNED samples REQ-002 and REQ-004 reproduced against the guard and
  focused tests. No F-PIP-1 overclaim was found.

### DEL-12-02 — DEFECTIVE (4 PASS / 0 QUALIFIED / 25 FAIL)

- The twelve PARTIALLY_IMPLEMENTED rows, three stale declaration rows, the
  implemented-vocabulary divergence, and REM-001 all match the frozen
  implementation/kit breadth. The R0 unmapped export-review row is correctly
  absent because R1 maps the surface.
- **FAIL — 25 rows carry a contradictory execution statement.** Their
  `VerificationEvidence` first says the W4 pilot did not re-execute, then says
  the 11-test suite was “re-executed by this pilot.” The notes correctly say
  there was no W4 re-execution. Owning pilot must retain the R0 reviewer’s
  same-SHA 11-pass attribution and delete only the false W4-pilot clause.
- **FAIL — 11 of those rows carry `UNGATED` with
  `SelectableUnderCurrentLoop=NO`.** Under convention 6/addendum 3, the
  rows touching the recorded ungated breadth residual must be `YES`, matching
  DECL-005 and REM-001. Owning pilot must change only those 11 cells and
  update the notes selectability histogram from YES 2 / NO 31 to YES 13 / NO
  20.
- The exact convention-6 marker occurs only on REQ-003 and is correctly
  OWNER-routed; other SECURITY rows use deterministic-boundary language.

### DEL-12-03 — DEFECTIVE (3 PASS / 0 QUALIFIED / 3 FAIL)

- All ten non-ALIGNED rows match the frozen metadata-only helper, absent
  runtime/config/consent surfaces, stale Datasheet authority pointer, and open
  unhomed RF-001/RF-002 findings. REQ-006 and REQ-009 are correctly
  DOCUMENTED_UNIMPLEMENTED rather than vacuously aligned.
- **FAIL — REQ-004, EXC-002, and REM-002 use a non-standard marker**
  (`NONE_FOUND — SECURITY/privacy sufficiency review deferred, owner-gated`).
  R1 convention 6 adopted the exact string `NONE_FOUND — sufficiency review
  deferred, owner-gated`. Owning pilot must remove only the inserted
  `SECURITY/privacy ` text in those three cells and make the corresponding
  notes wording exact. Dispositions and routing do not change.
- The earlier off-enum `REMAINING_ITEM` values were already corrected by the
  owning pilot to `REMAINING_WORK`; this verifier confirmed the current file.

### DEL-12-04 — SOUND (4 PASS / 1 QUALIFIED / 0 FAIL)

- All seven PARTIALLY_IMPLEMENTED rows and both mismatch rows were checked
  against the deterministic metadata helper, tests, kit, and open review
  findings. Runtime/storage/provider/grant/quarantine breadth is not
  laundered into completion.
- REQ-006 alone carries the exact sufficiency marker because destructive
  quarantine/human review remains outside the deterministic helper. This is
  correct. **QUALIFIED** only for the disclosed aggregation grain: the other
  partial rows use NOT_APPLICABLE validation because they make bounded
  technical, not sufficiency, claims.
- ALIGNED samples REQ-003 and ACC-001 resolve and match the frozen tests/kit.

### DEL-12-05 — SOUND (3 PASS / 0 QUALIFIED / 0 FAIL)

- REQ-009 is correctly PARTIALLY_IMPLEMENTED: `IP_BOUNDARY_WARNING` is named,
  while provenance/assumption signals are not explicitly bound to named
  warning classes and the focused test does not cover STM-REQ-009.
- DECL-001..003 correctly isolate setup-era document framing from the aligned
  product threat-model substance. Procedure, status, and MEMORY are aligned.
- Zero convention-6 marker rows is correct for this documentation-content
  ledger: the rows verify stated threat-model content and exclusions without
  claiming that controls are implemented or security-sufficient. ALIGNED
  samples REQ-001 and REQ-010 passed source and negative-claim checks.

## 3. Correction routing and R3 carry-forward

1. DEL-12-02 owning pilot: remove the false W4 re-execution clauses from 25
   `VerificationEvidence` cells; change 11 `UNGATED` rows from selectability
   NO to YES; update notes counts. No disposition changes.
2. DEL-12-03 owning pilot: normalize the convention-6 string in three cells
   and notes. No disposition changes.
3. R3 must deduplicate the repeated unhomed RF-001/RF-002 families across
   DEL-12-01/03/04, preserve separately homed implementation breadth, and not
   interpret policy-/contract-grain ALIGNED rows as security assurance.
4. DEL-12-05’s warning-class gap and DEL-12-01’s rule-pack version/runtime
   breadth remain evidence findings, not R4/R5 authorization.

No lifecycle transition, register/DAG mutation, product edit, private-data
write, security assurance, certification, professional approval, or R4/R5
repair was performed.
