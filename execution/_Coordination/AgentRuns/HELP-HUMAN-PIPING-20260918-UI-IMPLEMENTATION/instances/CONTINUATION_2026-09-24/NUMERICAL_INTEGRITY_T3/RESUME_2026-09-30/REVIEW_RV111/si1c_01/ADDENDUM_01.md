# RV111 — addendum 01: SI1c repair round 1 confirmed

TASK (Type 2) RV111, continued by ROOT, the return path. The dispatch is ROOT's message for RR "RV111 passes SI1c; I88's repair round dispatched". I delegated nothing. Placeholders are as in `REVIEW.md`; no machine paths appear here or in `evidence/addendum_01/`. `REVIEW.md` and `SHA256SUMS` are untouched. This addendum and its evidence are sealed in `SHA256SUMS.addendum_01`.

## Result

**CONFIRMED.** Head `f5665f8862959b7a862d7d69543718f1d7240148` on `codex/piping-t3-si1c-20261007` is the pushed head (`ls-remote`). It is two commits over `7f233b2e01`:
- `fa78c80c32`: `RCR` and the runner tests;
- `f5665f8862`: `EE` tests only.

It closes SF-1 and N-1, carries out rulings 2 and 3, and acts on N-2. **No residual finding.** One observation, which needs no action, is in §4.

I formed my view from the diff and my own runs before I read I88's `REPAIR_01.md` (sha256 `78429963d3f224e9…`). Its claims agree with my results (§7).

## 1. SF-1: N06 and N09 now die, each on an assertion

I applied my round-0 patch strings unchanged to a fresh `git archive` copy of the head and ran the runner crate's whole `cargo test` (`evidence/addendum_01/mutants/`):
- **N06** (de-duplication removed): killed by `a_non_finite_input_listed_twice_is_named_once`, at `assert_eq!` (point_path_non_finite_run.rs:620). The left side has two `NonFiniteInput` records for x; the right side has one.
- **N09** (`trim` dropped from the raw-unit test): killed by `a_non_finite_value_in_a_padded_copy_of_the_declared_unit_is_named_not_unsupplied`, at `assert!(check.completeness_findings.is_empty())` (:658).

**The new tests use my probe's inputs.** The listed-twice test uses x NaN, plain and with b = 0.5. The padded-unit test covers both cases:
- referenced: one finding, `RULE_EVALUATOR_ERROR`, supplied, no value, the note;
- unreferenced: `USER_RULE_CHECKED` with no finding.

On the head my probe's five lines are byte-identical to round 0's candidate.

## 2. Ruling 2: the note is appended, in that order

- **The code.** `RCR`'s non-test change against `7f233b2e01` is exactly the note expression: `(false, note) → note`, `(true, None) → N-4's note`, `(true, Some(existing)) → "<existing>; <N-4's note>"`, plus a doc line (`evidence/addendum_01/differential/rcr_nontest.diff`). `RCR`'s `mod tests` is unchanged.
- **My mutants of the note form, all killed by assertions** (`a_non_finite_input_is_named_and_never_bound` and `the_n4_note_follows_an_existing_note`):
  - replace, the round-0 form (I88's N11);
  - prepend, "N-4; existing" (I88's N12);
  - a separator of a space for "; ";
  - the existing note only.

  Each fails at `assert_eq!` with the exact expected string, for example `Some("interval ±2e0 from receipt; non-finite value (NaN or ±inf, after unit normalization): not bound")`.
- **Both kinds of note are pinned:** the interval note ("interval ±5e-1 from receipt; …" and "interval ±2e0 from receipt; …") and the library provenance note ("resolved from private library invented_kind:… (value stays in the private library; never embedded in the rule pack); …").
- **Differential:** across my whole runner set, the only lines that differ from round 0 are those where an N-4 input already had a note. Every such note is exactly "<main's note>; <N-4's note>": 7,608 interval notes and 2,520 library notes over 10,128 lines. With those notes put back, every one of those lines equals round 0's.

## 3. Ruling 3: the split keeps every earlier assertion

I read all 32 removed lines of `7f233b2e01..f5665f8862` (`evidence/addendum_01/differential/removed_lines.txt`):
- **`EE` (23 lines):**
  - two renamed `fn` lines;
  - the split's old `fn` line and its 17-line `over_infinity` block, both of whose assertions (`OVERFLOWED_PRODUCT`; value `None`) reappear verbatim in `a_same_dimension_quotient_over_an_overflowing_divisor_blocks_at_the_multiply`. A script check finds no removed `EE` code line that is not re-added verbatim;
  - three comment lines.

  The other three cases (the largest ratio, a unit mismatch first, a zero divisor) stay in `same_dimension_quotients_of_finite_operands_are_unchanged`, with every assertion unchanged.
- **`RCR` (5 lines):** the old note expression (4 lines) and its doc line.
- **The runner test file (4 lines):**
  - the import list (2 lines), reflowed to add `LibraryValueBinding`;
  - one rename;
  - one assertion, replaced by a stricter one. A non-bounded input or s must still read N-4's note alone; a bounded x must read "interval ±5e-1 from receipt; <N-4's note>".

**No assertion was weakened or dropped.** `EE`'s non-test code is byte-identical to `7f233b2e01`'s (sha256 of the part before `#[cfg(test)]`: `2eecabcc3ab399f4…`).

## 4. N-2: the renames name what each test pins

| New name | What it asserts |
|---|---|
| `a_table_check_over_an_overflowing_argument_blocks_at_the_multiply` (runner) | interpolate, step and exact over `actual·1e300 − actual·1e300` all give `NonFiniteInput`/`multiply` with the product message |
| `nan_forming_interpolation_and_step_arguments_block_at_the_multiply` | interpolate and step over the NaN-forming argument give `OVERFLOWED_PRODUCT` |
| `generated_nan_forming_table_arguments_block_at_their_producer` | the two generated arguments block at `multiply` and `add_subtract` respectively |
| `same_dimension_quotients_of_finite_operands_are_unchanged` | the three finite-operand cases above |
| `a_same_dimension_quotient_over_an_overflowing_divisor_blocks_at_the_multiply` | the moved case |

`blocks_overflowing_same_dimension_quotient_instead_of_panicking` keeps its name. Its first half still pins the ratio arm's overflow block. **Observation (no action):** its second half (carried numerators, now blocked at `multiply`) is not what the name says, but N-2 was optional and the name is not false for the first half. I accept I88's choice.

## 5. Suites at the head, test by test

| Suite | Head `f5665f8862` |
|---|---|
| `expression_evaluator` lib / conformance_corpus / doc | **65** / 1 / 0 |
| `rule_check_runner` lib / acceptability / interval_bounds / invented_demo / point_path_non_finite_run / rule_interval_cases / doc | 14 / 4 / 11 / 3 / **11** / 1 / 0 |
| `rule_pack_document` lib / corpus_parity / invented_demo_document / doc | 6 / 1 / 3 / 0 (**10**) |
| `P/tests/test_rule_interval.py` (VENV python, `-v`, under the T3 lock) | **193 passed** |

**Test-name delta against round 0's candidate:**
- removed: the four renamed or split names;
- added: their four new names, `a_same_dimension_quotient_over_an_overflowing_divisor_blocks_at_the_multiply`, and the three new runner tests (`a_non_finite_input_listed_twice_is_named_once`, `a_non_finite_value_in_a_padded_copy_of_the_declared_unit_is_named_not_unsupplied`, `the_n4_note_follows_an_existing_note`).

Every test passes (`evidence/addendum_01/suites/`).

## 6. Scope and interval mode

- **The fence** against main `025c1cf326` is still exactly the five files. Against `7f233b2e01` only three change:
  - `EE`, in `mod tests` only;
  - `RCR`, the note expression and its doc only;
  - the runner test file.

  README and `test_rule_interval.py` are byte-identical.
- **My harness at the head** (`rv111_si1c.rs` and `rv111_probe.rs`, unchanged; one job through the lock):
  - all 144,407 evaluator point lines are byte-identical to round 0's candidate;
  - **all 433,221 interval lines are byte-identical to round 0's candidate and to main**;
  - the probe is identical;
  - of 106,884 runner lines, 96,756 are identical and the other 10,128 differ only by the appended notes (§2);
  - **every runner line validates** against `rule_check_run_result.schema.json`, with no `null`;
  - 0 violations (`evidence/addendum_01/differential/a1_report.json`). Dump hashes are in `dump_sha256_head2.txt`; the dumps are kept gzipped in scratch.

## 7. I88's `REPAIR_01.md` against my runs

Consistent:
- the head and commits;
- the three changed files and the fence;
- the note append form;
- the two SF-1 tests and the note test;
- the renames and the split;
- the suite counts (65; 11; 10; 193);
- `EE`'s non-test identity (`2eecabcc3ab399f4…`);
- N06 and N09 killed by the new tests alone;
- N11 and N12 killed by `the_n4_note_follows_an_existing_note` and `a_non_finite_input_is_named_and_never_bound`.

I88 wrote "not pushed (ROOT pushes)"; the remote ref is now `f5665f8862`. I did not re-run I88's harnesses or its 57 mutants.

## 8. Host

- Every cargo command went through `WT/tools/t3_cargo.sh` with `--offline --locked`: 10 jobs, each with a START and an END (`evidence/addendum_01/host/cargo_jobs_rv111_a1.txt`). Fresh copies were `WT/rv111/{head2,mut2}` (`git archive` of `f5665f8862`), with targets `WT/targets/rv111-{head2,mut2}`. I killed no job.
- pytest and the dump comparison (VENV python, with schema validation) ran under `/usr/bin/lockf -k WT/guard/cargo_job.lock`. No test binary was run directly.
- One wait per job; no wait of mine is running. All paths were absolute; scratch and `TMPDIR` were in `WT/scratch/rv111_si1c_01/`.
- No Git writes and no DEC-025.
- **Deleted:** `WT/rv111/` and `WT/targets/rv111-{head2,mut2}`. **Kept:** the head's dumps, gzipped, with their hashes.
