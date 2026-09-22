# B12 — Other held owner selections blocking implementation

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

Thirteen independent owner selections (T6-C04 decision groups D1 and D4–D15),
plus CF-001/CF-002 and PDU-031 from the W3 owner list. In each, the records
leave a choice open and the remaining work cannot proceed until the owner
either selects (and implementation follows with tests) or keeps the hold (and
the claim is narrowed or annotated). Each item is ruled separately. Section 3
gives one sub-packet per item.

**Holder: OWNER** for every item. Item D1 covers only the **selection** of
the human acceptance workflow OI-007. Its cause reading
(DEFERRED_BY_RULING vs PARTIAL_SLICE) belongs to C7.

## 2. Background

**Common facts.**
- T6 classified these 67 rows as PARTIALLY_IMPLEMENTED or
  DOCUMENTED_UNIMPLEMENTED, with AuthorityNeeded OWNER (after FIRM and FIELD
  corrections). Almost all are on IN_PROGRESS deliverables (T6 intro).
- T6's on-ruling mechanism is the same for every group. Each ruling is
  recorded as a DEC or PDU disposition. It authorises either R5 narrowing of
  the claims or candidate code briefs, and code briefs stay unexecuted until
  owner-steered work-graph selection.

**Rulings the items rest on (GOVERNING; `SD` lines at the freeze).**

| Item | Ruling or TBD | What it says or leaves open |
|---|---|---|
| D1 | OI-007 (`SD:575`) | Keeps the human approval workflow TBD. PRD §21.3 and DEC-081 (`SD:672`) keep MVP free of a formal acceptance workflow (T8-K7). |
| D8 | DEC-074 O1 (`SD:665`) | Origin of the rotational-visualisation hold (PDU-061). |
| D9 | DEC-020 (`SD:611`) | Stood up the ADR surface (ADR-0001). Later architecture decisions are DEC rows. |
| D10 | DEC-028 (`SD:619`) | Leaves the package compatibility window to bounded-tranche work. |
| D14 | DEC-054 (register D-27) | Conditional R4 gate with the benchmark/manual evidence system as an explicit residual. |
| D15 | PDU-031 | An OWNER_HOLD recorded in DEL-17-08 R01. |
| CF-001/002 | PRD open question 11 | Asks the question; no ruling answers it (`WAVES/W3/PKG-12/PKG-12_VERIFICATION.md:282-284`). |

**What the code does now (spot checks at the freeze).**
- `schemas/project_persistence.schema.yaml:506` and `:527` declare
  `invalidates_on_hash_change` (checked). There is no runtime invalidation
  check (D1).
- `lib.rs::save_local_library` and `save_local_rule_pack` exist. The OBSERVED
  and FIELD corrections on DEL-12-04 CLM-004.r01 and CLM-011.r01 say so (D4).
- Other code facts are cited per item from T6-C04 and not re-read here.

## 3. Items (options as they stand in T6-C04 unless noted)

| Item | Question | Options | Consequences |
|---|---|---|---|
| **D1** Acceptance workflow selection (OI-007) | Rule an acceptance workflow? | (a) Rule a workflow, then add a runtime hash-change invalidation check and a negative test. (b) Leave it unruled; hash binding stays declaration-only. | (a) Scope change or lifecycle path for the workflow, then a code brief. It also changes the DEL-15-04 rows held by ruling (T6-C08). (b) R5 annotates. T8-K7 proposes NO_ACTION under (b). |
| **D4** Private-library storage roots and secret provider | Choose the storage-root placement and a secret provider? | (a) Decide both, then wire product registration and credential-placeholder flows. (b) Keep the helper-only posture. | The remaining gap is rule-pack storage and root placement, not the absence of a store (OBSERVED/FIELD). (a) is a precondition for B10 wiring. |
| **D5** Telemetry consumer routes | Route consumer telemetry attempts through the seam? | (a) Authorise it separately, with TEL-TEST-006 and a product-level no-outbound test plus a security review record. (b) Keep only the modelled panel attempt. | (a) Code brief plus a review item (H3). All 3 rows are INVARIANT. |
| **D6** Unit namespace, alias policy, diagnostic-code namespace, angle semantics (U-010, U-016) | Select each policy? | Select each (then parser and mapping tests), or leave it TBD. | May intersect the unit-vocabulary reading (T8-K3, C7). DEL-02-02 is the unit contract owner. |
| **D7** Result-envelope home and comparison-result schema | Home ValidationResult/TransformResult and name an authoritative comparison-result/export schema? | (a) Accept an application-service or result-envelope home and an authoritative schema, then add conformance tests. (b) Leave the outputs unhomed. | Couples to B8 (comparison in product?) and B9 (PKG-13 status). If B9 is (c) retire, the DEL-13 rows lapse. |
| **D8** Results-viewer scope | Rotational visualisation semantics (DEC-074 O1 / PDU-061), supported-ratio and equipment-load path into the viewer, graphical comparison overlays with a state/run browser | Define and implement each, or narrow or amend the claim. | Overlays couple to B8. Two DEL-07-05 rows are CP-11 RESOLVED_PAIR. DEL-07-07 FG-01: producers must carry class, remediation and provenance (compare T11 SS-01, H3). |
| **D9** ADR form vs decision-log form after 2026-06-11 | Do later architecture decisions need ADRs? | (a) Record later decisions as ADRs. (b) Rule that DEC rows in the decision log satisfy AB-00-01. | (a) Record work to back-fill ADRs. (b) R5 narrows the DEL-00-01/00-02 text. |
| **D10** Package compatibility window (DEC-028) | Rule the window? | Rule it, then implement package open, round-trip and explicit migrate; or keep it open. | Relates to C02 `DEL-08-01:STATUS#remaining/R01` (T6) and to CAP-SHELL-048 (H1; B2). |
| **D11** Rule-pack combination supply (SOW-014) | How are code-specific combinations supplied? | (a) Build the DEL-06-02 combination handoff. (b) Record an owner reading that user-authored combinations satisfy it. | (b) Is an owner reading and R5 annotation. SOW-014's note already says code-specific combinations are supplied by user rule packs (`SD:122`). |
| **D12** Adapter family list after PRD v0.4 dropped §19.3 | Keep the model-creation, load-case and rule-pack-evaluation families? | Register them, or explicitly de-select them. | Couples to B10 (adapter runtime). |
| **D13** Mechanics residuals | (i) Re-dispose assessment rows G1, G2, G4, M2, M3 (DEL-04-01 R01)? (ii) Typed vocabulary for nonlinear result rows (DEL-04-04 R07)? | (i) Re-dispose, or close them by evidence. (ii) Route it as a DEL-08-04 result-semantics contract change (as the evidence routes it), or keep it open. | (ii) Couples to B1 (the loop owner emits those rows). |
| **D14** Release, QA and stage gates | (1) QA policy: benchmarks release-gating or advisory (DEL-09-01 CLM-032.r04, CP-11); (2) present the benchmark evidence system at the owner's gate (DEL-09-01 R03, DEC-054); (3) public benchmark comparison values (DEL-09-04 R02); (4) release-label vocabulary PB-TBD-003 (DEL-09-05 R01); (5) Phase H R6 stage gate (DEL-15-01 R01) | Select, or keep the gate closed (each). | (4) Is the same subject as **A10**'s "release-label floor PB-TBD-003" (T4A-C06 rows DEL-09-05 CLM-012/RQG-007 and CLM-029). They must be ruled together. See INDEX coverage note. |
| **D15** Export timestamp and generator policy (PDU-031) | Select a timestamp and generator policy? | Select one, or keep the fixed generator with no timestamp, as the tests enforce today. | OWNER_HOLD. Code change only under selection. |
| **CF-001 / CF-002** Secret provider and encrypted-storage default (DEL-12-04, W3 owner list) | Rule them, or confirm they stay TBD? | Rule, or confirm TBD. One CF-002 element, the package/container, is stale because SCA-003 settled it (T7-C04). | Same subject as D4. The rows are class **T7-C04**, which topic A8 takes (see section 5). |

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:575`, `:611`, `:619`, `:665`, `:672`; register D-27 | GOVERNING | Read |
| `project_persistence.schema.yaml:506`, `:527` | EVIDENCE (freeze) | Checked by this writer |
| Sealed forward ledgers for the 27 T6-C04 deliverables; adopted resolutions (RESOLVED_PAIR, OBSERVED, FIELD) | R2 records | Effective values from `CORPUS_CLAIMS.csv` |
| `PKG-12_VERIFICATION.md:282-284`, `:408-409` | R2 verification report | Verifier confirmed UNKNOWN · AUTHORITY_UNCLEAR for CF-001/002 |
| `CORPUS_CROSS_WAVE_VERIFICATION.md:257-263` (via T8-K7) | R2 verification report | Found both D1 cause readings defensible |
| PDU identifiers (PDU-031, PDU-061) | CONTEXT (July run) | Referenced by deliverable records only |
| T6-C04; T8-K7; T7-C04 decision 4 | R3 PROPOSAL | Classification only |

PKG-12 carries the run's highest first-pass error rate (9.5%; W3 assessment).
D4, D5 and CF rows are PKG-12 rows.

## 5. Affected claims

- **Class T6-C04** (87 rows; split between **B10** and **B12**). B12's portion
  is **67 rows**: decision groups D1 and D4–D15. Filter: T6-C04 rows minus
  B10's 20 D2/D3 keys.
  - **D1** (9): `DEL-05-04:SOW#CLM-004`; `DEL-05-04:SOW#CLM-008`
    (RESOLVED_PAIR); `DEL-05-04:SOW#CLM-011/REQ-05-04-008`;
    `DEL-05-04:SOW#CLM-011/REQ-05-04-014`; `DEL-05-04:SOW#CLM-013/REQ-05-04-008`;
    `DEL-05-04:SOW#CLM-013/REQ-05-04-014`; `DEL-05-04:SOW#CLM-020`;
    `DEL-05-04:SOW#CLM-023` (RESOLVED_PAIR);
    `DEL-05-04:SOW#completion-and-reliance-basis-epistemology/AC-001`.
  - **D4** (3): `DEL-12-04:SOW#CLM-004.r01` (OBSERVED);
    `DEL-12-04:SOW#CLM-004.r02`; `DEL-12-04:SOW#CLM-011.r01` (FIELD;OBSERVED).
  - **D5** (3): `DEL-12-03:SOW#CLM-011/TEL-REQ-009`;
    `DEL-12-03:SOW#CLM-011/TEL-REQ-010`; `DEL-12-03:SOW#CLM-013/TEL-TEST-006`.
  - **D6** (6): `DEL-02-02:SOW#CLM-014/U-010`; `DEL-02-02:SOW#CLM-014/U-016`;
    `DEL-02-02:SOW#CLM-015.r02`; `DEL-02-02:SOW#CLM-015.r08`;
    `DEL-02-02:SOW#CLM-015.r12`; `DEL-02-02:SOW#CLM-017/U-010`.
  - **D7** (8): `DEL-13-03:SOW#CLM-013.r06`; `DEL-13-03:SOW#CLM-028.r06`;
    `DEL-13-04:SOW#CLM-014.s02`; `DEL-14-04:SOW#CLM-008.r01`;
    `DEL-14-04:SOW#CLM-013/R-14-04-007`; `DEL-14-04:SOW#CLM-017.s01`;
    `DEL-14-04:SOW#CLM-023`; `DEL-14-04:SOW#CLM-035/OQ-14-04-002`.
  - **D8** (19):
    - DEL-07-05: `DEL-07-05:CONTEXT#description`; `DEL-07-05:SOW#CLM-004.r02`;
      `DEL-07-05:SOW#CLM-004.r03`; `DEL-07-05:SOW#CLM-004.r07`
      (RESOLVED_PAIR); `DEL-07-05:SOW#CLM-010`;
      `DEL-07-05:SOW#CLM-011/REQ-07-05-001`;
      `DEL-07-05:SOW#CLM-011/REQ-07-05-005` (RESOLVED_PAIR);
      `DEL-07-05:STATUS#remaining/R01`.
    - DEL-07-07: `DEL-07-07:SOW#CLM-016`; `DEL-07-07:STATUS#remaining/R01`.
    - DEL-07-08: `DEL-07-08:CONTEXT#anticipated-artifacts`;
      `DEL-07-08:CONTEXT#description`; `DEL-07-08:SOW#CLM-004.r01`;
      `DEL-07-08:SOW#CLM-004.r02`; `DEL-07-08:SOW#CLM-006`;
      `DEL-07-08:SOW#CLM-010`; `DEL-07-08:SOW#CLM-011/REQ-07-08-001`;
      `DEL-07-08:SOW#CLM-011/REQ-07-08-005`;
      `DEL-07-08:SOW#CLM-013/REQ-07-08-005`.
  - **D9** (3): `DEL-00-01:AB#normative-requirements/REQ-01-04`;
    `DEL-00-01:AB#purpose.s03`;
    `DEL-00-02:AB#resolved-decisions-former-tbd-and-human-ruling-q.r06`.
  - **D10** (2): `DEL-02-05:STATUS#remaining/R01`;
    `DEL-02-05:STATUS#remaining/R02`.
  - **D11** (3): `DEL-05-02:SOW#CLM-005`; `DEL-05-02:SOW#CLM-010/REQ-05-02-004`;
    `DEL-05-02:SOW#CLM-023.r03`.
  - **D12** (3): `DEL-10-01:SOW#CLM-005`;
    `DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-02`; `DEL-10-01:SOW#CLM-015/REQ-02`.
  - **D13** (2): `DEL-04-01:STATUS#remaining/R01`;
    `DEL-04-04:STATUS#remaining/R07`.
  - **D14** (5): `DEL-09-01:SOW#CLM-032.r04`; `DEL-09-01:STATUS#remaining/R03`;
    `DEL-09-04:STATUS#remaining/R02`; `DEL-09-05:STATUS#remaining/R01`;
    `DEL-15-01:STATUS#remaining/R01`.
  - **D15** (1): `DEL-17-08:STATUS#remaining/R01`.
- **Both views (T8 against the class route).** The 9 D1 rows appear in
  `T8_ROUTE_DISAGREEMENTS.csv`:
  - T8-K7 view: NO_ACTION, reading `DEFERRED_BY_RULING` with the disposition
    kept.
  - Class view: T6-C04, OWNER_DECISION.

  B12 decides only the selection. C7 carries the cause reading for owner
  confirmation. Both views stay visible, and this packet does not choose
  between them.
- **CF-001 / CF-002, cross-referenced and not counted in B12's portion.**
  `DEL-12-04:SOW#CLM-028/DEL-12-04-CF-001` and
  `DEL-12-04:SOW#CLM-028/DEL-12-04-CF-002` (UNKNOWN · AUTHORITY_UNCLEAR ·
  LOCAL_DESIGN). They are in class **T7-C04**, which is not a listed split
  class and which topic **A8** takes ("secret provider"). The topic text
  names CF-001/CF-002 under B12. The decision content is drafted here, and
  the row count stays with A8. See the INDEX coverage note.
- **Rows known only from OtherCorrections.** The D4 OBSERVED and FIELD
  corrections (existence of `save_local_library` and `save_local_rule_pack`).
  No B12 row entered T6-C04 through a FIRM AuthorityNeeded correction. All
  three such rows are in B10.
- **Packages (12):** PKG-00, 02, 04, 05, 07, 09, 10, 12, 13, 14, 15, 17.
  **Deliverables (22):** DEL-00-01, 00-02, 02-02, 02-05, 04-01, 04-04, 05-02,
  05-04, 07-05, 07-07, 07-08, 09-01, 09-04, 09-05, 10-01, 12-03, 12-04, 13-03,
  13-04, 14-04, 15-01, 17-08.

## 6. Risks

- **Undecided.** The claims stay partially met indefinitely. 15 of the 67
  rows are at INVARIANT tier (D1 9, D5 3, D4 1, D14 2), carrying the CLAIMS,
  IP_DATA, SECURITY and VALIDATION subjects. If a hold persists silently,
  records may be read as more complete than the product is (T6-C04 risk).
- **Per item.**
  - Selecting (a) opens code work that needs tests and, for D5 and D4, a
    security review (H3).
  - Keeping a hold needs R5 narrowing, or the records overstate.
  - D14(4) and CF-001/002 risk being ruled twice, or differently, if A10 and
    A8 are not taken together with them.

## 7. Recommended routing

- **D11.** The evidence leans one way: SOW-014's own note (`SD:122`) already
  says code-specific combinations are supplied by user rule packs, which
  supports option (b) as an owner reading. This is a reading, not a ruling.
- **D1.** Keeping OI-007 unruled matches PRD §21.3 and DEC-081's "no
  acceptance workflow created" (T8-K7). The evidence supports (b) as the
  status quo. Choosing (a) is a product-scope change.
- **All other items.** No recommendation; owner's call.

## 8. On-ruling mechanism

For every item:
- **Selection.** Record a DEC (or PDU disposition) through the owner decision
  path. Candidate CODE_FIX_CANDIDATE briefs (H2) then go through the change
  path under a production brief. They stay unexecuted until owner-steered
  work-graph selection.
- **Keep the hold.** Record a DEC or decision-log note. R5 record repair then
  narrows or annotates the affected rows (H4, separate authorization).
- **Special cases.**
  - D1(a) needs a scope-change or lifecycle-path act to create the workflow
    (T8-K7).
  - D9(b) and D11(b) are owner readings recorded as DECs, followed by R5.
  - D13(ii) is a DEL-08-04 result-semantics contract change through
    scope-change.
  - CF-001/002: the ruling is recorded in the register or the PRD, and the
    rows get R5 (T7-C04).

Nothing executes until the owner acts.

## 9. Dependencies

- **Rule together with.**
  - C7 (D1 cause reading; D6 unit-vocabulary tier).
  - A10 (D14 item 4, PB-TBD-003).
  - A8 (CF-001/002, secret provider).
- **Coordinates with.**
  - B10 (D4 is a precondition for wiring; D12 adapter families).
  - B8 (D7, D8 overlays).
  - B9 (D7).
  - B1 (D13).
  - B2 (D10 and CAP-SHELL-048).
- **Blocks.** The H2 and H4 rows of each group. In particular, R5 narrowing
  must not precede a ruling for any of the 67 rows.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
