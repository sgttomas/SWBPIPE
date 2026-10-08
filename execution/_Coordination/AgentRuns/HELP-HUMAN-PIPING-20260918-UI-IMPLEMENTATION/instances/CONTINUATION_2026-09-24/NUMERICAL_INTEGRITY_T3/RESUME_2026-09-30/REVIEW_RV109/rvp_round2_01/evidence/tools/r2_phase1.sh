#!/bin/bash
# RV109 round 2, phase 1: suites at I1 and at SP's head, one cargo job at a time (runjob.sh -> t3_cargo.sh).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
J=$S/runjob.sh
mkdir -p $WT/rv109/i1 $WT/rv109/head
[ -d $WT/rv109/i1/projects ] || (cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git archive 262bd687f0 | tar -x -C $WT/rv109/i1)
[ -d $WT/rv109/head/projects ] || (cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git archive 603e238517 | tar -x -C $WT/rv109/head)
for rev in i1 head; do
  PPD=$WT/rv109/$rev/projects/chirality-piping/core/product_physics
  RND=$WT/rv109/$rev/projects/chirality-piping/core/runner/headless
  RED=$WT/rv109/$rev/projects/chirality-piping/core/reporting/result_export
  $J r2_pp_reg_$rev   $PPD rv109-r2-$rev 0 test --locked --offline --no-fail-fast
  $J r2_wit_$rev      $PPD rv109-r2-$rev 0 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
  $J r2_pp_stale_$rev $PPD rv109-r2-$rev-stale 1 test --locked --offline --no-fail-fast --lib
  $J r2_runner_$rev   $RND rv109-r2-runner-$rev 0 test --locked --offline --no-fail-fast
  $J r2_re_$rev       $RED rv109-r2-re-$rev 0 test --locked --offline --no-fail-fast --test retained_precision_carriers
done
echo R2_PHASE1_DONE $(date -u '+%FT%TZ') > $S/r2/logs/phase1.done
