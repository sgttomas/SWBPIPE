# RV98: independent confirmation of I72's Pass B on the U8 candidate

TASK (Type 2), an independent reviewer. Read `BRIEFS/U8_COMMON.md` first. You hold the confirmation role RV89 held for Pass B. Read these first:
- `R/REVIEW_RV89/u4_g7_03/` (REVIEW, ADDENDUM_01 and ADDENDUM_02);
- I65's tools in `R/I65/u4_g7_06/_run_records/` and `R/I65/u9_refreeze_01/`.

## Confirm or refute

1. **The tool.** I72 ran either the fail-closed `g7_pass.sh` unchanged, or a no-build copy with RV89 N-1's restorations:
   - the verdict and exit code;
   - the early stops;
   - `text_summary`;
   - the delta-tool failure handling;
   - an accurate header.
   
   Diff it against I65's version. No gate may be weakened.
2. **The gate outputs.** Rerun the gate checks on I72's recorded outputs and compare them with `u4_g7_06` (on F) and `u9_refreeze_01` (on F′). Explain every difference.
3. **Your own delta inventory** from Pass A `ba1faa1c…` to the U8 head. Every added row must be test-class; any production-class row is BLOCKING. All 11 reviewed entries must match.
4. **The build-gate argument,** if I72 ran no build. It holds only if no PP or reader production file, `Cargo.lock` or reviewed input changed. If you need a registered build to settle it, say so; ROOT schedules it.

## Host

Read-only, with no cargo unless ROOT grants it. Scratch goes in `WT/scratch/rv98_u8_01/`.

## Output

- **The report:** `NUM/R/REVIEW_RV98/u8_passb_01/REVIEW.md` plus SHA256SUMS, containing the verdict, the counts and the findings.
- **Time box:** 2 hours.
- **End your turn** with the verdict, the counts, the sha256 and anything ROOT must rule on.
