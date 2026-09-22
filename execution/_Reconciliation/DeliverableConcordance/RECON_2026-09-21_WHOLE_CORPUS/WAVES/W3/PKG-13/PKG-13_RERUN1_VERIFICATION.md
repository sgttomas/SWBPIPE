VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-13 rerun cycle 1 verification: DEL-13-02 only

- **Run:** HELP-HUMAN-PIPING-20260921-RECONCILIATION. Wave W3. Sampling STANDARD.
- **Verifier:** a fresh, evidence-only TASK. Brief `briefs/R2-VERIFIER_brief.md`, SHA-256
  `47fb3c52…5b00`, checked before starting. Parent: HELP_HUMAN Agent 0.
- **Independence:** I am independent of both PKG-13 managers, their workers and the first PKG-13
  verifier. I did not read `PKG-13_VERIFICATION.md` or anything in `DEL-13-02/superseded_1/`.
- **Scope:** only the fresh DEL-13-02 ledgers.
  - `DEL-13-02_forward.csv`: SHA-256 `637db6f4…ab02`. This equals `DEL-13-02_SEAL.txt`, sealed
    2026-09-22T02:48:42Z.
  - `DEL-13-02_reverse.csv`: `88168f67…484f`.
  - `DEL-13-02_notes.md`: `20b722e9…dd50`.
- **Evidence basis:** the read-only checkout of `00115c71931b` at `{FREEZE}` (HEAD verified).

## 1. Mechanical checks

- **Single mode.** The run was `validate_ledger_v2.py --deliverable DEL-13-02 --forward … --reverse …
  --inventory ROUTING/PKG-13_capabilities.csv --notes-gap`. Result: **PASS**, 114 forward rows,
  73 required keys, 8 canonical, 0 findings.
- **Package batch.** The run was `--batch` over DEL-13-01, fresh DEL-13-02, DEL-13-03 and DEL-13-04.
  Result: **PASS**, 0 consistency findings. No wave `RESOLUTIONS.csv` exists yet; only drafts do.
- **Sentinels.**
  - Forward: `#END` with 114 body rows.
  - Reverse: `#END` with count 323, which equals the 323 routed capabilities.
- **Canonical conformance (CS-01 to CS-07).** Eight keyed rows match `CANONICAL_ASSIGNMENTS.csv` on
  every field.

## 2. Sample selection (deterministic)

- **Method:** candidate keys are sorted by SHA-256 of the full `ClaimKey`, and the lowest
  ceil(rate × n) are taken.
- **Class assignment:** each row goes to the first class it fits, in the order below.
- **Normative rows:** the `ALIGNED` normative 20% was drawn over all 32 `ALIGNED` normative rows,
  whatever their `PRODUCT_CALLER` tag, and weighted as follows:
  - all five `MEDIUM` rows;
  - the single `NONE`-verification row;
  - then the lowest hash.

| Class | Rows | Rate | Sampled | Keys (prefix `DEL-13-02:`) |
|---|---|---|---|---|
| `INVARIANT` tier (100%) | 8 | 100% | 8 | SOW#CLM-003.r13, CLM-009, CLM-010/R-13-02-005, CLM-012/R-13-02-005, CLM-013, CLM-018, CLM-019.r04, CLM-024 |
| `SharedTextCount > 1` | 6 | 100% | 6 | CONTEXT#package-reference, #decomposition-reference, #architecture-basis-injection, #architecture-basis-injection.s01, #sca-002-control-surface-refresh-note, #preparation-notes |
| `ALIGNED` with `GAP_WORDING_CHECKED:`/`OPEN_ACTION:` | 3 | 100% | 3 | SOW#CLM-003.r15, CLM-012/R-13-02-004, CONTEXT#scope-detail |
| `ALIGNED` with `PRODUCT_CALLER: NONE` | 50 | 25% | 13 | SOW#CLM-026, CLM-003.r11, CONTEXT#anticipated-artifacts, SOW#CLM-003.r01, CLM-010/R-13-02-003, CLM-011, CLM-019.r08, CLM-010/R-13-02-008, CLM-010/R-13-02-002, CLM-012/R-13-02-006, CLM-010/R-13-02-007, CLM-023, CLM-004.r01 |
| Other non-aligned | 17 | 25% | 5 | SOW#CLM-027, CLM-017.r04, CLM-017.r03, CLM-006, CLM-019.r07 |
| `ALIGNED` normative | 32 | 20% | 7 (3 not already sampled) | CLM-016 (NONE), CLM-010/R-13-02-009 (MEDIUM), CLM-012/R-13-02-009 (MEDIUM); overlaps: R-13-02-002, CLM-012/R-13-02-004, CLM-019.r08, R-13-02-003 |
| Structural and inherited canonical | 29 | 10% | 3 | SOW#purpose-and-objective-traceability, STATUS#remaining, SOW#CLM-002 |
| **Total unique** | 114 | | **41** | |

Rows not in the sample:
- **ISSUED, ACCEPTED_DIVERGENCE, AUTHORITY_CONFLICT, UNKNOWN and LIFECYCLE rows:** none. DEL-13-02 is
  IN_PROGRESS.
- **PROTECTED_CHECK and FROZEN_CONTRACT rows:** none.

## 3. Table: DEL-13-02

| Rows | Sampled per class | Firm | Weak | Field | Firm false-alignment (sampled ALIGNED normative) |
|---|---|---|---|---|---|
| 114 | INV 8 · SHARED 6 · GAP 3 · PC 13 · NONAL 5 · NORM +3 (7 in class) · STRUCT 3 = 41 | 0 | 3 | 0 | 0 / 13 = **0.0%** |

The 13 sampled `ALIGNED` normative rows:
- CLM-003.r15, CLM-012/R-13-02-004, R-13-02-003, CLM-019.r08, R-13-02-008, R-13-02-002;
- CLM-012/R-13-02-006, R-13-02-007, CLM-023, CLM-004.r01, CLM-016, R-13-02-009,
  CLM-012/R-13-02-009.

**Firm error rate on sampled rows:** 0 / 41. There is no firm error in a 100%-sampled class, so no
rerun is triggered.

## 4. Package-level firm false-alignment rate

The rerun covers DEL-13-02 only, so the package-level rate is **0.0%** (0 of 13). This is the
scale-out gate input, and the gate is ≤ 5%. It meets the gate.

## 5. Disagreements

There are no firm or field disagreements. The three weak rows below fall in classes that are already
open elsewhere in the corpus.

**W-1: `SOW#CLM-003.r15` and `SOW#CLM-023`. Removed-source pointer on `ALIGNED` rows.**
- **What the rows say:** both are `ALIGNED`. Each row's own Notes record that its `INIT.md` source
  pointer is stale (the file was removed 2026-07-04, `9c4caf8fd`). Both leave the pointer to FG-03
  on CLM-006.
- **What I found:** the substance holds in `schemas/constraint.schema.json`.
  - CLM-003.r15: the schema holds structure and policy constants only, and
    `tests/test_constraint_schema.py:251` scans for forbidden text.
  - CLM-023: every principle holds.
- **The conflict:** the same ledger disposes `CLM-017.r03` as non-aligned for the same removed
  `INIT.md` pointer. Two readings are possible:
  - Strict F1/F8: the source cell is part of the claim, so the pointer is an unmet element. F8's
    "only a record clause … is stale while the boundary holds" example assumes such a row carries a
    tier, which means it is non-aligned. The row would then be `STALE_REVIEW_OR_EVIDENCE ·
    BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO` (CP-02).
  - The worker's reading: the source is traceability, not claim (notes item 5).
- **Class:** the conventions do not settle this. It is the same open class as the draft W3
  resolutions for DEL-11-01 (CLM-005.r02 vs UG-REQ-005) and DEL-11-05 CLM-025.
- **Also:** CLM-003.r15's `GAP_WORDING_CHECKED` clause explains wording that the current scan no
  longer flags. It does not address the stale pointer. This is harmless.
- **Unsampled rows with the same pattern:** CLM-010/R-13-02-006 and CLM-008. They are reported,
  not counted.

**W-2: `SOW#CLM-017.r03`. Stale-class origin for an `INIT.md` file pointer.**
- **What the row says:** `STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE` (CP-02).
- **What I found:** `git log -S "Project invariants for protected data, missing-data findings"`
  traces the text to `7bee9ae41` (initial migration, in `Procedure.md`).
- **The conflict:** the pointer is a removed-file pointer, not a revision pin or review state. So a
  literal F3 reading gives `STALE_SETUP_SPECIFICATION` with the other fields unchanged. CP-02 gives
  `STALE_REVIEW_OR_EVIDENCE`.
- **Class:** this is the open class C2 already before Agent 0 (draft W3 resolutions for DEL-06-03
  CLM-007.r01 and DEL-06-02 CLM-008.r01: "whether CP pattern rows are within F3's origin test").
  It changes neither the tier nor the routing.

### Rows I checked and confirmed (evidence, summarised)

**FG-01, 8 `INVARIANT` rows, all 100%.** All three of the defect's facts hold at the freeze.
- `Parameter.value` in `schemas/constraint.schema.json` is a `oneOf` of string, boolean,
  `Quantity` or reference list, independent of `value_kind`. A `value_kind: quantity` parameter
  therefore validates with a bare string.
- `Quantity.dimension` omits `force_per_length`. That value is in the accepted vocabulary at
  `docs/SPEC.md` §4 (lines 171–178) and `schemas/units.schema.yaml:259`.
- `tests/test_constraint_schema.py` asserts `dimensions == ACCEPTED_PKG02_DIMENSIONS`, its own local
  set without `force_per_length`. So it passes while the schema diverges.

How the individual rows hold up:
- **Schema history:** the schema has not changed since `7bee9ae41`.
- **`Review_Findings.csv` `PKG13-DEL-13-02-PKG02-001`:** still `HumanDisposition=TBD`.
- **Non-aligned result:** each row's claim either asserts alignment or unit-safety (r13, CLM-009,
  CLM-012/R-005, CLM-024 bullet 3) or requires it (R-13-02-005, CLM-018 step 7, CLM-019.r04). So
  none can be `ALIGNED` (F1).
- **CLM-013:** its unmet element is a current unit-compatibility verification note. That note
  cannot be written truthfully until the schema is fixed, so grouping it under `POSSIBLE_DEFECT`
  is defensible.
- **Tier:** `INVARIANT` fits C3's first clause (the requirement restates CONTRACT `OPS-K-UNIT-1`
  and SPEC §4), and F8 applies because the gap touches unit safety itself.
- **Disposition:** I would accept `PARTIALLY_IMPLEMENTED`. Section 6 sets out the consistency
  question across the corpus.

**Shared-body rows, 6.**
- CS-01, CS-02, CS-03 and CS-04 conform, and CS-06 package-reference matches SOFTWARE_DECOMP
  rev 0.12 line 229.
- `.s01` (the PKG-00 `SEMANTIC_READY` statement): all eight PKG-00 `_STATUS.md` files are
  IN_PROGRESS at the freeze. F3 names this very example as a review state. The `.s01` fields are
  identical across DEL-13-01, 13-02, 13-03 and 13-04.

**Remaining samples.** All hold. Specific checks:
- **CLM-019.r08 / CLM-011:** the `json.tool` pass is recorded in `_run_records/TASK_RUN_2026-06-07_1127.md`
  over unchanged schema bytes, and marked not rerun. The test and schema are identical between the
  GATE candidate `9d55bce` and the freeze.
- **R-13-02-009:** `model.schema.yaml` carries `constraint_refs` (about line 1153) and
  `TraceabilityLink` (about line 1930).
- **CLM-026:** there is no constraint payload under `fixtures/` or `examples/`.
- **CLM-017.r04 and CLM-019.r07:** `_DEPENDENCIES.md:5` names DAG-007, and `_DAG/_LATEST.md` gives
  DAG-010.
- **CLM-006:** it pins rev 0.7, while the frozen decomposition is 0.12.
- **CLM-027:** the DEL-13-03 engine, the transform `constraint_refs` and the GUI panel exist, so
  "remain TBD" is behind the code. Its cause is `DOC_BEHIND_CODE`, and the text was first declared
  at `bc5759133`, which is not setup text.
- **CLM-002:** matches `Deliverables.csv` and `ContextBudgetQA.csv`.
- **CLM-016:** a procedural purpose, and the artifacts respect the boundary.

**Out-of-sample spot check (FG-04, CP-09).**
- `EVIDENCE_MAP` says `NONE_FOUND`. The only PASS parity record for TASK-PIP-13-02
  (`…/TASK-PIP-13-02/stage1_evidence/PARITY_REPORT.json`) binds SOW `43d9ea2f…`.
- The frozen SOW is `78a2ff6e…`, after `34fee9bdf` and `8fac6631a`.
- So `EVIDENCE_OVERTAKEN` is supported. The worker's A3 discovery corrects the evidence map for
  this deliverable.

**F7.** `constraint.schema.json` is loaded only by its test. `core/constraints/validation/engine.py`
names the schema in message strings but does not load it. Every sampled `ALIGNED` row makes a claim
about the schema, the data contract, so `ALIGNED` with `PRODUCT_CALLER: NONE` is correct under F7.

## 6. Batch consistency and shared-situation conflicts

- **Package batch:** PASS, 0 findings. The `.s01` rows are consistent across PKG-13.
- **Shared-situation conflict outside the batch's reach:** the unit-defect cluster is inconsistent
  across the corpus. The rows have different body hashes and no `CanonicalSituation`, so the batch
  cannot flag them.
  - DEL-13-02 FG-01 (8 rows): `PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT · INVARIANT · BASELINE ·
    REVIEW`.
  - DEL-13-01 `SOW#CLM-009/REQ-13-01-006`: `PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT ·
    PROJECT_BASELINE · BASELINE · REVIEW`.
  - DEL-03-07: `INVARIANT · … · OWNER`.
  - DEL-15-03: `IMPLEMENTED_DIFFERENTLY`, with `INVARIANT` and `PROJECT_BASELINE` mixed.
- **My view:** each value is defensible alone. C3 does not say whether unit safety (`OPS-K-UNIT-1`)
  is among INVARIANT's "covered subjects", and C6 does not choose between `PARTIALLY_IMPLEMENTED`
  and `IMPLEMENTED_DIFFERENTLY` for a false vocabulary declaration. I do not count this as an error
  against DEL-13-02. Agent 0 should settle the tier, the disposition and `AuthorityNeeded` for this
  cluster once, in RESOLUTIONS, so R3 clusters it uniformly. The worker proposed the same
  (together with DEL-13-01 CLM-005.r05).

## 7. Reverse pass

- **Answers:** 1 `CLAIMED_BY` and 322 `NOT_MINE`. There are no `PARTIAL`, `UNKEYED` or `CONSTRAINS`
  answers.
- **`CLAIMED_BY`, 100%:** RC-13-0308 (`CAP-DATA-007`, `schemas/constraint.schema.json`) is correct.
  It is the deliverable's anticipated artifact, and its keys are CLM-005, CONTEXT#anticipated-artifacts,
  OUT-001 and R-13-02-001. None is a `STATUS#remaining` key.
- **F5, 100% of overlaps:** 11 capabilities have `EntryPoints` that hit a path the forward ledger
  cites. One is RC-13-0308 (claimed). The other 10 are `NOT_MINE`, and each has a
  capability-specific reason:
  - RC-13-0046 units schema;
  - RC-13-0122 transform contract;
  - RC-13-0148 DEL-13-03 engine;
  - RC-13-0226 registers;
  - RC-13-0245 GUI workspace;
  - RC-13-0262 CONTRACT;
  - RC-13-0282 model schema;
  - RC-13-0286 IP boundary;
  - RC-13-0287 design-knowledge schema;
  - RC-13-0298 SPEC.

  All 10 reasons are correct.
- **`NOT_MINE` sample (10% = 33, SHA-256 order):**
  - The capabilities: RC-13-0002, 0207, 0263, 0307, 0072, 0062, 0100, 0076, 0005, 0115, 0063, 0265,
    0162, 0199, 0081, 0015, 0178, 0092, 0284, 0088, 0313, 0055, 0012, 0082, 0175, 0230, 0141,
    0058, 0299, 0078, 0060, 0258, 0166.
  - All are correct. None touches the constraint schema or its test. RC-13-0058's
    `fixtures/product_preview/` holds no constraint payload.
- **Capabilities claimed by more than one package deliverable:** I found none from DEL-13-02's
  side. Only RC-13-0308 is claimed here.
- **Suspected missed claims:** none. A search of all 323 capabilities for "constraint" and
  "provenance … schema" finds only RC-13-0308 as a constraint-entity surface. RC-13-0148 (engine)
  and RC-13-0245 (GUI) are rightly left to DEL-13-03 and the GUI area.
- **Routing sample against area, resolved through `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`:**
  - SAMPLE rows: 30 of 30 `NOT_MINE`.
  - AREA rows: 292 `NOT_MINE` and 1 `CLAIMED_BY`, out of 293.
  - None of the SAMPLE rows' evidence names a path this deliverable declares. The distributions
    agree.
  - This comparison is weak evidence of non-anchoring for a deliverable with one owned file.
    About half the sampled rows are recognisable from their paths.
- **Anchored answers:**
  - The worker read the first verifier's report and the routing file **after sealing the forward
    ledger** but **before the reverse pass** (notes, "After sealing"). The forward ledger is
    unaffected.
  - In the reverse pass, 312 answers use a surface-scoped template, and the 11 non-template answers
    are all path-driven. I see no sign that the prior report shaped any reverse answer.

## 8. For the owner

1. **Possible defect in the constraint schema (FG-01).** Two unit defects hold at the freeze.
   `tests/test_constraint_schema.py` passes only because it compares the schema with its own
   outdated copy of the vocabulary. The owning loop's fix is a schema and test change, followed by
   the still-open human disposition of `PKG13-DEL-13-02-PKG02-001`.
   - A constraint parameter declared as a quantity passes with a bare string and no unit.
   - The schema's dimension list lacks `force_per_length`, which the accepted PKG-02 unit
     vocabulary has contained since 2026-05-17.
2. **Uniform ruling needed.** The same kind of unit-vocabulary defect carries different tiers
   (`INVARIANT` or `PROJECT_BASELINE`), different dispositions and different `AuthorityNeeded`
   values across DEL-13-01, DEL-13-02, DEL-03-07 and DEL-15-03. The conventions do not settle it.
3. **Open convention class (W-1, W-2).** Rows citing the removed `INIT.md` are handled
   inconsistently. Some are `ALIGNED` with a note and some are non-aligned. The stale class
   (`STALE_REVIEW_OR_EVIDENCE` or `STALE_SETUP_SPECIFICATION`) also varies. The same class is
   already open from PKG-06 and PKG-11.
4. **Disclosure carried from the worker.** It ran one read-only `json.load` parse of the frozen
   schema, which wrote nothing and is not cited as evidence. The fence forbids builds and test runs.
   I regard this parse as neither, but record it.

These dispositions are agent judgments, not owner rulings. Nothing here states or implies release,
approval, compliance or certification. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).
