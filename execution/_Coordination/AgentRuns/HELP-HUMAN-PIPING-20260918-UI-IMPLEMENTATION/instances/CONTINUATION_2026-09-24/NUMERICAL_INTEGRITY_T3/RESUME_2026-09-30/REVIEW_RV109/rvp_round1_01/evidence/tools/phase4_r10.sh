#!/bin/bash
# RV109 phase 4: R10's discriminator on the head, then on the head with R10 applied; then restore.
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
CAND=$WT/rv109/cand/projects/chirality-piping/core/product_physics
SRC=$CAND/src
cp $S/probe/zz_rv109_probe.rs $SRC/zz_rv109_probe.rs
cp $S/probe/zz_rv109_rev.cand.rs $SRC/zz_rv109_rev.rs
grep -q 'mod zz_rv109_probe' $SRC/retained_memory_witness_tests.rs || printf '\n#[cfg(test)]\n#[path = "zz_rv109_probe.rs"]\nmod zz_rv109_probe;\n' >> $SRC/retained_memory_witness_tests.rs
cp $S/mutants/lib.pristine.rs $SRC/lib.rs
$S/runjob.sh r10_head $CAND rv109-mut 0 test --locked --offline --lib zz_rv109_r10_discriminator -- --ignored --nocapture --test-threads=1
python3 $S/tools/mutants.py $SRC/lib.rs apply R10_t4_after_reservation
$S/runjob.sh r10_mutant $CAND rv109-mut 0 test --locked --offline --lib zz_rv109_r10_discriminator -- --ignored --nocapture --test-threads=1
# Restore the candidate copy to the head's bytes.
cp $S/mutants/lib.pristine.rs $SRC/lib.rs
cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git show a8e719f5b4:projects/chirality-piping/core/product_physics/src/retained_memory_witness_tests.rs > $SRC/retained_memory_witness_tests.rs
rm -f $SRC/zz_rv109_probe.rs $SRC/zz_rv109_rev.rs
shasum -a 256 $SRC/lib.rs $SRC/retained_memory_witness_tests.rs > $S/logs/phase4_restored.sha256
echo PHASE4_DONE $(date -u '+%FT%TZ') > $S/logs/phase4.done
