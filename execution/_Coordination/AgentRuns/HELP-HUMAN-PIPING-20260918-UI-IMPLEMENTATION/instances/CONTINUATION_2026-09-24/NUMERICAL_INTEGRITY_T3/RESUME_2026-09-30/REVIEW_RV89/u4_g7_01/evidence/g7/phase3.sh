#!/bin/bash
# RV89 G7: restore my copy to the pristine tree, apply I65's proposal, and rerun what could move.
T=WT; G=WT/scratch/rv89_u4_g7_01; TG=$T/targets/rv89_g7
C=$T/rv89_g7/base/projects/chirality-piping/core; PM=$C/product_physics/Cargo.toml
rm -f $C/product_physics/src/rv89_g6_probe_tests.rs $C/product_physics/src/rv89_g7_probe_tests.rs $C/product_physics/tests/zz_rv89_sweep.rs $C/product_physics/tests/zz_rv89_g7_sweep.rs
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive ba1faa1c858ce3630a22767677310b1902a14b83 projects/chirality-piping/core/reporting/result_export/src/semantic_contract.rs projects/chirality-piping/core/reporting/result_export/src/retained_precision.rs projects/chirality-piping/core/product_physics/src/lib.rs projects/chirality-piping/core/product_physics/src/retained_memory.rs | tar -x -C $T/rv89_g7/base )
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git diff --no-index --stat /dev/null /dev/null >/dev/null 2>&1 ; true )
for f in reporting/result_export/src/semantic_contract.rs reporting/result_export/src/retained_precision.rs product_physics/src/lib.rs product_physics/src/retained_memory.rs; do
  ( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git show ba1faa1c858ce3630a22767677310b1902a14b83:projects/chirality-piping/core/$f ) | cmp - $C/$f || { echo "NOT PRISTINE $f"; exit 1; }
done
echo PRISTINE
( cd $T/rv89_g7/base && patch -p1 < R/I65/u4_g7_01/_run_records/pass_a/proposal/t17_v4_f5.diff ) || exit 1
$G/run.sh g7p_law $PM $TG/base --lib -- retained_memory --nocapture --test-threads=1
RUST_TEST_THREADS=1 $G/run.sh g7p_challenge $PM $TG/base --test retained_memory_challenge -- --nocapture --test-threads=1
$G/run.sh g7p_pp $PM $TG/base
echo PHASE3 DONE
