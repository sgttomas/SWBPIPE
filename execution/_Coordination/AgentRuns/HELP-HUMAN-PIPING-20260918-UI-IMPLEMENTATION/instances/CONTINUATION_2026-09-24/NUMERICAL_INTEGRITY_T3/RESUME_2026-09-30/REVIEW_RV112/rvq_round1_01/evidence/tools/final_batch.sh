#!/bin/bash
# RV112: run under ONE host-lock acquisition (direct runs of already-built test binaries; no cargo):
# the probes (mut binary), each copy's identity and reviewed inputs, and the head's (and, if given, b1's)
# witnesses and in-build record. Usage (via lockf): final_batch.sh <copies for wit/rec...>
WT=WT
S=$WT/scratch/rv112_rvq_01
mkdir -p $S/identity $S/final
bin_of() { # bin_of <log> <crate dir>
  local b; b=$(grep -o 'Running unittests src/lib.rs ([^)]*)' "$1" | sed 's/.*(\(.*\))/\1/' | tail -1)
  case "$b" in /*) echo "$b" ;; *) echo "$2/$b" ;; esac
}
MPP=$WT/rv112/mut/projects/chirality-piping/core/product_physics
MB=$(bin_of $S/logs/mut_H01.log $MPP)
echo "BIN=$MB" > $S/final/probe_bin.txt; shasum -a 256 "$MB" >> $S/final/probe_bin.txt
( cd $MPP && env CARGO_MANIFEST_DIR=$MPP TMPDIR=$S/tmp "$MB" zz_rv112 --nocapture --test-threads=1 ) > $S/final/probes.log 2>&1; echo $? > $S/final/probes.rc
( cd $MPP && env RV112_MUT=G04 CARGO_MANIFEST_DIR=$MPP TMPDIR=$S/tmp "$MB" zz_rv112_retained_error_text_reads_every_slot_at_c b1_sa_retained_error_text_reads_every_case_slot --test-threads=1 ) > $S/final/probe_G04.log 2>&1; echo $? > $S/final/probe_G04.rc
for c in i1 sa head b1; do
  [ -f $S/logs/pp_reg_$c.log ] || continue
  CP=$WT/rv112/$c/projects/chirality-piping/core/product_physics
  B=$(bin_of $S/logs/pp_reg_$c.log $CP)
  ( cd $CP && env CARGO_MANIFEST_DIR=$CP TMPDIR=$S/tmp "$B" identity_carries_every_key_in_order reviewed_inputs_bind_the_lock_and_the_reader_statics --nocapture --test-threads=1 ) > $S/identity/$c.log 2>&1; echo $? > $S/identity/$c.rc
done
for c in i1 sa head; do
  [ -f $S/logs/pp_stale_$c.log ] || continue
  CP=$WT/rv112/$c/projects/chirality-piping/core/product_physics
  B=$(bin_of $S/logs/pp_stale_$c.log $CP)
  ( cd $CP && env CARGO_MANIFEST_DIR=$CP TMPDIR=$S/tmp "$B" identity_carries_every_key_in_order --nocapture --test-threads=1 ) > $S/identity/${c}_stale.log 2>&1; echo $? > $S/identity/${c}_stale.rc
done
for c in "$@"; do
  CP=$WT/rv112/$c/projects/chirality-piping/core/product_physics
  B=$(bin_of $S/logs/pp_reg_$c.log $CP)
  ( cd $CP && env CARGO_MANIFEST_DIR=$CP TMPDIR=$S/tmp RUST_TEST_THREADS=2 "$B" witness_ --ignored --nocapture --test-threads=1 ) > $S/logs/wit_$c.log 2>&1; echo $? > $S/logs/wit_$c.rc
  ( cd $CP && env CARGO_MANIFEST_DIR=$CP TMPDIR=$S/tmp "$B" profile_in_build_record structural_budgets_are_u3s --nocapture --test-threads=1 ) > $S/logs/rec_$c.log 2>&1; echo $? > $S/logs/rec_$c.rc
done
