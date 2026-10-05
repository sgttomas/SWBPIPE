#!/bin/bash
# RV93: one cargo job. usage: cg.sh <log-name> <tree: cand|base|mut|...> <target-subdir> [extra env assignments as ENV=... before --] -- <cargo args...>
# Runs in WT/rv93/<tree>/projects/chirality-piping/core/product_physics with --locked --offline,
# CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=2, RUSTFLAGS unset unless given, under a 3600 s alarm.
set -u
WT=WT
S=$WT/scratch/rv93_u3_grant2_01
name=$1; tree=$2; tgt=$3; shift 3
envs=()
while [ $# -gt 0 ] && [ "$1" != "--" ]; do envs+=("$1"); shift; done
shift
ps -p 5387 >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
dir=${PPDIR:-$WT/rv93/$tree/projects/chirality-piping/core/product_physics}
echo "== $name start $(date -u +%FT%TZ)" 
( cd $dir && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$WT/targets/rv93/$tgt ${envs[@]+"${envs[@]}"} perl -e 'alarm shift; exec @ARGV' 3600 cargo "$1" --locked --offline "${@:2}" > $S/logs/$name.log 2>&1 )
rc=$?
echo "== $name exit=$rc $(date -u +%FT%TZ)"
grep -E "^test result|panicked|^error" $S/logs/$name.log | head -40
exit $rc
