#!/bin/bash
# I67 U7 slice F part 2: one cargo test run, as I66's cargo_run.sh. Args: label manifest target-dir [extra cargo args...]
set -u
WT=WT; S=$WT/scratch/i67_u6d/r9
label=$1; manifest=$2; target=$3; shift 3
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
echo "== $label start $(date -u +%FT%TZ)"
( cd $(dirname $manifest) && env -u RUSTFLAGS TMPDIR=$WT/scratch/i67_u6d/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $manifest --target-dir $target "$@" > $S/logs/$label.log 2>&1 )
rc=$?; echo "== $label exit=$rc $(date -u +%FT%TZ)"
awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$label.log | sed -E 's#-[0-9a-f]{16}\)#)#' | sort > $S/logs/$label.outcomes
echo "$label: $(grep -c ' ok$' $S/logs/$label.outcomes) ok, $(grep -c 'FAILED$' $S/logs/$label.outcomes) failed, $(grep -c 'ignored$' $S/logs/$label.outcomes) ignored"
