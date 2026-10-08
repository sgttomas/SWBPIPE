#!/bin/bash
# RV109 phase 2: install the reviewer's probe (cfg(test), archive copies only) and run it on
# base and candidate, registered build, one cargo job at a time through the T3 lock.
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
I86=$R/I86/b1_w_probe_01/_run_records/inputs
FIXFILES=$(cd $WT/rv109/cand/projects/chirality-piping/fixtures && { find product_preview -name '*.request.json' | sort; echo product_preview/invented_dec092_temperature_g_request.json; ls results/retained_precision_milestone_successor_*.json results/retained_precision_l0_successor_*.json; } | paste -sd, -)
echo "$FIXFILES" > $S/probe/fixture_files.txt
# The candidate's witnesses in the Stale build, before the probe is installed.
$S/runjob.sh wit_stale_cand $WT/rv109/cand/projects/chirality-piping/core/product_physics rv109-cand-stale 1 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
for rev in base cand; do
  SRC=$WT/rv109/$rev/projects/chirality-piping/core/product_physics/src
  cp $S/probe/zz_rv109_probe.rs $SRC/zz_rv109_probe.rs
  cp $S/probe/zz_rv109_rev.$rev.rs $SRC/zz_rv109_rev.rs
  if ! grep -q 'mod zz_rv109_probe' $SRC/retained_memory_witness_tests.rs; then
    printf '\n#[cfg(test)]\n#[path = "zz_rv109_probe.rs"]\nmod zz_rv109_probe;\n' >> $SRC/retained_memory_witness_tests.rs
  fi
done
for rev in base cand; do
  PPD=$WT/rv109/$rev/projects/chirality-piping/core/product_physics
  rm -f $S/probe/out_$rev.jsonl
  RV109_FIX=$WT/rv109/$rev/projects/chirality-piping/fixtures RV109_FIX_FILES="$FIXFILES" \
  RV109_FILES="$I86/b2_k1e3.json,$I86/c1.json" RV109_OUT=$S/probe/out_$rev.jsonl \
    $S/runjob.sh probe_$rev $PPD rv109-$rev 0 test --locked --offline --lib zz_rv109_probe_all -- --ignored --nocapture --test-threads=1
done
echo PHASE2_DONE $(date -u '+%FT%TZ') > $S/logs/phase2.done
