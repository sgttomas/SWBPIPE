# T11 — method checks: dependency, terminology, reused evidence, sub-claim splits, shared surfaces

T11 ran the five R3 method checks that no other task covers, over the 102 accepted ledgers (effective values from `R3/CORPUS_CLAIMS.csv`), the freeze at `00115c71` and approved DAG-010. The main results:

- **Dependency records.** The local dependency registers and DAG-010 agree on every edge ID. They differ in one respect: 30 rows are ACTIVE locally but RETIRED in DAG-010. In addition, 651 of the 1,402 active DAG-010 edges cite evidence files that no longer exist at the freeze. The 8 PKG-00 deliverables have no `Dependencies.csv`. This is by design and is recorded as a finding, not an error.
- **Terminology.** Three terms or decisions are read inconsistently across packages, and each needs an owner reading:
  - "JCS-compatible" hashing, read six ways;
  - what counts as a protected-content review record;
  - CP-10 against the "resolved-TBD" reading of holds that the code settled without a ruling.
- **Minted sub-claims.** Batch mode cannot compare `.sNN` sub-claims across ledgers. By hand:
  - SR-1: all 81 SR sub-claims share one disposition, but the cause splits 42/39.
  - The injected still-TBD list is assessed three different ways.
  - 16 deliverables carry the same list without assessing it at all.
- **Shared surfaces.** Of the 137 OWNED capabilities that other deliverables COVER or CONSTRAIN:
  - 128 have compatible readings;
  - 5 share one tension over diagnostic-field preservation;
  - 2 have incompatible readings (npm workspace authority; canonical hashing);
  - 2 are a split already recorded as OBSERVED (the runtime edit contract).

The REUSED_EVIDENCE and SUBCLAIM_SPLIT checks are scripted candidate screens, not complete proofs. The Coverage section says what each screen covers. T11 re-disposes no row and changes no ledger. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

`X/` in `T11_METHOD.csv` Evidence abbreviates `projects/chirality-piping/execution/PKG-*/1_Working/`. `ABI` abbreviates `CONTEXT#architecture-basis-injection`. Paths are at the freeze unless they start with `R3/`, `WAVES/` or a run file name.

## Findings

### DEPENDENCY (7 rows)

| ID | Finding | Route |
|---|---|---|
| D-01 | 94 of 102 deliverables carry `Dependencies.csv`. The eight **DEL-00-01..08 have none**, by design: their `_DEPENDENCIES.md` (e.g. DEL-00-01 L19) sends readers to the aggregate DAG. DAG-010 has 0 edges from PKG-00 and 580 active edges into it. Their pointer names DAG-007, a stale pointer that T4A already classifies. | NO_ACTION |
| D-02 | **Local mirror drift.** The 1,487 IDs match on both sides, and every compared field agrees except Status. 30 rows in 15 deliverables are ACTIVE locally and RETIRED in DAG-010, all in `DAG-010_DuplicateEdgeWorklist.csv` (duplicates retired in DAG-007 but never propagated). This contradicts `DAG_Audit.md:15` ("synchronized mirrors"). Three ledger rows rely on the local state: DEL-07-08 SOW#CLM-005.r04 (ALIGNED on E0846 being ACTIVE), DEL-07-08 CLM-005.r01, and DEL-08-04 CLM-020.r06 (20 ACTIVE; DAG-010 has 19). | R5_RECORD_REPAIR |
| D-03 | **Stale edge provenance.** 651 of 1,402 active edges cite evidence that is missing at the freeze: 421 cite the retired four-document kit, 171 cite a bare `_CONTEXT.md`, and 59 cite other removed paths. For 13 deliverables every edge is affected (DEL-03-07, 07-02, 16-01, 16-03, 17-01..09). Sequencing is not affected. | REVIEW |
| D-04 | **DEL-07-09 has no architecture-basis edge** to PKG-00, the only one of the 94. Its `_DEPENDENCIES.md:14` says such rows "are preserved here", but the register has none. DAG-010 records SCA-009 as exactly three execution edges. | REVIEW |
| D-05 | **Ownership redirects with no DAG edge.** 18 of the 26 OWNERSHIP_ELSEWHERE rows name another deliverable, forming 8 pairs; 6 of those pairs have no edge in either direction. The largest is DEL-07-03→DEL-07-09, where 9 rows record the SCA-009/DEC-094 load-case landing (CLM-005.r04 is CONTESTED). | REVIEW |
| D-06 | **Shared surfaces with no DAG edge.** 59 of the 180 owner/relation pairs have no edge. COVERS/CONSTRAINS is not a sequencing relation; the list is informational. | NO_ACTION |
| D-07 | **Satisfaction against lifecycle.** No SATISFIED edge has a target below its required maturity. The 580 PKG-00 edges rest on the SR-1 reading (see S-01, T8). | NO_ACTION |

### TERMINOLOGY (4 rows)

- **T-01 "JCS-compatible" (OWNER_DECISION).** One behaviour, sorted-compact JSON bytes rather than RFC 8785, takes six readings across 13 deliverables:
  1. ALIGNED, because a label or enum says JCS: DEL-13-04 CLM-031 ("JCS over sorted JSON"), DEL-02-04 CLM-005.r06, and DEL-10-01 REQ-14 ×2. DEL-16-03 CLM-011 is also ALIGNED.
  2. PARTIAL_SLICE: DEL-14-01 and DEL-02-05.
  3. AUTHORITY_UNCLEAR: DEL-02-02.
  4. POSSIBLE_DEFECT: DEL-08-02 and DEL-17-02 REQ-007.
  5. UNKNOWN: DEL-17-09 CLM-018.
  6. CONTRACT_VERSION_ADVANCED: DEL-14-02.

  Meanwhile DEL-00-04 REQ-04-03 is ALIGNED on canonicalization "implemented once". The MBF writer labels sorted-compact bytes `JCS_compatible_json_payload_hash` (`core/handoff/caepipe_mbf/package.py:734,743,790`).
  - *Decision:* does "JCS-compatible" require RFC 8785 bytes?
  - *Options as they stand in the evidence:* (a) move sorted-compact paths (Python persistence, model-state, exporters, audit manifest) to the shared canonical_json crate; or (b) amend the AB-00-04/DEC-010/DEC-017 basis to accept labelled sorted-compact hashing, and relabel.
  - *On ruling:* R5 record repairs to the ledgers' SOWs, plus code-fix candidates.

  This extends the W3 owner item on hash labels with the cross-package split in cause, tier and routing.
- **T-02 Protected-content review evidence standard (OWNER_DECISION).** The split:
  - 24 rows in 11 deliverables are UNKNOWN/EVIDENCE_NOT_LOCATED/INVARIANT because the named review record is missing (following the W1 DEL-07-02 CLM-026 resolution).
  - Comparable claims elsewhere are ALIGNED on the worker's own reading or an automated denylist: DEL-07-05 VER-07-05-002, DEL-07-07 VER-07-07-004, DEL-03-07 CLM-017.r06, DEL-13-01 CLM-011.r02, DEL-10-03 REQ-07, DEL-15-04 CLM-012.r04 and DEL-17-03 CLM-011.r07.
  - The OBSERVED resolution on DEL-06-01 CLM-013 notes that agent self-reviews exist.
  - *Decision:* does an agent reading or a denylist test satisfy a claim that names a protected-content review?
  - *Options:* (a) yes, and the UNKNOWN rows can be re-read; or (b) no, and the ALIGNED rows are under-assessed and one review closes both groups.

  This is not evidence of protected content.
- **T-03 CP-10 against "resolved-TBD" (OWNER_DECISION).** Holds that the code settled with no located ruling take CP-10 in 49 rows: AUTHORITY_UNCLEAR with owner routing. Other rows take DOC_BEHIND_CODE and record catch-up; DEL-04-01 cites a worker-notebook rule for this. The split holds on the same topic (schema file layout: DEL-02-01 and DEL-02-02 against DEL-06-01) and inside single ledgers:
  - DEL-10-03 REQ-09 against CLM-015;
  - DEL-07-02 CLM-010.r02 against .r01, .r04 and .r05.

  *Decision:* do DEC-012 implementation-level TBDs that the code settled need a ruling (CP-10), or are they F3 record catch-up?
- **T-04 (NO_ACTION).** SR-1, DEC-009 and the unit vocabulary belong to T8 and are not re-classed here.

### REUSED_EVIDENCE (5 rows; scripted screen)

- **R-01.** `canonical_json` and `hashService.ts` back both ALIGNED rows (DEL-00-04 REQ-04-03; DEL-02-05 REQ-02-05-012) and POSSIBLE_DEFECT rows (DEL-08-02 CLM-004.r02, CLM-011.r02 and CLM-006.r02). "Implemented once" and a separate non-JCS audit-manifest canonicalization cannot both hold. Part of T-01. OWNER_DECISION.
- **R-02.** The same invented fixtures back ALIGNED and UNKNOWN rows: `invented_preview_model.json` (DEL-03-05 against DEL-03-04), `previewService.ts` (DEL-07-01 against DEL-07-02) and `rule_pack.schema.yaml` (DEL-06-02 against DEL-06-01). Part of T-02. OWNER_DECISION.
- **R-03.** `apps/desktop/src/types.ts` backs DEL-07-08 CLM-004.r06 (ALIGNED) and DEL-08-04 CLM-011.r05 (POSSIBLE_DEFECT, class and remediation overwritten). See SS-01. ENGINEERING_AUTHORITY.
- **R-04.** `caepipe_mbf/package.py` is cited as implementation for DEL-17-01 REQ-013 (ALIGNED), while DEL-17-04 CLM-005.r10 says the option does not exist (CP-11). The readings are compatible, but the citation is misleading. REVIEW.
- **R-05.** The other 11 of the 17 screened paths were judged compatible (different aspects of the same file), and so was the PCF fixture found through SHARED_SURFACE. They were judged on the Notes and summary text. NO_ACTION.

### SUBCLAIM_SPLIT (7 rows; scripted screen plus hand review)

- **S-01 SR-1 (REVIEW; the reading belongs to T8).** 81 SR sub-claims, all STALE_REVIEW_OR_EVIDENCE/LOCAL_DESIGN and OWNER_CONFIRMED. The cause splits 42 RECORD_DRIFT against 39 SCOPE_REDIRECTED_BY_RULING, and the split runs inside six packages (PKG-03, 04, 08, 09, 12, 14).
- **S-02 (R5_RECORD_REPAIR).** The SR statement is present but has no sub-claim in DEL-07-03, 07-04 and 07-06.
- **S-03 (R5_RECORD_REPAIR).** The still-TBD sub-claims split three ways: 52 are STALE_SETUP_SPECIFICATION/SCOPE_REDIRECTED_BY_RULING, 9 are STALE_SETUP_SPECIFICATION/DOC_BEHIND_CODE (DEL-03-04..06, contested; DEL-04-01..06) and 6 are STALE_REVIEW_OR_EVIDENCE/SCOPE_REDIRECTED_BY_RULING. The undated MEMORY TBD sub-claims split 4/4 on the same causes. Every row needs the same repair; the split changes only which R3 class counts it.
- **S-04 (R5_RECORD_REPAIR).** 16 deliverables carry the identical still-TBD list (container, solver, grammar, CI provider and others) with no sub-claim; the freeze lines are cited in the CSV. Only DEL-03-01/02/03 are CONTESTED for this, so 13 are unflagged.
- **S-05 (OWNER_DECISION).** The 9 resolved-baseline sub-claims take 6 signatures (DEC-009 and hash basis), and the PKG-17 parents are not split at all. DEC-009 belongs to T8's cluster; the hash part is T-01.
- **S-06 (OWNER_DECISION, within the T4B rename ruling).** The rename sub-claims split on tier: LOCAL_DESIGN (DEL-05-04, DEL-05-05) against PROJECT_BASELINE (DEL-17-06 frozen profile ID, DEL-02-05 `.opsproj`).
- **S-07 (NO_ACTION).** The frontmatter-pin and PKG-00 AB sub-claim families are uniform.

### SHARED_SURFACE (4 consolidated rows + 137 per-capability rows)

| Per-capability result | Count | Route |
|---|---|---|
| COMPATIBLE_ALIGNED: all owner and relation keys ALIGNED | 49 | NO_ACTION |
| COMPATIBLE_WITH_DIVERGENCE: partial or stale rows, no opposite reading | 71 | NO_ACTION |
| COMPATIBLE_DIFFERENT_ASPECT: flagged by the screen, read by hand, no conflict | 8 | NO_ACTION |
| TENSION (SS-01): diagnostic fields at boundary crossings | 5 | ENGINEERING_AUTHORITY |
| INCOMPATIBLE_READING: CAP-CHECKS-021 (SS-03), CAP-COREC-001 (SS-04) | 2 | OWNER_DECISION |
| KNOWN_SPLIT (SS-02, OBSERVED): CAP-DATA-009, CAP-COREB-020 | 2 | OWNER_DECISION |

- **SS-01 Diagnostic field preservation (AB-00-06; ENGINEERING_AUTHORITY).**
  - DEL-00-06 REQ-06-02 (POSSIBLE_DEFECT) finds that the runner's solve-to-export binding drops `affected_object` and supplies a fixed remediation (`result_envelope_binding.rs:159-182` as cited).
  - On the same surfaces, DEL-10-05 CLM-003.r04, DEL-07-07 CLM-004.r05 and REQ-07-07-005, and DEL-07-08 CLM-004.r06 are ALIGNED.
  - The same synthesized-class behaviour is POSSIBLE_DEFECT in DEL-00-06, DEL-08-04 and DEL-08-03 CLM-026.r03, but PARTIAL_SLICE in DEL-04-01 REQ-011, DEL-06-02 CLM-006.r04 and DEL-08-03 REQ-003.
  - The OwnerKeys of CAP-DATA-016 leave out DEL-08-04 CLM-011.r05, the row that carries the defect.

  *Decision (engineering, or owner per the DEL-00-06 RemainingWork):* either the boundary mappings must carry every field except class (a code-fix candidate for the runner binding and the desktop export writer), or the mappings are accepted and REQ-06-02 is refined (R5 record repair).
- **SS-02 Runtime edit contract.** The contract behind CAP-DATA-009 and CAP-COREB-020 carries FG-DEL-16-01-01; both ledgers already ask the owner to choose the governing contract.
- **SS-03 npm workspace (CAP-CHECKS-021).** DEL-10-04 is ALIGNED "under DEC-074 O3". DEL-00-02 CP-10 says no ruling selects it.
  - *Decision:* does DEC-074 O3 settle the package-manager choice?
  - *Options:* (a) yes, and DEL-00-02 .s03 is re-read; or (b) no, and the owner rules the choice or records it as an accepted implementation choice.
- **SS-04 Canonical hashing (CAP-COREC-001).** The same incompatibility as R-01 and T-01.

## Coverage

All counts come from read-only scripts over `R3/CORPUS_CLAIMS.csv` (9,889 rows), the sealed forward and reverse ledgers (with the `SAMPLE_MANIFEST` mapping, as `tools/synthesize_r3.py` uses), `R3/CAPABILITY_COVERAGE.csv`, and the freeze. No draft resolutions file (`RESOLUTIONS_DRAFT*.csv`, including `W2/RESOLUTIONS_DRAFT.csv`) was read, and no other task's output was read.

- **CSV.** 164 body rows plus `#END,,,,,164`, with CRLF line endings. By check: DEPENDENCY 7, TERMINOLOGY 4, REUSED_EVIDENCE 5, SUBCLAIM_SPLIT 7, SHARED_SURFACE 141. By route: NO_ACTION 134, OWNER_DECISION 14, ENGINEERING_AUTHORITY 7, REVIEW 5, R5_RECORD_REPAIR 4. Every route is in the vocabulary.
- **SHARED_SURFACE is exactly once.** The population, OWNED capabilities with a non-empty Relations field, has 137 members out of 302 OWNED. The CSV has 137 per-capability rows with 137 distinct IDs, and the set equals the population (script check: `exact True`, no duplicates). Every owner and relation key resolves in the effective corpus: 0 missing and 0 relations without a key, over 278 relation answers (190 COVERS, 88 CONSTRAINS).
- **DEPENDENCY.** 94 `Dependencies.csv` files (1,487 rows) were compared row by row with DAG-010 `DependencyEdges.csv` (1,487 rows) on DependencyID, From, Target, Status, SatisfactionStatus, RequiredMaturity, Direction, DependencyType and TargetType. DAG-010 in the working tree is byte-identical to the freeze. 1,402 active edges were checked for evidence-file existence, and 975 active deliverable pairs were compared with the 8 ledger ownership-redirect pairs and the 180 shared-surface pairs.
- **REUSED_EVIDENCE is a screen, not a proof.** It tokenizes ImplementationEvidence and VerificationEvidence (normalizing `#L`, `:line` and `(…)`) into 1,985 distinct paths. It pairs an ALIGNED normative row (REQUIREMENT, ACCEPTANCE, EXCLUSION or DECLARED_STATE) with a row on the same path that is DOCUMENTED_UNIMPLEMENTED, IMPLEMENTED_DIFFERENTLY or caused by POSSIBLE_DEFECT, EVIDENCE_NOT_LOCATED or VERIFICATION_REMOVED. That gave 29,029 candidate pairs on 235 paths, of which 176 are code paths. Keeping only paths cited by at most 20 rows, pairs across deliverables and non-IMPLEMENTED_DIFFERENTLY opposites left 17 paths, all read by hand. Not covered:
  - paths cited by more than 20 rows (broad files such as the registers or `SOFTWARE_DECOMP.md`);
  - ContextRefs and Notes-only citations;
  - opposite meanings that do not show in the disposition, such as two ALIGNED rows with contradictory Notes.
- **SUBCLAIM_SPLIT is a screen plus hand review.** There are 426 `.sNN` sub-claims in 45 parent families; 28 families span deliverables. `SOW#CLM-nnn` families are numbered per deliverable, so they are not comparable across deliverables and were excluded. Comparable families screened: ABI (157 sub-claims in 81 deliverables, grouped by topic into SR, still-TBD and resolved-baseline), `SOW.sNN` (38), `MEMORY.sNN` (10), `AB.s01` (5), `STATUS.s01` (1), and the PKG-00 `AB#open-holds/purpose/normative-requirements` families (38). For 93 deliverables, the freeze `_CONTEXT.md` Architecture Basis Injection section was scanned for SR and still-TBD text that has no sub-claim. Topic grouping used summary keywords, so a sub-claim worded unusually may be misfiled.
- **TERMINOLOGY is a hand-directed screen** over Notes and ClaimSummary for JCS/RFC 8785 (132 rows), protected/invented review claims (619 normative rows), "settled in code/practice" (40 rows), "no located ruling" (79 rows), and layer-against-tier consistency on all 3,299 divergent rows. The layer/tier check found only two outliers, DEL-08-05 REQ-010 (IP_DATA at PROJECT_BASELINE) and DEL-01-01 AC-001 (CLAIMS at PROJECT_BASELINE), which T5B and T6 see in their partitions. The screen is not exhaustive for other terms.
- **Key check.** Every full claim key cited in the CSV exists in `CORPUS_CLAIMS.csv`. The only non-matches are formatting tokens (`DEL:COVERS` in the Evidence column, and `.r01/.r03` shorthand).

## R3 observations

These are observations, not corrections. The effective values stand.

1. **Still-TBD list not assessed in 13 deliverables** (S-04): DEL-03-07, 07-03..08, 08-03..06, 09-04 and 13-02. Each freeze `_CONTEXT.md` carries the same list, and it includes items since ruled. The ledgers mint no sub-claim, and the parent rows either cover the pin only or read the list as accurate (DEL-07-05, DEL-13-02). 52 other ledgers read the identical list as stale. DEL-03-01/02/03 already carry the same point as CONTESTED.
2. **SR statement not assessed** in DEL-07-03, 07-04 and 07-06 (S-02).
3. **ALIGNED on a retired edge.** DEL-07-08 SOW#CLM-005.r04 is ALIGNED on "DAG-002-E0844..E0846 present and ACTIVE", but E0846 is RETIRED in DAG-010. The dependency itself survives through DEL-13-04-D003, so the substance holds and the record does not. DEL-08-04 CLM-020.r06 counts 20 ACTIVE rows, where DAG-010 gives 19 (D-02).
4. **MBF JCS label unrecorded in its owning ledger.** `core/handoff/caepipe_mbf/package.py:734,743,790` labels sorted-compact bytes as JCS-compatible. This is recorded only in DEL-17-02 (REQ-007 Notes). DEL-17-04, which owns the MBF writer, carries no hash-basis row, and its architecture-basis parent is unsplit (WEAK).
5. **OwnerKeys understate divergence.** CAP-DATA-016 (DEL-08-04) omits CLM-011.r05, and CAP-DATA-009 (DEL-16-01) omits CLM-009.r01. In both cases the omitted row carries the defect on the same capability, so a reader of `CAPABILITY_COVERAGE.csv` alone sees an all-ALIGNED owner.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
