# DEL-04-04 — notes (W2, PKG-04)

TASK worker, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`, gate wave W2.
Forward ledger sealed: `DEL-04-04_forward.csv`, SHA-256 `cc63802abc938868694bc586c5534434f99cdf53ffd99754d3d9716ab58692c3`,
72 rows (70 required keys plus 2 CS-04 `.sNN` sub-claims). Reverse:
`DEL-04-04_reverse.csv`, 340 answers. Evidence read only from the frozen
checkout `00115c719`. Cross-deliverable conventions are in
`../_WORKER_DEL-04-04_NOTES.md`.

## Path aliases

- `NS` = `core/solver/nonlinear_supports/src/lib.rs` (this deliverable's
  crate, package `open_pipe_stress_nonlinear_supports`).
- `NI` = `core/solver/nonlinear_integration/src/lib.rs` (DEC-044 integration
  tranche that owns the assembled loop).
- `PP` = `core/product_physics/src/lib.rs` (product caller of both crates;
  the desktop Tauri app and the headless runner depend on it).
- `HB` = `core/runner/headless/src/result_envelope_binding.rs`.
- `NB` = `validation/benchmarks/nonlinear/src/lib.rs`.
- Test cases are cited as `<file>::<fn>`; per-test pass status comes only
  from the B4_4 sweep log (`GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`,
  raw log `sweep.log.gz`), always "not rerun".
- The four documents (Datasheet, Specification, Guidance, Procedure) were
  folded into `ScopeOfWork.md` by PR #229 (`fcb21c717`).

## Named R0 repairs carried

- OUT-001 (matrix) cites the CHANGE-P1-PKG04 parity report and claim map;
  their production hash equals the frozen SOW, so it is `ALIGNED` (CP-09).
- REQ-08 cites the `product_physics` passthrough test
  `assembled_loop_context_is_a_pure_passthrough` as partial verification;
  cause `VERIFICATION_REMOVED` (PR #787, commit `b43cc00c4`, removed the
  R14 envelope-binding tests), `PROTECTED_CHECK`, `INVARIANT`.
- Reverse: the old CAP-SOLVER-020 behaviour (visible nonconvergence plus
  loop assumptions/limitations) is RC-04-0298, answered `PARTIAL`.

## Judgment calls

- **Origin test (F3).** Checked with `git grep` at `7bee9ae41` and
  `git log -S` on the frozen history. Setup text (CLM-005 friction row,
  CLM-013, CLM-015 to CLM-019, CLM-021, CLM-023) is present at the initial
  migration → `STALE_SETUP_SPECIFICATION`. REQ-09 and CLM-003's deferrals row
  date from 2026-06-21, AC-001 from 2026-07-14 → `STALE_REVIEW_OR_EVIDENCE`.
- **Sparse live path (FG-DEL-04-04-01).** CLM-003, REQ-09 and AC-001 still
  call sparse live-path adoption unresolved although DEC-050 bound the lane
  and DEC-053 made sparse interactive the default. `LOCAL_DESIGN` (catch-up,
  no decision).
- **Unit metadata (FG-DEL-04-04-02).** CLM-004 and REQ-06:
  `NonlinearSupportUnitMetadata` exists, only the accepted-dimension case is
  tested, and no caller outside the crate builds it; classifier fields stay
  raw `f64`. `PARTIALLY_IMPLEMENTED`, `INVARIANT` (OPS-K-UNIT-1), `REVIEW`.
- **REQ-01.** The frame-only boundary holds by structure, but the named
  solver-boundary test was not located (F1) → `PARTIALLY_IMPLEMENTED`,
  MEDIUM.
- **REQ-03 is `ALIGNED`.** After PR #787 the canonical 0.2 export carries the
  nonlinear count, flag, state-code and residual rows as row disclosures, and
  the export contract fixture and tests assert that. The headless producer
  test asserts the nonconvergence diagnostic. Closing typed vocabulary is a
  separate DEL-08-04 action (R07).
- **Remaining (F2).** R01 is `ALIGNED` with `OPEN_ACTION` REQ-06. R02 to R07
  describe open actions that no governing row in this ledger carries, so each
  takes the gap disposition: R02 `DOCUMENTED_UNIMPLEMENTED` (DEC-067 names a
  future D-XX); R03 and R06 `INVARIANT · VALIDATION` (model qualification,
  external validation); R04 and R05 `DEFERRED_BY_RULING` (DEC-052); R07
  `PARTIALLY_IMPLEMENTED`, `OWNER` (closing it changes the result-semantics
  contract).
- **Setup procedure blocks.** The D-41 "setup-era statements retained as
  historical where applicable" clause (CLM-007/CLM-027) does not change
  sibling rows (C1), so CLM-015 to CLM-017 are judged as current text. R0 had
  CLM-016 `ALIGNED` as history; this ledger marks it stale because "keep all
  writes inside the DEL-04-04 folder" contradicts the implemented crates.
- **CLM-011 and CLM-026** are `ALIGNED` at MEDIUM. CLM-011 concerns the SOW
  text, read in full; no protected-content scan was run. CLM-026 lists no
  source conflict, and none awaiting a ruling was found.
- **F7.** Product physics calls the classifier, so classifier rows need no
  `PRODUCT_CALLER` marker.

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

No change to sealed rows. Observations:

- The assembled loop (RC-04-0149), the bounded friction force (RC-04-0157),
  contact initialization (RC-04-0063) and loop convergence/context
  publication (RC-04-0298) sit in `nonlinear_integration`. DEC-044 places the
  loop outside DEL-04-04, but no issued key owns that tranche. I answered
  `PARTIAL` (R0 repair), and R3 should treat the tranche as an ownership gap.
- Product wiring (RC-04-0285), the loop-context passthrough (RC-04-0150), the
  export builder (RC-04-0080), the regression runner (RC-04-0180), and the
  nonlinear fixtures, policies and hand calculations (RC-04-0144, 0132, 0232)
  are `COVERS`: they carry DEL-04-04's reporting and verification obligations
  without being its code.

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
