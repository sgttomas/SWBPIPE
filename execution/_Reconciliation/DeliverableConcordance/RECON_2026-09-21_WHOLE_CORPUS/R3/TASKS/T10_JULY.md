# T10 — July cross-check

This note compares our effective dispositions (`R3/CORPUS_CLAIMS.csv`, 9,889 rows, 102 deliverables) with the July concordance (`DELIVERABLE_CONCORDANCE_2026-07-11_1305/CLAIM_CONCORDANCE.csv`, 2,492 rows, 101 deliverables plus 8 unmapped surfaces). The July file is a cross-check only, not evidence or authority. It is also a pre-R5 snapshot: all 532 of its divergent claims fall in the scope of a July proposed update (`PROPOSED_DELIVERABLE_UPDATES.csv`, PDU-001..077), and the July R5 run (`R5_RUN_SUMMARY.md`, 2026-07-12) acted on those updates the next day. The claim IDs do not match, so I matched at three grains: deliverable, deliverable × topic (ClaimClass, and separately a structural ClaimType facet) and, where the source text allows it, deliverable × requirement ID. **At deliverable grain, nothing July flagged has fully cleared.** 97 of 101 July deliverables are divergent in both snapshots (96 with at least one shared disposition). Four deliverables that were fully aligned in July (DEL-03-02, DEL-03-03, DEL-03-07, DEL-17-04) are now divergent, and DEL-07-09 is new. **The requirement-ID anchor is the most precise view** but covers only 424 July claims. On it, 22 July divergences are resolved, 21 persist (15 in the same disposition, 6 in a different one) and 86 claims that July called ALIGNED are now divergent. They are mostly PARTIALLY_IMPLEMENTED with cause PARTIAL_SLICE. Much of the apparent growth, from 21% of rows divergent in July to 33% now, comes from method differences rather than product drift: we cut the corpus four times finer, keyed DECLARED_STATE records separately and used STALE_REVIEW_OR_EVIDENCE heavily where July used it twice. It should not be read as regression. Of 8 surfaces July left unmapped, 3 now have an owning deliverable and 5 are still unowned.

## How I matched

- **Deliverable.** July `PackageID/DeliverableID` against our `DeliverableID`. Both use the same identifiers. July's `UNMAPPED` pseudo-deliverable is handled separately (see below).
- **Topic, subject facet (`CLASS:<ClaimClass>`).** Both snapshots use the same ten-value ClaimClass vocabulary, so the pair (deliverable, ClaimClass) is the topic. Limitation: the two runs classified independently. On the 424 requirement-anchored claims, July's class matches one of our anchored rows' classes only 203 times (48%). A class-grain change may therefore be reclassification, not a change of state. That is why rows that exist on only one side get their own labels.
- **Topic, structural facet (`TYPE:<ClaimType>`).** This facet is more stable: on anchored claims, July REQUIREMENT matches our REQUIREMENT 400/424 times. For comparability, July REMAINING_WORK is matched against our rows with `Remaining = YES`. Of those, 124 are typed DECLARED_STATE and 27 REMAINING_WORK. Our NON_NORMATIVE, CONTAINER, CONTEXT and HISTORY types have no July counterpart.
- **Requirement-ID anchor (`ANCHOR:<July ClaimID>`, supplementary).** Where July's `NormativeSource` names a requirement or acceptance ID of the same deliverable, the ID is normalised: DEL- prefix optional; REQ/RQ/R, ACC/AC/A, V and OUT kinds; number compared as an integer. It is matched to our ITEM and SUBCLAIM keys whose last segment carries the same ID. 424 of 2,492 July claims anchor this way. The anchor uses the ID in the source document, not July's own serial. For example, July `DEL-17-08-REQ-009` cites source `DEL-17-08-REQ-013` and anchors to `DEL-17-08:SOW#CLM-014/DEL-17-08-REQ-013`. The CSV lists the 129 anchored claims that are divergent on either side; 295 are aligned in both.
- **Divergent** means the same rule on both sides: the disposition is not ALIGNED, NOT_ASSESSED or COVERED_BY_CHILDREN. That is our `Divergent` flag. July has none of the latter two values. "Substantive" in Evidence excludes the two STALE_* record-currentness dispositions.

`Change` vocabulary: RESOLVED_SINCE_JULY; PERSISTS_SAME_FORM (at least one divergent disposition in common); PERSISTS_CHANGED_FORM (divergent in both, no disposition in common); NEW_SINCE_JULY (July present and aligned, now divergent); NEW_TOPIC (no July rows for the topic, now divergent); NEW_DELIVERABLE; ALIGNED_BOTH; JULY_TOPIC_NOT_REKEYED (July divergent, no current rows in that topic); JULY_ONLY_ALIGNED; CURRENT_ONLY_ALIGNED.

## Results by grain

| Grain | Resolved | Persists same | Persists changed | New since July | New topic/deliverable | July topic not rekeyed | Aligned both | One side only, aligned |
|---|---|---|---|---|---|---|---|---|
| Deliverable `(ALL)` (102) | 0 | 96 | 1 | 4 | 1 | 0 | 0 | 0 |
| `CLASS:` (710) | 12 | 133 | 56 | 211 | 111 | 21 | 35 | 86 July / 45 now |
| `TYPE:` (860) | 5 | 145 | 45 | 118 | 95 | 7 | 45 | 52 July / 348 now |
| `ANCHOR:` (424; 129 in CSV) | 22 | 15 | 6 | 86 | — | — | 295 | — |
| Unmapped surfaces (8) | 3 | 5 | — | — | — | — | — | — |

Corpus totals: July 532/2,492 divergent (21.3%). Now 3,299/9,889 (33.4%). Substantive divergence at deliverable grain:
- 66 deliverables substantive in both snapshots;
- 29 substantive now but not in July;
- 2 substantive in July but not now (DEL-04-02, DEL-10-05; both keep only STALE_* rows);
- 4 with none in either snapshot.

## Resolved since July

- **Anchored (22 claims, most reliable).** All 22 had July PDU scope.
  - DEL-07-06 REQ-001, 002, 003, 005, 008: VERIFIED_NOT_VALIDATED → ALIGNED (PDU-045).
  - DEL-03-06 REQ-003, 004 and DEL-04-01 REQ-006: ACCEPTED_DIVERGENCE → ALIGNED (PDU-070, PDU-068).
  - DEL-17-08 REQ-009, 015, 018, 021: PARTIALLY_IMPLEMENTED → ALIGNED (PDU-010, 020, 031).
  - The others: DEL-02-04 REQ-014, DEL-04-02 REQ-007, DEL-07-01 REQ-001, DEL-09-02 REQ-007 and 008, DEL-13-04 REQ-004, DEL-17-03 REQ-006, DEL-17-05 REQ-014, DEL-17-06 REQ-007 and DEL-17-09 REQ-001.
- **Class grain (12 topics).**
  - DEL-00-01 DOCUMENTATION: three STALE_SETUP_SPECIFICATION declared-state claims (PDU-054).
  - Nine single-claim PARTIALLY_IMPLEMENTED, DOCUMENTED_UNIMPLEMENTED, VERIFIED_NOT_VALIDATED or ACCEPTED_DIVERGENCE topics: DEL-02-05 REPORTING, DEL-03-08 MECHANICS, DEL-10-04 SECURITY, DEL-12-03 GUI, DEL-12-04 REPORTING, DEL-13-01 WORKFLOW, DEL-13-03 SCHEMA, DEL-15-02 SECURITY and DEL-17-05 SECURITY.
  - Two REMAINING_STATE_MISMATCH topics (DEL-14-03 WORKFLOW, DEL-15-03 SECURITY; PDU-060).
  - Caveat: DEL-03-08 MECHANICS is resolved at class grain, yet its anchor (DEL-03-08-REQ-003) persists as PARTIALLY_IMPLEMENTED in another class. This shows the class-drift limitation.
- **Remaining (TYPE:REMAINING_WORK).** DEL-11-01, DEL-11-03 and DEL-16-01: July REMAINING_STATE_MISMATCH → current Remaining rows aligned (PDU-060).
- **Deliverable grain.** None. Every July-divergent deliverable still carries divergent rows.

## Persisting

- **Anchored, same disposition (15).** Mostly PARTIALLY_IMPLEMENTED with cause PARTIAL_SLICE: DEL-03-04 REQ-006, DEL-04-04 REQ-006, DEL-04-05 REQ-006, DEL-09-01 REQ-004, DEL-10-01 REQ-002 and 010, DEL-17-05 REQ-015, DEL-17-08 REQ-023 and 026, DEL-17-09 REQ-007. The others:
  - DEL-04-04 REQ-008: cause VERIFICATION_REMOVED.
  - DEL-08-05 REQ-010: DEFERRED_BY_RULING.
  - VERIFIED_NOT_VALIDATED/VALIDATION_GAP: DEL-07-03 REQ-011, DEL-07-06 REQ-004, DEL-13-04 REQ-007.

  These are July-era implementation gaps that R5 did not close.
- **Anchored, changed form (6).**
  - DEL-07-03 REQ-002, 005 and 006: PARTIALLY_IMPLEMENTED/DOCUMENTED_UNIMPLEMENTED → ACCEPTED_DIVERGENCE, cause OWNERSHIP_ELSEWHERE. The REQ-002 and REQ-005 rows carry exception resolutions, visible in the CSV Evidence.
  - DEL-03-08 REQ-003: VERIFIED_NOT_VALIDATED → PARTIALLY_IMPLEMENTED.
  - DEL-15-03 REQ-008: PARTIALLY_IMPLEMENTED → IMPLEMENTED_DIFFERENTLY, DOC_BEHIND_CODE.
  - DEL-17-08 REQ-024: PARTIALLY_IMPLEMENTED → STALE_SETUP_SPECIFICATION, DOC_BEHIND_CODE.
- **Class and type grain.**
  - 133 CLASS topics persist in the same form. The dominant pairs are STALE_SETUP_SPECIFICATION → STALE_SETUP_SPECIFICATION (83) and PARTIALLY_IMPLEMENTED → PARTIALLY_IMPLEMENTED (47).
  - 56 persist in changed form, chiefly STALE_SETUP_SPECIFICATION → STALE_REVIEW_OR_EVIDENCE (86 disposition pairs across topics).
  - Every July ACCEPTED_DIVERGENCE topic that still exists is now divergent in another form. Pairs: STALE_REVIEW_OR_EVIDENCE 14, STALE_SETUP_SPECIFICATION 10, PARTIALLY_IMPLEMENTED 7.
  - For Remaining, 22 of 27 persisting topics changed form. July REMAINING_STATE_MISMATCH is now mostly DOCUMENTED_UNIMPLEMENTED or STALE_* on `STATUS#remaining/Rnn` rows, e.g. DEL-15-02, DEL-09-03, DEL-17-06.
- **July topics not rekeyed.** 21 CLASS topics and 7 TYPE topics have July divergence but no current rows in that topic. Every one had July PDU scope. They are listed in the CSV with their July IDs. Most appear to be reclassified (class drift) rather than lost. TYPE examples:
  - DEL-04-05 EXC-001, DEL-07-03 EXC-001, DEL-17-05 EXC-001/002, DEL-17-08 EXC-001/002: July EXCLUSION rows, where our DEL-04-05, DEL-07-03, DEL-17-05 and DEL-17-08 ledgers have no EXCLUSION-typed rows.
  - DEL-01-02, DEL-10-05 and DEL-17-02: July Remaining items where our census has no Remaining row.

  I did not verify item-level coverage for these 28 topics. Agent 0 may want a spot check.

## New since July

- **Deliverables.** DEL-07-09 is new (20/111 divergent; FIELD exceptions on PALETTE_OPERATION_ROUTING). Four July-aligned deliverables are now divergent:
  - DEL-03-02 and DEL-03-03: REPRESENTATION_MIGRATED/BASIS_POINTER_STALE, with CONTESTED `CONTEXT#architecture-basis-injection` rows.
  - DEL-03-07: including 3 POSSIBLE_DEFECT rows, plus CONTESTED/OBSERVED rows `SOW#CLM-021.s02` and `SOW#CLM-026.r01`.
  - DEL-17-04: substantive 0 → 10, including DOC_BEHIND_CODE and NOT_STARTED.
- **Anchored (86 claims).** July ALIGNED, now divergent: PARTIALLY_IMPLEMENTED 50, STALE_SETUP_SPECIFICATION 12, IMPLEMENTED_DIFFERENTLY 9, UNKNOWN 3, others 12. Leading causes: PARTIAL_SLICE 41, POSSIBLE_DEFECT 11, DOC_BEHIND_CODE 11, SCOPE_REDIRECTED_BY_RULING 5. These are the clearest cases where July and we read the same requirement differently. The CSV names each one.
- **Topic grain.** 211 CLASS topics were aligned in July and are divergent now; 111 are divergent in classes July did not use for that deliverable. The dominant new dispositions are STALE_REVIEW_OR_EVIDENCE (148 topics), STALE_SETUP_SPECIFICATION (117) and PARTIALLY_IMPLEMENTED (87). The STALE_* share is mostly a difference in method: we keyed separate declared-state surfaces (STATUS, AB, MEMORY, CONTEXT) and read them against a later tree.

## July unmapped surfaces

Each July surface is matched to our capability by its entry-point path in `RUN/IMPLEMENTATION_SURFACES.csv`. Its current status comes from `R3/CAPABILITY_COVERAGE.csv`.

| July | Current capability | Status now | Change |
|---|---|---|---|
| SURF-011 build-readiness | CAP-FEATC-006 | OWNED (DEL-10-04) | RESOLVED |
| SURF-021 export unit disclosure | CAP-FEATC-013 | UNMAPPED_RELATION_ONLY (DEL-08-04, DEL-17-02 COVER) | PERSISTS |
| SURF-050 telemetry panel | CAP-FEATC-035 | OWNED (DEL-12-03) | RESOLVED |
| SURF-104 core/product_preview | CAP-PHYS-031..034 | UNMAPPED / UNMAPPED_RELATION_ONLY | PERSISTS |
| SURF-170 root package.json | CAP-CHECKS-021 | OWNED (DEL-10-04) | RESOLVED |
| SURF-211 tools/REGISTRY.md | CAP-CHECKS-019 | UNMAPPED | PERSISTS |
| SURF-212 semantic-refresh fan-in | CAP-CHECKS-018 | UNMAPPED | PERSISTS |
| SURF-213 dependency rectification | CAP-CHECKS-017 | UNMAPPED | PERSISTS |

The July R5 summary records D-42 as AWAITING_RULING for SURF-011 and SURF-021. Our ledgers now show DEL-10-04 claiming CAP-FEATC-006, while CAP-FEATC-013 has no owner. Agent 0 may want to check whether D-42 was ruled or whether the DEL-10-04 claim predates or anticipates a ruling. This is a cross-check pointer, not a finding of fact. The four persisting ownership gaps overlap the T1 scope.

## Routing implications (proposals only)

T10 adds no classes. For R4 it points to three things:
- The anchored persisting items are long-standing implementation gaps: PARTIAL_SLICE, VALIDATION_GAP, VERIFICATION_REMOVED and DEFERRED_BY_RULING. Proposed route: CODE_FIX_CANDIDATE or ENGINEERING_AUTHORITY via T6/T7.
- The anchored new-since-July items, 86 of them, including 11 POSSIBLE_DEFECT. Proposed route: REVIEW. Two readings of the same requirement differ, and the later reading governs.
- The persisting unmapped surfaces, including D-42 SURF-021. Proposed route: OWNER_DECISION or SCOPE_CHANGE_HANDOFF via T1.

No July value is used to challenge an effective value.

## Coverage

- **Output.** `T10_JULY.csv`: 1,809 body rows plus `#END`, CRLF, written with the Python `csv` module.
  - 102 `(ALL)` rows: every one of our 102 deliverables and all 101 July deliverables.
  - 710 `CLASS:` rows.
  - 860 `TYPE:` rows.
  - 129 `ANCHOR:` rows, one per anchored July claim that is divergent on at least one side. A July claim anchored to several current keys still gets one row.
  - 8 unmapped-surface rows.
- **Check.** A script rebuilt the (deliverable, topic) key set from both source files and confirmed it equals the CSV key set, with no duplicates and no duplicate anchors. The row totals in each family sum to 2,484 July rows (2,492 minus the 8 unmapped) and 9,889 current rows. So every row of both files falls into exactly one `CLASS:` topic, one `TYPE:` topic and one deliverable.
- **Counts reproduced.** 532 July divergent. 3,299 current divergent, matching R3_PLAN. 424 July claims anchored.
- **Not read.** No draft resolutions file (`RESOLUTIONS_DRAFT*.csv`) and no other task's files. The July material was read only under `DELIVERABLE_CONCORDANCE_2026-07-11_1305/`: CLAIM_CONCORDANCE.csv, PROPOSED_DELIVERABLE_UPDATES.csv and R5_RUN_SUMMARY.md.

## R3 observations

None. I found no effective value that the July comparison gives evidence against. The July file is not evidence, and differences are reported as cross-check signals only.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
