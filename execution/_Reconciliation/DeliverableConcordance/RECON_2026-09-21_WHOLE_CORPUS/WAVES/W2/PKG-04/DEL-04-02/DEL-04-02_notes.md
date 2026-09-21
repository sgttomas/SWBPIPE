# DEL-04-02 — Straight pipe element: worker notes (W2, PKG-04)

Run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`, gate wave W2, worker G1
(TASK). Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`. Forward
ledger sealed before the routing file was read (B1). Agent judgments only;
nothing here is an owner ruling. Shared treatments are in
`../_WORKER_DEL-04-01_NOTES.md`.

## Path aliases

- Deliverable folder paths contain spaces; `ScopeOfWork.md`, `_STATUS.md`,
  `_CONTEXT.md`, `MEMORY.md` and `_run_records/*` are named in Notes only.
- Parity records are cited in their repository-root form
  (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`).
- The crate at `core/solver/straight_pipe` has the package name
  `open_pipe_stress_straight_pipe`; the SOW names only the path.

## Judgment calls

- **SURFACE row:** frontmatter pin (revision 0.8 at 69ac259a) under CP-02;
  CP-04 is not recorded (path only).
- **Resolved TBDs** (CLM-004 unit API, CLM-005 conditions, CLM-006, CLM-008
  open questions, CLM-013 benchmark cases, RQ-003 hook): STALE_SETUP_SPECIFICATION
  · DOC_BEHIND_CODE (F3: first present at 7bee9ae41).
- **Envelope binding (FG-DEL-04-02-01: CLM-014, CLM-025, AC-001):**
  STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE. The solver-to-result-envelope
  binding landed 2026-07-19 (commit 723c95b0f; runner
  `result_envelope_binding.rs`). It has bounded coverage: rows outside the
  vocabulary are disclosed, and a DEL-08-04 vocabulary follow-on is open. The
  texts date from 2026-07-12 and 2026-07-14.
- **Protected-content review (FG-DEL-04-02-02: CLM-021):** same treatment
  as DEL-04-01. The 2026-06-05 QA report is the last dedicated check.
  straight_pipe changed on 2026-09-05 (R02 station point-input validation),
  and no DEC-058 scan record exists.
- **RQ-004 weight hooks:** ALIGNED with `PRODUCT_CALLER: NONE` for
  `weight_hook`. Product self-weight generation computes its own mass per
  length.
- **RQ-007:** ALIGNED. It names no field list. Contrast DEL-04-01 REQ-011,
  which does and is PARTIALLY_IMPLEMENTED.
- **STATUS:** ALIGNED. Three July history entries sit above the 2026-04-30
  entries, out of order; the dates themselves agree.
  `STATUS#remaining` is empty and pre-typed, so it is NOT_ASSESSED.

## Canonical departures

None.

## Convention friction

As DEL-04-01: CP-09 applied to VER-001; the protected-review row uses the
CP-09 analogue.

## UNKNOWN rows

None.

## Reverse pass

- 7 CLAIMED_BY, 1 PARTIAL, 4 COVERS, 0 UNKEYED, 328 NOT_MINE.
- Did the reverse pass change my view of anything sealed? One possible
  defect, which I would correct. The routing inventory records that the
  station resultant sweep (`recover_station_resultants`,
  `recover_station_resultant_sweep`; RC-04-0002) and
  `StraightPipeBoundaryMetadata` (RC-04-0015) have no product_physics caller.
  The product does its own station recovery. The sealed ALIGNED row
  `SOW#CLM-010` cites the crate and says "product-called for stiffness and
  recovery", without `PRODUCT_CALLER: NONE` for those two surfaces. Under F7
  the claim is about the engine slice, so ALIGNED still fits. The row should
  carry `PRODUCT_CALLER: NONE` for the station and metadata surfaces. A fresh
  worker or the verifier can add it; I did not edit the sealed row.

## Batch consistency

`--batch` over the three sealed forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
