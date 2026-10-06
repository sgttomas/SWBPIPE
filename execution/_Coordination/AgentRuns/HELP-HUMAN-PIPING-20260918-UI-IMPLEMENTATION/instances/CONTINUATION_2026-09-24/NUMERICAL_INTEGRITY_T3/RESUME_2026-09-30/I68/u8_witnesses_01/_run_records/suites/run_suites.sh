#!/bin/bash
# I68 U8-1: suites, base (b1e2d7741e archive) and candidate (WT/f2a-u8 working tree). One cargo
# job at a time through WT/tools/t3_cargo.sh; --locked --offline; CARGO_BUILD_JOBS=4;
# RUST_TEST_THREADS=2; RUSTFLAGS unset (registered) or --cfg=i68_u8_stale (Stale); each under a
# 7200 s alarm (which also covers the lock wait).
set -u
WT=WT
S=$WT/scratch/i68_u8_witnesses; L=$S/logs
BASE=$S/base/projects/chirality-piping/core; CAND=$WT/f2a-u8/projects/chirality-piping/core
outcomes() { awk '/^ *Running /{t=($2=="unittests")?$3:$2} /^ *Doc-tests /{t="doctests:"$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ - should panic/,""); sub(/ \(line [0-9]+\)/,""); print t" :: "$0}' $L/$1.log | sed -E 's#\(?[^ ]*/deps/##; s#-[0-9a-f]{16}\)?##' | sort > $L/$1.outcomes; }
count() { echo "$1: $(grep -c ' ok$' $L/$1.outcomes) ok, $(grep -c ' FAILED$' $L/$1.outcomes) failed, $(grep -c ' ignored' $L/$1.outcomes) ignored"; grep ' FAILED$' $L/$1.outcomes; }
run() { # name crate_dir target_dir [RUSTFLAGS]
  pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $1 start $(date -u +%FT%TZ)"
  ( cd $2 && if [ -n "${4:-}" ]; then export RUSTFLAGS="$4"; else unset RUSTFLAGS; fi; unset CARGO_ENCODED_RUSTFLAGS
    CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$3 perl -e 'alarm shift; exec @ARGV' 7200 $WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast > $L/$1.log 2>&1 )
  echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; count $1
}
for s in "$@"; do case "$s" in
  base_reg_pp) run base_reg_pp $BASE/product_physics $WT/targets/i68-u8/base ;;
  base_reg_rx) run base_reg_rx $BASE/reporting/result_export $WT/targets/i68-u8/base ;;
  base_reg_runner) run base_reg_runner $BASE/runner/headless $WT/targets/i68-u8/base ;;
  cand_reg_pp) run cand_reg_pp $CAND/product_physics $WT/targets/i68-u8 ;;
  cand_reg_rx) run cand_reg_rx $CAND/reporting/result_export $WT/targets/i68-u8 ;;
  cand_reg_runner) run cand_reg_runner $CAND/runner/headless $WT/targets/i68-u8 ;;
  cand_stale_pp) run cand_stale_pp $CAND/product_physics $WT/targets/i68-u8/stale "--cfg=i68_u8_stale" ;;
  base_stale_pp) run base_stale_pp $BASE/product_physics $WT/targets/i68-u8/base-stale "--cfg=i68_u8_stale" ;;
esac; done
