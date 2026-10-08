#!/bin/bash
# I85 B1-ST: suites, base (47a3bdfcf5 archive, P only) and candidate (WT/b1). One cargo job at a
# time through WT/tools/t3_cargo.sh; --locked --offline; CARGO_BUILD_JOBS=4; RUST_TEST_THREADS=2;
# RUSTFLAGS unset (registered) or --cfg=i85_b1_st_stale (Stale); TMPDIR in scratch; each job under
# a 7200 s alarm (which also covers the lock wait). After I68's u8_witnesses_01 run_suites.sh.
set -u
WT=WT
S=$WT/scratch/i85_b1_st; L=$S/logs; T=$WT/targets/i85-b1-st
BASE=$S/base/projects/chirality-piping/core; CAND=$WT/b1/projects/chirality-piping/core
outcomes() { awk '/^ *Running /{t=($2=="unittests")?$3:$2} /^ *Doc-tests /{t="doctests:"$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ - should panic/,""); sub(/ \(line [0-9]+\)/,""); print t" :: "$0}' $L/$1.log | sed -E 's#\(?[^ ]*/deps/##; s#-[0-9a-f]{16}\)?##' | sort > $L/$1.outcomes; }
count() { echo "$1: $(grep -c ' ok$' $L/$1.outcomes) ok, $(grep -c ' FAILED$' $L/$1.outcomes) failed, $(grep -c ' ignored' $L/$1.outcomes) ignored"; grep ' FAILED$' $L/$1.outcomes; }
run() { # name crate_dir target_dir [RUSTFLAGS] [extra cargo args...]
  local name=$1 dir=$2 target=$3 flags=${4:-}; shift 4 2>/dev/null || shift $#
  pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $name start $(date -u +%FT%TZ)"
  ( cd $dir && if [ -n "$flags" ]; then export RUSTFLAGS="$flags"; else unset RUSTFLAGS; fi; unset CARGO_ENCODED_RUSTFLAGS
    TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$target perl -e 'alarm shift; exec @ARGV' 7200 $WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast "$@" > $L/$name.log 2>&1 )
  echo "== $name exit=$? $(date -u +%FT%TZ)"; outcomes $name; count $name
}
for s in "$@"; do case "$s" in
  cand_reg_pp) run cand_reg_pp $CAND/product_physics $T "" ;;
  base_reg_pp) run base_reg_pp $BASE/product_physics $T/base "" ;;
  cand_stale_pp) run cand_stale_pp $CAND/product_physics $T/stale "--cfg=i85_b1_st_stale" ;;
  base_stale_pp) run base_stale_pp $BASE/product_physics $T/base-stale "--cfg=i85_b1_st_stale" ;;
  cand_reg_runner) run cand_reg_runner $CAND/runner/headless $T "" ;;
  base_reg_runner) run base_reg_runner $BASE/runner/headless $T/base "" ;;
  cand_reg_re_carriers) run cand_reg_re_carriers $CAND/reporting/result_export $T "" --test retained_precision_carriers ;;
  cand_reg_witness) run cand_reg_witness $CAND/product_physics $T "" --lib witness_ -- --ignored --nocapture --test-threads=1 ;;
  base_reg_witness) run base_reg_witness $BASE/product_physics $T/base "" --lib witness_ -- --ignored --nocapture --test-threads=1 ;;
  cand_stale_witness) run cand_stale_witness $CAND/product_physics $T/stale "--cfg=i85_b1_st_stale" --lib witness_ -- --ignored --nocapture --test-threads=1 ;;
esac; done
