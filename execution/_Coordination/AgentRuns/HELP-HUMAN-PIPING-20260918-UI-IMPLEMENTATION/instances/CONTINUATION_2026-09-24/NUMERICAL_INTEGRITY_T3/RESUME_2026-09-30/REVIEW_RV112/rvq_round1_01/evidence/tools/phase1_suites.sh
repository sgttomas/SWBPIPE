#!/bin/bash
# RV112 phase 1: the suites on pristine copies (I1, SA, b1 = SP's 56c5579f07, head = b1-a 9812c83ded), one cargo job at a time.
WT=WT
S=$WT/scratch/rv112_rvq_01
J=$S/tools/runjob.sh
for c in i1 sa b1 head; do
  PP=$WT/rv112/$c/projects/chirality-piping/core/product_physics
  RN=$WT/rv112/$c/projects/chirality-piping/core/runner/headless
  $J pp_reg_$c   $PP rv112-$c 0 test --locked --offline --no-fail-fast
  $J wit_$c      $PP rv112-$c 0 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
  $J rec_$c      $PP rv112-$c 0 test --locked --offline --no-fail-fast --lib -- profile_in_build_record structural_budgets_are_u3s --nocapture --test-threads=1
  $J pp_stale_$c $PP rv112-$c-stale 1 test --locked --offline --no-fail-fast --lib
  $J runner_$c   $RN rv112-runner-$c 0 test --locked --offline --no-fail-fast
done
echo PHASE1_DONE $(date -u '+%FT%TZ') > $S/logs/phase1.done
