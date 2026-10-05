#!/bin/bash
# RV95: one cargo test run. Args: label manifest target-dir [env-assignments|-] [extra cargo args...]
# One cargo job at a time (callers chain them), --locked --offline, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=2.
set -u
T3=WT
S=$T3/scratch/rv95_u9_01
label=$1; manifest=$2; target=$3; extra_env=${4:--}; shift 4
ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
[ "$extra_env" = "-" ] && extra_env=""
mkdir -p $S/logs $S/tmp
echo "== $label start $(date -u +%FT%TZ)"
( cd $(dirname $manifest) && env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 $extra_env perl -e 'alarm shift; exec @ARGV' 5400 cargo test --locked --offline --no-fail-fast --manifest-path $manifest --target-dir $target "$@" > $S/logs/$label.log 2>&1 )
rc=$?; echo "== $label exit=$rc $(date -u +%FT%TZ)"
awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$label.log | sed -E 's#-[0-9a-f]{16}\)#)#' | sort > $S/logs/$label.outcomes
echo "$label: $(grep -c ' ok$' $S/logs/$label.outcomes) ok, $(grep -c 'FAILED$' $S/logs/$label.outcomes) failed, $(grep -c 'ignored$' $S/logs/$label.outcomes) ignored"
