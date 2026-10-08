#!/bin/bash
# RV109 round 2: the W-C2 document pins on the I3 scratch merge (after r2_i3.sh).
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/r2
J=$S/runjob.sh
PPD=$WT/rv109/i3/projects/chirality-piping/core/product_physics
cp $X/probe/zz_rv109_rev.head.rs $PPD/src/zz_rv109_rev.rs
$J r2_i3_docs $PPD rv109-r2-i3 0 test --locked --offline --lib zz_rv109_wc2_documents -- --ignored --nocapture --test-threads=1
echo R2_I3B_DONE $(date -u '+%FT%TZ') > $X/logs/i3b.done
