# W5 Fan-in Verification — PKG-15 (Handoff and External Prover Workflow)

Verifier: independent highest-available-capability GPT-5 fan-in verifier,
high-effort adversarial pass. This verifier did not pilot a PKG-15 ledger.
Scope: every self-flagged row, every non-`ALIGNED` row, and at least two
`ALIGNED` rows in each of `DEL-15-01..04`, checked against frozen source
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`, pinned plan §§6–8,
`R1_CONVENTIONS.md`, and PKG-00..14 calibration. This verifier edited no pilot
ledger or notes file. All dispositions remain agent judgments, never owner,
engineering, professional, legal, security, validation, compatibility, or
release rulings.

**Pre-correction verdicts: DEL-15-01 SOUND · DEL-15-02 DEFECTIVE · DEL-15-03
SOUND · DEL-15-04 SOUND.** Initial checked-row tally: **49 PASS / 0 QUALIFIED /
3 FAIL** across 52 scoped rows. The three failures were one deterministic
addendum-13 confidence correction cluster. The owning pilot corrected all
three without a discovery rerun or any ClaimType, Disposition, AuthorityNeeded,
or selectability change; §4 records the independent closure backcheck.

## 1. Mechanical, reproducibility, and containment checks

- All four CSVs parse at exactly 20 columns with the adopted header and
  RFC-4180 CRLF. ClaimIDs are deliverable-prefixed, type-matched,
  three-digit-padded, and contiguous within ClaimType. Controlled vocabularies,
  declaration reliability, recorded-item gate defaults, and selectability
  checks pass.
- The ledgers contain **117 rows** (31/31/30/25). Aggregate ClaimType
  histogram: REQUIREMENT 44 / ACCEPTANCE 23 / EXCLUSION 16 / DECLARED_STATE
  24 / REMAINING_WORK 10. Aggregate disposition histogram: ALIGNED 95 /
  ACCEPTED_DIVERGENCE 2 / PARTIALLY_IMPLEMENTED 2 /
  STALE_SETUP_SPECIFICATION 9 / REMAINING_STATE_MISMATCH 9. Every per-ledger
  type/disposition histogram in the notes reproduces.
- Pre-correction confidence recount: HIGH 102 / MEDIUM 15. Authority recount:
  NO 87 / OWNER 29 / ENGINEERING 1. SourceReliability recount: UNVERIFIED 93 /
  NOT_APPLICABLE 24. The confidence defect identified below changes only the
  package confidence recount to HIGH 99 / MEDIUM 18.
- Each declaration census is exactly the four-document kit plus `_STATUS.md`
  and `MEMORY.md`; no mapped owned module has a README. Bootstrap text is
  byte-exact, appears only on `DECL-005`, and is excluded from residual, gate,
  and selectability analysis. All 117 rows are mechanically `NO` because the
  only substantive recorded program item, DEL-15-01 Phase H, is
  `stage-gated: v0.2 R6` (with the additional DEL-17-01 vendor-question gate).
- Exact convention-6 SECURITY sufficiency-deferral marker count is zero.
  SECURITY and professional/data-boundary rows uniformly use explicit-reason
  `NOT_APPLICABLE`; mechanics rows that explicitly keep conversion suitability
  open use `NONE_FOUND` without manufacturing a SECURITY marker.
- Independent verifier re-execution used `PYTHONDONTWRITEBYTECODE=1`, external
  `PYTHONPYCACHEPREFIX`, and pytest `-p no:cacheprovider`. The direct
  handoff-schema, target-mapping-contract, and external-prover-boundary scripts
  passed; the export-workflow pytest suite produced **6 passed**; all three
  JSON schemas parsed. No Cargo command or in-tree `py_compile` ran.
- Frozen ignored-aware porcelain before and after contained exactly the six
  addendum-9 allow-listed incident paths and no seventh path: project
  `.pytest_cache/`; two reporting `Cargo.lock` files; reporting and tests
  `__pycache__/`; and nonlinear benchmark `target/`. Tracked porcelain stayed
  empty. The frozen worktree was neither cleaned nor modified.

## 2. Per-ledger scoped verification

### DEL-15-01 — SOUND (16 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all eight non-ALIGNED rows (`REQ-012`, `ACC-003`,
`DECL-001..004`, `REM-002..003`) plus aligned/self-flagged `REQ-004`,
`REQ-006`, `REQ-011`, `ACC-004`, `EXC-003..004`, `DECL-005`, and `REM-001`.

- REQ-012/ACC-003 satisfy addenda 5 and 11. DEC-028 is a named human ruling
  selecting a multi-member archive strategy while expressly leaving member
  ordering, atomic save, compatibility, and related mechanics bounded. The
  current `container=TBD` contract is therefore an accepted bounded divergence,
  not evidence that physical packaging is complete.
- DECL-001..004 each continue to state that the container strategy itself is
  unresolved. STALE is correctly surface-local; target-neutrality, concrete
  mapping/coverage gates, and surviving physical-mechanics deferrals remain
  valid.
- REM-002/003 accurately preserve technically addressed but formally OPEN/TBD
  review findings omitted from `_STATUS.md`. MEDIUM/OWNER does not infer
  disposition from lifecycle advancement. REM-001 and DECL-005 correctly keep
  the stage-gated Phase H residual separate from bootstrap mechanics.
- Unit disclosure is ALIGNED only at preservation grain. Negative
  professional/protected-content and provider-compatibility boundaries are
  appropriately marker-free and do not claim conversion validation, runtime
  redaction, target suitability, or external execution.

### DEL-15-02 — DEFECTIVE (11 PASS / 0 QUALIFIED / 3 FAIL)

Scoped rows: all seven non-ALIGNED rows (`REQ-011`, `DECL-002..004`,
`REM-001..003`) plus aligned `REQ-001`, `REQ-005`, `REQ-008..009`, `ACC-002`,
`ACC-003`, and `EXC-004`.

- REQ-011 correctly isolates a privacy-metadata versus runtime-enforcement
  gap: schema shape and strict payload boundaries exist, but the builder copies
  caller privacy context without enforcing redaction or diagnosing unsafe
  embedded-payload flags. PARTIALLY_IMPLEMENTED with explicit-reason
  `NOT_APPLICABLE` does not assert privacy/legal sufficiency.
- REQ-009/ACC-002 correctly apply addendum 13 to the load-bearing
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN` package-audit finding: MEDIUM/OWNER
  while independent current code/tests support ALIGNED technical substance.
- DECL-002..004 correctly identify live revision-0.7/DAG-006 authority framing;
  DECL-001 correctly treats legacy local row IDs as bounded derivative mirror
  evidence rather than current graph authority. SECURITY rows are restrained.
- **FAIL — REM-001, REM-002, and REM-003 encode `Confidence=HIGH`.** All three
  rows describe technically addressed findings whose human disposition is
  still pending and whose status/home remains absent. REM-001 is literally
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN`; RF-001/RF-002 remain OPEN with
  `HumanDisposition=TBD`. Binding addendum 13 and the PKG-13..15 sibling
  calibration cap this evidence species at MEDIUM and route OWNER. The notes
  themselves state that addendum 13 yields MEDIUM+OWNER, contradicting their
  own HIGH 25 / MEDIUM 6 histogram.

Owning-pilot correction: change only `Confidence` on REM-001/002/003 from HIGH
to MEDIUM and update the notes confidence histogram to HIGH 22 / MEDIUM 9.
Keep Disposition `REMAINING_STATE_MISMATCH`, AuthorityNeeded OWNER, all row
text, and every other histogram unchanged; then revalidate CRLF and package
counts.

### DEL-15-03 — SOUND (13 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all five non-ALIGNED rows (`REQ-008`, `DECL-002`, `REM-001..003`)
plus self-flagged/aligned `REQ-004`, `REQ-006..007`, `ACC-002`, `ACC-005..006`,
and `EXC-003..004`.

- REQ-008 correctly records a compound architecture/canonicalization gap.
  Provider-neutral Python workflow behavior and schema-first validation exist,
  but the requirement names a Rust core/application-service basis; stable
  sorted compact JSON and preserved supplied checksum labels do not prove
  RFC-8785/JCS conformance. PARTIALLY_IMPLEMENTED/ENGINEERING preserves both
  facts without declaring the implementation unacceptable.
- DECL-002 carries explicit revision-0.7/DAG-006 current-source wording and is
  correctly STALE. The other four active documents are aligned on their
  post-remediation current facts; dated MEMORY entries remain historical.
- REM-001..003 use MEDIUM/OWNER for the same technically-addressed,
  human-undispositioned species that DEL-15-02 must normalize. They do not
  recast the findings as current product defects.
- Unit/hash preservation, invented-fixture evidence, prohibited-wording
  diagnostics, and unsupported-target behavior are ALIGNED only at the current
  provider-neutral workflow grain. They do not establish target conversion,
  checksum correctness, physical packaging, commercial compatibility,
  external execution, or professional reliance.

### DEL-15-04 — SOUND (9 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: both non-ALIGNED rows (`DECL-002`, `REM-001`) plus aligned
`REQ-001`, `REQ-006..008`, `ACC-001`, `ACC-003`, and `EXC-003`.

- DECL-002's Datasheet explicitly presents revision 0.7 as accepted current
  decomposition, so STALE/OWNER follows the binding current-pointer rule. The
  other declaration surfaces state current implementation facts or protected
  dated history.
- REM-001 correctly preserves technically repaired RF-001 as
  OPEN/HumanDisposition-TBD with MEDIUM/OWNER. The separately human-accepted
  `DEL-15-04-PKG02-001` row is RESOLVED and correctly omitted from the REM
  census.
- REQ-007/ACC-003 correctly encode a conditional negative boundary: no
  automatic acceptance record is created; any future human acceptance remains
  external, human-owned, and hash-bound. No actual acceptance is inferred.
- Unit-policy and external-prover metadata remain disclosure-only. Invented
  fixtures, negative authority checks, and rejected probe text are not
  commercial compatibility, external verification, legal clearance, or
  security/professional assurance.

## 3. Correction route and R3 carry-forward

1. DEL-15-02 correction route (closed in §4): the owning pilot normalized
   REM-001..003 confidence to MEDIUM and updated notes confidence counts; no
   discovery rerun or disposition change.
2. Preserve ownership grain: DEL-15-01 owns the canonical handoff envelope,
   DEL-15-02 target mapping/unsupported-behavior vocabulary, DEL-15-03 the
   provider-neutral runtime export workflow, DEL-15-04 descriptive
   external-prover boundary metadata, and DEL-17 the wire-format adapters.
3. Deduplicate DEC-028 strategy resolution from surviving archive mechanics.
   A ruled multi-member archive is not evidence of deterministic package
   construction, target adapters, atomic save, compatibility, or release.
4. Join JCS/RFC-8785 concerns with PKG-08/14 once: stable JSON serialization,
   preservation of supplied checksum labels, schema-permitted canonicalization
   vocabulary, and actual conformance are different evidence grains.
5. Unit metadata preservation and `conversion_performed=false` witnesses do
   not validate target-specific conversion correctness or engineering
   suitability. Preserve this distinction across PKG-14/15/17.
6. Deduplicate the pending finding state/home species by underlying finding,
   not by REM-row count. Technical remediation, lifecycle recommendation, and
   formal human disposition remain separate.
7. Treat DAG-002/DAG-006 local rows as derivative mirror evidence under current
   DAG-007 authority. ACTIVE/schema-valid mirrors and TBD satisfaction cells do
   not establish dependency closure or authorize a register edit.
8. Rejected professional-authority probe text is diagnostic test input, not an
   accepted output state. Provider-neutral flags and metadata do not prove
   target equivalence, commercial parser coverage, or external-solver results.

No lifecycle transition, product repair, dependency/DAG/register mutation,
decision packet, R4/R5 work, external validation, approval, certification,
sealing, authentication, compliance determination, legal clearance, security
assurance, commercial compatibility, or release-readiness action was
performed.

## 4. Post-correction independent backcheck and final verdict

The DEL-15-02 owning pilot changed only REM-001/002/003 `Confidence` from HIGH
to MEDIUM and updated the notes histogram to HIGH 22 / MEDIUM 9. Independent
backcheck confirms all three rows retain `REMAINING_STATE_MISMATCH`, OWNER,
their original evidence and work text, and non-selectable status. This exactly
resolves the addendum-13 failures without substantive judgment drift.

Package revalidation reconfirms four exact-header, 20-column RFC-4180 CRLF
ledgers; 117 rows; controlled enums; type-matched contiguous ClaimIDs;
declaration/bootstrap/selectability rules; unchanged ClaimType and Disposition
histograms; and corrected package confidence HIGH 99 / MEDIUM 18.
`git diff --check` passes for the corrected ledger and notes. Frozen
ignored-aware porcelain still contains exactly the six addendum-9 allow-listed
incident paths and no seventh path. The verifier made no pilot-artifact edit.

**FINAL VERDICTS: DEL-15-01 SOUND · DEL-15-02 SOUND · DEL-15-03 SOUND ·
DEL-15-04 SOUND. Final checked-row tally: 52 PASS / 0 QUALIFIED / 0 FAIL.
PKG-15 fan-in is CLOSED.** R3 carry-forward observations remain evidence-only
aggregation cautions and authorize no repair or governed-state change.
