#!/bin/bash
# RV93 addendum 01: cargo jobs on the final memory head (7f07a2f7b4), one at a time.
set -u
WT=WT; S=$WT/scratch/rv93_u3_grant2_01; X=$S/ext
P=projects/chirality-piping/core
guard() { ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
job() { name=$1; dir=$2; shift 2; guard; echo "== $name start $(date -u +%FT%TZ)"
  ( cd $dir && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 "$@" > $X/logs/$name.log 2>&1 ); echo "== $name exit=$? $(date -u +%FT%TZ)"; }
tests() { awk '/^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print}' $X/logs/$1.log | sort > $X/logs/$1.tests; echo "   $1: ok $(grep -c ' ok$' $X/logs/$1.tests) failed $(grep -c FAILED $X/logs/$1.tests) ignored $(grep -c ignored $X/logs/$1.tests)"; grep FAILED $X/logs/$1.tests | sed 's/^/   /'; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $X/logs/$1.log | sed -E 's#-[0-9a-f]{16}##; s#[^ ]*/deps/##; s#^[^ ]*/(tests|src)/#\1/#' | sort > $X/logs/$1.outcomes; }
# 1. sweeps (registered, Stale) on the head + sweep module
job sweep_head_reg $WT/rv93/headsweep/$P/product_physics RV93_SWEEP=$X/sweep_head_reg.tsv perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --lib --target-dir $WT/targets/rv93/headsweep zz_rv93_route_sweep -- --ignored --nocapture
job sweep_head_stale $WT/rv93/headsweep/$P/product_physics RV93_SWEEP=$X/sweep_head_stale.tsv RUSTFLAGS=--cfg=rv93_stale perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --lib --target-dir $WT/targets/rv93_stale/headsweep zz_rv93_route_sweep -- --ignored --nocapture
for b in reg stale; do cmp $X/sweep_head_$b.tsv $S/sweep/cand_$b.tsv && echo "   sweep $b: head == grant 2 == base (byte-identical)" || echo "   sweep $b: DIFFERS"; done
# 2. PP all targets, registered and Stale (head, clean copy)
job pp_head_reg $WT/rv93/head/$P/product_physics perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --all-targets --no-fail-fast --target-dir $WT/targets/rv93/head
tests pp_head_reg
job pp_head_stale $WT/rv93/head/$P/product_physics RUSTFLAGS=--cfg=rv93_stale perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --all-targets --no-fail-fast --target-dir $WT/targets/rv93_stale/head
tests pp_head_stale
cmp -s $X/logs/pp_head_reg.tests $X/logs/pp_head_stale.tests && echo "   PP registered == Stale outcomes"
# 3. the in-build profile record (T17_V4's effect on the maxima)
job profile_head $WT/rv93/head/$P/product_physics perl -e 'alarm shift; exec @ARGV' 1800 cargo test --locked --offline --lib --target-dir $WT/targets/rv93/head retained_memory::law_tests::profile_in_build_record -- --nocapture
grep -h "I65_G5_PROFILE" $X/logs/profile_head.log | sed 's/^test [^ ]* \.\.\. //'
# 4. the actual Direct entry from a non-test build of the head
job nontest_head $WT/rv93/headsweep/$P/product_physics perl -e 'alarm shift; exec @ARGV' 1800 cargo run --locked --offline --example rv93_direct --target-dir $WT/targets/rv93/headsweep
grep -h RV93_NONTEST $X/logs/nontest_head.log
# 5. runner/headless (registered) on the head
job runner_head $WT/rv93/head/$P/runner/headless perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --target-dir $WT/targets/rv93/runner_head
outcomes runner_head; echo "   runner: ok $(grep -c ' ok$' $X/logs/runner_head.outcomes) failed $(grep -c FAILED $X/logs/runner_head.outcomes)"
cmp -s $X/logs/runner_head.outcomes $S/logs/runner_cand.outcomes && echo "   runner outcomes == grant 2's" || diff $S/logs/runner_cand.outcomes $X/logs/runner_head.outcomes | head
echo "== done"
