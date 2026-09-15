# TASK writer return — result compatibility and pressure-safe foundation

Status: **COMPLETE WITH ROOT-HELD INTEGRATION QUALIFICATION**

The released compatibility product is implemented within `WRITE_SCOPE_RELEASED_V4.json` (SHA-256 `5bcb111158fffb4c6be0cb0989e7555d22a2d4dd63bd203839c9600ea32031b2`). The immutable source checkpoint is `ANALYSIS_LIFECYCLE_CHECKPOINT_V2.json` (SHA-256 `8c2c7154aaf150f8d0db5eed481a9c1d1a93d9a2501511113fc45d5539a94d6b`).

## Delivered behavior

- A checked Rust canonical JSON profile and private batched subprocess protocol reject duplicate keys, unsafe integral values, unsupported numbers, malformed Unicode, unstable IDs, and unsupported container shapes while preserving the existing global canonicalizer.
- TypeScript freezes a validated plain JSON tree before its first asynchronous boundary, preserves literal `__proto__` keys without prototype mutation, and rejects accessors, symbols, sparse arrays, non-enumerable/custom serialization, cycles, lone surrogates, and unsafe numbers.
- Python accepts only plain built-in JSON containers, freezes them once, rejects malformed payload and item-ID Unicode before process launch, and invokes one explicitly built controller executable without runtime build, PATH search, shell, or fallback.
- Analysis record 0.2 binds the received result, every row, manifest, solver/build identity, settings/unit/model-state evidence, semantic contract, load bases, diagnostics, provenance, professional boundary, and complete record projection. `created_at` is explicitly unavailable (`null`) rather than fabricated.
- Rule-check changes create a new immutable analysis-record revision identified by `analysis_run_record` SHA-256 while preserving the mechanics `run_id`. Delayed success and failure cannot publish over a newer model/solve/rule revision; current failure restores the prior aggregate.
- Historical 0.1 dispatch distinguishes the complete frozen Python and abbreviated desktop profiles. Python’s numeric-free record projection can verify while lossy result preimages remain unverifiable. Desktop equal replay may match; unequal replay without original collation/preimage is unverifiable. Result and record findings remain scoped.
- Report, result interpretation/export, local FEA, native package, and rendered report consumers select `received_result` for 0.2 and `result_envelope` for 0.1 while preserving their own downstream vocabulary.
- Persistence retains distinct 0.2 record revisions with the same mechanics `run_id` by using the complete record checksum as revision identity.
- Stress-neutral 0.2 materializes exactly nine members, checked hashes, manifest-seed and complete-package projections, exact received claims, strict closed family schemas, 830 rows, 828 eligible unit witnesses, and two explicit withheld diagnostic-work witnesses. The source semantic inventory contains 60 signatures; the shipped 830-row population exercises 48 signatures. Native shipped sparse+dense coverage plus CLI supplements and frontend composition is the truthful final qualification matrix.
- Dev and distribution Playwright producer tests are authored but were not run.

## Verification

- Python focused matrix: **50 passed**, 94 deprecation warnings.
- Desktop focused matrix: **85 passed** across eight files.
- Targeted App lifecycle/report/stress matrix: **3 passed**, 172 skipped by the name filter.
- TypeScript `--noEmit`: passed.
- Canonicalizer Rust: **10 passed**.
- Existing desktop WASM build: passed for operation and self-weight engines with wasm-bindgen 0.2.123.
- Desktop production build: passed (`tsc -b`, Vite, 1705 modules).
- Nine maintained Cargo graphs: locked offline metadata passed.

Exact commands, results, known setup failures, and dispositions are in `_run_records/FINAL_FOCUSED_EVIDENCE_V2.json` (SHA-256 `0f1dd451706ef5a39c0c36bb9e81db72e44a2b5ec104fffd4b2a0382bdf6b1dd`).

## Setup and held work

Controller setup is:

`PYTHONDONTWRITEBYTECODE=1 python tools/serialization/build_checked_json.py`

It builds `core/serialization/canonical_json/target/checked-json/release/openpipestress_jcs_ijson`. Pytest controller setup builds once; xdist workers inherit the prepared executable. Runtime adapter calls remain build-free.

Held for root’s serialized integration lease:

- `npm run test:e2e --workspace apps/desktop`
- `npm run test:e2e:dist --workspace apps/desktop`
- native Tauri GUI/build/test commands
- full DEC-025/registered sweep
- Git commit, push, merge, or handoff mutations

No browser, GUI, Playwright, native Tauri build/test, full sweep, or Git action was run. I did not delegate.

## Fence and limitations

The exact manifest records 64 changed in-fence paths with baseline and candidate SHA-256 values. Build-generated `apps/desktop/tsconfig.tsbuildinfo` and all Python `__pycache__`/`.pyc` files were removed before return. Root-owned concurrent deltas in `core/product_physics/src/lib.rs` and `core/product_physics/src/pressure_exact.rs` were observed and left untouched. No adjacent path amendment is outstanding.

Root still owns native unchanged save/reopen qualification, the authored Playwright runs, full registered sweep, separate integrated review, and Git lifecycle actions.
