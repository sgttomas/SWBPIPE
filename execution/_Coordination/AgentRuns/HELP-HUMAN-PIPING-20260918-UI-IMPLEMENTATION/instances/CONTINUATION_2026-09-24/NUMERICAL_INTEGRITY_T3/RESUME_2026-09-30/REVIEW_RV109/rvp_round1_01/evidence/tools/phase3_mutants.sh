#!/bin/bash
# RV109 phase 3: the mutants, one at a time, in the reviewer's candidate copy (probe removed first),
# each with PP's whole lib suite including the ignored witnesses, registered, target rv109-mut.
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
CAND=$WT/rv109/cand/projects/chirality-piping/core/product_physics
LIB=$CAND/src/lib.rs
PRISTINE_SHA=84fba5ca57905d69ea979ce815e584f26eb95ac45d32a54e30918d7580edd586
mkdir -p $S/mutants
# Remove the probe from the candidate copy: restore the witness file from the head and delete the probe files.
cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git show a8e719f5b4:projects/chirality-piping/core/product_physics/src/retained_memory_witness_tests.rs > $CAND/src/retained_memory_witness_tests.rs
rm -f $CAND/src/zz_rv109_probe.rs $CAND/src/zz_rv109_rev.rs
GIT_OPTIONAL_LOCKS=0 git show a8e719f5b4:projects/chirality-piping/core/product_physics/src/lib.rs > $S/mutants/lib.pristine.rs
[ "$(shasum -a 256 $S/mutants/lib.pristine.rs | cut -d' ' -f1)" = "$PRISTINE_SHA" ] || { echo "pristine sha mismatch" > $S/mutants/ABORT; exit 1; }
cp $S/mutants/lib.pristine.rs $LIB
if [ "${SKIP_PRISTINE:-0}" != 1 ]; then
  $S/runjob.sh mut_pristine $CAND rv109-mut 0 test --locked --offline --no-fail-fast --lib -- --include-ignored
  cp $S/logs/mut_pristine.log $S/mutants/pristine.log
fi
MUTANTS=${MUTANTS:-$(python3 $S/tools/mutants.py list | cut -f1)}
for m in $MUTANTS; do
  cp $S/mutants/lib.pristine.rs $LIB
  python3 $S/tools/mutants.py $LIB apply $m > $S/mutants/$m.apply 2>&1 || { echo "apply failed" >> $S/mutants/$m.apply; continue; }
  shasum -a 256 $LIB > $S/mutants/$m.lib.sha256
  $S/runjob.sh mut_$m $CAND rv109-mut 0 test --locked --offline --no-fail-fast --lib -- --include-ignored
  cp $S/logs/mut_$m.log $S/mutants/$m.log
done
cp $S/mutants/lib.pristine.rs $LIB
shasum -a 256 $LIB > $S/mutants/restored.sha256
echo PHASE3_DONE $(date -u '+%FT%TZ') > $S/logs/phase3.done
