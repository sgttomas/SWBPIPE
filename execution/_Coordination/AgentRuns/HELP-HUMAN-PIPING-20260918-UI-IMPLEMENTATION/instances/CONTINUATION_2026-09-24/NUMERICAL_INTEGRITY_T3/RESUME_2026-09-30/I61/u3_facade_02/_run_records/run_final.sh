#!/bin/bash
set -u
W=WT; S=$W/scratch/i61_u3_facade_02; S1=$W/scratch/i61_u3_facade_01
C=$W/f2a-facade/projects/chirality-piping/core; A=$S/cand/projects/chirality-piping/core; ST=$S/stub/projects/chirality-piping/core/product_physics
FILES="Cargo.toml src/lib.rs src/retained_memory.rs src/retained_product.rs src/retained_wire.rs src/retained_wire_tests.rs src/retained_facade_tests.rs"
for f in $FILES; do cp $C/product_physics/$f $A/product_physics/$f; cmp $C/product_physics/$f $A/product_physics/$f || exit 7; done
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
guard; ( cd $C/product_physics && env CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 1200 cargo build --locked --offline --lib --manifest-path $C/product_physics/Cargo.toml --target-dir $W/targets/i61-u3/product_physics > $S/logs/final_build.log 2>&1 ); echo "== final_build exit=$?"
grep -E "^warning: " $S/logs/final_build.log | sort > $S/logs/final_build.warnings
run final_pp $C/product_physics/Cargo.toml $W/targets/i61-u3/product_physics "I61_U3_OUT=$S/out"
run final_runner $C/runner/headless/Cargo.toml $W/targets/i61-u3/runner
guard; echo "== final_sweep start $(date -u +%FT%TZ)"; ( cd $A/product_physics && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 I61_U3_SWEEP=$S/sweep_final.tsv perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path $A/product_physics/Cargo.toml --target-dir $W/targets/i61-u3/product_physics --test zz_i61_u3_fixture_sweep -- --ignored --nocapture > $S/logs/final_sweep.log 2>&1 ); echo "== final_sweep exit=$? $(date -u +%FT%TZ)"
cmp $S1/sweep/sweep_base.tsv $S/sweep_final.tsv && echo "== sweep identical to base (b54caba7ab)"
# The disposable stub: refresh to the final candidate, re-apply, rerun.
for f in $FILES src/s11g_tests.rs; do cp $C/product_physics/$f $ST/$f; done
python3 $S/stub_patch.py $ST $S/stub_test_tail.rs
guard; ( cd $ST && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 I61_U3_OUT=$S/out perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path $ST/Cargo.toml --target-dir $W/targets/i61-u3/stub --lib retained_facade_tests -- --nocapture > $S/logs/stub_e2e.log 2>&1 ); echo "== stub exit=$? $(date -u +%FT%TZ)"
