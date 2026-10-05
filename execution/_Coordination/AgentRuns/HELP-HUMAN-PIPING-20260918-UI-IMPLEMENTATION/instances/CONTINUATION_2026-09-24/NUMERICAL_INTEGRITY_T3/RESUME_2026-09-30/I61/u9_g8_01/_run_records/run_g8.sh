#!/bin/bash
# I61 U9 G8: the Direct-entry controls on a git archive of the PR head (WT/scratch/i61_u9g8_01/tree), registered and Stale.
# One cargo job at a time; own targets under WT/targets/i61-u9g8/.
set -u
T=WT; S=$T/scratch/i61_u9g8_01
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $S/tree/projects/chirality-piping/core/product_physics && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 I61_U3G2_SWEEP=$S/out/sweep_$1.tsv I61_U9G8_OUT=$S/out/pressure_$1.json ${3:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --lib --target-dir $2 zz_i61_ -- --ignored --nocapture > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; grep -E "I61_U3G2_SWEEP|I61_U9G8_OUT|test result" $S/logs/$1.log; }
run reg $T/targets/i61-u9g8/reg
run stale $T/targets/i61-u9g8/stale "RUSTFLAGS=--cfg=i61_u9g8_stale"
echo DONE $(date -u +%FT%TZ)
