# RV87: independent re-derivation review of U4 grant 4 (publication, precommit, transfer, completion; the composed admission maximum)

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path, and you do not delegate. **You did not write this packet. Re-derive; do not re-read.** The author's scripts and outputs are leads, not oracles.

## The candidate

- **The packet:** `R/I65/u4_g4_01/`, including `ADDENDUM_L128.md` and its `.l128` outputs, at the NUM revision in your dispatch prompt. Start with RETURN.md, then COMPOSITION_G4.md, PUBLICATION_READER.md, TRANSFER_COMPLETION.md, API_G4.md, NOTES_G4.md and the addendum.
- **The basis:**
  - I65's plan `R/I65/u4_plan_01/PLAN.md` (T16–T19, T24);
  - the G2 and G3 packets;
  - in `T3/ROOT_RULINGS_V1.md`: the U4 rulings from "U4 plan: decisions…" through "U4 G4: the margin rule trips; l ≤ 128 adopted…", including the restated admission law ("RV84 on U4 G3…", S-5) and the margin rule (≤ 0.9 M at illustrative strides).
- **The code the new terms price,** at NUM: U1 (`PP/src/retained_wire.rs`), U3 (`PP/src/lib.rs`'s permitted dispatch, `retained_w1`, the notice reservation and the transfer; `retained_product.rs`'s frozen split and staging), and the precommit reader `P/core/reporting/result_export/src/retained_precision.rs`.
- **Division of labour:** RV83 is confirming its R-items and RV84 its S-items and N-3, all on the same packet. **You own the new terms and the composed maximum.** Where you touch a repaired item, re-derive its number; don't re-audit its repair.

## Review, in priority order

1. **The composed admission maximum under `l ≤ 128`.**
   - Re-derive, with your own stdlib Python, every phase of both branches through caller completion (X1–X*, W1–W5) in both modes.
   - Confirm the maximum is ≤ 0.9 M at illustrative strides, and state its margin.
   - State which figures rest on ASSUMED strides, and how sensitive the maximum is to a 10% stride error.
2. **T16, publication** (`retained_wire.rs`). Re-derive from source at the caps: the staged envelope, the `to_value` tree, canonical rendering with the per-string temporaries and ε = 2 under D1.11, the hashes, and old/new output growth. Check that nothing is double counted against T25's shared route.
3. **T17, the precommit reader.**
   - Re-derive its peak over the receipt at the caps from the reader's actual code: the invocation `Value` deep copy, the clones, collects, formats and statics.
   - Check that the 13 `include_str!` statics are counted once and bound by the D-6 extension.
4. **T18 and T19.**
   - The staged copy (D-b), the N1 reserve, and the transfer's move-only claim, checked by reading U3's code.
   - Direct completion and s(output), including the inline admission report.
5. **The text work's new parts** (U1's own text; anything G4 added to TAV_X and TAV_W). Spot-check at least 20 sites against source.
6. **API_G4.md.** Are the per-gate facts sufficient and readable without allocation? Are the S-6 hook signatures implementable by I61 within D-5?
7. **NOTES_G4.md §5, the G5 carry list.** Is it complete, judged against every routed item in the rulings since G2?

## Host and method

- **Records review only.** Use stdlib Python under `NUM/R/REVIEW_RV87/u4_g4_01/_run_records/`, with scratch in `WT/scratch/rv87_u4_g4_01/`. Nothing goes to the system temp directory. Reading installed std and dependency sources is allowed.
- **Never:** Cargo builds, Git writes, index operations, installs, new tooling, or solver, DEC-025 or native jobs. Git reads use `GIT_OPTIONAL_LOCKS=0`.
- **The memory guard** must be running. Other TASKs are working: RV83, RV84, I66 in `WT/f2a-carriers`, and possibly I61 and I65. Don't touch their files.

## Output

- **The report:** `NUM/R/REVIEW_RV87/u4_g4_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per item.
  
  Include a SHA256SUMS. Use placeholder paths only.
- **Time box:** 4 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
