#!/bin/bash
# I89 B1-SA follow-up 01 (ROOT's ruling: the parked-slot patch on b1-a). On head 9812c83ded
# (WT/b1-a), registered, one cargo job at a time through WT/tools/t3_cargo.sh:
# 1. the build identity; 2. PP's whole --lib suite (the law tests included);
# 3. the c = 1 successor pins, written by the pin tests' own output variables, compared with
#    I1's (S/pins/base, from run_pins.sh) and the fixtures;
# 4. the negative check in S/i2 (byte-equal to the head's P): the new parked-slot test against
#    the unpatched retained_error_text (SA's head 6b62606778) must fail; the patch is restored.
set -u
WT=WT
S=$WT/scratch/i89_b1_sa; L=$S/logs/followup_01; T=$WT/targets/i89-b1-sa
mkdir -p $L $S/pins/cand2
CAND=$WT/b1-a/projects/chirality-piping/core/product_physics
job() { local name=$1 dir=$2 target=$3; shift 3
  ( cd $dir && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
    TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$target perl -e 'alarm shift; exec @ARGV' 7200 $WT/tools/t3_cargo.sh test --locked --offline "$@" > $L/$name.log 2>&1 )
  echo "$name exit=$?"; grep -aE '^test .* FAILED$|^test result' $L/$name.log; }
job identity $CAND $T --lib identity_carries_every_key_in_order -- --nocapture
grep -a "I65_G5_IDENTITY" $L/identity.log | sed 's#.*I65_G5_IDENTITY ##' > $L/identity.txt
grep -qF "identity: \"$(cat $L/identity.txt)\"" $CAND/src/retained_memory.rs && echo "identity: the registered one"
job lib $CAND $T --no-fail-fast --lib
awk '/^test .* \.\.\. (ok|FAILED|ignored)/{print}' $L/lib.log | sort > $L/lib.outcomes
rm -f $S/pins/cand2/*
( cd $CAND && unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
  I61_U3_OUT=$S/pins/cand2 I61_U3G2_OUT=$S/pins/cand2 I68_U8_OUT=$S/pins/cand2 TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=$T \
    $WT/tools/t3_cargo.sh test --locked --offline --lib -- --exact \
    retained_facade_tests::u3_permitted_path_publishes_the_pinned_successor \
    retained_facade_tests::u3g2_direct_entry_publishes_the_pinned_successor \
    retained_facade_tests::u8_l0_isolated_node_publishes_pinned_successor > $L/pins.log 2>&1 )
echo "pins exit=$?"; grep -a "^test " $L/pins.log
( cd $S/pins && shasum -a 256 cand2/* > $L/pins_cand2.sha256
  for f in base/*; do c=cand2/${f#base/}; if cmp -s $f $c; then echo "identical to I1 ${f#base/}"; else echo "DIFFERENT ${f#base/}"; fi; done )
P=$WT/b1-a/projects/chirality-piping/fixtures/results
for m in sparse_interactive dense_scrutiny; do
  cmp -s $S/pins/cand2/u3g2_successor_$m.json $P/retained_precision_milestone_successor_$m.json && echo "fixture milestone $m equal"
  cmp -s $S/pins/cand2/retained_precision_l0_successor_$m.json $P/retained_precision_l0_successor_$m.json && echo "fixture l0 $m equal"
done
RM=$S/i2/projects/chirality-piping/core/product_physics/src/retained_memory.rs
patched=$(shasum -a 256 $RM | cut -d' ' -f1); echo "S/i2 retained_memory.rs $patched"
cp $S/tmp/rm_head.rs $RM
job unpatched_parked_test $S/i2/projects/chirality-piping/core/product_physics $T/i2 --lib b1_sa_retained_error_text_reads_every_case_slot
grep -a -A3 "panicked" $L/unpatched_parked_test.log | head -4
cp $S/tmp/rm_patched.rs $RM
[ "$(shasum -a 256 $RM | cut -d' ' -f1)" = "$patched" ] && echo "patch restored in S/i2"
