# R3 plan — cross-package synthesis

R3 reconciles the 102 accepted R2 ledgers with each other. It covers
duplicate or incompatible ownership, shared surfaces, unmapped implementation,
inconsistent decisions and terminology, stale verification and validation,
lifecycle mismatches and Remaining-state defects. It prepares the R4 decision
packets. Per the method (R3), synthesis changes no deliverable, dependency,
code, lifecycle state or DAG. It writes only under `R3/`, plus the
orchestration record.

## Basis

- **Accepted inputs.** The sealed forward and reverse ledgers under
  `WAVES/W*/PKG-*/DEL-*/`, excluding `superseded_<n>/`. They are read with the
  four adopted resolutions files, combined in
  `WAVES/CROSS_WAVE/ALL_WAVES_RESOLUTIONS_COMBINED.csv`. The verification
  reports and the three wave assessments are also inputs: W1 gate assessment
  and ruling, W2 gate assessment, and `WAVES/W3/W3_ASSESSMENT.md`.
- **Deterministic layer.** `tools/synthesize_r3.py` builds the tables in
  `R3/`, and `--check` reproduces them byte for byte:
  - `CORPUS_CLAIMS.csv`: 9,889 claim rows, sealed and effective values.
  - `PACKAGE_SUMMARY.csv`.
  - `CLUSTER_MATRIX.csv`: 3,299 divergent effective rows by cause, tier and
    disposition.
  - `CAPABILITY_COVERAGE.csv`: 598 capabilities.
  - `REMAINING_CENSUS.csv`: all 102 deliverables; 151 Remaining rows,
    matching the inventory.

  Every figure in an R3 output must reproduce from these rows.
- **Capability ownership terms.** Owning answers are `CLAIMED_BY`, `PARTIAL`
  and `UNKEYED`. `COVERS` and `CONSTRAINS` are relations. The coverage
  statuses are: OWNED 302, OWNED_SHARED 15, PARTIAL_ONLY 105, OWNED_UNKEYED
  45, UNMAPPED 63, UNMAPPED_RELATION_ONLY 67 and DUPLICATE_OWNERSHIP 1.
- **July ledger.** The July concordance
  (`../DELIVERABLE_CONCORDANCE_2026-07-11_1305/`) was hidden from R2 workers.
  R3 uses it only as a cross-check (T10), never as evidence or authority.

## Tasks (TASK, Type 2, read-only, one output pair each, under `R3/TASKS/`)

| ID | Scope | Output |
|---|---|---|
| T1 | Capabilities with status UNMAPPED or UNMAPPED_RELATION_ONLY in areas CHECKS, COREB, COREC, DATA, DOCS, FEATB, FEATC, PHYS, SOLVER and VIEW (73) | `T1_UNMAPPED.csv` + `T1_NOTES.md` |
| T2 | The same statuses in areas SHELL and WSUI (57) | `T2_UNMAPPED.csv` + `T2_NOTES.md` |
| T3 | OWNED_UNKEYED, DUPLICATE_OWNERSHIP, PARTIAL_ONLY and OWNED_SHARED capabilities (166) | `T3_OWNERSHIP.csv` + `T3_NOTES.md` |
| T4A | Divergent rows with cause BASIS_POINTER_STALE (781), record and pointer hygiene | `T4A_CLASSES.csv` + `T4A_CLASSES.md` |
| T4B | REPRESENTATION_MIGRATED, RENAME_OR_IDENTITY or RECORD_DRIFT (625). The 86 RENAME_OR_IDENTITY rows stay one class, for a single R4 ruling (CONVENTIONS C6(e)) | `T4B_CLASSES.csv` + `T4B_CLASSES.md` |
| T5A | DOC_BEHIND_CODE (576), documents behind the product | `T5A_CLASSES.csv` + `T5A_CLASSES.md` |
| T5B | EVIDENCE_OVERTAKEN, SCOPE_REDIRECTED_BY_RULING, SCOPE_GREW_BY_DIRECTION or CONTRACT_VERSION_ADVANCED (583) | `T5B_CLASSES.csv` + `T5B_CLASSES.md` |
| T6 | PARTIAL_SLICE, NOT_STARTED, DEFERRED_BY_RULING or OWNERSHIP_ELSEWHERE (521), implementation gaps | `T6_CLASSES.csv` + `T6_CLASSES.md` |
| T7 | POSSIBLE_DEFECT, VALIDATION_GAP, VERIFICATION_REMOVED, EVIDENCE_NOT_LOCATED, AUTHORITY_UNCLEAR or OTHER (213), defects, evidence and authority | `T7_CLASSES.csv` + `T7_CLASSES.md` |
| T8 | The contested corpus clusters in `W3_ASSESSMENT.md`: SR-1, DEC-009, unit vocabulary, F1 on CONTEXT, export plan, tier of in-scope REQs, acceptance-workflow cause. For each, a proposed reading with its evidence, or an R4 routing where only the owner can decide | `T8_CLUSTERS.md` + `T8_ROWS.csv` |
| T9 | Lifecycle and Remaining: REMAINING_STATE_MISMATCH, LIFECYCLE_REASSESSMENT_REQUIRED, the Remaining census, lifecycle state against evidence (including PKG-00 SEMANTIC_READY and the ISSUED DEL-01-01), and stale verification or validation (VERIFIED_NOT_VALIDATED, STALE_REVIEW_OR_EVIDENCE) | `T9_LIFECYCLE.csv` + `T9_LIFECYCLE.md` |
| T11 | The method R3 checks no other task covers. **Cross-package dependencies:** each deliverable's `Dependencies.csv` at the freeze, against the ledgers and DAG-010. **Terminology and decisions used inconsistently across packages.** **Evidence reused with incompatible meanings:** the same evidence path cited for opposite dispositions. **Cross-ledger splits on minted `.sNN` sub-claims**, which batch mode cannot see (for example the architecture-basis and SR-1 splits). **Shared surfaces:** OWNED capabilities that other deliverables COVER or CONSTRAIN (137), checked for incompatible readings | `T11_METHOD.csv` + `T11_METHOD.md` |
| T12 | The 666 rows marked `PRODUCT_CALLER: NONE` (F7), of which 641 are not divergent: code exists and is tested, but no product path calls it. Cluster them by package and area, name the unreached engines, and propose routing | `T12_UNREACHED.csv` + `T12_UNREACHED.md` |
| T10 | July cross-check: our effective dispositions against the July `CLAIM_CONCORDANCE.csv`, at the deliverable and topic grain (resolved since July, persisting, new) | `T10_JULY.csv` + `T10_JULY.md` |

**Partition and exactly-once coverage.**
- T4A–T7 partition the divergent rows (`Divergent = YES` in
  `CORPUS_CLAIMS.csv`) by effective CauseTag. Every divergent row has a
  cause tag, so together they cover all 3,299 divergent rows exactly once
  (781 + 625 + 576 + 583 + 521 + 213). Each classes CSV maps every row in its
  partition to one class.
- T1–T3 cover every capability whose coverage status is not OWNED: 130 + 166
  = 296 of 598.
  - The 302 OWNED capabilities have exactly one owning deliverable, affirmed
    in that deliverable's verified reverse pass, so their ownership is not
    reassessed.
  - Where other deliverables COVER or CONSTRAIN an OWNED capability (137 of
    them), T11 checks for incompatible readings.
  - NOT_ROUTED is empty; every capability was routed.
- T8 covers the rows named by the clusters. A row may appear in both T8 and
  T4A–T7. Where they disagree, T8's reading shapes Agent 0's R4 decision
  packets. It changes no row value.
- T9, T10, T11 and T12 are cross-cutting views. They add no classes to the
  T4A–T7 partition.

**Class contents.** A class (T4A–T7) records:
- a short name and description;
- its CauseTag, AuthorityTier and disposition signature;
- its full claim population (keys);
- the packages it touches;
- the owning authority for a decision: OWNER, ENGINEERING, REVIEW, SCOPE_CHANGE, HELPS_HUMANS, EXTERNAL or NONE;
- the recommended routing: R5 record repair, scope-change handoff, code-fix brief candidate, engineering authority, owner decision, review, or no action (for example accepted divergences);
- the exact on-ruling mechanism;
- the risk if left unrepaired.

Aim for about 3–8 classes per task. Keep exceptions visible as their own
classes.

**Concurrency.** All 14 tasks run together: Agent 0 plus 14 tasks is 15 live
agents. Once they are running, the remaining slot is kept for:
- a re-run of a failed task;
- the independent review, once the tasks finish.

There is no further independent work to use it for (Direction 6: a stated
reason).

## Agent 0 integration (after T1–T12)

1. Check each task output: exactly-once coverage, keys exist in
   `CORPUS_CLAIMS.csv`, and counts reproduce.
2. Write `R3/R3_SYNTHESIS.md`. It holds the class table (about 15–20 corpus
   classes plus exceptions, each with its full population), the final
   unmapped set, the ownership findings and the contested-cluster outcomes.
3. Write `R3/COVERAGE_AND_QA.md`: exactly-once proofs, reproduction record,
   spot checks and method limits. The limits include the `.sNN` batch blind
   spot, sampled verification, the tier hint on DEL-15-02, and first-pass
   error rates.
4. Write the separate handoffs. Each is a proposal and none is executed:
   - `R3/SCOPE_CHANGE_HANDOFF/`: deliverables to retire, merge or create, and
     ownership assignments.
   - `R3/CODE_FIX_BRIEF_CANDIDATES/`: candidate briefs, not executed.
   - `R3/ENGINEERING_AUTHORITY/`: items for engineering authority.
5. Write `R4/DECISION_PACKETS/`: one packet per owner-decision class, with
   options, evidence and reliability, affected claim IDs and packages, risks,
   recommended routing and the exact on-ruling mechanism. It also carries the
   collected owner items from the W2 and W3 assessments.
6. Independent fresh-context review of R3 and R4, then the R3/R4 PR. The R4
   owner gate follows. The run stops at R4 under D-73; R5 and R6 need a
   separate owner authorization.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
