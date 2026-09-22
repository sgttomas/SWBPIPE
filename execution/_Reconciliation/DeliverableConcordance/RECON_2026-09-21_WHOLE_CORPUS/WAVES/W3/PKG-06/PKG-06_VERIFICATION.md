VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-06 verification (wave W3, sampling STANDARD)

Fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Parent: HELP_HUMAN Agent 0.
Brief: `briefs/R2-VERIFIER_brief.md` (SHA-256 `47fb3c52…5b00`, checked and
matching). Evidence checkout: `00115c71931bcae79909602d653740d3bb72dfa1`
(freeze, read-only; `git rev-parse HEAD` confirmed). Rules applied:
`CONVENTIONS.md` (Parts A to F), `CANONICAL_SITUATIONS.md`,
`AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`. The PKG-06 manager's transcripts
and the worker returns and notes were read for their flags only. They were
not used as evidence for any claim.

These are agent verification judgments. They are not owner rulings, and they
do not state or imply any release, approval, compliance or certification
(F-PIP-2; claims taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** For all five ledgers, the forward SHA-256 I recomputed matches
  the `_SEAL.txt` line and the manager's return. The five reverse hashes also
  match the return.
- **Single mode.** I ran `validate_ledger_v2.py` with `--forward --reverse
  --inventory ROUTING/PKG-06_capabilities.csv --notes-gap`, with the repo
  root set once to the freeze and once to the worktree. All five pass with
  0 findings both times:

  | Deliverable | Forward rows | Required keys | Canonical rows |
  |---|---|---|---|
  | DEL-06-01 | 93 | 71 | 7 |
  | DEL-06-02 | 139 | 88 | 7 |
  | DEL-06-03 | 90 | 65 | 7 |
  | DEL-06-04 | 116 | 80 | 7 |
  | DEL-06-05 | 124 | 70 | 7 |

- **Batch mode.** `--batch` over the five forward ledgers: PASS, 0
  consistency findings. No `WAVES/W3/RESOLUTIONS.csv` exists.
- **Cross-wave batch.** I also ran `--batch` over all 44 sealed W1 and W2
  ledgers plus these five, with `WAVES/W2/RESOLUTIONS.csv`. It reports 4
  findings: DEL-07-06 R04, DEL-07-05 CLM-004.r07 and REQ-07-05-005, and
  DEL-00-07 REQ-07-03. None of them is a PKG-06 row. The PKG-06 CP-11 rows
  are in the majority group (`PROJECT_BASELINE · SECURITY`).
- **Keyed canonical rows (CS-01, 02, 04, 06, 07).** The validator checks
  these on 100% of rows. All conform.
- **Cited test cases.** I checked all 151 `path::case` tokens in the five
  forward ledgers against the freeze. Every file exists and every named case
  is present.
- **Gate evidence.** `GATE_EVIDENCE/B4_4_SWEEP_9D55/sweep.log.gz` registers
  `cargo test` for the five rule crates (`completeness_checker`,
  `expression_evaluator`, `rule_check_runner`, `rule_pack_document`,
  `rule_pack_lifecycle`). `SUMMARY.json` records a 1,138-pass Python run.
  The ledgers' "not rerun" citations are therefore supported (A6).

## Sampling (deterministic)

Each class's candidate keys were sorted by the SHA-256 of the full claim key,
and the lowest fraction was taken, rounded up. STANDARD rates were used:

| Code | Class | Rate |
|---|---|---|
| C100 | `AuthorityTier=INVARIANT`, or disposition UNKNOWN, ACCEPTED_DIVERGENCE, AUTHORITY_CONFLICT or LIFECYCLE_REASSESSMENT_REQUIRED, or baseline PROTECTED_CHECK or FROZEN_CONTRACT | 100% |
| SH | `SharedTextCount > 1` in `CLAIM_KEYS_V2.csv` (a minted `.sNN` or `.rNN` key takes its parent's count) | 100% |
| F2/4 | ALIGNED, with Notes carrying `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% |
| F7 | ALIGNED, with Notes carrying `PRODUCT_CALLER: NONE` | 25% |
| NA | Other non-aligned rows | 25% |
| AN | ALIGNED normative rows | 20% |
| Q | Structural rows, inherited canonical rows and other quiet rows | 10% |

- Each row falls in the first class that matches, in the order above.
- No deliverable is ISSUED. No row carries `PROTECTED_CHECK` or
  `FROZEN_CONTRACT`.
- No row carries `PRODUCT_CALLER: NONE`, so the F7 class is empty. The G1
  and G2 notes both identify product callers for the rule engines:
  `validate_rule_pack`, `compute_rule_pack_document_checksum` and
  `run_rule_checks` in `apps/desktop/src-tauri/src/lib.rs`. I confirmed
  these at lines 2695–2830. `rule_check_runner` depends on the evaluator and
  the completeness checker (its `Cargo.toml`).

Disclosed departures:

- **AN class.** It was selected by hash only, not weighted toward LOW or
  MEDIUM confidence. To compensate, I reviewed every G2 row that bears on
  protected content (MEDIUM confidence) outside the sample. The results are
  in §4.2.
- **Outside the sample.** Beyond the sample, I read all rows the workers
  flagged, and every row in the package that touches INIT.md, lifecycle
  states, MEMORY or protected content.

In total, 193 of 562 forward rows were sampled.

## 1. Package-level result

- **Firm disagreements on sampled rows: 0.** There is 1 firm disagreement
  outside the sample (§4.2).
- **Weak disagreements on sampled rows: 4.** There are more outside the
  sample, in 4 clusters.
- **Field disagreements on sampled rows: 1.** There are 2 outside the
  sample.
- **Package firm false-alignment rate: 0 / 56 sampled `ALIGNED` normative
  rows = 0.0%** (gate ≤ 5%: met). The outside-sample firm row is an ALIGNED
  ACCEPTANCE row. Adding it would give 1 / 57 = 1.8%, still within the
  gate.
- **No deliverable meets a rerun condition.** No firm rate exceeds 10%. No
  firm error falls in a 100%-sampled class that changes tier or owner
  routing.

## 2. Per-deliverable tables

Cells read "total / sampled". "False-align" is the firm false-alignment rate
among sampled `ALIGNED` normative rows, drawn from every class.

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-06-01 | 93 | 7/7 | 8/8 | 13/13 | 0 | 23/6 | 18/4 | 24/3 | 41 | 0 | 1 | 0 | 0/16 = 0% |
| DEL-06-02 | 139 | 6/6 | 7/7 | 9/9 | 0 | 43/11 | 25/5 | 49/5 | 43 | 0 | 1 (+3 outside) | 0 (+1 outside) | 0/11 = 0% |
| DEL-06-03 | 90 | 2/2 | 10/10 | 9/9 | 0 | 24/6 | 8/2 | 37/4 | 33 | 0 | 1 | 0 | 0/10 = 0% |
| DEL-06-04 | 116 | 0 | 11/11 | 8/8 | 0 | 32/8 | 28/6 | 37/4 | 37 | 0 | 0 (+4 outside) | 0 (+1 outside) | 0/10 = 0% |
| DEL-06-05 | 124 | 0 | 18/18 | 2/2 | 0 | 30/8 | 34/7 | 40/4 | 39 | 0 (+1 outside) | 1 (+2 outside) | 1 | 0/9 = 0% |

Sampled keys (suffixes after `<DEL>:`):

- **DEL-06-01**
  - C100: SOW#CLM-004.r08, CLM-011/REQ-06-01-007, CLM-011/REQ-06-01-011,
    CLM-013, CLM-014, CLM-020, CLM-021.s01.
  - SH: CONTEXT#objective-support, #package-reference,
    #decomposition-reference, #architecture-basis-injection (with .s01 and
    .s02), #preparation-notes; STATUS#remaining.
  - F2/4: SOW#CLM-004.r10, CLM-005, CLM-006, CLM-011/REQ-06-01-001 to -006
    and -008, CLM-012.r05, CLM-028; CONTEXT#anticipated-artifacts.
  - NA: SOW.s01, CLM-003, CLM-010, CLM-012.r02, CLM-017; MEMORY.s01.
  - AN: CLM-004.r02, .r06, .r12; CLM-026.
  - Q: CLM-015, CLM-029; CONTEXT#description.
- **DEL-06-02**
  - C100: purpose OUT-001; CLM-006.r01; CLM-013/REQ-06-02-007 and -011;
    CLM-016/REQ-06-02-007; CLM-023.
  - SH: the seven CONTEXT keyed and sub-claim rows.
  - F2/4: CLM-005.r04, .r05; CLM-013/REQ-06-02-004 and -009;
    CLM-016/REQ-06-02-004; CLM-032; STATUS#remaining/R01, R02, R03.
  - NA: SOW.s02; CLM-004; CLM-006.r04, .r05; CLM-010;
    CLM-013/REQ-06-02-006; CLM-015.r01, .r05; CLM-016/REQ-06-02-006;
    CLM-022.r03, .r06.
  - AN: CLM-006.r02; CLM-013/REQ-06-02-001, -002 and -008;
    CLM-016/REQ-06-02-002.
  - Q: CLM-008.r07, CLM-026, CLM-029, CLM-035; CONTEXT#context-budget-qa.
- **DEL-06-03**
  - C100: CLM-011/R-DEL-06-03-002; CLM-014.
  - SH: the seven CONTEXT rows; CLM-007.r01, .r02; STATUS#remaining.
  - F2/4: purpose OUT-001; CLM-005; CLM-011/R-DEL-06-03-001, -003, -004
    and -005; CLM-028, CLM-029; CONTEXT#description.
  - NA: SOW; AC-001; CLM-013.r03; CLM-015.s01; CLM-019.r05; CLM-024.
  - AN: CLM-026, CLM-027.
  - Q: CLM-001, CLM-007.r04, .r08; MEMORY.
- **DEL-06-04**
  - SH: the seven CONTEXT rows; CLM-002.r03, .r04, .r05; CLM-019.r05.
  - F2/4: CLM-003.r05; CLM-004.r04; CLM-007, CLM-014, CLM-021;
    CLM-025.r03; STATUS#remaining/R01, R02.
  - NA: SOW; CLM-005, CLM-017, CLM-018; CLM-019.r04; CLM-025.r05; matrix
    OUT-001; VER-001.
  - AN: CLM-003.r04; CLM-004.r03; CLM-010/R-06-04-009; CLM-019.r03;
    CLM-025.r06, .r07.
  - Q: deliverable-definition-ontology; CLM-002, CLM-012, CLM-025.
- **DEL-06-05**
  - SH: the seven CONTEXT rows; CLM-003.r03 to .r08; CLM-007.r01, .r04,
    .r10; CLM-012.r01; STATUS#remaining.
  - F2/4: CLM-006.r06; CLM-025.r05.
  - NA: SOW.s01; CLM-005.r04; CLM-006; CLM-017; CLM-018.r03; CLM-020.r04;
    CLM-025.r06; MEMORY.
  - AN: purpose OUT-001; CLM-006.r02, .r03; CLM-011/DEL-06-05-REQ-04 and
    -06; CLM-013/DEL-06-05-VER-05; CLM-018.r04.
  - Q: governing-values-and-decisions-axiology; CLM-007.r11; CLM-025;
    CONTEXT#anticipated-artifacts.

### Checks behind the main sampled readings

These were confirmed at the freeze:

- **INVARIANT UNKNOWN rows (IP_DATA; FG-DEL-06-01-02, FG-DEL-06-02-02,
  FG-DEL-06-03-02).**
  - `validation/evidence/releases/` does not exist, so there is no DEC-058
    scan record.
  - The only human disposition in PKG-06 is
    `…/1_Working/_run_records/WORKING_ITEMS_RUN_2026-06-05_2207_PKG06_HUMAN_DISPOSITION.md`.
    It covers four PKG-02 technical findings. Its own boundary clause
    excludes protected standards data.
  - Agent boundary statements do exist, but they are self-reviews that
    predate the frozen bytes:
    - DEL-06-05 `TASK_RUN_2026-04-30_1033_validation.md` and
      `TASK_RUN_2026-06-05_2159.md` (an `rg` scan);
    - DEL-06-01 `WORKING_ITEMS_RUN_2026-06-12_TP-C2-SCHEMA-001.md` §"Boundary
      review".
  - The demo was changed afterwards (a0625a0d5, 3663a7547, ad27dc4d7 and
    f737985ab, all 2026-06-14). No record binds the frozen bytes.
    `UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA · REVIEW` stands.
    `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN` would also be defensible,
    with the same tier and routing.
- **DEL-06-01 REQ-06-01-011 (PARTIALLY_IMPLEMENTED, INVARIANT).** Confirmed.
  `schemas/rule_pack.schema.yaml` has `PrivacyClass.public_invented_example`
  but no `if/then` that ties it to invented values or a notice. The only
  `allOf` conditionals are on `declaration_form` and `payload_kind`.
- **DEL-06-02 OUT-001 (PARTIALLY_IMPLEMENTED, INVARIANT · SECURITY).**
  Confirmed. `core/rules/expression_evaluator/src/lib.rs` (2,331 lines) has
  no depth, size or recursion limit. The document decoder has none either.
- **DEL-06-02 CP-10 tolerance rows (CLM-006.r03, CLM-015.r04).**
  - DEC-024 and DEC-026 govern *verification* tolerance classes (D-04 packet
    scope), not rule-comparison tolerance.
  - The corpus README says tolerance-bearing comparisons "wait on the D-04
    ruling". D-04 has since ruled without addressing them.
  - `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · OWNER` stands.
- **DEL-06-02 Remaining R01 to R03.** These are accurate, and each names its
  governing row (F2). `BLOCKED_RUNTIME_NOT_SELECTED` is at
  `core/adapters/framework/adapter_framework.py:532`. The source has 31 unit
  tests, and the corpus has 69 case files plus a README.
- **DEL-06-04 CLM-025.r05 (and R-06-04-008, outside the sample).**
  `audit_manifest_entry` exists (`rule_pack_lifecycle/src/lib.rs:237`). The
  product report request sends empty `rule_pack_refs`. `PARTIAL_SLICE`
  stands (F7).
- **DEL-06-04 CLM-019.r05.** The 2026-06-16 VALID record is in commit
  28219696d, which is also the last change to `Dependencies.csv`. "Not
  rerun" is correct (A6).
- **DEL-06-04 CP-03 rows ALIGNED against the G1 CP-03 rows.** This is not a
  conflict. The DEL-06-04 PDU-054 declaration carries no revision pin, and
  the G1 and DEL-06-05 declarations pin revision 0.8 and DAG-007.

## 3. (Package-level) scale-out gate input

Firm false alignment: **0 / 56 = 0.0%** on sampled ALIGNED normative rows.
The gate is ≤ 5%.

## 4. Disagreements

### 4.1 On sampled rows

**W1 — DEL-06-01:SOW#CLM-005 (weak).**

- *Row:* EXCLUSION · ALIGNED. The setup-pass conditions are read as history
  or conditional.
- *Found:* Bullet 1 ("This setup pass does not create or edit
  `schemas/rule_pack.schema.yaml`, `docs/SPEC.md`, examples, evaluator
  code…") has the same substance as CLM-010 paragraph 2 and CLM-017. The same
  ledger disposes those two rows `STALE_SETUP_SPECIFICATION ·
  DOC_BEHIND_CODE`. DEL-06-05 disposes the equivalent write-boundary text
  (CLM-003.r08) as stale too.
- *Right values under the ledger's own reading:* `STALE_SETUP_SPECIFICATION
  · DOC_BEHIND_CODE · LOCAL_DESIGN · NONE · RECORD · NO` (FG-DEL-06-01-03),
  or a `.sNN` split of bullet 1.
- *Why weak:* The conventions do not settle when "this setup pass does not…"
  counts as accurate history rather than a current declaration. The worker
  of DEL-06-05 raised the same question on REQ-07.

**W2 — DEL-06-02:SOW.s02 and DEL-06-03:SOW#CLM-007.r01 (weak, one
cluster).**

- *Rows:* CP-02 with its canonical class, `STALE_REVIEW_OR_EVIDENCE ·
  BASIS_POINTER_STALE`. The first is a SPEC §6 section reference; the second
  is an INIT.md file pointer.
- *Found:* Both texts were first present at `7bee9ae41`. The INIT.md text
  is in each four-document `Datasheet.md` at that commit. F3's exceptions
  cover revision pins, review states and metadata. They do not cover file
  pointers or section references. G2 applied F3 to the same situation, with
  `CANONICAL_DEPARTURE:` (DEL-06-04 CLM-006, CLM-017; DEL-06-05 CLM-007.r03,
  CLM-018.r02). G1 kept CP-02's class, both here and on DEL-06-02 CLM-008.r01
  outside the sample.
- *Right values under a literal reading of F3:* `STALE_SETUP_SPECIFICATION ·
  BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO`.
- *Why weak:* CP-02 names "section reference" explicitly, and F3 says its
  exceptions "keep … the canonical table intact". Agent 0 should settle
  whether pattern (CP) rows are within F3's origin test. Tier and routing do
  not change.

**W3 — DEL-06-05:SOW#CLM-018.r04 (weak, with a field defect).**

- *Row:* REQUIREMENT · ALIGNED MEDIUM, "Protected-data posture: no protected
  standards text, formulas, tables, examples or proprietary data used".
  `VerificationEvidence` is `tests/test_report_protected_content_linter.py`.
- *Found:*
  - That test checks only the report-linter schema and its
    `fixtures/report_lint/invented/*` files. It does not scan
    `examples/rule_packs/invented_demo.yaml`, the notice or any rule-pack
    artifact.
  - ALIGNED therefore rests on an agent reading and the demo's
    self-declared `protected_content_review` status.
  - G1 disposes the same substance as `UNKNOWN · EVIDENCE_NOT_LOCATED ·
    INVARIANT · IP_DATA · REVIEW`, for example DEL-06-01 CLM-004.r08 and
    DEL-06-02 CLM-006.r01. So does the W1 FIRM resolution on DEL-07-02
    CLM-026.
- *Right values under the G1 and W1 reading:* `UNKNOWN ·
  EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA · REVIEW`. The field defect
  holds under either reading: the cited test does not exercise the claim.
- *Why weak:* This row states a posture and names no review. Whether an
  agent inspection of visibly invented content suffices is a reading the
  conventions should settle (see §5, conflict C1).

**F1 — DEL-06-05:MEMORY (field).**

- *Row:* The SURFACE row is `HISTORY · STALE_SETUP_SPECIFICATION ·
  DOC_BEHIND_CODE`, because the undated Open Items block is overtaken.
- *Found:* The finding is right, but the vehicle is wrong. C1 says accurate
  history is ALIGNED and "an undated current declaration inside a history
  surface is DECLARED_STATE". Parts that take different dispositions need a
  `.sNN`. G1 did this on all three of its ledgers (MEMORY ALIGNED plus
  MEMORY.s01 DECLARED_STATE).
- *Right values:* MEMORY → `HISTORY · ALIGNED`. The Open Items block →
  `MEMORY.s01 · DECLARED_STATE · STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE
  · LOCAL_DESIGN · NONE · RECORD · NO`.

### 4.2 Outside the sample (not in the rates)

**X1 — DEL-06-05:SOW#CLM-020.r05 (firm).**

- *Row:* ACCEPTANCE · ALIGNED MEDIUM. The claim is "Check: no
  standards-derived tables, formulas, allowables or certification language
  in local artifacts". The row says "Holds at the freeze". It cites the
  report-linter test and `tests/test_claims_language_surface.py`.
- *Found:*
  - The SOW check (line 287) is "Search local artifacts for
    standards-derived tables, formulas, allowables, or certification
    language".
  - The claims-language test covers the certification-language element
    repo-wide. It is included in the gate sweep's Python run.
  - The report-linter test does not cover the rest.
  - The only search record is DEL-06-05 `TASK_RUN_2026-06-05_2159.md`, which
    predates the demo changes of 2026-06-14.
  - Asserting a search result with no record that binds the frozen bytes
    breaches A6 and CP-12.
- *Right values:* `UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA ·
  REVIEW`. `RemainingWork`: locate or run a protected-content search or
  review of the demo, the notice and the deliverable folder at the frozen
  bytes.

**X2 — G2 protected-content absence rows (weak cluster; see conflict C1).**
Each is ALIGNED MEDIUM on an agent reading, and each cites the report-linter
test, which does not exercise it:

- DEL-06-04: CLM-004.r01, CLM-010/R-06-04-003, CLM-012/V-06-04-007 and
  CLM-019.r02 ("Data boundary scan").
- DEL-06-05: CLM-005.r01 and CLM-012.r02.

V-06-04-007 and CLM-019.r02 are checks, like X1. They are weaker cases,
because the DEL-06-04 crates carry lifecycle metadata and invented test
payloads only. Right values under the G1 reading: as X1.

**X3 — DEL-06-02:SOW#CLM-024.r05 and .r06 (weak, and a field defect on
.r05).**

- *Rows:* `STALE_SETUP_SPECIFICATION · RECORD_DRIFT · RECORD;LIFECYCLE`.
- *Found:*
  - .r06 declares "current state remains SEMANTIC_READY". F3 as
    owner-confirmed (Direction 8) treats readiness and lifecycle states as
    review states, which stay `STALE_REVIEW_OR_EVIDENCE` whatever their
    origin. .r05 checks the same status value.
  - The .r05 Note says "`tools/validation/validate_enum.py` does not exist
    at the freeze". That is wrong: the file exists at the repository root.
    DEL-06-05 CLM-020.r03 cites it correctly.
- *Right values:* `STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT · LOCAL_DESIGN ·
  NONE · RECORD;LIFECYCLE · NO`, and a corrected Note.

**X4 — DEL-06-02:SOW#CLM-008.r01 (weak).** INIT.md pointer. Same as W2.

**X5 — DEL-06-04:MEMORY (field).** Same as F1.

**X6 — DEL-06-01 FG-DEL-06-01-02 Notes (observation, not a disagreement).**
"No such review was located" is accurate for reviews. But the agent
boundary statements listed in §2 exist. They are self-reviews, overtaken by
later demo changes, and could have been cited as the last record.

**Flags from the workers, checked:**

- DEL-06-04 R-06-04-004 (ALIGNED despite the canonical_json divergence
  beyond 2^53): agreed. `core/serialization/canonical_json/src/lib.rs:21-26`
  documents the divergence as outside the I-JSON envelope that RFC 8785
  assumes. Rule-pack quantities decode as `f64` (`number_member`).
- DEL-06-05 REQ-07 and CLM-003.r08, read as stale: agreed for the sampled
  CLM-003.r08. Write-boundary and execution-mode rows in an identification
  table read as current declarations. The inconsistent row is DEL-06-01
  CLM-005 (W1), not these.
- INIT.md rows, CP-02 with F3: see W2.
- DEL-06-04 C-06-04-002 (CP-10): agreed. The enum was settled by the
  DEL-06-01 schema, following the row's own proposal. No ruling covers it.
  DEC-038 and DEC-039 ratify other schema members.
- DEL-06-02 CLM-006.r03 and CLM-015.r04 (CP-10): agreed. See §2.
- DEL-06-01 CLM-006 (the eleven groups, "to resolve later"): agreed, ALIGNED.

## 5. Batch consistency and shared-situation conflicts

Single mode, batch mode and the cross-wave batch are reported in §0: no
PKG-06 finding. Four conflicts are invisible to batch mode, because the
bodies differ or the `.sNN` keys have no hash. All four split along the G1
and G2 worker boundary:

- **C1 — protected-content absence claims.**
  - G1 (DEL-06-01 to -03) disposes public-artifact "no protected content"
    claims as `UNKNOWN · INVARIANT · IP_DATA · REVIEW` when no review is
    located. That is 10 rows in 3 FindingGroups.
  - G2 (DEL-06-04, -05) disposes equivalent claims ALIGNED MEDIUM on an
    agent reading. That is 8 rows: X1 (firm), W3 and the six X2 rows.
  - This conflict changes tier and routing: `INVARIANT/REVIEW` against a
    quiet row.
  - My reading: rows that name a review, a scan or a search are UNKNOWN
    (A6, CP-12, W1 precedent). Pure prohibitions are contestable.
  - Agent 0 should resolve C1 corpus-wide in `WAVES/W3/RESOLUTIONS.csv`.
- **C2 — CP-02 non-revision pointers of setup origin.** G1 keeps
  `STALE_REVIEW_OR_EVIDENCE`. G2 applies F3 (`STALE_SETUP_SPECIFICATION`)
  with a departure. See W2 and X4.
- **C3 — undated MEMORY blocks.** G1 uses a `.s01` DECLARED_STATE row. G2
  disposes the HISTORY surface. See F1 and X5.
- **C4 — lifecycle-state statements.** DEL-06-02 CLM-024.r06 uses
  `STALE_SETUP_SPECIFICATION` against F3's owner-confirmed exception. See
  X3.

## 6. Reverse pass

**Checks run.**

- I checked all 19 non-`NOT_MINE`, non-`COVERS` answers: 7 `CLAIMED_BY`,
  8 `PARTIAL` and 4 `CONSTRAINS`, including the multi-claimed capabilities.
  There are no `UNKEYED` answers.
- I checked a 10% hash-selected sample of `NOT_MINE` per deliverable:
  29 + 29 + 29 + 28 + 29 = 144 rows. I found no misallocation.
- **F5.** I checked every `NOT_MINE` answer whose capability entry points
  hit a path the deliverable's forward ledger cites. That is 79 rows (11,
  18, 19, 12, 19), against the brief's required 20%. All 79 give
  capability-specific reasons. None uses the per-deliverable template.
- No answer anchors on a `STATUS#…` key.

**Answer distribution against routing.** The mapping uses
`ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`: 248 AREA rows and 44 SAMPLE rows per
deliverable.

| | AREA | SAMPLE |
|---|---|---|
| NOT_MINE | 1,199 / 1,240 (96.7%) | 220 / 220 (100%) |
| CLAIMED_BY / PARTIAL / COVERS / CONSTRAINS | 41 | 0 |

- Two SAMPLE rows have entry points on paths the package cites:
  - RC-06-0158 (CAP-COREB-035, the unit-catalog command in the shared
    desktop `lib.rs`);
  - RC-06-0053 (CAP-COREB-037, `core/units/README.md`).
- Every deliverable answered both `NOT_MINE`, with specific reasons.
  Correct.
- All 41 claim-type answers are on AREA rows. About half the SAMPLE rows are
  recognisable from their paths, so this comparison supports, but does not
  prove, that the answers were unanchored.

**Capabilities claimed by more than one deliverable.**

- **RC-06-0006** (`rule_check_runner`): DEL-06-02 PARTIAL (the evaluation
  stage) and DEL-06-03 PARTIAL (the completeness stage). The two partitions
  complement each other.
- **RC-06-0041** (`fixtures/rule_expressions/`): DEL-06-02 PARTIAL (the
  conformance corpus) and DEL-06-04 PARTIAL (the checksum-binding payloads).
  These partitions complement each other too.

**Suspected missed claims (for R3).**

- **RC-06-0006.** The runner's orchestration and ratio-against-limit
  acceptability stage has no owner in PKG-06. DEL-06-04 and DEL-06-05 call
  it "C4 rule-check work". R3 should find its owner, possibly in PKG-10.
- **RC-06-0256** (the expression codec in `rule_pack_document`). DEL-06-02
  claims only "the evaluator side", on the ground that the crate sits in
  DEL-06-04's document crate. DEL-06-04 answers NOT_MINE ("serves DEL-06-01
  and DEL-06-02 grammar handling"). The rest of the codec, including
  encoding, is unowned.
- **RC-06-0290** (the desktop Rule-check completeness panel). It is
  answered NOT_MINE package-wide. DEL-06-03's notes say it derives "blocked"
  findings heuristically, separately from the checker. So there are two
  completeness explanations, for R3.

## 7. For the owner

1. **Protected-content review is missing for the public rule-pack
   artifacts.**
   - No DEC-058 scan and no reviewer disposition covers the public rule-pack
     schema, the invented demo pack, the notice, or the evaluator and
     checker fixtures at the frozen bytes.
   - The package's two workers disagree on how to record this (C1). Ten
     rows route it to you as `INVARIANT · REVIEW`, and eight more treat it
     as satisfied.
   - One review record would close all of them.
2. **The rule evaluator has no resource bounds.** It has no depth or size
   limits and it recurses on untrusted pack input. DEL-06-02 OUT-001 records
   this as an `INVARIANT · SECURITY` partial.
3. **Two items were settled in code with no ruling (CP-10, for your
   decision):**
   - rule-comparison tolerance is exact, with DEC-022's corpus pinning it
     (DEL-06-02 CLM-006.r03 and CLM-015.r04);
   - the redistribution and privacy status enum (DEL-06-04 C-06-04-002).
4. **For R3:**
   - active identifiers carrying the former name: the crates
     `open_pipe_stress_{rule_pack_lifecycle,expression_evaluator,completeness_checker}`,
     `rule_pack_kind: open_pipe_stress_rule_pack`, and the
     `expression_language` token;
   - SOFTWARE_DECOMP rev 0.12 OI-006 still reads TBD after DEC-022;
   - no product report or export populates `rule_pack_refs`.
5. **Agent 0 should record** C1 to C4 and the rows above in
   `WAVES/W3/RESOLUTIONS.csv`. The sealed ledgers stay unchanged (F6).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
