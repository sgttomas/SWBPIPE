#!/bin/bash
# Calibration: CI's numerical cargo profile (every discovered manifest; cargo test --offline --locked)
# on a git archive of main, on this Mac. K4 at B under ROOT's host rule: -j 4, RUST_TEST_THREADS=2, its own target.
M=$1; OUT=$2; mkdir -p $OUT
export RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$3 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2
cd $M/projects/chirality-piping
python3 - <<'PY' > $OUT/manifests.txt
import importlib.util, sys
from pathlib import Path
spec = importlib.util.spec_from_file_location('r', 'tools/release/check_release_readiness.py'); m = importlib.util.module_from_spec(spec); sys.modules['r'] = m; spec.loader.exec_module(m)
for p in m.discover_cargo_manifests(Path('.')): print(p)
PY
for mf in $(cat $OUT/manifests.txt); do cargo fetch --locked --manifest-path $mf > /dev/null 2>&1 || echo "FETCH-FAIL $mf"; done
i=0
for mf in $(cat $OUT/manifests.txt); do
  log=$OUT/$(printf '%03d' $i)_$(echo $mf | tr '/' '_').log
  cargo test --offline --locked --no-fail-fast --manifest-path $mf > $log 2>&1; rc=$?
  pass=$(grep -E '^test result:' $log | awk '{s+=$4} END {print s+0}'); fail=$(grep -E '^test result:' $log | awk '{s+=$6} END {print s+0}'); ign=$(grep -E '^test result:' $log | awk '{s+=$8} END {print s+0}')
  echo "$rc passed=$pass failed=$fail ignored=$ign $mf"
  i=$((i+1))
done
echo SUITES-DONE
