# I62 review repair 07e: RV79-S1, D34, RV81-N1 and RV78-N1 (last round before acceptance)

**Basis:** "Round 04 closed; 07e is the last repair round before acceptance" (NUM `94fe144302`).

**Starting state:** READER at `abcb16fd27`.

**Run window:** 2026-10-04T02:09:28Z (first tool call) to the 02:15:03Z freeze of SHARED_SNAPSHOT_07E, inside the 45-minute box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo. Writes are within the fence. No stop condition was reached, and nothing was weakened.

## Changed READER files

| File | Before (07d) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | 031334e29f… | 3333142b4ca20108deef3ea371f101e21516049255cfac31d59c0dac741a8af5 |
| P/tests/test_retained_precision_contract.py | 25d2a64003… | e8f22d8bc7e0551cd722b928b13a8c76ee5339035752fd153079c1096fff180c |
| P/fixtures/results/retained_precision_cases.json | 12da125d9d… | bbca15d94055227da364ce4b8a7223d1350ac5b85b1219c94b1405b43c56340a |

These are unchanged: the schema (`07951edacf`), the schema test, the definition, the semantic fixture and the results yaml.

## Changes

**D34 (Python):** a JSON number equal to −0 anywhere in the receipt fails G2 ENCODING.
- It runs after the G2 encoding walk and before D32's normalization.
- It covers the enum and const integer fields: `G5aError.quantity_kind` and `source_decline.constructor_counts.directional_springs`.
- Reader-local test `test_negative_zero_anywhere_in_the_receipt_d34` covers both fields and a U counter. No base carries those two fields.

**RV78-N1, the rehash indexing rule (format and Python harness).** The rule is written into SHARED_SNAPSHOT_07E.json `format_rule`:
- **What an index is:** a strict integral value. It is a JSON number, never a boolean, that is finite, integral, ≥ 0 and not −0. So 0.0 is index 0, while 0.5, true, −0.0 and −1 are not indexes.
- **Unresolved references:** a reference that is not an index, or does not resolve, is skipped.
- **Preparation hash:** recomputed only when the attempt resolves and every one of its members is prepared.
- **Order:** preparation hashes, then selected source identities, then the publication hash, then the receipt hash, then `after_rehash` applied literally.
- **Python harness:** `_rehash_ref` implements the rule. There is no `int()` truncation, and booleans are rejected. Pinned by `test_rehash_index_rule_07e`.

**Corpus 07e (the delta from 07d):**

| Entry | Kind | Expected | Basis |
|---|---|---|---|
| `g0_receipt_version_non_integral` (`receipt_version` 1.5) | mutation | G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED | RV79-S1 |
| `g0_case_limit_non_integral` (20000000000.5) | mutation | G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED | RV79-S1 |
| `g0_invocation_limit_non_integral` (60000000000.5) | mutation | G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED | RV79-S1 |
| `verification_estimate_names_rotation_row` (layout row 3, node 0 RX, rotation) | mutation | G5 ATTEMPT_MISMATCH | RV81-N1 (D33) |
| `verification_estimate_names_moment_row` (layout row 31, end action member 0 i RX, moment) | must-pass | pass | RV81-N1 (D33) |

Every 07d entry is byte-identical (asserted).

## Results

- **Full suite:** `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **365 passed, 0 failed** (`pytest_07e.log`).
- **Every entry, installed 07e** (`PYTHON_OUTCOMES_07E.json`): 15 of 15 bases with expected classifications; 263 of 263 mutations at their expected first failure; 22 of 22 must-pass entries.
- **Staged, then installed:** 07e was staged and validated in memory first, and the installed corpus is byte-identical to the stage.
- **Mutants** (`mutants_07e.txt`): each of these is killed:
  - M29 (`_integral` truncating non-integral floats), by the three S1 pins;
  - D34 dropped;
  - D33 admitting rotation;
  - D33 refusing moment.

**Counts:** 15 cases, 263 mutations, 22 must-pass entries.

**For I63 and I64:** SHARED_SNAPSHOT_07E.json is here, and READER's corpus hash is `bbca15d940`.
