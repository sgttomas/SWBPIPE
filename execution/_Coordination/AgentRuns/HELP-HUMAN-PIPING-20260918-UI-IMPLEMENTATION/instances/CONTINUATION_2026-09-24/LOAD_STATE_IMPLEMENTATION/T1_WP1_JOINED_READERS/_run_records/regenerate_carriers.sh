#!/bin/sh
# Mechanical regeneration of the T1_WP1_JOINED_READERS generated files.
# Run from WORKING_ROOT (projects/chirality-piping). Required environment:
#   PY                 the session Python (venv) interpreter
#   CARGO_TARGET_DIR   a scratch cargo target outside the repository
#   OPENPIPESTRESS_CHECKED_JSON_BIN, OPENPIPESTRESS_UNITS_BIN  the pre-built helper binaries
# Toolchain: cargo +1.97.1 --locked --offline -j 2.
set -eu
RR=execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION/T1_WP1_JOINED_READERS/_run_records
F=core/reporting/result_export/tests/fixtures
# 1. SF-1 fallback request (witness + one 1e-6 N tip UZ force) and its producer output.
"$PY" "$RR/make_fallback_request.py"
(cd core/product_physics && cargo +1.97.1 build --release --locked --offline -j 2 --example physics_source_connected)
EX="$CARGO_TARGET_DIR/release/examples/physics_source_connected"
"$EX" sparse "$F/load_reference_fallback_uz.request.json" > "$F/load_reference_fallback_uz-sparse_interactive.raw.json"
"$EX" dense "$F/load_reference_fallback_uz.request.json" > "$F/load_reference_fallback_uz-dense_scrutiny.raw.json"
#    Control: the same build reproduces a committed joined raw byte for byte.
"$EX" sparse fixtures/product_preview/load_reference_source/eigen_motion.request.json | cmp - fixtures/product_preview/load_reference_source/eigen_motion-sparse_interactive.raw.json
# 2. Shared case file (expectations authored, not captured).
"$PY" "$RR/make_cases.py"
# 3. Canonical results 0.3 documents (Rust derivative).
(cd core/reporting/result_export && LOAD_REFERENCE_SOURCE_WRITE_FIXTURES=1 cargo +1.97.1 test --locked --offline -j 2 --test load_reference_source_contract canonical_documents)
# 4. AnalysisRun 0.3 records (Python).
LOAD_REFERENCE_SOURCE_WRITE_FIXTURES=1 PYTHONDONTWRITEBYTECODE=1 "$PY" -m pytest -q -p no:cacheprovider tests/test_load_reference_source_readers.py -k analysis_run_carrier
