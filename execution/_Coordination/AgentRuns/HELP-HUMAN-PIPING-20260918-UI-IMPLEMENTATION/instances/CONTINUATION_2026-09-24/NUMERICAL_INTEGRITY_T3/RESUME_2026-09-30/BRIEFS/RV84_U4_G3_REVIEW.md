# RV84: independent re-derivation review of U4 grant 3 (producer composition at the caps)

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path, and you do not delegate. **You did not write this packet. Re-derive; do not re-read.** The author's scripts and outputs are leads, not oracles.

## The candidate

- **The packet:** `R/I65/u4_g3_01/`, committed at NUM. Your dispatch prompt gives the revision. It holds RETURN.md, ORDINARY.md, RESIDUALS_G3.md, TEXT.md, COMPOSITION.md, STACK_INVENTORY.md, G2_AMENDMENTS.md, `_run_records/` and SHA256SUMS (56 files).
- **The basis:**
  - I65's plan `R/I65/u4_plan_01/PLAN.md` (§3 terms, §5–§7);
  - the G2 packet `R/I65/u4_g2_01/`;
  - RV83's G2 review `R/REVIEW_RV83/u4_g2_01/REVIEW.md`;
  - the rulings in `T3/ROOT_RULINGS_V1.md` from "U4 plan: decisions, and two questions for the owner" through "U4 G3 verified; D1.10 and D1.11 adopted; the margin rule".
- **Division of labour:** RV83 is separately confirming that its own G2 findings are repaired. You review G3's derivations and composition as a whole. Where you touch the same term (T07, T22, T25, text), re-derive the numbers. Don't merely check that a repair exists.

## Review, in priority order

1. **The composition and the fit (COMPOSITION.md).**
   - The branch structure: X (exact-block selected) and W (W1 runs).
   - Whether `admit` must cover `max(E_X, E_W + G4)`.
   - Whether any phase is double counted or omitted, and whether the moving/requested treatment (T21) is sound.
   - Re-derive the four headline figures with your own stdlib Python, and state which rest on ASSUMED strides.
2. **T25, selected source-blocks finalization.** Re-derive its owners at the caps from source, field by field where I65 used a reading, including `hash(publication)` and the ε expansion factor (6, or 2 under D1.11). Check that D1.11 really caps the expansion at 2 in serde_json and canonical_json.
3. **Text (TEXT.md, T08).**
   - Is the call graph complete for D1? I65 found and fixed a defect where multi-segment path calls were dropped. Sample at least 40 reachable functions, and at least 15 claimed exclusions, against source yourself.
   - Look for remaining blind spots: trait-method dispatch, closures, macros, and `impl Display` invoked through `{}`.
   - Re-derive the multiplicities and the top ten contributors.
   - Check D1.10's argument: that no D1 input reaches the self-weight validation module, and that the per-load parse attempt is priced.
4. **T05 and the O-N row.** Check the monotonicity lemmas and the six milestone sub-expressions against I54.
5. **T07 and T22 as repaired** (re-derive; RV83 checks that its findings are addressed).
6. **T11–T15 and T21.** Re-derive at the caps, including U1's `OrdinarySeed` and the grant-2 delta.
7. **STACK_INVENTORY.md.** Check the recursion inventory, the absence of mutual recursion, the thread-local finding and the R/k confirmation.
8. **COMPOSITION §4,** the `LateFacts` and `CompleteFacts` fields and bounds. Are they sufficient for G-B and G-C, and readable allocation-free?

## Host and method

- **Records review only.** Use stdlib Python under `NUM/R/REVIEW_RV84/u4_g3_01/_run_records/`, and scratch in `WT/scratch/rv84_u4_g3_01/`. Nothing goes to the system temp directory. Reading the installed std and cached dependency sources is allowed.
- **Never:** Cargo builds, Git writes, index operations, installs, new tooling, or solver, DEC-025 or native jobs. Git reads use `GIT_OPTIONAL_LOCKS=0`.
- **The memory guard** must be running. Other TASKs are working: I61 on U3 in `WT/f2a-facade`, RV83 on its G2 confirmation, and I65 on G4. Don't touch their files.

## Output

- **The report:** `NUM/R/REVIEW_RV84/u4_g3_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per review item.
  
  Include a SHA256SUMS. Use placeholder paths only.
- **Time box:** 4 hours from your first tool call. Report anything unfinished.
- **End your turn** with a concise status: the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
