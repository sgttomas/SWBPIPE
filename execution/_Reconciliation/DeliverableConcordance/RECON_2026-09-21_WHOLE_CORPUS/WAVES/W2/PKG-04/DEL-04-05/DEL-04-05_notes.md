# DEL-04-05 — notes (W2, PKG-04)

TASK worker, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`, gate wave W2.
Forward ledger sealed: `DEL-04-05_forward.csv`, SHA-256 `e3570bd7bc8817a301d2d34d26941500807496c5fe933355f62d2e000e3deb49`,
71 rows (69 required keys plus 2 CS-04 `.sNN` sub-claims). Reverse:
`DEL-04-05_reverse.csv`, 340 answers. Evidence read only from the frozen
checkout `00115c719`. Cross-deliverable conventions are in
`../_WORKER_DEL-04-04_NOTES.md`.

## Path aliases

- `PH` = `core/solver/performance_harness/src/lib.rs` (this deliverable's
  crate, package `open_pipe_stress_solver_performance_harness`).
- `SD` = `core/solver/sparse_direct/src/lib.rs` (DEC-023 in-repo sparse
  solver, measured by the harness).
- `VB` = `validation/benchmarks/` (DEC-050 and DEC-053 observation and
  policy records).
- Per-test pass status comes only from the B4_4 sweep log (not rerun).

## Judgment calls

- **Setup text dominates.** Almost every SOW block is text present at
  `7bee9ae41` describing a future harness. Where the implemented harness or a
  ruling overtakes it, the row is `STALE_SETUP_SPECIFICATION`, with cause
  `DOC_BEHIND_CODE` (future-harness framing) or `SCOPE_REDIRECTED_BY_RULING`
  (library and size bands settled by DEC-023/DEC-053, FG-DEL-04-05-02).
- **Release thresholds.** Each "thresholds TBD" statement is read as release
  thresholds, which are still unset; the bounded DEC-050 generated-grid and
  DEC-053 observation-set policies are not release thresholds. CLM-019 and
  CLM-029 are therefore `ALIGNED`. CLM-004 is stale only for "pending solver
  prototype".
- **F7.** The harness is a test suite with no product caller, and its claims
  concern the harness itself; its `ALIGNED` rows carry `PRODUCT_CALLER: NONE`.
- **RQ-001** (separation from solver logic) holds by structure, but its own
  hook (a module boundary review once implementation exists) was not
  located → `PARTIALLY_IMPLEMENTED`, `PROJECT_BASELINE` (AB-00-02), MEDIUM.
- **RQ-004 / CLM-013 (IP/data boundary).** Fixtures are generated in code
  as `InventedPublic`, and unknown or protected provenance is rejected
  (tested). The only review (PB-001, 2026-05-16 agent audit) predates the
  grid fixture and the DEC-050/DEC-053 sets → RQ-004
  `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN`. CLM-013 requires a
  human/IP review disposition, which was not located → `UNKNOWN`. Both
  `INVARIANT · IP_DATA;RECORD`.
- **RQ-005** is `PARTIALLY_IMPLEMENTED`: records keep every disclosure field,
  but the requirement's result-envelope test hook is live (the result schema
  exists) and unmet.
- **Unit safety (FG-DEL-04-05-01).** RQ-006 and CLM-021: fixture unit
  metadata is a reproducibility basis only, with no dimensional check;
  PKG02-001 is pending a human disposition. R01 is `ALIGNED` with
  `OPEN_ACTION` RQ-006.
- **Remaining.** R02 (release sparse thresholds) and R03 (hosted-CI sparse
  evidence gated by DEC-059) are accurate and carried by no governing row, so
  each is `DOCUMENTED_UNIMPLEMENTED · DEFERRED_BY_RULING` (F2).
- **PDU-055 declarations** (CLM-002, CLM-010, CLM-025) pin SOFTWARE_DECOMP
  0.8 and DAG-007, so they take CP-03 with CP-02 fields; their delegation to
  Remaining is not relied on (A4).
- **OUT-001 (matrix).** CP-09 overtaken: the PASS parity hash `afc3a2da...`
  differs from the frozen SOW `914f5949...`.

## Canonical departures

None. All seven CS rows take their assigned fields.

## Convention friction (smallest fix)

1. **Evidence paths with spaces.** Part D evidence columns may not hold
   spaces, but every file inside a deliverable folder
   (`execution/PKG-04_Solver Core and Numerical Methods/...`) has spaces in
   its path. Such files (ScopeOfWork, _STATUS, Review_Findings, run records)
   are cited in `ContextRefs` and named in Notes, with `NOT_APPLICABLE` in the
   evidence column where they were the only evidence. Fix: let the validator
   accept a quoted or percent-encoded path token, or a `DELDIR:` alias.
2. **CS-04 sub-claims are unkeyed.** The `.s01` (PKG-00 `SEMANTIC_READY`) and
   `.s02` (still-TBD list) sub-claims carry text shared by eleven
   deliverables, but `.sNN` keys have no `BodySHA256`, so batch mode cannot
   check them across workers. Fix: issue the CS-04 parts as keyed sub-units,
   or add them to the canonical table.
3. **Conditional verification hooks** ("once implementation exists", "once
   the result schema exists") are treated as live once their condition has
   passed; an unmet hook then makes the requirement non-aligned (F1). A line
   in C6 would make this explicit.
4. **Tier for missing verification of a holding contract invariant.** F8
   covers a stale record clause, not a missing test or review for a boundary
   that holds structurally. I used `INVARIANT` with layer `BASELINE` (the
   gap touches the invariant's subject). A ruling either way would settle it.
5. **`.rNN` rows not used.** No block was split; mixed blocks are judged
   whole with the gap's disposition and the aligned parts named in Notes.

## UNKNOWN rows — smallest next check

- `DEL-04-05:SOW#CLM-013`: search AgentRuns and DEL-04-05 run records for a
  human or IP review disposition of the invented benchmark fixture basis.
  Where none exists, record one for the chain, grid and DEC-053 fixture sets.

## Did the reverse pass change my view?

No. The harness capabilities (RC-04-0254, 0220, 0141, 0038) and the committed
observation and policy records (RC-04-0071, 0116) map onto the keys the
forward ledger used. The sparse solver capabilities are measured, not owned,
which is consistent with RQ-001.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed forward ledgers
(DEL-04-04, DEL-04-05, DEL-04-06): **PASS, 0 consistency findings**. No
pair needed justification or correction.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row. Since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining` (C9).

## Boundary disclosure

While computing final hashes I ran a `shasum` glob over `WAVES/W2/PKG-04/*/`,
which printed the forward-ledger hashes and SEAL lines of DEL-04-01 to
DEL-04-03 (another worker's folders). I read no ledger content from them and
nothing in my ledgers depends on them. Reported for the verifier.

Agent dispositions are not owner rulings. No release, approval, compliance or
certification claim is made or implied. Standard claim fence applies
(F-PIP-2; claims taxonomy per DEC-081).
