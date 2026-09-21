# DEL-04-06 — notes (W2, PKG-04)

TASK worker, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`, gate wave W2.
Forward ledger sealed: `DEL-04-06_forward.csv`, SHA-256 `8450435e6e1568ffda21f85cef02f571cea593ac46cc7457791787d132964156`,
84 rows (82 required keys plus 2 CS-04 `.sNN` sub-claims). Reverse:
`DEL-04-06_reverse.csv`, 340 answers. Evidence read only from the frozen
checkout `00115c719`. Cross-deliverable conventions are in
`../_WORKER_DEL-04-04_NOTES.md`.

## Path aliases

- `DG` = `core/solver/diagnostics/src/lib.rs` (this deliverable's crate,
  package `open_pipe_stress_solver_diagnostics`); `DGR` = its README.
- `PP::solver_diag_code` = the product mapping of `SolverDiagnosticCode` to
  product diagnostic strings.
- `JUN5` = `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md`
  (the "19 tests" record).
- Per-test pass status comes only from the B4_4 sweep log (not rerun). The
  frozen crate has 24 tests.

## Judgment calls

- **Most SOW text dates from the 2026-06-05 document alignment**
  (`2c93dea83`), after the initial migration, so stale rows are mostly
  `STALE_REVIEW_OR_EVIDENCE`. CLM-022 and CLM-023 (four-document checks) are
  present at `7bee9ae41` → `STALE_SETUP_SPECIFICATION`.
- **Sparse TBD overtaken (FG-DEL-04-06-01).** The text was written six days
  before DEC-023 (2026-06-11), which "resolves the SparseSolverTbd
  instrumented TBD"; DEC-053 later promoted sparse to the default. CLM-004,
  CLM-005, CLM-007, REQ-010, CLM-028, CLM-031 and CLM-033 still treat sparse
  selection as unresolved → `SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN`.
  AC-001 and CLM-030 are `ALIGNED`, because they state only that the TBD
  diagnostics exist, which is true.
- **"19 tests" evidence (FG-DEL-04-06-02).** CLM-008, CLM-015/REQ-001 and
  CLM-019 rely on the June 5 record → `EVIDENCE_OVERTAKEN`.
- **F7.** Product physics consumes the crate's codes and severities, but the
  frame, support and primitive-load mapping helpers, condition-ratio
  classification, `with_canonical_ref` and `with_quantity_unit` have no
  product caller (only benchmarks, the harness or the GUI mirror). The claims
  are about the crate, so those rows are `ALIGNED` with `PRODUCT_CALLER: NONE`
  naming the uncalled surfaces.
- **MEMORY** opens with an undated PDU-055 current declaration, so the single
  MEMORY unit is assessed as `DECLARED_STATE` (C1), CP-03 with CP-02 fields.
  Its 2026-05-01 "Open TBDs" list is treated as that day's history.
- **STATUS#remaining** is empty and pre-typed `NON_NORMATIVE`. The STATUS
  surface is `ALIGNED` at MEDIUM; the empty list is declared state only (A4).
- **OUT-001 (matrix).** CP-09 overtaken: the PASS parity hash `4d314c20...`
  differs from the frozen SOW `799c8855...`.

## Possible defects for R3 (not dispositions of keyed claims)

- `default_remediation(SparseSolverTbd)` still says "Complete profile-direct
  sparse assembly and governed default sparse-solver promotion before
  external performance reliance". The same diagnostic's message says DEC-053
  promoted sparse interactive use. The code contradicts itself and DEC-053.
- `core/solver/diagnostics/README.md` says default sparse promotion "remains
  TBD", and `core/solver/sparse_direct/README.md` says "the live product solve
  path still uses the dense verification interface". Both contradict DEC-053
  and the code.
- `DiagnosticProvenance::solver_generated` emits the runtime string
  `source_name: "OpenPipeStress solver diagnostics"`. That is an active code
  identifier carrying the former name (CP-04 residue per the R0 ruling
  addendum). The SOW does not name it, so no SURFACE row carries it.

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

## UNKNOWN rows

None.

## Did the reverse pass change my view?

No change to sealed rows. The inventory describes RC-04-0117 as diagnostics
"recording that sparse default promotion ... remain undecided", which is
consistent with FG-DEL-04-06-01. SOW-053's runtime detect-and-report
obligation reaches the product through `product_diag_from_solver_diag` and
fail-closed handling (RC-04-0049), answered `PARTIAL`. Singular detection
itself is frame-kernel code (RC-04-0209, `COVERS`).

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
