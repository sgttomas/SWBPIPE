#!/bin/bash
# I85 B1-SP checkpoint R3': the suites at I1 and at the SP head, then the head's pin bytes.
set -u
WT=WT; S=$WT/scratch/i85_b1_st; L=$S/sp01/logs
echo "== start $(date -u +%FT%TZ)"
$S/sp01/sp_suites.sh cand_reg_pp base_reg_pp cand_stale_pp base_stale_pp cand_reg_witness base_reg_witness cand_reg_runner base_reg_runner cand_reg_re_carriers
out=$S/sp01/pins; rm -rf $out; mkdir -p $out
( cd $WT/b1/projects/chirality-piping/core/product_physics && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
  I61_U3_OUT=$out I61_U3G2_OUT=$out I68_U8_OUT=$out TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/i85-b1-st \
    $WT/tools/t3_cargo.sh test --locked --offline --lib -- --exact \
    retained_facade_tests::u3_permitted_path_publishes_the_pinned_successor \
    retained_facade_tests::u3g2_direct_entry_publishes_the_pinned_successor \
    retained_facade_tests::u8_l0_isolated_node_publishes_pinned_successor > $L/pins_cand.log 2>&1 )
echo "pins rc=$?"; grep "^test " $L/pins_cand.log
cd $out && shasum -a 256 * > pins.sha256 && cat pins.sha256
P=$WT/b1/projects/chirality-piping/fixtures/results
for m in sparse_interactive dense_scrutiny; do
  cmp -s u3g2_successor_$m.json $P/retained_precision_milestone_successor_$m.json && echo "fixture milestone $m equal (u3g2)"
  cmp -s u3_successor_$m.json $P/retained_precision_milestone_successor_$m.json && echo "fixture milestone $m equal (u3)"
  cmp -s retained_precision_l0_successor_$m.json $P/retained_precision_l0_successor_$m.json && echo "fixture l0 $m equal"
done
echo "== end $(date -u +%FT%TZ)"
