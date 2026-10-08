#!/bin/bash
# RV112: everything left, one host-lock acquisition per step, one step at a time (the per-mutant and
# per-copy acquisitions of phases 1-2b waited up to 10+ min each behind about 14 other waiters), with
# every remaining runtime mutant in ONE acquisition.
set -u
WT=WT
S=$WT/scratch/rv112_rvq_01
J=$S/tools/runjob.sh
LOCK=$WT/guard/cargo_job.lock
MPP=$WT/rv112/mut/projects/chirality-piping/core/product_physics
BIN=$(sed -n 's/^BIN=//p' $S/mutants/bin.txt)
guard() { pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "memory guard not running $(date -u +%FT%TZ)" >> $S/logs/ABORT; exit 97; }; }
todo=(); for m in $(cut -f1 $S/tools/mutants.tsv); do [ -f $S/mutants/$m.rc ] || todo+=("$m"); done
batch() { guard; /usr/bin/lockf -k $LOCK $S/tools/batch_run.sh "$BIN" "$@"; }
step() { echo "$(date -u +%FT%TZ) $*" >> $S/logs/phase_rest.steps; }
step M_all; [ ${#todo[@]} -gt 0 ] && batch "${todo[@]}"
step runner_sa; $J runner_sa $WT/rv112/sa/projects/chirality-piping/core/runner/headless rv112-runner-sa 0 test --locked --offline --no-fail-fast
step pp_reg_head; $J pp_reg_head $WT/rv112/head/projects/chirality-piping/core/product_physics rv112-head 0 test --locked --offline --no-fail-fast
step pp_stale_head; $J pp_stale_head $WT/rv112/head/projects/chirality-piping/core/product_physics rv112-head-stale 1 test --locked --offline --no-fail-fast --lib
step runner_head; $J runner_head $WT/rv112/head/projects/chirality-piping/core/runner/headless rv112-runner-head 0 test --locked --offline --no-fail-fast
step H01
F=$MPP/src/retained_memory.rs
cp $F $S/mutants/retained_memory.schemata.rs
python3 - "$F" <<'PY'
import sys
p = sys.argv[1]; t = open(p, encoding="utf-8").read()
old = "    notice_reserve_bytes: NOTICE_RESERVE_BYTES * caps::LOAD_CASES as u64,\n"
assert t.count(old) == 1
open(p, "w", encoding="utf-8").write(t.replace(old, "    notice_reserve_bytes: NOTICE_RESERVE_BYTES,\n"))
PY
shasum -a 256 $F > $S/mutants/H01.src.sha256
$J mut_H01 $MPP rv112-mut 0 test --locked --offline --no-fail-fast --lib -- --skip zz_rv112
cp $S/logs/mut_H01.log $S/mutants/H01.log; cp $S/logs/mut_H01.rc $S/mutants/H01.rc
cp $S/mutants/retained_memory.schemata.rs $F
shasum -a 256 $F > $S/mutants/restored.sha256
step final_batch; guard; /usr/bin/lockf -k $LOCK $S/tools/final_batch.sh head
step done_essential; echo ESSENTIAL_DONE $(date -u '+%FT%TZ') > $S/logs/essential.done
step pp_reg_b1; $J pp_reg_b1 $WT/rv112/b1/projects/chirality-piping/core/product_physics rv112-b1 0 test --locked --offline --no-fail-fast
step final_batch_b1; guard; /usr/bin/lockf -k $LOCK $S/tools/final_batch.sh b1
step done; echo PHASE_REST_DONE $(date -u '+%FT%TZ') > $S/logs/phase_rest.done
