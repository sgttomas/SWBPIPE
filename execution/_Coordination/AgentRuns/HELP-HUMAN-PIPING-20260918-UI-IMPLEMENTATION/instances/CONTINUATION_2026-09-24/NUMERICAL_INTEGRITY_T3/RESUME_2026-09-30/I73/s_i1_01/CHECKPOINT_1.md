# I73 — S-I1 checkpoint 1: the interval evaluator

TASK (Type 2) I73, dispatched by ROOT (HELP_HUMAN). Brief `R/BRIEFS/I73_S_I1.md` (sha256 `9e40b13a…`). This is a checkpoint, not the final return: deliverable 1 (the interval mode in `expression_evaluator`), its own tests, and the point-mode differential (control 1) for `expression_evaluator`. Runner wiring, the Python reference and the case file follow after ROOT continues me.

Placeholders: `WT` is the T3 host root; `NUM = WT/numerics`; `P = projects/chirality-piping`; `T`, `R` and D2 as in the dispatch. No machine paths are recorded.

## 1. Basis read (origins and sha256)

| Record | sha256 |
|---|---|
| `NUM/AGENTS.md` | `f96feb19d297c74e…` |
| `NUM/agents/AGENT_TASK.md` | `1a13a5b00b3ce01f…` |
| `NUM/P/AGENTS.md` | `d9f2b23ad50669fd…` |
| `R/BRIEFS/I73_S_I1.md` | `9e40b13abb2d85d8…` |
| `R/BRIEFS/COMMON.md` | `871240f2ed90b1fd…` |
| D2, `T/DESIGN_STANDING/DESIGN.md` (rev 5b.2): §0.3, §0.4, §4.11.1–§4.11.6 | `edc78f9c20db9848…` |
| `R/I61/u8_plan_01/PLAN.md` §3 | `f274a6149051292c…` |
| `T/DESIGN_NUMERICS/DESIGN.md` §6 | `fb62ef4a8282e53f…` |
| `RR` (`T/ROOT_RULINGS_V1.md`), the 2026-10-05 U8-plan ruling, decision 13 | `cc8c0459b56b09f8…` |

NUM was at `7c2ce2637f` when read. Base head for every comparison: `c1bfc460fc` (worktree `WT/s-i1`, branch `codex/piping-s-i1-20261005`).

**Host note.** This session started in a different isolated worktree, and its edit guard refused writes to `WT/s-i1`. I switched this agent into `WT/s-i1` with the host's supported worktree switch (no Git write), then edited there. I will switch the same way into NUM to write these records.

## 2. What is done

**One changed file, purely additive:** `P/core/rules/expression_evaluator/src/lib.rs`, base sha256 `461eb4f4…`, candidate sha256 `9b529e7f71e58cbc866bcf1f8fca56ba70dc88ae22038bdb078aa2915baee5af`. `git diff -U0` has two insertion hunks and **zero removed lines**:
- after `validate_finite` (base line 1467): **1,052 lines**, the interval mode;
- after the existing `mod tests` (base line 2331): **1,030 lines**, a new `mod interval_tests`.

Nothing else in the worktree is changed. No `Cargo.toml` or lock change, no new dependency, and no schema change.

**Public API (all new; nothing existing is altered):**
- `evaluate_interval(&EvaluationInput, &[IntervalBinding]) -> IntervalEvaluationResult`
  - It reads the same `EvaluationInput` as `evaluate`.
  - Each `IntervalBinding { variable_id, enclosure: Option<Enclosure> }` overlays an enclosure on one bound variable. `None` means the input has no finite enclosure.
- `enclosure_from_bound(q, b) -> Option<Enclosure>` forms `[next_down(fl(q − b)), next_up(fl(q + b))]`. When b = 0 it gives the exact point q.
- The types `Truth {True, False, Indeterminate}`, `Enclosure {lo, hi}` (with `bits_text()` giving `[0x<lo bits>,0x<hi bits>]`), `IntervalValue`, `IntervalQuantity`, `IntervalNote`, `IntervalNoteCode` and `IntervalEvaluationResult`.

**How it is built:**
- **Structure is checked by the point path itself.** For each node, the interval path first calls the point path's own checker on a "shadow" value of the same kind and metadata. The placeholder value is 1.0, so it is never a zero divisor. The checkers are `eval_unary`, `eval_binary`, `eval_compare`, `eval_logical`, `eval_select`, `eval_variable_ref`, the literal arm of `eval_expression`, `validate_table`, `build_binding_map`, `check_required_variables`, `check_grammar_version` and `collect_statuses`.
  - So type, dimension, unit, table, grammar and status findings are identical in code, subject, wording and order to `evaluate`'s.
  - The two point functions that evaluate their own operands, `eval_aggregate` and `eval_table_expression`, are mirrored line by line, with their strings and the same `?` short-circuits.
- **The enclosure follows D2's table:**
  - add, subtract, multiply and divide: the four endpoint products or quotients;
  - abs: three branches;
  - negate, min and max: exact;
  - compare: T/F/U with strict ends; `=` is T only for two equal points;
  - Kleene not, and, or;
  - eager select: U takes the hull for quantities, and gives agreement-or-U for booleans;
  - step lookup: the hull of the spanned rows;
  - exact lookup: a point argument follows the point path; anything else is U;
  - unsupported and unsafe forms: blocked as today.
- **Exhaustive matches.** Every `Expression` and operator match is exhaustive with no `_` arm. Impossible kind combinations push a blocking `TypeMismatch`.

## 3. The outward-rounding approach, and why

**The approach:**
- After every floating operation, `next_down` is applied to the lower end and `next_up` to the upper end. These are `f64::next_down`/`next_up`, stable since Rust 1.86; the host and CI pin 1.97.1.
- This is D2 §4.11.3's "outward step after each operation".
- Exact operations take no step: negate, abs, min, max, hull, step-lookup rows, and a point argument that hits an interpolation or exact-lookup row.
- Any non-finite end, before or after the step, gives no enclosure.

**Why this approach and not directed rounding:**
- **Rust has no supported directed rounding.** Changing the FP environment is outside the language's float model, and LLVM assumes round-to-nearest.
- **It is bit-identical across languages.** Python (`math.nextafter`) and TS (the bit-level `outward` helper) reproduce the same bits. That lets the shared case file pin enclosure bits, with no host dependence.
- **It is always sound.** The cost is at most one extra ulp per operation, which only widens the enclosure.
- **Ties are fixed.** `min2` and `max2` use a fixed comparison order, so signed-zero ties give the same bits in every language.

## 4. Tests and results

**Suites:** `cargo test --locked --offline` on the evaluator crate, through `WT/tools/t3_cargo.sh`, in target `WT/targets/i73-s-i1`.

| Crate suite | Base `c1bfc460fc` | Candidate |
|---|---|---|
| `expression_evaluator` lib | 31 passed | **48 passed** (+17: the new `interval_tests`; all 31 existing tests unchanged and passing) |
| `expression_evaluator` `tests/conformance_corpus.rs` | 1 passed | 1 passed |
| doc-tests | 0 | 0 |

`rule_check_runner` (14 + 4 + 3) and `rule_pack_document` (6 + 1 + 3) were run on the base only, for later comparison; their candidate runs come with the runner wiring.

**The 17 new tests:**
- **Exact bits:**
  - the outward step on points, for example `1 + 2 → [next_down(3), next_up(3)]`;
  - every sign combination of × and ÷;
  - the three abs branches;
  - bound formation, including a subnormal bound;
  - aggregates;
  - select hull;
  - step lookup.
- **Three-valued compares** for all six operators, with strictness at shared ends.
- **Kleene logic.**
- **D2 §4.11.5's negative controls:**
  - `abs(x) ≥ c` with x straddling 0;
  - `x·x ≤ c` (written `(x/1)·x`, since stress×stress has no grammar product);
  - division by a range containing 0;
  - `not(x > c)`;
  - two interval inputs with an interior extremum;
  - `=` and `≠` on overlapping ranges;
  - `select` with a U condition;
  - interpolation partly out of range.

  Each reads U.
- **Overlay validation.**
- **Structural parity with `evaluate`** on 27 broken expressions, plus the grammar-version and status cases.
- **`interval_outcomes_are_sound_against_the_point_path`**, a seeded property test:
  - 4,000 generated formulas over the whole grammar, each with three inputs (two stress, one ratio) given random bounds;
  - each is checked against the **unchanged point path** at 16 sampled points per box (ends, neighbouring ulps, interior points, row arguments and zero);
  - **T** requires every point to evaluate to `true`; **F** requires every point to evaluate to `false`; a finite enclosure must contain every point value; an interval-mode block requires every point to block;
  - tally: T 763, F 770, U 752, quantity 1,237, blocked 478.

**Mutants** (`_run_records/checkpoint1/ee_mutants.py`): each patch applies exactly once, and `cargo test --lib` must fail. **All 11 were killed.**

| Mutant | Killed by |
|---|---|
| **M1** outward step removed (brief's control 3a) | 6 tests, including `every_floating_operation_steps_outward_even_on_points` and `rounding_that_hides_a_real_excess_is_not_a_pass` |
| **M2** U arm of every compare → T (3b) | 8 tests, including the comparisons test and the property test |
| **M4a** multiply made inward (3d) | the sign-combination test and the property test |
| M4b abs straddle made inward | the abs test and the property test |
| M4c interpolation as D2's literal point hull (see §5.1) | the rounding-near-a-row test and the property test |
| M5 indeterminate poisoning removed | 3 tests |
| M6 divisor containing 0 not refused | 2 tests |
| M7 Kleene `T and U = T` | 2 tests |
| M8 select-U takes the then-branch | 2 tests |
| M9 step hull keeps only the first row | 2 tests |
| M10 interpolation range check dropped | 2 tests |

The brief's mutant 3c, "indeterminate collapsed to pass", is a runner mutant and comes with the runner wiring.

## 5. Control 1 for `expression_evaluator`: the point-mode differential

**How it was run:**
- A scratch harness, `i73_point_diff.rs`, was put into the `tests/` folder of two disposable trees:
  - a `git archive c1bfc460fc` copy;
  - the same archive with the candidate `lib.rs` laid over it.

  The harness is the base `conformance_corpus.rs` up to its `#[test]` (704 lines, byte-for-byte), plus `_run_records/checkpoint1/point_diff_tail.rs`.
- It dumps `evaluate`'s full `Debug` output, and the result's value bits, for:
  - all **69 committed conformance-corpus cases**;
  - **36,000 seeded generated inputs**: 6,000 formulas over the whole grammar × 6 point bindings, with varied status sets, missing and required variables, and a wrong grammar version.

**Result:**
- Both dumps have **36,069 lines** and the **identical sha256 `e74a69c43af2b277…`**. `cmp` reports them identical.
- Mix: 2,669 `true`, 2,598 `false`, 5,970 quantities, 24,814 blocked, and 18 point-path panics (see §6.3), identical on both sides.
- Point mode is unchanged by construction, since no point-path line is edited. The differential shows it empirically.

## 6. Readings beyond D2's text that need a ruling

### 6.1 Interpolation: D2's literal rule is unsound, so I implemented per-segment outward evaluation (RULING NEEDED)

**What D2 says.** D2 §4.11.3 gives "the hull of the interpolated values at lo, at hi and at every row argument strictly inside, each evaluated as the point path does and then widened outward". Read literally (one ulp around each point value), this breaks D2's own lemma (§4.11.4) in two ways. Both are reproduced in `_run_records/checkpoint1/interp_probe.py`.

**The counterexample.** Rows (−1e10, 1e20), (1, 8000), (2, 8000), with x ∈ [0.5, 2]:
- **The point path:** at x = next_down(1), the point path returns **0.0**. The rise 8000 − 1e20 rounds to −1e20, and the fraction rounds to 1. The literal hull is [7999.999…, 5000003584.000001]. So `interp(x) ≥ 4000` would read **T**, while the point path fails at next_down(1).
- **The exact value:** at x = 0.5 the exact value is 5000007999.5, above that hull's top.

**What I implemented.** For every segment the argument range meets, the point path's own formula is evaluated over the clipped range, with D2's outward step after each of its six floating operations, and the segment enclosures are joined. A point argument at a row argument still returns that row's exact value.
- By the lemma, each operation encloses both the exact and the point-path values, so this follows D2's general rule ("after every floating operation…").
- Mutant M4c, which is the literal rule, is killed.

**Asked of ROOT:** confirm this reading, or rule otherwise. A D2 text amendment may be wanted.

### 6.2 Any indeterminate part makes the whole check indeterminate (RULING NEEDED)

**What D2 lists.** D2 lists U for a divisor range containing 0, for interpolation partly out of range, and for "any operation that produces a non-finite or NaN end".

**What I implemented.** These causes, plus step lookup out of range (§6.4), exact lookup over a non-point, and an input with no finite enclosure, each record a note. **Any note makes the whole result U** (a quantity loses its enclosure). They are not U locally, inside Kleene logic.

**Why.** The point path is eager:
- it blocks on a zero divisor or an out-of-range table argument even inside an untaken `select` branch or an `or` whose other side is true;
- it carries infinities and NaN onward, and it can panic on them (§6.3).

A local U could be rescued, for example `T or U = T`, while the point path blocks at some value in the box. That breaks D2's theorem ("T only if its predicate holds for every point assignment"). Mutant M5 (poisoning removed) is killed by the explicit untaken-branch and `or` tests and by the property test, which shows the poisoning is needed.

**Asked of ROOT:** confirm. This is stricter than a local reading, and D2's rationale for interpolation ("part of the range would block") points this way.

### 6.3 Pre-existing point-path panics (a finding outside S-I1's authority; not changed)

The ordinary `evaluate` panics instead of returning a finding in two cases. Both are reachable with large magnitudes (products near 1e154 or more, or tiny divisors):
- a same-dimension quotient that overflows: `divide` calls `ratio_quantity(…).expect(…)`, base line 1457;
- a NaN argument to `interpolate` or step `lookup`: the `.expect("… always has a … row")` calls.

**Observed:** 18 of the 36,069 differential inputs, identical on base and candidate, and 117 of the property test's samples.

**Effect on S-I1:**
- I did not touch these lines, because the point mode must stay unchanged.
- Interval mode is unaffected: its shadow uses 1.0, and a non-finite intermediate is U under §6.2.
- In the runner, a check with no interval input still follows the point path, so the panic remains reachable there exactly as today.

**Asked of ROOT:** log or assign separately.

### 6.4 Smaller readings (no ruling needed unless ROOT disagrees)

- **Step lookup out of range** is U: D2 is silent, and I followed interpolation's rule. An exact lookup with a point argument follows the point path, including its blocking findings.
- **A divisor equal to the point [0, 0]** is U with `divide_by_zero_range`, per D2's literal "contains 0". It is not the point path's `DivisionByZero` block. Both are non-pass.
- **U causes beyond divide-by-zero** have no wire code of their own. In the evaluator they are `IntervalNoteCode`s with stable tokens (`table_argument_range`, `exact_lookup_range`, `non_finite_enclosure`). In the runner I plan to map only `divide_by_zero_range` to D2's `RULE_INTERVAL_DIVIDE_BY_ZERO_RANGE`, and to name the others in the `RULE_RESULT_INDETERMINATE` message after the fixed-form enclosure. No new codes.

## 7. Plan for the continuation, with two interface choices for ROOT to see

**Choice 1: the runner gets an additive entry point, not new struct fields.**
- src-tauri builds `RuleCheckRunInput { … }` and `SolverResultBinding { … }` with struct literals (`src-tauri/src/lib.rs` near 2911, 3253 and 3319), and src-tauri is outside my fence. Adding a field would break its build.
- So the runner gets `run_rule_checks_with_bounds(&RuleCheckRunInput, &[SolverResultBound { input_id, absolute_bound }])`, and `run_rule_checks(input)` stays exactly today's path. With no bounds, it is byte-for-byte today's path.
- D2 §4.11.5 puts `SolverResultBinding.absolute_bound` in S-I2, which edits src-tauri anyway. S-I2 can add the field then and pass it through, or keep the side list.

**Choice 2: what the runner does with bounds and outcomes.**
- **Bounds:**
  - b > 0 binds `enclosure_from_bound(q, b)`;
  - each end is unit-normalized with an outward step after each of the units crate's four floating operations;
  - b = 0 binds the point;
  - an invalid b (negative, NaN or infinite) is a blocking completeness finding, so the input is treated as unsupplied.
- **When interval mode runs:** only when at least one formula variable carries an interval.
- **Outcomes:** T, F and U map to the statuses and codes of D2 §4.11.5, including the synthesized `Compare`.
  - The finding message is `enclosure=[0x…,0x…] unit=<u>`.
  - For an interval quantity check, `computed_value` is **omitted**, since no single value is computed.
  - `BoundInput.note` reads `interval ±<b> from receipt`, with b in `{:e}` form.

  These are readings, and they appear in the final RETURN.

**Then:** the Python reference `P/core/analysis_runs/rule_interval.py`; the case file `P/fixtures/rule_interval/rule_interval_cases.json`, about 60 cases with independently derived expectations and an exact-rational soundness oracle; the Python tests; runner control 1 over the committed packs and run fixtures; mutant 3c; and the suites, including src-tauri and the Python sweep.

## 8. Run records

These are in `_run_records/checkpoint1/`. Paths are sanitized to `WT`/`<WT>`.
- `cargo_run.sh`: the lock wrapper invocation used for every cargo command.
- `ee_mutants.py` and `ee_mutants_results.json`.
- `point_diff_tail.rs`: the differential harness tail.
- `interp_probe.py`: the §6.1 counterexample, standard library only.
- `*.summary.txt`: suite lines for the base and candidate runs and the differential runs.

The 10 MB dumps stay in host scratch (`WT/scratch/i73_s_i1/diff_ee/`), with the sha256 given in §5.

**Commands:** `cargo test --locked --offline` (the candidate lib run added `-- --nocapture --test-threads 2` to print the tally) and `cargo test --locked --offline --test i73_point_diff`. Every cargo command ran through `WT/tools/t3_cargo.sh` with `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `RUSTUP_TOOLCHAIN=1.97.1` and `CARGO_INCREMENTAL=0`, in the targets `WT/targets/i73-s-i1` and `-base`, with the memory guard running.

There were no Git writes, installs, or native, DEC-025 or solver jobs, and nothing was written to the system temp directory.
