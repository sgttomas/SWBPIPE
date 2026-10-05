#!/bin/bash
# I66 U7 slice F: one cargo test run. Args: label manifest target-dir [env-assignment|-] [extra cargo args...]
# One cargo job at a time (callers chain them), --locked --offline, CARGO_BUILD_JOBS=4,
# RUST_TEST_THREADS=2, the memory guard checked first, a 3600 s alarm.
set -u
WT=WT; S=$WT/scratch/i66_u7r
label=$1; manifest=$2; target=$3; extra_env=${4:--}; shift 4
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
[ "$extra_env" = "-" ] && extra_env=""
echo "== $label start $(date -u +%FT%TZ)"
( cd $(dirname $manifest) && env -u RUSTFLAGS TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 $extra_env perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $manifest --target-dir $target "$@" > $S/logs/$label.log 2>&1 )
rc=$?; echo "== $label exit=$rc $(date -u +%FT%TZ)"
awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$label.log | sed -E 's#-[0-9a-f]{16}\)#)#' | sort > $S/logs/$label.outcomes
echo "$label: $(grep -c ' ok$' $S/logs/$label.outcomes) ok, $(grep -c 'FAILED$' $S/logs/$label.outcomes) failed, $(grep -c 'ignored$' $S/logs/$label.outcomes) ignored"
