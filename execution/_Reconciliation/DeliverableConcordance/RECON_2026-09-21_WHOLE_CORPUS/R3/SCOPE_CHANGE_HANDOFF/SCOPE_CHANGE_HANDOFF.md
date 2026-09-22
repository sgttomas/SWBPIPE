# H1 — Scope-change handoff (R3 integration, proposal only)

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration TASK H1.
Parent: HELP_HUMAN Agent 0. Item register: `SCOPE_CHANGE_ITEMS.csv` (same
folder, 241 items, CRLF, `#END` row carries the count).

This file proposes. It does not amend `SOFTWARE_DECOMP.md`, the DAG, any
SOW, `_CONTEXT.md`, `_STATUS.md`, ledger row, code or lifecycle state. The
owner's Direction 1 places scope change and a DAG rebuild outside this run as
typical follow-ons (`OWNER_DIRECTIONS.md`, Direction 1). Nothing here
executes until the owner acts at R4, and then only through the holder named
below.

## 1. Holder and path

- **Holder:** WORKING_ITEMS (workflow: scope-change), variant SOFTWARE, amending
  `execution/_Decomposition/SOFTWARE_DECOMP.md` and its `_ScopeChange/` state.
  The scope-change contract requires three human checkpoints (proposed change
  and impact; exact amendment and propagation plan; audited poststate). This
  handoff is candidate input for checkpoint group 1 only
  (`workflows/scope-change/resources/contract.md`, "Non-negotiable
  invariants").
- **Owner choices first.** 117 of 241 items carry `BlockedOnPacket`. For
  those, the scope-change run may not frame the amendment until the named
  packet is ruled (method R4: affected repair paths stop until the holder
  acts). Section 5 gives the table.
- **After scope change.** Accepted amendments authorise, through separate R5
  authorization, the record repairs that follow: new or re-pointed keys, SOW
  citations of the landing, and targeted reverse re-asks of the deliverables
  that area routing never reached.

## 2. Scope and coverage

The H1 scope in `R3_INTEGRATION_TOPICS.md` ("Handoffs") is:

1. every T1–T3 capability classified PRODUCT_UNOWNED, ROUTING_GAP,
   PARTIAL_UNOWNED_REMAINDER, UNKEYED_SCOPE_GAP or DUPLICATE (this set also
   contains every capability routed SCOPE_CHANGE_HANDOFF);
2. class T6-C07 (17 claim rows, the only class routed SCOPE_CHANGE_HANDOFF in
   `CLASS_INDEX.csv`);
3. the VIEW-017 duplicate (included in 1);
4. the crosswalk items placed in H1 (all fall inside 1; see section 6).

**Reproducing filters.**
- Capabilities: `R3/CAPABILITY_DISPOSITIONS.csv` where `Classification` is in
  {PRODUCT_UNOWNED, ROUTING_GAP, PARTIAL_UNOWNED_REMAINDER, UNKEYED_SCOPE_GAP,
  DUPLICATE}: 227 capabilities (57 + 44 + 68 + 57 + 1). By source: T1 45, T2
  50, T3 132.
- Claims: `R3/CLASS_ASSIGNMENTS.csv` where `ClassID = T6-C07`: 17 keys, all
  `Route = SCOPE_CHANGE_HANDOFF`, `Authority = SCOPE_CHANGE`.
- No row of `T8_ROWS.csv`, `T9_LIFECYCLE.csv`, `T11_METHOD.csv` or
  `T12_UNREACHED.csv` is routed SCOPE_CHANGE_HANDOFF, and none of the 17
  T6-C07 keys is in `T8_ROUTE_DISAGREEMENTS.csv`. Capability items carry no
  claim key, so no T8 second view applies to them.

**Item grain.** One item per capability, except two crosswalk groupings that
the topic file treats as one decision: CAP-COREB-040/041 + CAP-DATA-046
(H1-104) and CAP-FEATC-004/005 (H1-107). One item per T6-C07 claim key.
227 capabilities → 224 items; + 17 claim items = 241.

| Kind | Items |
|---|---|
| ASSIGN | 110 |
| CREATE | 57 |
| KEY_ISSUE | 56 (57 capabilities) |
| REASSIGN | 17 (T6-C07) |
| DUPLICATE_RESOLVE | 1 (VIEW-017) |
| MERGE, RETIRE | 0 (RETIRE appears only as an option inside H1-106) |

`Kind` states the shape of the proposal, not a decision. CREATE means the
evidence proposes a new owner as the leading option; the packet lists the
alternatives. Each item's `Proposal` keeps the task's own ProposedOwner text,
class, route, group and confidence. Following the brief, `ROUTING_GAPS.csv`
`Rank = PRIMARY` was not relied on; the ProposedOwner text was read instead.

The 69 T1–T3 capabilities outside these classes (NON_DELIVERABLE 35,
SHARED_OK 34) are recorded by Agent 0 through `CAPABILITY_DISPOSITIONS.csv`
and are not H1 items.

## 3. Basis and reliability

- **Accepted results:** effective values in `R3/CORPUS_CLAIMS.csv` and the
  deterministic `CAPABILITY_COVERAGE.csv`, `CAPABILITY_DISPOSITIONS.csv`,
  `CLASS_ASSIGNMENTS.csv`, `ROUTING_GAPS.csv` and `CLASS_ROUTE_TOTALS.md`.
- **Proposals:** every classification, proposed owner, group and route in
  `T1_UNMAPPED.csv`, `T2_UNMAPPED.csv`, `T3_OWNERSHIP.csv` and their notes,
  and the T6-C07 treatment in `T6_CLASSES.md` ("T6-C07"). These rest on
  sealed reverse answers, SOW text and freeze reading. None was built or run.
- **Freeze spot checks (00115c719, evidence only):** `SOFTWARE_DECOMP.md:223`
  (PKG-07 scope names modeler, editors, warning, solve and results UX, not a
  shell), `:600` (DEC-009 shell as runtime basis), `:635` (DEC-044 PKG-04
  integration tranche), `:665` (DEC-074), `:667` (DEC-076), `:685`
  (DEC-094), `:694` (DEC-103 item 5); SCA-009 `Vocabulary_Annex.md:116,125-128`
  (rows 17, 21–24); `core/model_operations/operation_applier/src/lib.rs:866`
  (`apply_operation`) and `:2003` (`deliverable_refs` DEL-16-02/DEL-16-03);
  `apps/desktop/src-tauri/tauri.conf.json:23` (CSP null);
  `apps/desktop/src/App.tsx:145-149` (App, AppSession);
  `core/gui/editors/engine.py:26` (`EDITOR_KINDS`);
  `core/solver/linear_supports/src/lib.rs:431` (`apply_linear_supports`).
  All paths are under `projects/chirality-piping/` at the freeze.
- **Weaker evidence, flagged in items:** 20 LOW-confidence capabilities; the
  T2 G2 reading that DEL-07-08 history holds misfiled work (worker notes plus
  `DEL-07-08_notes.md`); T1 O-1 (possible misreading of DEC-103 item 5).

## 4. Item groups

Groups are ordered by the packet that must act first. Each group states what
the scope-change run would amend once that packet is ruled. The options
themselves are drafted in the packets and are not repeated here.

### A. Product solve path: `core/product_physics` and the DEC-044 loop (H1-001–H1-028; 28 items; B1)

- **Capabilities.** T1 F1 (PHYS-001, 002, 003, 005, 006, 009, 010, 017, 018,
  022, 027; SOLVER-037; PHYS-021), T3-G2 (PHYS-004, 008, 011, 013, 014, 015,
  023, 024, 026), T3-G3 (SOLVER-036, 038, 039, 040, 041) and PHYS-007
  (T3 item 8, placed in B1 by the crosswalk).
- **Evidence.** Every PKG-04/PKG-05 member and DEL-10-05 disclaims the adapter
  (for example DEL-04-01 RC-04-0061, DEL-10-05 RC-10-0256, RC-10-0109).
  DEC-044 (`SOFTWARE_DECOMP.md:635`) assigns the loop to a PKG-04 integration
  tranche that no deliverable instantiates. `T1_NOTES.md` F1;
  `T3_NOTES.md` T3-G2, T3-G3.
- **Amendment shape once B1 rules.** CREATE a PKG-04 integration deliverable
  (with DEC-044's DEL-04-04 re-point), or ASSIGN into an existing member
  (DEL-04-01, DEL-04-04, DEL-10-05), or record shared infrastructure (T1 notes
  that this option leaves no verification owner). PHYS-022 leads with
  DEL-04-01 and PHYS-021 with DEL-03-06, so both are ASSIGN; PHYS-007 is a
  KEY_ISSUE whose home B1 settles (DEL-03-08 or the product-physics owner).
- **Held route.** PHYS-021 is also routed REVIEW by T1 (DEL-03-06 reverse
  re-ask). See section 7.
- **Linked items outside the group.** H1-175 (PHYS-028 key in DEL-05-01;
  code in `product_physics`), H1-215 (SOLVER-070, nonlinear README part) and
  H1-225 (DEL-04-03 CLM-010.r12, product support path).

### B. Desktop workspace shell (H1-029–H1-062; 34 items; B2)

- **Capabilities.** T2-G1 (19: SHELL-001, 002, 004, 005, 017, 018, 033, 051;
  WSUI-001, 002, 003, 004, 005, 007, 009, 010, 012, 020, 043), WSUI-011
  (T2-G2 session state), T2-G4 operations UI (WSUI-027, 028) and T3-G4
  (COREB-035, SHELL-003, 006, 010, 052, 053, 058, 061, VIEW-034, WSUI-008,
  019, 044).
- **Evidence.** All PKG-07 members answer NOT_MINE for shell chrome
  (`T2_NOTES.md` T2-G1); the PKG-07 package scope names no shell
  (`SOFTWARE_DECOMP.md:223`); DEC-009 treats the Tauri shell as a runtime
  basis (`:600`). Partial owners assign the rest to "the shell"
  (`T3_NOTES.md` T3-G4). Unowned examples: CSP null at
  `apps/desktop/src-tauri/tauri.conf.json:23`, and the dead native-menu
  listener in the CAP-SHELL-004 surface note.
- **Amendment shape once B2 rules.** CREATE a PKG-07 workspace-shell
  UX_UI_SLICE deliverable (32 CREATE items), or extend DEL-07-01/07-02
  (SOW-020 carriers) or DEL-07-08. WSUI-027/028 lead with DEL-07-08 (SOW
  CLM-004 names operation diff review), alternative DEL-07-09. T3 also offers
  "non-deliverable host wiring" for T3-G4.
- **Linked items.** H1-076 (SHELL-020, B3 and B2), H1-108 (SHELL-048,
  conditional B2), H1-121 (SHELL-016, shell as alternative).

### C. Runtime model-operation application (H1-063–H1-076; 14 items; B3)

- **Capabilities.** T3-G1: COREB-002, 004, 006, 007, 008, 009, 010, 011, 013,
  014, 015, 016, 017 and SHELL-020. COREB-012 (also T3-G1) is placed by the
  crosswalk as its own ASSIGN item in group E.
- **Evidence.** `apply_operation` at
  `core/model_operations/operation_applier/src/lib.rs:866`;
  `deliverable_refs` DEL-16-02/DEL-16-03 at `:2003`; DEL-16-02 REQ-16-02-005
  puts application outside its slice; DEL-16-03 answers UNKEYED
  (RC-16-0151, -0248, -0252); PKG-16 verifier: "No issued key owns runtime
  application" (`PKG-16_VERIFICATION.md:356`).
- **Amendment shape once B3 rules.** ASSIGN to DEL-16-03 or DEL-16-02 by SOW
  amendment and key issue, or CREATE a PKG-16 application deliverable.
- **Linked items.** H1-239/H1-240 (DEL-16-01 runtime edit contract, T6-C07,
  also B3); H1-154, H1-162, H1-193 (T3-G1-tagged keys in DEL-16-02 and
  DEL-07-01 that need not wait).

### D. Editors and panels awaiting B4, B5 or B6 (H1-077–H1-101; 25 items)

| Packet | Items | Capabilities |
|---|---|---|
| B4 load-case and self-weight editors | H1-083–085, H1-097 (+H1-096) | FEATB-022, 023, 025; FEATC-031 |
| B5 rule-check run panel | H1-081, 082, 101 | FEATB-008, 009; WSUI-035 (follows the run surface) |
| B5 attribution panels | H1-086, 095 | FEATB-031, FEATC-021 |
| B5 hanger landing | H1-078, 079, 080, 093 | COREB-046, DATA-006, DATA-051, FEATC-016 |
| B5 palette landing | H1-099, 100 | VIEW-025, 026 |
| B5 preview fixtures | H1-098 | SHELL-008 |
| B5 shared GUI helper | H1-077 | COREB-030 |
| B4 and B5 | H1-096 | FEATC-023 (helper follows FEATC-016, 022, 031) |
| B6 panels outside a no-GUI SOW | H1-087–092, 094 | FEATC-002, 003, 007, 008, 014, 015, 019 |

- **Evidence.** `T1_NOTES.md` F2 and F3 (DEC-094 `SOFTWARE_DECOMP.md:685`
  against the DEL-07-09 envelope; annex rows 21–23); `T2_NOTES.md` T2-G6;
  `T3_NOTES.md` T3-G5 and T3-G9. The DEC-094 re-point itself is shown by
  `core/gui/editors/engine.py:26`.
- **Amendment shape.** Mostly ASSIGN with SOW scope extension to the
  deliverable the packet selects. Several candidates are DEL-07-09, which is
  OPEN (`CORPUS_CLAIMS.csv` LifecycleState), so B11 (promotion of DEL-07-09)
  bears on any landing there. For DATA-006/051 T1 O-1 notes that DEL-03-07's
  reverse reason may misread DEC-103 item 5.

### E. Crosswalk decisions placed in H1 (H1-102–H1-108; 7 items, 10 capabilities)

These carry no packet. The scope-change run's own checkpoint 1 is where the
human confirms them.

| Item | Kind | Capabilities | Leading proposal | Blocked |
|---|---|---|---|---|
| H1-104 | ASSIGN | COREB-040, COREB-041, DATA-046 | DEL-08-02 (JCS hash basis, SOW#CLM-011.r02); alternative CREATE a serialization owner | — (A2 related) |
| H1-103 | ASSIGN | COREB-039 | DEL-02-05 by extension, or CREATE (same owner question as H1-104) | — (A2 related) |
| H1-105 | ASSIGN | COREC-044 | DEL-06-02 by extension, or a PKG-10 service (verifier suggestion) | — |
| H1-106 | ASSIGN | DOCS-008 | adopt `docs/MANIFEST.json` into DEL-01-01, or retire it | A6 for the adopt option |
| H1-102 | ASSIGN | COREB-012 | composite-support metadata contract; DEL-04-03 was never routed; validation stays DEL-16-02 | — (B3 related) |
| H1-107 | KEY_ISSUE | FEATC-004, FEATC-005 | one key home for annex row 21: DEL-07-02 (form surface) with DEL-07-01 | — |
| H1-108 | ASSIGN | SHELL-048 | DEL-08-01 now; DEL-02-05 once the `.opsproj` project container exists | B2, only if the project-container alternative is taken |

Visible exceptions: CONTESTED DEL-02-05 REQ-02-05-005 and CONTESTED DEL-08-01
CLM-004.r05 (H1-103, H1-104), DEL-08-02 R2 IMPLEMENTED_DIFFERENTLY /
POSSIBLE_DEFECT (H1-103). H1-104 and H1-103 should be decided together: they
are the same crate family (`T1_NOTES.md` F3; `T3_NOTES.md` T3-G9).

### F. Routing gaps: assign to a named deliverable (H1-109–H1-147; 39 items)

- **Capabilities.** The 44 ROUTING_GAP capabilities less PHYS-021 (group A),
  DATA-006, DATA-051 and FEATC-016 (group D, hanger landing) and SHELL-048
  (group E): T1 F2/F3 routing gaps, T2-G3
  (persistence commands to DEL-02-05), T2-G5 (desktop fronts of engines),
  T3-G8. By `CLASS_ROUTE_TOTALS.md`, the primary owner's package was never
  routed the capability for 30 of the 44.
- **Amendment shape.** ASSIGN by targeted reverse re-ask of the named
  deliverable, then key issue under scope change or R5 record repair where
  only a key is missing (T2-G5 "On-ruling mechanism"; T3-G8 "Proposal").
  Leading targets: DEL-02-05 (9), DEL-03-07, DEL-16-02, DEL-16-04, DEL-08-01,
  DEL-08-04.
- **Items with a precondition.** H1-113 (DOCS-001, top-level README: target
  is the ISSUED DEL-01-01, and the copy carries the CLM-009.s01 posture
  conflict; A6 and A8). H1-119 (SHELL-013 comparison bases: DEL-07-08 or
  DEL-14-04 depends on whether comparison engines enter the product; B8).
  H1-110, H1-116, H1-117 (COREC-053, FEATB-029, FEATB-030: T1 routes REVIEW
  first; section 7).
- **Visible exceptions.** OBSERVED DEL-12-04:SOW#CLM-011.r01 (H1-134);
  FIELD DEL-12-04:SOW#CLM-011.r07 (H1-110); FIRM relations on
  DEL-00-06 REQ-06-03 (H1-111, H1-112); T2 R3_OBSERVATION 1 on WSUI-018
  (H1-143: DEL-07-07 forward rows cite code its reverse answer calls shared).

### G. Keying gaps: issue keys in the building deliverable (H1-148–H1-201; 54 items)

- **Capabilities.** T3-G6 UNKEYED_SCOPE_GAP, less FEATC-004/005 (group E) and
  PHYS-007 (group A). Largest: DEL-07-01 (14), DEL-07-02 (10), DEL-07-03 (6),
  DEL-04-01 (4, `curved_bend` under DEC-070), DEL-08-01 (3).
- **Amendment shape.** KEY_ISSUE. The deliverable built the work (run
  records, `deliverable_id`, MEMORY) but no issued key covers it. Where the
  scope already names the work, the key issue is an R5 record repair; where
  it does not (for example DEL-07-03 rule-pack authoring, CLM-036.r05
  SCOPE_GREW_BY_DIRECTION), scope change adds it first.
- **Held.** H1-176 (PHYS-029): T3 routes ENGINEERING_AUTHORITY because the
  nearest key waits on the owner-held pressure reference model. Section 7.
- **Kept visible.** H1-148 (CHECKS-007): DEL-00-08 names DEL-10-04, which
  denies it; DEL-12-03 calls it tooling.

### H. Deflected and governance remainders (H1-202–H1-215; 14 items)

- **Capabilities.** T3-G9 remainders not placed elsewhere (CHECKS-030,
  COREC-039, COREC-041, DOCS-021, DOCS-049, FEATC-027, SHELL-023, SHELL-044,
  SOLVER-070) and T3-G7 governing documents (DOCS-009, 012, 013, 014, 016).
- **Amendment shape.** ASSIGN to the owner the other side's reason already
  implies (T3-G9 "Proposal"). DOCS-009/012/013/014 are routed NO_ACTION by T3:
  the proposal is to record the remainder as project governance with no
  amendment, for the human to confirm or decline at checkpoint 1. DOCS-016 is
  an R5 pointer repair in DEL-11-05. The reverse-pass tie-break gap behind
  T3-G9 (T3 O-6) is a method item in C6, not here.

### I. Other unowned panels and helpers with a best-supported owner (H1-216–H1-223; 8 items)

FEATC-012 → DEL-12-02; FEATC-022 → DEL-16-04; FEATC-029 → DEL-14-02
(CONTESTED DEL-14-02:SOW#CLM-024 kept visible; its F1 reading is in C7);
SHELL-011 → DEL-08-04; SHELL-026 → DEL-02-02; VIEW-027 → DEL-16-02 (toolkit
owner DEL-07-09 is coverage-only); VIEW-030 → DEL-07-07 or DEL-07-04; VIEW-033
→ DEL-07-02. ASSIGN by scope extension (`T1_NOTES.md` F2; `T2_NOTES.md`
T2-G6).

### J. Duplicate claim (H1-224; DUPLICATE_RESOLVE)

VIEW-017, deformed-shape overlay, is CLAIMED_BY DEL-07-01 (REQ-02) and
DEL-07-05 (CLM-004.r03, CLM-010). T3 proposes DEL-07-05 owns and DEL-07-01
COVERS as rendering host; the alternatives are DEL-07-01 owns, or a split
between rendering and result semantics (`T3_NOTES.md` T3-G11). The code sits
in DEL-07-01's `PipeViewport.tsx` (T3 cites :4182). Holder: the scope-change
run's checkpoint 1; no packet.

### K. Behaviour landed elsewhere: T6-C07 (H1-225–H1-241; 17 claim items; REASSIGN)

All 17 rows are OWNERSHIP_ELSEWHERE, layer RECORD, class Authority
SCOPE_CHANGE; AuthorityNeeded NO on 14 and OWNER on 3 (`T6_CLASSES.md`
"T6-C07"). Per the T6 on-ruling mechanism, each item records the landing and
the claiming deliverable; the scope-change outcome then authorises R5 repair
of both SOWs (the claiming SOW cites the landing, or the landing takes the
claim). No code change is implied unless the assignment moves behaviour.

| Item | Claim key | Landing | Blocked |
|---|---|---|---|
| H1-225 | DEL-04-03:SOW#CLM-010.r12 | `core/product_physics` support application (OBSERVED) | B1 |
| H1-226 | DEL-05-02:STATUS#remaining/R02 | DEL-09-04 regeneration | — |
| H1-227 | DEL-06-04:SOW#CLM-010/R-06-04-007 | DEL-06-03 completeness via `run_rule_checks` | — |
| H1-228, 229 | DEL-07-01:SOW#CLM-006.r05; CLM-011/DEL-07-01-REQ-07 | diagnostics and missing-data panels | — |
| H1-230–233 | DEL-07-02:SOW#CLM-004.r02; CLM-015/RQ-002; CLM-018.r07; CLM-025.r05 | rule-pack manager and private-library features (`RulePackManagerPanel.tsx`) | — |
| H1-234 | DEL-07-05:SOW#CLM-011/REQ-07-05-004 | SolvePanel and the unowned RuleCheckRunPanel | B5 |
| H1-235 | DEL-08-02:SOW#CLM-011.r09 | result-envelope and package status fields | — |
| H1-236 | DEL-12-04:SOW#CLM-007.r03 | DEL-12-03 telemetry tests | — |
| H1-237 | DEL-12-04:SOW#CLM-007.r04 | plugin-verification default deny; grant model unselected | B10 |
| H1-238 | DEL-13-01:SOW#CLM-005.r07 | desktop Design Knowledge panel | B9 |
| H1-239, 240 | DEL-16-01:SOW#CLM-009.r01, r03 (FIELD) | applier intent contract (FG-DEL-16-01-01; T11 SS-02) | B3 |
| H1-241 | DEL-17-02:CONTEXT#anticipated-artifacts | contract-level tables (DEC-076) or common schema files | UNASSIGNED |

H1-241 needs an owner choice that no topic list carries; it is reported to
Agent 0 as UNASSIGNED and not drafted here.

## 5. Packet dependencies

Items whose `BlockedOnPacket` names each holder (an item naming two holders is
counted under both; 117 distinct items are blocked).

| Packet or holder | Items | Where |
|---|---|---|
| B2 desktop workspace shell | 36 | group B (34), H1-076, H1-108 (conditional) |
| B1 product solve path | 30 | group A (28), H1-215 (README part), H1-225 |
| B3 runtime application and edit contract | 16 | group C (14), H1-239, H1-240 |
| B5 unowned or disputed panels | 15 | group D (14), H1-234 |
| B6 GUI panels vs no-GUI SOW | 7 | group D |
| B4 load-case and self-weight editors | 5 | group D |
| A6 ISSUED DEL-01-01 change path | 2 | H1-106 (adopt option), H1-113 |
| A8 product posture | 1 | H1-113 |
| B8 model-state persistence and comparison | 1 | H1-119 |
| B9 PKG-13 product status | 1 | H1-238 |
| B10 plugin and adapter grant model | 1 | H1-237 |
| REVIEW (T1 route; no H3 item) | 4 | H1-017, H1-110, H1-116, H1-117 |
| ENGINEERING (T3 route; no H3 item or packet) | 1 | H1-176 |
| UNASSIGNED owner choice | 1 | H1-241 |

Packets that bear on H1 items without blocking them: A2 (hash basis, for
H1-103/H1-104), B11 (DEL-07-09 is OPEN; any DEL-07-09 landing in groups D and
I), B12 D4 (storage roots, for H1-134/H1-214), C7 (F1 reach on CONTEXT rows,
for H1-218). H1 blocks nothing in P1–P3; H4 record repairs that re-point keys
into a newly scoped owner wait on the corresponding H1 item once it is
accepted.

## 6. Crosswalk placements

| Crosswalk candidate | H1 item |
|---|---|
| T1 decision 7: canonical-JSON crate (COREB-040/041, DATA-046) | H1-104 ASSIGN (DEL-08-02 best supported) |
| T2 decision 4: CAP-SHELL-048 | H1-108 ASSIGN (DEL-08-01; B2 only if DEL-02-05 container needed) |
| T3 item 7: annex row 21 key home (FEATC-004/005) | H1-107 KEY_ISSUE |
| T3 item 10: COREB-039 | H1-103 ASSIGN |
| T3 item 10: COREC-044 runner orchestration | H1-105 ASSIGN |
| T3 item 10: DOCS-008 `MANIFEST.json` | H1-106 ASSIGN (retire is the alternative) |
| T3 item 10: COREB-012 | H1-102 ASSIGN |
| T3-G11 VIEW-017 duplicate (H1 scope) | H1-224 DUPLICATE_RESOLVE |

The other crosswalk rows go to packets or H2 and are not drafted here.
PHYS-007 (crosswalk: B1) appears as H1-007 because it is in H1's
classification scope; it is blocked on B1.

## 7. Held items and coverage notes

- **REVIEW routes with no H3 item.** T1 routes CAP-COREC-053, CAP-FEATB-029,
  CAP-FEATB-030 and CAP-PHYS-021 to REVIEW (a reverse re-ask of DEL-02-04,
  DEL-02-05 and DEL-03-06) before any record repair. The H3 scope excludes
  T1–T3 ("T1–T3 carry no route column"), but these T1 rows state a route in
  Notes. They are held here and reported to Agent 0.
- **ENGINEERING_AUTHORITY route with no holder.** CAP-PHYS-029 (dormant D-67
  kernel; nearest key DEL-05-03 RQ-001.s01 waits on the owner-held pressure
  reference model; `T3_OWNERSHIP.csv`). No packet or H3 item names it.
  Reported to Agent 0.
- **UNASSIGNED owner choice.** `DEL-17-02:CONTEXT#anticipated-artifacts`
  (T6-C07; AuthorityNeeded OWNER): keep the four export schemas as
  contract-level tables under DEC-076, or author common schema files
  (`T6_CLASSES.md` T6-C07 class notes). No topic list carries it.
- **Routes inside H1 scope that are not SCOPE_CHANGE_HANDOFF.** The task routes
  of the 227 capabilities are SCOPE_CHANGE_HANDOFF 94, R5_RECORD_REPAIR 84,
  OWNER_DECISION 27, REVIEW 4, NO_ACTION 4, ENGINEERING_AUTHORITY 1, and
  13 T1 "Route: with X" rows that inherit a sibling's route (12 inherit
  OWNER_DECISION, 1 SCOPE_CHANGE_HANDOFF). H1 takes them all because the
  topic file scopes H1 by classification. R5_RECORD_REPAIR-routed items
  (mostly KEY_ISSUE and ROUTING_GAP ASSIGN) can proceed as record repair where
  the scope already names the work; H4 does not list them, since H4 covers
  `CLASS_ASSIGNMENTS.csv` and T8/T9/T11/T12 only.
- **W2 routing-gap list.** W2 named CAP-COREC-053, CAP-SHELL-032/048 and
  CAP-PHYS-039 (`W2_GATE_ASSESSMENT.md:128`). The first three are H1-110,
  H1-130 and H1-108. CAP-PHYS-039 is OWNED (DEL-10-05) in
  `CAPABILITY_COVERAGE.csv` and needs no item.
- **Rows known only from OtherCorrections.** Two rows outside H1's classes
  carry an ownership-elsewhere reading only in OtherCorrections: 
  `DEL-16-02:SOW#CLM-010/REQ-16-02-002` (class T5A-C05; OBSERVED: same finding
  as FG-DEL-16-01-01, the H1-239/H1-240 subject, B3) and
  `DEL-08-04:SOW#CLM-011.r12` (no class; CONTESTED candidate
  PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE: export panels consume
  MechanicsResult directly). They are cited for context; they are not H1
  items.
- **Code-fix readings seen in passing.** T2 notes code-fix readings on
  CAP-SHELL-013 (fixture-specific comparison bases) and CAP-SHELL-028
  (NON_DELIVERABLE; route CODE_FIX_CANDIDATE); neither is in the H2 scope
  list. Reported to Agent 0, not drafted.

## 8. On-acceptance mechanism

1. The owner rules the blocking packets at R4 (B1–B6, B8–B10, A6, A8, and
   the UNASSIGNED item if Agent 0 places it).
2. A WORKING_ITEMS scope-change run (SOFTWARE variant) takes the accepted
   items as its checkpoint-1 proposal: new deliverables (groups A, B, possibly
   C), SOW scope extensions (groups D, F, H, I), key issue (group G), the
   VIEW-017 resolution and the T6-C07 bindings. Checkpoint 2 fixes the exact
   `SOFTWARE_DECOMP.md`, DAG and `_CONTEXT.md`/`_STATUS.md` writes; the DAG
   rebuild follows.
3. Separately authorised R5 tranches then repair records: reverse answers,
   keys, and SOW citations of landings. ISSUED DEL-01-01 is touched only
   through the A6 change path (H1-106, H1-113).
4. Targeted reverse re-asks are needed for the 30 ROUTING_GAP capabilities
   whose primary owner's package was never routed the capability
   (`CLASS_ROUTE_TOTALS.md`), and for the four REVIEW-routed T1 rows.

## 9. Risks

- **If left undecided.** The product solve path, the desktop shell and the
  only mutating path of every editor operation stay outside every
  deliverable's acceptance, verification and Remaining lists (groups A–C).
  Built work in 57 keying gaps stays invisible to later loops. Known defects
  (for example the CAP-SHELL-004 dead listener) have no owner to route to.
- **Of acting on this register without the packets.** Creating shell or
  product-physics deliverables before B1/B2 would pre-empt the owner's
  choice among create, extend and shared infrastructure.
- **Of the grain.** One item per capability overstates the number of
  decisions: groups A, B and C are three decisions each spanning 14–34 items.
  The scope-change run should take each group as one amendment.
- **Evidence limits.** Proposed owners rest on reverse answers and scope
  text; NOT_MINE answers were read only for candidate owners (T3 limits), and
  20 capabilities are LOW confidence.

## 10. Reconciliation

A read-only script re-derived both populations from `CAPABILITY_DISPOSITIONS.csv`
and `CLASS_ASSIGNMENTS.csv` and compared them with `SCOPE_CHANGE_ITEMS.csv`.
Result: 241 items with unique IDs; the capability set equals the 227-capability
filter (0 missing, 0 extra, 0 duplicated; PARTIAL_UNOWNED_REMAINDER 68,
PRODUCT_UNOWNED 57, UNKEYED_SCOPE_GAP 57, ROUTING_GAP 44, DUPLICATE 1); the
REASSIGN keys equal the 17 T6-C07 keys; every `Kind` is in the brief's
vocabulary; CRLF throughout; `#END` count 241; 117 items blocked. Counts in
this file were taken from that script.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
handoff describes records, ownership and proposed routing only. It makes no
certification, code-compliance, professional-approval or
engineering-acceptance claim.
