# DEL-09-03 Nonlinear support regression suite — worker notes (W3, PKG-09, worker G1)

Forward ledger `DEL-09-03_forward.csv`: 107 rows, all 70 required keys, plus:

- the `.rNN` rows of the split blocks CLM-003, 004, 007, 014 and 027;
- three `.sNN` sub-claims: `SOW.s01` and two sub-claims of the Architecture
  Basis Injection block.

CLM-012 is assessed directly and not split. The ledger was sealed before the
routing file was read.

## Path aliases

These are as for DEL-09-01.

- The parity records are at the repository root:
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-09-03/`.
- The owner's C-B selection is recorded at
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/OWNER_RULING_2026-08-11.md`.
  It is cited only in ContextRefs, as context.
- `SOFTWARE_DECOMP.md#L637` is DEC-046.

## Judgment calls

1. **CP-04 SURFACE row.** The SOW text names "OpenPipeStress" (CLM-010 and
   the CLM-012 table). The front-matter pin is `SOW.s01` (CP-02). Code residue
   has the same class: the crate name `open_pipe_stress_nonlinear_benchmarks`
   and the provenance strings.
2. **Convergence thresholds (FG-DEL-09-03-02).** DEC-046 (2026-06-20)
   created governed convergence records.
   - Seed-scope and multi-support records now carry accepted values.
   - The owner's C-B selection (2026-08-11) recorded a release-scope policy
     for the active-set count, with no release act.
   - Setup-era "TBD" rows are therefore STALE_SETUP_SPECIFICATION ·
     SCOPE_REDIRECTED_BY_RULING: CLM-014.r01 and OI-09-03-001. AC-001 was
     written after the migration, so it is STALE_REVIEW_OR_EVIDENCE with the
     same cause.
   - REQ-09-03-007 is ALIGNED. It asks that thresholds stay TBD until
     solver-maturity evidence exists, and they did.
3. **Removed `INIT.md` (CLM-007.r01).** `INIT.md` was removed on 2026-07-04
   (`9c4caf8fd`). The reference is a stale pointer: CP-02 ·
   BASIS_POINTER_STALE. REQ-09-03-003's source-basis column also cites it;
   the substance of that requirement holds.
4. **Unit requirements are ALIGNED here, unlike DEL-09-01 and DEL-09-02.**
   REQ-09-03-006 and CLM-004.r07 ask for "unit-aware and dimensionally
   checkable", not "checked". Explicit fixture-local mm-N-N-m labels and
   canonical dimension labels satisfy that wording. The difference from the
   PARTIALLY_IMPLEMENTED unit rows in DEL-09-01 and DEL-09-02 follows the
   text, not an inconsistency.
5. **CLM-027.r05 is PARTIALLY_IMPLEMENTED.** The row asks that iteration
   counts and tolerance basis be recorded "through the result envelope".
   - They are recorded in the crate's `assembled_convergence_observations`.
   - The runner regression report carries states, changed supports, the
     convergence flag, the residual and diagnostic codes, but no iteration
     count or policy reference.
   - The runner's regression binding covers only the five unit active-set
     fixtures, not the assembled inventory.
6. **Remaining R01.** It is DOCUMENTED_UNIMPLEMENTED · NOT_STARTED ·
   AuthorityNeeded REVIEW. Both PKG09-0903-PKG02 findings keep
   HumanDisposition TBD, and no SOW row carries the disposition act. This is
   the same treatment as DEL-09-01 R04.
7. **Four-document residue (CP-01).** This covers CLM-013 (setup acceptance
   criteria), CLM-015, CLM-020, CLM-021 and CLM-022. Of the later-pass checks
   in CLM-021, the human review gate is still pending.

## Canonical departures

None. All CS rows use the assigned values.

## Convention friction

The release-scope C-B record (`release_convergence_policy.dec046.c-b.json`)
sits in the DEL-09-03 crate folder. It came from the DEL-09-04 owner-gate
run, and no test or code references it. The reverse pass answers it as
PARTIAL (RC-09-0195).

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row. RC-09-0195's
inventory note confirms that no test or code reads the release-scope record.
That matches the forward reading: the record governs without a release act.

## Batch consistency

`--batch` over the three G1 ledgers returned PASS with 0 findings.

## Selectability

SelectableUnderCurrentLoop is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
