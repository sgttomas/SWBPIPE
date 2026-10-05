# I67 return: the U7 repair round, part 2 (RV94 S-1, N-2 and N-3)

I67 is a TASK (Type 2) under ROOT. I worked to ROOT's repair-round message, RV94's `u7_01` S-1, N-2 and N-3, and RR "RV94 on U7: PASS; the summary aligned across languages…" (NUM `979130f4e1`). I followed I66's `u7_repair_01` return (`3e956b18…`). I did not delegate.

**Verdict: done. All three items are in. Every control passes; the mutant results are in §4.**
- **S-1:**
  - D-U7-4's description now names the summary.
  - Two `summary` forms with the invocation use a new value, `by_validated_class_current`.
  - Each language asserts its own side: Python and Rust Current (69), TS not-Current (97).
  - The case file stays **v4** (§1).
- **N-2:** TS's reader docstring now carries the Python and Rust wording, plus "standing comes from the carriers (D2 4.9.4)". It is line-neutral.
- **N-3:** a scope clause names the class and each language's code, and all three languages pin it. A shared corpus probe was added, because none existed.
- **Suites:** no existing outcome changes in Vitest, `tsc`, Python's retained suites or `result_export`. The only differences are the new tests.
- **The TS oracle diff:** TS changes on no input, as expected.

## Basis, host and fence

- **Worktree:** `WT/f2a-u7`, at `8c84e7ae14` on `codex/piping-f2a-u7-20261004`. The work is uncommitted, for ROOT to commit.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, 21:00Z to about 21:35Z. The memory guard (PID 5387) ran throughout.
- **Host** (`_run_records/runtime.txt`):
  - No install and no WASM build. The base lane carries byte copies of the worktree's ignored prebuilt `public/` files.
  - Python ran with the I52 CLIs.
  - Cargo: the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time, in `WT/targets/i67-u7f/{re-base9,re-cand9,re-mut9}`.
  - `TMPDIR` was set to `WT/scratch/i67_u6d/tmp`.
  - Nothing native, solver-at-scale or DEC-025.
- **Lanes:**
  - `base9` is a `git archive` of `8c84e7ae14`.
  - `cand9` is `base9` plus the 9 changed files. Its `execution/` is a link to `base9`'s, and the file sets are otherwise identical.
  - `mut9` (TS mutants) and `pyrs9` (Python and Rust mutants) equal the final worktree (`lanes_vs_worktree.txt`).

| File (TS = P/apps/desktop/src) | sha256 | ± | Change |
|---|---|---|---|
| TS/features/results/retainedPrecision.ts | `76b1aa18…` | +2 −2 | N-2 docstring (1,339 lines, line-neutral) |
| TS/features/results/retainedPrecision.test.ts | `385a626a…` | +12 | N-3: TS's code for all three fields; the probe's TS expectation |
| TS/features/results/retainedPrecisionIntegration.test.tsx | `1fac2453…` | +19 −3 | The summary vocabulary, counted independently; D-U7-4's four forms written out, the summary forms equal to their standing twins; the N-3 scope regex |
| P/fixtures/results/retained_precision_carrier_cases.json | `cf82deab…` | +61 −4 | D-U7-4: description, ruling, 2 summary forms. The note defines `by_validated_class_current`; the N-3 scope clause |
| P/fixtures/results/retained_precision_cases.json | `482449bf…` | +115 | One mutation appended: the shared N-3 probe `g7_not_required_quality_enum_invalid` |
| P/tests/test_retained_precision_carriers.py | `dc128cd4…` | +4 −4 | Four one-line changes (§3) |
| P/tests/test_retained_precision_contract.py | `262cd9e7…` | +5 −5 | Corpus pins (278 mutations; two per-reader G7 entries); the docstring |
| P/core/reporting/result_export/tests/retained_precision_carriers.rs | `da36dc1c…` | +4 −4 | Four one-line changes (§3) |
| P/core/reporting/result_export/tests/retained_precision_contract.rs | `1f5bb45c…` | +2 −2 | `mutations.len()` 278; the comment |

Every Python and Rust change replaces an existing line. No line is added or removed.

## 1. S-1: D-U7-4's summary, declared

- **The description** gains: "The summary follows the same split (RV94 S-1)." Each language counts the per-case summary as Current only when its own standing for the requested refs is `numerically_eligible`. So, with the form's invocation and requested refs:
  - Rust and Python give the Current summary (`by_validated_class_current`, 69 withheld on each milestone);
  - TS, without the live native capture, gives the not-Current one (`by_validated_class`, 97).
- **The ruling** cites RV94 S-1, the RR heading, and I66's and my repair records.
- **The forms:** `invocation_without_native_capture:summary` and `stale_current_model_same_case_ids:summary`, on both milestones.
  - Each has exactly its standing twin's inputs (`capture: "none"`, or `current_model_edits`; the fixture invocation; requested `invocation`), with `subject: "summary"`.
  - Expected: Rust and Python `{summary: "by_validated_class_current"}`, TS `{summary: "by_validated_class"}`.
- **The vocabulary** (the note). `by_validated_class_current` is `by_validated_class` with the Current withheld count, the absolute and not-covered rows only. `by_validated_class` is the not-Current count whether or not the form has an invocation. The old parenthetical "(these forms carry no invocation)" is replaced.
- **The version stays v4,** and the note says so. No field is added. Every consumer, old and new, refuses an unknown summary value loudly:
  - Python and Rust require a non-empty expectation;
  - TS requires a listed value.

  So no reader can silently misread the new value.

**Each language asserts its own side:**
- **Python and Rust:** the declared-difference consumers map the new value onto I66's `expected_summary(..., current=True)` / `expected_summary_with(.., true)`, with the form's refs. Each also requires that the Current value is exercised.
- **TS:**
  - The consumer counts the expected summary from the reader's classes, not through the seam.
  - The structure test writes out all four D-U7-4 forms with TS's side, and checks each summary form's inputs equal its standing twin's.
  - Slice F's not-Current test now covers the summary forms too (`withheld` 97, `NATIVE_CAPTURE_REQUIRED`).

## 2. N-2: TS's reader docstring

Before: "Ordered standalone reader; eligibility as in C1:160."

After: "The accepted ordered reader (G0-G8). D-U6-1: every gate runs; since U7 a valid invocation-bound statement of a solved model whose cases are selected or not_required reads eligible. Standing comes from the carriers (D2 4.9.4). Hashes bind the supplied statements; they do not establish producer origin (D-U7-6)."

The second line keeps "No registration, mutable eligibility cache, or private proof replay." The test helper `c160` is unchanged; RV94 called renaming it optional.

## 3. N-3: the clause, the pins and the shared probe

**The clause** (appended to `scope`):

> An invalid enum value in a not_required case's quality is refused at G7 with each language's own code (Python SOURCE_NUMERICAL_CASE_INVALID, Rust SOURCE_NUMERICAL_CASE_INVALID, TS SOURCE_PRODUCER_CONTRACT_UNSUPPORTED, TS's G7 contract check firing first): a value outside the vocabulary of accuracy_evidence, structural_status or model_matrix_fidelity in the numerical_quality case of a not_required receipt case, made hash-consistent (observed for all three fields, RV94 u7_01 N-3; pinned by the shared corpus probe g7_not_required_quality_enum_invalid), so for this class G7 parity compares the gate and each language asserts its own code (ROOT_RULINGS_V1 "RV94 on U7: …"; I67 u7_repair_01).

**The shared probe.** None existed: the only per-reader entry was `g7_maximum_off_enclosure`.
- **What it is:** a corpus mutation appended at index 277, so no existing slice moves. It is 07j's `not_required_second_case_checks_passed`, plus `numerical_quality.cases[1].accuracy_evidence = "estimated"`, with rehash `all`.
- **Expected:** `{G7, SOURCE_NUMERICAL_CASE_INVALID}`. `expected_by_reader` gives Python and Rust the same code and TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, the 06b precedent.
- **Corpus pins:**
  - Python: `(15, 278, 24)`, and the per-reader list gains the probe.
  - Rust: `mutations.len()` is 278.
  - No existing slice tally changes.
- **Snapshot name:** the corpus is no longer exactly 07j. ROOT may want to name it (07k).

**The pins in each language:**

| Language | Scope fence (one line) | Code |
|---|---|---|
| Python | The clause's phrase and "Python SOURCE_NUMERICAL_CASE_INVALID" | The corpus runner checks the probe (`expected`) |
| Rust | The phrase and "Rust SOURCE_NUMERICAL_CASE_INVALID" | `shared_rehashed_first_failure_mutations` (`expected_by_reader.rust`) |
| TS | `/An invalid enum value … own code[^.]*TS SOURCE_PRODUCER_CONTRACT_UNSUPPORTED/` | The corpus runner (`expected_by_reader.typescript`), plus a new test over all three fields on the 07j entry. That test also checks the entry is eligible unedited |

**The Python and Rust lines** (4 in each file, each an edit of an existing line):
1. the scope fence;
2. the summary vocabulary mapping;
3. the coverage check requiring the Current value;
4. **an unplanned one.** I66's `test_d_u7_4_forms_python_side_standing_and_summary` and `u7_d_u7_4_forms_rust_side_standing_and_summary` read `form.expected.<lang>.standing` for every D-U7-4 form. With summary forms present, that failed in my first candidate run (KeyError in Python; 1 failed in Rust; `py_cand`, `re_cand` in `suites/`). The line now accepts each form's own side: standing `numerically_eligible`, or summary `by_validated_class_current`. Both tests now run their standing and summary checks on all four forms. That is wider, not narrower.

ROOT allowed one fence line per consumer. Items 2 to 4 go beyond that. I made them because S-1 asks each language to assert its own side, and because without item 4 two of I66's tests break.

## 4. Controls

**Suites** (`suites/`; base = lane `base9` at `8c84e7ae14`, candidate = `cand9`):

| Suite | Base | Candidate | Change |
|---|---|---|---|
| Vitest | 3,542 / 3,542 | 3,552 / 3,552 | No existing test changes outcome. 10 new: 4 declared summary-form tests, 4 not-Current summary tests on them, the N-3 test, and the probe's corpus test |
| `tsc` | clean | clean | — |
| Python, three retained suites | 462 passed | 463 passed | +1: the probe's corpus test. Every other outcome is identical |
| `result_export` | 171 ok | 171 ok | Identical; the probe runs inside `shared_rehashed_first_failure_mutations` |

**The TS oracle diff** (`oracle/`, `ts_oracle_diff_repair.txt`: PASS):
- **Inputs:** regenerated with I66's `gen_inputs.py`.
  - The base lane gives 381, equal to I66's `104fb714…`.
  - The candidate gives 386: those 381 byte-identical, plus the probe and the 4 summary-form inputs.
- **Setup:** slice F's TS dumper ran in both lanes on the 386 inputs.
- **Results:**
  - **No TS field changes on any input.** TS's only product change is the docstring.
  - On the 4 summary-form inputs, TS reads its declared side: `ipc` standing `needs_recompute` with `NATIVE_CAPTURE_REQUIRED`, and the not-Current summary (97). Its standing rule with the capture held true (`live`) reads Current (69), as Python and Rust do.
  - The probe reads `G7:SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`.

**Mutants.** None is killed only by a compile error.

TS (`mutants/mutants_r9.*`, lane `mut9`, the 4 retained and 9 related TS files):

| Id | Mutation | TS |
|---|---|---|
| S01 | Summary forms removed | killed |
| S02 | TS side flipped to Current | killed |
| S03 | Python side flipped | passes (Python's own test kills it; see below) |
| S04 | Rust side flipped | passes (Rust's own test kills it) |
| S05 | A summary form's inputs differ from its twin | killed |
| N01 | N-3 clause removed | killed |
| N02 | Clause misstates TS's code | killed |
| N03 | Probe removed | killed |
| N04 | Probe's TS expectation flipped | killed |

Python and Rust (`mutants/pyrs_mutants_r9.*`, lane `pyrs9`):

| Mutant | Python | Rust |
|---|---|---|
| control | passes | passes |
| S01: summary forms removed | killed (the Current value is no longer exercised) | killed (`summary:current` missing) |
| S02: TS side flipped | passes, as designed (TS kills it) | passes, as designed |
| S03: Python side flipped | killed | passes, as designed |
| S04: Rust side flipped | passes, as designed | killed |
| N01: clause removed | killed | killed |
| N02p: clause misstates Python's code | killed | passes, as designed |
| N02r: clause misstates Rust's code | passes, as designed | killed |
| N03: probe removed | killed (count pin) | killed (`mutations.len()`) |
| N04p: probe's Python expectation flipped | killed | passes, as designed |
| N04r: probe's Rust expectation flipped | passes, as designed | killed |

**Every mutant is killed by the language or languages whose side it changes,** and none only by a compile error. TS's lane `mut9` equalled the final bytes for every file TS reads. Its Python and Rust test files were resynced after the TS run, which does not read them.

## 5. For ROOT

1. **The corpus gains one entry** (278 mutations). If the snapshot name matters to the basis (07j → 07k), ROOT names it. Its sha256 is now `482449bf…`.
2. **The Python and Rust edits** are four replaced lines per file, beyond the one-line fence (§3).
3. **The version stays v4,** with the reason given in the note (§1).
4. **First-run disclosure:** my first candidate run of the Python and Rust suites failed I66's two D-U7-4 side tests. I repaired that by item 4 in §3 and reran (`py_cand2`, `re_cand2`). The Python and Rust mutants ran only on the final bytes.
