# W5 Fan-in Verification — PKG-17 (Export Format Interoperability)

Verifier: independent highest-available-capability GPT-5 fan-in verifier,
high-effort adversarial pass. This verifier did not pilot a PKG-17 ledger.
Scope: every self-flagged row, every non-`ALIGNED` row, and at least two
`ALIGNED` rows in each of `DEL-17-01..09`, checked against frozen source
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`, pinned plan §§6–8,
`R1_CONVENTIONS.md`, and PKG-00..16 calibration. This verifier edited no pilot
ledger or notes file. Dispositions remain agent judgments, never owner,
engineering, vendor, professional, legal, security, validation,
compatibility, target-support, or release rulings.

**Pre-correction verdicts: DEL-17-01 DEFECTIVE · DEL-17-02 SOUND · DEL-17-03
SOUND · DEL-17-04 DEFECTIVE · DEL-17-05 DEFECTIVE · DEL-17-06 SOUND ·
DEL-17-07 SOUND · DEL-17-08 DEFECTIVE · DEL-17-09 SOUND.** Checked-row tally:
**153 PASS / 0 QUALIFIED / 8 FAIL** across 161 scoped rows. All failures are
bounded convention/currentness corrections; none requires a discovery rerun.

## 1. Mechanical, reproducibility, and containment checks

- All nine CSVs parse at exactly 20 columns with the adopted header and
  RFC-4180 CRLF. Controlled enums, type-matched three-digit ClaimIDs, per-type
  sequence continuity, declaration reliability, and basic selectability checks
  pass.
- The ledgers contain **375 rows** (30/63/28/39/37/35/57/49/37). Aggregate
  ClaimType histogram: REQUIREMENT 163 / ACCEPTANCE 102 / EXCLUSION 35 /
  DECLARED_STATE 54 / REMAINING_WORK 21. Pre-correction disposition histogram:
  ALIGNED 313 / STALE_SETUP_SPECIFICATION 27 / STALE_REVIEW_OR_EVIDENCE 1 /
  PARTIALLY_IMPLEMENTED 26 / REMAINING_STATE_MISMATCH 4 /
  VERIFIED_NOT_VALIDATED 1 / ACCEPTED_DIVERGENCE 3. DEL-17-01's required
  declaration-currentness correction changes only ALIGNED to 312 and STALE to
  28.
- Confidence recount: HIGH 333 / MEDIUM 42. Authority recount before the
  DEL-17-01 correction: NO 300 / OWNER 65 / ENGINEERING 10; final is NO 299 /
  OWNER 66 / ENGINEERING 10. SourceReliability: UNVERIFIED 321 /
  NOT_APPLICABLE 54. All 375 rows remain mechanically non-selectable.
- Every declaration census is exactly the four-document kit plus `_STATUS.md`
  and `MEMORY.md`; no mapped implementation directory has a deliverable-owned
  README. Bootstrap text appears only on `DECL-005`. Pre-correction,
  DEL-17-04's string is not byte-exact and DEL-17-05/08 improperly retain
  bootstrap source/gate metadata; the required corrections are below.
- Exact convention-6 SECURITY sufficiency-deferral marker count is zero across
  28 SECURITY rows. Protected-content, professional-boundary, and negative
  target claims use restrained explicit-reason `NOT_APPLICABLE`; actual
  validation gaps use `NONE_FOUND`. DEL-17-05 uniquely uses invalid
  `UNVERIFIED —` prefixes in two ValidationEvidence cells; those must become
  `NONE_FOUND —` while SourceReliability remains UNVERIFIED.
- Independent verifier re-execution used `PYTHONDONTWRITEBYTECODE=1`, external
  `PYTHONPYCACHEPREFIX`, and pytest `-p no:cacheprovider`. The seven target
  package suites (native JSON, MBF, external-run/CSV, stress-neutral, PCF,
  review geometry, and adapter SDK) produced **64 passed**. No Cargo command or
  in-tree `py_compile` ran.
- Frozen ignored-aware porcelain before and after contained exactly the six
  addendum-9 allow-listed incident paths and no seventh path: project
  `.pytest_cache/`; two reporting `Cargo.lock` files; reporting and tests
  `__pycache__/`; and nonlinear benchmark `target/`. Tracked porcelain stayed
  empty. The frozen worktree was neither cleaned nor modified.

## 2. Per-ledger scoped verification

### DEL-17-01 — DEFECTIVE (14 PASS / 0 QUALIFIED / 1 FAIL)

Scoped rows: self-flagged `REQ-002..003`, `REQ-006..008`, `ACC-002`, `ACC-005`,
`REM-001..006`, `DECL-005`, and aligned declaration `DECL-004`.

- Source admission, citation-boundary, consumer-grain, and protected-content
  rows correctly remain UNVERIFIED and ALIGNED without asserting live vendor
  reachability, current external content, compatibility, legal clearance, or
  solver validation.
- REM-001..006 preserve six formal source-basis TBD species without
  double-counting the seven dossier questions. Their ALIGNED status means the
  controls remain visible; OWNER does not select or answer them under D-41.
- **FAIL — DECL-004 calls the Procedure validation instructions current and
  reproducible, but its commands cannot execute as written from one cwd.** At
  frozen repository root, `tools/validation/check_*` exists but
  `execution/PKG-17...` does not; at the project root, `execution/PKG-17...`
  exists but the two project-local `tools/validation/check_*` scripts do not.
  Invoking root tools with an absolute/project-prefixed deliverable path passes,
  but that is a corrected invocation, not the displayed command. Binding
  current-procedure calibration requires DECL-004
  `STALE_SETUP_SPECIFICATION`, AuthorityNeeded OWNER, in-row evidence naming
  the split-root failure, and RemainingWork limited to a later authorized
  path/cwd refresh. Notes histograms become ALIGNED 29 / STALE 1 and NO 23 /
  OWNER 7. No substantive source-basis row changes.

### DEL-17-02 — SOUND (50 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all 44 contract requirements, all four non-ALIGNED rows
(`DECL-002..004`, `REM-001`), and aligned `ACC-001..002`.

- All 44 requirements trace one-for-one to the human-readable common package,
  profile, manifest, stable-ID-map, loss-report, source, unit, provenance, and
  boundary contract. They are ALIGNED at contract-definition grain only;
  sampled downstream schemas/builders corroborate consumption without
  transferring implementation ownership to DEL-17-02.
- Contract alignment does not establish JCS fidelity, target compatibility,
  parser/writer completeness, runtime/API integration, professional reliance,
  or release readiness.
- DECL-002..004 contain verified lifecycle/revision/DAG current-pointer drift
  and are correctly STALE while preserving historical derivative records.
  REM-001 correctly uses STALE_REVIEW_OR_EVIDENCE: its RF-001 premise says the
  root validator tools are absent, but all named root tools exist and passed;
  the still-stale Procedure DAG instruction is a separate declaration fact.

### DEL-17-03 — SOUND (8 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: both non-ALIGNED rows (`REQ-006`, `REM-001`) plus self-flagged or
aligned `ACC-005`, `REQ-005`, `ACC-006`, `EXC-005`, `REQ-001`, and `ACC-001`.

- REQ-006 accurately distinguishes deterministic timestamp-free sorted compact
  JSON from unproved RFC-8785/JCS conformance. PARTIALLY_IMPLEMENTED/MEDIUM/
  ENGINEERING is an interoperability-evidence gap, not cryptographic or
  professional adjudication.
- REM-001 correctly records the open DAG artifact-flag mismatch as unhomed
  currentness work without authorizing a DAG/register edit. RF-001 is
  human-dispositioned and resolved, so it is correctly omitted.
- Import/fixture/boundary rows establish the bounded builder behavior only;
  they do not prove target compatibility or broaden negative authority claims.

### DEL-17-04 — DEFECTIVE (11 PASS / 0 QUALIFIED / 1 FAIL)

Scoped rows: all five REM rows, `ACC-008..009`, `DECL-001..002`, `ACC-001`,
`ACC-007`, and mandatory bootstrap `DECL-005`.

- All substantive rows are sound at bounded invented-foundation grain. Target
  version, record subset, direct ID carriage, specialized warning policy, and
  aggregate RF-002 remain visibly open; ALIGNED does not mean target support.
- Python/desktop, source/writer, unit/conversion, and lifecycle/review evidence
  are correctly separated. RF-001 is resolved and not duplicated.
- **FAIL — DECL-005 does not copy the frozen `_STATUS.md` bootstrap.** It adds
  the full SHA plus `(ruled: D-41, 2026-07-11)` while frozen status contains the
  standard `(gated: D-41)` seed. Replace only RecordedRemaining with the
  byte-exact frozen string used by the other package ledgers. Keep
  RemainingSource/Gate `NONE_RECORDED`, Selectable NO, and histograms unchanged.

### DEL-17-05 — DEFECTIVE (11 PASS / 0 QUALIFIED / 4 FAIL)

Scoped rows: every 11 non-ALIGNED rows (`REQ-014..015`, `ACC-006`,
`EXC-001..002`, `DECL-001..004`, `REM-001..002`), mandatory `DECL-005`, and
aligned samples `REQ-006`, `REQ-011`, `REQ-001`.

- REQ-014/015 correctly preserve user-CSV privacy-default and coordinate/loss-
  uncertainty gaps. ACC-006 correctly uses VERIFIED_NOT_VALIDATED because an
  opt-in record shape exists but no user-owned executable/profile evidence
  establishes live target behavior.
- EXC-001/002 and DECL-001..004 correctly stale active Phase A/no-artifact
  wording while preserving commercial, IP, compatibility, solver, and
  professional boundaries. Python and desktop slices remain adjacent evidence,
  not one demonstrated schema-validated runtime route.
- **FAIL — ACC-006 and REM-002 use `ValidationEvidence=UNVERIFIED — ...`.**
  `UNVERIFIED` belongs in SourceReliability; absent live/target validation must
  use `NONE_FOUND — ...` per plan §6 and the run-wide vocabulary. Replace only
  the prefix; dispositions and confidence remain unchanged.
- **FAIL — DECL-005 scopes bootstrap mechanics as a residual.** Set
  RemainingSource and GateOrStageConstraint from `_STATUS.md Remaining` /
  `gated: D-41` to `NONE_RECORDED`; keep byte-exact RecordedRemaining and NO.
- **FAIL — REM-001/002 place paraphrased Review_Findings content in
  RecordedRemaining and invented gates in GateOrStageConstraint even though
  `_STATUS.md` has no such items.** Set RecordedRemaining and gate to
  `NONE_RECORDED`, retain each Review_Findings source and Selectable NO, and
  preserve the finding/TBD substance in DeclaredState/RemainingWork. Update
  notes to call them unhomed rather than recorded status residuals.

### DEL-17-06 — SOUND (11 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all nine non-ALIGNED rows (`REQ-007`, `ACC-005..006`, `ACC-008`,
`DECL-001..004`, `REM-001`) plus aligned `REQ-001` and `ACC-001`.

- Missing conditional assumption/reproducibility passthrough, mixed setup-era
  acceptance wording, incomplete traceability, and JCS conformance are bounded
  PARTIALLY_IMPLEMENTED rows with appropriate ownership/engineering routes.
- Four current document surfaces are correctly STALE for no-artifact/future
  wording. REM-001 meets addendum 11 because the named human CHECKING action
  permits bounded deferral while leaving formal RF-001 disposition open.
- CSV text normalization remains distinct from JSON canonicalization; unit
  presence is not conversion or engineering suitability; diagnostic-only
  comparison is not equivalence or pass/fail authority.

### DEL-17-07 — SOUND (11 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all four non-ALIGNED rows (`DECL-001..002`, `DECL-004`, `REM-002`)
plus self-flagged/aligned `REM-001`, `REM-003`, `ACC-004`, `REQ-011`,
`ACC-003`, `REQ-001`, and `ACC-001`.

- Declaration staleness is limited to active Phase A/future writer/profile/test
  prose. REM-002 correctly preserves an OPEN/TBD finding omitted from status;
  REM-001/003 correctly keep overlapping target-profile/behavior TBDs visible.
- Deterministic invented straight-pipe PCF output is a bounded foundation, not
  family completeness, real downstream import, target compatibility, solver
  validation, or source-profile closure. Desktop unit witnesses do not expand
  Python support breadth.

### DEL-17-08 — DEFECTIVE (21 PASS / 0 QUALIFIED / 2 FAIL)

Scoped rows: all 20 non-ALIGNED rows, mandatory `DECL-005`, and aligned samples
`REQ-001`/`ACC-001`.

- The twelve PARTIALLY_IMPLEMENTED rows accurately isolate coordinate-transform
  provenance, identifier taxonomy, manifest/profile materialization,
  deterministic-policy declaration, family coverage, diagnostics, requirement
  traceability, application-service binding, GLB validation, and visual QA.
- Seven stale rows correctly identify active Phase A/no-writer/schema/fixture/
  GUI declarations without weakening target/viewer, visual-review, solver, IP,
  or professional boundaries. REM-001 meets addendum 11: named human CHECKING
  approval permitted the open wording warning as nonblocking but did not close
  it.
- **FAIL — DECL-005 retains bootstrap RemainingSource/Gate metadata.** Set
  `_STATUS.md Remaining` and `gated: D-41` to `NONE_RECORDED`; keep the exact
  bootstrap text and NO.
- **FAIL — REM-001 places a review-finding paraphrase and later-cleanup gate in
  RecordedRemaining/Gate despite bootstrap-only status.** Set both to
  `NONE_RECORDED`, retain Review_Findings as the evidence source and NO, and
  keep the permitted-deferral substance in the other row fields/notes.

### DEL-17-09 — SOUND (16 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all 12 non-ALIGNED rows plus self-flagged/aligned `REQ-008`,
`ACC-003`, `REQ-002`, and `ACC-001`.

- REQ-001 correctly preserves the JCS-label gap. REQ-007 and four acceptance
  rows accurately identify missing checklist grains rather than overclaiming
  target readiness, provenance completeness, mechanics/rule validation, or
  human signoff.
- ACC-005 and DECL-001..004 correctly distinguish the later bounded
  schema/builder/fixture/test foundation from obsolete Phase A/no-implementation
  wording. REM-001 is a permitted CHECKING deferral, not formal closure.
- Admission metadata, target registries, denied grants, and diagnostics do not
  implement a loader, sandbox, network/process boundary, writer, public
  endpoint, target validation, or compatibility.

## 3. Correction routes and R3 carry-forward

1. DEL-17-01 correction route (closed in §4): the owning pilot staled DECL-004
   for the split-root command-path defect, routed OWNER, and updated notes.
2. DEL-17-04 correction route (closed in §4): the owning pilot restored the
   byte-exact frozen bootstrap on DECL-005.
3. DEL-17-05 correction route (closed in §4): the owning pilot normalized two
   ValidationEvidence prefixes, excluded bootstrap source/gate metadata, and
   cleared two unhomed finding paraphrases/gates from RecordedRemaining while
   retaining their evidence sources.
4. DEL-17-08 correction route (closed in §4): the owning pilot excluded
   bootstrap source/gate metadata and cleared the unhomed RF-002 paraphrase/
   gate from RecordedRemaining.
5. Preserve ownership: DEL-17-01 source admission; DEL-17-02 common contract;
   DEL-17-03 native project JSON; DEL-17-04 MBF writer; DEL-17-05 external-run/
   CSV evidence; DEL-17-06 stress-neutral review package; DEL-17-07 PCF;
   DEL-17-08 review geometry; DEL-17-09 adapter SDK/checklist foundation.
6. Deduplicate target version/profile/source TBDs by source ID and decision,
   while retaining target-local implementation evidence and distinct formal
   finding state. Do not multiply aggregate review rows into product defects.
7. Join JCS/RFC-8785 risk across PKG-08/14/15/17. Stable sorted JSON, required
   checksums, permitted labels, and actual conformance remain distinct.
8. Keep unit disclosure, deterministic conversion witnesses, target-format
   interpretation, and engineering suitability separate. Invented fixtures and
   optional future user-owned executions are never solver/professional
   validation.
9. Preserve JSON glTF versus binary GLB, centerline versus rendered/surface
   geometry, Python versus desktop coverage, and structural versus viewer QA as
   separate evidence grains.
10. DAG-005/006 references are derivative/historical evidence under current
    DAG-007 authority. Artifact flags, TBD/PENDING satisfaction, and open
    findings authorize no DAG/register/dependency mutation.

No lifecycle transition, product repair, dependency/DAG/register mutation,
source admission, decision packet, R4/R5 work, external execution, release,
compatibility, approval, certification, sealing, authentication, compliance
determination, legal clearance, security assurance, professional reliance,
target support, or solver validation action was performed.

## 4. Post-correction independent backcheck and final verdict

Independent frozen-source backcheck confirms all four correction clusters:

- DEL-17-01 DECL-004 is STALE_SETUP_SPECIFICATION/HIGH/OWNER with the exact
  split-root command-path evidence and bounded R5-only refresh; notes now
  recount ALIGNED 29 / STALE 1 and NO 23 / OWNER 7.
- DEL-17-04 DECL-005 now carries the byte-exact frozen D-41 bootstrap with
  bootstrap source/gate metadata excluded.
- DEL-17-05 ACC-006 and REM-002 now use `NONE_FOUND —`; DECL-005 excludes
  bootstrap source/gate metadata; REM-001/002 use
  `RecordedRemaining=NONE_RECORDED` and `GateOrStageConstraint=NONE_RECORDED`
  while retaining their Review_Findings evidence sources and NO selectability.
- DEL-17-08 DECL-005 excludes bootstrap source/gate metadata; REM-001 uses
  NONE_RECORDED recorded/gate cells while retaining its review source,
  ACCEPTED_DIVERGENCE/MEDIUM/OWNER, and NO.

Package revalidation reconfirms nine exact-header, 20-column RFC-4180 CRLF
ledgers; 375 rows; fully controlled enums; type-matched contiguous ClaimIDs;
54 declaration rows; byte-exact bootstrap placement and exclusion; all-NO
selectability; zero convention-6 markers across 28 SECURITY rows; no invalid
ValidationEvidence prefixes; and final aggregate dispositions ALIGNED 312 /
STALE_SETUP_SPECIFICATION 28 / STALE_REVIEW_OR_EVIDENCE 1 /
PARTIALLY_IMPLEMENTED 26 / REMAINING_STATE_MISMATCH 4 /
VERIFIED_NOT_VALIDATED 1 / ACCEPTED_DIVERGENCE 3. Final authority is NO 299 /
OWNER 66 / ENGINEERING 10; other histograms are unchanged. `git diff --check`
passes for all corrected package artifacts. Frozen ignored-aware porcelain
still contains exactly the six addendum-9 allow-listed incident paths and no
seventh path. The verifier made no pilot-artifact edit.

**FINAL VERDICTS: DEL-17-01 SOUND · DEL-17-02 SOUND · DEL-17-03 SOUND ·
DEL-17-04 SOUND · DEL-17-05 SOUND · DEL-17-06 SOUND · DEL-17-07 SOUND ·
DEL-17-08 SOUND · DEL-17-09 SOUND. Final checked-row tally: 161 PASS / 0
QUALIFIED / 0 FAIL. PKG-17 fan-in is CLOSED.** R3 carry-forward observations
remain evidence-only aggregation cautions and authorize no repair or governed-
state change.
