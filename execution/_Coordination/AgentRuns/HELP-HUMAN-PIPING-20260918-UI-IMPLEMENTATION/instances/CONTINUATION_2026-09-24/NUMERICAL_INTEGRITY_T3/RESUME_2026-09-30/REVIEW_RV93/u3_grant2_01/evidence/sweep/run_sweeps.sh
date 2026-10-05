#!/bin/bash
# RV93: the four route sweeps, one cargo job at a time (registered: default build; Stale: RUSTFLAGS set,
# separate target dir). Writes WT/scratch/rv93_u3_grant2_01/sweep/<tag>.tsv.
set -u
WT=WT; S=$WT/scratch/rv93_u3_grant2_01
one() { tag=$1; tree=$2; tgt=$3; shift 3
  ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $tag start $(date -u +%FT%TZ)"
  ( cd $WT/rv93/$tree/projects/chirality-piping/core/product_physics && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 RV93_SWEEP=$S/sweep/$tag.tsv "$@" perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --lib --target-dir $tgt zz_rv93_route_sweep -- --ignored --nocapture > $S/logs/sweep_$tag.log 2>&1 )
  echo "== $tag exit=$? $(date -u +%FT%TZ)"; grep -E "RV93_SWEEP|test result" $S/logs/sweep_$tag.log
}
one cand_reg candsweep $WT/targets/rv93/sweep_cand
one base_reg basesweep $WT/targets/rv93/sweep_base
one cand_stale candsweep $WT/targets/rv93_stale/cand RUSTFLAGS=--cfg=rv93_stale
one base_stale basesweep $WT/targets/rv93_stale/base RUSTFLAGS=--cfg=rv93_stale
