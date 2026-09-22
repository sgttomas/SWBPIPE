VERDICT: RERUN DEL-13-02

# PKG-13 verification (wave W3, sampling STANDARD)

This is the fresh, evidence-only package verifier for run
`HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Parent: HELP_HUMAN Agent 0.

- **Brief:** `briefs/R2-VERIFIER_brief.md`. Its SHA-256 (`47fb3c52…5b00`) was
  checked and matches.
- **Evidence checkout:** `00115c71931bcae79909602d653740d3bb72dfa1`, the
  read-only freeze. I confirmed it with `git rev-parse HEAD`.
- **Rules applied:** `CONVENTIONS.md` Parts A to F, `CANONICAL_SITUATIONS.md`
  and `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- **Manager and worker records:** I read the PKG-13 manager's transcripts, the
  G1 return and the worker notes only for their flags. I did not use them as
  evidence for any claim.

These are agent verification judgments, not owner rulings. They do not state
or imply any release, approval, compliance or certification (F-PIP-2; claims
taxonomy per DEC-081).

## 0. Integrity and mechanical checks

- **Seals.** For all four ledgers, the forward SHA-256 I recomputed matches
  both the `_SEAL.txt` line and the manager's return. The four reverse hashes
  also match the return.
- **Single mode.** I ran `validate_ledger_v2.py` with `--forward --reverse
  --inventory ROUTING/PKG-13_capabilities.csv --notes-gap`, once with the repo
  root set to the freeze and once to the worktree. All four ledgers pass with
  0 findings both times:

  | Deliverable | Forward rows | Required keys | Canonical rows |
  |---|---|---|---|
  | DEL-13-01 | 104 | 66 | 9 |
  | DEL-13-02 | 85 | 73 | 8 |
  | DEL-13-03 | 112 | 63 | 9 |
  | DEL-13-04 | 110 | 92 | 9 |

- **Batch mode.** `--batch` over the four forward ledgers: PASS, 0
  consistency findings. `WAVES/W3/RESOLUTIONS.csv` does not exist yet.
- **Cross-wave batch.** I ran `--batch` over all 89 forward ledgers present
  (W1, W2 and W3) with `WAVES/W2/RESOLUTIONS.csv`. It reports 10 findings,
  none of them on a PKG-13 row.
- **Keyed canonical rows (CS-01 to CS-07).** The validator checks these on
  100% of rows, and all conform.
- **Test-case tokens.** The PKG-13 ledgers cite no `path::case` tokens. I
  confirmed at the freeze every named test the Notes rely on, for example
  `test_missing_data_and_unresolved_references_are_explicit_findings` at
  `tests/test_constraint_validation.py:240`.
- **Gate evidence.** The rows' "not rerun" citations of
  `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (the Python suite run)
  are supported (A6).
- **Lifecycle.** All four deliverables are `IN_PROGRESS`, so none is ISSUED.
  No row carries `PROTECTED_CHECK` or `FROZEN_CONTRACT`.

## Sampling (deterministic)

Within each class, I sorted the candidate keys by the SHA-256 of the full
claim key and took the lowest fraction, rounded up. The STANDARD rates were
used.

| Code | Class | Rate |
|---|---|---|
| C100 | `AuthorityTier=INVARIANT`; or disposition UNKNOWN, ACCEPTED_DIVERGENCE, AUTHORITY_CONFLICT or LIFECYCLE_REASSESSMENT_REQUIRED; or baseline PROTECTED_CHECK or FROZEN_CONTRACT | 100% |
| SH | `SharedTextCount > 1` in `CLAIM_KEYS_V2.csv`. A minted `.sNN` or `.rNN` key takes its parent's count | 100% |
| F2/4 | ALIGNED, with Notes carrying `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% |
| F7 | ALIGNED, with Notes carrying `PRODUCT_CALLER: NONE` | 25% |
| NA | Other non-aligned rows | 25% |
| AN | ALIGNED normative rows | 20% |
| Q | Structural rows, inherited canonical rows and other quiet rows | 10% |

- Each row falls in the first class it matches, in the order above.
- **Disclosed departure.** The AN class was selected by hash only. It was not
  weighted toward LOW or MEDIUM confidence, because every ALIGNED normative
  row in PKG-13 is HIGH confidence.
- **Reads outside the sample.** I also read every row the worker flagged. I
  read every row in all four ledgers whose summary or Notes mention units,
  dimensions, PKG-02 or the `Parameter` value rule. I read every DEC-009 row.
- **Total:** 130 of 411 forward rows were sampled.

## 1. Package-level result

- **Firm disagreements on sampled rows: 2.**
  - DEL-13-01 CLM-005.r05, in class F7.
  - DEL-13-02 CLM-019.r04, in class F2/4.
  - There are 4 more firm rows outside the sample, all in DEL-13-02, all
    flagged by the worker or of the same substance.
- **Weak disagreements on sampled rows: 2.** DEL-13-02 CLM-018 and DEL-13-03
  CLM-005.r04.
- **Field disagreements on sampled rows: 2.** DEL-13-01 MEMORY and DEL-13-01
  REQ-13-01-011. There is 1 more outside the sample.
- **Package firm false-alignment rate: 1 / 37 sampled ALIGNED normative rows
  = 2.7%.** The gate is 5% or less, so it is met on the sample. The package
  has one more firm false alignment on a normative row outside the sample:
  DEL-13-02 CLM-012/R-13-02-005, an ACCEPTANCE row. Counting it gives
  2 / 38 = 5.3%, which is above the gate. §7 states this plainly.
- **Rerun conditions:**
  - **DEL-13-02: RERUN.** CLM-019.r04 is a firm error in a 100%-sampled class
    (F2/4). It changes the tier (quiet to `PROJECT_BASELINE`) and the routing
    (`NO` to `REVIEW`). Four more firm rows outside the sample (§4.2) confirm
    that the ledger needs a fresh worker. The worker asked for that itself in
    its notes (B1: sealed ledgers are corrected only by a fresh worker).
  - **DEL-13-01: accept with a contested row.** Its firm error rate is 1/30 =
    3.3%. The firm row is in the 25% F7 class, so no rerun condition is met.
    It goes to `RESOLUTIONS.csv` (F6).
  - **DEL-13-03 and DEL-13-04: accept.** Neither has a firm error.

## 2. Per-deliverable tables

Cells read "total / sampled". "False-align" is the firm false-alignment rate
among sampled ALIGNED normative rows, drawn from every class.

| Deliverable | Rows | C100 | SH | F2/4 | F7 | NA | AN | Q | Sampled | Firm | Weak | Field | False-align |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| DEL-13-01 | 104 | 1/1 | 9/9 | 2/2 | 23/6 | 21/6 | 9/2 | 39/4 | 30 | 1 | 0 | 2 | 0/9 = 0% |
| DEL-13-02 | 85 | 0 | 7/7 | 3/3 | 15/4 | 14/4 | 15/3 | 31/4 | 25 | 1 (+4 outside) | 1 | 0 (+1 outside) | 1/9 = 11.1% |
| DEL-13-03 | 112 | 0 | 9/9 | 7/7 | 20/5 | 20/5 | 13/3 | 43/5 | 34 | 0 | 1 | 0 | 0/9 = 0% |
| DEL-13-04 | 110 | 2/2 | 12/12 | 8/8 | 14/4 | 22/6 | 21/5 | 31/4 | 41 | 0 | 0 | 0 | 0/10 = 0% |

Sampled keys (suffixes after `<DEL>:`):

- **DEL-13-01**
  - C100: SOW#CLM-009/REQ-13-01-011.
  - SH: CONTEXT#objective-support, #preparation-notes,
    #decomposition-reference, #package-reference,
    #architecture-basis-injection (with .s01 and .s02),
    #sca-002-control-surface-refresh-note; STATUS#remaining.
  - F2/4: SOW#CLM-011.r04, .r05.
  - F7: SOW#CLM-004.r01, CLM-008, CLM-009/REQ-13-01-003 and -010,
    CLM-005.r05, CLM-011.r06.
  - NA: STATUS, SOW#CLM-019.r04, CLM-012, CLM-004.r04, CONTEXT, SOW.
  - AN: SOW#CLM-018.r05, CLM-004.r02.
  - Q: SOW#CLM-019.r05, MEMORY, CONTEXT#scope-detail, SOW#CLM-018.
- **DEL-13-02**
  - SH: CONTEXT#package-reference, #sca-002-control-surface-refresh-note,
    #architecture-basis-injection (with .s01 and .s02), #preparation-notes,
    #decomposition-reference.
  - F2/4: SOW#CLM-012/R-13-02-004, CLM-019.r04; CONTEXT#scope-detail.
  - F7: SOW#CLM-010/R-13-02-002, -003, -007 and -008.
  - NA: SOW#CLM-006, CLM-019.r07, CLM-017, output-and-evaluation-matrix/OUT-001.
  - AN: SOW#CLM-019.r08, CLM-012/R-13-02-006, CLM-018.
  - Q: SOW#CLM-026, CLM-027, CLM-013, purpose-and-objective-traceability.
- **DEL-13-03**
  - SH: CONTEXT#preparation-notes, #architecture-basis-injection (with .s01,
    .s02 and .s03), #sca-002-control-surface-refresh-note,
    #decomposition-reference, #package-reference, #objective-support.
  - F2/4: SOW#CLM-033, CLM-017, CLM-025, CLM-013.r03, CLM-008, CLM-006.r06;
    STATUS#remaining/R01.
  - F7: SOW#CLM-005.r04, CLM-011, CLM-005.r07, CLM-005.r01, CLM-013.r05.
  - NA: VER-001, SOW#CLM-022.s01, CONTEXT, SOW#CLM-004.r05, STATUS.
  - AN: SOW#CLM-015, CLM-023.r02, CLM-023.r04.
  - Q: SOW#CLM-023, CONTEXT#scope-coverage, CONTEXT#description,
    SOW#CLM-006.r04, CLM-029.
- **DEL-13-04**
  - C100: STATUS#remaining/R03, SOW#CLM-017/DEL-13-04-REQ-007.
  - SH: CONTEXT#architecture-basis-injection (with .s01, .s02 and .s03),
    SOW#CLM-027, CLM-005, CLM-040, CONTEXT#sca-002-control-surface-refresh-note,
    #decomposition-reference, #objective-support, #preparation-notes,
    #package-reference.
  - F2/4: STATUS#remaining/R01, R02; SOW#CLM-021, CLM-017/DEL-13-04-REQ-004,
    CLM-036, CLM-010, CLM-048, CLM-022.
  - F7: SOW#CLM-013, CLM-008, CLM-039, CLM-017/DEL-13-04-REQ-001.
  - NA: SOW#CLM-034.r01, CLM-035, CLM-034.r10, CLM-014.s02, CLM-007, CLM-024.
  - AN: SOW#CLM-031, CLM-034.r05, CLM-034.r04, CLM-032, CLM-028.
  - Q: STATUS#remaining, SOW#CLM-046, CLM-016,
    completion-and-reliance-basis-epistemology.

### Checks behind the main sampled readings

These were confirmed at the freeze:

- **PKG-02 dimension vocabulary (drives F1, F2 and X1 to X4).**
  - `schemas/units.schema.yaml` `$defs.DimensionId` has 30 values, including
    `force_per_length` (line 259). `git log -S force_per_length` shows the
    value present since `7bee9ae41`.
  - `schemas/design_knowledge.schema.json` and `schemas/constraint.schema.json`
    `$defs.Quantity.dimension` each have 29 values. Both lack
    `force_per_length`.
  - Both structural tests assert equality with a local
    `ACCEPTED_PKG02_DIMENSIONS` set (line 89) that also lacks it. The tests
    therefore pin the incomplete set.
  - The DEL-13-03 engine's `CANONICAL_DIMENSIONS` equals DimensionId exactly
    (30 = 30), as does the DEL-13-04 contract's set. A `force_per_length`
    quantity can pass the engine and still fail both public schemas.
  - The worker's post-seal flag is right in substance. It is wrong for two of
    its six rows (§4.3).
- **`Parameter.value` versus `value_kind`.** Confirmed in both schemas.
  `value` is a `oneOf` that does not depend on `value_kind`, so a `quantity`
  parameter validates with a bare string. DEL-13-03 catches this at runtime
  (`CV-PARAMETER-QUANTITY-KIND-MISMATCH`, `engine.py:701`).
- **DEL-13-01 REQ-13-01-011** (`IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
  INVARIANT · VALIDATION · OWNER`). Confirmed.
  - Tauri `load_design_knowledge` (`lib.rs:1511`) reads
    `fixtures/product_preview/invented_design_knowledge.json`.
  - That document's shape is not the DEL-13-01 schema: `provenance` is the
    bare string `invented_example`, and records carry kind, title and
    summary. The panel does not validate it against the schema.
  - The disposition, tier and routing stand.
- **DEL-13-03 and DEL-13-04 DEC-009 rows** (FG-DEL-13-03-01,
  FG-DEL-13-04-01).
  - DEC-009 (SOFTWARE_DECOMP line 600) adopts "Rust core/application
    services" as the runtime baseline.
  - Both engines are Python, and I found no permitting ruling.
  - The rows stand, but §5 records a cross-package conflict about how this
    situation should be classified.
- **DEL-13-04 REQ-007** (`VERIFIED_NOT_VALIDATED · INVARIANT · VALIDATION ·
  ENGINEERING`). It stands on the deliverable's own held basis (CLM-021, the
  D-41 T2B record).
- **DEL-13-04 STATUS#remaining/R03** (`PARTIALLY_IMPLEMENTED · INVARIANT`).
  It stands. F2 applies because no governing row carries broader
  trace-completeness validation.
- **DEL-13-04 CLM-007.** `docs/PRD.md` is v0.4 and carries §8.3 (line 262)
  and FR-MOD-007 (line 452). CP-02 is correct.
- **CP-09 rows** (DEL-13-02 OUT-001, DEL-13-03 VER-001). The PASS parity
  records exist and bind SOW hash `43d9ea2f…`, not the frozen `78a2ff6e…`,
  which I recomputed. `EVIDENCE_OVERTAKEN` is correct.
- **F3 origin tests.** CLM-012 of DEL-13-01 and the CP-01 rows I sampled
  trace to `7bee9ae41`. `STALE_SETUP_SPECIFICATION` is correct.
- **F7 callers.** Confirmed. `validate_constraint_envelope` is called only
  from `tests/`. The transform and adapter are called only from tests, and
  `core/handoff/exporter/workflow.py:254` names `contract.py` as a string.
  Both schemas are loaded only by their tests.
- **DEL-13-03 CLM-005.r07.** It has exactly five `IP_BOUNDARY_WARNING`
  emission sites.

## 3. Package-level scale-out gate input

Firm false alignment is **1 / 37 = 2.7%** on sampled ALIGNED normative rows.
The gate is 5% or less. Counting the confirmed normative row outside the
sample gives 2 / 38 = 5.3%.

## 4. Disagreements

### 4.1 On sampled rows

**F1 — DEL-13-01:SOW#CLM-005.r05 (firm; class F7).**

- *Row:* DECLARED_STATE · ALIGNED HIGH. The claim is that "Quantity requires
  value, unit, dimension and provenance; the dimension enum matches the
  accepted PKG-02 vocabulary, including `slope`" (SOW line 83).
- *Found:* The enum lacks `force_per_length`, which is in the PKG-02
  DimensionId (§2). The test compares the schema against a local copy of the
  set, not against `units.schema.yaml`. The declared state is false for the
  claim's subject (C6(b)).
- *Right values:* `IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
  PROJECT_BASELINE · NONE · BASELINE · REVIEW`, with `RemainingWork`: add
  `force_per_length` to `Quantity.dimension` and to the test set, or derive
  the test set from `units.schema.yaml`. This follows the worker's own
  classification of the unit-rule gap on REQ-13-01-006.
  `LOCAL_DESIGN · NO` is also defensible if Agent 0 treats the enum as a
  plain catch-up. The row is DECLARED_STATE, so it does not enter the
  false-alignment rate.

**F2 — DEL-13-02:SOW#CLM-019.r04 (firm; class F2/4, 100%).**

- *Row:* ACCEPTANCE · ALIGNED. The check is "Unit check | Unit-bearing
  values are unit-aware or blocked as `TBD`" (SOW line 257).
  `GAP_WORDING_CHECKED` reads: "the check as worded concerns Quantity values".
- *Found:* The check is not worded as concerning Quantity. It covers
  unit-bearing values.
  - The same ledger's R-13-02-005 is PARTIALLY_IMPLEMENTED. It records that
    a parameter declared `value_kind: quantity` "may carry a bare string with
    no unit metadata", which the schema accepts (§2).
  - Such a value is neither unit-aware nor blocked as TBD. That is an unmet
    element of this check, recorded in the row's own Notes.
  - F1 says such a row is not ALIGNED "even when the same gap is also
    recorded on another row". The F4 clause rests on a narrowing the text
    does not support.
- *Right values:* `PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT · PROJECT_BASELINE
  · NONE · BASELINE · REVIEW`, MEDIUM confidence, as on R-13-02-005.
- *Consequence:* This changes the tier and the routing on a 100%-sampled
  class, so it meets the brief's rerun condition for DEL-13-02.
- *Contrast:* DEL-13-01 CLM-011.r04 is not the same case (§4.3). Its expected
  result names `Quantity` explicitly.

**W1 — DEL-13-02:SOW#CLM-018 (weak).**

- *Row:* REQUIREMENT (Steps 1–12) · ALIGNED.
- *Found:* Step 7 reads "Ensure every unit-bearing physical quantity in the
  model is represented through the implemented `Quantity` contract". The
  `Parameter` loophole means the schema does not ensure this.
- *Right values under a strict reading:* a `.sNN` for step 7 with R-13-02-005's
  values.
- *Why weak:* Steps are method instructions for producing the schema. The
  conventions do not settle whether a step is judged by its outcome.

**W2 — DEL-13-03:SOW#CLM-005.r04 (weak; class F7).**

- *Row:* REQUIREMENT · ALIGNED, with `PRODUCT_CALLER: NONE`. The claim is
  "Domain-core and adapter paths may not bypass unit checks, provenance
  checks, or public/private data boundaries".
- *Found:*
  - The engine performs the checks, but no product or adapter path consumes
    constraints through it.
  - Constraint status does reach the desktop without the engine:
    - `DesignWorkspacePanel.tsx:204-209` hard-codes
      `constraint_warning_count: 1` and
      `constraint_validation_has_blocking_findings: true` as
      "core_contract_evidence";
    - `lib.rs:1895` labels a proposal's `constraint_validation` as
      `warning_computed_context_requires_human_review`.
- *Right values under the F7 and C6(a) reading* (the claim is about paths):
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN · NONE · RECORD · NO`.
- *Why weak:* The engine is itself a domain-core path, and on the engine
  reading the row holds. The desktop labels are static display values, not a
  bypass of a unit check.

**D1 — DEL-13-01:MEMORY (field).**

- *Row:* The SURFACE row is HISTORY · ALIGNED. Its Notes say the undated
  "Remaining TBDs" list "reads as a current declaration".
- *Found:* C1 says an undated current declaration inside a history surface
  is DECLARED_STATE. The row found the list accurate, so its disposition is
  unaffected.
- *Right values:* a `MEMORY.s01 · DECLARED_STATE · ALIGNED` row for the list.
  This is the same vehicle point as PKG-06 F1 and C3.

**D2 — DEL-13-01:SOW#CLM-009/REQ-13-01-011 (field; class C100).**

- *Row:* `VerificationEvidence` is `core/product_preview/service.py`, and
  `VerificationClass` is `UNIT`.
- *Found:* That file is implementation, not a test. The disposition stands.
- *Right values:* `apps/desktop/src/services/previewService.test.ts` (line
  431 asserts the `load_design_knowledge` invoke) with the GATE token, or
  `NONE_FOUND`.

### 4.2 Outside the sample (not in the rates; all DEL-13-02)

**X1 — SOW#CLM-012/R-13-02-005 (firm; ACCEPTANCE, normative).**

- *Row:* ALIGNED. The claim is that the test checks "exact alignment with the
  accepted PKG-02 dimension vocabulary" (SOW line 167).
- *Found:* The test checks equality with a local set that omits
  `force_per_length`. The alignment it claims is not exact.
- *Right values:* `IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
  PROJECT_BASELINE · NONE · BASELINE · REVIEW`, with the same caveat as F1.

**X2 — SOW#CLM-003 (firm; DECLARED_STATE).** The "Unit posture" attribute
says "the current dimension enum matches the accepted PKG-02 dimension
vocabulary" (line 61). The facts are as in F1. The other attributes hold.
- *Right values:* a `.sNN` row for the unit-posture attribute with F1's
  values, and the block otherwise ALIGNED.

**X3 — SOW#CLM-009 (firm; DECLARED_STATE).** The implementation-evidence
text claims "PKG-02 dimension vocabulary alignment" (line 125). Right values
are as in F1, as a `.sNN` row or on the block.

**X4 — SOW#CLM-024 (firm; CONTEXT, ALIGNED).** Bullet 3 says "the test
fixes the dimension enum to the accepted PKG-02 vocabulary" (line 310). The
worker did not flag this row. Right values: a `.sNN` row for bullet 3 with
F1's values.

**X5 — SOW#CLM-010/R-13-02-005 (field).** The disposition stands. The Note
"the dimension enum matches PKG-02" is wrong for the reason given in F1.

### 4.3 Worker flags, checked

The worker flagged six rows after sealing, for a possible false alignment on
`force_per_length`:

- **DEL-13-01 CLM-005.r05:** agreed. See F1.
- **DEL-13-01 CLM-003:** not agreed. Its unit attribute states only the
  explicit-unit posture ("require explicit unit metadata unless explicitly
  dimensionless"). It makes no vocabulary claim, so ALIGNED stands.
- **DEL-13-01 CLM-011.r04:** not agreed, so ALIGNED stands.
  - The expected result is Quantity-scoped: "`Quantity` requires `value`,
    `unit`, `dimension`, and `provenance`; no dimensionless fallback hides
    missing units".
  - The missing enum value forces `TBD`. It does not force a dimensionless
    fallback.
  - The row's `GAP_WORDING_CHECKED` clause is accurate.
- **DEL-13-02 CLM-003, CLM-009 and CLM-012/R-13-02-005:** agreed. See X2,
  X3 and X1.

The other flags:

- **The invariant rows** (13-01 REQ-13-01-011, 13-04 REQ-007, 13-04 R03):
  agreed (§2).
- **The DEC-009 Python-core departure** (FG-DEL-13-03-01, FG-DEL-13-04-01):
  agreed on the finding, tier and routing. §5 records a classification
  conflict.
- **"Held residual" declarations ALIGNED** (DEL-13-03 CLM-008, 017, 025,
  033, CLM-006.r06; DEL-13-04 CLM-010, 021, 022, 036, 048): agreed.
  - Each block's claim is the declaration of a hold, or an instruction to
    hold, and each is true at the freeze.
  - The open work sits on a governing row that is not aligned (CLM-013.r06,
    CLM-014.s02, REQ-007).
  - F1 is not engaged: no unmet element of these blocks' own claims is
    recorded.

## 5. Batch consistency and shared-situation conflicts

- **Mechanical checks.** Single mode, batch mode and the cross-wave batch
  report no PKG-13 finding (§0). Within PKG-13, every shared-body row (SH,
  100% sampled) is consistent across the four ledgers.
- **Conflicts that batch mode cannot see:**
  - **C1 — DEC-009 Python-core departure (cross-package).**
    - PKG-13 (four rows, two FindingGroups) uses `IMPLEMENTED_DIFFERENTLY ·
      POSSIBLE_DEFECT · PROJECT_BASELINE · NONE · BASELINE · OWNER`.
    - DEL-14-04 (CLM-004.r07, ABI.s02) and DEL-17-07 (ABI) use
      `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · NONE
      · RECORD;BASELINE · OWNER` for the same situation.
    - Tier and routing agree; the cause and the layers differ.
    - My reading: DEC-009 is ruled, not an open hold. C6(d)'s
      `AUTHORITY_UNCLEAR` limb therefore does not fit, and `POSSIBLE_DEFECT`
      is the closer cause.
    - Agent 0 should settle this corpus-wide in `WAVES/W3/RESOLUTIONS.csv`.
      R3 will want these rows as one cluster.
  - **C2 — PKG-02 vocabulary claims (within PKG-13).** DEL-13-03 CLM-006.r04
    (ALIGNED: the engine equals DimensionId) is correct. The DEL-13-01 and
    DEL-13-02 schema rows claiming the same alignment are not (F1, X1 to X4).
    This is not a batch conflict, because the bodies differ. One fresh
    DEL-13-02 worker and a resolution for DEL-13-01 CLM-005.r05 close it.

## 6. Reverse pass

**Checks run.**

- I checked all 11 answers that are not `NOT_MINE`: 5 `CLAIMED_BY`, 2
  `PARTIAL` and 4 `COVERS`. There are no `UNKEYED` or `CONSTRAINS` answers.
  Each is supported by the forward ledger it cites.
- I checked a 10% hash-selected sample of `NOT_MINE` per deliverable:
  32 + 33 + 33 + 32 = 130 rows. I found no misallocation.
- **F5.** I checked every `NOT_MINE` answer whose capability entry points hit
  a path cited by the deliverable's own forward ledger. That is 29 rows
  (17, 4, 4, 4), against the brief's required 20%. All 29 give
  capability-specific reasons.
- No answer anchors on a `STATUS#…` key.

**Answer distribution against routing.** The mapping uses
`ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`: 293 AREA rows and 30 SAMPLE rows per
deliverable.

| | AREA | SAMPLE |
|---|---|---|
| NOT_MINE | 1,161 / 1,172 (99.1%) | 120 / 120 (100%) |
| CLAIMED_BY / PARTIAL / COVERS | 11 | 0 |

- Three SAMPLE rows have entry points on a path that DEL-13-01 cites
  (`apps/desktop/src-tauri/src/lib.rs`):
  - RC-13-0001, project hashes;
  - RC-13-0255, self-weight load plan;
  - RC-13-0302, report-package request.
  All three are answered `NOT_MINE` with specific reasons, which is correct.
- All claim-type answers are on AREA rows. The comparison supports
  unanchored answering, but it is weak evidence at this scale.

**Capabilities claimed by more than one deliverable.**

- **RC-13-0148** (`validate_constraint_envelope`): DEL-13-03 `CLAIMED_BY` and
  DEL-13-01 `COVERS` (through REQ-13-01-011's consumer-path claim). These
  are complementary, and there is no ownership conflict.

**Suspected missed claims (for R3).**

- **RC-13-0245** (the design-workspace engine composing the constraint-warning
  panel).
  - DEL-13-03 answers `NOT_MINE` with its template reason. The engine
    consumes constraint validation output, and DEL-13-03's SOW names GUI
    presentation as downstream TBD work.
  - `COVERS` would be more accurate. F5 does not apply, because the entry
    points do not hit DEL-13-03's cited paths.
- **Desktop constraint status with no engine behind it.**
  - `DesignWorkspacePanel.tsx:204-209` shows fixed constraint counts as "core
    contract evidence". The `lib.rs` proposal and preview payloads carry
    `constraint_validation` labels (`:1895`, `:4213` and others).
  - No PKG-13 deliverable claims or covers these surfaces, and no engine
    produces them.

**Anchored answers.** None found.

## 7. For the owner

1. **The two public PKG-13 schemas omit a PKG-02 unit dimension.**
   - `design_knowledge.schema.json` and `constraint.schema.json` both lack
     `force_per_length`. Their tests lock in the incomplete set, and the
     deliverables' Scope of Work files say the sets "match" PKG-02.
   - The constraint engine and the transform accept the value, so a record
     can pass the engine and still fail the schema.
   - Five ALIGNED rows declare the match, and all five are wrong at the
     freeze (F1, X1 to X4). A sixth row's Note repeats it (X5). Two further
     rows were flagged by the worker but make no vocabulary claim (§4.3).
2. **The scale-out gate is met on the sample (2.7%) but not with all
   confirmed rows.** Adding the one confirmed normative false alignment
   outside the sample (DEL-13-02 CLM-012/R-13-02-005) gives 5.3%, above the
   5% gate. The DEL-13-02 rerun is expected to remove it.
3. **Two engines in PKG-13 are Python, where DEC-009 adopted a Rust core.**
   These are the constraint validator and the physical-to-analytical
   transform. No ruling permits this, so it goes to you at R4. The same
   situation appears in PKG-14 and PKG-17, classified slightly differently
   (§5 C1).
4. **The desktop knowledge path does not use the design-knowledge schema.**
   The live desktop Knowledge panel reads a preview-shaped fixture with
   string provenance, and does not validate it against the design-knowledge
   schema (DEL-13-01 REQ-13-01-011, `INVARIANT`, OWNER).
5. **The desktop shows constraint-validation status that no engine
   produced** (§6). The constraint validator has no product caller.
6. **For R3:**
   - the transform emits former-name provenance strings ("OpenPipeStress
     transform contract", "OpenPipeStress physical-to-analytical adapter");
   - the adapter labels `json.dumps(sort_keys=True)` hashes as `"JCS"`. That
     is not RFC 8785 for all inputs; the SOW calls it "JCS vocabulary" and
     holds release canonicalization as TBD;
   - the preview design-knowledge fixture's `document_kind` carries the
     former name.
7. **Agent 0 should:**
   - record F1 (DEL-13-01), W1, W2, D1, D2 and C1 in
     `WAVES/W3/RESOLUTIONS.csv`;
   - launch a fresh worker for DEL-13-02 covering F2 and X1 to X5.
   The sealed ledgers stay unchanged (F6).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
