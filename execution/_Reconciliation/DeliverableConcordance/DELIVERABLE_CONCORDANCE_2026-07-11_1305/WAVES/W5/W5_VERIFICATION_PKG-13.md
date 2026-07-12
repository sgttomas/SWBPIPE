# W5 Fan-in Verification — PKG-13 (Physical Design Knowledge and Constraint Engine)

Verifier: independent highest-available-capability GPT-5 fan-in verifier,
high-effort adversarial pass. Scope: all self-flagged rows, every
non-`ALIGNED` row, and at least two `ALIGNED` rows in each of
`DEL-13-01..04`, checked against the frozen evidence worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7` under `R1_CONVENTIONS.md`, the
PKG-00..12 cross-ledger calibration, and the addendum-9 containment controls.
Dispositions remain agent judgments, never owner, engineering, legal,
security, or professional rulings. This verifier did not edit a ledger or
notes file.

**Verdicts: DEL-13-01 SOUND · DEL-13-02 SOUND · DEL-13-03 DEFECTIVE ·
DEL-13-04 DEFECTIVE.** Checked-row tally: **33 PASS / 0 QUALIFIED / 4 FAIL**
across 37 scoped rows. The failures are deterministic owning-pilot
corrections; no discovery rerun is required.

## 1. Mechanical and containment checks

- All four CSVs parse at 20 columns with the byte-exact run header and
  RFC-4180 CRLF. ClaimIDs are type-matched and contiguous. The ledgers contain
  97 rows (20/30/16/31).
- Recounted ClaimType and Disposition histograms reproduce the four notes:
  13-01 types 11/0/3/6/0 and dispositions 18 ALIGNED / 1 PARTIAL / 1 STALE;
  13-02 types 10/8/5/6/1 and dispositions 28 ALIGNED / 1 STALE / 1 REMAINING
  STATE MISMATCH; 13-03 types 8/0/2/6/0 and dispositions 12 ALIGNED / 1
  PARTIAL / 3 STALE; 13-04 types 12/9/4/6/0 and dispositions 28 ALIGNED / 2
  PARTIAL / 1 VERIFIED NOT VALIDATED. Confidence, authority, and
  selectability recounts also reproduce every count stated in the notes.
- Each declaration census is the four-document kit plus `_STATUS.md` and
  `MEMORY.md`; no deliverable-owned README exists. Bootstrap text occurs only
  on the status declaration row and is excluded from substantive residual and
  selectability analysis.
- Independent verifier re-execution used external Python cache placement,
  `PYTHONDONTWRITEBYTECODE=1`, and pytest `-p no:cacheprovider`.
  `test_design_knowledge_schema.py` and `test_constraint_schema.py` passed by
  direct execution; the constraint-validation, transform, and solver-boundary
  suites produced **26 passed**. No Cargo command or in-tree `py_compile` was
  used.
- Frozen ignored-aware porcelain before and after showed exactly the six
  addendum-9 incident allow-list paths and no seventh path.

## 2. Per-ledger verification

### DEL-13-01 — SOUND (9 PASS / 0 QUALIFIED / 0 FAIL)

- REQ-005 and REQ-006 correctly apply addendum 13: current schema and focused
  tests independently establish the substance, while the load-bearing PKG-02
  findings remain pending human disposition, so MEDIUM/OWNER is required
  without changing ALIGNED.
- REQ-011 is correctly PARTIALLY_IMPLEMENTED. The governed schema carries the
  no-bypass vocabulary, while the product-preview loader performs a JSON read
  of a separate simplified fixture shape without validation against the
  design-knowledge schema. The row remains bounded and does not indict every
  downstream consumer.
- DECL-004 accurately isolates the current Procedure revision-0.7 pointer as
  STALE; the dated MEMORY entry remains historical. DECL-003's deliverable-
  scoped no-public-example statement is not falsified by downstream preview
  fixtures.
- SECURITY rows REQ-003 and EXC-001 correctly use explicit-reason
  `NOT_APPLICABLE`, not the convention-6 marker: these are bounded artifact-
  content/exclusion checks, not accepted sufficiency deferrals. Additional
  ALIGNED samples REQ-001 and REQ-010 resolve to the schema and passing test.

### DEL-13-02 — SOUND (9 PASS / 0 QUALIFIED / 0 FAIL)

- DECL-004 correctly confines rev-pointer staleness to the Procedure surface;
  REM-001 accurately records the unhomed pending human disposition without
  recasting it as a technical failure or accepted divergence.
- REQ-005 and ACC-004 are correctly MEDIUM/OWNER because the PKG-02 finding is
  load-bearing, while the exact dimension-vocabulary test independently
  supports ALIGNED at schema-contract grain.
- SECURITY rows REQ-006, ACC-005, and EXC-003 correctly use bounded
  explicit-reason `NOT_APPLICABLE` and make no legal/security sufficiency
  claim. Additional ALIGNED samples REQ-001 and REQ-007 match the schema and
  focused test.

### DEL-13-03 — DEFECTIVE (6 PASS / 0 QUALIFIED / 2 FAIL)

- REQ-006 correctly records the absent formal application result-envelope
  mapping/runtime route as PARTIALLY_IMPLEMENTED. DECL-001 and DECL-002
  correctly encode current revision-0.7 authority pointers as STALE, and
  DECL-004 correctly treats the Procedure's in-tree `py_compile` instruction
  as overtaken by binding addendum 9.
- Additional ALIGNED samples REQ-002 and REQ-003 reproduce against the
  deterministic diagnostic serialization and explicit missing-data tests.
- **FAIL — REQ-005 and EXC-001 use `NONE_FOUND — ...` in
  `ValidationEvidence` for deterministic protected/private boundary checks.**
  The binding PKG-10/11/12 calibration requires deterministic negative guards,
  protected-content scans, and invented-fixture boundaries to use an
  explicit-reason `NOT_APPLICABLE`; the exact convention-6 `NONE_FOUND`
  marker is reserved for an expressly accepted owner-gated sufficiency
  deferral. These rows disclaim sufficiency and identify no accepted
  deferral, so their current prefix conflicts with the binding encoding even
  though their ALIGNED dispositions and OWNER routing are substantively
  sound. The owning pilot must replace only these two `ValidationEvidence`
  cells with `NOT_APPLICABLE — deterministic negative boundary behavior at
  current module/invented-fixture grain; no security, legal, or redistribution
  sufficiency claim`. Update the notes' SECURITY-marker explanation to state
  that these two cells use explicit-reason `NOT_APPLICABLE`. No disposition,
  confidence, authority, or histogram changes.

### DEL-13-04 — DEFECTIVE (9 PASS / 0 QUALIFIED / 2 FAIL)

- REQ-004 and ACC-003 correctly preserve the object/DTO versus field-scalar
  traceability split as PARTIALLY_IMPLEMENTED. REQ-007 is correctly
  VERIFIED_NOT_VALIDATED at mechanics grain: implementation and tests
  establish the 3D centerline/frame target, but not a broader engineering
  validation basis.
- REQ-011 remains ALIGNED at rule grain: its no-independent-dispatch-authority
  rule survives the overtaken DAG/revision pointer. SECURITY rows REQ-008,
  ACC-006, and EXC-001 correctly use explicit-reason `NOT_APPLICABLE` and do
  not claim sufficiency. Additional ALIGNED samples REQ-001 and REQ-002 passed
  deterministic transform/diagnostic traces.
- **FAIL — DECL-001 and DECL-002 omit current revision-pointer drift.** The
  Specification's governing-standards table calls `SOFTWARE_DECOMP.md`
  revision 0.7 the accepted decomposition, and the Datasheet's sources list
  likewise identifies revision 0.7, while the frozen authority front matter
  is `current_basis`, revision 0.8. PKG-00's binding W2–W5 calibration requires
  STALE on every census declaration surface carrying this current pointer;
  DEL-13-03 applies that same rule to the equivalent two surfaces. The owning
  pilot must change DECL-001 and DECL-002 to
  `STALE_SETUP_SPECIFICATION`, set `AuthorityNeeded=OWNER`, describe the
  0.7-to-0.8 pointer drift in the row evidence, and record later authorized
  pointer refresh in `RemainingWork`. Update notes histograms to ALIGNED 26 /
  PARTIALLY_IMPLEMENTED 2 / VERIFIED_NOT_VALIDATED 1 /
  STALE_SETUP_SPECIFICATION 2 and authority to NO 25 / ENGINEERING 3 / OWNER
  3. Technical requirement/acceptance dispositions do not change.

## 3. Correction routing and R3 carry-forward

1. DEL-13-03 owning pilot: normalize REQ-005 and EXC-001 validation encoding
   to explicit-reason `NOT_APPLICABLE` and align the notes wording. No
   substantive disposition change.
2. DEL-13-04 owning pilot: encode the Specification and Datasheet revision
   pointers on DECL-001/002 as STALE, route OWNER, and update notes counts.
3. R3 should deduplicate the corpus-wide revision-0.7 pointer species while
   preserving the distinct affected declaration surfaces. It should also
   preserve the difference between pending-human-disposition rows and
   independently established technical substance.
4. Runtime/schema binding, formal result-envelope integration, field-level
   transform traceability, and mechanics validation remain separately homed
   technical work; none is R4/R5 authorization or a release/professional
   ruling.

No lifecycle transition, register/DAG/dependency mutation, product edit,
private-data write, security assurance, certification, sealing, professional
approval, code-compliance finding, or R4/R5 repair was performed.
