#!/bin/bash
T=WT; P=$T/scratch/rv89_u4_g7_01/pr; TG=$T/targets/rv89_pr
C=$T/rv89_pr/base/projects/chirality-piping/core; PM=$C/product_physics/Cargo.toml; F=projects/chirality-piping/core/reporting/result_export/src/source_blocks.rs
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive 92a5a9da1c $F | tar -x -C $T/rv89_pr/base )
python3 $P/instrument_int.py $C new || exit 1
cp $P/rv89_int_probe.rs $C/product_physics/src/rv89_int_probe.rs
printf '\n#[cfg(test)]\n#[path = "rv89_int_probe.rs"]\nmod rv89_int_probe;\n' >> $C/product_physics/src/lib.rs
cp $P/zz_rv89_int_sweep.rs $C/product_physics/tests/zz_rv89_int_sweep.rs
RUST_TEST_THREADS=1 $P/run.sh int_probe_new $PM $TG/base --lib -- rv89int_integer_reach_and_allocations --nocapture --test-threads=1
RV89_SWEEP_INPUTS=$T/scratch/rv89_u4_g5/inputs RV89_SWEEP_OUT=$P/sweep_int.tsv $P/run.sh int_sweep $PM $TG/base --test zz_rv89_int_sweep -- --ignored --nocapture
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive 92a5a9da1c $F | tar -x -C $T/rv89_pr/base )
python3 $P/instrument_int.py $C old || exit 1
RUST_TEST_THREADS=1 $P/run.sh int_probe_old $PM $TG/base --lib -- rv89int_integer_reach_and_allocations --nocapture --test-threads=1
rm -f $C/product_physics/src/rv89_int_probe.rs $C/product_physics/tests/zz_rv89_int_sweep.rs
( cd $T/numerics && GIT_OPTIONAL_LOCKS=0 git archive 6b9bb19a5f $F projects/chirality-piping/core/product_physics/src/lib.rs | tar -x -C $T/rv89_pr/base )
echo PHASEINT DONE
