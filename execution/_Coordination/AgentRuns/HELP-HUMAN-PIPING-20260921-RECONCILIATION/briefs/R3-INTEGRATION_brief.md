# Brief: R3 integration drafting (TASK, Type 2)

You are a TASK agent in R3 integration for run
HELP-HUMAN-PIPING-20260921-RECONCILIATION. Your parent is HELP_HUMAN Agent 0.
You do not delegate. Your launch message gives you three things: your ID
({TASK}), your topic list or scope, and your output folder ({OUT}, a folder
under `RUN`). Your topic IDs are defined in `RUN/R3_INTEGRATION_TOPICS.md`.
Read its "Boundaries and split classes" and "Crosswalk" sections first.

## Purpose

R3 synthesis has classified the whole corpus. You draft one part of what
the owner will see at the R4 gate: decision packets or a handoff. You
propose, and nothing more. You never repair, rule, decide, or change a row,
deliverable, code, lifecycle state, DAG or instruction.

`RUN` is `{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

## Accepted basis (read-only)

- **R3 plan:** `RUN/R3_PLAN.md`.
- **Deterministic R3 tables:**
  - `RUN/R3/CORPUS_CLAIMS.csv` (effective values, `OtherCorrections`, flags);
  - `CLASS_INDEX.csv` (55 classes: name, rows, authority, route);
  - `CLASS_ASSIGNMENTS.csv` (every divergent row with its class and route);
  - `NO_ACTION_ROWS.csv`, `CAPABILITY_DISPOSITIONS.csv` and
    `T8_ROUTE_DISAGREEMENTS.csv` (the 113 T8 rows whose route differs from
    the class route: show both views for them);
  - `CAPABILITY_COVERAGE.csv`, `ROUTING_GAPS.csv` (use `Rank = PRIMARY` for
    proposed owners), `REMAINING_CENSUS.csv`,
    `PACKAGE_SUMMARY.csv`, `CLUSTER_MATRIX.csv`, `CLASS_ROUTE_TOTALS.md`,
    `SYNTHESIS_STATS.md`.
- **R3 task outputs,** all in `RUN/R3/TASKS/`:
  - `T1_UNMAPPED.csv`, `T1_NOTES.md`, `T2_*`, `T3_OWNERSHIP.csv`, `T3_NOTES.md`
    (capabilities);
  - `T4A…T7_CLASSES.{csv,md}` (classes);
  - `T8_CLUSTERS.md`, `T8_ROWS.csv` (contested clusters);
  - `T9_LIFECYCLE.*`;
  - `T10_JULY.*` (July cross-check, context only: never evidence or authority);
  - `T11_METHOD.*`;
  - `T12_UNREACHED.*`.

  You may read all of them.
- **R2 records:**
  - the assessments: `RUN/WAVES/W1/W1_GATE_ASSESSMENT.md`, `W1_GATE_RULING.md`,
    `RUN/WAVES/W2/W2_GATE_ASSESSMENT.md`, `RUN/WAVES/W3/W3_ASSESSMENT.md`. The
    W2 and W3 assessments carry the collected owner items.
  - the verification reports.
  - the adopted resolutions `RUN/WAVES/{W1,W2,W3,CROSS_WAVE}/RESOLUTIONS.csv`.
    Ignore every `RESOLUTIONS_DRAFT*.csv` and
    `CROSS_PACKAGE/RESOLUTIONS_MERGED_DRAFT.csv`.
  - the sealed ledgers `RUN/WAVES/W*/PKG-*/DEL-*/`. Ignore `superseded_<n>/`.
- **Governance context:**
  - `RUN/CONVENTIONS.md`, `RUN/CANONICAL_SITUATIONS.md`,
    `RUN/AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`;
  - the decision register `{REPO}/projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md`
    and its packets;
  - the owner directions
    `{REPO}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/OWNER_DIRECTIONS.md`.
- **Method:** `{REPO}/workflows/reconciliation/resources/method.md`, sections R4
  and R5.
- **Frozen source:** `{FREEZE}`, a read-only checkout of `00115c719`. It is
  evidence, never authority.

## Rules

1. **Authority.** Effective values are accepted results. Task outputs are
   proposals. Where tasks disagree, show both views and do not choose. Owner,
   engineering, review, scope-change, HELPS_HUMANS and external decisions each
   stay with their holder. Say which holder each item needs.
2. **Coverage.**
   - Account for every item in your assigned topic list or scope, and cite its
     source rows or classes.
   - A packet names every affected class and the row count of **its
     portion**. For a class wholly in your topic, that is the full class
     count. For a split class (listed in the topic file), list exactly the
     keys of your portion; never claim the whole class. Give the claim keys,
     or a filter over `CLASS_ASSIGNMENTS.csv` or a task CSV that reproduces
     them exactly.
   - Before you return, reconcile your counts against those files with a
     script.
3. **No duplication.** Stay inside your topic list. If you find a
   cross-reference to another list, cite it by topic ID; do not repeat its
   content. If an owner decision you find is on no list, report it in your
   return as UNASSIGNED; do not draft it.
4. **Evidence.** Cite ledger keys, report sections and `path:line` at the
   freeze.
   - Run no builds or tests. Do not use git.
   - Quote no protected, private or third-party copyrighted content.
   - Follow DEC-043: no equation sources.
5. **Claim fence.** Make no certification, code-compliance, professional
   approval or engineering-acceptance claims (F-PIP-2; DEC-081).
6. **Blockers.** Method R4: affected repair paths stop until the responsible
   holder acts. In H2 and H4, every row whose class, or whose T8 reading,
   needs an owner or review decision carries a `BlockedOnPacket` (a topic ID
   or an H3 item ID). Every row also carries its class `Authority`.
7. **Writes.**
   - Write only inside your output folder.
   - Put scratch files under `{OUT}/_scratch/` and delete them before you
     return.
   - Run Python with `PYTHONDONTWRITEBYTECODE=1`.
   - Use no network. Never write in `{FREEZE}`.
8. **API overload.** The API returns intermittent 529 errors. Write
   incrementally, one packet or section at a time, and keep generations short.
   If you are resumed, continue from what is on disk.

## Decision packet format (P1–P3)

Write one file per topic: `{OUT}/<TopicID>_<short-name>.md`. Each packet has
these sections, in order:

1. **Decision.** The question in one or two sentences, and who decides:
   OWNER, ENGINEERING, WORKING_ITEMS (workflow: review), WORKING_ITEMS
   (workflow: scope-change), HELPS_HUMANS or EXTERNAL.
2. **Background.** What was decided before (register rows, DEC numbers,
   rulings) and what the code does now. Cite both.
3. **Options.** Two to four options as they stand in the evidence. For each,
   give its consequences for deliverables, code and other packets.
4. **Evidence and reliability.** Key sources and their reliability class. Say
   what is verified and what comes only from worker notes or code reading.
5. **Affected claims.** Classes and counts, packages and deliverables, and the
   reproducing filter. Include rows known only from `OtherCorrections`.
6. **Risks.** What goes wrong if the question is left undecided, and the risk
   of each option.
7. **Recommended routing.** Give a recommendation only where the evidence
   supports one, and say so plainly. Otherwise write "no recommendation;
   owner's call".
8. **On-ruling mechanism.** Exactly what each option would authorise, and
   through which workflow or change path. Examples: an R5 record repair on
   named deliverables; a scope-change handoff; a code-fix brief under a
   production brief; the ISSUED change path; a lifecycle workflow; a
   convention amendment through HELPS_HUMANS. Nothing executes until the
   owner acts, and R5 needs separate authorization.
9. **Dependencies.** The other packets this one depends on or blocks.

Also write `{OUT}/INDEX.md`: one line per packet, giving the ID, the question
and the holder.

## Handoff formats (H1–H4)

- **H1 scope change.** `{OUT}/SCOPE_CHANGE_HANDOFF.md` plus
  `{OUT}/SCOPE_CHANGE_ITEMS.csv`, with columns `ItemID, Kind, Capabilities,
  Deliverables, Proposal, BlockedOnPacket, Evidence`. `Kind` is one of
  CREATE, ASSIGN, REASSIGN, MERGE, RETIRE, KEY_ISSUE or DUPLICATE_RESOLVE.
- **H2 code-fix candidates.** `{OUT}/CODE_FIX_CANDIDATES.md`,
  `{OUT}/CODE_FIX_ROWS.csv` (columns `ClaimKey, DeliverableID, ClassID,
  Authority, CFB, BlockedOnPacket`, one row per code-fix row in scope), and
  one `{OUT}/CFB-<nn>_<short-name>.md` per candidate brief. Each brief gives
  scope, affected claims, evidence, acceptance checks, the protected-content
  status, and `BlockedOnPacket`. None is executed. Group by engine or area.
  Keep each brief to one coherent change.
- **H3 engineering and review register.** `{OUT}/ENGINEERING_AND_REVIEW.md`
  plus `{OUT}/ENGINEERING_AND_REVIEW_ITEMS.csv`, with columns `ItemID, Route,
  Subject, Classes, Rows, Deliverables, Question, Evidence, BlockedOnPacket`.
- **H4 R5 tranche proposal.** `{OUT}/R5_TRANCHE_PROPOSAL.md` plus
  `{OUT}/R5_REPAIR_ROWS.csv`, with columns `Key, KeyKind, DeliverableID,
  ClassID, Authority, Tranche, BlockedOnPacket`. It holds one row per
  record-repair row. `KeyKind` is `CLAIM` for a claim key, or `ITEM` for a
  T9 or T11 subject-keyed item (then `Key` is its `Item` or `Subject` and
  `ClassID` names the source task and class).
  - Partition the rows by owning deliverable into tranches.
  - Mark the rows that wait on a packet.
  - Give the ISSUED DEL-01-01 its own change-path tranche.
  - This is a proposal only; R5 needs a separate owner authorization.

## Return

Return one line:

`R3-INT {TASK} DONE files=<n> items=<n> <main file>=<sha256>`

Then add no more than 12 lines:
- counts;
- UNASSIGNED owner decisions found;
- any coverage problem;
- any disagreement between tasks that you left open.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
