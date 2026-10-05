#!/bin/bash
T=WT
S=$T/scratch/rv89_u4_g5; R=$T/rv89; TG=$T/targets/rv89
PPDIR=$R/cand/projects/chirality-piping/core/product_physics
cp $S/rv89_probe_tests.rs $PPDIR/src/rv89_probe_tests.rs
grep -q rv89_probe $PPDIR/src/retained_memory.rs || printf '\n#[cfg(test)]\n#[path = "rv89_probe_tests.rs"]\nmod rv89_probe;\n' >> $PPDIR/src/retained_memory.rs
$S/run.sh cand_probe $PPDIR/Cargo.toml $TG/cand --lib rv89_probe -- --nocapture
echo PHASE3 DONE
