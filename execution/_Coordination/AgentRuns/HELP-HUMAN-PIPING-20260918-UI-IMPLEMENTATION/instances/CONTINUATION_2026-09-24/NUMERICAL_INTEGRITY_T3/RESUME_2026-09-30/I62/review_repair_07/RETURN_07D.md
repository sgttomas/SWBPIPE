# I62 review repair 07d: D31–D33, RV78-N1 and RV78-N2

**Basis** (NUM `f4d5cbbe49`):
- "T1 and T2: I61's analysis and the rulings" (D31, D32);
- "Round 03 closed; the 07d repair round" (D33, and the RV78-N1/N2 notes).

**Starting state:** READER at `a894d9d0ba`.

**Run window:** 2026-10-04T01:29:36Z (first tool call) to the 01:38:02Z freeze of SHARED_SNAPSHOT_07D, inside the 90-minute box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo. Writes are within the fence. **No stop condition was reached.** No check was removed or weakened: D31 admits one more version, as ruled, and D32 makes the existing integer tests value-based.

## Changed READER files

| File | Before (07c) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | 59e5b1cc98… | 031334e29fb150e3bdb63f4b88816679cf83d4243f91db05a94ee9a8a6afa8f3 |
| P/tests/test_retained_precision_contract.py | bd35681666… | 25d2a6400328bb865276ecfc9f3027ba711f18235e5af5c866dd72ab91d1c163 |
| P/fixtures/results/retained_precision_cases.json | d33667719e… | 12da125d9dcfcb55debe2fc1ab1eadcdaf1b35fd188206b251d48f563309e184 |

These are unchanged: the schema (`07951edacf`), the schema test, the definition, the semantic fixture and the results yaml.

## Python

**D31:** G8 admits model `schema_version` 0.1.0, 0.2.0 or 0.3.0. 0.4.0 stays excluded (G8 INVOCATION_MISMATCH, the code all three readers use for this check).

**D32 (integers by value):** a new `_integral(v)` accepts a number that is finite, integral and not −0, and never a bool.
- **G0:** `receipt_version` and the 20B/60B limits are compared by value (the old PY:1559–1561).
- **G1:** the source-identity and preparation-hash index sites use `_integral` (the old PY:1572, 1578).
- **After G2:** `_normalize_integrals` converts every integral float in the receipt to `int`, once. The schema's only receipt numbers are U and I32, and G2 has already checked them.
- **G3:** the complete-old and coverage-roster sites (the old PY:1606, 1617) also use `_integral`.
- No `type(x) is int` test remains on a receipt value.

**D33:** a `verification_estimate` reason must name a force or moment row (G5 ATTEMPT).

**Harness (D32):** rehashing indexes by integral value (`int(...)`), per RV79-N-e.

## Snapshot 07d (SHARED_SNAPSHOT_07D.json)

**Counts:** 15 cases, 259 mutations (5 new), 21 must-pass entries (2 new).

| Entry | Kind | Expected | 07c Python reader |
|---|---|---|---|
| `model_schema_version_0_1_0_accepted` (D31) | must-pass (`invocation_edits`) | pass | G8 INVOCATION |
| `model_schema_version_0_4_0_rejected` (D31) | mutation (`invocation_edits`) | G8 INVOCATION_MISMATCH | G8 INVOCATION |
| `integral_float_integers_and_references` (D32) | must-pass: 32 edits writing integer fields and references as integral floats (`receipt_version`, the limits, `charged`, ids, refs, indices and members) | pass, same classifications | G0 |
| `forged_source_identity_float_source_ref` (D32) | mutation: `source_ref` 0.0, then `after_rehash` forges the identity with a literal receipt hash | G1 RECEIPT_MISMATCH | **pass** (RV79-E1) |
| `verification_estimate_names_translation_row` (D33, RV80 PR16) | mutation | G5 ATTEMPT_MISMATCH | pass |
| `ready_attempt_under_facade_failure_cause` (RV78-N1) | mutation (RV78's edits) | G5 PRODUCT_ATTEMPT_MISMATCH | G5 PRODUCT_ATTEMPT |
| `ready_attempt_under_prepared_product_failure` (RV78-N1) | mutation (RV78's edits) | G5 PRODUCT_ATTEMPT_MISMATCH | G5 PRODUCT_ATTEMPT |

**Changed 07c entry (RV78-N2):** `unavailable_attempt_under_source_error_cause` now has `source_decline.constructor_counts.nodes` 0, consistent with `no_nodes` (C2:56). Its expectation is unchanged at G5 PRODUCT_ATTEMPT in both readers.

**Not added:** the D19 Ready-direction control under `receipt_failure`. Native emission of a per-case `receipt_failure` beside a selected case is not established (RV78-N1), so it stays a reader-logic control.

## Results

- **Full suite:** `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **358 passed, 0 failed** (`pytest_07d.log`).
- **New tests:** `test_integers_by_value_at_every_site_d32` (each of the five sites), `test_model_schema_versions_d31` and `test_verification_estimate_names_force_or_moment_d33`.
- **Every entry, installed 07d** (`PYTHON_OUTCOMES_07D.json`): 15 of 15 bases with expected classifications; 259 of 259 mutations at their expected first failure; 21 of 21 must-pass entries.
- **Staged, then installed:** 07d was staged and validated in memory first, and the installed corpus is byte-identical to the stage.
- **Mutants** (`mutants_07d.txt`): each of these is killed:
  - G0 host-int, for `receipt_version` and for the limits;
  - G1 host-int, for the identity and for the preparation hash;
  - no normalization after G2;
  - D31 reverted;
  - D33 dropped.

**For I63 and I64:** SHARED_SNAPSHOT_07D.json is here, and READER's corpus hash is `12da125d9d`.
