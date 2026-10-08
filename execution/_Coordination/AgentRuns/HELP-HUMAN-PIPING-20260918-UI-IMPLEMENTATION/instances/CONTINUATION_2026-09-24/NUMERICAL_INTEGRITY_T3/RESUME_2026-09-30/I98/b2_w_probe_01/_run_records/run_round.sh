#!/bin/bash
# I98 B2-W: one probe round (three cargo jobs through WT/tools/t3_cargo.sh, each waiting first
# while more than one /usr/bin/lockf process runs, so an already-queued job goes first).
# Usage: WT=<t3 root> run_round.sh <round number> [set...]   sets: main, cb1, r7, r7b, extra (I98_EXTRA_NAME, I98_EXTRA_FILES, I98_EXTRA_BYPASS)
set -u
: "${WT:?WT}"
S=$WT/scratch/i98_b2w
N=$1; shift
SETS=${*:-main cb1 r7}
cd "$S/arch/projects/chirality-piping/core/product_physics" || exit 2
run() { # <log name> <test name> [env...]
  local log=$1 test=$2; shift 2
  while [ "$(pgrep -f '^/usr/bin/lockf' | wc -l)" -gt 1 ]; do sleep 15; done
  env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR="$S/tmp" CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=1 \
    CARGO_TARGET_DIR="$WT/targets/i98-b2w" I98_OUT="$S/out" "$@" \
    "$WT/tools/t3_cargo.sh" test --locked --offline --lib -- --ignored --nocapture --test-threads=1 --exact \
    "retained_memory::zz_i98_probe::$test" > "$S/logs/round${N}_${log}.log" 2>&1
  echo "round${N}_${log} rc=$?"
}
I=$S/inputs
for set in $SETS; do
  case $set in
    main) run main zz_i98_files I98_FILES="$I/milestone.json:$I/cb2_case_c.json:$I/cb3_a.json:$I/cb3_v1_b.json:$I/cb3_v1_ab.json:$I/i86_case_a.json" ;;
    cb1)  run cb1 zz_i98_files I98_BYPASS=1 I98_FILES="$I/cb1_ab.json" ;;
    r7)   run r7 zz_i98_plain I98_DUMP_PLAIN=1 I98_FILES="$I/r7_cb1.json:$I/r7_cb2.json:$I/r7_cb3_v1.json" ;;
    r7b)  run r7b zz_i98_plain I98_DUMP_PLAIN=1 I98_FILES="$I/r7_cb1_halfb.json" ;;
    extra) run "${I98_EXTRA_NAME:-extra}" zz_i98_files ${I98_EXTRA_BYPASS:+I98_BYPASS=1} I98_FILES="${I98_EXTRA_FILES:?I98_EXTRA_FILES}" ;;
  esac
done
