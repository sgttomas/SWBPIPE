#!/bin/sh
# Mechanical regeneration of the load_reference_* carrier fixtures from the
# frozen raw envelopes (rerun after the manager regenerates the producer output).
# Machine paths are this session's; adjust for another host.
set -eu
WR=/home/user/wt/loadstate/projects/chirality-piping
PY=/home/user/dec025-venv/bin/python
export OPENPIPESTRESS_CHECKED_JSON_BIN=$WR/core/serialization/canonical_json/target/checked-json/release/openpipestress_jcs_ijson
export OPENPIPESTRESS_UNITS_BIN=$WR/core/units/target/units-authority/release/openpipestress_units
# 1. Shared case file (expectations are authored, not captured).
$PY "$(dirname "$0")/make_mutations.py" "$WR"
# 2. Canonical results 0.3 documents (Rust derivative).
(cd "$WR/core/reporting/result_export" && CARGO_TARGET_DIR=/home/user/cargo-targets/lscp3readers/result_export \
  LOAD_REFERENCE_WRITE_FIXTURES=1 cargo +1.97.1 test --locked --offline -j 1 --test load_reference_contract canonical_documents)
# 3. AnalysisRun 0.3 records (Python).
(cd "$WR" && LOAD_REFERENCE_WRITE_FIXTURES=1 $PY -m pytest -q -p no:cacheprovider tests/test_load_reference_readers.py -k carrier_fixture)
# 4. Stress-neutral 0.3 packages: blocked (core/handoff/stress_neutral/package_v0_3.py edit denied by host permission).
