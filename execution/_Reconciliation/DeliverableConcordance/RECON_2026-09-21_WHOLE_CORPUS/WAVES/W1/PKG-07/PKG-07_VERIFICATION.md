VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-07 verification: wave W1, DOUBLE sampling

- Run: HELP-HUMAN-PIPING-20260921-RECONCILIATION (R2, wave W1).
- Verifier: fresh TASK. Evidence only, independent of the PKG-07 manager and its workers.
- Brief: `briefs/R2-VERIFIER_brief.md`. SHA-256 `db8884ab…303e`, checked before starting.
- Evidence checkout: `00115c71931bcae79909602d653740d3bb72dfa1`, read only.
- Judged against `CONVENTIONS.md`, `CANONICAL_SITUATIONS.md` and `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- I did not read the manager's validator transcripts. I ran `validate_ledger_v2.py` myself. No builds, no test runs, no git writes, no network.

Every disposition here is an agent judgment, not an owner ruling. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. How the sample was drawn

- **Candidates.** Every forward body row: 1,034 rows across 9 ledgers.
- **Class assignment.** Each row went to the first class that applies:
  - **A (100%):** `INVARIANT` tier; the dispositions `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`, `UNKNOWN` and `LIFECYCLE_REASSESSMENT_REQUIRED`; `PROTECTED_CHECK` or `FROZEN_CONTRACT` baseline. No PKG-07 deliverable is ISSUED, so no ISSUED rows.
  - **B (100%):** `SharedTextCount > 1` in `CLAIM_KEYS_V2.csv`.
  - **C (50%):** other non-aligned rows.
  - **D (40%):** `ALIGNED` normative rows (`REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`).
  - **E (20%):** everything else, meaning structural rows, inherited canonical rows and aligned non-normative rows. Aligned non-normative rows fall in no class the brief lists, so I sampled them at the structural rate. This is disclosed as my choice.
- **Selection.** Within each deliverable and class, candidates were sorted by SHA-256 of the `ClaimKey` and the lowest `ceil(rate × n)` were taken.
  - Class D was first ordered by weakness score, then by hash. The score counts LOW or MEDIUM confidence, `NONE_FOUND` verification and `NONE` verification class.
- **Result.** 425 rows sampled. 47 are CS-keyed rows, which the validator checks field by field.
- **Reverse pass.**
  - 100% of the 124 `CLAIMED_BY`, `PARTIAL` and `UNKEYED` answers. There are no `CONSTRAINS` answers.
  - 10% of `NOT_MINE`: 417 of 4,166, the lowest by SHA-256 of `<DEL>:<RC-id>`.
  - Every `NOT_MINE` answer, sampled or not, whose entry point hits a path the deliverable's own forward ledger cites as implementation evidence (51 rows).

## 1. Per-deliverable tables

Class key: A = 100% risk classes, B = 100% shared text, C = 50% non-aligned, D = 40% aligned normative, E = 20% structural, canonical and aligned context.

"FFA" is the firm false-alignment rate among sampled `ALIGNED` normative rows.

| Deliverable | Rows | Sampled A/B/C/D/E (total) | Firm | Weak | Field | Aligned normative sampled | FFA |
|---|---|---|---|---|---|---|---|
| DEL-07-01 | 152 | 1/17/19/17/11 (65) | 1 | 0 | 4 | 17 | 1/17 = 5.9% |
| DEL-07-02 | 111 | 4/8/13/14/8 (47) | 3 | 0 | 3 | 14 | 3/14 = 21.4% |
| DEL-07-03 | 133 | 12/10/14/11/12 (59) | 0 | 4 | 0 | 11 | 0% |
| DEL-07-04 | 86 | 0/5/7/11/9 (32) | 0 | 0 | 0 | 11 | 0% |
| DEL-07-05 | 105 | 0/11/18/10/8 (47) | 0 | 2 | 0 | 10 | 0% |
| DEL-07-06 | 115 | 6/13/17/8/9 (53) | 0 | 1 | 1 | 9 | 0% |
| DEL-07-07 | 125 | 0/11/18/13/10 (52) | 0 | 4 | 0 | 13 | 0% |
| DEL-07-08 | 96 | 0/4/17/10/7 (38) | 0 | 0 | 0 | 10 | 0% |
| DEL-07-09 | 111 | 2/2/8/2/18 (32) | 0 | 0 | 3 | 2 | 0% |
| **PKG-07** | **1,034** | **25/81/131/96/92 (425)** | **4** | **11** | **11** | **97** | **4/97 = 4.1%** |

- The "aligned normative sampled" count is 97 rather than 96. One B-class row is also an `ALIGNED` normative row: DEL-07-06 `CLM-024.r07`.
- **Firm error rate per deliverable:**
  - DEL-07-02 is highest at 3/47 = 6.4%.
  - DEL-07-01 is 1/65 = 1.5%.
  - Every other deliverable is 0%.
- **Rerun thresholds.** No deliverable exceeds 10%. No firm error falls on a 100%-sampled class, so no firm error changes a tier or the owner routing. No rerun is triggered.

## 2. Package-level firm false-alignment rate

**4 / 97 = 4.1%.** This is within the 5% gate, but only just (section 7, item 1).

## 3. Disagreements

### Firm (4). All four are false alignments.

**F1. `DEL-07-01:SOW#CLM-005.r05`** (REQUIREMENT, ALIGNED, LOW)
- **The rule.** If implementation needs to choose a component or state library, record the choice through the architecture decision path.
- **The row says** no library was chosen, so the trigger has not occurred.
- **What I found.** The same ledger disposes `CLM-004.r05`, `CLM-012.r05` and `CLM-028.r03` as CP-10. Those rows say the state and component hold is settled in code, by React built-ins and custom session contexts, without a ruling. DEL-07-02, DEL-07-03 and DEL-07-08 apply the same CP-10 reading. The choice was therefore made and was not recorded, so this row cannot be ALIGNED. Evidence: `apps/desktop/package.json` and `apps/desktop/src/features/workspace/WorkspaceSessionContext.tsx` at the freeze.
- **Right values:** IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · NONE · RECORD · CP-10 · AuthorityNeeded OWNER. It should share the CP-10 finding.

**F2. `DEL-07-02:SOW#CLM-025.r04`** (ACCEPTANCE, ALIGNED)
- **The row says** UI tests confirm that missing solve-required and rule-check-required values are explicit findings. Its own note reads "Rule-check-specific negative test not located".
- **What I found.** `PropertyInspector.tsx` emits `BEND_RULE_INPUT_MISSING` and `BRANCH_RULE_INPUT_MISSING`. No model-tree or table test asserts either flag; they are grepped only in `features/rule-packs/*`. The rule-check half of the acceptance is unmet.
- **Right values:** PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN · NONE · RECORD · NO.

**F3. `DEL-07-02:SOW#CLM-018.r06`** (ACCEPTANCE, ALIGNED)
- **The row asks for** negative UI tests for solve-required and rule-check-required gaps.
- **What I found:** the same gap as F2, and the row's own note records it.
- **Right values:** the same as F2.

**F4. `DEL-07-02:SOW#CLM-026`** (REQUIREMENT, ALIGNED, LOW)
- **The requirement** is a Records list. It includes "Fixture and screenshot provenance notes" and "Protected-content review evidence where applicable".
- **The row's note** says both were not located and that "that gap is carried on RQ-008 rather than repeated here". The common-defect rule (C1) does not license aligning a row whose own element is unmet. RQ-008 itself is UNKNOWN.
- **Right values:** UNKNOWN · EVIDENCE_NOT_LOCATED · LOCAL_DESIGN · NONE · RECORD. RemainingWork should match RQ-008 (locate or run the fixture-provenance and protected-content review).

### Weak (11)

**W1. `DEL-07-06:STATUS#remaining/R04`** (ALIGNED, MEDIUM). This row is the batch conflict; see section 5.
- **The shared text.** Short panels can require scrolling; the transient inspector accessibility-tree (AX) omission is unexplained; no macOS audit or WCAG conformance is claimed. The same body appears as DEL-07-01 R05, DEL-07-02 R04 and DEL-07-09 R04, all disposed UNKNOWN · EVIDENCE_NOT_LOCATED · LOCAL_DESIGN · RECORD.
- **What I found.** The latest observation, `FINAL_ACCEPTANCE.json`, was recorded on 2026-09-17. `apps/desktop/src` changed substantially from 2026-09-18 to 2026-09-21: the SWBPIPE shell, canvas and table work, including `1ad964e30` "contain coordinate table horizontal scrolling". A runtime-behaviour declaration last observed before those changes is better held UNKNOWN than ALIGNED.
- **Resolution.** I resolve the pair toward the majority: UNKNOWN · EVIDENCE_NOT_LOCATED · LOCAL_DESIGN · NONE · RECORD · NO, with the RemainingWork used by DEL-07-02 R04.
- **Why weak rather than firm.** ALIGNED is defensible on the reading that text accurate to the last record stands until contrary evidence exists. The conventions do not settle which reading applies.

**W2–W5. DEL-07-03 R-005 rows in FG-DEL-07-03-02:** `SOW#CLM-012/DEL-07-03-R-005`, `SOW#CLM-005.r04`, `SOW#CLM-008/DEL-07-03-R-005` and `SOW#CLM-017`. All are ACCEPTED_DIVERGENCE · OWNERSHIP_ELSEWHERE · PROJECT_BASELINE.
- **The rows say** DEC-094 re-points the R-005 ownership landing to DEL-07-09, so the load-case editor's absence from DEL-07-03 is sanctioned.
- **What I found.** DEC-094 does say "DEL-07-03-R-005/R-006 ownership landing re-points to DEL-07-09". But the SCA-009 `Vocabulary_Annex.md`, which DEC-094 adopts as the DEL-07-09 coverage contract, says:
  - "DEL-07-09 never dispatches implementation";
  - rows 4 and 8 land in "DEL-07-03 (load-case editor surface, R-005 lineage)".

  Meanwhile the load-case editor exists at `apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx`.
- **The two readings:**
  - Coverage for R-005 sits with DEL-07-09, and the implementation surface sits with DEL-07-03. On this reading R-005 is implemented and the "documented unimplemented in this owner" declarations are stale.
  - The whole requirement moved, which is the worker's reading.
- **Right values.** I would not re-dispose without an authority reading. The candidate is UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE · RECORD · OWNER, consistent with DEL-07-09's own AUTHORITY_UNCLEAR row on palette code landing.
- **R-006 rows are consistent.** The annex lands support forms in DEL-07-02 (rows 2, 12 and 13), and DEL-07-02 answers PARTIAL on RC-07-0098.
- **Related rows, not counted.** `CLM-018` and `CLM-012/DEL-07-03-R-002` rest on the same reading.

**W6–W9. DEL-07-07 open-issue rows:** `SOW#CLM-032/OI-07-07-001`, `-002`, `-003` and `SOW#CLM-005.r06`, all STALE_REVIEW_OR_EVIDENCE.
- **What I found.** These TBD open-issue texts have been present since the initial migration (`git log -S`: 7bee9ae41, 2026-05-18), which makes them setup-era origin text.
- **Right value:** STALE_SETUP_SPECIFICATION. DEL-07-01 and DEL-07-02 use that class for the same kind of setup-era TBD rows.
- **Why weak.** Cause, tier and layers are unchanged. The SOW migration (2026-07-14) re-declared the text, which lets it read as a later declaration, and C6(c) does not settle that.

**W10–W11. DEL-07-05 `SOW#CLM-005.r03` and `SOW#CLM-033.r01`.** The same issue as W6–W9: setup-era TBD text disposed STALE_REVIEW_OR_EVIDENCE.

### Field (11)

| Key | Field | Row has | Should be | Evidence |
|---|---|---|---|---|
| DEL-07-01 `SOW#CLM-013/DEL-07-01-VER-03` | VerificationEvidence | NOT_APPLICABLE, "not rerun" | the DEL-07-01 run record `_run_records/TASK_RUN_2026-06-16_dependency-semantic-refresh_DEL-07-01.md` | Line 32 records VALID. `Dependencies.csv` last changed in the same commit, `28219696d`. ALIGNED is supported, but the record is not cited |
| DEL-07-01 `SOW#CLM-022.r02` | VerificationEvidence | same | same | same |
| DEL-07-01 `SOW#CLM-013/DEL-07-01-VER-04` | VerificationEvidence | NOT_APPLICABLE | the 2026-05-10 enum record and the 2026-06-16 schema record, marked "not rerun" | Without a cited record, CP-12 would force UNKNOWN. DEL-07-03 V-007 shows the correct pattern |
| DEL-07-01 `SOW#CLM-022.r03` | VerificationEvidence | same | same | same |
| DEL-07-02 `SOW#CLM-015/DEL-07-02-RQ-008` | AuthorityNeeded | NO | REVIEW | The next step is a protected-content and fixture-provenance review, not a no-decision catch-up |
| DEL-07-02 `SOW#CLM-025.r06` | AuthorityNeeded | NO | REVIEW | same |
| DEL-07-02 `SOW#CLM-018.r08` | AuthorityNeeded | NO | REVIEW | same |
| DEL-07-06 `SOW#CLM-026` | VerificationClass | BROWSER_E2E | UNIT | The cited test is `tests/test_accessibility_usability_baseline.py::test_gui_baseline_remains_verified_not_validated_with_contrast_target_held`, a Python unit test |
| DEL-07-09 `PALETTE_OPERATION_ROUTING#all-row-routing-and-bounded-residuals.r02` | CauseTag | EVIDENCE_OVERTAKEN | RECORD_DRIFT | "Review pending" was overtaken by a completed review: `REVIEW_RETURN_V2.md` reads "Verdict: PASS". EVIDENCE_OVERTAKEN means records that no longer bind the frozen bytes, which is not the case. DEL-07-03 R03/R04 use RECORD_DRIFT for the same fact |
| DEL-07-09 `PALETTE_ORGANIZATION_CONTRACT#evidence-disposition` | CauseTag | EVIDENCE_OVERTAKEN | RECORD_DRIFT | same |
| DEL-07-09 `PALETTE_ORGANIZATION_CONTRACT#n7-f1-f2-f3-repair-amendment-final-rereview-pend.s04` | CauseTag | EVIDENCE_OVERTAKEN | RECORD_DRIFT | same |

### Checked and agreed

Rows I checked specifically and agree with:

- **ACCEPTED_DIVERGENCE for R-006 and the SCA-009 boundary note.** DEC-094 and SCA-009 action 11 support them.
- **VERIFIED_NOT_VALIDATED (VNV) holds:**
  - DEL-07-03 PDU-049 and DEL-07-06 PDU-045/046, as INVARIANT · OWNER_HOLD. D-68 says "The previously held independent usability work remains held".
- **DEL-07-02 IP UNKNOWN rows.** No protected-content or fixture-provenance review was found by `git grep`.
- **DEL-07-05:**
  - Ratio CP-11: `ResultsPanel.tsx:114` shows `ratioCount={0}`.
  - Rotational-overlay residual: `buildDeformationOverlay` maps only `global_nodal_displacement_*`.
- **DEL-07-08 REQ-07-08-002 POSSIBLE_DEFECT.** `DesignWorkspacePanel.tsx:204–216` hard-codes `core_contract_evidence` counts.
- **DEL-07-04 IP_BOUNDARY_WARNING test gap.** It is only asserted as "available" and as a referenced class in `App.test.tsx`.
- **CP-07 on the PR #789, D-72 and N7 Remaining items.**
- **CP-08 on `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json`.** `git ls-files` finds no such file.
- **CP-04 on the DEL-07-01, 03, 05, 06 and 07 SOW SURFACE rows.**
- **CP-05 dates.**
- **Missing INIT.md and retired FR-003/FR-013.** `INIT.md` is absent from the freeze, and FR-003/FR-013 have zero occurrences in the frozen PRD.
- **CS-06 identity values.** They match `Deliverables.csv`: M for 07-02, 07-04, 07-06 and 07-07; L for the others; objectives match.

## 4. Mechanical conformance

- **Single mode.** `validate_ledger_v2.py` on each ledger, forward plus reverse, returned PASS for all nine with 0 findings. That covers 1,034 forward rows, and the CS rows are checked field by field.
- **Batch mode.** Over the nine forward ledgers it returned **FAIL with 1 consistency finding** (section 5).

## 5. Batch consistency and shared-situation conflicts

- **The one conflict.** Body `1840ad3a…` (the short-panel and accessibility-tree Remaining item):
  - DEL-07-01 R05, DEL-07-02 R04 and DEL-07-09 R04 are UNKNOWN · EVIDENCE_NOT_LOCATED · LOCAL_DESIGN · RECORD.
  - DEL-07-06 R04 is ALIGNED and has no `CANONICAL_DEPARTURE`.
  - Resolved by the verifier toward UNKNOWN (W1). The sealed DEL-07-06 ledger still carries ALIGNED, so the batch validator will keep failing until Agent 0 either records this resolution or has a fresh worker correct the row.
- **Other shared Remaining bodies are consistent across all copies:**
  - `60023b82`: 4 rows, CP-07, BaselineClass NONE;
  - `ab6a050b`: 3 rows, CP-07, RULED_CRITERION;
  - `b2546323`: 2 rows, CP-08, OWNER.
- **Field inconsistencies across the package (not counted per row):**
  - **Remaining-item ClaimType.** DEL-07-06, and part of DEL-07-01 through 07-07, use REMAINING_WORK. DEL-07-01, 02 and 09 mostly use DECLARED_STATE, including on identical bodies.
  - **Remaining items with accurate text and an open action.** These take ALIGNED (DEL-07-06 R01, R05, R06; DEL-07-03 R02; DEL-07-02 R05–R07), DOCUMENTED_UNIMPLEMENTED (DEL-07-05 R01) or PARTIALLY_IMPLEMENTED (DEL-07-07 R01). A4 and CP-06 cover only the no-open-action case, so the conventions do not settle this (section 7).
  - **Unsampled rows, observation only.** DEL-07-08 `SOW#CLM-004.r04` ("Exact dependency versions: TBD") is ALIGNED. DEL-07-01 `SOW#CLM-012.r04` (the same fact) is STALE_SETUP_SPECIFICATION.

## 6. Reverse pass

**Distribution.**
- 9 × 487 answers:
  - `NOT_MINE` 4,166;
  - `COVERS` 93;
  - `PARTIAL` 48;
  - `CLAIMED_BY` 48;
  - `UNKEYED` 28;
  - `CONSTRAINS` 0.
- **Sample-row control.** All 108 answers on SAMPLE-routed control rows (12 per deliverable) are `NOT_MINE`. On AREA rows, 4,058 of 4,275 answers (94.9%) are `NOT_MINE`. No deliverable claimed an out-of-area control row. As the brief notes, about half the control rows can be recognised from their paths, so this is a weak test.

**All 124 owner-type answers checked.** All are plausible. `UNKEYED` answers correctly cite annex landings: rows 19–21 and 24 for DEL-07-01 and DEL-07-02, and rule-pack authoring for DEL-07-03.

**Capabilities claimed by more than one deliverable:**
- **RC-07-0365** (deformed-shape overlay, `PipeViewport.tsx::buildDeformationOverlay`) is `CLAIMED_BY` both DEL-07-01 (REQ-02) and DEL-07-05 (CLM-004.r03). This is a true double claim for R3.
- **Shared-infrastructure splits (PARTIAL by two or three deliverables).** These look reasonable:
  - RC-07-0006, 0078, 0080, 0117, 0164, 0173, 0239, 0311, 0346 and 0447;
  - RC-07-0122 (UNKEYED for DEL-07-01, PARTIAL for DEL-07-06);
  - RC-07-0074 and 0257 (UNKEYED for both DEL-07-01 and DEL-07-02, matching annex row 21).

**Suspected missed claims:**
- **Load-case manager.** RC-07-0255, 0396, 0454 and 0308 point at `features/load-cases/LoadCaseManagerPanel.tsx`.
  - Annex rows 4 and 8 land the load-case editor surface in DEL-07-03, which answered `NOT_MINE`, citing DEC-094.
  - **RC-07-0454** received no non-`NOT_MINE` answer from any PKG-07 deliverable, not even a DEL-07-09 `COVERS`.
  - This is tied to W2–W5.
- **Workspace shell.** No PKG-07 deliverable claims RC-07-0384 (shell composition), 0293 (shell layout rules), 0280/0028 (menu bar and model), 0158 (stage rail) or 0418 (toolbar). DEL-07-09 answered `NOT_MINE` on the toolbar, which includes undo and redo (vocabulary row 10). Possibly DEL-00-05 or unowned; for R3.
- **Screened sample.** The mechanical screen found 51 `NOT_MINE` answers whose entry point hits a path the deliverable itself cites. Apart from the load-case items above, they are shared files cited as evidence (`package.json`, `docs/*.md`, `operationService.ts`, `types.ts`) and are correctly `NOT_MINE`. The 13 keyword-flagged rows in the 10% sample are all correct.

**Anchored answers.**
- Most `NOT_MINE` reasons are one template per deliverable. For example:
  - "Desktop feature 'X' is outside DEL-07-0n's scope" (about 117–125 each for DEL-07-01, 02 and 09);
  - "No … claim of DEL-07-0n owns or covers this capability" (about 450–478 each for DEL-07-03 through 08).

  These templates are path- or area-driven rather than capability-driven, so `NOT_MINE` answers carry little independent evidence. I found no wrong `NOT_MINE` beyond the load-case items.
- Owner-type answers cite specific keys and read as unanchored.

## 7. What the owner must see

1. **The scale-out gate is only just passed.** Package firm false alignment is 4.1% against the 5% gate.
   - All four false alignments share one pattern: the row's own Notes record an unmet element, but the row is marked ALIGNED because the gap is "carried on" or "dispositioned on" another row.
   - Recommendation: before scale-out, add a brief rule that a row whose own note records an unmet element of its claim may not be ALIGNED.
   - DEL-07-02's aligned-normative sample has 3 firm false alignments in 14 rows, about 21%.
2. **One shared-situation conflict remains in the sealed ledgers.** DEL-07-06 R04 is ALIGNED against three UNKNOWN copies. I resolved it toward UNKNOWN, but batch mode stays FAIL until the resolution is recorded or the row is corrected. Gate condition 3 depends on how Agent 0 records this.
3. **DEC-094 wording conflicts with the SCA-009 annex over where R-005 lands.**
   - DEC-094 says the R-005/R-006 ownership landing re-points to DEL-07-09.
   - The annex says DEL-07-09 never receives implementation and lands the load-case editor surface in DEL-07-03 (rows 4 and 8).
   - The implemented Load Case Manager is claimed by no PKG-07 deliverable, and RC-07-0454 has no answer at all.
   - DEL-07-03's ACCEPTED_DIVERGENCE rows depend on the first reading. An owner reading is needed.
4. **Two convention gaps should be settled before 102-deliverable scale-out:**
   - (a) Remaining items with accurate text and an open action are disposed three different ways, and their ClaimType varies. A4 and CP-06 do not cover this case.
   - (b) STALE_SETUP_SPECIFICATION versus STALE_REVIEW_OR_EVIDENCE for setup-era text re-declared by the SOW migration.
5. **One double claim for R3.** RC-07-0365, the deformation overlay, is claimed by both DEL-07-01 and DEL-07-05.
6. **Workspace-shell capabilities are unclaimed within PKG-07.** These are the menu, stage rail, shell composition and toolbar.
7. **Active code identifiers carry the former name but no SOW names them.** They are `openpipestress-preview-design-workspace-*.json` and `openpipestress.technical_preview.design_authoring_comparison_workspace` in `DesignWorkspacePanel.tsx`, and the `core/gui/model_tree/engine.py` PROVENANCE `source_name`. Workers routed them as R3 observations under CP-04's "names" wording. The R4 rename class should pick them up.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
