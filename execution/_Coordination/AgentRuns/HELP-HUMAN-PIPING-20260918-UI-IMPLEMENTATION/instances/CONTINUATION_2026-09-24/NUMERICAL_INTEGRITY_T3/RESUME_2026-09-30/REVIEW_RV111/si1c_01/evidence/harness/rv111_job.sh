#!/bin/bash
# RV111: one cargo job under the T3 host lock.
# Usage: rv111_job.sh <tree> <crate> <logname> [cfg] -- <cargo args...>
#   tree: base | cand | ibase | graft | mut ; crate: expression_evaluator | rule_check_runner
WT=WT
S=$WT/scratch/rv111_si1c_01
tree=$1; crate=$2; logname=$3; shift 3
cfg=""
if [ "$1" != "--" ]; then cfg=$1; shift; fi
shift  # the --
export TMPDIR=$S/tmp
export CARGO_TARGET_DIR=$WT/targets/rv111-$tree
export RV111_OUT=$S/dumps/$tree
if [ -n "$cfg" ]; then export RUSTFLAGS="--cfg $cfg"; fi
mkdir -p "$RV111_OUT" "$S/logs"
cd "$WT/rv111/$tree/projects/chirality-piping/core/rules/$crate" || exit 2
echo "$(date -u '+%FT%TZ') rv111 job start tree=$tree crate=$crate cfg=$cfg args=$*" >> "$S/logs/driver.log"
"$WT/tools/t3_cargo.sh" "$@" > "$S/logs/$logname.log" 2>&1
rc=$?
echo "$(date -u '+%FT%TZ') rv111 job end tree=$tree crate=$crate rc=$rc log=$logname" >> "$S/logs/driver.log"
exit $rc
