#!/bin/bash
# I61 U3 grant 2: the fixture sweeps (scratch copies only). One cargo job at a time.
set -u
T=WT; S=$T/scratch/i61_u3_grant2_02
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
sweep() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $S/$2/projects/chirality-piping/core/product_physics && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 I61_U3G2_SWEEP=$S/sweep/$1.tsv ${4:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --lib --target-dir $3 zz_i61_u3g2_sweep -- --ignored --nocapture > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; grep -E "I61_U3G2_SWEEP|test result" $S/logs/$1.log; }
mkdir -p $S/sweep
sweep sweep_cand_reg cand $T/targets/i61-u3g2/reg
sweep sweep_cand_stale cand $T/targets/i61-u3g2/stale "RUSTFLAGS=--cfg=i61_u3g2_stale"
