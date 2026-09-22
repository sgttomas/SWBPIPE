VERDICT: FINDINGS

# R3-PLAN-REVIEW — independent review of the R3 tool, plan and brief

Reviewer: fresh-context TASK (Type 2), run HELP-HUMAN-PIPING-20260921-RECONCILIATION, parent HELP_HUMAN Agent 0. I took no part in this run before this review.
Scope: commits `dfcce81e4..f4967ad04`. Reviewed bytes:
- `RUN/tools/synthesize_r3.py`, sha256 `061e7ff951e3a168904df60b0f03bc3e8ea174ed58c3f84f13bd3fc5220969ea`
- `RUN/R3_PLAN.md`, sha256 `e6fb6e67d858458fc7d23e320bd29943ae01426d71cb187e97e7da076c779c73`
- `AR/briefs/R3-TASK_brief.md`, sha256 `248500f30c11d07831f97165c235b0e43163d35a54a2d5924f31988be9626d44`

Method: I ran the tool's `--check`, then wrote my own read-only script. It walks `WAVES/` independently, skipping `superseded*`, and recomputes every R3 table. Everything it checked matched, except where a finding below says otherwise. No files were edited, no git writes were made and nothing touched the network.

Nothing is BLOCKING. The tool is correct and its outputs reproduce. Six SHOULD-FIX items affect what the 12 tasks will be told or will cover, so they should change before dispatch.

## Findings

1. **SHOULD-FIX — The per-task row counts in R3_PLAN are stale.** Four task counts in the plan are wrong, although both group totals are right. My counts come from the effective `CauseTag` on `Divergent=YES` rows of `CORPUS_CLAIMS.csv`:

   | Task | Plan says | Effective count |
   |---|---|---|
   | T4A | 777 | 781 |
   | T4B | 629 | 625 (339 + 86 + 200) |
   | T5A | 575 | 576 |
   | T5B | 584 | 583 (202 + 359 + 14 + 8) |

   T6 (521) and T7 (213) are correct. So are the sums 1,406 and 1,159. The T4A and T5A figures in the plan equal the *sealed* counts (777 and 575; the sealed divergent total is 3,273, not 3,299). The plan's counts were therefore taken, at least partly, before resolutions were applied. Each task must reconcile its coverage against its population, and the launch message will quote these numbers. With the plan's numbers, T4A, T4B, T5A and T5B will each report a coverage mismatch, or bend their output to fit it. Fix: correct the four numbers, or have each launch message give the filter only and the tool's own count.

2. **SHOULD-FIX — "Effective values" overstates what the tool applies; corrections in `OtherCorrections` are invisible.** The tool substitutes only the five value fields. The combined resolutions file has 582 rows with a non-empty `OtherCorrections`, and none of it reaches `CORPUS_CLAIMS.csv`. Examples by count of rows naming each field:

   | Field named in `OtherCorrections` | Rows | By class |
   |---|---|---|
   | `AuthorityNeeded` | 58 | FIRM 23, WEAK 11, RESOLVED_PAIR 3, FIELD 7, CONTESTED candidates 14 |
   | `CanonicalSituation` | 10 | |
   | `FindingGroup` | 14 | |
   | `RemainingWork` | 16 | |
   | `VerificationEvidence` | 125 | |

   For instance, `DEL-07-01:SOW#CLM-005.r05` carries "CanonicalSituation CP-10; AuthorityNeeded OWNER". `CORPUS_CLAIMS.csv` still shows the sealed `AuthorityNeeded`, `CanonicalSituation` and `FindingGroup` for such rows.

   The brief says effective values are "the sealed values with adopted resolutions applied". Tasks will take owning authority (rule 4) and routing from `AuthorityNeeded`, and T9 relies on `RemainingWork` and `VerificationEvidence`. Fix, either or both:
   - add an `OtherCorrections` column (or at least an effective `AuthorityNeeded`, `CanonicalSituation` and `FindingGroup`) to `CORPUS_CLAIMS.csv`;
   - state in the brief that only the five value fields are substituted, and that a task must read `OtherCorrections` for every key where `ResolutionClasses` is non-empty.

   This does not affect the partition. Disposition and CauseTag are applied correctly.

3. **SHOULD-FIX — Two R3 obligations recorded in the ruled conventions are not assigned.**
   - (a) CONVENTIONS C6(e), ruling item 3, says: "R3 clusters these findings as one class for a single R4 ruling". This means the DEC-101 rename residue in SOW files, cause `RENAME_OR_IDENTITY`, which is 86 effective rows. T4B mixes this cause with REPRESENTATION_MIGRATED and RECORD_DRIFT. Neither the plan nor the brief tells T4B to keep it as exactly one class. W2_GATE_ASSESSMENT also notes that rename residue in ISSUED text "is routed twice".
   - (b) Rule F7 (W1_GATE_RULING) requires `PRODUCT_CALLER: NONE` in Notes "for R3 clustering". 666 rows carry the marker: 646 ALIGNED, 19 PARTIALLY_IMPLEMENTED and 1 STALE_REVIEW_OR_EVIDENCE. Because ALIGNED rows are not divergent, no task examines 646 of them.

   Fix: add (a) to T4B's launch scope. Assign (b), for example to T3, to T9, or to a new task using a spare slot.

4. **SHOULD-FIX — Method R3 coverage has gaps (question 4).** Method R3 names cross-package dependencies, inconsistent terminology, and reused evidence with incompatible meanings. No task is assigned any of them:
   - Cross-package dependencies: nothing compares declared deliverable dependencies with the cross-package evidence. T6 `OWNERSHIP_ELSEWHERE` is only a proxy.
   - Terminology: T8 covers unit vocabulary only.
   - Reused evidence: nothing groups `ImplementationEvidence` or `VerificationEvidence` paths cited by several deliverables with different dispositions.

   The `.sNN` cross-ledger blind spot, flagged in W3_ASSESSMENT "for R3", is listed only as a method limit (plan step 3). Yet four verifiers found real splits there by hand. The plan says the three spare slots have "no further independent work to fill them". A T11 covering cross-ledger consistency (shared evidence, `.sNN` sub-claims across ledgers, terminology, dependencies) would fill one slot and close the gap. At minimum, the plan should name who covers each of these method items, or record their omission as a disclosed limit.

5. **SHOULD-FIX — "T1–T3 partition the capabilities by coverage status" is not true as written.** T1 to T3 cover 73 + 57 + 166 = 296 of 598 capabilities. Status OWNED (302) is assigned to no task. That may be right: one CLAIMED_BY owner is the healthy case. But the exactly-once proof in COVERAGE_AND_QA needs the exclusion stated, with its reason. Two OWNED subsets are worth at least a scripted look:
   - 137 OWNED capabilities also have COVERS or CONSTRAINS answers from other deliverables. These are shared surfaces, which method R3 names.
   - OWNED capabilities whose owning key is itself divergent, or whose owner came only through SAMPLE routing.

   Fix: reword the plan to "T1–T3 cover every non-OWNED status; OWNED (302) is excluded because …", or add a light OWNED check to T3.

6. **SHOULD-FIX — The routing vocabulary lacks REVIEW and NO_ACTION.** Rule 4's owning-authority list includes REVIEW and NONE. The routing list is R5_RECORD_REPAIR, SCOPE_CHANGE_HANDOFF, CODE_FIX_CANDIDATE, ENGINEERING_AUTHORITY and OWNER_DECISION, so it has no route for either of them. Method R4 explicitly distinguishes "WORKING_ITEMS (workflow: review)". The 14 ACCEPTED_DIVERGENCE rows already carry a ruling with a trigger (C6(e)). They need a route such as NO_ACTION or TRACK_TRIGGER, or tasks will invent new owner decisions for them. The `Route` column in the T8 and T9 CSVs has no stated vocabulary at all. Fix: add `REVIEW` and `NO_ACTION` (or `ALREADY_AUTHORISED`), and say the same enum applies to the T8 and T9 `Route` columns.

7. **MINOR — The T1–T3 classification set has no class for partial coverage with an unowned remainder.** PARTIAL_ONLY has 105 capabilities, 44 of them with several PARTIAL owners. Where the partial owners together do not cover the capability, no class fits. SHARED_OK means the split is complementary, and PRODUCT_UNOWNED means no deliverable owns it at all. Suggest adding `PARTIAL_REMAINDER_UNOWNED`, or telling tasks to use PRODUCT_UNOWNED and name the uncovered part.

8. **MINOR — T10's July output could reach other tasks.** The brief limits reading the July concordance to T10. It does not forbid reading other tasks' outputs under `R3/TASKS/`, and T10 writes its outputs there while the other tasks are running. Add: "Do not read other tasks' files under `R3/TASKS/`."

9. **MINOR — The `OwnerKeys` delimiter is ambiguous.** In `CAPABILITY_COVERAGE.csv`, `OwnerKeys` joins entries with `;`. A reverse `ClaimKey` may itself be a `;` list (B2), and 332 owning reverse rows are. So `DEL-a=K1;K2;DEL-b=K3` does not parse cleanly. I checked all 554 key tokens and every one exists in the forward ledgers, so no value is wrong; only machine parsing is affected. Suggest `|` between entries.

10. **MINOR — Brief hygiene.**
    - Tell tasks to ignore `WAVES/W2/RESOLUTIONS_DRAFT.csv`, `WAVES/W3/RESOLUTIONS_DRAFT_PART*.csv` and `WAVES/W3/CROSS_PACKAGE/RESOLUTIONS_MERGED_DRAFT.csv`.
    - Tell tasks to run their scripts with `PYTHONDONTWRITEBYTECODE=1`. Importing run tools writes `tools/__pycache__/`, which is outside their write boundary.
    - State the divergent filter explicitly as `Divergent=YES` and `CauseTag ∈ {…}` on `CORPUS_CLAIMS.csv`, so that no task recomputes "divergent" differently.

11. **MINOR — T8's wording could read as a ruling.** The plan says "Where they disagree, T8's reading governs for R4." Clarify that it governs only Agent 0's packet framing. It is a proposed reading and changes no row value. The brief's rule 1 already implies this, but the plan text is what Agent 0 relays.

## Q1 — Tool correctness

- **Reproduction.** `PYTHONDONTWRITEBYTECODE=1 python3 RUN/tools/synthesize_r3.py --run-dir RUN --check` prints "CHECK PASS: all R3 synthesis outputs reproduce".
- **Inputs.**
  - An independent `os.walk` that skips `superseded*` finds 102 forward and 102 reverse ledgers. The tool's glob `DEL-*/DEL-*_forward.csv` cannot match `DEL-*/superseded_1/…` (depth), and it asserts that exclusion.
  - Three superseded folders exist: DEL-03-07, DEL-13-02 and DEL-15-02. None is read.
  - The combined resolutions file (599 rows over 597 keys) equals the multiset union of the W1 (47), W2 (122), W3 (425) and CROSS_WAVE (5) files, with 0 differences.
- **Substitution.**
  - For all 9,889 rows, my recomputation of the five effective fields, the five `Sealed*` fields, `ResolutionClasses`, `AuthorityNeeded`, `LifecycleState`, `CanonicalSituation` and `ClaimType` matches with 0 mismatches.
  - Only rows with a non-empty Disposition set values: OWNER_CONFIRMED 81, FIRM 26, RESOLVED_PAIR 26 and WEAK 24. This is the validator's `load_resolutions` filter.
  - FIELD, CONTESTED and OBSERVED rows stay visible through `ResolutionClasses`.
  - The two duplicate keys, flagged in W3_ASSESSMENT, are handled: both rows are listed and neither sets values.
- **BaselineClass.** Substituting BaselineClass as well is right for an effective view. The validator's `CONSISTENCY_FIELDS` omits BaselineClass because batch mode compares cross-ledger consistency, and CP-04 handles BaselineClass separately. The omission is not a claim that resolutions leave BaselineClass alone. Resolution rows record the verifier's full values. The only rows where substitution blanks a sealed value are four rows resolved to ALIGNED: `DEL-01-01:SOW#CLM-010/REQ-01-01-05`, `DEL-09-04:CONTEXT`, `DEL-09-05:CONTEXT` and `DEL-11-01:SOW#CLM-028`. There, CauseTag, AuthorityTier and BaselineClass become empty, which is the sealed ALIGNED convention (Part D; W3_ASSESSMENT's post-review correction). The limitation is the unapplied `OtherCorrections` (finding 2), not the choice of fields.
- **Reverse-answer mapping.**
  - The manifest has 6,086 rows (AREA 5,565, SAMPLE 521) with unique `(PackageID, RoutedID)` pairs. Every `RoutedID` differs from its `CapabilityID`, so the mapping is needed, and the tool does it through `(package, RoutedID)`.
  - Every reverse file's row count equals its package's routed count, so none is incomplete.
  - Answer totals: NOT_MINE 34,359, COVERS 389, CLAIMED_BY 319, PARTIAL 178, CONSTRAINS 141 and UNKEYED 57.
  - Every answered capability exists in IMPLEMENTATION_SURFACES, and none is NOT_ROUTED.
  - For 25 random capabilities, `RoutedTo`, `AreaRouted` and `SampleRouted` match.
- **Coverage statuses.** My independent status function agrees on all 598 capabilities: OWNED 302, OWNED_SHARED 15, PARTIAL_ONLY 105, OWNED_UNKEYED 45, UNMAPPED 63, UNMAPPED_RELATION_ONLY 67 and DUPLICATE_OWNERSHIP 1 (CAP-VIEW-017: DEL-07-01 and DEL-07-05). The precedence is sensible:
  - DUPLICATE_OWNERSHIP needs two or more CLAIMED_BY answers.
  - With exactly one CLAIMED_BY, the status is OWNED_SHARED if any other owning answer exists, otherwise OWNED.
  - OWNED_UNKEYED needs every owning answer to be UNKEYED.
  - UNKEYED together with PARTIAL gives PARTIAL_ONLY (10 capabilities). The UNKEYED answer stays visible in `Owners`, and T3 takes both statuses, so nothing is lost.
  - No capability has CLAIMED_BY plus UNKEYED.
  - The relation-only split (UNMAPPED_RELATION_ONLY) is useful.
- **Remaining.** All 151 flagged rows are `STATUS#remaining/*` keys: 124 DECLARED_STATE and 27 REMAINING_WORK. No other key is flagged. Per-deliverable counts equal `DELIVERABLE_INVENTORY.RemainingItems` for all 102 deliverables: 65 with items and 37 with 0. `REMAINING_CENSUS.RemainingRows` also matches.
- **Divergent.** Every disposition except ALIGNED, NOT_ASSESSED and COVERED_BY_CHILDREN counts as divergent, which gives 3,299 rows. No divergent row lacks a CauseTag, and no quiet row carries one.
- **ACCEPTED_DIVERGENCE.** Its 14 rows count as divergent. That is correct for R3: a C6(e) accepted divergence is a scheduled catch-up whose trigger is still pending, and it must stay visible for R4 and R5. The rows fall into T6 (9, OWNERSHIP_ELSEWHERE) and T5B (5). The brief should give them a no-new-decision route (finding 6).
- **Spot checks.** Beyond the full recomputation above, I checked 25 random CORPUS_CLAIMS rows against the sealed ledger and inventory (deliverable and package), and 25 random coverage rows. CLUSTER_MATRIX matches all 73 cells, totalling 3,299. PACKAGE_SUMMARY matches all 19 rows and every disposition column. All 554 OwnerKeys tokens exist in the forward ledgers.

## Q2 — Partition

- **Row partition.** The CauseTag sets for T4A–T7 are disjoint and cover all 19 causes present, so every divergent row falls in exactly one task. The totals are right: 1,406 + 1,159 + 521 + 213 = 3,299. Four of the per-task counts are wrong (finding 1).
- **Unmapped capabilities.** 130 in total, split correctly by area: T1 73, T2 57 (SHELL 38, WSUI 19), none elsewhere.
- **T3.** Takes 166 capabilities: 45 + 1 + 105 + 15.
- **Uncovered statuses.** OWNED (302) is uncovered and the plan does not say so (finding 5). OWNED_SHARED is covered by T3.
- **Uncovered rows.** 646 ALIGNED `PRODUCT_CALLER: NONE` rows (finding 3b). Aligned rows generally are out of scope, which is reasonable.
- **T9 and T10.** As the plan says, they are cross-cutting and add no row classes.

## Q3 — Brief adequacy and safety

- **Authority rules** are right: effective values are accepted, there is no re-disposition, `R3_OBSERVATION` replaces correction, routing is proposed only, and owner, engineering and scope-change decisions stay with those authorities.
- **Stated fences.** The write boundary is `R3/TASKS/` plus a scratch folder that is deleted before return. The brief bars git, the network and writes to FREEZE, and forbids builds and tests. It states the claim fence (F-PIP-2, DEC-081) and the DEC-043 exclusion, and bars quoting protected content.
- **Formats.** The CSV formats are unambiguous. Class metadata lives only in Markdown. A machine-readable class table would help Agent 0's integration, but that is optional.
- **Placeholders.** `{REPO}`, `{FREEZE}` and `{TASK}` depend on the launch message. That is acceptable if Agent 0 fills them.
- **Adequacy.** For a context-free agent the brief is adequate apart from findings 2, 6, 7, 8 and 10.
- **Scope.** Nothing invites repair or ruling. T8's "proposed reading" wording should be kept clearly non-ruling (finding 11).
- **July ledger.** Limiting it to T10 as a cross-check is consistent with the approved PLAN.md line 72: "July ledger hidden from workers; R3 cross-check only". It is also consistent with the owner direction, which excludes the July *plan*, not the July ledger as a cross-check. Finding 8 would close the one leak path.

## Q4 — Method conformance

| Method R3 item | Where the plan covers it |
|---|---|
| Duplicate or incompatible ownership | T3 |
| Shared surfaces | T3, but OWNED plus relations is not examined (finding 5) |
| Unmapped implementation | T1, T2 |
| Inconsistent decisions | T8 (contested clusters) |
| Stale verification and validation, lifecycle mismatches, Remaining defects | T9 |
| Cross-package dependencies | Gap (finding 4) |
| Terminology beyond unit vocabulary | Gap (finding 4) |
| Reused evidence with incompatible meanings | Gap (finding 4) |
| `.sNN` cross-ledger consistency | Recorded only as a limit (finding 4) |

The convention-level R3 obligations in C6(e) and F7 are also missing (finding 3).

## Q5 — Containment

- **Changed files.** `git diff --name-only dfcce81e4..f4967ad04` lists only paths under RUN (the tool, `R3/*`, `R3_PLAN.md`, one appended PHASE_OPEN line in `RUN_STATE.jsonl`) and the AR brief. Nothing outside RUN or AR changed.
- **Protected records.** No changes to `WAVES/` ledgers or resolutions, the inventory, deliverables, code or the DAG.
- **Tool behaviour.** It writes only `RUN/R3/`, and in `--check` mode it writes nothing.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
