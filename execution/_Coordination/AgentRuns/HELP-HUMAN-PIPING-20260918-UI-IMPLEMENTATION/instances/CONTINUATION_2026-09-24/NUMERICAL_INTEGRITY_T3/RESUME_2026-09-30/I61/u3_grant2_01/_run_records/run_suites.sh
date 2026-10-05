#!/bin/bash
# I61 U3 grant 2: suites. One cargo job at a time, --locked --offline, CARGO_BUILD_JOBS=4,
# RUST_TEST_THREADS=2, memory guard checked first, each under a 3600 s alarm.
set -u
T=WT; S=$T/scratch/i61_u3_grant2_01; W=$T/f2a-memory/projects/chirality-piping/core
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##; s#[^ ]*/deps/##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
count() { echo "$1: $(grep -c ' ok$' $S/logs/$1.outcomes) ok, $(grep -c FAILED $S/logs/$1.outcomes) failed, $(grep -c ignored $S/logs/$1.outcomes) ignored"; grep FAILED $S/logs/$1.outcomes; }
case "$1" in
  reg_pp) run reg_pp $W/product_physics/Cargo.toml $T/targets/i61-u3g2/reg; count reg_pp ;;
  stale_pp) run stale_pp $W/product_physics/Cargo.toml $T/targets/i61-u3g2/stale "RUSTFLAGS=--cfg=i61_u3g2_stale"; count stale_pp ;;
  reg_runner) run reg_runner $W/runner/headless/Cargo.toml $T/targets/i61-u3g2/reg-runner; count reg_runner ;;
esac
