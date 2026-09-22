# DEL-09-02 Stress recovery benchmark suite — worker notes (W3, PKG-09, worker G1)

Forward ledger `DEL-09-02_forward.csv`: 112 rows, all 69 required keys, plus:

- the `.rNN` rows of the split blocks CLM-004, 005, 008, 014, 023, 029 and
  030;
- four `.sNN` sub-claims:
  - `SOW.s01` (front-matter pin);
  - `RQ-001.s01` (four-document verification hook);
  - two sub-claims of the Architecture Basis Injection block.

CLM-003, CLM-016 and CLM-032 are not split; no row in them diverges. The
ledger was sealed before the routing file was read.

## Path aliases

These are as for DEL-09-01 (see `../DEL-09-01/DEL-09-01_notes.md`).

- The parity records are at the repository root, under
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-09-02/`.
- `LIB#L35` is the crate's `INTERNAL_ASSERTION_EPSILON`.
- `SOFTWARE_DECOMP.md#L616` is DEC-025, and `#L618` is DEC-027.

## Judgment calls

1. **The CP-04 SURFACE row.** The SOW names the active identifier
   `open_pipe_stress_result_export` (CLM-008, PDU-039 paragraph), so CP-04
   (default variant, AuthorityNeeded OWNER) sits on the SURFACE row. The
   front-matter pin is `SOW.s01` (CP-02). Code residue has the same class:
   the crate name `open_pipe_stress_stress_benchmarks` and the provenance
   strings.
2. **STATUS SURFACE (CP-05).** Last Updated reads 2026-07-12, but the History
   has an entry dated 2026-07-16 (the DEC-081 alignment).
3. **Remaining R01 (CP-07, MEDIUM).** R01 lists "canonical conversion" as
   TBD, but DEC-018 ruled it on 2026-06-10. The CI-gate item is partly ruled
   by DEC-025 (the evidence-gate posture). The other holds are accurate. This
   is the same unit-catalog drift as DEL-09-01 R02, with the same fields.
4. **Protected-content review (FG-DEL-09-02-02).** CLM-005.r07 and
   CLM-014.r04 are UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA;RECORD,
   with AuthorityNeeded REVIEW.
   - The only screening record is the 2026-06-06 agent SELF_CHECK
     (`_REVIEW.md` AC-003).
   - Two fixtures came later, on 2026-07-10:
     STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS and
     STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS. For them only in-code
     self-certification was found. Their DEL-03-08 run record shows no review.
   - The exclusion requirements (RQ-003, CLM-004.r04) are judged ALIGNED
     (MEDIUM) on the fixture code. I inspected both later fixtures' constants:
     they are invented, and the absolute mill tolerance is user-entered, with
     no catalog value or default. This follows the W1 DEL-07-02 CLM-026
     precedent.
5. **Unit rows (FG-DEL-09-02-01).** CLM-004.r07, RQ-005 and CLM-014.r03 are
   PARTIALLY_IMPLEMENTED · PROJECT_BASELINE · BASELINE.
   - The fixture-local N-m-Pa labels are explicit.
   - Comparisons are bare f64 values with no dimensional check and no binding
     to the DEC-018 project basis.
   - This uses the same fields as the DEL-09-01 unit rows.
6. **Tolerance rows (CLM-004.r05, RQ-007, CLM-014.r05) are ALIGNED.** The
   suite compares against an absolute 1e-9 epsilon. That is tighter than the
   DEC-026 relative seed at stress magnitudes, so tighten-only holds. Every
   `tolerance_policy` slot is unresolved.
7. **CLM-008 questions.**
   - r01 (accepting authority) is STALE_SETUP_SPECIFICATION ·
     SCOPE_REDIRECTED_BY_RULING. DEC-027 made the owner the maintainer and
     release authority, and DEC-026 is the tolerance route.
   - r02 and r05 are overtaken by code.
   - r03 and r04 are still open, so they are ALIGNED.
   - The block itself is assessed directly for its PDU-039 paragraph, which is
     accurate.
8. **CLM-023.r05** is ALIGNED. The 2026-06-16 run record shows the schema
   validator returned VALID, and `Dependencies.csv` is unchanged since
   `28219696d`. The check was not rerun.
9. **PRD §22.3 content.** PRD v0.4 §22.3 also lists SIF-multiplier and
   flexibility-factor behaviour. The DEL-09-02 SOW does not claim these, so
   no row was added (A1). Noted for R3.

## Canonical departures

None. All CS rows use the assigned values.

## Convention friction

The formal witness pilot and its validator tooling were authored in DEL-09-02
tranches (TP-WITNESS-023B, TP-VERIFY-017), but no issued key names them. The
reverse pass answers both as UNKEYED, with CLM-015 as the nearest key.

## UNKNOWN rows: smallest next check

- **CLM-005.r07 and CLM-014.r04.** Locate a protected-content review record,
  agent or human, that covers the two TP-PMM-P3 stress fixtures added on
  2026-07-10. The first place to look is the TP-PMM-P3 tranche review
  returns. If none exists, run the review and record it.

## Reverse pass

- The reverse pass did not change my view of any sealed row.
- New context: RC-09-0073 (witness pilot) and RC-09-0117 (witness tooling)
  are UNKEYED here. R3 may place them with the validation-manual deliverable.
- RC-09-0109 (the physics-audit crate) is PARTIAL, following its R10 run
  record's mapping.

## Batch consistency

`--batch` over the three G1 ledgers returned PASS with 0 findings.

## Selectability

SelectableUnderCurrentLoop is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
