# C7 — Readings Agent 0 can record without a ruling, presented for owner confirmation

Packet writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
This is a proposal. It repairs, rules and changes nothing. No row value changes
under any reading below; differences from effective values stay
`R3_OBSERVATION`s in `R3/TASKS/T8_ROWS.csv`.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.
`F:` = the freeze `projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`.

## 1. Decision

Six readings turn on convention text and frozen evidence, not on product
substance. T8 proposes that Agent 0 record five of them in `R3_SYNTHESIS.md`;
because C3, C7 and F1 are RULED conventions, they come to the owner for
confirmation, as the SEMANTIC_READY reading did under Direction 8
(`OWNER_DIRECTIONS.md:139`). The sixth is the contested SCA-009 re-point on
DEL-07-03. For each, the owner **confirms** the reading or **replaces** it.

| ID | Reading | Source | Rows |
|---|---|---|---|
| R1 | SR-1 cause is `RECORD_DRIFT` | T8-K1 | 87 |
| R2 | Unit-vocabulary tier follows the gap (INVARIANT for unit-safety gaps; PROJECT_BASELINE for the enum catch-up); AuthorityNeeded REVIEW; DEL-15-03 split out | T8-K3 | 23 |
| R3 | F1 reaches CONTEXT-typed rows | T8-K4 | 3 |
| R4 | Scope-item test for the tier of in-scope REQs | T8-K6 | 7 |
| R5 | Acceptance-workflow cause is `DEFERRED_BY_RULING` | T8-K7 | 12 |
| R6 | The sanctioned SCA-009 re-point on DEL-07-03: confirm or replace the effective reading on 6 CONTESTED rows | T6-C09 | 9 (6 contested) |

**Holder: OWNER** (confirmation of convention readings; no new decision on
substance). Boundaries: the SEMANTIC_READY **lifecycle target** is A5; the
**selection** of the acceptance workflow (OI-007) is B12; the DEL-15-03
redaction substance is C1. C7 owns only the cause and tier readings.

## 2. Background

- **Conventions** (`RUN/CONVENTIONS.md`): C3 tiers (line 200), C7 one cause and
  the gap's cause wins (line 289), F1 (line 406), F3 with the Direction 8
  readiness clause (line 428), F8 tier of the gap wins (line 469). CS-07
  (`RUN/CANONICAL_SITUATIONS.md:36`) says Context Envelope blocks are judged
  normally.
- **Direction 8** confirmed the SEMANTIC_READY disposition and said the status
  is outdated and should be advanced on later approval; the cause split
  (42 RECORD_DRIFT / 39 SCOPE_REDIRECTED_BY_RULING across 81 OWNER_CONFIRMED rows)
  was left for R3 (`W3_ASSESSMENT.md:46`).
- **Governing evidence per reading (freeze).**
  - R1: `F:execution/_Decomposition/SOFTWARE_DECOMP.md:432` and `:86` keep PKG-00
    at SEMANTIC_READY as architecture-basis candidate (SCA-001); `:35` records
    D-43/SCA-006 as scope-neutral; all eight PKG-00 `_STATUS.md` read
    IN_PROGRESS (T8-K1).
  - R2: OPS-K-UNIT-1 (`F:docs/CONTRACT.md:33`); `F:schemas/units.schema.yaml:259`
    and `F:docs/SPEC.md:174` carry `force_per_length` (T8-K3).
  - R4: scope items SOW-072, SOW-023, SOW-074, SOW-075 at
    `SOFTWARE_DECOMP.md:180`, `:131`, `:182`, `:183` (T8-K6).
  - R5: PRD §21.3 (`F:docs/PRD.md:1340-1344`); DEC-081
    (`SOFTWARE_DECOMP.md:672`); OI-007 (`:575`); `F:core/project_persistence/service.py:127`
    seeds empty acceptance refs (T8-K7).
  - R6: SCA-009/DEC-094 (`SOFTWARE_DECOMP.md:41`, `:685`) re-point the
    DEL-07-03 R-005/R-006 landing to DEL-07-09; DEL-07-03 `EDITOR_KINDS`
    excludes load-case and support kinds (`F:core/gui/editors/engine.py:26`, T6-C09).

## 3. Options

For each reading: (a) **confirm**; (b) **replace** with the competing reading
in the evidence.

- **R1.** (a) RECORD_DRIFT for all 87 (a readiness label lags the status
  records; no ruling changed what the sentence asks). (b) SCOPE_REDIRECTED_BY_RULING
  (the G1 reading carrying the CS-05 gate supersession over). Route is
  R5_RECORD_REPAIR after A5 either way; (b) sends one situation to two classes.
- **R2.** (a) As in the table; rows route CODE_FIX_CANDIDATE under REVIEW, with
  engineering naming legitimately dimensionless fields for DEL-03-07. (b) Keep
  the effective AuthorityNeeded OWNER on DEL-03-07 (routing it with C1 subject 3)
  and a PROJECT_BASELINE tier on DEL-13-01 REQ-13-01-006. Under (a) the eight
  CONTESTED DEL-13-02 rows stay in H2; if Agent 0 instead settled the cluster as
  an owner item, they would move to an owner route (T7-C06 exceptions).
- **R3.** (a) F1 reaches CONTEXT rows: DEL-14-01/02 `CLM-024` read
  PARTIALLY_IMPLEMENTED and join their FindingGroups; DEL-05-02 `CLM-021` goes to
  REVIEW. (b) CONTEXT purpose rows may stay ALIGNED with a GAP_WORDING_CHECKED
  clause.
- **R4.** (a) A REQ reads PROJECT_BASELINE only when the scope item itself names
  the unmet element: DEL-14-02 r09 and DEL-07-05 ×2 PROJECT_BASELINE; DEL-17-02 ×2,
  DEL-17-07, DEL-17-09 LOCAL_DESIGN. (b) Any REQ tracing to an IN scope item
  reads PROJECT_BASELINE (which empties LOCAL_DESIGN for REQs). (c) All seven
  stay as sealed (DEL-07-05 ×2 and the PKG-17 rows LOCAL_DESIGN, DEL-14-02 r09
  PROJECT_BASELINE), with no corpus test recorded.
- **R5.** (a) DEFERRED_BY_RULING for DEL-05-04 and DEL-15-04 (the MVP carries no
  acceptance workflow; DEC-081 creates none). T8 rates this MEDIUM confidence:
  DEC-081 states what it does not create rather than "defer". (b) PARTIAL_SLICE
  for DEL-05-04 (its schema slice landed; enforcement did not).
- **R6.** (a) Confirm the effective ACCEPTED_DIVERGENCE · OWNERSHIP_ELSEWHERE ·
  PROJECT_BASELINE: SCA-009 is a named ruling permitting the state (C6(g)); an
  optional R5 repair adds the DEL-07-09 citation. (b) Adopt the contested
  candidate UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE · OWNER: the rows become
  an owner decision on whether SCA-009 closes R-002, R-005 and R-006 for DEL-07-03
  ("sanctioned split closes them" vs "visibility is not closure", per CLM-018).

## 4. Evidence and reliability

| Source | Reliability | Shows |
|---|---|---|
| `RUN/R3/TASKS/T8_CLUSTERS.md` K1, K3, K4, K6, K7; `T8_ROWS.csv` | R3 proposal | Readings, members, observations |
| `RUN/R3/TASKS/T6_CLASSES.md` T6-C09 | R3 proposal | Re-point class and contested rows |
| `RUN/R3/TASKS/T11_METHOD.md` S-01 | R3 proposal | Independent count of the SR-1 split |
| `RUN/R3/T8_ROUTE_DISAGREEMENTS.csv` | Deterministic | Class route vs T8 route |
| Adopted `RESOLUTIONS.csv` (W1, W2, W3, CROSS_WAVE); `CORPUS_CROSS_WAVE_VERIFICATION.md` | Verifier resolutions | CONTESTED, OWNER_CONFIRMED, RESOLVED_PAIR rows |
| Freeze lines in §2 | Governing text and code, read by T8/T6 | Basis of each reading |

This writer relied on T8's and T6's freeze citations and did not re-open them.
T8 disclosed read-only `git` use inside the freeze checkout and re-derived every
cited fact with plain `grep`/`find` (T8 observation 5).

## 5. Affected claims

C7 discusses **T6-C09 whole** (route carrier `NO_ACTION_ROWS.csv`) and the
**T8 rows of five clusters** (filter). It does not claim portions of the classes those T8 rows sit in; those classes are
counted by their packets or handoffs, which carry `BlockedOnPacket C7` where a
reading changes their route.

| Set | Rows | Filter |
|---|---|---|
| T6-C09 | 9 (whole class; discussed, not claimed) | `CLASS_ASSIGNMENTS.csv` `ClassID == T6-C09` |
| T8 rows, R1–R5 | 132 | `R3/TASKS/T8_ROWS.csv` `Cluster` in {SR-1, UNIT_VOCABULARY, F1_ON_CONTEXT, TIER_IN_SCOPE_REQ, ACCEPTANCE_WORKFLOW} |

**T6-C09 keys.** `DEL-07-03:SOW#CLM-005.r04`*, `SOW#CLM-005.r05`,
`SOW#CLM-008/DEL-07-03-R-005`*, `SOW#CLM-008/DEL-07-03-R-006`,
`SOW#CLM-012/DEL-07-03-R-002`*, `SOW#CLM-012/DEL-07-03-R-005`*,
`SOW#CLM-012/DEL-07-03-R-006`, `SOW#CLM-017`*, `SOW#CLM-018`* (* = CONTESTED, 6).
All nine also appear in `R3/NO_ACTION_ROWS.csv` (class route NO_ACTION), which
Agent 0 records; C7 only asks the owner to confirm or replace the reading.

**Where the T8 rows sit, and both route views** (from `CLASS_ASSIGNMENTS.csv`
and `T8_ROUTE_DISAGREEMENTS.csv`):

| Cluster | Rows | Classes (owner) | T8 route | Rows where class route differs |
|---|---|---|---|---|
| SR-1 | 87 | T4B-C03 42, T5B-C04 39 (A5); T4B-C08 2, T5A-C08 2, T5B-C08 1, T4A-C05 1 (H3/H4) | R5_RECORD_REPAIR | 83: class OWNER_DECISION 81 (A5), REVIEW 2 |
| UNIT_VOCABULARY | 23 | T7-C05 11 (C1); T7-C06 10, T6-C03 1 (H2); T4B-C07 1 (H4) | CODE_FIX 14, OWNER 8, R5 1 | 3: DEL-03-07 ×3, class OWNER_DECISION (C1) |
| F1_ON_CONTEXT | 3 | none (not divergent) | CODE_FIX 2, REVIEW 1 | 3: NOT_DIVERGENT |
| TIER_IN_SCOPE_REQ | 7 | T6-C01 4, T6-C02 1 (H2); T6-C04 2 (B12) | NO_ACTION 4, OWNER 2, CODE_FIX 1 | 4: DEL-17-02 ×2, 17-07, 17-09 — class CODE_FIX (H2) vs T8 NO_ACTION for the tier only |
| ACCEPTANCE_WORKFLOW | 12 | T6-C04 9 (B12); T6-C08 2 (`NO_ACTION_ROWS.csv`); 1 not divergent | NO_ACTION | 10: DEL-05-04 ×9 class OWNER_DECISION (B12); DEL-15-04 AC-001 NOT_DIVERGENT |

- For the four PKG-17 tier rows, T8's NO_ACTION concerns only the tier; their
  implementation gaps stay with T6 (H2). Both views apply at once.
- For DEL-05-04 ×9, R5(a) reads the cause as a deferral; B12 still owns whether
  and which acceptance workflow is selected.

**Rows known only from `OtherCorrections`.** SR-1: the 81 OWNER_CONFIRMED
resolutions and 6 adjacent CONTESTED/OBSERVED/WEAK rows (T8-K1). Unit
vocabulary: 8 CONTESTED DEL-13-02 rows; FIRM DEL-13-01 `CLM-005.r05`; OBSERVED
DEL-03-07 `CLM-021.s02`; FIELD DEL-15-03 tier corrections (4 rows, not applied).
F1: 3 CONTESTED. Tier: 7 RESOLVED_PAIR. Acceptance: 4 RESOLVED_PAIR, 1 CONTESTED.
T6-C09: 6 CONTESTED.

**Packages.** PKG-01 to PKG-17 (SR-1 spans PKG-01–16); DEL-07-03 for R6.

## 6. Risks

- **Undecided.** One situation lands in two R4 classes (SR-1: record drift and
  ruling redirect; acceptance: deferral and partial slice), so H4 tranches and R6
  counts split. 87 context surfaces keep an outdated readiness label feeding
  sealed briefs. Purpose rows claim coverage the product lacks (R3). An in-scope
  capability can route as local design (R4). R-002 closure stays ambiguous
  between DEL-07-03 and DEL-07-09 (R6).
- **Confirming.** R5's MEDIUM confidence: the deferral rests on PRD §21.3 more
  than on DEC-081's wording. R2 moves DEL-03-07 out of C1 into H2.
- **Replacing.** R1(b), R5(b) keep the splits. R6(b) opens a new owner decision
  on an already-ruled re-point.

## 7. Recommended routing

The evidence supports confirming R1, R3 and R4 as T8 reads them; each follows
from convention text with no competing ruling found. R2 is supported except for
the DEL-03-07 AuthorityNeeded, where engineering input is needed on
dimensionless fields; the evidence does not settle whether that makes it an
owner item. R5: no recommendation (MEDIUM confidence). R6: the evidence supports
confirming the effective reading (a named ruling permits the state); the six
contested rows show the replacement is defensible, so the owner's call stands.

## 8. On-ruling mechanism

- **Confirm (a).** Agent 0 records the reading in `R3_SYNTHESIS.md` as
  owner-confirmed. No sealed ledger is patched (F6). Rows join their routes:
  R1 — R5 record repair of the readiness sentence after A5's lifecycle act,
  under separate R5 authorization (D-73); R2 — H2 briefs (with the pending human
  disposition of `PKG13-DEL-13-02-PKG02-001`) and engineering input for DEL-03-07
  via H3; R3 — the rows join FG-DEL-14-02-02 / FG-DEL-14-01-02 routing, DEL-05-02
  to H3 review; R4 — DEL-07-05 ×2 go to B12's owner routing at PROJECT_BASELINE;
  R5 — NO_ACTION until an acceptance workflow is authorised (B12); R6 — optional
  R5 citation repair on DEL-07-03.
- **Replace (b), or R4(c).** Agent 0 records the replacement reading; rows are recounted in
  the partition before H4 is finalised. R6(b) opens a DEC/SCA clarification owner
  decision on SCA-009 closure.
- If a reading should bind later runs, a convention amendment through HELPS_HUMANS.
- Nothing executes until the owner acts. R5 needs separate authorization.

## 9. Dependencies

- **Depends on:** B4 (DEC-094 and the DEL-07-09 envelope bear on R6; B4 lists
  C7 as blocked by it).
- **Blocks:** A5's R5 follow-through (R1 sets the cause the repair records); C1
  subject 3 route (R2); H2 unit-vocabulary briefs (R2); H4 SR-1 rows and H2/H3
  rows of the F1, tier and acceptance clusters (BlockedOnPacket C7).
- **Placed here by Agent 0 (question attached to the owner session; options not
  drafted):** U1 — one wave-level reading of the DEL-05-02/05-03 AC-001 rows:
  `DEL-05-02:SOW#completion-and-reliance-basis-epistemology/AC-001`,
  `DEL-05-03:SOW#completion-and-reliance-basis-epistemology/AC-001` (T5B-C06).
- **Related:** B12 (acceptance-workflow selection; DEL-07-05 ratio choice); C6 (method items that are not T8
  clusters).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
