#!/bin/bash
# RV112 phase 2b: the remaining runtime mutants in batches of 6 per host-lock acquisition (the
# per-mutant acquisitions of phase 2 waited 4-5 min each behind other jobs); then H01 (B-6 without
# x C, const: its own build) and restore; then the probe rerun.
set -u
WT=WT
S=$WT/scratch/rv112_rvq_01
J=$S/tools/runjob.sh
PP=$WT/rv112/mut/projects/chirality-piping/core/product_physics
BIN=$(sed -n 's/^BIN=//p' $S/mutants/bin.txt)
todo=()
for m in $(cut -f1 $S/tools/mutants.tsv); do [ -f $S/mutants/$m.rc ] || todo+=("$m"); done
i=0
while [ $i -lt ${#todo[@]} ]; do
  batch=("${todo[@]:$i:6}")
  pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "memory guard not running" > $S/mutants/ABORT; exit 97; }
  /usr/bin/lockf -k $WT/guard/cargo_job.lock $S/tools/batch_run.sh "$BIN" "${batch[@]}"
  i=$((i + 6))
done
F=$PP/src/retained_memory.rs
cp $F $S/mutants/retained_memory.schemata.rs
python3 - "$F" <<'PY'
import sys
p = sys.argv[1]; t = open(p, encoding="utf-8").read()
old = "    notice_reserve_bytes: NOTICE_RESERVE_BYTES * caps::LOAD_CASES as u64,\n"
assert t.count(old) == 1
open(p, "w", encoding="utf-8").write(t.replace(old, "    notice_reserve_bytes: NOTICE_RESERVE_BYTES,\n"))
PY
shasum -a 256 $F > $S/mutants/H01.src.sha256
$J mut_H01 $PP rv112-mut 0 test --locked --offline --no-fail-fast --lib -- --skip zz_rv112
cp $S/logs/mut_H01.log $S/mutants/H01.log; cp $S/logs/mut_H01.rc $S/mutants/H01.rc
cp $S/mutants/retained_memory.schemata.rs $F
shasum -a 256 $F > $S/mutants/restored.sha256
$J probe2 $PP rv112-mut 0 test --locked --offline --no-fail-fast --lib zz_rv112 -- --nocapture --test-threads=1
echo PHASE2B_DONE $(date -u '+%FT%TZ') > $S/logs/phase2b.done
