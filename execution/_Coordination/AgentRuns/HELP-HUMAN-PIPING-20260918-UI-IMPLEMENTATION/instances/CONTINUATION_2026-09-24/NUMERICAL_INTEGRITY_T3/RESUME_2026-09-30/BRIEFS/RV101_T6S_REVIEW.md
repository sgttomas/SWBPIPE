# RV101: independent review of the T6 successor-output slice

TASK (Type 2), a fresh independent reviewer. You wrote none of this work, and you are not an F2a or reader reviewer of record. Read `BRIEFS/T6S_COMMON.md` first; its host rules bind you, but you write nothing in the slice's tree. **Build your own oracles;** don't rely on the implementers' tests.

## The candidate

- **The diff:** the slice branch head that ROOT gives you, against main `c1bfc460fc`.
- **The accounts:** `R/I75/t6s_01/` and `R/I76/t6s_01/`.
- **The plan and the rulings:** I74's PLAN and RR "I73's checkpoint 1 and I74's plan ruled; …".

## Review, in priority order

1. **Closure.** No successor can be exported except by the two panels, and only at `numerically_eligible` standing with the live native capture.
   - Check the 18 other surfaces, the report package and the Rule-check panel.
   - The output policy is exhaustive: a new route fails `tsc`.
   - **Without a product caller,** a successor loaded from disk or copied cannot reach eligible standing (decision 3).
2. **Contract faithfulness.**
   - The TS result document equals Rust `derive_document`'s, byte for byte. Re-derive the goldens yourself.
   - Every `absolute_verified` and `not_covered` row is disclosed, never valued or unlabelled (D2 §4.9.9).
   - No producer-origin claim, no standing token and no invocation appear in an export.
   - b is formatted as Rust's `{:e}`.
   - Stress-neutral S-d: the withheld-witness codes, their severity and D-U6-2 text, the receipt whole, the transport header, and schema validity with no schema change.
3. **Nothing weakened and nothing outside the fence.**
   - The full diff touches only the files T6S_COMMON names.
   - No refusal assertion was loosened beyond the two panels' deliberate admission and the reworded text.
   - Every committed non-successor fixture exports byte-identically to the base through both builders. Run your own differential.
4. **The dispatcher.** The `$ref` is correct. The equivalence test covers every committed v0.3 document. Build your own documents that the version file refuses, and check that the dispatcher refuses them too.
5. **N-5.** Check the masking-layer test, and that S1's public-API equivalence is argued correctly. Confirm nothing in `RE/src/` changed.
6. **Mutants.** Rerun the implementers' mutants and add at least four of your own. Report survivors.

## Host

- **Your copy:** `WT/rv101/` (a `git archive` of the head), with targets `WT/targets/rv101/` and logs `WT/scratch/rv101_t6s_01/`. Delete the copy and targets afterwards.
- **Cargo** only through `WT/tools/t3_cargo.sh`.
- No Git writes or installs.

## Output

- **The report:** `NUM/R/REVIEW_RV101/t6s_01/REVIEW.md` plus SHA256SUMS. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item.
- **After repairs,** you confirm them.
- **Time box:** 6–9 hours.
- **End your turn** with the verdict, the counts, the sha256 and anything ROOT must rule on.
