#!/bin/bash
# I61 U3 grant 2, final runs on the candidate as left in WT/f2a-memory. One cargo job at a time.
set -u
T=WT; S=$T/scratch/i61_u3_grant2_01
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
warn() { guard; ( cd $S/$2/projects/chirality-piping/core/product_physics && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 perl -e 'alarm shift; exec @ARGV' 3600 cargo $3 --locked --offline --lib --target-dir $T/targets/i61-u3g2/$4 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$?"; grep -E '^warning' -A4 $S/logs/$1.log | grep -E '^warning|-->' | sed -E 's#-->.*/(core/[^:]*):[0-9]+:[0-9]+#--> \1#' > $S/logs/$1.warnings; }
$S/sync_trees.sh
$S/run_suites.sh reg_pp
$S/run_suites.sh stale_pp
$S/run_suites.sh reg_runner
$S/run_sweeps.sh 2>&1 | grep -v unreg
# Warnings: production lib build and the lib test build, base (0c7827b6ad) against the candidate.
warn warn_base_build base build base
warn warn_cand_build mut build mut
warn warn_base_test base test base "--no-run"
warn warn_cand_test mut test mut "--no-run"
for k in build test; do cmp $S/logs/warn_base_$k.warnings $S/logs/warn_cand_$k.warnings && echo "== $k warnings identical to base"; done
