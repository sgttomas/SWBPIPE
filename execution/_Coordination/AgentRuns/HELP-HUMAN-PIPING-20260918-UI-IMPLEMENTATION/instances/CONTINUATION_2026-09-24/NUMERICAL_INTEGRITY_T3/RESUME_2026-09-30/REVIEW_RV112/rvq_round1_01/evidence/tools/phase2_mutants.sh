#!/bin/bash
# RV112 phase 2: build the mut copy once (schemata + probe), then under the host lock run the
# pristine control, the probes, and each runtime mutant (RV112_MUT=<id>) over PP's whole lib suite;
# then the const mutant H01 (B-6 without ×C) as its own build, and restore.
set -u
WT=WT
S=$WT/scratch/rv112_rvq_01
J=$S/tools/runjob.sh
PP=$WT/rv112/mut/projects/chirality-piping/core/product_physics
mkdir -p $S/mutants
$J mut_build $PP rv112-mut 0 test --locked --offline --no-fail-fast --lib --no-run
BIN=$(grep -o 'Executable unittests src/lib.rs ([^)]*)' $S/logs/mut_build.log | sed 's/.*(\(.*\))/\1/' | tail -1)
case "$BIN" in /*) ;; *) BIN=$PP/$BIN ;; esac
echo "BIN=$BIN" > $S/mutants/bin.txt
[ -x "$BIN" ] || { echo "no test binary" >> $S/mutants/bin.txt; exit 1; }
shasum -a 256 "$BIN" >> $S/mutants/bin.txt
runbin() { # runbin <name> <RV112_MUT value> <libtest args...>
  local name=$1 mut=$2; shift 2
  pgrep -f "$WT/guard/memguard.sh" >/dev/null || { echo "memory guard not running" > $S/mutants/$name.log; return 97; }
  ( cd $PP && /usr/bin/lockf -k $WT/guard/cargo_job.lock env RV112_MUT="$mut" CARGO_MANIFEST_DIR=$PP TMPDIR=$S/tmp RUST_TEST_THREADS=2 RUST_BACKTRACE=0 "$BIN" "$@" ) > $S/mutants/$name.log 2>&1
  echo $? > $S/mutants/$name.rc
}
runbin pristine "" --skip zz_rv112
runbin probes "" zz_rv112 --nocapture --test-threads=1
for m in $(cut -f1 $S/tools/mutants.tsv); do
  runbin $m $m --skip zz_rv112
done
# H01: the const B-6 mutant, one rebuild, then restore.
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
cp $S/logs/mut_H01.log $S/mutants/H01.log
cp $S/mutants/retained_memory.schemata.rs $F
shasum -a 256 $F > $S/mutants/restored.sha256
echo PHASE2_DONE $(date -u '+%FT%TZ') > $S/logs/phase2.done
