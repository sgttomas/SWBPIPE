#!/bin/bash
# I65 U4 G5 part 2: the S1 witnesses, one process each (an overflow aborts its process only).
set -u
T=WT
P=$T/f2a-memory/projects/chirality-piping/core/product_physics; S=$T/scratch/i65_u4_g5_01
profile=${1:-test}
for w in witness_w1_milestone witness_w2_cap_maximal witness_w2b_cap_maximal_solvable witness_w3_exact_selected witness_w4_preparation_refusal witness_w6_force_scaled witness_w7_fault_fallbacks witness_headroom_w1_at_one_mebibyte; do
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  extra=""; [ "$profile" = release ] && extra="--release"
  ( cd $P && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=1 perl -e 'alarm shift; exec @ARGV' 1800 cargo test --locked --offline --lib $extra --target-dir $T/targets/i65-g5/cand $w -- --ignored --nocapture --test-threads=1 > $S/logs/$w.$profile.log 2>&1 )
  code=$?
  echo "== $w profile=$profile exit=$code"
  grep -E '^I65_G5_WITNESS|^test result|panicked|has overflowed|SIGABRT|SIGSEGV' $S/logs/$w.$profile.log | cut -c1-220
done
