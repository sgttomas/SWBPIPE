#!/bin/bash
S=WT/scratch/i66_u7f; TG=WT/targets/i66-u7f
for L in base cand; do $S/cargo_run.sh dump_rs_$L $S/$L/projects/chirality-piping/core/reporting/result_export/Cargo.toml $TG/re-$L-lane "I66_U7F_INPUTS=$S/oracle/inputs.json I66_U7F_OUT=$S/oracle/rs_$L.json" --test zz_i66_u7f_dump -- --ignored; done
echo DUMP_RS_DONE
