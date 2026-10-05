# I66 return: the U6 repair round after RV88 (U6a, U6b, U6c)

I66 is a TASK (Type 2) under ROOT and the owner of U6. This round follows ROOT's message "RV88 passed U6a, U6c and U6b" and RR "RV88 on U6a, U6c, U6b (and U6d): all PASS; repair rounds and the shared declared-difference cases" (ROOT_RULINGS_V1). I66 did not delegate.

**Verdict: all seven items are done, with no stop.**
- Every control passes.
- All 29 new mutants are killed by failing tests. They include RV88's survivors R01, R02, R05, R06, Q02 and Q06, and the W06 schema mutant.
- **No existing outcome changed:**
  - the 24-file sweep (plus the schema and carrier tests);
  - result_export;
  - the 5 other Python consumer suites;
  - the 62-document carrier sweep, which has 0 differences.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers` on the merged head `924c6284cb` (which includes U6e), uncommitted.
  - No Git writes. Git reads used `GIT_OPTIONAL_LOCKS=0`.
  - The base lane is a `git archive` of `924c6284cb`: all of P except execution records, but including the PKG-15 handoff fixtures that two sweep files read.
- **When:** 2026-10-04, from about 11:40Z to 12:20Z. The memory guard (PID 5387) was checked before every run.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time. Target directories:
  - `WT/targets/i66-u6a` for the candidate;
  - `i66-u6r-base` for the base;
  - `i66-u6r-mut` for the mutant lane.
- **Python:** `REPO_ROOT/projects/chirality-piping/.venv`, with the I52 CLIs and `PYTHONDONTWRITEBYTECODE=1`.
- **Not run:** npm, installs, new tooling, native, solver and DEC-025 jobs.
- **Scratch:** `WT/scratch/i66_u6_repairs/`, holding `base` and `mut`.
- **The fence held: 7 files,** all named by the seven items.

| File | sha256 | Item |
|---|---|---|
| P/core/analysis_runs/compatibility.py | `da1adacf6bc30a0b5c4ad662cb6aa6e613d1397f766489a502ed2fcf03281de8` | 1 |
| P/core/analysis_runs/records.py | `5eb5e5a5c9021dce7fa0905f1a323b1ccd2253a9e44a75b8c4adad66359446e0` | 1 |
| P/core/reporting/result_export/src/derivative.rs | `85d22b77bf9c2270495ba558e9eff3a9ce25fb0b89b75f3cf0da24caa70fb9a4` | 2 |
| P/core/reporting/result_export/tests/retained_precision_carriers.rs | `12899e2113607edcf082d047a4139e5da8ced1df7a68796907e05293ee50e98e` | 2, 3, 6, 7 |
| P/fixtures/results/retained_precision_carrier_cases.json | `21b158dec2ea3268245a76a45c21afc3da7612e31d292b357b5511b56d5eb96c` | 1, 7 |
| P/tests/test_retained_precision_carriers.py | `7ae883eb3ca8f795bbaae71b81efb462ae88b829c690b45034c13d4d5f7f6799` | 1, 4, 7 |
| P/tests/test_retained_precision_schema.py | `2f8b51b550f5accecc04ebaffb1451cee1118d1b6e022546ee40e4a4ddbe69cc` | 5 |

## 1. U6b S-1: the legacy token row (repaired)

- **The fix in `compatibility.py`:**
  - A new `_has_retained_rows(source)` checks whether any `results` row carries `recovery_method == contribution_preserving_multiprecision_v1`.
  - `_source_contract`'s 0.1.0 branch now runs it before returning, on the raw path only (`check_receipt`), and refuses with `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. This is Rust's order: the metadata check, then `forbid_retained_rows`.
  - The 0.2.0 raw guard now calls the same helper, with unchanged behaviour.
  - The header-only transport path still reads no rows, as Rust's `for_source_metadata` does.
- **The other two Python paths that skip dispatch:**
  - the explicit 0.2 constructor refuses token rows with `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN`, as it does a receipt member (F-U6b-3);
  - the 0.1.0 wrapper (`records.py`) refuses them with `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, as it does a receipt (D-U6-9).
- **Result:** a legacy 0.1.0 source with the token on one row (the last) is refused by the following, and its standing is `unsupported`:
  - raw dispatch;
  - `build_analysis_run`;
  - `build_analysis_run_v0_2`;
  - the 0.1.0 wrapper.

  Another method string is still accepted everywhere.
- **The test:** `test_legacy_sources_with_a_token_row_are_refused_on_every_python_path`.
- **Shared cases.** The same forms are now shared cases (§7), refused identically by Rust and Python. TS's expectations are the same.

## 2. U6a S-2: the disclosure names the SI unit (repaired)

`class_disclosure(kind, unit, class)` now takes the row's unit. Both callers, derive and validate, pass `row["unit"]`, so the pair stays exact.

**The exact new text** of an `absolute_verified` disclosure, which TS must match byte for byte:

```
{kind}: retained_precision_absolute_verified; verified only to the receipt's absolute bound b = {b} {SI} (binary64 {bits}), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance
```

- **`{b}`** is Rust `{:e}` of the bound: the shortest round-trip digits, a lowercase `e`, and no `+` on the exponent (for example `5.4215527659630466e-24`). Note for TS: JavaScript's `toExponential()` writes `e+N` for non-negative exponents, which Rust does not. Every bound on hand has a negative exponent, but TS's formatter must match Rust's for all values.
- **`{bits}`** is the 16-digit lowercase hex of the bound's bits.
- **`{SI}`** is the unit the reader normalizes the row's unit to (`retained_precision.rs` `normalized`, `row_kind`). It uses the repository's own unit tokens, so the moment unit is written `N*m`, not `N·m`.

| Row unit | SI unit in the message |
|---|---|
| `m`, `mm` | `m` |
| `rad` | `rad` |
| `N`, `kN` | `N` |
| `N*m`, `kN*m` | `N*m` |
| `Pa`, `MPa` | `Pa` |

**What changed:**
- **The old text** was `… b = {b} (binary64 {bits}) in the SI unit of this quantity, below …`.
- **The new text** names the unit after b, and the phrase "in the SI unit of this quantity" is gone.
- **The `not_covered` text is unchanged.**

**A real message** (sparse milestone, a `mm` row):

```
displacement_magnitude: retained_precision_absolute_verified; verified only to the receipt's absolute bound b = 5.4215527659630466e-24 m (binary64 3b1a378ea78c5ce9), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance
```

- **Per mode, the 69 absolute rows** are 2 `mm`, 31 `N`, 20 `N*m`, 15 `MPa` and 1 `Pa`.
- **One real message per unit** is in `_run_records/s2_messages.txt`, both modes.

**A unit the reader never classes absolute.** This is unreachable for a validated statement, because the reader classes a row absolute only in the ten units above. If it ever happened, the row is still withheld, with the `not_covered` disclosure and no bound claimed. It is never published as a value. This is tested directly through the seam.

**The tests:**
- the exact message for one row;
- the SI unit for all ten row units;
- the unknown-unit fallback;
- in the derivative test, every absolute row's message is checked against an independent unit table, and the derivative must cover m, N, N*m and Pa.

## 3. U6a S-1: the four forms (tests)

All four are in `retained_precision_carriers.rs`:
- **R01, a token on a non-first row only:** a base source with the token on its last row, or a middle row, is refused at raw, standing and derive. Metadata still passes, because it is header-only. Another method string is admitted.
- **R02, a member on a base derivative:** the receipt, null, `{}` or a string on a base derivative is refused `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` by `validate_document`. On a raw base source, null, `{}` and a string are refused at raw and at metadata.
- **R06, a legacy 0.1.0 source with a receipt:** a new test, `u6a_legacy_sources_carrying_a_receipt_or_token_are_refused`.
  - The receipt, null or `{}` is refused at raw, metadata, standing and derive.
  - A token on the last row is refused at raw, standing and derive.
  - The plain legacy source still dispatches, stands `needs_recompute` and derives.
- **R05, the two-case requested-ref order:** through the seam with eligibility set, a two-case receipt is `numerically_eligible` with its refs in case order. It is `needs_recompute` when the refs are reversed, have only the first or only the second case, or repeat a case.

RV88's R01, R02, R05 and R06 are all killed (§ Mutants).

## 4. U6b N-3: Q02 and Q06 (tests)

- **Q02:** a base source with the token only on its last row, or only on a middle row, is refused at dispatch and stands `unsupported`. `other_method` is still admitted. The shared cases add the same form on a legacy source and on a preview-physics-1 source.
- **Q06:** the Python two-case seam test, with the same forms as R05.
- **Both are killed:** P04 (Q02) and P05 (Q06).

## 5. U6c N-1 and N-4 (tests; claim corrected)

**The correction.** My U6c record said the RV78-N2 probes were run as "discriminating pairs". That was true only of Y0, Y9 and Y10:
- Y1–Y8 and Y11 ran on corpus case 0 alone;
- Y4 used `{}`, which is refused by shape whatever the branch says;
- Y11 used a non-row object, which is likewise refused by shape.

The summary's "over all 17" overstated what was done. **It is now true:**
- **`test_rv78_n2_receipt_and_branch_probes` is parametrized over all 17 successor statements.** For each statement:
  - the Y0 control validates;
  - each of Y1–Y8 and Y11, plus a missing receipt and a null receipt, is refused;
  - the Y10 projection validates, and Y9 (that projection with a receipt) is refused.
- **Y4 now uses a shape-valid foreign member:** the real physics-source n05 `source_block_recovery`. A new test, `test_rv78_n2_y4_member_is_shape_valid_where_it_belongs`, shows it is shape-valid: the physics-source n05 document is valid with it and refused without it. So its refusal on the successor branch is the branch's own.
- **Y11 now uses a real derivative value row,** load-reference-1's first displacement row, moved onto the document's basis.
  - The control (the scaffold with that row) is valid for all 17.
  - The same row with the W1 token is refused.

**N-4 (W06).** The stress-neutral successor test adds `no_source_annotations`, which is refused. Its `source_block_recovery` probe uses the same real member as Y4.

**Mutants:** Y4 (the successor branch admits `source_block_recovery`), Y11 (value rows admit the token) and W06 are all killed.

## 6. U6a N-3: the seams have no product callers (test)

`u6_doc_hidden_seams_have_no_product_callers` walks every `.rs` file under P, skipping `target`, `node_modules`, `tests`, `execution`, `fixtures`, `.git`, `.venv` and `dist`. That is more than 50 files. It reads only the text before the first `#[cfg(test)]`, so inline test modules are not counted as product code.

| Seam | Defining file | Occurrences of `seam(` there | Anywhere else |
|---|---|---|---|
| `class_binding_refusal` | semantic_contract.rs | exactly 2 | 0 |
| `retained_standing_from` | semantic_contract.rs | exactly 3 | 0 |
| `classification_summary_from` | semantic_contract.rs | exactly 2 | 0 |
| `class_disclosure` | derivative.rs | exactly 3 | 0 |

The counts in the defining files are the definition plus today's internal calls. So a new internal caller also fails the test and needs review.

**Python analogue:** `test_private_seams_have_no_product_callers`. Under P/core and P/tools, `_retained_standing_from`, `_classification_summary_from` and `_class_binding_refusal` appear only in `compatibility.py`, with pinned counts.

**Mutants killed:**
- N3a: a call added in derivative.rs;
- N3b: the seam named in src-tauri `lib.rs`;
- P09: a seam named in records.py.

## 7. The shared declared differences (`retained_precision_carrier_cases.json`, now format v2)

The file was regenerated by `_run_records/make_carrier_cases_v2.py`. The 14 v1 cases are unchanged in content and order.

**What v2 adds:**
- **`"shape"` on each fixture:** `"milestone"` (`{id, invocation, source}`) or `"raw"` (the file is the source, and its cases have no invocation).
- **Two raw fixtures:**
  - `legacy_preview_0_1`: `fixtures/product_preview/invented_mechanics_result.json`, `fb6724aa…`;
  - `preview_physics_1_invented_sparse`: `fixtures/results/preview_physics_invented_sparse.json`, `a8d18b7f…`.
- **Six F-5 guard cases** (items 1 and 3). On each raw fixture: a token on the last row only, a `{}` member and a null member. Each expects `unsupported` and `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. **These are parity cases, not differences.** That makes 20 cases.
- **`declared_differences`: exactly 4 entries.** Each has an id, a kind, the ruling it cites, a description, its subject, its fixtures (both milestones), its edits and one expectation per language:

| Id | Subject | Rust | Python | TypeScript |
|---|---|---|---|---|
| `I67-F1:unregistered_invalid_statement` (edited row, no invocation) | standing | `unsupported` | `unsupported` | `needs_recompute`, finding `RETAINED_PRECISION_VALIDATION_REQUIRED` |
| `I67-F2:display_only_binding_precheck` (valid statement, no invocation) | binding, every row | `by_validated_class` | `by_validated_class` | `every_row:RULE_QUANTITY_NOT_COVERED`, notice `N_RP_UNVALIDATED` |
| `F-U6b-2:python_refuses_transport` (unedited successor, header-only dispatch) | transport | `ok` | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | `ok` |
| `F5:refused_statement_binding` (edited row; kind `semantics`, the same in all three) | binding, every row | `every_row:RULE_QUANTITY_NOT_COVERED` | same | same, notice `N_RP_UNVALIDATED` |

**How Rust and Python consume it** (`u6_declared_differences_rust`; `test_declared_differences_python`):
- Each asserts that the ids are exactly these 4, and that every entry has a ruling and all three languages.
- Each computes its own expectation in both modes:
  - **standing:** with the given invocation and refs;
  - **transport:** Rust `for_source_metadata`, Python `_source_contract(check_receipt=False)`;
  - **binding:** every row, either against the reader's validated classes (which must include both refused and binding rows) or every row `RULE_QUANTITY_NOT_COVERED`.
- **A fifth difference, or one missing, fails both tests.**
- The shared-case tests read v2, with raw fixtures, and expect 20 cases.

**For I67 (TS):**
- the format string is now `I66-U6-CARRIER-CASES-v2`;
- fixtures carry `shape`;
- there are 6 new cases on the two raw fixtures;
- the `typescript` expectations above come from I67's RETURN (F1, F2) and RV88's U6b §3 table. I67 should confirm them, or report a defect.

U6d is not on this branch, so TS's consumer must be updated in I67's round before the branches meet.

## Controls

**1. The 24-file sweep, plus `test_retained_precision_schema.py` and `test_retained_precision_carriers.py`** (`sweep.compare.txt`). Base is the archive of `924c6284cb`; the candidate is the worktree.
- **Base:** 1,823 passed, 30 skipped, 0 failed.
- **Candidate:** 1,843 passed, 30 skipped, 0 failed.
- **No test that passes at base fails, and nothing newly fails.**
- **The single "removed" id is a rename.** It is `test_rv78_n2_receipt_and_branch_probes`, now parametrized over the 17 statements. The arithmetic:
  - 1,823 − 1 + 17 (the parametrized probes)
  - \+ 1 (the Y4 shape test)
  - \+ 3 new carrier tests (declared differences, legacy token, Python seam guard)
  - = 1,843.

**2. result_export** (`cargo test`, every target; `*_result_export*.outcomes`):
- base 164 passed;
- candidate 167 passed.
- All 164 existing outcomes are identical. The 3 additions are `u6_declared_differences_rust`, `u6_doc_hidden_seams_have_no_product_callers` and `u6a_legacy_sources_carrying_a_receipt_or_token_are_refused`.
- The final run used the final bytes, after a doc-comment rewrap in derivative.rs; its outcomes are identical.

**3. The other Python consumers** of these carriers (`extra_test_files.txt`; `extra.compare.txt`): 428 passed and 2 skipped at base and in the candidate, with 0 differences.

**4. The existing-identity document sweep** (`doc_sweep.py`; `docsweep_{base,cand}.tsv`; `docsweep.diff`, empty):
- **Scope:** 62 committed result documents, with 11 Python carrier outcomes each: dispatch, transport, standing ×2, reason, binding, summary, build, validate, the 0.2 constructor and the 0.1.0 wrapper.
- **Result: 0 differences.**

**5. Nothing weakened.**
- No check was removed. The 0.2.0 token guard was moved into the helper with the same behaviour, as the document sweep and suites show.
- The probe rework replaces non-discriminating probes with discriminating ones over more statements.
- No reader file and no flag was touched.

**6. Mutants: 29, all killed by failing tests, with no compile or syntax error** (`mutants.py`, `mutants_run1.json`). The mutant lane passed its baselines first: Rust 13/13 and Python 127/127.

| Group | Mutants |
|---|---|
| Python S-1 | P01 (the 0.1.0 branch skips the guard), P02 (the 0.2 constructor admits token rows), P03 (the wrapper admits token rows), P06 (the wrapper reads the first row only), P07 (the legacy guard runs on transport), P08 (token by key presence) |
| RV88 U6b (rerun) | P04 = Q02, P05 = Q06, P11 = Q04, P12 = Q01, P13 = Q05 |
| Transport | P10 (Python transport admitted) |
| Seams | P09, N3a, N3b |
| RV88 U6a | R01, R02, R05, R06 |
| S-2 | S2a (mm printed as `mm`), S2b (MPa printed as `MPa`), S2c (unit dropped), S2d (`kN*m` mapped to `N`), S2e (an unknown unit claims absolute), S2f (validate assumes `Pa`), S2g (`rad` unmapped) |
| Schemas | Y4, Y11, W06 |

The mutants ran before the doc-comment rewrap in derivative.rs. Comments are the only difference between the lane and the final file.

## Notes

- **The SI symbol `N*m`** follows the row unit tokens. If ROOT prefers `N·m`, as RV88 suggested, it is a one-line change in Rust and TS together.
- **The `{:e}` formatting caution for TS** is in item 2.
- **The 0.1.0 wrapper's token refusal** goes beyond D-U6-9's receipt rule. ROOT's message listed the wrapper among the paths that accepted the form, so it now refuses too. The code is the wrapper's existing `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.

## Records

- `_run_records/` holds:
  - the runners, the case generator, the mutant harness, the comparison and document-sweep scripts;
  - the sweep and suite outcomes with their comparisons;
  - the document sweep and its empty diff;
  - the result_export outcomes;
  - the mutant results;
  - the S-2 messages;
  - the candidate diff against `924c6284cb` and the changed-file hashes.

  All use placeholder paths.
- SHA256SUMS covers this folder.
