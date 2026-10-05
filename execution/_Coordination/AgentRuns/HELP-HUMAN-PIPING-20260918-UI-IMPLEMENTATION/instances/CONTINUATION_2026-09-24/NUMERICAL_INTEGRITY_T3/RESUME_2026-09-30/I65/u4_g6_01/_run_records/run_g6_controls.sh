#!/bin/bash
# I65 U4 G5 part 2: controls. The candidate copy is the part-1 commit (1e323058f3, git archive)
# with part 2's seven files overlaid; the base is 8abb5274a9 (part 1's base). One cargo job at a
# time, --locked --offline, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=2, memory guard checked first.
set -u
T=WT
G5=$T/scratch/i65_u4_g5_01; S=$T/scratch/i65_u4_g6_01; W=$T/f2a-memory/projects/chirality-piping/core
B=$G5/base/projects/chirality-piping/core; C=$S/cand3/projects/chirality-piping/core
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
PART2="loads/stress_recovery/src/elastic_extrema.rs product_physics/src/retained_memory.rs product_physics/src/retained_memory_law_tests.rs solver/frame_kernel/src/structural.rs product_physics/src/retained_memory_witness_tests.rs product_physics/tests/retained_memory_challenge.rs solver/frame_kernel/src/structural/retained_resource.rs"
rm -rf $S/cand3; mkdir -p $S/cand3; cp -R $G5/snap_1e32/projects $S/cand3/; cp -R $G5/base/projects/chirality-piping/apps $S/cand3/projects/chirality-piping/
for f in $PART2; do mkdir -p $(dirname $C/$f); cp $W/$f $C/$f; done
diff -rq -x target $W $C && echo "== cand3 equals the worktree core tree"
cp $G5/zz_i65_g5_fixture_sweep.rs $C/product_physics/tests/
run g6_sweep $C/product_physics/Cargo.toml $T/targets/i65-g6/cand3 "I65_G5_SWEEP=$S/sweep_g6.tsv" "--test zz_i65_g5_fixture_sweep -- --ignored --nocapture"
cmp $G5/sweep_base.tsv $S/sweep_g6.tsv && echo "== sweep identical to base (8abb5274a9)"
run g6_pp $C/product_physics/Cargo.toml $T/targets/i65-g6/cand3
diff <(sed -E 's/ \(.*//' $G5/logs/base_pp.outcomes) <(sed -E 's/ \(.*//' $S/logs/g6_pp.outcomes) | head -80
run g6_runner $C/runner/headless/Cargo.toml $T/targets/i65-g6/cand3-runner
diff $G5/logs/base_runner.outcomes $S/logs/g6_runner.outcomes && echo "== runner outcomes identical"
run g6_fk $C/solver/frame_kernel/Cargo.toml $T/targets/i65-g6/cand3-fk
diff $G5/logs/base_fk.outcomes $S/logs/g6_fk.outcomes && echo "== FK outcomes identical"
run g6_sr $C/loads/stress_recovery/Cargo.toml $T/targets/i65-g6/cand3-sr
diff $G5/logs/base_sr.outcomes $S/logs/g6_sr.outcomes && echo "== SR outcomes identical"
for l in g6_fk g6_sr g6_pp g6_runner; do echo "$l: $(grep -c " ok$" $S/logs/$l.outcomes) ok, $(grep -c FAILED $S/logs/$l.outcomes) failed, $(grep -c ignored $S/logs/$l.outcomes) ignored"; done
