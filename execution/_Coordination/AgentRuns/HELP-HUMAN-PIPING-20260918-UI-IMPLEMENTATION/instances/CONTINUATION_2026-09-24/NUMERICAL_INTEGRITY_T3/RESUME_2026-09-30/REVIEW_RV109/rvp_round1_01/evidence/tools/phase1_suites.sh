#!/bin/bash
# RV109 phase 1: the suites on pristine base and candidate copies, one cargo job at a time.
WT=WT
S=$WT/scratch/rv109_rvp_01
J=$S/runjob.sh
PPB=$WT/rv109/base/projects/chirality-piping/core/product_physics
PPC=$WT/rv109/cand/projects/chirality-piping/core/product_physics
RNB=$WT/rv109/base/projects/chirality-piping/core/runner/headless
RNC=$WT/rv109/cand/projects/chirality-piping/core/runner/headless
REB=$WT/rv109/base/projects/chirality-piping/core/reporting/result_export
REC=$WT/rv109/cand/projects/chirality-piping/core/reporting/result_export
$J pp_reg_base  $PPB rv109-base 0 test --locked --offline --no-fail-fast
$J pp_reg_cand  $PPC rv109-cand 0 test --locked --offline --no-fail-fast
$J wit_base     $PPB rv109-base 0 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
$J wit_cand     $PPC rv109-cand 0 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
$J pp_stale_base $PPB rv109-base-stale 1 test --locked --offline --no-fail-fast --lib
$J pp_stale_cand $PPC rv109-cand-stale 1 test --locked --offline --no-fail-fast --lib
$J runner_base  $RNB rv109-runner-base 0 test --locked --offline --no-fail-fast
$J runner_cand  $RNC rv109-runner-cand 0 test --locked --offline --no-fail-fast
$J re_carriers_cand $REC rv109-re-cand 0 test --locked --offline --no-fail-fast --test retained_precision_carriers
echo PHASE1_DONE $(date -u '+%FT%TZ') > $S/logs/phase1.done
