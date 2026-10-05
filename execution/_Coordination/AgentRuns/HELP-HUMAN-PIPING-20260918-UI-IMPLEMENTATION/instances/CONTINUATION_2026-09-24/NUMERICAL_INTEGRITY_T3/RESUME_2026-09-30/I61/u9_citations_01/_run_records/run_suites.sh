#!/bin/bash
# I61 u9_citations_01: suites on WT/f2a-u7 (e543c3d8f3 plus the comment rewording, uncommitted). One cargo job at a time.
set -u
T=WT; S=WT/scratch/i61_u9_citations_01; L=$T/f2a-u7/projects/chirality-piping
VENV=REPO_ROOT/projects/chirality-piping/.venv
export TMPDIR=$S/tmp PYTHONDONTWRITEBYTECODE=1 OPENPIPESTRESS_CHECKED_JSON_BIN=$T/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=$T/targets/i52-readers/units/release/openpipestress_units
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
RETAINED="tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py tests/test_retained_precision_carriers.py"
(cd $L && $VENV/bin/python -m pytest -q -p no:cacheprovider --basetemp=$S/tmp/pyr $RETAINED > $S/logs/py_retained.log 2>&1); echo "py_retained exit=$? $(tail -1 $S/logs/py_retained.log)"
(cd $L/core/reporting/result_export && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 cargo test --locked --offline --no-fail-fast --target-dir $T/targets/i61-u9c/re > $S/logs/re.log 2>&1); echo "re exit=$? $(grep '^test result' $S/logs/re.log | awk '{p+=$4; f+=$6; i+=$8} END {print p" passed, "f" failed, "i" ignored"}')"
(cd $L/core/product_physics && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 cargo test --lib --no-run --locked --offline --target-dir $T/targets/i61-u7r > $S/logs/pp_build.log 2>&1); echo "pp_build exit=$?"
(cd $L/core/product_physics && env -u RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 cargo test --tests --no-run --locked --offline --target-dir $T/targets/i61-u7r > $S/logs/pp_build_tests.log 2>&1); echo "pp_build_tests exit=$?"
(cd $L/apps/desktop && ../../node_modules/.bin/vitest run --maxWorkers=2 > $S/logs/vitest.log 2>&1); echo "vitest exit=$? $(grep -E 'Test Files|Tests ' $S/logs/vitest.log | tr '\n' ' ')"
(cd $L/apps/desktop && ../../node_modules/.bin/tsc --noEmit -p tsconfig.json > $S/logs/tsc.log 2>&1); echo "tsc exit=$? lines=$(wc -l < $S/logs/tsc.log)"
echo DONE $(date -u +%FT%TZ)
