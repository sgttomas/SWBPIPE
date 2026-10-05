#!/bin/bash
# I65 U4 G6 repair (RV87 SF-1..SF-3, RV89 S-1..S-3): controls, witnesses, the registered scratch
# copy (with registration.g6r.diff) and the mutant run, on the repaired code. One cargo job at a
# time, --locked --offline, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=2, memory guard checked first.
set -u
T=WT
S=$T/scratch/i65_u4_g6_01; G5=$T/scratch/i65_u4_g5_01; W=$T/f2a-memory/projects/chirality-piping/core
C=$S/cand3/projects/chirality-piping/core; RG=$S/reg/projects/chirality-piping/core
G=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g6_01/_run_records
export TMPDIR=$S/tmp
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
count() { echo "$1: $(grep -c ' ok$' $S/logs/$1.outcomes) ok, $(grep -c FAILED $S/logs/$1.outcomes) failed, $(grep -c ignored $S/logs/$1.outcomes) ignored"; grep FAILED $S/logs/$1.outcomes; }
diff -rq -x target -x zz_i65_g5_fixture_sweep.rs $W $C && echo "== cand3 equals the worktree core tree"
# 1. the unregistered candidate
run r_sweep $C/product_physics/Cargo.toml $T/targets/i65-g6/cand3 "I65_G5_SWEEP=$S/sweep_g6r.tsv" "--test zz_i65_g5_fixture_sweep -- --ignored --nocapture"
cmp $G5/sweep_base.tsv $S/sweep_g6r.tsv && echo "== unregistered sweep identical to base (8abb5274a9)"
run r_pp $C/product_physics/Cargo.toml $T/targets/i65-g6/cand3; count r_pp
diff <(sed -E 's/ \(.*//' $S/logs/add_pp.outcomes) <(sed -E 's/ \(.*//' $S/logs/r_pp.outcomes); echo "== r_pp vs G6 (add_pp) outcomes diff above"
run r_runner $C/runner/headless/Cargo.toml $T/targets/i65-g6/cand3-runner; count r_runner
diff $G5/logs/base_runner.outcomes $S/logs/r_runner.outcomes && echo "== runner outcomes identical to base"
# 2. the witnesses, both builds (the worktree itself)
$S/run_g6_witness.sh > $S/logs/run_g6r_witness.out 2>&1; grep -E '^== |I65_G5_PROFILE' $S/logs/run_g6r_witness.out
# 3. the registered scratch copy (registration.g6r.diff applied)
diff -rq -x target -x zz_i65_g5_fixture_sweep.rs $C $RG | sed "s#$T#WT#g"
cp $G5/zz_i65_g5_fixture_sweep.rs $RG/product_physics/tests/
run r_reg_sweep $RG/product_physics/Cargo.toml $T/targets/i65-g6/reg "I65_G5_SWEEP=$S/sweep_reg_g6r.tsv" "--test zz_i65_g5_fixture_sweep -- --ignored --nocapture"
cmp $S/sweep_reg2.tsv $S/sweep_reg_g6r.tsv && echo "== registered sweep identical to G6's registered sweep (sweep_reg2)"
run r_reg_pp $RG/product_physics/Cargo.toml $T/targets/i65-g6/reg; count r_reg_pp
run r_reg_challenge $RG/product_physics/Cargo.toml $T/targets/i65-g6/reg "" "--test retained_memory_challenge -- --nocapture"
grep -E '^I65_G5_CHALLENGE |test result' $S/logs/r_reg_challenge.log
run r_reg_runner $RG/runner/headless/Cargo.toml $T/targets/i65-g6/reg-runner; count r_reg_runner
diff $G5/logs/base_runner.outcomes $S/logs/r_reg_runner.outcomes && echo "== registered runner outcomes identical to base"
# 4. mutants: every set, on the repaired candidate. Record note: this full run was stopped after
# 12 mutants (all P1, all KILLED; mutants_g6r_full_partial.out.jsonl) and replaced by the run of the
# sets the repair can affect, RV, P2, G6 and G6R (controls/mutants_g6r.out.jsonl):
#   python3 $G/mutants_g6.py $W $S/mut/projects/chirality-piping/core $T/targets/i65-g6/mut G6R,G6,P2,RV
rm -rf $S/mut; mkdir -p $S/mut && cp -R $S/cand3/projects $S/mut/
guard; echo "== mutants start $(date -u +%FT%TZ)"
python3 $G/mutants_g6.py $W $S/mut/projects/chirality-piping/core $T/targets/i65-g6/mut > $S/mutants_g6r.out.jsonl 2> $S/logs/mutants_g6r.err
echo "== mutants exit=$? $(date -u +%FT%TZ)"
python3 -c "
import json,collections
rows=[json.loads(l) for l in open('$S/mutants_g6r.out.jsonl')]
print(len(rows), sorted(collections.Counter((r.get('set'),r['result']) for r in rows).items()))
for r in rows:
    if r['result']!='KILLED': print(r.get('set'), r['mutant'], r['result'], r.get('equivalent'))
"
echo "== done $(date -u +%FT%TZ)"
