VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-16 verification — wave W1 (DOUBLE sampling)

Fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION` (parent: HELP_HUMAN Agent 0).
Brief: `R2-VERIFIER_brief.md`, SHA-256
`db8884ab100e28f0be5c7aa7217e9214fb10362a70b162c17896dea5a224303e` (matched).
Evidence read from the frozen checkout at
`00115c71931bcae79909602d653740d3bb72dfa1`. No build, test run, git write or
ledger edit was made. The manager's validator transcripts were not used as
evidence. Every judgment here is an agent judgment, not an owner ruling.

## 0. Inputs and seals

| Ledger | Rows | Seal SHA-256 (matches file) |
|---|---|---|
| DEL-16-01 forward | 72 | `fa302f3b0b747c119ad421e57a19591d673e7573157a6ea1dcec6a62538b9257` |
| DEL-16-02 forward | 68 | `f2b271d93bf8d7929a31c0a363a636d72c02d9a1f3fb58ef0a39ea2bb51b4913` |
| DEL-16-03 forward | 73 | `5c37b376589d54232e51bb9723cb441d7ce5bb6474130b5f152c0836aa73ea3c` |
| DEL-16-04 forward | 71 | `ed05c5bbbb46bc3e7454d3a6ec273b270cbe2a2f1cb729fbf1808534283db5d9` |

Each reverse file answers all 353 routed capabilities. My own run of
`validate_ledger_v2.py` in single-deliverable mode (forward, reverse and
inventory) returned PASS with 0 findings for all four deliverables.

## 1. Sample selection

Sampling follows `R0_REVIEW.md` §7, with the starred rates doubled. Each row
goes into exactly one class, taking the first that applies:

- **A100**: `INVARIANT` tier, `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`,
  `UNKNOWN`, `LIFECYCLE_REASSESSMENT_REQUIRED`, `PROTECTED_CHECK`,
  `FROZEN_CONTRACT` or ISSUED. Sampled at 100%. **The package has no row in
  this class** (see §7).
- **S100**: the unit's `SharedTextCount` in `CLAIM_KEYS_V2.csv` is above 1.
  A `.sNN` key takes its parent's count. Sampled at 100%.
- **NA50**: other non-aligned rows. Sampled at 50%.
- **AL40**: `ALIGNED` rows of type `REQUIREMENT`, `ACCEPTANCE` or
  `EXCLUSION`. Sampled at 40%. The weighted rows come first: `LOW` or
  `MEDIUM` confidence, `NONE_FOUND` verification, or verification class
  `NONE`.
- **ST20**: structural, inherited-canonical and all other quiet rows. Sampled
  at 20%. This includes quiet `DECLARED_STATE`, `HISTORY` and
  `REMAINING_WORK` rows, which no named class covers.

How the rows were picked:

- Within each class, candidates are ordered by the SHA-256 of `ClaimKey`,
  and the lowest ceil(rate × n) are taken.
- In AL40, the weighted rows are ordered first, then the rest.
- Classes are drawn package-wide, not per deliverable.

| Class | Package rows | Sampled |
|---|---|---|
| A100 | 0 | 0 |
| S100 | 35 | 35 |
| NA50 | 78 | 39 |
| AL40 | 40 | 16 |
| ST20 | 131 | 27 |
| **Total** | 284 | 117 |

Sampled keys (prefix each with the deliverable ID and a colon):

- DEL-16-01 S100 (10): `CONTEXT#architecture-basis-injection`, `CONTEXT#architecture-basis-injection.s01`, `CONTEXT#architecture-basis-injection.s02`, `CONTEXT#decomposition-reference`, `CONTEXT#objective-support`, `CONTEXT#package-reference`, `CONTEXT#preparation-notes`, `CONTEXT#sca-002-control-surface-refresh-note`, `CONTEXT#scope-coverage`, `CONTEXT#scope-detail`
- DEL-16-01 NA50 (10): `SOW`, `SOW.s01`, `SOW#CLM-003`, `SOW#CLM-006`, `SOW#CLM-009.r01`, `SOW#CLM-012`, `SOW#CLM-016`, `SOW#CLM-023`, `SOW#CLM-026/CT-003`, `SOW#output-and-evaluation-matrix/OUT-001`
- DEL-16-01 AL40 (6): `CONTEXT#description`, `SOW#CLM-008`, `SOW#CLM-009.r05`, `SOW#CLM-009.r07`, `SOW#completion-and-reliance-basis-epistemology/AC-001`, `SOW#purpose-and-objective-traceability/OUT-001`
- DEL-16-01 ST20 (6): `CONTEXT#context-budget-qa`, `MEMORY`, `SOW#CLM-009`, `SOW#CLM-011`, `SOW#CLM-025`, `SOW#completion-and-reliance-basis-epistemology`
- DEL-16-02 S100 (10): the same ten `CONTEXT#…` keys as DEL-16-01
- DEL-16-02 NA50 (9): `CONTEXT#description`, `SOW`, `SOW#CLM-005`, `SOW#CLM-010/REQ-16-02-003`, `SOW#CLM-010/REQ-16-02-005`, `SOW#CLM-012`, `SOW#CLM-022`, `SOW#CLM-026`, `STATUS`
- DEL-16-02 AL40 (2): `SOW#CLM-010/REQ-16-02-008`, `SOW#purpose-and-objective-traceability/OUT-001`
- DEL-16-02 ST20 (7): `CONTEXT#context-envelope`, `SOW#CLM-001`, `SOW#CLM-003`, `SOW#CLM-014`, `SOW#CLM-015`, `SOW#CLM-025`, `SOW#production-and-verification-method-praxeology`
- DEL-16-03 S100 (8): `CONTEXT#architecture-basis-injection`, `.s01`, `.s02`, `CONTEXT#decomposition-reference`, `CONTEXT#objective-support`, `CONTEXT#package-reference`, `CONTEXT#preparation-notes`, `CONTEXT#sca-002-control-surface-refresh-note`
- DEL-16-03 NA50 (12): `SOW`, `SOW#CLM-002`, `SOW#CLM-005`, `SOW#CLM-009`, `SOW#CLM-016`, `SOW#CLM-020`, `SOW#CLM-022`, `SOW#CLM-023`, `SOW#CLM-026`, `SOW#completion-and-reliance-basis-epistemology/AC-001`, `SOW#output-and-evaluation-matrix/OUT-001`, `STATUS#remaining/R01`
- DEL-16-03 AL40 (4): `CONTEXT#description`, `SOW#CLM-010/DEL-16-03-REQ-001`, `SOW#CLM-010/DEL-16-03-REQ-008`, `SOW#purpose-and-objective-traceability/OUT-001`
- DEL-16-03 ST20 (7): `CONTEXT`, `CONTEXT#context-envelope`, `MEMORY`, `SOW#CLM-014`, `SOW#purpose-and-objective-traceability`, `STATUS#history`, `STATUS#remaining`
- DEL-16-04 S100 (7): `CONTEXT#architecture-basis-injection`, `.s01`, `.s02`, `CONTEXT#decomposition-reference`, `CONTEXT#package-reference`, `CONTEXT#preparation-notes`, `CONTEXT#sca-002-control-surface-refresh-note`
- DEL-16-04 NA50 (8): `SOW`, `SOW#CLM-009/REQ-16-04-02`, `SOW#CLM-009/REQ-16-04-08`, `SOW#CLM-014`, `SOW#CLM-016`, `SOW#CLM-019`, `SOW#completion-and-reliance-basis-epistemology/AC-001`, `STATUS`
- DEL-16-04 AL40 (4): `CONTEXT#description`, `SOW#CLM-008`, `SOW#CLM-009/REQ-16-04-01`, `SOW#CLM-009/REQ-16-04-09`
- DEL-16-04 ST20 (7): `CONTEXT#anticipated-artifacts`, `CONTEXT#context-envelope`, `CONTEXT#scope-coverage`, `SOW#CLM-018`, `SOW#CLM-022`, `SOW#CLM-024`, `STATUS#remaining/R01`

## 2. Per-deliverable results

### DEL-16-01 — Structured model operation schema

| Rows | Sampled per class | Firm | Weak | Field | Firm false alignment (sampled ALIGNED normative) |
|---|---|---|---|---|---|
| 72 | A100 0 · S100 10 · NA50 10 of 19 · AL40 6 of 12 · ST20 6 of 31 (32 in total) | 0 | 0 | 1 | 0 of 6 = 0% |

### DEL-16-02 — Operation validation and diff preview

| Rows | Sampled per class | Firm | Weak | Field | Firm false alignment |
|---|---|---|---|---|---|
| 68 | A100 0 · S100 10 · NA50 9 of 21 · AL40 2 of 6 · ST20 7 of 31 (28 in total) | 0 | 0 | 0 | 0 of 2 = 0% |

### DEL-16-03 — User acceptance and operation audit trail

| Rows | Sampled per class | Firm | Weak | Field | Firm false alignment |
|---|---|---|---|---|---|
| 73 | A100 0 · S100 8 · NA50 12 of 22 · AL40 4 of 12 · ST20 7 of 31 (31 in total) | 0 | 1 | 0 | 0 of 4 = 0% |

### DEL-16-04 — Agent rationale and professional-boundary controls

| Rows | Sampled per class | Firm | Weak | Field | Firm false alignment |
|---|---|---|---|---|---|
| 71 | A100 0 · S100 7 · NA50 8 of 16 · AL40 4 of 10 · ST20 7 of 38 (26 in total) | 1 | 2 | 0 | 1 of 4 = 25% |

Rerun test:

- No deliverable's firm error rate on its sampled rows is above 10%.
  DEL-16-04 has the highest: 1 of 26 = 3.8%.
- No firm error falls in a 100%-sampled class.
- The rerun criteria are therefore not met.

## 3. Package-level firm false-alignment rate

**1 of 16 sampled ALIGNED normative rows = 6.3%.** This is above the 5%
scale-out gate.

- The denominator is small: a single row decides the result.
- The only firm false alignment is DEL-16-04 `REQ-16-04-09` (§4, item F1).
- Counting weak disagreements as well, the ALIGNED-normative rate is 2 of 16
  = 12.5%.

## 4. Disagreements

### Firm

**F1. `DEL-16-04:SOW#CLM-009/REQ-16-04-09`**

- **What the row says:** ALIGNED, MEDIUM confidence, no cause or tier. The
  worker's own note says the requirement is "partially evidenced beyond the
  rationale slice" and that the "integration TBD until concrete paths are
  assigned" clause "remains a fair statement".
- **What I found:**
  - The requirement is that plugin, adapter, persistence, report and
    application-service paths "that touch this control surface" preserve
    schema, provenance, private-data, protected-content, diagnostics, hash
    and professional-boundary checks. Its verification cell defers every path
    except the rationale slice (DEL-16-04 `ScopeOfWork.md` L149).
  - Concrete paths do now touch the rationale:
    - persistence of rationale through `editor_intents` in the SCA-003 store
      (`apps/desktop/src-tauri/src/lib.rs` L506/L523/L695); the worker's own
      REQ-16-04-02 note says the same;
    - the report export of proposal rationale
      (`apps/desktop/src/features/report/ReportPanel.tsx`); the worker's own
      STATUS R01 note says the same.
  - No cited evidence shows that those paths preserve the listed checks.
  - No plugin or adapter path touches the rationale, so that part of the
    requirement holds only because the behaviour does not exist. Rule C6(a)
    forbids ALIGNED for such a claim.
  - The Python rationale engine has no product caller. A search of `core/`,
    `apps/desktop/src` and `src-tauri/src` finds only the engines calling
    each other.
- **Right values:** `PARTIALLY_IMPLEMENTED` · `PARTIAL_SLICE`.
  - Tier: `INVARIANT`. The row restates `docs/SPEC.md` §4.4–4.5 checks whose
    subjects are private data, protected content and the professional
    boundary (C3). `PROJECT_BASELINE` is also defensible if SPEC §4.4–4.5 is
    read as baseline rather than as a boundary invariant.
  - Layers: `IP_DATA;CLAIMS`. BaselineClass `NONE`. `AuthorityNeeded`
    `NO`.
  - `RemainingWork`: show each existing touching path (store and report)
    preserving the listed checks, and restate the stale "until concrete
    paths are assigned" clause.
  - At minimum, the verification clause is stale under the worker's own W-3
    rule (`STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE`). ALIGNED is not
    supportable on any reading.

### Weak

**W1. `DEL-16-03:CONTEXT#description`**

- **What the row says:** ALIGNED, MEDIUM confidence. Python audit records
  carry the fields; the durable-retention gap is recorded as FG-02.
- **What I found:**
  - The description is "record accepted/rejected operations, affected
    entities, actor/source metadata, timestamps, and assumptions". The only
    recorder is `core/model_operations/audit_trail/engine.py`, which no
    product code calls.
  - In the runtime:
    - acceptance receipts and timestamps are session-only (`persistence_status
      session_state_only_not_yet_saved`,
      `core/model_operations/operation_applier/src/lib.rs` L1984);
    - the ledger records only held proposals
      (`apps/desktop/src/features/operations/OperationLedgerPanel.tsx`
      L151/L228).
  - The worker disposed REQ-005, CLM-023 and STATUS R01, which describe the
    same subject, as `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
    PROJECT_BASELINE · BASELINE`.
- **Right values:** defensible as ALIGNED only if a test-only implementation
  counts as satisfying a CONTEXT description. Otherwise it is
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · NONE · BASELINE`,
  under FG-DEL-16-03-02. The conventions should settle whether an
  implementation with no product caller satisfies such a claim.

**W2 and W3. `DEL-16-04:SOW#CLM-009/REQ-16-04-08` and
`DEL-16-04:SOW#completion-and-reliance-basis-epistemology/AC-001`**

- **What the rows say:** `STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING
  · LOCAL_DESIGN · NONE · RECORD · NO` (FG-DEL-16-04-01, DEC-081).
- **What I found:**
  - The disposition and cause are correct. DEC-081 does name DEL-16-04 as
    "Registry and lint home". `docs/claims_registry.md`,
    `tools/validation/validate_claims_language.py` and its test exist at the
    freeze.
  - REQ-08 cites `docs/IP_AND_DATA_BOUNDARY.md` and restates the IP and data
    boundary. AC-001 restates the human-authority boundary.
  - C3 gives `INVARIANT` to claims that restate a boundary invariant, with the
    subject in `DivergenceLayers`.
  - C3 also gives `LOCAL_DESIGN` to "setup residue that a deliverable
    catch-up would repair with no decision". The stale part here is only the
    "linter integration TBD" clause.
- **Right values:** unresolved. If INVARIANT, use `INVARIANT` with layers
  `IP_DATA;RECORD` (REQ-08) and `CLAIMS;RECORD` (AC-001). That would move
  both rows into the 100% class and into R4 boundary routing. The
  conventions should settle which tier governs a record-drift clause inside
  a boundary-restating requirement.

### Field

**D1. `DEL-16-01:SOW#CLM-009.r01`**

The unsampled sibling `.r03` in FG-DEL-16-01-01 is affected the same way.

- **What the row says:** `IMPLEMENTED_DIFFERENTLY · OWNERSHIP_ELSEWHERE ·
  PROJECT_BASELINE · OWNER`. The row cites `schemas/viewport_editor.schema.yaml`
  (DEL-07-01) as the carrier of the applier's intent taxonomy.
- **What I found:**
  - The disposition, tier and `OWNER` are sound:
    - the applier never loads `schemas/model_operation.schema.json`;
    - DEC-094 binds the vocabulary to the implemented taxonomy
      (`SOFTWARE_DECOMP.md` L685).
  - The evidence and cause are wrong:
    - `viewport_editor.schema.yaml` carries a different, smaller
      `CommandType` enum (L621–632) and does not define
      `EditorOperationIntent`;
    - the intent taxonomy lives in `apps/desktop/src/types.ts` L645
      (`operation_kind: create|connect|delete|insert|modify`) and in the
      applier (`core/model_operations/operation_applier/src/lib.rs`
      `check_kinds`);
    - the applier's header names itself the "DEL-16-02 / DEL-16-03 runtime
      seam" (lib.rs L3).
  - The worker's own reverse answer contradicts `OWNERSHIP_ELSEWHERE`. On
    RC-16-0224 it states that the applier's "supported operation/change-kind
    taxonomy is the operation definition DEL-16-01 describes".
- **Right values:**
  - Replace `schemas/viewport_editor.schema.yaml` in `ImplementationEvidence`
    with `apps/desktop/src/types.ts#L645`.
  - Cause: `OWNERSHIP_ELSEWHERE` still holds if it means the DEL-16-02/03
    seam or the UI types, not DEL-07-01. Under the worker's W-4 rule the cause
    would instead be `DOC_BEHIND_CODE`, because the substitute contract
    landed inside PKG-16's own applier.
  - Disposition, tier and `AuthorityNeeded` are unchanged.

### Checked and agreed (summary)

- **DEL-16-01:**
  - The change-kind enum has 16 values and the test asserts all 16
    (CLM-003).
  - The fixture covers only the original 9 change kinds.
  - The schema `$id` and title, and the fixture contributor, carry the former
    product name (CP-04 on the SOW surface).
  - The revision 0.7 and 0.8 pins are present (CP-02/CP-03 rows).
  - The FG-02 rulings DEC-094 and SCA-003 hold.
  - The CS-04 `.s01` rows are right: the `_CONTEXT` refresh to revision 0.7
    (commit 1f4de36d9, 2026-06-03) postdates SCA-003 (revision 0.6).
- **DEL-16-02:**
  - The applier's `schema_validation` comes from `check_intent_structure`,
    with no JSON Schema validation, and it has no constraint stage. The
    desktop sets `constraint_validation: not_run` (src-tauri lib.rs L4213).
    This confirms FG-02 and FG-03.
  - The `DAG-006/DAG-002_EdgeDispositionReview.md` pointer is wrong; the
    file is in `_DAG/DAG-002/`.
  - CP-05 is correct: Last Updated 2026-07-12 against a history entry of
    2026-07-16.
- **DEL-16-03:**
  - The session-only receipts and the SQLite persistence of `editor_intents`
    are confirmed.
  - The R01 PARTIALLY_IMPLEMENTED disposition under C6(a) is correct.
- **DEL-16-04:**
  - DEC-081's "Registry and lint home: DEL-16-04" is confirmed.
  - The `_STATUS` file was last touched by commit 9b5207670 on 2026-07-28,
    with no Last Updated change. The CP-05-analogous row is correct.
- **All deliverables:** CP-09 applies. `EVIDENCE_MAP.csv` shows PASS parity
  records for DEL-16-01 to 16-04, but their production hashes do not match
  the frozen SOW hashes.
- **Context used as authority:** no sampled row used a context record as
  authority. Merged PRs and run records appear only in Notes or ContextRefs.

### Outside the sample (observations only; not counted)

- **`DEL-16-02:SOW#CLM-010/REQ-16-02-002`** (FG-DEL-16-02-02) records the
  same finding as FG-DEL-16-01-01, that the runtime does not validate against
  the DEL-16-01 schema. The fields differ:

  | Row | Cause | Tier | `AuthorityNeeded` |
  |---|---|---|---|
  | FG-DEL-16-01-01 | `OWNERSHIP_ELSEWHERE` | `PROJECT_BASELINE` | `OWNER` |
  | REQ-16-02-002 | `DOC_BEHIND_CODE` | `LOCAL_DESIGN` | `OWNER` |

  REQ-002 restates SOW-069's "schema validation", so `PROJECT_BASELINE` is
  the consistent tier. `DOC_BEHIND_CODE` is a poor fit: the text is not
  behind the code, because the runtime simply does not do this.
- **`DEL-16-03:SOW#CLM-011`** is ALIGNED although it declares audit-log
  persistence "TBD". Its own note says "the container is ruled". DEL-16-01
  disposed the same kind of container-TBD text `STALE_REVIEW_OR_EVIDENCE ·
  SCOPE_REDIRECTED_BY_RULING` (FG-DEL-16-01-02). This is a within-worker
  inconsistency.
- **`DEL-16-02:STATUS#remaining/R01`** (center-of-gravity hold). The worker
  asked the verifier to consider CP-10. The hold concerns `set_field`
  application semantics. The creation path that requires a center-of-gravity
  vector arrived through the reviewed vocabulary round 3 work. I do not read
  this as a hold settled without a ruling. I have not decided it.

## 5. Batch consistency and shared situations

- `validate_ledger_v2.py --batch` over the four PKG-16 forward ledgers:
  **PASS, 0 consistency findings**. No `CANONICAL_DEPARTURE` is used.
- The 35 inherited rows match `CANONICAL_ASSIGNMENTS.csv` field for field
  (9 each for DEL-16-01, 16-02 and 16-04, and 8 for DEL-16-03, which has no
  Scope Detail row).
- I compared the CS-06 inputs by hand: the PKG-16 package-table row
  (`SOFTWARE_DECOMP.md` L232), `Deliverables.csv` and the SOW-069/SOW-070
  rows of `ScopeLedger.csv`. All are OK.
- There is no shared-situation conflict within PKG-16.
- I also ran a batch across both W1 packages (PKG-16 and the PKG-07
  ledgers). It returned 1 finding, and no PKG-16 row is involved:
  `DEL-07-06:STATUS#remaining/R04` is ALIGNED while the other 3 rows sharing
  its body are UNKNOWN. That finding belongs to the PKG-07 verifier.

## 6. Reverse pass

- **Coverage.** Every CLAIMED_BY (8), PARTIAL (32) and UNKEYED (7) answer
  was checked; there are no CONSTRAINS answers. A 10% SHA-ordered sample of
  NOT_MINE answers was also checked: 135 of 1,344, of which 18 name
  PKG-16-adjacent paths. All NOT_MINE answers in the sample are acceptable.
- **Answer distribution: SAMPLE rows against AREA rows.**

  | Routing | Answers | NOT_MINE | Other answers |
  |---|---|---|---|
  | SAMPLE | 84 | 82 (97.6%) | 2 |
  | AREA | 1,328 | 1,262 (95.0%) | 66 |

  - The only SAMPLE answers other than NOT_MINE both fall on RC-16-0248
    (CAP-SHELL-020, native/wasm batch validate/apply): 16-02 PARTIAL and
    16-03 UNKEYED. That capability genuinely belongs to PKG-16, and its path
    names give it away.
  - The other 20 SAMPLE capabilities (solver, physics, shell, reporting) are
    correctly NOT_MINE.
  - The distribution shows no sign of answers driven by path recognition
    beyond the inherent recognisability of the paths.
- **Capabilities claimed by more than one PKG-16 deliverable.** No capability
  has two CLAIMED_BY answers. The overlaps are:
  - RC-16-0224 (OperationOutcome and kind taxonomy): 16-02 CLAIMED_BY, 16-01
    PARTIAL, 16-03 PARTIAL.
  - Apply side, RC-16-0151, -0248 and -0252: 16-02 PARTIAL (validate/block)
    and 16-03 UNKEYED (application and receipts). **No issued key owns
    runtime application**, although the crate emits `deliverable_refs
    ["DEL-16-02","DEL-16-03"]` (lib.rs L2003).
  - RC-16-0285 (fixtures): 16-01 PARTIAL and 16-02 UNKEYED (contract
    corpus).
  - The 11 applier resolver capabilities: 16-01 PARTIAL and 16-02 PARTIAL.
- **Suspected missed claims.**
  - RC-16-0286 (`schemas/operation_outcome.schema.json`). DEL-16-03 answers
    NOT_MINE here but PARTIAL on RC-16-0224 for the same acceptance record.
    The two answers are inconsistent (weak).
  - The DEC-081 claims-language lint
    (`tools/validation/validate_claims_language.py`) does not appear in the
    R1 inventory or in any package routing file. It could not be claimed.
    This is an R3 unmapped-set item.
- **Anchored answers.**
  - DEL-16-01's 11 PARTIAL answers on the applier resolvers use one identical
    boilerplate reason ("Implements a structured operation kind of the kind
    DEL-16-01 defines"). They conflict with its forward note that the runtime
    taxonomy is another deliverable's contract (§4, item D1). COVERS would
    be more consistent with its forward FG-01.
  - I found no answer driven by knowledge of the forward ledger.

## 7. What the owner must see

1. **The scale-out gate input for PKG-16 is 6.3%, above the 5% gate.**
   - It rests on one row out of 16: DEL-16-04 REQ-16-04-09 was marked ALIGNED
     although the worker's own note calls it only partially evidenced.
   - No deliverable meets the rerun criteria.
   - The contested rows are listed in §4 for a fresh correction worker; the
     sealed ledgers cannot be edited in place.
2. **No PKG-16 row carries `INVARIANT` tier. This includes DEL-16-04, the
   professional-boundary controls deliverable.**
   - Every requirement that restates the professional boundary or the IP and
     data boundary is either ALIGNED or tiered `LOCAL_DESIGN`.
   - As a result, the 100% verification class, and R4 boundary routing,
     receive nothing from this package.
   - The conventions should settle whether a stale or record-drift clause
     inside a boundary-restating requirement takes `INVARIANT` tier (§4,
     W2/W3).
3. **Runtime and documented contracts diverge across the whole package.**
   - The product edit route uses `EditorOperationIntent` through the single
     Rust applier. It never validates against the DEL-16-01 JSON Schema.
   - It has no constraint-validation stage, although SOW-069 names one.
   - All four SOWs describe Python engines that no product code calls.
   - Runtime application and receipts are owned by no issued key.
   - The workers correctly routed the contract choice to the owner (FG-DEL-16-01-01
     and FG-DEL-16-02-02) and the constraint gap to PROJECT_BASELINE
     (FG-DEL-16-02-03).
4. **Durable accepted/rejected history (SOW-070) is incomplete.** Acceptance
   decisions, receipts and timestamps are session-only. The package records
   this consistently as `PARTIALLY_IMPLEMENTED · PROJECT_BASELINE`.
5. **The DEC-081 claims lint is missing from the capability inventory.**
6. **Gate evidence covers a different commit.** The rows cite
   `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`, whose candidate is
   `9d55bce37a…`, not the freeze commit `00115c719`. Pass status is asserted
   only at suite level and was not rerun.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
