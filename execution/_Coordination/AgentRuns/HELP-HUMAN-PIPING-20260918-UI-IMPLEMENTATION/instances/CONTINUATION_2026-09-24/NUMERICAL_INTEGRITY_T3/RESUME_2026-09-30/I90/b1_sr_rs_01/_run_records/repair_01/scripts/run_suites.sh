#!/bin/bash
# I90 B1-SR-RS: suites, base (I1 262bd687f0 archive, P only) and candidate (WT/b1-r). One cargo job at a
# time through WT/tools/t3_cargo.sh; --locked --offline; CARGO_BUILD_JOBS=4; RUST_TEST_THREADS=2;
# RUSTFLAGS unset (registered); TMPDIR in scratch. After I85's run_suites.sh.
set -u
WT=WT
S=$WT/scratch/i90_b1_sr_rs; L=$S/logs; T=$WT/targets/i90-b1-sr-rs
BASE=$S/base/projects/chirality-piping/core; CAND=$WT/b1-r/projects/chirality-piping/core
outcomes() { awk '/^ *Running /{t=($2=="unittests")?$3:$2} /^ *Doc-tests /{t="doctests:"$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ - should panic/,""); sub(/ \(line [0-9]+\)/,""); print t" :: "$0}' $L/$1.log | sed -E 's#\(?[^ ]*/deps/##; s#-[0-9a-f]{16}\)?##' | sort > $L/$1.outcomes; }
count() { echo "$1: $(grep -c ' ok$' $L/$1.outcomes) ok, $(grep -c ' FAILED$' $L/$1.outcomes) failed, $(grep -c ' ignored' $L/$1.outcomes) ignored"; grep ' FAILED$' $L/$1.outcomes; }
run() { # name crate_dir target_dir [extra cargo args...]
  local name=$1 dir=$2 target=$3; shift 3
  pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $name start $(date -u +%FT%TZ)"
  ( cd $dir && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
    TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$target $WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast "$@" > $L/$name.log 2>&1 )
  echo "== $name exit=$? $(date -u +%FT%TZ)"; outcomes $name; count $name
}
pins() { # side: the pin tests' own output variables write the successor documents
  local side=$1 dir=$2 target=$3; rm -rf $S/pins/$side; mkdir -p $S/pins/$side
  ( export I61_U3_OUT=$S/pins/$side I61_U3G2_OUT=$S/pins/$side I68_U8_OUT=$S/pins/$side
    run ${side}_pins $dir $target --lib -- --exact \
      retained_facade_tests::u3_permitted_path_publishes_the_pinned_successor \
      retained_facade_tests::u3g2_direct_entry_publishes_the_pinned_successor \
      retained_facade_tests::u8_l0_isolated_node_publishes_pinned_successor )
}
for s in "$@"; do case "$s" in
  base_re) run base_re $BASE/reporting/result_export $T/base -- --nocapture ;;
  cand_re) run cand_re $CAND/reporting/result_export $T -- --nocapture ;;
  base_pp) run base_pp $BASE/product_physics $T/base ;;
  cand_pp) run cand_pp $CAND/product_physics $T ;;
  base_headless) run base_headless $BASE/runner/headless $T/base ;;
  cand_headless) run cand_headless $CAND/runner/headless $T ;;
  base_pins) pins base $BASE/product_physics $T/base ;;
  cand_pins) pins cand $CAND/product_physics $T ;;
  r1_re) run r1_re $CAND/reporting/result_export $T -- --nocapture ;;
  r1_pins) pins r1 $CAND/product_physics $T ;;
  *) echo "unknown $s" ;;
esac; done
