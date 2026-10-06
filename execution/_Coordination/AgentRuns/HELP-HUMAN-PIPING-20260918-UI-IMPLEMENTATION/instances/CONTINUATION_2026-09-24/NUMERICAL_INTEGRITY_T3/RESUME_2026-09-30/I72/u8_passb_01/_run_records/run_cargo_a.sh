#!/bin/bash
# I72 U8-4 copy of I65 u4_g7_06's run_cargo_a.sh. Path retarget only: logs and TMPDIR under WT/scratch/i72_u8/,
# cargo targets under WT/targets/i72-u8/. The jobs, flags, timeouts and outcome extraction are I65's.
# I65 U4 G7 Pass A, item 5: the registered build of the integrated tree (work/) and NUM f172f86abe
# unregistered (num/). One cargo job at a time; --locked --offline; CARGO_BUILD_JOBS=4,
# RUST_TEST_THREADS=2 (1 for the witnesses); memguard checked before each job; TMPDIR in scratch.
# Pass A ran with targets work/ and work-runner/ (before the per-tag names); the jobs are the same.
set -u
T=${I65_T:?}; G7=$T/scratch/i72_u8; TAG=${1:-a}; WK=${2:-$G7/work}
C=$WK/projects/chirality-piping/core; N=$G7/num/projects/chirality-piping/core
export TMPDIR=$G7/tmp
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $G7/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $G7/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${4:-} > $G7/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
count() { echo "$1: $(grep -c ' ok$' $G7/logs/$1.outcomes) passed, $(grep -c FAILED $G7/logs/$1.outcomes) failed, $(grep -c ignored $G7/logs/$1.outcomes) ignored"; grep FAILED $G7/logs/$1.outcomes; }
run ${TAG}_pp $C/product_physics/Cargo.toml $T/targets/i72-u8/work-$TAG; count ${TAG}_pp
guard
( cd $C/product_physics && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=1 perl -e 'alarm shift; exec @ARGV' 1800 cargo test --locked --offline --target-dir $T/targets/i72-u8/work-$TAG --test retained_memory_challenge -- --nocapture > $G7/logs/${TAG}_challenge.log 2>&1 )
echo "== challenge exit=$?"; grep -E '^I65_G5_CHALLENGE |^test result' $G7/logs/${TAG}_challenge.log
run ${TAG}_runner $C/runner/headless/Cargo.toml $T/targets/i72-u8/work-runner-$TAG; count ${TAG}_runner
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2_deep_milestone_publishes witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  guard
  ( cd $C/product_physics && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=1 perl -e 'alarm shift; exec @ARGV' 1800 cargo test --locked --offline --lib --target-dir $T/targets/i72-u8/work-$TAG $w -- --ignored --nocapture --test-threads=1 > $G7/logs/${TAG}_$w.log 2>&1 )
  echo "== $w exit=$?"; grep -E '^I65_G5_WITNESS|^test result|panicked|has overflowed|SIGABRT|SIGSEGV' $G7/logs/${TAG}_$w.log | cut -c1-220
done
if [ "$TAG" = a ]; then
  run num_pp $N/product_physics/Cargo.toml $T/targets/i72-u8/num; count num_pp
  run num_runner $N/runner/headless/Cargo.toml $T/targets/i72-u8/num-runner; count num_runner
fi
echo "== done $(date -u +%FT%TZ)"
