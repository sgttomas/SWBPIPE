# RV79 confirmation review 04: the Python reader at READER `abcb16fd27` (snapshot 07d)

RV79 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN, Agent 0) for a scoped round 04 (workflow §3). ROOT is the return path. RV79 wrote none of the repairs and did not delegate. All evidence below comes from RV79's own probes, mutants and oracle on the new head.

- **Run:** first tool call 2026-10-04T01:46:19Z; report frozen about 02:07Z, inside the 45-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`); no install, new tooling, Cargo, native, solver or DEC-025 job.
- **Copies:** two `git archive` copies of the head in WT/rv79, deleted afterwards.
- **Paths** use the placeholders WT, P, NUM, R, T3 and VENV.

## Verdict: PASS

| Severity | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 3 |

**E1 is fixed (D32).**
- Every E1 probe now gives the ruled outcome. That includes the forged source identity with `source_ref: 0.0` (G1) and the forged preparation hash with a float `attempt_ref`, which now fails G1 with and without an invocation.
- Booleans where an integer is required are rejected: G0 for the G0 fields, G1 (closed shape) elsewhere.
- All 48 earlier RV79 probes still agree with their rulings. The D31 and D33 probes agree, and the oracle shows 0 mismatches.
- Apart from the ruled D31 widening, no check was removed or weakened.

**The one SHOULD-FIX is a test gap (S1).** My mutant M29 makes `_integral` truncate non-integral floats. It survives, yet a single defect separates it: `receipt_version: 1.5` gives G0 in the reviewed reader and G1 under M29.

## Candidate

| Item | Value |
|---|---|
| READER head | `abcb16fd27d7c3ccd019f2533eb261d4c564fdc7` (07d); NUM at `83732c5677` |
| `P/core/analysis_runs/retained_precision.py` | `031334e29fb150e3bdb63f4b88816679cf83d4243f91db05a94ee9a8a6afa8f3` (was `59e5b1cc98`) |
| `P/tests/test_retained_precision_contract.py` | `25d2a6400328bb865276ecfc9f3027ba711f18235e5af5c866dd72ab91d1c163` |
| Corpus 07d / schema | `12da125d9dcfcb55debe2fc1ab1eadcdaf1b35fd188206b251d48f563309e184` (15 / 259 / 21) / `07951edacf` (unchanged) |
| Diffs since `a894d9d0ba` | Python 106 lines (sha256 `0ef5eaa748`); tests 78 lines (`7c05b34194`). Reproduce with `git diff a894d9d0ba abcb16fd27 -- <file>`. |

**Baseline:** the brief's pytest command gave **358 passed** (`baseline_pytest.log`).

## 1. E1's disposition and booleans

These are my round-03 probes (`R/REVIEW_RV79/reader_confirm_03/rv79_probes_d25.py`, unchanged; `probes_d25_on_abcb.jsonl`). Each edits the parsed base and re-hashes only the receipt.

| Probe | 07c | 07d (this head) | Ruled |
|---|---|---|---|
| Forged source identity with `source_ref: 0.0` | PASS | **G1 RECEIPT** (1594) | G1 (D32) |
| `source_ref: 0.0` alone | PASS | PASS, identical classifications | PASS (D25) |
| Forged preparation hash with `attempt_ref: 1.0`, with an invocation | G8 | **G1** (1602) | G1 |
| The same, without an invocation | PASS | **G1** (1602) | G1 |
| Noncanonical `kernel_member` (D23) with `source_ref: 0.0` | G5a | **G3** (1627) | G3 |
| Foreign coverage body with `source_ref: 0.0` | G5a | **G3** (1640) | G3 |
| The int-reference controls | G1 | G1 | G1 |

**Booleans** (`rv79_probes_r04.py`, `probes_r04.jsonl`):
- `receipt_version: true` and `case_limit: true` give **G0** (1580, 1582): D2's "absent or of the wrong type".
- `true`/`false` as `charged`, a case `source_ref`, a preparation `attempt_ref`, an old `member`, an `execution_order` index or a record `index` each give **G1 RECEIPT** (1585, closed shape). A JSON boolean is not a number, and the G1 integrity checks are never reached for it.
- `_integral(True)` is None, and `uint()` at G2 also excludes bool.
- `-0.0` in a U field (`charged`, an `execution_order` index) gives **G2 ENCODING**.

**The D32 implementation:**
- `_integral` (553–557) accepts an int, or a finite integral float that is not −0, and never a bool.
- G0 (1580, 1582) and the G1 index sites (1592, 1598) use it.
- `_normalize_integrals(receipt)` (560–569) runs once, after G2 (1603), on the snapshot copy, and the caller's object is unchanged. I checked whether it can alter a value: every numeric field in the schema is a U or I32 with an encoding tag, or an integer enum/const. The only untagged `"type": "number"` is `RawRow.value`, which sits outside the receipt and is not normalized. So G1 and G2 have already proved every normalized float integral, and the `int()` cannot truncate.

**N-e (the harness):** fixed. `apply_mutation` indexes by `int(...)` (test lines 106 and 111), and the shared must-pass `integral_float_integers_and_references` runs through it.

## 2. Diff review `a894d9d0ba..abcb16fd27`

| Change | Lines | Assessment |
|---|---|---|
| D32 `_integral`, plus normalization after G2 | 553–569, 1603 | Correct, and the normalization cannot truncate (above) |
| D32 at G0: version and limits by value | 1580, 1582 | Correct. Booleans and −0 are rejected; 1.0 and 20000000000.0 are accepted. |
| D32 at G1: identity and preparation index sites | 1592, 1598 | Correct; E1 is closed |
| D32 at the G3 sites | 1626, 1637 | Correct, though redundant after the G2 normalization (M28 is equivalent) |
| D31: model `schema_version` ∈ {0.1.0, 0.2.0, 0.3.0} | 1387 | Correct. My probes: 0.1.0, 0.2.0 and 0.3.0 pass with identical classifications; 0.4.0 and "0.1" give G8 INVOCATION (`probes_d31_d33.jsonl`). |
| D33: `verification_estimate` names a force or moment row | 623–624 | Correct. A translation row gives G5 ATTEMPT; a force row passes. |
| Harness: indexing by integral value | test 106, 111 | Correct (N-e) |
| New tests for D31, D32 and D33 | test 608–647 | Present; they kill M23, M26 and M27 (below) |

**Removed lines:** the four `type(x) is int` guards, the G0 type tests (each replaced by a value test) and the old 0.2.0/0.3.0 rule (replaced by D31). **Nothing was weakened.**

**Regression check:** `compare_r04.jsonl` reruns all 48 earlier RV79 probes from rounds 01–03, and all 48 agree with the current rulings. The D28 `verification_estimate` probe now stops at the D33 line, with the same G5 ATTEMPT. The M21 vector still separates (G5a versus G8).

## 3. Mutation testing

`rv79_mutants_04.py` runs the 22 earlier mutants plus seven aimed at D32 (M23–M29). Each run started from the pristine `031334e29f`, which was restored and verified afterwards. Results are in `mutants_04.json`.

| Mutant | Result | Reason |
|---|---|---|
| M01–M04, M06–M11, M14–M16, M18–M22 | **killed (18)** | — |
| M05, M12, M17 | survive | deferred bases (N17 overshoot, budget-failure caching, L = 0) |
| M13 | survives | equivalent: subsumed by the D6c trigger check |
| **M23** `_integral` accepts bool | **killed** | `test_g0_union_d2` |
| **M26** normalization after G2 removed | **killed** | must-pass `integral_float_integers_and_references` |
| **M27** G1 identity site back to a host-int test | **killed** | the shared `forged_source_identity_float_source_ref`, plus the D32 test |
| M24 `_integral` drops its −0 exclusion | survives | Equivalent for single defects. A −0 index fails G2 either way; only a dual defect (−0 plus a forged hash) moves from G2 to G1. |
| M25 normalization skips list items | survives | Equivalent: no site type-tests list elements; every index uses `int()` or `_at`, and Python's `==`, `in`, `sorted` and hashing treat 1.0 as 1 |
| M28 G3 D23 site back to a host-int test | survives | Equivalent: the G2 normalization already made the reference an int |
| **M29** `_integral` truncates non-integral floats | survives | **A genuine gap** (S1) |

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S1** | SHOULD-FIX | tests; `_integral` 556 | No test kills M29. `rv79_m29_vector.py` (`m29_vector_run.txt`): `receipt_version: 1.5`, `case_limit: 20000000000.5` and `invocation_limit: 60000000000.5` each give **G0** in the reviewed reader but **G1** under M29, because the truncated value equals the constant. The existing tests use 2.0 and integral floats only. | Add these three single-defect G0 pins, reader-local or shared. |
| N1 | NOTE | `_encoding` 167–182; `_normalize_integrals` | **−0.0 on the two integer fields the schema writes as enum/const.** These are `G5aError.quantity_kind` (enum [0,1]) and `source_decline.constructor_counts.directional_springs` (const 0). At unit level, `_shape` and `_encoding` accept −0.0 there, and the normalization turns it into 0 (`probes_r04.jsonl`, last two lines). D32 says "not −0", and C1's G2 row says "no −0 counter". No base carries either field, and JCS hashes −0 as 0. | At G2, reject −0.0 on every receipt number (all are integers). Alternatively, have ROOT rule that the enum/const integer fields are exempt. |
| N2 | NOTE | 1626, 1637 | The G3 `_integral` sites are redundant after the G2 normalization (M28 is equivalent), so the G3 assertions in the D32 test exercise the normalization, not those sites. That is harmless. | None |
| N3 | NOTE | — | Carried: N-a (the harness rehashes with reader functions; RV78's independent rehash covers it) and N-b (M05, M12 and M17 need deferred bases). | — |

## Arithmetic

`R/REVIEW_RV79/reader_review_01/rv79_oracle.py` gives 0 mismatches on all 20 checks (`oracle_run_04.txt`).

## For ROOT

- **N1:** whether D32's "not −0" covers the two enum/const integer fields. I recommend a G2 rejection of −0.0 anywhere in the receipt.
- **S1:** a test-only repair; it needs no ruling.

## Evidence (this folder; SHA256SUMS covers every file)

| File | Content |
|---|---|
| `baseline_pytest.log` | 358 passed |
| `probes_d25_on_abcb.jsonl` | the E1 probes (`R/REVIEW_RV79/reader_confirm_03/rv79_probes_d25.py`, unchanged, `a5fc874f5c`) |
| `rv79_probes_r04.py`, `probes_r04.jsonl` | booleans, −0, and unit checks of `_integral`, `_shape`/`_encoding` and the normalization |
| `rv79_probes_d31_d33.py`, `probes_d31_d33.jsonl` | D31 and D33 |
| `compare_r04.jsonl` | all 48 earlier RV79 probes against current rulings (the confirm_02 and confirm_03 scripts, unchanged): 48/48 agree |
| `rv79_mutants_04.py`, `mutants_04.json` | 29 mutants |
| `rv79_m29_vector.py`, `m29_vector_run.txt` | S1 |
| `oracle_run_04.txt` | the oracle rerun |

**Rerun:** from a fresh `git archive abcb16fd27` of READER (P/core, tests, fixtures, schemas), with both BIN variables set, run each script with `VENV/bin/python` from P. Bulk logs are in WT/scratch/rv79_reader_confirm04/.

## Basis read (sha256 prefix)

- T3/ROOT_RULINGS_V1.md (`db351ca375`), from "T1 and T2: I61's analysis and the rulings" to the end (D31–D33, the 07d round, T1 confirmed);
- R/I62/review_repair_07/RETURN_07D.md (`a66f88d924`).
