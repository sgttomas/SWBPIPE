#!/bin/bash
# RV109 round 2: I85's I3 front-run, reproduced on a scratch merge: SP's head 603e238517 with SR-RS
# b5cb7faaeb's three RE files (the head leaves RE untouched since I1, SR-RS's base). No Git writes.
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/r2
J=$S/runjob.sh
I3=$WT/rv109/i3
mkdir -p $I3 $X/i3
[ -d $I3/projects ] || (cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git archive 603e238517 | tar -x -C $I3)
for f in projects/chirality-piping/core/reporting/result_export/src/retained_precision.rs \
         projects/chirality-piping/core/reporting/result_export/src/source_blocks.rs \
         projects/chirality-piping/core/reporting/result_export/tests/retained_precision_contract.rs; do
  (cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git show b5cb7faaeb:$f) > $I3/$f
done
shasum -a 256 $I3/projects/chirality-piping/core/reporting/result_export/src/retained_precision.rs $I3/projects/chirality-piping/core/reporting/result_export/src/source_blocks.rs \
  $I3/projects/chirality-piping/core/reporting/result_export/tests/retained_precision_contract.rs | sed "s|$I3/||" > $X/i3/overlay.sha256
PPD=$I3/projects/chirality-piping/core/product_physics
RED=$I3/projects/chirality-piping/core/reporting/result_export
# 1. The merge's registered PP suite, with output (pins print), one thread.
$J r2_i3_pp $PPD rv109-r2-i3 0 test --locked --offline --no-fail-fast -- --nocapture --test-threads=1
# 2. RE: SR-RS's contract tests and the carriers on the merge.
$J r2_i3_re $RED rv109-r2-i3-re 0 test --locked --offline --no-fail-fast --test retained_precision_contract --test retained_precision_carriers
# 3. The reviewer's W-C2 probe on the merge.
D=$PPD/src
cp $X/probe/zz_rv109_probe.rs $D/zz_rv109_probe.rs
cp $X/probe/zz_rv109_rev.head.rs $D/zz_rv109_rev.rs
grep -q 'mod zz_rv109_probe' $D/retained_memory_witness_tests.rs || printf '\n#[cfg(test)]\n#[path = "zz_rv109_probe.rs"]\nmod zz_rv109_probe;\n' >> $D/retained_memory_witness_tests.rs
mkdir -p $X/i3/wc2
RV109_WC2_DIR=$X/i3/wc2 $J r2_i3_probe $PPD rv109-r2-i3 0 test --locked --offline --lib zz_rv109_wc2_rederive -- --ignored --nocapture --test-threads=1
echo R2_I3_DONE $(date -u '+%FT%TZ') > $X/logs/i3.done
