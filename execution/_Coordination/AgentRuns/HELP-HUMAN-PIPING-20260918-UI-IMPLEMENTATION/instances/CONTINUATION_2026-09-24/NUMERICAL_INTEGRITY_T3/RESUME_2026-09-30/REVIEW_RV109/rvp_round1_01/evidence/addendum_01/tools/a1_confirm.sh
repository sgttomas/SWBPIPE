#!/bin/bash
# RV109 ADDENDUM_01: confirm ST's repair round 1 (head 98a77c716e). One cargo job at a time via
# runjob.sh -> WT/tools/t3_cargo.sh. Copies and targets are fresh and deleted afterwards.
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
A=$S/a1
J=$S/runjob.sh
HEAD_SHA=98a77c716ef0bf1126c855d1f99ccb0778a5457a
PRISTINE_SHA=84fba5ca57905d69ea979ce815e584f26eb95ac45d32a54e30918d7580edd586
mkdir -p $WT/rv109/head
cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git archive $HEAD_SHA | tar -x -C $WT/rv109/head
PPH=$WT/rv109/head/projects/chirality-piping/core/product_physics
REH=$WT/rv109/head/projects/chirality-piping/core/reporting/result_export
LIB=$PPH/src/lib.rs
[ "$(shasum -a 256 $LIB | cut -d' ' -f1)" = "$PRISTINE_SHA" ] || { echo "head lib.rs sha mismatch" > $A/ABORT; exit 1; }
cp $LIB $A/mutants/lib.pristine.rs
$J a1_pp_reg_head   $PPH rv109-a1-head 0 test --locked --offline --no-fail-fast
$J a1_wit_head      $PPH rv109-a1-head 0 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
$J a1_pp_stale_head $PPH rv109-a1-head-stale 1 test --locked --offline --no-fail-fast --lib
$J a1_wit_stale_head $PPH rv109-a1-head-stale 1 test --locked --offline --no-fail-fast --lib witness_ -- --ignored --nocapture --test-threads=1
$J a1_re_carriers_head $REH rv109-a1-re 0 test --locked --offline --no-fail-fast --test retained_precision_carriers
$J a1_mut_pristine  $PPH rv109-a1-mut 0 test --locked --offline --no-fail-fast --lib -- --include-ignored
for m in $(python3 $S/tools/mutants.py list | cut -f1); do
  cp $A/mutants/lib.pristine.rs $LIB
  python3 $S/tools/mutants.py $LIB apply $m > $A/mutants/$m.apply 2>&1 || { echo "apply failed" >> $A/mutants/$m.apply; continue; }
  shasum -a 256 $LIB | sed 's|/.*/src/|PP/|' > $A/mutants/$m.lib.sha256
  $J a1_mut_$m $PPH rv109-a1-mut 0 test --locked --offline --no-fail-fast --lib -- --include-ignored
  cp $S/logs/a1_mut_$m.log $A/mutants/$m.log
done
cp $A/mutants/lib.pristine.rs $LIB
shasum -a 256 $LIB | sed 's|/.*/src/|PP/|' > $A/mutants/restored.sha256
cp $S/logs/a1_mut_pristine.log $A/mutants/pristine.log
echo A1_DONE $(date -u '+%FT%TZ') > $A/logs/a1.done
