# RV90 round 02: confirmation of the U6e repair round (snapshot 07h)

RV90 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path, and nothing was delegated. This round continues from RV90 round 01 (`R/REVIEW_RV90/u6e_reader_round_01/REVIEW.md`, sha256 `186b2235…`) with that context.

- **Candidate:** `cc4dd61d67` on `codex/piping-f2a-readers-round-20261004`, on top of `5e1e2625ac`.
- **I61's account:** `R/I61/u6e_reader_round_02/` (SHA256SUMS 29 of 29 OK) and `R/I61/u6e_reader_round_01/ADDENDUM_01.md`.

## Verdict: CONFIRMED

All five round-01 findings (S1, N1, N2, N3, N4) and the count correction are **fixed**. Nothing new was found.

**0 BLOCKING · 0 SHOULD-FIX · 0 NOTE** (new findings)

| Item | Status | Evidence |
|---|---|---|
| S1 | **Fixed** | On my 6 malformed `affected_refs` probes, all three 07h readers agree at the gate. Codes differ only on the three unlisted probes, which reach G7, and there each reader's code equals its own 07f G7 code. The related typed-integrity probe now agrees too. Python's 07g predicate is killed by the new shared entry. |
| N1 | **Fixed** | "F5 for case 0 only" is killed in Python, Rust and TypeScript, each only by `f5_ordinary_refs_second_case_relaxed_d6a_form`. |
| N2 | **Fixed** | TypeScript's "length check dropped" and Rust's `exact.starts_with` are each killed only by `f5_ordinary_refs_strict_prefix`. The must-pass control `f5_envelope_reordered_exact_list` passes all three readers at 66/1/6/1, not eligible. |
| N3 | **Fixed** | Rust's probe is now A2's list plus the other-scope element. With the F5 line removed, it is admitted (`left: Null`), so F5 alone refuses it. |
| N4 | **Fixed** | `d37.basis` gains the facade-order premise as a pure suffix, and every other `d37` key is byte-identical. I re-checked the premise in PP source. |
| Count correction | **Fixed** | `ADDENDUM_01.md` corrects §3 to 130, which matches my round-01 rerun. `RETURN.md` is byte-unchanged, and that folder's SHA256SUMS passes 20 of 20. |

**Other checks:**
- No 07g outcome changed, in any of the three readers, except through the 4 new entries.
- The suites pass on 07h: Python 391, Rust `result_export` 164, TypeScript 448, `tsc` 0.
- Every reader matches the corpus on all 315 entries. The completeness flags are still false.

## Scope of the change

`git diff 5e1e2625ac cc4dd61d67` touches 4 files:
- **The Python reader:** only F5's predicate (`P/core/analysis_runs/retained_precision.py:914–916`) is replaced by `isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]`.
- **The Python contract tests:** count pins, the milestone test's helper, and the new S1 test parametrized over string, number and object.
- **The Rust contract tests:** count pins, `snapshot_07h_mutation_outcomes` and the N3 probe.
- **The corpus.**

The Rust and TypeScript reader sources, and `product_physics`, are byte-unchanged from `844448112f`.

**The corpus**, in my own diff of 07g against 07h:
- `version`, `provenance`, `arithmetic` and all 15 `cases` are identical.
- Mutations 0–273 and must-pass entries 0–21 are byte-identical.
- 3 mutations and 1 must-pass entry are appended.
- `d37.basis` is a strict suffix extension, and `d37` is otherwise identical.
- The corpus sha256 is `d0a4ee21…`. The milestone fixtures are byte-unchanged (`ac6986b0…`, `6cd1d249…`).

**I rebuilt the new entries independently.** All three match my round-01 constructions exactly:
- `f5_affected_refs_string_names_no_case` sets `diagnostics[0].affected_refs` to the case id as a string, and the base list still names that id.
- `f5_ordinary_refs_strict_prefix` uses diagnostic 0 moved to the end of the envelope, with A2's list for that envelope minus its last element. The omitted element is the moved load diagnostic, and the integrity diagnostic is still listed.
- The must-pass entry is the same envelope with the full list.
- `f5_ordinary_refs_second_case_relaxed_d6a_form` sets case 1 of `two_case_synthetic` to its integrity diagnostic only.

## S1: the probes on the 07h readers

I regenerated my round-01 probes with the same scripts and applier. The Python 07g reader reproduces round 01's outcomes exactly (equal JSON). Here are the 7 non-array probes, with the 07f and 07g readers for reference (`_run_records/probe_table_07h.md`):

| Probe | 07f (py / rs / ts) | 07g (py / rs / ts) | 07h (py / rs / ts) |
|---|---|---|---|
| string, listed | G7 / G7 / G7 | **G7** / G5 / G5 | G5 / G5 / G5 ATTEMPT |
| string, unlisted | G7 / G7 / G7 | **G5** / G7 / G7 | G7 / G7 / G7 |
| number, listed | G7 / G7 / G7 | **G5 PRODUCT_ATTEMPT** / G5 / G5 | G5 / G5 / G5 ATTEMPT |
| number, unlisted | G7 / G7 / G7 | **G5 PRODUCT_ATTEMPT** / G7 / G7 | G7 / G7 / G7 |
| object, listed | G7 / G7 / G7 | **G7** / G5 / G5 | G5 / G5 / G5 ATTEMPT |
| object, unlisted | G7 / G7 / G7 | **G5** / G7 / G7 | G7 / G7 / G7 |
| typed integrity, string (related) | G7 / G5 / G7 | G7 / G5 / G5 | G5 / G5 / G5 ATTEMPT |

- **The gate agrees in 07h on all 7 probes.** Wherever the gate is G5, the code is `RETAINED_PRECISION_ATTEMPT_MISMATCH` in all three readers.
- **The G7 codes are each reader's accepted per-language code.** Python and TypeScript give `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`; Rust gives `SOURCE_PREVIEW_PHYSICS_ARRAY_INVALID`. Each equals that reader's own 07f code on the same probe.
- **The other 29 probes are unchanged from the 07g readers** (milestone receipts and their 9 variants each, second case, strict prefix, envelope-reordered), apart from the two duplicate string probes above. Both milestone receipts still pass with 25/69/3/1 and 25/69/3/2.
- **Python's 07g predicate (my RV90:PY05) is killed by 4 tests:** the shared `f5_affected_refs_string_names_no_case` and the three cases of `test_f5_non_array_affected_refs_names_no_case_s1`.

## N1 and N2: the round-01 survivors

Each mutant is one edit in a pristine `git archive cc4dd61d67` lane, running that language's full suite. The bytes were restored and compared after every run, and nothing failed to compile or collect (`_run_records/mutants/`).

| Mutant | Round 01 | 07h | Killed by |
|---|---|---|---|
| PY01: F5 for case 0 only | survived | **killed** | `test_shared_draft_first_failure_controls[f5_ordinary_refs_second_case_relaxed_d6a_form]` only (1 test) |
| RS01: F5 for case 0 only | survived | **killed** | `shared_rehashed_first_failure_mutations` with the single failing entry `f5_ordinary_refs_second_case_relaxed_d6a_form`, and the `snapshot_07h_mutation_outcomes` tally (2 of 3; only that entry has `match:false`) |
| TS02: F5 for case 0 only | survived | **killed** | the named test `f5_ordinary_refs_second_case_relaxed_d6a_form` only |
| RS02: F5 as `exact.starts_with(refs)` | survived | **killed** | `shared_rehashed_first_failure_mutations` with the single failing entry `f5_ordinary_refs_strict_prefix`, and the 07h tally (only that entry has `match:false`) |
| TS01: F5 length check dropped | survived | **killed** | `f5_ordinary_refs_strict_prefix` only |
| TS04: F5 `Array.isArray` guard dropped | survived | **killed** | `f5_affected_refs_string_names_no_case` only |
| PY05: S1 reverted to 07g's predicate (new) | — | **killed** | the shared S1 entry and the 3 cases of the S1 local test |

**The must-pass control** `f5_envelope_reordered_exact_list` passes in Python, Rust and TypeScript, both through each suite and through my own applier and dumps. It gives classes 66/1/6/1, not eligible, `needs_recompute`, the same as the unmodified base. So the strict-prefix entry is refused for the prefix alone.

## N3, N4 and the count correction

**N3.**
- Rust's `d6_d7_ordinary_and_diagnostic_relations` (`P/core/reporting/result_export/tests/retained_precision_contract.rs:2444–2457`) now probes the base's exact list plus the other-scope diagnostic. The test also asserts the element is not already listed.
- With F5's line removed from the reader, the test fails at "F5: a listed diagnostic of another scope is refused" with `left: Null`. So F5 alone refuses it.

**N4.** The appended sentence is correct against source:
- `lib.rs:2962` calls `prepare_case`, `:2968` calls `solve_native`, and `:2971` calls `freeze_candidate` only after `solve_native` returns Ok.
- `project_candidate` (`retained_product.rs:3644–3646`) is called only from the `#[cfg(test)]` impls at `:3811` and `:3833`, and from `retained_product_tests.rs` and `retained_wire_tests.rs`, which `lib.rs:124–127` declares `#[cfg(test)]`.
- `product_physics` is unchanged from `844448112f` to `cc4dd61d67`.
- No reader test reads `basis`.

**The count correction.**
- `ADDENDUM_01.md` replaces RETURN §3's "197" with **130** for `PY_M50_statement_normalized`. That matches I61's own `mutants_py.json` and my round-01 rerun (130).
- `RETURN.md` and every other file in that folder are byte-unchanged since NUM `3f07f1d927`. Only the addendum and one SHA256SUMS line were added, and SHA256SUMS passes 20 of 20.

## No unintended outcome change

As in round 01, I applied every entry myself (RV90's `helpers.py` applier and rehash) and fed the documents to each reader through its public entry. For Rust and TypeScript, the 07h reader code is byte-identical to the 07g code, and its outputs on 07g equal my round-01 outputs exactly.

| Comparison | Python | Rust | TypeScript |
|---|---|---|---|
| 07g reader on 07g against 07h reader on 07h: changed 07g entries | 0/311 | 0/311 | 0/311 |
| New entries in 07h | the 3 mutations are G5 ATTEMPT; the must-pass passes | same | same |
| 07g reader against 07h reader, both on 07g | 0 differences | identical code | identical code |
| 07g reader against 07h reader, both on 07h | only `f5_affected_refs_string_names_no_case` (07g gives G7, 07h gives G5) | identical code | identical code |
| 07h reader on 07h against the corpus expectation | 0 misses of 315 | 0 of 315 | 0 of 315 |
| Any `numerical_eligible: true` | none | none | none |

**Cross-reader parity on 07h.** Every entry agrees, except the ruled per-reader G7 entry `g7_maximum_off_enclosure`.

## Suites on 07h (`_run_records/test_runs_r2.txt`)

| Reader | Result |
|---|---|
| Python (contract and schema) | 391 passed |
| Rust `result_export` | 164 passed over 12 binaries (`retained_precision_contract`: 61) |
| TypeScript vitest | 448 passed |
| TypeScript `tsc --noEmit` | exit 0 |

The completeness flags are still false: `_IMPLEMENTATION_COMPLETE = False` (`PY:30`), `IMPLEMENTATION_COMPLETE: bool = false` (`RS:4269`) and `SUMMARY_COVERAGE_COMPLETE = false` (`TS:97`).

I did not rerun ROOT's 24-file Python sweep (1,774 passed and 3 failed on branch order). That is outside this confirmation.

## For ROOT

Nothing to rule on. Two optional observations:
- **The Python-local S1 test pins the number and object forms**; the only shared S1 entry is the string form. All three readers agree on all three forms (table above), so adding shared number and object entries would only harden parity.
- **Python's `resolves` still reads `… or []`.** I61 notes this, and it is unreachable for a different outcome, because F5 has already fixed the listed refs to arrays that name the case. I agree.

## Host and method disclosures

**Host.**
- **Memory guard:** PID 5387 was running throughout, and each Rust mutant run checked it.
- **Cargo:** default toolchain (1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time. Targets are under `WT/targets/rv90/`.
- **Python:** the checked-JSON and units CLIs were rebuilt from the 07h archive (`--locked --offline --release`). The interpreter is the existing project venv.

**TypeScript.**
- **Disclosure:** REPO_ROOT's `P/node_modules` is symlinked into each copy, and the prebuilt `wasm-engine` and `self-weight-engine` were copied from `WT/f2a-readers`. Nothing was built or installed.
- A real `apps/desktop/node_modules` directory holds the vite caches.

**Review-copy harnesses.** `rv90_dump.rs` and `rv90_dump.test.ts` were added only to the 07h copy, after its suites ran. They were never in the mutant lane during suite runs.

**Writes.** Only these locations were written:
- `WT/rv90/` (h, g, mut);
- `WT/targets/rv90/`;
- `WT/scratch/rv90_u6e/r2/`;
- this folder.

**Not done.** No Git writes: reads only, with `GIT_OPTIONAL_LOCKS=0`. No installs, and no solver, native or DEC-025 jobs.

**Copies.** The copies and targets are deleted after this report.

**Time.** 2026-10-04, from about 04:57 to 05:10 local (MDT).

## Records (`_run_records/`)

- **Scripts:**
  - `compare_r2.py`, `mutants_r2.py`;
  - from round 01: `helpers.py`, `apply_corpus.py`, `py_validate.py`, `apply_probes*.py`, `rv90_dump.rs`, `rv90_dump.test.ts`.
- **Outputs:**
  - `compare_r2_out.txt`, `probe_table_07h.md`, `test_runs_r2.txt`;
  - `outcomes/*.json`, covering the Python 07g and 07h readers and the Rust and TypeScript 07h readers, on 07g, 07h and the probes;
  - `mutants/*.json`.

The applied JSONL documents are regenerable and not kept. The probe set's sha256 was `82c2c992…`.

`SHA256SUMS` covers every file in this folder.
