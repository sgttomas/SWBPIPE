# R4 gate index

Agent 0 integration record for the R4 owner gate of run
HELP-HUMAN-PIPING-20260921-RECONCILIATION. It lists every packet and handoff,
shows that every divergent row has one route owner, and places the items the
drafters found on no topic list. It proposes; nothing here rules. R5 and R6
need separate owner authorization (D-73 authorizes R0–R4).

## 1. What the owner is asked to decide

29 decision packets, all held by the OWNER, some then executed by another
holder:

| Set | Packets | Index |
|---|---|---|
| P1 authority, baseline, architecture | A1–A10 | `DECISION_PACKETS/P1/INDEX.md` |
| P2 ownership, scope, product path | B1–B12 | `DECISION_PACKETS/P2/INDEX.md` |
| P3 product intent, evidence, validation, method | C1–C7 | `DECISION_PACKETS/P3/INDEX.md` |

Four handoffs carry the non-owner routes. Each row in them names the packet or
H3 item it waits on (`BlockedOnPacket`).

| Handoff | Holder | Content | Folder |
|---|---|---|---|
| H1 scope change | WORKING_ITEMS (workflow: scope-change), after the owner | 241 items over 227 capabilities and 17 T6-C07 rows | `R3/SCOPE_CHANGE_HANDOFF/` |
| H2 code-fix candidates | production brief, after any blocking ruling | 54 briefs, 386 rows | `R3/CODE_FIX_BRIEF_CANDIDATES/` |
| H3 engineering and review | ENGINEERING; WORKING_ITEMS (workflow: review) | 35 items | `R3/ENGINEERING_AUTHORITY/` |
| H4 R5 tranche proposal | R5, once separately authorized | 2,234 rows in 20 tranches | `R4/R5_TRANCHE_PROPOSAL/` |

H4 and H2 finished before H3 existed and name review items by class. The
mappings to H3 IDs are `R4/R5_TRANCHE_PROPOSAL/H3_TOKEN_MAP.csv` and
`R3/CODE_FIX_BRIEF_CANDIDATES/H3_TOKEN_MAP.csv`.

## 2. Coverage of the 3,299 divergent rows

Every divergent row has exactly one class (`R3/CLASS_ASSIGNMENTS.csv`), and
every class route has one carrier:

| Class route | Rows | Carrier | Check |
|---|---:|---|---|
| OWNER_DECISION | 444 | A1–C7 | `tools/check_r4_coverage.py` → `R4/PACKET_CLAIMS.csv`: every row in exactly one packet; all 24 stated portions match |
| R5_RECORD_REPAIR | 2,099 | H4 | every class row once (Agent 0 check at H4 return) |
| CODE_FIX_CANDIDATE | 375 | H2 | every class row once; 386 with T8/T12 and W3 items |
| REVIEW | 295 | H3 (ER-01..07) | every class in an item |
| ENGINEERING_AUTHORITY | 11 | H3 (ER-08..14) | every row in an item |
| SCOPE_CHANGE_HANDOFF | 17 | H1 | all 17 keys present |
| NO_ACTION | 58 | `R3/NO_ACTION_ROWS.csv` | recorded for R6 accounting; T6-C09 and T7-C08 are put to the owner in C7 and C3 |

The capability views are covered separately: H1 carries the 227 capabilities
needing a scope action, and B1–B6 carry the 101 whose owner is an owner
question.

**Split classes.** The topic file listed four split classes. The drafters
found three more, each split cleanly with no row claimed twice. All seven are
in the coverage script.

| Class | Portions |
|---|---|
| T4A-C06 (14) | A1 7 · A7 5 · A10 2 |
| T5B-C07 (22) | A6 16 · A10 6 |
| T5A-C05 (17) | C1 14 · B6 3 |
| T6-C04 (87) | B12 67 · B10 20 |
| T4B-C01 (86), not listed | A4 85 · A6 1 |
| T4B-C02 (5), not listed | A6 4 · A2 1 |
| T5B-C09 (5), not listed | C4 4 · B4 1 |

## 3. Deliberate dual views

These rows appear in two places on purpose. The owner rules the standard or
the choice, and another holder does the work. Neither place repeats the
other's options.

- **T7-C09 and T5B-C02 (protected-content review).** C2 asks the owner what
  standard satisfies the claim. H3 ER-06 and ER-04 carry the review itself.
- **T7-C07 on DEL-05-03 (2 rows).** C3 asks whether the pressure reference
  hold stays. H3 ER-12 carries the engineering question once the hold lifts.
- **NO_ACTION classes put to the owner.** T7-C08 (10 rows) is discussed in
  C3, and T6-C09 (9 rows) in C7. Both stay recorded in
  `R3/NO_ACTION_ROWS.csv` unless the owner's answer changes their route.
- **T7-C04 (A8).** A8 counts the rows. B5 (palette landing) and B12
  (CF-001/CF-002, secret provider) draft the choices. Rule A8 with B5 and B12.
- **PB-TBD-003.** B12 D14 and A10's release-label floor are one subject.
  Rule them together.
- **Keys decided in a packet while their class route stays with a handoff.**
  `DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-08` (C2) and
  `DEL-11-04:SOW#CLM-011/R-DEL-11-04-002` (C4). Both are marked blocked on
  their packet in `CODE_FIX_ROWS.csv`. C5 decides 11 DEL-17-07/08 rows that
  stay with H2 and H3. The H2 row `DEL-17-07:SOW#CLM-021` is marked blocked
  on C5.

## 4. Items found on no topic list

The drafters reported these without drafting them, as the brief required.
Agent 0 placed each one with the packet whose owner session is nearest to its
subject. The packet names the item in one line as an attached question; it
does not draft options for it. Every row behind a U-item is marked blocked on
its host packet in the handoffs, so none of them counts as ready.

| ID | Item | Rows behind it | Host | Holder |
|---|---|---|---|---|
| U1 | One wave-level reading of the DEL-05-02/05-03 `…/AC-001` rows (T5B-C06) | 2 H4 rows, token `C7` | C7 | OWNER |
| U2 | Local `Dependencies.csv` rows still show dependencies DAG-010 retired: R5 record repair, or the DAG rebuild outside this run (profile §8) | 15 H4 rows, token `C6`. CFB-38 and CFB-50 carry the profile §8 guard | C6 | OWNER. A DAG rebuild needs separate authorization |
| U3 | DEL-17-02's four export schemas: contract-level tables under DEC-076, or common schema files | H1-241, token `B7` | B7 | OWNER, then scope-change |
| U4 | Does the CONVENTIONS F3 origin test apply to the item or the assessed sentence? | `DEL-12-02:SOW#CLM-037/REXC-OI-002` (T5A-C08, in H3 ER-02) | C6 | OWNER |
| U5 | Narrow an INVARIANT restatement instead of implementing it | `DEL-03-08:SOW#CLM-026` and RQ-004; `DEL-06-01:SOW#CLM-011/REQ-06-01-011` (T6-C03). H2 token `C1` added to the H3 block | C1 | OWNER |
| U6 | Are storage, rule-pack and library commands exempt from the diagnostics envelope? | The CFB-14 rows (DEL-00-03 REQ-03-02, DEL-00-06 REQ-06-03, CLM-004.r05), H2 token `C1` | C1 (diagnostic class) | OWNER |
| U7 | T6-C02 rows offering "narrow or defer by ruling" outside A2, B3, B7, B8. RV6's scan found the remainder: two rows | `DEL-03-01:SOW#CLM-011/REQ-03-01-007`, `DEL-03-01:SOW#production-and-verification-method-praxeology/VER-001` (CFB-09), H2 token `C6` | C6 M2 (the DEL-03-01 round trip) | OWNER |
| U8 | Owner discharge of the deferred DEL-11-03 theory scopes | `DEL-11-03` CLM-010.s02 and CLM-004.r01 (CFB-28), H2 token `C4` | C4 | OWNER |
| U9 | No DivergenceLayers value fits a pure product gap | none (method item; `WAVES/W3/W3_ASSESSMENT.md:98`) | C6 | OWNER; HELPS_HUMANS for the convention |
| U10 | The rerun-launch hint as a method rule | none | already drafted as C6 M11 | OWNER; HELPS_HUMANS |

**Other gaps H1 and P1 reported:**

- **Four T1 REVIEW capabilities with no H3 item** (CAP-COREC-053,
  FEATB-029, FEATB-030, PHYS-021). H3 now carries them as ER-35, and H1 items
  H1-017, 110, 116 and 117 are blocked on it.
- **CAP-PHYS-029** (T3 route ENGINEERING_AUTHORITY, waiting on the
  owner-held pressure reference model). It is attached to C3, beside the
  DEL-05-03 hold, and H1-176 is blocked on C3.
- **84 capabilities T1–T3 route to R5 record repair.** They are outside H4,
  which covers class rows and T9/T11 items only. Their repair follows H1's key
  issue and runs through H1, under separate R5 authorization.
- **CAP-SHELL-013 and CAP-SHELL-028 code-fix readings.** These are outside
  H2's scope. They follow B2 (shell owner). A code-fix brief is drafted only
  after B2 rules.
- **The frozen `LICENSE.md` heading still reads "OpenPipeStress License".**
  P1 saw it in passing and placed it in no packet. It is attached to A4
  (rename ruling).
- **A1 against H4 on the DEL-17-01/02 injection rows.** A1 asks for a
  block; T8 and H4's first reading said none was needed. Agent 0 chose to
  block, because a block costs no repair. H4 carries A1 on those two rows
  and a guard on the five T4A-C01 main injection rows. The other view stays
  in H4 §6.

## 5. Disagreements left visible

The tasks disagree on some routes, and the packets and handoffs show both
views without choosing. The main sets are:

- **T8 against the class route:**
  - DEC-009 rows (A1);
  - export-plan rows (A7);
  - SR-1, 81 rows (A5);
  - DEL-03-07 ×3 (C1/C7);
  - DEL-05-04 ×9 (B12/C7);
  - four PKG-17 in-scope REQ rows (C7);
  - DEL-10-01/10-03 CLM-004;
  - DEL-17-08 X-002.

  The full list is `R3/T8_ROUTE_DISAGREEMENTS.csv` (113 rows).
- **T9 against the class route:** three STATUS Remaining rows (H3 §7), and
  T7-C08 against T9-C11 (C3).
- **T12 against the class route:** DEL-14-01 rows (B10), DEL-15-04 AC-001, and
  DEL-13-01/13-03 CLM-005.
- **Task against task:** the canonical-JSON owner is DEL-08-02 in T1 but
  DEL-02-05 or a new deliverable in T3 (H1-103/104).
- **Worker readings:** the three DEL-05-05 CONTESTED readings (H3).

## 6. Suggested ruling order

Several packets gate others. The order that unblocks the most work is:

1. **A5** (PKG-00 lifecycle) and **C7** (readings). A5 gates 82 owner rows and
   the SEMANTIC_READY text. C7's readings set how 132 T8 rows are routed.
2. **A4** (rename) and **A9** (declaration blocks): 298 rows of mechanical
   R5 work that turn on one treatment each.
3. **B1, B2, B7** (solve path, shell, handoff path). B1 and B2 block 64 of
   the 167 H1 CREATE and ASSIGN items. B7 carries the largest T12 cluster
   (T12-C01, 213 rows).
4. **A1, A2, A3, C1**: the code-direction questions that gate H2 briefs.
5. **The rest.** Each packet's §9 gives its dependencies.

The readiness figures are in the handoffs. H4 has 1,280 rows ready once R5 is
authorized. H2 has 18 briefs with no blocker. H1 has 124 items with no packet
blocker.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
