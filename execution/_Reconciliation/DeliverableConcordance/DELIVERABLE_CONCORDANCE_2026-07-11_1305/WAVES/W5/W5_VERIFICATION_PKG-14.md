# W5 Fan-in Verification — PKG-14 (Model States, Analysis Runs, and Comparison)

Verifier: independent highest-available-capability GPT-5 fan-in verifier,
high-effort adversarial pass. This verifier did not pilot any PKG-14 ledger.
Scope: every self-flagged row, every non-`ALIGNED` row, and at least two
`ALIGNED` rows in each of `DEL-14-01..05`, checked against the frozen evidence
worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`, the pinned plan §§6–8,
`R1_CONVENTIONS.md`, and the PKG-00..13 calibration. Dispositions remain agent
judgments, never owner, engineering, security, legal, validation,
professional, or release rulings. This verifier edited no pilot ledger or
notes file.

**Pre-correction verdicts: DEL-14-01 SOUND · DEL-14-02 SOUND · DEL-14-03
SOUND · DEL-14-04 DEFECTIVE · DEL-14-05 SOUND.** Initial checked-row tally:
**77 PASS / 0 QUALIFIED / 2 FAIL** across 79 scoped rows. The two failures were
one bounded evidence-description defect duplicated across a
requirement/acceptance pair. The owning pilot corrected both without a
discovery rerun or histogram change; §4 records the independent backcheck and
final all-SOUND verdict.

## 1. Mechanical, reproducibility, and containment checks

- All five CSVs parse at exactly 20 columns with the adopted header and
  RFC-4180 CRLF. All ClaimIDs are deliverable-prefixed, type-matched,
  three-digit padded, and contiguous within ClaimType. Controlled enums,
  lifecycle values, confidence, authority, and selectability vocabularies
  pass.
- The ledgers contain **141 rows** (26/28/30/24/33). Recounted aggregate
  ClaimType histogram: REQUIREMENT 51 / ACCEPTANCE 36 / EXCLUSION 21 /
  DECLARED_STATE 30 / REMAINING_WORK 3. Recounted disposition histogram:
  ALIGNED 100 / PARTIALLY_IMPLEMENTED 17 / STALE_SETUP_SPECIFICATION 20 /
  REMAINING_STATE_MISMATCH 2 / VERIFIED_NOT_VALIDATED 2. Every per-ledger
  histogram in the notes reproduces.
- Confidence recount: HIGH 113 / MEDIUM 28. Authority recount: NO 96 / OWNER
  39 / ENGINEERING 6. SourceReliability recount: UNVERIFIED 111 /
  NOT_APPLICABLE 30. All 141 rows are mechanically non-selectable; no current
  ungated residual exists in this package.
- Every declaration census is exactly the four-document kit plus `_STATUS.md`
  and `MEMORY.md`; none of the mapped implementation/schema surfaces has a
  deliverable-owned README. Each bootstrap string is byte-exact, appears only
  on `DECL-005`, and is excluded from residual, gate, and selectability
  analysis. DEL-14-01's separate Phase G residual is represented once as
  REM-001 and propagated only to the requirement/acceptance surfaces it
  touches; its `stage-gated: v0.2 R3` suffix correctly yields `NO`.
- Convention-6 exact SECURITY sufficiency-deferral marker count is zero.
  Every SECURITY row uses an explicit-reason `NOT_APPLICABLE` except
  DEL-14-01-REQ-009, whose `NONE_FOUND` records an actual bounded runtime
  control-reach gap rather than an accepted sufficiency deferral. This is the
  restraint required by the PKG-10..13 calibration.
- Independent verifier re-execution at the frozen SHA used
  `PYTHONDONTWRITEBYTECODE=1`, external `PYTHONPYCACHEPREFIX`, and pytest
  `-p no:cacheprovider`. The combined model-state, persistence, analysis-run,
  comparison-contract, model-state-comparison, and analysis-run-comparison
  suite produced **44 passed**; all four JSON schemas parsed with
  `python3 -m json.tool`. No Cargo command and no in-tree `py_compile` ran.
- Frozen ignored-aware porcelain before and after contained exactly the six
  addendum-9 incident allow-list paths and no seventh path: project
  `.pytest_cache/`; the two reporting `Cargo.lock` files; the reporting and
  tests `__pycache__/` paths; and the nonlinear benchmark `target/`. Tracked
  porcelain remained empty. The frozen worktree was neither cleaned nor
  modified.

## 2. Per-ledger scoped verification

### DEL-14-01 — SOUND (14 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all six non-ALIGNED rows (`REQ-009`, `ACC-005`, `DECL-001..004`)
plus self-flagged/aligned `REQ-008`, `REQ-010`, `ACC-004`, `EXC-001..003`,
`DECL-005`, and `REM-001`.

- REQ-009/ACC-005 correctly isolate a runtime external-reference control-reach
  gap without converting a bounded schema/fixture result into legal or
  security assurance. `PARTIALLY_IMPLEMENTED`, MEDIUM, OWNER, and
  actual-gap `NONE_FOUND` are mutually consistent.
- DECL-001..004 each carry a falsified current-state fact: implemented schema,
  persistence, tests, accepted container decisions, or revision-0.8 authority
  overtakes setup/future wording. STALE is surface-local; surviving archive
  mechanics and Phase G breadth are not declared complete.
- The bounded negative professional/content rows correctly use
  explicit-reason `NOT_APPLICABLE`. EXC-002's MEDIUM source-search qualifier
  is appropriately narrower than a repository-wide absence proof.
- DECL-005 and REM-001 preserve the difference between bootstrap mechanics
  and the accepted stage-gated Phase G program. The schema/persistence slice
  does not claim ownership of every project-persistence, desktop, audit, or
  report consumer.

### DEL-14-02 — SOUND (16 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all nine non-ALIGNED rows (`REQ-005..007`, `ACC-004..005`,
`DECL-001..004`), aligned schema samples `REQ-001`/`ACC-001`, all three
SECURITY rows (`REQ-009`, `ACC-007`, `EXC-004`), and `DECL-005..006`.

- REQ-005/ACC-005 accurately distinguish an explicit run-level unit-system
  reference from comprehensive negative coverage of unit-bearing referenced
  values. The mechanics cells explicitly state that numeric validation is not
  claimed.
- REQ-006 correctly records partial diagnostic reach: blocking vocabulary,
  provenance, and core-field validation exist, but the test suite does not
  establish every solve/rule-required input branch.
- REQ-007/ACC-004 correctly identify the `JCS` label versus stable sorted
  compact `json.dumps` implementation gap. Determinism is tested; full
  RFC-8785/JCS equivalence is not demonstrated.
- DECL-001..004 are fact-specifically stale, while DECL-006 correctly leaves
  dated MEMORY authority entries historical. Bootstrap-only status and the
  header-only findings register justify the zero REM row census.
- SECURITY rows are schema/redaction/content checks at bounded artifact grain
  and correctly avoid both blanket convention-6 markers and public-export
  assurance claims.

### DEL-14-03 — SOUND (17 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all eight non-ALIGNED rows (`REQ-007`, `ACC-005`,
`DECL-001..004`, `REM-001..002`) plus aligned/self-flagged `REQ-003`,
`REQ-005`, `REQ-009..010`, `ACC-004`, `ACC-006..007`, and `EXC-004..005`.

- REQ-007/ACC-005 correctly separate implemented caller-declared unit-field
  guards from absent governed field discovery/normalization. Missing metadata
  is directly exercised; incompatible-dimension/differing-unit branches are
  source-inspected but not overclaimed as full governed mechanics evidence.
- REQ-003/ACC-004 reproduce the explicit mapping path, unresolved visibility,
  and mapping-context preservation. They do not relocate mapping-policy
  ownership from DEL-14-05.
- REQ-005/ACC-006 use reproducible invented-fixture evidence for deterministic
  software behavior while explicitly withholding independent suitability
  validation. That is consistent with their acceptance/validation-harness
  grain and does not warrant a VERIFIED_NOT_VALIDATED downgrade.
- DECL-001..004 contain verified current-pointer, artifact-TBD, or obsolete
  dependency-conflict declarations; STALE is confined to those declaration
  surfaces. REM-001/002 exactly preserve two OPEN, TBD-human-disposition review
  findings omitted from `_STATUS.md`, so REMAINING_STATE_MISMATCH with
  MEDIUM/OWNER satisfies addendum 13.
- Negative professional/protected-content rows use restrained explicit-reason
  `NOT_APPLICABLE`; no security, legal, compliance, or professional conclusion
  is inferred.

### DEL-14-04 — DEFECTIVE (12 PASS / 0 QUALIFIED / 2 FAIL)

Scoped rows: all ten non-ALIGNED rows (`REQ-001`, `REQ-003..004`, `REQ-007`,
`ACC-002..003`, `DECL-001..004`) plus aligned `REQ-002`, `REQ-005`,
`REQ-008`, and `EXC-003`.

- REQ-003/ACC-003 correctly use VERIFIED_NOT_VALIDATED: mixed-unit mechanics,
  explicit conversions, and negative diagnostics are reproducibly tested, but
  the corpus is agent-generated and is not an independent engineering
  suitability basis.
- REQ-004 and REQ-007 correctly preserve category-coverage and canonical
  comparison-output schema gaps. REQ-002 and REQ-005 correctly preserve run
  context and caller-governed tolerance profiles without claiming schema or
  threshold authority.
- DECL-001..004 each contain verified setup/future or revision-0.7/DAG-006
  current framing overtaken at the frozen SHA. SECURITY/content rows are
  bounded and marker-free.
- **FAIL — REQ-001 and ACC-002 misdescribe the missing stable-ID path as a
  test-only gap.** Frozen `core/comparison/analysis_run/engine.py` iterates
  caller-supplied mapping records and accepts `automatic_match` or
  `manual_match` as controlled status values; it does not derive mappings by
  equal stable IDs. `tests/test_analysis_run_comparison.py` supplies manual
  mappings, and no automatic stable-ID matching implementation is present.
  The current `ImplementationEvidence` and `RemainingWork` therefore imply an
  implemented-but-unexercised automatic path that the source does not contain.
  The PARTIALLY_IMPLEMENTED/MEDIUM/NO judgment is sound, so this is a bounded
  evidence-description correction, not a discovery rerun.

Owning-pilot correction for both rows: replace the automatic-path wording with
the exact distinction **“deterministic caller-supplied mapping consumption is
implemented and manually exercised; automatic stable-ID matching/derivation is
absent.”** Update `ImplementationEvidence`, `VerificationEvidence`, and
`RemainingWork` accordingly, and make the same correction in the notes' key
judgment and cross-ledger risk. Keep Disposition, Confidence, AuthorityNeeded,
ClaimType, ClaimClass, and all histograms unchanged.

### DEL-14-05 — SOUND (18 PASS / 0 QUALIFIED / 0 FAIL)

Scoped rows: all eight non-ALIGNED rows (`REQ-008`, `ACC-001`, `ACC-007..008`,
`DECL-001..004`) plus aligned/self-flagged `REQ-004`, `REQ-007`, `REQ-009`,
`REQ-011`, `ACC-004`, `ACC-009..011`, and `EXC-004..005`.

- REQ-008/ACC-008 correctly separate a reserved report-section reference
  contract from absent status/limitation fields, rendering, and fixtures.
  ACC-007 correctly distinguishes CSV contract shape from a materialized
  parse/round-trip exporter fixture.
- ACC-001 correctly leaves engine-level equivalent-input repeatability partial
  while REQ-001 remains ALIGNED at schema-support grain. DEL-14-03/04 own the
  engines; DEL-14-05 owns vocabulary and contract shape.
- REQ-004/ACC-004 correctly establish no-default/unit-aware schema guards, not
  governed numeric suitability. Their explicit open-validation statements and
  ENGINEERING routing do not convert agent evidence into an engineering
  ruling.
- Professional, privacy, and protected-content pairs use bounded negative
  evidence and explicit-reason `NOT_APPLICABLE`; no convention-6 marker is
  manufactured. REQ-011/ACC-011 accurately distinguish schema-valid ACTIVE
  dependency mirrors from closure: four satisfaction cells remain TBD.
- DECL-001..004 are stale only for materialized enum/field/schema/test/current-
  authority facts. Governed values, source-safe examples, runtime engines,
  rendering, and final layout remain genuine deferrals.

## 3. Correction routing and R3 carry-forward

1. **DEL-14-04 correction route (closed in §4):** the owning pilot corrected
   REQ-001/ACC-002 and the notes to record absent automatic stable-ID
   derivation rather than merely absent focused testing. No histogram or
   disposition changed.
2. R3 must preserve ownership grain: DEL-14-01 owns the immutable-state
   contract, DEL-14-02 the run-record contract, DEL-14-03/04 the two engines,
   and DEL-14-05 mapping/tolerance/export vocabulary. Persistence, preview,
   reporting, audit, and desktop surfaces are shared consumers, not duplicate
   authoritative owners.
3. Deduplicate the full-RFC-8785/JCS gap across run records, state hashing,
   persistence, and audit consumers; stable sorted JSON behavior is not the
   same claim as full JCS conformance.
4. Aggregate unit findings by behavior grain: explicit reference metadata,
   caller-declared field guards, caller-supplied conversion, governed
   normalization contracts, and engineering validation are separate species.
5. Keep three export states distinct: schema contract exists; engine output
   exists; canonical runtime export/report rendering is absent. The repeated
   rows across PKG-08, PKG-14, and later handoff/export packages must not be
   counted as independent implementations or independent gaps.
6. Deduplicate revision-0.7/DAG-006/setup-TBD staleness as a cross-corpus
   species while retaining each affected declaration surface. Do not merge
   genuine governed-value/rendering deferrals into that stale-document family.
7. Preserve DEL-14-03's two OPEN review findings as status-homing/dependency
   observations. They are not proof that the bounded engine is absent and are
   not authority to change dependencies, status, or documents.

No lifecycle transition, product repair, dependency/DAG/register mutation,
decision packet, R4/R5 work, professional approval, certification, sealing,
authentication, compliance determination, legal clearance, security
assurance, or release-readiness action was performed.

## 4. Post-correction independent backcheck and final verdict

The DEL-14-04 owning pilot corrected REQ-001/ACC-002 and the notes. Independent
backcheck against frozen `core/comparison/analysis_run/engine.py` and
`tests/test_analysis_run_comparison.py` confirms that both rows now state the
exact source-supported distinction: deterministic caller-supplied mapping
consumption is implemented and manually exercised; `automatic_match` is a
recognized status token, but automatic stable-ID mapping derivation is absent
rather than merely untested. The notes' key judgment carries the same
distinction. No pilot-artifact edit was made by the verifier.

Disposition, Confidence, AuthorityNeeded, ClaimType, ClaimClass, and all
histograms are unchanged. Post-correction package validation reconfirms five
exact-header, 20-column RFC-4180 CRLF ledgers; 141 rows; controlled enums;
type-matched contiguous ClaimIDs; declaration/bootstrap/selectability rules;
and the original per-ledger and aggregate histograms. `git diff --check`
passes for the corrected ledger and notes. Frozen ignored-aware porcelain
still contains exactly the six addendum-9 allow-listed incident paths and no
seventh path.

**FINAL VERDICTS: DEL-14-01 SOUND · DEL-14-02 SOUND · DEL-14-03 SOUND ·
DEL-14-04 SOUND · DEL-14-05 SOUND. Final checked-row tally: 79 PASS / 0
QUALIFIED / 0 FAIL. PKG-14 fan-in is CLOSED.** R3 carry-forward items in §3
remain evidence-only aggregation cautions; they do not authorize repair or
governed-state change.
