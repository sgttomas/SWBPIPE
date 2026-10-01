# ROOT disposition of the KF1 inclusive-work failure

RV29's single full-suite command found
kf1_golden_stop_rule_work_where_collapses_occur failing on the first frozen
candidate. The existing absolute values describe R7-only stop-rule work; the
selected correction intentionally charges the certificate to that same stage.
This mismatch is not by itself a wrong-charge or availability finding. Complete
suite diagnostics still must be retained. No historical value may be replaced
by an observed new total, no >= relaxation, and no changed selected outcome.

ROOT adopts RV29's proposed criterion-preserving adaptation in principle:
recreate the exact report for the same retained candidate/verification states,
exercise compare_states directly and keep the old R7-only golden numbers;
separately check inclusive stop_rule_work equals the R7 result plus certificate
contribution. Preserve all unrelated per-stage numbers, selected/publication
checks, T-infinity=T512, and exact T64 minus T-infinity tracker-collapse deltas.
Independent absolute certificate ledgers and PM16 still carry the new component's
accounting proof; a self-replay is not offered as that independent oracle.

The manager may ask I22 for a small proposed patch in owned C evidence showing
this adaptation and its helper setup. No maintained edit in kf1_tracker_tests.rs
is yet granted: ROOT will read that concrete patch and the complete failure
before authorizing this additional test path. Keep C's independent new-test and
fixed-source work moving; do not expand the numerical helper or instrumentation
API to make this test pass. Return a concrete dependency if existing private test
access cannot express the direct check without such expansion.
