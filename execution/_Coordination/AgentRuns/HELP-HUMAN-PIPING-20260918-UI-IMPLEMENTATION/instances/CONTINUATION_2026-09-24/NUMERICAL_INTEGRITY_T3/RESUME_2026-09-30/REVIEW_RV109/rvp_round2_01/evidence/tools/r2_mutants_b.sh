#!/bin/bash
# RV109 round 2: the reviewer's mutants of SP at 603e238517, each on a fresh copy of the pristine
# sources (own copy WT/rv109/mut; one cargo job at a time; PP lib tests, registered build).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/r2
J=$S/runjob.sh
MUT=$WT/rv109/mut
mkdir -p $MUT $X/mutants
[ -d $MUT/projects ] || (cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git archive 603e238517 | tar -x -C $MUT)
PPD=$MUT/projects/chirality-piping/core/product_physics
D=$PPD/src
for f in lib.rs retained_product.rs retained_wire.rs; do cp $D/$f $X/mutants/$f.pristine; done
python3 $X/sp2_mutants.py list | grep "^B_" > $X/mutants/list_b.tsv
# Pristine control on the same target and flags.
# (pristine control: batch A)

for m in $(cut -f1 $X/mutants/list_b.tsv); do
  for f in lib.rs retained_product.rs retained_wire.rs; do cp $X/mutants/$f.pristine $D/$f; done
  python3 $X/sp2_mutants.py $D apply $m > $X/mutants/$m.apply 2>&1 || { echo "apply failed" >> $X/mutants/$m.apply; continue; }
  $J r2_mut_$m $PPD rv109-r2-mut 0 test --locked --offline --no-fail-fast --lib
  cp $S/logs/r2_mut_$m.log $X/mutants/$m.log
done
for f in lib.rs retained_product.rs retained_wire.rs; do cp $X/mutants/$f.pristine $D/$f; done
shasum -a 256 $D/*.rs | sed "s|$D/||" > $X/mutants/restored_src_b.sha256
echo R2_MUT_DONE $(date -u '+%FT%TZ') > $X/logs/mut_b.done
