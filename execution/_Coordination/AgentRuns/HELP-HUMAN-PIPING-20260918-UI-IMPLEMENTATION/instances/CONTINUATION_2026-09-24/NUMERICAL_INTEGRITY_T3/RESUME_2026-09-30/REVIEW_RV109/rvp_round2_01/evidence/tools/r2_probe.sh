#!/bin/bash
# RV109 round 2: the reviewer's probe at I1 and at SP's head (own copies; one cargo job at a time).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/r2
J=$S/runjob.sh
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
I86=$R/I86/b1_w_probe_01/_run_records/inputs
FILES=$I86/a_s0.1.json,$I86/b2_k1e3.json,$I86/c1.json,$I86/i3_c1_case_a.json,$I86/i3_c1_case_b.json,$I86/i3_c1_case_c.json,$I86/i3_c1_three_case.json
FIXFILES=$(cat $S/probe/fixture_files.txt)
mkdir -p $X/probe/wc2
for rev in i1 head; do
  D=$WT/rv109/$rev/projects/chirality-piping/core/product_physics/src
  cp $X/probe/zz_rv109_probe.rs $D/zz_rv109_probe.rs
  cp $X/probe/zz_rv109_rev.$rev.rs $D/zz_rv109_rev.rs
  grep -q 'mod zz_rv109_probe' $D/retained_memory_witness_tests.rs || printf '\n#[cfg(test)]\n#[path = "zz_rv109_probe.rs"]\nmod zz_rv109_probe;\n' >> $D/retained_memory_witness_tests.rs
done
for rev in i1 head; do
  D=$WT/rv109/$rev/projects/chirality-piping/core/product_physics
  rm -f $X/probe/out_$rev.jsonl $X/probe/stage_$rev.jsonl
  RV109_FIX=$WT/rv109/$rev/projects/chirality-piping/fixtures RV109_FIX_FILES="$FIXFILES" RV109_FILES="$FILES" \
  RV109_OUT=$X/probe/out_$rev.jsonl RV109_STAGE_OUT=$X/probe/stage_$rev.jsonl RV109_WC2_DIR=$X/probe/wc2 \
    $J r2_probe_$rev $D rv109-r2-$rev 0 test --locked --offline --lib zz_rv109 -- --ignored --nocapture --test-threads=1
done
echo R2_PROBE_DONE $(date -u '+%FT%TZ') > $X/logs/probe.done
