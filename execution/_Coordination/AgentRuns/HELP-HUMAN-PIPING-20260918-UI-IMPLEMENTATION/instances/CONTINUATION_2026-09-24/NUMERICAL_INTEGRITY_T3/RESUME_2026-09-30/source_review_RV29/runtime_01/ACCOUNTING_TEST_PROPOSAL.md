# Proposal to adapt the affected accounting test without changing its oracle

Status: **PROPOSED ONLY to ROOT; no patch, new run, or expected-table change.**
Scope: the failing `kf1_golden_stop_rule_work_where_collapses_occur` test after
the intentional addition of certificate work to the existing stop_rule stage.
This does not propose changing any numerical, selection or availability criterion.

The old fixture's absolute stop_rule number measured R7 alone. The new record
contains R7 plus certificate admission. A mismatch of those differently defined
quantities is not, by itself, wrong charging. Preserve both accounting layers:

1. Keep the existing two RF-LARGE models, selected outcomes, precisions, all old
   golden numbers for R7/refinement/bounded-gate/shared-condition, and the exact
   tracker evaluation price17,506. Obtain an R7-only `compare_states` result for
   the same retained candidate and verification state, using the matching report
   recreated explicitly in the test from that state/prep/group. Compare that
   R7-only result to the unchanged old golden. The report recomputation is test
   setup and must not be included in the recorded solve's work.
2. On that same bound state/report, separately evaluate the certificate stage
   and require successful certification with the same published bits/radii.
   Assert the solve's inclusive stop_rule_work and stages.stop_rule equal the
   independently anchored R7 result plus the stage's charged certificate work.
   This decomposition assertion is stage-closure evidence, not an independent
   absolute certificate oracle. The frozen RV29 operation ledgers and forthcoming
   PC40–43/PM16 tests supply that independent absolute certificate discriminator.
3. Retain production T=512 versus T=infinity equality and exact T=64 excesses
   768*17,506 / 640*17,506. Require the certificate work and certified output to
   be invariant under these tracker settings, so its addition cancels only
   where separately shown equal. Preserve unrelated-stage assertions and the
   existing same_solve/same_attempts differential coverage.

This proposed adaptation keeps the old R7 absolute anchor instead of replacing
it with newly observed inclusive totals. Do not use merely `new>=old`, update the
old numbers from the failing output, remove the assertion, or let a newly
unselected model disappear from the comparison. Recreating a report must use
existing private test-accessible APIs and explicit frozen state identity; do
not retain reports in production merely to support this test. If implementing
this bounded separation requires a broader helper or production change, return
that exact issue to ROOT before expanding scope.

ROOT decides the criterion-preserving test amendment and any new run grant.
RV29 must backcheck its concrete delta and changed test coverage. No such
amendment or re-execution is performed in runtime_01.
