# H4 — R5 record-repair tranche proposal

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H4.
Frozen source `00115c719`. Row file: `R5_REPAIR_ROWS.csv` (2,234 rows).
Revised after review RV7 (findings 3–10, 17), RV2 finding 2 and RV3
finding 4, with Agent 0's decisions applied.

**Path convention.** Freeze paths (`execution/...`, `docs/...`, `init/...`,
`governance/...`) are relative to `projects/chirality-piping/` at `00115c719`.
Run paths (`R3/...`, `WAVES/...`) are relative to the run evidence folder.
H3 tokens resolve through the sibling `H3_TOKEN_MAP.csv`, which Agent 0
maintains.

**Status.** This is a proposal. It authorises nothing. D-73 item 4 (O-A)
limits this run to R0–R4 and says R5 and R6 need a separate owner
authorization (register row D-73, codified as DEC-110). No row, deliverable,
lifecycle state, DAG or instruction changes because of this file. Every
repair path below stays stopped until its holder acts (method R4) and the
owner separately authorises R5.

## 1. Scope and counts

| Source | In H4 | How it enters |
|---|---|---|
| `R3/CLASS_ASSIGNMENTS.csv`, `Route = R5_RECORD_REPAIR` | 2,099 CLAIM rows, 20 classes | all of them |
| `R3/TASKS/T8_ROWS.csv`, `Route = R5_RECORD_REPAIR` (88) | +83 CLAIM rows | 81 SR-1 rows whose class routes OWNER_DECISION (T4B-C03 42, T5B-C04 39), and 2 SR-1-adjacent rows in T5A-C08 (REVIEW). The other 5 are already class rows |
| `R3/TASKS/T9_LIFECYCLE.csv`, `Route = R5_RECORD_REPAIR` (50) | +17 ITEM rows | 4 T9-C12 corpus subclass items (SRE-3, 4, 6, 7) and 13 per-deliverable REMAINING_CENSUS items. The other 33 are claim keys already in class rows |
| `R3/TASKS/T11_METHOD.csv`, `Route = R5_RECORD_REPAIR` (4) | +35 ITEM rows | D-02 (15 deliverables), S-02 (3), S-04 (16), one row each; S-03 one corpus row |
| `R3/TASKS/T12_UNREACHED.csv`, `Route = R5_RECORD_REPAIR` (1) | 0 | `DEL-00-03:AB#realized-artifacts.r02` is already a T4B-C08 row |
| **Total** | **2,234** (2,182 CLAIM, 52 ITEM) | one row per (Key, DeliverableID) |

Each claim key appears once. Where a key is both a class row and a T8, T9 or
T12 row, it keeps its class `ClassID` and `Authority`, and the task reading
shows up in `BlockedOnPacket` and §6. For ITEM rows, `Key` is the T9 `Item`
or the T11 `Subject`, and `ClassID` names the task and its class. The census
Key `REMAINING_CENSUS` repeats, so ITEM rows are unique on (Key,
DeliverableID).

Out of scope, by the topic file. The whole-class owner routes (T4A-C06/C08,
T4B-C01–C04, T5B-C07/C09, T5A-C05) are not here, except for the T8 SR-1 rows
above. `DEL-11-05:SOW#CLM-031` (T5B-C04) has no T8 row, so it stays with A5
only. Code-fix, review and scope-change rows belong to H2, H3 and H1.

## 2. Tranche design

**Partition rule.** Each row goes to exactly one tranche, set by its owning
deliverable (profile §8: "Owner-ruled repair tranches, partitioned by owning
deliverable"; method R5). A tranche is one package. Its write set is the
folders of the deliverables it names, plus any governed artifact that a row's
RemainingWork names for that deliverable. Two exceptions:
- **`R5-ISSUED-DEL-01-01`** holds every DEL-01-01 row. DEL-01-01 is ISSUED
  (freeze `execution/PKG-01_*/1_Working/DEL-01-01_*/_STATUS.md:3`), and the
  profile §4 sends any change to it through the governed scope-change
  process. T4A observation 1 and T9-C10 ask that all DEL-01-01 edits be
  treated as one ISSUED-path decision, so this tranche also takes the
  `_CONTEXT.md`, MEMORY and STATUS rows, whose BaselineClass is NONE.
- **`R5-XVIEW`** holds the five corpus items (T9-C12 SRE-3/4/6/7, T11 S-03).
  These are cross-views over rows already counted in the classes, and they
  write nothing themselves. At R6 they serve as a check that the class
  repairs covered the population the view names.

**Vehicle.** Each package tranche is one chirality-change PR, or one PR for
each deliverable if a package PR would be too large. It makes no lifecycle
change. `R5-ISSUED-DEL-01-01` runs only on the path that A6 names: a reissue
or an amendment through the scope-change workflow, or no edit at all.

**Order inside every tranche.**
1. **Wave A: ready rows** (`BlockedOnPacket` empty, 1,280 rows). These need
   only the R5 authorization. They are the unblocked rows of the NONE-authority
   classes T4A-C01/C03/C05, T4B-C05–C08, T5B-C03/C05/C06/C08/C10 and T7-C11,
   plus the unblocked items. Each of those classes has named exceptions in §4.
   Only T7-C11 and T5B-C03 are ready as whole classes.
2. **Wave B: rows waiting on an H3 review item** (605 rows). Of these, 604
   carry a class token for one of the REVIEW-authority classes (T4A-C04,
   T5A-C01/C02/C03/C04/C07/C08). One more, `DEL-13-02:STATUS#remaining/R01`,
   carries only the T9-C04 item token. The review named for each class must
   come before its repair.
3. **Wave C: rows waiting on a packet** (376 rows). They follow the ruling
   they name. A row can wait on both an H3 item and a packet, so waves B and
   C overlap by 27 rows: T5A-C04 19, T5A-C01 4 (the DEL-04-0x `CONTEXT` rows
   and `DEL-17-05:SOW#CLM-011`), T5A-C08 2, T5A-C02 1, and DEL-13-02 R01 1.
   A row runs only when all of its blockers have cleared.
4. **After the tranche,** the H3 re-review of SOW parity (T5B-C01, CP-09) for
   the same deliverable runs. T5B's sequencing note says that every SOW text
   repair changes the bytes a parity record binds.

**Guards for every tranche.** Each guard is taken from the class records.
- Do not advance or set any lifecycle state. Where a row states readiness,
  record the current `_STATUS.md` state (F3, Direction 8; T4B-C08, T5A-C01,
  T9-C08).
- Leave the unruled remainder of every TBD or hold open (T5B-C05/C06).
- T4A-C01 edits touch the keyed blocks only, never sibling `.sNN` content.
- On the five T4A-C01 main injection rows
  `DEL-13-03`, `DEL-13-04`, `DEL-14-03`, `DEL-14-04` and
  `DEL-15-02:CONTEXT#architecture-basis-injection`, repair only the pin.
  Leave the DEC-009 / Resolved Baseline text as it stands until A1 rules
  (A1 §5 and §9). These rows stay in Wave A under this guard.
- Do not copy the successor path `init/init-prompt.md` from ledger Notes. It
  does not exist at the freeze, where `init/` holds
  only `dev-loop-init-prompt.md` and `taskmgmt-init-prompt.md` (T4A
  observation 3).
- Leave rename residue in the same files as it is until A4 rules. No H4 row
  carries the RENAME_OR_IDENTITY cause (all 86 are in T4B-C01), but the
  repairs edit the same SOWs.
- Describe hash-profile labels as they stand. A2 owns the label question.
  The 6 DEL-14-02 T5B-C10 rows wait on A2. The 2 DEL-00-06 anchor rows do
  not.
- Quote no protected content, and move no protected check, tolerance or
  limit (profile §4). Edits to governed artifacts are covered only by their
  row's ruling, for example the PKG-01 policy TBD blocks and
  `CONTRIBUTING.md` in the T4B-C08 PARTIALLY_IMPLEMENTED rows.

## 3. Tranches

| Tranche | Deliverables | Rows (CLAIM / ITEM) | Wave A ready | Wait on H3 | Wait on packet | Packets named (rows) |
|---|---|---|---|---|---|---|
| R5-PKG-00 | 8 | 57 (57 / 0) | 45 | 11 | 1 | A3 1 |
| R5-PKG-01 | 3 | 63 (63 / 0) | 56 | 3 | 4 | A5 3, A9 1, C7 3 |
| R5-ISSUED-DEL-01-01 | 1 | 5 (5 / 0) | 0 | 0 | 5 | A5 1, A6 5, C7 1 |
| R5-PKG-02 | 5 | 152 (152 / 0) | 102 | 33 | 17 | A5 7, A9 10, C7 7 |
| R5-PKG-03 | 8 | 138 (133 / 5) | 97 | 29 | 12 | A5 8, A9 3, C6 1, C7 8 |
| R5-PKG-04 | 6 | 124 (124 / 0) | 55 | 52 | 20 | A5 9, A9 11, C7 6 |
| R5-PKG-05 | 5 | 81 (78 / 3) | 62 | 8 | 11 | A5 5, A9 3, C6 1, C7 7 |
| R5-PKG-06 | 5 | 142 (142 / 0) | 79 | 40 | 23 | A5 7, A9 16, C7 6 |
| R5-PKG-07 | 9 | 234 (222 / 12) | 131 | 73 | 30 | A5 8, A9 21, C6 1, C7 8 |
| R5-PKG-08 | 6 | 151 (145 / 6) | 63 | 58 | 30 | A5 7, A9 22, C6 1, C7 6 |
| R5-PKG-09 | 5 | 157 (152 / 5) | 80 | 51 | 26 | A5 6, A9 20, C7 6 |
| R5-PKG-10 | 5 | 152 (149 / 3) | 87 | 41 | 26 | A5 9, A9 17, C7 9 |
| R5-PKG-11 | 5 | 156 (155 / 1) | 80 | 52 | 24 | A5 5, A9 19, C7 5 |
| R5-PKG-12 | 5 | 126 (124 / 2) | 74 | 37 | 15 | A5 8, A9 5, C6 2, C7 5 |
| R5-PKG-13 | 4 | 74 (71 / 3) | 53 | 8 | 14 | A5 4, A9 7, C6 2, C7 5 |
| R5-PKG-14 | 5 | 109 (107 / 2) | 40 | 36 | 33 | A2 6, A5 5, A9 20, C6 2, C7 5 |
| R5-PKG-15 | 4 | 73 (72 / 1) | 50 | 4 | 19 | A5 4, A9 9, C6 6, C7 4 |
| R5-PKG-16 | 4 | 72 (71 / 1) | 39 | 22 | 30 | A1 19, A5 4, A9 6, B3 19, C1 1, C6 1, C7 4 |
| R5-PKG-17 | 9 | 163 (160 / 3) | 84 | 47 | 34 | A1 2, A5 1, A7 4, A9 23, B6 1, C6 3 |
| R5-XVIEW | corpus | 5 (0 / 5) | 3 | 0 | 2 | A4 1, A6 1 |
| **Total** | 102 + corpus | **2,234 (2,182 / 52)** | **1,280** | **605** | **376** | |

Rows in some columns overlap: a row can wait on an H3 item and a packet at
once. The Deliverables column counts the deliverables that have at least one
row. PKG-01 has three because DEL-01-01 is in its own tranche.

## 4. Classes in scope

| ClassID | Rows | Authority | BlockedOnPacket | Repair (from the class record) |
|---|---|---|---|---|
| T4A-C01 | 198 | NONE | A5;C7 on `DEL-02-04` and `DEL-02-05:CONTEXT#architecture-basis-injection`; guard, not a block, on the 5 A1 injection rows (§2) | Re-pin the keyed `_CONTEXT.md` blocks to revision 0.12 and refresh the scope-item lists |
| T4A-C02 | 213 | OWNER (treatment choice) | A9 (all) | Re-pin to 0.12/DAG-010, or retire the declaration (a history mark on the 5 MEMORY/STATUS units). One choice covers all rows |
| T4A-C03 | 190 | NONE | A1 on `DEL-17-01` and `DEL-17-02:CONTEXT#architecture-basis-injection` (§6) | Re-pin SOW/AB/`.sNN` pins. Name amendments through the latest SCA |
| T4A-C04 | 68 | REVIEW | H3:T4A-C04 (all) | Cleared by the C01 file repair. The reviewer decides the ALIGNED/non-aligned reading. A file-level SEMANTIC_READY refresh would follow A5 |
| T4A-C05 | 86 | NONE | A5;C7 on `DEL-10-05:SOW#CLM-002.r05` (SR-1-adjacent) | Per-row repoint to an existing successor |
| T4B-C05 | 336 | NONE | — | Four-document references become `ScopeOfWork.md`. Retired front matter is marked as history |
| T4B-C06 | 59 | NONE | A6 on the 2 DEL-01-01 rows | Set Last Updated. Reconcile the 3 analogue surfaces |
| T4B-C07 | 21 | NONE | C7 + H3 on DEL-13-02 R01 | Restate or remove each Remaining item, citing the ruling or event |
| T4B-C08 | 59 | NONE | A3 on 1 row; A5 on the 8 T9-C08 rows (2 also carry C7, as they are SR-1-adjacent); A6 on 1 row | Restate each declaration against the record that overtook it. For the 7 PARTIALLY_IMPLEMENTED rows, update the governed artifact |
| T4B-C03 (T8 view) | 42 | OWNER | A5;C7 (all) | SR-1 injection restated after the PKG-00 lifecycle choice |
| T5A-C01 | 205 | REVIEW | H3:T5A-C01 (all); +A5 on DEL-04-01/02/03 `CONTEXT`; +B6 on `DEL-17-05:SOW#CLM-011` | Mark setup-session text as history, or retire it |
| T5A-C02 | 207 | REVIEW | H3:T5A-C02 (all); +A7 on 1 row | Restate declarations against the frozen implementation |
| T5A-C03 | 94 | REVIEW | H3:T5A-C03 (all) | Same as C02 |
| T5A-C04 | 19 | REVIEW | H3:T5A-C04;A1;B3 (all); +C1 on `DEL-16-02:SOW#CLM-026` | Document the applier seam in the DEL-16-02/03 SOWs, once the engine choice is made |
| T5A-C07 | 6 | REVIEW | H3:T5A-C07 (all) | Restate the landed baseline in `_STATUS.md` Remaining |
| T5A-C08 (T8 view) | 2 | REVIEW | H3:T5A-C08;A5;C7 | `DEL-10-01:SOW#CLM-004` and `DEL-10-03:SOW#CLM-004` |
| T5B-C03 | 15 | NONE | — | Pending-review clauses marked complete. Test counts relabelled as history |
| T5B-C04 (T8 view) | 39 | OWNER | A5;C7 (all) | Same as T4B-C03 |
| T5B-C05 | 71 | NONE | C6 on 1 row; A6 on the DEL-01-01 row | Remove ruled items from Still-TBD lists. Mark the gate rule superseded (D-43) |
| T5B-C06 | 226 | NONE | C6 on 4 rows; C7 on the 2 DEL-05-02/05-03 AC-001 rows (formerly U1) | Restate TBDs as settled, citing the ruling. Keep the remainder open |
| T5B-C08 | 9 | NONE | A5;C7 on `DEL-09-05:SOW#CLM-003` | Restate the write scope as the paths the deliverable owns |
| T5B-C10 | 8 | NONE | A2 on the 6 DEL-14-02 rows | Describe the D-67 0.2 contract. Repoint the DEL-00-06 enum anchor |
| T7-C11 | 9 | NONE | — | Align construction notes with the schema |
| T9-REMAINING_CENSUS | 13 ITEM | NONE | Inherited from the mismatch items each one names (H3:T5A-C07 on 3) | Census check after the item repairs |
| T9-C12/SRE-3, 4, 6, 7 | 4 ITEM | NONE | A6 (SRE-4), A4 (SRE-7) | Cross-view only (see §7) |
| T11 D-02 | 15 ITEM | NONE | C6 (formerly U2) | Local `Dependencies.csv` Status against DAG-010 |
| T11 S-02 | 3 ITEM | NONE | A5;C7 | Unassessed SR statement in DEL-07-03/04/06 |
| T11 S-03 | 1 ITEM | NONE | — | Same Still-TBD text repair across three cause signatures |
| T11 S-04 | 16 ITEM | NONE | — | Still-TBD list present but never sub-claimed |

The class counts reproduce as follows. For CLAIM rows, filter
`CLASS_ASSIGNMENTS.csv` on `Route == R5_RECORD_REPAIR`. Add the
`T8_ROWS.csv` rows with `Route == R5_RECORD_REPAIR` whose class route is
not R5. For ITEM rows, filter `T9_LIFECYCLE.csv` and `T11_METHOD.csv` on
`Route == R5_RECORD_REPAIR` and drop the Items that are claim keys.

## 5. Blockers: what each row waits on, and who holds it

| Token | Rows | Holder | Why these rows wait |
|---|---|---|---|
| A9 | 213 | OWNER | T4A-C02 needs one treatment choice, (a) re-pin or (b) retire, inside the R5 ruling |
| H3:T5A-C01 / C02 / C03 / C04 / C07 / C08, H3:T4A-C04 | 604 | WORKING_ITEMS (workflow: review) | The class authority is REVIEW (topic file H3). The token is the ClassID, and `H3_TOKEN_MAP.csv` resolves it to the H3 register item (ER-02, ER-15–ER-20). With the one row that carries only a T9 token, Wave B has 605 rows |
| H3:T9-C02/…R01, H3:T9-C04/…R01 | 2 | review (ER-20, ER-24) | T9 routes `DEL-12-01:STATUS#remaining/R01` and `DEL-13-02:STATUS#remaining/R01` to REVIEW, and the class routes them to R5 (§6) |
| A5 | 101 | OWNER | The SEMANTIC_READY lifecycle target and the review/lifecycle states (T9-C12 SRE-5). The 101 rows are: 81 SR-1 core rows; 6 SR-1-adjacent rows; 6 more T9-C08 rows (the other 2 of the 8 are among the adjacent rows, so they are counted once); 3 T11 S-02 items; the 3 DEL-04-0x `CONTEXT` rows; and the `DEL-02-04` and `DEL-02-05:CONTEXT#architecture-basis-injection` rows (A5 §5 and §9) |
| C7 | 95 | OWNER (confirmation of Agent 0 readings) | Three readings. The SR-1 cause reading RECORD_DRIFT (label only; it does not change the repair), on the SR-1 rows and the two DEL-02-04/05 rows. The unit-vocabulary reading on `DEL-13-02:STATUS#remaining/R01`. And the wave-level reading of the 2 DEL-05-02/05-03 AC-001 rows, formerly U1, which Agent 0 placed in C7 |
| A1 | 21 | OWNER | The 19 T5A-C04 rows: the repaired SOW must not choose an engine that DEC-009 has yet to settle. The 2 DEL-17-01/02 injection rows: A1 asks for them to be held (A1 §5 and §9). The T8 view that they are not blocked is kept visible in §6 |
| B3 | 19 | OWNER | T5A-C04. The runtime applier ownership (T5A-C04 is a named B3 source) must be settled before the repaired SOW names a seam |
| A2 | 6 | OWNER | The 6 DEL-14-02 T5B-C10 rows. A2 says its answer on the hash basis and label settles these rows (A2 §5 and §9) |
| B6 | 1 | OWNER | `DEL-17-05:SOW#CLM-011`. B6 says a text catch-up on this row would silently ratify the mounted panel (B6 §3, §5 and §9) |
| C1 | 1 | OWNER | `DEL-16-02:SOW#CLM-026` waits on the DEL-16-02 runtime schema-validation intent (T5A-C05 REQ-16-02-002) |
| A6 | 6 | OWNER | The ISSUED DEL-01-01 change path: the 5 tranche rows and the SRE-4 view |
| C6 | 20 | OWNER or HELPS_HUMANS | 5 rows wait on the tier for a "Still TBD item since ruled". The 15 T11 D-02 items (formerly U2, which Agent 0 placed in C6) wait on whether syncing the dependency mirror is record repair or DAG-path work (§8). The 5 rows are the CONTESTED `DEL-15-02:CONTEXT#architecture-basis-injection.s02` and the 4 OBSERVED FG-06 rows (`DEL-15-02:SOW#CLM-004.r07`, `CLM-012.r02`, `CLM-019.r09`, `CLM-020.s03`) |
| A7 | 4 | OWNER | T8-K5 reads the deleted export-plan citation on these class-R5 rows as joining the owner population (§6) |
| A3 | 1 | OWNER | `DEL-00-01:AB#open-holds-and-routed-questions.s01`. The CONTESTED split sends the state-library clause to CP-10 |
| A4 | 1 | OWNER | SRE-7 view. Its 85 rename rows go to A4 and not to R5 |

**Conditional dependency, not marked on rows.** T5B observation 2 says that
if C6 rules PROJECT_BASELINE for "Still TBD item since ruled", then T5B-C05
and parts of T5B-C06 (and the matching T11 S-03/S-04 repairs) would move
from R5 to owner confirmation. The evidence cannot say today which T5B-C06
rows would move, so only the five T5B rows named in the evidence carry C6 on this ground. If
C6 is not ruled before R5 is authorised, hold T5B-C05 and T11 S-03/S-04 as
well.

## 6. Rows where the task view differs from the class route

These rows show both views. The route is not chosen here.

| Rows | Class view | Task view | H4 treatment |
|---|---|---|---|
| 81 SR-1 core (T4B-C03 42, T5B-C04 39) | OWNER_DECISION | T8-K1: R5_RECORD_REPAIR once the owner acts on the PKG-00 status | Included, blocked A5;C7 |
| `DEL-10-01:SOW#CLM-004`, `DEL-10-03:SOW#CLM-004` (T5A-C08) | REVIEW | T8-K1 adjacent: R5 | Included, blocked H3:T5A-C08;A5;C7 |
| `DEL-17-07:SOW#CLM-004`, `CLM-027`, `DEL-17-09:SOW#CLM-007` (T4B-C05); `DEL-17-08:SOW#CLM-007/X-002` (T5A-C02) | R5 | T8-K5: OWNER_DECISION (the plan citation joins the owner population) | Included, blocked A7. The four-document or doc-behind-code part of each row could be repaired, but the plan pointer could not. Keep the row whole |
| `DEL-17-01:CONTEXT#architecture-basis-injection`, `DEL-17-02:…` (T4A-C03) | R5 | T8-K2: NO_ACTION (no DEC-009 element) | Included and blocked A1, as Agent 0 decided (the conservative choice). **Other view:** T8-K2 and the first H4 draft find no DEC-009 part (T4A-C03 RESOLVED_PAIR), which would leave only the amendment-list pin as a C03 repair, not blocked. A1 asks for the rows to be held until it rules (A1 §5 and §9) |
| `DEL-12-01:STATUS#remaining/R01` (T5A-C07), `DEL-13-02:STATUS#remaining/R01` (T4B-C07) | R5 | T9: REVIEW (T9-C02; T9-C04) | Included, blocked on H3. The DEL-13-02 item also needs the human disposition of finding PKG13-DEL-13-02-PKG02-001 |
| 8 T9-C08 rows (`DEL-06-02:SOW#CLM-024.r05`/`.r06`, `DEL-08-01:SOW#CLM-003`, `DEL-10-04:SOW#CLM-003.r08`, `DEL-12-05:SOW#CLM-003.r11`/`CLM-020.r09`/`CLM-028.r06`, `DEL-17-02:SOW#CLM-003.r04`) | R5 | T9-C08: R5 ("no lifecycle change needed"); the T9-C12 table puts the same rows in SRE-5 → OWNER_DECISION | Included, blocked A5. T9 disagrees with itself here, and A5 owns SRE-5 |

## 7. The ITEM rows

- **T9 REMAINING_CENSUS (13).** These are the deliverables whose census
  route is R5: DEL-05-02, 05-04, 07-03, 07-05, 08-06, 09-01, 09-02, 09-04,
  09-05, 10-02, 10-03, 10-04 and 11-02. Each census item names mismatch
  items that are already CLAIM rows. The item closes when those rows are
  repaired and the census is re-read at R6 (A4: "Remaining is executable
  truth"). It inherits their blockers. `DEL-05-04 R01` has a choice: record
  the held negative (FG-DEL-05-04-01), or rule it out of scope. The second
  would be an owner or scope choice (T4B-C07).
- **T9-C12 SRE-3 (25 rows), SRE-4 (737), SRE-6 (56), SRE-7 (436).** These are
  corpus views over rows in T4A–T5B. SRE-4 contains the 9 DEL-01-01 pins
  (T4A-C08), which are not in H4. SRE-7 contains the 85 rename rows (T4B-C01,
  A4), also not in H4. At R6, compare each view's filter with the repaired
  rows.
- **T11 D-02 (15 deliverables).** There are 30 local `Dependencies.csv` rows
  that are ACTIVE, while DAG-010 lists them as RETIRED
  (`execution/_DAG/DAG-010/DAG-010_DuplicateEdgeWorklist.csv`). This
  contradicts `execution/_DAG/DAG-010/DAG_Audit.md:15` at the freeze, which
  calls the local files synchronized mirrors. The three ALIGNED ledger rows
  that rely on the local state (`DEL-07-08:SOW#CLM-005.r01`/`.r04`,
  `DEL-08-04:SOW#CLM-020.r06`) are named in the T11 finding. They are not
  divergent claim rows. Blocked on C6 (formerly U2).
- **T11 S-02 (DEL-07-03, 07-04, 07-06).** The SR statement has no sub-claim.
  The repair is the SR-1 repair, so these rows are blocked A5;C7.
- **T11 S-03 (corpus).** Covers 67 architecture-basis Still-TBD sub-claims
  and 8 MEMORY sub-claims, split across three cause signatures. They need
  one text repair. The rows are counted in their classes.
- **T11 S-04 (16 deliverables).** The same injected Still-TBD list appears
  (for example freeze `execution/PKG-14_*/1_Working/DEL-14-01_*/_CONTEXT.md:55`
  shows the list form), but no sub-claim was minted. The repair extends the
  T5B-C05 pass to these files. At R6 the accounting has no claim key for
  these edits. **R6 needs a rule for accounting edits made without a claim
  key.** That rule is part of the R5 authorization.

## 8. Former UNASSIGNED items (now placed by Agent 0)

- **U1 → C7.** `DEL-05-02:SOW#completion-and-reliance-basis-epistemology/AC-001`
  and `DEL-05-03:…/AC-001` (T5B-C06, CONTESTED). A boundary reading makes
  the row met; a declaration reading makes it stale. T5B says this "needs
  one wave-level reading". The rows now carry `C7`.
- **U2 → C6.** T11 D-02 routes the local dependency-mirror drift to R5. The
  profile §8 routes "Dependency graph changes" to "A separate owner-directed
  DAG rebuild". Syncing mirror Status to DAG-010 leaves the approved graph
  unchanged, but it does edit dependency registers. Whether this is record
  repair or DAG-path work is now a C6 question. The 15 items carry `C6`.

## 9. What an owner authorization of R5 would need to say

1. Which tranches and waves are authorised. Wave A is authorisable today.
   Waves B and C are authorised as their H3 items and packets clear.
2. The A9 treatment for T4A-C02: (a) or (b).
3. Whether `R5-ISSUED-DEL-01-01` runs at all. This follows A6, including
   whether its non-SOW surfaces (CONTEXT, MEMORY, STATUS) may go through the
   ordinary path.
4. The change path: a chirality-change PR per tranche, and the scope-change
   workflow for the ISSUED tranche.
5. The R6 accounting basis: this row file as the authorised manifest, with
   held and deferred rows recorded explicitly (method R5/R6), and the rule
   for edits without a claim key (§7).
6. **Owner-route rows join the manifest when ruled.** This file holds only
   rows routed to R5 (plus the T8 SR-1 rows). Some rows route to the owner
   today, for example the A3, A4, A8 and A10 portions that the packets say
   block "H4 repair". When a packet rules that such a row gets an R5 repair,
   the row joins the R5 manifest. It enters the tranche of its owning
   deliverable and is accounted for at R6 like any other row. Until then it
   is not an R5 row.

## 10. Coverage check (script, read-only)

The script `_scratch/gen.py` built the CSV and was deleted after the run;
its logic is stated here. For the review revision, a patch script applied
the Agent 0 row decisions (A2 ×6, B6 ×1, A1 ×2, A5;C7 ×2, U1→C7 ×2,
U2→C6 ×15). The reconciliation was then re-run over all 2,099 class R5 rows
once. It checked the following:
- The CLAIM keys equal the union of: class keys with `Route = R5`, T8 keys
  with Route R5, T9 Items with Route R5 that are claim keys, and T12 keys
  with Route R5. The result is 2,182 keys with no duplicates.
- Every class row count equals `CLASS_INDEX.csv` `Rows` for the 20 R5
  classes (total 2,099).
- There are 52 ITEM rows: 17 from T9 and 35 from T11. The T11 rows expand
  the named deliverable lists.
- Each row has one tranche. Tranche totals sum to 2,234.
- Every row whose Authority is REVIEW or OWNER has a non-empty
  `BlockedOnPacket`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Nothing
here claims certification, code compliance, professional approval or
engineering acceptance.
