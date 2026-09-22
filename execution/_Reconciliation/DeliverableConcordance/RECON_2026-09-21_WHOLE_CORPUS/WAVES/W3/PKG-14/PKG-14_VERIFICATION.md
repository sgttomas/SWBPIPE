VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-14 verification (wave W3, sampling STANDARD)

This is the report of the fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. The parent is HELP_HUMAN Agent 0.

- **Brief.** I checked the SHA-256 of `briefs/R2-VERIFIER_brief.md`
  (`47fb3c52…5b00`) and it matches.
- **Evidence checkout.** I confirmed the freeze HEAD is
  `00115c71931bcae79909602d653740d3bb72dfa1`.
- **Rules applied.** `CONVENTIONS.md` Parts A to F (F1 to F8 judged like any
  other rule), `CANONICAL_SITUATIONS.md` and
  `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- **Manager and worker records.** I read `W3-PKG-14-MANAGER/` (the manager
  return, the validator transcripts and both worker returns) and the two
  `_WORKER_*_NOTES.md` files only for the flags they raise. None of them is
  used as evidence about a claim.
- **Other W3 records.** For precedent on corpus-wide contested classes I
  consulted `WAVES/W1/RESOLUTIONS.csv`, `WAVES/W2/RESOLUTIONS.csv` and the W3
  drafts. I did not read the July 2026 ledgers.

These are agent verification judgments. They are not owner rulings, and they
do not state or imply any release, approval, compliance or certification
(F-PIP-2; claims taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** I recomputed the SHA-256 of all five forward ledgers. Each one
  matches its `_SEAL.txt` line, `MANAGER_RETURN.md` and the worker return.
  The five reverse hashes also match the worker returns.
- **Single mode.** I ran `validate_ledger_v2.py` against the freeze with
  `--forward --reverse --inventory ROUTING/PKG-14_capabilities.csv
  --notes-gap`. Every ledger passes with 0 findings:

  | Deliverable | Forward rows | Required keys | Canonical rows | Result |
  |---|---|---|---|---|
  | DEL-14-01 | 78 | 64 | 9 | PASS, 0 findings |
  | DEL-14-02 | 73 | 60 | 9 | PASS, 0 findings |
  | DEL-14-03 | 77 | 74 | 8 | PASS, 0 findings |
  | DEL-14-04 | 110 | 81 | 8 | PASS, 0 findings |
  | DEL-14-05 | 109 | 60 | 9 | PASS, 0 findings |

- **Batch mode.** `--batch` over the five forward ledgers returned `PASS
  batch of 5 ledgers: 0 consistency findings`, the same as the manager's
  run. `WAVES/W3/RESOLUTIONS.csv` does not exist, so I did not pass
  `--resolutions`.
- **Keyed canonical rows.** The validator checks the CS-01 to CS-07 rows
  mechanically on 100%. All 43 conform.
- **Package facts.**
  - No PKG-14 deliverable is ISSUED.
  - No row is `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT` or
    `LIFECYCLE_REASSESSMENT_REQUIRED`.
  - The 100% class (C100) holds 12 rows:
    - 6 `UNKNOWN` rows at tier `INVARIANT`;
    - 2 `PARTIALLY_IMPLEMENTED` rows at `INVARIANT`;
    - 3 `VERIFIED_NOT_VALIDATED` rows at `INVARIANT`;
    - 1 `FROZEN_CONTRACT` row (the DEL-14-02 STATUS surface).
  - No row carries `PROTECTED_CHECK`.

## Sampling (deterministic)

- **Method.** Within each class I sorted the candidate keys by the SHA-256
  of the full claim key and took the lowest fraction, rounded up. Sampling
  was STANDARD, so no rate was doubled.
- **Class order.** Each row falls into the first class that matches:
  1. C100 (100%);
  2. SH, rows whose unit has `SharedTextCount > 1` (100%). A minted `.sNN`
     key takes its parent's count;
  3. F2/4, `ALIGNED` rows carrying `GAP_WORDING_CHECKED:` or
     `OPEN_ACTION:` (100%);
  4. F7, `ALIGNED` rows carrying `PRODUCT_CALLER: NONE` (25%);
  5. NA, other non-aligned rows (25%);
  6. AN, `ALIGNED` normative rows (20%);
  7. Q, structural rows, inherited canonical rows and other quiet rows
     (10%).
- **Disclosed departure.** AN rows were selected by hash alone. They were
  not weighted toward LOW or MEDIUM confidence. (No AN row in this package
  is LOW.)

**Rows judged beyond the sample.** I also judged every row that the workers
or the manager flagged, and every row needed for the cross-deliverable
comparison in §5:

- DEL-14-01: CLM-011.r09, CLM-027 and CLM-011.r01 (FG-01 and FG-02).
- DEL-14-02: OUT-001, CLM-011.r03, r05, r09 and r10, and CLM-024.
- DEL-14-03: CLM-032/C-14-03-001, AC-001, REQ-14-03-001 and OUT-001
  (both keys).
- DEL-14-04: CLM-004.r07, R-14-04-004 and R-14-04-007, and the SOW and
  STATUS surfaces.
- DEL-14-05: CLM-011.r08, CLM-013.r03, `CONTEXT#anticipated-artifacts` and
  the SOW surface.

**Total: 167 of 447 forward rows sampled.**

## 1. Package-level result

- **Firm disagreements on sampled rows: 0.**
- **Weak disagreements: 6** forward rows, in three patterns (§4). Two of
  them belong to a corpus-wide cluster that is already contested.
- **Field disagreements on sampled rows: 0.** There is 1 field observation
  beyond the sample (§4.4).
- **Package firm false-alignment rate: 0 / 43 sampled `ALIGNED` normative
  rows = 0.0%.** The scale-out gate is 5% or less, so it is met.
- **Rerun test.** No deliverable meets a rerun condition. Every firm error
  rate is 0%, and there is no firm error in any 100%-sampled class.

## 2. Per-deliverable tables

Cells read "total / sampled".

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | Firm false-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-14-01 | 78 | 5/5 | 9/9 | 11/11 | 5/2 | 20/5 | 3/1 | 25/3 | 36 | 0 | 1 | 0 | 0/10 = 0% |
| DEL-14-02 | 73 | 3/3 | 8/8 | 5/5 | 0 | 30/8 | 7/2 | 20/2 | 28 | 0 | 1 | 0 | 0/3 = 0% |
| DEL-14-03 | 77 | 1/1 | 9/9 | 6/6 | 15/4 | 23/6 | 2/1 | 21/3 | 30 | 0 | 2 | 0 | 0/9 = 0% |
| DEL-14-04 | 110 | 2/2 | 9/9 | 9/9 | 26/7 | 35/9 | 2/1 | 27/3 | 40 | 0 | 1 | 0 | 0/13 = 0% |
| DEL-14-05 | 109 | 1/1 | 8/8 | 2/2 | 22/6 | 42/11 | 6/2 | 28/3 | 33 | 0 | 1 | 0 (1 beyond sample) | 0/8 = 0% |
| **Package** | **447** | 12/12 | 43/43 | 33/33 | 68/19 | 150/39 | 20/7 | 121/14 | **167** | **0** | **6** | 0 (1) | **0/43 = 0.0%** |

The denominator for false alignment is every sampled row that is `ALIGNED`
with claim type `REQUIREMENT`, `ACCEPTANCE` or `EXCLUSION`, whatever its
sampling class.

Sampled keys (the part after `<DEL>:`):

- **DEL-14-01:**
  - C100: SOW#CLM-011.r09, CLM-013, CLM-021, CLM-027 and CLM-020.
  - SH: the nine CONTEXT canonical blocks and ABI sub-claims .s01 to .s03.
  - F2/4: CLM-024, STATUS R01 and R02, AC-001, MEMORY,
    CONTEXT#description, purpose OUT-001, CLM-011.r10, CLM-014,
    CLM-011.r07 and r03.
  - F7: CLM-011.r06 and CLM-025.
  - NA: SOW, CLM-011.r01, CLM-023, STATUS and CLM-005.
  - AN: CONTEXT#context-envelope.
  - Q: CLM-018/DEL-02-05, CONTEXT#anticipated-artifacts and CLM-029.
- **DEL-14-02:**
  - C100: CLM-014, CLM-021 and STATUS.
  - SH: the eight CONTEXT canonical blocks and sub-claims.
  - F2/4: STATUS R01 and R02, CONTEXT#description, MEMORY and CLM-024.
  - NA: CONTEXT, CLM-020, CLM-011.r06, CLM-005, CLM-004, CLM-009, CLM-010
    and CLM-012.
  - AN: CONTEXT#context-envelope and CLM-011.r02.
  - Q: CONTEXT#scope-detail and CLM-015.
- **DEL-14-03:**
  - C100: CLM-012/REQ-14-03-010.
  - SH: the nine CONTEXT canonical blocks and sub-claims.
  - F2/4: CLM-027, CLM-028, CLM-014, CONTEXT#scope-detail, and REQ-003
    and REQ-007.
  - F7: CLM-033, REQ-006, CLM-021 and REQ-008.
  - NA: CLM-006, CONTEXT, CLM-023, CLM-020, STATUS R01 and CLM-005.
  - AN: REQ-009.
  - Q: the praxeology wrapper, CONTEXT#anticipated-artifacts and CLM-025.
- **DEL-14-04:**
  - C100: CLM-017.s02 and CLM-008.r02.
  - SH: the nine CONTEXT canonical blocks and sub-claims.
  - F2/4: STATUS R01 and R02, AC-001, CONTEXT#scope-detail, R-14-04-003,
    CLM-026, CLM-006.r04, CLM-005 and CLM-037.
  - F7: CLM-027, CLM-012, CLM-033, CONTEXT#description, CLM-006.r02,
    R-14-04-006 and CLM-024.
  - NA: CLM-015.r04, CLM-020, CLM-005.s01, CLM-014, CLM-015.r03, purpose
    OUT-001, CLM-006.r07, CLM-004.r08 and CLM-023.
  - AN: R-14-04-009.
  - Q: CLM-003, CLM-035 and MEMORY.
- **DEL-14-05:**
  - C100: STATUS R01.
  - SH: the eight CONTEXT canonical blocks and sub-claims.
  - F2/4: STATUS R02 and CLM-029.
  - F7: CLM-011.r02, CLM-020.r01, CLM-027, CLM-020.r06, CLM-011.r09 and
    CLM-011.r07.
  - NA: CLM-011.r03, AC-001, CLM-026.s01, CLM-006.r05, CLM-006.r02,
    CLM-005.r02, CLM-006.r04, CLM-013.r04, CLM-013.r05, CLM-006.r06 and
    CLM-013.r09.
  - AN: CLM-018.r05 and CLM-018.r01.
  - Q: CLM-003, CLM-005 and the praxeology wrapper.

## 3. What was confirmed (selected)

These are the points I checked against the freeze that carry the package's
main findings. All are consistent with the rows.

- **JCS gap (DEL-14-01 FG-01).**
  - `core/project_persistence/service.py` L7 and L28 label serialization
    `SORTED_COMPACT_JSON` and state "not RFC 8785/JCS".
  - `schemas/model_state.schema.json` fixes
    `canonicalization: sorted_compact_json_payload_hashes`.
  - DEC-010, AB-00-04 and SPEC 4.4 still state a JCS-compatible basis.
  - `PARTIALLY_IMPLEMENTED · PROJECT_BASELINE` on CLM-005 and ABI .s03 is
    correct. Agreed.
- **No product model-state save path (DEL-14-01 CLM-011.r01).**
  - The desktop `create_local_project` (lib.rs L2256) takes model, intents,
    proposal, mechanics result and run.
  - No model-state record is created.
  - Round trips exist only in `tests/test_project_persistence_service.py`
    L491 to L521.
  - F7 and `PARTIAL_SLICE` are correct.
- **Protected-content review records (the six `UNKNOWN · INVARIANT`
  rows).** My A3 discovery found no protected-content or private-data
  review for model-state, analysis-run or state-comparison fixtures:
  - I ran `git grep` for each DEL-ID over root and project `execution/`.
  - I checked the deliverable `_REVIEW.md` files (PKG-02 compatibility
    audits only) and `Review_Findings.csv`.
  - Following the W1 DEL-07-02 CLM-026 resolution, `UNKNOWN ·
    EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA;RECORD · REVIEW` with a
    smallest check is correct.
  - DEL-14-01 R10 is `ALIGNED` on inspection, while DEL-14-03 REQ-010 is
    `UNKNOWN`. The difference is justified because REQ-010's own row
    carries the gate as its verification element.
- **Empty reference binding (DEL-14-02 FG-02).**
  - `core/analysis_runs/records.py` L132 and L133 and
    `analysisRunCompatibility.ts` L67 hard-code `rule_pack_refs: []` and
    `library_refs: []`.
  - The PROJECT_BASELINE rows (OUT-001, CLM-004, CLM-005, CLM-020 and
    CLM-011.r03) are correct.
  - The r09 CP-11 treatment is correct, with the tier following the gap
    under F8.
- **DEL-14-02 STATUS, CP-04 frozen-contract variant.**
  - `openpipestress_jcs_ijson_v1` is an active const in the 0.2 builder and
    schema.
  - `RECORD;BASELINE · FROZEN_CONTRACT · PROJECT_BASELINE · OWNER` matches
    CP-04.
- **Section-property oracle exclusion (DEL-14-04 PDU-047).**
  - `SUPPORTED_RESULT_FAMILIES` in `core/comparison/analysis_run/engine.py`
    L16 excludes `section_property`.
  - No validation basis was located.
  - `VERIFIED_NOT_VALIDATED · VALIDATION_GAP · INVARIANT · VALIDATION ·
    ENGINEERING` is correct (A5).
- **DAG-007 enum adoption (DEL-14-03 C-14-03-001 and AC-001).**
  - `_DAG/DAG-007/APPROVAL_RECORD.md` L32 to L86 adopts canonical v3.1
    enums; legacy values are kept for read compatibility only.
  - The 2026-06-16 refresh found the mirror already canonical.
  - Treating both rows as overtaken, at MEDIUM confidence because the
    approval does not name the conflict ID, is correct. I agree with the
    worker-flagged treatment.
- **RF-001 and RF-002 (DEL-14-03 STATUS R01).**
  - Both have an empty `HumanDisposition` at the freeze.
  - No governing row carries the disposition act, so `DOCUMENTED_UNIMPLEMENTED
    · REVIEW` under F2 is correct.
- **Unmatched-classification enum (DEL-14-05 FG-03, CP-10).**
  - SOW line 199 conditions validation "once enum values are human-approved
    or source-defined".
  - The schema fixes eight values and no approval record was located.
  - CP-10 fits (`IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR ·
    PROJECT_BASELINE · OWNER`).
- **Tests cited on sampled `ALIGNED` rows.** I spot-checked the following
  and confirmed each at the cited location:
  - the prohibited-claims scan (`test_model_state_comparison.py` L415);
  - the no-`default`-keyword assertion (`test_model_state_schema.py` L76);
  - `derive_exact_result_id_mappings` (engine L229);
  - the contract tests (`test_comparison_contracts.py` L135, L341 and
    L472);
  - the 11 result-delta tests;
  - the B4.4 gate count of 1,138 Python tests passed (`SUMMARY.json`).

## 4. Disagreements

### 4.1 Weak: CONTEXT-typed purpose rows kept `ALIGNED` while their Notes record an unmet element (F1 reading)

- **DEL-14-02:SOW#CLM-024** (F2/4 class).
  - *Row:* `CONTEXT · ALIGNED`, with `GAP_WORDING_CHECKED:` "the purpose
    names … references … as intent; the reference gap is a requirement
    defect on CLM-011.r03".
  - *Found:* The purpose text (SOW L333 to L338) says run records are
    "bound to the exact model state, execution context, diagnostics,
    references, and hashes". Both builders emit empty
    `rule_pack_refs`/`library_refs` (`core/analysis_runs/records.py` L132
    and L133; `apps/desktop/src/services/analysisRunCompatibility.ts` L67).
    OUT-001 in the same ledger is `PARTIALLY_IMPLEMENTED` for exactly this
    element.
  - *F1 reading:* "even when the same gap is also recorded on another row"
    points to `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
    NONE · BASELINE · FG-DEL-14-02-02`.
  - *Why weak, not firm:* The row is typed `CONTEXT`, a purpose statement,
    not a normative unit. The conventions do not say whether F1 reaches
    non-normative purpose text. If it does, this becomes firm on a
    100%-sampled class and changes the tier.
- **DEL-14-01:SOW#CLM-024** (F2/4 class). This is the same pattern.
  - The purpose names a "reproducible saved-state surface for design
    iteration, comparison, reporting, and handoff".
  - The Notes say product saving is carried on CLM-011.r01, which is
    `PARTIALLY_IMPLEMENTED` because no product save path exists.
  - Under the F1 reading, the right values are `PARTIALLY_IMPLEMENTED ·
    PARTIAL_SLICE · PROJECT_BASELINE · NONE · BASELINE · FG-DEL-14-01-02`.
- **Ask.** Agent 0 should settle once, corpus-wide, whether F1 applies to
  `CONTEXT`-typed purpose units.

### 4.2 Weak: the DEC-009 Python-core departure is judged three ways inside PKG-14 (known corpus cluster)

- **DEL-14-03:CONTEXT#architecture-basis-injection.s03** (SH class) and
  **DEL-14-03:SOW#CLM-006** (NA class).
  - *Rows:* `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
    BASELINE · AuthorityNeeded NO`. RemainingWork offers "or record an
    owner decision that the Python engine is the accepted service".
  - *Found:* `core/comparison/model_state/engine.py` is a Python module
    with no Rust or application-service counterpart. DEC-009
    (`SOFTWARE_DECOMP.md` L600) adopts Rust core/application services. No
    ruling permitting Python domain engines was located.
- **DEL-14-04:CONTEXT#architecture-basis-injection.s02** (SH class) and
  CLM-004.r07.
  - *Rows:* the same fact, judged `IMPLEMENTED_DIFFERENTLY ·
    AUTHORITY_UNCLEAR · PROJECT_BASELINE · RECORD;BASELINE · OWNER`.
  - *Note:* AUTHORITY_UNCLEAR is doubtful, because DEC-009 is not silent.
    Other packages used `POSSIBLE_DEFECT` for the same fact.
- **DEL-14-01:SOW#CLM-005** records the same fact in Notes only; the row's
  disposition is carried by the JCS gap.
- **Right values.** These are unresolved and belong to the open
  corpus-wide cluster: `WAVES/W3/RESOLUTIONS_DRAFT_*` records DEL-13-03 and
  DEL-13-04 as CONTESTED, "cause POSSIBLE_DEFECT vs AUTHORITY_UNCLEAR;
  Agent 0 settles corpus-wide", naming DEL-14-04.
- **My reading.** Under C6(d), implementation that departs from a claim is
  `IMPLEMENTED_DIFFERENTLY`. DEL-14-03's `PARTIAL_SLICE` / `NO` routing
  therefore understates the owner question. It stays weak because the
  class is contested and DEL-14-03's F7 framing (no service boundary, no
  caller) is also true.
- **Ask.** Fold DEL-14-03 ABI .s03 and CLM-006 into the same cluster, so
  that one ruling sets cause and routing for all of them.

### 4.3 Weak: DEL-14-05:SOW#CLM-026.s01 routes the unmatched-values part as a no-decision catch-up

- *Row:* `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE · LOCAL_DESIGN · NO`.
  RemainingWork: "mapping and unmatched enums … are now defined by the
  schema".
- *Found:* The sub-claim says the accessible *sources* do not define
  unmatched classification values (SOW L387). That remains true: the values
  exist only in `schemas/comparison_mapping.schema.json`, with no source or
  approval. The same ledger routes that exact fact to the owner as CP-10
  (FG-DEL-14-05-03 on CLM-006.r02, CLM-011.r03 and CLM-013.r03).
- *Right values:* The disposition can stand for the CSV-column and
  JSON-flag parts. The RemainingWork should exclude the unmatched values,
  or cite FG-DEL-14-05-03, so that R3 does not repair by text edit what the
  owner has yet to confirm.
- *Why weak:* A split between two causes on one sub-claim is not something
  the conventions settle. The owner routing is already carried by the
  FG-03 rows.

### 4.4 Field observation (beyond the sample): DEL-14-05:SOW

- The Notes record the former product name in the deliverable's own schema
  `$id` (`https://openpipestress.org/schemas/comparison_{mapping,tolerance}.schema.json`)
  and in both titles, as active identifiers. The RemainingWork names only
  CLM-012.
- DEL-14-01:SOW handles the same kind of residue (the model-state schema
  `$id` and title) inside its RemainingWork.
- For R3 clustering, both should be treated alike. Under CP-04 these are
  active code identifiers the deliverable names; they are residue.

### 4.5 Observations, not disagreements

- **DEL-14-02:SOW#CLM-011.r02** (AN).
  - The row is `ALIGNED` while `model_state_ref` is synthesised as
    `state:<model_ref>:preview`.
  - I accept it: the requirement's own verification column admits "or
    equivalent validated binding".
  - The product binds the exact input manifest by SHA-256 and rejects a
    model mismatch (`analysisRunCompatibility.ts` L40), and records
    `model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest`
    (L69).
  - R3 may want to link it to DEL-14-01 FG-02.
- **DEL-14-05:SOW#CLM-011.r07** (F7).
  - The row is `ALIGNED` with an overtaken "exact columns TBD" rider noted.
  - DEL-14-04 disposes some overtaken status riders as
    `STALE_SETUP_SPECIFICATION`.
  - This follows the worker's disclosed rule (notes item 6: requirement
    rows whose substance holds stay aligned). C6(b) supports it.

## 5. Batch consistency and shared-situation conflicts

- **Mechanical batch result.** PASS, 0 findings. My run matches the
  manager's `BATCH_PKG-14.txt`.
- **Minted sub-claims the batch cannot see.** Two conflicts sit on minted
  `.sNN` keys, which have no `BodySHA256` in `CLAIM_KEYS_V2.csv`:
  1. **SR-1, the SEMANTIC_READY readiness sub-claim** (`CONTEXT#architecture-basis-injection.s01`).
     - The body is the same in all five deliverables.
     - DEL-14-01, 14-02 and 14-03 (G1) use `RECORD_DRIFT`.
     - DEL-14-04 and 14-05 (G2) use `SCOPE_REDIRECTED_BY_RULING`.
     - Disposition, tier and layers agree.
     - This is the corpus-wide contested cluster SR-1 (W2 `RESOLUTIONS.csv`).
       I record it here without counting it against either worker.
  2. **The DEC-009 Python-core finding** (§4.2): three treatments across
     DEL-14-01, 14-03 and 14-04.
- **Same-titled blocks.** I compared these across the five ledgers and
  found no further conflicts:
  - CS-06 rows;
  - the ABI "Still TBD" sub-claims (all `STALE_SETUP_SPECIFICATION ·
    SCOPE_REDIRECTED_BY_RULING`);
  - the D-41 CP-03 declarations (all CP-02 fields);
  - OUT-001 matrix rows (all CP-09 `EVIDENCE_OVERTAKEN`);
  - MEMORY history rows (all `ALIGNED`).

## 6. Reverse-pass findings

- **Coverage.** Each deliverable answered all 322 routed capabilities: 297
  AREA and 25 SAMPLE, resolved through `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`.
- **CLAIMED_BY, PARTIAL, UNKEYED and CONSTRAINS (100% checked).** All are
  correct. There are no PARTIAL or CONSTRAINS answers.

  | Deliverable | Answer | Capabilities |
  |---|---|---|
  | DEL-14-01 | CLAIMED_BY | RC-14-0289 (model-state schema) |
  | DEL-14-02 | CLAIMED_BY | RC-14-0059, 0127 (SAMPLE), 0203, 0241 and 0256 |
  | DEL-14-02 | UNKEYED | RC-14-0146, legacy checksum verification (`core/analysis_runs/legacy.py`). No issued key covers verifying 0.1 preimages under D-67; the nearest key is CLM-011.r07. This is reasonable |
  | DEL-14-03 | CLAIMED_BY | RC-14-0126 |
  | DEL-14-04 | CLAIMED_BY | RC-14-0078 and 0150 |
  | DEL-14-05 | CLAIMED_BY | RC-14-0186 and 0265 |

  The COVERS answers are coherent and none of them duplicates an owner:
  - DEL-14-02: RC-14-0022 and 0077;
  - DEL-14-04: RC-14-0186, 0256 and 0265;
  - DEL-14-05: RC-14-0078 and 0150.
- **Capabilities claimed by more than one PKG-14 deliverable.** None.
- **F5 (NOT_MINE where EntryPoints hit a forward-cited path).** I checked
  all 34 such rows, which is more than the 20% minimum. The counts per
  deliverable are 13, 9, 3, 2 and 7. Every reason addresses the overlapping
  path's use and the capability.
  - G1's lib.rs reasons are composed from a template: "cites lib.rs only
    for create_local_project … The capability '<quoted>' is not … work".
    That is minimal but acceptable, because the true reason is
    path-specific.
  - The DEL-14-02 copies carry the residue "(and, for DEL-14-01, no
    model-state records)". This is cosmetic.
  - Rows that overlap only through the `Tests` column (for example
    DEL-14-04 RC-14-0041 and 0289, which use a template reason) are outside
    F5 as written, since F5 names `EntryPoints`.
- **NOT_MINE 10% sample.** I judged 33, 32, 33, 32 and 32 rows per
  deliverable, for example:
  - DEL-14-01: RC-14-0127, 0241 and 0077;
  - DEL-14-02: RC-14-0078 and 0122;
  - DEL-14-04: RC-14-0241;
  - DEL-14-05: RC-14-0146 and 0063.

  I found 0 disagreements.
- **Suspected missed claims.** None firm.
  - RC-14-0093, the desktop comparison workspace (`ComparisonPanel.tsx`),
    is NOT_MINE for all five. That is consistent with its homing in the
    Phase G Remaining item to the comparison GUI owner (DEL-07-08).
  - RC-14-0132, 0179 and 0210 (state-comparison report sections and parity
    fixture) go to DEL-08-06.
  - R3 owns the final unmapped set.
- **Anchored answers.**
  - SAMPLE rows: 1 of 125 answers is non-NOT_MINE. It is DEL-14-02
    CLAIMED_BY RC-14-0127, the desktop 0.2 run-record builder. The
    claim is correct on the evidence, and the path is one the forward
    ledger cites.
  - AREA rows: 11 of 1,485 answers are CLAIMED_BY or UNKEYED, and 7 are
    COVERS.
  - For DEL-14-02, the SAMPLE claim rate is 1/25 (4%) against an AREA rate
    of 5/297 (1.7%, including the UNKEYED answer).
  - At these counts the difference is not a signal. About half the SAMPLE
    rows are recognisable from their paths, which is inherent.
  - Every SAMPLE row whose EntryPoints name a path the package declares was
    checked: RC-14-0034, 0127 and 0199. All are correct.

## 7. For the owner, plainly

1. **No rerun is needed.** Firm false alignment is 0% (0 of 43), and there
   are no firm errors.
2. **Protected-content reviews.** Six IP_DATA `INVARIANT` rows are `UNKNOWN`,
   because no protected-content or private-data review record exists for
   the model-state, analysis-run and state-comparison fixtures. Inspection
   found only invented data, but the review the Scopes of Work ask for was
   not located. Routing: REVIEW.
3. **Model-state hashes are not JCS.** They use sorted-compact JSON, while
   DEC-010 and SPEC still require a JCS-compatible basis. There is also no
   product path that saves a named model state. Both are PROJECT_BASELINE
   findings in DEL-14-01.
4. **Empty reference lists.** The analysis-run record builders always write
   empty rule-pack and library reference lists, although SOW-072 names them
   as binding categories (DEL-14-02, PROJECT_BASELINE).
5. **Owner decisions pending:**
   - **DEC-009.** Whether Python engines under `core/` satisfy the Rust-core
     baseline. The rows are split three ways in this package; see the
     corpus cluster.
   - **Unmatched-classification values.** DEL-14-05's eight values were
     fixed in the schema with no approval record (CP-10).
   - **Frozen-contract rename.** The `openpipestress_jcs_ijson_v1` rename
     is an R4 candidate.
6. **Engineering validation is held.** DEL-14-04 PDU-047 and DEL-14-05
   tolerance suitability have no validation basis. Unit tests are not
   validation (A5).
7. **Two convention questions for Agent 0:**
   - whether F1 reaches `CONTEXT`-typed purpose units (§4.1);
   - how minted `.sNN` rows can be brought under batch consistency, so
     that clusters such as SR-1 and DEC-009 are caught mechanically (§5).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
