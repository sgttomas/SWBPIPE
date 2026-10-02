# I23 additive tail prediction correction11

Confirmed independently from actual used/shift_up/net source behavior.
Start **2026-10-01 16:18:01 UTC**; return **2026-10-01T16:20:11Z**
(130.4 seconds, within5 minutes).

The prior I23 prediction was wrong because it minimized the live significant
limbs instead of retaining ExactWideSum.used's spare zero limbs:
- R09: used2→5→10; M2 start5/drop320. Both bits300 and0 are discarded,
  leaving direct exact1 (+8p0), without a retained midpoint/tie mechanism.
- R10: used2→5→6; top limb5 is zero, so faulty bits320/cut192 clears every
  live bit133/5/0. from_integer_rounded returns signed_zero(false), tokenZ+.
  The prospective R10 actual-token qualifier is corrected from+8p0 toZ+.

Both original expected upper tokens remain1+2^-127, unchanged. The exact
run_targeted line102 numeric assertion, row/terms/width/precision and exclusions
remain. No generic failure is credited. CORRECTION.json precisely cites each
superseded sealed prediction/qualifier field; CORRECTED_ELIGIBILITY.json carries
the corrected prospective interpretation. Predictions are not runtime results.

Both old packet seals and all580 files across the five prepared copies were
rechecked unchanged. No original expectation, corpus, test, selector, fault,
overlay, copy or target was written. No Rust/runtime/Git/index/model/solver or
delegation occurred. Only this additive correction packet was created.
Same RV29 must backcheck before any ROOT diagnostic runtime grant.
