#!/bin/bash
# I65 U4 G5: candidate runs (the worktree's changed files overlaid on a copy of base).
set -u
T=WT
S=$T/scratch/i65_u4_g5_01; W=$T/f2a-memory/projects/chirality-piping/core; C=$S/cand/projects/chirality-piping/core
tag=${1:-cand}
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 2400 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
rm -rf $S/cand; mkdir -p $S/cand; cp -R $S/base/projects $S/cand/
for f in Cargo.toml build.rs src/build_identity.rs src/lib.rs src/retained_memory.rs src/retained_memory_law_tests.rs src/retained_product.rs src/retained_facade_tests.rs; do
  [ -f $W/product_physics/$f ] && cp $W/product_physics/$f $C/product_physics/$f
done
for f in $(cd $W/product_physics && ls src/*.rs tests/*.rs 2>/dev/null); do cmp -s $W/product_physics/$f $C/product_physics/$f || echo "DIFF-NOT-COPIED $f"; done
run ${tag}_sweep $C/product_physics/Cargo.toml $T/targets/i65-g5/cand-copy "I65_G5_SWEEP=$S/sweep_${tag}.tsv" "--test zz_i65_g5_fixture_sweep -- --ignored --nocapture"
run ${tag}_pp $C/product_physics/Cargo.toml $T/targets/i65-g5/cand-copy
run ${tag}_runner $C/runner/headless/Cargo.toml $T/targets/i65-g5/cand-runner
cmp $S/sweep_base.tsv $S/sweep_${tag}.tsv && echo "== sweep identical to base (8abb5274a9)"
diff <(sed -E 's/ \(.*//' $S/logs/base_pp.outcomes) <(sed -E 's/ \(.*//' $S/logs/${tag}_pp.outcomes) | head -60
diff $S/logs/base_runner.outcomes $S/logs/${tag}_runner.outcomes && echo "== runner outcomes identical"
