# DEL-13-02 notes — W3 PKG-13, rerun cycle 1

This is a fresh worker encoding DEL-13-02 under CONVENTIONS Part F. The files from
the first run were moved unchanged into `superseded_1/` before any work began.
I did not read them. The forward ledger is sealed
(`DEL-13-02_SEAL.txt`, SHA-256 `637db6f4…ab02`). I read the verification report
and the routing file only after sealing.

## Path aliases

- `DEL` = `projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/`.
  Evidence columns cite these paths in full; they contain spaces and resolve at the freeze.
- Tokens that start at the project root (`schemas/…`, `tests/…`, `core/…`) resolve under
  `projects/chirality-piping/`.
- Parity and claim-map records sit at the repository root, under
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…/TASK-PIP-13-02/`.

## Judgment calls

1. **FG-01: unit finding (8 PARTIALLY_IMPLEMENTED rows, plus Remaining R01).** At the freeze
   the schema has two unit defects.
   - `Parameter.value` is a `oneOf` of string, boolean, `Quantity` or reference list, and it
     does not depend on `value_kind`. A parameter declared `value_kind: quantity` therefore
     validates with a bare string and no unit metadata.
   - `Quantity.dimension` lacks `force_per_length`. SPEC §4 (line 172) and the `units.schema.yaml`
     DimensionId list it as accepted PKG-02 vocabulary. TP-PHYS-014-A added it on 2026-05-17,
     the day after DEL-13-02's alignment work. The test compares the enum with its own frozen
     local set, so it passes while the schema diverges.

   CONTRACT maps OPS-K-UNIT-1 to schema validation, so I tagged the cause `POSSIBLE_DEFECT`.
   The rows are CLM-003.r13, CLM-009, R-13-02-005, CLM-012/R-13-02-005, CLM-013, CLM-018 (step 7),
   CLM-019.r04 and CLM-024 (bullet 3). CLM-009 and CLM-024 are assessed at block level. CLM-018
   is also at block level, with its other steps noted as holding.
2. **Tier for FG-01: INVARIANT.** The requirement restates CONTRACT OPS-K-UNIT-1 and SPEC §4
   ("dimensionless values are not a fallback for missing units"). The gap touches unit safety
   itself: a bare-string quantity is admitted. F8 therefore gives INVARIANT, with layer
   `BASELINE` (the accepted PKG-02 unit contract).
3. **STATUS#remaining/R01 is `REMAINING_STATE_MISMATCH`, not ALIGNED with `OPEN_ACTION`.** The
   action is open: the human disposition is still `TBD`. But the clause "despite current
   unit-vocabulary evidence" asserts evidence that no longer matches the code, so F2's
   accurate-text condition fails.
4. **FG-02: downstream "remain TBD" wording (3 rows).** The rows are CLM-004.r07, CLM-013.s01
   and CLM-027. Runtime constraint validation exists as the DEL-13-03 engine with tests. The
   transform contract carries `constraint_refs`, and a constraint warning panel exists in
   `core/gui/design_workspace`. None of these has a shown product caller.
   - Rows whose main substance is that "remain TBD" status are STALE_REVIEW_OR_EVIDENCE ·
     DOC_BEHIND_CODE. The text was first declared on 2026-06-07 (`bc5759133`), so it is not
     setup text.
   - Where the clause trails a row about something else (CLM-004.r03, CLM-005, R-13-02-009
     verification, AC-001), I read it as the scope boundary, which holds. I judged those rows
     on their main substance.
5. **FG-03: stale pointers (CP-02).** CLM-006, CLM-015, CLM-017.r02/r03/r04 and CLM-019.r07 pin
   revision 0.7 or 0.8, DAG-006 or DAG-007, or cite `INIT.md` (removed 2026-07-04, `9c4caf8fd`).
   Rows where INIT.md is only one of several sources for a requirement stay on their substance
   and note the stale pointer.
6. **FG-04: parity (CP-09).** The evidence map says `NONE_FOUND`. The A3 discovery step found
   PASS parity records (`PARITY_REPORT.json`, `PARITY_2.json`) that bind SOW `43d9ea2f…`. The
   frozen SOW is `78a2ff6e…`, after `34fee9bdf` and `8fac6631a`. So AC-001, VER-001 and the
   matrix OUT-001 are EVIDENCE_OVERTAKEN.
7. **CP-04: rename residue.** The schema's `$id` (`https://openpipestress.org/…`) and title carry
   the former name. This is recorded once, on the SOW SURFACE row, with default fields: it is
   not one of the four special identifiers. R-13-02-010, CLM-003.r05 and the other rows that
   describe the `$id` are ALIGNED on substance.
8. **F7.** Only its test loads the schema; nothing in product code references it. Every ALIGNED
   row that cites the schema carries `PRODUCT_CALLER: NONE`. The claims are about the schema
   (the data contract), so F7 allows ALIGNED.
9. **CP-01.** In CLM-020, the "Updated Datasheet.md …" line traces to `7bee9ae41`, so it is
   STALE_SETUP_SPECIFICATION.
10. **CONTEXT#architecture-basis-injection.s01.** The "PKG-00 at SEMANTIC_READY" statement: all
    eight PKG-00 deliverables are IN_PROGRESS. Readiness states count as review states (F3), so
    this is STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT. Advancing any lifecycle state is for the
    owner.
11. **Unsplit blocks** (C1 allows assessing a block directly): CLM-002, CLM-011, CLM-023, CLM-025
    and CLM-027. Split blocks: CLM-003, CLM-004, CLM-012, CLM-017 and CLM-019. CLM-013 uses
    `.s01`, since it has no `.rNN` keys.
12. **Observation, not a finding.** This schema's Reference type is `DesignKnowledgeRecord`,
    where `model.schema.yaml` uses `DesignKnowledge`. It is noted on R-13-02-002 for R3.

## Canonical departures

None. CLM-015 is a CP-03 "current declaration" row. I wrote `CP-02` in `CanonicalSituation`
because its disposition follows CP-03's rule for superseded pins (CP-02), and its delegation
clause is recorded as not relied on (A4).

## Convention friction

- F2 is silent where a Remaining item's action is open but a subsidiary clause is inaccurate.
  I treated inaccuracy as failing the "text is accurate" condition.
- C6 does not settle whether a false declared-state vocabulary claim is PARTIALLY_IMPLEMENTED
  or IMPLEMENTED_DIFFERENTLY. I used PARTIALLY_IMPLEMENTED throughout FG-01, because each such
  row also holds true elements, and to keep the group uniform.

## UNKNOWN rows

None.

## Gate evidence and incidental reads

- Test pass status comes from `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (pytest over
  `tests/`, 1,138 passed, 0 failed; not rerun).
- The `json.tool` result is taken from the 2026-06-07 run record (not rerun).
- Disclosure: while reading, I ran a single read-only `json.load` parse of the frozen schema in
  the freeze checkout. It wrote nothing and I do not cite it as evidence.

## After sealing: verification report (DEL-13-02 findings)

The report's DEL-13-02 findings are F2 (CLM-019.r04), W1 (CLM-018), and X1 to X5 (R-13-02-005
verification, CLM-003, CLM-009, CLM-024, R-13-02-005 note). My sealed ledger reached the same
non-aligned result on every one of those rows independently. Differences to record:

- **Tier.** For its rows the verifier proposes `PROJECT_BASELINE`; I used `INVARIANT` (item 2
  above). Its layer (`BASELINE`), baseline class (`NONE`), cause (`POSSIBLE_DEFECT`) and
  `AuthorityNeeded` (`REVIEW`) match mine.
- **Disposition on declared-state rows.** For X1 (verification row) and the DEL-13-01 analogue
  the verifier proposes `IMPLEMENTED_DIFFERENTLY`; I used PARTIALLY_IMPLEMENTED. Both are
  defensible.
- **Vehicle.** The verifier suggests `.sNN` rows for CLM-003, CLM-009 and CLM-024. CLM-003 has
  issued `.rNN` keys, so `.r13` is the correct vehicle (C1 bars `.sNN` there). CLM-009 and
  CLM-024 are assessed at block level with the other elements noted as holding.

Agent 0 may want to settle the tier and disposition choice together with the DEL-13-01
CLM-005.r05 resolution, so the FG-01 cluster is uniform in R3. Reading the report does not change
my view of anything sealed.

## Reverse pass

- **RC-13-0308** (the constraint schema) is `CLAIMED_BY`.
- **Ten capabilities** whose entry points the forward ledger cites get specific `NOT_MINE`
  reasons (F5): the DEL-13-03 engine, the DEL-13-04 transform, the GUI workspace composer, the
  units, model and design knowledge schemas, the registers, CONTRACT, the IP boundary and SPEC.
- **The other 312** are `NOT_MINE`, with the surface-scoped template reason.

The reverse pass did not change my view of anything sealed.

## Batch consistency

`--batch` over my one forward file returned PASS with 0 findings. The package batch over all
four ledgers is for the parent.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since 2026-09-19, Piping
selects work through owner-steered work graphs.

Dispositions here are agent judgments, not owner rulings. Nothing here states or implies release,
approval, compliance or certification. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
