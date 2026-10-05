# I66 return: the U7 repair round, RV94 S-1 (the summary aligned with the standing)

I66 is a TASK (Type 2) under ROOT. This round follows ROOT's message "a U7 repair round from RV94 S-1", RV94's `u7_01` S-1 and N-4, and RR "RV94 on U7: PASS; the summary aligned across languages…" (NUM `979130f4e1`). I66 did not delegate.

**Verdict: done, with no stop.**
- Python and Rust now count the summary as Current **only when the standing with the caller's requested refs is eligible.** This mirrors TS's `e5e1693ceb`.
- **The oracle diff passes.** Only `summary` changes, on exactly 6 inputs, in both languages.
- Every suite matches the base apart from the new tests.
- All 8 mutants are killed.
- The Rust change is **line-neutral**.

## Basis, host and fence

- **Worktree:** `WT/f2a-u7` at `ffe65ef203`, uncommitted. No Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **Files I did not touch:** I61's and I65's concurrent PP edits (`lib.rs`, `retained_facade_tests.rs`, `retained_memory.rs`), the case file (I67's next), and I67's `node_modules` link.
- **When:** 2026-10-04, about 20:25Z to 21:00Z. The memory guard (PID 5387) was checked before every run.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time, in my own dirs under `WT/targets/i66-u7f/`. Stale is `RUSTFLAGS=--cfg=i61_u3g2_stale`.
- **Python:** `REPO_ROOT/P/.venv` with the I52 CLIs.
- **Not run:** native, solver or DEC-025 jobs, and no install.
- **Lanes,** under `WT/scratch/i66_u7r/`:
  - `base` is a `git archive` of `ffe65ef203`;
  - `cand` is the worktree copy used for the Rust dump;
  - `candx` is base plus only my two product files, for PP and the runner, so that the concurrent PP comment edits stay out;
  - `mut_py` and `mut_rs` are the mutant lanes.

| File | sha256 | Change |
|---|---|---|
| P/core/analysis_runs/compatibility.py | `920d6859afa958eb5bdcb6287a8ec832eca3a7ac7600c02c878c0b6aad14dc36` | `classification_summary(source, invocation=None, requested_basis_refs=None)`; `_classification_summary_from(validation, source, requested_basis_refs)` |
| P/core/reporting/result_export/src/semantic_contract.rs | `fbbc1a165d2008275c13771b89396e4e975d581de0b5826826316ed73730818a` | `classification_summary(source, invocation, requested_basis_refs: &[Value])`; `classification_summary_from(validation, source, requested_basis_refs: &[Value])`. **Line-neutral** |
| P/tests/test_retained_precision_carriers.py | `49ac611bec69502502775ff48b148475a6206d1cbb4e3e165894af0d28d2e5aa` | Pins moved, 2 new tests, and the consumer passes the form's refs |
| P/core/reporting/result_export/tests/retained_precision_carriers.rs | `d419e52e113bd662bd41e5ed63e0d22660e6aeb74b526da7e656149491c0e215` | Pins moved, 1 new test, and the consumer passes the form's refs |
| P/core/reporting/result_export/tests/retained_precision_contract.rs | `6a06c61f83fc6d009d04f8fdfa13c3bd16b90659c513447fece8cc318de6a5d9` | 1 new test (07j) |

## 1. The alignment

**Before:** Python and Rust took the summary's requested cases from the invocation's own model. So an invocation-bound statement read with other requested refs, or with none, stood `needs_recompute` while its summary was Current (69 withheld). That is RV94's N-4.

**Now:**
- The summary takes the **caller's** requested refs.
- `withheld` is the Current count only when `_retained_standing_from` / `retained_standing_from` with those refs is `numerically_eligible`.
- With no refs, the default, or with other refs, it is the not-Current count.
- **That is TS's rule:** `classificationSummaryFrom(..., eligible ? requestedRefs(model) : [])`.
- **The invocation now only binds the validation.**

The **one product-facing entry** in each language gains the parameter. Neither language has a non-test caller (RV94 N-4). The private seams take the refs in place of the invocation.

**Rust line-neutrality.** `semantic_contract.rs` stays at 855 lines, and its hunks are `-660,5 +660,5`, `-666 +666`, `-675 +675` and `-678,10 +678,10`.
- **What filled the lines:** the 9 lines that derived refs from the invocation, plus the `current` line, became an 8-line comment and a 2-line `current` binding. That binding borrows the caller's slice and builds no list.
- **Rule keys:** these functions sit in a region G7 already classes unreachable on D1 (QUALIFICATION_G7 rows 22–24, `semantic_contract.rs` 549–715). No Pass B rule key or premise pin is on these lines.
- **Untouched:** `retained_precision.rs`, including :4252, :4253 and :4305.
- **The seam guard's pinned bare-name counts in `semantic_contract.rs` hold:** `retained_standing_from` 3, `classification_summary_from` 2.

## 2. Tests

All are in both languages unless marked.

- **Both D-U7-4 forms, both modes:** `test_d_u7_4_forms_python_side_standing_and_summary` and `u7_d_u7_4_forms_rust_side_standing_and_summary`. Each language asserts its own side:
  - with the invocation and the form's requested refs, `numerically_eligible`;
  - the Current summary, 69 withheld, equal to the absolute count;
  - with other requested refs, `needs_recompute` and the not-Current summary, 97.
  
  TS's side (`needs_recompute`, 97) is pinned in TS. Declaring it in the case file is I67's next step.
- **Other requested refs, on the milestone, both modes:** standing `needs_recompute` and the not-Current summary. Also, with no refs, and without the invocation, the summary is not-Current. These are in `test_classification_summary_counts_validated_classes` and `u6a_classification_summary_counts_validated_classes`. The seam calls now pass refs too.
- **07j** (`not_required_second_case_checks_passed`, the cases `[selected, not_required]`): `test_07j_not_required_case_omitted_or_reordered_is_not_current` and `u7_07j_not_required_case_omitted_or_reordered_is_not_current`.
  - With the receipt's order, the standing is `numerically_eligible` and the summary is Current.
  - With the not_required case **omitted**, or the two **reordered**, the standing is `needs_recompute` and `withheld` is **[73, 0]**, the not-Current count.
  - Before this repair it was [1, 0] in Python and Rust.
- **The declared-difference consumers** pass the form's requested refs to the summary. The `none:summary` forms carry no invocation, so their values are unchanged.
  - Both consumers gain `expected_summary(..., current)`, so I67 can map the new Current vocabulary value onto it.
  - An unknown value still fails loudly.

## 3. Controls

**1. The oracle diff, rerun** (`oracle/`).
- **Inputs:** 381, generated from `ffe65ef203`'s fixtures, so 07j's must-pass entry is included (`inputs_sha256.txt`).
- **Setup:** each language's dump runs in the base lane (`ffe65ef203`) and in the candidate. It adds the token with the invocation's own cases, which is the old summary's rule.
- **Result: PASS in both languages** (`oracle_diff_repair.txt`):
  - **only `summary` changes;** no other field changes on any input;
  - **it changes on exactly the expected 6:** `milestone|{sparse,dense}|other_requested` and `carrier|{mode}:no_requested` / `:other_requested`. On each, `withheld` goes 69 → 97, with the token `needs_recompute`;
  - **the candidate's summary is Current if and only if the carrier token is `numerically_eligible`,** on every passing input (0 violations). The base followed the old rule with 0 violations.
- **D-U7-4's forms and the `…:invocation` cases are unchanged:** Current, because their requested refs equal the invocation's cases. That is the residual TS difference I67 declares.

**2. Suites** (`suite_totals.txt`, `suites/`, `sweep.compare.txt`).

| Suite | Base `ffe65ef203` | Candidate | Change |
|---|---|---|---|
| Python 24-file sweep plus the retained schema and carrier suites (the 24 include the contract suite) | 1,846 passed, 30 skipped | 1,848 passed, 30 skipped | +2 new Python tests. No outcome changed |
| result_export | 169 ok (+1 ignored: my dump harness, in the base lane only) | 171 ok | +2 new Rust tests. Every existing outcome identical |
| PP registered (D1 crate changed, so run) | 705 ok, 1 failed (t13), 9 ignored | identical | — |
| PP Stale | identical | identical | — |
| runner/headless | 85 ok, 2 failed (the known `load_reference` pair) | identical | — |

PP and the runner ran on `candx` (base plus my two product files). The concurrent PP comment edits in the worktree are not part of this evidence.

**3. Mutants** (`mutants.py`, `mutants_py_phase.json`, `mutants_rs_phase.json`). **8 run, 8 killed by failing tests, none by a compile or syntax error.** The lanes passed their baselines first (Python 28/28, Rust 78/78).

| Mutant | Python | Rust |
|---|---|---|
| Condition removed: the statement's own cases decide Current again, the old rule (P1, R1) | killed | killed |
| Always Current (P2, R2) | killed | killed |
| Never Current (P3, R3) | killed | killed |
| The caller's refs dropped at the entry (P4, R4) | killed | killed |

## 4. Notes

- **API.** Python's and Rust's `classification_summary` gain a requested-refs argument. Rust's is required; Python's defaults to none, which means not Current. RV94 found no non-test caller in either language, so nothing else needed changing.
- **For I67** (the case file, next):
  - D-U7-4's summary forms with an invocation should expect Current (69) for Python and Rust, given the form's requested refs, and not-Current (97) for TS;
  - both consumers already pass the form's refs and have `expected_summary(..., current)` ready for the new vocabulary value.
- **For RV94's confirmation:** N-4 is resolved in Python and Rust. Their summary no longer says Current when the standing does not.

## Records

- `_run_records/` holds:
  - the runners, the PP and runner chain, and the mutant harness;
  - `oracle/`: the generator, the Python dumper, both Rust dump harnesses, the comparer, the four dumps and the inputs' sha256;
  - `suites/`: base and candidate outcomes;
  - the sweep comparison and the mutant results;
  - the candidate diff (my five files) and their hashes.

  All use placeholder paths.
- SHA256SUMS covers this folder.
