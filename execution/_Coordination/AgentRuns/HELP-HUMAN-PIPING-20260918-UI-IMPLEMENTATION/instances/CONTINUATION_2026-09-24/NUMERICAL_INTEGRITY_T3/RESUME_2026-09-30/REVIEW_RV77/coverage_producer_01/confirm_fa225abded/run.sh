#!/bin/bash
# RV77 confirmation at fa225abded. ROOT interim ruling: DEVELOPER_DIR=/Library/Developer/CommandLineTools
# in each cargo command's environment (Xcode licence unaccepted); no system setting changed.
set -u
W=WT
L=$W/scratch/rv77_coverage_producer_01; CF=$L/confirm_fa225abded; C=$W/rv77c
D=$W/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/REVIEW_RV77/coverage_producer_01/mutants
FC=projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs
RR=projects/chirality-piping/core/product_physics/src/retained_receipt.rs
FK=$C/projects/chirality-piping/core/solver/frame_kernel/Cargo.toml; PP=$C/projects/chirality-piping/core/product_physics/Cargo.toml
export DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2
ct() { perl -e 'alarm shift; exec @ARGV' 1500 cargo test --locked --offline "$@"; }
restore() { cp $CF/orig/final_case.rs $C/$FC; cp $CF/orig/retained_receipt.rs $C/$RR; }
mut() { local id=$1 diff=$2 tests=$3; restore
  if [ -n "$diff" ]; then (cd $C && patch -p1 --no-backup-if-mismatch < $diff) > $CF/mutants/$id.apply 2>&1 || { echo "$id APPLY FAILED"; restore; return; }; fi
  : > $CF/mutants/$id.log
  for t in $tests; do cr=${t%%:*}; f=${t#*:}; if [ $cr = fk ]; then M=$FK; TD=$W/targets/rv77/frame_kernel; else M=$PP; TD=$W/targets/rv77/product_physics; fi
    echo "=== $t" >> $CF/mutants/$id.log; ct --manifest-path $M --target-dir $TD --lib $f -- --nocapture >> $CF/mutants/$id.log 2>&1; echo "=== exit $t $?" >> $CF/mutants/$id.log; done
  restore; echo "$id restored=$(shasum -a 256 $C/$FC $C/$RR | cut -c1-16 | tr '\n' ' ')"
  grep -E "^=== exit|FAILED$|^test result|^error(\[|:)" $CF/mutants/$id.log | sed 's/^/  /'; }
run() { local name=$1; shift; date -u +%FT%TZ > $CF/$name.start; ct "$@" > $CF/$name.log 2>&1; echo "exit=$? end=$(date -u +%FT%TZ)" > $CF/$name.exit; echo "$name $(cat $CF/$name.exit)"; grep -E "^test result|FAILED$|^error" $CF/$name.log | head -20; }
mut NONE "" "fk:product_certificate pp:retained"
mut R1 $D/R1_floor_refusal_only_when_L_nonzero.diff "fk:product_certificate pp:retained"
mut R6 $D/R6_null_allowed_on_passed_g5a.diff "pp:retained"
mut R10 $D/R10_source_presence_not_required.diff "pp:retained"
run fk_lib --manifest-path $FK --target-dir $W/targets/rv77/frame_kernel --lib
run fk_s11 --manifest-path $FK --target-dir $W/targets/rv77/frame_kernel --test s11_site_table
run pp_lib --manifest-path $PP --target-dir $W/targets/rv77/product_physics --lib
echo CONFIRM_DONE
