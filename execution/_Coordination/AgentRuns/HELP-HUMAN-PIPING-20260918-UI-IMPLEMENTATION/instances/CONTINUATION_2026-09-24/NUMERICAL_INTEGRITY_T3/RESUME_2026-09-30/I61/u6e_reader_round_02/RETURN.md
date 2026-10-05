# I61 RETURN: U6e repair round (RV90 S1, N1, N2, N3, N4; snapshot 07h)

**Status: complete, with no stop.** All items are uncommitted in WT/f2a-readers-round, on top of `5e1e2625ac`.
- **S1 is repaired.** Python's F5 now treats a non-array `affected_refs` as naming no case, as Rust and TypeScript do. On RV90's six probes all three readers agree at the gate, and at the code wherever the gate is not G7.
- **Snapshot 07h adds 3 mutations and 1 must-pass entry,** and appends the facade-order premise to `d37.basis` (N4). Nothing else in the corpus changes (asserted).
- **All three readers pass 07h in full,** and each matches the corpus expectation on every entry.
- **No 07g outcome changes.** The only differences are the 4 new entries.
- **The milestone receipts are byte-unchanged** and still pass with 25/69/3/1 and 25/69/3/2. The completeness flags are still false.
- **RV90's 6 surviving F5 mutants are all killed,** each by the new entry aimed at it. With the S1 revert, the cross-reader analogues and five re-expressed round-1 F5 mutants, 14 of 14 are killed and none fails to compile or collect.
- **N3 (optional) is done.** **Item 6** is recorded as `u6e_reader_round_01/ADDENDUM_01.md`, following the U5 precedent.
- **U4 G5 part 2 has not returned to me during this round,** so there is no boundary state to report.

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT, with no descendants. The grant is ROOT's message, together with RV90's review (`R/REVIEW_RV90/u6e_reader_round_01/`, at NUM `a45a3201f8`), which I treated as read-only.
- **Time:** 2026-10-04, about 10:10Z to 10:42Z.
- **Host:**
  - The memory guard (PID 5387) ran throughout.
  - Cargo ran with `--locked --offline`, `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`, one job of mine at a time, each under a 1200 s perl alarm.
  - Another agent's cargo job (I65, under `targets/i65-g5`) was running on the host during my first Rust run. I did not touch it.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`. I used `git archive 5e1e2625ac` to make the scratch base, the mutant tree and the TypeScript lanes.
- **The non-Rust tooling, all existing:**
  - **Python:** pytest. The checked-JSON and units authorities are I52's prebuilt binaries in WT/targets/i52-readers, as before.
  - **TypeScript:** vitest and tsc in scratch lanes. P's `node_modules` is symlinked from the main checkout, and the prebuilt `public/` is copied from the U6e lane, itself a copy of the U1 lane. Nothing was built or installed.
- **Writes:**
  - four files in WT/f2a-readers-round: the Python reader, the Python and Rust contract tests, and the corpus;
  - WT/scratch/i61_u6e_07h/ (`old` base archive, `stage`, `mut`, `lane`, `lane_base`, `probes`, logs);
  - WT/targets/i61-u6e/;
  - this folder;
  - `u6e_reader_round_01/ADDENDUM_01.md`, plus one appended line in that folder's SHA256SUMS.
- **Other agents' files:** none touched.

## 1. S1: Python's F5 membership is a list test

**The change** is at `P/core/analysis_runs/retained_precision.py:914–916`, in `_g5_ordinary`:

```python
fail(refs == [d["id"] for d in diags if isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]
              and not str(d.get("code")).startswith("RETAINED_PRECISION_")])
```

This is ROOT's predicate verbatim. A string, an object or a number in `affected_refs` now names no case. The reader no longer does a substring test, a key test or raises a `TypeError`.

**The shared pin:** `f5_affected_refs_string_names_no_case`, on base `ordinary_prepared_synthetic`.
- The first load diagnostic's `affected_refs` is set to the case id as a string, and the base list still names it.
- Expected: G5 `ATTEMPT_MISMATCH` in all three readers.
- It kills Python's 07g predicate (G7 instead), RV90's TS04 (`Array.isArray` guard dropped) and a Rust string-equality analogue (§5).

**The Python-local pin:** `test_f5_non_array_affected_refs_names_no_case_s1`, parametrized over string, number and object.
- Listed, each is G5 `ATTEMPT_MISMATCH`.
- Unlisted, F5 passes and G7 refuses the malformed diagnostic (`SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`), as every reader did before F5.
- The number and object forms are Python-specific risks (a key test, and an exception that fell back to `PRODUCT_ATTEMPT`). The single shared entry does not cover them.

**RV90's probes on the 07h readers.** The full table, with the 07g readers alongside, is `_run_records/probes/probe_table.md`.

| Probe | Python 07h | Rust 07h | TypeScript 07h |
|---|---|---|---|
| string, listed | G5 ATTEMPT | G5 ATTEMPT | G5 ATTEMPT |
| string, unlisted | G7 EVIDENCE_INVALID | G7 ARRAY_INVALID | G7 EVIDENCE_INVALID |
| number, listed | G5 ATTEMPT | G5 ATTEMPT | G5 ATTEMPT |
| number, unlisted | G7 EVIDENCE_INVALID | G7 ARRAY_INVALID | G7 EVIDENCE_INVALID |
| object, listed | G5 ATTEMPT | G5 ATTEMPT | G5 ATTEMPT |
| object, unlisted | G7 EVIDENCE_INVALID | G7 ARRAY_INVALID | G7 EVIDENCE_INVALID |
| typed integrity, string (RV90's related probe) | G5 ATTEMPT | G5 ATTEMPT | G5 ATTEMPT |

- **The gate agrees on all 7 probes.** Every code agrees except the three unlisted G7 probes, which carry the readers' accepted per-language G7 codes (06b settlement). These are the same codes the 07f readers gave.
- **The typed-integrity probe diverged at the 07f base** (Rust G5; Python and TypeScript G7). Under F5 all three now refuse it at G5, because F5 runs before the typed reference in each reader.
- **One Python predicate is left as it was:** `resolves` in `_g5_ordinary` (`:928`) still reads `by_id[ref].get("affected_refs") or []`. It is reached only for a ref in `listed`, which is the F5-checked list, so the ref's `affected_refs` is always a list that names the case. The `or []` cannot change an outcome there. I left it unchanged as outside S1.

## 2. N1 and N2: F5 on a later case, and a strict prefix

**N1:** `f5_ordinary_refs_second_case_relaxed_d6a_form`, on base `two_case_synthetic`.
- Case 1's list (`case:zero-load`) is reduced to 07f's relaxed form: its integrity diagnostic only.
- Expected: G5 `ATTEMPT_MISMATCH`. Every 07g reader refuses it.
- "F5 for case 0 only" admits it, in every reader.

**N2:** `f5_ordinary_refs_strict_prefix`, on base `ordinary_prepared_synthetic`. This is RV90's construction.
- The first load diagnostic is moved to the end of the envelope, so A2's list becomes the other five loads, then the integrity diagnostic, then the moved load.
- The entry lists that list without its last element. This is a strict prefix that still lists the integrity diagnostic, so only F5's length or equality can refuse it.
- Expected: G5 `ATTEMPT_MISMATCH`.

**The control:** the must-pass entry `f5_envelope_reordered_exact_list` lists the exact list over the same reordered envelope.
- It passes all three readers with the base's classifications: 66/1/6/1, not eligible, `needs_recompute`.
- So the strict-prefix entry is refused for the prefix alone, never for the reordering.

## 3. N4: the facade-order premise in `d37.basis`

The corpus `d37.basis` now ends with this sentence:

> Facade order (PP/core/product_physics/src/lib.rs:2962-2974, unchanged from 844448112f to 5e1e2625ac): prepare_case :2962, then solve_native :2968, and freeze_candidate :2971 runs only after solve_native returns Ok; the only other caller, project_candidate (retained_product.rs:3644-3646), is reached only from #[cfg(test)] code. So capture (b) never sees Native not entered, and C--------- is not a `capture` record.

- **I checked it in the source.** The only non-test call of `freeze_candidate` is `lib.rs:2971`. `project_candidate` is called only from `#[cfg(test)]` impls (`retained_product.rs:3810–3847`) and from `retained_product_tests.rs`.
- **`product_physics` is byte-identical** between `844448112f` and `5e1e2625ac`.
- **Nothing else in `d37` changed** (asserted: stage order, marks, records, kinds and unknown kinds). The readers' D37 tests do not read `basis`.

## 4. N3: Rust's D6a probe isolates the other-scope element

In `d6_d7_ordinary_and_diagnostic_relations` (`P/core/reporting/result_export/tests/retained_precision_contract.rs:2444`):
- the probe now lists the base's exact list plus the other-scope diagnostic, so that element is the only defect;
- it is still G5 `ATTEMPT`, and the next assertion still admits the exact list alone.

The old `[integrity, other]` form was also 07f's relaxed form, so F5 refused it with or without `other`.

## 5. Controls

| Control | Result |
|---|---|
| All three readers pass 07h in full | **Python** 391 passed (contract and schema; 384 on 07g, plus 3 new mutations, 1 must-pass and the 3-case S1 test). **Rust** `result_export` 164 passed over 12 binaries (163 on 07g, plus `snapshot_07h_mutation_outcomes`; `retained_precision_contract` now 61). **TypeScript** vitest 448 passed (444 plus the 4 entries), and `tsc --noEmit` exits 0 |
| Parity on every entry | Each reader asserts the corpus outcome on all 277 mutations and passes all 23 must-pass entries and 15 bases. The per-reader G7 entry keeps its `expected_by_reader`. Python's per-entry outcomes: `PYTHON_OUTCOMES_07H.json`, with 277 of 277 and 23 of 23 matched. Rust's 07h slice: `rust_outcomes_07h.txt`, 3 of 3. TypeScript names one test per entry |
| No 07g outcome changes except through the new entries | Python's 07g reader on 07g against the 07h reader on 07h: only the 4 new entries differ. Both readers on 07g: 0 differences. Both readers on 07h: only `f5_affected_refs_string_names_no_case` differs (07g's reader gives G7) (`OUTCOME_DELTA_07G_07H.json`). The Rust and TypeScript readers are byte-unchanged (`changed_files_sha256.txt`), and every 07g entry and its expectation is byte-identical in 07h |
| Milestone receipts unchanged | Both fixtures are byte-identical (`ac6986b0…`, `6cd1d249…`). All three 07h readers pass them with 25/69/3/1 and 25/69/3/2 (probe table). Each reader's milestone F5 test still passes |
| Flags stay false | `_IMPLEMENTATION_COMPLETE = False` (`PY:30`), `IMPLEMENTATION_COMPLETE: bool = false` (`RS:4269`) and `SUMMARY_COVERAGE_COMPLETE = false` (`TS:97`) |
| New entries kill RV90's survivors | See the mutant table |
| No check weakened or removed | One check changed: S1 narrows what *names* a case to arrays, and so refuses more inputs in Python. Every other check and test is kept, and the N3 probe is stricter. No reader loses a refusal on any probe or corpus entry |

**Mutants** (`_run_records/mutants_07h.py` and `mutants_07h_*.json`; one edit each; each language's full suite; pristine bytes restored and compared after every run):

| Mutant | Lang | Result | Killed by |
|---|---|---|---|
| RV90:PY01, F5 for case 0 only (N1) | py | killed | `f5_ordinary_refs_second_case_relaxed_d6a_form` only |
| RV90:RS01, F5 for case 0 only (N1) | rs | killed | the shared mutation test and `snapshot_07h_mutation_outcomes`, at that entry only |
| RV90:TS02, F5 for case 0 only (N1) | ts | killed | `f5_ordinary_refs_second_case_relaxed_d6a_form` only |
| RV90:RS02, F5 as `exact.starts_with(refs)` (N2) | rs | killed | the shared mutation test and the 07h slice, at `f5_ordinary_refs_strict_prefix` only |
| RV90:TS01, F5 length check dropped (N2) | ts | killed | `f5_ordinary_refs_strict_prefix` only |
| RV90:TS04, F5 `Array.isArray` guard dropped (S1) | ts | killed | `f5_affected_refs_string_names_no_case` only |
| I61:PY05, S1 reverted to 07g's `name in (… or [])` | py | killed | that shared entry and the 3 cases of the S1 local test (4 tests) |
| I61:PY06, F5 prefix-tolerant (`exact[:len(refs)]`, the Python analogue of N2) | py | killed | `f5_ordinary_refs_strict_prefix` only |
| I61:RS06, a string `affected_refs` equal to the case id names it (the Rust analogue of S1) | rs | killed | the shared mutation test and the 07h slice, at `f5_affected_refs_string_names_no_case` only |
| I61:PY01, F5 removed (re-expressed on the edited line) | py | killed | 14 tests |
| I61:PY02, F5 includes RETAINED_PRECISION_* (re-expressed) | py | killed | 201 tests |
| I61:PY03, F5 order-free (re-expressed) | py | killed | `f5_ordinary_refs_out_of_envelope_order` |
| I61:PY04, F5 case filter removed (re-expressed) | py | killed | 201 tests |
| I61:RS01, F5 removed (rerun, because N3 changed its probe) | rs | killed | 5 tests, including `d6_d7_ordinary_and_diagnostic_relations`, so the sharpened probe now depends on F5 alone |

Each of RV90's six survivors is killed by exactly the new entry aimed at it, and by no other test. The trees were `git archive 5e1e2625ac` with the four candidate files overlaid, and were confirmed identical to WT before the runs. I did not rerun round 1's other 17 mutants: Python's M33–M39 and PY_M50, Rust's RS02–RS05, and TypeScript's TS01–TS05. They touch lines this round leaves byte-unchanged. Their killing tests are unchanged too, and the corpus only gains entries.

## 6. Item 6: the U6e RETURN count

`u6e_reader_round_01/ADDENDUM_01.md` corrects §3: Python's M50 analogue is killed by **130** tests, not 197.
- 130 is the `killing_test_count` already recorded in that folder's `mutants_py.json`. 197 belongs to PY02 and PY04.
- RETURN.md's bytes are unchanged, so RV90's reviewed hash still holds. The addendum's line is appended to that folder's SHA256SUMS, as was done for U5's ADDENDUM_01.

## Records (`_run_records/`)

- **Corpus:** `build_07h.py` (staging, with every unchanged part asserted) and `snapshot_07h.py`. The snapshot itself is `../SHARED_SNAPSHOT_07H.json`.
- **Outcomes:** `outcomes.py`, `delta.py`, `PYTHON_OUTCOMES_07H.json`, `OUTCOME_DELTA_07G_07H.json` and `rust_outcomes_07h.txt`.
- **Probes:**
  - `probes.py` generates them (applied JSONL sha256 `20eb3d792acf…`, regenerable, not copied);
  - `py_dump.py`, `i61_dump.rs` and `i61_dump.test.ts` are scratch-only dump harnesses, never in WT;
  - `probe_table.py` and `probes/` (6 outcome files and `probe_table.md`).
- **Mutants:** `mutants_07h.py` and `mutants_07h_*.json`.
- **Candidate:** `candidate_code.diff` (code and tests against `5e1e2625ac`; the corpus is covered by `build_07h.py` and its hash), `candidate_status.txt`, `changed_files_sha256.txt` and `test_runs.txt`.
