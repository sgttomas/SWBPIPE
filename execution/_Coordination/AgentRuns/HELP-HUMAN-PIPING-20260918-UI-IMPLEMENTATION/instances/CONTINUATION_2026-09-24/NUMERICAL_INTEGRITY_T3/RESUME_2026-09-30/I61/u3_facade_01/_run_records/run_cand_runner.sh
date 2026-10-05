#!/bin/bash
set -u
W=WT; S=$W/scratch/i61_u3_facade_01
M=$S/cand/projects/chirality-piping/core/runner/headless/Cargo.toml
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
echo "== cand_runner start $(date -u +%FT%TZ)"
( cd $(dirname $M) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $M --target-dir $W/targets/i61-u3/runner > $S/logs/cand_runner.log 2>&1 ); echo "== cand_runner exit=$? $(date -u +%FT%TZ)"
awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/cand_runner.log | sort > $S/logs/cand_runner.outcomes
