#!/bin/bash
# I66 U6 repairs: one result_export cargo test run. Args: label manifest target-dir [extra cargo args...]
set -u
WT=WT; S=$WT/scratch/i66_u6_postf_02
label=$1; manifest=$2; target=$3; shift 3
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
echo "== $label start $(date -u +%FT%TZ)"
( cd $(dirname $manifest) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${I66_ENV:-} perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $manifest --target-dir $target "$@" > $S/logs/$label.log 2>&1 )
rc=$?; echo "== $label exit=$rc $(date -u +%FT%TZ)"
awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$label.log | sed 's#-[0-9a-f]\{16\})#)#' | sort > $S/logs/$label.outcomes
grep -E "^test result:" $S/logs/$label.log | awk '{p+=$4; f+=$6; i+=$8} END {print "passed="p" failed="f" ignored="i}'
