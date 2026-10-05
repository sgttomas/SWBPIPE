#!/bin/bash
# I65 U4 G6 addendum (ROOT 2(a)): controls, witnesses and mutants on the final code.
set -u
T=WT
S=$T/scratch/i65_u4_g6_01; W=$T/f2a-memory/projects/chirality-piping/core; C=$S/cand3/projects/chirality-piping/core
G=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g6_01/_run_records
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
for f in product_physics/src/retained_memory.rs product_physics/src/retained_memory_law_tests.rs product_physics/src/retained_memory_witness_tests.rs product_physics/tests/retained_memory_challenge.rs solver/frame_kernel/src/structural/retained_resource.rs; do cp $W/$f $C/$f; done
diff -rq -x target -x zz_i65_g5_fixture_sweep.rs $W $C && echo "== cand3 equals the worktree core tree"
run add_sweep $C/product_physics/Cargo.toml $T/targets/i65-g6/cand3 "I65_G5_SWEEP=$S/sweep_add.tsv" "--test zz_i65_g5_fixture_sweep -- --ignored --nocapture"
cmp $T/scratch/i65_u4_g5_01/sweep_base.tsv $S/sweep_add.tsv && echo "== sweep identical to base (8abb5274a9)"
run add_pp $C/product_physics/Cargo.toml $T/targets/i65-g6/cand3
echo "add_pp: $(grep -c ' ok$' $S/logs/add_pp.outcomes) ok, $(grep -c FAILED $S/logs/add_pp.outcomes) failed, $(grep -c ignored $S/logs/add_pp.outcomes) ignored"; grep FAILED $S/logs/add_pp.outcomes
run add_runner $C/runner/headless/Cargo.toml $T/targets/i65-g6/cand3-runner
diff $T/scratch/i65_u4_g5_01/logs/base_runner.outcomes $S/logs/add_runner.outcomes && echo "== runner outcomes identical"
$S/run_g6_witness.sh > $S/logs/run_g6_witness3.out 2>&1
grep -E '^== ' $S/logs/run_g6_witness3.out
rm -rf $S/mut; mkdir -p $S/mut && cp -R $S/cand3/projects $S/mut/
guard; echo "== mutants start $(date -u +%FT%TZ)"
python3 $G/mutants_g6.py $W $S/mut/projects/chirality-piping/core $T/targets/i65-g6/mut > $S/mutants_g6b.out.jsonl 2> $S/logs/mutants_g6b.err
echo "== mutants exit=$? $(date -u +%FT%TZ)"
python3 -c "
import json,collections
rows=[json.loads(l) for l in open('$S/mutants_g6b.out.jsonl')]
print(len(rows), sorted(collections.Counter((r.get('set'),r['result']) for r in rows).items()))
for r in rows:
    if r['result']!='KILLED': print(r.get('set'), r['mutant'], r['result'], r.get('equivalent'))
"
