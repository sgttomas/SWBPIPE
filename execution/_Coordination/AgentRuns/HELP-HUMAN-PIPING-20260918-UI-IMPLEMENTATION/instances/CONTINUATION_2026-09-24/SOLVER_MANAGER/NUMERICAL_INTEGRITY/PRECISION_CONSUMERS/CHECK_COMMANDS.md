# Serialized focused checks — pending parent CPU grant

Run from repository root. Resolve `PIPING_ROOT` to `projects/chirality-piping`; set `PRECISION_CANONICAL_TARGET`, `PRECISION_CORE_TARGET`, `PRECISION_DOCUMENTS`, `HEADLESS_PRECISION_OUTPUT_DIR` and `PIPING_TEST_PYTHON` to parent-assigned temporary paths/runtime. Keep standalone canonical and joined core targets separate. Actual chosen paths belong in `_run_records/EXECUTION_CONTEXT.json` and raw logs.

1. First run the standalone parser probe **before any canonical_json feature edit**, preserving the failure or pass:

```sh
CARGO_BUILD_JOBS=1 cargo test --offline --manifest-path "$PIPING_ROOT/core/serialization/canonical_json/Cargo.toml" --target-dir "$PRECISION_CANONICAL_TARGET" --test precision_transport -- --nocapture
```

If that proves float_roundtrip necessary, add only that serde feature in the owned canonical_json manifest, rerun this probe, and retain both logs. Do not change JCS code or the I-JSON safe-integer guard. Re-freeze the changed manifest and affected checks.

2. Run standalone canonical tests and build its checked CLI with the final feature closure:

```sh
CARGO_BUILD_JOBS=1 cargo test --offline --manifest-path "$PIPING_ROOT/core/serialization/canonical_json/Cargo.toml" --target-dir "$PRECISION_CANONICAL_TARGET"
CARGO_BUILD_JOBS=1 cargo build --offline --manifest-path "$PIPING_ROOT/core/serialization/canonical_json/Cargo.toml" --target-dir "$PRECISION_CANONICAL_TARGET" --features checked-cli --bin openpipestress_jcs_ijson
```

3. Run full result-export tests, including all historical row/hash tests and new precision vectors; capture documents for schema checking:

```sh
RESULTS_RUST_CONTRACT_OUTPUT_DIR="$PRECISION_DOCUMENTS" CARGO_BUILD_JOBS=1 cargo test --offline --manifest-path "$PIPING_ROOT/core/reporting/result_export/Cargo.toml" --target-dir "$PRECISION_CORE_TARGET"
```

4. Run headless tests with the actual current product dependency, capturing source/derivative evidence:

```sh
HEADLESS_PRECISION_OUTPUT_DIR="$HEADLESS_PRECISION_OUTPUT_DIR" CARGO_BUILD_JOBS=1 cargo test --offline --manifest-path "$PIPING_ROOT/core/runner/headless/Cargo.toml" --target-dir "$PRECISION_CORE_TARGET"
```

5. Point the Python canonical adapter to the standalone checked executable; run exact analysis/source/record/schema/persistence/neutral compatibility and new precision tests. The maintained actual-headless test must run with artifacts rather than skip:

```sh
OPENPIPESTRESS_CHECKED_JSON_BIN="$PRECISION_CANONICAL_TARGET/debug/openpipestress_jcs_ijson" HEADLESS_PRECISION_OUTPUT_DIR="$HEADLESS_PRECISION_OUTPUT_DIR" PYTHONDONTWRITEBYTECODE=1 PYTHONPATH="$PIPING_ROOT:$PIPING_ROOT/tests" "$PIPING_TEST_PYTHON" -m pytest -q "$PIPING_ROOT/tests/test_precision_consumer_contract.py" "$PIPING_ROOT/tests/test_stress_neutral_precision.py" "$PIPING_ROOT/tests/test_analysis_run_compatibility.py" "$PIPING_ROOT/tests/test_analysis_run_schema.py" "$PIPING_ROOT/tests/test_project_persistence_service.py" "$PIPING_ROOT/tests/test_stress_neutral_export_package.py" "$PIPING_ROOT/tests/test_result_export_v0_2.py"
"$PIPING_TEST_PYTHON" "$PIPING_ROOT/tests/test_result_export_v0_2.py" --outputs "$PRECISION_DOCUMENTS"
"$PIPING_TEST_PYTHON" "$PIPING_ROOT/tests/test_result_export_v0_2.py" --outputs "$HEADLESS_PRECISION_OUTPUT_DIR"
```

Frontend/native owner additionally runs the shared 26 vectors through the actual candidate WASM/native JSON hash and storage routes; standalone canonical feature closure cannot be inferred from joined product builds. Fresh complete-diff independent review follows repairs and freeze. No build/test command above has run yet.
