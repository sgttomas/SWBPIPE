# I62 review repair 07a: D18, positive section echo terms

**Basis:** ruling D18, "Phase 2 returns; D18 section truth includes positive echo terms" (NUM `253ac9404e`). D10 is corrected in place.

**Starting state:** READER at `babcce075e`.

**Run window:** 2026-10-04T00:19:27Z to the 00:21:14Z freeze of SHARED_SNAPSHOT_07A, inside the 30-minute box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo. Writes are within the fence.

## Changed READER files

| File | Before (07) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | dddac2fa96… | 9669b88359f0f40756d0d27d0aa503f2ddd35ce4ac1174363671e12b33c8611e |
| P/fixtures/results/retained_precision_cases.json | 90f6e4ed9b… | a6fa398731c35245baca098e322a9846399c7535b2d4cd882a5058010de456c9 |
| P/tests/test_retained_precision_contract.py | a0ead06840… | 2d2ca008c9cd24f0bb6e70dfcf85c5e65e42b7c20212d3fae1112bda2030a65b |

These are unchanged: the schema (`07951edacf`), the schema test (`90bbd5660c`), the definition, the semantic fixture and the results yaml.

## Changes

**Python (G5b section echo, `_g5_numeric`):** each echoed term must equal the source's and be positive (`from_bits(term) > 0`), or G5b SECTION_MISMATCH. The terms are `area`, `section_modulus`, `length`, `axial_stiffness` and `torsional_stiffness`.
- Native basis: PP:2360, 2442 and 2462; endpoint_maximum.rs:128.
- A zero, negative-zero or negative term now fails explicitly. It no longer reaches the arithmetic fallback.

**Corpus (the delta in SHARED_SNAPSHOT_07A.json):**
- **`g5b_zero_section_area`:** the expectation changes from G5b SCALE_MISMATCH to G5b SECTION_MISMATCH. Its edits are unchanged.
- **New `g5b_zero_section_length`:** on `ordinary_prepared_synthetic`, the source and selection `section_terms[0].length` are both set to `0000000000000000`. It expects G5b SECTION_MISMATCH.
  - The 07 Python reader reported G8 PREPARATION for it, so it pins the positivity rule.
- Everything else is byte-identical (asserted).

**Tests:**
- `test_g5b_arithmetic_fault_reports_g5b_d10` is replaced by `test_g5b_section_echo_terms_positive_d18`. It covers each of the five terms with +0, −0 and −1, and expects G5b SECTION_MISMATCH each time.
- The 07 count test now expects 15 / 236 / 19.

## Results

- **Full suite:** `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **328 passed, 0 failed** (`pytest_07a.log`).
- **Every entry** (`PYTHON_OUTCOMES_07A.json`): 15 of 15 bases with expected classifications; 236 of 236 mutations at their expected first failure; 19 of 19 must-pass entries.

**Counts:** 15 cases, 236 mutations, 19 must-pass entries.
