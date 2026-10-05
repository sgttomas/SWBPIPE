#!/bin/bash
# RV93: the cargo jobs after the mutants, one at a time.
set -u
WT=WT; S=$WT/scratch/rv93_u3_grant2_01
guard() { ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
job() { name=$1; dir=$2; shift 2; guard; echo "== $name start $(date -u +%FT%TZ)"
  ( cd $dir && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 "$@" > $S/logs/$name.log 2>&1 ); echo "== $name exit=$? $(date -u +%FT%TZ)"
  grep -E "^test result" $S/logs/$name.log | awk '{p+=$4; f+=$6; i+=$8} END {print "   passed="p" failed="f" ignored="i}'; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#-[0-9a-f]{16}##; s#[^ ]*/deps/##; s#^[^ ]*/(tests|src)/#\1/#' | sort > $S/logs/$1.outcomes; }
P=projects/chirality-piping/core
rm -f $S/logs/permits_pp_positive.log $S/logs/permits_runner.log
# 1. The probe again (input fallbacks; the distinct-id no-permit variant), as the permit log's positive control.
job probe_reg_2 $WT/rv93/probe/$P/product_physics RV93_OUT=$S/out_reg RV93_PERMIT_LOG=$S/logs/permits_pp_positive.log perl -e 'alarm shift; exec @ARGV' 1800 cargo test --locked --offline --lib --target-dir $WT/targets/rv93/probe zz_rv93_ -- --test-threads=1 --nocapture
echo "   positive-control permits: $(wc -l < $S/logs/permits_pp_positive.log 2>/dev/null || echo 0)"
# 2. runner/headless against the instrumented PP: every permit any runner test is granted.
job runner_probe $WT/rv93/probe/$P/runner/headless RV93_PERMIT_LOG=$S/logs/permits_runner.log perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --target-dir $WT/targets/rv93/runner_probe
echo "   runner permits: $(cat $S/logs/permits_runner.log 2>/dev/null | wc -l)"; outcomes runner_probe
# 3. runner/headless on the clean candidate and on base.
job runner_cand $WT/rv93/cand/$P/runner/headless perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --target-dir $WT/targets/rv93/runner_cand
outcomes runner_cand
job runner_base $WT/rv93/base/$P/runner/headless perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --target-dir $WT/targets/rv93/runner_base
outcomes runner_base
cmp $S/logs/runner_cand.outcomes $S/logs/runner_base.outcomes && echo "   runner outcomes: candidate == base"
cmp $S/logs/runner_cand.outcomes $S/logs/runner_probe.outcomes && echo "   runner outcomes: probe == candidate"
# 4. The five grant-2 tests in the Stale build (their ordinary-route branch).
job stale_u3g2 $WT/rv93/candsweep/$P/product_physics RUSTFLAGS=--cfg=rv93_stale perl -e 'alarm shift; exec @ARGV' 1800 cargo test --locked --offline --lib --target-dir $WT/targets/rv93_stale/cand u3g2_
# 5. Production (non-test) lib builds: warnings, base vs candidate.
for t in base cand; do job build_$t $WT/rv93/$t/$P/product_physics perl -e 'alarm shift; exec @ARGV' 1800 cargo build --locked --offline --lib --target-dir $WT/targets/rv93/build_$t
  grep -E '^warning|-->' $S/logs/build_$t.log | sed -E 's#-->.*/(core/[^:]*):([0-9]+):[0-9]+#--> \1:\2#' > $S/logs/build_$t.warnings; done
cmp $S/logs/build_base.warnings $S/logs/build_cand.warnings && echo "   production build warnings identical ($(grep -c '^warning' $S/logs/build_cand.warnings) lines)"
# 6. The actual Direct entry from a NON-test build (an example links PP without cfg(test)), candidate and base.
for t in cand base; do job nontest_$t $WT/rv93/${t}sweep/$P/product_physics perl -e 'alarm shift; exec @ARGV' 1800 cargo run --locked --offline --example rv93_direct --target-dir $WT/targets/rv93/sweep_$t
  grep RV93_NONTEST $S/logs/nontest_$t.log; done
echo "== done"
