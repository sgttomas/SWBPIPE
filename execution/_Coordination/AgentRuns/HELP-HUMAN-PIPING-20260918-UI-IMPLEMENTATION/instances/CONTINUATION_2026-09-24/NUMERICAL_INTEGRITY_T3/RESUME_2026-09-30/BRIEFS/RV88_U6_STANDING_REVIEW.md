# RV88: standing independent reviewer for U6 (carriers and standing)

TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You review each U6 unit as it lands, keeping your context between units** (workflow §2). Fresh reviewers take the final U6f review and U7's switch.

**Read `BRIEFS/U6_FANOUT_COMMON.md`**, the plan `R/I66/u6_scoping_01/PLAN.md` and the U6 rulings. **You did not write this code. Don't rely on the authors' tests as your oracles.**

## The first unit: U6a at `844448112f`

The branch is `codex/piping-f2a-carriers-20261004`, against base NUM `7e4f5a51dd`. I66's account is `R/I66/u6a_slice_01/RETURN.md`.

1. **Existing identities unchanged.** Run your own sweep over existing fixtures and envelopes, comparing dispatch, metadata, standing, freshness, binding refusals and the `derive_document` bytes and validation against base. Also run the result_export suite and runner/headless, and PP's U1 and U3 pin tests built against the new result_export.
2. **The slice, by your own derivation.** The receipt survives `derive_document` and `validate_document` byte-equal and revalidates. Each row's disposition follows the reader's validated class. The 69 `absolute_verified` rows are disclosed with their bound (D-U6-2), and the rest keep their table disposition.
3. **The downgrade guards and every refusal code:** the relabelled successor in every form; receipt and disclosure mutations; a receipt on a base source.
4. **D-U6-1.** The Python public entry must agree with the draft on all 320 entries of the 07f corpus, with eligibility still gated by `_IMPLEMENTATION_COMPLETE`, which stays false.
5. **Findings F5 and F6:**
   - **F5:** a refused statement makes binding refuse every row with `RULE_QUANTITY_NOT_COVERED`. Is that the right fail-closed behaviour?
   - **F6:** is the new class-disclosure message truthful, claiming nothing beyond the receipt's bound?
6. **Mutants:** re-run I66's 62, and add at least five of your own.

## Host

- Your copy comes from `git archive` of the reviewed commit, in `WT/rv88/`, with targets `WT/targets/rv88/` and scratch `WT/scratch/rv88_u6/`. Delete the copies afterwards. Never write to the system temp directory.
- Other host rules are as in U6_FANOUT_COMMON.md.

## Output

The report goes to `NUM/R/REVIEW_RV88/u6a_01/REVIEW.md`. It contains a verdict, counts, findings (path:line, evidence, remedy) and SHA256SUMS. Time box: 2.5 h. End with the verdict, the counts, one line per finding, the sha256, and anything ROOT must rule on. **Later units come by message.**
