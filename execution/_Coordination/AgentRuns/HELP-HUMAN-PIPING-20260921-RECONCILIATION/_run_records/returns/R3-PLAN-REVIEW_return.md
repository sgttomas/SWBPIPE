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

## Backcheck 1

**Backcheck verdict: PASS.** Every original finding is resolved. The changes introduce three MINOR items (B1-1 to B1-3). None needs to change before dispatch, so Agent 0 may fold them into the launch messages.

Scope: commit `b3bfd0ebc` (`f4967ad04..HEAD`). Reviewed bytes:
- `tools/synthesize_r3.py`, sha256 `bf3487d6ae537bedc335173ad69ee27a109f8172449931d4de3081d01c428eab`
- `R3_PLAN.md`, sha256 `ee80d371ac3b1f709ba0a79823201385fe1af6c6584fe009f45e3b4c24573cdb`
- `briefs/R3-TASK_brief.md`, sha256 `4463e540cb71c212816dfd02897391e4f81f7ca9134867989039f0f737f6a309`

The diff also carries this return file, committed unchanged at sha256 `2f03902f…`, and three RUN_STATE lines: LAUNCH, RETURN and FIX. Nothing else outside RUN or AR changed.

### Tool

- **Reproduction.** `--check` prints "CHECK PASS". Only three outputs changed: CORPUS_CLAIMS, CAPABILITY_COVERAGE and SYNTHESIS_STATS. PACKAGE_SUMMARY, CLUSTER_MATRIX and REMAINING_CENSUS hashes are unchanged, and the divergent count is still 3,299.
- **`OtherCorrections`.** My script rebuilt it for all 9,889 rows (`[Class] text`, joined with ` || `, in resolution-file order) with 0 mismatches. 580 claim rows carry it: 582 resolution rows minus the two duplicate keys. No correction text contains `||`, so the separator is unambiguous.
- **`ProductCallerNone`.** 666 rows, 0 mismatches against a substring search of the sealed Notes. 25 of them are divergent, so the "641 not divergent" in the plan and SYNTHESIS_STATS is right. (My earlier "646 ALIGNED" counted sealed dispositions; effective resolutions account for the difference.)
- **`OwnerKeys`.** Entries are now separated by ` | `, and a single owner's keys stay `;`-joined. The brief states this.

### Original findings

1. **Counts:** RESOLVED. The plan now gives 781, 625, 576 and 583, which match my effective counts. It states the filter as `Divergent = YES` by effective CauseTag.
2. **`OtherCorrections`:** RESOLVED, in both the tool and the brief. The brief states exactly which five fields are substituted, says every other correction is not applied, and tells tasks to take it into account, including for AuthorityNeeded.
3. **Unassigned obligations:** RESOLVED.
   - (a) T4B now keeps the 86 RENAME_OR_IDENTITY rows as one class under C6(e).
   - (b) T12 covers all 666 `PRODUCT_CALLER: NONE` rows, one CSV row each.
4. **Method gaps:** RESOLVED. T11 covers dependencies, terminology and decisions, reused evidence, `.sNN` splits and shared surfaces. Each has a `Check` value.
5. **OWNED exclusion:** RESOLVED. The plan now gives the reason (a single owner affirmed in a verified reverse pass). The 137 OWNED capabilities with relations go to T11. My count confirms 137.
6. **Routes:** RESOLVED. The vocabulary now has seven routes, including REVIEW and NO_ACTION, and applies to "every Route or routing field". The plan's class contents match.
7. **Partial ownership class:** RESOLVED. PARTIAL_UNOWNED_REMAINDER is added.
8. **Reading other tasks' files:** RESOLVED. Rule 8 forbids it and keeps the July material to T10.
9. **OwnerKeys delimiter:** RESOLVED.
10. **Brief hygiene:** RESOLVED. The brief now says to ignore draft files, sets PYTHONDONTWRITEBYTECODE and states the exact divergent filter. One gap remains in the draft pattern (B1-1).
11. **T8 wording:** RESOLVED. T8's reading "shapes Agent 0's R4 decision packets. It changes no row value."

### New items

- **B1-1 MINOR — The draft-file pattern misses one file.** The brief says to ignore `RESOLUTIONS_DRAFT_*.csv`. The W2 draft is named `WAVES/W2/RESOLUTIONS_DRAFT.csv`, with no suffix, so the pattern does not match it. The risk is low, because the brief also lists the four adopted files explicitly and says "ignore the draft … files". Suggest `RESOLUTIONS_DRAFT*.csv`.
- **B1-2 MINOR — T11's inputs need their locations stated.**
  - The plan names "DAG-010" without a path. It is `projects/chirality-piping/execution/_DAG/DAG-010/`, under the freeze.
  - Only 94 `Dependencies.csv` files exist in the piping tree for 102 deliverables. I counted in the working tree, which RUN_STATE records as having no piping product change since the freeze. T11 should be told to report deliverables with no dependency file rather than treat the absence as an error.
  - T11 carries five checks. It is workable because it has no exactly-once duty, but the launch message should say that REUSED_EVIDENCE and SUBCLAIM_SPLIT are scripted candidate screens that report their method and limits, not exhaustive proofs.
  - T12's `Engine` and `Area` columns are not defined. Suggest `Area` = the IMPLEMENTATION_SURFACES area, and `Engine` = the module or crate path at the freeze.
- **B1-3 MINOR — Stale plan wording.**
  - The heading "Agent 0 integration (after T1–T10)" should read T1–T12. Step 1's output checks should include T11 and T12.
  - The concurrency paragraph now reserves a single slot but still ends "no further independent work to fill them". 15 live agents under the owner's 16 maximum, with one slot held for re-runs and review, is consistent with Direction 6.

### Brief sufficiency for all 14 tasks

- **T1–T3:** population, statuses, classification set and CSV format are all given.
- **T4A–T7:** filter, class contents, routes and exactly-once check are all given.
- **T8:** cluster list (plan), CSV format and route vocabulary.
- **T9:** scope (plan), CSV format and routes.
- **T10:** July path limited to T10, a cross-check only, with a CSV format.
- **T11 and T12:** checks, CSV formats and routes. Only B1-2 is missing.

The common rules (authority, observations, visibility, evidence, DEC-043, claim fence, writes, independence) apply to every task. The brief is sufficient for a context-free agent, provided each launch message states the task's exact filter and count, and supplies `{REPO}` and `{FREEZE}`.

## Integration review

**Integration verdict: FINDINGS.** Nothing is BLOCKING. Six SHOULD-FIX items should change before the seven drafting agents are dispatched.

Scope: commits `fdaedd4f9..5e89655fd`. Reviewed bytes:
- `tools/index_r3_classes.py`, sha256 `6976ae85e3767e2aa0c925df93d9c2993e560b031cd8766c25e9f78d570257e2`
- `R3_INTEGRATION_TOPICS.md`, sha256 `53fb08176f7057b5e1e1c9fdc4d0850771643a3cc20e1d5dfbd53765aa8beb8c`
- `briefs/R3-INTEGRATION_brief.md`, sha256 `fee811e815c821d23bdcbf9d68a8f82aa2eddeffd428b826522e68c4899247fb`

As before, I used only read-only scripts, with scratch files under the session scratchpad.

### Findings

I1. **SHOULD-FIX — The topic file is not a complete crosswalk of owner-decision candidates.** Its header says every candidate from R2/R3 is assigned. I compared it with the task notes' "Owner-decision candidates" lists and the W2/W3 owner lists. The following are either not named in any topic or are only arguably covered.
   - **Task lists:**
     - T1 decision 7 (owner of the canonical-JSON crate: CAP-COREB-040, CAP-COREB-041, CAP-DATA-046);
     - T2 decision 4 (CAP-SHELL-048: DEL-08-01 now, or DEL-02-05 later);
     - T3 items 7 (FEATC-004/005 key home for annex row 21), 8 (PHYS-007: DEL-03-08 against the product-physics owner; arguably B1) and 10 (COREB-039, COREC-044 runner orchestration owner, DOCS-008 `MANIFEST.json`, COREB-012);
     - T9-C12 (SRE-5, review and lifecycle states across 82 deliverables; A5 cites only T9-C07);
     - T11 SS-02 (runtime edit contract FG-DEL-16-01-01, CAP-DATA-009 and CAP-COREB-020; arguably B3) and SS-03 (package manager CAP-CHECKS-021; arguably A3). Both carry route OWNER_DECISION in `T11_METHOD.csv`.
   - **W2 list:**
     - the DEL-03-07 question of whether the draft policy `IP_AND_DATA_BOUNDARY.md` §4 counts as governing;
     - "unit safety only at metadata grain";
     - the product section and mass routine;
     - the five W2 convention questions beyond SR-1: schema-only SOW once product physics lands; envelope round-trip against AB-00-04; CP-09 on VER-001; import-checker CONSTRAINS against COVERS. None of these appears in T8, C6 or C7.
   - **W3 list:**
     - desktop store root, unmatched-classification list and REXC-CON-002 (B12 names "private-library storage roots" only);
     - the frozen-contract rename `openpipestress_jcs_ijson_v1`, which could fall to A2, A4 or B12;
     - the product and code items that are not obviously owner calls: empty rule-pack and library references; PCF writes 0 for a missing coordinate; the desktop millimetre assumption; the missing `unit_system_disclosure.json`; the MBF diagnostic that cannot fire; the `TBD`/`tbd` token.

   Rule 3 of the brief ("report UNASSIGNED") catches omissions only if a writer happens to come across them. Fix: add an explicit crosswalk, one line per W2/W3 bullet and per task candidate. Each line should give one topic, or a handoff (H1–H4), or "resolved in R2/R3" with its reference.

I2. **SHOULD-FIX — Some candidates are double-assigned, and split classes have no row split.**
   - **DEL-01-01:SOW.** A4 "picks the one route for `DEL-01-01:SOW`". A6 covers "the double-routed SOW row". That is the same decision in two packets, which is exactly the double routing T4B warns about (T4B obs 4). Assign it to one topic and cross-reference it from the other.
   - **SR-1.** A5 cites T8 SR-1, and C7 carries "SR-1 cause RECORD_DRIFT". State the boundary: A5 takes the lifecycle target and restatement, C7 takes cause-reading confirmation only.
   - **Acceptance workflow.** C7 (cause DEFERRED_BY_RULING) and B12 (selection of the OI-007 workflow) overlap in the same way. State the boundary.
   - **Row split.** T4A-C06 (14 rows) is split across A1, A7 and A10. T5B-C07, T5A-C05, T6-C04 (D1–D15) and T7-C06/C07 are also split. Brief rule 2 says a packet "names every affected class and its full row count". Read literally, each packet claims the whole class, and Agent 0 cannot prove that the owner decisions account for every row exactly once. Fix: for each split class, give each topic's portion as a key list or exact filter, and require the portions to sum to the class count.

I3. **SHOULD-FIX — H1–H4 do not cover all non-owner routes (question 4).** My counts by route over `CLASS_ASSIGNMENTS` and the task CSVs:

   | Source | Route | Rows | Status |
   |---|---|---|---|
   | T8 | CODE_FIX_CANDIDATE | 17 | 5 missing from H2 (see below) |
   | T8 | R5_RECORD_REPAIR | 88 | 83 missing from H4 (see below) |
   | T12 | R5_RECORD_REPAIR | 1 | Missing from H4 |
   | T9, T11, T12 | ENGINEERING_AUTHORITY, REVIEW | | Covered by H3 |
   | T11 | R5_RECORD_REPAIR | 4 | Covered by H4 |
   | T12 | CODE_FIX_CANDIDATE | 17 | Covered by H2 |

   - **Missing from H2:** 5 of the T8 code-fix rows are not code-fix rows in `CLASS_ASSIGNMENTS`. Two are F1_ON_CONTEXT rows that are not divergent. Three are UNIT_VOCABULARY rows whose class route is OWNER_DECISION.
   - **Missing from H4:** 83 of the T8 record-repair rows (SR-1) have class route OWNER_DECISION (81) or REVIEW (2). The one T12 record-repair row is `DEL-00-03:AB#realized-artifacts.r02` (T12-C06).
   - **Where T8 disagrees with the class route.** DEC-009: 4 rows, T8 says OWNER, the class says CODE_FIX. ACCEPTANCE_WORKFLOW: 9 rows, T8 says NO_ACTION, the class says OWNER. TIER_IN_SCOPE_REQ: 4 rows, T8 says NO_ACTION, the class says CODE_FIX. The plan says T8's reading shapes the packets, but the H scopes are defined only from `CLASS_ASSIGNMENTS`, so handoff writers will never see T8's view.
   - **T1–T3 have no Route column**, so "T1–T3 capabilities routed SCOPE_CHANGE_HANDOFF" (H1) and "routed … REVIEW" (H3) select nothing there. H1's classification list is the real scope, and it is adequate. NON_DELIVERABLE (35) and SHARED_OK (34) go to no handoff; see I6.

   Fix: add T8 and T12 to H2 and H4, and T8 to H3. Give every H writer the list of T8/class route disagreements, to show both views.

I4. **SHOULD-FIX — Authority and route diverge, and the handoffs drop the authority.** Several classes have one owning authority but a different route:

   | Classes | Authority | Route | Rows |
   |---|---|---|---|
   | T5A-C01–C04, C07 | REVIEW | R5_RECORD_REPAIR | 531 |
   | T4A-C04 | REVIEW | R5_RECORD_REPAIR | 68 |
   | T6-C03, T7-C06 | REVIEW | CODE_FIX_CANDIDATE | 76 |
   | T4A-C02 | OWNER (treatment) | R5_RECORD_REPAIR | 213 |
   | T6-C08, T6-C09, T7-C08 | OWNER | NO_ACTION | 55 |

   H3 is defined by route, so the REVIEW-authority repair and code-fix rows never reach the review register. Neither the H2 nor the H4 CSV has an Authority column, so the review step is lost. Fix: add `Authority` to the `R5_REPAIR_ROWS` and CFB outputs, and mark rows that need review before repair (or cite an H3 item).

   T6-C09 also needs a topic. It covers the sanctioned SCA-009 re-point, and T6 asks that its six CONTESTED rows go to "Agent 0's contested-resolution set so R4 can confirm or replace". No topic carries it. C7 is the natural home.

I5. **SHOULD-FIX — Brief gaps against method R4 and R6.** The packet format covers every element method R4 lists: decision and holder (with the six holders distinguished), options, evidence and reliability, affected claims and filter, risks, recommended routing, on-ruling mechanism and dependencies. The gaps are:
   - (a) Rule 2 must allow the split-class portions described in I2.
   - (b) `{OUT}` is used but never defined. Say that the launch message's output folder is `{OUT}`.
   - (c) The H4 CSV is keyed by `ClaimKey`, but the T9 and T11 record-repair items it must include are subjects, not claim keys (for example T11 `Check/Subject`). Allow an `ItemID` or `Subject` key for those rows.
   - (d) The brief says T10 is "context only" but does not add "never evidence or authority", as the R3 plan does. Add it.
   - (e) Method R4 says "stop affected repair paths until the responsible human acts". H2 and H4 carry `BlockedOnPacket`, which is good, but the brief should say that every row whose class, or whose T8 reading, needs an owner or review decision must carry a blocker.

I6. **SHOULD-FIX — NO_ACTION rows should be recorded explicitly.** Method R6 requires that authorised no-change and no-repair rows be "record[ed] … explicitly rather than dropping them from accounting". NO_ACTION rows total 58 class rows (T6-C08/C09/C10, T7-C08), plus T8 18, T9 80, T11 134 and T12 118. They should appear in a NO_ACTION register in `R3_SYNTHESIS.md` (or an `R3/NO_ACTION_ROWS.csv` Agent 0 builds deterministically), with the task's stated reason, so that R5 and R6 accounting stays exact. The same goes for NON_DELIVERABLE (35) and SHARED_OK (34) capabilities, which belong in the final unmapped-set record. No handoff writer needs them.

I7. **MINOR — `ROUTING_GAPS.csv` counts alternative owners as owners.** `ProposedOwner` is free text. The tool takes every `DEL-xx-yy` it contains, including alternatives and rejected candidates ("DEL-02-05 (alternative DEL-12-01)"); 90 capability rows name several deliverables. So "345 proposed-owner links", "63 never AREA-routed" and "39 of 58" count candidate links, not proposed owners. Label them as such, or take only the first-named deliverable, before C6 cites the numbers.

I8. **MINOR — `class_table` never resets its header.** The table parser keeps its header across every later `|` table in the file. It is safe today: the set and count asserts pass, and I cross-checked each class's Route and Authority against the class section text and found no disagreement. But a later table whose first cell contains `Cnn` could silently overwrite the metadata. Suggest resetting the header on the first non-table line.

### Q1 — Tool

- `--check` prints "CHECK PASS". The asserts confirm three things: the class set equals the CSV set for each task; each class-table row count equals the CSV count; and the assignments cover all 3,299 divergent rows exactly once.
- `CLASS_INDEX` metadata matches each task's summary table. Where the class detail sections state a route or authority, they agree with it. Spot checks: T4A-C06, T4B-C03, T4B-C04, T4B-C08, T5A-C05, T5A-C06, T6-C07 and T7-C05.
- Route totals (classes, rows): R5_RECORD_REPAIR 20 classes, 2,099 rows; OWNER_DECISION 16, 444; CODE_FIX_CANDIDATE 5, 375; REVIEW 7, 295; NO_ACTION 4, 58; SCOPE_CHANGE_HANDOFF 1, 17; ENGINEERING_AUTHORITY 2, 11.
- Caveats: I7 and I8.

### Q2 — Topic assignment

Every class with OWNER authority or OWNER_DECISION route is cited in some topic, except T6-C08 and T6-C09. T6-C08 is a ruling-held NO_ACTION class and needs no packet. T6-C09 does (I4). T7-C08 is in C3. The remaining gaps and overlaps are in I1 and I2.

### Q3 — Brief

- The brief is safe. It forbids repair, ruling and decision. It keeps holders distinct, bounds writes to one output folder with a scratch folder, and states the claim fence, DEC-043, the no-git rule and the no-network rule. Its no-duplication rule is sound once I2 is fixed.
- It is sufficient apart from I2, I3, I4 and I5.

### Q4 — Handoff scopes

Not complete; see I3 and I4. NO_ACTION needs no handoff but should be recorded explicitly (I6).

### Q5 — Containment

- `git diff --name-only fdaedd4f9..5e89655fd` touches only RUN (`R3/`, `R3/TASKS/`, the tool, the topics file, `RUN_STATE.jsonl`) and the AR brief.
- No ledger, resolution, inventory, deliverable, code or DAG file changed. The working tree was clean before this append.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Integration backcheck 1

**Backcheck verdict: FINDINGS.** Every original finding (I1–I8) is resolved. The changes introduce one SHOULD-FIX (B2-1), a contradiction between two rules the writers will read. It is small, so a one-sentence fix in the topic file or the launch messages would clear it. There are also three MINOR items.

Scope: commits `cd7feac01` and `ab39f5712` (`5e89655fd..ab39f5712`). Reviewed bytes:
- `tools/index_r3_classes.py`, sha256 `12093c22ac7a0cec240dd6c7299ce3f3165e0d4bc5740273f369de2da531848b`
- `R3_INTEGRATION_TOPICS.md`, sha256 `804365fed61b88cb8b175354c0d705196d683f6ec72bd311080fa6ededb01dad`
- `briefs/R3-INTEGRATION_brief.md`, sha256 `7db21ce45a4e091e5f29eef8a7cc6559cb3a830ad2d16fb528ed15a232236958`

`--check` prints "CHECK PASS". The diff touches only RUN (`R3/`, the tool, the topics file, `RUN_STATE.jsonl`) and AR (the brief, and this file committed unchanged at `2cc3302c…`).

### Original findings

- **I1 (crosswalk): RESOLVED.** Every candidate I listed now has exactly one placement. PHYS-007, the product section and mass routine, SS-02, SS-03, T9-C12, the W2 convention questions and the draft-policy question are placed in topic text. The remaining items are placed in the crosswalk (H1 for the T1/T2/T3 ownership items; H2 for the W3 product/code items). For `openpipestress_jcs_ijson_v1`, A2 takes the contract version and A4 the rename residue, a stated boundary. Writers still report anything else as UNASSIGNED.
- **I2 (double assignment): RESOLVED.** A6 owns `DEL-01-01:SOW` and A4 only cross-references it. A5 and C7 divide SR-1, and B12 and C7 divide the acceptance workflow. The split classes are listed with a portion rule, and Agent 0 checks that portions sum to class counts. See B2-1 for one contradiction this rule creates.
- **I3 (handoff coverage): RESOLVED.** H2 gains the T8 and T12 code-fix rows. H3 gains T8, and correctly notes that T1–T3 carry no route column. H4 gains the T8, T9, T11 and T12 record-repair rows. `T8_ROUTE_DISAGREEMENTS.csv` has 113 rows, matching my independent count. My earlier I3 overstated the T12 record-repair gap: that row (`DEL-00-03:AB#realized-artifacts.r02`) was already class-routed R5.
- **I4 (authority): RESOLVED.** H3 now takes one item per class with REVIEW or ENGINEERING authority on a repair or code-fix route. H2 and H4 carry an `Authority` column, and brief rule 6 requires a `BlockedOnPacket`. T6-C09 goes to C7.
- **I5 (brief): RESOLVED.** The brief now:
  - (a) replaces full class counts with split portions;
  - (b) defines `{OUT}`;
  - (c) adds `Key`/`KeyKind` to the H4 CSV;
  - (d) says T10 is never evidence or authority;
  - (e) adds the blocker rule.
- **I6 (NO_ACTION): RESOLVED.** `NO_ACTION_ROWS.csv` has 408 rows: CLASS 58, T8 18, T9 80, T11 134, T12 118. That equals the sum I gave. No reason text contains an embedded newline, so the line-based register counts in CLASS_ROUTE_TOTALS are safe. `CAPABILITY_DISPOSITIONS.csv` has 296 rows, exactly the set of non-OWNED capabilities.
- **I7 (routing gaps): RESOLVED.** Among 239 PRIMARY links, 30 of the 44 ROUTING_GAP capabilities were never routed to the primary owner's package. I recomputed both numbers. COVERAGE_AND_QA.md §2 quotes them with the `Rank = PRIMARY` basis.
- **I8 (header reset): RESOLVED.** The header resets at the first non-table line after the class rows.

### New items

- **B2-1 SHOULD-FIX — The split-class rule contradicts the H2 and H3 scopes.**
  - "Boundaries and split classes" lists T7-C06 as split between C2 and H2, and T7-C07 between C3 and H3. It requires the portions to sum to the class count.
  - But the H2 scope still says "every row routed CODE_FIX_CANDIDATE … (T7-C06)", and the H3 scope says "every row or class routed ENGINEERING_AUTHORITY", which is all of T7-C07.
  - Elsewhere the intended pattern is overlap with a blocker, not a partition. For example, the 81 SR-1 rows sit in A5 and also in H4 with `BlockedOnPacket A5`. T4A-C02 sits in A9 and also in H4.
  - Read literally, the portion-sum check fails for T7-C06 and T7-C07, or writers drop the DEL-04-04 row from one side.
  - Fix: apply the portion rule among packets only. A handoff covers every row on its route, marked `BlockedOnPacket` for rows a packet decides. Alternatively, take T7-C06 and T7-C07 off the split list and have H2 and H3 block the relevant rows on C2 and C3.
- **B2-2 MINOR — The NO_ACTION register carries 15 rows that have another route.** They are the T8 NO_ACTION rows whose class route is OWNER_DECISION (9), CODE_FIX_CANDIDATE (4) or R5_RECORD_REPAIR (2), so they are also in P packets, H2 or H4. The register has no `ClassRoute` column, so R6 could count these rows as both "no-repair" and "repair". Suggest adding `ClassRoute`, or cross-referencing `T8_ROUTE_DISAGREEMENTS.csv` when Agent 0 writes R3_SYNTHESIS. (Three keys appear under more than one source, which is harmless.)
- **B2-3 MINOR — The H2 inputs overlap, and some H2 items have no claim key.**
  - All 17 T12 code-fix rows and 12 of the 17 T8 code-fix rows are already code-fix rows in `CLASS_ASSIGNMENTS`. The phrase "one row per code-fix row" implies deduplication, but say so.
  - The W3 product and code items (PCF zero coordinate, millimetre assumption, and so on) do not appear by those terms in any R3 task output. They may have no claim key. `CODE_FIX_ROWS.csv` has only `ClaimKey`, so give it the same `Key`/`KeyKind` as H4.
- **B2-4 MINOR — "PRIMARY" is a heuristic.** It means the first deliverable named in free text. Some rows begin with a candidate list ("No existing GUI owner; candidates DEL-07-04 or DEL-06-02 …; else CREATE") or a nearest-existing note ("CREATE …; nearest existing: DEL-07-01"). The summary line's "primary = first named" is honest. Writers should still read `ProposedOwner` rather than treat `Rank` as the task's decision.

### Assessment

- The tool, the topic file and the brief are otherwise sufficient and safe.
- Topic coverage is now complete, as far as I can check against the task outputs and the W2/W3 lists.
- The handoffs cover every non-owner route. NO_ACTION is recorded explicitly.
- Containment is clean.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
