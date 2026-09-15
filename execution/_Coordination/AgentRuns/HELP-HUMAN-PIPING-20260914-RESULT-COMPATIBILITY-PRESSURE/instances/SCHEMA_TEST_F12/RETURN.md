# F12 schema-container test repair return

Status: `TEST_ONLY_REPAIR_READY`

The regression was a stale test-layout assumption. `analysis_run.schema.json` is now the exact-version dispatcher and therefore has no top-level `$defs`; the preserved Python 0.1 definitions remain in `analysis_run.v0.1.schema.json`. The generated Python 0.1 record already validated through the dispatcher, so no product or schema defect was found.

The sole source edit replaces private schema-layout inspection with observable dispatcher behavior. A genuine record built by `build_preview_analysis_run_envelope` is accepted for both retained `canonical_truth` values. Mutations are rejected for the SCA-003 profile, decision, storage role, canonical truth, public/direct SQL exposure, hosted database, required network, rebuildability, required-field presence, and container shape.

## Source freeze

- `tests/test_analysis_run_records.py` before SHA-256: `32b84518b856c4ebc7f5960d3b40dbe4cbd8d2eaa6f019f19898a6a4aa56616d`
- `tests/test_analysis_run_records.py` after SHA-256: `b40df20cceac6ebcf85ec7111b005fd57cfcad64b46947897ddcabaa91c15a04`
- Exact diff: `_run_records/F12_V1/exact_source.diff`, SHA-256 `2757c8caa1f9ad51c45d98c1d3a7b787d7b19e8ec2a8d01c30a5e9cdb71cbb54`

## Verification

The complete focused analysis-run record and compatibility files passed with the supplied Python and checked-JSON executable: `24 passed, 110 warnings in 8.06s`, exit `0`. `git diff --check` passed, exit `0`.

Machine-readable evidence is `_run_records/F12_V1/EXECUTION_RESULT_V1.json`; raw outputs are alongside it. Root owns independent review, integration, and the complete DEC-025 rerun.
