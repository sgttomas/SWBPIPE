#!/bin/bash
# RV109 addendum 01: I85's I3 step at 03f55e7178 against I3 (2ba2f81863). One heavy job of mine at a
# time (each through runjob.sh -> WT/tools/t3_cargo.sh, under the slot locks).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/a1
J=$S/runjob.sh
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
I86=$R/I86/b1_w_probe_01/_run_records/inputs
cd $X
for pair in i3:2ba2f81863 head:03f55e7178 mut:03f55e7178; do
  rev=${pair%%:*}; sha=${pair##*:}
  mkdir -p $WT/rv109/a1_$rev
  [ -d $WT/rv109/a1_$rev/projects ] || (cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git archive $sha | tar -x -C $WT/rv109/a1_$rev)
done
# 1. The suites, I3 against the head.
for rev in i3 head; do
  PPD=$WT/rv109/a1_$rev/projects/chirality-piping/core/product_physics
  RND=$WT/rv109/a1_$rev/projects/chirality-piping/core/runner/headless
  RED=$WT/rv109/a1_$rev/projects/chirality-piping/core/reporting/result_export
  $J a1_pp_reg_$rev   $PPD rv109-a1-$rev 0 test --locked --offline --no-fail-fast
  $J a1_wit_$rev      $PPD rv109-a1-$rev 0 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
  $J a1_pp_stale_$rev $PPD rv109-a1-$rev-stale 1 test --locked --offline --no-fail-fast --lib
  $J a1_runner_$rev   $RND rv109-a1-runner-$rev 0 test --locked --offline --no-fail-fast
  $J a1_re_$rev       $RED rv109-a1-re-$rev 0 test --locked --offline --no-fail-fast --test retained_precision_carriers
done
# 2. The probe, on both.
FILES=$I86/a_s0.1.json,$I86/b2_k1e3.json,$I86/c1.json,$I86/i3_c1_case_a.json,$I86/i3_c1_case_b.json,$I86/i3_c1_case_c.json,$I86/i3_c1_three_case.json
FIXFILES=$(cat $S/probe/fixture_files.txt)
for rev in i3 head; do
  D=$WT/rv109/a1_$rev/projects/chirality-piping/core/product_physics/src
  cp $X/probe/zz_rv109_probe.rs $D/zz_rv109_probe.rs
  cp $X/probe/zz_rv109_rev.head.rs $D/zz_rv109_rev.rs
  grep -q 'mod zz_rv109_probe' $D/retained_memory_witness_tests.rs || printf '\n#[cfg(test)]\n#[path = "zz_rv109_probe.rs"]\nmod zz_rv109_probe;\n' >> $D/retained_memory_witness_tests.rs
  mkdir -p $X/probe/$rev/wc2 $X/probe/$rev/doc $X/probe/$rev/rev
  rm -f $X/probe/$rev/out.jsonl $X/probe/$rev/stage.jsonl
  RV109_FIX=$WT/rv109/a1_$rev/projects/chirality-piping/fixtures RV109_FIX_FILES="$FIXFILES" RV109_FILES="$FILES" RV109_ORDERS=abc,cba,aa2 \
  RV109_OUT=$X/probe/$rev/out.jsonl RV109_STAGE_OUT=$X/probe/$rev/stage.jsonl RV109_WC2_DIR=$X/probe/$rev/wc2 RV109_DOC_DIR=$X/probe/$rev/doc RV109_REV_DIR=$X/probe/$rev/rev \
    $J a1_probe_$rev $WT/rv109/a1_$rev/projects/chirality-piping/core/product_physics rv109-a1-$rev 0 test --locked --offline --lib zz_rv109 -- --ignored --nocapture --test-threads=1
done
# 3. The mutants, on a fresh head copy.
PPD=$WT/rv109/a1_mut/projects/chirality-piping/core/product_physics
D=$PPD/src
mkdir -p $X/mutants
for f in lib.rs retained_product.rs retained_wire.rs retained_facade_tests.rs; do cp $D/$f $X/mutants/$f.pristine; done
python3 $X/a1_mutants.py list > $X/mutants/list.tsv
$J a1_mut_pristine $PPD rv109-a1-mut 0 test --locked --offline --no-fail-fast --lib
cp $S/logs/a1_mut_pristine.log $X/mutants/pristine.log
for m in $(cut -f1 $X/mutants/list.tsv); do
  for f in lib.rs retained_product.rs retained_wire.rs retained_facade_tests.rs; do cp $X/mutants/$f.pristine $D/$f; done
  python3 $X/a1_mutants.py $D apply $m > $X/mutants/$m.apply 2>&1 || { echo "apply failed" >> $X/mutants/$m.apply; continue; }
  $J a1_mut_$m $PPD rv109-a1-mut 0 test --locked --offline --no-fail-fast --lib
  cp $S/logs/a1_mut_$m.log $X/mutants/$m.log
done
for f in lib.rs retained_product.rs retained_wire.rs retained_facade_tests.rs; do cp $X/mutants/$f.pristine $D/$f; done
shasum -a 256 $D/lib.rs $D/retained_product.rs $D/retained_wire.rs $D/retained_facade_tests.rs | sed "s|$D/||" > $X/mutants/restored_src.sha256
echo A1_DONE $(date -u '+%FT%TZ') > $X/logs/a1.done
