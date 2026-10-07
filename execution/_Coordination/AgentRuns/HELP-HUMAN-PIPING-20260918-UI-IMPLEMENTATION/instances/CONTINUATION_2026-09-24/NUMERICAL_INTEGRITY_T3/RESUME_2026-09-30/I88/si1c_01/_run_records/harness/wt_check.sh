#!/bin/bash
# I88: the candidate working tree's three crate suites, through the T3 lock.
set -u
WT=WT
S=$WT/scratch/i88_si1c
export TMPDIR=$S/tmp
R=$WT/s-i1c/projects/chirality-piping/core/rules
for crate in expression_evaluator rule_check_runner rule_pack_document; do
  echo "$(date -u '+%FT%TZ') begin wt_$crate" >> $S/logs/driver.log
  (cd $R/$crate && env RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
     CARGO_TARGET_DIR=$WT/targets/i88-si1c-wt $WT/tools/t3_cargo.sh test --locked --offline > $S/logs/wt_$crate.log 2>&1)
  echo "$(date -u '+%FT%TZ') end wt_$crate rc=$?" >> $S/logs/driver.log
done
