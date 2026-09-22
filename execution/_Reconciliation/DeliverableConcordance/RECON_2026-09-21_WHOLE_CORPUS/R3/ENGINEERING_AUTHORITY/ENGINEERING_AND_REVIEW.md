# H3 — Engineering and review register (R3 integration)

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H3.
Parent: HELP_HUMAN Agent 0. Brief: `briefs/R3-INTEGRATION_brief.md`
(SHA-256 `58e45596…9016`). Topic assignment: `R3_INTEGRATION_TOPICS.md`
(SHA-256 `026eb58b…f63a`), handoff H3. Frozen source `00115c719` (evidence,
never authority).

This register proposes. It rules nothing, repairs nothing and changes no row,
deliverable, code, lifecycle state, DAG or instruction. Every item names the
holder whose act must come first; under method R4 the affected repair paths
(the H2 code-fix briefs and the H4 R5 tranches that touch the same rows) stop
until that holder acts. R5 needs its own owner authorisation (D-73).

The machine-readable companion is `ENGINEERING_AND_REVIEW_ITEMS.csv` (35
items, CRLF line endings, `#END` row gives the item count).

## 1. Scope and how it was drawn

The H3 scope in the topic file has three parts. All three were drawn by script
from the accepted R3 tables, not by hand.

| Part | Source | Filter | Rows |
|---|---|---|---:|
| Class rows routed REVIEW or ENGINEERING_AUTHORITY | `R3/CLASS_ASSIGNMENTS.csv` | `Route in {REVIEW, ENGINEERING_AUTHORITY}` | 306 |
| Class rows whose authority is REVIEW or ENGINEERING on a repair route | `R3/CLASS_ASSIGNMENTS.csv` | `Authority in {REVIEW, ENGINEERING}` and `Route in {R5_RECORD_REPAIR, CODE_FIX_CANDIDATE}` | 675 |
| Task rows routed REVIEW or ENGINEERING_AUTHORITY | `R3/TASKS/T8_ROWS.csv`, `T9_LIFECYCLE.csv`, `T11_METHOD.csv`, `T12_UNREACHED.csv` (`#END` rows excluded) | `Route in {REVIEW, ENGINEERING_AUTHORITY}` | T8 1, T9 12, T11 12, T12 94 |
| T1 capabilities routed REVIEW (added at review, RV5 finding 2) | `R3/TASKS/T1_UNMAPPED.csv` | Notes route REVIEW; H1 items H1-017, H1-110, H1-116, H1-117 | 4 capabilities |

The class part is 17 classes and 981 claim rows:

| Class | Rows | Authority | Class route | Register item(s) |
|---|---:|---|---|---|
| T4A-C04 | 68 | REVIEW | R5_RECORD_REPAIR | ER-15 |
| T5A-C01 | 205 | REVIEW | R5_RECORD_REPAIR | ER-16 |
| T5A-C02 | 207 | REVIEW | R5_RECORD_REPAIR | ER-17 |
| T5A-C03 | 94 | REVIEW | R5_RECORD_REPAIR | ER-18 |
| T5A-C04 | 19 | REVIEW | R5_RECORD_REPAIR | ER-19 |
| T5A-C06 | 10 | REVIEW | REVIEW | ER-01 |
| T5A-C07 | 6 | REVIEW | R5_RECORD_REPAIR | ER-20 |
| T5A-C08 | 18 | REVIEW | REVIEW | ER-02 |
| T5B-C01 | 175 | REVIEW | REVIEW | ER-03 |
| T5B-C02 | 12 | REVIEW | REVIEW | ER-04 |
| T6-C03 | 34 | REVIEW | CODE_FIX_CANDIDATE | ER-21 |
| T6-C05 | 35 | REVIEW | REVIEW | ER-05 |
| T6-C06 | 4 | ENGINEERING | ENGINEERING_AUTHORITY | ER-08, ER-09, ER-10, ER-11 (1 each) |
| T7-C06 | 42 | REVIEW | CODE_FIX_CANDIDATE | ER-22 |
| T7-C07 | 7 | ENGINEERING | ENGINEERING_AUTHORITY | ER-12 (2), ER-13 (1), ER-14 (4) |
| T7-C09 | 31 | REVIEW | REVIEW | ER-06 |
| T7-C10 | 14 | REVIEW | REVIEW | ER-07 |
| **Total** | **981** | | | |

No class in scope is split with a P1–P3 packet, so every class count here is
the full class count. T6-C06, T7-C07 and T12-C06 are split only among H3's own
items; each portion's keys are listed in §4.

**Reproducing filter (class rows).** Over `R3/CLASS_ASSIGNMENTS.csv`:
`Route in {REVIEW, ENGINEERING_AUTHORITY} OR Authority in {REVIEW, ENGINEERING}`.
That returns exactly the 981 rows of the 17 classes above; no class outside
them matches.

## 2. Coverage reconciliation (script)

A read-only script (`csv` module, `PYTHONDONTWRITEBYTECODE=1`) rebuilt the
scope from the files above and compared it with the item definitions:

- class rows: expected 981, covered 981, duplicates 0, missing 0, extra 0;
  every class count matches `CLASS_INDEX.csv`;
- T8: expected 1, covered 1; T9: expected 12, covered 12; T11: expected 12,
  covered 12; T12: expected 94, covered 94; no duplicates, none missing;
- T1: 4 capabilities routed REVIEW (CAP-COREC-053, CAP-FEATB-029,
  CAP-FEATB-030, CAP-PHYS-021), all in ER-35;
- 35 items; the CSV `Rows` column sums to 1,094 = 981 class rows + 1 T8 row
  + 3 T9 rows that have no in-scope class row (ER-24 2, ER-25 1) + 12 T11 rows
  + 94 T12 rows, less 1 T12 key that is also a T7-C06 class row
  (`DEL-13-01:SOW#CLM-005.r05`, counted once in ER-22), + 4 T1 capabilities
  (ER-35; capabilities, not claim rows).

How the 12 T9 rows and the T12 REVIEW row are carried:

| Task row | Carried by | Note |
|---|---|---|
| T9 `STALE_VV:SRE-1` (169 CP-09 rows, 84 deliverables) | ER-03 | The 169 rows are exactly the T5B-C01 rows with `CanonicalSituation = CP-09`; the other 6 T5B-C01 rows are the VER-001 rows judged with the CP-09 outcome |
| T9 `STALE_VV:SRE-2` (7 rows) | ER-04 | The 7 INVARIANT rows of T5B-C02 |
| T9 `STALE_VV:VNV-B` (T9-C11b, 4 rows) | ER-14 | Same 4 keys as the T7-C07 portion in ER-14 |
| T9 census DEL-14-05 (ENGINEERING) | ER-14 | Item R01 is a T7-C07 row |
| T9 census DEL-02-02, DEL-07-01, DEL-07-06 | ER-07 | Their UNKNOWN items (R01, R05, R04) are T7-C10 rows |
| T9-C02 `DEL-12-01:STATUS#remaining/R01` and census DEL-12-01 | ER-20 | Class T5A-C07 routes R5; T9 routes REVIEW. Both views shown |
| T9-C04 `DEL-13-02:STATUS#remaining/R01` and census DEL-13-02 | ER-24 | Class T4B-C07 (R5, authority NONE) against T9 REVIEW. Both views shown |
| T9 census DEL-01-03 | ER-25 | Item R02 is class T7-C04 (OWNER). Both views shown |
| T12 `DEL-13-01:SOW#CLM-005.r05` (T12-C03, REVIEW) | ER-22 | Also a T7-C06 row and a T8 unit-vocabulary row |

**Rows known only from `OtherCorrections`.** No row outside the 17 classes
has an `OtherCorrections` entry that sets AuthorityNeeded to REVIEW or
ENGINEERING. Three rows outside the classes carry an effective AuthorityNeeded
of REVIEW: `DEL-13-02:STATUS#remaining/R01` (T4B-C07; carried as ER-24) and
`DEL-08-04:SOW#CLM-011.r05` and `CLM-025.r04` (T7-C05, OWNER class, topic C1;
named in ER-31 as context only, not claimed). Within the classes, 92 rows carry
`OtherCorrections`; the ones that bear on the review act (unapplied FIELD cause
corrections, CONTESTED splits, FIRM AuthorityNeeded upgrades) are named in the
item they affect.

## 3. Holders and route labels

| Route label in the CSV | Holder | Meaning |
|---|---|---|
| `REVIEW` | WORKING_ITEMS (workflow: review) | The review act is itself the next step; its outcome routes the row (R5, NO_ACTION or a new code-fix candidate) |
| `REVIEW_BEFORE_R5` | WORKING_ITEMS (workflow: review) | The class routes to R5 record repair, but its authority is REVIEW: the review named here precedes the H4 tranche for these rows |
| `REVIEW_BEFORE_CODE_FIX` | WORKING_ITEMS (workflow: review) | The class routes to a code-fix brief (H2), but its authority is REVIEW: the named independent review gates reliance on the fix |
| `ENGINEERING_AUTHORITY` | ENGINEERING | An engineering convention, qualification basis or validation basis must be recorded before any code fix or record repair |

`BlockedOnPacket` names the P1–P3 topic that must be decided first. "(partial)"
means only the keys listed in §4 wait on that packet; the rest of the item can
proceed once its own holder acts. "-" means no packet blocks the item.

## 4. Register items

Reliability labels used below: **accepted** = effective values in
`CORPUS_CLAIMS.csv` / `CLASS_ASSIGNMENTS.csv`; **task proposal** = R3 task
output; **ledger** = sealed worker ledger Notes or RemainingWork (worker
reading, not a review); **freeze-checked** = a `path:line` this task opened at
the freeze and found as cited; **as cited** = a freeze citation carried from a
task output and not re-opened here.

### 4.1 Review acts that are themselves the route

#### ER-01 — Local semantic departures: adopt or restore (T5A-C06, 10 rows)

- **Holder.** WORKING_ITEMS (workflow: review). Escalate to OWNER only if the
  reviewer finds a product-level choice (T5A proposal).
- **Rows.** All 10 T5A-C06 keys (listed in `T5A_CLASSES.md` T5A-C06
  Population); 7 deliverables: DEL-07-01, DEL-07-02 (2), DEL-09-05 (2),
  DEL-11-02, DEL-12-03, DEL-12-04 (2), DEL-12-05.
- **Question.** Per row, is the implemented semantics adopted (then an R5 text
  catch-up) or does the text prevail (then a new code-fix candidate)?
  Cases: gate-outcome vocabulary (DEL-09-05), credential record kinds and the
  derived transmission guard (DEL-12-04), whole-event telemetry rejection
  stricter than the text (DEL-12-03), AC-001 not describing the built boundary
  (DEL-07-01, DEL-07-02, DEL-11-02), unlisted modules (DEL-07-02 CLM-019), and
  the DEL-12-05 threat model not revised after the export-route redaction
  changes although its own trigger list requires it.
- **Evidence.** Ledger RemainingWork reads each as text catch-up (ledger);
  `DEL-12-05:SOW#CLM-036` records `threat_model.md` last changed 2026-06-06
  (ledger). No resolution rows.
- **What the review act authorises.** Per-row adopt/restore outcomes; adopted
  rows join the H4 R5 tranche of their deliverable; restored rows become H2
  candidates. For DEL-12-05 the output is a recorded threat-model review.
- **BlockedOnPacket.** None. The DEL-07-02 CLM-019 rename residue is A4's, not
  this item's.

#### ER-02 — Cause or vehicle contested in adopted resolutions (T5A-C08, 18 rows)

- **Holder.** WORKING_ITEMS (workflow: review). T5A proposed that Agent 0
  harmonise these causes at R3 integration; that harmonisation was not
  performed at integration (no R3 or R4 record shows it). It passes to the
  ER-02 reviewer, who applies the C7 readings where C7 covers the row (SR-1
  cause, `.s02` rows). One element is an owner convention question (below).
- **Rows.** All 18 T5A-C08 keys (`T5A_CLASSES.md` T5A-C08 Population); 16
  deliverables.
- **Question.** Which cause (and, for `DEL-06-04:MEMORY` and `DEL-06-05:MEMORY`,
  which vehicle) does each row carry before its text repair?
- **Both views (T8 against class).** `DEL-10-01:SOW#CLM-004` and
  `DEL-10-03:SOW#CLM-004`: class route REVIEW (T5A-C08); T8 route
  R5_RECORD_REPAIR as SR-1-adjacent rows (`T8_ROUTE_DISAGREEMENTS.csv`). Not
  chosen here.
- **Unapplied corrections (from `OtherCorrections`, not effective).** FIELD
  cause SCOPE_REDIRECTED_BY_RULING on DEL-00-05/06/07/08
  `CONTEXT#anticipated-artifacts`; FIELD cause RECORD_DRIFT on
  `DEL-01-02:SOW#CLM-010.r03`; FIELD vehicle HISTORY · ALIGNED plus an unminted
  `MEMORY.s01` on DEL-06-04/05. Applying them would move up to 5 rows out of
  T5A (T5A observation 1) and would need a key minted in a later ledger
  revision.
- **BlockedOnPacket (partial).**
  - C7 (SR-1 cause reading): `DEL-03-04:CONTEXT#architecture-basis-injection.s02`,
    `DEL-03-05:CONTEXT#architecture-basis-injection.s02`,
    `DEL-03-06:CONTEXT#architecture-basis-injection.s02`,
    `DEL-10-01:SOW#CLM-004`, `DEL-10-03:SOW#CLM-004`.
  - A5 (SEMANTIC_READY lifecycle target): `DEL-10-01:SOW#CLM-004`,
    `DEL-10-03:SOW#CLM-004`.
  - A3 (the unmatched-classification list, CP-10): `DEL-14-05:SOW#CLM-026.s01`,
    whose repair must wait for owner confirmation of FG-DEL-14-05-03 (the
    FindingGroup is the unmatched-classification schema rows, CP-10, per
    `WAVES/W3/PKG-14/DEL-14-05/DEL-14-05_forward.csv`).
  - The `.s02` batch blind spot is also a C6 method item (dependency, not a
    block).
- **Owner element not on any topic list.** `DEL-12-02:SOW#CLM-037/REXC-OI-002`
  turns on the CONVENTIONS F3 origin test (item identity against the assessed
  sentence), a RULED convention. Reported as UNASSIGNED (§6).

#### ER-03 — SOW parity and claim-map re-review against current bytes (T5B-C01, 175 rows)

- **Holder.** WORKING_ITEMS (workflow: review).
- **Rows.** All 175 T5B-C01 keys; 84 deliverables in 17 packages (every
  package except PKG-00). Also carries T9 item `STALE_VV:SRE-1` (the 169
  CP-09 rows of the class).
- **Question.** Re-run the SOW parity and claim-map review of each affected
  `ScopeOfWork.md` at its current bytes, so each `OUT-001`/`VER-001` row cites a
  PASS record whose production hash equals the reviewed SOW.
- **Evidence.** `EVIDENCE_MAP.csv`: PASS records exist but
  `AnyPassMatchesFrozen = NO` (task proposal, from the R2 evidence map).
  Example: `DEL-02-02:SOW#output-and-evaluation-matrix/OUT-001` parity hash
  `3fd8885d…` against frozen SOW `d84a099a…` (T5B, as cited).
- **Sequencing.** The re-review must follow the same deliverable's SOW text
  repairs (the H4 tranches for T5A-C01–C03, T5B-C03/C05–C08/C10 and T4A rows);
  a parity record made first would go stale again under CP-09 (T5B sequencing
  note). This is the reverse of the other REVIEW_BEFORE_R5 items: here R5 comes
  first and the review follows.
- **BlockedOnPacket (partial).**
  - C6 (CP-09 scope, W2 convention question "CP-09 on VER-001"):
    `DEL-05-02:SOW#purpose-and-objective-traceability/OUT-001` and
    `DEL-05-03:SOW#purpose-and-objective-traceability/OUT-001` (CONTESTED:
    is a purpose-block OUT-001 a CP-09 row at all?).
  - B7 (canonical handoff path; the DEL-15-02 defaults wait on B7 per H2):
    `DEL-15-02:SOW#production-and-verification-method-praxeology/VER-001`,
    the one PARTIALLY_IMPLEMENTED row, which also needs verification of
    defaulted inputs and provenance.
- **Visible exceptions.** OBSERVED (DecisionBasis empty, no value change):
  DEL-10-04 and DEL-10-05 `OUT-001` and `VER-001`.

#### ER-04 — Protected-content and boundary review records predate later changes (T5B-C02, 12 rows)

- **Holder.** WORKING_ITEMS (workflow: review), using the owner-owned scan and
  sole-signatory arrangement where the release-scan path applies
  (`_REGISTER.md` D-20, RULED 2026-07-04).
- **Rows.** All 12 T5B-C02 keys: DEL-04-01 (2), DEL-04-02, DEL-04-03,
  DEL-04-05, DEL-05-01 (2), DEL-11-01 (2), DEL-11-02 (2), DEL-11-03. Also
  carries T9 item `STALE_VV:SRE-2` (the 7 INVARIANT rows, layers
  `IP_DATA;RECORD`).
- **Question.** Run a dated protected-content review over each named crate,
  fixture set, guide and SOW at current bytes, binding the reviewed hashes.
- **Evidence.** Last dedicated check located: the 2026-06-05 lifecycle QA row,
  PB-001 of 2026-05-16 and the R18 agent review of 2026-07-25; later kernel
  edits, the 2026-09-05 P4 repair and the 2026-09-18 guide rename overtook
  them (T5B, ledger). Workers' reads found no protected content on the five
  PKG-11 rows; a worker read is not a review, and nothing here says the
  boundary holds. FIRM resolution on
  `DEL-05-01:SOW#production-and-verification-method-praxeology/VER-001` sets
  AuthorityNeeded REVIEW and asks for a shared FindingGroup with
  `CLM-016/REQ-05-01-003` (accepted via `OtherCorrections`).
- **BlockedOnPacket.** C2 (what counts as a protected-content review record,
  and the absence of such reviews). The review can be scoped now; whether its
  output satisfies these rows depends on C2's answer.

#### ER-05 — Human review dispositions pending (T6-C05, 35 rows)

- **Holder.** WORKING_ITEMS (workflow: review): one batched review packet per
  deliverable.
- **Rows.** All 35 T6-C05 keys; 23 deliverables (listed in `T6_CLASSES.md`
  T6-C05). 14 are `Review_Findings.csv` rows with HumanDisposition TBD, 21 are
  partial slices whose closure the ledger conditions on a review act.
- **Question.** Record HumanDisposition for each open finding and each named
  sufficiency act; read findings aimed at the retired four-document kit (for
  example DEL-12-01 RF-002, DEL-15-03 RF-001/002, DEL-17-03 RF-002) against the
  current `ScopeOfWork.md`.
- **Evidence.** `DEL-12-01:STATUS#remaining/R02`: at the freeze, RF-001 and
  RF-002 in that deliverable's `Review_Findings.csv` (lines 2–3) are TBD and
  OPEN (T6, as cited). PKG02-001 dimensional-basis findings on DEL-04-04/05 are
  TECHNICALLY_ADDRESSED_PENDING_HUMAN (ledger).
- **What the review act authorises.** Dispositions only; recording them is R5
  record work afterwards. Where a review asks for added fixture provenance
  records or tests (DEL-17-07), those are H2 candidates, not part of the act.
- **BlockedOnPacket (partial).**
  - C5 (PCF/glTF fixture provenance): 7 DEL-17-07 rows
    (`DEL-17-07:SOW#CLM-007`, `DEL-17-07:SOW#CLM-017/DEL-17-07-REQ-040`,
    `DEL-17-07:SOW#CLM-025`, `DEL-17-07:SOW#CLM-031`, `DEL-17-07:SOW#CLM-034`,
    `DEL-17-07:SOW#completion-and-reliance-basis-epistemology/AC-001`,
    `DEL-17-07:SOW#production-and-verification-method-praxeology/VER-001`) and
    `DEL-17-08:SOW#production-and-verification-method-praxeology/VER-001`.
    `DEL-17-07:STATUS#remaining/R01` is not C5's: its finding RF-001 concerns
    DAG-006 wording, not fixture provenance, and C5 does not list it; it waits
    on the ER-05 review alone.
  - C6 (CP-09 on VER-001):
    `DEL-04-04:SOW#production-and-verification-method-praxeology/VER-001`,
    `DEL-04-05:SOW#production-and-verification-method-praxeology/VER-001`
    (both OBSERVED) and
    `DEL-04-06:SOW#production-and-verification-method-praxeology/VER-001`
    (CONTESTED). If
    the CP-09 reading is adopted these three leave the class for the T5B
    partition (task proposal).

#### ER-06 — Protected-content / IP-data review record not located (T7-C09, 31 rows)

- **Holder.** WORKING_ITEMS (workflow: review), once per fixture surface (about
  11 surfaces), not per row.
- **Rows.** All 31 T7-C09 keys; 14 deliverables (DEL-02-05, 03-04, 03-05,
  04-05, 06-01, 06-02, 06-03, 06-05, 07-02, 09-02, 14-01, 14-02, 14-03, 17-07).
- **Question.** Record a protected-content and private-data disposition for
  each surface.
- **Evidence.** No DEC-058 scan record exists under `validation/evidence/releases/`
  at the freeze (freeze-checked: the directory is absent). FIRM/FIELD
  corrections set AuthorityNeeded REVIEW on DEL-07-02 RQ-008, CLM-018.r08,
  CLM-025.r06, CLM-026 and DEL-06-05 CLM-020.r05 (accepted via
  `OtherCorrections`). Six DEL-06-01 rows are OBSERVED: agent self-reviews
  exist but were overtaken by the 2026-06-14 demo changes. This is an absence
  of a record, not evidence of a breach.
- **BlockedOnPacket.** C2 (what counts as a record) for the item; C5 (partial)
  for `DEL-17-07:SOW#CLM-017/DEL-17-07-REQ-041` and `DEL-17-07:SOW#CLM-045`
  (AUTHORITY_UNCLEAR: a fixture's values coincide with one entry of a
  published dimensional table; this register does not restate them).

#### ER-07 — Other evidence not located: smallest checks (T7-C10, 14 rows)

- **Holder.** WORKING_ITEMS (workflow: review), read-only investigation. The
  native re-observation needs a separately authorised runtime observation.
- **Rows.** All 14 T7-C10 keys; 12 deliverables. Also carries the T9 census
  rows for DEL-02-02 (R01), DEL-07-01 (R05) and DEL-07-06 (R04).
- **Checks.** One no-bypass trace for DEL-02-01 REQ-10, DEL-02-02 U-002 and
  CLM-035, DEL-02-03 R10; native re-observation of short-panel scrolling and
  the inspector AX omission (DEL-07-01 R05, DEL-07-02 R04, DEL-07-06 R04,
  DEL-07-09 R04); undefined referents (DEL-00-04 "provider expansion",
  DEL-02-02 R01 PDU-037 matrix); missing provenance notes (DEL-03-05 CLM-014,
  CONTESTED: the check was already run); a no-transmission test (DEL-08-05
  CLM-013.r08, INVARIANT); a dependency-validator run (DEL-15-02 CLM-021.r07);
  run-record hosting (DEL-00-02 `.s02`).
- **What the review act authorises.** Each result confirms the row (then R5 or
  NO_ACTION) or finds a defect (then a new H2 candidate).
- **BlockedOnPacket.** None.

### 4.2 Engineering authority items

No item below is a code fix or a record repair until the engineering basis is
recorded. This register asserts nothing about engineering adequacy; it records
only that a basis is absent from the records.

| Item | Key(s) | Question | Evidence | BlockedOnPacket |
|---|---|---|---|---|
| ER-08 | `DEL-03-05:SOW#CLM-010.r10` | COG coordinate convention and reference frame | Ledger: no convention or frame defined; the D-41 R5 T2B backcheck recorded it held; the applier keeps COG edits unsupported | - |
| ER-09 | `DEL-04-04:STATUS#remaining/R03` | Vetted basis for retained-spring mixed-friction adequacy | Ledger: no adequacy evidence found (INVARIANT, VALIDATION) | - |
| ER-10 | `DEL-13-04:STATUS#remaining/R03` | Independent validation of field-level trace completeness | Ledger (T6-C06). Moot if B9 retires PKG-13 (dependency) | - |
| ER-11 | `DEL-17-06:STATUS#remaining/R01` | Disposition of the two withheld diagnostic-work unit witnesses in the accepted 830-row fixture | Ledger: the test asserts 830 rows, 828 witnesses and two withheld-witness diagnostics | - |
| ER-12 | `DEL-05-03:CONTEXT#description`, `DEL-05-03:SOW#CLM-011/DEL-05-03-RQ-001.s01` | Pressure reference model (thin-wall membrane against exact annulus) and its companion decisions | T7-C07; the 2026-09-08 investigation lists seven decisions; `_REGISTER.md` D-67 adopted only a private dormant pressure kernel (accepted register). BaselineClass OWNER_HOLD | C3 (the owner lifts the D01–D06 hold first) |
| ER-13 | `DEL-02-02:STATUS#remaining/R05` | Independent conversion and normalisation witness under DEC-018 | Ledger: no independent witness found | - |
| ER-14 | `DEL-13-04:SOW#CLM-017/DEL-13-04-REQ-007`, `DEL-14-04:SOW#CLM-008.r02`, `DEL-14-04:SOW#CLM-017.s02`, `DEL-14-05:STATUS#remaining/R01` | Validation basis for tolerance suitability, comparison mechanics and the transform 3D frame target; the section-property oracle does not qualify | T7-C07; T9-C11b; T9 census DEL-14-05 | C3 (context only: C3 records these engineering holds as needing no owner act; the owner may only reprioritise) |

**Split portions.** T6-C06 (4 rows) = ER-08 + ER-09 + ER-10 + ER-11; T7-C07
(7 rows) = ER-12 (2) + ER-13 (1) + ER-14 (4). Keys as in the table.

**On-ruling mechanism (all).** Engineering records the convention,
qualification basis, witness or vetted source. Only then do H2 candidates (for
example consuming COG in mechanics) or H4 record repairs follow. For ER-12,
activation and validation go through the physics-audit activation plan after
the owner lifts the hold. Nothing is promoted from verification to validation
by this register (A5).

### 4.3 Review that must precede a repair (REVIEW authority on R5 or code-fix routes)

These eight classes keep their repair route (H4 for R5, H2 for code fixes).
Each item states the review that must come before the repair is relied on.
H4 marks their rows as waiting on the ER item named here; H2 does the same
for its briefs.

#### ER-15 — CONTEXT SURFACE rows restating the keyed pins (T4A-C04, 68 rows; REVIEW_BEFORE_R5)

- **Rows.** All 68 T4A-C04 keys (whole-file `DEL-xx-yy:CONTEXT` rows); 68
  deliverables in 16 packages (no PKG-09 or PKG-16).
- **Review before repair.** One consistency reading of the 102 whole-file
  CONTEXT rows: 68 non-aligned here, 1 in T4A-C08 (DEL-01-01), 30 ALIGNED and
  3 STALE_SETUP_SPECIFICATION. The C1 common-defect rule arguably supports
  ALIGNED, with the pins left to the keyed CS-01/CS-04 rows (T4A proposal).
  The reviewer confirms the rows are one situation with those keyed rows, so
  the T4A-C01 tranche clears them with no separate edit.
- **Evidence.** CONTESTED package splits on DEL-05-01/02/03 and
  DEL-10-01/02/03 (`OtherCorrections`); OBSERVED cosmetic FindingGroup gaps on
  DEL-00-03/04. `DEL-00-03:CONTEXT` points readers to revision 0.9 (T4A, as
  cited at `…/DEL-00-03_…/_CONTEXT.md:6`).
- **BlockedOnPacket (partial).** Any refresh of the file-level basis statement
  that names PKG-00 at SEMANTIC_READY waits on C7 (cause) and A5 (lifecycle
  target). Keys whose ledger ClaimSummary or Notes name SEMANTIC_READY (filter
  over the sealed forward ledgers; a lower bound, since the file text itself
  was not re-scanned): `DEL-03-04:CONTEXT`, `DEL-03-05:CONTEXT`,
  `DEL-03-06:CONTEXT`, `DEL-06-01:CONTEXT`, `DEL-06-02:CONTEXT`,
  `DEL-06-03:CONTEXT`, `DEL-13-02:CONTEXT`.
- **Risk if left.** Count asymmetry only: the pin defect is counted twice on 68
  deliverables and once on 30.

#### ER-16 — Setup-session framing overtaken (T5A-C01, 205 rows; REVIEW_BEFORE_R5)

- **Rows.** All 205 T5A-C01 keys; 45 deliverables in 11 packages.
- **Review before repair.** Confirm per deliverable that the retire-or-mark-
  historical text is accurate. The revision marks setup-session statements
  historical or removes them, leaves the governed subject untouched and never
  sets or advances a lifecycle state (F3, Direction 8).
- **Evidence.** Ledger Notes, for example `DEL-07-01:SOW#CLM-019.r02` (write
  scope limited "for this setup session" while the implementation lives under
  `apps/desktop/src`, `schemas/` and `core/gui/viewport_editor`). FIELD
  VerificationClass corrections on 11 DEL-10-04/05 rows, OBSERVED on 2, WEAK on
  `DEL-09-01:SOW#CLM-003.r09` (`OtherCorrections`; evidence fields only).
- **BlockedOnPacket (partial).**
  - A5 (SEMANTIC_READY target; rows that name it as a completion target, from
    the ledger filter): `DEL-07-01:SOW#CLM-022.r05`,
    `DEL-07-03:SOW#CLM-014/DEL-07-03-V-008`, `DEL-08-01:SOW#CLM-015`,
    `DEL-08-05:SOW#CLM-015`, `DEL-10-01:SOW#CLM-025`, `DEL-10-03:SOW#CLM-026`,
    `DEL-11-05:SOW#CLM-003.r08`, `DEL-11-05:SOW#CLM-016`,
    `DEL-11-05:SOW#CLM-022`, `DEL-12-05:SOW#CLM-021`. Dropping the setup target
    needs no ruling; restating a current target does.
  - C7 (SR-1 cause): `DEL-04-01:CONTEXT`, `DEL-04-02:CONTEXT`,
    `DEL-04-03:CONTEXT` (each cites a "PKG-00 at SEMANTIC_READY" statement among
    its overtaken parts).

#### ER-17 — Setup-era TBDs and future-tense text answered by implementation (T5A-C02, 207 rows; REVIEW_BEFORE_R5)

- **Rows.** All 207 T5A-C02 keys; 52 deliverables in 11 packages.
- **Review before repair.** Confirm that each restated TBD names the implemented
  construct and its code anchor, that engineering values the notes say are
  still unset stay TBD, that "remains TBD" checks are restated so they can be
  evaluated, and that placements settled without a ruling are recorded as
  settled in practice (for example `DEL-10-04:SOW#CLM-031.s03`). The repair
  records what was built; it ratifies no engineering value.
- **Both views (T8 against class).** `DEL-17-08:SOW#CLM-007/X-002`: class
  route R5_RECORD_REPAIR (T5A-C02); T8 route OWNER_DECISION in the EXPORT_PLAN
  cluster. Not chosen here.
- **BlockedOnPacket (partial).** A7 (deleted export plan) for
  `DEL-17-08:SOW#CLM-007/X-002`.

#### ER-18 — Post-migration declarations overtaken (T5A-C03, 94 rows; REVIEW_BEFORE_R5)

- **Rows.** All 94 T5A-C03 keys; 25 deliverables in 12 packages.
- **Review before repair.** Confirm that each restated boundary, evidence state
  or prerequisite status matches the frozen implementation (for example R15
  redaction-route binding, runner verbs executing suite payloads, the DEC-053
  sparse default), that open items stay open, and that elements a FindingGroup
  routes to an owner part keep their own route (for example DEL-12-01 AC-001
  defers to FG-DEL-12-01-01/02/03).
- **Evidence.** FIRM on `DEL-03-06:SOW#completion-and-reliance-basis-epistemology/AC-001`
  (disposition changed to STALE_REVIEW_OR_EVIDENCE; SCOPE_REDIRECTED_BY_RULING
  "also defensible"); OBSERVED on three DEL-10-05 rows.
- **BlockedOnPacket.** None beyond the FindingGroup owner parts, which carry
  their own routes and packets.

#### ER-19 — Model-operation records omit the ruled runtime seam (T5A-C04, 19 rows; REVIEW_BEFORE_R5)

- **Rows.** All 19 T5A-C04 keys: DEL-16-02 (10), DEL-16-03 (9); FindingGroups
  FG-DEL-16-02-01 and FG-DEL-16-03-01.
- **Review before repair.** Confirm that the SOW revisions name the product
  seam (the Rust operation applier `core/model_operations/operation_applier`,
  outcome envelope `operation_outcome.schema.json`; DEC-020, SCA-003 per the
  ledgers) and restate the Python engines as test-only modules, without
  choosing an engine the owner has yet to choose.
- **Evidence.** Both Python engines have test-only callers (ledger); none of
  the 19 rows carries `ProductCallerNone = YES` (T5A observation 3), so T12
  does not see them.
- **BlockedOnPacket.** A1 (DEC-009: Python against Rust status of the engines;
  applies to the whole revision). C1 (partial): `DEL-16-02:SOW#CLM-026`, which
  records the runtime/schema-validation divergence and waits on C1's ruling on
  DEL-16-02 runtime schema validation. B3 (runtime applier and edit contract)
  is a dependency.

#### ER-20 — Stale Remaining items (T5A-C07, 6 rows; REVIEW_BEFORE_R5)

- **Rows.** `DEL-07-01:STATUS#remaining/R06`, `DEL-07-02:STATUS#remaining/R08`,
  `DEL-07-05:STATUS#remaining/R02`, `DEL-08-06:STATUS#remaining/R01`,
  `DEL-09-04:STATUS#remaining/R01`, `DEL-12-01:STATUS#remaining/R01`. Also
  carries T9-C02 (DEL-12-01 R01) and the T9 census row for DEL-12-01.
- **Review before repair.** Confirm each restated Remaining item; for DEL-12-01
  R01, an owner or reviewer chooses which store tests satisfy LFSP-REQ-011.
  DEL-09-04 R01 keeps the owner-gated MAINTAINER_REVIEWED promotion open (not
  decided here). Existing Remaining text is never authority (Directions 2–3).
- **Both views.** `DEL-12-01:STATUS#remaining/R01`: class route
  R5_RECORD_REPAIR with authority REVIEW (T5A); T9 route REVIEW (T9-C02). Both
  require the reviewer's choice first; they differ on whether the review is the
  route or a precondition. Not chosen here.
- **Evidence.** `apps/desktop/src/features/results/ResultsPanel.tsx:114`
  renders `GoverningRatioState ratioCount={0}` (freeze-checked); the ledger
  records this drift as deliberate since PR #787.
- **BlockedOnPacket.** None. B12 (results-viewer scope) is a dependency for
  DEL-07-05 R02.

#### ER-21 — Protected-subject implementation remainder (T6-C03, 34 rows; REVIEW_BEFORE_CODE_FIX)

- **Rows.** All 34 T6-C03 keys; 18 deliverables in 10 packages.
- **Review before reliance.** Each H2 brief on these rows is flagged
  protected-subject and needs an independent review under the review workflow,
  with the focused negative tests named per row: IP/data (for example
  protected-content detection only on synthetic markers, DEL-08-05), security
  and privacy, claims boundary, unit validation (presence-only unit handling,
  DEL-15-02, DEL-03-08) and no-bypass (DEL-06-02).
- **Evidence.** `core/reporting/protected_content_linter/src/lib.rs:294` loops
  over `synthetic_markers()` (freeze-checked). FIELD tier correction to
  INVARIANT on `DEL-12-04:SOW#CLM-011.r05`; FIELD DEC-058 scan evidence on
  `DEL-08-04:SOW#CLM-013/V-7`; CONTESTED on `DEL-03-04:SOW#CLM-011/DEL-03-04-RQ-005`
  and `DEL-08-05:SOW#completion-and-reliance-basis-epistemology/AC-001`
  (`OtherCorrections`).
- **BlockedOnPacket (partial).**
  - B7: `DEL-15-02:SOW#CLM-005.r01`, `DEL-15-02:SOW#CLM-011.r05`,
    `DEL-15-02:SOW#CLM-027.r02` (the three T6-C03 rows of DEL-15-02; the
    DEL-15-02 defaults wait on B7 per H2).
  - B10 (plugin/adapter runtime; CP-11 no-bypass holds only by absence):
    `DEL-06-02:SOW#CLM-006.r05`, `DEL-06-02:SOW#CLM-013/REQ-06-02-010`,
    `DEL-06-02:SOW#CLM-016/REQ-06-02-010`.
  - C4 (DEL-11-04 professional-boundary list against DEC-081/DEC-107):
    `DEL-11-04:SOW#CLM-011/R-DEL-11-04-002` (CONTESTED;FIELD).
- **Owner element not on any topic list.** Where RemainingWork offers "or
  narrow the principle" (`DEL-03-08:SOW#CLM-026`,
  `DEL-06-01:SOW#CLM-011/REQ-06-01-011`), narrowing an INVARIANT restatement is
  an owner decision (T6 proposal). Reported as UNASSIGNED (§6).

#### ER-22 — Code/test fix candidates (T7-C06, 42 rows; REVIEW_BEFORE_CODE_FIX)

- **Rows.** All 42 T7-C06 keys; 12 deliverables in 8 packages. Also carries the
  T12 REVIEW row `DEL-13-01:SOW#CLM-005.r05` (same key).
- **Review before reliance.** Each of the about ten briefs goes through the
  chirality-change PR path with independent review; affected rows are
  re-verified in a later concordance, not edited in place. Two brief-time
  choices: DEL-17-02/03 (align the code to `tbd`, or record the variance in the
  DEL-17-02 contract, which is a record repair instead); DEL-13-02 (after the
  unit-vocabulary reading).
- **Views on `DEL-13-01:SOW#CLM-005.r05`.** Class T7-C06 route
  CODE_FIX_CANDIDATE with authority REVIEW (FIRM); T8 unit-vocabulary route
  CODE_FIX_CANDIDATE; T12 (T12-C03) route REVIEW. The T8 and class routes agree
  on the 10 DEL-13-01/13-02 unit rows. Not chosen here.
- **Evidence.** `core/handoff/target_mapping/contract.py:239` defaults
  `mapping_status` to "mapped" (freeze-checked). Other citations as in
  `T7_CLASSES.md` T7-C06 (as cited).
- **BlockedOnPacket (partial).**
  - C7 (unit-vocabulary tier reading): the 8 CONTESTED DEL-13-02 rows
    (`DEL-13-02:SOW#CLM-003.r13`, `DEL-13-02:SOW#CLM-009`,
    `DEL-13-02:SOW#CLM-010/R-13-02-005`, `DEL-13-02:SOW#CLM-012/R-13-02-005`,
    `DEL-13-02:SOW#CLM-013`, `DEL-13-02:SOW#CLM-018`,
    `DEL-13-02:SOW#CLM-019.r04`, `DEL-13-02:SOW#CLM-024`) and
    `DEL-13-01:SOW#CLM-005.r05`, `DEL-13-01:SOW#CLM-009/REQ-13-01-006`.
  - B9 (PKG-13 product status): the same 10 DEL-13-01/13-02 rows.
  - B7: the 13 T7-C06 rows of DEL-15-02: `DEL-15-02:CONTEXT#context-envelope`,
    `DEL-15-02:MEMORY`, `DEL-15-02:SOW#CLM-005.r02`, `DEL-15-02:SOW#CLM-005.r03`,
    `DEL-15-02:SOW#CLM-011.r06`, `DEL-15-02:SOW#CLM-011.r07`,
    `DEL-15-02:SOW#CLM-020`, `DEL-15-02:SOW#CLM-020.s01`,
    `DEL-15-02:SOW#CLM-021.r03`, `DEL-15-02:SOW#CLM-026`,
    `DEL-15-02:SOW#CLM-027.r01`, `DEL-15-02:SOW#CLM-027.r03`,
    `DEL-15-02:SOW#completion-and-reliance-basis-epistemology/AC-001`.
  - C2 (PR #787's removed protected check): `DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-08`.
  - The W3 product items on DEL-17-02/03/04/07 are H2's; they carry no packet.

### 4.4 Items from the T8, T9, T11 and T12 routes with no in-scope class row

#### ER-23 — F1 edge row `DEL-05-02:SOW#CLM-021` (T8-K4; REVIEW)

- **Holder.** WORKING_ITEMS (workflow: review).
- **Source.** `T8_ROWS.csv` (cluster F1_ON_CONTEXT, route REVIEW). The row is
  not divergent (`T8_ROUTE_DISAGREEMENTS.csv`: class route NOT_DIVERGENT).
- **Question.** Is the deferred rule-pack supply gap an element of this CONTEXT
  guidance row's own subject? If yes, F1 applies and the row joins its
  FindingGroup's routing; if no, it stays ALIGNED.
- **BlockedOnPacket.** C7 (owner confirmation that F1 reaches CONTEXT-typed
  rows).

#### ER-24 — `DEL-13-02:STATUS#remaining/R01` (T9-C04 and census DEL-13-02; REVIEW)

- **Holder.** WORKING_ITEMS (workflow: review).
- **Both views.** Class T4B-C07 routes R5_RECORD_REPAIR with authority NONE;
  T9 routes REVIEW; the effective AuthorityNeeded in `CORPUS_CLAIMS.csv` is
  REVIEW. Not chosen here.
- **Question.** Record the human disposition of finding
  PKG13-DEL-13-02-PKG02-001 read against the constraint enum at the freeze (the
  finding's premise, that the enum equals the accepted set, no longer holds),
  then restate the item.
- **BlockedOnPacket.** C7 (unit vocabulary) and B9 (PKG-13 status). The same
  disposition is part of the DEL-13-02 RemainingWork in ER-22.

#### ER-25 — DEL-01-03 census: UNKNOWN R02 (T9 census; REVIEW)

- **Both views.** T9 routes the census row REVIEW (smallest next check, C6(h));
  `DEL-01-03:STATUS#remaining/R02` is class T7-C04 (OWNER_DECISION, topic A8).
- **Question.** Run the smallest check on R02 (AUTHORITY_UNCLEAR) once A8
  settles whether authority is silent or in conflict.
- **BlockedOnPacket.** A8.

#### ER-26 — DAG-010 edge evidence files absent at the freeze (T11 D-03; REVIEW)

- **Holder.** WORKING_ITEMS (workflow: review) for the reading; any DAG change
  is outside this run (Direction 1; D-73).
- **Finding (task proposal, scripted).** 651 of 1,402 active edges cite an
  EvidenceFile that does not resolve at the freeze: 421 cite the retired
  four-document kit, 171 a bare `_CONTEXT.md`, 59 other removed paths. For 13
  deliverables every active edge is affected (DEL-03-07, 07-02, 16-01, 16-03,
  17-01..09). Sequencing is unaffected.
- **Question.** Re-point the provenance column at the next DAG refresh?
- **BlockedOnPacket.** None. Candidate input to the post-run DAG rebuild.

#### ER-27 — DEL-07-09 has no architecture-basis edge to PKG-00 (T11 D-04; REVIEW)

- **Question.** Intended omission (then `DEL-07-09 _DEPENDENCIES.md:14`, which
  says such rows "are preserved here", is wrong) or a missing edge? DAG-010
  records SCA-009 as exactly three execution edges (as cited).
- **BlockedOnPacket.** None. Dependencies: B11 (DEL-07-09 promotion), C7
  (SCA-009 re-point confirmation).

#### ER-28 — Ownership redirects with no DAG-010 edge (T11 D-05; REVIEW)

- **Pairs.** DEL-07-03>DEL-07-09; DEL-16-01>DEL-07-01; DEL-07-01>DEL-07-04;
  DEL-06-04>DEL-06-03; DEL-12-04>DEL-02-04; DEL-05-02>DEL-09-04.
- **Question.** Is each redirect also a sequencing dependency for the next DAG
  refresh? An ownership redirect is not necessarily an edge.
- **BlockedOnPacket (partial).** C7 for DEL-07-03>DEL-07-09 (the SCA-009
  re-point: 9 ledger rows, of which T11 names `DEL-07-03:SOW#CLM-005.r04` as
  CONTESTED; C7 carries the T6-C09 confirmation of its 6 CONTESTED rows). B3 for
  DEL-16-01>DEL-07-01 (runtime edit contract, `DEL-16-01:SOW#CLM-009.r01/.r03`).

#### ER-29 — MBF pass-through citation implies an absent implementation (T11 R-04; REVIEW)

- **Question.** Should `DEL-17-01:SOW#CLM-012/DEL-17-01-REQ-013` keep citing
  `core/handoff/caepipe_mbf/package.py` as ImplementationEvidence when
  `DEL-17-04:SOW#CLM-005.r10` records the option as not started (CP-11)? The
  readings are compatible; the citation misleads.
- **BlockedOnPacket.** None. Outcome is an R5 evidence-field repair.

#### ER-30 — SR-1 sub-claim cause split across 81 deliverables (T11 S-01; REVIEW)

- **Question.** Apply one cause to the 81 SR sub-claims (42 RECORD_DRIFT
  against 39 SCOPE_REDIRECTED_BY_RULING; six packages split internally). The
  cause reading is T8's (SR-1, RECORD_DRIFT proposed).
- **BlockedOnPacket.** C7 (SR-1 cause reading) and A5 (the SEMANTIC_READY
  status the sentence restates).

#### ER-31 — Diagnostic field preservation across boundaries (T11 SS-01; ENGINEERING_AUTHORITY)

- **Holder.** ENGINEERING, or OWNER per the DEL-00-06 RemainingWork (T11). This
  is the engineering part of the diagnostics boundary split; the method part
  is in C6 and the product-intent part in C1.
- **Source rows (7 T11 rows).** SS-01; R-03 (`apps/desktop/src/types.ts`
  Diagnostic type); per-capability TENSION rows CAP-COREB-029, CAP-DATA-013,
  CAP-DATA-016, CAP-DATA-023, CAP-WSUI-032.
- **Deliverables (16).** Filter: the union of the `DEL-xx-yy` identifiers in the
  `Deliverables` column of those 7 rows of `R3/TASKS/T11_METHOD.csv`
  (Subject starting `SS-01` or `R-03`, or equal to one of the five CAP IDs):
  DEL-00-03, DEL-00-05, DEL-00-06, DEL-00-07, DEL-04-01, DEL-05-04, DEL-06-02,
  DEL-07-04, DEL-07-06, DEL-07-07, DEL-07-08, DEL-08-03, DEL-08-04, DEL-10-05,
  DEL-14-02, DEL-14-04.
- **Question.** Must the runner solve-to-export binding and the desktop export
  writer carry every diagnostic field except class (then an H2 candidate), or
  are the mappings accepted and AB-00-06 REQ-06-02 refined (then R5)?
- **Evidence.** `core/runner/headless/src/result_envelope_binding.rs:159-182`
  (freeze-checked: the mapping sets the affected object from the diagnostic's
  own id and supplies a fixed remediation string). ALIGNED on
  DEL-10-05 CLM-003.r04, DEL-07-07 CLM-004.r05 and REQ-07-07-005, DEL-07-08
  CLM-004.r06; POSSIBLE_DEFECT on DEL-00-06 REQ-06-02, DEL-08-04 CLM-011.r05
  and CLM-025.r04, DEL-08-03 CLM-026.r03; PARTIAL_SLICE on DEL-04-01 REQ-011,
  DEL-06-02 CLM-006.r04, DEL-08-03 REQ-003 (T11, from effective values).
  CAP-DATA-016's OwnerKeys omit the defect row CLM-011.r05 (T11 observation 5).
  The DEL-08-03, DEL-08-04 and DEL-00-06 rows are T7-C05 (topic C1); they are
  named here as context and not claimed.
- **BlockedOnPacket.** C1 (whether the producer's diagnostic class and
  remediation must be carried is an owner intent ruling).

#### ER-32, ER-33, ER-34 — Engines replaced by a product port (T12-C06, 93 of 94 rows; ENGINEERING_AUTHORITY)

For each duplicate, engineering decides: (a) keep it as a named parity or
validation oracle, with a parity check against the product implementation
(then an H2 test brief); or (b) retire it (then H1 scope change). R5 records
the chosen status in each SOW. The 94th T12-C06 row,
`DEL-00-03:AB#realized-artifacts.r02`, is routed R5_RECORD_REPAIR and is H4's.

| Item | Engine and portion (filter over `T12_UNREACHED.csv`: `ClusterID = T12-C06`, `Route = ENGINEERING_AUTHORITY`, `Engine =` …) | Rows | Product twin | BlockedOnPacket |
|---|---|---:|---|---|
| ER-32 | `core/section_properties/calculator.py` (DEL-03-08) | 52 | `derive_pipe_section`, `core/product_physics/src/lib.rs:6321` (freeze-checked; calculator entry at `calculator.py:63`, freeze-checked) | A1 (whether a Python engine under `core/` may stand against the Rust core, including as an oracle) |
| ER-33 | `core/loads/user_loads` (DEL-05-05) | 38 | primitive_loads and straight_pipe; the crate's only non-test caller is `validation/benchmarks/mechanics/Cargo.toml:19` (freeze-checked) | C6 (partial: the corpus-level F7 reading) for the 4 CONTESTED rows `SOW#CLM-010.r06`, `SOW#CLM-010.r08`, `SOW#CLM-010.r10`, `SOW#completion-and-reliance-basis-epistemology/AC-001` |
| ER-34 | `core/library_import/provenance_checker.py` (`DEL-03-07:SOW#CLM-005.s01`, `DEL-03-07:SOW#CLM-012.s01`, `DEL-03-07:SOW#CLM-023`) | 3 | The Rust library-import port the product calls (T12, as cited) | A1 |

**Both views on the 4 DEL-05-05 CONTESTED rows.** Effective ALIGNED (F7 engine
reading) against PARTIALLY_IMPLEMENTED · PROJECT_BASELINE (as DEL-05-02
REQ-05-02-008 and DEL-05-03 RQ-005 read the same shape) or VERIFIED_NOT_VALIDATED
· VALIDATION_GAP (agent-produced TP-PHYS witnesses). Not chosen here.

### 4.5 T1 capabilities routed REVIEW

#### ER-35 — Four routing-gap capabilities awaiting a reverse pass (T1; REVIEW)

- **Holder.** WORKING_ITEMS (workflow: review): a reverse pass by each proposed
  owner deliverable.
- **Rows.** 4 capabilities (not claim rows): CAP-COREC-053, CAP-FEATB-029,
  CAP-FEATB-030, CAP-PHYS-021. H1 carries them as ASSIGN items H1-110, H1-116,
  H1-117 and H1-017, whose `BlockedOnPacket` names ER-35. T1–T3 carry no route
  column; the REVIEW route is in the T1 Notes text.
- **Question.** Does the proposed owner accept each capability as within its
  scope, before the H1 assignment and an R5 record repair?

| Capability | T1 classification, confidence | Proposed owner | Evidence (T1, as cited) |
|---|---|---|---|
| CAP-COREC-053 (plugin manifest verification) | ROUTING_GAP, MEDIUM | DEL-02-04 | `core/adapters/framework/plugin_verification.py` and its test; DEL-02-04 REQ-14 (`ScopeOfWork.md:183`); COREC area routing omitted PKG-02 |
| CAP-FEATB-029 (project validation preflight) | ROUTING_GAP, MEDIUM | DEL-02-05 | `apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx:275`; `DEL-02-05:CONTEXT#description`; FEATB area routing omitted PKG-02 |
| CAP-FEATB-030 (project storage audit) | ROUTING_GAP, LOW | DEL-02-05 (alternative DEL-12-01 for the local-only boundary element) | `apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx:205`; DEL-12-01 `SOW#CLM-003.r01` COVERS relation |
| CAP-PHYS-021 (expansion-joint macro-element) | ROUTING_GAP, MEDIUM | DEL-03-06 | `core/product_physics/src/lib.rs` `build_expansion_joint_user_stiffness_elements`; DEL-03-06 `MEMORY.md:120-158` (tranches landed under DEC-045); scope fit needs a check because DEL-03-06 CONTEXT names EJ fields only |

- **What the review act authorises.** An accept or decline per capability; an
  accept feeds H1's ASSIGN and a later R5 record repair; a decline returns the
  capability to H1 for another owner.
- **BlockedOnPacket (partial).** B1 for CAP-PHYS-021 (product solve path in
  `core/product_physics`), as H1-017 records. The other three carry no packet.

## 5. Disagreements between tasks left open

1. `DEL-10-01:SOW#CLM-004`, `DEL-10-03:SOW#CLM-004`: class REVIEW (T5A-C08)
   against T8 R5_RECORD_REPAIR (ER-02).
2. `DEL-17-08:SOW#CLM-007/X-002`: class R5 (T5A-C02) against T8 OWNER_DECISION,
   EXPORT_PLAN (ER-17).
3. `DEL-12-01:STATUS#remaining/R01`: class R5 with REVIEW authority (T5A-C07)
   against T9 REVIEW (ER-20).
4. `DEL-13-02:STATUS#remaining/R01`: class R5, authority NONE (T4B-C07) against
   T9 REVIEW and effective AuthorityNeeded REVIEW (ER-24).
5. `DEL-01-03:STATUS#remaining/R02`: class OWNER (T7-C04) against the T9 census
   REVIEW route (ER-25).
6. `DEL-13-01:SOW#CLM-005.r05`: class and T8 CODE_FIX_CANDIDATE against T12
   REVIEW (ER-22).
7. `DEL-05-02:SOW#CLM-021`: T8 REVIEW against NOT_DIVERGENT (ER-23).
8. The four DEL-05-05 CONTESTED rows: three readings (ER-33).
9. SS-01 holder: T11 names ENGINEERING, or OWNER per the DEL-00-06
   RemainingWork; T7 places the owner-intent part in T7-C05 (ER-31).

## 6. UNASSIGNED owner decisions found

1. **CONVENTIONS F3 origin test (item identity or assessed sentence).**
   Raised by `DEL-12-02:SOW#CLM-037/REXC-OI-002` (T5A-C08; T5A says the element
   goes to OWNER as a change to a RULED convention). Not on the P1–P3 lists or
   the crosswalk. Related but distinct: A7 uses F3 origin to set the export-plan
   disposition class.
2. **Narrowing an INVARIANT restatement instead of implementing it.** Offered as
   an option by `DEL-03-08:SOW#CLM-026` and `DEL-06-01:SOW#CLM-011/REQ-06-01-011`
   (T6-C03; T6 says it goes to the owner as a decision-packet item). Not on any
   list.

Neither is drafted here.

## 7. Packet dependencies of this register

| Packet | Items waiting (whole or partial) |
|---|---|
| A1 | ER-19, ER-32, ER-34 |
| A3 | ER-02 (partial) |
| A5 | ER-02, ER-15, ER-16 (partial); ER-30 |
| A7 | ER-17 (partial) |
| A8 | ER-25 |
| B1 | ER-35 (partial) |
| B3 | ER-28 (partial); dependency of ER-19 |
| B7 | ER-03, ER-21, ER-22 (partial) |
| B9 | ER-22 (partial), ER-24 |
| B10 | ER-21 (partial) |
| C1 | ER-19 (partial), ER-31 |
| C2 | ER-04, ER-06; ER-22 (partial) |
| C3 | ER-12 (ER-14 context only) |
| C4 | ER-21 (partial) |
| C5 | ER-05, ER-06 (partial) |
| C6 | ER-03, ER-05, ER-33 (partial) |
| C7 | ER-23, ER-24, ER-30; ER-02, ER-15, ER-16, ER-22, ER-28 (partial) |

H4 should mark every R5 row of T4A-C04, T5A-C01–C04 and T5A-C07 as waiting on
ER-15 to ER-20 respectively, and the ER-03 re-review as following those
tranches. H2 should mark every brief touching T6-C03 or T7-C06 rows as needing
the ER-21 or ER-22 review before reliance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
register describes records and evidence only. It makes no certification,
code-compliance, professional-approval or engineering-acceptance claim, and
nothing in it establishes engineering adequacy or that a protected-content
boundary holds.
