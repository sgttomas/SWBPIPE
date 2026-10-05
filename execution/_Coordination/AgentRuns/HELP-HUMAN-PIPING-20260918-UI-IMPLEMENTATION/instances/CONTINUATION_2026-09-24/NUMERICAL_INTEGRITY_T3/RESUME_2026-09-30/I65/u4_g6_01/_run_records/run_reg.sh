#!/bin/bash
# I65 U4 G6: the prepared registration change, applied ONLY to a scratch copy (reg/), tested:
# the fixture sweep (against base), the PP suite (against the unregistered G6 candidate).
set -u
T=WT
S=$T/scratch/i65_u4_g6_01; G5=$T/scratch/i65_u4_g5_01; C=$S/reg/projects/chirality-piping/core
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
cp $G5/zz_i65_g5_fixture_sweep.rs $C/product_physics/tests/
run reg_sweep $C/product_physics/Cargo.toml $T/targets/i65-g6/reg "I65_G5_SWEEP=$S/sweep_reg.tsv" "--test zz_i65_g5_fixture_sweep -- --ignored --nocapture"
cmp $G5/sweep_base.tsv $S/sweep_reg.tsv && echo "== sweep identical to base" || echo "== sweep differs from base (expected for in-domain Direct inputs)"
run reg_pp $C/product_physics/Cargo.toml $T/targets/i65-g6/reg
echo "reg_pp: $(grep -c ' ok$' $S/logs/reg_pp.outcomes) ok, $(grep -c 'FAILED' $S/logs/reg_pp.outcomes) failed, $(grep -c 'ignored' $S/logs/reg_pp.outcomes) ignored"
grep FAILED $S/logs/reg_pp.outcomes
