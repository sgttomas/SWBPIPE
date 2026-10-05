# I62 review repair 07, phase B2: snapshot 07 installed in READER

**Basis:** ROOT's B2 grant: install staged 07, add D17, and update the Python tests.

**Run window:** 2026-10-04T00:05:27Z to the 00:07:28Z freeze of SHARED_SNAPSHOT_07, well inside the 45-minute box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo, no installs. READER writes are within the five-file fence; the Python reader and the schema test are unchanged in B2.

## Changed READER files

| File | Before (B1 / 06d) | Now |
|---|---|---|
| P/fixtures/results/retained_precision_cases.json | d02701ed6a… (06d) | 90f6e4ed9b5ebd00eb5d459397476c1e459b0a923eb1209037f8f86e0009367e |
| P/schemas/retained_precision_mp_v2.schema.json | f943ebd351… | 07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c (= staged) |
| P/tests/test_retained_precision_contract.py | 5fb053fa46… (B1) | a0ead068405324b5b89ff98ba309cda275d06c07037e0fa84b4fbf64caced186 |
| P/core/analysis_runs/retained_precision.py | dddac2fa96… (B1) | unchanged |
| P/tests/test_retained_precision_schema.py | 90bbd5660c… | unchanged |

**How the corpus was installed:**
- The staged corpus was installed byte-identical first (`cb148a0f7d`).
- Then `add_d17.py` appended one mutation, giving `90f6e4ed9b`. The staged schema is installed unchanged.

## D17

`ordinary_dangling_ref_plus_source_preparation_null` is on F′ (`two_case_facade_after_certificate_synthetic`). It has two edits:
- `ordinary_attempts[1].diagnostic_refs = ["diagnostic:missing"]`, a dangling ordinary reference;
- `sources[1].preparation = null`, which breaks D4a.

**Expected:** G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`.

**Python already orders class 2 this way:**
1. `_g5_ordinary` (ordinary references, ATTEMPT);
2. then the D4c case pass;
3. then the per-attempt C3 association (PRODUCT_ATTEMPT).

The reader-local test `test_class2_ordinary_before_association_d17` pins both single defects and the dual.

## Python result on installed 07

- `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **327 passed, 0 failed** (`pytest_b2.log`, input hashes listed there).
- `prefix_attached_old_input_unbound` now passes at G8 on its F′ rebase.
- New test `test_snapshot_07_counts_and_entry_format` asserts:
  - the counts 15 / 235 / 19;
  - `rehash:"all"` only;
  - unique ids;
  - that the only `expected_by_reader` entry is the G7 one.
- **Every entry** (`PYTHON_OUTCOMES_07.json`): 15 of 15 bases with expected classifications; 235 of 235 mutations at their expected first failure; 19 of 19 must-pass entries.

## Snapshot 07 (SHARED_SNAPSHOT_07.json)

**Counts:** 15 cases, 235 mutations (57 new), 19 must-pass entries (1 new).

**06d content changed (asserted to be the only changes):**
- P′ case 1 gains `source_ref: null` (D9b).
- `prefix_attached_old_input_unbound` is rebased onto F′ (checkpoint A, D8).

**What the JSON records:**
- each new entry, with its expected gate and code, its source decision, its observed Python outcome, and the 06d reader's outcome;
- the deferred list with reasons;
- the schema changes;
- the hashes of the shared files.

**Shared files:**

| File | sha256 | Changed from 06d |
|---|---|---|
| definition `retained_precision_prepared_ordinary_v1.json` | 3e0779a45a… | no |
| semantic fixture | c74742ce6a… | no |
| corpus | 90f6e4ed9b… | yes |
| schema | 07951edacf… | yes |
| results yaml | 4585a45fcf… | no |

**For I63 and I64:** phase 2 adopts 07, D8, D9 and D17.
