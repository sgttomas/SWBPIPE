# I23 — additive used-limb prediction correction

ROOT adopts the need for correction identified by RV29 tail_sources_15;
no runtime is authorized. Existing I23 gets one 5-minute source-only addendum.
Read that sealed review and independently recompute the two exact traces using
actual ExactWideSum.used/shift_up behavior, including spare zero limbs.

Correct R09's mechanism from minimal-limb/tie reasoning to used2→5→10,
dropping320 bits and yielding direct1 (+8p0). Correct R10's fault prediction
from minimal134-bit/6-bit truncation to used2→5→6, faulty bits320/cut192,
yielding canonical positive zero (Z+). Bind exact source and integer arithmetic.
If the trace does not reproduce, report the discrepancy instead of copying it.

The protected unmutated result remains the original1+2^-127 token in BOTH
unchanged corpus rows. Exact selectors, all original tests, both fault patches,
combined overlay and every prepared copy remain byte-identical. Only the
erroneous predicted fault mechanism/token and its prospective eligibility
description are corrected additively. R10 must reach the same original numeric
assertion with the same fixed terms; setup/refusal/earlier-loop failures remain
excluded. No generic failure or lowered expected value becomes acceptable.

Write only A1 R/I23/r09_r10_tail_correction_11. Cite superseded sealed fields
precisely, give the corrected runtime interpretation and preserve their bytes.
No source/corpus/copy/target edit, Rust, test, model, Git/index or delegation.
Same RV29 must backcheck before any ROOT diagnostic runtime grant.
