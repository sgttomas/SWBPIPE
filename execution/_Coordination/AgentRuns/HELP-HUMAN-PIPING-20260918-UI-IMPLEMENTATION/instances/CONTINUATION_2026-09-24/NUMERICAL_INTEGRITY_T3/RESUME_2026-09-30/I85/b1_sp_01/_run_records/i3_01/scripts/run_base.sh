#!/bin/bash
# I85 B1-SP I3: the base suites and pins at 2ba2f81863 (archive S/base_i3), one cargo job at a time.
set -u
WT=WT; S=$WT/scratch/i85_b1_st
echo "== start $(date -u +%FT%TZ)"
SUITE_LOGS=$S/sp03/suites $S/sp03/sp_suites.sh base_reg_pp base_stale_pp base_reg_runner base_reg_witness
out=$S/sp03/pins_base; rm -rf $out; mkdir -p $out
( cd $S/base_i3/projects/chirality-piping/core/product_physics && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
  I61_U3_OUT=$out I61_U3G2_OUT=$out I68_U8_OUT=$out TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/i85-b1-st/base \
    $WT/tools/t3_cargo.sh test --locked --offline --lib -- --exact \
    retained_facade_tests::u3_permitted_path_publishes_the_pinned_successor \
    retained_facade_tests::u3g2_direct_entry_publishes_the_pinned_successor \
    retained_facade_tests::u8_l0_isolated_node_publishes_pinned_successor > $S/sp03/logs/pins_base.log 2>&1 )
echo "pins rc=$?"; grep "^test " $S/sp03/logs/pins_base.log
(cd $out && shasum -a 256 * > pins.sha256 && cat pins.sha256)
echo "== end $(date -u +%FT%TZ)"
