#!/bin/bash
# RV109 round 2: batch A's surviving mutants on the I3 scratch merge (SP's head + SR-RS b5cb7faaeb's
# reader): does the accepted reader (precommit) or a reviewer check catch each after I3?
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/r2
J=$S/runjob.sh
PPD=$WT/rv109/i3/projects/chirality-piping/core/product_physics
D=$PPD/src
mkdir -p $X/i3mut
cp $X/probe/zz_rv109_rev.head.rs $D/zz_rv109_rev.rs
for m in M11_sources_back_reversed M15_selected_diagnostics_reversed M16_frozen_attempt_ref_zero M17_frozen_attempt_id_zero M19_frozen_run_owner_ordinal M28_headline_tie_location_only M32_headline_stress_only; do
  for f in lib.rs retained_product.rs retained_wire.rs; do cp $X/mutants/$f.pristine $D/$f; done
  python3 $X/sp2_mutants.py $D apply $m > $X/i3mut/$m.apply 2>&1 || { echo "apply failed" >> $X/i3mut/$m.apply; continue; }
  RV109_ORDERS=abc,cba,bca,aa2,aca2,a2ba $J r2_i3mut_$m $PPD rv109-r2-i3 0 test --locked --offline --lib zz_rv109_wc2_rederive -- --ignored --nocapture --test-threads=1
  cp $S/logs/r2_i3mut_$m.log $X/i3mut/$m.log
done
for f in lib.rs retained_product.rs retained_wire.rs; do cp $X/mutants/$f.pristine $D/$f; done
shasum -a 256 $D/lib.rs $D/retained_product.rs $D/retained_wire.rs | sed "s|$D/||" > $X/i3mut/restored_src.sha256
echo R2_I3MUT_DONE $(date -u '+%FT%TZ') > $X/logs/i3mut.done
