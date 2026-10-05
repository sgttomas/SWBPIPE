# I61: plan U8, the F2a-breadth roadmap, and S-I1's readiness

TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path. You do not delegate. **Planning only:** no source edits, no Git writes, and no cargo, solver or native jobs. You may read source, records and Git history.

## Basis

- **#1082 merged** the D1 milestone to main at `0b00b8e8b6` (parents M `5fdc5ab601` and F′ `5488136a19`). The merge record is `T3/IMPLEMENTATION/F2A_D1_MERGE/` (RECORD.md, ERRATA.md).
- **The owner's sequencing decision:** RR "Owner decision: F2a's delivery steps move to the end of F2a; erratum E-1". The order is:
  1. U8;
  2. F2a's numerical breadth, on the registered dev/test build;
  3. the release identity, registered once;
  4. public activation with native Current;
  5. S-I2, F2b per family, and F3.
  
  S-I1 may run alongside.
- **U8 as defined:**
  - `R/I61/step4_plan_01/PLAN.md` §U8;
  - RR "Snapshot 05b verified; … producer-solved witnesses deferred" (the native Ceiling row and the L = 0 base need real receipts);
  - RR:8884 (D-9: the Ceiling witness may revisit the caps);
  - RR:11067 (RV93 N-5, a real-input Candidate test, deferred to U8).
- **The open F2a obligations:** CHANGE_RECORD §5 in the merged package.
- **The design:** `T3/DESIGN_NUMERICS/DESIGN.md` §4.3–§4.4.1 and §6, with D2 (`T3/DESIGN_STANDING/DESIGN.md`) for S-I.
- **The re-qualification rules:** QUALIFICATION §11 (in the package's `copies/QUALIFICATION.md`); RR:10407 and :10474; RV95 N-6.
- **The process rule** from RR "DEC-025 on F finds…": the full 40-manifest suite runs on a candidate before any freeze.

## Deliverable: `NUM/R/I61/u8_plan_01/PLAN.md` plus SHA256SUMS (placeholder paths only)

1. **U8, concretely.** For each witness (the native Ceiling row, the L = 0 base, RV93 N-5):
   - what it proves;
   - which real case or input produces it;
   - whether that case lies inside D1's admission domain and caps. If it does not, say what a cap or domain change costs: the G7-style re-qualification, Pass B, and the review;
   - which producer path emits its receipt;
   - what changes in the reader corpus (the next snapshot) and in all three readers;
   - its tests and controls.
   
   Then give the slices, the write sets, the TASK and reviewer allocation, the order, and an estimate. Flag anything that needs a ROOT ruling or an owner decision, and include a recommendation.
2. **The F2a-breadth roadmap:** the units between U8 and the release registration.
   - Order the CHANGE_RECORD §5.2 obligations into units with dependencies.
   - For each unit, say whether it changes the D1 call graph (and so needs re-qualification), widens the admission domain, or changes the contract or readers.
   - Give rough estimates.
   - Propose where the release-identity registration and activation sit at the end. Name what the T6 conflict blocks (checklist item 4), and what the owner would have to decide.
3. **S-I1's readiness.** From D2's plan:
   - S-I1's write set and dependencies;
   - whether it can start now in parallel without touching F2a's files;
   - its review needs and an estimate.
4. **Main's movement.** Main will move as the owner releases held PRs. State how NUM should absorb main before U8's implementation: the merge-base and any expected conflicts in S files.

## Budget and return

3 hours. Return once, with the plan's sha256, the decisions needed (yours numbered, each with a recommendation), and the estimate summary.
