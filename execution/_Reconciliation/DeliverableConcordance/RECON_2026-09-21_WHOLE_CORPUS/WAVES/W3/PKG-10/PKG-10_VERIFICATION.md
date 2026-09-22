VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-10 verification (wave W3, sampling STANDARD)

This is the fresh, evidence-only package verifier's report for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. The parent is HELP_HUMAN Agent 0.

- **Brief.** `briefs/R2-VERIFIER_brief.md`. I checked its SHA-256
  (`47fb3c52…5b00`) and it matches.
- **Evidence checkout.** `00115c71931bcae79909602d653740d3bb72dfa1`, the
  read-only freeze. I confirmed HEAD.
- **Rules applied.** `CONVENTIONS.md` Parts A to F (F1 to F8 judged like any
  other rule), `CANONICAL_SITUATIONS.md` and
  `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- **Manager and worker records.** I read the PKG-10 manager's transcripts,
  the worker returns and the worker notes only for the flags they raise. None
  of them is used as evidence about a claim.

These are agent verification judgments. They are not owner rulings, and they
do not state or imply any release, approval, compliance or certification
(F-PIP-2; claims taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** I recomputed the SHA-256 of all five forward ledgers. Each
  matches its `_SEAL.txt` line and `MANAGER_RETURN.md`. The five reverse
  hashes and the five notes hashes also match the worker returns.
- **Single mode.** I ran `validate_ledger_v2.py` against the freeze with
  `--forward --reverse --inventory ROUTING/PKG-10_capabilities.csv
  --notes-gap`. All five ledgers pass with 0 findings:

  | Deliverable | Forward rows | Required keys | Canonical rows | Result |
  |---|---|---|---|---|
  | DEL-10-01 | 94 | 85 | 7 | PASS, 0 findings |
  | DEL-10-02 | 72 | 70 | 7 | PASS, 0 findings |
  | DEL-10-03 | 88 | 79 | 7 | PASS, 0 findings |
  | DEL-10-04 | 128 | 71 | 7 | PASS, 0 findings |
  | DEL-10-05 | 99 | 66 | 7 | PASS, 0 findings |

- **Batch mode.** `--batch` over the five forward ledgers returned `PASS
  batch of 5 ledgers: 0 consistency findings`, the same as the manager's
  run. `WAVES/W3/RESOLUTIONS.csv` does not exist, so `--resolutions` was not
  used.
- **Keyed canonical rows (CS-01, 02, 04, 06, 07).** The validator checks
  these mechanically on 100% of rows. All 35 conform.
- **Package facts.** No PKG-10 deliverable is ISSUED. No row carries tier
  `INVARIANT`, baseline class `PROTECTED_CHECK` or `FROZEN_CONTRACT`, or a
  disposition of `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`, `UNKNOWN` or
  `LIFECYCLE_REASSESSMENT_REQUIRED`. The C100 class is therefore empty.

## Sampling (deterministic)

- **Method.** Within each class I sorted the candidate keys by the SHA-256
  of the full claim key and took the lowest fraction, rounded up. STANDARD
  sampling was applied, so no rate was doubled.
- **Class order.** Each row falls into the first class that matches, in
  this order:
  1. C100 (100%);
  2. SH, rows whose unit has `SharedTextCount > 1` (100%). A minted `.sNN`
     key takes its parent's count;
  3. F2/4, `ALIGNED` rows with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:`
     (100%);
  4. F7, `ALIGNED` rows with `PRODUCT_CALLER: NONE` (25%);
  5. NA, other non-aligned rows (25%);
  6. AN, `ALIGNED` normative rows (`REQUIREMENT`, `ACCEPTANCE`,
     `EXCLUSION`) (20%);
  7. Q, structural rows, inherited canonical rows and other quiet rows
     (10%).
- **Disclosed departure.** AN rows were selected by hash alone, not weighted
  toward LOW or MEDIUM confidence.

**Rows beyond the sample.** Every row the workers or the manager flagged was
also judged. These are:

- DEL-10-01 REQ-02 and STATUS R02;
- DEL-10-02 REQ-10-02-02, REQ-10-02-07 and STATUS R03;
- DEL-10-03 CLM-035.r01, r04 and r05, REQ-09 and the purpose OUT-001;
- DEL-10-04 CLM-013.s02, STATUS R02, REQ-10-04-01, the CONTEXT SURFACE row
  and the context envelope;
- DEL-10-05 R-04 (the test-reduction note).

I also compared every same-titled block across the five ledgers, for
cross-worker consistency. The findings from that comparison are in §4.2.

**In total, 135 of 481 forward rows were sampled.** The keys are listed in
§2.

## 1. Package-level result

- **Firm disagreements on sampled rows: 1.** It is in DEL-10-05, on a Q-class
  `DECLARED_STATE` row. It is not in a 100%-sampled class, and it changes
  neither the tier nor the owner routing.
- **Weak disagreements: 3 forward** and **2 reverse**.
- **Field disagreements:** one systemic pattern across the G2 ledgers
  (DEL-10-04 and DEL-10-05). §4.3 describes it.
- **Package firm false-alignment rate: 0 / 21 sampled `ALIGNED` normative
  rows = 0.0%.** The scale-out gate is 5% or less, so it is met.
- **Rerun test.** No deliverable meets a rerun condition. The highest firm
  error rate is DEL-10-05's, at 1 in 23 (4.3%), which is under the 10%
  threshold. There is no firm error in a 100%-sampled class.

## 2. Per-deliverable tables

Cells read "total / sampled".

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-10-01 | 94 | 0 | 10/10 | 2/2 | 16/4 | 37/10 | 4/1 | 25/3 | 30 | 0 | 1 | 0 | 0/6 = 0% |
| DEL-10-02 | 72 | 0 | 8/8 | 2/2 | 7/2 | 26/7 | 7/2 | 22/3 | 24 | 0 | 0 | 0 | 0/3 = 0% |
| DEL-10-03 | 88 | 0 | 9/9 | 1/1 | 0 | 34/9 | 17/4 | 27/3 | 26 | 0 | 0 | 0 | 0/5 = 0% |
| DEL-10-04 | 128 | 0 | 11/11 | 1/1 | 0 | 40/10 | 19/4 | 57/6 | 32 | 0 | 0 | 1 systemic (23 sampled rows) | 0/4 = 0% |
| DEL-10-05 | 99 | 0 | 7/7 | 0 | 0 | 31/8 | 15/3 | 46/5 | 23 | 1 | 2 | 1 systemic (14 sampled rows) | 0/3 = 0% |
| **Package** | **481** | 0 | 45/45 | 6/6 | 23/6 | 168/44 | 62/14 | 177/20 | **135** | **1** | **3** | systemic | **0/21 = 0.0%** |

Sampled keys (the part after `<DEL>:`):

- **DEL-10-01:**
  - SH: CONTEXT#objective-support, #scope-coverage, #decomposition-reference,
    #architecture-basis-injection (and .s01, .s02), #package-reference and
    #preparation-notes; SOW#CLM-024 and CLM-025.
  - F2/4: REQ-15 and STATUS R01.
  - F7: REQ-05, CLM-006, REQ-01 and REQ-11.
  - NA: SOW, CLM-001, 011, 015, 004, REQ-09, CLM-028, CLM-034.r02, VER-001
    and CLM-016.
  - AN: CLM-031.
  - Q: CONTEXT#context-budget-qa, the axiology wrapper and CLM-019.
- **DEL-10-02:**
  - SH: the eight CONTEXT canonical blocks and sub-claims.
  - F2/4: STATUS R01 and R02.
  - F7: CLM-024 and CLM-004.
  - NA: REQ-10-02-08, VER-001, CLM-020, 013, STATUS, CLM-006 and 023.
  - AN: REQ-10-02-09 and CLM-021.
  - Q: CLM-029, 001 and 015.
- **DEL-10-03:**
  - SH: the CONTEXT canonical blocks and sub-claims; CLM-025 and CLM-026.
  - F2/4: CLM-005.
  - NA: CONTEXT, VER-001, CLM-029, 032, 011, 016, 001, STATUS R01 and
    CLM-020.
  - AN: CLM-031, REQ-06, CLM-021 and AC-001.
  - Q: CLM-013, 035 and 010.
- **DEL-10-04:**
  - SH: CONTEXT#preparation-notes, #decomposition-reference,
    #package-reference and #architecture-basis-injection (with .s01 and
    .s02); CLM-003.r02, r03, r04, r07 and r08.
  - F2/4: STATUS R01.
  - NA: CLM-018, 031.s03, 028.s02, 025, 015.r01, 010, SOW, 005.r02, 019 and
    AC-001.
  - AN: REQ-10-04-06, CONTEXT#description, CLM-015.r08 and REQ-10-04-07.
  - Q: CLM-028.s01, 013, 005.r05, CONTEXT#register-references, CLM-032 and
    004.
- **DEL-10-05:**
  - SH: CONTEXT#preparation-notes, #architecture-basis-injection (with .s01
    and .s02), #decomposition-reference and #package-reference;
    STATUS#remaining.
  - NA: CLM-027.r04, 019, 002.r05, 023, 011.r01, VER-001, 029 and 016.
  - AN: purpose OUT-001, CLM-005.s02 and R-04.
  - Q: CONTEXT, CLM-004.r04, 002.r04, 022 and 024.

## 3. Package firm false-alignment rate

**0 / 21 = 0.0%** (the gate is 5% or less, so it is met). These are the 21
sampled `ALIGNED` normative rows:

- **DEL-10-01 (6):** CLM-006, REQ-01, REQ-05, REQ-11, REQ-15 and CLM-031.
- **DEL-10-02 (3):** REQ-10-02-09, CLM-021 and CLM-024.
- **DEL-10-03 (5):** CLM-005, REQ-06, AC-001, CLM-021 and CLM-031.
- **DEL-10-04 (4):** REQ-10-04-06, REQ-10-04-07, CLM-015.r08 and
  CONTEXT#description.
- **DEL-10-05 (3):** OUT-001, CLM-005.s02 and R-04.

I checked the substance of each against the freeze. The checks are listed
below.

- **DEL-10-01.** I checked the `api/api_boundary_contract.yaml` operation
  registry:
  - 9 commands, 8 queries and 7 jobs;
  - the `required_guards` on import and export;
  - `telemetry_allowed`;
  - `no_bypass.must_not_claim_code_compliance`.

  I also confirmed that only tests read the contract. `git grep` finds no
  consumer in `core/`, `apps/` or `tools/`, so `PRODUCT_CALLER: NONE` is
  accurate.
- **DEL-10-02.** I checked the schema's TBD enforcement, its no-bypass
  matrix and the test list in `tests/test_adapter_framework_contract.py`.
- **DEL-10-03.** I checked the `ContractStatus` and `GuidanceAssessment`
  consts in the handoff schema, and that the desktop
  `LocalFeaHandoffPanel` imports it.
- **DEL-10-04.** I checked `tools/release/` (the export allowlist, the
  release-candidate scan and the packaging record).
- **DEL-10-05.** I checked that the runner links `product_physics`, and
  that the Tauri crate uses it too.

## 4. Disagreements

### 4.1 Firm

**F-1. `DEL-10-05:SOW#CLM-004.r04`.** Class Q, sampled.

- **The row says:** "Condition: external import/export format list TBD;
  schema-first JSON envelopes are the baseline". It is `ALIGNED`, MEDIUM,
  and cites DEC-012. The worker note says "no ruling fixes a list".
- **What I found:**
  - DEC-012 keeps the format list TBD "unless a sealed brief or later human
    ruling resolves them". I read DEC-012 in
    `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md`
    at L603.
  - The same decomposition's OI-004 (L572) states that the specific
    supported import/export formats "are now decomposed through PKG-17".
    It names them: native JSON, CAEPIPE MBF, stress-neutral CSV/JSON,
    conservative PCF, GLB/glTF review geometry and adapter SDK work. That
    decomposition came through the accepted SCA-004 amendment. Only the
    exact target field coverage stays TBD.
  - The text was first present at `7bee9ae41` (checked with `git log -S`).
  - The other PKG-10 worker disposed the identical situation as
    non-aligned in DEL-10-01 `CLM-034.r05` ("Exact import/export format
    list and priorities: TBD"), taking `STALE_SETUP_SPECIFICATION ·
    SCOPE_REDIRECTED_BY_RULING`. The batch validator cannot see this,
    because the bodies differ and no CP ID is shared.
- **Right values:** `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING ·
  LOCAL_DESIGN · NONE · RECORD · AuthorityNeeded NO`, citing SCA-004 and
  OI-004. The runner's own schema-first JSON I/O element holds, and belongs
  in Notes.
- **Effect:** none on tier routing, owner routing or the false-alignment
  rate, because the row is `DECLARED_STATE`.

### 4.2 Weak

**W-1. `DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-05`.** Class F7, sampled. The
same reading applies to `CLM-006` (Conditions, also F7 and sampled).

- **The rows say:** import and export boundaries validate units, provenance
  and protected-content risk before data enters core workflows. Both rows
  are `ALIGNED` with `PRODUCT_CALLER: NONE`, on the strength of the
  contract's `required_guards`.
- **What I found:**
  - The contract declares the guards, and only tests read it.
  - CLM-007 says this deliverable "records a contract boundary, not an
    implementation", so reading the rows as contract claims is defensible
    under F7.
  - The same worker disposed the near-identical runtime wording in
    DEL-10-02 `REQ-10-02-02` ("Every import path validates units") as
    `PARTIALLY_IMPLEMENTED` (CP-11).
  - The reverse inventory shows that live product import paths exist
    outside both contracts: the Tauri library-import validation, RC-10-0279
    and RC-10-0203.
- **Needs a convention:** whether "shall validate" in an API_CONTRACT-type
  deliverable is a claim about the contract or about runtime behaviour. I
  would not re-dispose the rows.

**W-2. `DEL-10-05:CONTEXT`.** Class Q, sampled. The same applies to
`DEL-10-04:CONTEXT`, which is out of the sample.

- **The row says:** the SURFACE row is `ALIGNED`, identity only.
- **The other worker:** G1 took the same file in DEL-10-01, 02 and 03 as
  `STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE` (CP-02), because the
  file names revision 0.7 as `current_basis`.
- **Assessment:** both readings are defensible. In both, the CS-01 keyed
  row already carries the pin. The conventions do not say whether a
  keyed-row defect also belongs on the SURFACE row. R3 should read the two
  groups as one situation.

**W-3. `DEL-10-05:SOW#CLM-002.r05`.** Class NA, sampled.

- **The row says:** the setup lifecycle target `SEMANTIC_READY`, sourced to
  the missing `skills/semantic-matrix-build/SKILL.md`. It takes
  `STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE` (CP-02).
- **Similar statements elsewhere in the package** take different causes:
  - DEL-10-04 `CLM-003.r08` ("current setup state SEMANTIC_READY") is
    `RECORD_DRIFT`;
  - DEL-10-01 `CLM-004` and DEL-10-03 `CLM-004` (identification tables
    carrying the same lifecycle target) are `STALE_SETUP_SPECIFICATION ·
    DOC_BEHIND_CODE`, with the artifact-form gap chosen as the row's cause.
- **Assessment:** each choice is defensible under the one-cause rule. F3
  (Direction 8) makes readiness states review states, but it does not fix
  the cause. The disposition for r05 is right; only the cause is contested.

### 4.3 Field

**FD-1 (systemic, G2 ledgers).**

- **The pattern:** 76 rows in DEL-10-04 (23 of them sampled) and 42 rows in
  DEL-10-05 (14 sampled) carry `VerificationEvidence=NOT_APPLICABLE` with
  `VerificationClass` set to `STATIC_CHECK` or `DOCUMENT_REVIEW`. Examples:
  - DEL-10-04 `CLM-003.r02`, `CLM-010` and `STATUS#remaining/R02`;
  - DEL-10-05 `CLM-002.r04` and `CONTEXT`.
- **Why it is a field error:** C8 names the kind of verification evidence;
  a class with no evidence token is inconsistent. The right value is
  `NONE`, or else the checked register or file should be cited as
  evidence.
- **Related:** 76 and 64 rows respectively leave `DecisionBasis` empty. The
  G1 ledgers fill it on every row.
- **Effect:** no disposition changes.

### 4.4 Worker-raised items I judged (no disagreement)

- **DEL-10-04 `CLM-013.s02` (possible defect).** Confirmed at the freeze.
  - `BuildReadinessPanel.tsx` hard-codes `bundle_active: false`
    (L116, L168) and `installer_format`/`signing_status: "TBD"`. It emits
    `BUILD-READINESS-BUNDLE-INACTIVE` and lists the matrix, signing and
    publishing as TBD.
  - `apps/desktop/src-tauri/tauri.conf.json` has `bundle.active: true`,
    `targets: ["app"]` and two icons.
  - The panel is mounted in `App.tsx` (L649, L1435).
  - `IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · REVIEW`
    is right.
- **CP-10 rows.** These are DEL-10-01 `CLM-034.r02` and DEL-10-03
  `CLM-035.r01`, `r04` and `r05` and `REQ-09`.
  - Each is a setup-era Human-Ruling-Queue TBD that the code settled under a
    human-authorised dispatch brief, which is context under A3.
  - I found no ruling on file placement, field layout or the label
    vocabulary.
  - The two workers treat these consistently. The CP-10 fields are right.
- **DEL-10-02 `adapter_framework.py` TBD enforcement.** The code is as
  reported: `REQUIRED_TBD_DECISIONS` includes `ci_provider` and
  `physical_project_container`, and L760–770 rejects non-TBD values.
  - The finding message reads "must remain TBD in DEL-10-02 … Do not
    select … CI … in this deliverable". That reads as a deliverable-local
    guard against selecting these items here, more than as a claim that
    they are unruled.
  - No key covers it. I agree it is not a row change. It is an R3
    observation: the packet reports ruled items (DEC-025, DEC-028) as TBD.
- **DEL-10-05 R-04 (commit `b43cc00c4`, tests cut from 8 to 2).**
  Confirmed.
  - The commit rewrote `result_envelope_binding.rs`. It replaced the old
    builder tests, such as `structural_failure_appends_blocking_runner_diagnostic`
    and `producer_rejects_missing_envelope_checksum_structurally`, with two
    tests on the new qualified-evidence builder.
  - `lib.rs` still tests the result-envelope checksum
    (`result_requires_schema_vocabulary_and_result_envelope_checksum`).
  - No SOW row or ledger evidence cites the removed tests, so
    `VERIFICATION_REMOVED` (whose condition is that a cited test was
    deleted) does not apply. R-04 `ALIGNED` MEDIUM stands. It is an R3
    note only.
- **DEL-10-01 `REQ-02`.** PRD v0.4 no longer carries a §19.3 family list.
  The contract has no model-creation or load-case family and no rule-pack
  evaluation operation. `PARTIALLY_IMPLEMENTED · OWNER` is right.
- **DEL-10-02 `REQ-10-02-07`.** It cites DEC-074 O7/E5, which is excluded
  and was not read. The flag is correctly raised under R0 §7.

## 5. Batch consistency and shared situations

- **Batch mode.** PASS, 0 findings. No `RESOLUTIONS.csv` exists for W3.
- **Shared bodies within PKG-10.** Both pairs have identical profiles:
  - DEL-10-01 `CLM-024` and DEL-10-03 `CLM-025` (Records, CP-01);
  - DEL-10-01 `CLM-025` and DEL-10-03 `CLM-026` (Completion Condition,
    `DOC_BEHIND_CODE`).
- **Consistency across the corpus.** All 45 SH rows conform on inspection.
  Among them are the Architecture Basis Injection `.s01` and `.s02` rows:
  `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT` for the PKG-00 `SEMANTIC_READY`
  statement, and `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING`
  for the Still-TBD items. Both workers match.
- **Conflicts the batch validator cannot see.** They surfaced when I
  compared same-titled blocks:
  - the import/export format-list TBD (F-1);
  - the CONTEXT SURFACE treatment (W-2);
  - the cause of the `SEMANTIC_READY` lifecycle-target statements (W-3).

## 6. Reverse pass

- **Coverage.** Every non-`NOT_MINE` answer was checked (100%, 57 in all):

  | Deliverable | CLAIMED_BY | PARTIAL | UNKEYED | COVERS | NOT_MINE | Total |
  |---|---|---|---|---|---|---|
  | DEL-10-01 | 2 | 0 | 0 | 1 | 384 | 387 |
  | DEL-10-02 | 4 | 1 | 0 | 0 | 382 | 387 |
  | DEL-10-03 | 2 | 1 | 0 | 2 | 382 | 387 |
  | DEL-10-04 | 14 | 2 | 2 | 6 | 363 | 387 |
  | DEL-10-05 | 12 | 1 | 1 | 0 | 373 | 387 |

  I agree with every one of these answers. Key facts I checked:
  - the contract's and schema's `deliverable_id`;
  - the attribution of `package.json` and SURF-011 under DEC-074 O3 and
    DEC-076;
  - that the CI workflows are the ones DEC-093 accepts;
  - that `result_envelope_binding.rs`, `benchmark_binding.rs` and
    `redaction_binding.rs` sit in the DEL-10-05 crate.
- **NOT_MINE sample (10%).** I checked 39, 39, 39, 37 and 38 rows per
  deliverable (hash-lowest). All answers are correct, with one weak
  finding:
  - **W-R2:** some reasons from G1's category template name the wrong
    owner area, although the answer is right:
    - RC-10-0381 (UI performance measurement) is labelled "DEL-10-04
      area";
    - RC-10-0356 (mechanics benchmark fixtures) is labelled "headless
      runner surface (DEL-10-05)";
    - in DEL-10-02, RC-10-0005 (coverage telemetry) is labelled "PKG-12
      area".
- **F5 sample (at least 20% of `NOT_MINE` rows whose EntryPoints hit a
  forward-cited path).** There are 4, 3, 2, 14 and 34 such rows. I sampled
  1, 1, 1, 3 and 7. All reasons address the path overlap specifically, for
  example "cites App.tsx only as the mount point of its
  BuildReadinessPanel" and "cites product_physics lib.rs only as the shared
  preview service".
  - **W-R1:** G2's reasons are path templates that echo the capability's
    own text; DEL-10-05 uses one for 26 `product_physics` capabilities. They
    are accurate. Whether an echoed path template meets F5's "must address
    that capability" is for the conventions to settle.
- **Capabilities claimed by more than one PKG-10 deliverable:** none. No
  capability has more than one `CLAIMED_BY`, `PARTIAL` or `UNKEYED` answer.
  - RC-10-0190 (the API contract) is claimed only by DEL-10-01. DEL-10-02
    and 03 answer `NOT_MINE` with specific reasons.
- **Suspected missed claims:** none firm.
  - The library-import validation path (RC-10-0279, RC-10-0203) is a live
    product import path that validates imports. DEL-10-02 answers
    `NOT_MINE`, which is right for ownership. R3 should still weigh it
    against DEL-10-02 `REQ-10-02-02` ("every import path validates units")
    and DEL-10-01 `REQ-05` (see W-1).
  - Three GUI surfaces are correctly marked `PARTIAL` or `UNKEYED` for R3:
    - the DEL-10-05 `HeadlessRunnerPanel` (possibly
      `IMPLEMENTED_UNDOCUMENTED`);
    - the DEL-10-02 `AdapterFrameworkPanel`;
    - the DEL-10-03 `LocalFeaHandoffPanel`.
- **Anchored answers.** There were 22 routing-sample rows, resolved through
  `SAMPLE_MANIFEST.csv`: 110 answers in all, every one `NOT_MINE`.
  - The area rows' answers are 1,774 `NOT_MINE` out of 1,825 (97.2%), with
    34 `CLAIMED_BY`, 9 `COVERS`, 5 `PARTIAL` and 3 `UNKEYED`.
  - The sampled rows come from the SOLVER, VIEW, WSUI, FEATB and COREB
    areas. None of their EntryPoints hits a path the PKG-10 deliverables
    declare, so an all-`NOT_MINE` result is the expected outcome.
  - I see no sign of anchoring. The test has little power here, because
    no sampled row was a plausible PKG-10 capability.

## 7. For the owner, stated plainly

1. **Scale-out gate input.** The firm false-alignment rate is 0.0% (0 of
   21), within the 5% gate. PKG-10 does not need a rerun.
2. **One contested row to record in `WAVES/W3/RESOLUTIONS.csv`.**
   DEL-10-05 `CLM-004.r04` (F-1). The format-list TBD was overtaken by
   OI-004 and SCA-004, and the other worker already treats it that way.
3. **A possible code defect for R4.** It is confirmed at the freeze, and it
   goes to REVIEW, not to a decision. The in-product build-readiness panel
   reports the app bundle as inactive, and signing, matrix and publishing
   as TBD. The shipped Tauri config has the bundle active, and DEC-057 and
   DEC-089 ruled those items.
4. **Items for the owner to confirm at R4 (CP-10).** Code settled three
   Human-Ruling-Queue holds with no ruling: the API contract file layout
   (DEL-10-01), and the handoff schema's location and field layout and its
   advisory-label vocabulary (DEL-10-03).
5. **An owner decision (DEL-10-01 `REQ-02`).** The public API family list
   rests only on local design now that PRD v0.4 has dropped §19.3. The
   model-creation, load-case and rule-pack-evaluation families are absent.
6. **R3 observations. None changes a row.**
   - DEL-10-02's adapter packet still reports the ruled CI provider and
     container as TBD.
   - DEL-10-05's result-envelope builder lost six unit tests in
     `b43cc00c4`, when they were replaced by tests for the rewritten
     builder. No cited test was removed.
   - The two workers read two conventions differently:
     - W-2: whether a CONTEXT SURFACE row repeats the pin that its keyed
       CS-01 row already carries;
     - W-1: whether "shall validate" in a contract-type deliverable is a
       claim about the contract or about runtime behaviour.

     A convention note would settle both before later waves.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
