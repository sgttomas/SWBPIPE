#!/bin/bash
# I89 B1-SA: the committed c = 1 successor pins' bytes, written by the pin tests' own output
# variables (I61_U3_OUT, I61_U3G2_OUT, I68_U8_OUT), in base (I1 archive) and candidate,
# registered build, one cargo job each through WT/tools/t3_cargo.sh. After I85's run_pins.sh.
set -u
WT=WT
S=$WT/scratch/i89_b1_sa; L=$S/logs; T=$WT/targets/i89-b1-sa
for side in cand base; do
  if [ $side = cand ]; then dir=$WT/b1-a/projects/chirality-piping/core/product_physics; target=$T; else dir=$S/base/projects/chirality-piping/core/product_physics; target=$T/base; fi
  out=$S/pins/$side; rm -rf $out; mkdir -p $out
  ( cd $dir && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
    I61_U3_OUT=$out I61_U3G2_OUT=$out I68_U8_OUT=$out TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$target \
      $WT/tools/t3_cargo.sh test --locked --offline --lib -- --exact \
      retained_facade_tests::u3_permitted_path_publishes_the_pinned_successor \
      retained_facade_tests::u3g2_direct_entry_publishes_the_pinned_successor \
      retained_facade_tests::u8_l0_isolated_node_publishes_pinned_successor > $L/pins_$side.log 2>&1 )
  echo "pins_$side rc=$?"; grep "^test " $L/pins_$side.log
done
cd $S/pins && shasum -a 256 cand/* base/* > pins.sha256
for f in cand/*; do b=base/${f#cand/}; if cmp -s $f $b; then echo "identical ${f#cand/}"; else echo "DIFFERENT ${f#cand/}"; fi; done
P=$WT/b1-a/projects/chirality-piping/fixtures/results
for m in sparse_interactive dense_scrutiny; do
  cmp -s cand/u3g2_successor_$m.json $P/retained_precision_milestone_successor_$m.json && echo "fixture milestone $m equal"
  cmp -s cand/retained_precision_l0_successor_$m.json $P/retained_precision_l0_successor_$m.json && echo "fixture l0 $m equal"
done
