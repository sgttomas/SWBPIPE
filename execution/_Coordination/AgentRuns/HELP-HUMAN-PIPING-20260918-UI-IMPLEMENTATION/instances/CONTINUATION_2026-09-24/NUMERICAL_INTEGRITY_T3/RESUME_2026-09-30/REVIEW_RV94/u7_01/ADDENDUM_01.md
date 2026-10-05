# RV94 ADDENDUM_01: confirmation of the U7 repair round

**Reviewer:** RV94, the same TASK (Type 2), confirming its own findings at ROOT's request. ROOT is the return path. No descendants. I wrote none of the repairs.

**Basis:** ROOT's message; RR "RV94 on U7: PASS; the summary aligned across languages; the stale-comment repair; a public-activation checklist" (NUM `979130f4e1`), "The RV94 repair round, part 1…" (NUM `e5ecf0f136`) and "…part 2…"; my `REVIEW.md` (sha256 `f621439e…`).

**Candidate:** the U7 head `e543c3d8f3` against my reviewed `ffe65ef203`, 14 files, +402 / −87:
- `fc575c7e56`, the stale "no permit" comments (I61, I65);
- `8c84e7ae14`, the Python and Rust summary alignment (I66, S-1 and N-4);
- `e543c3d8f3`, D-U7-4's summary forms, the N-2 docstring, the N-3 clause and probe (snapshot "07k"), and the test edits (I67).

**Host:**
- **Copies:** fresh `git archive` copies (P without `execution/`) of `ffe65ef203` and `e543c3d8f3` in `WT/rv94/`, with a pristine copy and three mutant lanes. The `node_modules` symlinks and WASM copies are as in REVIEW.md.
- **Builds and logs:** targets in `WT/targets/rv94/` (registered and Stale in separate dirs); logs in `WT/scratch/rv94_u7_01/` (`r2/`).
- **Toolchains and the memory guard:** as in REVIEW.md, one cargo job at a time; memguard 5387 checked before every job.
- **Never:** no Git writes, nothing native or DEC-025, no job output in the system temp directory (the harness's own task stdout files excepted, as disclosed in REVIEW.md).
- **Cleanup:** copies and targets are deleted afterwards.

**Method:**
- My REVIEW.md generator, oracle and dumpers, unchanged except as follows:
  - Python's and Rust's dumpers now pass the caller's requested refs to the new `classification_summary` signature, and also record the no-refs summary (`rv94_dump_py2.py`, `zz_rv94_dump2.rs`);
  - a second comparer (`rv94_compare2.py`) applies the aligned summary rule: Current iff the carrier token with the **caller's** requested refs is eligible.
- **The inputs.** The regenerated input file is byte-identical to REVIEW.md's 420 inputs plus the new corpus probe, 421 in all. Its sha256 without the probe line equals the earlier `ad94d29a…`. All three languages were dumped on both heads.

## Verdict: **PASS** (the repair round is confirmed)

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE (new) | 2 |

| REVIEW.md finding | Status at `e543c3d8f3` |
|---|---|
| S-1 | **Closed.** The summary follows each language's own standing for the caller's refs. The only remaining summary difference, the live capture, is declared in D-U7-4 and pinned in all three languages |
| N-1 | Routed to the public-activation checklist, item 1 (RR). Nothing to confirm in code |
| N-2 | **Closed.** The TS docstring is truthful (text only) |
| N-3 | **Closed.** The scope clause is truthful and pinned in three languages, with a shared probe |
| N-4 | **Resolved.** Python and Rust no longer substitute the invocation's cases for the caller's refs |

## 1. S-1 and N-4: the summary is aligned

**Oracle rerun** (`evidence/r2/cmp2_py.txt`, `cmp2_rs.txt`; dumps in `evidence/r2/`):

| | Python | Rust |
|---|---|---|
| Mismatches with the oracle on `e543c3d8f3` (421 inputs) | 0 | 0 |
| Reader output or token changed vs `ffe65ef203` | 0 | 0 |
| Summaries changed | 14 | 14 (the same 14) |
| Summary Current exactly when the token is eligible | 421 / 421 | 421 / 421 |
| The no-refs summary is never Current | yes | yes |

- **The 14 summary changes are exactly the oracle's set:** the inputs whose requested refs differ from the receipt's cases while the reader is eligible. They are:
  - the 4 carrier `no_requested` / `other_requested` cases;
  - my 8 milestone variants (requested refs empty, other, duplicated, or of the wrong type);
  - 07j with the not_required case omitted or reordered.

  Each is now not-Current: [97] on the milestones and [73, 0] on 07j, where before it was [69] and [1, 0].
- Python's and Rust's earlier dumps of the 420 inputs on `ffe65ef203` are reproduced exactly by this round's `ffe65ef203` lane.
- **Across languages** (`e543c3d8f3`):
  - Python and Rust agree on token and summary on all 421 inputs.
  - TS's standing seam, with a live capture, agrees with them on token **and** summary on all 419 inputs TS can express. The 2 wrong-ref-type variants are inexpressible: TS's requested refs are always load-case refs.
  - In every language and lane (Python, Rust, TS with and without a live capture), the summary is Current exactly when that language's standing is eligible.
- **The only remaining difference is D-U7-4's.** On both D-U7-4 forms, Python and Rust give `numerically_eligible` with the Current summary [69]. TS without the live capture gives `needs_recompute` / `NATIVE_CAPTURE_REQUIRED` with [97]; this is my seam without a live capture, and the IPC no-capture and stale-model scenarios give the same.
- **TS unchanged.** The TS dumps (reader, both seams, all IPC scenarios) are byte-identical between `ffe65ef203` and `e543c3d8f3`, and identical to REVIEW.md's on the 420 shared inputs. TS's only production change is the N-2 docstring.
- **N-4: resolved.**
  - Python `classification_summary(source, invocation, requested_basis_refs=None)` (`compatibility.py:348–366`) and Rust `classification_summary(source, invocation, requested_basis_refs)` (`semantic_contract.rs:664–687`) count Current only through `retained_standing_from` with the caller's refs. The invocation now only binds the validation.
  - There is still no non-test caller of either summary in Python, Rust, the runner or the Tauri crate.
  - The Rust edit keeps `semantic_contract.rs` at 855 lines.

## 2. The declaration (S-1), N-2 and N-3

- **D-U7-4 is truthful and complete.** Its description now names the summary split.
  - The two new `summary` forms read exactly the inputs of their standing twins (TS's structure test asserts this).
  - Their expectations (Python/Rust `by_validated_class_current`, TS `by_validated_class`) match what each language computes on my inputs: 69 against 97.
  - The note defines `by_validated_class_current`, and keeps `by_validated_class` as the not-Current count with or without an invocation.
  - The format stays v4; unknown values fail loudly in all three languages (mutant R25 below).
- **The fence extension widens rather than narrows.** I66's side tests asserted `form.expected.<lang>.standing == "numerically_eligible"` on every D-U7-4 form. That would fail on (Rust) or raise for (Python) the new summary forms. I67's one-line replacement in each language admits the summary forms and checks each form's whole expectation exactly: `{standing: numerically_eligible}` for standing forms, `{summary: by_validated_class_current}` otherwise.
  - The loop body still asserts, for every form, eligible standing, the Current summary of 69, and the not-Current summary with other refs.
  - Each per-form check is therefore stricter than before (whole-dict equality), and no assertion was removed.
  - The other added lines are the scope fence, the `by_validated_class_current` mapping with a coverage check (`summary:current` / `("summary", "by_validated_class_current")`), and the probe's count pins.
- **N-2: truthful.** `retainedPrecision.ts:1307–1308` now uses the Python/Rust wording, adds "Standing comes from the carriers (D2 4.9.4)" and the D-U7-6 sentence, and stays line-neutral (1,339 lines). It is text only, so no test pins it, as with the Python and Rust docstrings.
- **N-3: truthful and pinned.**
  - The clause names the class, all three fields and each language's code.
  - On my inputs, all three fields give Python/Rust `SOURCE_NUMERICAL_CASE_INVALID` and TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, on both heads.
  - The probe `g7_not_required_quality_enum_invalid` (mutation 278, the `accuracy_evidence` edit) reproduces this in each reader: Python reads `expected`, Rust `expected_by_reader.rust`, TS `expected_by_reader.typescript`.
  - All three languages assert the clause. TS also tests all three fields on 07j.
  - The corpus is "07k" (`482449bf…`), 07j plus the probe. The first 277 mutations, the bases, the must-pass entries and d37 are unchanged.

## 3. The comment repairs (`fc575c7e56`)

- **Comment-only:** every changed line in PP `lib.rs` (`:2185`, `:2286`, `:2919`), `retained_facade_tests.rs:1–2` and `retained_memory.rs` (`:6–8`, `:2742`, `:2847`) is a `//`, `///` or `//!` line.
- **Line-neutral:** `lib.rs` 24,333, `retained_facade_tests.rs` 827 and `retained_memory.rs` 3,076 lines, all equal to `ffe65ef203`; every hunk replaces n lines with n.
- **True at this head:**
  - `REGISTERED_PROFILES` holds exactly one profile (`retained_memory.rs:948–961`), the dev/test identity, with `threshold_bytes: 4_026_531_840` (M, D-7).
  - `identity_match` returns `Missing` only when nothing is registered (`:970–971`), so "`Missing` … (none since G6)" holds.
  - Every other identity is `Stale`, and my Stale sweep publishes no successor.
  - A permit exists only through `admission()` for that profile. The headless route is refused at admission and takes `into_parts()`, so "only D1 Direct calls in the registered build get a permit" holds.
  - `into_publication()` returns `Ordinary` without a permit.

## 4. Nothing else changed

- **Published bytes:** my own builds of `e543c3d8f3` give the registered sweep `9a74ff16…` (2 successor rows) and the Stale sweep `0e2db8b8…`, equal to REVIEW.md's. The registered Direct entry writes `ac6986b0…` / `6cd1d249…`.
- **Gates, codes, classes:** the reader output (gate, code, `invocation_bound`, eligibility, publication hash, class counts and class digest) and the token are identical between the two heads in Python, Rust and TS on all 420 shared inputs. The probe gives the codes above.
- **Suites:** each differs from `ffe65ef203` only by additions:

  | Suite | At `ffe65ef203` | At `e543c3d8f3` | Change |
  |---|---|---|---|
  | Python retained | 460 | 463 | +3 (the D-U7-4 side test, the 07j omitted/reordered test, the probe) |
  | `result_export` | 169 | 171 | +2 |
  | Vitest | 3,542 | 3,552 | +10, 0 changed outcomes, 0 removed |
  | tsc | clean | clean | — |
  | PP registered and Stale | 705 ok / 1 (t13) / 10 ignored | identical, test by test | — |
  | runner/headless | 85 / 2 | identical | — |

## 5. Mutants for each repair

The mutants (`evidence/r2/mutants2_{py,rs,ts}.json`; `rv94_mutants2.py`) are one edit each, run in each language's lane, which was checked pristine before and after. A kill is a failing test.

| Id | Repair | Mutation | Python | Rust | TS |
|---|---|---|---|---|---|
| R01 | N-4 | Python summary takes the invocation's cases again | **killed** | — | — |
| R02 | S-1 | Python summary Current from the reader's flag alone | **killed** | — | — |
| R03 | S-1 | Python: no refs means the receipt's order (Current by default) | **killed** | — | — |
| R11 | S-1 | Rust summary Current from the reader's flag alone | — | **killed** (also by behaviour) | — |
| R12 | N-4 | Rust summary takes the receipt's order instead of the caller's refs | — | **killed** | — |
| R21 | S-1 decl. | TS side set to `by_validated_class_current` | survives | survives | **killed** |
| R22 | S-1 decl. | Python side set to `by_validated_class` | **killed** | survives | survives |
| R23 | S-1 decl. | Rust side set to `by_validated_class` | survives | **killed** | survives |
| R24 | S-1 decl. | The two summary forms removed | **killed** | **killed** | **killed** |
| R25 | S-1 decl. | An unknown summary value | **killed** | **killed** | **killed** |
| R26 | N-3 | The clause removed | **killed** | **killed** | **killed** |
| R27 | N-3 | The clause misstates TS's code | survives | survives | **killed** |
| R31 | N-3 | The probe removed | **killed** | **killed** | **killed** |
| R32 | N-3 | The probe's TS code changed | survives | survives | **killed** |
| R33 | N-3 | The probe's Rust code changed | survives | **killed** | survives |
| R34 | N-3 | The probe's `expected_by_reader.python` changed | survives | survives | survives |
| R35 | N-3 | The probe's `expected` changed | **killed** | survives | survives |

- **Each mutant is killed by the language whose side it changes,** except R34 (N-6).
- The TS docstring (N-2) and the PP comments are text and carry no mutant.

## New notes

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N-5 | NOTE | `P/fixtures/results/retained_precision_cases.json` (mutations `g7_not_required_quality_enum_invalid` and, before it, `g7_maximum_off_enclosure`); `P/tests/test_retained_precision_contract.py:240` | **Python never reads `expected_by_reader.python`.** Its first-failure test compares the reader with `expected`, and the snapshot test checks only which entries carry `expected_by_reader`. A wrong `expected_by_reader.python` therefore goes unnoticed (R34 survives in all three languages). Today `expected` equals it in both per-reader entries, so nothing is wrong. The pattern dates from the 06b `g7_maximum_off_enclosure` precedent. | Optional: Python asserts `expected_by_reader.python == expected` when present, or reads `expected_by_reader.python` as Rust and TS read theirs. |
| N-6 | NOTE | `fc575c7e56` (PP `lib.rs`, `retained_memory.rs`); `8c84e7ae14` (`result_export/src/semantic_contract.rs`) | **RV89's Pass B fence condition (d)** (RR "RV89 confirms U4 Pass B on the U7 head…": "no production `.rs` change lands in the D1 crates; otherwise rerun Pass B on that head") **is literally met by this round.** PP's comments and `semantic_contract.rs`'s summary edit are production `.rs` changes in crates on D1. They are comment-only or, for Rust, outside D1's reach: PP never calls `classification_summary`. All are line-neutral, and RR records that no Pass B rule key falls on an edited line. My sweeps and PP suites show no published or outcome change. I did not run Pass B (RV89's). | ROOT records whether U9's mechanical Pass B rerun covers this round, or asks RV89 to confirm the fence, before U9's cut. |

## For ROOT to rule

1. **N-6:** confirm that the Pass B fence is handled (U9's gate or RV89).
2. **N-5:** optional, wider F2a.

## Records

`evidence/r2/` (placeholder paths only):
- the scripts: the v2 dumpers, `rv94_compare2.py`, `rv94_mutants2.py` and the cargo chain;
- the dumps in all three languages on both heads, and the TS IPC dumps;
- the comparison outputs;
- the mutant results;
- the sweep TSVs and hashes, and the live successor hashes;
- the outcome lists.

`inputs_sha256.txt` records the input file, which is not kept. The updated `SHA256SUMS` covers the whole folder, REVIEW.md included.

**Cleanup:** `WT/rv94/` and `WT/targets/rv94/` are deleted again; scratch logs remain.
