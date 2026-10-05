# RV88 confirmation of I66's U6 repair round (`da274dd961`)

RV88, the standing independent U6 reviewer, is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). It confirms its own U6a, U6b and U6c findings against I66's repair round (`R/I66/u6_repairs_01/RETURN.md`; RR "I66's U6 repair round committed; U6d merged into the carriers branch; TS follows the shared format"), with its earlier context. ROOT is the return path, and RV88 did not delegate.

**The comparison:** candidate `da274dd961`, against its parent `924c6284cb` (the merged U6a/U6b/U6c/U6e head). U6d's merge (`52052ece61`) and its 7 expected TS shared-parity failures are out of scope.

## Verdict: NOT CONFIRMED on one item (U6a N-3); every other item is CONFIRMED. Nothing is blocking.

| Item | Result |
|---|---|
| U6b S-1: Python refuses a legacy 0.1.0 source with a W1 token row on every path | **FIXED** |
| U6a S-2: the disclosure names the SI unit | **FIXED** (unit map and `{:e}` checked independently) |
| U6a S-1: R01, R02, R05 and R06 killed | **FIXED** (4/4 killed) |
| U6b N-3: Q02 and Q06 killed | **FIXED** (2/2 killed) |
| U6c N-1 and N-4: Y-probes over all 17 statements, Y4 and Y11 discriminate, W06 pinned | **FIXED** |
| U6a N-3: the `#[doc(hidden)]` guard test | **NOT FIXED as intended.** The test exists, but it is blind to everything after a file's first `#[cfg(test)]`. That is 38% of scanned Rust lines, including almost all of src-tauri `lib.rs` and runner/headless `lib.rs`. RV88's mutant G1 (a seam named at src-tauri lib.rs:3209) **survives**. |
| The shared `declared_differences` section | **CONFIRMED.** It has exactly 4 ruled entries, and Rust and Python consume it. All Rust and Python expectations reproduce independently. The TS expectations match RV88's U6d observations. One description is inaccurate for TS (N-1). |

**Existing behaviour is unchanged** (§1):
- result_export goes from 164 to 167, with the 164 existing outcomes identical;
- RV88's Rust sweep of 97 existing envelopes and 6 injected forms matches base, except the known F1 nondeterministic fixture;
- RV88's Python carrier sweep: 69 existing envelopes are identical to base; the token forms are now refused on all 7 legacy envelopes.

## Basis and host

- **The copies:** `git archive` of `da274dd961` and `924c6284cb` (P without the records tree) into `WT/rv88/r_{cand,base}`, with a mutant copy in `WT/rv88/r_mut`. Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **The candidate's files match I66's record.** The 7 changed files equal I66's `changed_files_sha256.txt`.
- **Cargo:** `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time. Targets are under `WT/targets/rv88/`. The checked-JSON and units CLIs were built `--release` from the candidate archive. Python ran from REPO_ROOT's `.venv`.
- **When:** 2026-10-04, from 12:33Z to 13:00Z, with the memory guard (PID 5387) running throughout.
- **Not run:** nothing native, at scale or DEC-025, and no install.

## 1. Existing behaviour

| Control | Base `924c6284cb` | Candidate `da274dd961` |
|---|---|---|
| result_export (`cargo test`, every target) | 164 passed | 167 passed: the 164 identical, plus `u6_declared_differences_rust`, `u6_doc_hidden_seams_have_no_product_callers` and `u6a_legacy_sources_carrying_a_receipt_or_token_are_refused` |
| RV88's Rust sweep (`zz_rv88_sweep.rs`, run 2: identified plus legacy envelopes) | 114 envelopes | 97 existing identical, apart from F1's `rejected_stress_range/sparse_interactive` codes, which also vary between base runs. `!receipt`, `!null`, `!token0` and `!tokenlast` are refused as before; `!othertoken` and `!r2notice` are identical. |
| RV88's Python carrier sweep (`rv88_py_carrier_sweep.py`) | 86 envelopes | 69 existing identical. `!token0` and `!tokenlast` are refused on 67/69: the 2 others are preview-physics-1 envelopes with no rows. At base, the 7 legacy envelopes were admitted. `!receipt` and `!null` are refused 69/69; `!othertoken` and `!r2notice` are identical. |
| `test_retained_precision_schema.py` plus `test_retained_precision_carriers.py` | — | 78 passed |

**A new check on every identity:** `validate_document` now requires each source row to carry a string `unit` (derivative.rs:562, `SOURCE_UNIT_MISSING`). `derive_document` already required that (:218–221), so no derivable document is affected, and the sweep shows no change.

## 2. U6b S-1: FIXED

- **The fix** is compatibility.py's new `_has_retained_rows`. It now runs in the 0.1.0 branch (raw path only, :382–383), in the 0.2.0 raw guard, and in the explicit v0.2 constructor (:86). records.py's 0.1.0 wrapper (:91–96) refuses token rows too.
- **RV88's own probe** (`rv88_rep_py.py`; the token on the first, middle and last row of the legacy fixture):

| Path | W1 token | Another method string |
|---|---|---|
| raw `_source_contract` | `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | admitted, as base |
| transport (header only) | admitted (reads no rows, as Rust's `for_source_metadata`) | admitted |
| `numerical_use_standing` | `unsupported` | `needs_recompute` |
| `build_analysis_run` | refused, `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | 0.2.0 record |
| `build_analysis_run_v0_2` | refused, `ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN` | 0.2.0 record |
| 0.1.0 wrapper | refused, `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` | 0.1.0 record |

- **Rust and Python now agree on this form.** Rust refuses it at raw, standing and derive (RV88's `zz_rv88_refusals.rs` reran clean on the candidate). The 6 new shared cases (legacy and preview-physics-1: a token on the last row, a `{}` member, a null member) pass in RV88's own Rust and Python mappings (§7).

## 3. U6a S-2: FIXED

**The new text:** `… absolute bound b = {b} {SI} (binary64 {bits}), below the relative accuracy floor; …`. RV88 checked it with its own oracle (`rv88_rep_check.py`), never with I66's tables:
- **The SI table, built from D2 §4.9.10's admitted units:** m and mm → `m`; rad → `rad`; N and kN → `N`; N\*m and kN\*m → `N*m`; Pa and MPa → `Pa`. Any other unit has no class bound.
- **A formatter for Rust `{:e}`:** the shortest round-trip digits as `d[.ddd]e<exp>`, with no `+` and no exponent padding.
- **The bound bits** come from the receipt's own `absolute_verified` list.

| Check | Result |
|---|---|
| All 138 class disclosures in the two milestone derivatives (69 per mode) equal RV88's rebuilt message byte for byte | 138/138. Per mode: mm→m 2, N 31, N\*m 20, MPa→Pa 15, Pa 1. |
| `class_disclosure` over 15 units × 9 bounds | 135/135. The bounds are 0, the minimum subnormal, 2^-1022, 1.0 (`1e0`), 1.5 (`1.5e0`), about 1e20 (`e20`, no `+`), `f64::MAX` (`1.7976931348623157e308`), about 1e-6, and a milestone bound. The 10 admitted units give the SI form; `unitless`, `mode_code`, `""`, `deg`, `kPa` and `N·m` give the `not_covered` disclosure with no bound. |
| The `not_covered` text is unchanged | 15/15 |
| Swapping a message's unit (m→mm, Pa→MPa, N\*m→N·m, N→kN) | refused by `validate_document`, 8/8 |

- **The text is truthful.** It now names the unit b is published in, and still claims nothing beyond the receipt's bound.
- **The unknown-unit fallback,** a withheld row with the `not_covered` code and no bound, claims less than the class. It is unreachable for a validated statement. This is acceptable.
- **`N*m` was accepted by ROOT.** TS must print the same `{:e}` form (I66's note on `toExponential`'s `e+N`). That is I67's follow-up.

## 4. U6a S-1, U6b N-3 and U6c N-4: the mutants (`rv88_rep_mutants.py` → `mutants.json`)

Each mutant ran in `WT/rv88/r_mut` against I66's own tests only: the 3 result_export targets, the Python carrier tests, or the schema test.

| Mutant | Result | Killed by |
|---|---|---|
| R01: the row guard reads only the first row | **killed** | `u6a_downgrades_to_a_base_identity_are_refused`, `u6a_legacy_sources_…` |
| R02: `validate_document` ignores a null member | **killed** | `u6a_downgrades_to_a_base_identity_are_refused` |
| R05: requested refs are order-insensitive | **killed** | `u6a_standing_rule_conjuncts_with_eligibility_set` |
| R06: the member guard exempts legacy 0.1.0 | **killed** | `u6a_legacy_sources_…`, `u6a_shared_carrier_cases_rust` |
| Q02: the token helper reads only the first row (retargeted to `_has_retained_rows`) | **killed** | `test_shared_carrier_cases_python`, `test_downgrades_are_refused` |
| Q06: requested refs are order-insensitive (Python) | **killed** | `test_standing_rule_conjuncts_with_eligibility_set` |
| W06: the stress-neutral successor's `source_annotations` is optional | **killed** | `test_stress_neutral_successor_branch_is_transport_shape_only` ×2 |
| New, V1: the moment unit is spelled `N·m` | killed | the derivative and class-map tests |
| New, V2: `kN` is unmapped | killed | `u6a_each_class_maps_…` |
| New, W09: the stress-neutral successor admits `source_block_recovery` | killed | the stress-neutral successor test ×2 |
| New, W10: the results successor admits `source_block_recovery` (the Y4 target) | killed | `test_rv78_n2_receipt_and_branch_probes[…]` |
| New, G2: a seam named at the top of runner `result_envelope_binding.rs` | killed | `u6_doc_hidden_seams_have_no_product_callers` |
| New, **G1: a seam named inside src-tauri `solver_result_row_value` (lib.rs:3209)** | **SURVIVED** | — (§6) |

## 5. U6c N-1 and N-4: FIXED

- **Y1–Y11 run over all 17 statements.** `test_rv78_n2_receipt_and_branch_probes` is parametrized over all 17 successor statements; RV88 counted 17 passing ids. Each id runs the Y0 control, Y1–Y8 and Y11, a missing receipt, a null receipt, Y10 and Y9.
- **Y4 discriminates.** It uses the real physics-source n05 `source_block_recovery`. `test_rv78_n2_y4_member_is_shape_valid_where_it_belongs` shows that member is valid on its own branch and required there, and W10 (the successor branch's exclusion removed) is killed.
- **Y11 discriminates.** It uses a real derivative value row: the control with the row is valid, and only the token is refused.
- **W06 is pinned** (killed). The schemas are unchanged in this round, so RV88's U6c probes still stand.
- **I66 corrected its U6c record's claim,** as N-1 asked.

## 6. U6a N-3: NOT FIXED as intended

`u6_doc_hidden_seams_have_no_product_callers` (retained_precision_carriers.rs:754–789) reads each product `.rs` file **only up to its first `#[cfg(test)]`** (`text.split("#[cfg(test)]").next()`). Many product files have a `#[cfg(test)]` *declaration* near the top, such as `#[cfg(test)] mod …;` or `#[cfg(test)] use …;`, so the rest of the file is never scanned:
- `apps/desktop/src-tauri/src/lib.rs`: scanned to line 12 of 8,604;
- `core/runner/headless/src/lib.rs`: to line 11 of 1,630;
- `core/product_physics/src/lib.rs`: to line 25 of 24,066.

Across the 208 product files the test reads, **96,080 of 250,392 lines (38%) are never scanned**. Those lines include every `result_export` call in src-tauri `lib.rs` (10) and runner `lib.rs` (12), which are exactly where a product caller of these seams would appear.
- **G1:** a seam named at src-tauri lib.rs:3209 (inside `solver_result_row_value`, beside the real `rule_binding_refusal` call) **survives**.
- **I66's N3b** was killed only because it was placed above line 12.
- **A second limitation:** a text guard also misses an aliased import (`use …::retained_standing_from as f;`). Remedy (b) below closes that gap too.

**Remedy:** two changes, then a check.
- **(a)** Exclude only `#[cfg(test)]`-gated *items*:
  - drop the `mod …;` or `use …;` line that follows it;
  - for `mod … {`, drop through the matching brace.
  
  Or scan whole files and exclude only inline `mod tests {…}` blocks, by brace matching.
- **(b)** Count the bare seam name, not `name(`. That also catches an `as` alias.
- **Then** re-run a mutant like G1. The Python guard (`test_private_seams_have_no_product_callers`) counts whole files and has no such blind spot.

This was a NOTE, and it does not gate anything. The seams are `#[doc(hidden)]`, and no product caller exists today: RV88 grepped the whole tree and found none.

## 7. The shared `declared_differences` section: CONFIRMED, with one description NOTE

**The structure is right.** The case file is format v2, with 20 cases (the 14 v1 cases unchanged, plus 6 F-5 guard cases on two raw fixtures) and exactly 4 `declared_differences`. Each difference has an id, a kind, a ruling, a description, a subject, fixtures, edits, and one expectation per language. Rust (`u6_declared_differences_rust`) and Python (`test_declared_differences_python`) assert the 4 ids and compute their own expectations.

**RV88's independent computation.** RV88 recomputed each Rust and Python expectation in both modes (`zz_rv88_rep.rs`, `rv88_rep_py.py`). For binding, its oracle is the receipt's own `absolute_verified` list, not the reader's classes.

| Difference | Rust | Python | TypeScript (from RV88's U6d probes) |
|---|---|---|---|
| I67-F1, an invalid statement with no invocation: standing | `unsupported` ✓ | `unsupported` ✓ | `needs_recompute` + `VALIDATION_REQUIRED` ✓ (an unregistered delivery) |
| I67-F2, a valid statement with no invocation: binding | `by_validated_class` ✓ (69 absolute rows refused, the rest bind) | ✓ | every row `NOT_COVERED`, notice `N_RP_UNVALIDATED` ✓ (precheck 98 or 99 of 98 or 99) |
| F-U6b-2, transport | `ok` ✓ | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` ✓ | `ok` ✓, for the unedited statement |
| F5, a refused statement: binding | every row `NOT_COVERED` ✓ | ✓ | ✓, notice `N_RP_UNVALIDATED` |

**The shared cases.** All 20 equal their expectations in RV88's own Rust and Python mappings (`rep_facts.tsv`, `rep_py.tsv`).

**Is anything missing?** RV88 knows of no other cross-language input-output difference among the carriers:
- TS's registration voiding on a key reorder (U6d N-2) is TS-internal and fail-closed;
- the Rust `0.0`-against-`0` derivative round trip (U6a N-5) belongs to a carrier that Python and TS do not have.

**N-1 (description):** F-U6b-2 says "Rust and TS run the transport checks". In TS, the compared operation is the **header route** (`sourceContract`), which checks shape only. No TS carrier calls `validateRetainedPrecisionTransport` (at `52052ece61` only the reader defines it). So for a *tampered* transported statement, a `receipt_sha256` edit for example, Rust refuses (G1), Python refuses, and the TS header route still reads `retained_preview_physics`. The values in the file are right for the unedited statement, but the TS part of the description overstates. TS confers no standing from the header route alone, so no reliance follows.
- **Remedy (I67's follow-up, or U6f):** state "TS: header route, shape only (no G0–G2)", or make TS's transport consumer call `validateRetainedPrecisionTransport`.
- **And:** add an entry, or case, for a tampered transported statement if TS keeps the shape-only route.

## Counts

- **Items:** 6 confirmed fixed, 1 not fixed (U6a N-3, a NOTE).
- **Mutants:** 13 run; 12 killed by I66's tests, 1 survived (G1).
- **New findings:** **N-1** (the TS transport description in `declared_differences`). U6a N-3 remains open.

## For ROOT to rule

1. **U6a N-3:** a 2-line fix to the guard's scope (§6) in I66's next touch of the test file, or at U6f. It is non-blocking.
2. **N-1:** route the TS transport description, or the TS transport consumer, to I67's follow-up.

## Records

Everything is in `_run_records/`, with placeholder paths only. **The scripts:**
- `zz_rv88_rep.rs`, `zz_rv88_sweep.rs` and `zz_rv88_refusals.rs`;
- `rv88_rep_check.py`, `rv88_rep_py.py`, `rv88_rep_mutants.py`, `rv88_sweep_compare.py`, `rv88_py_carrier_sweep.py` and `rv88_py_sweep_compare.py`.

**The outputs:**
- `rep_facts.tsv`, `rep_py.tsv` and `refusals.tsv`;
- `sweep_{base,cand}_2.tsv.gz` and `sweep_summary.txt`;
- `pysweep_{base,cand}.tsv.gz` and `pysweep_summary.txt`;
- `result_export_{base,cand}.names`;
- `mutants.json`.

SHA256SUMS covers this folder.
