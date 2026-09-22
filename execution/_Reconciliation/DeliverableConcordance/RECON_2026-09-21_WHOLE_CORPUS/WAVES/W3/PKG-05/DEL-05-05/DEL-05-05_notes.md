# DEL-05-05 Concentrated and distributed user load application — worker notes (W3, PKG-05, worker G2)

Forward ledger `DEL-05-05_forward.csv`: 86 rows. That is 55 required keys,
28 optional `.rNN` keys (the CLM-003, CLM-010 and CLM-012 blocks are split,
all-or-none) and 3 sub-claims. Sealed at SHA-256
`d6ec78c0ebbda28ab283b0a7909bdf26cbf77afadb200d0716cbe1de139a2dd7`.
Reverse file `DEL-05-05_reverse.csv`: 419 capabilities (3 CLAIMED_BY,
3 PARTIAL, 3 COVERS, 410 NOT_MINE).

## Path aliases

- The crate `core/loads/user_loads` is the Cargo package
  `open_pipe_stress_user_loads`.
- Its only non-test dependant is `validation/benchmarks/mechanics`.
- Hand-calculation witnesses are under `validation/hand_calcs/mechanics/`
  (`tp_phys_004/005/006/007/009_*.md`).
- The package-level R11 run record is
  `projects/chirality-piping/execution/PKG-05_…/1_Working/_run_records/WORKING_ITEMS_RUN_2026-09-05_PHYSICS_AUDIT_REPAIRS.md`.
- Deliverable-local `Datasheet.md`, `Specification.md`, `Guidance.md` and
  `Procedure.md` are absent at the freeze.

## Judgment calls

1. **F7: the crate has no product caller.** No app or product crate depends
   on `user_loads`. Every ALIGNED row whose subject is the crate carries
   `PRODUCT_CALLER: NONE`. The product authors and solves the same three load
   kinds through another path:
   - the Load Cases manager and operation seam
     (`concentrated_force`, `concentrated_moment`, `distributed_force`);
   - `product_physics` via `primitive_loads` and the `StraightPipeElement`
     helpers.

   R1 (support the categories as explicit inputs) is ALIGNED on both paths.
2. **FG-DEL-05-05-01: product integration landed outside the crate and the
   SOW does not record it (5 rows).** The rows are STALE_REVIEW_OR_EVIDENCE ·
   DOC_BEHIND_CODE · LOCAL_DESIGN · RECORD · NO: CLM-003.r02, CLM-003.r08,
   CLM-013, CLM-024 and CLM-026.
   - What the SOW says (last edited 2026-07-16): the crate is the
     implementation surface, and GUI and result integration are TBD.
   - What exists: GUI creation editors (2026-06-11, SMOKE TP-MAC-98/99/100,
     attributed to DEL-05-05) and the R11 product repair (2026-09-05,
     attributed to "OUT-001/R7 product-integration").
   - Tolerance policy was also structured by DEC-026.
   - Whether the unconsumed crate should remain a separate engine, or be
     retired or wired in, is an ownership question for R3. I did not treat it
     as a defect.
3. **CLM-003.r06 (result posture) is ALIGNED at MEDIUM confidence.** The TBD
   there is literally true for the crate's recovery hooks, which no result
   envelope consumes.
4. **Validation evidence.** R7-R10 and the matching verification rows cite
   agent-produced hand-calculation witnesses as ValidationEvidence, marked
   UNVERIFIED (no human disposition). The claims assert implemented
   behaviour, not validated reliance, so I did not use
   VERIFIED_NOT_VALIDATED. A verifier who reads R7-R10 as
   engineering-correctness claims may prefer VERIFIED_NOT_VALIDATED ·
   VALIDATION_GAP · INVARIANT · VALIDATION.
5. **CLM-012.r02 (no-default and no-compliance verification) is ALIGNED.**
   - The no-default half is tested: missing quantity blocks application, and
     the total is deferred until the length is known.
   - The no-compliance half rests on the boundary text and on the crate
     having no status or claim output. No test asserts the absence of
     compliance language.
6. **Observation for the verifier (not a disposition).** The product applies
   authored concentrated force and moment under the preview category
   "occasional", and distributed force under "weight".
   - This is disclosed by the warning `LOAD_CATEGORY_PREVIEW_MAPPED`
     (`product_physics` `authored_category_preview_mapping`).
   - It creates no code combination, but it is a software-chosen category.
     Its owner is product physics or DEL-05-01, not this crate.
7. **Rename residue (SOW.s01, CP-04 default variant).** The Cargo package
   name `open_pipe_stress_user_loads` is an active code identifier the SOW
   names.
8. **F3 origin tests used.**
   - CLM-019 "Four deliverable documents exist": `225547c38` (2026-06-04), so
     STALE_REVIEW_OR_EVIDENCE.
   - CLM-020 record list: `7bee9ae41`, so STALE_SETUP_SPECIFICATION.
   - CLM-003.r08 combined TBD phrase: after the migration, so
     STALE_REVIEW_OR_EVIDENCE. Only the axial-provenance fragment existed in
     `MEMORY.md` at `7bee9ae41`.
9. **Architecture basis injection.** `.s01` and `.s02` are treated exactly as
   in DEL-05-04.
10. **D-41 declarations.** CLM-007, CLM-014 and CLM-021 (CP-03) are ALIGNED,
    because the crate evidence supports them. The "other breadth residual"
    (other element families and distribution shapes) is outside this SOW's
    requirements.

## Canonical departures

None.

## Convention friction

- F7 is awkward where the SOW's requirement source is product-level (SOW-052)
  but the SOW binds the deliverable to an unconsumed crate. I disposed R1 on
  both paths, and kept the crate-only rows ALIGNED with the marker.
- The empty `STATUS#remaining` heading is pre-typed NON_NORMATIVE. It stays
  NOT_ASSESSED even though FG-DEL-05-05-01 records open documentation drift.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row.
- Capability 0272 (product conversion of authored forces, moments and
  full/partial uniform loads) confirms the FG-DEL-05-05-01 path.
- PARTIAL answers name that product slice (0063, 0272) and the crate README
  (0213).
- F5: every NOT_MINE capability whose EntryPoints hit a cited path
  (product_physics, straight_pipe, primitive_loads, LoadCaseManagerPanel,
  model and results schemas, CONTRACT, registers) has a capability-specific
  reason.

## Batch consistency

`--batch` over both forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Agent
dispositions here are not owner rulings.
