# T2 sparse-outcome correction

The RETURN table's “All12 Passed” sparse-outcome cell is incorrect. The actual
12 sparse processes produced **8 Sensitive and4 Passed** outcomes, all with
runner exit0/classificationok and passing recorded parity. No numerical outcome
is reclassified. FULL_CHECKS.json, RESULTS_SUMMARY.json and the canonical raw
outputs already preserve these correct distinct outcome classes.

This corrects only the human-readable summary assumption carried from T1.
No runtime, comparison, bound, admission, data or policy changes. All12 W1
processes selected128 with verification256, including each recorded repeat.
The original26-payload seal and every base-packet byte remain unchanged; this
addendum is separately sealed and must accompany the return in review.
