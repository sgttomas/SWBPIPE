# C6 — Method items for owner or HELPS_HUMANS adoption

Packet writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
This is a proposal. It repairs, rules and changes nothing, and it amends no
convention.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.
`F:` = the freeze `projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`.

## 1. Decision

Fifteen method questions that R2 and R3 raised about the run's conventions,
tooling and gate rules. For each, the question is whether to adopt a reading
(for this run's R4/R5 and for R6 backcheck), amend a convention or tool for
later runs, or leave the item as a recorded limit.

**Holders.**
- **OWNER** where the item reads a RULED convention or a gate rule the owner
  set (CONVENTIONS Parts C and F; WAVE_PLAN gate and rerun rules): M1–M5, M7,
  M8, M12, M13.
- **HELPS_HUMANS** for changes to the reusable reconciliation workflow, its
  validator or its R1 routing design: M9, M10, M11, M14. HELPS_HUMANS drafts;
  the owner adopts (instruction changes need their own scope, AGENTS.md).
- **ENGINEERING** for the substance under M6 and M15 (units; diagnostic
  boundary), carried in H3; C6 decides only the classification reading.

| ID | Item | Source | Holder |
|---|---|---|---|
| M1 | Is a schema-only SOW stale once product physics lands? | W2; `WAVES/W2/PKG-03/PKG-03_VERIFICATION.md` W3 | OWNER |
| M2 | Do envelope-level round-trip tests satisfy AB-00-04, or only a library-level harness? | W2; PKG-03 W4 | OWNER |
| M3 | Is VER-001 a CP-09 SOW-parity row? | W2; `WAVES/W2/PKG-04/PKG-04_VERIFICATION.md` W-3; PKG-01 §CP-09 | OWNER |
| M4 | Import-checker reverse answers: CONSTRAINS or COVERS? | W2; PKG-03 §6 | OWNER |
| M5 | Does a draft policy (`IP_AND_DATA_BOUNDARY.md` §4, DEL-03-07) count as governing? | W2; T7-C01 exception | OWNER |
| M6 | Unit safety only at metadata grain: gap or accepted grain? | W2; PKG-04 §7 item 3 | OWNER (reading); ENGINEERING (substance) |
| M7 | The rerun-rule reading (quiet-class firm error; 100%-sampled class moving to INVARIANT) | W3; PKG-08 §7 item 1; PKG-15 §7 item 1 | OWNER |
| M8 | Should a post-gate package well over 5% (PKG-12 at 9.5%) come to the owner? | W3 | OWNER |
| M9 | The `.sNN` batch blind spot | W2; W3 consistency; T11 SUBCLAIM_SPLIT | HELPS_HUMANS |
| M10 | The routing-design gap | `R3/ROUTING_GAPS.csv`; `R3/COVERAGE_AND_QA.md` §3 item 2 | HELPS_HUMANS |
| M11 | The DEL-15-02 rerun-launch tier hint | W3 departure 5; COVERAGE_AND_QA §3 item 5 | HELPS_HUMANS (launch discipline); OWNER (whether to re-derive the tier) |
| M12 | A corpus-level F7 ruling on subject reading | T12 observation 2 | OWNER |
| M13 | The tier for "Still TBD item since ruled" | T5B observation 2 | OWNER |
| M14 | A reverse-pass tie-break for mutually deflected remainders | T3 O-6; T3-G9 | HELPS_HUMANS |
| M15 | The diagnostics boundary split (classification part) | T11 SS-01, R-03 | OWNER (reading); ENGINEERING part in H3 |

## 2. Background

- **Conventions** (`RUN/CONVENTIONS.md`): B2 reverse-pass answers (line 122);
  C3 tiers (line 200); C5 layers (line 228); C7 one cause, gap wins (line 289);
  F1 no aligned row with a recorded gap (line 406); F3 origin test and the
  Direction 8 readiness clause (line 428); F7 no product caller (line 462); F8
  tier of the gap wins (line 469). CP-09 (`RUN/CANONICAL_SITUATIONS.md:82`) and
  CS-04 (`:33`) define parity rows and the Still-TBD `.sNN` split.
- **Gate and rerun rules** are in `RUN/WAVE_PLAN.md` ("Verifier reruns", line
  46) and the verifier briefs. WAVE_PLAN sets no post-gate rate condition
  (`W3_ASSESSMENT.md:95`).
- **Prior readings.** Direction 8 (`OWNER_DIRECTIONS.md:139`) is the only owner
  confirmation of a convention reading in this run so far. The PKG-03 D1 and
  PKG-15 D1 rerun calls were each run as one rerun cycle (W3 assessment,
  Reruns).
- **Code.** M1: `F:core/product_physics/src/lib.rs` consumes DEL-03-03 bend
  fields (PKG-03 W3, lines 89–100 and 3860–3900 as cited). M2: the desktop
  store round-trips model snapshots with materials (PKG-03 W4). Neither was run.

## 3. Options

For every item the generic options are: (a) adopt reading X; (b) adopt reading
Y; (c) leave it a recorded limit for R6 (no change). Item-specific readings:

- **M1.** X: a SOW that describes only the schema is stale (DOC_BEHIND_CODE or
  SCOPE_REDIRECTED_BY_RULING) once DEC-045/DEC-070 product work consumes it. Y:
  it stays accurate for its schema subject (F7 subject reading). Affects the
  DEL-03-03..06 SOW rows and `DEL-03-05:SOW#completion-and-reliance-basis-epistemology/AC-001`.
- **M2.** X: envelope round-trip satisfies AB-00-04. Y: a library-level harness
  is required where the requirement names one. Affects
  `DEL-03-01:SOW#CLM-011/REQ-03-01-007`, `DEL-03-01:SOW#production-and-verification-method-praxeology/VER-001`,
  `DEL-03-05:SOW#CLM-010.r08`.
- **M3.** X: VER-001 is a CP-09 row (EVIDENCE_OVERTAKEN, AuthorityNeeded NO;
  ALIGNED where a PASS record matches). Y: VER-001 names a broader review and
  takes its own gap (PARTIALLY_IMPLEMENTED, REVIEW). Under X, T6 observation O3
  moves `DEL-04-04/05/06` VER-001 from T6-C05 to T5B, so exactly-once coverage
  must be re-checked.
- **M4.** X: CONSTRAINS (AB-00-07 basis). Y: COVERS. No routing effect; one
  relation should take one answer.
- **M5.** X: a draft policy is governing where no ruled text exists. Y: it is
  evidence only; claims citing it stay AUTHORITY_UNCLEAR. Decides the CP-10
  treatment of `DEL-03-07:SOW#CLM-026.r01` (the class itself is A3's) and the
  basis C2 and C5 use.
- **M6.** X: metadata-grain unit handling is a PARTIAL gap (INVARIANT, as
  sealed). Y: metadata grain is the accepted grain for these surfaces.
  Engineering supplies which fields need dimensional checks.
- **M7.** X: the rule fires literally (a firm error in a 100%-sampled class that
  moves a quiet row to INVARIANT forces a rerun; a quiet-class firm error does
  not). Y: such rows may be settled through RESOLUTIONS.csv (F6) instead. Under Y
  nothing is rerun; under X, DEL-08-05 might need a rerun (PKG-08 §7 item 1).
- **M8.** X: a post-gate package over 5% goes to the owner before acceptance.
  Y: acceptance on the verdict stands (as done for PKG-12). Under X, PKG-12
  would come back to the owner now.
- **M9.** X: extend batch mode to compare minted `.sNN` sub-claims under shared
  parents (tool change). Y: keep T11-style screens at R3.
- **M10.** X: widen R1 routing so proposed owners get a reverse pass on
  ROUTING_GAP capabilities (44 capabilities; 30 primary owners never routed).
  Y: accept R3 as the owner of the unmapped set, as the method says, and mark
  H1 items as resting on scope text and code only.
- **M11.** X: re-derive the INVARIANT tier on the DEL-15-02 rows independently.
  Y: accept it as contested evidence (R3's current treatment). Either way, adopt
  a launch-message rule that verifier conclusions are not passed to rerun workers.
- **M12.** X: one F7 subject test (engine claims vs app/runtime claims) applied
  corpus-wide. Y: leave per-row readings. Affects `DEL-13-03:SOW#CLM-005.r04`
  (WEAK), the four DEL-05-05 CONTESTED rows T12 names (`SOW#CLM-010.r06`,
  `.r07`, `.r08`, `.r10`) and
  `DEL-08-06:SOW#completion-and-reliance-basis-epistemology/AC-001`.
- **M13.** X: LOCAL_DESIGN (sealed in 58 T5B-C05 rows and many T5B-C06 rows).
  Y: PROJECT_BASELINE; then T5B-C05 and parts of T5B-C06 move from R5 record
  repair to an owner confirmation.
- **M14.** X: add a tie-break to B2 (for example, the deliverable whose SOW names
  the element takes it). Y: leave mutual deflection to R3 (T3-G9, 15
  capabilities) and H1.
- **M15.** X: one behaviour takes one cause across ledgers (POSSIBLE_DEFECT where
  a requirement is breached). Y: PARTIAL_SLICE and ALIGNED readings stand per
  subject. C1 subject 2 decides the substance; H3 carries SS-01.

## 4. Evidence and reliability

All items come from verifier reports, Agent 0 assessments and R3 task
observations (R3 proposals; verifier reports are sampled judgment). None has an
owner ruling behind it except the Direction 8 precedent. Screens behind M9 and
M15 are scripted candidate checks, not proofs (T11 Coverage). The M10 figures
were recomputed by this writer from `R3/ROUTING_GAPS.csv`: 44 ROUTING_GAP
capabilities; of their PRIMARY rows, 30 have `OwnerPackageAreaRouted = NO`.

## 5. Affected claims

C6 is a method packet. It claims **no class portion**. The rows below are those
each reading would move; they are counted by their class owners.

| Item | Keys or filter | Classes (counted by) |
|---|---|---|
| M1 | DEL-03-03..06 SOW rows named in PKG-03 W3; `DEL-03-05` AC-001 (ALIGNED, CONTESTED) | various T5A/T6 (H2–H4) |
| M2 | 3 keys in §3 | T6-C02 ×2 (H2); DEL-03-05 row not divergent |
| M3 | `DEL-04-0{1..6}:SOW#production-and-verification-method-praxeology/VER-001` | T5B-C01 ×3 (H3), T6-C05 ×3 (H3) |
| M4 | Reverse answers RC-03-0072, RC-03-0107 (no claim key) | none |
| M5 | `DEL-03-07:SOW#CLM-026.r01` | T7-C01 (A3) |
| M6 | `DEL-04-04:SOW#CLM-004`, `DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-06`, `DEL-04-05:SOW#CLM-012/DEL-04-05-RQ-006`, `DEL-04-05:SOW#CLM-021` | T6-C05 (H3) |
| M7 | DEL-08-05 `CONTEXT#description` and REQ-002 (PKG-08 §7); DEL-15-02 CLM-005/CLM-027 (PKG-15 §7) | various |
| M8 | PKG-12 (all 5 deliverables) | package-level |
| M9, M10, M14 | tooling; `R3/ROUTING_GAPS.csv` `Classification == ROUTING_GAP`; T3-G9 | none (H1 items rest on them) |
| M11 | `DEL-15-02:SOW#CLM-005.r01`, `.r02`, `.r03` | T6-C03 ×1, T7-C06 ×2 (H2) |
| M12 | 6 keys in §3 | T6-C01 ×1; others not divergent |
| M13 | `DEL-15-02:CONTEXT#architecture-basis-injection.s02` (CONTESTED); `DEL-15-02:SOW#CLM-004.r07`, `CLM-012.r02`, `CLM-019.r09`, `CLM-020.s03` (OBSERVED, FG-DEL-15-02-06); the 58 T5B-C05 Still-TBD rows | T5B-C05, T5B-C06 (H4) |
| M15 | T11 SS-01 row keys (`R3/TASKS/T11_METHOD.csv` Subject `SS-01`) | T7-C05 (C1) and ALIGNED rows |

**OtherCorrections carrying these items:** the CONTESTED/OBSERVED/WEAK texts on
the M2, M3, M5, M12 and M13 keys (see `CORPUS_CLAIMS.csv` `OtherCorrections`).

## 6. Risks

- **Undecided.** One situation keeps two classifications, so R5 tranches and R6
  backcheck count it twice or inconsistently (M1–M4, M12, M13, M15). Gate
  outcomes rest on a rule read two ways (M7, M8). Tool and routing gaps recur in
  later runs (M9, M10, M14). One tier stays not independently derived (M11).
- **Adopting a reading mid-run.** Readings that move rows between classes (M3,
  M13) change R3 counts; exactly-once coverage must be re-run before R5.

## 7. Recommended routing

- **M9, M11 (launch rule), M14:** the evidence supports a HELPS_HUMANS workflow
  or validator change for later runs; none changes this run's rows.
- **M10:** the evidence supports marking every H1 item on a ROUTING_GAP
  capability as not reverse-tested; whether to widen routing is HELPS_HUMANS'.
- All other items: no recommendation; owner's call.

## 8. On-ruling mechanism

- **Owner readings (M1–M8, M12, M13, M15 reading part).** Recorded by Agent 0 in
  `R3_SYNTHESIS.md` as owner-confirmed readings (the Direction 8 pattern). No
  sealed ledger is patched (F6). If a reading moves rows between classes, Agent 0
  recounts the partition before H4 is finalised. Readings meant to bind later runs
  go to HELPS_HUMANS as convention amendments.
- **HELPS_HUMANS changes (M9, M10, M11 launch rule, M14).** A workflow revision
  under `create-workflow` to the reconciliation workflow, validator or R1 routing
  design, adopted by the owner; not applied to this run's sealed records.
- **M8 under X.** PKG-12 findings go to the owner as a package item now.
- **M6, M15 substance.** H3 engineering items.
- Nothing executes until the holder acts. R5 needs separate authorization.

## 9. Dependencies

- **Blocks:** C2 and C5 (M5); C1 subject 2 route (M15); H4 tranche membership
  for T5B-C05/C06 (M13) and PKG-04 VER-001 (M3); H1 items on ROUTING_GAP
  capabilities (M10); H3 items for M6.
- **Depends on:** none.
- **Placed here by Agent 0 (questions attached to the owner session; options not
  drafted):**
  - U7, under M2 — T6-C02 rows offering "narrow or defer by ruling" on the
    round-trip subject: `DEL-03-01:SOW#CLM-011/REQ-03-01-007` and
    `DEL-03-01:SOW#production-and-verification-method-praxeology/VER-001`.
  - U2 — local `Dependencies.csv` rows still ACTIVE where DAG-010 retired them
    (T11 D-02): R5 record repair, or a DAG rebuild outside this run (which would
    need separate authorization).
  - U4 — does the CONVENTIONS F3 origin test apply to the item or to the assessed
    sentence? (`DEL-12-02:SOW#CLM-037/REXC-OI-002`, T5A-C08).
  - U9 — no DivergenceLayers value fits a pure product gap (claimed code exists,
    no product path calls it; `WAVES/W3/W3_ASSESSMENT.md:98`, PKG-15 verifier §6).
  - U10 — the rerun-launch hint as a method rule: already drafted as M11.
- **Related:** C7 (the T8 readings are separate; M13 is not a T8 cluster).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
