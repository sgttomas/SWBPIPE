#!/bin/bash
# I85 B1-SP I3: the head's suites and pins (WT/b1 at the I3 step's head), one cargo job at a time.
set -u
WT=WT; S=$WT/scratch/i85_b1_st
echo "== start $(date -u +%FT%TZ) head $(cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git rev-parse HEAD 2>/dev/null)"
SUITE_LOGS=$S/sp03/suites $S/sp03/sp_suites.sh cand_reg_pp cand_stale_pp cand_reg_runner cand_reg_witness cand_reg_re_carriers
out=$S/sp03/pins_head; rm -rf $out; mkdir -p $out
( cd $WT/b1/projects/chirality-piping/core/product_physics && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
  I61_U3_OUT=$out I61_U3G2_OUT=$out I68_U8_OUT=$out I85_WC2_OUT=$out TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/i85-b1-st \
    $WT/tools/t3_cargo.sh test --locked --offline --lib -- --exact \
    retained_facade_tests::u3_permitted_path_publishes_the_pinned_successor \
    retained_facade_tests::u3g2_direct_entry_publishes_the_pinned_successor \
    retained_facade_tests::u8_l0_isolated_node_publishes_pinned_successor \
    retained_facade_tests::b1_sp_w_c2_direct_entry_publishes_the_pinned_successor --nocapture > $S/sp03/logs/pins_head.log 2>&1 )
echo "pins rc=$?"; grep -a "^test \|B1_SP_WC2_PIN" $S/sp03/logs/pins_head.log
(cd $out && shasum -a 256 * > pins.sha256 && cat pins.sha256)
P=$WT/b1/projects/chirality-piping/fixtures/results
for m in sparse_interactive dense_scrutiny; do
  for f in retained_precision_w_c2_successor_$m.json retained_precision_l0_successor_$m.json; do cmp -s $out/$f $P/$f && echo "fixture $f equal"; done
  cmp -s $out/u3g2_successor_$m.json $P/retained_precision_milestone_successor_$m.json && echo "fixture milestone $m equal"
done
echo "== end $(date -u +%FT%TZ)"
