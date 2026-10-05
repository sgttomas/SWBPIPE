#!/bin/bash
# I65 U4 G5: base (8abb5274a9) reference runs, one cargo job at a time.
set -u
T=WT
S=$T/scratch/i65_u4_g5_01; B=$S/base/projects/chirality-piping/core
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 2400 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
cp $S/zz_i65_g5_fixture_sweep.rs $B/product_physics/tests/
run base_sweep $B/product_physics/Cargo.toml $T/targets/i65-g5/base "I65_G5_SWEEP=$S/sweep_base.tsv" "--test zz_i65_g5_fixture_sweep -- --ignored --nocapture"
run base_pp $B/product_physics/Cargo.toml $T/targets/i65-g5/base
run base_runner $B/runner/headless/Cargo.toml $T/targets/i65-g5/base-runner
