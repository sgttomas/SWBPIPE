# RV99: independent review of S-I1 (option C's interval evaluator)

TASK (Type 2), an independent reviewer, fresh and not an F2a reviewer. You wrote none of this code. **Build your own oracles;** don't rely on I73's tests.

## The candidate

- **The diff:** the S-I1 branch head that ROOT gives you, against its main base.
- **The account:** `R/I73/s_i1_01/RETURN.md`.
- **The design:** D2 §4.11 (`T3/DESIGN_STANDING/DESIGN.md`).
- **Readiness:** `R/I61/u8_plan_01/PLAN.md` §3.

## Review, in priority order

1. **Soundness.**
   - For every operator and function in the rule formula grammar, check the interval extension is enclosing and outward-rounded.
   - Build your own straddle cases at each comparison boundary. **No straddling result may pass or fail;** each must be indeterminate.
   - Check I73's soundness table rule by rule.
2. **Point mode unchanged.** Over the committed rule packs and run fixtures, `RuleCheckRunResult` must be byte-identical to main without interval inputs. Run your own differential.
3. **Parity.** Rust and Python agree on every case in the case file and on at least 30 cases of your own, including random intervals around boundaries.
4. **Mutants.** Rerun I73's mutants, and add at least four of your own. Report survivors.
5. **Fence and contracts.**
   - Nothing outside the fence changed.
   - No schema changed, and no dependency or lock moved, src-tauri's included.
   - The new codes and outcomes match D2's wording.

## Host

- **Your copy:** `WT/rv99/`, with targets `WT/targets/rv99/` and logs `WT/scratch/rv99_s_i1_01/`. Delete the copy and targets afterwards.
- **Cargo** one job at a time, coordinated through ROOT. No Git writes or installs.

## Output

- **The report:** `NUM/R/REVIEW_RV99/s_i1_01/REVIEW.md` plus SHA256SUMS. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item.
- **After repairs,** you confirm them.
- **Time box:** 4–6 hours.
- **End your turn** with the verdict, the counts, the sha256 and anything ROOT must rule on.
