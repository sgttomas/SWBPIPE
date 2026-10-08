#!/bin/bash
# I99 B3-W: one probe run (controls, then the input files), through WT/tools/t3_cargo.sh.
# usage: run_probe.sh <tag>   (WT from the environment)
set -u
S=$WT/scratch/i99_b3w
TAG=$1
mkdir -p $S/out/$TAG
while [ "$(pgrep -f '^/usr/bin/lockf' | wc -l)" -gt 1 ]; do sleep 20; done
FILES=$(for f in m3x n05 n06 m3x_mix_axial m3x_mix_anchor m3x_mix_lateral m1_twin_axial m1_twin_anchor m1_twin_lateral m3l fields; do printf '%s:' "$S/inputs/$f.json"; done)
cd $S/arch/projects/chirality-piping/core/product_physics
env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=$S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=1 CARGO_TARGET_DIR=$WT/targets/i99-b3w \
  I99_OUT=$S/out/$TAG I99_FILES="$FILES" \
  $WT/tools/t3_cargo.sh test --locked --offline --lib -- --ignored --nocapture --test-threads=1 --exact \
  retained_memory::zz_i99_probe::zz_i99_dump_builtin retained_memory::zz_i99_probe::zz_i99_controls retained_memory::zz_i99_probe::zz_i99_files \
  > $S/logs/probe_$TAG.log 2>&1
echo "rc=$?" >> $S/logs/probe_$TAG.log
date -u >> $S/logs/probe_$TAG.log
