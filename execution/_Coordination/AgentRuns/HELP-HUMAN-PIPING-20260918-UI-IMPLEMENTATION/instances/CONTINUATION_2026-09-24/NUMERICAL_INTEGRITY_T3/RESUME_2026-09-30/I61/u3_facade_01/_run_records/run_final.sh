#!/bin/bash
set -u
W=WT; S=$W/scratch/i61_u3_facade_01
C=$W/f2a-facade/projects/chirality-piping/core; A=$S/cand/projects/chirality-piping/core
for f in Cargo.toml src/lib.rs src/retained_memory.rs src/retained_product.rs src/retained_wire.rs src/retained_wire_tests.rs src/retained_facade_tests.rs; do cp $C/product_physics/$f $A/product_physics/$f; done
for f in Cargo.toml src/lib.rs src/retained_memory.rs src/retained_product.rs src/retained_wire.rs src/retained_wire_tests.rs src/retained_facade_tests.rs; do cmp $C/product_physics/$f $A/product_physics/$f || exit 7; done
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#^\(?[^ ]*/deps/([a-z0-9_]+)-[0-9a-f]+\)?#\1#' | sort > $S/logs/$1.outcomes; }
guard; ( cd $C/product_physics && env CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 1200 cargo build --locked --offline --lib --manifest-path $C/product_physics/Cargo.toml --target-dir $W/targets/i61-u3/product_physics > $S/logs/final_build.log 2>&1 ); echo "== final_build exit=$?"
grep -E "^warning: " $S/logs/final_build.log | sort > $S/logs/final_build.warnings
guard; echo "== final_pp start $(date -u +%FT%TZ)"; ( cd $C/product_physics && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 I61_U3_OUT=$S/out perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $C/product_physics/Cargo.toml --target-dir $W/targets/i61-u3/product_physics > $S/logs/final_pp.log 2>&1 ); echo "== final_pp exit=$? $(date -u +%FT%TZ)"; outcomes final_pp
guard; echo "== final_runner start $(date -u +%FT%TZ)"; ( cd $A/runner/headless && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $A/runner/headless/Cargo.toml --target-dir $W/targets/i61-u3/runner > $S/logs/final_runner.log 2>&1 ); echo "== final_runner exit=$? $(date -u +%FT%TZ)"; outcomes final_runner
guard; echo "== final_sweep start $(date -u +%FT%TZ)"; ( cd $A/product_physics && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 I61_U3_SWEEP=$S/sweep/sweep_final.tsv perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --manifest-path $A/product_physics/Cargo.toml --target-dir $W/targets/i61-u3/product_physics --test zz_i61_u3_fixture_sweep -- --ignored --nocapture > $S/logs/final_sweep.log 2>&1 ); echo "== final_sweep exit=$? $(date -u +%FT%TZ)"
cmp $S/sweep/sweep_base.tsv $S/sweep/sweep_final.tsv && echo "== sweep identical to base"
