VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-09 verification (wave W3, sampling STANDARD)

Fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Parent: HELP_HUMAN Agent 0.
Brief: `briefs/R2-VERIFIER_brief.md` (SHA-256 `47fb3c52…5b00`, checked and
matching). Evidence checkout: `00115c71931bcae79909602d653740d3bb72dfa1`
(freeze, read-only). Rules applied: `CONVENTIONS.md` (Parts A to F, Part F
judged like any other rule), `CANONICAL_SITUATIONS.md`,
`AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`, and `WAVES/W2/RESOLUTIONS.csv` for
the existing SR-1 cluster. I read the PKG-09 manager's transcripts and the
worker returns and notebooks for their flags only. I did not use them as
evidence for any claim.

These are agent verification judgments. They are not owner rulings, and they
do not state or imply any release, approval, compliance or certification
(F-PIP-2; claims taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** For all five ledgers, the forward SHA-256 I recomputed matches
  the `_SEAL.txt` line, the worker return and the manager return. The reverse
  hashes I recomputed also match the returns.
- **Single mode.** I ran `validate_ledger_v2.py` against the freeze with
  `--forward --reverse --inventory ROUTING/PKG-09_capabilities.csv
  --notes-gap`. All five ledgers pass with 0 findings:

  | Deliverable | Forward rows | Required keys | Canonical rows |
  |---|---|---|---|
  | DEL-09-01 | 124 | 74 | 7 |
  | DEL-09-02 | 112 | 69 | 7 |
  | DEL-09-03 | 107 | 70 | 7 |
  | DEL-09-04 | 68 | 67 | 7 |
  | DEL-09-05 | 93 | 88 | 7 |

- **Batch mode.** `--batch` over the five forward ledgers returns PASS with 0
  consistency findings, the same as the manager's run. `WAVES/W3/RESOLUTIONS.csv`
  does not exist yet.
- **Keyed canonical rows** (CS-01, 02, 04, 06, 07). The validator checks
  conformance on 100% of these rows. All conform.
- **Reverse files.** Each has 368 capability rows, which matches the routing
  file.

## Sampling (deterministic)

Candidate keys in each class were sorted by the SHA-256 of the full claim
key. In each deliverable I took the lowest fraction, rounded up. STANDARD
rates were used:

| Class | Rate used |
|---|---|
| C100 (INVARIANT tier, UNKNOWN, ACCEPTED_DIVERGENCE, AUTHORITY_CONFLICT, LIFECYCLE_REASSESSMENT_REQUIRED, PROTECTED_CHECK, FROZEN_CONTRACT) | 100% |
| SH (`SharedTextCount > 1`; a minted `.sNN` key takes its parent's count) | 100% |
| F2/4 (ALIGNED with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:`) | 100% |
| F7 (ALIGNED with `PRODUCT_CALLER: NONE`) | 25% (no rows in this package) |
| NA (other non-aligned) | 25% |
| AN (ALIGNED normative) | 20% |
| Q (structural and other quiet rows) | 10% |

- Each row falls in the first class that matches, in the order above.
- **Disclosed departure (same as W2):** AN rows were selected by hash only,
  not weighted toward LOW or MEDIUM confidence.
- No deliverable is ISSUED, so no deliverable has 100% sampling of all rows.
- In total I sampled **148 of 504** forward rows. The sampled keys are listed
  in §2.
- Beyond the sample, I also reviewed every row the workers flagged for
  verifiers (FG-DEL-09-01-06, the CP-10 rows, FG-DEL-09-05-06, and the
  G2 CANONICAL_DEPARTURE rows) and read every row of all five ledgers in
  summary form. Findings outside the sample are reported separately in §4.2.

## 1. Package-level result

- **Firm disagreements on sampled rows: 0.**
- **Weak disagreements on sampled rows: 3.** A further 5 findings are
  outside the sample.
- **Field disagreements on sampled rows: 0.** A further 3 rows outside the
  sample have one finding between them.
- **Package firm false-alignment rate: 0 / 29 sampled `ALIGNED` normative
  rows = 0.0%** (gate ≤ 5%: met).
- No deliverable meets a rerun condition. No deliverable's firm error rate
  exceeds 10%. No firm error falls in a 100%-sampled class that changes the
  tier or the owner routing.

## 2. Per-deliverable tables

Cells read "total / sampled". The "False-align" column is the firm
false-alignment rate among sampled `ALIGNED` normative rows. That count
includes the aligned normative rows sampled through the SH and F2/4 classes.

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-09-01 | 124 | 0 | 16/16 | 3/3 | 0 | 50/13 | 20/4 | 35/4 | 40 | 0 | 2 | 0 (+3 rows out of sample) | 0/5 = 0% |
| DEL-09-02 | 112 | 2/2 | 9/9 | 1/1 | 0 | 36/9 | 31/7 | 33/4 | 32 | 0 | 0 | 0 | 0/8 = 0% |
| DEL-09-03 | 107 | 0 | 11/11 | 2/2 | 0 | 32/8 | 22/5 | 40/4 | 30 | 0 | 0 (+1 out of sample) | 0 | 0/7 = 0% |
| DEL-09-04 | 68 | 1/1 | 7/7 | 0 | 0 | 19/5 | 18/4 | 23/3 | 20 | 0 | 0 (+1 out of sample) | 0 | 0/4 = 0% |
| DEL-09-05 | 93 | 1/1 | 7/7 | 0 | 0 | 39/10 | 24/5 | 22/3 | 26 | 0 | 1 (+2 out of sample) | 0 | 0/5 = 0% |

Sampled keys (claim-key suffixes after `<DEL>:`):

- **DEL-09-01.**
  - SH: CONTEXT #objective-support, #package-reference,
    #architecture-basis-injection (with .s01 and .s02),
    #preparation-notes, #decomposition-reference, #scope-coverage;
    SOW CLM-003.r02, r03, r04, r05, r08, r09; CLM-017; CLM-034.
  - F2/4: CLM-035.r04; CLM-026; STATUS#remaining/R01.
  - NA: SOW; CLM-002; CLM-020; CLM-006.r01, r04, r05, r07, r08;
    CLM-008.r02; CLM-032.r02; CLM-035.r02; VER-001; STATUS#remaining/R04.
  - AN: CLM-030; CLM-015.r01; RQ-007; purpose OUT-001.
  - Q: CLM-001; CLM-003; CLM-008.r01; output-and-evaluation-matrix.
- **DEL-09-02.**
  - C100: CLM-005.r07; CLM-014.r04.
  - SH: CONTEXT #objective-support, #package-reference,
    #architecture-basis-injection (with .s01 and .s02),
    #decomposition-reference, #preparation-notes, #scope-coverage;
    CLM-016.
  - F2/4: RQ-006.
  - NA: SOW; CLM-002; CLM-026; CLM-005.r02; CLM-007; CLM-008.r02; CLM-020;
    CLM-029.r03; CLM-031.
  - AN: CLM-004.r02, r06; RQ-001; CLM-014.r01, r02; CLM-030.r04; AC-001.
  - Q: CLM-005; CLM-025; CONTEXT #context-envelope, #context-budget-qa.
- **DEL-09-03.**
  - SH: CONTEXT #package-reference, #architecture-basis-injection (with
    .s01 and .s02), #decomposition-reference, #objective-support,
    #preparation-notes, #scope-coverage; CLM-003.r03, r05, r08.
  - F2/4: REQ-09-03-007; CLM-004.r01.
  - NA: CLM-005; CLM-017; CLM-019; CLM-020; CLM-022; CLM-027;
    CLM-027.r05; CLM-014.r03.
  - AN: CLM-004.r03; REQ-09-03-008; CLM-027.r01, r04; CLM-028.
  - Q: CLM-001; CLM-007.r05; CLM-016; CLM-023.
- **DEL-09-04.**
  - C100: STATUS#remaining/R02.
  - SH: CLM-021; CLM-028; CONTEXT #decomposition-reference,
    #package-reference, #architecture-basis-injection (with .s01),
    #preparation-notes.
  - NA: CLM-005; CLM-009; CLM-016; CLM-018; VER-001.
  - AN: VAL-REQ-007; VAL-REQ-008; CLM-024; CLM-026.
  - Q: CLM-001; governing-values-and-decisions-axiology;
    CONTEXT#description.
- **DEL-09-05.**
  - C100: STATUS#remaining/R01.
  - SH: CONTEXT #decomposition-reference, #architecture-basis-injection
    (with .s01 and .s02), #package-reference, #preparation-notes,
    #objective-support.
  - NA: CLM-003; CLM-006; CLM-010; CLM-011; CLM-012/RQG-005; CLM-013;
    CLM-015; CLM-022; STATUS; MEMORY.s01.
  - AN: CLM-014/RQG-003, RQG-006, RQG-007, RQG-012; CLM-026.
  - Q: STATUS#history; STATUS#remaining; CONTEXT#context-envelope.

## 3. What was checked against the freeze (selected)

- **Suites and product caller.**
  - The three suite crates exist at `validation/benchmarks/{mechanics,stress,nonlinear}`.
  - `core/runner/headless/Cargo.toml` depends on all three, and
    `benchmark_binding.rs` calls their observation and comparison accessors.
  - The workers' reading that suite rows have a product caller (so no F7
    markers) holds.
- **Mechanics inventory.**
  - `fixture_inventory()` lists 25 fixtures. The BENCHEVID
    `FAMILY_PROVENANCE_INDEX.csv` has 24 data rows and no DEC092 row, as
    CLM-006.r08 says.
  - `contains_forbidden_reliance_claim`, `missing_required_families`,
    `FIXTURE_UNIT_BASIS` and `ExpectedValue.tolerance_policy` exist where
    cited.
- **FG-DEL-09-01-06 (expansion-loop tolerance).**
  - `EXPANSION_LOOP_{REACTION,DISPLACEMENT,UX_T2}_RELATIVE_TOLERANCE` are
    5.0e-7 (`lib.rs` L3114–3122), with measured-rationale comments.
  - The other witness comparisons in the crate use 1.0e-9 (DEC092, CBDFE,
    CBPT).
  - DEC-026 (`SOFTWARE_DECOMP.md` L617) seeds the analytic class at 1.0e-9.
    Fixture-local overrides may only tighten, and any loosening is a
    governance event.
  - The hand-calc note (`expansion_loop_curved_bend_thermal.md` L421–451)
    proposes 1.0e-9 and says adoption into the governed record is a separate
    step. I found no governed record of the 5.0e-7 value. The worker's
    finding is supported.
- **Origin tests (F3).** `git log -S` on the freeze puts sampled setup-era
  strings at `7bee9ae41` and then at the SOW migration (for example
  DEL-09-01's "Anticipated future artifact…" and "Which benchmark cases gate
  release…").
- **DEL-09-02 invariant rows.**
  - `STRESS-TP-PMM-P3-MILLTOL-…` first appears in `3a44fb4da` (2026-07-10),
    after the 2026-06-06 self-check.
  - The DEL-09-04 owner-gates maintainer-review packet
    (`…20260810-DEL0904-OWNER-GATES/instances/A2_MAINTAINER_REVIEW/PACKET.md`
    §1) records zero qualifying review records across all 15 stress case
    pages. This is consistent with UNKNOWN · EVIDENCE_NOT_LOCATED. The worker
    did not cite this record; it would strengthen the rows.
- **DEL-09-04 R01.** The runner binds `export-results`
  (`openpipestress-runner.rs` L325–342), and five `del1005_export_results_*`
  witnesses exist. The stub clause is stale, as the row says.
- **DEL-09-05.**
  - The gate-record schema uses `pass`, `fail` and `TBD` only, with no
    four-outcome vocabulary. This confirms CLM-006.
  - `RELEASE_QUALITY_GATES.md` §8 still calls the `VALIDATION_STRATEGY.md`
    engineering-beta condition the governing floor. `VALIDATION_STRATEGY.md`
    §4 no longer has that condition (`c8748a04a`, 2026-06-07). This confirms
    FG-DEL-09-05-06.

## 4. Disagreements

### 4.1 Sampled rows

**W-1 (weak). DEL-09-01:SOW#CLM-030** (AN)
- **Row:** ALIGNED. The principles hold, including "keep final comparison
  tolerances and release thresholds TBD until approved".
- **Found:** the same ledger holds FG-DEL-09-01-06. The expansion-loop
  comparison uses an unapproved 5.0e-7 relative tolerance, which is looser
  than DEC-026 (`validation/benchmarks/mechanics/src/lib.rs` L3114–3122).
  CLM-015.r03, CLM-023 and CLM-024.r05 are therefore IMPLEMENTED_DIFFERENTLY.
  Principle 3 of CLM-030 covers the same subject.
- **Why weak:** the row is defensible only under the worker's reading that
  "final" tolerances means release-level values (`RELEASE_QUALITY_GATES` §10),
  as distinct from in-suite comparison constants. The fixture's
  `tolerance_policy` field is `None`.
- **Right values if FG-06 stands:** a `.sNN` for the tolerance principle, or
  the whole row, as IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
  PROJECT_BASELINE · BASELINE;VALIDATION · FG-DEL-09-01-06 · OWNER.
- **Same reading applies to:** CLM-004.r06 ("Tolerance posture", ALIGNED,
  outside the sample).
- **For R3:** settle this together with FG-06.

**W-2 (weak). DEL-09-01:SOW#CLM-003.r09** (SH)
- **Row:** STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE. It covers the
  identification field "Lifecycle state during setup: Draft setup evidence
  only; not implementation and not ISSUED".
- **Found:** F3 keeps readiness and lifecycle states as
  `STALE_REVIEW_OR_EVIDENCE` whatever their origin (owner-confirmed,
  Direction 8).
- **Why weak:** whether a phase-scoped "during setup" field counts as a
  lifecycle state is a reading the conventions should settle.
- **Right values under F3:** STALE_REVIEW_OR_EVIDENCE, with the cause, tier
  and layer unchanged.

**W-3 (weak). DEL-09-05:SOW#CLM-003** (NA)
- **Row:** STALE_SETUP_SPECIFICATION · SCOPE_GREW_BY_DIRECTION, for the whole
  identification block.
- **Found:** the block mixes two stale parts:
  - a setup-era write boundary (F3 origin: setup class);
  - a lifecycle target, "SEMANTIC_READY for setup review" (F3 exception:
    review-state class).
- **Right values:** C1 asks for `.sNN` rows when parts take different
  dispositions. The lifecycle-target part would be STALE_REVIEW_OR_EVIDENCE.
  The tier and routing are unchanged.

### 4.2 Outside the sample (reviewed because flagged or noticed)

**F-1 (field, 3 rows).** DEL-09-01 CLM-015.r03, CLM-023 and CLM-024.r05
(FG-DEL-09-01-06).
- **Rows:** `BaselineClass=RULED_CRITERION`.
- **Found:** C4 lists "tests, tolerances, oracles and limits" as
  `PROTECTED_CHECK`. Its AGENT clause uses `PROTECTED_CHECK` when the
  diverging artifact is a protected check that was weakened or contradicted.
  - The diverging artifact is the fixture comparison tolerance, which was
    loosened against the governed DEC-026 value.
  - DEC-026 is the criterion it diverges from, not the artifact.
- **Right values:** `BaselineClass=PROTECTED_CHECK`. The disposition, cause,
  tier (PROJECT_BASELINE), layers (BASELINE;VALIDATION) and OWNER routing are
  unchanged.
- **Why it matters:** PROTECTED_CHECK is a 100%-sampled class, and R3
  clusters by it. Under the sealed value, these rows fall outside both.

**W-4 (weak; shared-situation conflict across the two worker groups).**
Setup-origin dead file or section references.
- **G1 treatment:** DEL-09-03:SOW#CLM-007.r01 (the removed `INIT.md`) is
  STALE_REVIEW_OR_EVIDENCE under CP-02.
- **G2 treatment:** the same situation in DEL-09-04 and DEL-09-05 (for
  example DEL-09-04 CLM-005, VAL-REQ-004 and CLM-007; DEL-09-05
  RQG-002 to 005 and RQG-010) is STALE_SETUP_SPECIFICATION, each row with
  `CANONICAL_DEPARTURE:` citing F3.
- **My reading:** F3's exceptions cover keyed CS rows, revision pins, review
  states and metadata. They do not cover CP pattern rows or file and section
  references. G2's reading is the better-founded one, and the sampled G2
  rows (DEL-09-04 CLM-005 and DEL-09-05 RQG-005) are correct.
- **Right values for DEL-09-03 CLM-007.r01:** STALE_SETUP_SPECIFICATION,
  after confirming a `7bee9ae41` origin.
- **Why batch mode passed:** the G2 rows carry CANONICAL_DEPARTURE, and the
  G1 row is in a different ledger group.
- **Ask:** Agent 0 should record one resolution for the class (F6).

**W-5 (weak). DEL-09-05:SOW#CLM-012/RQG-007 and SOW#CLM-029**
(FG-DEL-09-05-06).
- **Rows:** BASIS_POINTER_STALE · LOCAL_DESIGN · RECORD with
  AuthorityNeeded OWNER.
- **Found:** the cited condition was not relocated; it was removed, and the
  replacement is the open PB-TBD-003 label decision.
- **Inconsistency:** DEL-09-05's own STATUS R01 routes the same PB-TBD-003
  subject as INVARIANT · CLAIMS.
- **Better-fitting values:** cause AUTHORITY_UNCLEAR (the governing sources
  are now silent on the floor), and a tier that matches R01 (INVARIANT ·
  CLAIMS), or at least PROJECT_BASELINE. LOCAL_DESIGN with OWNER is
  internally tense under the Part D definition of AuthorityNeeded.
- **Routing:** the owner routing is already OWNER, so it does not change.

**W-6 (weak; intra-package inconsistency).** CONTEXT SURFACE rows.
- **G1:** ALIGNED (DEL-09-01 to 09-03).
- **G2:** STALE_REVIEW_OR_EVIDENCE · CP-02 (DEL-09-04 and 09-05), because of
  the revision 0.7 pins.
- **Found:** those pins are already carried by the CS-01 and CS-04 rows. C1
  records a defect on the SURFACE row only when it is common to every item,
  which the pins are not.
- **Preferred values:** ALIGNED. Neither reading changes the tier or routing.

**Note (not counted).** DEL-09-04:SOW (SURFACE).
- The front-matter `decomposition_basis@eaad463` pin is mentioned only in
  Notes.
- G1 gave the same pin its own `SOW.s01` row (DEL-09-02 and DEL-09-03), or
  made it the SURFACE row (DEL-09-01). DEL-09-05's SOW row does the same as
  DEL-09-04.
- The CP-03 rows already carry the revision 0.8 pin, so no finding is lost.
  The difference is a clustering point for R3.

### 4.3 Worker flags checked and agreed

- **FG-DEL-09-01-06 disposition** (IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
  PROJECT_BASELINE · OWNER): agreed; see §3. The only disagreement is the
  baseline-class field (F-1).
- **DEL-09-02 CLM-005.r07 and CLM-014.r04** (UNKNOWN · EVIDENCE_NOT_LOCATED ·
  INVARIANT · IP_DATA;RECORD): agreed.
- **DEL-09-04 R02** (DOCUMENTED_UNIMPLEMENTED · INVARIANT · VALIDATION):
  agreed. The selection of public comparison values bears on
  engineering-validation acceptance (F8).
- **DEL-09-05 R01** (INVARIANT · CLAIMS): agreed.
- **CP-10 rows** DEL-09-01 CLM-008.r03 and CLM-035.r03: agreed.
- **Unit-catalog record drift against DEC-018** (DEL-09-01 FG-03 and FG-04;
  DEL-09-02 FG-01): agreed with MEDIUM confidence.
- **DEL-09-03 unit rows ALIGNED** because the claims say "checkable", not
  "checked": agreed. The difference follows the text.

## 5. Batch consistency and shared situations

- `--batch` over the five ledgers: PASS, 0 findings. No
  `WAVES/W3/RESOLUTIONS.csv` exists.
- **SR-1 (existing contested cluster).** `CONTEXT#architecture-basis-injection.s01`
  has these causes:
  - SCOPE_REDIRECTED_BY_RULING in DEL-09-01, 09-02 and 09-03 (G1);
  - RECORD_DRIFT in DEL-09-04 and 09-05 (G2).

  This is the corpus-wide SR-1 split already recorded in
  `WAVES/W2/RESOLUTIONS.csv`. It is not a new conflict; add these five keys
  to SR-1.
  - All five rows agree on STALE_REVIEW_OR_EVIDENCE · LOCAL_DESIGN · RECORD.
  - Batch mode cannot see these rows, because minted `.s01` keys carry no
    `BodySHA256`.
- **The G1/G2 conflict in W-4** is the only other shared-situation conflict
  found.

## 6. Reverse pass

Answer counts (368 rows each):

| Deliverable | CLAIMED_BY | PARTIAL | UNKEYED | COVERS | CONSTRAINS | NOT_MINE |
|---|---|---|---|---|---|---|
| DEL-09-01 | 2 | 1 | 0 | 1 | 0 | 364 |
| DEL-09-02 | 2 | 1 | 2 | 1 | 0 | 362 |
| DEL-09-03 | 2 | 2 | 0 | 1 | 0 | 363 |
| DEL-09-04 | 6 | 1 | 0 | 12 | 0 | 349 |
| DEL-09-05 | 2 | 0 | 1 | 3 | 0 | 362 |

**Checks on every CLAIMED_BY, PARTIAL and UNKEYED row (21 rows).** All
resolve through `SAMPLE_MANIFEST.csv` to AREA rows (CHECKS, DOCS and PHYS
areas). Each names the right suite, hand-calc folder, manual artifact or
checklist. I agree with all of them.

**Capabilities claimed by more than one PKG-09 deliverable:**
- RC-09-0109 (`CAP-CHECKS-031`, the `physics_audit_regression` crate) is
  PARTIAL in DEL-09-01, 09-02 and 09-03. Each reason cites the R10 run record
  that splits the crate across the three. The split is consistent, so there
  is no conflict. R3 should keep it as one shared capability.
- RC-09-0195 (the DEC-046 convergence policy records) is PARTIAL in DEL-09-03
  and COVERS in DEL-09-04. There is no double ownership.

**UNKEYED:**
- RC-09-0073 and RC-09-0117 (witness pilot and witness tooling) in
  DEL-09-02.
- RC-09-0065 (coverage-telemetry tool) in DEL-09-05.

Agreed. They are routed to R3 as the workers flagged.

**Suspected missed claims (weak).**
- **RC-09-0211** (`tools/release/check_release_readiness.py`) is NOT_MINE in
  all five deliverables. DEL-09-05's reason gives DEL-10-04 as the owner.
  - DEL-09-05's own fan-in records say its runs restored the tool's path
    (`TP_VERIFY_015_RELEASE_READINESS_PATH_CLOSEOUT.md` §2) and updated it
    (`TP_VERIFY_017…` "DEL-10-04 release-readiness command surface").
  - Ownership by DEL-10-04 is defensible, but COVERS for DEL-09-05 fits
    better than NOT_MINE.
- **RC-09-0212** (`VALIDATION_STRATEGY.md`) is NOT_MINE in DEL-09-05.
  DEL-09-04's own PARTIAL answer says §4 (release gate) serves DEL-09-05.
  COVERS would fit.

**NOT_MINE sample (10%, lowest SHA-256).**
- 37, 37, 37, 35 and 37 rows respectively; 38 distinct capabilities in all.
- None is a PKG-09 capability. Examples: solver, loads, GUI panels,
  fixtures, schemas and reporting.
- No disagreement.

**F5 (NOT_MINE rows whose EntryPoints hit a forward-cited path).**
- There are 4, 7, 5, 13 and 9 such rows. I checked 20% by hash (9 rows) and
  read all 38.
- G1's reasons are specific to each capability, and all pass.
- Most of G2's reasons for governing documents use one template ("governing
  project document cited only as a source basis…"). Each has a specific
  parenthetical naming the rows that cite the document.
- I accept these as meeting F5 at the margin (weak note, not counted). The
  specific clause is what satisfies F5, so R3 should not treat the template
  as sufficient elsewhere.

**Sampled against area rows (routing distribution).** There are 23 SAMPLE
routings per deliverable.

| Routing | NOT_MINE | Other answers |
|---|---|---|
| SAMPLE (115 answers) | 115 (100%) | 0 |
| AREA (1,725 answers) | 1,685 (97.7%) | 40 |

- None of the 23 SAMPLE capabilities names a path that PKG-09 deliverables
  declare. They are desktop shell, viewport, reporting and library-import
  items. RC-09-0344, the UI-foundation benchmark, belongs to the D-68
  performance work.
- The all-NOT_MINE result is therefore expected, not evidence of anchoring.
  About half are recognisable from their paths, as the brief notes.

**Anchoring.**
- Positive answers cite sealed forward keys, which the protocol allows.
- The workers report that the reverse pass changed no sealed view, and the
  reasons agree with that.
- DEL-09-01's PARTIAL on RC-09-0109 rests on a run record, not on the sealed
  ledger, which mentions the crate only in MEMORY. No anchoring defect was
  found.

## 7. For the owner, stated plainly

1. **One benchmark tolerance was loosened without a governed record.** The
   mechanics expansion-loop benchmark
   (`MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL`) passes at a fixture-local
   relative tolerance of 5.0e-7. DEC-026 sets the analytic class at 1.0e-9,
   allows fixture overrides only to tighten, and makes any loosening a
   governance event. I found no governed record of this loosening.
   - The rows (DEL-09-01 FG-06) correctly route it to you.
   - I would class it as a weakened protected check (F-1). This does not
     change who decides.
   - Its hand-calc note offers the alternative: use true axial rigidity at
     1.0e-9.
2. **Two stress fixtures added on 2026-07-10 have no located protected-content
   review.** The 2026-08-11 owner-gates packet confirms that no maintainer
   review of any case page exists. The rows are INVARIANT · IP_DATA and
   AuthorityNeeded REVIEW.
3. **The release checklist's release-label floor is a dangling reference.**
   `RELEASE_QUALITY_GATES.md` §8 calls a `VALIDATION_STRATEGY.md` condition
   the governing floor, but that condition was removed on 2026-06-07. The
   replacement is the open PB-TBD-003 decision.
4. **A classification point needs one ruling from Agent 0 or you.** Worker G1
   and worker G2 classify setup-era dead file and section references
   differently: CP-02 STALE_REVIEW_OR_EVIDENCE versus F3
   STALE_SETUP_SPECIFICATION (W-4). It changes no tier or routing.
5. The SR-1 cause split continues in this package, as expected.

Nothing here is a release, approval, compliance or certification statement.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
