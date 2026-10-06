#!/bin/bash
# usage: run_diff.sh <tag>   (outputs carry the tag, so no earlier process can write into them)
WT=WT
S=$WT/scratch/rv101_t6s_01
tag=$1
export TMPDIR=$S/tmp
run() {
  c=$1
  mkdir -p $S/out/diff_${tag}_$c
  cd $WT/rv101/$c/projects/chirality-piping/apps/desktop
  RV101_MANIFEST=$S/out/manifest_cand.json RV101_OUT=$S/out/diff_${tag}_$c.jsonl RV101_BYTES=$S/out/diff_${tag}_$c ../../node_modules/.bin/vitest run src/zz_rv101/zz_rv101_differential.test.ts --reporter=default > $S/logs/diff_${tag}_$c.log 2>&1
  echo "rc=$?" >> $S/logs/diff_${tag}_$c.log
}
run base & run cand & wait
