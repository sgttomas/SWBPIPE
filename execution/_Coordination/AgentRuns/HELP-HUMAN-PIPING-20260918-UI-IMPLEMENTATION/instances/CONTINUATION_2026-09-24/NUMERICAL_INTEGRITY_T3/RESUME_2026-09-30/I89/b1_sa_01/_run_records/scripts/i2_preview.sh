#!/bin/bash
# I89 B1-SA: an I2 preview (records only; no Git write). The tree S/i2 is a git archive of SP's
# head 56c5579f07 (P without P/execution) with SA's three files from 6b62606778 (SP's change
# and SA's are disjoint by file, so this is the merge's tree), plus parked_slots.diff (ROOT's
# note after R3′). Run 1: PP's whole --lib suite, registered. Run 2: the new parked-slot test
# against the unpatched retained_error_text (SA's head), which must fail; then the patch is put
# back and checked by sha256. Each job through WT/tools/t3_cargo.sh.
set -u
WT=WT
S=$WT/scratch/i89_b1_sa; L=$S/logs; T=$WT/targets/i89-b1-sa/i2
PP=$S/i2/projects/chirality-piping/core/product_physics
RM=$PP/src/retained_memory.rs
patched=$(shasum -a 256 $RM | cut -d' ' -f1); echo "patched retained_memory.rs $patched"
run() { local name=$1; shift
  ( cd $PP && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
    TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$T perl -e 'alarm shift; exec @ARGV' 7200 $WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast --lib "$@" > $L/$name.log 2>&1 )
  echo "$name exit=$?"; grep -aE '^test .* FAILED$|^test result' $L/$name.log; }
run i2_preview_lib
cp $RM $S/tmp/rm_patched.rs; cp $S/tmp/rm_head.rs $RM
run i2_unpatched_parked_test b1_sa_retained_error_text_reads_every_case_slot
cp $S/tmp/rm_patched.rs $RM
[ "$(shasum -a 256 $RM | cut -d' ' -f1)" = "$patched" ] && echo "patch restored"
