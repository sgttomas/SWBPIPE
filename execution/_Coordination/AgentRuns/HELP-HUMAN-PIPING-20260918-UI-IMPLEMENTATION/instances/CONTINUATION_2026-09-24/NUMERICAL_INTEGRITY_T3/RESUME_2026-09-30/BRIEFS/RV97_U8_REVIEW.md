# RV97: independent review of U8 (probe, PP witness tests, corpus 07l, reader alignment)

TASK (Type 2), an independent reviewer. Read `BRIEFS/U8_COMMON.md` first; its host rules bind you, but you write nothing in the U8 tree. You hold the review role RV93 held for U3 grant 2, the author of N-5: read `R/REVIEW_RV93/u3_grant2_01/` first.

**You wrote none of this work.** Build your own oracles for every claim that matters, and don't rely on the implementers' tests or tables.

## The candidate

The U8 branch head that ROOT gives you, against the U8 base head. The implementers' accounts are:
- `R/I68/u8_probe_01/` (PROBE.md);
- `R/I68/u8_witnesses_01/`;
- if L = 0 published: `R/I69/u8_corpus_07l_01/`, `R/I70/u8_rust_07l_01/` and `R/I71/u8_ts_07l_01/`.

## Review, in priority order

1. **The probe's facts, re-derived** on your own `git archive` copy in the registered build, both modes:
   - each fallback cause (Candidate for `first_load_only`, Preparation for `tiny_spring`, Native for W-C1's input) and its notice count and bytes;
   - W-C1's native reason;
   - the L = 0 outcome;
   - W-C2's per-case outcomes.
   
   Report any disagreement with PROBE.md.
2. **The witness tests are not vacuous.**
   - Each assertion fails under a mutant you build: wrong cause, two notices, plain bytes without the notice, a corrupted body-1 row, and a hook left armed.
   - The tests run from real inputs, with no fault hook armed (`hooks::armed_names()` empty).
3. **Nothing weakened, and nothing outside the fence.**
   - The full diff touches only the fenced test files, fixtures and corpus.
   - No production text in PP, readers or schemas changes, and no existing assertion is deleted or loosened.
   - The milestone still publishes `ac6986b0…` / `6cd1d249…`.
4. **If L = 0 published:**
   - the fixtures are byte-identical to the live successors (D-U6-5);
   - body 0 agrees with the milestone's independent reference within the unchanged criterion (U5's `u5_compare.py` with RV86's extract);
   - body 1's coverage rows are as PLAN §1.2 states;
   - **07l parity:** re-run every 07l mutation in all three readers and check each first gate and code, plus the must-pass entries. The provenance claim is truthful.
5. **Scope truth.** No record or comment claims a receipt Ceiling row (that is W-C2, B1's) or native Current evidence.

## Host

- **Your copy:** `WT/rv97/`, with targets `WT/targets/rv97/` and logs `WT/scratch/rv97_u8_01/`. Delete the copy and targets afterwards.
- **Cargo** one job at a time, coordinated with ROOT.

## Output

- **The report:** `NUM/R/REVIEW_RV97/u8_01/REVIEW.md` plus SHA256SUMS, with placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item.
- **After repairs,** you confirm them (same-reviewer confirmation).
- **Time box:** 4 hours.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
