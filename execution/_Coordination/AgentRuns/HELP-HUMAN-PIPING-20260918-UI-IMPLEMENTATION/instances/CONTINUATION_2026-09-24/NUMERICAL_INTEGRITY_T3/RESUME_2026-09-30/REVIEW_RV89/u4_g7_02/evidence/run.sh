#!/bin/bash
# RV89 G7: one cargo job at a time; memory guard checked before each run.
# usage: run.sh <label> <manifest> <target> [extra cargo args...]   (env passes through)
set -u
T=WT
G=$T/scratch/rv89_u4_g7_01/u7
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
label=$1; manifest=$2; target=$3; shift 3
echo "== $label start $(date +%T)"
( cd $(dirname $manifest) && env TMPDIR=$G/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=${RUST_TEST_THREADS:-2} GIT_OPTIONAL_LOCKS=0 \
   perl -e 'alarm shift; exec @ARGV' 3000 cargo test --locked --offline --no-fail-fast --manifest-path $manifest --target-dir $target "$@" > $G/logs/$label.log 2>&1 )
rc=$?
echo "== $label exit=$rc $(date +%T)"
awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $G/logs/$label.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##; s#^.*/deps/##' | sort > $G/logs/$label.outcomes
grep -E "^test result:" $G/logs/$label.log > $G/logs/$label.summary
