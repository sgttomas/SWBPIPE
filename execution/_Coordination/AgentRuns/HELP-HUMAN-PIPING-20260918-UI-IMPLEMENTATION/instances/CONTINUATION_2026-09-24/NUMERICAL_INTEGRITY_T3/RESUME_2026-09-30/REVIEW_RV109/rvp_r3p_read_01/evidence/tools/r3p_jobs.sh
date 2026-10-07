#!/bin/bash
# RV109 R3′ early read of SP's first part. One cargo job at a time via runjob.sh -> t3_cargo.sh.
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/r3p
J=$S/runjob.sh
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
I86=$R/I86/b1_w_probe_01/_run_records/inputs
I1_SHA=262bd687f0
SP_SHA=56c5579f07b1cf3203c178e10add51bba0b24a58
mkdir -p $WT/rv109/i1
[ -d $WT/rv109/i1/projects ] || (cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git archive $I1_SHA | tar -x -C $WT/rv109/i1)
SPD=$WT/rv109/sp/projects/chirality-piping/core/product_physics
I1D=$WT/rv109/i1/projects/chirality-piping/core/product_physics
shasum -a 256 $SPD/src/*.rs $SPD/src/retained_tests_hooks/grant2.rs | sed 's|/.*/src/|PP/|' > $X/logs/sp_pristine_src.sha256
# 1. The SP checkpoint's full PP suite (control, guards).
$J r3p_sp_full $SPD rv109-sp 0 test --locked --offline --no-fail-fast
# 2. Install the probe in both copies.
for rev in i1 sp; do
  D=$WT/rv109/$rev/projects/chirality-piping/core/product_physics/src
  cp $X/probe/zz_rv109_probe.rs $D/zz_rv109_probe.rs
  cp $X/probe/zz_rv109_rev.$rev.rs $D/zz_rv109_rev.rs
  grep -q 'mod zz_rv109_probe' $D/retained_memory_witness_tests.rs || printf '\n#[cfg(test)]\n#[path = "zz_rv109_probe.rs"]\nmod zz_rv109_probe;\n' >> $D/retained_memory_witness_tests.rs
done
FIXFILES=$(cat $S/probe/fixture_files.txt)
for rev in i1 sp; do
  D=$WT/rv109/$rev/projects/chirality-piping/core/product_physics
  rm -f $X/probe/out_$rev.jsonl $X/probe/stage_$rev.jsonl
  RV109_FIX=$WT/rv109/$rev/projects/chirality-piping/fixtures RV109_FIX_FILES="$FIXFILES" RV109_FILES="$I86/b2_k1e3.json,$I86/c1.json" \
  RV109_OUT=$X/probe/out_$rev.jsonl RV109_STAGE_OUT=$X/probe/stage_$rev.jsonl \
    $J r3p_probe_$rev $D rv109-$rev-probe 0 test --locked --offline --lib zz_rv109 -- --ignored --nocapture --test-threads=1
done
# 3. Restore the SP copy's sources, then the mutants.
D=$SPD/src
cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git show $SP_SHA:projects/chirality-piping/core/product_physics/src/retained_memory_witness_tests.rs > $D/retained_memory_witness_tests.rs
rm -f $D/zz_rv109_probe.rs $D/zz_rv109_rev.rs
cp $D/lib.rs $X/mutants/lib.pristine.rs; cp $D/retained_product.rs $X/mutants/retained_product.pristine.rs
for m in $(python3 $X/sp_mutants.py list | cut -f1); do
  cp $X/mutants/lib.pristine.rs $D/lib.rs; cp $X/mutants/retained_product.pristine.rs $D/retained_product.rs
  python3 $X/sp_mutants.py $D apply $m > $X/mutants/$m.apply 2>&1 || { echo "apply failed" >> $X/mutants/$m.apply; continue; }
  $J r3p_mut_$m $SPD rv109-sp-mut 0 test --locked --offline --no-fail-fast --lib
  cp $S/logs/r3p_mut_$m.log $X/mutants/$m.log
done
cp $X/mutants/lib.pristine.rs $D/lib.rs; cp $X/mutants/retained_product.pristine.rs $D/retained_product.rs
shasum -a 256 $SPD/src/*.rs $SPD/src/retained_tests_hooks/grant2.rs | sed 's|/.*/src/|PP/|' > $X/logs/sp_restored_src.sha256
echo R3P_DONE $(date -u '+%FT%TZ') > $X/logs/r3p.done
