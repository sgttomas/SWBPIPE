VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-15-RERUN1 verification: DEL-15-02, rerun cycle 1 (wave W3)

A fresh verifier checked this report against the evidence only. Brief:
`briefs/R2-VERIFIER_brief.md` (SHA-256 `47fb3c52…5b00`, matched). Sampling:
STANDARD. Evidence checkout: `00115c71931bcae79909602d653740d3bb72dfa1`
(the `HEAD` of the freeze was confirmed).

Scope: only the fresh DEL-15-02 ledgers in `WAVES/W3/PKG-15/DEL-15-02/`. I did
not read `superseded_1/` or `PKG-15_VERIFICATION.md`. I read the manager's
records in `_run_records/W3-PKG-15-RERUN1-MANAGER/`, including the
`LAUNCH_G1.md` launch message and the validator transcripts, as context only.
I read the other PKG-15 reverse files only for the four capability IDs that
DEL-15-02 claims or covers.

Inputs, with hashes recomputed:

- `DEL-15-02_forward.csv` `989d1fc9…13d75`, which matches `DEL-15-02_SEAL.txt`
  and the worker's report;
- `DEL-15-02_reverse.csv` `4f463026…eff99`;
- `DEL-15-02_notes.md` `30bee450…bee88`.

## 1. Result table: DEL-15-02

| Item | Value |
|---|---|
| Forward rows (body) | 134 (sentinel 134); 70 required keys; 9 CS-keyed |
| Reverse rows (body) | 290 (sentinel 290) |
| Sampled, 100%: INVARIANT / UNKNOWN (no ACCEPTED_DIVERGENCE, AUTHORITY_CONFLICT, LRR, ISSUED, PROTECTED_CHECK or FROZEN_CONTRACT rows exist) | 17 of 17 |
| Sampled, 100%: SharedTextCount > 1 | 10 of 10 |
| Sampled, 100%: ALIGNED with `GAP_WORDING_CHECKED:` / `OPEN_ACTION:` | 1 of 1 |
| Sampled, 25%: ALIGNED with `PRODUCT_CALLER: NONE` | 8 of 31 |
| Sampled, 25%: other non-aligned | 7 of 28 |
| Sampled, 20%: other ALIGNED normative | 1 of 4 |
| Sampled, 10%: structural and inherited canonical | 2 of 20 |
| **Deterministic sample total** | **46** |
| Targeted extra checks, outside the sample (see §4) | 5 |
| Firm / weak / field (deterministic sample) | 0 / 1 / 1 |
| Firm (targeted extra checks) | 1 (`SOW#CLM-020`) |
| Firm error rate, deterministic sample | 0 / 46 = 0.0% |
| Firm error rate, sample plus targeted checks | 1 / 51 = 2.0% |
| **Firm false-alignment rate, sampled ALIGNED normative rows** | **0 / 8 = 0.0%** (with the targeted checks: 1 / 11 = 9.1%; see §3) |

Selection method: in each class, candidate keys were sorted by SHA-256 of the
key string and the lowest `ceil(rate × n)` were taken. A row belongs to the
first class that fits, in the order of the table.

- 25% PC-NONE: CLM-012.r01, CLM-018, CLM-013/V-003, CLM-005.r05,
  CLM-011.r09, CLM-011.r01, CLM-030, CLM-011.r10.
- 25% non-aligned: CONTEXT, CONTEXT#architecture-basis-injection.s01,
  CLM-013/V-004, STATUS, CLM-017, CLM-006.r04, CLM-021.r06.
- 20% aligned normative: CLM-013/V-006.
- 10% structural and canonical: CONTEXT#context-budget-qa, CLM-011.
- Targeted (my own choice, not deterministic): CLM-020, CLM-019.r03,
  STATUS#remaining/R01, CLM-011.r11, CLM-011.r04.

Twenty-three quiet `DECLARED_STATE`, `CONTEXT` and `HISTORY` rows fall in no
rate class of the brief. They were not sampled.

## 2. Verdict reasoning

- No firm error falls in the deterministic sample, so the rerun thresholds
  are not reached. Those thresholds are a firm rate above 10%, or a firm error
  on a 100% class that changes tier or routing.
- The rerun trigger is resolved. The first run's defect was
  `SOW#CLM-005` (Conditions) recorded ALIGNED while its Units and
  Missing-values conditions were unmet.
  - The block is now CONTAINER.
  - `.r01` Units (FG-02), `.r02` Missing values (FG-01) and `.r03` Provenance
    (FG-03) are PARTIALLY_IMPLEMENTED, tier INVARIANT.
  - I confirmed each against `core/handoff/target_mapping/contract.py`:
    - L231–257: absent `mapping_status` defaults to `mapped`, absent
      `mapping_kind` and `value_kind` default to `metadata`, and absent
      provenance becomes `ENGINE_PROVENANCE`;
    - L110–173: diagnostics run after the defaults are applied;
    - L274: the unit check fires only for three `value_kind` names and never
      checks the unit against the dimension.
- One targeted check found the same pattern still present in one block,
  `SOW#CLM-020` (see §4). It is outside the deterministic sample, so it
  contests one row and does not trigger a rerun.

## 3. Package-level firm false-alignment rate (scale-out gate input)

This verification covers DEL-15-02 only. The rate is **0.0%** (0 of 8
sampled ALIGNED normative rows), within the 5% gate.

If my targeted checks are counted, one of 11 ALIGNED normative rows examined
is a false alignment (9.1%). Those checks were chosen because they looked
suspicious, so they are a biased sample. The 0.0% figure is the gate input
under the brief's deterministic selection. Agent 0 and the owner should know
that the only false alignment I found sits outside the sample.

## 4. Disagreements

**Firm (targeted check, outside the deterministic sample): `DEL-15-02:SOW#CLM-020`**

- *What the row says:* REQUIREMENT · ALIGNED · layers NONE · `PRODUCT_CALLER:
  NONE`. It is assessed directly for procedure steps 1–4, 6 and 8 (apart from
  the container line). Steps 5 (`.s01`), 7 (`.s02`) and the container line
  (`.s03`) are split out.
- *What I found:*
  - Step 2 of the block (`ScopeOfWork.md` lines 284–285 at the freeze) reads
    "Apply unit awareness, provenance, no silent defaults, professional-boundary,
    protected-content, and private-data constraints". That is the same
    substance as `CLM-005.r01`–`r03`, which this ledger records as unmet at
    INVARIANT (FG-01, FG-02, FG-03).
  - The `.s01` split carries only step 5's no-coercion clause. It does not
    carry step 2's unit-awareness and provenance elements.
  - The row's Notes never address step 2. They list "boundaries" among the
    steps the contract follows.
  - Under F1 and C6(i), the ALIGNED row covers an element whose gap is
    recorded elsewhere in the ledger. This is the defect class that caused
    the rerun.
- *Right values:* either split step 2 as `.s04` with REQUIREMENT ·
  PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT · INVARIANT · NONE · VALIDATION ·
  FG-DEL-15-02-01 (cross-referencing FG-02 and FG-03), leaving the block's
  remainder ALIGNED; or re-dispose the block row with those values.
  - No owner routing changes: `AuthorityNeeded` stays NO.
  - The ledger is sealed, so the pair goes to `WAVES/W3/RESOLUTIONS.csv`
    (F6).

**Weak: `DEL-15-02:CONTEXT#architecture-basis-injection.s02`** (and, by
extension, the other FG-06 rows: CLM-004.r07, CLM-012.r02, CLM-019.r09,
CLM-020.s03)

- *What the row says:* STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING
  · LOCAL_DESIGN · RECORD. The "physical project package/container" TBD is
  overtaken by DEC-028.
- *What I found:* the disposition and cause are right. For the tier, C3
  offers both LOCAL_DESIGN ("setup residue that a deliverable catch-up would
  repair with no decision, even when the stale text names a ruling") and
  PROJECT_BASELINE ("its substance conflicts with [a ruled decision]"). Text
  that still says a DEC-028-ruled item is TBD fits both. The worker's reading,
  that a catch-up needs no decision, is defensible.
- *Suggested settlement:* the conventions should say which tier a "Still TBD
  item since ruled" takes. CS-04 names that case but gives no tier.

**Field: `DEL-15-02:SOW#CLM-013/V-004`**

- *What the row says:* PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN ·
  layers RECORD. No provenance test and no unit/provenance validation-plan
  record exist.
- *What I found:* the disposition, cause and tier are defensible, and F8 lets
  a verification-coverage gap keep LOCAL_DESIGN. The layer is wrong. C5
  defines RECORD as documentation drift "with no protected layer affected",
  but a missing unit/provenance verification is a validation-and-provenance
  gap.
- *Right value:* DivergenceLayers = VALIDATION.

**Checked with no disagreement** (the 100% classes, summarised):

- **FG-01** (CLM-005.r02, CLM-011.r07, CLM-020.s01, CLM-021.r03, CLM-026,
  CLM-027.r01, AC-001, CONTEXT#context-envelope, MEMORY): the code confirms
  the silent defaults. PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT · INVARIANT ·
  VALIDATION is consistent across the group.
- **MEMORY:** the undated `## Boundary Decisions` section is assessed as a
  current declaration, which C1 allows. The worker could have used ClaimType
  HISTORY with a `.sNN` row for the section, but the outcome is the same.
- **FG-02** (CLM-005.r01, CLM-011.r05, CLM-027.r02): the unit check is
  presence-only (contract.py L44, L274–286; schema `unit_metadata` requires
  only non-empty strings).
- **FG-03** (CLM-005.r03, CLM-011.r06, CLM-027.r03): provenance substitution
  (L242, L256) and closed `Reference` objects (schema `$defs/Reference`,
  `additionalProperties: false`) confirmed. The IP_DATA layer is justified.
- **CLM-021.r07:** UNKNOWN · EVIDENCE_NOT_LOCATED · CP-12 is right. MEMORY's
  last dependency-validator pass is dated 2026-06-07, and the B4.4 sweep
  (`pytest -q tests`) does not run `validate_dependencies_schema.py`.
- **CONTEXT#architecture-basis-injection.s03:** UNKNOWN · AUTHORITY_UNCLEAR ·
  PROJECT_BASELINE is right. DEC-009 and DEC-010 say nothing on Python
  contract builders; the only Python mention in the decomposition I found is
  DEC-060's dev-tooling note. The smallest next check is given.
- **CLM-005.r04 (GAP_WORDING_CHECKED):** ALIGNED is supported.
  - `_privacy_diagnostics` blocks unaffirmed redaction and embedded payload
    flags.
  - A missing privacy context raises the blocking field diagnostic.
  - The schema's `PrivacyContext` and `Reference` objects are closed.
  - The checked clause is a real distinction, not an unmet element: the
    `public_metadata` label on the engine's own provenance is a labelling
    defect, and FG-03 records it.
- **F7:** `build_target_mapping_contract` is imported only by
  `tests/test_target_mapping_contract.py` and
  `tests/test_handoff_export_workflow.py`. The exporter module
  `core/handoff/exporter/workflow.py` accepts a contract as an argument but
  does not import the builder, and only that test calls the exporter.
  `HandoffPanel.tsx` (L236–242) emits its own
  `target_mapping` object. The `PRODUCT_CALLER: NONE` tags are accurate, and
  the sampled claims concern the contract itself.
- **Structural, CS and CP rows sampled:** CS-01, CS-02, CS-03, CS-04, CS-06
  and CS-07 are inherited as assigned. CP-02, CP-03, CP-05 and CP-09 are
  applied correctly.
  - OUT-001: the evidence map shows three PASS records at production hash
    `9fc7f3bb…`, and the frozen SOW is `4c2c8b23…`.
  - STATUS: Last Updated 2026-07-12, but a history entry is dated 2026-07-16.
- **F3 origin checks:**
  - CLM-021.r06: the "remain ACTIVE and unmodified" text is first found at
    `7bee9ae41`.
  - FG-06: "Canonical package container" is first found at `7bee9ae41`, and
    the `_CONTEXT.md` "physical project package/container" text also dates
    from `7bee9ae41`.
- **Dependency rows** (CLM-011.r10, CLM-019.r03): DAG-002-E0805 is ACTIVE in
  both the local `Dependencies.csv` and `DAG-010/DependencyEdges.csv`, and so
  are E0806–E0810.

## 5. Batch consistency

- Single mode (`--reverse --inventory --notes-gap`) on DEL-15-02: **PASS**,
  with 134 forward rows, 70 required keys, 9 canonical and 0 findings. I
  reran it myself.
- `--batch` over all four PKG-15 forward ledgers (DEL-15-01, fresh DEL-15-02,
  DEL-15-03, DEL-15-04): **PASS, 0 consistency findings**. I reran this too.
- Shared bodies:
  - `SOW#output-and-evaluation-matrix` and its `/OUT-001` are shared with
    DEL-15-03. The values are identical: CONTAINER · COVERED_BY_CHILDREN, and
    CP-09 STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN · LOCAL_DESIGN ·
    RECORD.
  - `SOW#CLM-003.r05` (SharedTextCount 5) has no row, because CLM-003 is
    assessed unsplit, which the all-or-none rule allows.
  - The CS-keyed CONTEXT bodies are inherited as issued.
- No shared-situation conflict.

## 6. Reverse pass

Answer checks:

- **CLAIMED_BY (2 of 2 checked):** RC-15-0235 (CAP-COREC-025, the builder) and
  RC-15-0253 (CAP-DATA-029, `schemas/target_mapping.schema.json`) are right.
  No other PKG-15 deliverable claims either.
- **PARTIAL (1 of 1 checked):** RC-15-0108 (CAP-FEATC-015, HandoffPanel) is
  defensible. DEL-15-03 also answers PARTIAL, and DEL-15-01 answers NOT_MINE.
- **COVERS (1 of 1 checked):** RC-15-0001 (CAP-COREC-028, authority-term
  screen) is right. DEL-15-04 claims it (CLAIMED_BY).
- **NOT_MINE 10%** (29 of 286, deterministic): all are correct. The template
  reason is acceptable where no path overlaps. Its capability text is cut off
  mid-word, which is cosmetic.
- **F5** (8 overlap rows by my path match; the worker listed 10, adding
  RC-15-0072 and RC-15-0103): the brief requires 20%. I checked all 10, and
  each gives a capability-specific reason.
  - Governing documents (TYPES, CONTRACT, IP_AND_DATA_BOUNDARY,
    PROFESSIONAL_BOUNDARY, claims_registry, DIRECTIVE, SPEC) are cited, not
    owned.
  - RC-15-0152, the exporter, belongs to DEL-15-03, which claims it.
  - RC-15-0226 is the dependency validator.
  - RC-15-0158 covers the registers.

Required findings:

- **Capabilities claimed by more than one PKG-15 deliverable:** RC-15-0108
  only, PARTIAL for both DEL-15-02 and DEL-15-03. The two claims divide the
  panel's `target_mapping` block and its export role, so they complement
  rather than conflict. R3 should note that DEL-15-01, owner of the canonical
  handoff package, answers NOT_MINE for the panel that builds that package.
- **Suspected missed claims:** none for DEL-15-02. RC-15-0017 (CAEPIPE MBF
  export) is rightly NOT_MINE: OI-015 names CAEPIPE MBF as a target, and
  target-specific mapping is gated to DEL-17-01 and DEL-17-02.
- **Routing sample vs area:** SAMPLE rows were 37 of 37 NOT_MINE; AREA rows
  were 249 of 253 NOT_MINE (98.4%), with 4 claim-type answers. No SAMPLE row
  names a DEL-15-02 declared path (`core/handoff/target_mapping/`,
  `schemas/target_mapping.schema.json`). The SAMPLE decoys are physics,
  solver and workspace-UI capabilities, which are easy to recognise from
  their paths, so the comparison shows only that nothing was falsely claimed.
- **Anchored answers:** none found.

## 7. For the owner, stated plainly

1. **Rerun independence was partly compromised on the trigger row.** The
   rerun launch message (`LAUNCH_G1.md`) told the worker, before sealing,
   that "the verifier's view is that the correct tier is INVARIANT" for
   CLM-005. So the INVARIANT tier on the CLM-005 split rows is not an
   independent re-derivation. I confirmed it independently under F8, but the
   launch practice should keep verifier conclusions out of rerun briefs until
   after sealing.
2. **One false alignment of the rerun's own defect class remains, outside the
   sample.** CLM-020's step 2 restates the unit, provenance and
   no-silent-default boundaries. Agent 0 should record it in
   `RESOLUTIONS.csv` with the values in §4.
3. **The product does not use this contract.** The shipped desktop
   `HandoffPanel` emits its own `target_mapping` preview object that does not
   follow `schemas/target_mapping.schema.json`. The DEL-15-02 builder is
   reached only from tests. The exporter that consumes its output is itself
   called only from a test.
   The ledger records this under F7. The deliverable's claims are about the
   contract, so the rows stand, but R3 should cluster the gap.
4. **The contract's real defects are INVARIANT-tier (OPS-K-DATA-2,
   OPS-K-UNIT-1, OPS-K-IP-2/DATA-3).** Absent record inputs are silently
   defaulted, including `mapping_status` becoming `mapped`. No dimensional
   check exists. Absent provenance is replaced by the engine's own
   `public_permissive` provenance. These are agent findings of possible
   defects, not rulings.
5. **Dependency register drift.** In DAG-010, `DEL-15-02-D001` and `D002` are
   RETIRED, while the local `Dependencies.csv` still shows them ACTIVE. No
   claim key covers this, so it is outside the ledger.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). The
dispositions here are agent judgments, not owner rulings. They make no
release, approval, compliance or certification claim.
