#!/bin/bash
set -u
WT/scratch/i65_u4_g5_01/run_cand.sh cand2
W=WT/f2a-memory/projects/chirality-piping/core/product_physics; C=WT/scratch/i65_u4_g5_01/mut/projects/chirality-piping/core/product_physics
for f in Cargo.toml build.rs src/build_identity.rs src/lib.rs src/retained_memory.rs src/retained_memory_law_tests.rs src/retained_product.rs; do cp $W/$f $C/$f; done
echo "== mutants start $(date -u +%FT%TZ)"
python3 WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g5_01/_run_records/mutants_g5.py $W $C WT/targets/i65-g5/mut > WT/scratch/i65_u4_g5_01/mutants2.out.jsonl 2> WT/scratch/i65_u4_g5_01/logs/mutants2.err
echo "== mutants exit=$? $(date -u +%FT%TZ)"
